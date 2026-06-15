use super::*;

use crate::m5_author_and_publish_preview::current_m5_author_publish_matrix;

fn board() -> M5ReloadContinuityBoard {
    current_m5_reload_continuity_board().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let board = board();
    assert_eq!(board.schema_version, M5_RELOAD_CONTINUITY_SCHEMA_VERSION);
    assert_eq!(board.record_kind, M5_RELOAD_CONTINUITY_RECORD_KIND);
    assert_eq!(board.validate(), Vec::new());
}

#[test]
fn summary_counts_match_cards() {
    let board = board();
    assert_eq!(board.summary, board.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_card() {
    let board = board();
    assert_eq!(board.cards.len(), board.artifact_families.len());
    for &family in &board.artifact_families {
        assert!(
            board.card(family).is_some(),
            "missing card for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_card_is_internally_consistent() {
    let board = board();
    assert!(board.all_cards_consistent());
    for card in &board.cards {
        assert!(
            card.card_consistent(),
            "card {} is internally inconsistent",
            card.card_id
        );
        assert_eq!(
            card.continuity_state,
            card.computed_continuity_state(),
            "card {} continuity drifted",
            card.card_id
        );
        assert_eq!(card.restart_scope, card.computed_restart_scope());
        assert_eq!(card.preserved_state, card.computed_preserved_state());
        assert_eq!(card.widening_review, card.computed_widening_review());
        assert_eq!(card.rollback_path, card.computed_rollback_path());
    }
}

#[test]
fn local_dev_and_sideload_and_unsigned_render_local_only() {
    let board = board();
    for card in &board.cards {
        if card.origin.caps_to_local_only() || card.signature_state.is_local_or_untrusted() {
            assert!(
                card.is_local_only(),
                "card {} must render local-only",
                card.card_id
            );
            assert_eq!(card.rendered_trust_posture, TrustPosture::UnsignedLocalOnly);
        }
    }
}

#[test]
fn signed_package_in_local_dev_workspace_does_not_inherit_trust() {
    // A signed-verified artifact in a local-dev workspace renders local-only: a reload
    // never inherits a verified badge just because the machine holds a trusted key.
    let board = board();
    let card = board
        .card(ArtifactFamily::SignedRecipePack)
        .expect("signed recipe pack card");
    assert_eq!(card.origin, WorkspaceOrigin::LocalDevWorkspace);
    assert_eq!(card.signature_state, SignatureState::SignedVerified);
    assert_eq!(card.declared_trust_posture, TrustPosture::VerifiedPublisher);
    assert_eq!(card.rendered_trust_posture, TrustPosture::UnsignedLocalOnly);
    assert!(card.is_local_only());
}

#[test]
fn widening_hot_reload_holds_instance_for_review() {
    let board = board();
    for card in &board.cards {
        if hot_reload_widens_authority(card.hot_reload_posture) {
            assert!(
                card.requires_fresh_review(),
                "card {} should require a fresh review",
                card.card_id
            );
            assert_eq!(
                card.load_state,
                LoadState::ReloadHeldForReview,
                "card {} must hold its instance for review",
                card.card_id
            );
            assert_eq!(card.restart_scope, RestartScope::HeldPendingReview);
            assert!(card.widening_review.requires_review());
        } else {
            assert_ne!(
                card.load_state,
                LoadState::ReloadHeldForReview,
                "card {} holds for review without a widening hot reload",
                card.card_id
            );
        }
    }
    assert!(board.held_pending_review_cards().count() >= 1);
}

#[test]
fn banner_explains_preserved_versus_restarted_state() {
    let board = board();
    for card in &board.cards {
        match card.restart_scope {
            RestartScope::NothingRestarts => {
                assert_eq!(
                    card.preserved_state,
                    PreservedState::InMemoryAndPersistedPreserved
                );
            }
            RestartScope::HostInstanceRelaunches => {
                assert_eq!(
                    card.preserved_state,
                    PreservedState::PersistedPreservedInMemoryReset
                );
            }
            RestartScope::HeldPendingReview => {
                assert_eq!(
                    card.preserved_state,
                    PreservedState::RunningInstanceUnchanged
                );
            }
            RestartScope::NoRunningInstance => {
                assert_eq!(card.preserved_state, PreservedState::NoRunningState);
            }
        }
    }
}

#[test]
fn packages_degrade_rather_than_disappear() {
    // Source-moved/unavailable or build-failed packages keep a card with an explicit
    // degraded continuity state; they never drop off the board.
    let board = board();
    let source_unavailable = board
        .card(ArtifactFamily::MirroredRegistryVariant)
        .expect("mirrored variant card");
    assert_eq!(
        source_unavailable.source_availability,
        SourceAvailability::SourceUnavailable
    );
    assert_eq!(
        source_unavailable.continuity_state,
        ContinuityState::SourceUnavailable
    );

    let build_failed = board
        .card(ArtifactFamily::TemplateArtifact)
        .expect("template artifact card");
    assert_eq!(build_failed.build_freshness, BuildFreshness::BuildFailed);
    assert_eq!(build_failed.continuity_state, ContinuityState::BuildFailed);

    let still_active = board
        .card(ArtifactFamily::SideLoadedPackage)
        .expect("side-loaded package card");
    assert_eq!(
        still_active.source_availability,
        SourceAvailability::SourceMoved
    );
    assert_eq!(
        still_active.continuity_state,
        ContinuityState::LastLoadedBuildStillActive
    );

    // Every claimed family is still represented.
    assert_eq!(board.cards.len(), board.artifact_families.len());
    assert!(board.degraded_cards().count() >= 3);
}

#[test]
fn last_loaded_build_record_is_retained_when_active() {
    // A running instance serving a last-loaded build never loses its continuity record.
    let board = board();
    for card in &board.cards {
        if card.continuity_state == ContinuityState::LastLoadedBuildStillActive {
            assert!(
                card.continuity_record_retained(),
                "card {} lost its last-loaded-build record",
                card.card_id
            );
            assert_eq!(card.rollback_path, RollbackPath::RevertToLastLoadedBuild);
        }
    }
}

#[test]
fn loaded_current_build_claims_are_honest() {
    let board = board();
    for card in &board.cards {
        if card.load_state == LoadState::LoadedCurrentBuild {
            assert_eq!(card.build_freshness, BuildFreshness::BuiltFromCurrentSource);
            assert_eq!(card.source_availability, SourceAvailability::SourcePresent);
            assert_eq!(card.continuity_state, ContinuityState::LoadedCurrentBuild);
        }
    }
}

#[test]
fn every_closed_vocabulary_is_exercised() {
    let board = board();
    for origin in WorkspaceOrigin::ALL {
        assert!(
            board.cards.iter().any(|c| c.origin == origin),
            "origin {} unexercised",
            origin.as_str()
        );
    }
    for runtime in RuntimeClass::ALL {
        assert!(
            board.cards.iter().any(|c| c.runtime_class == runtime),
            "runtime {} unexercised",
            runtime.as_str()
        );
    }
    for host in HostAbiClass::ALL {
        assert!(
            board.cards.iter().any(|c| c.host_abi == host),
            "host {} unexercised",
            host.as_str()
        );
    }
    for sig in SignatureState::ALL {
        assert!(
            board.cards.iter().any(|c| c.signature_state == sig),
            "signature {} unexercised",
            sig.as_str()
        );
    }
    for posture in TrustPosture::ALL {
        assert!(
            board
                .cards
                .iter()
                .any(|c| c.rendered_trust_posture == posture),
            "rendered trust {} unexercised",
            posture.as_str()
        );
    }
    for freshness in BuildFreshness::ALL {
        assert!(
            board.cards.iter().any(|c| c.build_freshness == freshness),
            "build freshness {} unexercised",
            freshness.as_str()
        );
    }
    for load in LoadState::ALL {
        assert!(
            board.cards.iter().any(|c| c.load_state == load),
            "load state {} unexercised",
            load.as_str()
        );
    }
    for reload in HotReloadPosture::ALL {
        assert!(
            board.cards.iter().any(|c| c.hot_reload_posture == reload),
            "hot-reload posture {} unexercised",
            reload.as_str()
        );
    }
    for source in SourceAvailability::ALL {
        assert!(
            board.cards.iter().any(|c| c.source_availability == source),
            "source availability {} unexercised",
            source.as_str()
        );
    }
    for state in ContinuityState::ALL {
        assert!(
            board.cards.iter().any(|c| c.continuity_state == state),
            "continuity state {} unexercised",
            state.as_str()
        );
    }
    for scope in RestartScope::ALL {
        assert!(
            board.cards.iter().any(|c| c.restart_scope == scope),
            "restart scope {} unexercised",
            scope.as_str()
        );
    }
    for preserved in PreservedState::ALL {
        assert!(
            board.cards.iter().any(|c| c.preserved_state == preserved),
            "preserved state {} unexercised",
            preserved.as_str()
        );
    }
    for review in WideningReview::ALL {
        assert!(
            board.cards.iter().any(|c| c.widening_review == review),
            "widening review {} unexercised",
            review.as_str()
        );
    }
    for rollback in RollbackPath::ALL {
        assert!(
            board.cards.iter().any(|c| c.rollback_path == rollback),
            "rollback path {} unexercised",
            rollback.as_str()
        );
    }
}

#[test]
fn cross_check_matrix_agrees_with_publish_gate() {
    let board = board();
    let matrix = current_m5_author_publish_matrix().expect("matrix parses");
    assert_eq!(board.cross_check_matrix(&matrix), Vec::new());
    for card in &board.cards {
        let row = matrix
            .family(card.artifact_family)
            .expect("matrix row for family");
        assert!(
            card.rendered_trust_posture.rank() <= row.published_trust_posture.rank(),
            "card {} renders stronger than the gate",
            card.card_id
        );
    }
}

#[test]
fn export_projection_reflects_the_board() {
    let board = board();
    let projection = board.export_projection();
    assert_eq!(projection.packet_id, board.packet_id);
    assert_eq!(projection.cards.len(), board.cards.len());
    assert!(projection.all_cards_consistent);
    assert_eq!(
        projection.local_only_count,
        board.local_only_cards().count()
    );
    assert_eq!(
        projection.held_pending_review_count,
        board.held_pending_review_cards().count()
    );
    assert_eq!(projection.degraded_count, board.degraded_cards().count());
    for (row, card) in projection.cards.iter().zip(&board.cards) {
        assert_eq!(row.card_id, card.card_id);
        assert_eq!(row.local_only, card.is_local_only());
        assert_eq!(row.requires_fresh_review, card.requires_fresh_review());
        assert_eq!(row.continuity_state, card.continuity_state.as_str());
        assert_eq!(row.rollback_path, card.rollback_path.as_str());
        assert!(row.banner.contains(card.continuity_state.as_str()));
    }
}

#[test]
fn overstated_rendered_trust_is_flagged() {
    let mut board = board();
    let card = board
        .cards
        .iter_mut()
        .find(|c| c.artifact_family == ArtifactFamily::LocalModelPack)
        .expect("local model card");
    card.rendered_trust_posture = TrustPosture::VerifiedPublisher;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ReloadContinuityViolation::RenderedTrustOverstated { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ReloadContinuityViolation::LocalPackageInheritedTrust { .. }
    )));
}

#[test]
fn silent_hot_reload_widening_is_flagged() {
    let mut board = board();
    // Let a widening hot reload run without holding the instance for review.
    let card = board
        .cards
        .iter_mut()
        .find(|c| c.load_state == LoadState::ReloadHeldForReview)
        .expect("a held card");
    card.load_state = LoadState::LoadedCurrentBuild;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ReloadContinuityViolation::HotReloadWideningNotHeld { .. }
    )));
}

