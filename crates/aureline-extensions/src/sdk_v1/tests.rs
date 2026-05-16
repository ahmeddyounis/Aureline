//! Unit and fixture coverage for the SDK v1 starter pack.

use serde::Deserialize;

use super::{
    evaluate_sdk_v1_starter_pack, host_contract_family_for_api_surface,
    project_sdk_v1_starter_pack_support_export, validate_sample_pack_extension_record,
    validate_sdk_v1_api_surface_record, validate_sdk_v1_manifest_authoring_guide_record,
    validate_sdk_v1_starter_pack_record, SamplePackEntryClass, SamplePackValidationClass,
    SdkV1ApiAvailabilityClass, SdkV1ApiSurfaceClass, SdkV1StarterPackDecisionClass,
    SdkV1StarterPackInput, SdkV1StarterPackReasonClass, SDK_V1_STARTER_PACK_RECORD_KIND,
    SDK_V1_STARTER_PACK_SCHEMA_VERSION, SDK_V1_STARTER_PACK_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::{HostContractFamilyClass, RedactionClass};

#[derive(Debug, Deserialize)]
struct StarterPackFixture {
    input: SdkV1StarterPackInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: SdkV1StarterPackDecisionClass,
    expected_reason_class: SdkV1StarterPackReasonClass,
    expected_claimed_api_surface_count: u32,
    expected_available_in_beta_surface_count: u32,
    expected_preview_in_beta_surface_count: u32,
    expected_wasm_sample_count: u32,
    expected_external_host_sample_count: u32,
    expected_authoring_guide_count: u32,
}

fn load_fixture(name: &str) -> StarterPackFixture {
    let raw = match name {
        "ready_for_authors_wasm_and_external_host" => include_str!(
            "../../../../fixtures/extensions/m3/sample_pack/ready_for_authors_wasm_and_external_host.json"
        ),
        "partially_ready_preview_surface" => include_str!(
            "../../../../fixtures/extensions/m3/sample_pack/partially_ready_preview_surface.json"
        ),
        "refused_missing_wasm_sample" => include_str!(
            "../../../../fixtures/extensions/m3/sample_pack/refused_missing_wasm_sample.json"
        ),
        "refused_authoring_guide_missing" => include_str!(
            "../../../../fixtures/extensions/m3/sample_pack/refused_authoring_guide_missing.json"
        ),
        "refused_retired_surface" => include_str!(
            "../../../../fixtures/extensions/m3/sample_pack/refused_retired_surface.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    // Every surface, guide, and sample in the input MUST validate
    // structurally before the evaluator runs (the evaluator's "refused"
    // states model invariants the validators surface as findings).
    for surface in &fixture.input.claimed_api_surfaces {
        let findings = validate_sdk_v1_api_surface_record(surface);
        assert!(
            findings.is_empty(),
            "fixture {name} api surface {} produced findings: {findings:?}",
            surface.api_surface_id
        );
    }
    for guide in &fixture.input.authoring_guides {
        let findings = validate_sdk_v1_manifest_authoring_guide_record(guide);
        assert!(
            findings.is_empty(),
            "fixture {name} authoring guide {} produced findings: {findings:?}",
            guide.manifest_guide_id
        );
    }
    for sample in &fixture.input.sample_pack_entries {
        let findings = validate_sample_pack_extension_record(sample);
        assert!(
            findings.is_empty(),
            "fixture {name} sample {} produced findings: {findings:?}",
            sample.sample_pack_id
        );
    }

    let record = evaluate_sdk_v1_starter_pack(fixture.input);
    assert_eq!(record.record_kind, SDK_V1_STARTER_PACK_RECORD_KIND);
    assert_eq!(
        record.sdk_v1_starter_pack_schema_version,
        SDK_V1_STARTER_PACK_SCHEMA_VERSION
    );
    assert_eq!(record.redaction_class, RedactionClass::MetadataSafeDefault);
    assert_eq!(
        record.decision_class, fixture.meta.expected_decision_class,
        "decision mismatch for {name}"
    );
    assert_eq!(
        record.reason_class, fixture.meta.expected_reason_class,
        "reason mismatch for {name}"
    );
    assert_eq!(
        record.claimed_api_surface_count, fixture.meta.expected_claimed_api_surface_count,
        "claimed_api_surface_count mismatch for {name}"
    );
    assert_eq!(
        record.available_in_beta_surface_count, fixture.meta.expected_available_in_beta_surface_count,
        "available_in_beta_surface_count mismatch for {name}"
    );
    assert_eq!(
        record.preview_in_beta_surface_count, fixture.meta.expected_preview_in_beta_surface_count,
        "preview_in_beta_surface_count mismatch for {name}"
    );
    assert_eq!(
        record.wasm_sample_count, fixture.meta.expected_wasm_sample_count,
        "wasm_sample_count mismatch for {name}"
    );
    assert_eq!(
        record.external_host_sample_count, fixture.meta.expected_external_host_sample_count,
        "external_host_sample_count mismatch for {name}"
    );
    assert_eq!(
        record.authoring_guide_count, fixture.meta.expected_authoring_guide_count,
        "authoring_guide_count mismatch for {name}"
    );

    let findings = validate_sdk_v1_starter_pack_record(&record);
    assert!(
        findings.is_empty(),
        "fixture {name} produced starter-pack validation findings: {findings:?}"
    );

    let export = project_sdk_v1_starter_pack_support_export(
        &record,
        &format!("sdk_v1_starter_pack_support_export:{}", record.starter_pack_id),
    );
    assert_eq!(
        export.record_kind,
        SDK_V1_STARTER_PACK_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.starter_pack_ref, record.starter_pack_id);
    assert_eq!(export.decision_class, record.decision_class);
    assert_eq!(export.reason_class, record.reason_class);
    assert_eq!(
        export.blocks_authoring,
        matches!(
            record.decision_class,
            SdkV1StarterPackDecisionClass::RefusedInconsistentInput
        )
    );
    assert_eq!(
        export.preview_disclosure_required,
        matches!(
            record.decision_class,
            SdkV1StarterPackDecisionClass::PartiallyReadyPreviewSurfacesOnly
        )
    );
    assert_eq!(export.redaction_class, RedactionClass::MetadataSafeDefault);
}

#[test]
fn ready_for_authors_round_trips() {
    run_fixture("ready_for_authors_wasm_and_external_host");
}

#[test]
fn partially_ready_round_trips() {
    run_fixture("partially_ready_preview_surface");
}

#[test]
fn refused_missing_wasm_sample_round_trips() {
    run_fixture("refused_missing_wasm_sample");
}

#[test]
fn refused_authoring_guide_missing_round_trips() {
    run_fixture("refused_authoring_guide_missing");
}

#[test]
fn refused_retired_surface_round_trips() {
    run_fixture("refused_retired_surface");
}

#[test]
fn host_contract_family_mapping_is_total_and_stable() {
    use SdkV1ApiSurfaceClass as S;
    assert_eq!(
        host_contract_family_for_api_surface(S::WasmComponentModelHostApi),
        HostContractFamilyClass::WasmComponentModel
    );
    assert_eq!(
        host_contract_family_for_api_surface(S::WasmCoreModuleHostApi),
        HostContractFamilyClass::WasmCoreModule
    );
    assert_eq!(
        host_contract_family_for_api_surface(S::ExternalHostSupervisedApi),
        HostContractFamilyClass::ExternalHostProcess
    );
    assert_eq!(
        host_contract_family_for_api_surface(S::HelperBinaryApi),
        HostContractFamilyClass::HelperBinary
    );
    assert_eq!(
        host_contract_family_for_api_surface(S::CompatibilityBridgeApi),
        HostContractFamilyClass::CompatibilityBridge
    );
    assert_eq!(
        host_contract_family_for_api_surface(S::RemoteSideComponentApi),
        HostContractFamilyClass::RemoteSideComponent
    );
}

#[test]
fn refused_when_pack_id_unprefixed() {
    let mut fixture = load_fixture("ready_for_authors_wasm_and_external_host");
    fixture.input.starter_pack_id = "broken-id-no-prefix".to_string();
    let record = evaluate_sdk_v1_starter_pack(fixture.input);
    assert_eq!(
        record.decision_class,
        SdkV1StarterPackDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.reason_class,
        SdkV1StarterPackReasonClass::RefusedPackIdUnprefixed
    );
}

#[test]
fn refused_when_sample_host_family_disagrees_with_api_surface() {
    let mut fixture = load_fixture("ready_for_authors_wasm_and_external_host");
    // Flip the wasm sample to claim it is a wasm sample by api class but
    // claim an external_host_process host family — the evaluator must
    // refuse closed.
    let wasm_sample = fixture
        .input
        .sample_pack_entries
        .iter_mut()
        .find(|e| e.host_contract_family_class == HostContractFamilyClass::WasmComponentModel)
        .expect("ready fixture must include a wasm sample");
    wasm_sample.host_contract_family_class = HostContractFamilyClass::ExternalHostProcess;
    let record = evaluate_sdk_v1_starter_pack(fixture.input);
    assert_eq!(
        record.decision_class,
        SdkV1StarterPackDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.reason_class,
        SdkV1StarterPackReasonClass::RefusedSampleHostFamilyDisagreesWithApiSurface
    );
}

#[test]
fn refused_when_runnable_sample_marked_documentation_only() {
    let mut fixture = load_fixture("ready_for_authors_wasm_and_external_host");
    let runnable = fixture
        .input
        .sample_pack_entries
        .iter_mut()
        .find(|e| !matches!(e.sample_entry_class, SamplePackEntryClass::ManifestAuthoringWalkthrough))
        .expect("ready fixture must include a runnable sample");
    runnable.sample_validation_class = SamplePackValidationClass::DocumentationOnly;
    let record = evaluate_sdk_v1_starter_pack(fixture.input);
    assert_eq!(
        record.decision_class,
        SdkV1StarterPackDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.reason_class,
        SdkV1StarterPackReasonClass::RefusedSampleValidationDocumentationOnlyOnRunnableEntry
    );
}

#[test]
fn refused_when_claimed_external_host_lane_has_no_external_host_sample() {
    // Drop the external-host sample from the ready fixture; the starter
    // pack still claims an external-host API surface so the evaluator
    // must refuse closed.
    let mut fixture = load_fixture("ready_for_authors_wasm_and_external_host");
    fixture
        .input
        .sample_pack_entries
        .retain(|e| e.host_contract_family_class != HostContractFamilyClass::ExternalHostProcess);
    let record = evaluate_sdk_v1_starter_pack(fixture.input);
    assert_eq!(
        record.decision_class,
        SdkV1StarterPackDecisionClass::RefusedInconsistentInput
    );
    assert_eq!(
        record.reason_class,
        SdkV1StarterPackReasonClass::RefusedNoExternalHostSampleForClaimedExternalHostLane
    );
}

#[test]
fn validator_flags_redaction_class_drift() {
    let fixture = load_fixture("ready_for_authors_wasm_and_external_host");
    let mut record = evaluate_sdk_v1_starter_pack(fixture.input);
    record.redaction_class = RedactionClass::SupportBundle;
    let findings = validate_sdk_v1_starter_pack_record(&record);
    assert!(
        findings
            .iter()
            .any(|f| f.check_id == "sdk_v1_starter_pack.redaction_class_must_be_metadata_safe"),
        "validator must flag redaction class drift; got {findings:?}"
    );
}

#[test]
fn api_surface_validator_flags_wasm_surface_without_wit_world_refs() {
    let mut fixture = load_fixture("ready_for_authors_wasm_and_external_host");
    let wasm_surface = fixture
        .input
        .claimed_api_surfaces
        .iter_mut()
        .find(|s| s.api_surface_class == SdkV1ApiSurfaceClass::WasmComponentModelHostApi)
        .expect("ready fixture must include a wasm surface");
    wasm_surface.covered_wit_world_refs.clear();
    let findings = validate_sdk_v1_api_surface_record(&fixture.input.claimed_api_surfaces[0]);
    assert!(
        findings.iter().any(|f| f.check_id
            == "sdk_v1_api_surface.wasm_surface_must_cite_wit_world_refs"),
        "validator must flag wasm surface without WIT world refs; got {findings:?}"
    );
}

#[test]
fn availability_classes_are_exhaustive() {
    use SdkV1ApiAvailabilityClass as A;
    // Ensure the closed availability vocabulary stays exhaustive in
    // serialization (catches accidental enum removals).
    for class in [
        A::AvailableInBeta,
        A::PreviewInBeta,
        A::NotAvailableUntilGeneral,
        A::RetiredPendingSuccessor,
    ] {
        let json = serde_json::to_string(&class).expect("availability class must serialize");
        let round: A = serde_json::from_str(&json).expect("availability class must round trip");
        assert_eq!(class, round);
    }
}
