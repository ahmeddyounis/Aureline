//! Unit tests for the token-overlay portability audit.

use super::*;

#[test]
fn seeded_audit_is_clean_and_validates() {
    let report = seeded_token_overlay_portability();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    assert!(report.blocking_findings.is_empty());
    validate_token_overlay_portability(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_envelope_is_stable() {
    let report = seeded_token_overlay_portability();
    assert_eq!(report.record_kind, TOKEN_OVERLAY_REPORT_RECORD_KIND);
    assert_eq!(report.schema_version, TOKEN_OVERLAY_SCHEMA_VERSION);
    assert_eq!(
        report.shared_contract_ref,
        TOKEN_OVERLAY_SHARED_CONTRACT_REF
    );
    assert_eq!(report.report_id, TOKEN_OVERLAY_REPORT_ID);
    assert_eq!(report.source_schema_ref, TOKEN_OVERLAY_SOURCE_SCHEMA_REF);
    assert_eq!(
        report.canonical_overlay_schema_ref,
        TOKEN_OVERLAY_CANONICAL_RECORD_SCHEMA_REF
    );
    assert_eq!(
        report.appearance_session_ref,
        TOKEN_OVERLAY_APPEARANCE_SESSION_REF
    );
    assert_eq!(
        report.published_report_ref,
        TOKEN_OVERLAY_PUBLISHED_REPORT_REF
    );
    assert_eq!(report.published_doc_ref, TOKEN_OVERLAY_PUBLISHED_DOC_REF);
}

#[test]
fn seeded_audit_is_deterministic() {
    assert_eq!(
        seeded_token_overlay_portability(),
        seeded_token_overlay_portability()
    );
}

#[test]
fn seeded_audit_covers_every_override_scope() {
    let report = seeded_token_overlay_portability();
    let scopes: std::collections::BTreeSet<_> = report
        .overlays
        .iter()
        .map(|overlay| overlay.scope)
        .collect();
    for scope in [
        OverrideScope::ThemePackageDefault,
        OverrideScope::ImportedTheme,
        OverrideScope::ExtensionContributed,
        OverrideScope::UserGlobal,
        OverrideScope::Profile,
        OverrideScope::Workspace,
        OverrideScope::PolicyManaged,
    ] {
        assert!(scopes.contains(&scope), "missing scope {}", scope.as_str());
    }
    assert_eq!(report.scope_covered_count, 7);
}

#[test]
fn seeded_audit_exercises_every_value_state() {
    let report = seeded_token_overlay_portability();
    let states: std::collections::BTreeSet<_> = report
        .all_entries()
        .map(|entry| entry.value_state)
        .collect();
    for state in [
        ValueState::Inherited,
        ValueState::Overridden,
        ValueState::Deprecated,
        ValueState::Unmapped,
    ] {
        assert!(
            states.contains(&state),
            "missing value state {}",
            state.as_str()
        );
    }
}

#[test]
fn collections_are_sorted() {
    let report = seeded_token_overlay_portability();

    let overlay_keys: Vec<_> = report
        .overlays
        .iter()
        .map(|overlay| (overlay.scope.precedence_rank(), overlay.overlay_id.clone()))
        .collect();
    let mut sorted = overlay_keys.clone();
    sorted.sort();
    assert_eq!(overlay_keys, sorted);

    for overlay in &report.overlays {
        let ids: Vec<_> = overlay.entries.iter().map(|e| e.entry_id.clone()).collect();
        let mut sorted_ids = ids.clone();
        sorted_ids.sort();
        assert_eq!(ids, sorted_ids);
    }

    let token_refs: Vec<_> = report
        .resolved_tokens
        .iter()
        .map(|r| r.token_ref.clone())
        .collect();
    let mut sorted_tokens = token_refs.clone();
    sorted_tokens.sort();
    assert_eq!(token_refs, sorted_tokens);

    let seq: Vec<_> = report
        .round_trip
        .stages
        .iter()
        .map(|s| s.sequence_index)
        .collect();
    let mut sorted_seq = seq.clone();
    sorted_seq.sort();
    assert_eq!(seq, sorted_seq);
}

#[test]
fn winning_scope_is_highest_precedence() {
    let report = seeded_token_overlay_portability();
    for resolved in &report.resolved_tokens {
        let max_rank = report
            .all_entries()
            .filter(|entry| entry.token_ref == resolved.token_ref)
            .map(|entry| entry.declared_scope.precedence_rank())
            .max()
            .expect("each resolved token must have a contributing entry");
        assert_eq!(
            resolved.winning_scope.precedence_rank(),
            max_rank,
            "wrong winner for {}",
            resolved.token_ref
        );
    }
}

#[test]
fn round_trip_is_lossless_and_preserves_unsupported() {
    let report = seeded_token_overlay_portability();
    assert!(report.round_trip_lossless);
    assert!(report.round_trip.lossless);
    for stage in &report.round_trip.stages {
        assert_eq!(stage.dropped_count, 0);
        assert_eq!(stage.rewritten_count, 0);
        assert_eq!(
            stage.preserved_count + stage.downgraded_count,
            stage.input_entry_count
        );
    }
    // The imported-theme chart slot and the deprecated extension alias survive
    // as disclosed downgrades rather than being dropped.
    assert_eq!(report.round_trip.unsupported_preserved_count, 2);
    for trace in &report.round_trip.entry_traces {
        assert!(trace.survived);
        assert_eq!(trace.origin_scope, trace.final_scope);
        if trace.disposition == EntryDisposition::Downgraded {
            assert!(trace.downgrade_class.is_downgrade());
        }
    }
}

#[test]
fn unmapped_entry_is_inert_and_disclosed() {
    let report = seeded_token_overlay_portability();
    let unmapped: Vec<_> = report
        .all_entries()
        .filter(|entry| entry.value_state == ValueState::Unmapped)
        .collect();
    assert_eq!(unmapped.len(), 1);
    let entry = unmapped[0];
    assert_eq!(
        entry.validation_state,
        OverlayValidationState::InertUnresolved
    );
    assert_eq!(entry.downgrade_class, DowngradeClass::InertUnsupportedToken);
    assert!(entry.unmapped_source_slot_ref.is_some());
}

#[test]
fn deprecated_entry_cites_replacement() {
    let report = seeded_token_overlay_portability();
    let deprecated: Vec<_> = report
        .all_entries()
        .filter(|entry| entry.value_state == ValueState::Deprecated)
        .collect();
    assert_eq!(deprecated.len(), 1);
    assert!(deprecated[0].deprecated_replacement_ref.is_some());
}

#[test]
fn support_export_quotes_every_object() {
    let report = seeded_token_overlay_portability();
    let export =
        TokenOverlaySupportExport::from_report(TOKEN_OVERLAY_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.appearance_session_ref));
    for overlay in &report.overlays {
        assert!(export.case_ids.contains(&overlay.overlay_id));
        for entry in &overlay.entries {
            assert!(export.case_ids.contains(&entry.entry_id));
        }
    }
    for resolved in &report.resolved_tokens {
        assert!(export.case_ids.contains(&resolved.token_ref));
    }
    assert!(export.case_ids.contains(&report.round_trip.proof_id));
}

#[test]
fn unsupported_treated_as_supported_is_blocking() {
    let mut entry = seed_entry(
        "color.chart.series_9",
        TokenFamily::ColorChart,
        OverrideScope::ImportedTheme,
        ValueState::Unmapped,
        OverlayValidationState::InertUnresolved,
        ProvenanceClass::ImportedFromThemePackage,
        PortabilityFlags {
            portability_class: PortabilityClass::FullyPortable,
            exportable: true,
            syncable: true,
            survives_unsupported_target: true,
        },
        DowngradeClass::None,
        None,
        Some("imported-slot:chart.series_9"),
        "Unsupported slot wrongly treated as fully supported.",
    );
    entry.blocking_findings = compute_entry_findings(&entry);
    let tokens: Vec<_> = entry
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains(&"entry_unsupported_treated_as_supported"));
    assert!(tokens.contains(&"entry_portability_inconsistent"));
}

