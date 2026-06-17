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
    repo_root().join(M5_SEARCH_NAVIGATION_QUALIFICATION_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5SearchNavigationQualificationPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_has_expected_envelope_and_coverage() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    assert_eq!(
        packet.record_kind,
        M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.packet_id,
        M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_ID
    );
    assert_eq!(packet.doc_ref, M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF);
    assert_eq!(
        packet.schema_ref,
        M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF
    );
    assert_eq!(
        packet.artifact_ref,
        M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF
    );

    for surface in SearchSurfaceClass::ALL {
        assert!(
            packet.claimed_surfaces.contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
    for mode in DeploymentMode::ALL {
        assert!(
            packet.deployment_modes.contains(&mode),
            "missing deployment mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_surface_once() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    for surface in SearchSurfaceClass::ALL {
        let rows = packet
            .qualification_rows
            .iter()
            .filter(|row| row.surface == surface)
            .count();
        assert_eq!(rows, 1, "expected one row for {}", surface.as_str());
    }
    assert_eq!(
        packet.qualification_rows.len(),
        SearchSurfaceClass::ALL.len()
    );
}

#[test]
fn canonical_packet_validates_and_is_export_safe() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());
    assert!(packet
        .qualification_rows
        .iter()
        .all(|row| row.published_state == QualificationStateClass::Qualified));
}

#[test]
fn every_surface_references_the_one_shared_model() {
    // Acceptance: all claimed M5 search surfaces reference one shared
    // query-session and result-identity model.
    let packet = seeded_m5_search_navigation_qualification_packet();
    for row in &packet.qualification_rows {
        assert_eq!(
            row.shared_query_session_ref,
            "schemas/search/query_session.schema.json"
        );
        assert_eq!(
            row.shared_result_identity_ref,
            crate::result_truth_packet::SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF
        );
        // Every surface binds the shared session + result-identity objects.
        for object in [
            SearchContractObjectClass::QuerySession,
            SearchContractObjectClass::ResultRef,
        ] {
            assert!(
                row.bound_object_ids.contains(&object.object_id()),
                "{} does not bind {}",
                row.qualification_row_id,
                object.as_str()
            );
        }
    }
    let mut session_refs: Vec<&str> = packet
        .qualification_rows
        .iter()
        .map(|row| row.shared_query_session_ref.as_str())
        .collect();
    session_refs.sort_unstable();
    session_refs.dedup();
    assert_eq!(
        session_refs.len(),
        1,
        "surfaces must share one session model"
    );
}

#[test]
fn every_contract_object_cites_its_own_proof() {
    // Guardrail: an object may not borrow an adjacent lane's proof.
    let packet = seeded_m5_search_navigation_qualification_packet();
    for object_class in SearchContractObjectClass::ALL {
        let row = packet
            .contract_objects
            .iter()
            .find(|row| row.object_class == object_class)
            .unwrap_or_else(|| panic!("missing object {}", object_class.as_str()));
        let refs = object_class.backing_refs();
        assert_eq!(row.backing_schema_ref, refs.schema_ref);
        assert_eq!(row.backing_fixture_ref, refs.fixture_ref);
        assert_eq!(row.backing_record_kind, refs.record_kind);
        assert_eq!(row.privacy_data_class, object_class.privacy_data_class());
    }
}

#[test]
fn result_state_vocabulary_is_complete_and_every_state_is_expressed() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    assert_eq!(
        packet.result_state_vocabulary.len(),
        ResultStateClass::ALL.len()
    );
    for state in ResultStateClass::ALL {
        let row = packet
            .result_state_vocabulary
            .iter()
            .find(|row| row.state_class == state)
            .unwrap_or_else(|| panic!("missing state {}", state.as_str()));
        assert_eq!(row.token, state.as_str());
        assert_eq!(row.narrows_scope, state.narrows_scope());
        // Each state must be expressible by at least one claimed surface.
        assert!(
            SearchSurfaceClass::ALL
                .into_iter()
                .any(|surface| surface.expressible_states().contains(&state)),
            "no surface expresses {}",
            state.as_str()
        );
    }
    // The narrowing states are exactly the six non-fresh states.
    let narrowing = packet
        .result_state_vocabulary
        .iter()
        .filter(|row| row.narrows_scope)
        .count();
    assert_eq!(narrowing, 6);
}

#[test]
fn privacy_bindings_cover_every_data_class_and_keep_text_local() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    for data_class in PrivacyDataClass::ALL {
        let binding = packet
            .privacy_bindings
            .iter()
            .find(|binding| binding.data_class == data_class)
            .unwrap_or_else(|| panic!("missing privacy binding {}", data_class.as_str()));
        assert!(binding.local_only_by_default);
    }
    let raw = packet
        .privacy_bindings
        .iter()
        .find(|binding| binding.data_class == PrivacyDataClass::RawQueryText)
        .expect("raw query text binding");
    assert_eq!(raw.privacy_class, PrivacyClass::LocalSensitive);
    assert_eq!(
        raw.consent_requirement,
        ConsentRequirement::ExplicitForShare
    );
}

