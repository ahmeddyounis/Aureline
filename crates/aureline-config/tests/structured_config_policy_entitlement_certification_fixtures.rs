use aureline_config::structured_config_policy_entitlement_certification::{
    parse_structured_config_policy_entitlement_certification,
    seeded_structured_config_policy_entitlement_certification,
    seeded_structured_config_policy_entitlement_certification_scenario, CertificationScenario,
    STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_RECORD_KIND,
    STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_SHARED_CONTRACT_REF,
};

const ARTIFACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/config/structured_config_policy_entitlement_certification.json",
);

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/structured_config_policy_entitlement_certification",
);

fn load_packet(path: &str) -> aureline_config::structured_config_policy_entitlement_certification::StructuredConfigPolicyEntitlementCertificationPacket {
    let body =
        std::fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    parse_structured_config_policy_entitlement_certification(&body)
        .unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn checked_in_artifact_matches_canonical_builder() {
    let checked = load_packet(ARTIFACT_PATH);
    let seeded = seeded_structured_config_policy_entitlement_certification();
    assert_eq!(checked, seeded);
}

#[test]
fn canonical_fixture_matches_seeded_builder() {
    let checked = load_packet(&format!("{FIXTURE_DIR}/canonical.json"));
    let seeded = seeded_structured_config_policy_entitlement_certification();
    assert_eq!(checked, seeded);
}

#[test]
fn degraded_fixtures_match_seeded_scenarios() {
    let cases = [
        (
            "stale_policy.json",
            seeded_structured_config_policy_entitlement_certification_scenario(
                CertificationScenario::StalePolicy,
            ),
        ),
        (
            "reauth_required.json",
            seeded_structured_config_policy_entitlement_certification_scenario(
                CertificationScenario::ReauthRequired,
            ),
        ),
        (
            "signer_rotation.json",
            seeded_structured_config_policy_entitlement_certification_scenario(
                CertificationScenario::SignerRotation,
            ),
        ),
    ];

    for (file, seeded) in cases {
        let checked = load_packet(&format!("{FIXTURE_DIR}/{file}"));
        assert_eq!(checked, seeded, "{file} drifted");
    }
}

#[test]
fn artifact_identity_is_stable() {
    let checked = load_packet(ARTIFACT_PATH);
    assert_eq!(
        checked.record_kind,
        STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_RECORD_KIND
    );
    assert_eq!(
        checked.shared_contract_ref,
        STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_SHARED_CONTRACT_REF
    );
}
