//! Bounded host-boundary cue and target-identity-handoff wedge on the
//! certified terminal pane.
//!
//! ## What the wedge is for
//!
//! Every run-capable lane in M1 must answer the same question before a user
//! acts: *which target am I about to run on, where did this lane come from,
//! and is the local desktop boundary crossed?* The shared
//! [`crate::badges::target_origin`] projection answers it for a single
//! snapshot. This wedge extends that truth across a sequence of session
//! events on **one certified wedge** — the bottom-panel terminal pane — so
//! the chrome can keep host-boundary cues and target identity legible while
//! a session opens, hands off to a different target, drops transport,
//! reconnects, is quarantined, or is closed.
//!
//! ## Why a per-event record, not just a per-snapshot badge
//!
//! The protected walk (open terminal, task, debug, provider) and the
//! failure drill ("hand off across host or target boundaries and confirm
//! the wedge preserves source/target identity instead of flattening them")
//! both need a record that *names the prior identity* alongside the
//! current one. A static badge cannot prove the wedge did not flatten the
//! handoff; a typed step record can. Each
//! [`HostBoundaryCueHandoffStep`] carries:
//!
//! - a closed [`HandoffKind`] (initial_open / target_handoff /
//!   reconnected_same_identity / transport_lost / quarantined /
//!   policy_blocked / closed),
//! - the *source* [`TargetIdentitySnapshot`] when applicable (always
//!   present on a target handoff or reconnect),
//! - the *current* [`TargetIdentitySnapshot`],
//! - the typed [`HostBoundaryCue`] from the shared badge vocabulary, plus
//!   the visible/honesty bits,
//! - an optional [`DegradedStateToken`] mapped from the lifecycle event so
//!   the chrome never has to re-derive degraded chrome locally,
//! - an `observed_at` stamp the chrome quotes verbatim.
//!
//! ## What "boundary cues survive" means here
//!
//! The wedge enforces — through both the API and a typed
//! [`HostBoundaryInvariantViolation`] vocabulary — that the cue never
//! collapses to [`HostBoundaryCue::Hidden`] while the lane is in a
//! degraded, reconnecting, or policy-blocked state. A remote target that
//! drops transport keeps lighting `local_to_remote`; the chrome additionally
//! surfaces the typed `Offline` degraded chip rather than swapping the
//! boundary cue with it.
//!
//! ## Bounded scope (deliberately)
//!
//! - Only the bottom-panel terminal pane is the certified wedge entry
//!   point in M1. Task / debug / provider lanes consume the shared
//!   [`TargetOriginBadgeSet`] projection but do not own a handoff record
//!   here; the wedge's [`HostBoundaryClaimLimit::SingleCertifiedWedgeOnly`]
//!   row is always rendered to make this explicit.
//! - The wedge does not start, attach, or reconnect real PTY transport.
//!   It records identity through the lifecycle and the chrome runs
//!   transitions against the canonical [`aureline_terminal::PtyHost`].
//! - The wedge does not duplicate the [`TargetOriginBadge`] vocabulary;
//!   every cue token comes from the shared badge module. Forking would
//!   defeat the M01-078 truth-source guarantee.

use serde::{Deserialize, Serialize};

use aureline_runtime::ExecutionContext;
use aureline_terminal::{PtySession, PtySessionId, SessionHeader, SessionLifecycleState};

use crate::badges::target_origin::{
    BadgeEntryPoint, HostBoundaryCue, OriginBadgeClass, TargetBadgeClass, TargetOriginBadge,
};
use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried in serialized
/// [`HostBoundaryCueCardRecord`] payloads.
pub const HOST_BOUNDARY_CUE_CARD_RECORD_KIND: &str = "host_boundary_cue_card_record";

/// Schema version for the [`HostBoundaryCueCardRecord`] payload shape.
pub const HOST_BOUNDARY_CUE_CARD_SCHEMA_VERSION: u32 = 1;

/// Prototype label carried on every card. The chrome MUST quote the token
/// verbatim and MUST NOT drop the chip even when the current step lands on a
/// trusted local target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype: host-boundary cues and target-identity handoff
    /// on one certified wedge (bottom-panel terminal pane).
    M1PrototypeHostBoundaryCuesAndTargetHandoff,
}

