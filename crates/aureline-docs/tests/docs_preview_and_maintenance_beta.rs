use std::path::{Path, PathBuf};

use aureline_docs::{
    seeded_docs_preview_and_maintenance_contract, DocsExampleValidationMode,
    DocsMaintenanceContract, DocsMaintenanceReviewPacket, DocsMaintenanceSurfaceProjection,
    DocsPreviewMode, DocsPreviewSanitizationState, DocsPublishBoundaryState,
    DocsSuggestionApplyPosture, DocsSuggestionTrigger,
};

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

const FIXTURE_DIR: &str = "fixtures/docs/m3/docs_preview_and_maintenance";

fn load_contract() -> DocsMaintenanceContract {
    let payload = std::fs::read_to_string(repo_path(&format!("{FIXTURE_DIR}/manifest.json")))
        .expect("manifest fixture reads");
    serde_json::from_str(&payload).expect("manifest fixture parses")
}

fn load_surface() -> DocsMaintenanceSurfaceProjection {
    let payload =
        std::fs::read_to_string(repo_path(&format!("{FIXTURE_DIR}/surface_projection.json")))
            .expect("surface fixture reads");
    serde_json::from_str(&payload).expect("surface fixture parses")
}

fn load_review_packet() -> DocsMaintenanceReviewPacket {
    let payload = std::fs::read_to_string(repo_path(&format!("{FIXTURE_DIR}/review_packet.json")))
        .expect("review packet fixture reads");
    serde_json::from_str(&payload).expect("review packet fixture parses")
}

#[test]
fn fixtures_match_seeded_records_and_validate() {
    let seeded = seeded_docs_preview_and_maintenance_contract();
    let fixture = load_contract();
    assert_eq!(
        fixture, seeded,
        "committed manifest fixture is stale; regenerate with the beta bin"
    );
    assert!(fixture.validate().is_empty(), "{:?}", fixture.validate());

    let surface = load_surface();
    assert_eq!(surface, seeded.surface_projection());
}

#[test]
fn rendered_preview_is_never_canonical_and_actions_are_keyboard_reachable() {
    let contract = load_contract();
    for mode in [
        DocsPreviewMode::Source,
        DocsPreviewMode::Split,
        DocsPreviewMode::Rendered,
    ] {
        let header = contract
            .preview_headers
            .iter()
            .find(|header| header.preview_mode == mode)
            .unwrap_or_else(|| panic!("preview mode {} is covered", mode.as_str()));
        assert!(
            header.commonmark_baseline,
            "CommonMark baseline is declared"
        );
        assert!(header.mode_toggle_keyboard_reachable);
        assert!(header.open_source_action.keyboard_reachable);
        if mode.renders_preview() {
            assert!(
                !header.rendered_is_not_canonical_note.trim().is_empty(),
                "rendered/split preview discloses it is not canonical source or proof"
            );
        } else {
            assert_eq!(
                header.sanitization_state,
                DocsPreviewSanitizationState::NotApplicable
            );
        }
    }
}

#[test]
fn suggestions_are_diff_based_evidence_backed_and_never_silent() {
    let contract = load_contract();
    for trigger in [
        DocsSuggestionTrigger::CodeDiff,
        DocsSuggestionTrigger::StaleExample,
        DocsSuggestionTrigger::ReleaseNoteDrift,
        DocsSuggestionTrigger::FailingSnippet,
        DocsSuggestionTrigger::ContractChange,
        DocsSuggestionTrigger::HumanNote,
    ] {
        assert!(
            contract
                .suggestion_cards
                .iter()
                .any(|card| card.trigger == trigger),
            "trigger {} is covered",
            trigger.as_str()
        );
    }
    for card in &contract.suggestion_cards {
        assert!(
            card.silent_rewrite_blocked,
            "{} blocks silent rewrite",
            card.card_id
        );
        assert!(
            !card.evidence_refs.is_empty(),
            "{} is evidence-backed",
            card.card_id
        );
        assert!(card.open_evidence_action.keyboard_reachable);
        if card.apply_posture.requires_review_diff() {
            assert!(
                card.review_diff_ref
                    .as_deref()
                    .is_some_and(|value| !value.is_empty()),
                "{} carries a review diff",
                card.card_id
            );
        }
    }
    // No card is apply-ready while the publish boundary is blocked-unscoped.
    assert!(contract.suggestion_cards.iter().all(|card| {
        card.publish_boundary_state != DocsPublishBoundaryState::BlockedUnscoped
            || card.apply_posture == DocsSuggestionApplyPosture::BlockedPendingEvidence
    }));
}

