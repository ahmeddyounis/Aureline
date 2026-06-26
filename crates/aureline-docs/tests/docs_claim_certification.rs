//! Fixture-driven coverage for the M5 documentation-claim certification packet.

use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_claim_certification_export, seeded_stable_docs_claim_certification_input,
    CertifiedDocsProfile, DocsClaimCertificationPacket, DocsEvidenceClass,
    DOCS_CLAIM_CERTIFICATION_ARTIFACT_REF, DOCS_CLAIM_CERTIFICATION_DOC_REF,
    DOCS_CLAIM_CERTIFICATION_FIXTURE_DIR, DOCS_CLAIM_CERTIFICATION_SCHEMA_REF,
    DOCS_CLAIM_CERTIFICATION_SUMMARY_REF, M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF,
    M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> DocsClaimCertificationPacket {
    let path = repo_root()
        .join(DOCS_CLAIM_CERTIFICATION_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn canonical_paths_exist_on_disk() {
    assert_exists(DOCS_CLAIM_CERTIFICATION_SCHEMA_REF);
    assert_exists(DOCS_CLAIM_CERTIFICATION_DOC_REF);
    assert_exists(DOCS_CLAIM_CERTIFICATION_ARTIFACT_REF);
    assert_exists(DOCS_CLAIM_CERTIFICATION_SUMMARY_REF);
    assert_exists(DOCS_CLAIM_CERTIFICATION_FIXTURE_DIR);
    // The frozen matrix this certification binds against.
    assert_exists(M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF);
    assert_exists(M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF);
}

#[test]
fn evidence_corpus_refs_resolve_on_disk() {
    // Every upstream support export and schema this packet certifies against
    // must exist; the certification is tied to the proof packets, not anecdote.
    for class in DocsEvidenceClass::ALL {
        for schema_ref in class.evidence_schema_refs() {
            assert_exists(schema_ref);
        }
        for artifact_ref in class.evidence_artifact_refs() {
            assert_exists(artifact_ref);
        }
    }
}

#[test]
fn checked_support_export_matches_seeded_input() {
    let from_disk =
        current_stable_docs_claim_certification_export().expect("checked export validates");
    let seeded = DocsClaimCertificationPacket::new(seeded_stable_docs_claim_certification_input());
    assert_eq!(
        from_disk, seeded,
        "checked support export drifted from the seeded certification input"
    );
}

#[test]
fn checked_summary_matches_rendered_summary() {
    let seeded = DocsClaimCertificationPacket::new(seeded_stable_docs_claim_certification_input());
    let on_disk = std::fs::read_to_string(repo_root().join(DOCS_CLAIM_CERTIFICATION_SUMMARY_REF))
        .expect("read checked summary");
    assert_eq!(
        on_disk.trim_end(),
        seeded.render_markdown_summary().trim_end(),
        "checked Markdown summary drifted from the rendered summary"
    );
}

#[test]
fn retest_pending_fixture_narrows_without_blocking() {
    let packet = load_fixture("source_class_evidence_stale_retest_pending.json");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Every profile depends on source class, so all are retest-pending.
    for profile in CertifiedDocsProfile::ALL {
        assert!(
            packet.retest_pending_profiles().contains(&profile),
            "profile {} should be retest-pending",
            profile.as_str()
        );
    }
    assert!(packet.publication_blockers().is_empty());
}

#[test]
fn browser_handoff_blocked_fixture_blocks_publication() {
    let packet = load_fixture("browser_handoff_evidence_stale_blocks_publication.json");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Profiles that do not touch browser handoff stay green.
    assert!(!packet
        .publication_blockers()
        .contains(&CertifiedDocsProfile::OnboardingLearning));
    // Handoff-bearing profiles are blocked but still present (labeled, not hidden).
    for profile in [
        CertifiedDocsProfile::DocsBrowser,
        CertifiedDocsProfile::HelpAbout,
        CertifiedDocsProfile::AiExplanation,
        CertifiedDocsProfile::SupportExport,
    ] {
        assert!(
            packet.publication_blockers().contains(&profile),
            "profile {} should block publication",
            profile.as_str()
        );
    }
}
