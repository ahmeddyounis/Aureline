use super::*;

fn packet() -> DocsEvidenceHandoffPacket {
    DocsEvidenceHandoffPacket::materialize(seeded_stable_docs_evidence_handoff_input())
}

#[test]
fn seeded_handoff_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.handoff_findings
    );
    assert_eq!(packet.record_kind, DOCS_EVIDENCE_HANDOFF_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_EVIDENCE_HANDOFF_SCHEMA_VERSION);
}

#[test]
fn handoff_covers_all_concrete_evidence_kinds() {
    let kinds: BTreeSet<EvidenceKind> = packet()
        .entries
        .iter()
        .flat_map(|e| e.bindings.iter().map(|b| b.evidence_kind))
        .collect();
    for required in EvidenceKind::REQUIRED_COVERAGE {
        assert!(
            kinds.contains(&required),
            "seed is missing evidence kind {required:?}"
        );
    }
    // The human-note kind is also demonstrated so the taxonomy is complete.
    assert!(kinds.contains(&EvidenceKind::HumanNote));
}

#[test]
fn every_entry_is_concretely_traced_and_reopenable() {
    for entry in packet().entries {
        assert!(entry.change.names_concrete_change());
        assert!(!entry.detail.trim().is_empty());
        assert!(!entry.bindings.is_empty());
        // Every entry binds to at least one concrete typed evidence object.
        assert!(entry.is_concretely_traced());
        // Support and review can reopen the same packet.
        assert!(entry.reopen.is_reopenable());
        for binding in &entry.bindings {
            assert!(binding.names_concrete_target());
            assert!(!binding.open_evidence_ref.trim().is_empty());
            assert!(!binding.provenance_disclosure_note.trim().is_empty());
            // A mirror/offline binding never claims authoritative live freshness.
            if binding.mirror_offline.is_mirror_or_offline() {
                assert!(!binding.freshness.is_authoritative_live());
            }
            // A non-first-party binding stays cited and is not presented as live.
            if binding.provenance.needs_citation() {
                assert!(binding.cited);
                assert!(!binding.freshness.is_authoritative_live());
            }
            // Touch each token so it stays stable across refactors.
            let _ = (
                binding.evidence_kind.as_str(),
                binding.scope.as_str(),
                binding.redaction_state.as_str(),
                binding.provenance.as_str(),
                binding.freshness.as_str(),
                binding.version_match.as_str(),
                binding.locality.as_str(),
                binding.mirror_offline.as_str(),
            );
        }
        let _ = (
            entry.change.change_kind.as_str(),
            entry.entry_scope.as_str(),
        );
    }
}

#[test]
fn seed_demonstrates_local_only_and_export_safe_scopes() {
    let packet = packet();
    assert!(packet.entries.iter().any(|e| e.is_export_safe()));
    assert!(packet
        .entries
        .iter()
        .any(|e| e.entry_scope == EvidenceScope::LocalOnly));
    // The local-only entry keeps its bindings local and never marks them export-safe.
    let local = packet
        .entries
        .iter()
        .find(|e| e.entry_scope == EvidenceScope::LocalOnly)
        .expect("a local-only entry is present");
    assert!(local.bindings.iter().all(|b| b.scope.is_local_only()));
    assert!(!local.is_export_safe());
}

#[test]
fn seed_demonstrates_originating_suggestion_link() {
    let packet = packet();
    assert!(packet
        .entries
        .iter()
        .any(|e| e.change.originating_suggestion_ref.is_some()));
}

#[test]
fn empty_entries_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries.clear();
    input.export.rows.clear();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert_eq!(packet.promotion_state, HandoffPromotionState::BlocksStable);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::EntriesEmpty));
}

#[test]
fn missing_required_evidence_kind_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    // Drop every release-object binding, leaving the taxonomy incomplete.
    for entry in input.entries.iter_mut() {
        entry
            .bindings
            .retain(|b| b.evidence_kind != EvidenceKind::ReleaseObject);
    }
    // Keep export rows consistent with the trimmed entries.
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::RequiredEvidenceKindMissing));
}

