use super::*;

const PACKET_ID: &str = "structure-compare-summary-controls:stable:0001";

const ARTIFACT_CONFIG: &str = "artifact:config/app.yaml";
const ARTIFACT_LOCKFILE: &str = "artifact:package/Cargo.lock";

fn structure_rows() -> Vec<StructureRow> {
    vec![
        StructureRow {
            component: M5ArtifactComponent::StructureRow,
            row_id: "row:config-tls".to_owned(),
            artifact_ref: ARTIFACT_CONFIG.to_owned(),
            object_path: "server.tls".to_owned(),
            object_category: StructuredObjectCategory::StructuredObject,
            change_kind: StructureChangeKind::Added,
            old_summary: String::new(),
            new_summary: "enabled: true (min_version: 1.3)".to_owned(),
            confidence_or_schema_note: "High confidence: matches the config schema".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            redaction_note: String::new(),
            raw_context_action: "Open raw YAML at server.tls".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "object_path".to_owned(),
                "change_kind".to_owned(),
                "new_summary".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned()
            ],
        },
        StructureRow {
            component: M5ArtifactComponent::StructureRow,
            row_id: "row:config-port".to_owned(),
            artifact_ref: ARTIFACT_CONFIG.to_owned(),
            object_path: "server.port".to_owned(),
            object_category: StructuredObjectCategory::StructuredObject,
            change_kind: StructureChangeKind::Modified,
            old_summary: "8080".to_owned(),
            new_summary: "9090".to_owned(),
            confidence_or_schema_note: "High confidence: scalar value change".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            redaction_note: String::new(),
            raw_context_action: "Open raw YAML at server.port".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "object_path".to_owned(),
                "change_kind".to_owned(),
                "old_summary".to_owned(),
                "new_summary".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned()
            ],
        },
        StructureRow {
            component: M5ArtifactComponent::StructureRow,
            row_id: "row:config-redacted".to_owned(),
            artifact_ref: ARTIFACT_CONFIG.to_owned(),
            object_path: "config.redacted_credential".to_owned(),
            object_category: StructuredObjectCategory::RedactedField,
            change_kind: StructureChangeKind::RedactedHidden,
            old_summary: String::new(),
            new_summary: String::new(),
            confidence_or_schema_note: "A value changed here; content is withheld".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::RedactedOrWithheld,
            redaction_note: "Field is redacted under the export posture; value stays hidden"
                .to_owned(),
            raw_context_action: "Redacted: raw value is not exportable here".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "object_path".to_owned(),
                "change_kind".to_owned(),
                "redaction_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned()
            ],
        },
        StructureRow {
            component: M5ArtifactComponent::StructureRow,
            row_id: "row:lock-removed".to_owned(),
            artifact_ref: ARTIFACT_LOCKFILE.to_owned(),
            object_path: "package[left-pad]".to_owned(),
            object_category: StructuredObjectCategory::PackageDelta,
            change_kind: StructureChangeKind::Removed,
            old_summary: "left-pad 1.3.0".to_owned(),
            new_summary: String::new(),
            confidence_or_schema_note: "High confidence: dependency dropped from the lockfile"
                .to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            redaction_note: String::new(),
            raw_context_action: "Open raw lockfile at package[left-pad]".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "object_path".to_owned(),
                "change_kind".to_owned(),
                "old_summary".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned()
            ],
        },
        StructureRow {
            component: M5ArtifactComponent::StructureRow,
            row_id: "row:lock-meta".to_owned(),
            artifact_ref: ARTIFACT_LOCKFILE.to_owned(),
            object_path: "package[serde].source".to_owned(),
            object_category: StructuredObjectCategory::MetadataField,
            change_kind: StructureChangeKind::MetadataOnly,
            old_summary: "registry+https://old.example".to_owned(),
            new_summary: "registry+https://new.example".to_owned(),
            confidence_or_schema_note: "Metadata-only: source registry moved, version unchanged"
                .to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredPartial,
            redaction_note: String::new(),
            raw_context_action: "Open raw lockfile at package[serde]".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "object_path".to_owned(),
                "change_kind".to_owned(),
                "old_summary".to_owned(),
                "new_summary".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn compare_summary_cards() -> Vec<CompareSummaryCard> {
    vec![
        CompareSummaryCard {
            component: M5ArtifactComponent::CompareSummaryCard,
            card_id: "card:config".to_owned(),
            artifact_ref: ARTIFACT_CONFIG.to_owned(),
            artifact_class_label: "application config (YAML)".to_owned(),
            change_counts: StructuredChangeCounts {
                added: 1,
                removed: 0,
                modified: 1,
                metadata_only: 0,
                redacted_hidden: 1,
                total_changed_objects: 3,
            },
            large_diff: false,
            risk_markers: vec![RiskMarkerNote {
                marker: CompareRiskMarker::RedactedContentPresent,
                severity: RiskSeverity::Caution,
                note: "One field is redacted; its value stays hidden in this compare".to_owned(),
            }],
            confidence_or_schema_note: "Structured compare against the config schema".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            compare_write_back_safety: "Compare-only: config is not written back from here"
                .to_owned(),
            raw_context_action: "Open the raw YAML diff".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "change_counts".to_owned(),
                "risk_markers".to_owned(),
                "raw_context_action".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF.to_owned()
            ],
        },
        CompareSummaryCard {
            component: M5ArtifactComponent::CompareSummaryCard,
            card_id: "card:lockfile".to_owned(),
            artifact_ref: ARTIFACT_LOCKFILE.to_owned(),
            artifact_class_label: "dependency lockfile".to_owned(),
            change_counts: StructuredChangeCounts {
                added: 40,
                removed: 12,
                modified: 8,
                metadata_only: 1,
                redacted_hidden: 0,
                total_changed_objects: 61,
            },
            large_diff: true,
            risk_markers: vec![RiskMarkerNote {
                marker: CompareRiskMarker::LargeChangeVolume,
                severity: RiskSeverity::Caution,
                note: "61 dependency objects changed; open the raw lockfile for the full list"
                    .to_owned(),
            }],
            confidence_or_schema_note:
                "Structured compare with partial coverage of source metadata".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredPartial,
            compare_write_back_safety: "Compare-only: the lockfile is regenerated, not hand-edited"
                .to_owned(),
            raw_context_action: "Open the raw lockfile diff".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "change_counts".to_owned(),
                "risk_markers".to_owned(),
                "raw_context_action".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn trust_review() -> StructureCompareControlsTrustReview {
    StructureCompareControlsTrustReview {
        object_identity_always_explicit: true,
        change_kind_distinct_per_object: true,
        old_new_summary_preserved: true,
        raw_context_always_reachable: true,
        scale_surfaced_without_flattening: true,
        risk_markers_explained: true,
        redacted_content_never_leaked: true,
        confidence_or_schema_note_explicit: true,
        compare_only_never_silently_writable: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> StructureCompareControlsConsumerProjection {
    StructureCompareControlsConsumerProjection {
        structure_row_shows_object_and_change_kind: true,
        compare_card_shows_counts_and_risk: true,
        raw_context_reachable_from_both: true,
        redacted_fields_shown_as_hidden_not_dropped: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        help_about_shows_truth: true,
    }
}

fn proof_freshness() -> StructureCompareControlsProofFreshness {
    StructureCompareControlsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<StructureCompareControlsDowngradeTrigger> {
    vec![
        StructureCompareControlsDowngradeTrigger::ProofStale,
        StructureCompareControlsDowngradeTrigger::SchemaUnrecognized,
        StructureCompareControlsDowngradeTrigger::LargeDiffTruncationRisk,
        StructureCompareControlsDowngradeTrigger::RedactionApplied,
        StructureCompareControlsDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<StructureCompareControlsConsumerSurface> {
    vec![
        StructureCompareControlsConsumerSurface::DiffCompareView,
        StructureCompareControlsConsumerSurface::MergeConflictWorkspace,
        StructureCompareControlsConsumerSurface::ArtifactBrowser,
        StructureCompareControlsConsumerSurface::CliHeadless,
        StructureCompareControlsConsumerSurface::SupportExport,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        STRUCTURE_COMPARE_CONTROLS_SCHEMA_REF.to_owned(),
        STRUCTURE_COMPARE_CONTROLS_DOC_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> StructureCompareControlsPacket {
    StructureCompareControlsPacket::new(StructureCompareControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Structure rows and compare summary cards".to_owned(),
        structure_rows: structure_rows(),
        compare_summary_cards: compare_summary_cards(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn structure_compare_controls_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn resolver_derives_summaries_from_change_kind() {
    let added = resolve_structure_row_disclosure(StructureChangeKind::Added);
    assert!(added.needs_new_summary);
    assert!(!added.needs_old_summary);

    let removed = resolve_structure_row_disclosure(StructureChangeKind::Removed);
    assert!(removed.needs_old_summary);
    assert!(!removed.needs_new_summary);

    let modified = resolve_structure_row_disclosure(StructureChangeKind::Modified);
    assert!(modified.needs_old_summary && modified.needs_new_summary);

    let redacted = resolve_structure_row_disclosure(StructureChangeKind::RedactedHidden);
    assert!(redacted.content_hidden);
    assert!(redacted.needs_redaction_note);
    assert!(!redacted.needs_old_summary && !redacted.needs_new_summary);
}

#[test]
fn added_row_missing_new_summary_fails() {
    let mut packet = packet();
    packet.structure_rows[0].new_summary = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ChangeSummaryMissing));
}

#[test]
fn removed_row_missing_old_summary_fails() {
    let mut packet = packet();
    packet.structure_rows[3].old_summary = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ChangeSummaryMissing));
}

#[test]
fn redacted_row_leaking_content_fails() {
    let mut packet = packet();
    packet.structure_rows[2].new_summary = "leaked value".to_owned();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::HiddenContentLeaked));
}

#[test]
fn redacted_row_without_note_fails() {
    let mut packet = packet();
    packet.structure_rows[2].redaction_note = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::RedactionNoteMissing));
}

#[test]
fn missing_object_identity_fails() {
    let mut packet = packet();
    packet.structure_rows[0].object_path = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ObjectIdentityMissing));
}

#[test]
fn missing_confidence_note_fails() {
    let mut packet = packet();
    packet.structure_rows[0].confidence_or_schema_note = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ConfidenceNoteMissing));
}

#[test]
fn missing_raw_context_action_row_fails() {
    let mut packet = packet();
    packet.structure_rows[0].raw_context_action = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::RawContextActionMissing));
}

