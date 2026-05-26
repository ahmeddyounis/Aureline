//! Unit tests for the entry hardening lineage projection.

use super::*;
use crate::entry::{
    build_project_entry_review, CloneDepthClass, CloneReviewOptions, EntryDestinationFacts,
    ImportReviewOptions, ProjectEntryReviewRequest,
};
use crate::{AdmissionSourceSurface, CleanupPosture, EntryVerb, ResultingMode, TargetKind};

fn clean_clone_request() -> ProjectEntryReviewRequest {
    ProjectEntryReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Clone,
        TargetKind::RemoteRepository,
        ResultingMode::CloneThenReview,
        "https://github.com/acme/payments.git",
    )
    .with_destination("~/Code/payments")
    .with_clone_options(CloneReviewOptions {
        clone_depth_class: CloneDepthClass::FullHistory,
        ..CloneReviewOptions::default()
    })
}

#[test]
fn clone_review_projects_stable_lineage() {
    let entry_review = build_project_entry_review(clean_clone_request());
    let record = project_entry_hardening_lineage("posture.clone.clean", &entry_review);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.verb_truth.entry_verb, EntryVerb::Clone);
    assert_eq!(
        record.target_kind_truth.topology_class,
        EntryTargetTopologyClass::DurableOpen
    );
    assert!(record.side_effect_posture.clone_never_grants_trust);
    assert!(record.side_effect_posture.dependency_restore_deferred);
    assert!(record.durable_checkpoint.set_up_later_offered);
    assert!(record.durable_checkpoint.open_minimal_offered);
}

#[test]
fn clone_with_partial_filter_yields_opened_sparse_topology() {
    let entry_review = build_project_entry_review(
        ProjectEntryReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            "https://github.com/acme/payments.git",
        )
        .with_destination("~/Code/payments")
        .with_clone_options(CloneReviewOptions {
            clone_depth_class: CloneDepthClass::PartialCloneFiltered,
            partial_filter_label: Some("blob:none".to_owned()),
            ..CloneReviewOptions::default()
        }),
    );
    let record = project_entry_hardening_lineage("posture.clone.sparse", &entry_review);
    assert_eq!(
        record.target_kind_truth.topology_class,
        EntryTargetTopologyClass::OpenedSparse
    );
    assert!(record.is_stable_qualified());
}

#[test]
fn import_inspect_only_yields_inspect_staging_topology() {
    let entry_review = build_project_entry_review(
        ProjectEntryReviewRequest::new(
            AdmissionSourceSurface::CommandPalette,
            EntryVerb::Import,
            TargetKind::PortableStatePackage,
            ResultingMode::InspectOnly,
            "~/Downloads/workspace.aureline-state.zip",
        )
        .with_import_options(ImportReviewOptions {
            schema_or_version_label: "portable state schema v1".to_owned(),
            cleanup_posture: CleanupPosture::NoCleanupRequired,
            ..ImportReviewOptions::default()
        }),
    );
    let record = project_entry_hardening_lineage("posture.import.inspect", &entry_review);
    assert!(record.is_stable_qualified());
    assert_eq!(
        record.target_kind_truth.topology_class,
        EntryTargetTopologyClass::InspectOnlyStaging
    );
    assert!(record.side_effect_posture.no_durable_write_before_review);
    assert!(
        record
            .side_effect_posture
            .no_state_rehydration_before_review
    );
}

#[test]
fn restore_resume_yields_restore_target_topology() {
    let entry_review = build_project_entry_review(ProjectEntryReviewRequest::new(
        AdmissionSourceSurface::StartCenter,
        EntryVerb::Restore,
        TargetKind::RecoveryCheckpoint,
        ResultingMode::RestoreLastSession,
        "checkpoint:last_session",
    ));
    let record = project_entry_hardening_lineage("posture.restore.last", &entry_review);
    assert_eq!(
        record.target_kind_truth.topology_class,
        EntryTargetTopologyClass::RestoreTarget
    );
    assert!(record.is_stable_qualified());
}

#[test]
fn duplicate_clone_destination_narrows_topology_to_parent_root_and_keeps_choice() {
    let entry_review = build_project_entry_review(
        ProjectEntryReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            "https://github.com/acme/payments.git",
        )
        .with_destination("/tmp/destination-already-cloned")
        .with_destination_facts(EntryDestinationFacts {
            previously_cloned_target_ref: Some(
                "previous_clone:github.com/acme/payments".to_owned(),
            ),
            policy_blocked: false,
        }),
    );
    let record = project_entry_hardening_lineage("posture.clone.duplicate", &entry_review);
    // A duplicate-clone target requires explicit choice; the record stays
    // Stable because the contract is working as designed.
    assert!(
        record
            .target_kind_truth
            .explicit_choice_required_when_colliding
    );
    assert!(record.is_stable_qualified());
}

#[test]
fn missing_compare_inspection_hook_narrows_record() {
    let entry_review = build_project_entry_review(clean_clone_request());
    let hooks = vec![
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::ReviewEntry,
            action_id: "entry_hardening.review_entry".to_owned(),
            label: "Review entry".to_owned(),
            available: false,
            disclosure: "Hook offline.".to_owned(),
        },
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::Export,
            action_id: "entry_hardening.export".to_owned(),
            label: "Export".to_owned(),
            available: true,
            disclosure: "Export hook reachable.".to_owned(),
        },
    ];
    let record = project_entry_hardening_lineage_with_hooks(
        "posture.clone.missing_hook",
        &entry_review,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&EntryHardeningNarrowReason::InspectionHookUnavailable));
}

#[test]
fn lineage_lines_render_every_pillar() {
    let entry_review = build_project_entry_review(clean_clone_request());
    let record = project_entry_hardening_lineage("posture.clone.lines", &entry_review);
    let lines = entry_hardening_lineage_lines(&record);
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Entry hardening lineage:")));
    assert!(lines.iter().any(|line| line.starts_with("Verb truth:")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Target-kind truth:")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Durable checkpoint:")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Side-effect posture:")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("Failure repair truth:")));
    assert!(lines.iter().any(|line| line.starts_with("Surface parity:")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}
