//! Canonical M5 runtime inspector cards — one first-class inspector per claimed M5
//! ecosystem family that surfaces actual runtime truth without raw supervisor traces.
//!
//! Where the [`activation-budget`](crate::m5_activation_budget) module reports what a
//! family *cost* to activate over a session, and the
//! [`lifecycle-actions`](crate::m5_lifecycle_actions) module models what happens to a
//! package after install, this module freezes the *runtime inspector* an author or
//! operator opens to read the live state of one installed or locally-built family:
//! its activation time, current host, granted capabilities, recent logs, recent
//! failures, hot-reload posture, and the quarantine/disable/re-enable actions offered.
//! The inspector must stay useful when the package is failing or quarantined, so each
//! card keeps a [`LastKnownGoodState`] visible whenever the current load failed or the
//! source path disappeared.
//!
//! Each [`RuntimeInspectorCard`] reuses the shared M5 vocabulary —
//! [`ArtifactFamily`], [`SourceClass`], [`RuntimeClass`], [`HostAbiClass`],
//! [`SignatureState`], [`TrustPosture`], [`AntiAbuseTransparency`], and
//! [`HotReloadPosture`] — plus the activation-budget capability and resource
//! vocabulary ([`CapabilityClass`], [`CapabilityExerciseState`], [`ActivationBucket`],
//! [`ResourcePressure`]), and adds the inspector-specific facts: the current
//! [`LoadState`], an [`ActivationProfile`] (activation time and memory), the
//! [`GrantedCapability`] set with declared-versus-exercised state, the redacted
//! [`LogEntry`] and [`RecentFailure`] history, and the [`InspectorAction`] set.
//!
//! The card is honest by construction. Three published values are **recomputed** from
//! the card's facts, and the stored values must equal the recomputation or validation
//! fails:
//!
//! - **the rendered trust tier** is capped by the signing state, so a card whose
//!   signature is an unsigned local-dev build, an unsigned side-load, or a revoked
//!   signature renders [`TrustPosture::UnsignedLocalOnly`] and never inherits a
//!   verified-publisher or enterprise-approved badge just because the machine holds a
//!   trusted key;
//! - **the review-trigger set** is computed from the hot-reload posture and the
//!   granted-capability state: a hot reload that would widen the runtime class, expand
//!   permissions, or add an external executable — or an undeclared exercised
//!   capability — each force a [`InspectorDisposition::FreshReviewRequired`] candidate,
//!   so widening authority can never take effect through a silent hot reload; and
//! - **the disposition** is the strongest of the load-state base, that fresh-review
//!   gate, and a hard [`InspectorDisposition::Quarantined`] for an anti-abuse
//!   quarantine.
//!
//! A card whose current load failed or whose source path disappeared must carry a
//! [`LastKnownGoodState`], and that last-good state can never render a stronger badge
//! than the family's current cap. The packet is checked in at
//! `artifacts/ecosystem/m5/m5-runtime-inspector.json` and embedded here, so this typed
//! consumer and any CI gate agree on every card without a cargo build in CI. The model
//! is metadata-only: every field is a typed state, a redacted display hint, or an
//! opaque ref. It carries no absolute filesystem paths, raw log bodies, supervisor
//! traces, signing secrets, or provider payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::ArtifactFamily;
use crate::m5_activation_budget::{
    ActivationBucket, CapabilityClass, CapabilityExerciseState, ResourcePressure,
};
use crate::m5_author_and_publish_preview::{
    AntiAbuseTransparency, HostAbiClass, HotReloadPosture, RuntimeClass, SignatureState,
    TrustPosture,
};
use crate::m5_marketplace_fact_views::SourceClass;

/// Supported M5 runtime-inspector schema version.
pub const M5_RUNTIME_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_RUNTIME_INSPECTOR_RECORD_KIND: &str = "m5_runtime_inspector";

/// Repo-relative path to the checked-in packet.
pub const M5_RUNTIME_INSPECTOR_PATH: &str = "artifacts/ecosystem/m5/m5-runtime-inspector.json";

/// Embedded checked-in packet JSON.
pub const M5_RUNTIME_INSPECTOR_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-runtime-inspector.json"
));

/// The current load state of an inspected runtime.
///
/// The inspector stays useful in every state. A [`LoadState::LoadFailed`] or
/// [`LoadState::SourceMissing`] card keeps its [`LastKnownGoodState`] visible rather
/// than collapsing to an empty surface, and a [`LoadState::QuarantineHeld`] or
/// [`LoadState::OperatorDisabled`] card still exposes its logs and crash history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadState {
    /// Loaded and running with no recent failures.
    LoadedHealthy,
    /// Loaded and running, but with recent failures on record.
    LoadedDegraded,
    /// The current load failed; the last-known-good state is shown instead.
    LoadFailed,
    /// The source path disappeared; the last-known-good state is shown instead.
    SourceMissing,
    /// Held under an anti-abuse quarantine.
    QuarantineHeld,
    /// Disabled by an operator.
    OperatorDisabled,
}

impl LoadState {
    /// Every load state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LoadedHealthy,
        Self::LoadedDegraded,
        Self::LoadFailed,
        Self::SourceMissing,
        Self::QuarantineHeld,
        Self::OperatorDisabled,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadedHealthy => "loaded_healthy",
            Self::LoadedDegraded => "loaded_degraded",
            Self::LoadFailed => "load_failed",
            Self::SourceMissing => "source_missing",
            Self::QuarantineHeld => "quarantine_held",
            Self::OperatorDisabled => "operator_disabled",
        }
    }

    /// Whether the runtime is currently loaded and running.
    pub const fn is_running(self) -> bool {
        matches!(self, Self::LoadedHealthy | Self::LoadedDegraded)
    }

    /// Whether this state requires a [`LastKnownGoodState`] to remain visible.
    ///
    /// A failed current load or a disappeared source path must never collapse the
    /// inspector — the last-good state stays on the card.
    pub const fn requires_last_known_good(self) -> bool {
        matches!(self, Self::LoadFailed | Self::SourceMissing)
    }

    /// Whether the runtime is held under an anti-abuse quarantine.
    pub const fn is_quarantine_held(self) -> bool {
        matches!(self, Self::QuarantineHeld)
    }

    /// Whether the runtime is disabled by an operator.
    pub const fn is_operator_disabled(self) -> bool {
        matches!(self, Self::OperatorDisabled)
    }
}

