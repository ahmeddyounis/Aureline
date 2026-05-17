//! Integration drill for the M3 filesystem-identity beta lane.
//!
//! Loads the checked-in corpus, re-proves the closed safety contract, and
//! ensures the support packet projection preserves the same target identity
//! truth across save, conflict-resolution, and support-export flows.

use std::path::PathBuf;

use aureline_vfs::identity_beta::{
    current_filesystem_identity_beta_corpus, current_filesystem_identity_beta_fixture_refs,
    load_filesystem_identity_beta_case, BetaCompareOutcome, BetaResolutionAction, DifficultyClass,
    FilesystemIdentityBetaCorpusEntry, FilesystemIdentityBetaEvaluator,
    FilesystemIdentityBetaSupportPacket, FILESYSTEM_IDENTITY_BETA_ADR_REF,
    FILESYSTEM_IDENTITY_BETA_CORPUS_DIR, FILESYSTEM_IDENTITY_BETA_CORPUS_MANIFEST_REF,
    FILESYSTEM_IDENTITY_BETA_DOC_REF, FILESYSTEM_IDENTITY_BETA_SCHEMA_REF,
    FILESYSTEM_IDENTITY_BETA_SUPPORT_PACKET_RECORD_KIND,
    FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF, REQUIRED_DIFFICULTY_CLASSES,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn corpus() -> Vec<FilesystemIdentityBetaCorpusEntry> {
    current_filesystem_identity_beta_corpus()
        .expect("checked-in filesystem-identity beta corpus must parse")
        .entries
}

#[test]
fn corpus_loads_and_validates() {
    let corpus = current_filesystem_identity_beta_corpus().expect("checked-in corpus must parse");
    FilesystemIdentityBetaEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
    assert!(!corpus.entries.is_empty(), "corpus must not be empty");
}

#[test]
fn corpus_covers_every_required_difficulty_class() {
    let entries = corpus();
    for required in REQUIRED_DIFFICULTY_CLASSES {
        assert!(
            entries
                .iter()
                .any(|entry| entry.case.difficulty_class == required),
            "required difficulty class {} has no seeded case",
            required.as_str()
        );
    }
}

#[test]
fn fixture_files_exist_on_disk() {
    let root = repo_root();
    let manifest_path = root.join(FILESYSTEM_IDENTITY_BETA_CORPUS_MANIFEST_REF);
    assert!(
        manifest_path.exists(),
        "manifest must exist on disk: {}",
        manifest_path.display()
    );
    let schema_path = root.join(FILESYSTEM_IDENTITY_BETA_SCHEMA_REF);
    assert!(
        schema_path.exists(),
        "schema must exist on disk: {}",
        schema_path.display()
    );
    let doc_path = root.join(FILESYSTEM_IDENTITY_BETA_DOC_REF);
    assert!(
        doc_path.exists(),
        "reviewer doc must exist on disk: {}",
        doc_path.display()
    );
    let adr_path = root.join(FILESYSTEM_IDENTITY_BETA_ADR_REF);
    assert!(
        adr_path.exists(),
        "frozen identity ADR must exist on disk: {}",
        adr_path.display()
    );
    let vocab_path = root.join(FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF);
    assert!(
        vocab_path.exists(),
        "vocabulary doc must exist on disk: {}",
        vocab_path.display()
    );
    let corpus_dir = root.join(FILESYSTEM_IDENTITY_BETA_CORPUS_DIR);
    assert!(
        corpus_dir.is_dir(),
        "corpus directory must exist on disk: {}",
        corpus_dir.display()
    );
    for fixture_ref in current_filesystem_identity_beta_fixture_refs() {
        let path = root.join(fixture_ref);
        assert!(
            path.exists(),
            "fixture must exist on disk: {}",
            path.display()
        );
    }
}

#[test]
fn cases_round_trip_through_serde() {
    for entry in corpus() {
        let yaml = serde_yaml::to_string(&entry.case).expect("case must serialize to yaml");
        let restored =
            load_filesystem_identity_beta_case(&yaml).expect("case must round-trip through yaml");
        assert_eq!(restored, entry.case, "{} round-trip", entry.case.case_id);
    }
}

#[test]
fn support_packet_preserves_identity_truth() {
    let corpus = current_filesystem_identity_beta_corpus().unwrap();
    let packet: FilesystemIdentityBetaSupportPacket = FilesystemIdentityBetaEvaluator::new()
        .support_packet(
            "packet:filesystem_identity_beta:drill",
            "2026-05-16T08:30:00Z",
            &corpus,
        )
        .expect("packet must build");
    assert_eq!(
        packet.record_kind,
        FILESYSTEM_IDENTITY_BETA_SUPPORT_PACKET_RECORD_KIND
    );
    assert!(packet.is_export_safe(), "packet must be export-safe");
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert_eq!(packet.cases.len(), corpus.entries.len());
    for (row, entry) in packet.cases.iter().zip(corpus.entries.iter()) {
        let case = &entry.case;
        assert_eq!(
            row.filesystem_identity_ref, case.support_export_alignment.filesystem_identity_ref,
            "support-packet row must preserve the same filesystem_identity_ref as the case"
        );
        assert!(
            row.all_refs_agree,
            "support-packet row must keep editor/git/restore/mutation/support refs aligned"
        );
        assert_eq!(
            row.writes_to_canonical_uri,
            case.alias_inspection.canonical_uri
        );
    }
}

#[test]
fn unicode_normalization_case_blocks_silent_overwrite() {
    let corpus = current_filesystem_identity_beta_corpus().unwrap();
    let entry = corpus
        .entries
        .iter()
        .find(|entry| entry.case.difficulty_class == DifficultyClass::UnicodeNormalization)
        .expect("corpus must include the unicode_normalization case");
    let resolution = &entry.case.conflict_resolution;
    assert_eq!(
        resolution.compare_outcome,
        BetaCompareOutcome::ExternalChangeDetected
    );
    assert!(resolution.silent_overwrite_forbidden);
    assert!(resolution.review_required);
    assert!(!resolution
        .resolution_actions
        .contains(&BetaResolutionAction::Write));
}

#[test]
fn bind_mount_overlay_case_records_restricted_blocker() {
    let corpus = current_filesystem_identity_beta_corpus().unwrap();
    let entry = corpus
        .entries
        .iter()
        .find(|entry| entry.case.difficulty_class == DifficultyClass::BindMountOverlay)
        .expect("corpus must include the bind_mount_overlay case");
    let review = &entry.case.save_target_review;
    assert!(review
        .blockers
        .iter()
        .any(|blocker| blocker.as_str() == "untrusted_workspace"));
    assert!(review
        .blockers
        .iter()
        .any(|blocker| blocker.as_str() == "review_required_before_save"));
}

#[test]
fn refuses_dropped_user_authored_files_preservation() {
    let mut corpus = current_filesystem_identity_beta_corpus().unwrap();
    corpus.entries[0].case.safety.preserves_user_authored_files = false;
    let err = FilesystemIdentityBetaEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("dropped user-files preservation must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.preserves_user_authored_files"));
}

