use super::*;
use std::collections::HashSet;

#[test]
fn seeded_canonical_packet_validates() {
    let packet = seeded_saved_query_governance_packet();
    assert_eq!(
        packet.record_kind,
        SAVED_QUERY_GOVERNANCE_PACKET_RECORD_KIND
    );
    assert_eq!(packet.packet_id, SAVED_QUERY_GOVERNANCE_PACKET_ID);
    let findings = packet.validate();
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    assert!(packet.is_export_safe());
}

#[test]
fn covers_every_privacy_class_once() {
    let packet = seeded_saved_query_governance_packet();
    assert_eq!(packet.saved_queries.len(), ALL_PRIVACY_CLASSES.len());
    let present = packet.present_privacy_classes();
    for privacy in ALL_PRIVACY_CLASSES {
        assert!(
            present.contains(&privacy),
            "missing privacy class {privacy:?}"
        );
    }
}

#[test]
fn realizes_full_sync_retention_and_redaction_vocabulary() {
    let packet = seeded_saved_query_governance_packet();
    let sync = packet.present_sync_classes();
    for class in ALL_SYNC_CLASSES {
        assert!(sync.contains(&class), "missing sync class {class:?}");
    }
    let retention = packet.present_retention_modes();
    for mode in ALL_RETENTION_MODES {
        assert!(retention.contains(&mode), "missing retention mode {mode:?}");
    }
    let redaction = packet.present_redaction_profiles();
    for profile in ALL_REDACTION_PROFILES {
        assert!(
            redaction.contains(&profile),
            "missing redaction profile {profile:?}"
        );
    }
    let data = packet.present_data_classes();
    for class in QueryDataClass::ALL {
        assert!(data.contains(&class), "missing data class {class:?}");
    }
}

#[test]
fn saved_queries_survive_reopen_migration_and_scope_drift() {
    // Acceptance: saved queries and history survive reopen, migration, and scope
    // drift without silent semantic breakage.
    let packet = seeded_saved_query_governance_packet();
    for row in &packet.saved_queries {
        assert!(row.survives_reopen);
        assert!(row.survives_migration);
        assert!(row.survives_scope_drift);
        assert!(!row.scope_drift.silent_semantic_break);
        assert_eq!(
            row.history_entry.saved_query_id_ref.as_deref(),
            Some(row.saved_query.saved_query_id.as_str())
        );
        assert_eq!(
            row.saved_query.scope_binding_id_ref,
            row.scope_pack.scope_binding_id
        );
    }

    // At least one row proves migration survival and at least one proves a
    // disclosed scope drift that requires a rerun.
    assert!(packet
        .saved_queries
        .iter()
        .any(|row| row.saved_query.migration_state
            == crate::query_artifacts::SearchArtifactMigrationState::MigratedFromPreviousVersion));
    let drifted = packet
        .saved_queries
        .iter()
        .find(|row| !row.scope_drift.scope_still_current())
        .expect("a drifted row");
    assert!(drifted.scope_drift.rerun_required);
    assert_eq!(
        drifted.scope_drift.result_semantics,
        SearchResultSemantics::ScopeChangedSinceCapture
    );
    assert_ne!(
        drifted.scope_drift.captured_stable_scope_id,
        drifted.scope_drift.current_stable_scope_id
    );
}

#[test]
fn raw_query_text_is_confined_to_a_single_local_only_row() {
    // Acceptance: raw query text is not synced, exported, or retained beyond
    // policy by default.
    let packet = seeded_saved_query_governance_packet();
    assert!(packet.raw_query_text_is_local_only());
    let literal_rows: Vec<_> = packet
        .saved_queries
        .iter()
        .filter(|row| row.saved_query.query_text.is_some())
        .collect();
    assert_eq!(
        literal_rows.len(),
        1,
        "only the local-only row keeps a literal"
    );
    let row = literal_rows[0];
    assert_eq!(
        row.saved_query.privacy_class,
        SavedQueryPrivacyClass::LocalOnlyPrivate
    );
    assert_eq!(row.saved_query.sync_class, SearchSyncClass::LocalOnly);
}

#[test]
fn retention_matrix_keeps_raw_query_text_local_only() {
    let packet = seeded_saved_query_governance_packet();
    let raw = packet
        .retention_row(QueryDataClass::RawQueryText)
        .expect("raw query text row");
    assert!(!raw.synced_by_default);
    assert_eq!(
        raw.local_retention_mode,
        SearchRetentionMode::LocalOnlyDefault
    );
    assert_eq!(raw.local_sync_class, SearchSyncClass::LocalOnly);
    assert_eq!(
        raw.default_redaction,
        SearchRedactionProfile::LiteralLocalOnly
    );
    // Raw text may only leave the device under explicit consent.
    assert_eq!(
        raw.on_sync_redaction,
        SearchRedactionProfile::ExplicitLiteralConsent
    );
    assert_eq!(
        raw.widening_basis,
        SearchRetentionWideningBasis::ExplicitUserOptIn
    );
}

