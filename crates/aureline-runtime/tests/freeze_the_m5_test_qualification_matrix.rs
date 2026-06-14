use aureline_runtime::freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix::{
    current_m5_test_qualification_matrix_export, DiscoverySnapshotClass, SessionAttemptClass,
    TestIntelligenceSurface, TestMatrixDowngradeTrigger, TestMatrixQualificationClass,
    TestQualificationMatrixPacket, TestQualificationMatrixViolation,
};

fn fixture(name: &str) -> TestQualificationMatrixPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_m5_test_qualification_matrix_export()
        .expect("checked-in test qualification matrix export should validate");
    assert!(packet.validate().is_empty());
    for surface in TestIntelligenceSurface::ALL {
        assert!(
            packet.represented_surfaces().contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn downgrade_drill_fixture_auto_narrows() {
    let packet = fixture("support_export_row_downgrades_on_unidentified_session_attempt.json");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.downgraded_row_count(), 1);

    let downgraded = packet
        .rows
        .iter()
        .find(|row| row.surface == TestIntelligenceSurface::SupportExportProjection)
        .expect("support-export row");
    assert!(downgraded.needs_downgrade());
    assert_eq!(downgraded.session_attempt_class, None);
    assert_eq!(
        downgraded.effective_qualification,
        TestMatrixQualificationClass::Held
    );
    assert!(
        downgraded.effective_qualification.rank() < downgraded.claimed_qualification.rank(),
        "downgraded row must rank strictly below its claim"
    );
    assert_eq!(
        downgraded.downgrade_trigger,
        Some(TestMatrixDowngradeTrigger::UnidentifiedSessionAttempt)
    );
}

#[test]
fn partial_discovery_stays_visible_in_fixture() {
    let packet = fixture("support_export_row_downgrades_on_unidentified_session_attempt.json");
    let notebook = packet
        .rows
        .iter()
        .find(|row| row.surface == TestIntelligenceSurface::NotebookTestCells)
        .expect("notebook row");
    assert_eq!(
        notebook.discovery_snapshot_class,
        Some(DiscoverySnapshotClass::PartialVisibleDiscovery)
    );
    assert!(notebook.partial_discovery_stays_visible);
    assert!(notebook.discovery_visibility_ok());
}

#[test]
fn imported_overlay_never_reads_as_local() {
    let packet = fixture("support_export_row_downgrades_on_unidentified_session_attempt.json");
    let overlay = packet
        .rows
        .iter()
        .find(|row| row.surface == TestIntelligenceSurface::CiImportOverlay)
        .expect("ci import overlay row");
    assert_eq!(
        overlay.session_attempt_class,
        Some(SessionAttemptClass::ImportedCiSession)
    );
    assert!(overlay.imported_results_not_shown_as_local);
    assert!(overlay.imported_local_separation_ok());
}

#[test]
fn claimed_row_losing_session_must_narrow() {
    let mut packet = fixture("support_export_row_downgrades_on_unidentified_session_attempt.json");
    // A claimed coverage row that loses its session-attempt class but keeps its
    // full claim must be rejected: the surface auto-narrows before promotion.
    let coverage_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == TestIntelligenceSurface::CoverageSurface)
        .expect("coverage row");
    coverage_row.session_attempt_class = None;
    let violations = packet.validate();
    assert!(violations
        .contains(&TestQualificationMatrixViolation::RowNotDowngradedOnUnidentifiedObjects));
}
