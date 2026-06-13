use super::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_policy_simulation_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_covers_every_family() {
    let packet = seeded_m5_policy_simulation_packet();
    for family in GovernedArtifactFamily::ALL {
        assert!(
            packet.rows.iter().any(|row| row.artifact_family == family),
            "missing family: {family:?}"
        );
    }
}

#[test]
fn every_row_covers_delete_and_export() {
    let packet = seeded_m5_policy_simulation_packet();
    for row in &packet.rows {
        for action in SimulatedActionClass::ALL {
            assert!(
                row.diff_for(action).is_some(),
                "{:?} missing {action:?}",
                row.entry_id
            );
        }
    }
}

#[test]
fn changed_and_unchanged_partition_is_exhaustive() {
    let packet = seeded_m5_policy_simulation_packet();
    for row in &packet.rows {
        assert_eq!(
            row.changed_actions().len() + row.unchanged_actions().len(),
            row.action_diffs.len(),
            "{:?} partition is not exhaustive",
            row.entry_id
        );
    }
}

#[test]
fn changed_flag_matches_outcome_comparison() {
    let packet = seeded_m5_policy_simulation_packet();
    for row in &packet.rows {
        for diff in &row.action_diffs {
            assert!(
                diff.changed_flag_is_consistent(),
                "{:?} {:?} changed flag inconsistent",
                row.entry_id,
                diff.action
            );
        }
    }
}

#[test]
fn packet_shows_both_changed_and_unchanged_actions() {
    let packet = seeded_m5_policy_simulation_packet();
    assert!(
        packet.impact_summary.changed_action_count > 0,
        "expected at least one changed action"
    );
    assert!(
        packet.impact_summary.unchanged_action_count > 0,
        "expected at least one unchanged action"
    );
}

#[test]
fn expiry_and_downgrade_effects_are_present() {
    let packet = seeded_m5_policy_simulation_packet();
    assert!(
        packet.impact_summary.families_with_expiry_change > 0,
        "expected at least one expiry change in the simulation"
    );
    assert!(
        packet.impact_summary.families_with_downgrade > 0,
        "expected at least one downgrade path in the simulation"
    );
    for row in &packet.rows {
        if row.expiry_effect.effect_class.changes_runtime() {
            assert!(
                !row.expiry_effect.runtime_consequence.trim().is_empty(),
                "{:?} expiry change must state its runtime consequence",
                row.entry_id
            );
        }
        if row.downgrade_path.path_class.is_downgrade() {
            assert!(
                row.downgrade_path.visible_before_publish,
                "{:?} downgrade must be visible before publish",
                row.entry_id
            );
        }
    }
}

#[test]
fn impacted_objects_use_runtime_record_classes() {
    let packet = seeded_m5_policy_simulation_packet();
    for row in &packet.rows {
        assert!(
            !row.impacted_objects.is_empty(),
            "{:?} must list at least one impacted object",
            row.entry_id
        );
        for object in &row.impacted_objects {
            assert_eq!(
                object.record_class_id, row.record_class_id,
                "{:?} impacted object must share the row record class",
                row.entry_id
            );
        }
    }
}

#[test]
fn simulation_shares_identities_with_runtime_packet() {
    let simulation = seeded_m5_policy_simulation_packet();
    let runtime = crate::m5_records_policy::seeded_m5_records_policy_packet();

    for sim_row in &simulation.rows {
        let runtime_row = runtime
            .rows
            .iter()
            .find(|row| row.entry_id == sim_row.entry_id)
            .expect("simulation row matches a runtime hold/retention row");
        assert_eq!(runtime_row.artifact_family, sim_row.artifact_family);
        assert_eq!(runtime_row.record_class_id, sim_row.record_class_id);

        // The simulated "current" outcome must equal the runtime pre-action outcome.
        let delete = sim_row
            .diff_for(SimulatedActionClass::Delete)
            .expect("delete diff present");
        let export = sim_row
            .diff_for(SimulatedActionClass::Export)
            .expect("export diff present");
        assert_eq!(
            delete.current_outcome, runtime_row.pre_delete_truth.projected_outcome,
            "{:?} current delete outcome must match runtime truth",
            sim_row.entry_id
        );
        assert_eq!(
            export.current_outcome, runtime_row.pre_export_truth.projected_outcome,
            "{:?} current export outcome must match runtime truth",
            sim_row.entry_id
        );
    }
}

#[test]
fn local_only_draft_managed_claim_is_rejected() {
    let mut packet = seeded_m5_policy_simulation_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| matches!(row.authority_boundary, AuthorityBoundaryClass::LocalOnly))
        .expect("a local-only row exists");
    row.draft_claims_managed_delete = true;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5PolicySimulationViolation::LocalOnlyDraftClaimsManagedDelete { .. }
    )));
}

#[test]
fn inconsistent_changed_flag_is_rejected() {
    let mut packet = seeded_m5_policy_simulation_packet();
    // Flip a `changed` flag so it disagrees with the compared outcomes.
    packet.rows[0].action_diffs[0].changed = !packet.rows[0].action_diffs[0].changed;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5PolicySimulationViolation::ChangedFlagInconsistent { .. }
    )));
}

#[test]
fn draft_must_advance_policy_epoch() {
    let mut packet = seeded_m5_policy_simulation_packet();
    packet.rows[0].draft_policy_epoch = packet.rows[0].current_policy_epoch.clone();

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5PolicySimulationViolation::DraftPolicyEpochNotAdvanced { .. }
    )));
}

#[test]
fn invisible_downgrade_is_rejected() {
    let mut packet = seeded_m5_policy_simulation_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.downgrade_path.path_class.is_downgrade())
        .expect("a downgrade row exists");
    row.downgrade_path.visible_before_publish = false;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5PolicySimulationViolation::DowngradeNotVisibleBeforePublish { .. }
    )));
}

#[test]
fn tampered_impact_summary_is_rejected() {
    let mut packet = seeded_m5_policy_simulation_packet();
    packet.impact_summary.changed_action_count += 1;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5PolicySimulationViolation::ImpactSummaryMismatch { .. }
    )));
}

#[test]
fn projections_cover_every_row() {
    let packet = seeded_m5_policy_simulation_packet();
    assert_eq!(packet.product_projection().len(), packet.rows.len());
    assert_eq!(packet.cli_headless_projection().len(), packet.rows.len());
    assert_eq!(packet.support_export_projection().len(), packet.rows.len());
}

#[test]
fn changed_action_export_matches_summary() {
    let packet = seeded_m5_policy_simulation_packet();
    assert_eq!(
        packet.changed_action_export().len(),
        packet.impact_summary.changed_action_count
    );
}

#[test]
fn checked_in_canonical_fixture_matches_seeded_packet() {
    let fixture =
        repo_root().join("fixtures/governance/m5_policy_impact_simulation/canonical_packet.yaml");
    let raw = std::fs::read_to_string(&fixture).expect("canonical fixture is readable");
    let parsed: M5PolicyImpactSimulationPacket =
        serde_yaml::from_str(&raw).expect("canonical fixture parses");

    assert!(
        parsed.validate().is_empty(),
        "canonical fixture must validate cleanly: {:?}",
        parsed.validate()
    );
    assert_eq!(
        parsed,
        seeded_m5_policy_simulation_packet(),
        "canonical fixture drifted from the seeded packet; regenerate it"
    );
}