/// The severity level of a redacted log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    /// Routine informational line.
    Info,
    /// A notable but non-error event.
    Notice,
    /// A recoverable warning.
    Warning,
    /// An error.
    Error,
}

impl LogLevel {
    /// Every log level, in declaration order.
    pub const ALL: [Self; 4] = [Self::Info, Self::Notice, Self::Warning, Self::Error];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Notice => "notice",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Whether this level is an error.
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

/// The class of a recent runtime failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    /// Activation exceeded its time budget.
    ActivationTimeout,
    /// The runtime crashed during activation.
    CrashOnActivation,
    /// The runtime crash-looped.
    CrashLoop,
    /// The external or remote host disconnected.
    HostDisconnect,
    /// A requested capability was denied.
    CapabilityDenied,
    /// Memory was exhausted.
    MemoryExhausted,
    /// A capability was exercised that was never declared.
    UndeclaredCapabilityUse,
}

impl FailureClass {
    /// Every failure class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ActivationTimeout,
        Self::CrashOnActivation,
        Self::CrashLoop,
        Self::HostDisconnect,
        Self::CapabilityDenied,
        Self::MemoryExhausted,
        Self::UndeclaredCapabilityUse,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivationTimeout => "activation_timeout",
            Self::CrashOnActivation => "crash_on_activation",
            Self::CrashLoop => "crash_loop",
            Self::HostDisconnect => "host_disconnect",
            Self::CapabilityDenied => "capability_denied",
            Self::MemoryExhausted => "memory_exhausted",
            Self::UndeclaredCapabilityUse => "undeclared_capability_use",
        }
    }

    /// Whether this failure is a crash loop.
    pub const fn is_crash_loop(self) -> bool {
        matches!(self, Self::CrashLoop)
    }
}

/// A reviewed action offered through the inspector card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorActionKind {
    /// View the (redacted) runtime logs.
    ViewLogs,
    /// Restart the runtime.
    Restart,
    /// Reload the runtime from its source.
    ReloadSource,
    /// Disable the package for this workspace.
    DisableForWorkspace,
    /// Disable the package globally.
    DisableGlobally,
    /// Quarantine the package.
    Quarantine,
    /// Re-enable a disabled or quarantined package (routes through review).
    ReEnable,
    /// Request a fresh review of a widened hot reload.
    RequestFreshReview,
}

impl InspectorActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ViewLogs,
        Self::Restart,
        Self::ReloadSource,
        Self::DisableForWorkspace,
        Self::DisableGlobally,
        Self::Quarantine,
        Self::ReEnable,
        Self::RequestFreshReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewLogs => "view_logs",
            Self::Restart => "restart",
            Self::ReloadSource => "reload_source",
            Self::DisableForWorkspace => "disable_for_workspace",
            Self::DisableGlobally => "disable_globally",
            Self::Quarantine => "quarantine",
            Self::ReEnable => "re_enable",
            Self::RequestFreshReview => "request_fresh_review",
        }
    }

    /// Whether this action would resume or reload the runtime, applying any pending
    /// widening — it must never be enabled while a fresh review is required.
    pub const fn applies_runtime_change(self) -> bool {
        matches!(self, Self::Restart | Self::ReloadSource)
    }
}

/// A computed trigger that forces a fresh review before a change applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorReviewTrigger {
    /// A hot reload would widen the runtime class.
    RuntimeClassWidened,
    /// A hot reload would expand permissions.
    PermissionsWidened,
    /// A hot reload would add an external executable.
    ExternalExecutableAdded,
    /// A capability was exercised that was never declared.
    UndeclaredCapabilityExercised,
}

impl InspectorReviewTrigger {
    /// Every review trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RuntimeClassWidened,
        Self::PermissionsWidened,
        Self::ExternalExecutableAdded,
        Self::UndeclaredCapabilityExercised,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeClassWidened => "runtime_class_widened",
            Self::PermissionsWidened => "permissions_widened",
            Self::ExternalExecutableAdded => "external_executable_added",
            Self::UndeclaredCapabilityExercised => "undeclared_capability_exercised",
        }
    }

    /// The minimum disposition this trigger forces.
    pub const fn min_disposition(self) -> InspectorDisposition {
        InspectorDisposition::FreshReviewRequired
    }
}

/// The disposition a runtime inspector card publishes.
///
/// Ordered low-to-high by [`InspectorDisposition::rank`]: a
/// [`InspectorDisposition::RunningHealthy`] card runs clean, and a
/// [`InspectorDisposition::Quarantined`] card is held pending anti-abuse review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorDisposition {
    /// Loaded and running with no recent failures or pending widening.
    RunningHealthy,
    /// Loaded and running, but with recent failures on record.
    RunningDegraded,
    /// The current load failed or the source disappeared; last-known-good is shown.
    ShowingLastKnownGood,
    /// A widening hot reload (or undeclared capability use) requires a fresh review.
    FreshReviewRequired,
    /// Disabled by an operator.
    OperatorDisabled,
    /// Held under an anti-abuse quarantine.
    Quarantined,
}

