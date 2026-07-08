use super::*;

const PACKET_ID: &str = "structured-artifact-review-accessibility:stable:0001";

fn trust_review() -> ArtifactReviewAccessibilityTrustReview {
    ArtifactReviewAccessibilityTrustReview {
        keyboard_reachable_on_every_claim: true,
        screen_reader_labeled_on_every_claim: true,
        cli_enum_exposed_on_every_claim: true,
        export_enum_exposed_on_every_claim: true,
        explanation_field_present_on_every_claim: true,
        no_component_pointer_only: true,
        no_component_export_opaque: true,
        desktop_never_stronger_than_cli: true,
        claim_narrows_when_structured_fidelity_weakens: true,
        structured_fidelity_never_overstated_under_weakening: true,
        raw_or_export_safe_fallback_kept_explicit: true,
        compare_only_never_promoted_to_writable_state: true,
    }
}

fn projection() -> ArtifactReviewAccessibilityProjection {
    ArtifactReviewAccessibilityProjection {
        exposes_keyboard_and_screen_reader_labels: true,
        exposes_cli_and_export_enums: true,
        exposes_explanation_fields: true,
        auto_narrows_on_uncertain_parser_schema: true,
        auto_narrows_on_unavailable_render_trust: true,
        auto_narrows_on_unavailable_write_back_safety: true,
        auto_narrows_on_unavailable_metadata: true,
        desktop_cli_export_semantics_identical: true,
        narrowing_prevents_overstated_structured_fidelity: true,
        every_component_reachable_non_visually: true,
    }
}

fn proof_freshness() -> ArtifactReviewAccessibilityProofFreshness {
    ArtifactReviewAccessibilityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ArtifactReviewAccessibilityDowngradeTrigger> {
    vec![
        ArtifactReviewAccessibilityDowngradeTrigger::ProofStale,
        ArtifactReviewAccessibilityDowngradeTrigger::ParserSchemaUncertain,
        ArtifactReviewAccessibilityDowngradeTrigger::RenderTrustUnavailable,
        ArtifactReviewAccessibilityDowngradeTrigger::WriteBackSafetyUnavailable,
        ArtifactReviewAccessibilityDowngradeTrigger::MetadataAvailabilityUnavailable,
        ArtifactReviewAccessibilityDowngradeTrigger::ClaimOverstated,
    ]
}

fn rendering_surfaces() -> Vec<ArtifactReviewRenderingSurface> {
    ArtifactReviewRenderingSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_DOC_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_CONSUMER_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_IDENTITY_DIFF_CONTROLS_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_MERGE_GENERATED_CONTROLS_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_MEDIA_TRUST_CONTROLS_CONTRACT_REF.to_owned(),
    ]
}

fn row_refs(component: M5ArtifactComponent) -> Vec<String> {
    vec![
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

fn human_component(component: M5ArtifactComponent) -> &'static str {
    match component {
        M5ArtifactComponent::ArtifactIdentityBar => "Artifact identity bar",
        M5ArtifactComponent::DiffModeSwitcher => "Diff-mode switcher",
        M5ArtifactComponent::StructureRow => "Structure row",
        M5ArtifactComponent::MergeDecisionRow => "Merge-decision row",
        M5ArtifactComponent::GeneratedArtifactNotice => "Generated-artifact notice",
        M5ArtifactComponent::RenderedCompareViewer => "Rendered compare viewer",
        M5ArtifactComponent::MediaMetadataRail => "Media metadata rail",
        M5ArtifactComponent::RedactionOrTrustBadgeSet => "Redaction or trust badge set",
        M5ArtifactComponent::CompareSummaryCard => "Compare summary card",
    }
}

fn claim_phrase(tier: ArtifactReviewClaimTier) -> &'static str {
    match tier {
        ArtifactReviewClaimTier::FullStructuredFidelity => {
            "shown with full structured and rendered fidelity"
        }
        ArtifactReviewClaimTier::StructuredCompareOnly => {
            "structured but compare-only; write-back is unavailable"
        }
        ArtifactReviewClaimTier::PartialStructure => {
            "partial structure only; the parser/schema is uncertain"
        }
        ArtifactReviewClaimTier::RawFallbackDisclosed => {
            "shown through an explicit raw/export-safe fallback"
        }
        ArtifactReviewClaimTier::MetadataWithheld => "with metadata withheld or redacted",
    }
}

