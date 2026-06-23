//! Freeze gate for the M5 retention/deletion bundle.
//!
//! The checked-in fixture
//! `fixtures/admin/m5-retention-deletion/canonical_retention.json` is the
//! published retention/deletion bundle. This gate rebuilds the bundle in code and
//! asserts it equals the fixture after a serialize round-trip, so the rendered
//! matrices cannot drift from the published artifact without failing CI. It also
//! re-proves support-export safety, full profile coverage, that every rendered
//! state is one the frozen matrix admits, that the data classes and delete
//! outcomes stay distinguished, that receipted deletes carry a receipt and
//! hold-blocked deletes name their hold, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass, AdminSurfaceClass};
use aureline_policy::m5_retention_deletion::{
    retention_deletion_bundle, retention_deletion_lines, ArtifactOwnerClass, DeleteOutcomeClass,
    DeletionLinkageClass, ExportFormatClass, PropagationTargetClass, RetentionDeletionBundle,
    M5_RETENTION_DELETION_RECORD_KIND, M5_RETENTION_DELETION_SCHEMA_REF, RETENTION_PROFILES,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-retention-deletion/canonical_retention.json")
}

fn load_fixture() -> RetentionDeletionBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = retention_deletion_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code retention/deletion bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-policy --example dump_m5_retention_deletion`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_RETENTION_DELETION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_RETENTION_DELETION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RetentionDeletionBundle =
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
fn bundle_renders_every_managed_profile() {
    let fixture = load_fixture();
    assert_eq!(fixture.profiles.len(), RETENTION_PROFILES.len());
    for profile in RETENTION_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.matrix.rows.is_empty());
        assert!(packet.matrix.coverage.locally_inspectable);
        assert!(packet.matrix.coverage.vendor_console_independent);
    }
}

#[test]
fn rendered_states_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::RetentionDeletionMatrix)
            .expect("surface present in matrix")
            .applicable_states
            .contains(&state)
    };
    for packet in &fixture.profiles {
        for row in &packet.matrix.rows {
            assert!(
                admitted(row.machine_state),
                "retention state {} not admitted by the matrix",
                row.machine_state.as_str()
            );
        }
        assert!(admitted(packet.matrix.coverage.coverage_state));
    }
}

#[test]
fn data_classes_and_outcomes_stay_distinguished() {
    let fixture = load_fixture();
    for class in ArtifactOwnerClass::ALL {
        assert!(
            fixture
                .profiles
                .iter()
                .any(|p| p.matrix.rows.iter().any(|r| r.data_class == class)),
            "data class {} never appears",
            class.as_str()
        );
    }
    for outcome in DeleteOutcomeClass::ALL {
        assert!(
            fixture.profiles.iter().any(|p| p
                .matrix
                .rows
                .iter()
                .any(|r| r.delete_outcome == outcome)),
            "delete outcome {} never appears",
            outcome.as_str()
        );
    }
}

#[test]
fn every_linkage_class_appears_and_receipts_holds_are_honest() {
    let fixture = load_fixture();
    for class in DeletionLinkageClass::ALL {
        assert!(
            fixture
                .profiles
                .iter()
                .any(|p| p.matrix.rows.iter().any(|r| r.has_linkage(class))),
            "linkage class {} never appears",
            class.as_str()
        );
    }
    for packet in &fixture.profiles {
        for row in &packet.matrix.rows {
            if row.machine_state == AdminStateClass::DeleteReceipted {
                assert!(row.has_linkage(DeletionLinkageClass::DestructionReceipt));
            }
            if row.machine_state == AdminStateClass::DeleteBlockedByHold {
                assert!(row.has_linkage(DeletionLinkageClass::LegalHold));
            }
        }
    }
}

#[test]
fn non_immediate_deletes_explain_their_remainder() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for row in &packet.matrix.rows {
            if row.delete_outcome.requires_remainder() {
                let rem = row
                    .remainder
                    .as_ref()
                    .expect("non-immediate row carries a remainder");
                assert!(!rem.what_remains.is_empty());
                assert!(!rem.expected_completion.is_empty());
            }
        }
    }
}

#[test]
fn states_propagate_into_support_offboarding_compliance_and_help_about() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for target in PropagationTargetClass::ALL {
            assert!(
                packet.matrix.propagates_to(target),
                "{}: state does not propagate into {}",
                packet.profile.as_str(),
                target.as_str()
            );
        }
    }
}

#[test]
fn every_row_exports_both_machine_and_plain_language() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        assert!(packet.matrix.offers(ExportFormatClass::MachineReadableJson));
        assert!(packet
            .matrix
            .offers(ExportFormatClass::PlainLanguageHandoff));
        for row in &packet.matrix.rows {
            assert!(row.has_export_parity());
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = retention_deletion_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Retention/deletion bundle")));
    for profile in RETENTION_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
