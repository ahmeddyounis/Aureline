//! Inline unit tests for the typed M5 WIT contract publication packet.

use super::*;

fn packet() -> WitContractPublicationPacket {
    current_wit_contract_publication().expect("checked-in packet parses into the model")
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let p = packet();
    assert_eq!(p.schema_version, WIT_CONTRACT_PUBLICATION_SCHEMA_VERSION);
    assert_eq!(p.record_kind, WIT_CONTRACT_PUBLICATION_RECORD_KIND);
    let violations = p.validate();
    assert!(
        violations.is_empty(),
        "checked-in packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn computed_summary_matches_recorded_summary() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn all_four_outcomes_are_covered() {
    let p = packet();
    for outcome in NegotiationOutcome::ALL {
        assert!(
            p.fixture_for_outcome(outcome).is_some(),
            "outcome {} must be covered by a fixture",
            outcome.as_str()
        );
    }
}

#[test]
fn standalone_fixtures_equal_embedded_fixtures() {
    let p = packet();
    for outcome in NegotiationOutcome::ALL {
        let standalone = load_negotiation_fixture(outcome).expect("standalone fixture parses");
        let embedded = p
            .fixture_for_outcome(outcome)
            .expect("embedded fixture present");
        assert_eq!(&standalone, embedded, "{}", outcome.as_str());
        assert_eq!(standalone.record_kind, WIT_NEGOTIATION_FIXTURE_RECORD_KIND);
        assert!(
            standalone.issues().is_empty(),
            "fixture {} must conform: {:?}",
            outcome.as_str(),
            standalone.issues()
        );
    }
}

#[test]
fn every_published_package_has_a_wit_file_ref() {
    let p = packet();
    for pkg in &p.packages {
        assert!(
            pkg.wit_package_ref.ends_with(".wit"),
            "package {} must cite a .wit file",
            pkg.package_identity
        );
    }
}

#[test]
fn editor_read_has_two_published_versions_and_a_deprecation() {
    let p = packet();
    let versions = p.packages_for_slug("editor-read");
    assert_eq!(versions.len(), 2, "editor-read publishes 0.1.0 and 0.2.0");
    assert_eq!(p.deprecated_packages().len(), 1);
    assert!(p
        .deprecated_packages()
        .iter()
        .any(|pkg| pkg.package_identity == "aureline:editor-read@0.1.0"));
}

#[test]
fn additive_minor_diff_is_adds_only_and_backward_compatible() {
    let p = packet();
    let diff = p
        .capability_diffs_for_slug("editor-read")
        .into_iter()
        .find(|d| d.change_class == ChangeClass::AdditiveMinor)
        .expect("editor-read has an additive-minor diff");
    assert!(diff.removed_capabilities.is_empty());
    assert!(diff.changed_capabilities.is_empty());
    assert!(!diff.added_capabilities.is_empty());
    assert_eq!(
        diff.compatibility_verdict,
        CompatibilityVerdict::BackwardCompatible
    );
    assert_eq!(diff.guest_action_required, GuestAction::None);
    assert!(diff.issues().is_empty());
}

#[test]
fn supported_fixture_admits_full_set_without_widening() {
    let p = packet();
    let fx = p
        .fixture_for_outcome(NegotiationOutcome::Supported)
        .unwrap();
    assert_eq!(
        fx.negotiated_capability_worlds,
        fx.declared_capability_worlds
    );
    assert!(!fx.fail_closed);
    assert!(!fx.guest_authority_widened);
}

#[test]
fn downgraded_fixture_narrows_with_typed_reasons_and_fails_closed() {
    let p = packet();
    let fx = p
        .fixture_for_outcome(NegotiationOutcome::Downgraded)
        .unwrap();
    assert!(fx.negotiated_capability_worlds.len() < fx.declared_capability_worlds.len());
    assert!(!fx.narrowing_reasons.is_empty());
    for entry in &fx.narrowing_reasons {
        assert!(!entry.repair_affordance_label.is_empty());
    }
    assert!(fx.fail_closed);
    assert!(fx.issues().is_empty());
}

#[test]
fn deprecated_fixture_admits_world_with_successor_notice() {
    let p = packet();
    let fx = p
        .fixture_for_outcome(NegotiationOutcome::Deprecated)
        .unwrap();
    assert!(!fx.deprecated_world_notices.is_empty());
    for notice in &fx.deprecated_world_notices {
        assert!(!notice.successor_world_ref.is_empty());
        assert!(fx.negotiated_capability_worlds.contains(&notice.world));
    }
    assert!(!fx.fail_closed);
}

#[test]
fn unsupported_skew_denies_world_fail_closed_no_widening() {
    let p = packet();
    let fx = p
        .fixture_for_outcome(NegotiationOutcome::UnsupportedSkew)
        .unwrap();
    assert!(!fx.unsupported_world_decisions.is_empty());
    let decision = &fx.unsupported_world_decisions[0];
    assert!(decision.unsupported_reason.is_skew());
    assert!(!fx
        .negotiated_capability_worlds
        .contains(&decision.declared_world_ref));
    assert!(fx.fail_closed);
    assert!(!fx.guest_authority_widened);
}

#[test]
fn support_export_has_one_row_per_package() {
    let p = packet();
    let export = p.support_export_projection();
    assert_eq!(export.packages.len(), p.packages.len());
    assert_eq!(export.outcomes_covered.len(), 4);
}

// --- Negative gates. --------------------------------------------------------

#[test]
fn widened_guest_authority_fails() {
    let mut p = packet();
    p.negotiation_fixtures[0].guest_authority_widened = true;
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::FixtureIssue { code, .. } if code == "guest_authority_widened"
        )),
        "a fixture that widens guest authority must fail"
    );
}

