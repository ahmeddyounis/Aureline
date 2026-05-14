//! Protected fixtures for schema registry and endpoint-policy inspection.

use std::collections::BTreeSet;
use std::path::Path;

use aureline_shell::inspectors::schema_registry::{
    ConsentPolicyState, DestinationClass, EndpointPolicyInspectionInput, OperationalSignalKind,
    SchemaRegistryInspector, SignalFreshnessClass, SignalRedactionClass,
    ALPHA_SCHEMA_REGISTRY_SOURCE_REF, CONSENT_LEDGER_SOURCE_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    cases: Vec<FixtureManifestCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureManifestCase {
    case_id: String,
    fixture_ref: String,
    required_claim_refs: Vec<String>,
    required_signal_kinds: Vec<String>,
    required_freshness_classes: Vec<String>,
    required_redaction_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EndpointPolicyFixture {
    case_id: String,
    input: EndpointPolicyInspectionInput,
    expect: EndpointPolicyExpectations,
}

#[derive(Debug, Deserialize)]
struct EndpointPolicyExpectations {
    min_schema_rows: usize,
    min_endpoint_policy_rows: usize,
    support_export_raw_payloads_excluded: bool,
    required_destination_classes: Vec<String>,
    required_consent_policy_states: Vec<String>,
    required_local_only_alternative_fragments: Vec<String>,
    cross_surface_signal_vocabulary_parity: bool,
}

fn fixture_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/inspectors/endpoint_policy_alpha")
}

fn load_yaml<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let payload = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn schema_registry_endpoint_policy_alpha_fixtures_project_same_signal_vocabulary() {
    let root = fixture_root();
    let manifest: FixtureManifest = load_yaml(&root.join("manifest.yaml"));
    let inspector = SchemaRegistryInspector::from_default_artifact_registers()
        .expect("load checked-in schema and consent registers");

    for case in manifest.cases {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let fixture: EndpointPolicyFixture = load_yaml(&repo_root.join(&case.fixture_ref));
        assert_eq!(fixture.case_id, case.case_id);

        let snapshot = inspector
            .inspect(fixture.input)
            .unwrap_or_else(|err| panic!("case {} failed inspection: {err}", case.case_id));

        assert!(
            snapshot.schema_rows.len() >= fixture.expect.min_schema_rows,
            "case {} schema row count",
            case.case_id
        );
        assert!(
            snapshot.endpoint_policy_rows.len() >= fixture.expect.min_endpoint_policy_rows,
            "case {} endpoint policy row count",
            case.case_id
        );
        assert_eq!(
            snapshot.support_export.raw_payloads_excluded,
            fixture.expect.support_export_raw_payloads_excluded,
            "case {} support export raw-payload posture",
            case.case_id
        );
        assert_eq!(
            snapshot.has_cross_surface_signal_vocabulary_parity(),
            fixture.expect.cross_surface_signal_vocabulary_parity,
            "case {} cross-surface signal vocabulary parity",
            case.case_id
        );

        assert_required_claims(&case, &snapshot.schema_rows);
        assert_required_destinations(&case, &fixture.expect, &snapshot.endpoint_policy_rows);
        assert_required_consent_states(&case, &fixture.expect, &snapshot.endpoint_policy_rows);
        assert_required_local_alternatives(&case, &fixture.expect, &snapshot.endpoint_policy_rows);
        assert_required_signal_vocab(&case, &snapshot.operational_signal_slices);
        assert_current_artifact_registers_are_quoted(&case, &snapshot.schema_rows);
    }
}

fn assert_required_claims(
    case: &FixtureManifestCase,
    rows: &[aureline_shell::inspectors::schema_registry::SchemaInspectionRow],
) {
    let observed: BTreeSet<_> = rows.iter().map(|row| row.claim_ref.as_str()).collect();
    for required in &case.required_claim_refs {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing claim {}",
            case.case_id,
            required
        );
    }
}

fn assert_required_destinations(
    case: &FixtureManifestCase,
    expect: &EndpointPolicyExpectations,
    rows: &[aureline_shell::inspectors::schema_registry::EndpointPolicyRow],
) {
    let observed: BTreeSet<_> = rows
        .iter()
        .map(|row| destination_token(row.destination_class))
        .collect();
    for required in &expect.required_destination_classes {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing destination {}",
            case.case_id,
            required
        );
    }
}

