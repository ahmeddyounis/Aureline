use super::*;

fn sample_success_run() -> ExperimentRunIdentity {
    ExperimentRunIdentity {
        record_kind: EXPERIMENT_RUN_IDENTITY_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        run_id: "run.experiment.001.success".to_owned(),
        title: "Training run v1".to_owned(),
        source_ref: "notebook.training.v1.ipynb".to_owned(),
        started_at: "2026-06-09T10:00:00Z".to_owned(),
        ended_at: Some("2026-06-09T10:30:00Z".to_owned()),
        outcome_class: ExperimentRunOutcomeClass::Success,
        commit_or_revision_ref: Some("git.commit.abc123".to_owned()),
        execution_origin_label: "local_host".to_owned(),
        environment_fingerprint_ref: "env.fp.python.312.fresh.01".to_owned(),
        summary: "Successful local training run on Python 3.12.".to_owned(),
    }
}

fn sample_failure_run() -> ExperimentRunIdentity {
    ExperimentRunIdentity {
        record_kind: EXPERIMENT_RUN_IDENTITY_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        run_id: "run.experiment.002.failure".to_owned(),
        title: "Training run v2".to_owned(),
        source_ref: "notebook.training.v2.ipynb".to_owned(),
        started_at: "2026-06-09T11:00:00Z".to_owned(),
        ended_at: Some("2026-06-09T11:05:00Z".to_owned()),
        outcome_class: ExperimentRunOutcomeClass::Failure,
        commit_or_revision_ref: Some("git.commit.def456".to_owned()),
        execution_origin_label: "managed_workspace:gpu-pool".to_owned(),
        environment_fingerprint_ref: "env.fp.remote.stale.01".to_owned(),
        summary: "Failed remote training run due to OOM.".to_owned(),
    }
}

fn sample_cancelled_run() -> ExperimentRunIdentity {
    ExperimentRunIdentity {
        record_kind: EXPERIMENT_RUN_IDENTITY_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        run_id: "run.experiment.003.cancelled".to_owned(),
        title: "Training run v3".to_owned(),
        source_ref: "notebook.training.v3.ipynb".to_owned(),
        started_at: "2026-06-09T12:00:00Z".to_owned(),
        ended_at: None,
        outcome_class: ExperimentRunOutcomeClass::Cancelled,
        commit_or_revision_ref: None,
        execution_origin_label: "local_host".to_owned(),
        environment_fingerprint_ref: "env.fp.python.312.fresh.01".to_owned(),
        summary: "User-cancelled local training run.".to_owned(),
    }
}

fn sample_partial_run() -> ExperimentRunIdentity {
    ExperimentRunIdentity {
        record_kind: EXPERIMENT_RUN_IDENTITY_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        run_id: "run.experiment.004.partial".to_owned(),
        title: "Training run v4".to_owned(),
        source_ref: "notebook.training.v4.ipynb".to_owned(),
        started_at: "2026-06-09T13:00:00Z".to_owned(),
        ended_at: Some("2026-06-09T13:20:00Z".to_owned()),
        outcome_class: ExperimentRunOutcomeClass::Partial,
        commit_or_revision_ref: Some("git.commit.ghi789".to_owned()),
        execution_origin_label: "remote_agent:eu-west".to_owned(),
        environment_fingerprint_ref: "env.fp.remote.partial.01".to_owned(),
        summary: "Partial remote training run; checkpoint saved.".to_owned(),
    }
}

fn sample_policy_blocked_run() -> ExperimentRunIdentity {
    ExperimentRunIdentity {
        record_kind: EXPERIMENT_RUN_IDENTITY_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        run_id: "run.experiment.005.policy".to_owned(),
        title: "Training run v5".to_owned(),
        source_ref: "notebook.training.v5.ipynb".to_owned(),
        started_at: "2026-06-09T14:00:00Z".to_owned(),
        ended_at: None,
        outcome_class: ExperimentRunOutcomeClass::PolicyBlocked,
        commit_or_revision_ref: None,
        execution_origin_label: "managed_workspace:restricted".to_owned(),
        environment_fingerprint_ref: "env.fp.remote.policy.01".to_owned(),
        summary: "Policy-blocked run on restricted managed workspace.".to_owned(),
    }
}