#[test]
fn negotiated_world_outside_declared_fails() {
    let mut p = packet();
    let fx = p
        .negotiation_fixtures
        .iter_mut()
        .find(|f| f.outcome == NegotiationOutcome::Supported)
        .unwrap();
    fx.negotiated_capability_worlds
        .push("aureline:network-egress@0.1.0".to_string());
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::FixtureIssue { code, .. }
                if code == "negotiated_widens_beyond_declared"
        )),
        "a negotiated world outside the declared set must fail"
    );
}

#[test]
fn silently_dropped_world_fails() {
    let mut p = packet();
    let fx = p
        .negotiation_fixtures
        .iter_mut()
        .find(|f| f.outcome == NegotiationOutcome::Downgraded)
        .unwrap();
    fx.narrowing_reasons.remove(0);
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::FixtureIssue { code, .. } if code.starts_with("silent_drop:")
        )),
        "a declared world dropped without a disposition must fail"
    );
}

#[test]
fn narrowed_world_left_in_negotiated_set_fails() {
    let mut p = packet();
    let fx = p
        .negotiation_fixtures
        .iter_mut()
        .find(|f| f.outcome == NegotiationOutcome::Downgraded)
        .unwrap();
    let narrowed = fx.narrowing_reasons[0].world.clone();
    fx.negotiated_capability_worlds.push(narrowed);
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::FixtureIssue { code, .. }
                if code.starts_with("narrowed_world_still_negotiated:")
        )),
        "a narrowed world that remains negotiated must fail"
    );
}

#[test]
fn deprecated_fixture_without_notice_fails() {
    let mut p = packet();
    let fx = p
        .negotiation_fixtures
        .iter_mut()
        .find(|f| f.outcome == NegotiationOutcome::Deprecated)
        .unwrap();
    fx.deprecated_world_notices.clear();
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::FixtureIssue { code, .. } if code == "deprecated_without_notice"
        )),
        "a deprecated outcome without a notice must fail"
    );
}

#[test]
fn additive_minor_diff_that_removes_a_capability_fails() {
    let mut p = packet();
    let diff = p
        .capability_diffs
        .iter_mut()
        .find(|d| d.change_class == ChangeClass::AdditiveMinor)
        .unwrap();
    diff.removed_capabilities
        .push("interface editor-read: func cursors".to_string());
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::DiffIssue { code, .. }
                if code == "additive_minor_removed_or_changed"
        )),
        "an additive-minor diff that removes a capability must fail"
    );
}

#[test]
fn deprecation_diff_without_successor_fails() {
    let mut p = packet();
    let diff = p
        .capability_diffs
        .iter_mut()
        .find(|d| d.change_class == ChangeClass::Deprecation)
        .unwrap();
    diff.to_package_ref.clear();
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::DiffIssue { code, .. } if code == "deprecation_without_successor"
        )),
        "a deprecation diff without a successor must fail"
    );
}

#[test]
fn duplicate_package_identity_fails() {
    let mut p = packet();
    let first = p.packages[1].package_identity.clone();
    p.packages[2].package_identity = first;
    assert!(
        p.validate()
            .iter()
            .any(|v| matches!(v, WitContractViolation::DuplicatePackageIdentity(_))),
        "two packages may not share an identity"
    );
}

#[test]
fn missing_outcome_fails() {
    let mut p = packet();
    p.negotiation_fixtures
        .retain(|f| f.outcome != NegotiationOutcome::UnsupportedSkew);
    p.summary = p.computed_summary();
    assert!(
        p.validate().iter().any(|v| matches!(
            v,
            WitContractViolation::MissingOutcome(NegotiationOutcome::UnsupportedSkew)
        )),
        "dropping a required outcome must fail"
    );
}
