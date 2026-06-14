//! Unit tests for the request-profile certification qualification packet.

use std::collections::BTreeSet;

use super::*;

fn packet() -> RequestProfileCertQualificationPacket {
    current_request_profile_certification_qualification()
        .expect("embedded request-profile certification packet must parse")
}

#[test]
fn embedded_packet_parses_and_is_populated() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        REQUEST_PROFILE_CERT_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        REQUEST_PROFILE_CERT_QUALIFICATION_RECORD_KIND
    );
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.profiles.is_empty());
    assert!(!packet.cases.is_empty());
    assert!(!packet.downgrade_rules.is_empty());
    assert!(!packet.upstream_refs.is_empty());
}

#[test]
fn embedded_packet_has_no_violations() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn embedded_summary_matches_computed() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_drill_corpus_class_is_covered() {
    let covered = packet().covered_corpus_classes();
    for required in [
        CertificationCorpusClass::SchemaStale,
        CertificationCorpusClass::OriginChangedRerun,
        CertificationCorpusClass::PersistedOperationDrift,
        CertificationCorpusClass::PersistedOperationDeprecation,
        CertificationCorpusClass::MirrorOfflineSnapshot,
        CertificationCorpusClass::ExportRedaction,
    ] {
        assert!(covered.contains(&required), "missing corpus {required:?}");
    }
}

#[test]
fn certification_is_not_desktop_only() {
    let packet = packet();
    assert!(packet
        .profiles
        .iter()
        .any(|profile| profile.profile_class.is_non_desktop()));
    assert!(
        !packet.offline_corpus_case_ids().is_empty(),
        "mirror/offline corpus must be exercised"
    );
    assert!(packet
        .profiles
        .iter()
        .all(|profile| !profile.live_online_only_fixtures));
}

#[test]
fn drift_deprecation_and_stale_cases_block_unsafe_fallback() {
    let packet = packet();
    for case in &packet.cases {
        if case.corpus_class.must_block_unsafe_fallback() {
            assert!(
                case.blocks_unsafe_fallback,
                "{} must block unsafe fallback",
                case.case_id
            );
        }
    }
    assert!(!packet.unsafe_fallback_blocking_case_ids().is_empty());
}

#[test]
fn history_retention_cases_keep_the_safe_default() {
    for case in &packet().cases {
        if case.dimension == CertificationDimension::HistoryRetention {
            assert!(
                case.preserves_safe_retention_default,
                "{} must preserve the safe retention default",
                case.case_id
            );
        }
    }
}

#[test]
fn managed_and_companion_profiles_isolate_desktop_local_trust() {
    let packet = packet();
    for profile in &packet.profiles {
        if profile.profile_class.must_isolate_local_trust() {
            assert!(
                profile.trust_isolated_from_desktop_local,
                "{} must isolate desktop-local trust",
                profile.profile_id
            );
        }
    }
    assert!(!packet.trust_isolated_case_ids().is_empty());
}

#[test]
fn consumes_the_request_lane_upstream_packets() {
    let packet = packet();
    for kind in [
        "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix",
        "ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows",
        "implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review",
        "add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules",
        "implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare",
        "ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability",
    ] {
        assert!(
            packet
                .upstream_refs
                .iter()
                .any(|row| row.upstream_record_kind == kind && row.integration_verified),
            "must reference {kind} as a verified upstream packet"
        );
    }
}

#[test]
fn downgrade_rules_are_automatic() {
    let packet = packet();
    assert!(!packet.downgrade_rules.is_empty());
    assert!(
        packet.downgrade_rules.iter().all(|rule| rule.automatic),
        "every downgrade rule must narrow automatically"
    );
}

#[test]
fn every_case_references_a_known_profile() {
    let packet = packet();
    let ids: BTreeSet<&str> = packet
        .profiles
        .iter()
        .map(|profile| profile.profile_id.as_str())
        .collect();
    for case in &packet.cases {
        assert!(
            ids.contains(case.profile_ref.as_str()),
            "{} references unknown profile {}",
            case.case_id,
            case.profile_ref
        );
    }
}

#[test]
fn live_only_fixture_certification_is_a_violation() {
    let mut packet = packet();
    packet.profiles[0].live_online_only_fixtures = true;
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        RequestProfileCertQualificationViolation::LiveOnlyFixtureCertification { .. }
    )));
}

#[test]
fn dropping_a_corpus_class_is_a_violation() {
    let mut packet = packet();
    // Drop all mirror/offline snapshot cases.
    packet
        .cases
        .retain(|case| case.corpus_class != CertificationCorpusClass::MirrorOfflineSnapshot);
    packet.summary = packet.computed_summary();
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        RequestProfileCertQualificationViolation::MissingCorpusClass {
            corpus_class: CertificationCorpusClass::MirrorOfflineSnapshot
        }
    )));
}

#[test]
fn unsafe_fallback_on_drift_is_a_violation() {
    let mut packet = packet();
    let case = packet
        .cases
        .iter_mut()
        .find(|case| case.corpus_class == CertificationCorpusClass::PersistedOperationDrift)
        .expect("a persisted-operation drift case must exist");
    case.blocks_unsafe_fallback = false;
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        RequestProfileCertQualificationViolation::UnsafeFallbackNotBlocked { .. }
    )));
}

#[test]
fn stable_profile_with_open_case_overclaims() {
    let mut packet = packet();
    let profile_id = packet.profiles[0].profile_id.clone();
    packet.profiles[0].claim_label = RequestProfileCertQualificationLabel::Stable;
    packet.profiles[0].displayed_label = RequestProfileCertQualificationLabel::Stable;
    // Force one of its cases open.
    let case = packet
        .cases
        .iter_mut()
        .find(|case| case.profile_ref == profile_id)
        .expect("profile must have at least one case");
    case.outcome = CertificationOutcome::Blocked;
    case.blocks_unsafe_fallback = true;
    case.downgrade_if_missing = true;
    packet.summary = packet.computed_summary();
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        RequestProfileCertQualificationViolation::StableProfileOverclaims { .. }
    )));
}
