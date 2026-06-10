use super::*;

fn sample_certified_row(
    lane: NotebookCertificationLaneKind,
    row_id: &str,
    sub_packet_ref: &str,
) -> NotebookCertificationRow {
    NotebookCertificationRow {
        record_kind: NOTEBOOK_CERTIFICATION_ROW_RECORD_KIND.to_owned(),
        notebook_certification_schema_version: NOTEBOOK_CERTIFICATION_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        lane_kind: lane,
        sub_packet_ref: sub_packet_ref.to_owned(),
        certification_state: NotebookCertificationState::CertifiedCurrent,
        rollback_path_state: NotebookCertificationRollbackPathState::Tested,
        downgrade_reasons: vec![],
        narrowing_action: NotebookCertificationNarrowingAction::AutomaticNarrowing,
        freshness_as_of: "2026-06-09T00:00:00Z".to_owned(),
        summary: format!("Certified row for {}.", lane.as_str()),
    }
}

fn sample_narrowed_row() -> NotebookCertificationRow {
    NotebookCertificationRow {
        record_kind: NOTEBOOK_CERTIFICATION_ROW_RECORD_KIND.to_owned(),
        notebook_certification_schema_version: NOTEBOOK_CERTIFICATION_SCHEMA_VERSION,
        row_id: "nb.cert.row.stale_experiment".to_owned(),
        lane_kind: NotebookCertificationLaneKind::Experiment,
        sub_packet_ref: "nb.experiment_lineage.packet.m5.01".to_owned(),
        certification_state: NotebookCertificationState::Narrowed,
        rollback_path_state: NotebookCertificationRollbackPathState::Defined,
        downgrade_reasons: vec![NotebookCertificationDowngradeReason::FreshnessExpired],
        narrowing_action: NotebookCertificationNarrowingAction::AutomaticNarrowing,
        freshness_as_of: "2026-05-01T00:00:00Z".to_owned(),
        summary: "Experiment lineage narrowed due to stale evidence.".to_owned(),
    }
}

