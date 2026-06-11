//! Canonical M5 activation-budget and exercised-capability records — runtime
//! cost and *actually used* capability modeled as first-class, reviewable truth
//! for the marketed M5 artifact families.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per family (including the family's published
//! activation-budget band), the [`marketplace fact-views`](crate::m5_marketplace_fact_views)
//! project that truth into the storefront, the [`install-review sheets`](crate::m5_install_review)
//! freeze how an install or update is reviewed before commit, and the
//! [`lifecycle actions`](crate::m5_lifecycle_actions) freeze what happens to a
//! package after install, this module freezes the *operational* dimension: per
//! session, what a package actually cost to activate and which of its declared
//! capabilities it actually exercised.
//!
//! Each [`ActivationBudgetRecord`] makes three things explicit that a generic
//! "extension is slow" toast hides:
//!
//! - **activation cost** — the [`ActivationBucket`] (cold/warm), the
//!   [`ActivationTrigger`] (lazy vs eager), the published
//!   [`ActivationBudgetBand`], cold-start and memory [`ResourcePressure`], the
//!   [`RestartBudgetState`], and the [`HostClass`] the runtime is bound to, so a
//!   reviewer can see operational cost alongside permissions and features;
//! - **declared vs exercised capability** — a per-[`CapabilityClass`]
//!   [`CapabilityExerciseState`] that separates a capability that was declared and
//!   exercised, declared but never used (an over-grant candidate), or — a policy
//!   violation — exercised without ever being declared; and
//! - **enforcement** — when a session exceeds an activation, cold-start, memory,
//!   restart, crash-loop, or undeclared-capability rule, the exact
//!   [`EnforcementReason`] codes and the [`EnforcementAction`] taken (throttle,
//!   downgrade, pause, or quarantine) with a recovery path, instead of a generic
//!   performance warning.
//!
//! The record is honest by construction. Its [`EnforcementReason`] set and the
//! [`EnforcementAction`] it publishes are **not** stored by hand: they are
//! recomputed from the record's facts, and the stored values must equal that
//! recomputation or validation fails. An over-budget activation throttles, memory
//! pressure downgrades, an exhausted restart budget pauses, and a crash loop or an
//! undeclared exercised capability quarantines; a healthy session with every
//! declared capability accounted for takes [`EnforcementAction::NoAction`]. Any
//! non-[`EnforcementAction::NoAction`] record must carry a recovery path, so a
//! throttle, downgrade, pause, or quarantine always names a way back.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-activation-budget.json`
//! and embedded here, so this typed consumer and any CI gate agree on every record
//! without a cargo build in CI. The model is metadata-only: every field is a typed
//! state, a count, or an opaque ref. It carries no credential bodies, raw provider
//! payloads, signing secrets, or registry tokens.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ActivationBudgetBand, ArtifactFamily, RuntimeOrigin,
};
use crate::m5_install_review::{HostClass, InstallScope};

/// Supported M5 activation-budget schema version.
pub const M5_ACTIVATION_BUDGET_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ACTIVATION_BUDGET_RECORD_KIND: &str = "m5_activation_budget";

/// Repo-relative path to the checked-in packet.
pub const M5_ACTIVATION_BUDGET_PATH: &str = "artifacts/ecosystem/m5/m5-activation-budget.json";

/// Embedded checked-in packet JSON.
pub const M5_ACTIVATION_BUDGET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-activation-budget.json"
));

/// Whether a session activated the runtime from cold or reused a warm runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationBucket {
    /// A cold activation that paid full start-up cost.
    Cold,
    /// A warm activation that reused an already-running runtime.
    Warm,
}

impl ActivationBucket {
    /// Every activation bucket, in declaration order.
    pub const ALL: [Self; 2] = [Self::Cold, Self::Warm];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cold => "cold",
            Self::Warm => "warm",
        }
    }

    /// Whether this is a cold activation.
    pub const fn is_cold(self) -> bool {
        matches!(self, Self::Cold)
    }
}

/// What triggered a package to activate.
///
/// A lazy trigger defers activation until the package is genuinely needed; an
/// eager trigger activates the package up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationTrigger {
    /// Eagerly activated at startup.
    EagerOnStartup,
    /// Lazily activated when a workspace opens.
    OnWorkspaceOpen,
    /// Lazily activated when a matching language or file type is seen.
    OnLanguageMatch,
    /// Lazily activated when one of its commands is invoked.
    OnCommandInvoke,
    /// Lazily activated when one of its views is opened.
    OnViewOpen,
    /// Activated only on explicit manual request.
    Manual,
}

