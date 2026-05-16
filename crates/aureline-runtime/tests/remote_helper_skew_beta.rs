use std::path::{Path, PathBuf};

use aureline_runtime::{
    CompatibilityWindow, CompatibilityWindowStatus, DroppedHelperCapability,
    EffectiveCapabilityPosture, HelperCapabilityResponse, MissingCapabilityReasonClass,
    NegotiationOutcome, RemoteHelperBetaCompatibilityRow, RemoteHelperBetaRecord,
    RemoteHelperBetaSupportExport, RemoteHelperLifecyclePhaseClass, RemoteHelperRepairPathClass,
    RemoteHelperSkewVisibilityClass,
    HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION, REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_path(name: &str) -> PathBuf {
    repo_root()
        .join("fixtures/runtime/m3/remote_helper_skew_beta")
        .join(name)
}

fn load_fixture(name: &str) -> FixtureRecord {
    let path = fixture_path(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
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

fn parse_dropped(items: &[FixtureDroppedCapability]) -> Vec<DroppedHelperCapability> {
    items
        .iter()
        .map(|item| DroppedHelperCapability {
            capability: item.capability.clone(),
            reason_class: match item.reason_class.as_str() {
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
            },
            visible_reason: item.visible_reason.clone(),
            retryable: item.retryable,
        })
        .collect()
}

fn response_from_fixture(fixture: &FixtureRecord) -> HelperCapabilityResponse {
    let visibility = parse_visibility(&fixture.skew_visibility);
    HelperCapabilityResponse {
        schema_version: HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
        request_id: fixture.envelope_id.clone(),
        row_id: format!(
            "drift_truth.helper_agent.{}",
            fixture.envelope_id.replace(':', ".")
        ),
        surface_ref: fixture.envelope_id.clone(),
        title: fixture.visible_summary.clone(),
        outcome: parse_outcome(&fixture.negotiation_outcome),
        selected_protocol_ref: fixture.visible_version_state.selected_protocol_version.clone(),
        negotiated_capabilities: fixture.negotiated_capabilities.clone(),
        dropped_capabilities: parse_dropped(&fixture.dropped_capabilities),
        mutation_allowed: fixture.mutation_allowed,
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
        source_refs: fixture.evidence_refs.compat_refs.clone(),
        client_manifest_digest: format!("digest:client:{}", fixture.envelope_id),
        helper_manifest_digest: format!("digest:helper:{}", fixture.envelope_id),
        compatibility_window: CompatibilityWindow {
            boundary_family: "desktop_cli_and_remote_agent".to_owned(),
            compatibility_row_ref: fixture.visible_version_state.compatibility_row_ref.clone(),
            version_skew_register_ref:
                "version_skew_register:remote.attach_envelope_and_drift".to_owned(),
            skew_case_ref: fixture.visible_version_state.skew_case_ref.clone(),
            skew_window_declaration_ref: fixture
                .visible_version_state
                .skew_window_declaration_ref
                .clone(),
            status: parse_window_status(visibility),
            selected_protocol_ref: fixture.visible_version_state.selected_protocol_version.clone(),
            source_refs: fixture.evidence_refs.compat_refs.clone(),
        },
    }
}

fn beta_record_from_fixture(fixture: &FixtureRecord) -> RemoteHelperBetaRecord {
    let response = response_from_fixture(fixture);
    let phase = parse_phase(&fixture.lifecycle_phase);
    RemoteHelperBetaRecord::from_response(
        &response,
        phase,
        fixture.reconnect_attempt,
        fixture.attach_session_ref.clone(),
        fixture.visible_version_state.client_version.clone(),
        fixture.visible_version_state.helper_version.clone(),
        fixture.compatibility_report_row_refs.clone(),
    )
}

#[test]
fn attach_adjacent_supported_admits_full_remote() {
    let fixture = load_fixture("attach_adjacent_supported.json");
    let record = beta_record_from_fixture(&fixture);

    assert_eq!(record.schema_version, REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION);
    assert_eq!(record.lifecycle_phase, RemoteHelperLifecyclePhaseClass::Attach);
    assert_eq!(
        record.skew_visibility,
        RemoteHelperSkewVisibilityClass::AdjacentSupported
    );
    assert_eq!(
        record.repair_path,
        RemoteHelperRepairPathClass::NoRepairRequired
    );
    assert!(record.mutation_allowed);
    assert!(!record.fails_closed_for_mutation());
    assert_eq!(record.skew_visibility_token, fixture.skew_visibility);
    assert_eq!(record.repair_path_token, fixture.repair_path);
}

#[test]
fn attach_unsupported_skew_fails_closed_with_upgrade_repair() {
    let fixture = load_fixture("attach_unsupported_skew_blocked.json");
    let record = beta_record_from_fixture(&fixture);

    assert_eq!(
        record.skew_visibility,
        RemoteHelperSkewVisibilityClass::OutsideSupportedWindow
    );
    assert_eq!(
        record.repair_path,
        RemoteHelperRepairPathClass::UpgradeOrRepin
    );
    assert!(!record.mutation_allowed);
    assert!(record.fails_closed_for_mutation());
    assert_eq!(record.negotiation_outcome, NegotiationOutcome::Refuse);
    assert!(record.negotiated_capabilities.is_empty());
}

#[test]
fn reconnect_probe_required_holds_for_drift_probe() {
    let fixture = load_fixture("reconnect_probe_required.json");
    let record = beta_record_from_fixture(&fixture);

    assert_eq!(
        record.lifecycle_phase,
        RemoteHelperLifecyclePhaseClass::Reconnect
    );
    assert_eq!(record.reconnect_attempt, fixture.reconnect_attempt);
    assert_eq!(
        record.skew_visibility,
        RemoteHelperSkewVisibilityClass::ProbeRequiredUntested
    );
    assert_eq!(
        record.repair_path,
        RemoteHelperRepairPathClass::RunDriftProbeOrReattach
    );
    assert!(record.fails_closed_for_mutation());
}

#[test]
fn compatibility_row_and_support_export_share_row_id() {
    let fixtures = [
        load_fixture("attach_adjacent_supported.json"),
        load_fixture("attach_unsupported_skew_blocked.json"),
        load_fixture("reconnect_probe_required.json"),
    ];
    let records: Vec<RemoteHelperBetaRecord> =
        fixtures.iter().map(beta_record_from_fixture).collect();
    let export = RemoteHelperBetaSupportExport::from_records(
        "support_export:remote_helper_skew_beta.integration",
        "2026-05-16T20:00:00Z",
        records.iter(),
    );

    assert_eq!(export.records.len(), 3);
    assert_eq!(export.compatibility_rows.len(), 3);
    for (record, compat) in records.iter().zip(export.compatibility_rows.iter()) {
        let projection = RemoteHelperBetaCompatibilityRow::from_record(record);
        assert_eq!(compat.row_id, record.row_id);
        assert_eq!(projection.row_id, record.row_id);
        assert_eq!(projection.envelope_id, record.envelope_id);
        assert_eq!(
            projection.compatibility_row_ref,
            record.visible_version_state.compatibility_row_ref
        );
    }
    assert!(export.any_record_fails_closed_for_mutation);
}

#[derive(Debug, Deserialize)]
struct FixtureRecord {
    envelope_id: String,
    lifecycle_phase: String,
    reconnect_attempt: u32,
    attach_session_ref: String,
    negotiation_outcome: String,
    effective_posture: String,
    mutation_allowed: bool,
    skew_visibility: String,
    repair_path: String,
    visible_version_state: FixtureVisibleVersion,
    negotiated_capabilities: Vec<String>,
    dropped_capabilities: Vec<FixtureDroppedCapability>,
    visible_summary: String,
    safe_continuation: String,
    evidence_refs: FixtureEvidenceRefs,
    support_packet_refs: Vec<String>,
    compatibility_report_row_refs: Vec<String>,
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

#[derive(Debug, Deserialize)]
struct FixtureDroppedCapability {
    capability: String,
    reason_class: String,
    visible_reason: String,
    retryable: bool,
}

#[derive(Debug, Deserialize)]
struct FixtureEvidenceRefs {
    compat_refs: Vec<String>,
}
