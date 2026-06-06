//! Shared terminal, debugger, presenter, and support-follow control qualification.
//!
//! This module embeds the canonical shared-control packet that collaboration,
//! terminal, debugger, support, docs, and companion surfaces consume before any
//! shared shell or debugger control can render as Stable. It keeps ordinary
//! presence/follow separate from mutating control grants, proves single-driver
//! semantics, and records replay-free join/restore behavior without retaining
//! raw terminal or debugger content by default.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for shared-control qualification packets.
pub const SHARED_CONTROL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the shared-control packet.
pub const SHARED_CONTROL_RECORD_KIND: &str = "shared_terminal_debug_control_plane";

/// Repo-relative path to the canonical packet.
pub const SHARED_CONTROL_PACKET_PATH: &str =
    "artifacts/collab/m4/shared-terminal-debug-control-plane.json";

/// Embedded canonical packet JSON.
pub const SHARED_CONTROL_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/collab/m4/shared-terminal-debug-control-plane.json"
));

/// Release label rendered for a shared-control lane after qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlQualificationLabel {
    /// Stable surface with a current packet and complete control evidence.
    Stable,
    /// Preview surface that is visible but outside the Stable control contract.
    Preview,
    /// Labs surface that is experimental and visibly below Preview.
    Labs,
    /// Surface has been withdrawn or hidden for the promoted build.
    Withdrawn,
}

impl SharedControlQualificationLabel {
    /// Returns whether the label renders at the Stable cutline.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns a widening rank used for claim-ceiling checks.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Withdrawn => 0,
            Self::Labs => 1,
            Self::Preview => 2,
            Self::Stable => 3,
        }
    }
}

/// Sensitive collaboration surface governed by a control grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlSurfaceKind {
    /// Shared terminal input and shell-control surface.
    SharedTerminal,
    /// Shared debugger control surface.
    SharedDebugger,
    /// Presenter or co-presenter handoff surface.
    PresenterHandoff,
    /// Support-follow or managed support inspection surface.
    SupportFollow,
    /// Rejoin, reconnect, or restore lane for a previously shared surface.
    RestoreRejoin,
}

impl SharedControlSurfaceKind {
    /// Returns whether the surface can mutate a terminal or debugger target.
    pub const fn is_sensitive_runtime(self) -> bool {
        matches!(self, Self::SharedTerminal | Self::SharedDebugger)
    }

    /// Returns whether the surface is governed by presenter/follow state.
    pub const fn is_presenter_surface(self) -> bool {
        matches!(self, Self::PresenterHandoff | Self::SupportFollow)
    }
}

/// Client or consuming boundary disclosed on the control lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlClientBoundary {
    /// Full desktop client.
    DesktopNative,
    /// Browser companion client.
    BrowserCompanion,
    /// Mobile companion client.
    MobileCompanion,
    /// Managed support console.
    SupportConsole,
    /// External guest client with limited authority.
    ExternalGuest,
}

impl SharedControlClientBoundary {
    /// Returns whether the boundary must begin view-only for shared control.
    pub const fn requires_view_only_entry(self) -> bool {
        matches!(
            self,
            Self::BrowserCompanion
                | Self::MobileCompanion
                | Self::SupportConsole
                | Self::ExternalGuest
        )
    }
}

/// Grant lifecycle state for mutating shared control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlGrantState {
    /// No mutating grant exists for this row.
    NoGrant,
    /// A participant requested control.
    Requested,
    /// A host or moderator offered control.
    Offered,
    /// Grant was accepted but is not yet active.
    Accepted,
    /// Grant is active for exactly one driver.
    Active,
    /// Grant was revoked.
    Revoked,
    /// Grant expired.
    Expired,
    /// Grant was denied.
    Denied,
    /// Restore or rejoin renders view-only until a fresh grant is accepted.
    ViewOnlyRestore,
}

impl SharedControlGrantState {
    /// Returns whether the row currently holds mutating authority.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the row represents an explicit grant path.
    pub const fn is_grant_path(self) -> bool {
        matches!(self, Self::Offered | Self::Accepted | Self::Active)
    }
}