impl PrototypeLabel {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeHostBoundaryCuesAndTargetHandoff => {
                "m1_prototype_host_boundary_cues_and_target_handoff"
            }
        }
    }

    /// Human-readable chip label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeHostBoundaryCuesAndTargetHandoff => {
                "Prototype — host-boundary cues and target-identity handoff (terminal wedge)"
            }
        }
    }
}

/// Closed handoff-kind vocabulary. Each step on the card carries exactly
/// one kind so chrome, support exports, and proof captures share the same
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffKind {
    /// First step on the card: the wedge opened a fresh session for the
    /// current target.
    InitialOpen,
    /// The wedge handed off the lane from a prior target to a new one (the
    /// canonical example: a local terminal moves to an SSH remote on the
    /// same workspace row). The source snapshot is always present and
    /// MUST differ from the current snapshot's canonical target id.
    TargetHandoff,
    /// Transport recovered against the same canonical target identity.
    /// The source snapshot is present and MUST share the canonical target
    /// id with the current snapshot.
    ReconnectedSameIdentity,
    /// Transport dropped. The header is preserved verbatim so the chrome
    /// keeps the same identity row visible while it surfaces the typed
    /// `Offline` degraded chip.
    TransportLost,
    /// Supervisor revoked the session. The chrome MUST surface
    /// `PolicyBlocked` next to the row but MUST NOT collapse the boundary
    /// cue to `Hidden`.
    Quarantined,
    /// Reachability / org policy denies the lane. The boundary cue
    /// switches to `PolicyBlocked` so the row is visibly not green.
    PolicyBlocked,
    /// Session is closing. The chrome MUST keep the boundary cue and
    /// identity visible until the consumer drops the row.
    Closed,
}

impl HandoffKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialOpen => "initial_open",
            Self::TargetHandoff => "target_handoff",
            Self::ReconnectedSameIdentity => "reconnected_same_identity",
            Self::TransportLost => "transport_lost",
            Self::Quarantined => "quarantined",
            Self::PolicyBlocked => "policy_blocked",
            Self::Closed => "closed",
        }
    }

    /// True when the kind requires a source target identity in addition to
    /// the current one.
    pub const fn requires_source(self) -> bool {
        matches!(self, Self::TargetHandoff | Self::ReconnectedSameIdentity)
    }

    /// True when the kind moves the lane into a degraded or non-interactive
    /// state. The wedge's invariants require the boundary cue to remain
    /// visible (i.e. not `Hidden`) whenever this returns `true`.
    pub const fn is_degraded_lifecycle(self) -> bool {
        matches!(
            self,
            Self::TransportLost | Self::Quarantined | Self::PolicyBlocked | Self::Closed
        )
    }
}

/// Frozen claim-limit vocabulary the chrome quotes verbatim under every
/// card. The set is intentionally small: it pins the wedge's M1 scope so
/// chrome cannot imply remote/provider parity across unrelated surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryClaimLimit {
    /// One certified wedge only — the bottom-panel terminal pane.
    SingleCertifiedWedgeOnly,
    /// No remote runtime / multi-host / fleet orchestration depth in M1.
    NoRemoteOrchestrationBreadth,
    /// Does not imply provider/auth parity across unrelated surfaces.
    NoProviderParityImplied,
    /// Records identity through the lifecycle; the wedge does not spawn,
    /// reconnect, or attach real PTY transport itself.
    NoTransportOrchestration,
}

impl HostBoundaryClaimLimit {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCertifiedWedgeOnly => "single_certified_wedge_only",
            Self::NoRemoteOrchestrationBreadth => "no_remote_orchestration_breadth",
            Self::NoProviderParityImplied => "no_provider_parity_implied",
            Self::NoTransportOrchestration => "no_transport_orchestration",
        }
    }

    /// Human-readable claim label rendered under the card.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleCertifiedWedgeOnly => {
                "One certified wedge only — bottom-panel terminal pane."
            }
            Self::NoRemoteOrchestrationBreadth => {
                "Not a remote-runtime, multi-host, or fleet orchestration platform."
            }
            Self::NoProviderParityImplied => {
                "Does not imply provider/auth parity across unrelated surfaces."
            }
            Self::NoTransportOrchestration => {
                "Records identity through the lifecycle; does not spawn or reconnect PTY transport."
            }
        }
    }

    /// Canonical M1 claim-limit set. Order is stable; chrome MUST render in
    /// this order.
    pub const fn canonical_set() -> [HostBoundaryClaimLimit; 4] {
        [
            Self::SingleCertifiedWedgeOnly,
            Self::NoRemoteOrchestrationBreadth,
            Self::NoProviderParityImplied,
            Self::NoTransportOrchestration,
        ]
    }
}

