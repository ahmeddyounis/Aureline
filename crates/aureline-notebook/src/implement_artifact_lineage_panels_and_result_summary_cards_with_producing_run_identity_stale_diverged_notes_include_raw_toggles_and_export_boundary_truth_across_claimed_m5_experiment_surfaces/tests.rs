use super::*;

const PACKET_ID: &str = ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_PACKET_ID;

fn packet() -> ArtifactLineagePanelResultSummaryCardControlsPacket {
    seeded_artifact_lineage_panel_result_summary_card_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_VERSION
    );
}

#[test]
fn traceability_is_derived_not_asserted() {
    use ArtifactTraceabilityClass as Trace;
    use M5LineageState as State;

    // Complete → fully traced; regenerated → regenerated (both fully traced).
    for (state, class) in [
        (State::LineageComplete, Trace::FullyTraced),
        (State::Regenerated, Trace::Regenerated),
    ] {
        let d = resolve_artifact_lineage(state);
        assert_eq!(d.traceability_class, class);
        assert!(d.is_fully_traced);
    }

    // Regenerated needs a regenerated note.
    let d = resolve_artifact_lineage(State::Regenerated);
    assert!(d.needs_regenerated_note);

    // Partial / derived-upstream-known → partially traced, needs partial note.
    for state in [State::LineagePartial, State::DerivedUpstreamKnown] {
        let d = resolve_artifact_lineage(state);
        assert_eq!(d.traceability_class, Trace::PartiallyTraced);
        assert!(!d.is_fully_traced);
        assert!(d.needs_partial_lineage_note);
    }

    // Broken → untraced, needs stale / diverged note.
    let d = resolve_artifact_lineage(State::LineageBroken);
    assert_eq!(d.traceability_class, Trace::Untraced);
    assert!(!d.is_fully_traced);
    assert!(d.needs_stale_or_diverged_note);

    // Derived-upstream-unknown → untraced, needs unknown-upstream note.
    let d = resolve_artifact_lineage(State::DerivedUpstreamUnknown);
    assert_eq!(d.traceability_class, Trace::Untraced);
    assert!(!d.is_fully_traced);
    assert!(d.needs_unknown_upstream_note);
}

#[test]
fn export_disposition_is_derived_not_asserted() {
    use M5SummaryExportScope as Scope;
    use SummaryExportDisposition as Disposition;

    // Summary / metadata scope → metadata-safe (metadata-only, no raw).
    for scope in [Scope::SummaryScope, Scope::MetadataScope] {
        let d = resolve_summary_export(scope);
        assert_eq!(d.export_disposition, Disposition::MetadataSafe);
        assert!(d.is_metadata_only);
        assert!(!d.includes_raw_payload);
    }

    // Evidence scope → evidence-scoped.
    let d = resolve_summary_export(Scope::EvidenceScope);
    assert_eq!(d.export_disposition, Disposition::EvidenceScoped);

    // Raw scope → raw-included, needs raw-inclusion warning.
    let d = resolve_summary_export(Scope::RawScope);
    assert_eq!(d.export_disposition, Disposition::RawIncluded);
    assert!(d.includes_raw_payload);
    assert!(d.needs_raw_inclusion_warning);
    assert!(!d.is_metadata_only);

    // Redacted scope → redacted, needs redaction note.
    let d = resolve_summary_export(Scope::RedactedScope);
    assert_eq!(d.export_disposition, Disposition::Redacted);
    assert!(d.needs_redaction_note);

    // Withheld → withheld, needs withheld note.
    let d = resolve_summary_export(Scope::ExportWithheld);
    assert_eq!(d.export_disposition, Disposition::Withheld);
    assert!(d.is_withheld);
    assert!(d.needs_withheld_note);
}

#[test]
fn lineage_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .lineage_panels
        .iter()
        .map(|p| p.lineage_disclosure().traceability_class)
        .collect();
    for class in ArtifactTraceabilityClass::ALL {
        assert!(
            classes.contains(&class),
            "missing traceability class {class:?}"
        );
    }
    let kinds: std::collections::BTreeSet<_> = packet
        .lineage_panels
        .iter()
        .map(|p| p.artifact_kind)
        .collect();
    for kind in M5ArtifactKindClass::ALL {
        assert!(kinds.contains(&kind), "missing artifact kind {kind:?}");
    }
    let states: std::collections::BTreeSet<_> = packet
        .lineage_panels
        .iter()
        .map(|p| p.lineage_state)
        .collect();
    for state in M5LineageState::ALL {
        assert!(states.contains(&state), "missing lineage state {state:?}");
    }
}

