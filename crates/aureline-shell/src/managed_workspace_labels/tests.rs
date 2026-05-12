//! Unit and fixture tests for the bounded managed-workspace lifecycle
//! labels wedge.
//!
//! Coverage:
//! - protected walk (authenticating -> connecting -> warming -> ready)
//!   renders a clean card with zero invariant violations and a visible
//!   prototype label / claim-limit set,
//! - named failure drill (reconnect -> read-only degraded -> suspended
//!   -> reprovisioning -> connecting -> warming -> ready) preserves
//!   honest transitions and keeps the four workspace-copy classes
//!   visibly distinct,
//! - skipped transitions are refused both at the API and surfaced as
//!   the typed `SkippedTransition` invariant on a hand-patched record,
//! - `reconnect` and `reprovisioning` pre-conditions are enforced,
//! - empty authority lineage is refused at the API,
//! - degraded labels carry a non-None recovery action,
//! - ready -> snapshot_only_view keeps writes blocked,
//! - the deterministic plaintext block quotes every section verbatim,
//! - the record round-trips through serde_json,
//! - three fixtures replay into the wedge with their expected payloads.

use std::path::Path;

use serde::Deserialize;

use super::*;

fn baseline_lineage(workspace_id: &str) -> ManagedAuthorityLineage {
    ManagedAuthorityLineage::new(
        workspace_id,
        "tenant:acme-org",
        "managed_org_tenant",
        "managed_control_plane_bearing",
    )
}

fn drive_to_ready(wedge: &mut ManagedWorkspaceLifecycleWedge, lineage: &ManagedAuthorityLineage) {
    wedge
        .open_authenticating(lineage.clone(), "mono:1")
        .expect("authenticating");
    wedge
        .record_connecting(lineage.clone(), "mono:2")
        .expect("connecting");
    wedge
        .record_warming(lineage.clone(), "mono:3")
        .expect("warming");
    wedge
        .record_ready(lineage.clone(), "mono:4")
        .expect("ready");
}

#[test]
fn protected_walk_renders_clean_card_with_visible_labels() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    let card = wedge.card();

    assert_eq!(
        card.record_kind,
        MANAGED_WORKSPACE_LIFECYCLE_CARD_RECORD_KIND
    );
    assert_eq!(
        card.schema_version,
        MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_VERSION
    );
    assert_eq!(card.workspace_id, "ws-managed-acme");
    assert_eq!(
        card.prototype_label_token,
        "m1_prototype_managed_workspace_lifecycle_labels"
    );
    assert_eq!(card.steps.len(), 4);
    let labels: Vec<&str> = card
        .steps
        .iter()
        .map(|step| step.label_token.as_str())
        .collect();
    assert_eq!(
        labels,
        vec!["authenticating", "connecting", "warming", "ready"]
    );
    assert_eq!(card.current_label, ManagedLifecycleLabelClass::Ready);
    assert_eq!(card.current_copy_class, WorkspaceCopyClass::LiveEnvironment);
    assert_eq!(card.current_recovery_action, RecoveryActionClass::None);
    assert!(card.current_admits_writes);
    assert!(card.invariants.is_empty());
    assert!(!card.has_invariant_violations);

    let tokens: Vec<&str> = card
        .claim_limits
        .iter()
        .map(|row| row.token.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "single_certified_wedge_only",
            "no_managed_control_plane_in_m1",
            "no_tenancy_orchestration",
            "no_lifecycle_executor_productization",
        ]
    );
}

#[test]
fn failure_drill_walks_reconnect_through_reprovisioning_back_to_ready() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    wedge
        .record_reconnecting(lineage.clone(), "mono:5", Some("transport_dropped"))
        .expect("reconnect");
    wedge
        .record_read_only_degraded(lineage.clone(), "mono:6", "snapshot_fallback")
        .expect("degraded");
    wedge
        .record_suspended(lineage.clone(), "mono:7", "user_paused")
        .expect("suspend");
    wedge
        .record_reprovisioning(lineage.clone(), "mono:8", "tenant_reprovisioned")
        .expect("reprovision");
    wedge
        .record_connecting(lineage.clone(), "mono:9")
        .expect("re-connect after reprovision");
    wedge
        .record_warming(lineage.clone(), "mono:10")
        .expect("warming on fresh copy");
    wedge
        .record_ready(lineage.clone(), "mono:11")
        .expect("ready on fresh copy");

    let card = wedge.card();
    let labels: Vec<&str> = card.steps.iter().map(|s| s.label_token.as_str()).collect();
    assert_eq!(
        labels,
        vec![
            "authenticating",
            "connecting",
            "warming",
            "ready",
            "reconnecting",
            "read_only_degraded",
            "suspended",
            "reprovisioning",
            "connecting",
            "warming",
            "ready",
        ]
    );
    // All four copy classes the spec requires the wedge to keep
    // visibly distinct must appear.
    let copy_classes: Vec<&str> = card
        .steps
        .iter()
        .map(|s| s.copy_class_token.as_str())
        .collect();
    assert!(copy_classes.contains(&"live_environment"));
    assert!(copy_classes.contains(&"snapshot_only_view"));
    assert!(copy_classes.contains(&"suspended_workspace"));
    assert!(copy_classes.contains(&"fresh_reprovisioned_copy"));

    // Every degraded posture step MUST carry a non-None recovery
    // action.
    for step in card.steps.iter().filter(|s| s.label.is_degraded_posture()) {
        assert_ne!(step.recovery_action, RecoveryActionClass::None);
    }
    assert!(!card.has_invariant_violations);
    assert_eq!(card.current_label, ManagedLifecycleLabelClass::Ready);
    assert!(card.current_admits_writes);
}