/// One claim-limit row carried on the serialized card payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryClaimLimitRow {
    pub token: String,
    pub label: String,
}

impl HostBoundaryClaimLimitRow {
    fn from_limit(limit: HostBoundaryClaimLimit) -> Self {
        Self {
            token: limit.as_str().to_owned(),
            label: limit.label().to_owned(),
        }
    }
}

/// Inspectable target-identity snapshot for one handoff step.
///
/// Every field is mirrored verbatim from the canonical upstream contracts
/// (execution context + session header) — the wedge never invents a
/// surface-local identity vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentitySnapshot {
    pub session_id: PtySessionId,
    pub workspace_id: String,
    pub target_class: TargetBadgeClass,
    pub target_class_token: String,
    pub target_label: String,
    pub canonical_target_id: String,
    pub origin_class: OriginBadgeClass,
    pub origin_class_token: String,
    pub origin_label: String,
    pub execution_context_ref: String,
    pub lifecycle_state: SessionLifecycleState,
    pub lifecycle_state_token: String,
    pub display_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    pub trust_state_token: String,
}

impl TargetIdentitySnapshot {
    fn from_context_and_header(context: &ExecutionContext, header: &SessionHeader) -> Self {
        let badge = TargetOriginBadge::project(BadgeEntryPoint::Terminal, context);
        Self {
            session_id: header.session_id.clone(),
            workspace_id: header.workspace_id.clone(),
            target_class: badge.target_class,
            target_class_token: badge.target_class_token,
            target_label: badge.target_label,
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            origin_class: badge.origin_class,
            origin_class_token: badge.origin_class_token,
            origin_label: badge.origin_label,
            execution_context_ref: context.execution_context_id.clone(),
            lifecycle_state: header.lifecycle_state,
            lifecycle_state_token: header.lifecycle_state_token.clone(),
            display_title: header.display_title.clone(),
            cwd_hint: header.cwd_hint.clone(),
            trust_state_token: badge.trust_state_token,
        }
    }
}

/// Errors the wedge raises on illegal handoff sequences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WedgeError {
    /// A record_* call landed before [`HostBoundaryCueWedge::open_initial`].
    NotInitialized,
    /// A record_* call landed after [`HostBoundaryCueWedge::record_closed`].
    AlreadyClosed,
    /// A target handoff was requested but the new canonical target id
    /// equals the prior one. Allowing this would flatten the source/target
    /// identity that the failure drill exists to defend.
    HandoffFlattensTargetIdentity {
        source_canonical_target_id: String,
        requested_canonical_target_id: String,
    },
    /// A reconnect was requested but the new canonical target id does not
    /// match the prior one. Reconnect-same-identity is the only reconnect
    /// kind in M1; a different identity would be a fresh open.
    ReconnectIdentityMismatch {
        source_canonical_target_id: String,
        requested_canonical_target_id: String,
    },
}

impl std::fmt::Display for WedgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "wedge has no initial open step"),
            Self::AlreadyClosed => write!(f, "wedge is already closed"),
            Self::HandoffFlattensTargetIdentity {
                source_canonical_target_id,
                requested_canonical_target_id,
            } => write!(
                f,
                "handoff would flatten target identity: source={source_canonical_target_id} requested={requested_canonical_target_id}",
            ),
            Self::ReconnectIdentityMismatch {
                source_canonical_target_id,
                requested_canonical_target_id,
            } => write!(
                f,
                "reconnect identity mismatch: source={source_canonical_target_id} requested={requested_canonical_target_id}",
            ),
        }
    }
}

impl std::error::Error for WedgeError {}

