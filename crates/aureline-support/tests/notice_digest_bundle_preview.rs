//! Support-bundle preview coverage for the dependency notice digest.

use std::path::{Path, PathBuf};

use aureline_notices::generate_notice_bundle;
use aureline_support::bundle::{
    add_notice_digest_preview_item, ExactBuildCapture, RedactionState, ReleaseChannelClass,
    SupportBundlePreviewBuilder, SUPPORT_ITEM_NOTICE_DIGEST,
};

const FIXTURE_BUILD_ID: &str =
    "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:notice123456";
const FIXTURE_TIMESTAMP: &str = "2026-05-15T08:00:00Z";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(FIXTURE_BUILD_ID, "0.0.0", ReleaseChannelClass::DevLocal)
}

#[test]
fn support_bundle_preview_includes_notice_digest_metadata_row() {
    let notices = generate_notice_bundle(repo_root()).expect("generate notice bundle");
    let digest_fingerprint = notices.notice_digest.digest_fingerprint.clone();
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:notice-digest:0001",
        "Notice digest support preview",
        FIXTURE_TIMESTAMP,
        fixture_capture(),
    );
    add_notice_digest_preview_item(&mut builder, &notices);

    let preview = builder.build().expect("build support preview");
    let row = preview
        .manifest
        .preview_items
        .iter()
        .find(|item| item.parity_binding.support_pack_item_id == SUPPORT_ITEM_NOTICE_DIGEST)
        .expect("notice digest preview row");

    assert_eq!(row.title, "Dependency notice digest");
    assert_eq!(
        row.redaction.redaction_state,
        RedactionState::NotRequiredMetadata
    );
    assert!(row.file_section_identity.source_refs.iter().any(|source| {
        source == "Cargo.lock" || source == "artifacts/governance/release_notice_seed.yaml"
    }));
    assert!(row.notes.contains(&digest_fingerprint));
    assert!(preview
        .manifest
        .preview_classification_summary
        .included_support_pack_item_ids
        .iter()
        .any(|item_id| item_id == SUPPORT_ITEM_NOTICE_DIGEST));
}
