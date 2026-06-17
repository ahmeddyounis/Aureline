use super::*;

use crate::m5_serialization_and_restore_matrix::current_m5_serialization_matrix;

fn packet() -> M5SerializationQualification {
    current_m5_serialization_qualification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_SERIALIZATION_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_SERIALIZATION_QUALIFICATION_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn upstream_serialization_matrix_parses() {
    // The qualification packet rests on the canonical serialization matrix; if that packet stops
    // parsing, the matrix vocabulary this packet reuses has drifted.
    current_m5_serialization_matrix().expect("upstream matrix parses");
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_family_has_at_least_one_row() {
    let packet = packet();
    for &family in &packet.families {
        assert!(
            packet.rows_for_family(family).next().is_some(),
            "missing row for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_fidelity,
            row.effective_fidelity(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.claim_publication,
            row.computed_publication(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_path,
            row.computed_downgrade_path(),
            "{}",
            row.row_id
        );
    }
}

#[test]
fn every_row_covers_all_drills_with_evidence() {
    let packet = packet();
    for row in &packet.rows {
        assert!(row.covers_all_drills(), "{} misses a drill", row.row_id);
        for result in &row.drill_results {
            assert!(
                result.has_required_evidence(),
                "{} drill {} ran without evidence",
                row.row_id,
                result.drill.as_str()
            );
        }
    }
}

#[test]
fn every_row_carries_required_evidence_refs() {
    let packet = packet();
    for row in &packet.rows {
        assert!(row.has_required_evidence(), "{}", row.row_id);
    }
}

#[test]
fn qualification_never_exceeds_matrix_claim() {
    // The cornerstone non-inheritance guarantee: the qualification never re-broadens a
    // matrix-narrowed surface.
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.published_fidelity.rank() <= row.matrix_claim.rank(),
            "{} publishes above its matrix claim",
            row.row_id
        );
        assert_eq!(
            row.matrix_packet_ref, M5_SERIALIZATION_QUALIFICATION_MATRIX_PACKET_REF,
            "{} binds to the wrong matrix packet",
            row.row_id
        );
    }
}

#[test]
fn narrowed_or_withheld_rows_offer_recovery_and_caveats() {
    let packet = packet();
    for row in &packet.rows {
        if row.claim_publication.is_narrowed() {
            assert!(row.downgrade_path.is_offered(), "{}", row.row_id);
            assert!(!row.caveats.is_empty(), "{}", row.row_id);
            assert!(!row.stale_or_missing_fields.is_empty(), "{}", row.row_id);
        }
    }
}

#[test]
fn withheld_rows_publish_no_qualified_classes() {
    let packet = packet();
    for row in packet.withheld_rows() {
        assert_eq!(
            row.published_fidelity,
            RestoreFidelityClass::ManualReview,
            "{}",
            row.row_id
        );
        assert!(
            row.qualified_classes.is_empty(),
            "{} withholds the claim but still lists qualified classes",
            row.row_id
        );
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in QualificationConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_rows_gate_consistent,
        packet.all_rows_gate_consistent()
    );
    assert_eq!(projection.published_count, packet.published_rows().count());
    assert_eq!(projection.narrowed_count, packet.narrowed_rows().count());
    assert_eq!(projection.withheld_count, packet.withheld_rows().count());
    for (row, export) in packet.rows.iter().zip(projection.rows.iter()) {
        assert_eq!(export.published_fidelity, row.published_fidelity.as_str());
        assert_eq!(export.published, row.is_published());
        assert_eq!(export.downgraded, row.is_downgraded());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export(
        "support:m5:serialization-qualification",
        "2026-06-16T13:00:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.qualification_packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
}

#[test]
fn published_fidelities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RestoreFidelityClass> =
        packet.rows.iter().map(|r| r.published_fidelity).collect();
    for label in RestoreFidelityClass::ALL {
        assert!(
            present.contains(&label),
            "no row publishes {}",
            label.as_str()
        );
    }
}

#[test]
fn claim_publications_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ClaimPublication> =
        packet.rows.iter().map(|r| r.claim_publication).collect();
    for publication in ClaimPublication::ALL {
        assert!(
            present.contains(&publication),
            "no row exercises {}",
            publication.as_str()
        );
    }
}

#[test]
fn deployment_modes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DeploymentMode> =
        packet.rows.iter().map(|r| r.deployment_mode).collect();
    for mode in DeploymentMode::ALL {
        assert!(
            present.contains(&mode),
            "no row exercises {}",
            mode.as_str()
        );
    }
}

#[test]
fn evidence_freshness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> =
        packet.rows.iter().map(|r| r.evidence_freshness).collect();
    for state in EvidenceFreshness::ALL {
        assert!(
            present.contains(&state),
            "no row exercises {}",
            state.as_str()
        );
    }
}