fn sample_fresh_fingerprint() -> ExperimentEnvironmentFingerprint {
    ExperimentEnvironmentFingerprint {
        record_kind: EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        fingerprint_id: "env.fp.python.312.fresh.01".to_owned(),
        environment_identity_label: "python3.12 + pandas2.3 + torch2.8".to_owned(),
        interpreter_kernel_label: "Python 3.12".to_owned(),
        package_toolchain_summary: Some("pandas 2.3, torch 2.8, numpy 2.0, uv 0.4".to_owned()),
        target_origin_label: "local_host".to_owned(),
        policy_epoch_ref: Some("policy.epoch.01".to_owned()),
        freshness_class: ExperimentEnvironmentFingerprintFreshnessClass::Fresh,
        last_known_good_at: Some("2026-06-09T10:00:00Z".to_owned()),
        summary: "Fresh local Python 3.12 environment fingerprint.".to_owned(),
    }
}

fn sample_stale_fingerprint() -> ExperimentEnvironmentFingerprint {
    ExperimentEnvironmentFingerprint {
        record_kind: EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        fingerprint_id: "env.fp.r.430.stale.01".to_owned(),
        environment_identity_label: "r4.3.0 + tidyverse 2.0".to_owned(),
        interpreter_kernel_label: "R 4.3.0".to_owned(),
        package_toolchain_summary: Some("tidyverse 2.0, dplyr 1.1".to_owned()),
        target_origin_label: "local_host".to_owned(),
        policy_epoch_ref: None,
        freshness_class: ExperimentEnvironmentFingerprintFreshnessClass::Stale,
        last_known_good_at: Some("2026-05-01T10:00:00Z".to_owned()),
        summary: "Stale local R 4.3.0 environment fingerprint.".to_owned(),
    }
}

fn sample_unresolved_fingerprint() -> ExperimentEnvironmentFingerprint {
    ExperimentEnvironmentFingerprint {
        record_kind: EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        fingerprint_id: "env.fp.unresolved.01".to_owned(),
        environment_identity_label: "unknown environment".to_owned(),
        interpreter_kernel_label: "unknown".to_owned(),
        package_toolchain_summary: None,
        target_origin_label: "local_host".to_owned(),
        policy_epoch_ref: None,
        freshness_class: ExperimentEnvironmentFingerprintFreshnessClass::Unresolved,
        last_known_good_at: None,
        summary: "Unresolved environment fingerprint.".to_owned(),
    }
}

fn sample_policy_blocked_fingerprint() -> ExperimentEnvironmentFingerprint {
    ExperimentEnvironmentFingerprint {
        record_kind: EXPERIMENT_ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        fingerprint_id: "env.fp.remote.policy.01".to_owned(),
        environment_identity_label: "python3.11 + unmanaged-packages".to_owned(),
        interpreter_kernel_label: "Python 3.11".to_owned(),
        package_toolchain_summary: None,
        target_origin_label: "managed_workspace:gpu-pool".to_owned(),
        policy_epoch_ref: Some("policy.epoch.blocked.01".to_owned()),
        freshness_class: ExperimentEnvironmentFingerprintFreshnessClass::PolicyBlocked,
        last_known_good_at: None,
        summary: "Policy-blocked remote environment fingerprint.".to_owned(),
    }
}

fn sample_local_file_dataset() -> DatasetCard {
    DatasetCard {
        record_kind: DATASET_CARD_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        dataset_id: "dataset.local.train.01".to_owned(),
        dataset_label: "Training data (local CSV)".to_owned(),
        source_class: DatasetSourceClass::LocalFile,
        snapshot_version_label: Some("v2026-06-01".to_owned()),
        size_estimate_label: Some("1.2M rows, 450 MB".to_owned()),
        sensitivity_redaction_class: DatasetSensitivityRedactionClass::Internal,
        location_class: DatasetLocationClass::LocalWorkspace,
        summary: "Local training CSV with internal sensitivity.".to_owned(),
    }
}

