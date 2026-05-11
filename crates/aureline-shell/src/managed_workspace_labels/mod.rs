//! Bounded managed-workspace lifecycle-labels wedge on one certified
//! prototype path.
//!
//! ## What the wedge owns
//!
//! Every managed-workspace surface in M1 must answer the same three
//! questions before the user trusts what they see on the row:
//! *which lifecycle state am I in, which class of workspace copy am I
//! looking at, and what recovery action is available right now?* The
//! shell already projects target identity and host-boundary cues for the
//! terminal lane through [`crate::host_boundary_cues`]. This wedge
//! extends that truth — for one bounded prototype wedge only — by
//! recording each managed-workspace lifecycle event as a typed
//! [`ManagedWorkspaceLifecycleStep`] that names the lifecycle label, the
//! workspace-copy class, the chrome degraded chip, the recovery action
//! offered, and the authority lineage the row was admitted under.
//!
//! ## Why a typed step-by-step record, not a single badge
//!
//! Managed-workspace lifecycles are deliberately *visible* — the user
//! has to be able to tell apart a live environment from a snapshot view,
//! a suspended workspace from a fresh reprovisioned copy, and a
//! reconnecting transient state from a read-only degraded one. A static
//! badge cannot prove the prototype walked the lifecycle honestly; a
//! per-step record can. Each step carries:
//!
//! - one [`ManagedLifecycleLabelClass`] from the closed M1 vocabulary
//!   (authenticating / connecting / warming / ready /
//!   reconnecting / read_only_degraded / suspended / reprovisioning /
//!   snapshot_only_view / closed),
//! - one [`WorkspaceCopyClass`] from the closed copy-class vocabulary
//!   (live_environment / snapshot_only_view / suspended_workspace /
//!   fresh_reprovisioned_copy / not_yet_admitted_copy_class),
//! - an optional [`DegradedStateToken`] mapped from the chrome
//!   vocabulary so the wedge never re-derives chrome chips locally,
//! - one [`RecoveryActionClass`] from the closed recovery-action
//!   vocabulary the chrome offers verbatim,
//! - the typed [`ManagedAuthorityLineage`] (workspace_id +
//!   managed_tenant_ref + identity_mode_token + locality_class_token)
//!   so the chrome can quote the authority/lineage truth on every step.
//!
//! ## What the wedge does NOT own (deliberately)
//!
//! - It does not run a managed control-plane, does not negotiate a
//!   managed tenancy, and does not productionise lifecycle execution.
//!   The wedge records lifecycle truth as supplied by the caller; the
//!   shared `single_certified_wedge_only` claim limit is always
//!   rendered to keep the prototype boundary explicit.
//! - It does not fork the locality / tenancy / key-mode vocabulary
//!   defined upstream in
//!   `schemas/governance/locality_tenancy_keymode.schema.json` — the
//!   wedge mirrors the relevant tokens verbatim on the authority
//!   lineage row.
//! - It does not duplicate the lifecycle vocabulary owned by
//!   [`aureline_workspace::WorkspaceLifecycleState`] for local
//!   workspaces. This wedge labels the *managed* lifecycle subset only;
//!   surfaces still consume the local lifecycle through the upstream
//!   crate.

use serde::{Deserialize, Serialize};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried in serialized
/// [`ManagedWorkspaceLifecycleCardRecord`] payloads.
pub const MANAGED_WORKSPACE_LIFECYCLE_CARD_RECORD_KIND: &str =
    "managed_workspace_lifecycle_card_record";

/// Schema version for the [`ManagedWorkspaceLifecycleCardRecord`] payload
/// shape.
pub const MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_VERSION: u32 = 1;

/// Frozen prototype-label vocabulary. The chrome MUST quote the token
/// verbatim and MUST NOT drop the chip even when every step lands on a
/// trusted local seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype: managed-workspace lifecycle labels on one
    /// certified wedge.
    M1PrototypeManagedWorkspaceLifecycleLabels,
}

impl PrototypeLabel {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeManagedWorkspaceLifecycleLabels => {
                "m1_prototype_managed_workspace_lifecycle_labels"
            }
        }
    }

    /// Human-readable chip label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeManagedWorkspaceLifecycleLabels => {
                "Prototype — managed-workspace lifecycle labels (bounded wedge)"
            }
        }
    }
}

/// Closed managed-workspace lifecycle-label vocabulary the wedge mints on
/// each step. The order in which labels may appear is enforced by the
/// wedge transition logic — steps cannot skip ahead and cannot reach a
/// degraded label without naming the prior live label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedLifecycleLabelClass {
    /// The wedge is collecting a managed identity through the system
    /// browser (or the equivalent device-code fallback). No workspace
    /// state is being read or written yet.
    Authenticating,
    /// The wedge is binding to the managed workspace target, after
    /// authentication succeeded but before the workspace is warm.
    Connecting,
    /// The wedge is warming the managed workspace — pulling the working
    /// set, hydrating caches, or restoring snapshot state. Editing must
    /// not commit yet.
    Warming,
    /// The wedge reports the managed workspace as fully ready for
    /// editing. This is the only label that admits writes against the
    /// live environment.
    Ready,
    /// Transport dropped or session expired; the wedge is restoring the
    /// same managed identity against the same canonical workspace id.
    /// Local edits queued during this state remain visible but are not
    /// confirmed against the managed copy.
    Reconnecting,
    /// The managed workspace is reachable but writes are not admitted —
    /// either because a managed policy paused them, the seat lost write
    /// scope, or the wedge fell back to a snapshot view. Reads remain
    /// honest.
    ReadOnlyDegraded,
    /// The managed workspace is suspended: the user (or the managed
    /// tenant) explicitly paused the live environment. No writes are
    /// admitted, no reconnect attempts are running, and the recovery
    /// action surfaces a resume path.
    Suspended,
    /// The managed workspace is being reprovisioned — the prior copy is
    /// gone or being replaced. The wedge MUST surface the fresh copy as
    /// a distinct workspace-copy class so chrome cannot quietly inherit
    /// the prior state's trust.
    Reprovisioning,
    /// The wedge surfaces a snapshot-only view of the managed workspace
    /// — for read-only review when the live environment is unreachable
    /// or not yet admitted. Writes are not admissible.
    SnapshotOnlyView,
    /// The wedge stopped following this managed workspace. The chrome
    /// keeps the prior identity row visible until the consumer drops
    /// the row.
    Closed,
}

impl ManagedLifecycleLabelClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authenticating => "authenticating",
            Self::Connecting => "connecting",
            Self::Warming => "warming",
            Self::Ready => "ready",
            Self::Reconnecting => "reconnecting",
            Self::ReadOnlyDegraded => "read_only_degraded",
            Self::Suspended => "suspended",
            Self::Reprovisioning => "reprovisioning",
            Self::SnapshotOnlyView => "snapshot_only_view",
            Self::Closed => "closed",
        }
    }

    /// Human-readable label rendered on the row.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Authenticating => "Authenticating",
            Self::Connecting => "Connecting",
            Self::Warming => "Warming",
            Self::Ready => "Ready",
            Self::Reconnecting => "Reconnecting",
            Self::ReadOnlyDegraded => "Read-only (degraded)",
            Self::Suspended => "Suspended",
            Self::Reprovisioning => "Reprovisioning",
            Self::SnapshotOnlyView => "Snapshot-only view",
            Self::Closed => "Closed",
        }
    }

    /// True when the label admits writes against the live managed
    /// environment. Only [`Self::Ready`] returns true in M1.
    pub const fn admits_writes(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// True when the label represents a degraded / non-interactive
    /// posture the wedge MUST pair with a non-empty
    /// [`RecoveryActionClass`] other than [`RecoveryActionClass::None`].
    pub const fn is_degraded_posture(self) -> bool {
        matches!(
            self,
            Self::Reconnecting
                | Self::ReadOnlyDegraded
                | Self::Suspended
                | Self::Reprovisioning
                | Self::SnapshotOnlyView
        )
    }
}

/// Closed workspace-copy-class vocabulary distinguishing the four
/// managed copies the spec requires the wedge to keep visibly distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceCopyClass {
    /// The managed workspace is admitting writes against the live
    /// environment.
    LiveEnvironment,
    /// A read-only view of a prior snapshot of the managed workspace.
    SnapshotOnlyView,
    /// The managed workspace is paused; the prior live copy is retained
    /// but no writes are admitted until resumed.
    SuspendedWorkspace,
    /// A fresh reprovisioned copy of the managed workspace. The wedge
    /// MUST NOT silently treat this as a continuation of the prior
    /// live environment.
    FreshReprovisionedCopy,
    /// The wedge has not yet bound to a copy (initial authenticating
    /// step). No copy class is implied.
    NotYetAdmittedCopyClass,
}

impl WorkspaceCopyClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveEnvironment => "live_environment",
            Self::SnapshotOnlyView => "snapshot_only_view",
            Self::SuspendedWorkspace => "suspended_workspace",
            Self::FreshReprovisionedCopy => "fresh_reprovisioned_copy",
            Self::NotYetAdmittedCopyClass => "not_yet_admitted_copy_class",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveEnvironment => "Live environment",
            Self::SnapshotOnlyView => "Snapshot-only view",
            Self::SuspendedWorkspace => "Suspended workspace",
            Self::FreshReprovisionedCopy => "Fresh reprovisioned copy",
            Self::NotYetAdmittedCopyClass => "No copy yet",
        }
    }
}

/// Closed recovery-action vocabulary the wedge surfaces verbatim on each
/// step. The chrome quotes the token; it does not invent local copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// No recovery action applies (the workspace is fully ready or
    /// closed).
    None,
    /// Reopen the system-browser auth flow to acquire a fresh managed
    /// session.
    ReauthenticateViaSystemBrowser,
    /// Retry the managed connection now (typically after a transient
    /// network failure).
    RetryConnection,
    /// Wait for the warm-up to complete; no user action required.
    WaitForWarm,
    /// Resume the suspended workspace.
    ResumeFromSuspension,
    /// Accept the fresh reprovisioned copy (acknowledge that the prior
    /// state is gone).
    AcceptReprovisionedState,
    /// Continue in the read-only snapshot view.
    ContinueInSnapshotView,
    /// Export the local-safe artifacts the wedge can still read.
    ExportLocalSafeArtifacts,
}

impl RecoveryActionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ReauthenticateViaSystemBrowser => "reauthenticate_via_system_browser",
            Self::RetryConnection => "retry_connection",
            Self::WaitForWarm => "wait_for_warm",
            Self::ResumeFromSuspension => "resume_from_suspension",
            Self::AcceptReprovisionedState => "accept_reprovisioned_state",
            Self::ContinueInSnapshotView => "continue_in_snapshot_view",
            Self::ExportLocalSafeArtifacts => "export_local_safe_artifacts",
        }
    }

    /// Human-readable label rendered on the row.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "No action required",
            Self::ReauthenticateViaSystemBrowser => "Reauthenticate via system browser",
            Self::RetryConnection => "Retry connection",
            Self::WaitForWarm => "Wait for warm-up",
            Self::ResumeFromSuspension => "Resume from suspension",
            Self::AcceptReprovisionedState => "Accept reprovisioned state",
            Self::ContinueInSnapshotView => "Continue in snapshot view",
            Self::ExportLocalSafeArtifacts => "Export local-safe artifacts",
        }
    }
}

