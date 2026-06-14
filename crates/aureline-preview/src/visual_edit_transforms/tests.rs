use super::*;

const PACKET_ID: &str = "m5-visual-edit-transforms:stable:0001";

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-edit:{id}")]
}

fn edits() -> Vec<VisualEditRow> {
    vec![
        // Exact round-trip apply on a React static attribute; previews the real
        // unified source diff and takes a checkpoint before apply.
        VisualEditRow {
            edit_id: "edit:react:attr:0001".to_owned(),
            framework_pack_family: "react".to_owned(),
            surface_label: "React component preview — static className attribute".to_owned(),
            outcome: VisualEditOutcomeClass::ExactRoundTripApply,
            round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
            construct_class: TransformConstructClass::StaticAttribute,
            selection_context_ref: "selection:react:attr:0001".to_owned(),
            source_target_ref: "span:react:attr:0001".to_owned(),
            preview_diff: PreviewDiffClass::RealSourceUnifiedDiff,
            rollback_class: RollbackClass::CheckpointRevertible,
            protected_path_posture: ProtectedPathPosture::Unprotected,
            review_posture: Some(MutationReviewPosture::ReviewRequired),
            transform_manifest: Some(TransformManifest {
                manifest_id: "manifest:react:attr:0001".to_owned(),
                pipeline_ref: "pipeline:preview-apply-revert".to_owned(),
                applies_real_source_diff: true,
                inverse_available: true,
            }),
            unsupported_card: None,
            label_summary: "Exact round-trip edit of a literal className; the real source diff is previewed and a checkpoint is taken before apply".to_owned(),
            observed_at: "2026-06-08T00:00:00Z".to_owned(),
            evidence_refs: ev("react:attr:0001"),
        },
        // Approximate round-trip apply on a React design-token style; a multi-file
        // diff and a working-tree snapshot back the revert on a protected path.
        VisualEditRow {
            edit_id: "edit:react:token:0001".to_owned(),
            framework_pack_family: "react".to_owned(),
            surface_label: "React component preview — design-token style value".to_owned(),
            outcome: VisualEditOutcomeClass::ApproximateRoundTripApply,
            round_trip: RoundTripCapabilityClass::ApproximateSourceRoundTrip,
            construct_class: TransformConstructClass::StaticStyleToken,
            selection_context_ref: "selection:react:token:0001".to_owned(),
            source_target_ref: "span:react:token:0001".to_owned(),
            preview_diff: PreviewDiffClass::RealSourceMultiFileDiff,
            rollback_class: RollbackClass::SnapshotRevertible,
            protected_path_posture: ProtectedPathPosture::ProtectedReviewRequired,
            review_posture: Some(MutationReviewPosture::ConfirmationRequired),
            transform_manifest: Some(TransformManifest {
                manifest_id: "manifest:react:token:0001".to_owned(),
                pipeline_ref: "pipeline:preview-apply-revert".to_owned(),
                applies_real_source_diff: true,
                inverse_available: false,
            }),
            unsupported_card: None,
            label_summary: "Approximate round-trip edit of a design-token value across files; the multi-file diff is previewed and a snapshot backs the revert".to_owned(),
            observed_at: "2026-06-08T00:00:00Z".to_owned(),
            evidence_refs: ev("react:token:0001"),
        },
        // Code-first fallback for a React dynamic-bound expression; degrades to a
        // suggestion diff with no write and preserves the selection.
        VisualEditRow {
            edit_id: "edit:react:dynamic:0001".to_owned(),
            framework_pack_family: "react".to_owned(),
            surface_label: "React component preview — dynamically bound style".to_owned(),
            outcome: VisualEditOutcomeClass::CodeFirstFallback,
            round_trip: RoundTripCapabilityClass::SourceOnlyFallback,
            construct_class: TransformConstructClass::DynamicBoundExpression,
            selection_context_ref: "selection:react:dynamic:0001".to_owned(),
            source_target_ref: "span:react:dynamic:0001".to_owned(),
            preview_diff: PreviewDiffClass::CodeFirstSuggestionDiff,
            rollback_class: RollbackClass::NoMutationNoRollback,
            protected_path_posture: ProtectedPathPosture::Unprotected,
            review_posture: None,
            transform_manifest: None,
            unsupported_card: Some(UnsupportedConstructCard {
                reason: UnsupportedConstructReason::DynamicBinding,
                preserves_selection_context: true,
                card_label: "This style is bound to a runtime expression; the visual edit degrades to a code-first source suggestion rather than guess the binding".to_owned(),
            }),
            label_summary: "A dynamically bound style cannot round-trip; the edit degrades to a code-first suggestion with the selection preserved".to_owned(),
            observed_at: "2026-06-08T00:00:00Z".to_owned(),
            evidence_refs: ev("react:dynamic:0001"),
        },
        // Inspect-only fallback for a React generated artifact; no diff, no write,
        // selection preserved. Gives React both a round-trip and a preview-only row.
        VisualEditRow {
            edit_id: "edit:react:generated:0001".to_owned(),
            framework_pack_family: "react".to_owned(),
            surface_label: "React component preview — compiled vendor stylesheet node".to_owned(),
            outcome: VisualEditOutcomeClass::InspectOnly,
            round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
            construct_class: TransformConstructClass::ExternalOrGeneratedArtifact,
            selection_context_ref: "selection:react:generated:0001".to_owned(),
            source_target_ref: "span:react:generated:0001".to_owned(),
            preview_diff: PreviewDiffClass::NoDiffInspectOnly,
            rollback_class: RollbackClass::NoMutationNoRollback,
            protected_path_posture: ProtectedPathPosture::Unprotected,
            review_posture: None,
            transform_manifest: None,
            unsupported_card: Some(UnsupportedConstructCard {
                reason: UnsupportedConstructReason::GeneratedOrExternalArtifact,
                preserves_selection_context: true,
                card_label: "This node comes from a compiled vendor stylesheet with no hand-authored span; the surface stays inspect-only and never writes back".to_owned(),
            }),
            label_summary: "A generated vendor node has no source span; the surface stays inspect-only and never auto-upgrades to a write".to_owned(),
            observed_at: "2026-06-08T00:00:00Z".to_owned(),
            evidence_refs: ev("react:generated:0001"),
        },
        // Code-first fallback on a Flutter widget under a blocked protected path;
        // the protected block forces the degrade.
        VisualEditRow {
            edit_id: "edit:flutter:protected:0001".to_owned(),
            framework_pack_family: "flutter".to_owned(),
            surface_label: "Flutter widget preview — generated list item under a protected path".to_owned(),
            outcome: VisualEditOutcomeClass::CodeFirstFallback,
            round_trip: RoundTripCapabilityClass::SourceOnlyFallback,
            construct_class: TransformConstructClass::ConditionalOrLoopGenerated,
            selection_context_ref: "selection:flutter:protected:0001".to_owned(),
            source_target_ref: "span:flutter:protected:0001".to_owned(),
            preview_diff: PreviewDiffClass::CodeFirstSuggestionDiff,
            rollback_class: RollbackClass::NoMutationNoRollback,
            protected_path_posture: ProtectedPathPosture::ProtectedBlocked,
            review_posture: None,
            transform_manifest: None,
            unsupported_card: Some(UnsupportedConstructCard {
                reason: UnsupportedConstructReason::ProtectedPathBlocked,
                preserves_selection_context: true,
                card_label: "This widget is generated inside a loop and its file is a blocked protected path; the edit degrades to a code-first suggestion the owner can review".to_owned(),
            }),
            label_summary: "A loop-generated widget on a blocked protected path cannot apply; the edit degrades to a code-first suggestion for owner review".to_owned(),
            observed_at: "2026-06-08T00:00:00Z".to_owned(),
            evidence_refs: ev("flutter:protected:0001"),
        },
    ]
}