fn sample_remote_url_dataset() -> DatasetCard {
    DatasetCard {
        record_kind: DATASET_CARD_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        dataset_id: "dataset.remote.val.01".to_owned(),
        dataset_label: "Validation data (remote URL)".to_owned(),
        source_class: DatasetSourceClass::RemoteUrl,
        snapshot_version_label: Some("sha256:abc123".to_owned()),
        size_estimate_label: Some("300K rows, 120 MB".to_owned()),
        sensitivity_redaction_class: DatasetSensitivityRedactionClass::Public,
        location_class: DatasetLocationClass::RemoteStorage,
        summary: "Public validation data fetched from remote URL.".to_owned(),
    }
}

fn sample_database_dataset() -> DatasetCard {
    DatasetCard {
        record_kind: DATASET_CARD_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        dataset_id: "dataset.db.test.01".to_owned(),
        dataset_label: "Test partition (database)".to_owned(),
        source_class: DatasetSourceClass::Database,
        snapshot_version_label: Some("partition_2026_06".to_owned()),
        size_estimate_label: Some("50K rows".to_owned()),
        sensitivity_redaction_class: DatasetSensitivityRedactionClass::Confidential,
        location_class: DatasetLocationClass::ManagedCache,
        summary: "Confidential test partition from managed database cache.".to_owned(),
    }
}

fn sample_generated_dataset() -> DatasetCard {
    DatasetCard {
        record_kind: DATASET_CARD_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        dataset_id: "dataset.gen.aug.01".to_owned(),
        dataset_label: "Augmented training set".to_owned(),
        source_class: DatasetSourceClass::Generated,
        snapshot_version_label: None,
        size_estimate_label: Some("2.4M rows, 900 MB".to_owned()),
        sensitivity_redaction_class: DatasetSensitivityRedactionClass::RedactedPreview,
        location_class: DatasetLocationClass::LocalWorkspace,
        summary: "Generated augmented training set with redacted preview.".to_owned(),
    }
}

fn sample_blocked_dataset() -> DatasetCard {
    DatasetCard {
        record_kind: DATASET_CARD_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        dataset_id: "dataset.blocked.pii.01".to_owned(),
        dataset_label: "PII-enriched dataset".to_owned(),
        source_class: DatasetSourceClass::Unknown,
        snapshot_version_label: None,
        size_estimate_label: Some("unknown size".to_owned()),
        sensitivity_redaction_class: DatasetSensitivityRedactionClass::Blocked,
        location_class: DatasetLocationClass::ProviderOnly,
        summary: "Blocked PII dataset; metadata only.".to_owned(),
    }
}

fn sample_current_artifact() -> ArtifactLineage {
    ArtifactLineage {
        record_kind: ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        artifact_id: "artifact.model.v1.current".to_owned(),
        producing_run_ref: "run.experiment.001.success".to_owned(),
        generator_step_label: "cell_7_export".to_owned(),
        environment_fingerprint_ref: Some("env.fp.python.312.fresh.01".to_owned()),
        save_location_class: ArtifactSaveLocationClass::LocalWorkspace,
        lineage_state_class: ArtifactLineageStateClass::Current,
        stale_diverged_note: None,
        summary: "Current model artifact from successful training run.".to_owned(),
    }
}

fn sample_stale_artifact() -> ArtifactLineage {
    ArtifactLineage {
        record_kind: ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        artifact_id: "artifact.report.v1.stale".to_owned(),
        producing_run_ref: "run.experiment.002.failure".to_owned(),
        generator_step_label: "cell_12_render".to_owned(),
        environment_fingerprint_ref: Some("env.fp.remote.stale.01".to_owned()),
        save_location_class: ArtifactSaveLocationClass::RemoteStorage,
        lineage_state_class: ArtifactLineageStateClass::Stale,
        stale_diverged_note: Some("Environment packages updated since run.".to_owned()),
        summary: "Stale report artifact from failed run.".to_owned(),
    }
}

fn sample_diverged_artifact() -> ArtifactLineage {
    ArtifactLineage {
        record_kind: ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        artifact_id: "artifact.chart.v1.diverged".to_owned(),
        producing_run_ref: "run.experiment.004.partial".to_owned(),
        generator_step_label: "cell_5_plot".to_owned(),
        environment_fingerprint_ref: None,
        save_location_class: ArtifactSaveLocationClass::ExportBuffer,
        lineage_state_class: ArtifactLineageStateClass::Diverged,
        stale_diverged_note: Some("Artifact was overwritten by a later manual export.".to_owned()),
        summary: "Diverged chart artifact overwritten after partial run.".to_owned(),
    }
}