/// Closed invariant-violation vocabulary surfaced on the card.
///
/// Each invariant is a single typed reason; the chrome quotes the token
/// verbatim and MUST render a visible failure row when one fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryInvariantViolation {
    /// The wedge is missing the prototype-label chip.
    MissingPrototypeLabel,
    /// The canonical claim-limit set is missing or out of order.
    ClaimLimitsMissingOrOutOfOrder,
    /// A step has no source identity but the kind requires one
    /// (target_handoff / reconnected_same_identity).
    MissingSourceIdentityOnHandoff,
    /// A target-handoff step lands on the same canonical target id as its
    /// source — the wedge flattened the identity.
    HandoffFlattensTargetIdentity,
    /// A reconnect step's source and current canonical target ids
    /// disagree. Reconnect-same-identity must keep the canonical target id
    /// stable.
    ReconnectIdentityMismatch,
    /// A step is in a degraded lifecycle (transport_lost / quarantined /
    /// policy_blocked / closed) but the boundary cue collapsed to
    /// `Hidden`. The chrome rule is: degraded states never erase the
    /// boundary cue.
    BoundaryCueDisappearsInDegradedState,
    /// A step is missing the `execution_context_ref`. Every step MUST
    /// carry the upstream context id so a support export can correlate
    /// rows.
    MissingExecutionContextRef,
}

impl HostBoundaryInvariantViolation {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "missing_prototype_label",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::MissingSourceIdentityOnHandoff => "missing_source_identity_on_handoff",
            Self::HandoffFlattensTargetIdentity => "handoff_flattens_target_identity",
            Self::ReconnectIdentityMismatch => "reconnect_identity_mismatch",
            Self::BoundaryCueDisappearsInDegradedState => {
                "boundary_cue_disappears_in_degraded_state"
            }
            Self::MissingExecutionContextRef => "missing_execution_context_ref",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "Prototype label chip is missing.",
            Self::ClaimLimitsMissingOrOutOfOrder => {
                "Canonical claim-limit set missing or out of order."
            }
            Self::MissingSourceIdentityOnHandoff => {
                "Handoff step is missing its source identity snapshot."
            }
            Self::HandoffFlattensTargetIdentity => {
                "Handoff step flattens source and current target identity."
            }
            Self::ReconnectIdentityMismatch => {
                "Reconnect step's canonical target id disagrees with its source."
            }
            Self::BoundaryCueDisappearsInDegradedState => {
                "Boundary cue collapses to Hidden in a degraded lifecycle state."
            }
            Self::MissingExecutionContextRef => {
                "Step is missing the upstream execution-context reference."
            }
        }
    }
}

/// One typed invariant row rendered on the card. The chrome quotes each
/// field verbatim. The `addressable_step_id` lets a support export tie the
/// row to the offending handoff step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryInvariantRow {
    pub violation: HostBoundaryInvariantViolation,
    pub violation_token: String,
    pub violation_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addressable_step_id: Option<String>,
}

impl HostBoundaryInvariantRow {
    fn new(violation: HostBoundaryInvariantViolation, step_id: Option<&str>) -> Self {
        Self {
            violation,
            violation_token: violation.as_str().to_owned(),
            violation_label: violation.label().to_owned(),
            addressable_step_id: step_id.map(str::to_owned),
        }
    }
}

/// Serializable handoff-step row carried on the card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryCueHandoffStep {
    pub step_id: String,
    pub kind: HandoffKind,
    pub kind_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<TargetIdentitySnapshot>,
    pub current: TargetIdentitySnapshot,
    pub boundary_cue: HostBoundaryCue,
    pub boundary_cue_token: String,
    pub boundary_cue_label: String,
    pub boundary_cue_visible: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    pub honesty_marker_present: bool,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

/// Serializable card payload. The chrome renders this struct directly;
/// export and proof flows quote it verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryCueCardRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    pub workspace_id: String,
    pub wedge_id: String,
    pub entry_point: BadgeEntryPoint,
    pub entry_point_token: String,
    pub steps: Vec<HostBoundaryCueHandoffStep>,
    pub current_boundary_cue: HostBoundaryCue,
    pub current_boundary_cue_token: String,
    pub current_boundary_cue_label: String,
    pub current_boundary_cue_visible: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_degraded_token: Option<String>,
    pub has_honesty_marker: bool,
    pub claim_limits: Vec<HostBoundaryClaimLimitRow>,
    pub invariants: Vec<HostBoundaryInvariantRow>,
    pub has_invariant_violations: bool,
    pub summary_line: String,
}