fn condition_phrase(condition: ArtifactReviewClaimCondition) -> &'static str {
    match condition {
        ArtifactReviewClaimCondition::StructuredTruthTrusted => {
            "parser/schema, render trust, write-back safety, and metadata are all trusted"
        }
        ArtifactReviewClaimCondition::ParserSchemaUncertain => {
            "parser/schema certainty is uncertain"
        }
        ArtifactReviewClaimCondition::RenderTrustUnavailable => "render trust is unavailable",
        ArtifactReviewClaimCondition::WriteBackSafetyUnavailable => {
            "merge/write-back safety is unavailable"
        }
        ArtifactReviewClaimCondition::MetadataUnavailable => {
            "metadata availability is stale or policy-blocked"
        }
    }
}

fn next_action_label(action: ArtifactReviewClaimNextAction) -> String {
    match action {
        ArtifactReviewClaimNextAction::ReparseAgainstSchema => {
            "Re-parse the artifact against a recognized schema to restore full structure".to_owned()
        }
        ArtifactReviewClaimNextAction::ReviewRawSafeFallback => {
            "Review the explicit raw/export-safe fallback while render trust is unavailable"
                .to_owned()
        }
        ArtifactReviewClaimNextAction::KeepCompareOnly => {
            "Keep this artifact compare-only; do not write back".to_owned()
        }
        ArtifactReviewClaimNextAction::RestoreMetadataAccess => {
            "Restore metadata access before relying on the withheld fields".to_owned()
        }
        ArtifactReviewClaimNextAction::ContinueStructuredReview => {
            "Continue the structured review".to_owned()
        }
    }
}