impl ActivationTrigger {
    /// Every activation trigger, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EagerOnStartup,
        Self::OnWorkspaceOpen,
        Self::OnLanguageMatch,
        Self::OnCommandInvoke,
        Self::OnViewOpen,
        Self::Manual,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EagerOnStartup => "eager_on_startup",
            Self::OnWorkspaceOpen => "on_workspace_open",
            Self::OnLanguageMatch => "on_language_match",
            Self::OnCommandInvoke => "on_command_invoke",
            Self::OnViewOpen => "on_view_open",
            Self::Manual => "manual",
        }
    }

    /// Whether this trigger defers activation until the package is needed.
    pub const fn is_lazy(self) -> bool {
        !matches!(self, Self::EagerOnStartup)
    }
}

/// Pressure on a measured runtime resource relative to its budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourcePressure {
    /// Comfortably within budget.
    Healthy,
    /// Approaching the budget ceiling.
    Elevated,
    /// Over budget.
    OverBudget,
    /// Pressure could not be established.
    Unknown,
    /// No budget applies to this resource for this session.
    NotApplicable,
}

impl ResourcePressure {
    /// Every resource-pressure state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Healthy,
        Self::Elevated,
        Self::OverBudget,
        Self::Unknown,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Elevated => "elevated",
            Self::OverBudget => "over_budget",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this resource is over budget.
    pub const fn is_over_budget(self) -> bool {
        matches!(self, Self::OverBudget)
    }
}

/// A class of capability a package may declare and exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClass {
    /// Read from the filesystem.
    FilesystemRead,
    /// Write to the filesystem.
    FilesystemWrite,
    /// Make network requests.
    NetworkAccess,
    /// Spawn child processes.
    ProcessSpawn,
    /// Read or write the clipboard.
    ClipboardAccess,
    /// Read stored secrets.
    SecretsRead,
    /// Emit telemetry.
    TelemetryEmit,
    /// Run model inference.
    ModelInference,
    /// Write workspace settings.
    WorkspaceSettingsWrite,
    /// Run background execution.
    BackgroundExecution,
}

impl CapabilityClass {
    /// Every capability class, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::FilesystemRead,
        Self::FilesystemWrite,
        Self::NetworkAccess,
        Self::ProcessSpawn,
        Self::ClipboardAccess,
        Self::SecretsRead,
        Self::TelemetryEmit,
        Self::ModelInference,
        Self::WorkspaceSettingsWrite,
        Self::BackgroundExecution,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilesystemRead => "filesystem_read",
            Self::FilesystemWrite => "filesystem_write",
            Self::NetworkAccess => "network_access",
            Self::ProcessSpawn => "process_spawn",
            Self::ClipboardAccess => "clipboard_access",
            Self::SecretsRead => "secrets_read",
            Self::TelemetryEmit => "telemetry_emit",
            Self::ModelInference => "model_inference",
            Self::WorkspaceSettingsWrite => "workspace_settings_write",
            Self::BackgroundExecution => "background_execution",
        }
    }
}

/// How a capability's declared grant relates to its exercised use this session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityExerciseState {
    /// Declared and exercised this session.
    DeclaredExercised,
    /// Declared but never exercised this session — an over-grant candidate.
    DeclaredUnused,
    /// Exercised without being declared — a policy violation.
    UndeclaredExercised,
}

impl CapabilityExerciseState {
    /// Every exercise state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DeclaredExercised,
        Self::DeclaredUnused,
        Self::UndeclaredExercised,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredExercised => "declared_exercised",
            Self::DeclaredUnused => "declared_unused",
            Self::UndeclaredExercised => "undeclared_exercised",
        }
    }

    /// Whether the capability was part of the declared manifest.
    pub const fn is_declared(self) -> bool {
        matches!(self, Self::DeclaredExercised | Self::DeclaredUnused)
    }

    /// Whether the capability was actually exercised this session.
    pub const fn is_exercised(self) -> bool {
        matches!(self, Self::DeclaredExercised | Self::UndeclaredExercised)
    }

    /// Whether this is an undeclared but exercised capability — a policy violation.
    pub const fn is_undeclared_exercise(self) -> bool {
        matches!(self, Self::UndeclaredExercised)
    }

    /// Whether this is a declared but unused capability — an over-grant candidate.
    pub const fn is_unused_grant(self) -> bool {
        matches!(self, Self::DeclaredUnused)
    }
}

/// A reason a session escalated above a no-action disposition.
///
/// Each reason is recomputed from the record's facts; the record's stored
/// [`ActivationBudgetRecord::enforcement_reasons`] must equal the recomputed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementReason {
    /// The session's activation budget band is over budget.
    ActivationBudgetExceeded,
    /// The session's cold-start cost is over budget.
    ColdStartBudgetExceeded,
    /// The session's memory use is over budget.
    MemoryBudgetExceeded,
    /// The session exhausted its restart budget.
    RestartBudgetExhausted,
    /// The runtime crash-looped.
    CrashLoopDetected,
    /// A capability was exercised that was never declared.
    UndeclaredCapabilityExercised,
}

