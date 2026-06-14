use aureline_runtime::scope_compatible_selection_objects_and_widened_selection_review::{
    current_portable_selection_export, PortableSelectionPacket, SelectionIntentKind,
    SelectorChannel, TargetCompatibilityClass, WidenedSelectionReviewState,
};

fn fixture(name: &str) -> PortableSelectionPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_portable_selection_export()
        .expect("checked-in portable selection export should validate");
    assert!(packet.validate().is_empty());

    for channel in SelectorChannel::ALL {
        assert!(
            packet.represented_channels().contains(&channel),
            "missing channel {}",
            channel.as_str()
        );
    }
    for intent in [
        SelectionIntentKind::RerunAll,
        SelectionIntentKind::RerunFailed,
        SelectionIntentKind::ChangedSince,
        SelectionIntentKind::SnapshotScoped,
    ] {
        assert!(
            packet.represented_intents().contains(&intent),
            "missing intent {}",
            intent.as_str()
        );
    }
}

#[test]
fn artifact_demonstrates_each_compatibility_outcome() {
    let packet = current_portable_selection_export().expect("export validates");

    let has = |class: TargetCompatibilityClass| {
        packet
            .assessments
            .iter()
            .any(|a| a.compatibility_class == class)
    };
    assert!(has(TargetCompatibilityClass::Compatible));
    assert!(has(TargetCompatibilityClass::WidenedNeedsReview));
    assert!(has(TargetCompatibilityClass::SnapshotDrifted));
    assert!(has(TargetCompatibilityClass::ImportedNotRerunnable));
}

#[test]
fn compatible_assessment_dispatches_others_do_not() {
    let packet = current_portable_selection_export().expect("export validates");
    for assessment in &packet.assessments {
        match assessment.compatibility_class {
            TargetCompatibilityClass::Compatible => assert!(
                assessment.dispatch_allowed(),
                "a compatible, preserved selection should dispatch"
            ),
            _ if !assessment.review_state.allows_dispatch() => assert!(
                !assessment.dispatch_allowed(),
                "a drifted / widened / blocked selection must not silently dispatch: {}",
                assessment.assessment_id
            ),
            _ => {}
        }
    }
}

#[test]
fn every_assessment_reconstructs_its_selection_and_fingerprint() {
    let packet = current_portable_selection_export().expect("export validates");
    for assessment in &packet.assessments {
        let selection = packet
            .selection(&assessment.selection_ref)
            .expect("assessment must reconstruct its originating selection");
        // The exact target fingerprints used for the selection are present.
        assert!(
            !selection.pinned_targets.is_empty()
                && selection
                    .pinned_targets
                    .iter()
                    .all(|t| !t.target_fingerprint_token.is_empty()),
            "support must reconstruct the target fingerprints for {}",
            assessment.assessment_id
        );
    }
}

#[test]
fn fixture_widened_review_resolves_without_silent_expansion() {
    let packet = fixture("widened_selection_review_resolves_without_silent_expansion.json");
    assert!(packet.validate().is_empty());

    let widened = packet
        .assessments
        .iter()
        .find(|a| a.compatibility_class == TargetCompatibilityClass::WidenedNeedsReview)
        .expect("a widened assessment");
    // The widening opened a review that the operator resolved by keeping the
    // original scope — so it preserves origin and may dispatch, but it was never
    // silently expanded.
    assert_eq!(
        widened.review_state,
        WidenedSelectionReviewState::RejectedKeepOriginal
    );
    assert!(!widened.added_target_ids.is_empty());
    assert!(widened.preserves_origin);
    assert!(widened.dispatch_allowed());
}

#[test]
fn fixture_imported_overlay_stays_blocked() {
    let packet = fixture("widened_selection_review_resolves_without_silent_expansion.json");
    let blocked = packet
        .assessments
        .iter()
        .find(|a| a.compatibility_class == TargetCompatibilityClass::ImportedNotRerunnable)
        .expect("imported assessment");
    assert_eq!(blocked.review_state, WidenedSelectionReviewState::Blocked);
    assert!(!blocked.dispatch_allowed());
}
