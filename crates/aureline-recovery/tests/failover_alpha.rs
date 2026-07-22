//! Protected fixture checks for backup, restore, and failover alpha ingestion.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use aureline_recovery::failover_alpha::{
    load_current_failover_alpha_corpus, load_failover_continuity_cases_from_dir,
    load_rehearsal_manifest, BackupRestoreFailoverRehearsalCase, FailoverAlphaCorpus,
    FailoverAlphaLoadError, FailoverAlphaViolation, FailoverProductPostureClass, RestoreClaimClass,
    BACKUP_CHECKPOINT_CLASSES_PATH, BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_PATH,
    BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_RECORD_KIND,
};
use aureline_support::recovery_ladder::{OutageClass, OutagePlaneClass};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn load_corpus() -> FailoverAlphaCorpus {
    load_current_failover_alpha_corpus(repo_root()).expect("failover alpha corpus loads")
}

fn assert_no_violations(violations: Vec<FailoverAlphaViolation>) {
    assert_eq!(violations, Vec::new());
}

fn cases_by_class(
    corpus: &FailoverAlphaCorpus,
) -> BTreeMap<OutageClass, &BackupRestoreFailoverRehearsalCase> {
    corpus
        .rehearsal_cases
        .iter()
        .map(|entry| (entry.case.outage_class_id, &entry.case))
        .collect()
}

#[test]
fn backup_checkpoint_classes_load_typed_vocabularies_and_validate() {
    let corpus = load_corpus();
    assert_no_violations(corpus.backup_checkpoint_classes.validate());

    assert_eq!(corpus.backup_checkpoint_classes.schema_version, 1);
    assert_eq!(
        corpus
            .backup_checkpoint_classes
            .recovery_promise_class_vocabulary
            .len(),
        5
    );
    assert_eq!(
        corpus
            .backup_checkpoint_classes
            .restore_target_class_vocabulary
            .len(),
        4
    );
    assert!(corpus
        .backup_checkpoint_classes
        .cross_class_invariants
        .iter()
        .any(|row| row.id == "authoritative_backup_is_only_universal_source"));
}

#[test]
fn rehearsal_manifest_loads_every_case_file_and_aligns_outage_planes() {
    let corpus = load_corpus();
    assert_no_violations(corpus.validate());

    assert_eq!(
        corpus.rehearsal_manifest.record_kind,
        BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_RECORD_KIND
    );
    assert_eq!(
        corpus.rehearsal_cases.len(),
        corpus.rehearsal_manifest.case_files.len()
    );
    assert_eq!(corpus.rehearsal_cases.len(), 4);

    let covered = corpus
        .rehearsal_cases
        .iter()
        .map(|entry| entry.case.outage_class_id)
        .collect::<BTreeSet<_>>();
    assert_eq!(covered, OutageClass::ALL.into_iter().collect());

    for entry in &corpus.rehearsal_cases {
        assert!(entry.fixture_ref.ends_with(&entry.manifest_ref.file));
        assert_eq!(
            entry.case.primary_plane_class,
            entry.case.outage_class_id.primary_plane_class()
        );
        assert_eq!(
            entry.case.primary_plane_class,
            entry.manifest_ref.expected_primary_plane_class
        );
        assert!(entry.case.aligns_with_taxonomy());
        assert!(entry.case.export_safety.is_metadata_only_safe());
    }
}

#[test]
fn rehearsal_cases_preserve_expected_restore_and_boundary_posture() {
    let corpus = load_corpus();
    let cases = cases_by_class(&corpus);

    let local_core = cases
        .get(&OutageClass::LocalCoreContinuity)
        .expect("local core case");
    assert_eq!(
        local_core.expected_product_posture.posture_class,
        FailoverProductPostureClass::ContinueLocalWithLimits
    );
    assert_eq!(
        local_core.expected_product_posture.restore_claim_class,
        RestoreClaimClass::NoRestoreImplied
    );

    let control = cases
        .get(&OutageClass::ControlPlaneImpairment)
        .expect("control-plane case");
    assert_eq!(control.primary_plane_class, OutagePlaneClass::ControlPlane);
    assert!(control.expected_product_posture.boundary_review_required);
    assert_eq!(
        control.expected_product_posture.restore_claim_class,
        RestoreClaimClass::LastKnownGoodIsStaleEvidenceOnly
    );

    let data = cases
        .get(&OutageClass::DataPlaneImpairment)
        .expect("data-plane case");
    assert_eq!(data.primary_plane_class, OutagePlaneClass::DataPlane);
    assert!(!data.expected_product_posture.boundary_review_required);
    assert_eq!(
        data.expected_product_posture.restore_claim_class,
        RestoreClaimClass::CompareBeforeRestoreRequiredForCachedOrMirrorData
    );

    let target = cases
        .get(&OutageClass::FullTargetLoss)
        .expect("full target loss case");
    assert_eq!(
        target.primary_plane_class,
        OutagePlaneClass::TargetAuthority
    );
    assert!(!target.plane_observation.target_reachable);
    assert_eq!(
        target.expected_product_posture.restore_claim_class,
        RestoreClaimClass::ExactRestoreRequiresMatchingTargetEvidence
    );
}