#[test]
fn skipping_warming_is_refused_at_the_api() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    wedge
        .open_authenticating(lineage.clone(), "mono:1")
        .expect("authenticating");
    wedge
        .record_connecting(lineage.clone(), "mono:2")
        .expect("connecting");
    let err = wedge.record_ready(lineage.clone(), "mono:3").unwrap_err();
    assert!(matches!(
        err,
        WedgeError::SkippedTransition {
            from: ManagedLifecycleLabelClass::Connecting,
            to: ManagedLifecycleLabelClass::Ready,
        }
    ));
    let card = wedge.card();
    assert_eq!(card.steps.len(), 2);
    // No invariants because the wedge rejected the call rather than
    // appending an illegal step.
    assert!(card.invariants.is_empty());
}

#[test]
fn hand_patched_skipped_transition_surfaces_typed_invariant() {
    // Simulate a buggy caller that bypassed the API and pushed an
    // illegal sequence (connecting -> ready, skipping warming) onto
    // the rendered card. The validator MUST surface the typed
    // `SkippedTransition` invariant rather than letting the chrome
    // render a clean card.
    let lineage = baseline_lineage("ws-managed-acme");
    // Mint a clean reference card first so we can clone valid steps.
    let mut clean = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    drive_to_ready(&mut clean, &lineage);
    let clean_card = clean.card();

    let mut forged = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    // Push authenticating + connecting + ready (skip warming).
    for index in [0, 1, 3] {
        forged.__test_push(clean_card.steps[index].clone());
    }
    let forged_card = forged.card();
    assert!(forged_card.has_invariant_violations);
    assert!(forged_card
        .invariants
        .iter()
        .any(|row| row.violation == ManagedLifecycleInvariantViolation::SkippedTransition));
}

#[test]
fn reconnect_without_prior_connection_is_refused() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    wedge
        .open_authenticating(lineage.clone(), "mono:1")
        .expect("authenticating");
    let err = wedge
        .record_reconnecting(lineage.clone(), "mono:2", Some("transport_dropped"))
        .unwrap_err();
    assert!(matches!(err, WedgeError::ReconnectWithoutPriorConnection));
}

#[test]
fn reprovision_without_prior_pause_is_refused() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    let err = wedge
        .record_reprovisioning(lineage.clone(), "mono:5", "tenant_reprovisioned")
        .unwrap_err();
    assert!(matches!(err, WedgeError::ReprovisionWithoutPriorPause));
}

#[test]
fn empty_lineage_is_refused() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = ManagedAuthorityLineage::new("", "", "", "");
    let err = wedge.open_authenticating(lineage, "mono:1").unwrap_err();
    assert!(matches!(err, WedgeError::EmptyAuthorityLineage));
}

#[test]
fn degraded_step_must_carry_recovery_action() {
    // A read_only_degraded step minted through the API always carries
    // continue_in_snapshot_view. If a caller patches the record after
    // the fact to use RecoveryActionClass::None, the validator MUST
    // surface the typed invariant.
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    wedge
        .record_read_only_degraded(lineage.clone(), "mono:5", "policy_pause")
        .expect("degraded");
    // Patch the last step to drop the recovery action.
    {
        let last = wedge.__test_last_mut();
        last.recovery_action = RecoveryActionClass::None;
        last.recovery_action_token = RecoveryActionClass::None.as_str().to_owned();
        last.recovery_action_label = RecoveryActionClass::None.label().to_owned();
    }
    let card = wedge.card();
    assert!(card.invariants.iter().any(|row| row.violation
        == ManagedLifecycleInvariantViolation::MissingRecoveryActionForDegradedState));
}

