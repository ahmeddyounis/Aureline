use super::*;
use std::collections::HashSet;

#[test]
fn seeded_canonical_packet_validates() {
    let packet = seeded_search_export_governance_packet();
    assert_eq!(
        packet.record_kind,
        SEARCH_EXPORT_GOVERNANCE_PACKET_RECORD_KIND
    );
    assert_eq!(packet.packet_id, SEARCH_EXPORT_GOVERNANCE_PACKET_ID);
    let findings = packet.validate();
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    assert!(packet.is_export_safe());
}

#[test]
fn covers_every_export_class_once() {
    let packet = seeded_search_export_governance_packet();
    assert_eq!(packet.export_rows.len(), ALL_EXPORT_CLASSES.len());
    let present = packet.present_export_classes();
    for class in ALL_EXPORT_CLASSES {
        assert!(present.contains(&class), "missing export class {class:?}");
    }
}

#[test]
fn realizes_full_redaction_and_consent_vocabulary() {
    let packet = seeded_search_export_governance_packet();
    let redaction = packet.present_redaction_states();
    for state in ALL_REDACTION_STATES {
        assert!(
            redaction.contains(&state),
            "missing redaction state {state:?}"
        );
    }
    let consent = packet.present_consent_classes();
    for class in ALL_CONSENT_CLASSES {
        assert!(consent.contains(&class), "missing consent class {class:?}");
    }
}

#[test]
fn support_and_incident_packets_carry_no_literal_query_text_by_default() {
    // Acceptance: support exports and incident packets explain what ran, what was
    // selected, and what was omitted without storing literal query text.
    let packet = seeded_search_export_governance_packet();
    for row in &packet.export_rows {
        if row.export_class.leaves_device() {
            assert!(
                row.export_packet.query_text.is_none(),
                "{} leaks literal query text off the device",
                row.row_id
            );
            assert!(!row.literal_query_text_included);
            // What ran, what was selected, what was omitted is still inspectable.
            assert!(!row.export_packet.query_session_id_ref.is_empty());
            assert!(!row.export_packet.included_result_refs.is_empty());
            assert!(!row.export_packet.evidence_refs.is_empty());
        }
    }
    // The support bundle still discloses omitted/hidden counts.
    let bundle = packet
        .export_row("export:support-bundle")
        .expect("bundle row");
    assert!(bundle.export_packet.count_summary.omitted_result_count > 0);
    assert!(bundle
        .export_packet
        .omitted_or_truncated_flags
        .contains(&"hidden_by_current_scope".to_owned()));
}

#[test]
fn literal_query_text_requires_elevated_consent_and_a_permitting_class() {
    // Acceptance: literal query text requires explicit higher-trust consent and is
    // confined to a local-only replay packet.
    let packet = seeded_search_export_governance_packet();
    let literal_rows: Vec<_> = packet
        .export_rows
        .iter()
        .filter(|row| row.literal_query_text_included)
        .collect();
    assert_eq!(
        literal_rows.len(),
        1,
        "only the local replay row keeps literal text"
    );
    let row = literal_rows[0];
    assert_eq!(row.export_class, SearchExportClass::LocalReplay);
    assert!(!row.export_class.leaves_device());
    assert_eq!(
        row.literal_query_consent,
        ExportConsentClass::QueryTextElevated
    );
    assert!(row.export_packet.query_text.is_some());
    assert_eq!(
        row.export_packet.redaction_state,
        SearchPacketRedactionState::RawQueryLocalOnly
    );
    assert!(packet.literal_query_text_is_local_only());
}

#[test]
fn managed_analytics_carries_no_query_material() {
    let packet = seeded_search_export_governance_packet();
    let row = packet
        .export_row("export:managed-analytics")
        .expect("managed analytics row");
    assert_eq!(row.export_class, SearchExportClass::ManagedAnalytics);
    assert!(row.export_packet.query_text.is_none());
    assert!(row.export_packet.query_hash.is_none());
    assert_eq!(
        row.export_packet.redaction_state,
        SearchPacketRedactionState::QueryMaterialOmittedByPolicy
    );
}

#[test]
fn every_packet_is_replay_safe_and_never_claims_live_results() {
    // Acceptance: replay-safe packets preserve intent and provenance without
    // claiming live current results.
    let packet = seeded_search_export_governance_packet();
    for row in &packet.export_rows {
        let replay = &row.replay_safety;
        assert!(
            !replay.claims_live_current_results,
            "{} claims live",
            row.row_id
        );
        assert!(replay.preserves_intent_and_provenance);
        assert_ne!(
            replay.result_semantics,
            SearchResultSemantics::CurrentLiveResults
        );
        assert_ne!(replay.snapshot_truth, SearchExportSnapshotTruth::LiveRerun);
        assert_eq!(replay.snapshot_truth, row.export_packet.snapshot_truth);
        if replay.scope_drifted() {
            assert!(replay.rerun_required_for_current_truth);
        }
    }
    // The incident row proves a disclosed drift that must rerun.
    let incident = packet
        .export_row("export:incident-packet")
        .expect("incident row");
    assert!(incident.replay_safety.scope_drifted());
    assert_eq!(
        incident.export_packet.snapshot_truth,
        SearchExportSnapshotTruth::ScopeChangedSinceCapture
    );
}