fn guardrails() -> VisualEditTransformGuardrails {
    VisualEditTransformGuardrails {
        source_canonical_no_second_writable_model: true,
        private_wording_never_hides_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        every_apply_emits_manifest_diff_and_rollback: true,
        ambiguous_constructs_never_silently_rewritten: true,
        protected_paths_and_ownership_preserved: true,
        apply_routes_through_shared_pipeline: true,
    }
}

fn consumer_projection() -> VisualEditTransformConsumerProjection {
    VisualEditTransformConsumerProjection {
        product_ingests_transforms: true,
        docs_help_ingests_transforms: true,
        diagnostics_ingests_transforms: true,
        support_export_ingests_transforms: true,
        release_control_ingests_transforms: true,
        release_distinguishes_preview_only_from_round_trip: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        VISUAL_EDIT_TRANSFORMS_SCHEMA_REF.to_owned(),
        VISUAL_EDIT_TRANSFORMS_DOC_REF.to_owned(),
        VISUAL_EDIT_TRANSFORMS_ARTIFACT_REF.to_owned(),
        "schemas/preview/browser_runtime_inspectors.schema.json".to_owned(),
        "schemas/preview/inspect_to_source_tree_mapping.schema.json".to_owned(),
        "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json".to_owned(),
    ]
}

fn packet() -> VisualEditTransformPacket {
    VisualEditTransformPacket::new(VisualEditTransformPacketInput {
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Visual-Edit Transforms".to_owned(),
        edits: edits(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-08T00:00:00Z".to_owned(),
    })
}

fn row_mut<'a>(packet: &'a mut VisualEditTransformPacket, edit_id: &str) -> &'a mut VisualEditRow {
    packet
        .edits
        .iter_mut()
        .find(|r| r.edit_id == edit_id)
        .unwrap_or_else(|| panic!("edit {edit_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_outcome_is_present() {
    let outcomes = packet().represented_outcomes();
    for outcome in VisualEditOutcomeClass::ALL {
        assert!(
            outcomes.contains(&outcome),
            "missing outcome: {}",
            outcome.as_str()
        );
    }
}

#[test]
fn apply_and_fallback_cases_present() {
    let packet = packet();
    assert_eq!(packet.apply_row_count(), 2);
    assert_eq!(packet.fallback_row_count(), 3);
}

#[test]
fn round_trip_and_preview_only_share_a_family() {
    assert!(packet().has_family_with_round_trip_and_preview_only());
}

#[test]
fn missing_outcome_fails() {
    let mut packet = packet();
    packet
        .edits
        .retain(|r| r.outcome != VisualEditOutcomeClass::InspectOnly);
    let violations = packet.validate();
    assert!(violations.contains(&VisualEditTransformViolation::RequiredOutcomeMissing));
    // Removing the only inspect-only row also removes the preview-only side of the
    // shared-family proof.
    assert!(violations.contains(&VisualEditTransformViolation::RoundTripVsPreviewOnlyCaseMissing));
}

#[test]
fn round_trip_mismatch_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").round_trip =
        RoundTripCapabilityClass::ApproximateSourceRoundTrip;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::RoundTripMismatch));
}