#[test]
fn snapshot_only_view_after_ready_keeps_writes_blocked() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    wedge
        .record_snapshot_only_view(lineage.clone(), "mono:5", "tenant_review_mode")
        .expect("snapshot view");
    let card = wedge.card();
    let last = card.steps.last().unwrap();
    assert_eq!(last.label, ManagedLifecycleLabelClass::SnapshotOnlyView);
    assert_eq!(last.copy_class, WorkspaceCopyClass::SnapshotOnlyView);
    assert!(!last.admits_writes);
    assert_eq!(
        last.recovery_action,
        RecoveryActionClass::ContinueInSnapshotView
    );
    assert_eq!(last.degraded_token.as_deref(), Some("Stale"));
    assert!(!card.has_invariant_violations);
}

#[test]
fn closed_step_seals_wedge() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    wedge
        .record_suspended(lineage.clone(), "mono:5", "user_paused")
        .expect("suspend");
    wedge
        .record_closed(lineage.clone(), "mono:6", Some("user_closed"))
        .expect("close");
    assert!(wedge.is_closed());
    let err = wedge.record_ready(lineage.clone(), "mono:7").unwrap_err();
    assert!(matches!(err, WedgeError::AlreadyClosed));
}

#[test]
fn render_plaintext_quotes_every_section_in_stable_order() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    wedge
        .record_reconnecting(lineage.clone(), "mono:5", Some("transport_dropped"))
        .expect("reconnect");
    let card = wedge.card();
    let text = card.render_plaintext();
    assert!(text.starts_with("[m1_prototype_managed_workspace_lifecycle_labels]"));
    assert!(text.contains("label=authenticating"));
    assert!(text.contains("label=connecting"));
    assert!(text.contains("label=warming"));
    assert!(text.contains("label=ready"));
    assert!(text.contains("label=reconnecting"));
    assert!(text.contains("recovery=retry_connection"));
    assert!(text.contains("lineage: workspace=ws-managed-acme tenant=tenant:acme-org"));
    assert!(text.contains("single_certified_wedge_only"));
    assert!(text.contains("no_managed_control_plane_in_m1"));
    assert!(text.contains("no_tenancy_orchestration"));
    assert!(text.contains("no_lifecycle_executor_productization"));
    assert!(text.contains("invariants:\n  - clean"));
}

#[test]
fn record_round_trips_through_serde_json() {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new("ws-managed-acme");
    let lineage = baseline_lineage("ws-managed-acme");
    drive_to_ready(&mut wedge, &lineage);
    let card = wedge.card();
    let json = serde_json::to_string(&card).expect("serialise");
    let parsed: ManagedWorkspaceLifecycleCardRecord =
        serde_json::from_str(&json).expect("round-trip");
    assert_eq!(parsed, card);
}