#[test]
fn summary_coverage_is_complete() {
    let packet = packet();
    let dispositions: std::collections::BTreeSet<_> = packet
        .summary_cards
        .iter()
        .map(|c| c.export_disclosure().export_disposition)
        .collect();
    for class in SummaryExportDisposition::ALL {
        assert!(
            dispositions.contains(&class),
            "missing export disposition {class:?}"
        );
    }
    let contents: std::collections::BTreeSet<_> = packet
        .summary_cards
        .iter()
        .map(|c| c.summary_content_class)
        .collect();
    for content in M5SummaryContentClass::ALL {
        assert!(contents.contains(&content), "missing content {content:?}");
    }
    let scopes: std::collections::BTreeSet<_> = packet
        .summary_cards
        .iter()
        .map(|c| c.export_scope_state)
        .collect();
    for scope in M5SummaryExportScope::ALL {
        assert!(scopes.contains(&scope), "missing scope {scope:?}");
    }
}

#[test]
fn every_panel_names_a_producing_run() {
    for panel in packet().lineage_panels {
        assert!(
            !panel.producing_run_id.trim().is_empty(),
            "panel {} has no producing run",
            panel.panel_id
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::MissingSourceContracts));
}

#[test]
fn empty_lineage_panels_fails() {
    let mut packet = packet();
    packet.lineage_panels.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::LineagePanelsMissing));
}

#[test]
fn empty_summary_cards_fails() {
    let mut packet = packet();
    packet.summary_cards.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::SummaryCardsMissing));
}

#[test]
fn lineage_panel_wrong_component_class_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].component = M5ExperimentComponentFamily::ResultSummaryCard;
    assert!(packet.validate().contains(
        &ArtifactLineagePanelResultSummaryCardViolation::LineagePanelWrongComponentClass
    ));
}

#[test]
fn summary_card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.summary_cards[0].component = M5ExperimentComponentFamily::ArtifactLineagePanel;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::SummaryCardWrongComponentClass));
}

#[test]
fn untraced_artifact_claiming_traced_fails() {
    let mut packet = packet();
    let panel = packet
        .lineage_panels
        .iter_mut()
        .find(|p| p.traceability_class == ArtifactTraceabilityClass::Untraced)
        .expect("untraced artifact present");
    panel.claims_fully_traced = true;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::TraceabilityMisrepresented));
}

#[test]
fn panel_without_producing_run_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].producing_run_id.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::ProducingRunMissing));
}

#[test]
fn missing_stale_or_diverged_note_fails() {
    let mut packet = packet();
    let panel = packet
        .lineage_panels
        .iter_mut()
        .find(|p| p.lineage_state == M5LineageState::LineageBroken)
        .expect("broken lineage present");
    panel.stale_or_diverged_note.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::StaleOrDivergedNoteMissing));
}

#[test]
fn missing_generator_step_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].generator_step_note.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::GeneratorStepMissing));
}

#[test]
fn raw_export_claiming_metadata_only_fails() {
    let mut packet = packet();
    let card = packet
        .summary_cards
        .iter_mut()
        .find(|c| c.export_disposition == SummaryExportDisposition::RawIncluded)
        .expect("raw export present");
    card.claims_metadata_only = true;
    assert!(packet.validate().contains(
        &ArtifactLineagePanelResultSummaryCardViolation::ExportDispositionMisrepresented
    ));
}

#[test]
fn metadata_card_with_raw_toggle_on_fails() {
    let mut packet = packet();
    let card = packet
        .summary_cards
        .iter_mut()
        .find(|c| c.export_disposition == SummaryExportDisposition::MetadataSafe)
        .expect("metadata-safe export present");
    // Turning the include-raw toggle on without the scope being raw is a misrepresentation:
    // raw inclusion must never be an accidental default.
    card.include_raw_toggle_on = true;
    assert!(packet.validate().contains(
        &ArtifactLineagePanelResultSummaryCardViolation::ExportDispositionMisrepresented
    ));
}