impl InspectorDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunningHealthy,
        Self::RunningDegraded,
        Self::ShowingLastKnownGood,
        Self::FreshReviewRequired,
        Self::OperatorDisabled,
        Self::Quarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunningHealthy => "running_healthy",
            Self::RunningDegraded => "running_degraded",
            Self::ShowingLastKnownGood => "showing_last_known_good",
            Self::FreshReviewRequired => "fresh_review_required",
            Self::OperatorDisabled => "operator_disabled",
            Self::Quarantined => "quarantined",
        }
    }

    /// Monotonic rank; higher means a stronger hold.
    pub const fn rank(self) -> u8 {
        match self {
            Self::RunningHealthy => 0,
            Self::RunningDegraded => 1,
            Self::ShowingLastKnownGood => 2,
            Self::FreshReviewRequired => 3,
            Self::OperatorDisabled => 4,
            Self::Quarantined => 5,
        }
    }

    /// The stronger (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    /// Whether the runtime is held (disabled or quarantined) rather than running.
    pub const fn is_held(self) -> bool {
        matches!(self, Self::OperatorDisabled | Self::Quarantined)
    }
}

/// The measured activation time and memory of an inspected runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivationProfile {
    /// Whether the last activation was cold or warm.
    pub bucket: ActivationBucket,
    /// Measured activation time in milliseconds (`0` when not currently activated).
    pub activation_millis: u32,
    /// Memory pressure relative to the family's budget.
    pub memory_pressure: ResourcePressure,
    /// Peak resident memory in mebibytes (`0` when not currently activated).
    pub peak_memory_mib: u32,
}

impl ActivationProfile {
    /// Whether the runtime is over its memory budget.
    pub const fn is_over_memory_budget(&self) -> bool {
        self.memory_pressure.is_over_budget()
    }
}

/// One granted capability, with its declared-versus-exercised state this session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrantedCapability {
    /// The class of capability granted.
    pub class: CapabilityClass,
    /// Whether the capability was exercised, unused, or undeclared this session.
    pub exercise_state: CapabilityExerciseState,
    /// Redacted target label for the capability.
    pub target_label: String,
    /// The author's rationale for the capability.
    pub rationale: String,
}

impl GrantedCapability {
    /// Whether this capability is an undeclared exercise — a policy violation.
    pub const fn is_undeclared_exercise(&self) -> bool {
        self.exercise_state.is_undeclared_exercise()
    }

    /// Whether this capability is a declared-but-unused over-grant.
    pub const fn is_unused_grant(&self) -> bool {
        self.exercise_state.is_unused_grant()
    }
}

/// A redacted recent log entry. Carries no raw log body or supervisor trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogEntry {
    /// The severity level.
    pub level: LogLevel,
    /// Stable, redacted log code.
    pub code: String,
    /// Redacted message label (never a raw payload).
    pub message_label: String,
    /// Monotonic sequence number used to order the entries.
    pub seq: u32,
}

/// A redacted recent failure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecentFailure {
    /// The class of failure.
    pub class: FailureClass,
    /// How many times this failure occurred in the window.
    pub occurrence_count: u32,
    /// Redacted label for when the failure was last seen.
    pub last_seen_label: String,
    /// Redacted detail label (never a raw stack trace).
    pub detail_label: String,
}

impl RecentFailure {
    /// Whether this record is a crash loop.
    pub const fn is_crash_loop(&self) -> bool {
        self.class.is_crash_loop()
    }
}

/// The last-known-good state kept visible when the current load failed or the source
/// disappeared.
///
/// Its rendered trust tier can never exceed the family's current cap, so a stale good
/// state never shows a stronger badge than the package may currently carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LastKnownGoodState {
    /// Opaque ref to the last-known-good revision.
    pub revision_ref: String,
    /// The runtime class the last-good state ran as.
    pub runtime_class: RuntimeClass,
    /// The host/ABI the last-good state ran on.
    pub host_abi: HostAbiClass,
    /// Redacted label for when the last-good state was captured.
    pub captured_label: String,
    /// The trust badge the last-good state rendered.
    pub rendered_trust_tier: TrustPosture,
    /// Reviewer-facing summary of the last-good state.
    pub summary: String,
}

/// A scoped action offered through the inspector card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectorAction {
    /// The kind of action.
    pub action_kind: InspectorActionKind,
    /// Opaque ref to the action.
    pub action_ref: String,
    /// Whether the action is currently enabled.
    pub enabled: bool,
}

