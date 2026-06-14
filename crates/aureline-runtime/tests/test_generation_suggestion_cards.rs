use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::test_generation_suggestion_cards_and_diff_first_apply::{
    current_test_generation_export, ApplyState, TestGenerationProposalPacket, ValidationPosture,
};

fn fixture(name: &str) -> TestGenerationProposalPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_test_generation_export()
        .expect("checked-in test-generation export should validate");
    assert!(packet.validate().is_empty());

    // Both an uncovered-path and a bug source are exercised, not merely declared.
    let sources = packet.represented_source_kinds();
    assert!(sources.iter().any(|s| s.is_uncovered_path()));
    assert!(sources.iter().any(|s| s.is_bug_source()));
}

#[test]
fn applied_cards_carry_sandbox_pass_preview_and_rerun_linkage() {
    let packet = current_test_generation_export().expect("export validates");
    for card in &packet.cards {
        if card.apply_posture.state.is_applied() {
            assert!(
                card.sandbox_validation.posture.is_validated_pass(),
                "card {} applied without a sandbox pass",
                card.card_id
            );
            assert!(card.apply_posture.preview_first);
            assert!(!card.apply_posture.widens_beyond_evidence);
            assert!(card.follow_on_rerun_ref.is_some());
            assert!(!card.is_imported());
        }
    }
    // The apply path is actually exercised.
    assert!(packet.applied_card_count() >= 1);
}

#[test]
fn imported_proposals_never_read_as_local() {
    let packet = current_test_generation_export().expect("export validates");
    let imported = packet
        .cards
        .iter()
        .find(|c| c.is_imported())
        .expect("an imported proposal");
    assert!(imported.origin_provider_ref.is_some());
    assert_eq!(
        imported.sandbox_validation.posture,
        ValidationPosture::ImportedUnvalidated
    );
    assert!(!imported.apply_posture.state.is_applied());
}

#[test]
fn proposals_are_evidence_bound_not_free_text() {
    let packet = current_test_generation_export().expect("export validates");
    for card in &packet.cards {
        assert!(
            card.evidence_reopenable(),
            "card {} is not bound to a reopenable evidence object",
            card.card_id
        );
    }
}

#[test]
fn template_and_invocation_identities_stay_distinct() {
    let packet = current_test_generation_export().expect("export validates");
    let kinds = packet.represented_subject_kinds();
    assert!(kinds.contains(&DurableTestNodeKind::ParameterizedTemplate));
    assert!(kinds.contains(&DurableTestNodeKind::ConcreteInvocation));
}

#[test]
fn fixture_sandbox_gate_blocks_apply() {
    let packet = fixture("sandbox_gate_blocks_apply.json");
    assert!(packet.validate().is_empty());

    // A not-yet-validated proposal is blocked from apply.
    let blocked = packet
        .cards
        .iter()
        .find(|c| c.apply_posture.state == ApplyState::BlockedNeedsValidation)
        .expect("a blocked card");
    assert!(!blocked.sandbox_validation.posture.is_validated_pass());
    assert!(!blocked.apply_posture.state.is_applied());

    // A widening proposal is routed to review rather than silently applied.
    let widening = packet
        .cards
        .iter()
        .find(|c| c.apply_posture.widens_beyond_evidence)
        .expect("a widening card");
    assert!(!widening.apply_posture.state.is_applied());
}
