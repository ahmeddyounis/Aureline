//! Conformance for the frozen M5 presentation qualification matrix.

use super::*;

fn packet() -> M5PresentationQualificationMatrixPacket {
    seeded_presentation_qualification_matrix_packet()
}

#[test]
fn seeded_packet_validates_clean() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {:?}",
        violations.iter().map(|v| v.as_str()).collect::<Vec<_>>()
    );
}

#[test]
fn coverage_spans_every_surface_kind() {
    let packet = packet();
    assert_eq!(
        packet.represented_surface_kinds().len(),
        PresentationSurfaceKind::ALL.len()
    );
}

#[test]
fn coverage_counts_match_expected_shape() {
    let packet = packet();
    assert_eq!(packet.rows.len(), 8);
    assert_eq!(packet.claimed_row_count(), 7);
    assert_eq!(packet.labs_row_count(), 1);
    assert_eq!(packet.shared_or_imported_row_count(), 1);
    assert_eq!(packet.downgraded_row_count(), 2);
}

#[test]
fn clean_presenter_walkthrough_holds_full_claim() {
    let row = packet()
        .row("presentation-qual:presenter-walkthrough:local:0001")
        .cloned()
        .expect("presenter walkthrough row");
    assert!(!row.needs_downgrade());
    assert_eq!(row.effective_grade, row.claimed_grade);
    assert_eq!(
        row.effective_grade,
        PresentationQualificationGrade::QualifiedClaimedSurface
    );
    assert!(row.speaker_note_privacy_ok());
    assert!(row.follow_truth.truth_holds());
    assert!(row.authority_separation_ok());
    assert!(row.layout_restore_ok());
    assert!(row.provenance_preserved());
}

#[test]
fn embedded_session_is_the_canonical_object_and_round_trips() {
    let row = packet()
        .row("presentation-qual:presenter-walkthrough:local:0001")
        .cloned()
        .expect("presenter walkthrough row");
    let session = row.session;
    assert_eq!(
        session.record_kind,
        crate::presentation_mode::PRESENTATION_SESSION_RECORD_KIND
    );
    assert!(!session.grants_mutation_authority);
    assert!(!session.grants_control_authority);
    assert!(session.speaker_notes_default_local_only);
    // Round-trips through its own canonical boundary shape.
    let json = serde_json::to_string(&session).expect("serializes");
    let parsed: PresentationSession = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, session);
}

#[test]
fn restore_round_trips_under_every_trigger() {
    for row in &packet().rows {
        assert!(
            row.restore_round_trips(),
            "row {} does not restore cleanly",
            row.surface_id
        );
        // The canonical restore path is reused directly.
        let outcome = restore_from_checkpoint(&row.session, RestoreTrigger::CrashRecovery);
        assert!(outcome.matches_checkpoint);
        assert!(!outcome.left_in_improvised_shell);
    }
}

#[test]
fn speaker_notes_default_local_and_never_export_bodies() {
    let row = packet()
        .row("presentation-qual:speaker-notes:local:0001")
        .cloned()
        .expect("speaker notes row");
    assert!(row.speaker_note_privacy.export_safe());
    assert!(row.session.all_notes_local_only());
    assert!(row.session.shared_notes_are_explicit());
    // No raw note body ever reaches the export-safe JSON.
    let json = packet().export_safe_json();
    assert!(!json.contains("Open with the request entry point"));
    assert!(!json.contains("Remind the room the notes stay local"));
}

#[test]
fn classroom_role_never_widens_authority() {
    let row = packet()
        .row("presentation-qual:classroom-teaching:shared:0001")
        .cloned()
        .expect("classroom row");
    assert_eq!(row.classroom_role, Some(TeachingRole::Moderator));
    assert!(row.classroom_role_consistent());
    assert!(row.authority_separation_ok());
    // A shared-session-linked surface rests on imported proof, never local.
    assert!(row.shared_or_imported());
    assert!(row.imported_posture_consistent());
    assert_eq!(
        row.effective_grade,
        PresentationQualificationGrade::QualifiedNarrowedSurface
    );
    assert!(!row.needs_downgrade());
}

#[test]
fn unavailable_surface_downgrades_with_keyboard_path() {
    let row = packet()
        .row("presentation-qual:unavailable-fallback:local:0001")
        .cloned()
        .expect("unavailable fallback row");
    assert!(row.needs_downgrade());
    assert!(row.properly_downgraded());
    assert!(row.keyboard_fallback_ok());
    assert_eq!(
        row.downgrade_trigger,
        Some(PresentationDowngradeTrigger::SurfaceUnavailableDowngraded)
    );
    assert!(row.downgrade_consistent());
}

#[test]
fn stale_proof_downgrades_below_claim() {
    let row = packet()
        .row("presentation-qual:presenter-walkthrough:stale:0001")
        .cloned()
        .expect("stale row");
    assert!(!row.verification_current());
    assert!(row.needs_downgrade());
    assert!(row.properly_downgraded());
    assert_eq!(
        row.downgrade_trigger,
        Some(PresentationDowngradeTrigger::StaleVerificationProof)
    );
}

#[test]
fn labs_row_makes_no_claim_to_downgrade() {
    let row = packet()
        .row("presentation-qual:audience-follow:labs-free-roam:0001")
        .cloned()
        .expect("labs row");
    assert!(!row.is_claimed());
    assert!(!row.needs_downgrade());
    assert_eq!(
        row.effective_grade,
        PresentationQualificationGrade::LabsUnadvertisedSurface
    );
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5PresentationQualificationMatrixPacket =
        serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_surfaces_and_downgrade() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Qualification Matrix"));
    assert!(summary.contains("presenter_walkthrough"));
    assert!(summary.contains("classroom role = `moderator`"));
    assert!(summary.contains("Downgraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_presentation_qualification_matrix_export()
        .expect("checked presentation qualification export validates");
    assert_eq!(checked, packet());
}

#[test]
fn grade_ranks_are_strictly_ordered() {
    assert!(
        PresentationQualificationGrade::QualifiedClaimedSurface.rank()
            > PresentationQualificationGrade::QualifiedNarrowedSurface.rank()
    );
    assert!(
        PresentationQualificationGrade::QualifiedNarrowedSurface.rank()
            > PresentationQualificationGrade::LabsUnadvertisedSurface.rank()
    );
    assert!(
        PresentationQualificationGrade::LabsUnadvertisedSurface.rank()
            > PresentationQualificationGrade::QualificationWithdrawn.rank()
    );
    assert!(
        PresentationQualificationGrade::QualificationWithdrawn.rank()
            > PresentationQualificationGrade::NotApplicable.rank()
    );
}