impl EnforcementReason {
    /// Every enforcement reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActivationBudgetExceeded,
        Self::ColdStartBudgetExceeded,
        Self::MemoryBudgetExceeded,
        Self::RestartBudgetExhausted,
        Self::CrashLoopDetected,
        Self::UndeclaredCapabilityExercised,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivationBudgetExceeded => "activation_budget_exceeded",
            Self::ColdStartBudgetExceeded => "cold_start_budget_exceeded",
            Self::MemoryBudgetExceeded => "memory_budget_exceeded",
            Self::RestartBudgetExhausted => "restart_budget_exhausted",
            Self::CrashLoopDetected => "crash_loop_detected",
            Self::UndeclaredCapabilityExercised => "undeclared_capability_exercised",
        }
    }

    /// The minimum enforcement action this reason forces.
    pub const fn min_action(self) -> EnforcementAction {
        match self {
            // An over-budget activation or cold start is a soft throttle.
            Self::ActivationBudgetExceeded | Self::ColdStartBudgetExceeded => {
                EnforcementAction::Throttled
            }
            // Memory pressure downgrades the package to a reduced runtime mode.
            Self::MemoryBudgetExceeded => EnforcementAction::Downgraded,
            // An exhausted restart budget pauses the package pending review.
            Self::RestartBudgetExhausted => EnforcementAction::Paused,
            // A crash loop or an undeclared exercised capability is a hard stop:
            // the package is quarantined.
            Self::CrashLoopDetected | Self::UndeclaredCapabilityExercised => {
                EnforcementAction::Quarantined
            }
        }
    }
}

/// The enforcement action a session publishes.
///
/// Ordered low-to-high by [`EnforcementAction::rank`]: an
/// [`EnforcementAction::NoAction`] session runs unimpeded, and an
/// [`EnforcementAction::Quarantined`] session is held pending review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementAction {
    /// The session runs unimpeded; no reason applies.
    NoAction,
    /// The session is throttled to stay within budget.
    Throttled,
    /// The session is downgraded to a reduced runtime mode.
    Downgraded,
    /// The session is paused pending review.
    Paused,
    /// The session is quarantined pending review.
    Quarantined,
}

impl EnforcementAction {
    /// Every enforcement action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoAction,
        Self::Throttled,
        Self::Downgraded,
        Self::Paused,
        Self::Quarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAction => "no_action",
            Self::Throttled => "throttled",
            Self::Downgraded => "downgraded",
            Self::Paused => "paused",
            Self::Quarantined => "quarantined",
        }
    }

    /// Monotonic rank; higher means a stricter action.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NoAction => 0,
            Self::Throttled => 1,
            Self::Downgraded => 2,
            Self::Paused => 3,
            Self::Quarantined => 4,
        }
    }

    /// The stricter (higher-rank) of two actions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// The restart budget a session consumed within its window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestartBudgetState {
    /// Restarts permitted within the window.
    pub window_restarts_allowed: u32,
    /// Restarts consumed within the window.
    pub window_restarts_used: u32,
    /// Whether the runtime crash-looped within the window.
    pub crash_loop_detected: bool,
    /// Opaque ref to the restart-window note, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_ref: Option<String>,
}

impl RestartBudgetState {
    /// Whether the session exhausted its restart budget.
    ///
    /// A session exhausts its budget once it has used at least one restart and has
    /// reached or passed the permitted count.
    pub const fn budget_exhausted(&self) -> bool {
        self.window_restarts_used > 0 && self.window_restarts_used >= self.window_restarts_allowed
    }
}

/// One capability and how its declared grant related to its exercised use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityUsage {
    /// The class of capability.
    pub capability_class: CapabilityClass,
    /// How the declared grant related to the exercised use this session.
    pub exercise_state: CapabilityExerciseState,
    /// Opaque ref to the usage evidence for this capability.
    pub evidence_ref: String,
}

