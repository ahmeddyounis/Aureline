//! Freeze gate for the M5 offboarding bundle.
//!
//! The checked-in fixture
//! `fixtures/admin/m5-offboarding/canonical_offboarding.json` is the published
//! offboarding bundle. This gate rebuilds the bundle in code and asserts it equals
//! the fixture after a serialize round-trip, so the rendered wizards cannot drift
//! from the published artifact without failing CI. It also re-proves support-export
//! safety, full profile coverage, that every rendered state is one the frozen
//! matrix admits, that the checkpoints stay ordered and complete, that no step
//! requires a paid seat, that blocked and failed steps retain a typed recovery, and
//! every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass, AdminSurfaceClass};
use aureline_policy::m5_offboarding::{
    offboarding_bundle, offboarding_lines, CheckpointKindClass, CheckpointOutcomeClass,
    ContinuityRightClass, ManagedCopyDispositionClass, OffboardingBundle, OffboardingScopeClass,
    OffboardingTriggerClass, RecoveryAffordanceClass, M5_OFFBOARDING_RECORD_KIND,
    M5_OFFBOARDING_SCHEMA_REF, OFFBOARDING_PROFILES,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-offboarding/canonical_offboarding.json")
}

fn load_fixture() -> OffboardingBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = offboarding_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code offboarding bundle drifted from the checked-in fixture; regenerate it with \
         `cargo run -p aureline-policy --example dump_m5_offboarding`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_OFFBOARDING_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_OFFBOARDING_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: OffboardingBundle =
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
    assert_eq!(fixture.profiles.len(), OFFBOARDING_PROFILES.len());
    for profile in OFFBOARDING_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.wizard.checkpoints.is_empty());
        assert!(packet.wizard.coverage.locally_inspectable);
        assert!(packet.wizard.coverage.vendor_console_independent);
        assert!(packet.wizard.coverage.completable_without_paid_seat);
    }
}

#[test]
fn rendered_states_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::OffboardingWizard)
            .expect("surface present in matrix")
            .applicable_states
            .contains(&state)
    };
    for packet in &fixture.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            assert!(
                admitted(checkpoint.machine_state),
                "offboarding state {} not admitted by the matrix",
                checkpoint.machine_state.as_str()
            );
        }
        assert!(admitted(packet.wizard.coverage.coverage_state));
    }
}

#[test]
fn checkpoints_are_ordered_and_every_kind_appears() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for kind in CheckpointKindClass::ALL {
            assert_eq!(
                packet
                    .wizard
                    .checkpoints
                    .iter()
                    .filter(|c| c.kind == kind)
                    .count(),
                1,
                "{}: kind {} not present exactly once",
                packet.profile.as_str(),
                kind.as_str()
            );
        }
        for window in packet.wizard.checkpoints.windows(2) {
            assert!(window[0].order < window[1].order);
        }
    }
}

#[test]
fn no_step_requires_a_paid_seat_to_recover_user_data() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        assert!(packet.wizard.coverage.completable_without_paid_seat);
        for checkpoint in &packet.wizard.checkpoints {
            assert!(!checkpoint.requires_paid_seat);
        }
        for trigger in &packet.wizard.triggers {
            assert!(!trigger.requires_active_seat_for_recovery);
        }
    }
}

#[test]
fn triggers_explain_impact_and_every_class_appears() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for trigger in &packet.wizard.triggers {
            assert!(!trigger.impacted_features.is_empty());
            assert!(!trigger.export_rights.is_empty());
            assert!(!trigger.local_safe_continuation.is_empty());
            assert!(!trigger.managed_copies_summary.is_empty());
        }
    }
    for class in OffboardingTriggerClass::ALL {
        assert!(
            fixture
                .profiles
                .iter()
                .any(|p| p.wizard.triggers.iter().any(|t| t.trigger == class)),
            "trigger class {} never appears",
            class.as_str()
        );
    }
}

#[test]
fn scopes_outcomes_and_dispositions_stay_distinguished() {
    let fixture = load_fixture();
    for scope in OffboardingScopeClass::ALL {
        assert!(
            fixture
                .profiles
                .iter()
                .any(|p| p.wizard.checkpoints.iter().any(|c| c.scope == scope)),
            "scope {} never appears",
            scope.as_str()
        );
    }
    for outcome in CheckpointOutcomeClass::ALL {
        assert!(
            fixture.profiles.iter().any(|p| p
                .wizard
                .checkpoints
                .iter()
                .any(|c| c.outcome == outcome)),
            "outcome {} never appears",
            outcome.as_str()
        );
    }
    for disposition in ManagedCopyDispositionClass::ALL {
        assert!(
            fixture.profiles.iter().any(|p| p
                .wizard
                .checkpoints
                .iter()
                .any(|c| c.managed_copies.disposition == disposition)),
            "disposition {} never appears",
            disposition.as_str()
        );
    }
}

#[test]
fn blocked_and_failed_flows_retain_typed_recovery() {
    let fixture = load_fixture();
    let mut any_failed = false;
    for packet in &fixture.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.outcome == CheckpointOutcomeClass::FailedRecoverable {
                any_failed = true;
            }
            if checkpoint.outcome.requires_recovery() {
                let recovery = checkpoint
                    .recovery
                    .as_ref()
                    .expect("blocked/failed checkpoint retains a recovery");
                assert!(!recovery.restore_checkpoint_ref.is_empty());
                assert!(!recovery.diagnostic_detail.is_empty());
                assert!(!recovery.next_step.is_empty());
                assert!(recovery.offers(RecoveryAffordanceClass::RestoreCheckpoint));
                assert!(recovery.offers(RecoveryAffordanceClass::RetainedDiagnostics));
                assert!(recovery.offers(RecoveryAffordanceClass::NextStepGuidance));
            }
        }
    }
    assert!(any_failed, "no failed-recoverable checkpoint appears");
}

#[test]
fn confirm_and_delete_checkpoints_are_confirmation_gated() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        assert!(packet
            .wizard
            .checkpoints
            .iter()
            .any(|c| c.kind == CheckpointKindClass::Confirm && c.confirmation_required));
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.kind == CheckpointKindClass::Delete {
                assert!(checkpoint.confirmation_required);
                assert!(checkpoint.deletion_schedule.is_some());
            }
        }
    }
}

#[test]
fn local_continuation_rights_are_offline_and_seat_free() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for right in ContinuityRightClass::ALL {
            let guarantee = packet
                .wizard
                .continuity
                .iter()
                .find(|g| g.right == right)
                .expect("continuation right present");
            assert!(guarantee.available_offline);
            assert!(!guarantee.requires_paid_seat);
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = offboarding_lines(&fixture);
    assert!(lines.iter().any(|line| line.contains("Offboarding bundle")));
    for profile in OFFBOARDING_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