// ---------------------------------------------------------------------------
// Fixture replay
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Fixture {
    workspace_id: String,
    #[serde(default)]
    wedge_id: Option<String>,
    lineage: FixtureLineage,
    steps: Vec<FixtureStep>,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureLineage {
    workspace_id: String,
    managed_tenant_ref: String,
    identity_mode_token: String,
    locality_class_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FixtureStep {
    OpenAuthenticating {
        observed_at: String,
    },
    RecordConnecting {
        observed_at: String,
    },
    RecordWarming {
        observed_at: String,
    },
    RecordReady {
        observed_at: String,
    },
    RecordReconnecting {
        observed_at: String,
        #[serde(default)]
        reason_code: Option<String>,
    },
    RecordReadOnlyDegraded {
        observed_at: String,
        reason_code: String,
    },
    RecordSuspended {
        observed_at: String,
        reason_code: String,
    },
    RecordReprovisioning {
        observed_at: String,
        reason_code: String,
    },
    RecordSnapshotOnlyView {
        observed_at: String,
        reason_code: String,
    },
    RecordClosed {
        observed_at: String,
        #[serde(default)]
        reason_code: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
struct FixtureExpect {
    current_label: String,
    current_copy_class: String,
    current_recovery_action: String,
    #[serde(default)]
    current_degraded_token: Option<String>,
    current_admits_writes: bool,
    has_invariant_violations: bool,
    steps: Vec<FixtureStepExpect>,
}

#[derive(Debug, Deserialize)]
struct FixtureStepExpect {
    label: String,
    copy_class: String,
    recovery_action: String,
    #[serde(default)]
    degraded_token: Option<String>,
    admits_writes: bool,
}

fn load_fixture(name: &str) -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/managed_workspace/m1_lifecycle_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn build_card_from_fixture(fixture: &Fixture) -> ManagedWorkspaceLifecycleCardRecord {
    let lineage = ManagedAuthorityLineage::new(
        fixture.lineage.workspace_id.clone(),
        fixture.lineage.managed_tenant_ref.clone(),
        fixture.lineage.identity_mode_token.clone(),
        fixture.lineage.locality_class_token.clone(),
    );
    let mut wedge = ManagedWorkspaceLifecycleWedge::new(&fixture.workspace_id);
    if let Some(id) = &fixture.wedge_id {
        wedge = wedge.with_wedge_id(id);
    }
    for step in &fixture.steps {
        match step {
            FixtureStep::OpenAuthenticating { observed_at } => {
                wedge
                    .open_authenticating(lineage.clone(), observed_at)
                    .expect("fixture: open_authenticating");
            }
            FixtureStep::RecordConnecting { observed_at } => {
                wedge
                    .record_connecting(lineage.clone(), observed_at)
                    .expect("fixture: record_connecting");
            }
            FixtureStep::RecordWarming { observed_at } => {
                wedge
                    .record_warming(lineage.clone(), observed_at)
                    .expect("fixture: record_warming");
            }
            FixtureStep::RecordReady { observed_at } => {
                wedge
                    .record_ready(lineage.clone(), observed_at)
                    .expect("fixture: record_ready");
            }
            FixtureStep::RecordReconnecting {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_reconnecting(lineage.clone(), observed_at, reason_code.as_deref())
                    .expect("fixture: record_reconnecting");
            }
            FixtureStep::RecordReadOnlyDegraded {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_read_only_degraded(lineage.clone(), observed_at, reason_code)
                    .expect("fixture: record_read_only_degraded");
            }
            FixtureStep::RecordSuspended {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_suspended(lineage.clone(), observed_at, reason_code)
                    .expect("fixture: record_suspended");
            }
            FixtureStep::RecordReprovisioning {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_reprovisioning(lineage.clone(), observed_at, reason_code)
                    .expect("fixture: record_reprovisioning");
            }
            FixtureStep::RecordSnapshotOnlyView {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_snapshot_only_view(lineage.clone(), observed_at, reason_code)
                    .expect("fixture: record_snapshot_only_view");
            }
            FixtureStep::RecordClosed {
                observed_at,
                reason_code,
            } => {
                wedge
                    .record_closed(lineage.clone(), observed_at, reason_code.as_deref())
                    .expect("fixture: record_closed");
            }
        }
    }
    wedge.card()
}

fn assert_fixture_matches(card: &ManagedWorkspaceLifecycleCardRecord, fixture: &Fixture) {
    assert_eq!(card.current_label_token, fixture.expect.current_label);
    assert_eq!(
        card.current_copy_class_token,
        fixture.expect.current_copy_class
    );
    assert_eq!(
        card.current_recovery_action_token,
        fixture.expect.current_recovery_action
    );
    assert_eq!(
        card.current_degraded_token.as_deref(),
        fixture.expect.current_degraded_token.as_deref(),
    );
    assert_eq!(
        card.current_admits_writes,
        fixture.expect.current_admits_writes
    );
    assert_eq!(
        card.has_invariant_violations,
        fixture.expect.has_invariant_violations
    );
    assert_eq!(card.steps.len(), fixture.expect.steps.len());
    for (step, expect) in card.steps.iter().zip(fixture.expect.steps.iter()) {
        assert_eq!(step.label_token, expect.label, "label mismatch");
        assert_eq!(step.copy_class_token, expect.copy_class, "copy mismatch");
        assert_eq!(
            step.recovery_action_token, expect.recovery_action,
            "recovery mismatch",
        );
        assert_eq!(
            step.degraded_token.as_deref(),
            expect.degraded_token.as_deref(),
            "degraded chip mismatch",
        );
        assert_eq!(
            step.admits_writes, expect.admits_writes,
            "admits_writes mismatch",
        );
    }
}

#[test]
fn fixture_protected_walk_replays_into_the_wedge() {
    let fixture = load_fixture("protected_walk_managed_lifecycle.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
}

#[test]
fn fixture_failure_drill_reconnect_then_reprovision_replays_into_the_wedge() {
    let fixture = load_fixture("failure_drill_reconnect_then_reprovision.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    // The drill MUST visit all four workspace-copy classes the spec
    // requires to remain visibly distinct.
    let copy_tokens: Vec<&str> = card
        .steps
        .iter()
        .map(|s| s.copy_class_token.as_str())
        .collect();
    for required in [
        "live_environment",
        "snapshot_only_view",
        "suspended_workspace",
        "fresh_reprovisioned_copy",
    ] {
        assert!(
            copy_tokens.contains(&required),
            "copy class {required} missing",
        );
    }
}

#[test]
fn fixture_snapshot_only_view_replays_with_writes_blocked() {
    let fixture = load_fixture("snapshot_only_view_keeps_writes_blocked.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    let last = card.steps.last().unwrap();
    assert!(!last.admits_writes);
    assert_eq!(last.copy_class_token, "snapshot_only_view");
}