impl HostBoundaryCueCardRecord {
    /// Deterministic plaintext block. Support exports and proof captures
    /// quote this verbatim — the format is stable across hosts and never
    /// bakes in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display,
        ));
        out.push_str(&format!(
            "wedge={} entry_point={} workspace={}\n",
            self.wedge_id, self.entry_point_token, self.workspace_id,
        ));
        out.push_str("steps:\n");
        for step in &self.steps {
            out.push_str(&format!(
                "  - id={} kind={} observed_at={} cue={} visible={}",
                step.step_id,
                step.kind_token,
                step.observed_at,
                step.boundary_cue_token,
                step.boundary_cue_visible,
            ));
            if let Some(token) = &step.degraded_token {
                out.push_str(&format!(" degraded={}", token));
            }
            if step.honesty_marker_present {
                out.push_str(" honesty_marker=true");
            }
            if let Some(reason) = &step.reason_code {
                out.push_str(&format!(" reason={}", reason));
            }
            out.push('\n');
            if let Some(source) = &step.source {
                out.push_str(&format!(
                    "      source: session={} target={} canonical={} lifecycle={}\n",
                    source.session_id,
                    source.target_class_token,
                    source.canonical_target_id,
                    source.lifecycle_state_token,
                ));
            }
            out.push_str(&format!(
                "      current: session={} target={} canonical={} lifecycle={} ctx={}\n",
                step.current.session_id,
                step.current.target_class_token,
                step.current.canonical_target_id,
                step.current.lifecycle_state_token,
                step.current.execution_context_ref,
            ));
        }
        out.push_str(&format!(
            "current_cue={} visible={}",
            self.current_boundary_cue_token, self.current_boundary_cue_visible,
        ));
        if let Some(token) = &self.current_degraded_token {
            out.push_str(&format!(" degraded={}", token));
        }
        out.push('\n');
        out.push_str(&format!("honesty_marker={}\n", self.has_honesty_marker));
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

    /// True when every step's boundary cue and honesty marker indicate the
    /// lane is currently visible and untainted. Used by the protected walk
    /// assertions on the trusted-local seed.
    pub fn is_clean_local_path(&self) -> bool {
        !self.has_invariant_violations && !self.has_honesty_marker
    }
}

/// Bounded host-boundary cue + target-identity-handoff wedge.
///
/// Construct with [`HostBoundaryCueWedge::new`], then drive the lifecycle:
/// `open_initial` -> any of `record_target_handoff` /
/// `record_reconnect` / `record_transport_lost` / `record_quarantined` /
/// `record_policy_blocked` -> `record_closed`. Call [`Self::card`] at any
/// point to obtain the current snapshot record.
#[derive(Debug, Clone)]
pub struct HostBoundaryCueWedge {
    workspace_id: String,
    wedge_id: String,
    entry_point: BadgeEntryPoint,
    steps: Vec<HostBoundaryCueHandoffStep>,
    closed: bool,
    next_step_sequence: u64,
}