#[test]
fn query_material_surfaces_keep_local_query_text_first() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    for row in &packet.qualification_rows {
        if row.persists_query_material {
            assert!(
                row.local_query_text_first,
                "{} demotes local-only query text below a sync/export path",
                row.qualification_row_id
            );
        }
        assert_eq!(
            row.persists_query_material,
            row.surface.persists_query_material()
        );
    }
}

#[test]
fn consumer_bindings_all_ingest_the_same_index() {
    let packet = seeded_m5_search_navigation_qualification_packet();
    for consumer in QualificationConsumerClass::ALL {
        let binding = packet
            .consumer_bindings
            .iter()
            .find(|binding| binding.consumer == consumer)
            .unwrap_or_else(|| panic!("missing consumer binding {}", consumer.as_str()));
        assert_eq!(binding.ingested_packet_id, packet.packet_id);
        assert_eq!(
            binding.qualification_row_count,
            packet.qualification_rows.len()
        );
        assert!(binding.narrow_on_stale_proof);
        for field in [
            "qualification_row_id",
            "surface",
            "published_state",
            "deployment_mode_coverage",
            "shared_query_session_ref",
            "shared_result_identity_ref",
            "expressible_states",
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
fn partial_index_stale_fixture_narrows_live_surfaces_only() {
    let packet = seeded_partial_index_stale_m5_search_navigation_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for row in &packet.qualification_rows {
        if row.surface.depends_on_live_index() {
            assert_eq!(
                row.published_state,
                QualificationStateClass::ScopeLimited,
                "{} must narrow when the index is partial/stale",
                row.qualification_row_id
            );
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "partial_index_stale_epoch"));
        } else {
            assert_eq!(
                row.published_state,
                QualificationStateClass::Qualified,
                "{} replays captured freshness and stays qualified",
                row.qualification_row_id
            );
        }
    }
}

#[test]
fn unconsented_query_text_fixture_narrows_query_material_surfaces() {
    let packet = seeded_unconsented_query_text_m5_search_navigation_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for row in &packet.qualification_rows {
        if row.surface.persists_query_material() {
            assert_eq!(
                row.published_state,
                QualificationStateClass::LocalQueryTextOnly,
                "{} must narrow when query-material consent is missing",
                row.qualification_row_id
            );
            // Local-only query text stays first-class even while sync/export narrow.
            assert!(row.local_query_text_first);
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "query_material_consent_missing"));
        } else {
            assert_eq!(row.published_state, QualificationStateClass::Qualified);
        }
    }
}

#[test]
fn degraded_fixtures_are_not_green_and_cite_real_rules() {
    for packet in [
        seeded_partial_index_stale_m5_search_navigation_qualification_packet(),
        seeded_unconsented_query_text_m5_search_navigation_qualification_packet(),
    ] {
        assert!(packet
            .qualification_rows
            .iter()
            .any(|row| row.published_state != QualificationStateClass::Qualified));
        let rule_ids: Vec<&str> = packet
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        for row in &packet.qualification_rows {
            if row.published_state != QualificationStateClass::Qualified {
                assert!(!row.downgrade_rule_ids.is_empty());
                for id in &row.downgrade_rule_ids {
                    assert!(rule_ids.contains(&id.as_str()), "unknown rule {id}");
                }
            }
        }
    }
}

#[test]
fn state_counts_track_published_rows() {
    let canonical = seeded_m5_search_navigation_qualification_packet();
    let counts = canonical.state_counts();
    assert_eq!(counts.qualified, canonical.qualification_rows.len());
    assert_eq!(counts.scope_limited, 0);

    let partial = seeded_partial_index_stale_m5_search_navigation_qualification_packet();
    let partial_counts = partial.state_counts();
    let live_surfaces = SearchSurfaceClass::ALL
        .into_iter()
        .filter(|surface| surface.depends_on_live_index())
        .count();
    assert_eq!(partial_counts.scope_limited, live_surfaces);

    let unconsented = seeded_unconsented_query_text_m5_search_navigation_qualification_packet();
    let unconsented_counts = unconsented.state_counts();
    let material_surfaces = SearchSurfaceClass::ALL
        .into_iter()
        .filter(|surface| surface.persists_query_material())
        .count();
    assert_eq!(unconsented_counts.local_query_text_only, material_surfaces);
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF,
        M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF,
        M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF,
        "fixtures/search/m5/m5-search-navigation-qualification/manifest.yaml",
        "fixtures/search/m5/m5-search-navigation-qualification/README.md",
        "fixtures/search/m5/m5-search-navigation-qualification/packet.json",
        "fixtures/search/m5/m5-search-navigation-qualification/partial_index_stale_scope_limited.json",
        "fixtures/search/m5/m5-search-navigation-qualification/unconsented_query_text_local_only.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    assert_eq!(
        load_fixture("packet.json"),
        seeded_m5_search_navigation_qualification_packet()
    );
}

#[test]
fn degraded_fixtures_match_seeded_variants() {
    assert_eq!(
        load_fixture("partial_index_stale_scope_limited.json"),
        seeded_partial_index_stale_m5_search_navigation_qualification_packet()
    );
    assert_eq!(
        load_fixture("unconsented_query_text_local_only.json"),
        seeded_unconsented_query_text_m5_search_navigation_qualification_packet()
    );
}
