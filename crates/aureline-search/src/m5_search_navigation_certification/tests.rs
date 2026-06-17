use std::fs;
use std::path::{Path, PathBuf};

use super::*;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_SEARCH_NAVIGATION_CERTIFICATION_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5SearchNavigationCertificationPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_has_expected_envelope_and_coverage() {
    let packet = seeded_m5_search_navigation_certification_packet();
    assert_eq!(
        packet.record_kind,
        M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.packet_id,
        M5_SEARCH_NAVIGATION_CERTIFICATION_PACKET_ID
    );
    assert_eq!(packet.doc_ref, M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF);
    assert_eq!(
        packet.schema_ref,
        M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_REF
    );
    assert_eq!(
        packet.artifact_ref,
        M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF
    );

    for lane in CertificationLaneClass::ALL {
        assert!(
            packet.certified_lanes.contains(&lane),
            "missing lane {}",
            lane.as_str()
        );
    }
    for surface in CertificationSurfaceClass::ALL {
        assert!(
            packet.parity_surfaces.contains(&surface),
            "missing parity surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_lane_once() {
    let packet = seeded_m5_search_navigation_certification_packet();
    for lane in CertificationLaneClass::ALL {
        let rows = packet
            .lane_rows
            .iter()
            .filter(|row| row.lane == lane)
            .count();
        assert_eq!(rows, 1, "expected one row for {}", lane.as_str());
        let audits = packet
            .parity_audits
            .iter()
            .filter(|audit| audit.lane == lane)
            .count();
        assert_eq!(audits, 1, "expected one parity audit for {}", lane.as_str());
    }
    assert_eq!(packet.lane_rows.len(), CertificationLaneClass::ALL.len());
}

#[test]
fn canonical_packet_validates_and_is_fully_certified() {
    let packet = seeded_m5_search_navigation_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());
    assert!(packet.is_fully_certified());
    assert!(packet
        .lane_rows
        .iter()
        .all(|row| row.certification_state == CertificationStateClass::Certified));
    assert!(packet
        .lane_rows
        .iter()
        .all(|row| row.evidence_freshness == EvidenceFreshnessClass::Fresh));
}

#[test]
fn every_lane_cites_its_own_evidence_proof() {
    // Own-proof guard: a lane may not borrow an adjacent lane's evidence.
    let packet = seeded_m5_search_navigation_certification_packet();
    for lane in CertificationLaneClass::ALL {
        let row = packet
            .lane_rows
            .iter()
            .find(|row| row.lane == lane)
            .unwrap_or_else(|| panic!("missing lane {}", lane.as_str()));
        let refs = lane.evidence_refs();
        assert_eq!(row.evidence_packet_id, refs.packet_id);
        assert_eq!(row.evidence_schema_ref, refs.schema_ref);
        assert_eq!(row.evidence_doc_ref, refs.doc_ref);
        assert_eq!(row.evidence_artifact_ref, refs.artifact_ref);
        assert_eq!(row.evidence_fixture_dir, refs.fixture_dir);
        assert_eq!(row.evidence_record_kind, refs.record_kind);
    }
    // Every lane cites a distinct evidence packet id.
    let mut packet_ids: Vec<&str> = packet
        .lane_rows
        .iter()
        .map(|row| row.evidence_packet_id.as_str())
        .collect();
    packet_ids.sort_unstable();
    packet_ids.dedup();
    assert_eq!(packet_ids.len(), CertificationLaneClass::ALL.len());
}

#[test]
fn freshness_policy_is_complete() {
    let packet = seeded_m5_search_navigation_certification_packet();
    assert!(packet.freshness_policy.recheck_window_days > 0);
    for class in EvidenceFreshnessClass::ALL {
        let row = packet
            .freshness_policy
            .freshness_states
            .iter()
            .find(|row| row.freshness_class == class)
            .unwrap_or_else(|| panic!("missing freshness state {}", class.as_str()));
        assert_eq!(row.token, class.as_str());
        assert_eq!(row.requires_retest, class.requires_retest());
    }
    // Only `fresh` does not require a re-test.
    let no_retest = packet
        .freshness_policy
        .freshness_states
        .iter()
        .filter(|row| !row.requires_retest)
        .count();
    assert_eq!(no_retest, 1);
}

#[test]
fn retest_pending_fixture_fails_stale_lane_closed() {
    // Acceptance: stale evidence narrows the lane automatically.
    let packet = seeded_retest_pending_m5_search_navigation_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for row in &packet.lane_rows {
        if row.lane == CertificationLaneClass::NavigationContinuity {
            assert_eq!(
                row.certification_state,
                CertificationStateClass::RetestPending
            );
            assert_eq!(row.evidence_freshness, EvidenceFreshnessClass::Stale);
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "evidence_stale_past_recheck_window"));
            assert!(row
                .downgrade_rule_ids
                .iter()
                .any(|id| id == "evidence_stale_requires_retest"));
        } else {
            assert_eq!(row.certification_state, CertificationStateClass::Certified);
        }
    }
    let counts = packet.state_counts();
    assert_eq!(counts.retest_pending, 1);
    assert_eq!(counts.certified, CertificationLaneClass::ALL.len() - 1);
}