#[test]
fn category_kind_inconsistent_fails() {
    let mut packet = packet();
    // A redacted field that no longer carries a redacted-hidden change kind.
    packet.structure_rows[2].change_kind = StructureChangeKind::Modified;
    packet.structure_rows[2].old_summary = "a".to_owned();
    packet.structure_rows[2].new_summary = "b".to_owned();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::CategoryChangeKindInconsistent));
}

#[test]
fn wrong_structure_component_class_fails() {
    let mut packet = packet();
    packet.structure_rows[0].component = M5ArtifactComponent::CompareSummaryCard;
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::StructureRowWrongComponentClass));
}

#[test]
fn missing_change_kind_coverage_fails() {
    let mut packet = packet();
    packet
        .structure_rows
        .retain(|row| row.change_kind != StructureChangeKind::Removed);
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::StructureChangeKindCoverageMissing));
}

#[test]
fn inconsistent_counts_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0]
        .change_counts
        .total_changed_objects = 99;
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ChangeCountsInconsistent));
}

#[test]
fn empty_compare_summary_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].change_counts = StructuredChangeCounts {
        added: 0,
        removed: 0,
        modified: 0,
        metadata_only: 0,
        redacted_hidden: 0,
        total_changed_objects: 0,
    };
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::EmptyCompareSummary));
}