fn sample_orphaned_artifact() -> ArtifactLineage {
    ArtifactLineage {
        record_kind: ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        artifact_id: "artifact.orphaned.01".to_owned(),
        producing_run_ref: "orphaned".to_owned(),
        generator_step_label: "unknown".to_owned(),
        environment_fingerprint_ref: None,
        save_location_class: ArtifactSaveLocationClass::LocalWorkspace,
        lineage_state_class: ArtifactLineageStateClass::Orphaned,
        stale_diverged_note: None,
        summary: "Orphaned artifact with no producing run.".to_owned(),
    }
}

fn sample_imported_artifact() -> ArtifactLineage {
    ArtifactLineage {
        record_kind: ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
        notebook_experiment_lineage_schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        artifact_id: "artifact.imported.01".to_owned(),
        producing_run_ref: "run.external.999".to_owned(),
        generator_step_label: "external_pipeline_step".to_owned(),
        environment_fingerprint_ref: Some("env.fp.external.01".to_owned()),
        save_location_class: ArtifactSaveLocationClass::ManagedArtifactStore,
        lineage_state_class: ArtifactLineageStateClass::Imported,
        stale_diverged_note: None,
        summary: "Imported artifact from external pipeline.".to_owned(),
    }
}

#[test]
fn success_run_validates_clean() {
    let run = sample_success_run();
    assert!(
        run.validate().is_empty(),
        "success run should be clean: {:?}",
        run.validate()
    );
}

#[test]
fn failure_run_validates_clean() {
    let run = sample_failure_run();
    assert!(
        run.validate().is_empty(),
        "failure run should be clean: {:?}",
        run.validate()
    );
}

#[test]
fn cancelled_run_validates_clean() {
    let run = sample_cancelled_run();
    assert!(
        run.validate().is_empty(),
        "cancelled run should be clean: {:?}",
        run.validate()
    );
}

#[test]
fn partial_run_validates_clean() {
    let run = sample_partial_run();
    assert!(
        run.validate().is_empty(),
        "partial run should be clean: {:?}",
        run.validate()
    );
}

#[test]
fn policy_blocked_run_validates_clean() {
    let run = sample_policy_blocked_run();
    assert!(
        run.validate().is_empty(),
        "policy blocked run should be clean: {:?}",
        run.validate()
    );
}

#[test]
fn run_rejects_empty_title() {
    let mut run = sample_success_run();
    run.title = "".to_owned();
    let findings = run.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "experiment_run_identity.title"));
}

#[test]
fn run_rejects_empty_source_ref() {
    let mut run = sample_success_run();
    run.source_ref = "".to_owned();
    let findings = run.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "experiment_run_identity.source_ref"));
}

#[test]
fn success_run_requires_ended_at() {
    let mut run = sample_success_run();
    run.ended_at = None;
    let findings = run.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "experiment_run_identity.success_requires_ended_at"));
}

#[test]
fn fresh_fingerprint_validates_clean() {
    let fp = sample_fresh_fingerprint();
    assert!(
        fp.validate().is_empty(),
        "fresh fingerprint should be clean: {:?}",
        fp.validate()
    );
}

#[test]
fn stale_fingerprint_validates_clean() {
    let fp = sample_stale_fingerprint();
    assert!(
        fp.validate().is_empty(),
        "stale fingerprint should be clean: {:?}",
        fp.validate()
    );
}

#[test]
fn unresolved_fingerprint_validates_clean() {
    let fp = sample_unresolved_fingerprint();
    assert!(
        fp.validate().is_empty(),
        "unresolved fingerprint should be clean: {:?}",
        fp.validate()
    );
}

#[test]
fn policy_blocked_fingerprint_validates_clean() {
    let fp = sample_policy_blocked_fingerprint();
    assert!(
        fp.validate().is_empty(),
        "policy_blocked fingerprint should be clean: {:?}",
        fp.validate()
    );
}