#[test]
fn refuses_admitted_destructive_reset() {
    let mut corpus = current_filesystem_identity_beta_corpus().unwrap();
    corpus.entries[0].case.safety.destructive_resets_present = true;
    let err = FilesystemIdentityBetaEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("admitted destructive reset must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.destructive_resets_present"));
}

#[test]
fn refuses_admitted_raw_private_material() {
    let mut corpus = current_filesystem_identity_beta_corpus().unwrap();
    corpus.entries[0].case.safety.raw_private_material_excluded = false;
    let err = FilesystemIdentityBetaEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("admitted raw private material must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.raw_private_material_excluded"));
}

#[test]
fn refuses_support_export_ref_mismatch() {
    let mut corpus = current_filesystem_identity_beta_corpus().unwrap();
    corpus.entries[0]
        .case
        .support_export_alignment
        .editor_file_identity_ref = "fsid:wrong".to_owned();
    let err = FilesystemIdentityBetaEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("misaligned support-export refs must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.support_export_alignment.refs_disagree"));
}

#[test]
fn refuses_corpus_missing_required_difficulty_class() {
    let full = current_filesystem_identity_beta_corpus().unwrap();
    let mut truncated = full.clone();
    truncated
        .entries
        .retain(|entry| entry.case.difficulty_class != DifficultyClass::UnicodeNormalization);
    let err = FilesystemIdentityBetaEvaluator::new()
        .validate_corpus(&truncated)
        .expect_err("removing a required difficulty class must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "corpus.required_difficulty_class_missing"));
}