/// Mutating action scope covered by a control grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlGrantScope {
    /// Terminal keyboard input.
    TerminalInput,
    /// Terminal paste commit.
    TerminalPaste,
    /// Terminal signal send, such as interrupt or terminate.
    SignalSend,
    /// Terminal resize propagation.
    TerminalResize,
    /// Debugger step, continue, pause, or stop.
    DebugStepContinue,
    /// Breakpoint edit.
    DebugBreakpointEdit,
    /// Debug evaluate or watch-expression execution.
    DebugEvaluate,
    /// Attach to a live process.
    DebugAttachLiveProcess,
    /// Presenter or co-presenter handoff.
    PresenterHandoff,
    /// Follow moderation, waypoint, or agenda overlay control.
    FollowModeration,
}

/// High-risk guardrail attached to a grant or attempted action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlGuardrail {
    /// Large or shell-like paste requires confirmation.
    HighRiskPasteReview,
    /// Secret-like content is redacted or denied before commit.
    SecretDetection,
    /// Clipboard bridge is visible and policy-gated.
    ClipboardBridgePolicy,
    /// Signal sends require an explicit confirmation or approval path.
    SignalSendApproval,
    /// Debug-evaluate requests require step-up or approval.
    DebugEvaluateStepUp,
    /// Environment or variable reveal is redacted or approval-gated.
    EnvironmentRevealRedaction,
    /// Attach to a live process requires separate approval.
    LiveProcessAttachApproval,
    /// Grant expiry and revocation are visible.
    VisibleExpiryAndRevoke,
}

/// Restore/rejoin posture for a shared-control row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlRestorePosture {
    /// Not a restore or rejoin row.
    NotRestore,
    /// Restored view is render-only; live control requires a fresh grant.
    ViewOnlyFreshGrantRequired,
    /// Restored transcript or state is evidence-only.
    EvidenceOnlyNoLiveControl,
    /// Participant has intentionally broken away from follow state.
    BreakawayViewOnly,
}

impl SharedControlRestorePosture {
    /// Returns whether restore/rejoin must be view-only.
    pub const fn requires_view_only(self) -> bool {
        !matches!(self, Self::NotRestore)
    }
}

/// Retention posture for audit and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlRetentionProfile {
    /// Live-only membership and control state.
    LiveOnly,
    /// Metadata-only audit of grant and handoff edges.
    MetadataAudit,
    /// Redacted transcript or debug evidence retained under session policy.
    RedactedEvidence,
    /// Elevated support evidence with explicit support consent.
    SupportEvidence,
}

impl SharedControlRetentionProfile {
    /// Returns whether raw runtime content would require explicit proof.
    pub const fn permits_raw_content_by_default(self) -> bool {
        false
    }
}

/// UI state for presenter/follow lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlFollowState {
    /// Follow state does not apply.
    NotApplicable,
    /// Viewer follows the presenter.
    FollowingPresenter,
    /// Viewer intentionally browsed independently.
    Breakaway,
    /// Viewer may return to the presenter.
    ReturnAvailable,
    /// Follow is degraded on the current client.
    FollowDegraded,
}

/// Event kind in the control-holder lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedControlLineageEventKind {
    /// Control grant was issued.
    GrantIssued,
    /// Grant was accepted.
    GrantAccepted,
    /// A single driver became active.
    DriverActivated,
    /// Handoff was requested.
    HandoffRequested,
    /// Handoff was accepted.
    HandoffAccepted,
    /// Driver broke away from presenter/follow state.
    Breakaway,
    /// Participant joined or restored as view-only.
    ViewOnlyJoin,
    /// Grant was revoked.
    GrantRevoked,
    /// Grant expired.
    GrantExpired,
    /// Guardrail denied or blocked a high-risk action.
    GuardrailDenied,
}