/// One activation-budget and exercised-capability record for a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivationBudgetRecord {
    /// Stable record id.
    pub record_id: String,
    /// Opaque ref to the catalog listing this session ran.
    pub listing_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this listing resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// The scope this session ran in.
    pub scope: InstallScope,
    /// Opaque ref to the session this record summarizes.
    pub session_ref: String,
    /// The host class the runtime is bound to.
    pub runtime_host_class: HostClass,
    /// The runtime origin of the package.
    pub runtime_origin: RuntimeOrigin,
    /// Whether the session activated cold or warm.
    pub activation_bucket: ActivationBucket,
    /// What triggered activation.
    pub activation_trigger: ActivationTrigger,
    /// The published activation-budget band.
    pub activation_budget_band: ActivationBudgetBand,
    /// Cold-start pressure for this session.
    pub cold_start_pressure: ResourcePressure,
    /// Memory pressure for this session.
    pub memory_pressure: ResourcePressure,
    /// The restart budget consumed this window.
    pub restart_budget: RestartBudgetState,
    /// The capability classes declared by the package's manifest.
    #[serde(default)]
    pub declared_capabilities: Vec<CapabilityClass>,
    /// Per-capability declared-vs-exercised usage.
    #[serde(default)]
    pub exercised_capabilities: Vec<CapabilityUsage>,
    /// Enforcement reasons; must equal the recomputed set.
    #[serde(default)]
    pub enforcement_reasons: Vec<EnforcementReason>,
    /// Enforcement action; must equal the recomputed value.
    pub enforcement_action: EnforcementAction,
    /// Opaque ref to the recovery path; required when an action is taken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_path_ref: Option<String>,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl ActivationBudgetRecord {
    /// Whether the session's activation budget band is over budget.
    pub const fn over_activation_budget(&self) -> bool {
        self.activation_budget_band.is_exceeded_trigger()
    }

    /// Whether the session's cold-start cost is over budget.
    pub const fn cold_start_over_budget(&self) -> bool {
        self.cold_start_pressure.is_over_budget()
    }

    /// Whether the session's memory use is over budget.
    pub const fn memory_over_budget(&self) -> bool {
        self.memory_pressure.is_over_budget()
    }

    /// Whether the session exhausted its restart budget.
    pub const fn restart_budget_exhausted(&self) -> bool {
        self.restart_budget.budget_exhausted()
    }

    /// Whether the runtime crash-looped this window.
    pub const fn crash_loop_detected(&self) -> bool {
        self.restart_budget.crash_loop_detected
    }

    /// Whether any capability was exercised without being declared.
    pub fn has_undeclared_exercised(&self) -> bool {
        self.exercised_capabilities
            .iter()
            .any(|usage| usage.exercise_state.is_undeclared_exercise())
    }

    /// The number of declared but unused capabilities — over-grant candidates.
    pub fn unused_declared_count(&self) -> usize {
        self.exercised_capabilities
            .iter()
            .filter(|usage| usage.exercise_state.is_unused_grant())
            .count()
    }

    /// The number of capabilities exercised without being declared.
    pub fn undeclared_exercised_count(&self) -> usize {
        self.exercised_capabilities
            .iter()
            .filter(|usage| usage.exercise_state.is_undeclared_exercise())
            .count()
    }

    /// The enforcement reasons recomputed from this record's facts, in canonical
    /// order.
    pub fn computed_enforcement_reasons(&self) -> Vec<EnforcementReason> {
        let mut reasons = Vec::new();
        if self.over_activation_budget() {
            reasons.push(EnforcementReason::ActivationBudgetExceeded);
        }
        if self.cold_start_over_budget() {
            reasons.push(EnforcementReason::ColdStartBudgetExceeded);
        }
        if self.memory_over_budget() {
            reasons.push(EnforcementReason::MemoryBudgetExceeded);
        }
        if self.restart_budget_exhausted() {
            reasons.push(EnforcementReason::RestartBudgetExhausted);
        }
        if self.crash_loop_detected() {
            reasons.push(EnforcementReason::CrashLoopDetected);
        }
        if self.has_undeclared_exercised() {
            reasons.push(EnforcementReason::UndeclaredCapabilityExercised);
        }
        reasons
    }

    /// The enforcement action recomputed from this record's reasons.
    pub fn computed_enforcement_action(&self) -> EnforcementAction {
        self.computed_enforcement_reasons()
            .into_iter()
            .fold(EnforcementAction::NoAction, |action, reason| {
                action.widen(reason.min_action())
            })
    }

    /// Whether the stored reasons and action agree with the recomputed values.
    pub fn gate_consistent(&self) -> bool {
        self.enforcement_reasons == self.computed_enforcement_reasons()
            && self.enforcement_action == self.computed_enforcement_action()
    }

    /// Whether the session ran unimpeded.
    pub fn runs_unimpeded(&self) -> bool {
        self.enforcement_action == EnforcementAction::NoAction
    }

    /// Whether the session required an enforcement intervention.
    pub fn requires_intervention(&self) -> bool {
        self.enforcement_action != EnforcementAction::NoAction
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ActivationBudgetSummary {
    /// Total session records.
    pub total_records: usize,
    /// Records that ran unimpeded.
    pub no_action_records: usize,
    /// Records that were throttled.
    pub throttled_records: usize,
    /// Records that were downgraded.
    pub downgraded_records: usize,
    /// Records that were paused.
    pub paused_records: usize,
    /// Records that were quarantined.
    pub quarantined_records: usize,
    /// Records that required any enforcement intervention.
    pub intervention_records: usize,
    /// Records over their activation budget.
    pub over_activation_budget_records: usize,
    /// Records that crash-looped.
    pub crash_loop_records: usize,
    /// Records that exercised an undeclared capability.
    pub undeclared_exercised_records: usize,
    /// Records carrying at least one declared-but-unused capability.
    pub unused_declared_grant_records: usize,
    /// Records that activated cold.
    pub cold_activation_records: usize,
    /// Records that activated warm.
    pub warm_activation_records: usize,
    /// Distinct package kinds across records.
    pub distinct_package_kinds: usize,
    /// Distinct host classes across records.
    pub distinct_host_classes: usize,
}

/// A redaction-safe export row projected from a session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationBudgetExportRow {
    /// Record id.
    pub record_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Scope token.
    pub scope: String,
    /// Runtime-host-class token.
    pub runtime_host_class: String,
    /// Runtime-origin token.
    pub runtime_origin: String,
    /// Activation-bucket token.
    pub activation_bucket: String,
    /// Activation-trigger token.
    pub activation_trigger: String,
    /// Activation-budget-band token.
    pub activation_budget_band: String,
    /// Cold-start-pressure token.
    pub cold_start_pressure: String,
    /// Memory-pressure token.
    pub memory_pressure: String,
    /// Restarts permitted within the window.
    pub window_restarts_allowed: u32,
    /// Restarts consumed within the window.
    pub window_restarts_used: u32,
    /// Whether the runtime crash-looped.
    pub crash_loop_detected: bool,
    /// Declared capability tokens.
    pub declared_capabilities: Vec<String>,
    /// Exercised-capability usage tokens (`capability:state`).
    pub exercised_capabilities: Vec<String>,
    /// Count of declared-but-unused capabilities.
    pub unused_declared_count: usize,
    /// Count of undeclared exercised capabilities.
    pub undeclared_exercised_count: usize,
    /// Enforcement-action token.
    pub enforcement_action: String,
    /// Enforcement-reason tokens.
    pub enforcement_reasons: Vec<String>,
    /// Recovery-path ref, when an action was taken.
    pub recovery_path_ref: Option<String>,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationBudgetExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5ActivationBudgetExportRow>,
    /// Whether every record's gate is consistent with its recomputation.
    pub all_gates_consistent: bool,
    /// Records that required any enforcement intervention.
    pub intervention_count: usize,
    /// Records that exercised an undeclared capability.
    pub undeclared_exercised_count: usize,
}

