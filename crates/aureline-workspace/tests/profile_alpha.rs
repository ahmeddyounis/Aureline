//! Protected fixtures for portable profile export and sync conflict review.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_device_registry_surface, review_non_widening_import, ArtifactPortabilityLabel,
    ConflictAction, ConflictReviewPacketAlpha, ImportApplyDecision, ImportApplyRequest,
    LocalFallbackPosture, NonPortableExclusionReason, PortableArtifactClass, PortableProfileExport,
    ProfileAlphaValidationError, StateSourcePosture, SyncConflictClassAlpha,
    SyncDeviceRegistryAlphaRecord, SyncTransportState, WideningVector,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(path: &str) -> String {
    let path = repo_root().join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

#[test]
fn portable_profile_fixture_explains_keymaps_saved_views_and_exclusions() {
    let payload = read_json("fixtures/profile/alpha/portable_profile_keymap_saved_view.json");
    let export: PortableProfileExport =
        serde_json::from_str(&payload).expect("profile fixture must parse");
    export.validate().expect("profile fixture must validate");

    let artifact_classes: BTreeSet<_> = export
        .artifacts
        .iter()
        .map(|artifact| artifact.artifact_class)
        .collect();
    assert!(artifact_classes.contains(&PortableArtifactClass::Keymap));
    assert!(artifact_classes.contains(&PortableArtifactClass::SavedView));
    assert!(export
        .non_portable_exclusions
        .contains(&NonPortableExclusionReason::SecretMaterial));
    assert!(export
        .non_portable_exclusions
        .contains(&NonPortableExclusionReason::DelegatedCredential));

    let projection = export.portability_projection();
    assert!(projection.iter().any(|row| {
        row.artifact_class == PortableArtifactClass::SavedView
            && row.portability_label == ArtifactPortabilityLabel::Portable
    }));
    assert!(projection.iter().any(|row| {
        row.artifact_class == PortableArtifactClass::SavedView
            && row.portability_label == ArtifactPortabilityLabel::Downgraded
    }));
    assert!(projection.iter().any(|row| {
        row.artifact_class == PortableArtifactClass::SavedView
            && row.portability_label == ArtifactPortabilityLabel::Excluded
    }));
}

#[test]
fn portable_profile_validator_blocks_transient_saved_view_state() {
    let payload = read_json("fixtures/profile/alpha/portable_profile_keymap_saved_view.json");
    let mut export: PortableProfileExport =
        serde_json::from_str(&payload).expect("profile fixture must parse");
    let saved_view = export
        .artifacts
        .iter_mut()
        .find(|artifact| artifact.artifact_class == PortableArtifactClass::SavedView)
        .expect("fixture must contain a saved view");
    saved_view.captures_transient_selection = true;

    let err = export
        .validate()
        .expect_err("transient saved-view state must be rejected");
    assert!(matches!(
        err,
        ProfileAlphaValidationError::SavedViewCarriesForbiddenState {
            reason: NonPortableExclusionReason::TransientSelection,
            ..
        }
    ));
}

#[test]
fn device_registry_fixture_projects_identity_revision_transport_and_fallback() {
    let payload =
        read_json("fixtures/sync/device_registry_alpha/local_authoritative_fallback.json");
    let devices: Vec<SyncDeviceRegistryAlphaRecord> =
        serde_json::from_str(&payload).expect("device fixture must parse");

    let rows = project_device_registry_surface(&devices).expect("device rows must validate");
    assert_eq!(rows.len(), 3);
    assert!(rows.iter().any(|row| {
        row.device_id == "dev-travel-0002"
            && row.device_revision_ref == "device-revision:travel:0017"
            && row.transport_state == SyncTransportState::Unavailable
            && row.local_fallback_posture == LocalFallbackPosture::LocalOnlyAuthoritative
    }));
}

#[test]
fn conflict_review_fixture_covers_required_classes_and_actions() {
    let payload =
        read_json("fixtures/sync/conflict_review_alpha/keymap_and_saved_view_conflicts.json");
    let packets: Vec<ConflictReviewPacketAlpha> =
        serde_json::from_str(&payload).expect("conflict fixture must parse");

    let mut classes = BTreeSet::new();
    for packet in &packets {
        packet.validate().expect("conflict packet must validate");
        classes.insert(packet.conflict_class);

        let actions: BTreeSet<_> = packet
            .action_offers
            .iter()
            .map(|offer| offer.action)
            .collect();
        assert!(actions.contains(&ConflictAction::KeepLocal));
        assert!(actions.contains(&ConflictAction::KeepSynced));
        assert!(actions.contains(&ConflictAction::Compare));
        assert_eq!(ConflictAction::KeepLocal.display_label(), "Keep local");
        assert_eq!(ConflictAction::KeepSynced.display_label(), "Keep synced");
        assert_eq!(ConflictAction::Compare.display_label(), "Compare");
    }

    for required in [
        SyncConflictClassAlpha::SameKeyDivergence,
        SyncConflictClassAlpha::PolicyLocked,
        SyncConflictClassAlpha::MissingCapability,
        SyncConflictClassAlpha::DeleteVsModify,
        SyncConflictClassAlpha::StaleRemote,
    ] {
        assert!(classes.contains(&required), "missing {required:?}");
    }
}

#[test]
fn non_widening_import_review_blocks_trust_and_managed_owner_widening() {
    let trust_widening = ImportApplyRequest {
        source_artifact_ref: "artifact:portable-profile:daily-driver".to_string(),
        target_scope: "user_profile".to_string(),
        current_owner: StateSourcePosture::LocalOnly,
        incoming_owner: StateSourcePosture::Imported,
        widening_vectors: vec![WideningVector::WorkspaceTrust],
        narrowing_only: false,
    };
    let review = review_non_widening_import(&trust_widening);
    assert_eq!(review.decision, ImportApplyDecision::Blocked);
    assert!(!review.allowed);

    let managed_owner = ImportApplyRequest {
        source_artifact_ref: "artifact:saved-view:provider-queue".to_string(),
        target_scope: "user_profile".to_string(),
        current_owner: StateSourcePosture::LocalOnly,
        incoming_owner: StateSourcePosture::ProviderOwned,
        widening_vectors: vec![WideningVector::None],
        narrowing_only: false,
    };
    let review = review_non_widening_import(&managed_owner);
    assert_eq!(review.decision, ImportApplyDecision::Blocked);
    assert!(review
        .reasons
        .iter()
        .any(|reason| reason.contains("policy or provider ownership")));
}

#[test]
fn non_widening_import_review_allows_scope_explicit_narrowing() {
    let request = ImportApplyRequest {
        source_artifact_ref: "artifact:portable-profile:daily-driver".to_string(),
        target_scope: "user_profile".to_string(),
        current_owner: StateSourcePosture::LocalOnly,
        incoming_owner: StateSourcePosture::Imported,
        widening_vectors: vec![WideningVector::None],
        narrowing_only: true,
    };
    let review = review_non_widening_import(&request);
    assert_eq!(review.decision, ImportApplyDecision::NarrowingOnly);
    assert!(review.allowed);
    assert!(review.reasons.is_empty());
}