/// One runtime inspector card for a claimed M5 ecosystem family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInspectorCard {
    /// Stable card id.
    pub card_id: String,
    /// Opaque ref to the inspected listing or local workspace.
    pub listing_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this card resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust source class.
    pub source_class: SourceClass,
    /// Namespaced extension identity of the form `publisher/extension`.
    pub extension_identity: String,
    /// Declared version label.
    pub extension_version: String,
    /// The runtime class of the inspected artifact.
    pub runtime_class: RuntimeClass,
    /// The host/ABI the artifact currently runs on.
    pub current_host: HostAbiClass,
    /// The signing/provenance state.
    pub signature_state: SignatureState,
    /// Opaque ref to the signature record, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_ref: Option<String>,
    /// The trust posture the listing claims before capping.
    pub claimed_trust_tier: TrustPosture,
    /// The rendered trust posture; must equal the recomputed cap.
    pub rendered_trust_tier: TrustPosture,
    /// The anti-abuse transparency state.
    pub anti_abuse: AntiAbuseTransparency,
    /// The hot-reload/relaunch posture of the family's local-dev loop.
    pub hot_reload_posture: HotReloadPosture,
    /// The current load state.
    pub load_state: LoadState,
    /// The measured activation time and memory.
    pub activation: ActivationProfile,
    /// The granted capabilities with declared-versus-exercised state.
    #[serde(default)]
    pub granted_capabilities: Vec<GrantedCapability>,
    /// Recent redacted log entries.
    #[serde(default)]
    pub recent_logs: Vec<LogEntry>,
    /// Recent failures.
    #[serde(default)]
    pub recent_failures: Vec<RecentFailure>,
    /// The last-known-good state, required when the current load failed or the source
    /// disappeared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good: Option<LastKnownGoodState>,
    /// Scoped actions offered through the card.
    #[serde(default)]
    pub actions: Vec<InspectorAction>,
    /// Review triggers; must equal the recomputed set.
    #[serde(default)]
    pub review_triggers: Vec<InspectorReviewTrigger>,
    /// Disposition; must equal the recomputed value.
    pub disposition: InspectorDisposition,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl RuntimeInspectorCard {
    /// Whether any recent failure is on record.
    pub fn has_recent_failures(&self) -> bool {
        !self.recent_failures.is_empty()
    }

    /// Whether any recent failure is a crash loop.
    pub fn has_crash_loop(&self) -> bool {
        self.recent_failures
            .iter()
            .any(RecentFailure::is_crash_loop)
    }

    /// Whether any granted capability is an undeclared exercise — a policy violation.
    pub fn has_undeclared_capability(&self) -> bool {
        self.granted_capabilities
            .iter()
            .any(GrantedCapability::is_undeclared_exercise)
    }

    /// Whether the family is quarantined under anti-abuse review.
    pub fn is_quarantined(&self) -> bool {
        self.anti_abuse.is_quarantined() || self.load_state.is_quarantine_held()
    }

    /// Whether this card requires a last-known-good state to remain visible.
    pub fn requires_last_known_good(&self) -> bool {
        self.load_state.requires_last_known_good()
    }

    /// Whether a last-known-good state is present on the card.
    pub fn last_known_good_visible(&self) -> bool {
        self.last_known_good.is_some()
    }

    /// The rendered trust tier recomputed from this card's facts.
    ///
    /// The rendered tier is the weaker of the claimed tier and the signing-state
    /// ceiling, so an unsigned local-dev build, an unsigned side-load, or a revoked
    /// signature can never inherit a trusted-publisher badge.
    pub fn computed_rendered_trust_tier(&self) -> TrustPosture {
        self.claimed_trust_tier
            .min(self.signature_state.trust_ceiling())
    }

    /// The review triggers recomputed from this card's facts, in canonical order.
    pub fn computed_review_triggers(&self) -> Vec<InspectorReviewTrigger> {
        let mut triggers = Vec::new();
        match self.hot_reload_posture {
            HotReloadPosture::RuntimeClassWidenedPendingReview => {
                triggers.push(InspectorReviewTrigger::RuntimeClassWidened);
            }
            HotReloadPosture::PermissionsWidenedPendingReview => {
                triggers.push(InspectorReviewTrigger::PermissionsWidened);
            }
            HotReloadPosture::ExternalExecutableAddedPendingReview => {
                triggers.push(InspectorReviewTrigger::ExternalExecutableAdded);
            }
            HotReloadPosture::NoWidening | HotReloadPosture::RelaunchOnly => {}
        }
        if self.has_undeclared_capability() {
            triggers.push(InspectorReviewTrigger::UndeclaredCapabilityExercised);
        }
        triggers
    }

    /// The disposition recomputed from this card's load state, triggers, and quarantine.
    pub fn computed_disposition(&self) -> InspectorDisposition {
        let mut disposition = match self.load_state {
            LoadState::LoadedHealthy => {
                if self.has_recent_failures() {
                    InspectorDisposition::RunningDegraded
                } else {
                    InspectorDisposition::RunningHealthy
                }
            }
            LoadState::LoadedDegraded => InspectorDisposition::RunningDegraded,
            LoadState::LoadFailed | LoadState::SourceMissing => {
                InspectorDisposition::ShowingLastKnownGood
            }
            LoadState::OperatorDisabled => InspectorDisposition::OperatorDisabled,
            LoadState::QuarantineHeld => InspectorDisposition::Quarantined,
        };
        // A widening hot reload or an undeclared exercised capability forces a fresh
        // review rather than a silent hot reload.
        if !self.computed_review_triggers().is_empty() {
            disposition = disposition.widen(InspectorDisposition::FreshReviewRequired);
        }
        // An anti-abuse quarantine is the strongest hold regardless of load state.
        if self.anti_abuse.is_quarantined() {
            disposition = disposition.widen(InspectorDisposition::Quarantined);
        }
        disposition
    }

    /// Whether the stored trust tier, triggers, and disposition agree with the
    /// recomputed values.
    pub fn gate_consistent(&self) -> bool {
        self.rendered_trust_tier == self.computed_rendered_trust_tier()
            && self.review_triggers == self.computed_review_triggers()
            && self.disposition == self.computed_disposition()
    }

    /// Whether the last-known-good state respects the family's current trust cap.
    ///
    /// A stale good state can never render a stronger badge than the package may
    /// currently carry.
    pub fn last_known_good_within_cap(&self) -> bool {
        match &self.last_known_good {
            Some(good) => {
                good.rendered_trust_tier.rank() <= self.computed_rendered_trust_tier().rank()
            }
            None => true,
        }
    }

    /// Whether the card offers an action of the given kind.
    pub fn offers_action(&self, kind: InspectorActionKind) -> bool {
        self.actions.iter().any(|a| a.action_kind == kind)
    }

    /// Whether the card offers an *enabled* action of the given kind.
    pub fn offers_enabled_action(&self, kind: InspectorActionKind) -> bool {
        self.actions
            .iter()
            .any(|a| a.action_kind == kind && a.enabled)
    }

    /// Whether the inspected runtime is currently running.
    pub fn is_running(&self) -> bool {
        self.load_state.is_running()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RuntimeInspectorSummary {
    /// Total inspector cards.
    pub total_cards: usize,
    /// Cards running healthy.
    pub running_healthy_cards: usize,
    /// Cards running degraded.
    pub running_degraded_cards: usize,
    /// Cards showing last-known-good state.
    pub showing_last_known_good_cards: usize,
    /// Cards requiring a fresh review.
    pub fresh_review_required_cards: usize,
    /// Cards disabled by an operator.
    pub operator_disabled_cards: usize,
    /// Cards held under an anti-abuse quarantine.
    pub quarantined_cards: usize,
    /// Cards with one or more recent failures.
    pub cards_with_failures: usize,
    /// Cards carrying a last-known-good state.
    pub cards_with_last_known_good: usize,
    /// Cards that render a local-only trust badge.
    pub local_only_trust_cards: usize,
    /// Cards with an undeclared exercised capability.
    pub undeclared_capability_cards: usize,
    /// Cards over their memory budget.
    pub over_memory_budget_cards: usize,
    /// Distinct package kinds across cards.
    pub distinct_package_kinds: usize,
}

/// A redaction-safe export row for one inspector card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeInspectorExportRow {
    /// Card id.
    pub card_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Runtime-class token.
    pub runtime_class: String,
    /// Current-host token.
    pub current_host: String,
    /// Load-state token.
    pub load_state: String,
    /// Activation time in milliseconds.
    pub activation_millis: u32,
    /// Memory-pressure token.
    pub memory_pressure: String,
    /// Signature-state token.
    pub signature_state: String,
    /// Rendered-trust-tier token.
    pub rendered_trust_tier: String,
    /// Number of granted capabilities.
    pub granted_capability_count: usize,
    /// Number of recent failures.
    pub recent_failure_count: usize,
    /// Disposition token.
    pub disposition: String,
    /// Review-trigger tokens.
    pub review_triggers: Vec<String>,
    /// Whether a last-known-good state is visible.
    pub last_known_good_visible: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeInspectorExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5RuntimeInspectorExportRow>,
    /// Whether every card's gate is consistent with its recomputation.
    pub all_gates_consistent: bool,
    /// Cards requiring a fresh review.
    pub fresh_review_required_count: usize,
    /// Cards held (disabled or quarantined).
    pub held_count: usize,
}

/// The typed M5 runtime-inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RuntimeInspector {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed source-class vocabulary (reused from the marketplace fact-views).
    pub source_classes: Vec<SourceClass>,
    /// Closed runtime-class vocabulary (reused from the publish-preview gate).
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary (reused from the publish-preview gate).
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed signature-state vocabulary (reused from the publish-preview gate).
    pub signature_states: Vec<SignatureState>,
    /// Closed trust-posture vocabulary (reused from the publish-preview gate).
    pub trust_postures: Vec<TrustPosture>,
    /// Closed anti-abuse vocabulary (reused from the publish-preview gate).
    pub anti_abuse_states: Vec<AntiAbuseTransparency>,
    /// Closed hot-reload-posture vocabulary (reused from the publish-preview gate).
    pub hot_reload_postures: Vec<HotReloadPosture>,
    /// Closed activation-bucket vocabulary (reused from the activation-budget lane).
    pub activation_buckets: Vec<ActivationBucket>,
    /// Closed resource-pressure vocabulary (reused from the activation-budget lane).
    pub resource_pressures: Vec<ResourcePressure>,
    /// Closed capability-class vocabulary (reused from the activation-budget lane).
    pub capability_classes: Vec<CapabilityClass>,
    /// Closed capability-exercise-state vocabulary (reused from the activation lane).
    pub capability_exercise_states: Vec<CapabilityExerciseState>,
    /// Closed load-state vocabulary.
    pub load_states: Vec<LoadState>,
    /// Closed log-level vocabulary.
    pub log_levels: Vec<LogLevel>,
    /// Closed failure-class vocabulary.
    pub failure_classes: Vec<FailureClass>,
    /// Closed action-kind vocabulary.
    pub action_kinds: Vec<InspectorActionKind>,
    /// Closed review-trigger vocabulary.
    pub review_triggers: Vec<InspectorReviewTrigger>,
    /// Closed disposition vocabulary.
    pub dispositions: Vec<InspectorDisposition>,
    /// The inspector cards.
    #[serde(default)]
    pub inspector_cards: Vec<RuntimeInspectorCard>,
    /// Summary counts.
    pub summary: M5RuntimeInspectorSummary,
}