/// Builds one accessibility row, deriving the claim, narrowing, notes, and labels
/// from the component and condition so the fixture stays self-consistent.
fn row(
    row_id: &str,
    component: M5ArtifactComponent,
    condition: ArtifactReviewClaimCondition,
) -> ArtifactReviewAccessibilityRow {
    let resolution = resolve_artifact_review_claim_narrowing(condition);
    let effective_claim = resolution.permitted_ceiling;

    let narrowing = if resolution.requires_narrowing {
        Some(ArtifactReviewClaimNarrowing {
            trigger: resolution
                .expected_trigger
                .expect("weakening condition has a trigger"),
            narrowed_to: resolution.permitted_ceiling,
            preserved_truth_note: format!(
                "{} stays keyboard-reachable, screen-reader labelled, and export-legible; only the structured-fidelity claim is narrowed",
                human_component(component)
            ),
            next_action: resolution.expected_next_action,
            next_action_label: next_action_label(resolution.expected_next_action),
        })
    } else {
        None
    };

    let raw_fallback_note = if resolution.needs_raw_fallback_note {
        format!(
            "An explicit raw/export-safe fallback for the {} stays available here",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };
    let compare_only_note = if resolution.needs_compare_only_note {
        "This artifact is compare-only; write-back is unavailable and is never applied silently"
            .to_owned()
    } else {
        String::new()
    };
    let redaction_note = if resolution.needs_redaction_note {
        "Metadata or content is withheld or redacted under the export/redaction posture".to_owned()
    } else {
        String::new()
    };

    ArtifactReviewAccessibilityRow {
        row_id: row_id.to_owned(),
        component,
        condition,
        effective_claim,
        keyboard_label: format!(
            "{}: focusable, Enter opens, Space toggles detail",
            human_component(component)
        ),
        screen_reader_label: format!(
            "{}, {}",
            human_component(component),
            claim_phrase(effective_claim)
        ),
        cli_enum_token: format!("{}:{}", component.as_str(), effective_claim.as_str()),
        export_enum_token: effective_claim.as_str().to_owned(),
        explanation_field: format!(
            "{} — {}",
            claim_phrase(effective_claim),
            condition_phrase(condition)
        ),
        rendering_surfaces: rendering_surfaces(),
        narrowing,
        raw_fallback_note,
        compare_only_note,
        redaction_note,
        is_pointer_only: false,
        is_export_opaque: false,
        desktop_stronger_than_cli: false,
        source_contract_refs: row_refs(component),
    }
}

/// The canonical row set: all nine components, covering all five conditions and all
/// five claim tiers.
fn accessibility_rows() -> Vec<ArtifactReviewAccessibilityRow> {
    vec![
        row(
            "row:identity-bar-trusted",
            M5ArtifactComponent::ArtifactIdentityBar,
            ArtifactReviewClaimCondition::StructuredTruthTrusted,
        ),
        row(
            "row:diff-mode-render-untrusted",
            M5ArtifactComponent::DiffModeSwitcher,
            ArtifactReviewClaimCondition::RenderTrustUnavailable,
        ),
        row(
            "row:structure-row-parser-uncertain",
            M5ArtifactComponent::StructureRow,
            ArtifactReviewClaimCondition::ParserSchemaUncertain,
        ),
        row(
            "row:merge-decision-write-back",
            M5ArtifactComponent::MergeDecisionRow,
            ArtifactReviewClaimCondition::WriteBackSafetyUnavailable,
        ),
        row(
            "row:generated-notice-trusted",
            M5ArtifactComponent::GeneratedArtifactNotice,
            ArtifactReviewClaimCondition::StructuredTruthTrusted,
        ),
        row(
            "row:rendered-compare-render-untrusted",
            M5ArtifactComponent::RenderedCompareViewer,
            ArtifactReviewClaimCondition::RenderTrustUnavailable,
        ),
        row(
            "row:media-rail-metadata",
            M5ArtifactComponent::MediaMetadataRail,
            ArtifactReviewClaimCondition::MetadataUnavailable,
        ),
        row(
            "row:redaction-badge-metadata",
            M5ArtifactComponent::RedactionOrTrustBadgeSet,
            ArtifactReviewClaimCondition::MetadataUnavailable,
        ),
        row(
            "row:compare-summary-write-back",
            M5ArtifactComponent::CompareSummaryCard,
            ArtifactReviewClaimCondition::WriteBackSafetyUnavailable,
        ),
    ]
}

fn packet_with(rows: Vec<ArtifactReviewAccessibilityRow>) -> ArtifactReviewAccessibilityPacket {
    ArtifactReviewAccessibilityPacket::new(ArtifactReviewAccessibilityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Structured-artifact review accessibility, headless, and export parity"
            .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: downgrade_triggers(),
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> ArtifactReviewAccessibilityPacket {
    packet_with(accessibility_rows())
}

#[test]
fn accessibility_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_canonical_row_is_honest() {
    for row in accessibility_rows() {
        assert!(row.claim_is_honest(), "row not honest: {}", row.row_id);
    }
}

#[test]
fn claim_narrowing_maps_condition_to_ceiling() {
    let trusted = resolve_artifact_review_claim_narrowing(
        ArtifactReviewClaimCondition::StructuredTruthTrusted,
    );
    assert_eq!(
        trusted.permitted_ceiling,
        ArtifactReviewClaimTier::FullStructuredFidelity
    );
    assert!(!trusted.requires_narrowing);
    assert!(trusted.expected_trigger.is_none());
    assert!(!trusted.needs_raw_fallback_note);
    assert!(!trusted.needs_compare_only_note);
    assert!(!trusted.needs_redaction_note);

    let parser = resolve_artifact_review_claim_narrowing(
        ArtifactReviewClaimCondition::ParserSchemaUncertain,
    );
    assert_eq!(
        parser.permitted_ceiling,
        ArtifactReviewClaimTier::PartialStructure
    );
    assert!(parser.requires_narrowing);
    assert!(parser.needs_raw_fallback_note);
    assert!(!parser.needs_compare_only_note);
    assert!(!parser.needs_redaction_note);

    let render = resolve_artifact_review_claim_narrowing(
        ArtifactReviewClaimCondition::RenderTrustUnavailable,
    );
    assert_eq!(
        render.permitted_ceiling,
        ArtifactReviewClaimTier::RawFallbackDisclosed
    );
    assert_eq!(
        render.expected_trigger,
        Some(ArtifactReviewAccessibilityDowngradeTrigger::RenderTrustUnavailable)
    );

    let write_back = resolve_artifact_review_claim_narrowing(
        ArtifactReviewClaimCondition::WriteBackSafetyUnavailable,
    );
    assert_eq!(
        write_back.permitted_ceiling,
        ArtifactReviewClaimTier::StructuredCompareOnly
    );
    assert!(write_back.needs_compare_only_note);
    assert!(write_back.needs_raw_fallback_note);
    assert!(!write_back.needs_redaction_note);

    let metadata =
        resolve_artifact_review_claim_narrowing(ArtifactReviewClaimCondition::MetadataUnavailable);
    assert_eq!(
        metadata.permitted_ceiling,
        ArtifactReviewClaimTier::MetadataWithheld
    );
    assert!(metadata.needs_redaction_note);
    assert!(metadata.needs_raw_fallback_note);
    assert!(!metadata.needs_compare_only_note);
}

// --- AC2: narrowing prevents overstated structured fidelity -------------------

#[test]
fn full_fidelity_claim_never_survives_a_weakening_condition() {
    // A component that keeps asserting full structured fidelity while the parser/schema
    // is uncertain overstates its truth and must be caught.
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::ParserSchemaUncertain)
        .expect("parser-uncertain row present");
    packet.accessibility_rows[index].effective_claim =
        ArtifactReviewClaimTier::FullStructuredFidelity;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn claim_ceiling_exceeded_on_render_untrusted_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::RenderTrustUnavailable)
        .expect("render-untrusted row present");
    // Claim structured-compare-only (rank 4) above the raw-fallback ceiling (rank 2).
    packet.accessibility_rows[index].effective_claim =
        ArtifactReviewClaimTier::StructuredCompareOnly;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn weakening_condition_without_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].narrowing = None;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ClaimNarrowingMissing));
}