#[test]
fn signed_deep_links_verify_disclose_and_preserve_return_path() {
    // Acceptance: search deep links disclose scope, freshness, and partiality and
    // preserve a supportable return path.
    let packet = seeded_saved_query_governance_packet();
    assert!(!packet.signed_deep_links.is_empty());
    let mut schemes = HashSet::new();
    for link in &packet.signed_deep_links {
        assert!(
            link.signature_verifies(),
            "{} did not verify",
            link.signed_link_id
        );
        assert!(!link.implies_live_current_certainty);
        assert!(link.freshness_is_intent_not_certainty());
        assert!(!link.return_anchor_ref.trim().is_empty());
        assert!(!link.completeness_note.trim().is_empty());
        assert!(link.partiality_disclosed);
        assert!(!link.deep_link.access_widening_allowed);
        assert!(
            link.deep_link
                .recipient_re_resolves_under_current_permissions
        );
        schemes.insert(link.signature_scheme);
    }
    for scheme in DeepLinkSignatureScheme::ALL {
        assert!(schemes.contains(&scheme), "missing scheme {scheme:?}");
    }
}

#[test]
fn tampering_with_a_signed_deep_link_disclosure_is_detected() {
    let mut packet = seeded_saved_query_governance_packet();
    let link = &mut packet.signed_deep_links[0];
    assert!(link.signature_verifies());
    link.completeness_note = "Everything is fully current and complete.".to_string();
    assert!(
        !link.signature_verifies(),
        "tampering with the completeness note must break the signature"
    );
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("signature must verify")));
}

#[test]
fn consumers_reuse_artifacts_without_widening_authority() {
    let packet = seeded_saved_query_governance_packet();
    for required in GovernanceConsumerClass::ALL {
        let projection = packet
            .consumer_projections
            .iter()
            .find(|projection| projection.consumer == required)
            .unwrap_or_else(|| panic!("missing consumer {}", required.as_str()));
        assert_eq!(projection.ingested_packet_id, packet.packet_id);
        assert!(projection.preserves_privacy_and_sync_class);
        assert!(projection.preserves_captured_vs_current_scope);
        assert!(projection.reuses_same_artifacts);
        assert!(!projection.widens_authority);
        assert!(projection.raw_query_text_excluded);
        assert!(projection.ambient_authority_excluded);
    }
}

#[test]
fn checked_in_packet_matches_seeded_canonical() {
    let checked =
        current_saved_query_governance_packet().expect("checked-in packet parses and validates");
    assert_eq!(checked, seeded_saved_query_governance_packet());
}

#[test]
fn support_export_redacts_all_raw_query_text() {
    let packet = seeded_saved_query_governance_packet();
    assert!(!packet.contains_no_raw_query_text());
    let export = packet.support_export("saved-query-export-1", "2026-06-17T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.redacted_packet.contains_no_raw_query_text());
    assert!(export.redacted_packet.validate().is_empty());
    // The redacted copy keeps everything else, including hashes and scope truth.
    assert_eq!(
        export.redacted_packet.saved_queries.len(),
        packet.saved_queries.len()
    );
    assert_eq!(export.redacted_packet, seeded_redacted_export_packet());
}

#[test]
fn redact_for_export_keeps_hashes_and_scope_metadata() {
    let packet = seeded_saved_query_governance_packet();
    let redacted = packet.redact_for_export();
    let row = redacted
        .saved_query_row("saved-query:local-private")
        .expect("local-private row");
    assert!(row.saved_query.query_text.is_none());
    assert!(row.saved_query.query_hash.is_some());
    assert_eq!(row.saved_query.query_text_mode, QueryTextMode::HashOnly);
}

#[test]
fn detects_synced_raw_query_text() {
    let mut packet = seeded_saved_query_governance_packet();
    packet
        .retention_matrix
        .iter_mut()
        .find(|row| row.data_class == QueryDataClass::RawQueryText)
        .unwrap()
        .synced_by_default = true;
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("never sync by default")));
}

#[test]
fn detects_silent_scope_break() {
    let mut packet = seeded_saved_query_governance_packet();
    packet.saved_queries[0].scope_drift.silent_semantic_break = true;
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("silent semantic break")));
}

#[test]
fn detects_deep_link_that_widens_access() {
    let mut packet = seeded_saved_query_governance_packet();
    packet.signed_deep_links[0]
        .deep_link
        .access_widening_allowed = true;
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("re-resolve")
            || finding.message.contains("widen access")));
}

#[test]
fn detects_missing_privacy_coverage() {
    // Dropping every artifact that realizes a privacy class must be caught, and
    // the declared coverage list must stay complete.
    let mut packet = seeded_saved_query_governance_packet();
    packet.covered_privacy_classes.pop();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("every privacy class")));

    // Removing the artifacts that realize a class is caught by the realization
    // check.
    let mut packet = seeded_saved_query_governance_packet();
    packet.saved_queries.retain(|row| {
        row.saved_query.privacy_class != SavedQueryPrivacyClass::SupportExportRedacted
    });
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("realizes privacy class")));
}

#[test]
fn detects_deep_link_claiming_live_certainty() {
    let mut packet = seeded_saved_query_governance_packet();
    packet.signed_deep_links[0].implies_live_current_certainty = true;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("must not imply live current certainty")));
}
