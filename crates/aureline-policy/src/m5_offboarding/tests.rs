//! Unit tests for the M5 offboarding bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(offboarding_bundle(), offboarding_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = offboarding_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = offboarding_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: OffboardingBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_managed_profile() {
    let bundle = offboarding_bundle();
    assert_eq!(bundle.profiles.len(), OFFBOARDING_PROFILES.len());
    for profile in OFFBOARDING_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.wizard.checkpoints.is_empty());
    }
}

#[test]
fn every_rendered_state_is_admitted_by_the_matrix() {
    let bundle = offboarding_bundle();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::OffboardingWizard)
            .expect("surface present")
            .applicable_states
            .contains(&state)
    };
    for packet in &bundle.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            assert!(
                admitted(checkpoint.machine_state),
                "{}: checkpoint state {} not admitted by the matrix",
                packet.profile.as_str(),
                checkpoint.machine_state.as_str()
            );
        }
        assert!(
            admitted(packet.wizard.coverage.coverage_state),
            "{}: coverage state {} not admitted by the matrix",
            packet.profile.as_str(),
            packet.wizard.coverage.coverage_state.as_str()
        );
    }
}

#[test]
fn checkpoints_are_ordered_and_complete_per_profile() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
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
            assert!(
                window[0].order < window[1].order,
                "{}: checkpoints are not in ascending order",
                packet.profile.as_str()
            );
        }
        for checkpoint in &packet.wizard.checkpoints {
            assert_eq!(checkpoint.order, checkpoint.kind.order());
        }
    }
}

#[test]
fn no_checkpoint_trigger_or_coverage_requires_a_paid_seat() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        assert!(packet.wizard.coverage.completable_without_paid_seat);
        for checkpoint in &packet.wizard.checkpoints {
            assert!(
                !checkpoint.requires_paid_seat,
                "{}: checkpoint {} requires a paid seat",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            );
        }
        for trigger in &packet.wizard.triggers {
            assert!(
                !trigger.requires_active_seat_for_recovery,
                "{}: trigger {} requires an active seat for recovery",
                packet.profile.as_str(),
                trigger.trigger.as_str()
            );
        }
    }
}

#[test]
fn every_trigger_explains_impact_and_all_classes_appear() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for trigger in &packet.wizard.triggers {
            assert!(!trigger.impacted_features.is_empty());
            assert!(!trigger.export_rights.is_empty());
            assert!(!trigger.local_safe_continuation.is_empty());
            assert!(!trigger.managed_copies_summary.is_empty());
        }
    }
    for class in OffboardingTriggerClass::ALL {
        assert!(
            bundle
                .profiles
                .iter()
                .any(|p| p.wizard.triggers.iter().any(|t| t.trigger == class)),
            "trigger class {} never appears",
            class.as_str()
        );
    }
}

#[test]
fn scopes_are_distinguished_and_all_present() {
    let bundle = offboarding_bundle();
    for scope in OffboardingScopeClass::ALL {
        assert!(
            bundle
                .profiles
                .iter()
                .any(|p| p.wizard.checkpoints.iter().any(|c| c.scope == scope)),
            "scope {} never appears",
            scope.as_str()
        );
    }
}

#[test]
fn confirm_and_delete_checkpoints_are_confirmation_gated() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        assert!(
            packet
                .wizard
                .checkpoints
                .iter()
                .any(|c| c.kind == CheckpointKindClass::Confirm && c.confirmation_required),
            "{}: no explicit confirm checkpoint",
            packet.profile.as_str()
        );
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.kind == CheckpointKindClass::Delete {
                assert!(
                    checkpoint.confirmation_required,
                    "{}: delete checkpoint {} is not confirmation-gated",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id
                );
            }
        }
    }
}

#[test]
fn managed_copies_remaining_names_what_where_when() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.managed_copies.remains() {
                assert!(
                    !checkpoint.managed_copies.what_remains.is_empty(),
                    "{}: checkpoint {} leaves a copy without naming what remains",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id
                );
                assert!(!checkpoint.managed_copies.cleared_when.is_empty());
            } else {
                assert!(checkpoint.managed_copies.what_remains.is_empty());
            }
        }
    }
}

#[test]
fn blocked_and_failed_checkpoints_retain_typed_recovery() {
    let bundle = offboarding_bundle();
    let mut any_failed = false;
    for packet in &bundle.profiles {
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
fn delete_checkpoints_carry_a_schedule_and_all_outcomes_appear() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.kind == CheckpointKindClass::Delete {
                let schedule = checkpoint
                    .deletion_schedule
                    .as_ref()
                    .expect("delete checkpoint has a schedule");
                if schedule.outcome.requires_remainder() {
                    assert!(!schedule.what_remains.is_empty());
                    assert!(!schedule.when.is_empty());
                }
            }
        }
    }
    for outcome in DeleteOutcomeClass::ALL {
        assert!(
            bundle
                .profiles
                .iter()
                .any(|p| p.wizard.checkpoints.iter().any(|c| c
                    .deletion_schedule
                    .as_ref()
                    .is_some_and(|s| s.outcome == outcome))),
            "delete outcome {} never appears",
            outcome.as_str()
        );
    }
}

