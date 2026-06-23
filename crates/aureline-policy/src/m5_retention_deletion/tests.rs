//! Unit tests for the M5 retention/deletion bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(retention_deletion_bundle(), retention_deletion_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = retention_deletion_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = retention_deletion_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: RetentionDeletionBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_managed_profile() {
    let bundle = retention_deletion_bundle();
    assert_eq!(bundle.profiles.len(), RETENTION_PROFILES.len());
    for profile in RETENTION_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.matrix.rows.is_empty());
    }
}

#[test]
fn every_rendered_state_is_admitted_by_the_matrix() {
    let bundle = retention_deletion_bundle();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::RetentionDeletionMatrix)
            .expect("surface present")
            .applicable_states
            .contains(&state)
    };
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            assert!(
                admitted(row.machine_state),
                "{}: row state {} not admitted by the matrix",
                packet.profile.as_str(),
                row.machine_state.as_str()
            );
        }
        assert!(
            admitted(packet.matrix.coverage.coverage_state),
            "{}: coverage state {} not admitted by the matrix",
            packet.profile.as_str(),
            packet.matrix.coverage.coverage_state.as_str()
        );
    }
}

#[test]
fn data_classes_are_distinguished_and_all_present() {
    let bundle = retention_deletion_bundle();
    for class in ArtifactOwnerClass::ALL {
        assert!(
            bundle
                .profiles
                .iter()
                .any(|p| p.matrix.rows.iter().any(|r| r.data_class == class)),
            "data class {} never appears",
            class.as_str()
        );
    }
}

#[test]
fn delete_outcomes_are_all_exercised() {
    let bundle = retention_deletion_bundle();
    for outcome in DeleteOutcomeClass::ALL {
        assert!(
            bundle.profiles.iter().any(|p| p
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
fn non_immediate_deletes_explain_their_remainder() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            if row.delete_outcome.requires_remainder() {
                let rem = row.remainder.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{}: non-immediate row {} has no remainder",
                        packet.profile.as_str(),
                        row.row_id
                    )
                });
                assert!(!rem.what_remains.is_empty());
                assert!(!rem.expected_completion.is_empty());
            } else {
                assert!(
                    row.remainder.is_none(),
                    "{}: immediate row {} carries a remainder",
                    packet.profile.as_str(),
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn deletion_linkages_stay_distinct_and_all_present() {
    let bundle = retention_deletion_bundle();
    // Every non-immediate delete carries at least one specific linkage.
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            if row.delete_outcome.requires_remainder() {
                assert!(
                    !row.linkages.is_empty(),
                    "{}: non-immediate row {} has no linkage",
                    packet.profile.as_str(),
                    row.row_id
                );
            }
        }
    }
    // Every linkage class appears across the bundle.
    for class in DeletionLinkageClass::ALL {
        assert!(
            bundle
                .profiles
                .iter()
                .any(|p| p.matrix.rows.iter().any(|r| r.has_linkage(class))),
            "linkage class {} never appears",
            class.as_str()
        );
    }
}

#[test]
fn receipted_deletes_carry_a_receipt_and_holds_are_named() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            if row.machine_state == AdminStateClass::DeleteReceipted {
                assert!(
                    row.has_linkage(DeletionLinkageClass::DestructionReceipt),
                    "{}: receipted row {} has no destruction receipt",
                    packet.profile.as_str(),
                    row.row_id
                );
            }
            if row.machine_state == AdminStateClass::DeleteBlockedByHold {
                assert!(
                    row.has_linkage(DeletionLinkageClass::LegalHold),
                    "{}: hold-blocked row {} names no hold",
                    packet.profile.as_str(),
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn stale_evidence_never_sits_under_a_confirmed_state() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            if row.evidence_age.is_stale() {
                assert!(
                    !requires_fresh_evidence(row.machine_state),
                    "{}: stale row {} shown under a confirmed state {}",
                    packet.profile.as_str(),
                    row.row_id,
                    row.machine_state.as_str()
                );
            }
        }
    }
}

#[test]
fn local_only_is_distinguished_from_hosted() {
    let bundle = retention_deletion_bundle();
    let any_local = bundle.profiles.iter().any(|p| {
        p.matrix
            .rows
            .iter()
            .any(|r| r.location == DataResidencyClass::LocalOnly)
    });
    let any_hosted = bundle.profiles.iter().any(|p| {
        p.matrix
            .rows
            .iter()
            .any(|r| r.location != DataResidencyClass::LocalOnly)
    });
    assert!(any_local && any_hosted);
}

#[test]
fn states_propagate_into_all_required_surfaces() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
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
fn every_row_has_export_parity_and_both_forms_offered() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            assert!(
                row.has_export_parity(),
                "{}: row {} lacks an export representation",
                packet.profile.as_str(),
                row.row_id
            );
        }
        assert!(packet.matrix.offers(ExportFormatClass::MachineReadableJson));
        assert!(packet
            .matrix
            .offers(ExportFormatClass::PlainLanguageHandoff));
    }
}

#[test]
fn every_profile_is_locally_inspectable_without_a_console() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
        assert!(packet.matrix.coverage.locally_inspectable);
        assert!(packet.matrix.coverage.vendor_console_independent);
    }
}

#[test]
fn blocked_deletes_escalate_beyond_the_local_user() {
    let bundle = retention_deletion_bundle();
    for packet in &bundle.profiles {
        for row in &packet.matrix.rows {
            if row.delete_outcome == DeleteOutcomeClass::Blocked {
                let owner = row
                    .remainder
                    .as_ref()
                    .expect("blocked row has a remainder")
                    .next_step_owner;
                assert_ne!(
                    owner,
                    OwnerEscalationRoleClass::LocalUser,
                    "{}: blocked row {} escalates only to the local user",
                    packet.profile.as_str(),
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = retention_deletion_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn consumer_parity_matches_the_matrix_declaration() {
    let bundle = retention_deletion_bundle();
    let declared = admin_plane_matrix()
        .surface(AdminSurfaceClass::RetentionDeletionMatrix)
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
    let bundle = retention_deletion_bundle();
    let lines = retention_deletion_lines(&bundle);
    assert!(lines
        .iter()
        .any(|l| l.contains("Retention/deletion bundle")));
    for profile in RETENTION_PROFILES {
        assert!(
            lines.iter().any(|l| l.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_a_receipted_delete_with_no_receipt() {
    let mut bundle = retention_deletion_bundle();
    let packet = &mut bundle.profiles[0];
    let row = packet
        .matrix
        .rows
        .iter_mut()
        .find(|r| r.machine_state == AdminStateClass::DeleteReceipted)
        .expect("a receipted row exists");
    row.linkages.clear();
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_non_immediate_delete_with_no_remainder() {
    let mut bundle = retention_deletion_bundle();
    let packet = &mut bundle.profiles[0];
    let row = packet
        .matrix
        .rows
        .iter_mut()
        .find(|r| r.delete_outcome.requires_remainder())
        .expect("a non-immediate row exists");
    row.remainder = None;
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_hold_blocked_delete_with_no_hold() {
    let mut bundle = retention_deletion_bundle();
    let packet = &mut bundle.profiles[0];
    let row = packet
        .matrix
        .rows
        .iter_mut()
        .find(|r| r.machine_state == AdminStateClass::DeleteBlockedByHold)
        .expect("a hold-blocked row exists");
    row.linkages
        .retain(|l| l.linkage != DeletionLinkageClass::LegalHold);
    assert!(bundle.validate().is_err());
}