#[test]
fn baseline_condition_with_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::StructuredTruthTrusted)
        .expect("trusted row present");
    packet.accessibility_rows[index].narrowing = Some(ArtifactReviewClaimNarrowing {
        trigger: ArtifactReviewAccessibilityDowngradeTrigger::ParserSchemaUncertain,
        narrowed_to: ArtifactReviewClaimTier::FullStructuredFidelity,
        preserved_truth_note: "note".to_owned(),
        next_action: ArtifactReviewClaimNextAction::ContinueStructuredReview,
        next_action_label: "Continue".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ClaimNarrowingUnexpected));
}

#[test]
fn narrowed_to_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.narrowed_to = ArtifactReviewClaimTier::MetadataWithheld;
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::NarrowedToMismatch));
}

#[test]
fn narrow_trigger_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::ParserSchemaUncertain)
        .expect("parser-uncertain row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.trigger = ArtifactReviewAccessibilityDowngradeTrigger::RenderTrustUnavailable;
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::NarrowTriggerMismatch));
}

#[test]
fn narrow_next_action_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::WriteBackSafetyUnavailable)
        .expect("write-back row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action = ArtifactReviewClaimNextAction::ContinueStructuredReview;
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::NarrowNextActionMismatch));
}

#[test]
fn narrow_missing_preserved_truth_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.preserved_truth_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::NarrowPreservedTruthMissing));
}

#[test]
fn narrow_missing_next_action_label_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action_label = String::new();
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::NarrowNextActionMissing));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ---------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ExplanationFieldMissing));
}

#[test]
fn rendering_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].rendering_surfaces =
        vec![ArtifactReviewRenderingSurface::DesktopFull];
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::RenderingSurfaceCoverageMissing));
}

#[test]
fn pointer_only_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_pointer_only = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::PointerOnlyComponent));
}

#[test]
fn export_opaque_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_export_opaque = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ExportOpaqueComponent));
}

#[test]
fn desktop_stronger_than_cli_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].desktop_stronger_than_cli = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::DesktopStrongerThanCli));
}

#[test]
fn raw_fallback_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].raw_fallback_note = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::RawFallbackNoteMissing));
}

#[test]
fn compare_only_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::WriteBackSafetyUnavailable)
        .expect("write-back row present");
    packet.accessibility_rows[index].compare_only_note = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::CompareOnlyNoteMissing));
}

#[test]
fn redaction_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ArtifactReviewClaimCondition::MetadataUnavailable)
        .expect("metadata row present");
    packet.accessibility_rows[index].redaction_note = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::RedactionNoteMissing));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].source_contract_refs =
        vec![M5_ARTIFACT_REVIEW_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn missing_component_coverage_fails() {
    let mut rows = accessibility_rows();
    rows.retain(|r| r.component != M5ArtifactComponent::CompareSummaryCard);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ComponentCoverageMissing));
}

#[test]
fn missing_condition_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only parser-uncertain row.
    rows.retain(|r| r.condition != ArtifactReviewClaimCondition::ParserSchemaUncertain);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ConditionCoverageMissing));
}

#[test]
fn missing_claim_tier_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only partial-structure row; that tier is then unreachable.
    rows.retain(|r| r.effective_claim != ArtifactReviewClaimTier::PartialStructure);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ClaimTierCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.accessibility_rows.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::AccessibilityRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .compare_only_never_promoted_to_writable_state = false;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::TrustReviewIncomplete));
}

