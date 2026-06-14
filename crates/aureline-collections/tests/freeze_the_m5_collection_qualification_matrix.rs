use aureline_collections::{
    current_m5_collection_qualification_matrix_export, CollectionMatrixDowngradeTrigger,
    CollectionMatrixQualificationClass, CollectionQualificationMatrixPacket,
    CollectionQualificationMatrixViolation, DenseCollectionSurface,
};

fn fixture(name: &str) -> CollectionQualificationMatrixPacket {
    let path = format!(
        "{}/../../fixtures/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_m5_collection_qualification_matrix_export()
        .expect("checked-in matrix export should validate");
    assert!(packet.validate().is_empty());
    for surface in DenseCollectionSurface::ALL {
        assert!(
            packet.represented_surfaces().contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn downgrade_drill_fixture_keeps_claim_below_evidence() {
    let packet = fixture("support_export_row_downgrades_on_unidentified_batch_action.json");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.downgraded_row_count(), 1);

    let downgraded = packet
        .rows
        .iter()
        .find(|row| row.surface == DenseCollectionSurface::SupportExportProjection)
        .expect("support-export row");
    assert!(downgraded.needs_downgrade());
    assert_eq!(
        downgraded.downgrade_trigger,
        Some(CollectionMatrixDowngradeTrigger::UnidentifiedBatchAction)
    );
    assert_eq!(
        downgraded.effective_qualification,
        CollectionMatrixQualificationClass::Held
    );
    assert!(
        downgraded.effective_qualification.rank() < downgraded.claimed_qualification.rank(),
        "downgraded row must rank strictly below its claim"
    );
}

#[test]
fn claimed_row_lacking_semantics_must_downgrade() {
    let mut packet = fixture("support_export_row_downgrades_on_unidentified_batch_action.json");
    // A claimed pipeline row that loses its filter-AST class but keeps its full
    // claim must be rejected: the surface auto-downgrades before promotion.
    let pipeline_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::PipelineRunList)
        .expect("pipeline row");
    pipeline_row.filter_ast_class = None;
    let violations = packet.validate();
    assert!(violations.contains(
        &CollectionQualificationMatrixViolation::RowNotDowngradedOnUnidentifiedSemantics
    ));
}