#[test]
fn apply_on_lossy_construct_fails() {
    let mut packet = packet();
    // Make the exact apply target an ambiguous dynamic-bound construct: a silent
    // lossy rewrite the spec forbids.
    row_mut(&mut packet, "edit:react:attr:0001").construct_class =
        TransformConstructClass::DynamicBoundExpression;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::ConstructUnsupportedForOutcome));
}

#[test]
fn apply_without_real_source_diff_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").preview_diff =
        PreviewDiffClass::CodeFirstSuggestionDiff;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::PreviewDiffMismatch));
}

#[test]
fn apply_without_revertible_rollback_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").rollback_class =
        RollbackClass::NoMutationNoRollback;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::RollbackMismatch));
}

#[test]
fn apply_against_blocked_protected_path_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").protected_path_posture =
        ProtectedPathPosture::ProtectedBlocked;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::ProtectedPathViolation));
}

#[test]
fn apply_without_review_posture_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").review_posture = None;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::ReviewPostureInconsistent));
}

#[test]
fn fallback_carrying_review_posture_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:dynamic:0001").review_posture =
        Some(MutationReviewPosture::ReviewRequired);
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::ReviewPostureInconsistent));
}

#[test]
fn apply_without_manifest_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").transform_manifest = None;
    let violations = packet.validate();
    assert!(violations.contains(&VisualEditTransformViolation::TransformManifestInconsistent));
    // The apply that lost its manifest no longer counts as a complete round-trip
    // apply; with both applies stripped, the case-missing guard also fires.
    row_mut(&mut packet, "edit:react:token:0001").transform_manifest = None;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::RoundTripApplyCaseMissing));
}

#[test]
fn manifest_not_applying_real_source_diff_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001")
        .transform_manifest
        .as_mut()
        .expect("manifest")
        .applies_real_source_diff = false;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::TransformManifestInconsistent));
}

#[test]
fn fallback_carrying_manifest_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:dynamic:0001").transform_manifest = Some(TransformManifest {
        manifest_id: "manifest:leak".to_owned(),
        pipeline_ref: "pipeline:preview-apply-revert".to_owned(),
        applies_real_source_diff: true,
        inverse_available: false,
    });
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::TransformManifestInconsistent));
}

#[test]
fn fallback_without_card_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:dynamic:0001").unsupported_card = None;
    let violations = packet.validate();
    assert!(violations.contains(&VisualEditTransformViolation::UnsupportedCardInconsistent));
}

#[test]
fn apply_carrying_card_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").unsupported_card =
        Some(UnsupportedConstructCard {
            reason: UnsupportedConstructReason::AmbiguousSourceMapping,
            preserves_selection_context: true,
            card_label: "Some precise but misplaced card label for an apply row".to_owned(),
        });
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::UnsupportedCardInconsistent));
}

#[test]
fn fallback_dropping_selection_context_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:dynamic:0001")
        .unsupported_card
        .as_mut()
        .expect("card")
        .preserves_selection_context = false;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::UnsupportedCardInconsistent));
}

#[test]
fn generic_card_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:dynamic:0001")
        .unsupported_card
        .as_mut()
        .expect("card")
        .card_label = "unsupported".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::UnsupportedCardInconsistent));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.edits[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != VISUAL_EDIT_TRANSFORMS_DOC_REF);
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet
        .guardrails
        .ambiguous_constructs_never_silently_rewritten = false;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .release_distinguishes_preview_only_from_round_trip = false;
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "edit:react:attr:0001").label_summary =
        "leaked Bearer abc123 token".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualEditTransformViolation::RawBoundaryMaterialInExport));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: VisualEditTransformPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().edits[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("outcome=exact_round_trip_apply"));
    assert!(chips.contains("round_trip=exact_source_round_trip"));
    assert!(chips.contains("construct=static_attribute"));
    assert!(chips.contains("diff=real_source_unified_diff"));
    assert!(chips.contains("rollback=checkpoint_revertible"));
    assert!(chips.contains("family=react"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Visual-Edit Transforms"));
    assert!(summary.contains("edit:flutter:protected:0001"));
    assert!(summary.contains("Manifest:"));
    assert!(summary.contains("Fallback:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_visual_edit_transforms_export()
        .expect("checked visual-edit transform export validates");
    assert_eq!(checked, packet());
}