#[test]
fn certified_row_validates_clean() {
    let row = sample_certified_row(
        NotebookCertificationLaneKind::DiffReview,
        "nb.cert.row.diff",
        "notebook_diff_packet_v1",
    );
    assert!(
        row.validate().is_empty(),
        "certified row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn narrowed_row_validates_clean() {
    let row = sample_narrowed_row();
    assert!(
        row.validate().is_empty(),
        "narrowed row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn certified_row_rejects_missing_rollback() {
    let mut row = sample_certified_row(
        NotebookCertificationLaneKind::Export,
        "nb.cert.row.export",
        "notebook_share_and_handoff_packet_v1",
    );
    row.rollback_path_state = NotebookCertificationRollbackPathState::Missing;
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_row.missing_rollback_forbids_certified"));
}

#[test]
fn certified_row_rejects_downgrade_reasons() {
    let mut row = sample_certified_row(
        NotebookCertificationLaneKind::Collaboration,
        "nb.cert.row.collab",
        "notebook_collaboration_follow_presenter_packet_v1",
    );
    row.downgrade_reasons = vec![NotebookCertificationDowngradeReason::PolicyBlocked];
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_row.no_downgrade_for_certified"));
}

#[test]
fn narrowed_row_rejects_empty_downgrade_reasons() {
    let mut row = sample_narrowed_row();
    row.downgrade_reasons = vec![];
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_row.narrowed_requires_reason"));
}

#[test]
fn row_rejects_empty_row_id() {
    let mut row = sample_certified_row(
        NotebookCertificationLaneKind::Experiment,
        "nb.cert.row.experiment",
        "nb.experiment_lineage.packet.m5.01",
    );
    row.row_id = "".to_owned();
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_row.row_id_required"));
}

#[test]
fn row_rejects_empty_sub_packet_ref() {
    let mut row = sample_certified_row(
        NotebookCertificationLaneKind::Narrowing,
        "nb.cert.row.narrowing",
        "notebook_certification_packet_v1",
    );
    row.sub_packet_ref = "".to_owned();
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_row.sub_packet_ref_required"));
}

#[test]
fn row_rejects_empty_summary() {
    let mut row = sample_certified_row(
        NotebookCertificationLaneKind::DiffReview,
        "nb.cert.row.diff",
        "notebook_diff_packet_v1",
    );
    row.summary = "".to_owned();
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_row.summary_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(
        NotebookCertificationLaneKind::DiffReview.as_str(),
        "diff_review"
    );
    assert_eq!(
        NotebookCertificationState::CertifiedCurrent.as_str(),
        "certified_current"
    );
    assert_eq!(
        NotebookCertificationDowngradeReason::EvidenceStale.as_str(),
        "evidence_stale"
    );
    assert_eq!(
        NotebookCertificationRollbackPathState::Exercised.as_str(),
        "exercised"
    );
    assert_eq!(
        NotebookCertificationNarrowingAction::ManualHold.as_str(),
        "manual_hold"
    );
}

#[test]
fn rollback_path_state_logic() {
    assert!(NotebookCertificationRollbackPathState::Tested.holds_label());
    assert!(NotebookCertificationRollbackPathState::Exercised.holds_label());
    assert!(!NotebookCertificationRollbackPathState::Defined.holds_label());
    assert!(!NotebookCertificationRollbackPathState::Missing.holds_label());
    assert!(NotebookCertificationRollbackPathState::Missing.forces_narrowing());
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookCertificationPacket {
        schema_version: NOTEBOOK_CERTIFICATION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "notebook_certification_packet_v1".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        certification_rows: vec![
            sample_certified_row(
                NotebookCertificationLaneKind::DiffReview,
                "nb.cert.row.diff",
                "notebook_diff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Export,
                "nb.cert.row.export",
                "notebook_share_and_handoff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Collaboration,
                "nb.cert.row.collab",
                "notebook_collaboration_follow_presenter_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Experiment,
                "nb.cert.row.experiment",
                "nb.experiment_lineage.packet.m5.01",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Narrowing,
                "nb.cert.row.narrowing",
                "notebook_certification_packet_v1",
            ),
        ],
        example_narrowed_rows: vec![sample_narrowed_row()],
        downgrade_rules: vec![
            "If freshness_expired, narrow to Stale.".to_owned(),
            "If rollback_path_missing, narrow to RuleMissing.".to_owned(),
            "If policy_blocked, narrow to Blocked.".to_owned(),
        ],
        rollback_path: vec![
            "Revert to previous certified packet revision.".to_owned(),
            "Notify downstream consumers via support export.".to_owned(),
            "Update docs and help surfaces with degraded-state labels.".to_owned(),
        ],
        freshness_slo_max_age_days: 30,
        warn_window_days: 7,
        summary:
            "Notebook diff, review, export, collaboration, and experiment certification packet v1."
                .to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn packet_rejects_duplicate_lane() {
    let packet = NotebookCertificationPacket {
        schema_version: NOTEBOOK_CERTIFICATION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "notebook_certification_packet_v1".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        certification_rows: vec![
            sample_certified_row(
                NotebookCertificationLaneKind::DiffReview,
                "nb.cert.row.diff",
                "notebook_diff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::DiffReview,
                "nb.cert.row.diff2",
                "notebook_diff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Export,
                "nb.cert.row.export",
                "notebook_share_and_handoff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Collaboration,
                "nb.cert.row.collab",
                "notebook_collaboration_follow_presenter_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Experiment,
                "nb.cert.row.experiment",
                "nb.experiment_lineage.packet.m5.01",
            ),
        ],
        example_narrowed_rows: vec![],
        downgrade_rules: vec![],
        rollback_path: vec![],
        freshness_slo_max_age_days: 30,
        warn_window_days: 7,
        summary: "Test packet.".to_owned(),
    };
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_packet.duplicate_lane"));
}

#[test]
fn packet_rejects_missing_lane() {
    let packet = NotebookCertificationPacket {
        schema_version: NOTEBOOK_CERTIFICATION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "notebook_certification_packet_v1".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        certification_rows: vec![
            sample_certified_row(
                NotebookCertificationLaneKind::DiffReview,
                "nb.cert.row.diff",
                "notebook_diff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Export,
                "nb.cert.row.export",
                "notebook_share_and_handoff_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Collaboration,
                "nb.cert.row.collab",
                "notebook_collaboration_follow_presenter_packet_v1",
            ),
            sample_certified_row(
                NotebookCertificationLaneKind::Experiment,
                "nb.cert.row.experiment",
                "nb.experiment_lineage.packet.m5.01",
            ),
        ],
        example_narrowed_rows: vec![],
        downgrade_rules: vec![],
        rollback_path: vec![],
        freshness_slo_max_age_days: 30,
        warn_window_days: 7,
        summary: "Test packet.".to_owned(),
    };
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_certification_packet.missing_lane"));
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_certification_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_CERTIFICATION_SCHEMA_VERSION);
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND
    );
}