/// Frozen claim-limit vocabulary the chrome quotes verbatim under every
/// card. The set is intentionally small: it pins the wedge's M1 scope
/// so chrome cannot imply broad managed-workspace breadth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedLifecycleClaimLimit {
    /// One certified bounded wedge only. Task / debug / provider /
    /// terminal lanes do not own a managed-workspace lifecycle card
    /// here.
    SingleCertifiedWedgeOnly,
    /// Does not run a managed control-plane in M1; the wedge only
    /// labels the lifecycle as supplied by the caller.
    NoManagedControlPlaneInM1,
    /// Does not orchestrate managed tenancy, seat issuance, or
    /// admin-console productization.
    NoTenancyOrchestration,
    /// Does not own lifecycle-executor productization; the wedge mints
    /// labels for one prototype path, not a managed runtime.
    NoLifecycleExecutorProductization,
}

impl ManagedLifecycleClaimLimit {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCertifiedWedgeOnly => "single_certified_wedge_only",
            Self::NoManagedControlPlaneInM1 => "no_managed_control_plane_in_m1",
            Self::NoTenancyOrchestration => "no_tenancy_orchestration",
            Self::NoLifecycleExecutorProductization => "no_lifecycle_executor_productization",
        }
    }

    /// Human-readable claim label rendered under the card.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleCertifiedWedgeOnly => {
                "One certified wedge only — bounded managed-workspace prototype path."
            }
            Self::NoManagedControlPlaneInM1 => {
                "M1 does not run a managed control-plane; the wedge labels lifecycle truth supplied by the caller."
            }
            Self::NoTenancyOrchestration => {
                "Does not orchestrate managed tenancy, seat issuance, or admin-console flows."
            }
            Self::NoLifecycleExecutorProductization => {
                "Does not own lifecycle-executor productization; bounded prototype labels only."
            }
        }
    }

    /// Canonical M1 claim-limit set. Order is stable; chrome MUST render
    /// in this order.
    pub const fn canonical_set() -> [ManagedLifecycleClaimLimit; 4] {
        [
            Self::SingleCertifiedWedgeOnly,
            Self::NoManagedControlPlaneInM1,
            Self::NoTenancyOrchestration,
            Self::NoLifecycleExecutorProductization,
        ]
    }
}

/// One claim-limit row carried on the serialized card payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedLifecycleClaimLimitRow {
    pub token: String,
    pub label: String,
}

impl ManagedLifecycleClaimLimitRow {
    fn from_limit(limit: ManagedLifecycleClaimLimit) -> Self {
        Self {
            token: limit.as_str().to_owned(),
            label: limit.label().to_owned(),
        }
    }
}

/// Inspectable authority-lineage block mirrored on every step. The
/// tokens are stable strings drawn from the upstream
/// locality/tenancy/key-mode vocabulary the seed must label without
/// forking. The wedge accepts opaque caller-supplied tokens (it does not
/// admit a managed tenant on its own) and refuses empty workspace ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedAuthorityLineage {
    pub workspace_id: String,
    pub managed_tenant_ref: String,
    pub identity_mode_token: String,
    pub locality_class_token: String,
}

impl ManagedAuthorityLineage {
    /// Construct a lineage row. Empty `workspace_id` is allowed only so
    /// the invariant validator can surface
    /// [`ManagedLifecycleInvariantViolation::MissingAuthorityLineage`]
    /// without panicking; the wedge API otherwise refuses empty ids.
    pub fn new(
        workspace_id: impl Into<String>,
        managed_tenant_ref: impl Into<String>,
        identity_mode_token: impl Into<String>,
        locality_class_token: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            managed_tenant_ref: managed_tenant_ref.into(),
            identity_mode_token: identity_mode_token.into(),
            locality_class_token: locality_class_token.into(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.workspace_id.is_empty()
            && !self.managed_tenant_ref.is_empty()
            && !self.identity_mode_token.is_empty()
            && !self.locality_class_token.is_empty()
    }
}

/// Errors the wedge raises on illegal lifecycle transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WedgeError {
    /// A `record_*` call landed before the wedge was opened.
    NotInitialized,
    /// A `record_*` call landed after the wedge was closed.
    AlreadyClosed,
    /// A transition skipped the canonical lifecycle ordering — for
    /// example moving from `authenticating` directly to `ready` without
    /// recording `connecting` and `warming` first. The wedge cannot
    /// silently flatten the lifecycle.
    SkippedTransition {
        from: ManagedLifecycleLabelClass,
        to: ManagedLifecycleLabelClass,
    },
    /// A reconnect step landed without a prior `ready` or
    /// `read_only_degraded` step. Reconnect is only meaningful once the
    /// workspace was admitted at least once.
    ReconnectWithoutPriorConnection,
    /// A reprovisioning step landed without a prior `suspended` /
    /// `read_only_degraded` step. The wedge cannot replace a live copy
    /// out of thin air.
    ReprovisionWithoutPriorPause,
    /// The caller supplied an empty managed authority lineage where one
    /// was required.
    EmptyAuthorityLineage,
}

impl std::fmt::Display for WedgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "wedge has no initial step"),
            Self::AlreadyClosed => write!(f, "wedge is already closed"),
            Self::SkippedTransition { from, to } => write!(
                f,
                "skipped managed lifecycle transition: from={} to={}",
                from.as_str(),
                to.as_str(),
            ),
            Self::ReconnectWithoutPriorConnection => write!(
                f,
                "reconnect requires a prior ready/read_only_degraded step",
            ),
            Self::ReprovisionWithoutPriorPause => write!(
                f,
                "reprovision requires a prior suspended/read_only_degraded step",
            ),
            Self::EmptyAuthorityLineage => write!(f, "authority lineage cannot be empty"),
        }
    }
}