#[test]
fn losing_the_last_loaded_record_is_flagged() {
    let mut board = board();
    // Drop the continuity record of a card whose last-loaded build is still active.
    let card = board
        .cards
        .iter_mut()
        .find(|c| c.continuity_state == ContinuityState::LastLoadedBuildStillActive)
        .expect("an active card");
    card.last_loaded_build_ref = None;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ReloadContinuityViolation::LastLoadedRecordMissing { .. }
    )));
}

#[test]
fn dishonest_loaded_current_build_is_flagged() {
    let mut board = board();
    // Claim loaded-current-build while the source is gone.
    let card = board
        .cards
        .iter_mut()
        .find(|c| c.load_state == LoadState::LoadedCurrentBuild)
        .expect("a loaded card");
    card.source_availability = SourceAvailability::SourceUnavailable;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ReloadContinuityViolation::LoadedCurrentBuildInconsistent { .. }
    )));
}

#[test]
fn card_exceeding_publish_gate_is_flagged() {
    let mut board = board();
    let matrix = current_m5_author_publish_matrix().expect("matrix parses");
    let card = board
        .cards
        .iter_mut()
        .find(|c| c.artifact_family == ArtifactFamily::SideLoadedPackage)
        .expect("side-loaded card");
    card.rendered_trust_posture = TrustPosture::EnterpriseApproved;
    let violations = board.cross_check_matrix(&matrix);
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ReloadContinuityViolation::CardExceedsPublishGate { .. }
    )));
}
