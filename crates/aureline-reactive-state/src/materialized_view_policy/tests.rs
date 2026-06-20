use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_materialized_view_policy();
    validate_materialized_view_policy(&packet)
        .expect("seeded policy must satisfy the frozen contract");
}

#[test]
fn seeded_fixtures_validate() {
    let packet = seeded_materialized_view_policy();
    let fixtures = seeded_materialized_view_policy_fixtures();
    assert_eq!(fixtures.len(), 12);
    for fixture in &fixtures {
        validate_materialized_view_policy_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn policy_covers_all_four_classes_once() {
    let packet = seeded_materialized_view_policy();
    assert_eq!(packet.classes.len(), 4);
    let seen: BTreeSet<_> = packet.classes.iter().map(|c| c.view_class).collect();
    assert_eq!(seen.len(), 4, "every class must appear exactly once");
}

#[test]
fn no_class_presents_authoritative_truth_on_read() {
    let packet = seeded_materialized_view_policy();
    for row in &packet.classes {
        assert_eq!(row.authority_on_read, ReadAuthority::DerivedProjection);
    }
}

#[test]
fn disposition_matrix_is_the_full_grid() {
    let packet = seeded_materialized_view_policy();
    assert_eq!(packet.disposition_matrix.len(), 4 * 5);
    for view_class in ViewClass::all() {
        for operation in LifecycleOperation::all() {
            let row = packet
                .disposition_matrix
                .iter()
                .find(|r| r.view_class == view_class && r.operation == operation)
                .expect("every class/operation pair must be covered");
            assert_eq!(row.disposition, disposition_for(view_class, operation));
        }
    }
}

#[test]
fn managed_and_exportable_do_not_inherit_ephemeral_lifecycle() {
    let ephemeral = class_semantics(ViewClass::EphemeralProjection);
    for guarded in [
        ViewClass::ExportableSnapshot,
        ViewClass::ManagedReplicatedView,
    ] {
        let sem = class_semantics(guarded);
        assert_ne!(sem.retention, ephemeral.retention);
        assert_ne!(sem.delete_semantics, ephemeral.delete_semantics);
        assert_ne!(sem.hold_offboarding, ephemeral.hold_offboarding);
        assert_ne!(sem.export, ephemeral.export);
        assert_ne!(sem.support_bundle, ephemeral.support_bundle);
    }
}

#[test]
fn clear_data_distinguishes_every_class() {
    let dispositions: BTreeSet<_> = ViewClass::all()
        .into_iter()
        .map(|vc| disposition_for(vc, LifecycleOperation::ClearData))
        .collect();
    assert_eq!(
        dispositions.len(),
        4,
        "each class must have a distinct clear-data disposition"
    );
}

#[test]
fn clear_data_preserves_exportable_snapshots() {
    // The exportable-snapshot class is a user-authored artifact; a
    // clear-data sweep must not evict it like an ephemeral cache.
    assert_eq!(
        disposition_for(ViewClass::ExportableSnapshot, LifecycleOperation::ClearData),
        Disposition::SavedArtifactPreserved
    );
    assert!(class_semantics(ViewClass::ExportableSnapshot).survives_clear_data);
}

#[test]
fn exportable_snapshot_restores_from_saved_copy() {
    assert_eq!(
        disposition_for(ViewClass::ExportableSnapshot, LifecycleOperation::Restore),
        Disposition::RestoredFromSavedArtifact
    );
    assert!(!class_semantics(ViewClass::ExportableSnapshot).rebuildable_from_authority);
}

#[test]
fn view_class_vocabulary_matches_envelope() {
    use crate::envelope::ViewClass as EnvelopeViewClass;
    assert_eq!(
        ViewClass::EphemeralProjection.as_str(),
        EnvelopeViewClass::EphemeralProjection.as_str()
    );
    assert_eq!(
        ViewClass::DurableLocalMaterialization.as_str(),
        EnvelopeViewClass::DurableLocalMaterialization.as_str()
    );
    assert_eq!(
        ViewClass::ExportableSnapshot.as_str(),
        EnvelopeViewClass::ExportableSnapshot.as_str()
    );
    assert_eq!(
        ViewClass::ManagedReplicatedView.as_str(),
        EnvelopeViewClass::ManagedReplicatedView.as_str()
    );
}

#[test]
fn drift_in_a_class_row_is_rejected() {
    let mut packet = seeded_materialized_view_policy();
    packet.classes[0].retention = RetentionClass::UntilArtifactDeleted;
    let report = validate_materialized_view_policy(&packet).expect_err("drift must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "packet.classes"));
}

#[test]
fn drift_in_the_matrix_is_rejected() {
    let mut packet = seeded_materialized_view_policy();
    packet.disposition_matrix[0].disposition = Disposition::RetainedUnderHold;
    let report = validate_materialized_view_policy(&packet).expect_err("drift must be rejected");
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.check_id == "packet.disposition_matrix"
                || v.check_id == "disposition.mismatch")
    );
}