impl M5RuntimeInspector {
    /// Returns the inspector card with the given id.
    pub fn inspector_card(&self, card_id: &str) -> Option<&RuntimeInspectorCard> {
        self.inspector_cards.iter().find(|c| c.card_id == card_id)
    }

    /// Inspector cards that are held (disabled or quarantined) or need a fresh review.
    pub fn cards_needing_attention(&self) -> impl Iterator<Item = &RuntimeInspectorCard> {
        self.inspector_cards.iter().filter(|c| {
            c.disposition.is_held()
                || c.disposition == InspectorDisposition::FreshReviewRequired
                || c.disposition == InspectorDisposition::ShowingLastKnownGood
        })
    }

    /// Whether every card's stored gate agrees with its recomputation.
    pub fn all_gates_consistent(&self) -> bool {
        self.inspector_cards
            .iter()
            .all(RuntimeInspectorCard::gate_consistent)
    }

    /// Whether every failing/missing-source card keeps a last-known-good state visible.
    pub fn all_required_last_known_good_present(&self) -> bool {
        self.inspector_cards
            .iter()
            .all(|c| !c.requires_last_known_good() || c.last_known_good_visible())
    }

    /// Recomputes the summary block from the inspector cards.
    pub fn computed_summary(&self) -> M5RuntimeInspectorSummary {
        let count_disposition = |disposition: InspectorDisposition| {
            self.inspector_cards
                .iter()
                .filter(|c| c.disposition == disposition)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> = self
            .inspector_cards
            .iter()
            .map(|c| c.package_kind)
            .collect();
        M5RuntimeInspectorSummary {
            total_cards: self.inspector_cards.len(),
            running_healthy_cards: count_disposition(InspectorDisposition::RunningHealthy),
            running_degraded_cards: count_disposition(InspectorDisposition::RunningDegraded),
            showing_last_known_good_cards: count_disposition(
                InspectorDisposition::ShowingLastKnownGood,
            ),
            fresh_review_required_cards: count_disposition(
                InspectorDisposition::FreshReviewRequired,
            ),
            operator_disabled_cards: count_disposition(InspectorDisposition::OperatorDisabled),
            quarantined_cards: count_disposition(InspectorDisposition::Quarantined),
            cards_with_failures: self
                .inspector_cards
                .iter()
                .filter(|c| c.has_recent_failures())
                .count(),
            cards_with_last_known_good: self
                .inspector_cards
                .iter()
                .filter(|c| c.last_known_good_visible())
                .count(),
            local_only_trust_cards: self
                .inspector_cards
                .iter()
                .filter(|c| c.rendered_trust_tier == TrustPosture::UnsignedLocalOnly)
                .count(),
            undeclared_capability_cards: self
                .inspector_cards
                .iter()
                .filter(|c| c.has_undeclared_capability())
                .count(),
            over_memory_budget_cards: self
                .inspector_cards
                .iter()
                .filter(|c| c.activation.is_over_memory_budget())
                .count(),
            distinct_package_kinds: package_kinds.len(),
        }
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// service-health, docs/help, and release/public-truth packets — render instead of
    /// restating runtime, trust, and disposition text by hand.
    pub fn export_projection(&self) -> M5RuntimeInspectorExportProjection {
        let rows = self
            .inspector_cards
            .iter()
            .map(|c| M5RuntimeInspectorExportRow {
                card_id: c.card_id.clone(),
                package_kind: c.package_kind.as_str().to_owned(),
                runtime_class: c.runtime_class.as_str().to_owned(),
                current_host: c.current_host.as_str().to_owned(),
                load_state: c.load_state.as_str().to_owned(),
                activation_millis: c.activation.activation_millis,
                memory_pressure: c.activation.memory_pressure.as_str().to_owned(),
                signature_state: c.signature_state.as_str().to_owned(),
                rendered_trust_tier: c.rendered_trust_tier.as_str().to_owned(),
                granted_capability_count: c.granted_capabilities.len(),
                recent_failure_count: c.recent_failures.len(),
                disposition: c.disposition.as_str().to_owned(),
                review_triggers: c
                    .review_triggers
                    .iter()
                    .map(|t| t.as_str().to_owned())
                    .collect(),
                last_known_good_visible: c.last_known_good_visible(),
                summary: format!(
                    "{}: {} on {}, activation {}ms, renders {}, {} failures, disposition {}",
                    c.package_kind.as_str(),
                    c.load_state.as_str(),
                    c.current_host.as_str(),
                    c.activation.activation_millis,
                    c.rendered_trust_tier.as_str(),
                    c.recent_failures.len(),
                    c.disposition.as_str(),
                ),
            })
            .collect();
        M5RuntimeInspectorExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_gates_consistent: self.all_gates_consistent(),
            fresh_review_required_count: self
                .inspector_cards
                .iter()
                .filter(|c| c.disposition == InspectorDisposition::FreshReviewRequired)
                .count(),
            held_count: self
                .inspector_cards
                .iter()
                .filter(|c| c.disposition.is_held())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5RuntimeInspectorViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_cards = BTreeSet::new();
        for card in &self.inspector_cards {
            if !seen_cards.insert(card.card_id.clone()) {
                violations.push(M5RuntimeInspectorViolation::DuplicateCardId {
                    card_id: card.card_id.clone(),
                });
            }
            self.validate_card(card, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5RuntimeInspectorViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5RuntimeInspectorViolation>) {
        if self.schema_version != M5_RUNTIME_INSPECTOR_SCHEMA_VERSION {
            violations.push(M5RuntimeInspectorViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_RUNTIME_INSPECTOR_RECORD_KIND {
            violations.push(M5RuntimeInspectorViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RuntimeInspectorViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "host_abi_classes",
                self.host_abi_classes == HostAbiClass::ALL.to_vec(),
            ),
            (
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "anti_abuse_states",
                self.anti_abuse_states == AntiAbuseTransparency::ALL.to_vec(),
            ),
            (
                "hot_reload_postures",
                self.hot_reload_postures == HotReloadPosture::ALL.to_vec(),
            ),
            (
                "activation_buckets",
                self.activation_buckets == ActivationBucket::ALL.to_vec(),
            ),
            (
                "resource_pressures",
                self.resource_pressures == ResourcePressure::ALL.to_vec(),
            ),
            (
                "capability_classes",
                self.capability_classes == CapabilityClass::ALL.to_vec(),
            ),
            (
                "capability_exercise_states",
                self.capability_exercise_states == CapabilityExerciseState::ALL.to_vec(),
            ),
            ("load_states", self.load_states == LoadState::ALL.to_vec()),
            ("log_levels", self.log_levels == LogLevel::ALL.to_vec()),
            (
                "failure_classes",
                self.failure_classes == FailureClass::ALL.to_vec(),
            ),
            (
                "action_kinds",
                self.action_kinds == InspectorActionKind::ALL.to_vec(),
            ),
            (
                "review_triggers",
                self.review_triggers == InspectorReviewTrigger::ALL.to_vec(),
            ),
            (
                "dispositions",
                self.dispositions == InspectorDisposition::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5RuntimeInspectorViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_card(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        for (field, value) in [
            ("card_id", &card.card_id),
            ("listing_ref", &card.listing_ref),
            ("display_label", &card.display_label),
            ("governance_family_ref", &card.governance_family_ref),
            ("extension_identity", &card.extension_identity),
            ("extension_version", &card.extension_version),
            ("summary", &card.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RuntimeInspectorViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: field,
                });
            }
        }

        // The extension identity must be the namespaced publisher/extension form.
        if !card.extension_identity.trim().is_empty() && !card.extension_identity.contains('/') {
            violations.push(M5RuntimeInspectorViolation::MalformedExtensionIdentity {
                card_id: card.card_id.clone(),
                identity: card.extension_identity.clone(),
            });
        }

        self.validate_signature(card, violations);
        self.validate_capabilities(card, violations);
        self.validate_logs(card, violations);
        self.validate_failures(card, violations);
        self.validate_last_known_good(card, violations);
        self.validate_actions(card, violations);
        self.validate_gate(card, violations);
    }

    fn validate_signature(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        // A signed or revoked artifact must name its signature record; an unsigned one
        // must not, so the signing state is never overstated or hidden.
        let expects_ref = matches!(
            card.signature_state,
            SignatureState::SignedVerified
                | SignatureState::SignedUnverified
                | SignatureState::RevokedSignature
        );
        if card.signature_ref.is_some() != expects_ref {
            violations.push(M5RuntimeInspectorViolation::SignatureRefInconsistent {
                card_id: card.card_id.clone(),
            });
        }
    }

    fn validate_capabilities(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        for capability in &card.granted_capabilities {
            for (field, value) in [
                ("capability.target_label", &capability.target_label),
                ("capability.rationale", &capability.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RuntimeInspectorViolation::EmptyField {
                        id: card.card_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_logs(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        for entry in &card.recent_logs {
            for (field, value) in [
                ("log.code", &entry.code),
                ("log.message_label", &entry.message_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RuntimeInspectorViolation::EmptyField {
                        id: card.card_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_failures(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        for failure in &card.recent_failures {
            for (field, value) in [
                ("failure.last_seen_label", &failure.last_seen_label),
                ("failure.detail_label", &failure.detail_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RuntimeInspectorViolation::EmptyField {
                        id: card.card_id.clone(),
                        field_name: field,
                    });
                }
            }
            if failure.occurrence_count == 0 {
                violations.push(M5RuntimeInspectorViolation::ZeroOccurrenceFailure {
                    card_id: card.card_id.clone(),
                });
            }
        }
    }

    fn validate_last_known_good(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        // A failed current load or a disappeared source must keep a last-known-good
        // state visible, so the inspector never collapses to an empty surface.
        if card.requires_last_known_good() && !card.last_known_good_visible() {
            violations.push(M5RuntimeInspectorViolation::MissingLastKnownGood {
                card_id: card.card_id.clone(),
            });
        }

        if let Some(good) = &card.last_known_good {
            for (field, value) in [
                ("last_known_good.revision_ref", &good.revision_ref),
                ("last_known_good.captured_label", &good.captured_label),
                ("last_known_good.summary", &good.summary),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RuntimeInspectorViolation::EmptyField {
                        id: card.card_id.clone(),
                        field_name: field,
                    });
                }
            }
            // A stale good state can never render a stronger badge than the family's
            // current cap, so trust history is never overstated.
            if !card.last_known_good_within_cap() {
                violations.push(M5RuntimeInspectorViolation::LastKnownGoodTrustExceedsCap {
                    card_id: card.card_id.clone(),
                    stored: good.rendered_trust_tier.as_str(),
                    cap: card.computed_rendered_trust_tier().as_str(),
                });
            }
        }
    }

    fn validate_actions(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        for action in &card.actions {
            if action.action_ref.trim().is_empty() {
                violations.push(M5RuntimeInspectorViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: "action_ref",
                });
            }
        }

        // The inspector always exposes its logs, so crash history and granted
        // capabilities are never hidden from a local authoring flow.
        if !card.offers_action(InspectorActionKind::ViewLogs) {
            violations.push(M5RuntimeInspectorViolation::MissingRequiredAction {
                card_id: card.card_id.clone(),
                action: InspectorActionKind::ViewLogs.as_str(),
            });
        }

        // A disabled or quarantined card must offer a re-enable path; it routes through
        // review rather than implying a risk-free toggle.
        if card.disposition.is_held() && !card.offers_action(InspectorActionKind::ReEnable) {
            violations.push(M5RuntimeInspectorViolation::MissingRequiredAction {
                card_id: card.card_id.clone(),
                action: InspectorActionKind::ReEnable.as_str(),
            });
        }

        // A fresh-review-required card must offer the request-fresh-review action and
        // must not expose an enabled restart or reload that would apply the widening
        // through a silent hot reload.
        if card.disposition == InspectorDisposition::FreshReviewRequired {
            if !card.offers_action(InspectorActionKind::RequestFreshReview) {
                violations.push(M5RuntimeInspectorViolation::MissingRequiredAction {
                    card_id: card.card_id.clone(),
                    action: InspectorActionKind::RequestFreshReview.as_str(),
                });
            }
            for action in &card.actions {
                if action.action_kind.applies_runtime_change() && action.enabled {
                    violations.push(M5RuntimeInspectorViolation::WideningAppliedWithoutReview {
                        card_id: card.card_id.clone(),
                        action: action.action_kind.as_str(),
                    });
                }
            }
        }
    }

    fn validate_gate(
        &self,
        card: &RuntimeInspectorCard,
        violations: &mut Vec<M5RuntimeInspectorViolation>,
    ) {
        let mut seen_triggers = BTreeSet::new();
        for trigger in &card.review_triggers {
            if !seen_triggers.insert(*trigger) {
                violations.push(M5RuntimeInspectorViolation::DuplicateReviewTrigger {
                    card_id: card.card_id.clone(),
                    trigger: trigger.as_str(),
                });
            }
        }

        // The rendered trust tier must equal the recomputed cap, so an unsigned local
        // or side-loaded artifact can never assert a stronger badge than its signing
        // state allows.
        let computed_tier = card.computed_rendered_trust_tier();
        if card.rendered_trust_tier != computed_tier {
            violations.push(M5RuntimeInspectorViolation::RenderedTrustMismatch {
                card_id: card.card_id.clone(),
                stored: card.rendered_trust_tier.as_str(),
                computed: computed_tier.as_str(),
            });
        }

        // A local-dev, side-loaded, or revoked artifact must never render a
        // trusted-publisher badge just because the machine holds a trusted key.
        if card.signature_state.is_local_or_untrusted()
            && card.rendered_trust_tier.is_trusted_badge()
        {
            violations.push(
                M5RuntimeInspectorViolation::LocalPackageInheritsTrustedBadge {
                    card_id: card.card_id.clone(),
                    tier: card.rendered_trust_tier.as_str(),
                },
            );
        }

        // The recorded triggers must equal the recomputed set, so a widening or
        // undeclared capability use can never be asserted or hidden by hand.
        if card.review_triggers != card.computed_review_triggers() {
            violations.push(M5RuntimeInspectorViolation::ReviewTriggersMismatch {
                card_id: card.card_id.clone(),
            });
        }

        // The published disposition must equal the recomputed gate.
        let computed_disposition = card.computed_disposition();
        if card.disposition != computed_disposition {
            violations.push(M5RuntimeInspectorViolation::DispositionMismatch {
                card_id: card.card_id.clone(),
                stored: card.disposition.as_str(),
                computed: computed_disposition.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 runtime-inspector packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RuntimeInspectorViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary field is not the canonical value.
    ClosedVocabularyMismatch {
        /// Field name.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Owning id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// Two inspector cards share an id.
    DuplicateCardId {
        /// Duplicated card id.
        card_id: String,
    },
    /// An extension identity is not in the namespaced publisher/extension form.
    MalformedExtensionIdentity {
        /// Owning card id.
        card_id: String,
        /// The malformed identity.
        identity: String,
    },
    /// The signature ref disagrees with the signing state.
    SignatureRefInconsistent {
        /// Owning card id.
        card_id: String,
    },
    /// A recent failure records a zero occurrence count.
    ZeroOccurrenceFailure {
        /// Owning card id.
        card_id: String,
    },
    /// A failing or missing-source card is missing its last-known-good state.
    MissingLastKnownGood {
        /// Owning card id.
        card_id: String,
    },
    /// A last-known-good state renders a stronger badge than the current cap.
    LastKnownGoodTrustExceedsCap {
        /// Owning card id.
        card_id: String,
        /// Stored last-known-good tier.
        stored: &'static str,
        /// The recomputed current cap.
        cap: &'static str,
    },
    /// A required action is absent.
    MissingRequiredAction {
        /// Owning card id.
        card_id: String,
        /// The missing action.
        action: &'static str,
    },
    /// A fresh-review-required card exposes an enabled restart or reload action.
    WideningAppliedWithoutReview {
        /// Owning card id.
        card_id: String,
        /// The offending action.
        action: &'static str,
    },
    /// A review trigger is repeated on one card.
    DuplicateReviewTrigger {
        /// Owning card id.
        card_id: String,
        /// The repeated trigger.
        trigger: &'static str,
    },
    /// The rendered trust tier disagrees with the recomputed cap.
    RenderedTrustMismatch {
        /// Owning card id.
        card_id: String,
        /// Stored value.
        stored: &'static str,
        /// Recomputed value.
        computed: &'static str,
    },
    /// A local or untrusted artifact renders a trusted-publisher badge.
    LocalPackageInheritsTrustedBadge {
        /// Owning card id.
        card_id: String,
        /// The rendered tier.
        tier: &'static str,
    },
    /// The stored review triggers disagree with the recomputed set.
    ReviewTriggersMismatch {
        /// Owning card id.
        card_id: String,
    },
    /// The stored disposition disagrees with the recomputed gate.
    DispositionMismatch {
        /// Owning card id.
        card_id: String,
        /// Stored value.
        stored: &'static str,
        /// Recomputed value.
        computed: &'static str,
    },
    /// The packet summary counts disagree with the inspector cards.
    SummaryMismatch,
}

impl fmt::Display for M5RuntimeInspectorViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateCardId { card_id } => {
                write!(f, "duplicate inspector card id {card_id}")
            }
            Self::MalformedExtensionIdentity { card_id, identity } => {
                write!(
                    f,
                    "card {card_id} extension identity {identity} is not publisher/extension form"
                )
            }
            Self::SignatureRefInconsistent { card_id } => {
                write!(
                    f,
                    "card {card_id} signature ref disagrees with its signing state"
                )
            }
            Self::ZeroOccurrenceFailure { card_id } => {
                write!(f, "card {card_id} records a failure with zero occurrences")
            }
            Self::MissingLastKnownGood { card_id } => {
                write!(
                    f,
                    "card {card_id} failed to load or lost its source but carries no last-known-good state"
                )
            }
            Self::LastKnownGoodTrustExceedsCap {
                card_id,
                stored,
                cap,
            } => {
                write!(
                    f,
                    "card {card_id} last-known-good renders {stored} above the current cap {cap}"
                )
            }
            Self::MissingRequiredAction { card_id, action } => {
                write!(f, "card {card_id} is missing required action {action}")
            }
            Self::WideningAppliedWithoutReview { card_id, action } => {
                write!(
                    f,
                    "card {card_id} requires a fresh review but exposes an enabled {action} action"
                )
            }
            Self::DuplicateReviewTrigger { card_id, trigger } => {
                write!(f, "card {card_id} repeats review trigger {trigger}")
            }
            Self::RenderedTrustMismatch {
                card_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "card {card_id} renders trust tier {stored} but the recomputed cap is {computed}"
                )
            }
            Self::LocalPackageInheritsTrustedBadge { card_id, tier } => {
                write!(
                    f,
                    "card {card_id} renders trusted-publisher badge {tier} for a local or untrusted artifact"
                )
            }
            Self::ReviewTriggersMismatch { card_id } => {
                write!(
                    f,
                    "card {card_id} review triggers disagree with the recomputed set"
                )
            }
            Self::DispositionMismatch {
                card_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "card {card_id} publishes disposition {stored} but the recomputed gate is {computed}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the inspector cards")
            }
        }
    }
}

impl Error for M5RuntimeInspectorViolation {}

/// Loads the embedded M5 runtime-inspector packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5RuntimeInspector`].
pub fn current_m5_runtime_inspector() -> Result<M5RuntimeInspector, serde_json::Error> {
    serde_json::from_str(M5_RUNTIME_INSPECTOR_JSON)
}

#[cfg(test)]
mod tests;