impl HostBoundaryCueWedge {
    /// Construct a wedge for the bottom-panel terminal pane on the given
    /// workspace. The wedge id is derived from the workspace id; surfaces
    /// MAY supply a custom id via [`Self::with_wedge_id`] when more than
    /// one wedge instance is rendered side by side (e.g. fixture replay).
    pub fn new(workspace_id: impl Into<String>) -> Self {
        let ws = workspace_id.into();
        let wedge_id = format!("host_boundary_cue_wedge:{ws}");
        Self {
            workspace_id: ws,
            wedge_id,
            entry_point: BadgeEntryPoint::Terminal,
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

    /// Record the first session opening for the wedge. Returns the
    /// projected current target identity snapshot for callers that want
    /// to chain assertions without re-projecting from the context.
    pub fn open_initial(
        &mut self,
        context: &ExecutionContext,
        session: &PtySession,
        observed_at: &str,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        if self.closed {
            return Err(WedgeError::AlreadyClosed);
        }
        let current = TargetIdentitySnapshot::from_context_and_header(context, session.header());
        let badge = TargetOriginBadge::project(BadgeEntryPoint::Terminal, context);
        let step = self.mint_step(
            HandoffKind::InitialOpen,
            None,
            current,
            badge.boundary_cue,
            initial_degraded_token(session.header()),
            badge.honesty_marker_present,
            observed_at,
            None,
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a target handoff onto a new target identity. The new
    /// canonical target id MUST differ from the current one or the wedge
    /// returns [`WedgeError::HandoffFlattensTargetIdentity`].
    pub fn record_target_handoff(
        &mut self,
        new_context: &ExecutionContext,
        new_session: &PtySession,
        observed_at: &str,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        let source = self.require_current()?.clone();
        let requested =
            TargetIdentitySnapshot::from_context_and_header(new_context, new_session.header());
        if requested.canonical_target_id == source.canonical_target_id {
            return Err(WedgeError::HandoffFlattensTargetIdentity {
                source_canonical_target_id: source.canonical_target_id,
                requested_canonical_target_id: requested.canonical_target_id,
            });
        }
        let badge = TargetOriginBadge::project(BadgeEntryPoint::Terminal, new_context);
        let step = self.mint_step(
            HandoffKind::TargetHandoff,
            Some(source),
            requested,
            badge.boundary_cue,
            initial_degraded_token(new_session.header()),
            badge.honesty_marker_present,
            observed_at,
            None,
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a successful reconnect against the same canonical target
    /// identity. The wedge refuses an identity mismatch.
    pub fn record_reconnect(
        &mut self,
        context: &ExecutionContext,
        session: &PtySession,
        observed_at: &str,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        let source = self.require_current()?.clone();
        let requested = TargetIdentitySnapshot::from_context_and_header(context, session.header());
        if requested.canonical_target_id != source.canonical_target_id {
            return Err(WedgeError::ReconnectIdentityMismatch {
                source_canonical_target_id: source.canonical_target_id,
                requested_canonical_target_id: requested.canonical_target_id,
            });
        }
        let badge = TargetOriginBadge::project(BadgeEntryPoint::Terminal, context);
        // Reconnect-same-identity preserves the boundary cue from the
        // source; the chrome surfaces a Warming chip while the lane
        // restabilises.
        let step = self.mint_step(
            HandoffKind::ReconnectedSameIdentity,
            Some(source),
            requested,
            badge.boundary_cue,
            Some(DegradedStateToken::Warming),
            true,
            observed_at,
            Some("reconnected_same_identity".to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a transport-lost lifecycle. The header identity is preserved
    /// verbatim from the prior step so the chrome keeps the row addressable;
    /// the boundary cue stays visible and the chrome surfaces an `Offline`
    /// degraded chip.
    pub fn record_transport_lost(
        &mut self,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        let prior = self.require_current()?.clone();
        let mut current = prior.clone();
        current.lifecycle_state = SessionLifecycleState::LostTransport;
        current.lifecycle_state_token = SessionLifecycleState::LostTransport.as_str().to_owned();
        let boundary_cue = self.last_visible_or_current_cue();
        let step = self.mint_step(
            HandoffKind::TransportLost,
            Some(prior),
            current,
            boundary_cue,
            Some(DegradedStateToken::Offline),
            true,
            observed_at,
            reason_code.map(str::to_owned),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a quarantine lifecycle.
    pub fn record_quarantined(
        &mut self,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        let prior = self.require_current()?.clone();
        let mut current = prior.clone();
        current.lifecycle_state = SessionLifecycleState::Quarantined;
        current.lifecycle_state_token = SessionLifecycleState::Quarantined.as_str().to_owned();
        let boundary_cue = self.last_visible_or_current_cue();
        let step = self.mint_step(
            HandoffKind::Quarantined,
            Some(prior),
            current,
            boundary_cue,
            Some(DegradedStateToken::PolicyBlocked),
            true,
            observed_at,
            Some(reason_code.to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a policy-blocked lifecycle. Switches the boundary cue to
    /// `PolicyBlocked` so the row is visibly not green.
    pub fn record_policy_blocked(
        &mut self,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        let prior = self.require_current()?.clone();
        let current = prior.clone();
        let step = self.mint_step(
            HandoffKind::PolicyBlocked,
            Some(prior),
            current,
            HostBoundaryCue::PolicyBlocked,
            Some(DegradedStateToken::PolicyBlocked),
            true,
            observed_at,
            Some(reason_code.to_owned()),
        );
        self.steps.push(step);
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Record a closed lifecycle. The wedge stops accepting further
    /// record_* calls.
    pub fn record_closed(
        &mut self,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<&HostBoundaryCueHandoffStep, WedgeError> {
        let prior = self.require_current()?.clone();
        let mut current = prior.clone();
        current.lifecycle_state = SessionLifecycleState::Closed;
        current.lifecycle_state_token = SessionLifecycleState::Closed.as_str().to_owned();
        let boundary_cue = self.last_visible_or_current_cue();
        let step = self.mint_step(
            HandoffKind::Closed,
            Some(prior),
            current,
            boundary_cue,
            Some(DegradedStateToken::Limited),
            true,
            observed_at,
            reason_code.map(str::to_owned),
        );
        self.steps.push(step);
        self.closed = true;
        Ok(self.steps.last().expect("step pushed"))
    }

    /// Materialise the current card record.
    pub fn card(&self) -> HostBoundaryCueCardRecord {
        let label = PrototypeLabel::M1PrototypeHostBoundaryCuesAndTargetHandoff;
        let claim_limits: Vec<HostBoundaryClaimLimitRow> = HostBoundaryClaimLimit::canonical_set()
            .into_iter()
            .map(HostBoundaryClaimLimitRow::from_limit)
            .collect();
        let invariants = self.validate_invariants();
        let has_invariant_violations = !invariants.is_empty();
        let (current_cue, current_visible, current_degraded, honesty) = self.current_state();
        let summary_line = self.summary_line();
        HostBoundaryCueCardRecord {
            record_kind: HOST_BOUNDARY_CUE_CARD_RECORD_KIND.to_owned(),
            schema_version: HOST_BOUNDARY_CUE_CARD_SCHEMA_VERSION,
            prototype_label_token: label.as_str().to_owned(),
            prototype_label_display: label.label().to_owned(),
            workspace_id: self.workspace_id.clone(),
            wedge_id: self.wedge_id.clone(),
            entry_point: self.entry_point,
            entry_point_token: self.entry_point.as_str().to_owned(),
            steps: self.steps.clone(),
            current_boundary_cue: current_cue,
            current_boundary_cue_token: current_cue.as_str().to_owned(),
            current_boundary_cue_label: current_cue.label().to_owned(),
            current_boundary_cue_visible: current_visible,
            current_degraded_token: current_degraded.map(|t| t.token().to_owned()),
            has_honesty_marker: honesty,
            claim_limits,
            invariants,
            has_invariant_violations,
            summary_line,
        }
    }

    /// Iterate steps in record order.
    pub fn steps(&self) -> &[HostBoundaryCueHandoffStep] {
        &self.steps
    }

    fn require_current(&self) -> Result<&TargetIdentitySnapshot, WedgeError> {
        if self.closed {
            return Err(WedgeError::AlreadyClosed);
        }
        match self.steps.last() {
            Some(step) => Ok(&step.current),
            None => Err(WedgeError::NotInitialized),
        }
    }

    fn last_visible_or_current_cue(&self) -> HostBoundaryCue {
        // Prefer the most recent visible cue so degraded lifecycle states
        // never erase the boundary truth. If every prior step happens to be
        // `Hidden` (trusted local desktop), keep that — the chrome still
        // surfaces the typed degraded chip on the row.
        for step in self.steps.iter().rev() {
            if step.boundary_cue.is_visible() {
                return step.boundary_cue;
            }
        }
        self.steps
            .last()
            .map(|step| step.boundary_cue)
            .unwrap_or(HostBoundaryCue::Hidden)
    }

    fn mint_step(
        &mut self,
        kind: HandoffKind,
        source: Option<TargetIdentitySnapshot>,
        current: TargetIdentitySnapshot,
        boundary_cue: HostBoundaryCue,
        degraded: Option<DegradedStateToken>,
        honesty_marker_present: bool,
        observed_at: &str,
        reason_code: Option<String>,
    ) -> HostBoundaryCueHandoffStep {
        let seq = self.next_step_sequence;
        self.next_step_sequence = self.next_step_sequence.saturating_add(1);
        let step_id = format!("{}:{}:{}", self.wedge_id, kind.as_str(), seq);
        HostBoundaryCueHandoffStep {
            step_id,
            kind,
            kind_token: kind.as_str().to_owned(),
            source,
            current,
            boundary_cue,
            boundary_cue_token: boundary_cue.as_str().to_owned(),
            boundary_cue_label: boundary_cue.label().to_owned(),
            boundary_cue_visible: boundary_cue.is_visible(),
            degraded_token: degraded.map(|t| t.token().to_owned()),
            honesty_marker_present,
            observed_at: observed_at.to_owned(),
            reason_code,
        }
    }

    fn current_state(&self) -> (HostBoundaryCue, bool, Option<DegradedStateToken>, bool) {
        match self.steps.last() {
            Some(step) => {
                let degraded = step
                    .degraded_token
                    .as_deref()
                    .and_then(degraded_token_from_str);
                (
                    step.boundary_cue,
                    step.boundary_cue_visible,
                    degraded,
                    step.honesty_marker_present,
                )
            }
            None => (HostBoundaryCue::Hidden, false, None, false),
        }
    }

    fn summary_line(&self) -> String {
        match self.steps.last() {
            Some(step) => {
                let cue = step.boundary_cue.label();
                let kind = step.kind_token.as_str();
                let canonical = step.current.canonical_target_id.as_str();
                format!(
                    "{count} step(s); latest {kind} on {canonical} — {cue}",
                    count = self.steps.len(),
                    kind = kind,
                    canonical = canonical,
                    cue = cue,
                )
            }
            None => "wedge not yet initialised".to_owned(),
        }
    }

    fn validate_invariants(&self) -> Vec<HostBoundaryInvariantRow> {
        let mut rows = Vec::new();
        // The prototype label and the canonical claim-limit set are always
        // present at the card layer; the checks below catch the wedge state
        // that would lie on a per-step basis.
        for step in &self.steps {
            if step.current.execution_context_ref.is_empty() {
                rows.push(HostBoundaryInvariantRow::new(
                    HostBoundaryInvariantViolation::MissingExecutionContextRef,
                    Some(&step.step_id),
                ));
            }
            if step.kind.requires_source() && step.source.is_none() {
                rows.push(HostBoundaryInvariantRow::new(
                    HostBoundaryInvariantViolation::MissingSourceIdentityOnHandoff,
                    Some(&step.step_id),
                ));
                continue;
            }
            if matches!(step.kind, HandoffKind::TargetHandoff) {
                if let Some(source) = &step.source {
                    if source.canonical_target_id == step.current.canonical_target_id {
                        rows.push(HostBoundaryInvariantRow::new(
                            HostBoundaryInvariantViolation::HandoffFlattensTargetIdentity,
                            Some(&step.step_id),
                        ));
                    }
                }
            }
            if matches!(step.kind, HandoffKind::ReconnectedSameIdentity) {
                if let Some(source) = &step.source {
                    if source.canonical_target_id != step.current.canonical_target_id {
                        rows.push(HostBoundaryInvariantRow::new(
                            HostBoundaryInvariantViolation::ReconnectIdentityMismatch,
                            Some(&step.step_id),
                        ));
                    }
                }
            }
            if step.kind.is_degraded_lifecycle()
                && matches!(step.boundary_cue, HostBoundaryCue::Hidden)
                && step.source.as_ref().map_or(false, |source| {
                    matches!(
                        source.target_class,
                        TargetBadgeClass::RemoteHost
                            | TargetBadgeClass::RemoteWorkspaceVm
                            | TargetBadgeClass::ManagedWorkspace
                            | TargetBadgeClass::PrebuildRuntime
                            | TargetBadgeClass::AiSandbox
                            | TargetBadgeClass::LocalContainer
                            | TargetBadgeClass::Devcontainer
                            | TargetBadgeClass::NotebookKernelRemote
                    )
                })
            {
                // A degraded lifecycle step on a non-local prior target MUST
                // keep the boundary cue visible; collapsing to Hidden would
                // erase the very truth the wedge exists to protect.
                rows.push(HostBoundaryInvariantRow::new(
                    HostBoundaryInvariantViolation::BoundaryCueDisappearsInDegradedState,
                    Some(&step.step_id),
                ));
            }
        }
        rows
    }
}

const fn initial_degraded_token(header: &SessionHeader) -> Option<DegradedStateToken> {
    match header.lifecycle_state {
        SessionLifecycleState::Requested | SessionLifecycleState::Starting => {
            Some(DegradedStateToken::Warming)
        }
        SessionLifecycleState::LostTransport => Some(DegradedStateToken::Offline),
        SessionLifecycleState::Quarantined => Some(DegradedStateToken::PolicyBlocked),
        SessionLifecycleState::Closed => Some(DegradedStateToken::Limited),
        SessionLifecycleState::Active | SessionLifecycleState::ReconnectedSameIdentity => None,
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