impl std::error::Error for WedgeError {}

/// Closed invariant-violation vocabulary surfaced on the card. The
/// chrome quotes each token verbatim and MUST render a visible failure
/// row when one fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedLifecycleInvariantViolation {
    /// The card is missing the prototype-label chip.
    MissingPrototypeLabel,
    /// The canonical claim-limit set is missing or out of order.
    ClaimLimitsMissingOrOutOfOrder,
    /// A step's label skipped a required intermediate state. (Catches
    /// callers who patch the record list directly rather than going
    /// through the API.)
    SkippedTransition,
    /// A degraded posture step is missing a recovery action (i.e. carries
    /// [`RecoveryActionClass::None`]).
    MissingRecoveryActionForDegradedState,
    /// A step is missing its workspace-copy class (or carries the
    /// `not_yet_admitted_copy_class` sentinel while claiming a
    /// post-authenticating label).
    CopyClassMissing,
    /// A step's lifecycle label and copy class disagree — for example a
    /// `ready` step that carries `suspended_workspace`.
    LabelAndCopyClassDisagree,
    /// A step is missing its managed authority lineage block.
    MissingAuthorityLineage,
    /// A reconnect step landed without a prior `ready` /
    /// `read_only_degraded` step.
    ReconnectingWithoutPriorConnection,
    /// A reprovisioning step landed without a prior `suspended` /
    /// `read_only_degraded` step.
    ReprovisioningWithoutPriorPause,
}

impl ManagedLifecycleInvariantViolation {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "missing_prototype_label",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::SkippedTransition => "skipped_transition",
            Self::MissingRecoveryActionForDegradedState => {
                "missing_recovery_action_for_degraded_state"
            }
            Self::CopyClassMissing => "copy_class_missing",
            Self::LabelAndCopyClassDisagree => "label_and_copy_class_disagree",
            Self::MissingAuthorityLineage => "missing_authority_lineage",
            Self::ReconnectingWithoutPriorConnection => "reconnecting_without_prior_connection",
            Self::ReprovisioningWithoutPriorPause => "reprovisioning_without_prior_pause",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "Prototype label chip is missing.",
            Self::ClaimLimitsMissingOrOutOfOrder => {
                "Canonical claim-limit set missing or out of order."
            }
            Self::SkippedTransition => "Lifecycle skipped a required intermediate state.",
            Self::MissingRecoveryActionForDegradedState => {
                "Degraded posture is missing a recovery action."
            }
            Self::CopyClassMissing => "Step is missing its workspace-copy class.",
            Self::LabelAndCopyClassDisagree => {
                "Lifecycle label and workspace-copy class disagree."
            }
            Self::MissingAuthorityLineage => "Step is missing its managed authority lineage.",
            Self::ReconnectingWithoutPriorConnection => {
                "Reconnect landed without a prior ready/read-only-degraded step."
            }
            Self::ReprovisioningWithoutPriorPause => {
                "Reprovisioning landed without a prior suspended/read-only-degraded step."
            }
        }
    }
}

/// One typed invariant row rendered on the card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedLifecycleInvariantRow {
    pub violation: ManagedLifecycleInvariantViolation,
    pub violation_token: String,
    pub violation_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addressable_step_id: Option<String>,
}

impl ManagedLifecycleInvariantRow {
    fn new(
        violation: ManagedLifecycleInvariantViolation,
        step_id: Option<&str>,
    ) -> Self {
        Self {
            violation,
            violation_token: violation.as_str().to_owned(),
            violation_label: violation.label().to_owned(),
            addressable_step_id: step_id.map(str::to_owned),
        }
    }
}

/// Serializable lifecycle step carried on the card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleStep {
    pub step_id: String,
    pub label: ManagedLifecycleLabelClass,
    pub label_token: String,
    pub label_display: String,
    pub copy_class: WorkspaceCopyClass,
    pub copy_class_token: String,
    pub copy_class_label: String,
    pub admits_writes: bool,
    pub recovery_action: RecoveryActionClass,
    pub recovery_action_token: String,
    pub recovery_action_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    pub authority_lineage: ManagedAuthorityLineage,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

/// Serializable card payload. The chrome renders this struct directly;
/// export and proof flows quote it verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleCardRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    pub workspace_id: String,
    pub wedge_id: String,
    pub steps: Vec<ManagedWorkspaceLifecycleStep>,
    pub current_label: ManagedLifecycleLabelClass,
    pub current_label_token: String,
    pub current_label_display: String,
    pub current_copy_class: WorkspaceCopyClass,
    pub current_copy_class_token: String,
    pub current_recovery_action: RecoveryActionClass,
    pub current_recovery_action_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_degraded_token: Option<String>,
    pub current_admits_writes: bool,
    pub claim_limits: Vec<ManagedLifecycleClaimLimitRow>,
    pub invariants: Vec<ManagedLifecycleInvariantRow>,
    pub has_invariant_violations: bool,
    pub summary_line: String,
}

