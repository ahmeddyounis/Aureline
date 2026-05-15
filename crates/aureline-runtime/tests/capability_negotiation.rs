use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_rpc::{
    ActorClass, ContractVersion, ErrorClass, Idempotency, ManifestDigest, MethodEntry, MethodKind,
    MethodManifest, MethodName, ScopeKind,
};
use aureline_runtime::{
    CapabilityEffectClass, CapabilityRequirementClass, CompatibilityWindow,
    CompatibilityWindowStatus, EffectiveCapabilityPosture, HelperCapabilityRequest,
    HelperCapabilityRequirement, NegotiationOutcome,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/remote/mixed_version_drift_alpha")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_manifest() -> FixtureManifest {
    let manifest_path = fixture_root().join("manifest.yaml");
    let payload = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", manifest_path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse {}: {err}", manifest_path.display()))
}

fn load_case(case_ref: &str) -> FixtureCase {
    let case_path = repo_root().join(case_ref);
    let payload = std::fs::read_to_string(&case_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", case_path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse {}: {err}", case_path.display()))
}

fn load_skew_window_register_refs() -> BTreeMap<String, String> {
    let path = repo_root().join("artifacts/compat/skew_windows.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    let windows: SkewWindows = serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));
    windows
        .declarations
        .into_iter()
        .map(|window| (window.skew_window_id, window.version_skew_register_ref))
        .collect()
}

fn case_with_expected_label(expected_label: &str) -> FixtureCase {
    let manifest = load_fixture_manifest();
    manifest
        .case_refs
        .iter()
        .map(|case_ref| load_case(case_ref))
        .find(|case| case.harness_expectations.expected_result_label == expected_label)
        .unwrap_or_else(|| panic!("fixture with expected result label {expected_label}"))
}

fn manifest_from_participant(schema_version: u32, participant: &Participant) -> MethodManifest {
    let version = ContractVersion::new(format!("{schema_version}.0.0"))
        .expect("fixture schema version yields a semver contract");
    let digest = ManifestDigest(format!(
        "digest:{}:{}",
        participant.participant_id,
        participant.capability_set.len()
    ));
    participant.capability_set.iter().fold(
        MethodManifest::new(&participant.component_family, version.clone(), digest),
        |manifest, capability| {
            manifest.with_method(MethodEntry {
                name: MethodName::new(capability).expect("fixture capability is method-like"),
                kind: MethodKind::Unary,
                scope: ScopeKind::Workspace,
                actor_classes: vec![ActorClass::User, ActorClass::Remote],
                contract_versions: vec![version.clone()],
                default_deadline_ns: 0,
                max_deadline_ns: 0,
                deadline_required: false,
                idempotency: Idempotency::NotApplicable,
                error_classes: vec![ErrorClass::Unavailable],
            })
        },
    )
}

fn request_from_case(case: &FixtureCase) -> HelperCapabilityRequest {
    let register_refs = load_skew_window_register_refs();
    let version_skew_register_ref = register_refs
        .get(&case.boundary.skew_window_ref)
        .unwrap_or_else(|| {
            panic!(
                "skew window {} has register ref",
                case.boundary.skew_window_ref
            )
        })
        .clone();
    let review_packet_refs = case
        .support_export
        .support_packet_refs
        .iter()
        .map(|item| item.replacen("support_packet:", "review_packet:", 1))
        .collect();

    HelperCapabilityRequest {
        request_id: case.envelope_id.clone(),
        row_id: format!(
            "drift_truth.helper_agent.{}",
            case.envelope_id.replace(':', ".")
        ),
        surface_ref: case.envelope_id.clone(),
        title: case.support_export.summary.clone(),
        client_manifest: manifest_from_participant(
            case.helper_capabilities_alpha_schema_version,
            &case.client,
        ),
        helper_manifest: manifest_from_participant(
            case.helper_capabilities_alpha_schema_version,
            &case.helper,
        ),
        requested_capabilities: case
            .requested_capabilities
            .iter()
            .map(|request| HelperCapabilityRequirement {
                capability: request.capability.clone(),
                requirement: CapabilityRequirementClass::from_token(&request.requirement)
                    .expect("fixture requirement token parses"),
                effect_class: CapabilityEffectClass::from_token(&request.effect_class)
                    .expect("fixture effect token parses"),
                visible_reason: request.visible_reason.clone(),
            })
            .collect(),
        compatibility_window: CompatibilityWindow {
            boundary_family: case.boundary.boundary_family.clone(),
            compatibility_row_ref: case.boundary.compatibility_row_ref.clone(),
            version_skew_register_ref,
            skew_case_ref: case.boundary.skew_case_ref.clone(),
            skew_window_declaration_ref: case.boundary.skew_window_ref.clone(),
            status: CompatibilityWindowStatus::from_token(&case.boundary.skew_status_class)
                .expect("fixture skew status token parses"),
            selected_protocol_ref: case.negotiation.selected_protocol_version.clone(),
            source_refs: case.evidence_refs.compat_refs.clone(),
        },
        downgrade_posture: EffectiveCapabilityPosture::from_token(
            &case.negotiation.effective_posture,
        )
        .expect("fixture effective posture token parses"),
        refusal_posture: EffectiveCapabilityPosture::from_token(
            &case.negotiation.effective_posture,
        )
        .expect("fixture effective posture token parses"),
        visible_summary: case.negotiation.visible_summary.clone(),
        safe_continuation: case.drift.safe_continuation.clone(),
        recovery_refs: case.drift.repair_actions.clone(),
        blocked_action_refs: case.drift.cancelled_mutation_refs.clone(),
        preserved_read_only_refs: case.drift.preserved_read_only_refs.clone(),
        retry_ref: case.negotiation.retryability.retry_after_ref.clone(),
        support_packet_refs: case.support_export.support_packet_refs.clone(),
        review_packet_refs,
        source_refs: case.evidence_refs.fixture_refs.clone(),
    }
}

#[test]
fn helper_capability_negotiation_matches_supported_manifest() {
    let case = case_with_expected_label("supported");
    let response = request_from_case(&case).negotiate();

    assert_eq!(response.outcome, NegotiationOutcome::Match);
    assert_eq!(
        response.negotiated_capabilities,
        case.negotiation.negotiated_capabilities
    );
    assert_eq!(response.dropped_capabilities.len(), 0);
    assert!(response.mutation_allowed);
    assert!(response.is_metadata_only());
}

#[test]
fn helper_capability_negotiation_downgrades_older_helper() {
    let case = case_with_expected_label("limited");
    let response = request_from_case(&case).negotiate();

    assert_eq!(response.outcome, NegotiationOutcome::Downgrade);
    assert_eq!(
        response.negotiated_capabilities,
        case.negotiation.negotiated_capabilities
    );
    assert_eq!(
        response.dropped_capabilities.len(),
        case.negotiation.dropped_capabilities.len()
    );
    assert!(!response.mutation_allowed);
    assert_eq!(
        response.effective_posture.as_str(),
        case.negotiation.effective_posture
    );
    assert_eq!(
        response.primary_recovery_ref.as_deref(),
        case.drift.repair_actions.first().map(String::as_str)
    );
}

#[test]
fn helper_capability_negotiation_refuses_incompatible_helper() {
    let case = case_with_expected_label("unsupported_skew");
    let response = request_from_case(&case).negotiate();

    assert_eq!(response.outcome, NegotiationOutcome::Refuse);
    assert!(response.negotiated_capabilities.is_empty());
    assert_eq!(
        response.dropped_capabilities.len(),
        case.requested_capabilities.len()
    );
    assert!(!response.mutation_allowed);
    assert_eq!(
        response.primary_recovery_ref.as_deref(),
        case.drift.repair_actions.first().map(String::as_str)
    );
    assert!(!response.support_packet_refs.is_empty());
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    case_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    helper_capabilities_alpha_schema_version: u32,
    envelope_id: String,
    boundary: Boundary,
    client: Participant,
    helper: Participant,
    requested_capabilities: Vec<CapabilityRequest>,
    negotiation: Negotiation,
    drift: Drift,
    support_export: SupportExport,
    evidence_refs: EvidenceRefs,
    harness_expectations: HarnessExpectations,
}

#[derive(Debug, Deserialize)]
struct Boundary {
    boundary_family: String,
    compatibility_row_ref: String,
    skew_window_ref: String,
    skew_case_ref: String,
    skew_status_class: String,
}

#[derive(Debug, Deserialize)]
struct Participant {
    participant_id: String,
    component_family: String,
    capability_set: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CapabilityRequest {
    capability: String,
    requirement: String,
    effect_class: String,
    visible_reason: String,
}

#[derive(Debug, Deserialize)]
struct Negotiation {
    selected_protocol_version: String,
    negotiated_capabilities: Vec<String>,
    dropped_capabilities: Vec<serde_yaml::Value>,
    effective_posture: String,
    visible_summary: String,
    retryability: Retryability,
}

#[derive(Debug, Deserialize)]
struct Retryability {
    retry_after_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Drift {
    safe_continuation: String,
    repair_actions: Vec<String>,
    cancelled_mutation_refs: Vec<String>,
    preserved_read_only_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SupportExport {
    support_packet_refs: Vec<String>,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct EvidenceRefs {
    fixture_refs: Vec<String>,
    compat_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HarnessExpectations {
    expected_result_label: String,
}

#[derive(Debug, Deserialize)]
struct SkewWindows {
    declarations: Vec<SkewWindowDeclaration>,
}

#[derive(Debug, Deserialize)]
struct SkewWindowDeclaration {
    skew_window_id: String,
    version_skew_register_ref: String,
}