#[test]
fn limited_fixture_fails_degraded_lane_closed_and_catches_overclaim() {
    // Acceptance: a degraded source / broken parity narrows to limited, and the
    // overclaiming surface is caught.
    let packet = seeded_limited_m5_search_navigation_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let lane = CertificationLaneClass::RankingExplainability;
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == lane)
        .expect("ranking lane row");
    assert_eq!(row.certification_state, CertificationStateClass::Limited);
    assert!(row.source_state_is_degraded);
    assert!(row
        .stale_proof_tokens
        .iter()
        .any(|token| token == "source_lane_degraded"));
    assert!(row
        .stale_proof_tokens
        .iter()
        .any(|token| token == "surface_parity_break"));

    let audit = packet
        .parity_audits
        .iter()
        .find(|audit| audit.lane == lane)
        .expect("ranking parity audit");
    assert!(!audit.all_in_parity);
    let overclaiming = audit
        .surface_parity
        .iter()
        .filter(|parity| !parity.in_parity)
        .count();
    assert_eq!(overclaiming, 1, "exactly one surface overclaims");
    // The overclaiming surface still projects the greener certified state.
    let broken = audit
        .surface_parity
        .iter()
        .find(|parity| !parity.in_parity)
        .expect("a broken parity row");
    assert_eq!(broken.projected_state_token, "certified");
}

#[test]
fn unsupported_fixture_fails_missing_lane_closed() {
    // Acceptance: missing evidence narrows to unsupported.
    let packet = seeded_unsupported_m5_search_navigation_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == CertificationLaneClass::SavedQueryPrivacy)
        .expect("saved-query lane row");
    assert_eq!(
        row.certification_state,
        CertificationStateClass::Unsupported
    );
    assert_eq!(row.evidence_freshness, EvidenceFreshnessClass::Missing);
    assert!(row.evidence_generated_at.is_empty());
    assert!(row
        .downgrade_rule_ids
        .iter()
        .any(|id| id == "evidence_missing_blocks_claim"));
    assert!(!packet.is_fully_certified());
}

#[test]
fn degraded_fixtures_are_not_green_and_cite_real_rules() {
    for packet in [
        seeded_retest_pending_m5_search_navigation_certification_packet(),
        seeded_limited_m5_search_navigation_certification_packet(),
        seeded_unsupported_m5_search_navigation_certification_packet(),
    ] {
        assert!(packet
            .lane_rows
            .iter()
            .any(|row| row.certification_state != CertificationStateClass::Certified));
        let rule_ids: Vec<&str> = packet
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        for row in &packet.lane_rows {
            if row.certification_state != CertificationStateClass::Certified {
                assert!(!row.downgrade_rule_ids.is_empty());
                assert!(!row.stale_proof_tokens.is_empty());
                for id in &row.downgrade_rule_ids {
                    assert!(rule_ids.contains(&id.as_str()), "unknown rule {id}");
                }
            }
        }
    }
}