#[test]
fn missing_raw_inclusion_warning_fails() {
    let mut packet = packet();
    let card = packet
        .summary_cards
        .iter_mut()
        .find(|c| c.export_disposition == SummaryExportDisposition::RawIncluded)
        .expect("raw export present");
    card.raw_inclusion_warning.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::RawInclusionWarningMissing));
}

#[test]
fn missing_withheld_note_fails() {
    let mut packet = packet();
    let card = packet
        .summary_cards
        .iter_mut()
        .find(|c| c.export_disposition == SummaryExportDisposition::Withheld)
        .expect("withheld export present");
    card.withheld_note.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::WithheldNoteMissing));
}

#[test]
fn missing_provenance_note_fails() {
    let mut packet = packet();
    packet.summary_cards[0].provenance_note.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::ProvenanceNoteMissing));
}

#[test]
fn missing_summary_handoff_note_fails() {
    let mut packet = packet();
    packet.summary_cards[0]
        .summary_evidence_raw_handoff_note
        .clear();
    assert!(packet.validate().contains(
        &ArtifactLineagePanelResultSummaryCardViolation::SummaryEvidenceRawHandoffNoteMissing
    ));
}

#[test]
fn lineage_panel_missing_trace_action_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].panel_actions = vec![ArtifactLineageAction::OpenArtifact];
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::LineagePanelActionsIncomplete));
}

#[test]
fn summary_card_missing_summary_only_action_fails() {
    let mut packet = packet();
    packet.summary_cards[0].card_actions = vec![SummaryCardAction::ReviewExportScope];
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::SummaryCardActionsIncomplete));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.lineage_panels[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.summary_cards[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::DispositionsMissing));
}

#[test]
fn panel_masking_provenance_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].masks_provenance_or_sensitivity_state = true;
    assert!(packet.validate().contains(
        &ArtifactLineagePanelResultSummaryCardViolation::ProvenanceOrSensitivityStateMasked
    ));
}

#[test]
fn panel_hiding_producing_run_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].hides_producing_run_or_lineage_state = true;
    assert!(packet.validate().contains(
        &ArtifactLineagePanelResultSummaryCardViolation::ProducingRunOrLineageStateHidden
    ));
}

#[test]
fn card_exposing_raw_by_default_fails() {
    let mut packet = packet();
    packet.summary_cards[0].exposes_raw_payload_by_default = true;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::RawPayloadExposedByDefault));
}

#[test]
fn card_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.summary_cards[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::AlternateStateLabelInvented));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].required_labels = vec![M5ExperimentRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.summary_cards[0].accessibility_routes =
        vec![M5ExperimentAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::AccessibilityRouteMissing));
}

#[test]
fn artifact_review_incomplete_fails() {
    let mut packet = packet();
    packet.artifact_review.raw_payload_never_included_by_default = false;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::ArtifactReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .producing_run_and_lineage_visible_before_trust = false;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.lineage_panels[0].deep_link_ref = "see https://internal.example/artifact".to_owned();
    assert!(packet
        .validate()
        .contains(&ArtifactLineagePanelResultSummaryCardViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Artifact lineage panels"));
    assert!(summary.contains("## Result summary cards"));
    assert!(summary.contains("untraced"));
    assert!(summary.contains("raw payload"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 lineage panels + 6 summary cards
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("artifact_lineage_panel"));
    assert!(csv.contains("result_summary_card"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_artifact_lineage_panel_result_summary_card_export()
        .expect("checked artifact lineage summary card export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-artifact-lineage-panel-result-summary-card-controls/lineage_panel_broken.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-artifact-lineage-panel-result-summary-card-controls/summary_card_raw_payload.json"
        )),
    ] {
        let packet: ArtifactLineagePanelResultSummaryCardControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as artifact lineage summary card packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_artifact_lineage_panel_result_summary_card_controls_lineage_panel_broken(),
        seeded_artifact_lineage_panel_result_summary_card_controls_summary_card_raw_payload(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
