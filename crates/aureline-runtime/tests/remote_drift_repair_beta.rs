use std::path::{Path, PathBuf};

use aureline_runtime::{
    CompatibilityWindow, CompatibilityWindowStatus, DroppedHelperCapability,
    EffectiveCapabilityPosture, HelperCapabilityResponse, MissingCapabilityReasonClass,
    NegotiationOutcome, RemoteDriftRepairDiagnosticsPacket, RemoteDriftRepairGuidance,
    RemoteHelperBetaRecord, RemoteHelperLifecyclePhaseClass, RemoteHelperSkewVisibilityClass,
    HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION, REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_path(name: &str) -> PathBuf {
    repo_root()
        .join("fixtures/runtime/m3/remote_drift")
        .join(name)
}

fn load_fixture(name: &str) -> FixtureRecord {
    let path = fixture_path(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn parse_visibility(token: &str) -> RemoteHelperSkewVisibilityClass {
    RemoteHelperSkewVisibilityClass::ALL
        .into_iter()
        .find(|class| class.as_str() == token)
        .unwrap_or_else(|| panic!("unknown skew visibility token {token}"))
}

fn parse_phase(token: &str) -> RemoteHelperLifecyclePhaseClass {
    match token {
        "attach" => RemoteHelperLifecyclePhaseClass::Attach,
        "reconnect" => RemoteHelperLifecyclePhaseClass::Reconnect,
        other => panic!("unknown lifecycle phase {other}"),
    }
}

fn parse_outcome(token: &str) -> NegotiationOutcome {
    match token {
        "match" => NegotiationOutcome::Match,
        "downgrade" => NegotiationOutcome::Downgrade,
        "refuse" => NegotiationOutcome::Refuse,
        other => panic!("unknown negotiation outcome {other}"),
    }
}

fn parse_posture(token: &str) -> EffectiveCapabilityPosture {
    EffectiveCapabilityPosture::from_token(token)
        .unwrap_or_else(|err| panic!("posture parse failed: {err}"))
}

fn parse_window_status(visibility: RemoteHelperSkewVisibilityClass) -> CompatibilityWindowStatus {
    match visibility {
        RemoteHelperSkewVisibilityClass::AdjacentSupported => CompatibilityWindowStatus::Supported,
        RemoteHelperSkewVisibilityClass::NarrowedSupportedWindow => {
            CompatibilityWindowStatus::BestEffort
        }
        RemoteHelperSkewVisibilityClass::ProbeRequiredUntested => {
            CompatibilityWindowStatus::Untested
        }
        RemoteHelperSkewVisibilityClass::OutsideSupportedWindow => {
            CompatibilityWindowStatus::Unsupported
        }
    }
}

fn reason_class(token: &str) -> MissingCapabilityReasonClass {
    match token {
        "helper_does_not_offer" => MissingCapabilityReasonClass::HelperDoesNotOffer,
        "client_requires_unknown_feature" => {
            MissingCapabilityReasonClass::ClientRequiresUnknownFeature
        }
        "outside_skew_window" => MissingCapabilityReasonClass::OutsideSkewWindow,
        "protocol_floor_mismatch" => MissingCapabilityReasonClass::ProtocolFloorMismatch,
        "policy_narrowed" => MissingCapabilityReasonClass::PolicyNarrowed,
        "trust_not_verified" => MissingCapabilityReasonClass::TrustNotVerified,
        "probe_required" => MissingCapabilityReasonClass::ProbeRequired,
        other => panic!("unknown reason class {other}"),
    }
}

fn record_from_fixture(fixture: &FixtureRecord) -> RemoteHelperBetaRecord {
    let visibility = parse_visibility(&fixture.skew_visibility);
    let phase = parse_phase(&fixture.lifecycle_phase);
    let dropped: Vec<DroppedHelperCapability> = fixture
        .dropped_capabilities
        .iter()
        .map(|drop| DroppedHelperCapability {
            capability: drop.capability.clone(),
            reason_class: reason_class(&drop.reason_class),
            visible_reason: drop.visible_reason.clone(),
            retryable: drop.retryable,
        })
        .collect();
    let response = HelperCapabilityResponse {
        schema_version: HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
        request_id: fixture.envelope_id.clone(),
        row_id: format!(
            "drift_truth.helper_agent.{}",
            fixture.envelope_id.replace(':', ".")
        ),
        surface_ref: fixture.envelope_id.clone(),
        title: fixture.visible_summary.clone(),
        outcome: parse_outcome(&fixture.negotiation_outcome),
        selected_protocol_ref: fixture
            .visible_version_state
            .selected_protocol_version
            .clone(),
        negotiated_capabilities: Vec::new(),
        dropped_capabilities: dropped,
        mutation_allowed: fixture.effective_posture == "full_remote"
            && fixture.negotiation_outcome == "match",
        effective_posture: parse_posture(&fixture.effective_posture),
        visible_summary: fixture.visible_summary.clone(),
        safe_continuation: fixture.safe_continuation.clone(),
        primary_recovery_ref: None,
        recovery_refs: Vec::new(),
        blocked_action_refs: Vec::new(),
        preserved_read_only_refs: Vec::new(),
        retry_ref: None,
        support_packet_refs: fixture.support_packet_refs.clone(),
        review_packet_refs: Vec::new(),
        source_refs: Vec::new(),
        client_manifest_digest: format!("digest:client:{}", fixture.envelope_id),
        helper_manifest_digest: format!("digest:helper:{}", fixture.envelope_id),
        compatibility_window: CompatibilityWindow {
            boundary_family: "desktop_cli_and_remote_agent".to_owned(),
            compatibility_row_ref: fixture.visible_version_state.compatibility_row_ref.clone(),
            version_skew_register_ref: "version_skew_register:remote.attach_envelope_and_drift"
                .to_owned(),
            skew_case_ref: fixture.visible_version_state.skew_case_ref.clone(),
            skew_window_declaration_ref: fixture
                .visible_version_state
                .skew_window_declaration_ref
                .clone(),
            status: parse_window_status(visibility),
            selected_protocol_ref: fixture
                .visible_version_state
                .selected_protocol_version
                .clone(),
            source_refs: Vec::new(),
        },
    };
    RemoteHelperBetaRecord::from_response(
        &response,
        phase,
        if matches!(phase, RemoteHelperLifecyclePhaseClass::Reconnect) {
            1
        } else {
            0
        },
        format!("attach_session:{}", fixture.envelope_id.replace(':', ".")),
        fixture.visible_version_state.client_version.clone(),
        fixture.visible_version_state.helper_version.clone(),
        fixture.compatibility_report_row_refs.clone(),
    )
}

fn run_case(name: &str) -> (FixtureRecord, RemoteDriftRepairGuidance) {
    let fixture = load_fixture(name);
    let record = record_from_fixture(&fixture);
    let guidance = RemoteDriftRepairGuidance::from_record(&record);
    (fixture, guidance)
}

#[test]
fn adjacent_supported_baseline_admits_no_repair_required() {
    let (fixture, guidance) = run_case("adjacent_supported_baseline.json");
    assert_eq!(
        guidance.schema_version,
        REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION
    );
    assert_eq!(
        guidance.primary_action.action_class_token,
        fixture.primary_action.action_class
    );
    assert_eq!(guidance.drift_reason_tokens, fixture.drift_reasons);
    assert!(!guidance.fails_closed_for_mutation);
    assert!(!guidance.any_action_requires_reapproval);
}

#[test]
fn version_mismatch_yields_upgrade_primary_with_authority_widening_flag() {
    let (fixture, guidance) = run_case("version_mismatch_upgrade_prompt.json");
    assert_eq!(
        guidance.primary_action.action_class_token,
        fixture.primary_action.action_class
    );
    assert_eq!(guidance.drift_reason_tokens, fixture.drift_reasons);
    assert!(guidance.fails_closed_for_mutation);
    assert!(guidance.any_action_requires_reapproval);
    assert!(guidance.primary_action.requires_reapproval);
}

#[test]
fn capability_mismatch_emits_downgrade_alternative() {
    let (fixture, guidance) = run_case("capability_mismatch_downgrade_prompt.json");
    assert_eq!(guidance.drift_reason_tokens, fixture.drift_reasons);
    assert_eq!(
        guidance.primary_action.action_class_token,
        fixture.primary_action.action_class
    );
    let alternative_classes: Vec<&str> = guidance
        .alternative_actions
        .iter()
        .map(|action| action.action_class_token.as_str())
        .collect();
    assert!(alternative_classes.contains(&"downgrade"));
    assert!(alternative_classes.contains(&"continue_local_only"));
}

#[test]
fn auth_mismatch_yields_contact_admin_primary_and_no_widening() {
    let (fixture, guidance) = run_case("auth_mismatch_contact_admin.json");
    assert_eq!(guidance.drift_reason_tokens, fixture.drift_reasons);
    assert_eq!(
        guidance.primary_action.action_class_token,
        fixture.primary_action.action_class
    );
    assert!(!guidance.any_action_requires_reapproval);
}

#[test]
fn route_mismatch_reconnect_emits_probe_with_reconnect_alternative() {
    let (fixture, guidance) = run_case("route_mismatch_reconnect_probe.json");
    assert_eq!(guidance.drift_reason_tokens, fixture.drift_reasons);
    assert_eq!(
        guidance.primary_action.action_class_token,
        fixture.primary_action.action_class
    );
    let alternative_classes: Vec<&str> = guidance
        .alternative_actions
        .iter()
        .map(|action| action.action_class_token.as_str())
        .collect();
    assert!(alternative_classes.contains(&"reconnect"));
}

#[test]
fn target_mismatch_continue_local_keeps_authority_narrowed() {
    let (fixture, guidance) = run_case("target_mismatch_continue_local.json");
    assert_eq!(guidance.drift_reason_tokens, fixture.drift_reasons);
    assert_eq!(
        guidance.primary_action.action_class_token,
        fixture.primary_action.action_class
    );
    assert_eq!(
        guidance.primary_action.authority_impact_token,
        "narrows_authority"
    );
    assert!(!guidance.any_action_requires_reapproval);
}

#[test]
fn diagnostics_packet_replays_in_product_reasoning() {
    let cases = [
        "adjacent_supported_baseline.json",
        "version_mismatch_upgrade_prompt.json",
        "capability_mismatch_downgrade_prompt.json",
        "auth_mismatch_contact_admin.json",
        "route_mismatch_reconnect_probe.json",
        "target_mismatch_continue_local.json",
    ];
    let guidance: Vec<RemoteDriftRepairGuidance> =
        cases.iter().map(|name| run_case(name).1).collect();
    let packet = RemoteDriftRepairDiagnosticsPacket::from_guidance(
        "remote_drift_repair_packet:integration_replay",
        "2026-05-16T20:00:00Z",
        guidance.iter(),
    );
    assert_eq!(packet.guidance_records.len(), cases.len());
    for token in [
        "version_mismatch",
        "capability_mismatch",
        "auth_mismatch",
        "route_mismatch",
        "target_mismatch",
    ] {
        assert!(
            packet
                .drift_reason_summary_tokens
                .iter()
                .any(|item| item == token),
            "missing reason token {token} in diagnostics packet"
        );
    }
    for token in [
        "no_repair_required",
        "upgrade",
        "downgrade",
        "reconnect",
        "run_drift_probe",
        "continue_local_only",
        "contact_admin_or_support",
    ] {
        assert!(
            packet
                .repair_action_summary_tokens
                .iter()
                .any(|item| item == token),
            "missing action token {token} in diagnostics packet"
        );
    }
    assert!(packet.any_record_fails_closed_for_mutation);
    assert!(packet.any_record_requires_reapproval);
}

#[derive(Debug, Deserialize)]
struct FixtureRecord {
    envelope_id: String,
    lifecycle_phase: String,
    drift_reasons: Vec<String>,
    primary_action: FixtureAction,
    #[allow(dead_code)]
    alternative_actions: Vec<FixtureAction>,
    visible_version_state: FixtureVisibleVersion,
    effective_posture: String,
    negotiation_outcome: String,
    skew_visibility: String,
    visible_summary: String,
    safe_continuation: String,
    dropped_capabilities: Vec<FixtureDroppedCapability>,
    support_packet_refs: Vec<String>,
    compatibility_report_row_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureAction {
    action_class: String,
    #[allow(dead_code)]
    authority_impact: String,
}

#[derive(Debug, Deserialize)]
struct FixtureVisibleVersion {
    client_version: String,
    helper_version: String,
    selected_protocol_version: String,
    skew_case_ref: String,
    skew_window_declaration_ref: String,
    compatibility_row_ref: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct FixtureDroppedCapability {
    capability: String,
    reason_class: String,
    visible_reason: String,
    retryable: bool,
}
