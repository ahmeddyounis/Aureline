use super::*;

#[test]
fn current_export_is_clean() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    assert!(export.is_clean(), "{export:?}");
    assert!(export.raw_private_material_excluded);
}

#[test]
fn exception_projections_are_visible_in_export() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    let rows = export.exception_expiry_packet.rows.len();
    assert_eq!(export.exception_request_sheets.len(), rows);
    assert_eq!(export.exception_approval_history.len(), rows);
    assert_eq!(export.exception_expiry_banners.len(), rows);
    assert_eq!(export.remembered_decision_revalidations.len(), rows);
    // Every approval-history row carries its lineage so support can replay it.
    assert!(export
        .exception_approval_history
        .iter()
        .all(|row| !row.events.is_empty()));
    // At the packet's own as_of with no observed drift, every remembered
    // decision still validates and none widens authority.
    assert!(export
        .remembered_decision_revalidations
        .iter()
        .all(|revalidation| !revalidation.must_reauthorize && !revalidation.widens_authority));
}

#[test]
fn projection_covers_every_family() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    assert_eq!(
        export.projection_rows.len(),
        export.hold_retention_packet.rows.len()
    );
}

#[test]
fn every_referenced_exception_resolves() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    let known = export.exception_expiry_packet.exception_ids();
    for exception_id in export.hold_retention_packet.referenced_exception_ids() {
        assert!(known.contains(&exception_id), "unresolved: {exception_id}");
    }
}

#[test]
fn unresolved_exception_ref_is_flagged() {
    let mut hold_retention_packet =
        aureline_records::m5_records_policy::seeded_m5_records_policy_packet();
    hold_retention_packet.rows[0]
        .exception_refs
        .push("m5-exception:does-not-exist".to_owned());
    let exception_expiry_packet =
        aureline_policy::m5_exception_expiry::seeded_m5_exception_expiry_packet();
    let policy_simulation_packet =
        aureline_records::m5_policy_simulation::seeded_m5_policy_simulation_packet();

    let violations = M5RecordsPolicyGovernanceSupportExport::cross_validate(
        &hold_retention_packet,
        &exception_expiry_packet,
        &policy_simulation_packet,
    );
    assert!(violations.iter().any(|violation| matches!(
        violation,
        M5RecordsPolicyGovernanceViolation::ExceptionRefUnresolved { .. }
    )));
}

#[test]
fn simulation_previews_every_runtime_family() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    assert_eq!(
        export.simulation_projection_rows.len(),
        export.hold_retention_packet.rows.len()
    );
    for runtime_row in &export.hold_retention_packet.rows {
        assert!(
            export
                .policy_simulation_packet
                .rows
                .iter()
                .any(|sim| sim.entry_id == runtime_row.entry_id),
            "simulation missing runtime family: {}",
            runtime_row.entry_id
        );
    }
}

#[test]
fn missing_simulation_family_is_flagged() {
    let hold_retention_packet =
        aureline_records::m5_records_policy::seeded_m5_records_policy_packet();
    let exception_expiry_packet =
        aureline_policy::m5_exception_expiry::seeded_m5_exception_expiry_packet();
    let mut policy_simulation_packet =
        aureline_records::m5_policy_simulation::seeded_m5_policy_simulation_packet();
    policy_simulation_packet.rows.remove(0);

    let violations = M5RecordsPolicyGovernanceSupportExport::cross_validate(
        &hold_retention_packet,
        &exception_expiry_packet,
        &policy_simulation_packet,
    );
    assert!(violations.iter().any(|violation| matches!(
        violation,
        M5RecordsPolicyGovernanceViolation::SimulationFamilyCoverageMismatch { .. }
    )));
}
