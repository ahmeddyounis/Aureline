//! Unit tests for the bounded host-boundary cue / target-identity-handoff
//! wedge.
//!
//! These tests cover the protected walk (trusted local terminal seed →
//! every step lands `Hidden` and clean) plus the named failure drill
//! (handoff from local to a remote SSH target → the wedge preserves source
//! identity, lights `local_to_remote`, and refuses to flatten). Adjacent
//! drills cover transport-loss + reconnect, quarantine, policy-blocked,
//! and the typed `HandoffFlattensTargetIdentity` invariant.

use std::path::Path;

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, ScopeClass, TargetClass, TrustState,
};
use aureline_terminal::{
    HostClass, OpenSessionRequest, PtyHost, PtySession, PtySessionId, TerminalTrustState,
};
use serde::Deserialize;

use super::*;

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "ws-host-boundary".to_owned(),
        profile_id: Some("prof.default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:ws-host-boundary:seed".to_owned(),
            capsule_hash: "sha256:seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "seed-0".to_owned(),
    })
}

fn open_local_session(host: &mut PtyHost) -> PtySessionId {
    host.open_session(OpenSessionRequest {
        workspace_id: "ws-host-boundary",
        host_class: HostClass::HostDesktop,
        display_title: "zsh",
        cwd_hint: Some("~/code/aureline"),
        execution_context_ref: "execution_context.local_desktop.workspace_root",
        trust_state: TerminalTrustState::Trusted,
        observed_at: "mono:0",
    })
}

fn open_remote_session(host: &mut PtyHost) -> PtySessionId {
    host.open_session(OpenSessionRequest {
        workspace_id: "ws-host-boundary",
        host_class: HostClass::RemoteAgentPrimary,
        display_title: "ssh prod",
        cwd_hint: Some("/srv/code"),
        execution_context_ref: "execution_context.ssh_remote.prod_host",
        trust_state: TerminalTrustState::Trusted,
        observed_at: "mono:1",
    })
}

fn open_container_session(host: &mut PtyHost) -> PtySessionId {
    host.open_session(OpenSessionRequest {
        workspace_id: "ws-host-boundary",
        host_class: HostClass::LocalContainer,
        display_title: "bash (container)",
        cwd_hint: Some("/workspaces/code"),
        execution_context_ref: "execution_context.container_local.devshell",
        trust_state: TerminalTrustState::Trusted,
        observed_at: "mono:2",
    })
}

fn mark_active(host: &mut PtyHost, id: &PtySessionId, observed_at: &str) {
    host.mark_starting(id, observed_at).unwrap();
    host.mark_active(id, observed_at).unwrap();
}

fn snapshot_session(host: &PtyHost, id: &PtySessionId) -> PtySession {
    host.session(id).expect("session must exist").clone()
}

fn resolve_local(resolver: &mut ExecutionContextResolver) -> aureline_runtime::ExecutionContext {
    resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ))
}

fn resolve_remote(resolver: &mut ExecutionContextResolver) -> aureline_runtime::ExecutionContext {
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.handoff",
        TrustState::Trusted,
        "mono:1",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");
    resolver.resolve(request)
}

fn resolve_container(resolver: &mut ExecutionContextResolver) -> aureline_runtime::ExecutionContext {
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.handoff",
        TrustState::Trusted,
        "mono:2",
    );
    request.override_target_class = Some(TargetClass::ContainerLocal);
    request.override_working_directory = Some("/workspaces/code");
    resolver.resolve(request)
}