impl ManagedWorkspaceLifecycleCardRecord {
    /// Deterministic plaintext block. Support exports and proof captures
    /// quote this verbatim — the format is stable across hosts and
    /// never bakes in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display,
        ));
        out.push_str(&format!(
            "wedge={} workspace={}\n",
            self.wedge_id, self.workspace_id,
        ));
        out.push_str("steps:\n");
        for step in &self.steps {
            out.push_str(&format!(
                "  - id={} label={} copy={} writes={} recovery={} observed_at={}",
                step.step_id,
                step.label_token,
                step.copy_class_token,
                step.admits_writes,
                step.recovery_action_token,
                step.observed_at,
            ));
            if let Some(token) = &step.degraded_token {
                out.push_str(&format!(" degraded={}", token));
            }
            if let Some(reason) = &step.reason_code {
                out.push_str(&format!(" reason={}", reason));
            }
            out.push('\n');
            out.push_str(&format!(
                "      lineage: workspace={} tenant={} identity={} locality={}\n",
                step.authority_lineage.workspace_id,
                step.authority_lineage.managed_tenant_ref,
                step.authority_lineage.identity_mode_token,
                step.authority_lineage.locality_class_token,
            ));
        }
        out.push_str(&format!(
            "current_label={} copy={} writes={} recovery={}",
            self.current_label_token,
            self.current_copy_class_token,
            self.current_admits_writes,
            self.current_recovery_action_token,
        ));
        if let Some(token) = &self.current_degraded_token {
            out.push_str(&format!(" degraded={}", token));
        }
        out.push('\n');
        out.push_str("claim_limits:\n");
        for row in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", row.token, row.label));
        }
        out.push_str("invariants:\n");
        if self.invariants.is_empty() {
            out.push_str("  - clean\n");
        } else {
            for row in &self.invariants {
                let suffix = row
                    .addressable_step_id
                    .as_deref()
                    .map(|id| format!(" (step={id})"))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "  - {}: {}{}\n",
                    row.violation_token, row.violation_label, suffix,
                ));
            }
        }
        out.push_str(&format!("summary: {}\n", self.summary_line));
        out
    }
}

/// Bounded managed-workspace lifecycle-labels wedge.
///
/// Construct with [`ManagedWorkspaceLifecycleWedge::new`], then drive
/// the lifecycle: `open_authenticating` -> `record_connecting` ->
/// `record_warming` -> `record_ready` -> any of
/// `record_reconnecting` / `record_read_only_degraded` /
/// `record_suspended` / `record_reprovisioning` /
/// `record_snapshot_only_view` -> `record_closed`. Call
/// [`Self::card`] at any point to obtain the current snapshot record.
#[derive(Debug, Clone)]
pub struct ManagedWorkspaceLifecycleWedge {
    workspace_id: String,
    wedge_id: String,
    steps: Vec<ManagedWorkspaceLifecycleStep>,
    closed: bool,
    next_step_sequence: u64,
}

impl ManagedWorkspaceLifecycleWedge {
    /// Construct a wedge bound to the given workspace id. The wedge id
    /// is derived from the workspace id; fixture replay may override it
    /// via [`Self::with_wedge_id`].
    pub fn new(workspace_id: impl Into<String>) -> Self {
        let ws = workspace_id.into();
        let wedge_id = format!("managed_workspace_lifecycle_wedge:{ws}");
        Self {
            workspace_id: ws,
            wedge_id,
            steps: Vec::new(),
            closed: false,
            next_step_sequence: 0,
        }
    }

    /// Override the wedge id (e.g. for fixture replay or proof-capture
    /// determinism).
    pub fn with_wedge_id(mut self, wedge_id: impl Into<String>) -> Self {
        self.wedge_id = wedge_id.into();
        self
    }

    /// Current wedge id.
    pub fn wedge_id(&self) -> &str {
        &self.wedge_id
    }

    /// Workspace this wedge is wired to.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// True once [`Self::record_closed`] has been called.
    pub const fn is_closed(&self) -> bool {
        self.closed
    }

    /// Iterate steps in record order.
    pub fn steps(&self) -> &[ManagedWorkspaceLifecycleStep] {
        &self.steps
    }