/// One lineage event used to reconstruct control-holder history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedControlLineageEvent {
    /// Stable event id.
    pub event_id: String,
    /// Event kind.
    pub event_kind: SharedControlLineageEventKind,
    /// Actor associated with the event.
    pub actor_ref: String,
    /// Previous control holder when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_holder_ref: Option<String>,
    /// New control holder when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_holder_ref: Option<String>,
    /// Target context at the time of the event.
    pub target_context_ref: String,
    /// UTC timestamp or deterministic fixture timestamp.
    pub observed_at: String,
}

impl SharedControlLineageEvent {
    fn complete(&self) -> bool {
        !self.event_id.trim().is_empty()
            && !self.actor_ref.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
            && !self.observed_at.trim().is_empty()
    }
}

/// One visible shared-control lane qualified by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedControlGrantRecord {
    /// Stable control-lane id.
    pub control_id: String,
    /// Human-readable title.
    pub title: String,
    /// Surface governed by the row.
    pub surface_kind: SharedControlSurfaceKind,
    /// Whether the promoted build exposes this lane.
    pub promoted_build_surface: bool,
    /// Claimed label before qualification.
    pub claim_label: SharedControlQualificationLabel,
    /// Label after qualification or downgrade.
    pub displayed_label: SharedControlQualificationLabel,
    /// Client boundary consuming this row.
    pub client_boundary: SharedControlClientBoundary,
    /// Target workspace, terminal, debugger, or support context.
    pub target_context_ref: String,
    /// Read-only view stream reference.
    pub view_stream_ref: String,
    /// Separate control channel reference.
    pub control_channel_ref: String,
    /// Presence or follow channel reference.
    pub presence_channel_ref: String,
    /// True when control transport is separated from presence/follow transport.
    pub transport_separated: bool,
    /// Current grant state.
    pub grant_state: SharedControlGrantState,
    /// Mutating scopes granted for this row.
    pub grant_scope: Vec<SharedControlGrantScope>,
    /// Issuer of the grant, when a grant exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_issuer_ref: Option<String>,
    /// Accepter of the grant, when a grant is accepted or active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_accepter_ref: Option<String>,
    /// Current driver, when control is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_driver_ref: Option<String>,
    /// Current presenter, moderator, or support presenter when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presenter_ref: Option<String>,
    /// Expiry for the grant or `session_end`.
    pub grant_expiry: String,
    /// Visible revoke action id or label.
    pub revoke_action: String,
    /// True when the row proves only one active driver.
    pub single_active_driver: bool,
    /// Restore or rejoin posture.
    pub restore_posture: SharedControlRestorePosture,
    /// True when join, restore, and reconnect cannot replay prior actions.
    pub no_replay_on_join: bool,
    /// True only if prior input, signals, or debugger actions were replayed.
    pub prior_actions_replayed: bool,
    /// Guardrails declared for the row.
    pub guardrails: Vec<SharedControlGuardrail>,
    /// True when high-risk actions require step-up or a separate approval.
    pub high_risk_actions_require_step_up: bool,
    /// Retention profile for audit and exports.
    pub retention_profile: SharedControlRetentionProfile,
    /// True only when raw terminal or debugger content is retained.
    pub raw_runtime_content_retained: bool,
    /// Presenter/follow state.
    pub follow_state: SharedControlFollowState,
    /// Driver and presenter lineage.
    pub lineage: Vec<SharedControlLineageEvent>,
    /// Indicators the UI must show.
    pub ui_indicators: Vec<String>,
    /// Participant actions shown on the surface.
    pub participant_actions: Vec<String>,
    /// Destinations that ingest this row rather than cloned text.
    pub projection_destinations: Vec<String>,
    /// Support/export refs that reconstruct the control lane.
    pub support_export_refs: Vec<String>,
    /// Proof and fixture refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

impl SharedControlGrantRecord {
    fn evidence_is_green(&self) -> bool {
        !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.support_export_refs.is_empty()
    }

    fn channel_separation_is_proven(&self) -> bool {
        self.transport_separated
            && !self.view_stream_ref.trim().is_empty()
            && !self.control_channel_ref.trim().is_empty()
            && !self.presence_channel_ref.trim().is_empty()
            && self.control_channel_ref != self.presence_channel_ref
    }