#[test]
fn change_traced_by_human_note_alone_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:changelog:retry_backoff_release")
        .expect("changelog entry present");
    entry.bindings = vec![EvidenceBinding {
        binding_id: "binding:changelog:note_only".to_owned(),
        evidence_kind: EvidenceKind::HumanNote,
        target_ref: "note:maintainer:changelog-context".to_owned(),
        display_path: "Maintainer note → changelog".to_owned(),
        label: "a free-form changelog note".to_owned(),
        scope: EvidenceScope::ExportSafeShared,
        redaction_state: EvidenceRedactionState::MetadataSafe,
        provenance: EvidenceProvenance::FirstPartyVerified,
        freshness: EvidenceFreshness::WarmCached,
        version_match: EvidenceVersionMatch::ExactBuildMatch,
        locality: EvidenceLocality::Local,
        mirror_offline: MirrorOfflinePosture::OnlineLive,
        provenance_disclosure_note: "a note".to_owned(),
        open_evidence_ref: "open-note:maintainer:changelog-context".to_owned(),
        detail: "a free-form note with no concrete evidence".to_owned(),
        cited: true,
        citation_ref: None,
    }];
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ChangeNotConcretelyTraced));
}

#[test]
fn empty_bindings_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].bindings.clear();
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::BindingsEmpty));
}

#[test]
fn missing_change_subject_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].change.doc_ref = "  ".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ChangeSubjectMissing));
}

#[test]
fn empty_section_anchor_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].change.section_anchor = Some("   ".to_owned());
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ChangeSubjectMissing));
}

#[test]
fn binding_target_missing_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].bindings[0].target_ref = "  ".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::BindingTargetMissing));
}

#[test]
fn binding_open_evidence_missing_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].bindings[0].open_evidence_ref = "  ".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::BindingOpenEvidenceMissing));
}

#[test]
fn missing_provenance_disclosure_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].bindings[0].provenance_disclosure_note = "  ".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ProvenanceDisclosureMissing));
}

#[test]
fn local_only_redaction_with_shared_scope_is_inconsistent() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    // The export-safe README source-file binding is marked local-only-redaction.
    input.entries[0].bindings[0].redaction_state =
        EvidenceRedactionState::LocalOnlyRedactionRequired;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ScopeRedactionInconsistent));
}

#[test]
fn export_safe_with_non_export_redaction_is_inconsistent() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].bindings[0].redaction_state = EvidenceRedactionState::NotApplicable;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ScopeRedactionInconsistent));
}

#[test]
fn local_only_unverified_marked_export_safe_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    let binding = &mut input.entries[0].bindings[0];
    binding.provenance = EvidenceProvenance::LocalOnlyUnverified;
    binding.freshness = EvidenceFreshness::Unverified;
    binding.version_match = EvidenceVersionMatch::UnknownTargetBuild;
    binding.cited = true;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::LocalOnlyMarkedExportSafe));
}

#[test]
fn entry_scope_wider_than_bindings_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    // Mark the local-only help entry export-safe while its bindings stay local.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:help:offline_runbook_note")
        .expect("local-only help entry present");
    entry.entry_scope = EvidenceScope::ExportSafeShared;
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::EntryScopeWiderThanBindings));
}

#[test]
fn offline_binding_claiming_live_freshness_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    // The mirror-served imported binding claims authoritative live freshness.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:help:offline_runbook_note")
        .expect("local-only help entry present");
    let binding = entry
        .bindings
        .iter_mut()
        .find(|b| b.mirror_offline == MirrorOfflinePosture::MirrorServed)
        .expect("mirror-served binding present");
    binding.freshness = EvidenceFreshness::AuthoritativeLive;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::OfflineClaimsLiveFreshness));
}

#[test]
fn non_first_party_presented_as_live_collapses_evidence_truth() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:help:offline_runbook_note")
        .expect("local-only help entry present");
    let binding = entry
        .bindings
        .iter_mut()
        .find(|b| b.provenance == EvidenceProvenance::Imported)
        .expect("imported binding present");
    // Force online + live to isolate the provenance collapse from the offline one.
    binding.mirror_offline = MirrorOfflinePosture::OnlineLive;
    binding.freshness = EvidenceFreshness::AuthoritativeLive;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::EvidenceTruthCollapsed));
}

#[test]
fn drifted_version_presented_as_live_collapses_version_truth() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    // The README source binding is first-party live; drift its version.
    input.entries[0].bindings[0].version_match = EvidenceVersionMatch::IncompatibleDriftDetected;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::VersionTruthCollapsed));
}

#[test]
fn uncited_non_first_party_binding_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:help:offline_runbook_note")
        .expect("local-only help entry present");
    let binding = entry
        .bindings
        .iter_mut()
        .find(|b| b.provenance.needs_citation())
        .expect("non-first-party binding present");
    binding.cited = false;
    binding.citation_ref = None;
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::BindingNotCited));
}

#[test]
fn entry_not_reopenable_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].reopen.available_in_support = false;
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::EntryNotReopenable));
}

#[test]
fn duplicate_entry_id_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    let clone = input.entries[0].clone();
    input.entries.push(clone);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::DuplicateEntryId));
}