#[test]
fn protected_walk_local_open_renders_hidden_cue_and_clean_invariants() {
    // Trusted local terminal seed; the wedge opens one session and the
    // card stays clean — no degraded chip, no honesty marker, no
    // invariant violations.
    let mut resolver = baseline_resolver();
    let context = resolve_local(&mut resolver);
    let mut host = PtyHost::new();
    let id = open_local_session(&mut host);
    mark_active(&mut host, &id, "mono:1");
    let session = snapshot_session(&host, &id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&context, &session, "mono:1")
        .expect("initial open");
    let card = wedge.card();

    assert_eq!(card.record_kind, HOST_BOUNDARY_CUE_CARD_RECORD_KIND);
    assert_eq!(card.schema_version, HOST_BOUNDARY_CUE_CARD_SCHEMA_VERSION);
    assert_eq!(card.workspace_id, "ws-host-boundary");
    assert_eq!(
        card.prototype_label_token,
        "m1_prototype_host_boundary_cues_and_target_handoff"
    );
    assert_eq!(card.entry_point, BadgeEntryPoint::Terminal);
    assert_eq!(card.entry_point_token, "terminal");
    assert_eq!(card.steps.len(), 1);
    let step = &card.steps[0];
    assert_eq!(step.kind, HandoffKind::InitialOpen);
    assert!(step.source.is_none());
    assert_eq!(step.boundary_cue, HostBoundaryCue::Hidden);
    assert!(!step.boundary_cue_visible);
    assert!(step.degraded_token.is_none());
    assert!(!step.honesty_marker_present);
    assert_eq!(step.current.target_class, TargetBadgeClass::LocalDesktop);
    assert_eq!(step.current.canonical_target_id, "localhost:darwin-arm64");

    assert!(card.invariants.is_empty());
    assert!(!card.has_invariant_violations);
    assert!(card.is_clean_local_path());

    // Canonical claim-limit set MUST be rendered in stable order.
    let tokens: Vec<&str> = card
        .claim_limits
        .iter()
        .map(|row| row.token.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "single_certified_wedge_only",
            "no_remote_orchestration_breadth",
            "no_provider_parity_implied",
            "no_transport_orchestration",
        ]
    );
}

