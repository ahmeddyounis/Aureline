//! Protected tests for the repair-transaction preview-skeleton beta evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::repair::{
    current_alpha_repair_seed_cases, ApplyModeClass, PreservedStateClass, RepairAlpha,
    RepairClassFamily, TransactionReversalClass,
};
use aureline_support::repair_transactions::{
    load_repair_preview_comparison, load_repair_preview_skeleton, RepairBlastRadiusClass,
    RepairCancellationClass, RepairCheckpointDispositionClass, RepairComparisonAxisClass,
    RepairCompensationClass, RepairPreviewComparison, RepairPreviewDispositionClass,
    RepairPreviewSkeleton, RepairPreviewSkeletonEvaluator, REPAIR_PREVIEW_COMPARISON_RECORD_KIND,
    REPAIR_PREVIEW_SKELETON_DOC_REF, REPAIR_PREVIEW_SKELETON_RECORD_KIND,
    REPAIR_PREVIEW_SKELETON_SCHEMA_REF, REPAIR_PREVIEW_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    scenarios: Vec<ManifestScenario>,
}

#[derive(Debug, Deserialize)]
struct ManifestScenario {
    scenario_id: String,
    skeleton_file: String,
    comparison_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/recovery/m3/repair_transaction_preview")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

struct LoadedScenario {
    scenario_id: String,
    skeleton: RepairPreviewSkeleton,
    comparisons: Vec<RepairPreviewComparison>,
}

fn load_scenarios() -> Vec<LoadedScenario> {
    load_manifest()
        .scenarios
        .into_iter()
        .map(|scenario| {
            let skeleton_path = fixture_dir().join(&scenario.skeleton_file);
            let skeleton_yaml = std::fs::read_to_string(&skeleton_path)
                .unwrap_or_else(|err| panic!("read {skeleton_path:?}: {err}"));
            let skeleton = load_repair_preview_skeleton(&skeleton_yaml)
                .unwrap_or_else(|err| panic!("parse {skeleton_path:?}: {err}"));
            let comparisons = scenario
                .comparison_files
                .iter()
                .map(|file| {
                    let path = fixture_dir().join(file);
                    let yaml = std::fs::read_to_string(&path)
                        .unwrap_or_else(|err| panic!("read {path:?}: {err}"));
                    load_repair_preview_comparison(&yaml)
                        .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
                })
                .collect::<Vec<_>>();
            LoadedScenario {
                scenario_id: scenario.scenario_id,
                skeleton,
                comparisons,
            }
        })
        .collect()
}

#[test]
fn beta_skeleton_corpus_validates_and_covers_blast_radius_and_compensation_classes() {
    let evaluator = RepairPreviewSkeletonEvaluator::new();
    let scenarios = load_scenarios();
    assert_eq!(scenarios.len(), 4);

    let mut blast_radius_seen = BTreeSet::new();
    let mut compensation_seen = BTreeSet::new();
    let mut checkpoint_seen = BTreeSet::new();
    let mut preview_disposition_seen = BTreeSet::new();
    let mut reversal_seen = BTreeSet::new();

    for scenario in &scenarios {
        let skeleton = &scenario.skeleton;
        evaluator
            .validate_skeleton(skeleton)
            .unwrap_or_else(|err| panic!("{}: {err:?}", scenario.scenario_id));

        assert_eq!(skeleton.record_kind, REPAIR_PREVIEW_SKELETON_RECORD_KIND);
        assert!(skeleton
            .preserved_state_classes
            .contains(&PreservedStateClass::UserAuthoredFiles));
        assert!(!skeleton.destructive_resets_present);
        assert!(skeleton.doctor_finding_ref.starts_with("doctor.finding."));
        assert!(
            skeleton
                .repair_transaction_ref
                .starts_with("repair_transaction:"),
            "skeleton {} must preserve the alpha repair_transaction:* id",
            skeleton.skeleton_id
        );

        if skeleton
            .checkpoint_disposition_class
            .requires_checkpoint_ref()
        {
            assert!(
                skeleton.checkpoint_ref.is_some(),
                "skeleton {} must name a checkpoint_ref",
                skeleton.skeleton_id
            );
        } else {
            assert!(
                skeleton.checkpoint_ref.is_none(),
                "skeleton {} must omit checkpoint_ref",
                skeleton.skeleton_id
            );
        }

        blast_radius_seen.insert(skeleton.blast_radius_class);
        compensation_seen.insert(skeleton.compensation_class);
        checkpoint_seen.insert(skeleton.checkpoint_disposition_class);
        preview_disposition_seen.insert(skeleton.preview_disposition_class);
        reversal_seen.insert(skeleton.reversal_class);

        for comparison in &scenario.comparisons {
            evaluator
                .validate_comparison(comparison)
                .unwrap_or_else(|err| panic!("{}: {err:?}", scenario.scenario_id));
            assert_eq!(
                comparison.record_kind,
                REPAIR_PREVIEW_COMPARISON_RECORD_KIND
            );
            evaluator
                .validate_comparison_against_skeleton(skeleton, comparison)
                .unwrap_or_else(|err| panic!("{}: {err:?}", scenario.scenario_id));
            assert!(comparison.is_consistent());
        }
    }

    assert_eq!(
        blast_radius_seen,
        [
            RepairBlastRadiusClass::SingleObjectClass,
            RepairBlastRadiusClass::MultiObjectClassSameFamily,
            RepairBlastRadiusClass::MultiObjectClassCrossFamily,
            RepairBlastRadiusClass::NoLocalBlastEscalationOnly,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        compensation_seen,
        [
            RepairCompensationClass::RegenerateFromAuthoritativeSource,
            RepairCompensationClass::SemanticInverseCompensation,
            RepairCompensationClass::ManualFollowupRequired,
            RepairCompensationClass::AuditOnlyNoStateChange,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
    assert!(checkpoint_seen.contains(&RepairCheckpointDispositionClass::DurablePreApplyCheckpoint));
    assert!(checkpoint_seen.contains(&RepairCheckpointDispositionClass::NoCheckpointEscalationOnly));
    assert!(reversal_seen.contains(&TransactionReversalClass::Regenerate));
    assert!(reversal_seen.contains(&TransactionReversalClass::Compensating));
    assert!(reversal_seen.contains(&TransactionReversalClass::Manual));
    assert!(reversal_seen.contains(&TransactionReversalClass::AuditOnly));
    assert!(
        preview_disposition_seen.contains(&RepairPreviewDispositionClass::CancellablePendingReview)
    );
    assert!(preview_disposition_seen.contains(&RepairPreviewDispositionClass::RefusedNoLocalRepair));
}

#[test]
fn beta_support_packet_preserves_transaction_id_and_reversal_class_and_is_export_safe() {
    let evaluator = RepairPreviewSkeletonEvaluator::new();
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let packet = evaluator
            .support_packet(
                format!("packet:{}", scenario.scenario_id),
                "2026-05-15T08:40:00Z",
                &scenario.skeleton,
                &scenario.comparisons,
            )
            .unwrap_or_else(|err| panic!("{}: {err:?}", scenario.scenario_id));
        assert_eq!(
            packet.record_kind,
            REPAIR_PREVIEW_SUPPORT_PACKET_RECORD_KIND
        );
        assert_eq!(packet.doc_ref, REPAIR_PREVIEW_SKELETON_DOC_REF);
        assert_eq!(packet.schema_ref, REPAIR_PREVIEW_SKELETON_SCHEMA_REF);
        assert_eq!(
            packet.repair_transaction_ref, scenario.skeleton.repair_transaction_ref,
            "packet must preserve the alpha transaction id verbatim"
        );
        assert_eq!(
            packet.reversal_class, scenario.skeleton.reversal_class,
            "packet must preserve the alpha reversal class verbatim"
        );
        assert_eq!(packet.comparison_rows.len(), scenario.comparisons.len());
        assert!(
            packet.is_export_safe(),
            "packet {} must be export-safe",
            scenario.scenario_id
        );
        assert!(!packet.destructive_resets_present);
    }
}

#[test]
fn beta_skeleton_compiles_from_every_alpha_transaction_seed_case() {
    let evaluator = RepairPreviewSkeletonEvaluator::new();
    let alpha = RepairAlpha::new();
    let seeds = current_alpha_repair_seed_cases().expect("alpha seeds parse");
    assert!(!seeds.is_empty());

    let mut blast_radius_seen = BTreeSet::new();
    let mut compensation_seen = BTreeSet::new();

    for seed in &seeds {
        let transaction = alpha
            .transaction_from_seed(seed)
            .unwrap_or_else(|err| panic!("transaction from {}: {err}", seed.case_id));

        let skeleton = evaluator.from_alpha_transaction(
            &transaction,
            format!("repair_preview_skeleton:{}", seed.case_id),
            transaction
                .initiating_finding_codes
                .first()
                .cloned()
                .expect("seed cites a doctor finding"),
            format!("support:repair-preview-beta:{}", seed.case_id),
            "2026-05-15T08:35:00Z",
        );

        evaluator
            .validate_skeleton(&skeleton)
            .unwrap_or_else(|err| panic!("compile-and-validate {}: {err:?}", seed.case_id));

        assert_eq!(skeleton.repair_transaction_ref, seed.repair_transaction_id);
        assert_eq!(skeleton.reversal_class, seed.transaction_reversal_class);

        // Apply-mode → checkpoint disposition truth.
        match transaction.apply_mode_class {
            ApplyModeClass::ApplyWithCheckpoint | ApplyModeClass::ApplyWithRollbackOnFailure => {
                assert_eq!(
                    skeleton.checkpoint_disposition_class,
                    RepairCheckpointDispositionClass::DurablePreApplyCheckpoint
                );
                assert!(skeleton.checkpoint_ref.is_some());
            }
            ApplyModeClass::ApplyRefusedEscalationOnly => {
                assert_eq!(
                    skeleton.checkpoint_disposition_class,
                    RepairCheckpointDispositionClass::NoCheckpointEscalationOnly
                );
                assert_eq!(
                    skeleton.preview_disposition_class,
                    RepairPreviewDispositionClass::RefusedNoLocalRepair
                );
            }
            ApplyModeClass::DryRunPreviewOnly | ApplyModeClass::ApplyObserveOnlyNoWrite => {
                assert_eq!(
                    skeleton.checkpoint_disposition_class,
                    RepairCheckpointDispositionClass::NoCheckpointObserveOnly
                );
            }
        }

        // Repair-class family → blast radius coherence.
        if seed.repair_class_family == RepairClassFamily::GuidedExportEscalation {
            assert_eq!(
                skeleton.blast_radius_class,
                RepairBlastRadiusClass::NoLocalBlastEscalationOnly
            );
            assert!(skeleton.affected_object_rows.is_empty());
        } else {
            assert!(
                !skeleton.affected_object_rows.is_empty(),
                "non-escalation skeleton must list at least one affected object",
            );
        }

        blast_radius_seen.insert(skeleton.blast_radius_class);
        compensation_seen.insert(skeleton.compensation_class);
    }

    assert!(blast_radius_seen.contains(&RepairBlastRadiusClass::NoLocalBlastEscalationOnly));
    assert!(compensation_seen.contains(&RepairCompensationClass::RegenerateFromAuthoritativeSource));
    assert!(compensation_seen.contains(&RepairCompensationClass::SemanticInverseCompensation));
    assert!(compensation_seen.contains(&RepairCompensationClass::AuditOnlyNoStateChange));
}

#[test]
fn beta_evaluator_refuses_destructive_or_inconsistent_skeletons_and_comparisons() {
    let evaluator = RepairPreviewSkeletonEvaluator::new();
    let baseline = load_scenarios()
        .into_iter()
        .find(|scenario| scenario.scenario_id == "cache_index_rebuild_single_object")
        .expect("baseline scenario");
    let skeleton = baseline.skeleton.clone();

    // Destructive reset refused.
    let mut destructive = skeleton.clone();
    destructive.destructive_resets_present = true;
    let err = evaluator.validate_skeleton(&destructive).unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.destructive_reset_declared"));

    // user_authored_files preservation removed.
    let mut without_authored = skeleton.clone();
    without_authored
        .preserved_state_classes
        .retain(|class| *class != PreservedStateClass::UserAuthoredFiles);
    let err = evaluator.validate_skeleton(&without_authored).unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id
            == "repair_preview.user_authored_files_must_be_preserved"));

    // Checkpoint disposition / ref mismatch.
    let mut missing_checkpoint = skeleton.clone();
    missing_checkpoint.checkpoint_ref = None;
    let err = evaluator
        .validate_skeleton(&missing_checkpoint)
        .unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.checkpoint_ref_required"));

    let mut spurious_checkpoint = skeleton.clone();
    spurious_checkpoint.checkpoint_disposition_class =
        RepairCheckpointDispositionClass::NoCheckpointObserveOnly;
    let err = evaluator
        .validate_skeleton(&spurious_checkpoint)
        .unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.checkpoint_ref_unexpected"));

    // Blast radius / affected-object coherence.
    let mut wrong_blast = skeleton.clone();
    wrong_blast.blast_radius_class = RepairBlastRadiusClass::NoLocalBlastEscalationOnly;
    let err = evaluator.validate_skeleton(&wrong_blast).unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.blast_radius_object_mismatch"));

    // Audit-only compensation must not list affected objects.
    let mut wrong_audit = skeleton.clone();
    wrong_audit.compensation_class = RepairCompensationClass::AuditOnlyNoStateChange;
    let err = evaluator.validate_skeleton(&wrong_audit).unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.audit_only_lists_affected_objects"));

    // Authorized + strong-compensation must travel through comparison first.
    let mut authorized_compensating = skeleton.clone();
    authorized_compensating.compensation_class =
        RepairCompensationClass::SemanticInverseCompensation;
    authorized_compensating.preview_disposition_class =
        RepairPreviewDispositionClass::AuthorizedForApply;
    let err = evaluator
        .validate_skeleton(&authorized_compensating)
        .unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.authorized_requires_comparison"));

    // Comparison empty axes / wrong-binding rejection.
    let mut comparison = baseline.comparisons[0].clone();
    comparison.differing_axes.clear();
    let err = evaluator.validate_comparison(&comparison).unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.comparison_axes_empty"));

    let mut wrong_bound = baseline.comparisons[0].clone();
    wrong_bound.bound_skeleton_ref =
        "repair_preview_skeleton:other_transaction.other_case".to_owned();
    let err = evaluator
        .validate_comparison_against_skeleton(&skeleton, &wrong_bound)
        .unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.comparison_bound_ref_mismatch"));

    // Mixed-skeleton packet refused.
    let other_scenario = load_scenarios()
        .into_iter()
        .find(|scenario| scenario.scenario_id == "extension_quarantine_compensating")
        .expect("other scenario");
    let cross_comparisons = other_scenario.comparisons.clone();
    let err = evaluator
        .support_packet(
            "packet:mixed",
            "2026-05-15T08:45:00Z",
            &skeleton,
            &cross_comparisons,
        )
        .unwrap_err();
    assert!(err
        .violations
        .iter()
        .any(|violation| violation.check_id == "repair_preview.comparison_bound_ref_mismatch"));
}

#[test]
fn beta_support_packet_axes_cover_every_comparison_axis_class() {
    let evaluator = RepairPreviewSkeletonEvaluator::new();
    let scenarios = load_scenarios();
    let mut axes_seen = BTreeSet::new();
    let mut cancellations_seen = BTreeSet::new();
    for scenario in &scenarios {
        let packet = evaluator
            .support_packet(
                format!("packet:axes:{}", scenario.scenario_id),
                "2026-05-15T08:50:00Z",
                &scenario.skeleton,
                &scenario.comparisons,
            )
            .unwrap();
        for row in &packet.comparison_rows {
            for axis in &row.differing_axes {
                axes_seen.insert(*axis);
            }
            cancellations_seen.insert(row.cancellation_class);
        }
    }
    for required in [
        RepairComparisonAxisClass::BlastRadiusDiff,
        RepairComparisonAxisClass::CompensationDiff,
        RepairComparisonAxisClass::AffectedObjectDiff,
        RepairComparisonAxisClass::PreservedStateDiff,
        RepairComparisonAxisClass::CheckpointDiff,
        RepairComparisonAxisClass::ReversalClassDiff,
    ] {
        assert!(
            axes_seen.contains(&required),
            "fixture corpus must cover {:?}",
            required
        );
    }
    for required in [
        RepairCancellationClass::ContinueComparison,
        RepairCancellationClass::CancelBeforeApply,
        RepairCancellationClass::ReadyForApplyReview,
    ] {
        assert!(
            cancellations_seen.contains(&required),
            "fixture corpus must cover cancellation {:?}",
            required
        );
    }
}