#[test]
fn consumers_read_the_same_packets_under_the_same_privacy_rules() {
    // Acceptance: search privacy rules stay consistent across desktop, CLI,
    // support export, and managed analytics.
    let packet = seeded_search_export_governance_packet();
    let mut seen = HashSet::new();
    for projection in &packet.consumer_projections {
        assert_eq!(projection.ingested_packet_id, packet.packet_id);
        assert!(projection.preserves_redaction_mode);
        assert!(projection.preserves_count_and_omission_disclosure);
        assert!(projection.preserves_replay_safety);
        assert!(projection.reuses_same_export_packets);
        assert!(projection.literal_query_text_excluded);
        assert!(projection.ambient_authority_excluded);
        seen.insert(projection.consumer);
    }
    for consumer in SearchExportConsumerClass::ALL {
        assert!(seen.contains(&consumer), "missing consumer {consumer:?}");
    }
}

#[test]
fn embedded_export_packets_are_export_safe() {
    let packet = seeded_search_export_governance_packet();
    for row in &packet.export_rows {
        assert!(
            row.export_packet.validate_export_safe().is_empty(),
            "{} embedded packet is not export-safe",
            row.row_id
        );
        assert_eq!(
            row.export_packet.destination,
            row.export_class.export_destination()
        );
    }
}

#[test]
fn checked_in_packet_matches_seeded_canonical() {
    let checked =
        current_search_export_governance_packet().expect("checked-in packet parses and validates");
    assert_eq!(checked, seeded_search_export_governance_packet());
}

#[test]
fn support_export_redacts_all_literal_query_text() {
    let packet = seeded_search_export_governance_packet();
    assert!(!packet.contains_no_literal_query_text());
    let export = packet.support_export("search-export-1", "2026-06-17T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.redacted_packet.contains_no_literal_query_text());
    assert!(export.redacted_packet.validate().is_empty());
    // The redacted copy keeps everything else, including hashes, refs, and counts.
    assert_eq!(
        export.redacted_packet.export_rows.len(),
        packet.export_rows.len()
    );
    assert_eq!(
        export.redacted_packet,
        seeded_redacted_search_export_packet()
    );
}

#[test]
fn redact_for_export_keeps_hashes_refs_and_counts() {
    let packet = seeded_search_export_governance_packet();
    let redacted = packet.redact_for_export();
    let row = redacted
        .export_row("export:local-replay")
        .expect("local replay row");
    assert!(row.export_packet.query_text.is_none());
    assert!(row.export_packet.query_hash.is_some());
    assert_eq!(row.export_packet.query_text_mode, QueryTextMode::HashOnly);
    assert_eq!(
        row.export_packet.redaction_state,
        SearchPacketRedactionState::QueryHashOnly
    );
    assert!(!row.literal_query_text_included);
    assert_eq!(row.literal_query_consent, ExportConsentClass::MetadataOnly);
    // Counts and refs survive redaction.
    assert!(!row.export_packet.included_result_refs.is_empty());
    assert!(!row.export_packet.evidence_refs.is_empty());
}

#[test]
fn detects_literal_text_leaving_the_device() {
    let mut packet = seeded_search_export_governance_packet();
    let row = packet
        .export_rows
        .iter_mut()
        .find(|row| row.export_class == SearchExportClass::SupportBundle)
        .expect("support bundle row");
    row.export_packet.query_text = Some("kind:file flaky".to_owned());
    row.literal_query_text_included = true;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("literal query text must never leave the device")
        || finding.message.contains("must not contain raw query text")));
}

#[test]
fn detects_literal_text_without_elevated_consent() {
    let mut packet = seeded_search_export_governance_packet();
    let row = packet
        .export_rows
        .iter_mut()
        .find(|row| row.export_class == SearchExportClass::LocalReplay)
        .expect("local replay row");
    row.literal_query_consent = ExportConsentClass::MetadataOnly;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("requires explicit elevated consent")));
}

#[test]
fn detects_packet_claiming_live_results() {
    let mut packet = seeded_search_export_governance_packet();
    packet.export_rows[0]
        .replay_safety
        .claims_live_current_results = true;
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("never claim live current results")));
}

#[test]
fn detects_dropped_omission_disclosure() {
    let mut packet = seeded_search_export_governance_packet();
    let row = packet
        .export_rows
        .iter_mut()
        .find(|row| row.export_class == SearchExportClass::SupportBundle)
        .expect("support bundle row");
    row.export_packet.omitted_or_truncated_flags.clear();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("omitted/truncated flags")));
}

#[test]
fn detects_managed_analytics_carrying_query_material() {
    let mut packet = seeded_search_export_governance_packet();
    let row = packet
        .export_rows
        .iter_mut()
        .find(|row| row.export_class == SearchExportClass::ManagedAnalytics)
        .expect("managed analytics row");
    row.export_packet.query_hash = Some("fnv1a64:deadbeefdeadbeef".to_owned());
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("must not carry literal or hash query material")));
}

#[test]
fn detects_missing_export_class_coverage() {
    let mut packet = seeded_search_export_governance_packet();
    packet.covered_export_classes.pop();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("every export class")));

    let mut packet = seeded_search_export_governance_packet();
    packet
        .export_rows
        .retain(|row| row.export_class != SearchExportClass::IncidentPacket);
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("realizes export class")));
}
