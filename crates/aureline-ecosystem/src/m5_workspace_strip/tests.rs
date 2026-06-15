use super::*;

use crate::m5_author_and_publish_preview::current_m5_author_publish_matrix;

fn board() -> M5LocalWorkspaceStripBoard {
    current_m5_workspace_strip_board().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let board = board();
    assert_eq!(board.schema_version, M5_WORKSPACE_STRIP_SCHEMA_VERSION);
    assert_eq!(board.record_kind, M5_WORKSPACE_STRIP_RECORD_KIND);
    assert_eq!(board.validate(), Vec::new());
}

#[test]
fn summary_counts_match_strips() {
    let board = board();
    assert_eq!(board.summary, board.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_strip() {
    let board = board();
    assert_eq!(board.strips.len(), board.artifact_families.len());
    for &family in &board.artifact_families {
        assert!(
            board.strip(family).is_some(),
            "missing strip for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_strip_is_internally_consistent() {
    let board = board();
    assert!(board.all_strips_consistent());
    for strip in &board.strips {
        assert_eq!(
            strip.rendered_trust_posture,
            strip.effective_trust_posture(),
            "strip {} renders beyond its signing/origin ceiling",
            strip.strip_id
        );
        assert!(
            strip.strip_consistent(),
            "strip {} is internally inconsistent",
            strip.strip_id
        );
    }
}

#[test]
fn local_dev_and_sideload_and_unsigned_render_local_only() {
    let board = board();
    for strip in &board.strips {
        if strip.origin.caps_to_local_only() || strip.signature_state.is_local_or_untrusted() {
            assert!(
                strip.is_local_only(),
                "strip {} must render local-only",
                strip.strip_id
            );
            assert_eq!(
                strip.rendered_trust_posture,
                TrustPosture::UnsignedLocalOnly
            );
        }
    }
}

#[test]
fn signed_package_in_local_dev_workspace_does_not_inherit_trust() {
    // A signed-verified artifact in a local-dev workspace renders local-only: a local
    // build never inherits a verified badge just because the machine holds a trusted key.
    let board = board();
    let strip = board
        .strip(ArtifactFamily::SignedRecipePack)
        .expect("signed recipe pack strip");
    assert_eq!(strip.origin, WorkspaceOrigin::LocalDevWorkspace);
    assert_eq!(strip.signature_state, SignatureState::SignedVerified);
    assert_eq!(
        strip.declared_trust_posture,
        TrustPosture::VerifiedPublisher
    );
    assert_eq!(
        strip.rendered_trust_posture,
        TrustPosture::UnsignedLocalOnly
    );
    assert!(strip.is_local_only());
}

#[test]
fn revoked_mirror_backed_artifact_renders_local_only() {
    let board = board();
    let strip = board
        .strip(ArtifactFamily::MirroredRegistryVariant)
        .expect("mirrored variant strip");
    assert_eq!(strip.origin, WorkspaceOrigin::MirrorBacked);
    assert_eq!(strip.signature_state, SignatureState::RevokedSignature);
    assert_eq!(
        strip.rendered_trust_posture,
        TrustPosture::UnsignedLocalOnly
    );
}

#[test]
fn widening_hot_reload_holds_instance_for_review() {
    let board = board();
    for strip in &board.strips {
        if hot_reload_widens_authority(strip.hot_reload_posture) {
            assert!(
                strip.requires_fresh_review(),
                "strip {} should require a fresh review",
                strip.strip_id
            );
            assert_eq!(
                strip.load_state,
                LoadState::ReloadHeldForReview,
                "strip {} must hold its instance for review",
                strip.strip_id
            );
        } else {
            assert_ne!(
                strip.load_state,
                LoadState::ReloadHeldForReview,
                "strip {} holds for review without a widening hot reload",
                strip.strip_id
            );
        }
    }
    assert!(board.reload_held_strips().count() >= 1);
}

#[test]
fn strips_distinguish_local_only_from_published_and_mirror() {
    let board = board();
    assert!(board.local_only_strips().count() >= 1);
    assert!(board
        .strips
        .iter()
        .any(|s| s.origin == WorkspaceOrigin::PublishedRegistryBacked));
    assert!(board
        .strips
        .iter()
        .any(|s| s.origin == WorkspaceOrigin::MirrorBacked));
    // Mirror-backed and published strips are not local-only unless their signing state
    // forces it.
    for strip in board.published_or_mirror_strips() {
        if !strip.signature_state.is_local_or_untrusted() {
            assert!(
                !strip.is_local_only(),
                "published/mirror strip {} unexpectedly local-only",
                strip.strip_id
            );
        }
    }
}

#[test]
fn build_and_load_states_are_coherent() {
    let board = board();
    for strip in &board.strips {
        if strip.load_state.needs_loadable_build() {
            assert!(
                strip.build_freshness.is_loadable(),
                "strip {} loaded without a loadable build",
                strip.strip_id
            );
        }
        if strip.load_state == LoadState::LoadedCurrentBuild {
            assert_eq!(
                strip.build_freshness,
                BuildFreshness::BuiltFromCurrentSource
            );
        }
    }
}

#[test]
fn every_closed_vocabulary_is_exercised() {
    let board = board();
    for origin in WorkspaceOrigin::ALL {
        assert!(
            board.strips.iter().any(|s| s.origin == origin),
            "origin {} unexercised",
            origin.as_str()
        );
    }
    for runtime in RuntimeClass::ALL {
        assert!(
            board.strips.iter().any(|s| s.runtime_class == runtime),
            "runtime {} unexercised",
            runtime.as_str()
        );
    }
    for host in HostAbiClass::ALL {
        assert!(
            board.strips.iter().any(|s| s.host_abi == host),
            "host {} unexercised",
            host.as_str()
        );
    }
    for sig in SignatureState::ALL {
        assert!(
            board.strips.iter().any(|s| s.signature_state == sig),
            "signature {} unexercised",
            sig.as_str()
        );
    }
    for posture in TrustPosture::ALL {
        assert!(
            board
                .strips
                .iter()
                .any(|s| s.rendered_trust_posture == posture),
            "rendered trust {} unexercised",
            posture.as_str()
        );
    }
    for freshness in BuildFreshness::ALL {
        assert!(
            board.strips.iter().any(|s| s.build_freshness == freshness),
            "build freshness {} unexercised",
            freshness.as_str()
        );
    }
    for load in LoadState::ALL {
        assert!(
            board.strips.iter().any(|s| s.load_state == load),
            "load state {} unexercised",
            load.as_str()
        );
    }
}

#[test]
fn cross_check_matrix_agrees_with_publish_gate() {
    let board = board();
    let matrix = current_m5_author_publish_matrix().expect("matrix parses");
    assert_eq!(board.cross_check_matrix(&matrix), Vec::new());
    // Every strip renders no stronger than the gate would publish.
    for strip in &board.strips {
        let row = matrix
            .family(strip.artifact_family)
            .expect("matrix row for family");
        assert!(
            strip.rendered_trust_posture.rank() <= row.published_trust_posture.rank(),
            "strip {} renders stronger than the gate",
            strip.strip_id
        );
    }
}

#[test]
fn export_projection_reflects_the_board() {
    let board = board();
    let projection = board.export_projection();
    assert_eq!(projection.packet_id, board.packet_id);
    assert_eq!(projection.strips.len(), board.strips.len());
    assert!(projection.all_strips_consistent);
    assert_eq!(
        projection.local_only_count,
        board.local_only_strips().count()
    );
    assert_eq!(
        projection.reload_held_count,
        board.reload_held_strips().count()
    );
    for (row, strip) in projection.strips.iter().zip(&board.strips) {
        assert_eq!(row.strip_id, strip.strip_id);
        assert_eq!(row.local_only, strip.is_local_only());
        assert_eq!(row.requires_fresh_review, strip.requires_fresh_review());
        assert_eq!(
            row.rendered_trust_posture,
            strip.rendered_trust_posture.as_str()
        );
    }
}

#[test]
fn overstated_rendered_trust_is_flagged() {
    let mut board = board();
    // Force a local-dev strip to claim a verified-publisher badge.
    let strip = board
        .strips
        .iter_mut()
        .find(|s| s.artifact_family == ArtifactFamily::LocalModelPack)
        .expect("local model strip");
    strip.rendered_trust_posture = TrustPosture::VerifiedPublisher;
    let violations = board.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5WorkspaceStripViolation::RenderedTrustOverstated { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5WorkspaceStripViolation::LocalWorkspaceInheritedTrust { .. }
    )));
}

#[test]
fn silent_hot_reload_widening_is_flagged() {
    let mut board = board();
    // Let a widening hot reload run without holding the instance for review.
    let strip = board
        .strips
        .iter_mut()
        .find(|s| s.load_state == LoadState::ReloadHeldForReview)
        .expect("a held strip");
    strip.load_state = LoadState::LoadedCurrentBuild;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5WorkspaceStripViolation::HotReloadWideningNotHeld { .. }
    )));
}

#[test]
fn strip_exceeding_publish_gate_is_flagged() {
    let mut board = board();
    let matrix = current_m5_author_publish_matrix().expect("matrix parses");
    // Render a local-only family stronger than the gate would publish.
    let strip = board
        .strips
        .iter_mut()
        .find(|s| s.artifact_family == ArtifactFamily::SideLoadedPackage)
        .expect("side-loaded strip");
    strip.rendered_trust_posture = TrustPosture::EnterpriseApproved;
    let violations = board.cross_check_matrix(&matrix);
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5WorkspaceStripViolation::StripExceedsPublishGate { .. })));
}