#[test]
fn failure_drill_handoff_preserves_source_and_lights_local_to_remote() {
    // Failure drill: open local terminal, then hand off to an SSH remote.
    // The wedge MUST preserve the source identity, light
    // `local_to_remote`, and refuse to flatten the canonical target id.
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let remote_context = resolve_remote(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);
    let remote_id = open_remote_session(&mut host);
    mark_active(&mut host, &remote_id, "mono:2");
    let remote_session = snapshot_session(&host, &remote_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .expect("initial open");
    wedge
        .record_target_handoff(&remote_context, &remote_session, "mono:2")
        .expect("target handoff lands");

    let card = wedge.card();
    assert_eq!(card.steps.len(), 2);
    let handoff = &card.steps[1];
    assert_eq!(handoff.kind, HandoffKind::TargetHandoff);
    let source = handoff.source.as_ref().expect("source identity preserved");
    assert_eq!(source.target_class, TargetBadgeClass::LocalDesktop);
    assert_eq!(source.canonical_target_id, "localhost:darwin-arm64");
    assert_ne!(source.session_id, handoff.current.session_id);
    assert_ne!(
        source.canonical_target_id,
        handoff.current.canonical_target_id
    );
    assert_eq!(handoff.current.target_class, TargetBadgeClass::RemoteHost);
    assert_eq!(handoff.boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(handoff.boundary_cue_visible);

    assert_eq!(card.current_boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(card.current_boundary_cue_visible);
    assert!(card.invariants.is_empty());
    assert!(!card.has_invariant_violations);
}

#[test]
fn handoff_that_flattens_target_identity_is_rejected() {
    // A handoff onto the same canonical target id MUST be rejected with
    // `HandoffFlattensTargetIdentity`. The wedge cannot let a buggy caller
    // smuggle a flat identity past the API.
    let mut resolver = baseline_resolver();
    let context = resolve_local(&mut resolver);
    let mut host = PtyHost::new();
    let id_a = open_local_session(&mut host);
    mark_active(&mut host, &id_a, "mono:1");
    let session_a = snapshot_session(&host, &id_a);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&context, &session_a, "mono:1")
        .expect("initial open");
    // Try to "hand off" to a second local session — the canonical target
    // id is the same, so the wedge MUST reject.
    let id_b = open_local_session(&mut host);
    mark_active(&mut host, &id_b, "mono:2");
    let session_b = snapshot_session(&host, &id_b);
    let err = wedge
        .record_target_handoff(&context, &session_b, "mono:2")
        .unwrap_err();
    assert!(matches!(
        err,
        WedgeError::HandoffFlattensTargetIdentity { .. }
    ));

    // The wedge state is unchanged after the rejection so the chrome can
    // surface the failure without losing the prior step.
    let card = wedge.card();
    assert_eq!(card.steps.len(), 1);
}

#[test]
fn transport_lost_then_reconnect_keeps_cue_visible_on_remote_lane() {
    // After a remote handoff, drop transport. The boundary cue MUST stay
    // visible (`local_to_remote`) and the wedge MUST surface an Offline
    // degraded chip with an honesty marker. A subsequent reconnect-same-
    // identity preserves the canonical target id.
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let remote_context = resolve_remote(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);
    let remote_id = open_remote_session(&mut host);
    mark_active(&mut host, &remote_id, "mono:2");
    let remote_session_active = snapshot_session(&host, &remote_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    wedge
        .record_target_handoff(&remote_context, &remote_session_active, "mono:2")
        .unwrap();

    wedge
        .record_transport_lost("mono:3", Some("network_drop"))
        .unwrap();

    {
        let card = wedge.card();
        let lost = card.steps.last().expect("transport_lost step");
        assert_eq!(lost.kind, HandoffKind::TransportLost);
        // Boundary cue MUST stay visible — degraded states never erase
        // the boundary truth.
        assert_eq!(lost.boundary_cue, HostBoundaryCue::LocalToRemote);
        assert!(lost.boundary_cue_visible);
        assert_eq!(lost.degraded_token.as_deref(), Some("Offline"));
        assert!(lost.honesty_marker_present);
        assert_eq!(card.current_boundary_cue, HostBoundaryCue::LocalToRemote);
        assert_eq!(card.current_degraded_token.as_deref(), Some("Offline"));
        assert!(card.has_honesty_marker);
        assert!(!card.has_invariant_violations);
    }

    // Drive the terminal host through reconnect so the session lifecycle
    // reads ReconnectedSameIdentity, then snapshot.
    host.mark_lost_transport(&remote_id, "mono:3", Some("network_drop"))
        .unwrap();
    host.mark_reconnected_same_identity(&remote_id, "mono:4")
        .unwrap();
    let remote_session_reconnected = snapshot_session(&host, &remote_id);
    wedge
        .record_reconnect(&remote_context, &remote_session_reconnected, "mono:4")
        .unwrap();

    let card = wedge.card();
    let reconnected = card.steps.last().expect("reconnect step");
    assert_eq!(reconnected.kind, HandoffKind::ReconnectedSameIdentity);
    let source = reconnected.source.as_ref().expect("source preserved");
    assert_eq!(
        source.canonical_target_id,
        reconnected.current.canonical_target_id
    );
    assert_eq!(reconnected.boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(reconnected.boundary_cue_visible);
    assert_eq!(reconnected.degraded_token.as_deref(), Some("Warming"));
    assert!(!card.has_invariant_violations);
}

#[test]
fn quarantine_keeps_remote_boundary_cue_visible_and_lights_policy_blocked_chip() {
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let remote_context = resolve_remote(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);
    let remote_id = open_remote_session(&mut host);
    mark_active(&mut host, &remote_id, "mono:2");
    let remote_session = snapshot_session(&host, &remote_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    wedge
        .record_target_handoff(&remote_context, &remote_session, "mono:2")
        .unwrap();
    wedge
        .record_quarantined("mono:3", "supervisor_revoked")
        .unwrap();

    let card = wedge.card();
    let quar = card.steps.last().unwrap();
    assert_eq!(quar.kind, HandoffKind::Quarantined);
    // The cue does not collapse — chrome reads `local_to_remote` plus a
    // `PolicyBlocked` chip side by side.
    assert_eq!(quar.boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(quar.boundary_cue_visible);
    assert_eq!(quar.degraded_token.as_deref(), Some("PolicyBlocked"));
    assert!(quar.honesty_marker_present);
    assert_eq!(quar.reason_code.as_deref(), Some("supervisor_revoked"));
    assert!(!card.has_invariant_violations);
}

#[test]
fn policy_blocked_step_switches_boundary_cue_to_policy_blocked() {
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    wedge
        .record_policy_blocked("mono:2", "trust_revoked")
        .unwrap();

    let card = wedge.card();
    let blocked = card.steps.last().unwrap();
    assert_eq!(blocked.kind, HandoffKind::PolicyBlocked);
    assert_eq!(blocked.boundary_cue, HostBoundaryCue::PolicyBlocked);
    assert!(blocked.boundary_cue_visible);
    assert_eq!(blocked.degraded_token.as_deref(), Some("PolicyBlocked"));
    assert!(card.has_honesty_marker);
    assert!(!card.has_invariant_violations);
}

#[test]
fn closed_step_seals_the_wedge_and_preserves_prior_identity() {
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    wedge
        .record_closed("mono:2", Some("user_closed"))
        .unwrap();
    assert!(wedge.is_closed());

    let err = wedge
        .record_target_handoff(&local_context, &local_session, "mono:3")
        .unwrap_err();
    assert!(matches!(err, WedgeError::AlreadyClosed));

    let card = wedge.card();
    assert_eq!(card.steps.len(), 2);
    let closed = card.steps.last().unwrap();
    assert_eq!(closed.kind, HandoffKind::Closed);
    assert_eq!(closed.current.lifecycle_state, SessionLifecycleState::Closed);
    assert_eq!(closed.degraded_token.as_deref(), Some("Limited"));
}

#[test]
fn handoff_to_container_lights_local_to_container_cue() {
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let container_context = resolve_container(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);
    let container_id = open_container_session(&mut host);
    mark_active(&mut host, &container_id, "mono:2");
    let container_session = snapshot_session(&host, &container_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    wedge
        .record_target_handoff(&container_context, &container_session, "mono:2")
        .unwrap();

    let card = wedge.card();
    let handoff = card.steps.last().unwrap();
    assert_eq!(handoff.kind, HandoffKind::TargetHandoff);
    assert_eq!(handoff.boundary_cue, HostBoundaryCue::LocalToContainer);
    assert!(handoff.boundary_cue_visible);
    assert_eq!(
        handoff.current.target_class,
        TargetBadgeClass::LocalContainer
    );
}

#[test]
fn record_before_open_initial_is_rejected_with_not_initialized() {
    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    let err = wedge
        .record_transport_lost("mono:0", Some("never_opened"))
        .unwrap_err();
    assert!(matches!(err, WedgeError::NotInitialized));
}

#[test]
fn reconnect_identity_mismatch_is_rejected() {
    // Reconnect-same-identity must keep the canonical target id stable.
    // A caller that asks the wedge to "reconnect" onto a different target
    // is asking for a fresh open, not a reconnect — refuse explicitly.
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let remote_context = resolve_remote(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);
    let remote_id = open_remote_session(&mut host);
    mark_active(&mut host, &remote_id, "mono:2");
    let remote_session = snapshot_session(&host, &remote_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    let err = wedge
        .record_reconnect(&remote_context, &remote_session, "mono:2")
        .unwrap_err();
    assert!(matches!(err, WedgeError::ReconnectIdentityMismatch { .. }));
    assert_eq!(wedge.card().steps.len(), 1);
}

#[test]
fn render_plaintext_quotes_every_step_in_stable_order() {
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let remote_context = resolve_remote(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);
    let remote_id = open_remote_session(&mut host);
    mark_active(&mut host, &remote_id, "mono:2");
    let remote_session = snapshot_session(&host, &remote_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    wedge
        .record_target_handoff(&remote_context, &remote_session, "mono:2")
        .unwrap();
    wedge
        .record_transport_lost("mono:3", Some("network_drop"))
        .unwrap();

    let card = wedge.card();
    let text = card.render_plaintext();
    assert!(text.starts_with("[m1_prototype_host_boundary_cues_and_target_handoff]"));
    assert!(text.contains("entry_point=terminal"));
    assert!(text.contains("kind=initial_open"));
    assert!(text.contains("kind=target_handoff"));
    assert!(text.contains("kind=transport_lost"));
    assert!(text.contains("cue=local_to_remote"));
    assert!(text.contains("degraded=Offline"));
    // Claim limits MUST be quoted verbatim in the deterministic block.
    assert!(text.contains("single_certified_wedge_only"));
    assert!(text.contains("no_remote_orchestration_breadth"));
    assert!(text.contains("no_provider_parity_implied"));
    assert!(text.contains("no_transport_orchestration"));
    // No invariant violations on the protected/failure-drill walk.
    assert!(text.contains("invariants:\n  - clean"));
}

#[test]
fn record_round_trips_through_serde_json() {
    let mut resolver = baseline_resolver();
    let local_context = resolve_local(&mut resolver);
    let mut host = PtyHost::new();
    let local_id = open_local_session(&mut host);
    mark_active(&mut host, &local_id, "mono:1");
    let local_session = snapshot_session(&host, &local_id);

    let mut wedge = HostBoundaryCueWedge::new("ws-host-boundary");
    wedge
        .open_initial(&local_context, &local_session, "mono:1")
        .unwrap();
    let card = wedge.card();
    let json = serde_json::to_string(&card).expect("serialise");
    let parsed: HostBoundaryCueCardRecord =
        serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, card);
}

#[test]
fn fixture_protected_walk_replays_into_the_wedge() {
    let fixture: WedgeFixture = load_fixture("protected_walk_local_terminal.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
}

#[test]
fn fixture_failure_drill_handoff_preserves_source_and_lights_remote_cue() {
    let fixture: WedgeFixture = load_fixture("failure_drill_local_to_remote_handoff.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    // Failure-drill specific: the second step MUST carry both source and
    // current canonical target identity and they MUST differ.
    let handoff = card
        .steps
        .iter()
        .find(|step| step.kind == HandoffKind::TargetHandoff)
        .expect("target handoff step");
    let source = handoff.source.as_ref().expect("source identity preserved");
    assert_ne!(source.canonical_target_id, handoff.current.canonical_target_id);
}

#[test]
fn fixture_transport_loss_drill_keeps_cue_visible_through_degraded_state() {
    let fixture: WedgeFixture = load_fixture("transport_loss_keeps_remote_cue_visible.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    // Specific to this drill: the boundary cue stays `local_to_remote`
    // on the transport_lost step and the wedge surfaces Offline + an
    // honesty marker.
    let lost = card
        .steps
        .iter()
        .find(|step| step.kind == HandoffKind::TransportLost)
        .expect("transport_lost step");
    assert_eq!(lost.boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(lost.boundary_cue_visible);
    assert_eq!(lost.degraded_token.as_deref(), Some("Offline"));
    assert!(lost.honesty_marker_present);
}

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct WedgeFixture {
    workspace_id: String,
    wedge_id: Option<String>,
    steps: Vec<WedgeFixtureStep>,
    expect: WedgeFixtureExpect,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum WedgeFixtureStep {
    InitialOpen(WedgeFixtureSession),
    TargetHandoff(WedgeFixtureSession),
    Reconnect(WedgeFixtureSession),
    TransportLost {
        observed_at: String,
        #[serde(default)]
        reason_code: Option<String>,
    },
    Quarantined {
        observed_at: String,
        reason_code: String,
    },
    PolicyBlocked {
        observed_at: String,
        reason_code: String,
    },
    Closed {
        observed_at: String,
        #[serde(default)]
        reason_code: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureSession {
    host_class: WedgeFixtureHostClass,
    target_class: WedgeFixtureTargetClass,
    working_directory: String,
    execution_context_ref: String,
    display_title: String,
    cwd_hint: Option<String>,
    observed_at: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WedgeFixtureHostClass {
    HostDesktop,
    RemoteAgentPrimary,
    LocalContainer,
}

impl WedgeFixtureHostClass {
    fn to_host_class(self) -> HostClass {
        match self {
            Self::HostDesktop => HostClass::HostDesktop,
            Self::RemoteAgentPrimary => HostClass::RemoteAgentPrimary,
            Self::LocalContainer => HostClass::LocalContainer,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WedgeFixtureTargetClass {
    LocalHost,
    SshRemote,
    ContainerLocal,
}

impl WedgeFixtureTargetClass {
    fn to_target_class(self) -> TargetClass {
        match self {
            Self::LocalHost => TargetClass::LocalHost,
            Self::SshRemote => TargetClass::SshRemote,
            Self::ContainerLocal => TargetClass::ContainerLocal,
        }
    }
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureExpect {
    steps: Vec<WedgeFixtureStepExpect>,
    current_boundary_cue: String,
    current_boundary_cue_visible: bool,
    has_honesty_marker: bool,
    has_invariant_violations: bool,
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureStepExpect {
    kind: String,
    boundary_cue: String,
    boundary_cue_visible: bool,
    #[serde(default)]
    degraded_token: Option<String>,
    honesty_marker_present: bool,
    current_target_class: String,
    #[serde(default)]
    source_target_class: Option<String>,
}

fn load_fixture(name: &str) -> WedgeFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/targets/m1_target_identity_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn build_card_from_fixture(fixture: &WedgeFixture) -> HostBoundaryCueCardRecord {
    let mut resolver = baseline_resolver_for(&fixture.workspace_id);
    let mut host = PtyHost::new();
    let mut wedge = HostBoundaryCueWedge::new(&fixture.workspace_id);
    if let Some(id) = &fixture.wedge_id {
        wedge = wedge.with_wedge_id(id);
    }
    for step in &fixture.steps {
        match step {
            WedgeFixtureStep::InitialOpen(session) => {
                let observed_at = session.observed_at.clone();
                let (context, pty_session) =
                    open_fixture_session(&mut resolver, &mut host, &fixture.workspace_id, session);
                wedge
                    .open_initial(&context, &pty_session, &observed_at)
                    .expect("fixture initial open");
            }
            WedgeFixtureStep::TargetHandoff(session) => {
                let observed_at = session.observed_at.clone();
                let (context, pty_session) =
                    open_fixture_session(&mut resolver, &mut host, &fixture.workspace_id, session);
                wedge
                    .record_target_handoff(&context, &pty_session, &observed_at)
                    .expect("fixture target handoff");
            }
            WedgeFixtureStep::Reconnect(session) => {
                let observed_at = session.observed_at.clone();
                let (context, pty_session) =
                    open_fixture_session(&mut resolver, &mut host, &fixture.workspace_id, session);
                wedge
                    .record_reconnect(&context, &pty_session, &observed_at)
                    .expect("fixture reconnect");
            }
            WedgeFixtureStep::TransportLost {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_transport_lost(observed_at, reason_code.as_deref())
                    .expect("fixture transport_lost");
            }
            WedgeFixtureStep::Quarantined {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_quarantined(observed_at, reason_code)
                    .expect("fixture quarantined");
            }
            WedgeFixtureStep::PolicyBlocked {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_policy_blocked(observed_at, reason_code)
                    .expect("fixture policy_blocked");
            }
            WedgeFixtureStep::Closed {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_closed(observed_at, reason_code.as_deref())
                    .expect("fixture closed");
            }
        }
    }
    wedge.card()
}

fn open_fixture_session(
    resolver: &mut ExecutionContextResolver,
    host: &mut PtyHost,
    workspace_id: &str,
    session: &WedgeFixtureSession,
) -> (aureline_runtime::ExecutionContext, PtySession) {
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        &session.observed_at,
    );
    request.override_target_class = Some(session.target_class.to_target_class());
    request.override_working_directory = Some(session.working_directory.as_str());
    let context = resolver.resolve(request);
    let id = host.open_session(OpenSessionRequest {
        workspace_id,
        host_class: session.host_class.to_host_class(),
        display_title: &session.display_title,
        cwd_hint: session.cwd_hint.as_deref(),
        execution_context_ref: &session.execution_context_ref,
        trust_state: TerminalTrustState::Trusted,
        observed_at: &session.observed_at,
    });
    host.mark_starting(&id, &session.observed_at).unwrap();
    host.mark_active(&id, &session.observed_at).unwrap();
    let snapshot = host.session(&id).expect("session opens").clone();
    (context, snapshot)
}

fn baseline_resolver_for(workspace_id: &str) -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: workspace_id.to_owned(),
        profile_id: Some("prof.default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: format!("caps:{workspace_id}:seed"),
            capsule_hash: "sha256:seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "seed-0".to_owned(),
    })
}

fn assert_fixture_matches(card: &HostBoundaryCueCardRecord, fixture: &WedgeFixture) {
    assert_eq!(card.steps.len(), fixture.expect.steps.len());
    for (step, expect) in card.steps.iter().zip(fixture.expect.steps.iter()) {
        assert_eq!(step.kind_token, expect.kind, "step kind mismatch");
        assert_eq!(
            step.boundary_cue_token, expect.boundary_cue,
            "boundary cue mismatch for step {}",
            step.step_id,
        );
        assert_eq!(
            step.boundary_cue_visible, expect.boundary_cue_visible,
            "boundary cue visibility mismatch for step {}",
            step.step_id,
        );
        assert_eq!(
            step.degraded_token.as_deref(),
            expect.degraded_token.as_deref(),
            "degraded token mismatch for step {}",
            step.step_id,
        );
        assert_eq!(
            step.honesty_marker_present, expect.honesty_marker_present,
            "honesty marker mismatch for step {}",
            step.step_id,
        );
        assert_eq!(
            step.current.target_class_token, expect.current_target_class,
            "current target class mismatch for step {}",
            step.step_id,
        );
        match (step.source.as_ref(), expect.source_target_class.as_deref()) {
            (Some(source), Some(expected)) => assert_eq!(
                source.target_class_token, expected,
                "source target class mismatch for step {}",
                step.step_id,
            ),
            (None, None) => {}
            other => panic!("source presence mismatch on step {}: {:?}", step.step_id, other),
        }
    }
    assert_eq!(
        card.current_boundary_cue_token, fixture.expect.current_boundary_cue,
        "current boundary cue token mismatch",
    );
    assert_eq!(
        card.current_boundary_cue_visible, fixture.expect.current_boundary_cue_visible,
        "current boundary cue visibility mismatch",
    );
    assert_eq!(
        card.has_honesty_marker, fixture.expect.has_honesty_marker,
        "honesty marker mismatch on the card",
    );
    assert_eq!(
        card.has_invariant_violations, fixture.expect.has_invariant_violations,
        "invariant-violation presence mismatch on the card",
    );
}