#[test]
fn duplicate_binding_id_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    let clone = input.entries[0].bindings[0].clone();
    input.entries[1].bindings.push(clone);
    rebuild_export(&mut input);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::DuplicateBindingId));
}

#[test]
fn export_dropping_redaction_preservation_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.preserves_redaction = false;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportDropsPreservation));
}

#[test]
fn export_scope_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].entry_scope = EvidenceScope::LocalOnly;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportScopeMismatch));
}

#[test]
fn export_change_kind_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].change_kind = DocsChangeKind::SuggestionProposal;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportChangeKindMismatch));
}

#[test]
fn export_export_safe_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].export_safe = false;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportExportSafeMismatch));
}

#[test]
fn export_binding_count_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].binding_count = 99;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportBindingCountMismatch));
}

#[test]
fn export_evidence_kinds_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].evidence_kinds = vec![EvidenceKind::HumanNote];
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportEvidenceKindsMismatch));
}

#[test]
fn export_cited_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].cited = false;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportCitedMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows.pop();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.export.rows[0].entry_id_ref = "entry:does-not-exist".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ExportEntryOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.handoff_degradations.push(HandoffDegradation {
        degradation_class: HandoffDegradationClass::ScopeNarrowedForExport,
        severity: HandoffFindingSeverity::Narrowing,
        summary: "the handoff was narrowed to the export-safe bindings before sharing".to_owned(),
        entry_id_ref: None,
        evidence_ref: None,
    });
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        HandoffPromotionState::NarrowedBelowStable
    );
    assert!(packet.handoff_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.handoff_degradations.push(HandoffDegradation {
        degradation_class: HandoffDegradationClass::QuarantinedSource,
        severity: HandoffFindingSeverity::Blocking,
        summary: "an evidence source is quarantined and must not back a docs claim".to_owned(),
        entry_id_ref: Some("entry:readme:config_example_fix".to_owned()),
        evidence_ref: None,
    });
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert_eq!(packet.promotion_state, HandoffPromotionState::BlocksStable);
}

#[test]
fn degradation_referencing_unknown_entry_is_orphan() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.handoff_degradations[0].entry_id_ref = Some("entry:does-not-exist".to_owned());
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_scope_drifts() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.consumer_projections[0].preserves_scope = false;
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input
        .consumer_projections
        .retain(|p| p.surface != HandoffConsumerSurface::AiExplanation);
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.entries[0].detail = "matched on bearer abc123 token in the source".to_owned();
    let packet = DocsEvidenceHandoffPacket::materialize(input);
    assert!(packet
        .handoff_findings
        .iter()
        .any(|f| f.finding_kind == HandoffFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_entries_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for entry in &packet.entries {
        assert!(summary.contains(&entry.entry_id));
    }
    assert!(summary.contains("Scope"));
    assert!(summary.contains("evidence"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-12T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsEvidenceHandoffSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        DOCS_EVIDENCE_HANDOFF_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_docs_evidence_handoff_export()
        .expect("checked docs-evidence-handoff export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:docs_evidence_handoff:retry_backoff_release"
    );
    assert_eq!(export.packet.promotion_state, HandoffPromotionState::Stable);
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/docs-evidence-handoff/mirror_offline_narrows.json"
            )),
            HandoffPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/docs-evidence-handoff/local_only_marked_export_safe_blocks_stable.json"
            )),
            HandoffPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/docs-evidence-handoff/untraced_change_blocks_stable.json"
            )),
            HandoffPromotionState::BlocksStable,
        ),
    ] {
        let fixture: DocsEvidenceHandoffFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = DocsEvidenceHandoffPacket::materialize(fixture.input);
        assert_eq!(
            packet.promotion_state, expected,
            "fixture `{}` expected {:?}, findings: {:?}",
            fixture.case_name, expected, packet.handoff_findings
        );
        for expected_kind in fixture.expect.expected_finding_kinds {
            assert!(
                packet
                    .handoff_findings
                    .iter()
                    .any(|f| f.finding_kind.as_str() == expected_kind),
                "fixture `{}` expected finding `{}`",
                fixture.case_name,
                expected_kind
            );
        }
    }
}

/// Rebuilds the export rows from the current entries so a structural mutation in a
/// test does not also trip the unrelated export-coverage checks.
fn rebuild_export(input: &mut DocsEvidenceHandoffPacketInput) {
    input.export.rows = input.entries.iter().map(export_row).collect();
}

#[derive(Debug, Deserialize)]
struct DocsEvidenceHandoffFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: DocsEvidenceHandoffPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