#[test]
fn failover_continuity_cases_load_by_directory_and_keep_local_safe_baseline_visible() {
    let corpus = load_corpus();
    assert_eq!(corpus.continuity_cases.len(), 4);

    let names = corpus
        .continuity_cases
        .iter()
        .map(|entry| entry.case.fixture_metadata.name.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        names,
        [
            "local_safe_only_mode",
            "partial_queue_retry_continuity",
            "regional_failover_changed_boundary",
            "service_family_outage",
        ]
        .into_iter()
        .collect()
    );

    for entry in &corpus.continuity_cases {
        assert!(entry.fixture_ref.ends_with(".yaml"));
        assert!(entry.case.preserves_local_safe_posture());
        assert_eq!(
            entry.case.failover_banner.local_safe_baseline_ref,
            entry.case.local_safe_baseline.baseline_id
        );
        assert!(!entry.case.failover_banner.continuity_action_rows.is_empty());
    }

    let regional = corpus
        .continuity_cases
        .iter()
        .find(|entry| entry.case.fixture_metadata.name == "regional_failover_changed_boundary")
        .expect("regional failover continuity case");
    assert!(
        regional
            .case
            .failover_banner
            .boundary_change_note
            .boundary_change_required
    );
    assert_eq!(
        regional
            .case
            .failover_banner
            .boundary_change_note
            .boundary_axes_summary
            .len(),
        5
    );
}

#[test]
fn in_memory_yaml_is_bounded_and_parse_errors_are_redaction_safe() {
    let oversized = " ".repeat(4 * 1024 * 1024 + 1);
    assert!(matches!(
        load_rehearsal_manifest(&oversized),
        Err(FailoverAlphaLoadError::ResourceLimitExceeded {
            resource: "input bytes",
            ..
        })
    ));

    let private_value = "private-hostname-do-not-export.example";
    let malformed = format!("schema_version: [{private_value}");
    let error = load_rehearsal_manifest(&malformed).expect_err("malformed YAML must fail");
    let rendered = error.to_string();
    assert!(rendered.contains("protected rehearsal manifest"));
    assert!(!rendered.contains(private_value));
}

#[test]
fn in_memory_yaml_shape_is_bounded_before_typed_projection() {
    let oversized_sequence = format!("rows:\n{}", "  - value\n".repeat(4_097));
    assert!(matches!(
        load_rehearsal_manifest(&oversized_sequence),
        Err(FailoverAlphaLoadError::ResourceLimitExceeded {
            resource: "sequence entries",
            ..
        })
    ));

    let oversized_scalar = format!("value: {}\n", "x".repeat(256 * 1024 + 1));
    assert!(matches!(
        load_rehearsal_manifest(&oversized_scalar),
        Err(FailoverAlphaLoadError::ResourceLimitExceeded {
            resource: "scalar bytes",
            ..
        })
    ));
}

#[test]
fn rehearsal_manifest_case_paths_cannot_escape_the_repository() {
    let temp = tempfile::tempdir().expect("temporary parent");
    let fake_repo = temp.path().join("repo");
    let manifest_path = fake_repo.join(BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_PATH);
    let backup_path = fake_repo.join(BACKUP_CHECKPOINT_CLASSES_PATH);
    fs::create_dir_all(manifest_path.parent().expect("manifest parent"))
        .expect("create manifest directory");
    fs::create_dir_all(backup_path.parent().expect("backup parent"))
        .expect("create backup directory");
    fs::copy(
        repo_root().join(BACKUP_CHECKPOINT_CLASSES_PATH),
        &backup_path,
    )
    .expect("copy backup artifact");

    let manifest =
        fs::read_to_string(repo_root().join(BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_PATH))
            .expect("read source manifest")
            .replacen(
                "file: local_core_continuity.yaml",
                "file: ../../../../escaped.yaml",
                1,
            );
    fs::write(&manifest_path, manifest).expect("write malicious manifest");
    fs::write(temp.path().join("escaped.yaml"), "schema_version: 1\n")
        .expect("write escaped target");

    let error = load_current_failover_alpha_corpus(&fake_repo)
        .expect_err("manifest traversal must be rejected");
    assert!(matches!(&error, FailoverAlphaLoadError::UnsafePath { .. }));
    assert!(!error.to_string().contains(&fake_repo.display().to_string()));
}

#[test]
fn continuity_directory_rejects_oversized_yaml_before_parsing() {
    let temp = tempfile::tempdir().expect("temporary fixture directory");
    let path = temp.path().join("oversized.yaml");
    let file = File::create(&path).expect("create sparse fixture");
    file.set_len(4 * 1024 * 1024 + 1)
        .expect("extend sparse fixture");

    assert!(matches!(
        load_failover_continuity_cases_from_dir(temp.path()),
        Err(FailoverAlphaLoadError::ResourceLimitExceeded {
            resource: "input bytes",
            ..
        })
    ));
}

#[cfg(unix)]
#[test]
fn continuity_directory_rejects_yaml_symlinks() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().expect("temporary fixture directory");
    let target = temp.path().join("target.txt");
    fs::write(&target, "schema_version: 1\n").expect("write symlink target");
    symlink(&target, temp.path().join("redirect.yaml")).expect("create YAML symlink");

    assert!(matches!(
        load_failover_continuity_cases_from_dir(temp.path()),
        Err(FailoverAlphaLoadError::UnsafeFileType { .. })
    ));
}