#[test]
fn fresh_fingerprint_requires_last_known_good() {
    let mut fp = sample_fresh_fingerprint();
    fp.last_known_good_at = None;
    let findings = fp.validate();
    assert!(findings.iter().any(|f| {
        f.check_id == "experiment_environment_fingerprint.fresh_requires_last_known_good"
    }));
}

#[test]
fn policy_blocked_fingerprint_requires_epoch() {
    let mut fp = sample_policy_blocked_fingerprint();
    fp.policy_epoch_ref = None;
    let findings = fp.validate();
    assert!(findings.iter().any(|f| {
        f.check_id == "experiment_environment_fingerprint.policy_blocked_requires_epoch"
    }));
}

#[test]
fn local_file_dataset_validates_clean() {
    let ds = sample_local_file_dataset();
    assert!(
        ds.validate().is_empty(),
        "local file dataset should be clean: {:?}",
        ds.validate()
    );
}

#[test]
fn remote_url_dataset_validates_clean() {
    let ds = sample_remote_url_dataset();
    assert!(
        ds.validate().is_empty(),
        "remote url dataset should be clean: {:?}",
        ds.validate()
    );
}

#[test]
fn database_dataset_validates_clean() {
    let ds = sample_database_dataset();
    assert!(
        ds.validate().is_empty(),
        "database dataset should be clean: {:?}",
        ds.validate()
    );
}

#[test]
fn generated_dataset_validates_clean() {
    let ds = sample_generated_dataset();
    assert!(
        ds.validate().is_empty(),
        "generated dataset should be clean: {:?}",
        ds.validate()
    );
}

#[test]
fn blocked_dataset_validates_clean() {
    let ds = sample_blocked_dataset();
    assert!(
        ds.validate().is_empty(),
        "blocked dataset should be clean: {:?}",
        ds.validate()
    );
}

#[test]
fn dataset_rejects_empty_label() {
    let mut ds = sample_local_file_dataset();
    ds.dataset_label = "".to_owned();
    let findings = ds.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "dataset_card.dataset_label"));
}

#[test]
fn redacted_dataset_requires_size_estimate() {
    let mut ds = sample_generated_dataset();
    ds.size_estimate_label = None;
    let findings = ds.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "dataset_card.redacted_requires_size_estimate"));
}

#[test]
fn blocked_dataset_requires_size_estimate() {
    let mut ds = sample_blocked_dataset();
    ds.size_estimate_label = None;
    let findings = ds.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "dataset_card.redacted_requires_size_estimate"));
}

#[test]
fn current_artifact_validates_clean() {
    let art = sample_current_artifact();
    assert!(
        art.validate().is_empty(),
        "current artifact should be clean: {:?}",
        art.validate()
    );
}

#[test]
fn stale_artifact_validates_clean() {
    let art = sample_stale_artifact();
    assert!(
        art.validate().is_empty(),
        "stale artifact should be clean: {:?}",
        art.validate()
    );
}

#[test]
fn diverged_artifact_validates_clean() {
    let art = sample_diverged_artifact();
    assert!(
        art.validate().is_empty(),
        "diverged artifact should be clean: {:?}",
        art.validate()
    );
}

#[test]
fn orphaned_artifact_validates_clean() {
    let art = sample_orphaned_artifact();
    assert!(
        art.validate().is_empty(),
        "orphaned artifact should be clean: {:?}",
        art.validate()
    );
}

#[test]
fn imported_artifact_validates_clean() {
    let art = sample_imported_artifact();
    assert!(
        art.validate().is_empty(),
        "imported artifact should be clean: {:?}",
        art.validate()
    );
}

#[test]
fn artifact_rejects_empty_producing_run_ref() {
    let mut art = sample_current_artifact();
    art.producing_run_ref = "".to_owned();
    let findings = art.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "artifact_lineage.producing_run_ref"));
}

#[test]
fn artifact_rejects_empty_generator_step_label() {
    let mut art = sample_current_artifact();
    art.generator_step_label = "".to_owned();
    let findings = art.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "artifact_lineage.generator_step_label"));
}

#[test]
fn stale_artifact_requires_note() {
    let mut art = sample_stale_artifact();
    art.stale_diverged_note = None;
    let findings = art.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "artifact_lineage.stale_diverged_requires_note"));
}

