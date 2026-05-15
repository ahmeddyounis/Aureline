//! Alpha tests for build-farm provenance-chain validation.

use aureline_build_farm::{
    ProvenanceChainRecord, SignatureProjection, TrustDomain, PROVENANCE_CHAIN_RECORD_KIND,
    SIGNATURE_PROJECTION_RECORD_KIND,
};

fn valid_record(
    sequence: u32,
    source_domain: TrustDomain,
    destination_domain: TrustDomain,
    transition_class: &str,
) -> ProvenanceChainRecord {
    ProvenanceChainRecord::new(
        format!("provenance-chain:alpha:{sequence}"),
        sequence,
        source_domain,
        destination_domain,
        "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a",
        "artifact_bundle:alpha.preview.0_8_0_alpha_1.release_family",
        "digest_set:alpha.preview.0_8_0_alpha_1.release_family",
        format!("evidence:alpha:{transition_class}"),
        transition_class.to_owned(),
    )
}

fn valid_projection() -> SignatureProjection {
    SignatureProjection::new(
        "signature_projection:alpha.preview.0_8_0_alpha_1.release_family",
        "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a",
        "artifact_bundle:alpha.preview.0_8_0_alpha_1.release_family",
        "digest_set:alpha.preview.0_8_0_alpha_1.release_family",
        "seed_not_signed",
        "provenance_seed_only",
        vec![
            valid_record(
                0,
                TrustDomain::BuildAgent,
                TrustDomain::PublisherKey,
                "request_signature_by_digest",
            ),
            valid_record(
                1,
                TrustDomain::PublisherKey,
                TrustDomain::ReleaseRegistry,
                "project_signature_to_registry",
            ),
            valid_record(
                2,
                TrustDomain::ReleaseRegistry,
                TrustDomain::MirrorOrigin,
                "preserve_origin_signature_for_mirror",
            ),
        ],
    )
}

#[test]
fn valid_signature_projection_accepts_build_sign_registry_mirror_chain() {
    let projection = valid_projection();

    assert_eq!(projection.record_kind, SIGNATURE_PROJECTION_RECORD_KIND);
    assert!(projection.carries_valid_chain());
    assert!(projection.raw_signing_material_excluded);
    assert_eq!(projection.provenance_chain.len(), 3);
    assert_eq!(
        projection.provenance_chain[0].record_kind,
        PROVENANCE_CHAIN_RECORD_KIND
    );
}

#[test]
fn chain_rejects_record_with_disallowed_domain_transition() {
    let invalid = ProvenanceChainRecord::new(
        "provenance-chain:alpha:invalid",
        0,
        TrustDomain::MirrorOrigin,
        TrustDomain::PublisherKey,
        "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a",
        "artifact_bundle:alpha.preview.0_8_0_alpha_1.release_family",
        "digest_set:alpha.preview.0_8_0_alpha_1.release_family",
        "evidence:alpha:mirror_to_publisher",
        "mirror_attempts_to_reassert_origin_signature",
    );

    let violations = invalid.validate();

    assert!(violations.iter().any(|violation| {
        violation.check_id == "provenance_chain.domain_transition"
            && violation.reference == "provenance-chain:alpha:invalid"
    }));
}

#[test]
fn projection_rejects_chain_continuity_break() {
    let mut projection = valid_projection();
    projection.provenance_chain[1] = valid_record(
        1,
        TrustDomain::BuildAgent,
        TrustDomain::PublisherKey,
        "restart_without_publisher_handoff",
    );

    let violations = projection.validate();

    assert!(violations
        .iter()
        .any(|violation| violation.check_id == "signature_projection.chain_continuity"));
}