#[test]
fn large_diff_without_scale_marker_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[1].risk_markers.clear();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ScaleRiskMarkerMissing));
}

#[test]
fn redacted_card_without_redacted_marker_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].risk_markers.clear();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::RedactedRiskMarkerMissing));
}

#[test]
fn risk_marker_without_note_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].risk_markers[0].note = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::RiskMarkerNoteMissing));
}

#[test]
fn missing_compare_write_back_safety_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].compare_write_back_safety = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::CompareWriteBackSafetyMissing));
}

#[test]
fn missing_card_confidence_note_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].confidence_or_schema_note = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::CompareSummaryConfidenceNoteMissing));
}

#[test]
fn missing_raw_context_action_card_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].raw_context_action = String::new();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::RawContextActionMissing));
}

#[test]
fn wrong_compare_component_class_fails() {
    let mut packet = packet();
    packet.compare_summary_cards[0].component = M5ArtifactComponent::StructureRow;
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::CompareSummaryCardWrongComponentClass));
}

#[test]
fn unpaired_artifact_fails() {
    let mut packet = packet();
    packet.compare_summary_cards.pop();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ComparePairingIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.redacted_content_never_leaked = false;
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.raw_context_reachable_from_both = false;
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&StructureCompareControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Structure rows"));
    assert!(summary.contains("## Compare summary cards"));
    assert!(summary.contains("dependency lockfile"));
    assert!(summary.contains("redacted_hidden"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_structure_compare_controls_export()
        .expect("checked structure compare controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structure-compare-summary-controls/schema_unrecognized_raw_fallback.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structure-compare-summary-controls/large_diff_scale_surfaced.json"
        )),
    ] {
        let packet: StructureCompareControlsPacket = serde_json::from_str(raw)
            .expect("fixture parses as structure compare controls packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_STRUCTURE_COMPARE_SUMMARY_ARTIFACTS` so ordinary test
/// runs never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_STRUCTURE_COMPARE_SUMMARY_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-structure-compare-summary-controls-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-structure-compare-summary-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: the config compare drops to a raw/export-safe fallback because
    // no schema recognizes it — scale and risk stay surfaced, raw stays reachable.
    let mut unrecognized = packet.clone();
    unrecognized.packet_id =
        "structure-compare-summary-controls:fixture:schema-unrecognized".to_owned();
    if let Some(card) = unrecognized
        .compare_summary_cards
        .iter_mut()
        .find(|card| card.artifact_ref == ARTIFACT_CONFIG)
    {
        card.schema_fidelity = M5ArtifactFidelityState::SchemaUnrecognized;
        card.confidence_or_schema_note =
            "No schema recognizes this config; compare falls back to the raw/export-safe view"
                .to_owned();
        card.risk_markers.push(RiskMarkerNote {
            marker: CompareRiskMarker::SchemaFidelityNarrowed,
            severity: RiskSeverity::Caution,
            note: "Schema fidelity narrowed to raw fallback; read the raw YAML for detail"
                .to_owned(),
        });
    }
    assert!(
        unrecognized.validate().is_empty(),
        "{:?}",
        unrecognized.validate()
    );
    std::fs::write(
        fixture_dir.join("schema_unrecognized_raw_fallback.json"),
        format!("{}\n", unrecognized.export_safe_json()),
    )
    .expect("write schema-unrecognized fixture");

    // Fixture 2: an even larger lockfile diff, critical scale marker, raw reachable.
    let mut large = packet.clone();
    large.packet_id = "structure-compare-summary-controls:fixture:large-diff".to_owned();
    if let Some(card) = large
        .compare_summary_cards
        .iter_mut()
        .find(|card| card.artifact_ref == ARTIFACT_LOCKFILE)
    {
        card.change_counts = StructuredChangeCounts {
            added: 210,
            removed: 45,
            modified: 33,
            metadata_only: 4,
            redacted_hidden: 0,
            total_changed_objects: 292,
        };
        card.large_diff = true;
        card.risk_markers = vec![RiskMarkerNote {
            marker: CompareRiskMarker::LargeChangeVolume,
            severity: RiskSeverity::Critical,
            note: "292 dependency objects changed; the raw lockfile diff stays fully inspectable"
                .to_owned(),
        }];
    }
    assert!(large.validate().is_empty(), "{:?}", large.validate());
    std::fs::write(
        fixture_dir.join("large_diff_scale_surfaced.json"),
        format!("{}\n", large.export_safe_json()),
    )
    .expect("write large-diff fixture");
}