#[test]
fn transfer_checkpoints_name_a_transfer_owner() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.kind == CheckpointKindClass::Transfer {
                assert!(
                    checkpoint.transfer.is_some(),
                    "{}: transfer checkpoint {} names no transfer plan",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id
                );
            }
        }
    }
}

#[test]
fn local_continuation_rights_are_offline_and_seat_free() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for right in ContinuityRightClass::ALL {
            let guarantee = packet
                .wizard
                .continuity
                .iter()
                .find(|g| g.right == right)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: continuation right {} missing",
                        packet.profile.as_str(),
                        right.as_str()
                    )
                });
            assert!(guarantee.available_offline);
            assert!(!guarantee.requires_paid_seat);
        }
        assert!(packet
            .wizard
            .has_kind(CheckpointKindClass::LocalContinuation));
    }
}

#[test]
fn every_checkpoint_has_export_parity_and_both_forms_offered() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            assert!(
                checkpoint.has_export_parity(),
                "{}: checkpoint {} lacks an export representation",
                packet.profile.as_str(),
                checkpoint.checkpoint_id
            );
        }
        assert!(packet.wizard.offers(ExportFormatClass::MachineReadableJson));
        assert!(packet
            .wizard
            .offers(ExportFormatClass::PlainLanguageHandoff));
    }
}

#[test]
fn stale_evidence_never_sits_under_a_confirmed_state() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        for checkpoint in &packet.wizard.checkpoints {
            if checkpoint.evidence_age.is_stale() {
                assert!(
                    !requires_fresh_evidence(checkpoint.machine_state),
                    "{}: stale checkpoint {} shown under a confirmed state {}",
                    packet.profile.as_str(),
                    checkpoint.checkpoint_id,
                    checkpoint.machine_state.as_str()
                );
            }
        }
    }
}

#[test]
fn every_outcome_and_disposition_is_exercised() {
    let bundle = offboarding_bundle();
    for outcome in CheckpointOutcomeClass::ALL {
        assert!(
            bundle.profiles.iter().any(|p| p
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
            bundle.profiles.iter().any(|p| p
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
fn every_profile_is_locally_inspectable_without_a_console() {
    let bundle = offboarding_bundle();
    for packet in &bundle.profiles {
        assert!(packet.wizard.coverage.locally_inspectable);
        assert!(packet.wizard.coverage.vendor_console_independent);
        assert!(packet.wizard.coverage.completable_without_paid_seat);
    }
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = offboarding_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn consumer_parity_matches_the_matrix_declaration() {
    let bundle = offboarding_bundle();
    let declared = admin_plane_matrix()
        .surface(AdminSurfaceClass::OffboardingWizard)
        .expect("surface present")
        .consumed_by
        .clone();
    assert!(!declared.is_empty());
    for packet in &bundle.profiles {
        for consumer in &declared {
            assert!(
                packet.consumers.contains(consumer),
                "{}: packet does not serve declared consumer {:?}",
                packet.profile.as_str(),
                consumer
            );
        }
    }
}

#[test]
fn human_readable_projection_mentions_every_profile() {
    let bundle = offboarding_bundle();
    let lines = offboarding_lines(&bundle);
    assert!(lines.iter().any(|l| l.contains("Offboarding bundle")));
    for profile in OFFBOARDING_PROFILES {
        assert!(
            lines.iter().any(|l| l.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_a_blocked_checkpoint_with_no_recovery() {
    let mut bundle = offboarding_bundle();
    let packet = &mut bundle.profiles[1];
    let checkpoint = packet
        .wizard
        .checkpoints
        .iter_mut()
        .find(|c| c.outcome.requires_recovery())
        .expect("a blocked/failed checkpoint exists");
    checkpoint.recovery = None;
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_paid_seat_requirement() {
    let mut bundle = offboarding_bundle();
    bundle.profiles[0].wizard.checkpoints[1].requires_paid_seat = true;
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_delete_with_no_schedule() {
    let mut bundle = offboarding_bundle();
    let packet = &mut bundle.profiles[0];
    let checkpoint = packet
        .wizard
        .checkpoints
        .iter_mut()
        .find(|c| c.kind == CheckpointKindClass::Delete)
        .expect("a delete checkpoint exists");
    checkpoint.deletion_schedule = None;
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_remaining_managed_copy_with_no_remainder() {
    let mut bundle = offboarding_bundle();
    let packet = &mut bundle.profiles[1];
    let checkpoint = packet
        .wizard
        .checkpoints
        .iter_mut()
        .find(|c| c.managed_copies.remains())
        .expect("a checkpoint leaves a managed copy");
    checkpoint.managed_copies.what_remains = String::new();
    checkpoint.managed_copies.cleared_when = String::new();
    assert!(bundle.validate().is_err());
}
