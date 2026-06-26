//! Freeze gate for the M5 debug qualification set.
//!
//! The checked-in fixture `fixtures/debug/m5_debug_qualification/canonical_set.json` is the
//! published set. This gate rebuilds the set in code and asserts it equals the fixture after
//! a serialize round-trip, so the debugger qualification contract cannot drift from the
//! published artifact without failing CI. It also re-proves support-export safety, that every
//! debugger object family is claimed, every surface category and qualification status is
//! materialized, every publication channel is published, stable is only published when earned,
//! every degraded row narrows below stable, every active downgrade rule covers the rows it
//! triggers, every cited evidence packet and producing module exists on disk, and every frozen
//! invariant holds. This test runs under `cargo test --workspace`, so stable promotion cannot
//! harden a debugger claim without current proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_debug_contracts::DebugObjectClass;
use aureline_debug::m5_debug_qualification::{
    m5_debug_qualification_set, ClaimPublicationChannel, DebugQualificationSet,
    DebugQualificationStatus, DebugRowCategory, M5_DEBUG_QUALIFICATION_RECORD_KIND,
    M5_DEBUG_QUALIFICATION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_debug_qualification/canonical_set.json")
}

fn load_fixture() -> DebugQualificationSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_debug_qualification_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code debug qualification set drifted from the checked-in fixture; regenerate \
         it with `cargo run -p aureline-debug --example dump_m5_debug_qualification > \
         fixtures/debug/m5_debug_qualification/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_DEBUG_QUALIFICATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_DEBUG_QUALIFICATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: DebugQualificationSet =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn set_materializes_every_family() {
    let fixture = load_fixture();
    for class in DebugObjectClass::ALL {
        assert!(
            fixture.covers_object_class(class),
            "missing object class {}",
            class.as_str()
        );
    }
    for category in DebugRowCategory::ALL {
        assert!(
            fixture.row_in_category(category).is_some(),
            "missing category {}",
            category.as_str()
        );
    }
    for status in DebugQualificationStatus::ALL {
        assert!(
            fixture.row_with_status(status).is_some(),
            "missing status {}",
            status.as_str()
        );
    }
    for channel in ClaimPublicationChannel::ALL {
        assert!(
            fixture.publication_for_channel(channel).is_some(),
            "missing channel {}",
            channel.as_str()
        );
    }
}

#[test]
fn stable_is_earned_and_degraded_rows_narrow() {
    let fixture = load_fixture();
    for r in &fixture.qualification_rows {
        if r.published_maturity.is_stable() {
            assert_eq!(r.status, DebugQualificationStatus::Certified);
        }
        if r.status.triggers_narrowing() {
            assert!(
                !r.published_maturity.is_stable(),
                "degraded row {} still publishes stable",
                r.row_id
            );
            assert!(r.narrowed);
            assert!(!r.narrowing_reason.is_empty());
        }
    }
}

#[test]
fn active_rules_cover_every_triggered_row() {
    let fixture = load_fixture();
    for rule in fixture.downgrade_rules.iter().filter(|r| r.active) {
        for row in &fixture.qualification_rows {
            if row.degradations().contains(&rule.trigger) {
                assert!(
                    rule.affected_row_refs.contains(&row.row_id),
                    "rule {} omits triggered row {}",
                    rule.rule_id,
                    row.row_id
                );
                assert!(row.published_maturity.rank() >= rule.resulting_maturity.rank());
            }
        }
    }
}

#[test]
fn every_evidence_packet_and_producer_exists_on_disk() {
    let root = repo_root();
    let fixture = load_fixture();
    for schema in &fixture.source_schema_refs {
        assert!(
            root.join(schema).exists(),
            "source schema {schema} does not exist"
        );
    }
    for producer in &fixture.producer_refs {
        assert!(
            root.join(producer).exists(),
            "producer {producer} does not exist"
        );
    }
    for r in &fixture.qualification_rows {
        for evidence in &r.evidence_refs {
            assert!(
                root.join(evidence).exists(),
                "row {} evidence {} does not exist",
                r.row_id,
                evidence
            );
        }
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_debug_qualification.md",
        "schemas/debug/m5_debug_qualification.schema.json",
        "artifacts/debug/m5_debug_qualification.md",
        "fixtures/debug/m5_debug_qualification/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
