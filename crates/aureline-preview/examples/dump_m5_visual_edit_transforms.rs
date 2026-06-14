//! Conformance dump for the M5 visual-edit transform packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::visual_edit_transforms::*;
use aureline_preview::{MutationReviewPosture, RoundTripCapabilityClass};

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-edit:{id}")]
}

fn edits() -> Vec<VisualEditRow> {
    vec![
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
        packet_id: "m5-visual-edit-transforms:stable:0001".to_owned(),
        set_label: "M5 Visual-Edit Transforms".to_owned(),
        edits: edits(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-08T00:00:00Z".to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