    fn active_grant_is_complete(&self) -> bool {
        !self.grant_state.is_active()
            || (self.single_active_driver
                && self.current_driver_ref.is_some()
                && self.grant_issuer_ref.is_some()
                && self.grant_accepter_ref.is_some()
                && !self.grant_scope.is_empty()
                && !self.grant_expiry.trim().is_empty()
                && !self.revoke_action.trim().is_empty()
                && self
                    .lineage
                    .iter()
                    .any(|event| event.event_kind == SharedControlLineageEventKind::GrantAccepted)
                && self.lineage.iter().any(|event| {
                    event.event_kind == SharedControlLineageEventKind::DriverActivated
                }))
    }

    fn restore_is_replay_free(&self) -> bool {
        self.no_replay_on_join
            && !self.prior_actions_replayed
            && (!self.restore_posture.requires_view_only() || !self.grant_state.is_active())
    }

    fn boundary_cannot_silently_widen(&self) -> bool {
        !self.client_boundary.requires_view_only_entry()
            || !self.grant_state.is_active()
            || !self.displayed_label.is_stable()
    }

    fn guardrails_cover_surface(&self) -> bool {
        let guardrails: BTreeSet<SharedControlGuardrail> =
            self.guardrails.iter().copied().collect();
        let common = guardrails.contains(&SharedControlGuardrail::VisibleExpiryAndRevoke);
        match self.surface_kind {
            SharedControlSurfaceKind::SharedTerminal => {
                common
                    && guardrails.contains(&SharedControlGuardrail::HighRiskPasteReview)
                    && guardrails.contains(&SharedControlGuardrail::SecretDetection)
                    && guardrails.contains(&SharedControlGuardrail::ClipboardBridgePolicy)
                    && guardrails.contains(&SharedControlGuardrail::SignalSendApproval)
            }
            SharedControlSurfaceKind::SharedDebugger => {
                common
                    && guardrails.contains(&SharedControlGuardrail::DebugEvaluateStepUp)
                    && guardrails.contains(&SharedControlGuardrail::EnvironmentRevealRedaction)
                    && guardrails.contains(&SharedControlGuardrail::LiveProcessAttachApproval)
            }
            SharedControlSurfaceKind::PresenterHandoff
            | SharedControlSurfaceKind::SupportFollow
            | SharedControlSurfaceKind::RestoreRejoin => common,
        }
    }

    fn presenter_truth_is_visible(&self) -> bool {
        !self.surface_kind.is_presenter_surface()
            || (self.presenter_ref.is_some()
                && self.follow_state != SharedControlFollowState::NotApplicable
                && self.ui_indicators.iter().any(|indicator| {
                    indicator.contains("presenter") || indicator.contains("follow")
                }))
    }

    fn ui_truth_is_complete(&self) -> bool {
        let indicators: BTreeSet<&str> = self.ui_indicators.iter().map(String::as_str).collect();
        indicators.contains("target_context")
            && indicators.contains("control_holder_or_view_only")
            && indicators.contains("grant_expiry_or_restore_state")
            && indicators.contains("paste_secret_guardrails")
            && !self.participant_actions.is_empty()
    }

    fn projection_truth_is_complete(&self) -> bool {
        let projections: BTreeSet<&str> = self
            .projection_destinations
            .iter()
            .map(String::as_str)
            .collect();
        projections.contains("desktop")
            && projections.contains("browser_companion")
            && projections.contains("mobile_follow")
            && projections.contains("docs_help")
            && projections.contains("support_export")
    }

    fn raw_content_retention_is_allowed(&self) -> bool {
        !self.raw_runtime_content_retained
            || self.retention_profile.permits_raw_content_by_default()
    }
}

/// Summary counts for the canonical shared-control packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedControlSummary {
    /// Total exposed rows.
    pub exposed_lane_count: usize,
    /// Rows rendering at Stable.
    pub stable_lane_count: usize,
    /// Rows narrowed below Stable.
    pub downgraded_lane_count: usize,
    /// Active driver rows.
    pub active_driver_lane_count: usize,
    /// Rows proving view-only restore or join behavior.
    pub replay_free_restore_lane_count: usize,
    /// Rows proving browser/mobile/support view-only boundaries.
    pub boundary_view_only_lane_count: usize,
}