#[test]
fn every_downgrade_trigger_is_published_with_fail_closed_mapping() {
    let packet = seeded_m5_search_navigation_certification_packet();
    for trigger in CertificationDowngradeTriggerClass::ALL {
        let rule = packet
            .downgrade_rules
            .iter()
            .find(|rule| rule.trigger_class == trigger)
            .unwrap_or_else(|| panic!("missing trigger {}", trigger.as_str()));
        assert_eq!(rule.source_state, CertificationStateClass::Certified);
        assert!(!rule.evidence_refs.is_empty());
    }
    // The fail-closed mapping covers every non-certified terminal state.
    let downgraded: Vec<CertificationStateClass> = packet
        .downgrade_rules
        .iter()
        .map(|rule| rule.downgraded_state)
        .collect();
    assert!(downgraded.contains(&CertificationStateClass::RetestPending));
    assert!(downgraded.contains(&CertificationStateClass::Limited));
    assert!(downgraded.contains(&CertificationStateClass::Unsupported));
}

#[test]
fn consumer_bindings_all_ingest_the_same_index() {
    let packet = seeded_m5_search_navigation_certification_packet();
    for consumer in CertificationConsumerClass::ALL {
        let binding = packet
            .consumer_bindings
            .iter()
            .find(|binding| binding.consumer == consumer)
            .unwrap_or_else(|| panic!("missing consumer binding {}", consumer.as_str()));
        assert_eq!(binding.ingested_packet_id, packet.packet_id);
        assert_eq!(binding.lane_row_count, packet.lane_rows.len());
        assert!(binding.narrow_on_stale_evidence);
        for field in [
            "certification_row_id",
            "lane",
            "certification_state",
            "evidence_freshness",
            "recheck_by",
            "stale_proof_tokens",
            "downgrade_rule_ids",
        ] {
            assert!(
                binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field),
                "{} binding must preserve {field}",
                consumer.as_str()
            );
        }
    }
}

#[test]
fn support_export_wraps_packet_and_excludes_private_material() {
    let packet = seeded_m5_search_navigation_certification_packet();
    let export = packet.support_export("export-1", "2026-06-17T00:00:00Z");
    assert_eq!(
        export.record_kind,
        M5_SEARCH_NAVIGATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.certification_packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
    assert!(export.ambient_authority_excluded);
    assert_eq!(export.certification_packet, packet);
}

#[test]
fn tampered_certified_row_with_stale_evidence_is_rejected() {
    // Guardrail: a green claim on stale evidence must not validate.
    let mut packet = seeded_m5_search_navigation_certification_packet();
    packet.lane_rows[0].evidence_freshness = EvidenceFreshnessClass::Stale;
    let violations = packet.validate();
    assert!(
        !violations.is_empty(),
        "a certified row on stale evidence must fail validation"
    );
}

#[test]
fn tampered_borrowed_evidence_is_rejected() {
    // Guardrail: a lane that borrows another lane's evidence packet must fail.
    let mut packet = seeded_m5_search_navigation_certification_packet();
    packet.lane_rows[0].evidence_packet_id = "search.m5.ranking_explainability.v1".to_owned();
    assert!(!packet.validate().is_empty());
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        M5_SEARCH_NAVIGATION_CERTIFICATION_SCHEMA_REF,
        M5_SEARCH_NAVIGATION_CERTIFICATION_DOC_REF,
        M5_SEARCH_NAVIGATION_CERTIFICATION_ARTIFACT_REF,
        "fixtures/search/m5/m5-search-navigation-certification/manifest.yaml",
        "fixtures/search/m5/m5-search-navigation-certification/README.md",
        "fixtures/search/m5/m5-search-navigation-certification/packet.json",
        "fixtures/search/m5/m5-search-navigation-certification/retest_pending_stale_evidence.json",
        "fixtures/search/m5/m5-search-navigation-certification/limited_degraded_source.json",
        "fixtures/search/m5/m5-search-navigation-certification/unsupported_missing_evidence.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    assert_eq!(
        load_fixture("packet.json"),
        seeded_m5_search_navigation_certification_packet()
    );
}

#[test]
fn degraded_fixtures_match_seeded_variants() {
    assert_eq!(
        load_fixture("retest_pending_stale_evidence.json"),
        seeded_retest_pending_m5_search_navigation_certification_packet()
    );
    assert_eq!(
        load_fixture("limited_degraded_source.json"),
        seeded_limited_m5_search_navigation_certification_packet()
    );
    assert_eq!(
        load_fixture("unsupported_missing_evidence.json"),
        seeded_unsupported_m5_search_navigation_certification_packet()
    );
}
