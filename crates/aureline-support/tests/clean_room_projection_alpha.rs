//! Protected tests for clean-room rebuild projection honesty.

use std::path::{Path, PathBuf};

use aureline_support::publication_dry_run::{
    current_alpha_publication_manifest, current_clean_room_rebuild_projection,
    current_publication_rehearsal_methodology, CLEAN_ROOM_REBUILD_PROJECTION_RECORD_KIND,
    CURRENT_ALPHA_KNOWN_LIMITS_PATH, CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH,
    CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_PATH,
    PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn assert_repo_ref_exists(root: &Path, reference: &str) {
    if reference.starts_with("build-id:")
        || reference.starts_with("clean_room_rebuild_projection.")
        || reference.starts_with("known_limit:")
        || reference.starts_with("publication_rehearsal.")
    {
        return;
    }
    let path = reference.split('#').next().expect("split ref");
    assert!(root.join(path).exists(), "{reference} must resolve on disk");
}

#[test]
fn rehearsal_methodology_validates_against_methodology_only_known_limit() {
    let methodology = current_publication_rehearsal_methodology().expect("methodology parses");
    let projection = current_clean_room_rebuild_projection().expect("projection validates");

    assert!(methodology.is_methodology_only());
    assert_eq!(methodology.packet_state, "methodology_only");
    assert!(methodology
        .known_limit_refs
        .contains(&PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID.to_owned()));
    assert!(methodology
        .bundle_bindings
        .iter()
        .all(|binding| binding.publication_result == "keep_methodology_only"));
    assert_eq!(
        projection.methodology_only_known_limit_ref,
        PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID
    );
    assert_eq!(projection.known_limit_note_state, "active");
}

#[test]
fn clean_room_projection_never_claims_rebuild_execution() {
    let projection = current_clean_room_rebuild_projection().expect("projection validates");

    assert_eq!(projection.validate(), Vec::new());
    assert_eq!(
        projection.record_kind,
        CLEAN_ROOM_REBUILD_PROJECTION_RECORD_KIND
    );
    assert_eq!(
        projection.projection_state,
        "alpha_methodology_only_projection"
    );
    assert!(projection.projection_only);
    assert!(!projection.actual_clean_room_rebuild_executed);
    assert_eq!(
        projection.rebuild_execution_state,
        "not_executed_methodology_projection"
    );
    assert!(projection.respects_methodology_only_known_limit());

    for non_claim in [
        "actual_clean_room_rebuild_execution",
        "public_head_to_head_comparison",
        "published_performance_claim",
        "certified_or_replacement_grade_public_wording",
    ] {
        assert!(
            projection
                .explicit_non_claims
                .contains(&non_claim.to_owned()),
            "projection must explicitly avoid {non_claim}"
        );
    }
}

#[test]
fn projection_carries_publication_manifest_coverage_and_resolvable_sources() {
    let root = repo_root();
    let manifest = current_alpha_publication_manifest().expect("manifest parses");
    let projection = current_clean_room_rebuild_projection().expect("projection validates");
    let support_projection = manifest.support_projection();

    assert_eq!(
        projection.projected_artifact_family_keys,
        support_projection.required_family_keys
    );
    assert_eq!(
        projection.publication_posture_classes,
        support_projection.posture_classes
    );
    assert_eq!(projection.receipt_count, support_projection.receipt_count);
    assert_eq!(
        projection.blocking_blocker_count,
        support_projection.blocking_blocker_count
    );

    for required_source in [
        CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH,
        CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_PATH,
        CURRENT_ALPHA_KNOWN_LIMITS_PATH,
    ] {
        assert!(
            projection
                .methodology_source_refs
                .contains(&required_source.to_owned()),
            "projection missing source {required_source}"
        );
    }
    for reference in &projection.methodology_source_refs {
        assert_repo_ref_exists(&root, reference);
    }
}
