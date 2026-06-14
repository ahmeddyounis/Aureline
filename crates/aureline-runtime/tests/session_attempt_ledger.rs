use aureline_runtime::session_plans_attempt_records_and_execution_lineage::{
    current_session_attempt_ledger_export, AttemptKind, AttemptOutcome, LineageProvenanceClass,
    SessionAttemptLedgerPacket, SessionFlow, SessionPlanMode,
};

fn fixture(name: &str) -> SessionAttemptLedgerPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/session-plans-attempt-records-and-execution-lineage/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_session_attempt_ledger_export()
        .expect("checked-in session attempt ledger export should validate");
    assert!(packet.validate().is_empty());

    for flow in SessionFlow::ALL {
        assert!(
            packet.represented_flows().contains(&flow),
            "missing flow {}",
            flow.as_str()
        );
    }
    for mode in [
        SessionPlanMode::RunSelected,
        SessionPlanMode::RerunFailed,
        SessionPlanMode::ImportProviderJoin,
    ] {
        assert!(
            packet.represented_modes().contains(&mode),
            "missing mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn artifact_demonstrates_each_attributable_flow() {
    let packet = current_session_attempt_ledger_export().expect("export validates");

    // Every session reopens its own append-only attempt history in index order.
    for session in &packet.sessions {
        let attempts = packet.session_attempts(&session.session_id);
        assert!(
            !attempts.is_empty(),
            "session {} must have an attempt history",
            session.session_id
        );
        for (offset, attempt) in attempts.iter().enumerate() {
            assert_eq!(attempt.attempt_index as usize, offset + 1);
        }
    }
}

#[test]
fn imported_attempt_never_reads_as_a_local_rerun() {
    let packet = current_session_attempt_ledger_export().expect("export validates");

    let imported = packet
        .attempts
        .iter()
        .find(|a| a.kind == AttemptKind::ImportedJoin)
        .expect("an imported join attempt");
    assert!(imported.is_imported_attempt());
    assert_eq!(
        imported.lineage.provenance_class,
        LineageProvenanceClass::ImportedReadOnly
    );
    assert!(imported.origin_provider_ref.is_some());
    assert!(imported.outcome.is_imported_outcome());
    assert!(!imported.outcome.is_passing());

    // A local parity rerun lives on the same ledger but keeps local lineage.
    let parity = packet
        .attempts
        .iter()
        .find(|a| a.kind == AttemptKind::LocalParityRerun)
        .expect("a local parity rerun");
    assert!(!parity.is_imported_attempt());
    assert_eq!(parity.flow, SessionFlow::LocalWorkspace);
    assert_eq!(
        parity.lineage.provenance_class,
        LineageProvenanceClass::LocalAuthoritative
    );
    assert!(parity.predecessor_attempt_ref.is_some());
}

#[test]
fn every_attempt_reopens_its_session_and_targets() {
    let packet = current_session_attempt_ledger_export().expect("export validates");
    for attempt in &packet.attempts {
        let session = packet
            .session(&attempt.session_ref)
            .expect("attempt must reopen its session");
        let session_ids = session.target_ids();
        for covered in &attempt.covered_target_ids {
            assert!(
                session_ids.contains(covered.as_str()),
                "attempt {} covers a target outside its session",
                attempt.attempt_id
            );
        }
    }
}

#[test]
fn fixture_imported_stale_join_stays_read_only() {
    let packet = fixture("imported_stale_join_stays_read_only.json");
    assert!(packet.validate().is_empty());

    let stale = packet
        .attempts
        .iter()
        .find(|a| a.kind == AttemptKind::ImportedJoin)
        .expect("imported join");
    // Stale imported evidence is held read-only and never rolls up green.
    assert_eq!(stale.outcome, AttemptOutcome::ImportedStale);
    assert!(stale.is_imported_attempt());
    assert!(!stale.outcome.is_passing());
}