#[test]
fn dropped_round_trip_entry_is_blocking() {
    let trace = RoundTripEntryTrace {
        record_kind: TOKEN_OVERLAY_ROUND_TRIP_TRACE_RECORD_KIND.to_owned(),
        entry_ref: "entry:user_global:color.accent.primary".to_owned(),
        token_ref: "color.accent.primary".to_owned(),
        origin_scope: OverrideScope::UserGlobal,
        final_scope: OverrideScope::UserGlobal,
        disposition: EntryDisposition::Dropped,
        downgrade_class: DowngradeClass::None,
        channels_traversed: vec![RoundTripChannel::ExportBundle],
        survived: false,
        explanation: "Silently dropped on export.".to_owned(),
        blocking_findings: Vec::new(),
    };
    let findings = compute_trace_findings(&trace);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "round_trip_entry_dropped"));
}

#[test]
fn scope_lost_round_trip_entry_is_blocking() {
    let trace = RoundTripEntryTrace {
        record_kind: TOKEN_OVERLAY_ROUND_TRIP_TRACE_RECORD_KIND.to_owned(),
        entry_ref: "entry:workspace:color.accent.primary".to_owned(),
        token_ref: "color.accent.primary".to_owned(),
        origin_scope: OverrideScope::Workspace,
        final_scope: OverrideScope::UserGlobal,
        disposition: EntryDisposition::Preserved,
        downgrade_class: DowngradeClass::None,
        channels_traversed: vec![RoundTripChannel::ExportBundle],
        survived: true,
        explanation: "Scope flattened on import.".to_owned(),
        blocking_findings: Vec::new(),
    };
    let findings = compute_trace_findings(&trace);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "round_trip_scope_lost"));
}

#[test]
fn flattened_overlay_is_blocking() {
    let mut overlay = seed_overlay(
        OverrideScope::UserGlobal,
        OverlayValidationState::Valid,
        vec![seed_entry(
            "color.accent.primary",
            TokenFamily::ColorFunctionalAccent,
            OverrideScope::UserGlobal,
            ValueState::Overridden,
            OverlayValidationState::Valid,
            ProvenanceClass::AuthoredInProduct,
            PortabilityFlags {
                portability_class: PortabilityClass::FullyPortable,
                exportable: true,
                syncable: true,
                survives_unsupported_target: true,
            },
            DowngradeClass::None,
            None,
            None,
            "User-global accent override.",
        )],
    );
    overlay.structured = false;
    let findings = compute_overlay_findings(&overlay);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "overlay_flattened_to_opaque_blob"));
}

#[test]
fn round_trips_through_serde() {
    let report = seeded_token_overlay_portability();
    let json = serde_json::to_string(&report).expect("serialize");
    let parsed: TokenOverlayPortabilityReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(report, parsed);
}