/// The typed M5 activation-budget packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ActivationBudget {
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
    /// Closed scope vocabulary (reused from the install-review sheets).
    pub scopes: Vec<InstallScope>,
    /// Closed host-class vocabulary (reused from the install-review sheets).
    pub host_classes: Vec<HostClass>,
    /// Closed runtime-origin vocabulary (reused from the governance matrix).
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed activation-budget-band vocabulary (reused from the governance matrix).
    pub activation_budget_bands: Vec<ActivationBudgetBand>,
    /// Closed activation-bucket vocabulary.
    pub activation_buckets: Vec<ActivationBucket>,
    /// Closed activation-trigger vocabulary.
    pub activation_triggers: Vec<ActivationTrigger>,
    /// Closed resource-pressure vocabulary.
    pub resource_pressures: Vec<ResourcePressure>,
    /// Closed capability-class vocabulary.
    pub capability_classes: Vec<CapabilityClass>,
    /// Closed capability-exercise-state vocabulary.
    pub capability_exercise_states: Vec<CapabilityExerciseState>,
    /// Closed enforcement-reason vocabulary.
    pub enforcement_reasons: Vec<EnforcementReason>,
    /// Closed enforcement-action vocabulary.
    pub enforcement_actions: Vec<EnforcementAction>,
    /// The session records.
    #[serde(default)]
    pub records: Vec<ActivationBudgetRecord>,
    /// Summary counts.
    pub summary: M5ActivationBudgetSummary,
}

impl M5ActivationBudget {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&ActivationBudgetRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Records scoped to the given scope.
    pub fn records_in_scope(
        &self,
        scope: InstallScope,
    ) -> impl Iterator<Item = &ActivationBudgetRecord> {
        self.records.iter().filter(move |r| r.scope == scope)
    }

    /// Records that required an enforcement intervention.
    pub fn records_requiring_intervention(&self) -> impl Iterator<Item = &ActivationBudgetRecord> {
        self.records.iter().filter(|r| r.requires_intervention())
    }