/// Canonical packet for shared terminal/debugger control qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedTerminalDebugControlPlane {
    /// Packet schema version.
    pub schema_version: u32,
    /// Packet record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable artifact reference.
    pub artifact_ref: String,
    /// User-facing documentation reference.
    pub docs_ref: String,
    /// JSON Schema reference.
    pub schema_ref: String,
    /// Fixture directory reference.
    pub fixture_ref: String,
    /// Shared-control rows.
    pub control_grants: Vec<SharedControlGrantRecord>,
    /// Summary counts.
    pub summary: SharedControlSummary,
}

impl SharedTerminalDebugControlPlane {
    /// Recomputes the summary from control-lane state.
    pub fn computed_summary(&self) -> SharedControlSummary {
        let exposed: Vec<&SharedControlGrantRecord> = self
            .control_grants
            .iter()
            .filter(|record| record.promoted_build_surface)
            .collect();

        SharedControlSummary {
            exposed_lane_count: exposed.len(),
            stable_lane_count: exposed
                .iter()
                .filter(|record| record.displayed_label.is_stable())
                .count(),
            downgraded_lane_count: exposed
                .iter()
                .filter(|record| !record.displayed_label.is_stable())
                .count(),
            active_driver_lane_count: exposed
                .iter()
                .filter(|record| record.grant_state.is_active())
                .count(),
            replay_free_restore_lane_count: exposed
                .iter()
                .filter(|record| record.restore_posture.requires_view_only())
                .filter(|record| record.restore_is_replay_free())
                .count(),
            boundary_view_only_lane_count: exposed
                .iter()
                .filter(|record| record.client_boundary.requires_view_only_entry())
                .filter(|record| !record.grant_state.is_active())
                .count(),
        }
    }

