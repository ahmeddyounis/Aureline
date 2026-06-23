//! Freeze gate for the M5 activity-objects bundle.
//!
//! The checked-in fixture
//! `fixtures/activity/m5-activity-objects/canonical_bundle.json` is the published
//! bundle. This gate rebuilds the bundle in code and asserts it equals the fixture
//! after a serialize round-trip, so the durable activity object model and its
//! rendered rows cannot drift from the published artifact without failing CI. It
//! also re-proves support-export safety, that every M5 job family has a durable
//! object, that every row reproduces from its object, that archive / expiry is one
//! shared truth across surfaces, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_activity_objects::{
    activity_objects_bundle, archive_state_for, render_row, ActivityObjectsBundle,
    ArchiveStateClass, JobFamilyClass, M5_ACTIVITY_OBJECTS_RECORD_KIND,
    M5_ACTIVITY_OBJECTS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-activity-objects/canonical_bundle.json")
}

fn load_fixture() -> ActivityObjectsBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = activity_objects_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code activity-objects bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-activity --example dump_m5_activity_objects`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ACTIVITY_OBJECTS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ACTIVITY_OBJECTS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: ActivityObjectsBundle =
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
fn every_job_family_has_a_durable_object() {
    let fixture = load_fixture();
    for family in JobFamilyClass::ALL {
        let entry = fixture
            .family(family)
            .unwrap_or_else(|| panic!("family {} has an entry", family.as_str()));
        assert!(!entry.spinner_or_toast_only);
        assert!(entry.long_running);
        assert!(
            fixture.objects.iter().any(|o| o.job_family == family),
            "family {} has a durable object",
            family.as_str()
        );
    }
}

#[test]
fn every_row_is_reproducible_from_its_object() {
    let fixture = load_fixture();
    assert_eq!(fixture.rows.len(), fixture.objects.len());
    for object in &fixture.objects {
        let row = fixture
            .row(&object.activity_job_id)
            .expect("object has a row");
        assert_eq!(
            &render_row(object),
            row,
            "row for object {} must reproduce",
            object.activity_job_id
        );
    }
}

#[test]
fn archive_state_is_deterministic_and_shared_across_surfaces() {
    let fixture = load_fixture();
    for object in &fixture.objects {
        assert_eq!(
            object.archive_state,
            archive_state_for(
                object.progress.progress_state,
                &object.retention,
                object.age_days
            ),
            "archive state for {} must recompute",
            object.activity_job_id
        );
        let row = fixture.row(&object.activity_job_id).expect("row");
        for projection in &row.surface_projections {
            assert_eq!(
                projection.archive_state, object.archive_state,
                "row {} surface {} archive state diverged",
                object.activity_job_id, projection.consumer_token
            );
        }
    }
}

#[test]
fn corpus_exercises_active_archived_and_expired() {
    let fixture = load_fixture();
    for state in [
        ArchiveStateClass::Active,
        ArchiveStateClass::Archived,
        ArchiveStateClass::Expired,
    ] {
        assert!(
            fixture.objects.iter().any(|o| o.archive_state == state),
            "corpus must exercise {}",
            state.as_str()
        );
    }
}