#[test]
fn downgrade_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<QualificationDowngradePath> =
        packet.rows.iter().map(|r| r.downgrade_path).collect();
    for path in QualificationDowngradePath::ALL {
        assert!(
            present.contains(&path),
            "no row exercises {}",
            path.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<QualificationDowngradeReason> = packet
        .rows
        .iter()
        .flat_map(|r| r.downgrade_reasons.iter().copied())
        .collect();
    for reason in QualificationDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no row exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn drill_outcomes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DrillOutcome> = packet
        .rows
        .iter()
        .flat_map(|r| r.drill_results.iter().map(|d| d.outcome))
        .collect();
    for outcome in DrillOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no drill exercises {}",
            outcome.as_str()
        );
    }
}

#[test]
fn families_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<QualificationFamily> = packet.rows.iter().map(|r| r.family).collect();
    for family in QualificationFamily::ALL {
        assert!(
            present.contains(&family),
            "no row covers {}",
            family.as_str()
        );
    }
}

#[test]
fn published_rows_are_whole() {
    let packet = packet();
    assert!(
        packet.published_rows().count() >= 2,
        "fixture needs at least two published rows to prove the gate is not a blanket downgrade"
    );
    for row in packet.published_rows() {
        assert_eq!(row.matrix_claim, RestoreFidelityClass::ExactRestore);
        assert_eq!(row.evidence_freshness, EvidenceFreshness::Current);
        assert_eq!(row.drill_ceiling(), RestoreFidelityClass::ExactRestore);
        assert_eq!(row.published_fidelity, RestoreFidelityClass::ExactRestore);
        assert!(row.downgrade_reasons.is_empty());
        assert!(row.caveats.is_empty());
        assert!(!row.downgrade_path.is_offered());
        assert!(!row.is_downgraded());
        assert!(!row.qualified_classes.is_empty());
    }
}

#[test]
fn matrix_narrowed_row_adopts_narrowing() {
    let packet = packet();
    let row = packet
        .row("m5-serialization-qual:restore_fidelity:managed.fleet")
        .expect("restore-fidelity managed-fleet row");
    assert_eq!(
        row.published_fidelity,
        RestoreFidelityClass::CompatibleRestore
    );
    assert_eq!(row.claim_publication, ClaimPublication::Narrowed);
    assert_eq!(
        row.downgrade_path,
        QualificationDowngradePath::AdoptMatrixNarrowing
    );
    assert_eq!(
        row.downgrade_reasons,
        vec![QualificationDowngradeReason::MatrixNarrowed]
    );
}

#[test]
fn stale_evidence_row_refreshes_evidence() {
    let packet = packet();
    let row = packet
        .row("m5-serialization-qual:portable_state_review:companion.browser")
        .expect("portable-state companion row");
    assert_eq!(
        row.published_fidelity,
        RestoreFidelityClass::CompatibleRestore
    );
    assert_eq!(row.claim_publication, ClaimPublication::Narrowed);
    assert_eq!(
        row.downgrade_path,
        QualificationDowngradePath::RefreshEvidence
    );
    assert_eq!(
        row.downgrade_reasons,
        vec![QualificationDowngradeReason::EvidenceStale]
    );
}

#[test]
fn withheld_row_is_withheld_not_inherited() {
    let packet = packet();
    let row = packet
        .row("m5-serialization-qual:migration_remap:managed.fleet")
        .expect("migration-remap managed-fleet row");
    assert_eq!(row.published_fidelity, RestoreFidelityClass::ManualReview);
    assert_eq!(row.claim_publication, ClaimPublication::Withheld);
    assert_eq!(
        row.downgrade_path,
        QualificationDowngradePath::WithholdClaim
    );
    assert!(row.qualified_classes.is_empty());
    assert!(row
        .downgrade_reasons
        .contains(&QualificationDowngradeReason::MatrixNarrowed));
    assert!(row
        .downgrade_reasons
        .contains(&QualificationDowngradeReason::DrillFailed));
}

