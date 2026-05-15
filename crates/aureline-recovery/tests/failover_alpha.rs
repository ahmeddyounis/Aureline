//! Protected fixture checks for backup, restore, and failover alpha ingestion.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use aureline_recovery::failover_alpha::{
    load_current_failover_alpha_corpus, BackupRestoreFailoverRehearsalCase, FailoverAlphaCorpus,
    FailoverAlphaViolation, FailoverProductPostureClass, RestoreClaimClass,
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