    /// Validates structural invariants that do not depend on wall-clock arithmetic.
    pub fn validate(&self) -> Vec<SharedControlViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SHARED_CONTROL_SCHEMA_VERSION {
            violations.push(SharedControlViolation::SchemaVersion {
                expected: SHARED_CONTROL_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != SHARED_CONTROL_RECORD_KIND {
            violations.push(SharedControlViolation::RecordKind {
                expected: SHARED_CONTROL_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut ids = BTreeSet::new();
        for record in &self.control_grants {
            if !ids.insert(record.control_id.clone()) {
                violations.push(SharedControlViolation::DuplicateControlId {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.rank() > record.claim_label.rank() {
                violations.push(SharedControlViolation::DisplayedWiderThanClaim {
                    control_id: record.control_id.clone(),
                });
            }

            if record.promoted_build_surface
                && record.displayed_label.is_stable()
                && !record.evidence_is_green()
            {
                violations.push(SharedControlViolation::StableLaneWithoutEvidence {
                    control_id: record.control_id.clone(),
                });
            }

            if record.promoted_build_surface && !record.channel_separation_is_proven() {
                violations.push(SharedControlViolation::ControlChannelNotSeparated {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.active_grant_is_complete() {
                violations.push(SharedControlViolation::IncompleteActiveGrant {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.restore_is_replay_free() {
                violations.push(SharedControlViolation::ReplayOrHiddenRestoreAuthority {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.boundary_cannot_silently_widen() {
                violations.push(SharedControlViolation::BoundaryCanSilentlyWiden {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.guardrails_cover_surface() {
                violations.push(SharedControlViolation::MissingGuardrailCoverage {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.presenter_truth_is_visible() {
                violations.push(SharedControlViolation::MissingPresenterTruth {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.ui_truth_is_complete() {
                violations.push(SharedControlViolation::MissingUiTruth {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.projection_truth_is_complete() {
                violations.push(SharedControlViolation::MissingProjectionTruth {
                    control_id: record.control_id.clone(),
                });
            }

            if record.displayed_label.is_stable() && !record.raw_content_retention_is_allowed() {
                violations.push(SharedControlViolation::RawContentRetainedWithoutProfile {
                    control_id: record.control_id.clone(),
                });
            }

            if record
                .lineage
                .iter()
                .any(|lineage_event| !lineage_event.complete())
            {
                violations.push(SharedControlViolation::IncompleteLineageEvent {
                    control_id: record.control_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(SharedControlViolation::SummaryMismatch {
                expected: self.computed_summary(),
                actual: self.summary.clone(),
            });
        }

        violations
    }
}

/// Validation error for shared-control qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedControlViolation {
    /// Packet schema version differs from the supported version.
    SchemaVersion { expected: u32, actual: u32 },
    /// Packet record kind differs from the supported kind.
    RecordKind { expected: String, actual: String },
    /// Control id appears more than once.
    DuplicateControlId { control_id: String },
    /// Displayed label is wider than the claim label.
    DisplayedWiderThanClaim { control_id: String },
    /// Stable lane lacks evidence and support-export refs.
    StableLaneWithoutEvidence { control_id: String },
    /// Control channel is not separated from presence/follow transport.
    ControlChannelNotSeparated { control_id: String },
    /// Active grant lacks single-driver, holder, scope, expiry, or lineage truth.
    IncompleteActiveGrant { control_id: String },
    /// Join, restore, or reconnect can replay actions or resume hidden authority.
    ReplayOrHiddenRestoreAuthority { control_id: String },
    /// Browser, mobile, support, or guest boundary can widen silently.
    BoundaryCanSilentlyWiden { control_id: String },
    /// Required paste, secret, signal, debug, or expiry guardrail is absent.
    MissingGuardrailCoverage { control_id: String },
    /// Presenter/follow row lacks presenter or follow-state truth.
    MissingPresenterTruth { control_id: String },
    /// UI cannot show holder/view-only, target, restore, and guardrail state.
    MissingUiTruth { control_id: String },
    /// Stable lane is not projected into all required consuming surfaces.
    MissingProjectionTruth { control_id: String },
    /// Raw runtime content is retained without an explicit permissive profile.
    RawContentRetainedWithoutProfile { control_id: String },
    /// Driver lineage contains an incomplete event.
    IncompleteLineageEvent { control_id: String },
    /// Summary block drifted from control-lane state.
    SummaryMismatch {
        expected: SharedControlSummary,
        actual: SharedControlSummary,
    },
}

impl fmt::Display for SharedControlViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for SharedControlViolation {}

/// Parses the checked-in shared-control qualification packet.
pub fn current_shared_terminal_debug_control_plane(
) -> Result<SharedTerminalDebugControlPlane, Box<dyn Error + Send + Sync>> {
    Ok(serde_json::from_str(SHARED_CONTROL_PACKET_JSON)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_packet_is_valid() {
        let packet = current_shared_terminal_debug_control_plane()
            .expect("canonical shared-control packet parses");

        assert_eq!(packet.validate(), Vec::new());
    }

    #[test]
    fn restore_replay_blocks_stable() {
        let mut packet = current_shared_terminal_debug_control_plane()
            .expect("canonical shared-control packet parses");
        let restore = packet
            .control_grants
            .iter_mut()
            .find(|record| {
                record.displayed_label.is_stable() && record.restore_posture.requires_view_only()
            })
            .expect("fixture includes a restore row");

        restore.prior_actions_replayed = true;

        assert!(packet.validate().iter().any(|violation| matches!(
            violation,
            SharedControlViolation::ReplayOrHiddenRestoreAuthority { .. }
        )));
    }

    #[test]
    fn active_driver_requires_lineage() {
        let mut packet = current_shared_terminal_debug_control_plane()
            .expect("canonical shared-control packet parses");
        let active = packet
            .control_grants
            .iter_mut()
            .find(|record| record.grant_state.is_active())
            .expect("fixture includes an active driver row");

        active.lineage.clear();

        assert!(packet.validate().iter().any(|violation| matches!(
            violation,
            SharedControlViolation::IncompleteActiveGrant { .. }
        )));
    }
}