fn assert_required_consent_states(
    case: &FixtureManifestCase,
    expect: &EndpointPolicyExpectations,
    rows: &[aureline_shell::inspectors::schema_registry::EndpointPolicyRow],
) {
    let observed: BTreeSet<_> = rows
        .iter()
        .map(|row| consent_token(row.consent_or_policy_state))
        .collect();
    for required in &expect.required_consent_policy_states {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing consent/policy state {}",
            case.case_id,
            required
        );
    }
}

fn assert_required_local_alternatives(
    case: &FixtureManifestCase,
    expect: &EndpointPolicyExpectations,
    rows: &[aureline_shell::inspectors::schema_registry::EndpointPolicyRow],
) {
    for fragment in &expect.required_local_only_alternative_fragments {
        assert!(
            rows.iter()
                .any(|row| row.local_only_alternative.contains(fragment)),
            "case {} missing local-only alternative fragment {:?}",
            case.case_id,
            fragment
        );
    }
}

fn assert_required_signal_vocab(
    case: &FixtureManifestCase,
    rows: &[aureline_shell::inspectors::schema_registry::OperationalSignalSlice],
) {
    let signal_kinds: BTreeSet<_> = rows
        .iter()
        .map(|row| signal_kind_token(row.signal_kind))
        .collect();
    for required in &case.required_signal_kinds {
        assert!(
            signal_kinds.contains(required.as_str()),
            "case {} missing signal kind {}",
            case.case_id,
            required
        );
    }

    let freshness: BTreeSet<_> = rows
        .iter()
        .map(|row| freshness_token(row.freshness))
        .collect();
    for required in &case.required_freshness_classes {
        assert!(
            freshness.contains(required.as_str()),
            "case {} missing freshness class {}",
            case.case_id,
            required
        );
    }

    let redaction: BTreeSet<_> = rows
        .iter()
        .map(|row| redaction_token(row.redaction_class))
        .collect();
    for required in &case.required_redaction_classes {
        assert!(
            redaction.contains(required.as_str()),
            "case {} missing redaction class {}",
            case.case_id,
            required
        );
    }
}

fn assert_current_artifact_registers_are_quoted(
    case: &FixtureManifestCase,
    rows: &[aureline_shell::inspectors::schema_registry::SchemaInspectionRow],
) {
    let telemetry = rows
        .iter()
        .find(|row| row.claim_ref == "telemetry.ux_product_event")
        .unwrap_or_else(|| panic!("case {} missing telemetry row", case.case_id));
    assert_eq!(telemetry.source_register_ref, CONSENT_LEDGER_SOURCE_REF);
    assert_eq!(telemetry.title, "UX product-usage telemetry event");

    let support = rows
        .iter()
        .find(|row| row.claim_ref == "support.bundle_manifest")
        .unwrap_or_else(|| panic!("case {} missing support row", case.case_id));
    assert_eq!(support.source_register_ref, CONSENT_LEDGER_SOURCE_REF);
    assert_eq!(
        support.endpoint_class.as_deref(),
        Some("export_only_user_initiated")
    );

    let alpha_support = rows
        .iter()
        .find(|row| row.claim_ref == "schema_alpha:support_export.bundle_manifest")
        .unwrap_or_else(|| panic!("case {} missing alpha support schema row", case.case_id));
    assert_eq!(
        alpha_support.source_register_ref,
        ALPHA_SCHEMA_REGISTRY_SOURCE_REF
    );
    assert_eq!(
        alpha_support.schema_role.as_deref(),
        Some("support_export_packet_schema")
    );
}

fn destination_token(value: DestinationClass) -> &'static str {
    value.as_str()
}

fn consent_token(value: ConsentPolicyState) -> &'static str {
    value.as_str()
}

fn signal_kind_token(value: OperationalSignalKind) -> &'static str {
    value.as_str()
}

fn freshness_token(value: SignalFreshnessClass) -> &'static str {
    value.as_str()
}

fn redaction_token(value: SignalRedactionClass) -> &'static str {
    value.as_str()
}
