//! Cross-crate contract test for the M5 WIT contract publication packet.
//!
//! Confirms the checked-in packet, the four standalone negotiation fixtures, and
//! the capability diffs parse, validate, and prove the four required outcomes,
//! exactly as the dedicated CI gate and the Python validator do.

use aureline_extensions::implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts::{
    current_wit_contract_publication, load_negotiation_fixture, ChangeClass, CompatibilityVerdict,
    LifecycleLabel, NegotiationOutcome, PublicationState, WitContractPublicationPacket,
};

fn packet() -> WitContractPublicationPacket {
    current_wit_contract_publication().expect("checked-in packet parses")
}

#[test]
fn packet_validates_cleanly() {
    let violations = packet().validate();
    assert!(violations.is_empty(), "{violations:#?}");
}

#[test]
fn family_id_binds_the_public_contract_matrix_row() {
    let p = packet();
    assert_eq!(p.family_id, "extension_host_wit_world");
    assert_eq!(p.contract_matrix_row, "extension_host_wit_world");
    assert!(p
        .contract_matrix_ref
        .ends_with("m5-stability-lifecycle-map.json"));
}

#[test]
fn every_required_outcome_has_a_conforming_fixture() {
    let p = packet();
    for outcome in NegotiationOutcome::ALL {
        let embedded = p
            .fixture_for_outcome(outcome)
            .unwrap_or_else(|| panic!("missing {}", outcome.as_str()));
        assert!(embedded.issues().is_empty(), "{}", outcome.as_str());
        let standalone = load_negotiation_fixture(outcome).expect("standalone fixture parses");
        assert_eq!(&standalone, embedded);
    }
}

#[test]
fn at_least_one_fixture_fails_closed() {
    let p = packet();
    let fail_closed = p
        .negotiation_fixtures
        .iter()
        .filter(|f| f.fail_closed)
        .count();
    assert!(
        fail_closed >= 2,
        "downgraded and unsupported-skew fail closed"
    );
}

#[test]
fn published_packages_carry_a_lifecycle_label_and_wit_ref() {
    let p = packet();
    assert!(!p.packages.is_empty());
    for pkg in &p.packages {
        assert!(pkg.wit_package_ref.ends_with(".wit"));
        assert!(LifecycleLabel::ALL.contains(&pkg.lifecycle_label));
        if pkg.publication_state == PublicationState::Published {
            assert!(!pkg.compatibility_note.is_empty());
        }
    }
}

#[test]
fn capability_diff_matches_published_versions() {
    let p = packet();
    let diff = p
        .capability_diffs_for_slug("editor-read")
        .into_iter()
        .find(|d| d.change_class == ChangeClass::AdditiveMinor)
        .expect("editor-read additive-minor diff present");
    // The diff's endpoints must be published packages.
    let identities: Vec<&str> = p
        .packages
        .iter()
        .map(|pkg| pkg.package_identity.as_str())
        .collect();
    assert!(identities.contains(&diff.from_package_ref.as_str()));
    assert!(identities.contains(&diff.to_package_ref.as_str()));
    assert_eq!(
        diff.compatibility_verdict,
        CompatibilityVerdict::BackwardCompatible
    );
}