#[test]
fn projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .projection
        .narrowing_prevents_overstated_structured_fidelity = false;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewAccessibilityViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Accessibility rows"));
    assert!(summary.contains("artifact_identity_bar"));
    assert!(summary.contains("compare_summary_card"));
    assert!(summary.contains("raw_fallback_disclosed"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_artifact_review_accessibility_export()
        .expect("checked structured-artifact review accessibility export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-component-accessibility-parity/parser_schema_and_render_trust_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-component-accessibility-parity/write_back_and_metadata_narrowed.json"
        )),
    ] {
        let packet: ArtifactReviewAccessibilityPacket = serde_json::from_str(raw)
            .expect("fixture parses as structured-artifact review accessibility packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// The canonical rows plus extra scenario rows that demonstrate a normally-trusted
/// component auto-narrowing under an uncertain parser/schema and unavailable render
/// trust. The base rows keep full component / condition / tier coverage; the extra
/// rows show the narrowing.
fn fixture_parser_schema_and_render_trust_narrowed() -> ArtifactReviewAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:identity-bar-parser-narrowed",
        M5ArtifactComponent::ArtifactIdentityBar,
        ArtifactReviewClaimCondition::ParserSchemaUncertain,
    ));
    rows.push(row(
        "row:generated-notice-render-narrowed",
        M5ArtifactComponent::GeneratedArtifactNotice,
        ArtifactReviewClaimCondition::RenderTrustUnavailable,
    ));
    ArtifactReviewAccessibilityPacket::new(ArtifactReviewAccessibilityPacketInput {
        packet_id: "structured-artifact-review-accessibility:fixture:parser-schema-and-render-trust-narrowed"
            .to_owned(),
        surface_label:
            "Structured-artifact review accessibility: parser/schema uncertain and render trust unavailable, claim auto-narrowed"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            ArtifactReviewAccessibilityDowngradeTrigger::ParserSchemaUncertain,
            ArtifactReviewAccessibilityDowngradeTrigger::RenderTrustUnavailable,
            ArtifactReviewAccessibilityDowngradeTrigger::ClaimOverstated,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// The canonical rows plus extra scenario rows for a generated-artifact notice losing
/// write-back safety and an artifact identity bar losing metadata availability.
fn fixture_write_back_and_metadata_narrowed() -> ArtifactReviewAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:generated-notice-write-back-narrowed",
        M5ArtifactComponent::GeneratedArtifactNotice,
        ArtifactReviewClaimCondition::WriteBackSafetyUnavailable,
    ));
    rows.push(row(
        "row:identity-bar-metadata-narrowed",
        M5ArtifactComponent::ArtifactIdentityBar,
        ArtifactReviewClaimCondition::MetadataUnavailable,
    ));
    ArtifactReviewAccessibilityPacket::new(ArtifactReviewAccessibilityPacketInput {
        packet_id:
            "structured-artifact-review-accessibility:fixture:write-back-and-metadata-narrowed"
                .to_owned(),
        surface_label:
            "Structured-artifact review accessibility: write-back safety and metadata unavailable"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            ArtifactReviewAccessibilityDowngradeTrigger::WriteBackSafetyUnavailable,
            ArtifactReviewAccessibilityDowngradeTrigger::MetadataAvailabilityUnavailable,
            ArtifactReviewAccessibilityDowngradeTrigger::ClaimOverstated,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_ARTIFACT_REVIEW_ACCESSIBILITY_ARTIFACTS` so it never writes during
/// a normal test run. Run with the env var set to refresh the artifacts after a
/// contract change, then review the diff.
#[test]
fn regenerate_artifact_review_accessibility_artifacts() {
    if std::env::var("GEN_ARTIFACT_REVIEW_ACCESSIBILITY_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir =
        format!("{root}/artifacts/release/m5-structured-artifact-review-accessibility-proof");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{artifact_dir}/summary.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir =
        format!("{root}/fixtures/ui/m5-structured-artifact-review-component-accessibility-parity");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "parser_schema_and_render_trust_narrowed.json",
            fixture_parser_schema_and_render_trust_narrowed(),
        ),
        (
            "write_back_and_metadata_narrowed.json",
            fixture_write_back_and_metadata_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