    /// Record the initial authenticating step. Returns the minted step.
    pub fn open_authenticating(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        if self.closed {
            return Err(WedgeError::AlreadyClosed);
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Authenticating,
            WorkspaceCopyClass::NotYetAdmittedCopyClass,
            RecoveryActionClass::ReauthenticateViaSystemBrowser,
            Some(DegradedStateToken::Warming),
            lineage,
            observed_at,
            None,
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a connecting step. Requires a prior `authenticating` or
    /// `reprovisioning` step.
    pub fn record_connecting(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Authenticating
                | ManagedLifecycleLabelClass::Reprovisioning
        ) {
            return Err(WedgeError::SkippedTransition {
                from: prior,
                to: ManagedLifecycleLabelClass::Connecting,
            });
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let copy_class = match prior {
            ManagedLifecycleLabelClass::Reprovisioning => {
                WorkspaceCopyClass::FreshReprovisionedCopy
            }
            _ => WorkspaceCopyClass::NotYetAdmittedCopyClass,
        };
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Connecting,
            copy_class,
            RecoveryActionClass::RetryConnection,
            Some(DegradedStateToken::Warming),
            lineage,
            observed_at,
            None,
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a warming step. Requires a prior `connecting` step.
    pub fn record_warming(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(prior, ManagedLifecycleLabelClass::Connecting) {
            return Err(WedgeError::SkippedTransition {
                from: prior,
                to: ManagedLifecycleLabelClass::Warming,
            });
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        // Warming preserves the in-flight copy class from connecting —
        // either not_yet_admitted or fresh_reprovisioned.
        let copy_class = self
            .steps
            .last()
            .map(|step| step.copy_class)
            .unwrap_or(WorkspaceCopyClass::NotYetAdmittedCopyClass);
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Warming,
            copy_class,
            RecoveryActionClass::WaitForWarm,
            Some(DegradedStateToken::Warming),
            lineage,
            observed_at,
            None,
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record the ready step. Requires a prior `warming` or
    /// `reconnecting` step.
    pub fn record_ready(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Warming | ManagedLifecycleLabelClass::Reconnecting
        ) {
            return Err(WedgeError::SkippedTransition {
                from: prior,
                to: ManagedLifecycleLabelClass::Ready,
            });
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Ready,
            WorkspaceCopyClass::LiveEnvironment,
            RecoveryActionClass::None,
            None,
            lineage,
            observed_at,
            None,
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a reconnect step. Requires a prior `ready` or
    /// `read_only_degraded` step.
    pub fn record_reconnecting(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Ready | ManagedLifecycleLabelClass::ReadOnlyDegraded
        ) {
            return Err(WedgeError::ReconnectWithoutPriorConnection);
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        // Reconnecting preserves whichever copy class was current — a
        // ready->reconnecting carries live_environment; a
        // read_only_degraded->reconnecting carries the prior copy class
        // verbatim.
        let copy_class = self
            .steps
            .last()
            .map(|step| step.copy_class)
            .unwrap_or(WorkspaceCopyClass::LiveEnvironment);
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Reconnecting,
            copy_class,
            RecoveryActionClass::RetryConnection,
            Some(DegradedStateToken::Offline),
            lineage,
            observed_at,
            reason_code.map(str::to_owned),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a read-only-degraded step. Allowed from `ready` (policy
    /// pauses writes), `reconnecting` (we surfaced a snapshot fallback
    /// during reconnect), or `snapshot_only_view` (the user accepted a
    /// snapshot view explicitly).
    pub fn record_read_only_degraded(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Ready
                | ManagedLifecycleLabelClass::Reconnecting
                | ManagedLifecycleLabelClass::SnapshotOnlyView
        ) {
            return Err(WedgeError::SkippedTransition {
                from: prior,
                to: ManagedLifecycleLabelClass::ReadOnlyDegraded,
            });
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        // A degraded fallback carries snapshot_only_view as its copy
        // class because writes are not admitted.
        let step = self.mint_step(
            ManagedLifecycleLabelClass::ReadOnlyDegraded,
            WorkspaceCopyClass::SnapshotOnlyView,
            RecoveryActionClass::ContinueInSnapshotView,
            Some(DegradedStateToken::Limited),
            lineage,
            observed_at,
            Some(reason_code.to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a suspended step. Allowed from `ready` /
    /// `read_only_degraded`.
    pub fn record_suspended(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Ready | ManagedLifecycleLabelClass::ReadOnlyDegraded
        ) {
            return Err(WedgeError::SkippedTransition {
                from: prior,
                to: ManagedLifecycleLabelClass::Suspended,
            });
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Suspended,
            WorkspaceCopyClass::SuspendedWorkspace,
            RecoveryActionClass::ResumeFromSuspension,
            Some(DegradedStateToken::Limited),
            lineage,
            observed_at,
            Some(reason_code.to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a reprovisioning step. Requires a prior `suspended` /
    /// `read_only_degraded` step.
    pub fn record_reprovisioning(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Suspended
                | ManagedLifecycleLabelClass::ReadOnlyDegraded
        ) {
            return Err(WedgeError::ReprovisionWithoutPriorPause);
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Reprovisioning,
            WorkspaceCopyClass::FreshReprovisionedCopy,
            RecoveryActionClass::AcceptReprovisionedState,
            Some(DegradedStateToken::Warming),
            lineage,
            observed_at,
            Some(reason_code.to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a snapshot-only-view step. Allowed from `ready` or
    /// `read_only_degraded` so the chrome can offer a read-only review
    /// path.
    pub fn record_snapshot_only_view(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let prior = self.require_current_label()?;
        if !matches!(
            prior,
            ManagedLifecycleLabelClass::Ready | ManagedLifecycleLabelClass::ReadOnlyDegraded
        ) {
            return Err(WedgeError::SkippedTransition {
                from: prior,
                to: ManagedLifecycleLabelClass::SnapshotOnlyView,
            });
        }
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let step = self.mint_step(
            ManagedLifecycleLabelClass::SnapshotOnlyView,
            WorkspaceCopyClass::SnapshotOnlyView,
            RecoveryActionClass::ContinueInSnapshotView,
            Some(DegradedStateToken::Stale),
            lineage,
            observed_at,
            Some(reason_code.to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a closed step. The wedge stops accepting further
    /// `record_*` calls.
    pub fn record_closed(
        &mut self,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<&ManagedWorkspaceLifecycleStep, WedgeError> {
        let _ = self.require_current_label()?;
        if !lineage.is_complete() {
            return Err(WedgeError::EmptyAuthorityLineage);
        }
        let copy_class = self
            .steps
            .last()
            .map(|step| step.copy_class)
            .unwrap_or(WorkspaceCopyClass::NotYetAdmittedCopyClass);
        let step = self.mint_step(
            ManagedLifecycleLabelClass::Closed,
            copy_class,
            RecoveryActionClass::ExportLocalSafeArtifacts,
            Some(DegradedStateToken::Limited),
            lineage,
            observed_at,
            reason_code.map(str::to_owned),
        );
        self.steps.push(step);
        self.closed = true;
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Materialise the current card record.
    pub fn card(&self) -> ManagedWorkspaceLifecycleCardRecord {
        let label = PrototypeLabel::M1PrototypeManagedWorkspaceLifecycleLabels;
        let claim_limits: Vec<ManagedLifecycleClaimLimitRow> =
            ManagedLifecycleClaimLimit::canonical_set()
                .into_iter()
                .map(ManagedLifecycleClaimLimitRow::from_limit)
                .collect();
        let invariants = self.validate_invariants();
        let has_invariant_violations = !invariants.is_empty();
        let (
            current_label,
            current_copy,
            current_recovery,
            current_degraded,
            current_writes,
        ) = self.current_state();
        let summary_line = self.summary_line();
        ManagedWorkspaceLifecycleCardRecord {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_CARD_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_VERSION,
            prototype_label_token: label.as_str().to_owned(),
            prototype_label_display: label.label().to_owned(),
            workspace_id: self.workspace_id.clone(),
            wedge_id: self.wedge_id.clone(),
            steps: self.steps.clone(),
            current_label,
            current_label_token: current_label.as_str().to_owned(),
            current_label_display: current_label.label().to_owned(),
            current_copy_class: current_copy,
            current_copy_class_token: current_copy.as_str().to_owned(),
            current_recovery_action: current_recovery,
            current_recovery_action_token: current_recovery.as_str().to_owned(),
            current_degraded_token: current_degraded.map(|t| t.token().to_owned()),
            current_admits_writes: current_writes,
            claim_limits,
            invariants,
            has_invariant_violations,
            summary_line,
        }
    }

    fn require_current_label(&self) -> Result<ManagedLifecycleLabelClass, WedgeError> {
        if self.closed {
            return Err(WedgeError::AlreadyClosed);
        }
        match self.steps.last() {
            Some(step) => Ok(step.label),
            None => Err(WedgeError::NotInitialized),
        }
    }

    fn mint_step(
        &mut self,
        label: ManagedLifecycleLabelClass,
        copy_class: WorkspaceCopyClass,
        recovery_action: RecoveryActionClass,
        degraded: Option<DegradedStateToken>,
        lineage: ManagedAuthorityLineage,
        observed_at: &str,
        reason_code: Option<String>,
    ) -> ManagedWorkspaceLifecycleStep {
        let seq = self.next_step_sequence;
        self.next_step_sequence = self.next_step_sequence.saturating_add(1);
        let step_id = format!("{}:{}:{}", self.wedge_id, label.as_str(), seq);
        ManagedWorkspaceLifecycleStep {
            step_id,
            label,
            label_token: label.as_str().to_owned(),
            label_display: label.label().to_owned(),
            copy_class,
            copy_class_token: copy_class.as_str().to_owned(),
            copy_class_label: copy_class.label().to_owned(),
            admits_writes: label.admits_writes(),
            recovery_action,
            recovery_action_token: recovery_action.as_str().to_owned(),
            recovery_action_label: recovery_action.label().to_owned(),
            degraded_token: degraded.map(|t| t.token().to_owned()),
            authority_lineage: lineage,
            observed_at: observed_at.to_owned(),
            reason_code,
        }
    }

    fn current_state(
        &self,
    ) -> (
        ManagedLifecycleLabelClass,
        WorkspaceCopyClass,
        RecoveryActionClass,
        Option<DegradedStateToken>,
        bool,
    ) {
        match self.steps.last() {
            Some(step) => {
                let degraded = step
                    .degraded_token
                    .as_deref()
                    .and_then(degraded_token_from_str);
                (
                    step.label,
                    step.copy_class,
                    step.recovery_action,
                    degraded,
                    step.admits_writes,
                )
            }
            None => (
                ManagedLifecycleLabelClass::Authenticating,
                WorkspaceCopyClass::NotYetAdmittedCopyClass,
                RecoveryActionClass::ReauthenticateViaSystemBrowser,
                None,
                false,
            ),
        }
    }

    fn summary_line(&self) -> String {
        match self.steps.last() {
            Some(step) => format!(
                "{count} step(s); latest {label} on {copy} — recovery: {recovery}",
                count = self.steps.len(),
                label = step.label.label(),
                copy = step.copy_class.label(),
                recovery = step.recovery_action.label(),
            ),
            None => "wedge not yet initialised".to_owned(),
        }
    }

    fn validate_invariants(&self) -> Vec<ManagedLifecycleInvariantRow> {
        let mut rows = Vec::new();
        let mut seen_ready_or_degraded = false;
        let mut seen_pause = false;
        for (index, step) in self.steps.iter().enumerate() {
            if !step.authority_lineage.is_complete() {
                rows.push(ManagedLifecycleInvariantRow::new(
                    ManagedLifecycleInvariantViolation::MissingAuthorityLineage,
                    Some(&step.step_id),
                ));
            }
            if matches!(step.copy_class, WorkspaceCopyClass::NotYetAdmittedCopyClass)
                && !matches!(
                    step.label,
                    ManagedLifecycleLabelClass::Authenticating
                        | ManagedLifecycleLabelClass::Connecting
                        | ManagedLifecycleLabelClass::Warming
                        | ManagedLifecycleLabelClass::Closed
                )
            {
                rows.push(ManagedLifecycleInvariantRow::new(
                    ManagedLifecycleInvariantViolation::CopyClassMissing,
                    Some(&step.step_id),
                ));
            }
            if step.label.is_degraded_posture()
                && matches!(step.recovery_action, RecoveryActionClass::None)
            {
                rows.push(ManagedLifecycleInvariantRow::new(
                    ManagedLifecycleInvariantViolation::MissingRecoveryActionForDegradedState,
                    Some(&step.step_id),
                ));
            }
            // Label / copy-class consistency.
            let expected = expected_copy_class_for(step.label);
            if let Some(expected_class) = expected {
                if step.copy_class != expected_class
                    && !(matches!(
                        step.label,
                        ManagedLifecycleLabelClass::Reconnecting
                            | ManagedLifecycleLabelClass::Closed
                    ) && step.copy_class != WorkspaceCopyClass::NotYetAdmittedCopyClass)
                {
                    rows.push(ManagedLifecycleInvariantRow::new(
                        ManagedLifecycleInvariantViolation::LabelAndCopyClassDisagree,
                        Some(&step.step_id),
                    ));
                }
            }
            // Lifecycle ordering: catch direct patches of the step list.
            if index > 0 {
                let prior = self.steps[index - 1].label;
                if !is_admissible_transition(prior, step.label) {
                    rows.push(ManagedLifecycleInvariantRow::new(
                        ManagedLifecycleInvariantViolation::SkippedTransition,
                        Some(&step.step_id),
                    ));
                }
            }
            // Reconnect / reprovision pre-conditions on the rendered card.
            if matches!(step.label, ManagedLifecycleLabelClass::Reconnecting)
                && !seen_ready_or_degraded
            {
                rows.push(ManagedLifecycleInvariantRow::new(
                    ManagedLifecycleInvariantViolation::ReconnectingWithoutPriorConnection,
                    Some(&step.step_id),
                ));
            }
            if matches!(step.label, ManagedLifecycleLabelClass::Reprovisioning) && !seen_pause {
                rows.push(ManagedLifecycleInvariantRow::new(
                    ManagedLifecycleInvariantViolation::ReprovisioningWithoutPriorPause,
                    Some(&step.step_id),
                ));
            }
            if matches!(
                step.label,
                ManagedLifecycleLabelClass::Ready | ManagedLifecycleLabelClass::ReadOnlyDegraded
            ) {
                seen_ready_or_degraded = true;
            }
            if matches!(
                step.label,
                ManagedLifecycleLabelClass::Suspended
                    | ManagedLifecycleLabelClass::ReadOnlyDegraded
            ) {
                seen_pause = true;
            }
        }
        rows
    }
}

const fn expected_copy_class_for(label: ManagedLifecycleLabelClass) -> Option<WorkspaceCopyClass> {
    match label {
        ManagedLifecycleLabelClass::Authenticating => {
            Some(WorkspaceCopyClass::NotYetAdmittedCopyClass)
        }
        ManagedLifecycleLabelClass::Ready => Some(WorkspaceCopyClass::LiveEnvironment),
        ManagedLifecycleLabelClass::ReadOnlyDegraded => {
            Some(WorkspaceCopyClass::SnapshotOnlyView)
        }
        ManagedLifecycleLabelClass::Suspended => Some(WorkspaceCopyClass::SuspendedWorkspace),
        ManagedLifecycleLabelClass::Reprovisioning => {
            Some(WorkspaceCopyClass::FreshReprovisionedCopy)
        }
        ManagedLifecycleLabelClass::SnapshotOnlyView => {
            Some(WorkspaceCopyClass::SnapshotOnlyView)
        }
        ManagedLifecycleLabelClass::Connecting
        | ManagedLifecycleLabelClass::Warming
        | ManagedLifecycleLabelClass::Reconnecting
        | ManagedLifecycleLabelClass::Closed => None,
    }
}

const fn is_admissible_transition(
    from: ManagedLifecycleLabelClass,
    to: ManagedLifecycleLabelClass,
) -> bool {
    use ManagedLifecycleLabelClass::*;
    match (from, to) {
        (Authenticating, Connecting) => true,
        (Connecting, Warming) => true,
        (Warming, Ready) => true,
        (Ready, Reconnecting) => true,
        (Ready, ReadOnlyDegraded) => true,
        (Ready, Suspended) => true,
        (Ready, SnapshotOnlyView) => true,
        (Ready, Closed) => true,
        (Reconnecting, Ready) => true,
        (Reconnecting, ReadOnlyDegraded) => true,
        (Reconnecting, Closed) => true,
        (ReadOnlyDegraded, Reconnecting) => true,
        (ReadOnlyDegraded, Suspended) => true,
        (ReadOnlyDegraded, Reprovisioning) => true,
        (ReadOnlyDegraded, SnapshotOnlyView) => true,
        (ReadOnlyDegraded, Closed) => true,
        (Suspended, Reprovisioning) => true,
        (Suspended, Closed) => true,
        (Reprovisioning, Connecting) => true,
        (Reprovisioning, Closed) => true,
        (SnapshotOnlyView, ReadOnlyDegraded) => true,
        (SnapshotOnlyView, Closed) => true,
        _ => false,
    }
}

#[cfg(test)]
impl ManagedWorkspaceLifecycleWedge {
    /// Test-only: install a forged step directly onto the wedge so the
    /// validator can run against intentionally-broken sequences chrome
    /// could only produce by patching the record list directly.
    pub(crate) fn __test_push(&mut self, step: ManagedWorkspaceLifecycleStep) {
        self.next_step_sequence = self.next_step_sequence.saturating_add(1);
        self.steps.push(step);
    }

    /// Test-only: mutable handle on the last step so test helpers can
    /// emulate a buggy caller that strips a required field after the
    /// wedge minted the step.
    pub(crate) fn __test_last_mut(&mut self) -> &mut ManagedWorkspaceLifecycleStep {
        self.steps
            .last_mut()
            .expect("__test_last_mut requires at least one step")
    }
}

fn degraded_token_from_str(token: &str) -> Option<DegradedStateToken> {
    match token {
        "Warming" => Some(DegradedStateToken::Warming),
        "Cached" => Some(DegradedStateToken::Cached),
        "Partial" => Some(DegradedStateToken::Partial),
        "Stale" => Some(DegradedStateToken::Stale),
        "Offline" => Some(DegradedStateToken::Offline),
        "PolicyBlocked" => Some(DegradedStateToken::PolicyBlocked),
        "Limited" => Some(DegradedStateToken::Limited),
        "Unsupported" => Some(DegradedStateToken::Unsupported),
        "Experimental" => Some(DegradedStateToken::Experimental),
        "RetestPending" => Some(DegradedStateToken::RetestPending),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
