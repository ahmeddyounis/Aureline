//! Protected tests for alpha release-evidence graph reconstruction.

use std::path::{Path, PathBuf};

use aureline_support::release_evidence::{
    current_alpha_artifact_graph, AlphaReleaseEvidencePacket, ALPHA_ARTIFACT_GRAPH_RECORD_KIND,
    ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND, CURRENT_ALPHA_ARTIFACT_GRAPH_PATH,
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
        || reference.starts_with("commit:")
        || reference.starts_with("support.")
        || reference.starts_with("support_")
        || reference.starts_with("release.")
        || reference.starts_with("artifact_")
        || reference.starts_with("symbolication-")
        || reference.starts_with("rollback.")
        || reference.starts_with("docs.")
        || reference.starts_with("schema.")
        || reference.starts_with("provenance.")
        || reference.starts_with("advisory_")
        || reference.starts_with("alpha_")
        || reference.starts_with("rollout_ring.")
    {
        return;
    }
    let path = reference.split('#').next().expect("split ref");
    assert!(root.join(path).exists(), "{reference} must resolve on disk");
}

fn support_packet() -> AlphaReleaseEvidencePacket {
    current_alpha_artifact_graph()
        .expect("graph parses")
        .release_evidence_packet(
            "release.evidence.alpha.seed.preview",
            "2026-05-14T07:30:00Z",
        )
}

#[test]
fn checked_in_alpha_artifact_graph_validates_and_resolves_sources() {
    let root = repo_root();
    let graph = current_alpha_artifact_graph().expect("graph parses");

    assert_eq!(graph.record_kind, ALPHA_ARTIFACT_GRAPH_RECORD_KIND);
    assert_eq!(graph.validate(), Vec::new());
    assert_eq!(
        CURRENT_ALPHA_ARTIFACT_GRAPH_PATH,
        graph.evidence_collection.default_graph_ref
    );
    assert!(graph
        .exact_build_identity_ref
        .starts_with("build-id:aureline:preview:0.8.0-alpha.1"));

    for reference in graph.source_contract_refs.values() {
        assert_repo_ref_exists(&root, reference);
    }
    for root_descriptor in &graph.build_roots {
        assert_repo_ref_exists(&root, &root_descriptor.source_ref);
    }
    for input in &graph.provenance_inputs {
        assert_repo_ref_exists(&root, &input.source_ref);
    }
    for node in graph.artifact_nodes() {
        assert_eq!(
            node.exact_build_identity_ref,
            graph.exact_build_identity_ref
        );
        assert_repo_ref_exists(&root, &node.source_ref);
        assert_repo_ref_exists(&root, &node.digest_source_ref);
    }
    for reference in &graph.acceptance.protected_fixture_refs {
        assert_repo_ref_exists(&root, reference);
    }
}

#[test]
fn support_projection_reconstructs_release_center_fields_without_log_scraping() {
    let packet = support_packet();

    assert_eq!(
        packet.record_kind,
        ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND
    );
    assert!(packet.reconstructs_required_release_fields());
    assert_eq!(packet.candidate_version, "0.8.0-alpha.1");
    assert_eq!(packet.target_class, "public_preview");
    assert_eq!(packet.rollout_ring, "design_partner_preview");
    assert_eq!(packet.auth_source_class, "ci_oidc_release_identity");
    assert_eq!(
        packet.rollback_target_ref,
        "rollback.target.preview.previous_verified_build"
    );
    assert!(packet.raw_private_material_excluded);
    assert!(packet.digest_set.len() >= 6);
    assert!(packet
        .trust_domain_refs
        .contains(&"release_preview".to_owned()));
    assert!(packet
        .trust_domain_refs
        .contains(&"protected_engineering".to_owned()));

    let digest_families = packet
        .digest_set
        .iter()
        .map(|entry| entry.family_class.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for family in [
        "ide_binary",
        "cli_binary",
        "ide_debug_symbols",
        "docs_pack",
        "schema_export",
        "support_runbook_bundle",
        "release_evidence_packet",
        "update_metadata",
    ] {
        assert!(
            digest_families.contains(family),
            "digest set missing {family}"
        );
    }
}

#[test]
fn release_center_object_seed_carries_candidate_target_timeline_and_bundle() {
    let graph = current_alpha_artifact_graph().expect("graph parses");
    let objects = &graph.release_center_objects;

    assert_eq!(objects.release_candidate_descriptors.len(), 1);
    assert_eq!(objects.publish_target_descriptors.len(), 1);
    assert_eq!(objects.promotion_timeline_descriptors.len(), 1);
    assert_eq!(objects.artifact_bundle_descriptors.len(), 1);

    let candidate = &objects.release_candidate_descriptors[0];
    let target = &objects.publish_target_descriptors[0];
    let timeline = &objects.promotion_timeline_descriptors[0];
    let bundle = &objects.artifact_bundle_descriptors[0];

    assert_eq!(
        candidate.publish_target_refs,
        vec![target.publish_target_id.clone()]
    );
    assert_eq!(
        candidate.artifact_bundle_refs,
        vec![bundle.bundle_id.clone()]
    );
    assert_eq!(timeline.candidate_ref, candidate.candidate_id);
    assert_eq!(timeline.digest_set_ref, bundle.digest_set_ref);
    assert_eq!(timeline.rollback_target_ref, target.rollback_target_ref);
    assert!(bundle
        .artifact_node_refs
        .iter()
        .any(|reference| reference.contains("schema.release_center_object")));
    assert!(bundle
        .artifact_node_refs
        .iter()
        .any(|reference| reference.contains("support.affected_build_scope")));
}