    /// Whether every record's stored gate agrees with its recomputation.
    pub fn all_gates_consistent(&self) -> bool {
        self.records
            .iter()
            .all(ActivationBudgetRecord::gate_consistent)
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> M5ActivationBudgetSummary {
        let count_action = |action: EnforcementAction| {
            self.records
                .iter()
                .filter(|r| r.enforcement_action == action)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.records.iter().map(|r| r.package_kind).collect();
        let host_classes: BTreeSet<HostClass> =
            self.records.iter().map(|r| r.runtime_host_class).collect();
        M5ActivationBudgetSummary {
            total_records: self.records.len(),
            no_action_records: count_action(EnforcementAction::NoAction),
            throttled_records: count_action(EnforcementAction::Throttled),
            downgraded_records: count_action(EnforcementAction::Downgraded),
            paused_records: count_action(EnforcementAction::Paused),
            quarantined_records: count_action(EnforcementAction::Quarantined),
            intervention_records: self
                .records
                .iter()
                .filter(|r| r.requires_intervention())
                .count(),
            over_activation_budget_records: self
                .records
                .iter()
                .filter(|r| r.over_activation_budget())
                .count(),
            crash_loop_records: self
                .records
                .iter()
                .filter(|r| r.crash_loop_detected())
                .count(),
            undeclared_exercised_records: self
                .records
                .iter()
                .filter(|r| r.has_undeclared_exercised())
                .count(),
            unused_declared_grant_records: self
                .records
                .iter()
                .filter(|r| r.unused_declared_count() > 0)
                .count(),
            cold_activation_records: self
                .records
                .iter()
                .filter(|r| r.activation_bucket.is_cold())
                .count(),
            warm_activation_records: self
                .records
                .iter()
                .filter(|r| !r.activation_bucket.is_cold())
                .count(),
            distinct_package_kinds: package_kinds.len(),
            distinct_host_classes: host_classes.len(),
        }
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// admin audits, docs/help, and release/public-truth packets — render instead
    /// of restating activation-cost and exercised-capability text by hand.
    pub fn export_projection(&self) -> M5ActivationBudgetExportProjection {
        let rows = self
            .records
            .iter()
            .map(|r| M5ActivationBudgetExportRow {
                record_id: r.record_id.clone(),
                package_kind: r.package_kind.as_str().to_owned(),
                scope: r.scope.as_str().to_owned(),
                runtime_host_class: r.runtime_host_class.as_str().to_owned(),
                runtime_origin: r.runtime_origin.as_str().to_owned(),
                activation_bucket: r.activation_bucket.as_str().to_owned(),
                activation_trigger: r.activation_trigger.as_str().to_owned(),
                activation_budget_band: r.activation_budget_band.as_str().to_owned(),
                cold_start_pressure: r.cold_start_pressure.as_str().to_owned(),
                memory_pressure: r.memory_pressure.as_str().to_owned(),
                window_restarts_allowed: r.restart_budget.window_restarts_allowed,
                window_restarts_used: r.restart_budget.window_restarts_used,
                crash_loop_detected: r.crash_loop_detected(),
                declared_capabilities: r
                    .declared_capabilities
                    .iter()
                    .map(|c| c.as_str().to_owned())
                    .collect(),
                exercised_capabilities: r
                    .exercised_capabilities
                    .iter()
                    .map(|usage| {
                        format!(
                            "{}:{}",
                            usage.capability_class.as_str(),
                            usage.exercise_state.as_str()
                        )
                    })
                    .collect(),
                unused_declared_count: r.unused_declared_count(),
                undeclared_exercised_count: r.undeclared_exercised_count(),
                enforcement_action: r.enforcement_action.as_str().to_owned(),
                enforcement_reasons: r
                    .enforcement_reasons
                    .iter()
                    .map(|reason| reason.as_str().to_owned())
                    .collect(),
                recovery_path_ref: r.recovery_path_ref.clone(),
                governance_family_ref: r.governance_family_ref.clone(),
                summary: format!(
                    "{}: {} activation in {} scope on {}, band {}, action {}",
                    r.package_kind.as_str(),
                    r.activation_bucket.as_str(),
                    r.scope.as_str(),
                    r.runtime_host_class.as_str(),
                    r.activation_budget_band.as_str(),
                    r.enforcement_action.as_str(),
                ),
            })
            .collect();
        M5ActivationBudgetExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_gates_consistent: self.all_gates_consistent(),
            intervention_count: self
                .records
                .iter()
                .filter(|r| r.requires_intervention())
                .count(),
            undeclared_exercised_count: self
                .records
                .iter()
                .filter(|r| r.has_undeclared_exercised())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ActivationBudgetViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for record in &self.records {
            if !seen.insert(record.record_id.clone()) {
                violations.push(M5ActivationBudgetViolation::DuplicateRecordId {
                    record_id: record.record_id.clone(),
                });
            }
            self.validate_record(record, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5ActivationBudgetViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ActivationBudgetViolation>) {
        if self.schema_version != M5_ACTIVATION_BUDGET_SCHEMA_VERSION {
            violations.push(M5ActivationBudgetViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_ACTIVATION_BUDGET_RECORD_KIND {
            violations.push(M5ActivationBudgetViolation::UnsupportedRecordKind {
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
                violations.push(M5ActivationBudgetViolation::EmptyField {
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
            ("scopes", self.scopes == InstallScope::ALL.to_vec()),
            ("host_classes", self.host_classes == HostClass::ALL.to_vec()),
            (
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
            ),
            (
                "activation_budget_bands",
                self.activation_budget_bands == ActivationBudgetBand::ALL.to_vec(),
            ),
            (
                "activation_buckets",
                self.activation_buckets == ActivationBucket::ALL.to_vec(),
            ),
            (
                "activation_triggers",
                self.activation_triggers == ActivationTrigger::ALL.to_vec(),
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
            (
                "enforcement_reasons",
                self.enforcement_reasons == EnforcementReason::ALL.to_vec(),
            ),
            (
                "enforcement_actions",
                self.enforcement_actions == EnforcementAction::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5ActivationBudgetViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_record(
        &self,
        record: &ActivationBudgetRecord,
        violations: &mut Vec<M5ActivationBudgetViolation>,
    ) {
        for (field, value) in [
            ("record_id", &record.record_id),
            ("listing_ref", &record.listing_ref),
            ("display_label", &record.display_label),
            ("governance_family_ref", &record.governance_family_ref),
            ("session_ref", &record.session_ref),
            ("summary", &record.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ActivationBudgetViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: field,
                });
            }
        }
        if let Some(window_ref) = &record.restart_budget.window_ref {
            if window_ref.trim().is_empty() {
                violations.push(M5ActivationBudgetViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: "restart_budget.window_ref",
                });
            }
        }

        self.validate_record_capabilities(record, violations);
        self.validate_record_recovery(record, violations);
        self.validate_record_gate(record, violations);
    }

    fn validate_record_capabilities(
        &self,
        record: &ActivationBudgetRecord,
        violations: &mut Vec<M5ActivationBudgetViolation>,
    ) {
        let mut declared = BTreeSet::new();
        for class in &record.declared_capabilities {
            if !declared.insert(*class) {
                violations.push(M5ActivationBudgetViolation::DuplicateDeclaredCapability {
                    record_id: record.record_id.clone(),
                    capability: class.as_str(),
                });
            }
        }

        let mut seen_usage = BTreeSet::new();
        for usage in &record.exercised_capabilities {
            if usage.evidence_ref.trim().is_empty() {
                violations.push(M5ActivationBudgetViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: "exercised_capabilities.evidence_ref",
                });
            }
            if !seen_usage.insert(usage.capability_class) {
                violations.push(M5ActivationBudgetViolation::DuplicateCapabilityUsage {
                    record_id: record.record_id.clone(),
                    capability: usage.capability_class.as_str(),
                });
            }
            // A usage's exercise state must agree with the declared manifest: a
            // declared state must be backed by a declared capability, and an
            // undeclared exercise must not be.
            let is_declared = declared.contains(&usage.capability_class);
            let state_declared = usage.exercise_state.is_declared();
            if is_declared != state_declared {
                violations.push(M5ActivationBudgetViolation::CapabilityDeclarationMismatch {
                    record_id: record.record_id.clone(),
                    capability: usage.capability_class.as_str(),
                    state: usage.exercise_state.as_str(),
                });
            }
        }

        // Every declared capability must carry exactly one usage row, so a declared
        // grant can never go unaccounted for.
        for class in &declared {
            if !seen_usage.contains(class) {
                violations.push(
                    M5ActivationBudgetViolation::DeclaredCapabilityMissingUsage {
                        record_id: record.record_id.clone(),
                        capability: class.as_str(),
                    },
                );
            }
        }
    }

    fn validate_record_recovery(
        &self,
        record: &ActivationBudgetRecord,
        violations: &mut Vec<M5ActivationBudgetViolation>,
    ) {
        let has_recovery = record
            .recovery_path_ref
            .as_deref()
            .map(str::trim)
            .is_some_and(|r| !r.is_empty());
        // An enforced session must name a recovery path, and an unimpeded session
        // must not carry one — so a throttle, downgrade, pause, or quarantine always
        // names a way back and a clean session stays clean.
        if record.requires_intervention() && !has_recovery {
            violations.push(M5ActivationBudgetViolation::MissingRecoveryPath {
                record_id: record.record_id.clone(),
            });
        }
        if record.runs_unimpeded() && record.recovery_path_ref.is_some() {
            violations.push(M5ActivationBudgetViolation::UnexpectedRecoveryPath {
                record_id: record.record_id.clone(),
            });
        }
    }

    fn validate_record_gate(
        &self,
        record: &ActivationBudgetRecord,
        violations: &mut Vec<M5ActivationBudgetViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for reason in &record.enforcement_reasons {
            if !seen.insert(*reason) {
                violations.push(M5ActivationBudgetViolation::DuplicateEnforcementReason {
                    record_id: record.record_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The recorded reasons must equal the recomputed set, so an over-budget,
        // crash-loop, or undeclared-capability signal can never be asserted or hidden
        // by hand.
        if record.enforcement_reasons != record.computed_enforcement_reasons() {
            violations.push(M5ActivationBudgetViolation::EnforcementReasonsMismatch {
                record_id: record.record_id.clone(),
            });
        }

        // The published action must equal the recomputed gate.
        let computed = record.computed_enforcement_action();
        if record.enforcement_action != computed {
            violations.push(M5ActivationBudgetViolation::EnforcementActionMismatch {
                record_id: record.record_id.clone(),
                stored: record.enforcement_action.as_str(),
                computed: computed.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 activation-budget packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ActivationBudgetViolation {
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
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Record or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A record id appears more than once.
    DuplicateRecordId {
        /// Duplicate record id.
        record_id: String,
    },
    /// A declared capability appears more than once.
    DuplicateDeclaredCapability {
        /// Record id.
        record_id: String,
        /// Capability token.
        capability: &'static str,
    },
    /// A capability usage row appears more than once.
    DuplicateCapabilityUsage {
        /// Record id.
        record_id: String,
        /// Capability token.
        capability: &'static str,
    },
    /// A usage's exercise state disagrees with the declared manifest.
    CapabilityDeclarationMismatch {
        /// Record id.
        record_id: String,
        /// Capability token.
        capability: &'static str,
        /// Exercise-state token.
        state: &'static str,
    },
    /// A declared capability carries no usage row.
    DeclaredCapabilityMissingUsage {
        /// Record id.
        record_id: String,
        /// Capability token.
        capability: &'static str,
    },
    /// An enforced record is missing a recovery path.
    MissingRecoveryPath {
        /// Record id.
        record_id: String,
    },
    /// An unimpeded record carries a recovery path.
    UnexpectedRecoveryPath {
        /// Record id.
        record_id: String,
    },
    /// A record lists an enforcement reason more than once.
    DuplicateEnforcementReason {
        /// Record id.
        record_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A record's enforcement reasons disagree with the recomputed set.
    EnforcementReasonsMismatch {
        /// Record id.
        record_id: String,
    },
    /// A record's enforcement action disagrees with the recomputed gate.
    EnforcementActionMismatch {
        /// Record id.
        record_id: String,
        /// Stored action token.
        stored: &'static str,
        /// Recomputed action token.
        computed: &'static str,
    },
    /// The summary counts disagree with the records.
    SummaryMismatch,
}

impl fmt::Display for M5ActivationBudgetViolation {
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
            Self::DuplicateRecordId { record_id } => {
                write!(f, "duplicate activation-budget record id {record_id}")
            }
            Self::DuplicateDeclaredCapability {
                record_id,
                capability,
            } => {
                write!(
                    f,
                    "record {record_id} declares capability {capability} more than once"
                )
            }
            Self::DuplicateCapabilityUsage {
                record_id,
                capability,
            } => {
                write!(
                    f,
                    "record {record_id} reports usage for capability {capability} more than once"
                )
            }
            Self::CapabilityDeclarationMismatch {
                record_id,
                capability,
                state,
            } => {
                write!(
                    f,
                    "record {record_id} capability {capability} usage state {state} disagrees with its declared manifest"
                )
            }
            Self::DeclaredCapabilityMissingUsage {
                record_id,
                capability,
            } => {
                write!(
                    f,
                    "record {record_id} declares capability {capability} but reports no usage for it"
                )
            }
            Self::MissingRecoveryPath { record_id } => {
                write!(
                    f,
                    "record {record_id} took an enforcement action but names no recovery path"
                )
            }
            Self::UnexpectedRecoveryPath { record_id } => {
                write!(
                    f,
                    "record {record_id} runs unimpeded but carries a recovery path"
                )
            }
            Self::DuplicateEnforcementReason { record_id, reason } => {
                write!(f, "record {record_id} repeats enforcement reason {reason}")
            }
            Self::EnforcementReasonsMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} enforcement reasons disagree with the recomputed set"
                )
            }
            Self::EnforcementActionMismatch {
                record_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "record {record_id} publishes action {stored} but the recomputed gate is {computed}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the records")
            }
        }
    }
}

impl Error for M5ActivationBudgetViolation {}

/// Loads the embedded M5 activation-budget packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ActivationBudget`].
pub fn current_m5_activation_budget() -> Result<M5ActivationBudget, serde_json::Error> {
    serde_json::from_str(M5_ACTIVATION_BUDGET_JSON)
}

#[cfg(test)]
mod tests;
