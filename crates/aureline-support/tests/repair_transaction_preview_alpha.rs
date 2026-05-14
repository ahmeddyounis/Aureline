//! Protected tests for repair-transaction previews and journaling.

use std::path::{Path, PathBuf};

use aureline_support::repair::{
    current_alpha_repair_seed_cases, ApplyModeClass, CheckpointClass, ConfirmationClass,
    ForbiddenActionClass, OutcomeClass, PreviewStateClass, RepairAlpha, RepairClassFamily,
    RepairPreviewRequest, RepairSeedCase, TransactionReversalClass,
    REPAIR_MUTATION_JOURNAL_RECORD_KIND, REPAIR_OUTCOME_RECORD_KIND, REPAIR_PREVIEW_RECORD_KIND,
    REPAIR_SUPPORT_PACKET_RECORD_KIND, REPAIR_TRANSACTION_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    case_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/support/repair_preview_alpha")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_seed_cases() -> Vec<RepairSeedCase> {
    load_manifest()
        .case_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn checked_in_repair_seed_cases_compile_to_previewable_transactions() {
    let alpha = RepairAlpha::new();
    let seed_cases = load_seed_cases();
    assert_eq!(
        seed_cases,
        current_alpha_repair_seed_cases().expect("embedded cases parse")
    );
    assert_eq!(seed_cases.len(), 6);

    for seed in &seed_cases {
        assert_eq!(
            seed.validate(),
            Vec::new(),
            "seed {} validates",
            seed.case_id
        );
        let transaction = alpha
            .transaction_from_seed(seed)
            .unwrap_or_else(|err| panic!("compile {}: {err}", seed.case_id));
        assert_eq!(transaction.record_kind, REPAIR_TRANSACTION_RECORD_KIND);
        assert_eq!(transaction.validate(), Vec::new());
        assert_eq!(
            transaction.repair_transaction_id,
            seed.repair_transaction_id
        );
        assert_eq!(
            transaction.initiating_finding_codes,
            seed.initiating_finding_codes
        );

        let preview =
            transaction.compile_preview(RepairPreviewRequest::review_only("2026-05-14T08:00:00Z"));
        assert_eq!(preview.record_kind, REPAIR_PREVIEW_RECORD_KIND);
        assert_eq!(
            preview.repair_transaction_ref,
            transaction.repair_transaction_id
        );
        assert_eq!(
            preview.claimed_reversal_class,
            transaction.transaction_reversal_class
        );
        assert!(preview.exposes_blast_radius());
        assert!(!preview.authorizes_apply());
        assert!(preview
            .preserved_assertion_rows
            .iter()
            .any(|row| row.preserved_state_class
                == aureline_support::repair::PreservedStateClass::UserAuthoredFiles));

        if transaction.apply_mode_class.requires_checkpoint() {
            assert_eq!(
                preview.checkpoint_proposal.checkpoint_class,
                CheckpointClass::DurablePreApply
            );
            assert_eq!(
                preview.checkpoint_proposal.checkpoint_ref,
                transaction.checkpoint_ref
            );
        } else {
            assert_eq!(preview.checkpoint_proposal.checkpoint_ref, None);
        }
    }
}

#[test]
fn non_exact_repairs_require_strong_confirmation_before_apply_authorization() {
    let alpha = RepairAlpha::new();
    let seed = load_seed_cases()
        .into_iter()
        .find(|seed| seed.repair_class_family == RepairClassFamily::ExtensionIsolation)
        .expect("extension isolation seed");
    let transaction = alpha.transaction_from_seed(&seed).expect("transaction");
    assert_eq!(
        transaction.transaction_reversal_class,
        TransactionReversalClass::Compensating
    );

    let mut missing_strong =
        RepairPreviewRequest::authorized_for(&transaction, "2026-05-14T08:01:00Z");
    missing_strong.strong_confirmation_ack = false;
    let preview = transaction.compile_preview(missing_strong);

    assert_eq!(
        preview.confirmation_requirement.confirmation_class,
        ConfirmationClass::StrongConfirmationRequired
    );
    assert!(
        preview
            .confirmation_requirement
            .stronger_confirmation_required
    );
    assert_eq!(
        preview.preview_state_class,
        PreviewStateClass::DryRunCompletePendingReview
    );
    assert!(!preview.authorizes_apply());

    let authorized = transaction.compile_preview(RepairPreviewRequest::authorized_for(
        &transaction,
        "2026-05-14T08:02:00Z",
    ));
    assert_eq!(
        authorized.preview_state_class,
        PreviewStateClass::DryRunSafeApplyAuthorized
    );
    assert!(authorized.authorizes_apply());

    let cache_seed = load_seed_cases()
        .into_iter()
        .find(|seed| seed.repair_class_family == RepairClassFamily::DisposableStateRebuild)
        .expect("cache seed");
    let cache_transaction = alpha
        .transaction_from_seed(&cache_seed)
        .expect("cache transaction");
    let cache_preview = cache_transaction.compile_preview(RepairPreviewRequest::authorized_for(
        &cache_transaction,
        "2026-05-14T08:03:00Z",
    ));
    assert_eq!(
        cache_preview.confirmation_requirement.confirmation_class,
        ConfirmationClass::StandardReview
    );
    assert!(cache_preview.authorizes_apply());
}

#[test]
fn applied_or_escalated_repairs_emit_outcome_journal_and_support_packet_linkage() {
    let alpha = RepairAlpha::new();
    let records = load_seed_cases()
        .iter()
        .map(|seed| {
            let transaction = alpha.transaction_from_seed(seed).expect("transaction");
            let request =
                if transaction.apply_mode_class == ApplyModeClass::ApplyRefusedEscalationOnly {
                    RepairPreviewRequest::review_only("2026-05-14T08:04:00Z")
                } else {
                    RepairPreviewRequest::authorized_for(&transaction, "2026-05-14T08:04:00Z")
                };
            let preview = transaction.compile_preview(request);
            let outcome = transaction.outcome_from_preview(
                &preview,
                aureline_support::repair::RepairOutcomeRequest::from_seed(
                    seed,
                    "2026-05-14T08:05:00Z",
                ),
            );
            let journal = transaction.journal_entry(&preview, &outcome, "2026-05-14T08:06:00Z");
            aureline_support::repair::JournaledRepairRecord {
                transaction,
                preview,
                outcome,
                journal,
            }
        })
        .collect::<Vec<_>>();

    for record in &records {
        assert_eq!(record.outcome.record_kind, REPAIR_OUTCOME_RECORD_KIND);
        assert_eq!(
            record.journal.record_kind,
            REPAIR_MUTATION_JOURNAL_RECORD_KIND
        );
        assert_eq!(
            record.outcome.journal_lineage.initiating_diagnosis_refs,
            record.transaction.initiating_finding_codes
        );
        assert_eq!(
            record.journal.initiating_diagnosis_refs,
            record.transaction.initiating_finding_codes
        );
        assert_eq!(record.journal.repair_preview_ref, record.preview.preview_id);
        assert_eq!(record.journal.repair_outcome_ref, record.outcome.outcome_id);
        assert!(record
            .journal
            .source_lineage
            .diagnosis_ref
            .starts_with("doctor.finding."));

        if record.transaction.apply_mode_class == ApplyModeClass::ApplyRefusedEscalationOnly {
            assert_eq!(record.outcome.outcome_class, OutcomeClass::EscalatedNoApply);
            assert_eq!(
                record.transaction.transaction_reversal_class,
                TransactionReversalClass::AuditOnly
            );
            assert_eq!(record.journal.checkpoint_refs, Vec::<String>::new());
            assert!(record.outcome.escalation_packet_ref.is_some());
        } else {
            assert_eq!(
                record.outcome.outcome_class,
                OutcomeClass::AppliedSuccessRecovered
            );
            assert!(record.outcome.forbidden_action_assertions_held);
            assert!(!record.outcome.applied_change_rows.is_empty());
            if record.transaction.apply_mode_class.requires_checkpoint() {
                assert_eq!(
                    record.outcome.checkpoint_used_ref,
                    record.transaction.checkpoint_ref
                );
                assert!(!record.journal.checkpoint_refs.is_empty());
            }
        }
    }

    let packet = alpha.support_packet(
        "support:repair-preview-alpha",
        "2026-05-14T08:07:00Z",
        &records,
    );
    assert_eq!(packet.record_kind, REPAIR_SUPPORT_PACKET_RECORD_KIND);
    assert!(packet.is_export_safe());
    assert_eq!(packet.rows.len(), records.len());
    assert!(packet
        .rows
        .iter()
        .any(|row| row.strong_confirmation_required));
}

#[test]
fn forbidden_boundary_refuses_before_any_apply_rows_are_emitted() {
    let alpha = RepairAlpha::new();
    let seed = load_seed_cases()
        .into_iter()
        .find(|seed| seed.repair_class_family == RepairClassFamily::DisposableStateRebuild)
        .expect("cache seed");
    let transaction = alpha.transaction_from_seed(&seed).expect("transaction");
    let mut request = RepairPreviewRequest::authorized_for(&transaction, "2026-05-14T08:08:00Z");
    request
        .forbidden_action_violations
        .push(ForbiddenActionClass::WidenWorkspaceTrust);
    let preview = transaction.compile_preview(request);

    assert_eq!(
        preview.preview_state_class,
        PreviewStateClass::DryRunRefusedWidensTrust
    );
    assert_eq!(preview.impacted_change_rows, Vec::new());

    let outcome = transaction.outcome_from_preview(
        &preview,
        aureline_support::repair::RepairOutcomeRequest::from_seed(&seed, "2026-05-14T08:09:00Z"),
    );
    assert_eq!(
        outcome.outcome_class,
        OutcomeClass::RefusedPreApplyWidensTrust
    );
    assert!(!outcome.forbidden_action_assertions_held);
    assert_eq!(
        outcome.forbidden_action_violations,
        vec![ForbiddenActionClass::WidenWorkspaceTrust]
    );
    assert_eq!(outcome.applied_change_rows, Vec::new());
}
