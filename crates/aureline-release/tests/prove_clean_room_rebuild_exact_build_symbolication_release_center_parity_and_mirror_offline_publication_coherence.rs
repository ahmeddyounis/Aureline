//! Contract tests for the checked-in clean-room rebuild proof artifact.

use aureline_release::prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence::{
    current_clean_room_rebuild_proof, ChannelFamilyKind, CLEAN_ROOM_REBUILD_PROOF_RECORD_KIND,
    CLEAN_ROOM_REBUILD_PROOF_SCHEMA_VERSION,
};

fn artifact() -> aureline_release::prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence::CleanRoomRebuildProof{
    current_clean_room_rebuild_proof().expect("checked-in artifact parses into the model")
}

#[test]
fn checked_in_artifact_parses_and_validates() {
    let artifact = artifact();
    assert_eq!(
        artifact.schema_version,
        CLEAN_ROOM_REBUILD_PROOF_SCHEMA_VERSION
    );
    assert_eq!(artifact.record_kind, CLEAN_ROOM_REBUILD_PROOF_RECORD_KIND);
    let violations = artifact.validate();
    assert!(
        violations.is_empty(),
        "checked-in artifact must validate cleanly: {violations:#?}"
    );
}

#[test]
fn artifact_covers_holding_and_narrowed_rows() {
    let artifact = artifact();
    assert_eq!(artifact.rows_holding_claim().len(), 12);
    assert_eq!(artifact.rows_narrowed().len(), 3);
    assert_eq!(artifact.computed_summary().rows_on_active_waiver, 1);
}

#[test]
fn every_family_kind_has_a_row() {
    let artifact = artifact();
    let present: std::collections::BTreeSet<ChannelFamilyKind> =
        artifact.rows.iter().map(|row| row.family_kind).collect();
    for kind in ChannelFamilyKind::ALL {
        assert!(
            present.contains(&kind),
            "missing family kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn publication_decision_matches_computed() {
    let artifact = artifact();
    assert_eq!(
        artifact.publication.decision,
        artifact.computed_publication_decision()
    );
    assert_eq!(
        artifact.publication.blocking_rule_ids,
        artifact.computed_blocking_rule_ids()
    );
    assert_eq!(
        artifact.publication.blocking_row_ids,
        artifact.computed_blocking_row_ids()
    );
}