#[test]
fn validate_flags_overstated_fidelity() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.effective_fidelity() != RestoreFidelityClass::ExactRestore)
    {
        row.published_fidelity = RestoreFidelityClass::ExactRestore;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::OverstatedFidelity { .. }
        )));
    }
}

#[test]
fn validate_flags_exceeds_matrix() {
    let mut packet = packet();
    // Force a row to publish above its matrix claim without changing the gate's other inputs, so
    // the dedicated guard fires.
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.matrix_claim == RestoreFidelityClass::CompatibleRestore)
    {
        row.matrix_claim = RestoreFidelityClass::LayoutOnly;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::ExceedsMatrix { .. }
        )));
    }
}

#[test]
fn validate_flags_incomplete_drill_coverage() {
    let mut packet = packet();
    if let Some(row) = packet.rows.first_mut() {
        row.drill_results.pop();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::IncompleteDrillCoverage { .. }
        )));
    }
}

#[test]
fn validate_flags_drill_missing_evidence() {
    let mut packet = packet();
    if let Some(result) = packet
        .rows
        .iter_mut()
        .flat_map(|r| r.drill_results.iter_mut())
        .find(|d| d.outcome.was_run())
    {
        result.evidence_ref = None;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::DrillMissingEvidence { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != QualificationConsumerSurface::SupportExport);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5SerializationQualificationViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_stops_narrowing() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.narrows_on_downgrade = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_matrix_packet_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.rows.first_mut() {
        row.matrix_packet_ref = "artifacts/workspace/m5/not-the-matrix.json".to_owned();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::MatrixPacketMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_publication_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.claim_publication != ClaimPublication::Withheld)
    {
        row.claim_publication = ClaimPublication::Withheld;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SerializationQualificationViolation::PublicationMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5SerializationQualificationViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(QualificationFamily::MigrationRemap.as_str(), "migration_remap");
    assert_eq!(
        QualificationFamily::MissingSurfaceContinuity.as_str(),
        "missing_surface_continuity"
    );
    assert_eq!(DeploymentMode::CompanionBrowser.as_str(), "companion_browser");
    assert_eq!(QualificationDrill::SchemaJump.as_str(), "schema_jump");
    assert_eq!(
        QualificationDrill::PlaceholderContinuity.as_str(),
        "placeholder_continuity"
    );
    assert_eq!(DrillOutcome::NotRun.as_str(), "not_run");
    assert_eq!(EvidenceFreshness::Missing.as_str(), "missing");
    assert_eq!(ClaimPublication::Withheld.as_str(), "withheld");
    assert_eq!(
        QualificationDowngradeReason::DrillFailed.as_str(),
        "drill_failed"
    );
    assert_eq!(QualificationDowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(
        QualificationDowngradePath::AdoptMatrixNarrowing.as_str(),
        "adopt_matrix_narrowing"
    );
    assert_eq!(
        QualificationConsumerSurface::CompanionBrowserHandoff.as_str(),
        "companion_browser_handoff"
    );
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        DrillOutcome::Narrowed.fidelity_ceiling(),
        RestoreFidelityClass::CompatibleRestore
    );
    assert_eq!(
        DrillOutcome::Failed.fidelity_ceiling(),
        RestoreFidelityClass::ManualReview
    );
    assert_eq!(
        DrillOutcome::NotRun.fidelity_ceiling(),
        RestoreFidelityClass::ManualReview
    );
    assert_eq!(
        EvidenceFreshness::Aging.fidelity_ceiling(),
        RestoreFidelityClass::CompatibleRestore
    );
    assert_eq!(
        EvidenceFreshness::Expired.fidelity_ceiling(),
        RestoreFidelityClass::LayoutOnly
    );
    assert_eq!(
        EvidenceFreshness::Missing.fidelity_ceiling(),
        RestoreFidelityClass::ManualReview
    );
}