#[test]
fn example_validation_distinguishes_all_states() {
    let contract = load_contract();
    for mode in [
        DocsExampleValidationMode::Rendered,
        DocsExampleValidationMode::SyntaxChecked,
        DocsExampleValidationMode::ExecutedLocally,
        DocsExampleValidationMode::ExecutedRemotely,
        DocsExampleValidationMode::Unsupported,
        DocsExampleValidationMode::Skipped,
        DocsExampleValidationMode::Stale,
        DocsExampleValidationMode::NotValidated,
    ] {
        assert!(
            contract
                .finding_rows
                .iter()
                .any(|row| row.validation_mode == mode),
            "validation mode {} is exercised",
            mode.as_str()
        );
    }
    // A suppress-until-reviewed finding carries a suppression ref.
    assert!(contract.finding_rows.iter().any(|row| {
        row.suppression_state == aureline_docs::DocsFindingSuppressionState::SuppressedUntilReviewed
            && row.suppression_ref.is_some()
    }));
}

#[test]
fn maintenance_rows_preserve_publish_scope_and_block_unscoped_publish() {
    let contract = load_contract();

    let publish = contract
        .maintenance_rows
        .iter()
        .find(|row| row.publish_boundary_state == DocsPublishBoundaryState::PublishHandoffScoped)
        .expect("a scoped publish-handoff row exists");
    assert!(publish.publish_scope.is_scoped());
    assert!(!publish.publish_boundary_notes.is_empty());
    // Beta notes keep their channel scope so they cannot pass for stable docs.
    assert_eq!(publish.publish_scope.channel_scope.as_deref(), Some("beta"));

    let blocked = contract
        .maintenance_rows
        .iter()
        .find(|row| row.publish_boundary_state == DocsPublishBoundaryState::BlockedUnscoped)
        .expect("a blocked-unscoped row exists");
    assert!(!blocked.publish_scope.is_scoped());
    assert!(
        blocked.apply_export_action.is_none(),
        "blocked unscoped rows expose no apply/export action"
    );

    assert!(contract
        .maintenance_rows
        .iter()
        .any(|row| row.publish_boundary_state == DocsPublishBoundaryState::LocalOnly));

    // Pending counts stay consistent with referenced cards/findings.
    for row in &contract.maintenance_rows {
        assert_eq!(row.pending_suggestion_count, row.suggestion_card_refs.len());
        assert_eq!(row.pending_finding_count, row.finding_row_refs.len());
    }
}

#[test]
fn review_packet_preserves_publish_boundary_without_raw_bodies() {
    let contract = load_contract();
    let packet = load_review_packet();
    packet
        .validate_against_contract(&contract)
        .expect("review packet reconstructs from the contract");
    assert!(!packet.raw_document_bodies_exported);
    assert!(packet.handoff_banner.screenshot_free_review);
    assert!(!packet.omitted_material_classes.is_empty());

    let json = packet.export_safe_json();
    assert!(!json.contains("://"), "export carries no raw URLs");
    assert!(json.contains("\"raw_document_bodies_exported\": false"));
}

#[test]
fn manifest_points_at_the_schemas_and_help_doc() {
    let payload = std::fs::read_to_string(repo_path(&format!("{FIXTURE_DIR}/manifest.json")))
        .expect("manifest fixture reads");
    assert!(payload.contains("schemas/docs/docs_suggestion_card.schema.json"));
    assert!(payload.contains("schemas/docs/docs_maintenance_row.schema.json"));
    assert!(payload.contains("docs/help/m3/docs_preview_and_maintenance_beta.md"));
}
