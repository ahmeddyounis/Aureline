//! Unit tests for the canonical hover-card and documentation-peek model.

use super::*;

#[test]
fn model_builds_and_every_invariant_holds() {
    let model = hover_peek_model();
    assert_eq!(model.record_kind, M5_HOVER_PEEK_RECORD_KIND);
    assert_eq!(model.schema_ref, M5_HOVER_PEEK_SCHEMA_REF);
    assert_eq!(model.model_id, M5_HOVER_PEEK_MODEL_ID);
    assert!(
        model.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        model
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(model.is_support_export_safe());
    assert!(model.raw_payload_excluded);
}

#[test]
fn model_serialization_round_trips() {
    let model = hover_peek_model();
    let json = serde_json::to_string(&model).expect("model serializes");
    let restored: HoverPeekModel = serde_json::from_str(&json).expect("model round-trips");
    assert_eq!(model, restored);
}

#[test]
fn every_claimed_context_is_present() {
    let model = hover_peek_model();
    for context in HoverPeekContextClass::ALL {
        assert!(
            model.snapshot(context).is_some(),
            "missing snapshot for {}",
            context.as_str()
        );
    }
    assert_eq!(
        model.context_snapshots.len(),
        HoverPeekContextClass::ALL.len()
    );
}

#[test]
fn every_card_is_keyboard_invocable_and_provenance_labeled() {
    let model = hover_peek_model();
    for card in model.all_cards() {
        assert!(
            card.keyboard_invocable,
            "card {} must be keyboard invocable",
            card.card_id
        );
        assert!(!card.keyboard_command_id_ref.trim().is_empty());
        assert!(
            card.provenance_labeled(),
            "card {} must carry visible provenance",
            card.card_id
        );
    }
}

#[test]
fn cards_never_silently_retarget() {
    let model = hover_peek_model();
    for card in model.all_cards() {
        assert!(!card.retarget_on_later_provider);
        assert!(card.target.identity_locked);
        assert!(card.target.is_coherent());
        assert!(card.target_identity_locked());
    }
}

#[test]
fn non_live_states_are_disclosed_inline() {
    let model = hover_peek_model();
    for card in model.all_cards() {
        if !card.is_live() {
            assert!(
                card.inline_state_disclosed,
                "non-live card {} must disclose its state inline",
                card.card_id
            );
            assert!(!card.non_color_differentiator.trim().is_empty());
            assert!(card.non_live_state_disclosed());
        }
    }
}

#[test]
fn wrong_provider_fallback_is_not_styled_live() {
    let model = hover_peek_model();
    let sql = model
        .snapshot(HoverPeekContextClass::SqlEditor)
        .expect("sql");
    let card = &sql.card;
    assert_eq!(card.state_class, HoverPeekStateClass::WrongProviderFallback);
    assert!(!card.is_live());
    assert!(card.non_live_state_disclosed());
    assert!(card.source.provider_id.is_none());
    assert_ne!(sql.degrade_class, AssistDegradeClass::FullFidelity);
}

#[test]
fn raw_and_rendered_distinct_offers_open_raw_escape() {
    let model = hover_peek_model();
    let request = model
        .snapshot(HoverPeekContextClass::RequestEditor)
        .expect("request");
    let card = &request.card;
    assert_eq!(
        card.raw_rendered_mode,
        RawRenderedModeClass::RawAndRenderedDistinct
    );
    assert!(card.raw_rendered_mode.materially_differs());
    assert!(card
        .raw_escape_command_id_ref
        .as_ref()
        .is_some_and(|command| !command.trim().is_empty()));
    assert!(card.raw_form_summary.is_some());
    assert!(card.rendered_form_summary.is_some());
    assert!(card.raw_escape_when_distinct());
}

#[test]
fn equivalent_raw_rendered_needs_no_escape() {
    let model = hover_peek_model();
    let config = model
        .snapshot(HoverPeekContextClass::ConfigFile)
        .expect("config");
    let card = &config.card;
    assert_eq!(
        card.raw_rendered_mode,
        RawRenderedModeClass::RawAndRenderedEquivalent
    );
    assert!(!card.raw_rendered_mode.materially_differs());
    assert!(card.raw_escape_command_id_ref.is_none());
    assert!(card.raw_escape_when_distinct());
}

#[test]
fn pinned_and_promoted_cards_preserve_provenance_and_return_anchor() {
    let model = hover_peek_model();
    for card in model.all_cards() {
        if card.presentation_class.is_persisted() {
            assert!(
                card.provenance_labeled(),
                "persisted card {} must keep provenance visible",
                card.card_id
            );
            assert!(!card.source.source_label.trim().is_empty());
        }
        // Promotions always preserve provenance and the return anchor.
        assert!(card.promotions_preserve_provenance_and_continuity());
        for promotion in &card.promotions {
            assert_eq!(
                promotion.source_descriptor_id_ref,
                card.source.source_descriptor_id
            );
            assert_eq!(promotion.return_anchor_ref, card.target.return_anchor_ref);
        }
    }
}

#[test]
fn content_cards_offer_all_promotion_paths() {
    let model = hover_peek_model();
    for card in model.all_cards() {
        if card.offers_content() {
            for path in PeekPromotionPathClass::ALL {
                assert!(
                    card.promotions
                        .iter()
                        .any(|promotion| promotion.path_class == path),
                    "content card {} must offer {}",
                    card.card_id,
                    path.as_str()
                );
            }
        }
        assert!(card.offers_all_promotion_paths());
    }
}

#[test]
fn promoted_split_notebook_keeps_its_source() {
    let model = hover_peek_model();
    let notebook = model
        .snapshot(HoverPeekContextClass::NotebookCell)
        .expect("notebook");
    let card = &notebook.card;
    assert_eq!(
        card.presentation_class,
        HoverPeekPresentationClass::PromotedSplit
    );
    assert!(card.presentation_class.is_durable());
    assert_eq!(card.source.source_label, "Pyright");
    assert!(card.provenance_labeled());
}

#[test]
fn diff_review_and_graph_contexts_are_covered() {
    let model = hover_peek_model();
    let review = model
        .snapshot(HoverPeekContextClass::DiffReviewSurface)
        .expect("review");
    assert_eq!(
        review.card.presentation_class,
        HoverPeekPresentationClass::PromotedTab
    );
    assert!(review.base_editor_surface.is_none());

    let graph = model
        .snapshot(HoverPeekContextClass::GraphLinkedExplainer)
        .expect("graph");
    assert_eq!(graph.card.mode_class, HoverPeekModeClass::PeekCallHierarchy);
    assert!(graph.card.mode_class.is_peek());
    assert!(graph.base_editor_surface.is_none());
}

#[test]
fn large_file_suppresses_card_but_keeps_it_reachable() {
    let model = hover_peek_model();
    let large = model
        .snapshot(HoverPeekContextClass::LargeFileRestricted)
        .expect("large");
    let card = &large.card;
    assert_eq!(card.state_class, HoverPeekStateClass::Suppressed);
    assert!(!card.offers_content());
    assert!(card.keyboard_invocable);
    assert!(card.inline_state_disclosed);
    assert!(!card.accessibility_label.trim().is_empty());
    // A suppressed card has no content to promote into a durable surface, but still
    // returns to its anchor.
    assert!(card
        .promotions
        .iter()
        .all(|promotion| promotion.path_class == PeekPromotionPathClass::DismissReturn));
}

#[test]
fn imported_snapshot_generated_peek_is_disclosed() {
    let model = hover_peek_model();
    let generated = model
        .snapshot(HoverPeekContextClass::GeneratedFile)
        .expect("generated");
    let card = &generated.card;
    assert_eq!(card.state_class, HoverPeekStateClass::ImportedSnapshot);
    assert!(!card.is_live());
    assert!(card.non_live_state_disclosed());
    assert_eq!(generated.degrade_class, AssistDegradeClass::ReadOnlyNoApply);
}

#[test]
fn shared_contexts_reuse_editor_surface_vocabulary() {
    let model = hover_peek_model();
    let mut surfaces: Vec<EditorSurfaceClass> = model
        .context_snapshots
        .iter()
        .filter_map(|snapshot| snapshot.base_editor_surface)
        .collect();
    let total = surfaces.len();
    surfaces.sort_unstable();
    surfaces.dedup();
    assert_eq!(surfaces.len(), total, "base surfaces must be distinct");
    assert_eq!(total, EditorSurfaceClass::ALL.len());
}

#[test]
fn class_catalogs_have_unique_tokens() {
    let model = hover_peek_model();
    for catalog in [
        &model.mode_classes,
        &model.context_classes,
        &model.state_classes,
        &model.mapping_quality_classes,
        &model.raw_rendered_classes,
        &model.promotion_path_classes,
        &model.presentation_classes,
    ] {
        let mut tokens: Vec<&str> = catalog
            .iter()
            .map(|descriptor| descriptor.class_token.as_str())
            .collect();
        let total = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), total, "catalog tokens must be unique");
    }
    assert_eq!(
        model.context_classes.len(),
        HoverPeekContextClass::ALL.len()
    );
    assert_eq!(model.state_classes.len(), HoverPeekStateClass::ALL.len());
    assert_eq!(
        model.promotion_path_classes.len(),
        PeekPromotionPathClass::ALL.len()
    );
}

#[test]
fn lines_projection_renders_every_section() {
    let model = hover_peek_model();
    let lines = hover_peek_model_lines(&model);
    assert!(lines.iter().any(|line| line.contains("Hover-peek model")));
    assert!(lines.iter().any(|line| line.contains("Context snapshots:")));
    assert!(lines.iter().any(|line| line.contains("Invariants:")));
    for snapshot in &model.context_snapshots {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(snapshot.context_class.as_str())),
            "lines must mention context {}",
            snapshot.context_class.as_str()
        );
    }
}
