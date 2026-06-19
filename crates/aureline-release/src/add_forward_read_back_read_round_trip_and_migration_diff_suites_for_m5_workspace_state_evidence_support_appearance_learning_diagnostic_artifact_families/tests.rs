//! Unit tests for the typed M5 reader/writer compatibility suite.

use super::*;

fn suite() -> M5ReaderWriterCompatSuite {
    current_m5_reader_writer_compat_suite().expect("checked-in suite parses into the model")
}

#[test]
fn checked_in_suite_parses_and_validates() {
    let s = suite();
    assert_eq!(
        s.schema_version,
        M5_READER_WRITER_COMPAT_SUITE_SCHEMA_VERSION
    );
    assert_eq!(s.record_kind, M5_READER_WRITER_COMPAT_SUITE_RECORD_KIND);
    assert_eq!(s.suite_id, M5_READER_WRITER_COMPAT_SUITE_ID);
    let violations = s.validate();
    assert!(
        violations.is_empty(),
        "checked-in suite must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_matches_recomputed_counts() {
    let s = suite();
    assert_eq!(s.summary, s.computed_summary());
    assert_eq!(s.summary.total_suites, s.suites.len());
    assert_eq!(
        s.summary.write_back_suites + s.summary.compare_only_suites,
        s.suites.len()
    );
    assert_eq!(s.summary.fixtures_total, 3 * s.suites.len());
}

#[test]
fn every_family_covers_the_required_compatibility_behaviors() {
    let s = suite();
    assert!(!s.suites.is_empty());
    for fam in &s.suites {
        for required in [
            CaseKind::ForwardRead,
            CaseKind::BackRead,
            CaseKind::AdditiveField,
            CaseKind::UnknownFieldPreservation,
            CaseKind::MigrationDiff,
            CaseKind::Downgrade,
        ] {
            assert!(
                fam.case(required).is_some(),
                "{} must cover {required:?}",
                fam.family_id
            );
        }
        // Forward-read preserves unknown fields; the downgrade case narrows.
        assert!(
            fam.case(CaseKind::ForwardRead)
                .unwrap()
                .preserves_unknown_fields
        );
        assert_eq!(
            fam.case(CaseKind::Downgrade).unwrap().expected_outcome,
            ExpectedOutcome::Narrowed
        );
        // The migration diff is additive and adds the declared field.
        assert_eq!(fam.migration_diff.change_class, ChangeClass::Additive);
        assert!(fam.migration_diff.added_fields.contains(&fam.added_field));
        assert!(fam.migration_diff.removed_fields.is_empty());
        assert!(fam.migration_diff.changed_fields.is_empty());
    }
}

#[test]
fn write_back_posture_follows_reader_writer_posture() {
    let s = suite();
    for fam in &s.suites {
        assert_eq!(
            fam.write_back_posture,
            fam.reader_writer_posture.write_back_posture(),
            "{}: write-back posture must follow reader/writer posture",
            fam.family_id
        );
        if fam.writes_back() {
            // Write-back families carry a round-trip case and no compare-only case.
            assert!(fam.case(CaseKind::RoundTrip).is_some(), "{}", fam.family_id);
            assert!(
                fam.case(CaseKind::CompareOnly).is_none(),
                "{}",
                fam.family_id
            );
            assert!(fam.case(CaseKind::RoundTrip).unwrap().backup_first);
        } else {
            // Compare-only families carry a compare-only case that never writes back.
            let compare = fam.case(CaseKind::CompareOnly).expect("compare-only case");
            assert!(!compare.writes_back, "{}", fam.family_id);
            assert_eq!(
                compare.expected_outcome,
                ExpectedOutcome::CompatibleCompareOnly
            );
            assert!(fam.case(CaseKind::RoundTrip).is_none(), "{}", fam.family_id);
            // No case in a compare-only family writes back.
            assert!(
                fam.cases.iter().all(|c| !c.writes_back),
                "{}",
                fam.family_id
            );
        }
    }
}

#[test]
fn version_triple_is_strictly_increasing() {
    let s = suite();
    for fam in &s.suites {
        assert!(
            fam.prior_version < fam.current_version
                && fam.current_version < fam.unsupported_version,
            "{}: version triple must increase",
            fam.family_id
        );
    }
}

#[test]
fn duplicate_family_is_rejected() {
    let mut s = suite();
    let dup = s.suites[0].clone();
    s.suites.push(dup);
    s.summary = s.computed_summary();
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "suites.duplicate_family"),
        "a duplicate family must be rejected: {violations:#?}"
    );
}

#[test]
fn compare_only_family_writing_back_is_rejected() {
    let mut s = suite();
    let idx = s
        .suites
        .iter()
        .position(|f| !f.writes_back())
        .expect("at least one compare-only family");
    let case = s.suites[idx]
        .cases
        .iter_mut()
        .find(|c| c.case_kind == CaseKind::CompareOnly)
        .unwrap();
    case.writes_back = true;
    let violations = s.suites[idx].family_id.clone();
    let report = s.validate();
    assert!(
        report
            .iter()
            .any(|v| v.check_id == "cases.compare_only_writes_back"),
        "compare-only family {violations} writing back must be rejected: {report:#?}"
    );
}