#[test]
fn diverged_artifact_requires_note() {
    let mut art = sample_diverged_artifact();
    art.stale_diverged_note = None;
    let findings = art.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "artifact_lineage.stale_diverged_requires_note"));
}

#[test]
fn orphaned_artifact_requires_orphaned_run_ref() {
    let mut art = sample_orphaned_artifact();
    art.producing_run_ref = "run.some.other".to_owned();
    let findings = art.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "artifact_lineage.orphaned_run_ref"));
}

#[test]
fn packet_validates_clean() {
    let packet = ExperimentLineagePacket {
        schema_version: NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION,
        record_kind: EXPERIMENT_LINEAGE_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.experiment_lineage.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        experiment_run_outcome_classes: ExperimentRunOutcomeClass::ALL.to_vec(),
        experiment_environment_fingerprint_freshness_classes:
            ExperimentEnvironmentFingerprintFreshnessClass::ALL.to_vec(),
        dataset_source_classes: DatasetSourceClass::ALL.to_vec(),
        dataset_sensitivity_redaction_classes: DatasetSensitivityRedactionClass::ALL.to_vec(),
        dataset_location_classes: DatasetLocationClass::ALL.to_vec(),
        artifact_save_location_classes: ArtifactSaveLocationClass::ALL.to_vec(),
        artifact_lineage_state_classes: ArtifactLineageStateClass::ALL.to_vec(),
        example_experiment_run_identities: vec![
            sample_success_run(),
            sample_failure_run(),
            sample_cancelled_run(),
            sample_partial_run(),
            sample_policy_blocked_run(),
        ],
        example_experiment_environment_fingerprints: vec![
            sample_fresh_fingerprint(),
            sample_stale_fingerprint(),
            sample_unresolved_fingerprint(),
            sample_policy_blocked_fingerprint(),
        ],
        example_dataset_cards: vec![
            sample_local_file_dataset(),
            sample_remote_url_dataset(),
            sample_database_dataset(),
            sample_generated_dataset(),
            sample_blocked_dataset(),
        ],
        example_artifact_lineages: vec![
            sample_current_artifact(),
            sample_stale_artifact(),
            sample_diverged_artifact(),
            sample_orphaned_artifact(),
            sample_imported_artifact(),
        ],
        summary:
            "Experiment run identities, environment fingerprints, dataset cards, and artifact lineage packet v1."
                .to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_experiment_lineage_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_EXPERIMENT_LINEAGE_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, EXPERIMENT_LINEAGE_PACKET_RECORD_KIND);
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(ExperimentRunOutcomeClass::Success.as_str(), "success");
    assert_eq!(
        ExperimentRunOutcomeClass::PolicyBlocked.as_str(),
        "policy_blocked"
    );

    assert_eq!(
        ExperimentEnvironmentFingerprintFreshnessClass::Fresh.as_str(),
        "fresh"
    );
    assert_eq!(
        ExperimentEnvironmentFingerprintFreshnessClass::PolicyBlocked.as_str(),
        "policy_blocked"
    );

    assert_eq!(DatasetSourceClass::LocalFile.as_str(), "local_file");
    assert_eq!(DatasetSourceClass::Unknown.as_str(), "unknown");

    assert_eq!(
        DatasetSensitivityRedactionClass::RedactedPreview.as_str(),
        "redacted_preview"
    );
    assert_eq!(
        DatasetSensitivityRedactionClass::Blocked.as_str(),
        "blocked"
    );

    assert_eq!(
        DatasetLocationClass::LocalWorkspace.as_str(),
        "local_workspace"
    );
    assert_eq!(DatasetLocationClass::ProviderOnly.as_str(), "provider_only");

    assert_eq!(
        ArtifactSaveLocationClass::ManagedArtifactStore.as_str(),
        "managed_artifact_store"
    );
    assert_eq!(
        ArtifactSaveLocationClass::ExportBuffer.as_str(),
        "export_buffer"
    );

    assert_eq!(ArtifactLineageStateClass::Current.as_str(), "current");
    assert_eq!(ArtifactLineageStateClass::Orphaned.as_str(), "orphaned");
}
