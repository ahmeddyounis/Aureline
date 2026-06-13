//! Unit tests for the M5 sync-and-device review gate and its fail-closed guards.

use super::corpus::m5_sync_and_device_review_corpus;
use super::model::{
    BuildError, BundleSyncTrust, ConflictClass, ConflictDisposition, DeviceAction, DrillKind,
    M5SyncAndDeviceReview, M5SyncAndDeviceReviewInput, SyncReviewClaim, SyncScopeFamily,
    TrustWideningClass,
};

fn baseline_input() -> M5SyncAndDeviceReviewInput {
    let scenario = m5_sync_and_device_review_corpus()
        .into_iter()
        .find(|s| s.scenario_id == "fully_synced_baseline")
        .expect("baseline scenario exists");
    let record = scenario.record();
    M5SyncAndDeviceReviewInput {
        record_id: record.record_id,
        as_of: record.as_of,
        summary: record.summary,
        scope_bundles: record.scope_bundles,
        device_actions: record.device_actions,
        drills: record.drills,
        surface_truth: record.surface_truth,
        lifecycle_bindings: record.lifecycle_bindings,
    }
}

#[test]
fn baseline_qualifies_for_fully_synced() {
    let record = M5SyncAndDeviceReview::build(baseline_input()).expect("builds");
    assert_eq!(
        record.trust_qualification.claim_class,
        SyncReviewClaim::FullySynced
    );
    assert!(record.trust_qualification.qualifies_fully_synced);
    assert_eq!(
        record.trust_qualification.effective_trust_ceiling,
        BundleSyncTrust::Synced
    );
    assert!(record.trust_qualification.narrowing_reasons.is_empty());
}

#[test]
fn every_required_family_must_have_a_bundle() {
    let mut input = baseline_input();
    input
        .scope_bundles
        .retain(|bundle| bundle.family != SyncScopeFamily::Profiler);
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingFamily {
            family: SyncScopeFamily::Profiler
        }
    );
}

#[test]
fn bundle_cannot_claim_remote_authority() {
    let mut input = baseline_input();
    let bundle = input.scope_bundles.iter_mut().next().expect("a bundle");
    bundle.local_authoritative = false;
    let bundle_id = bundle.bundle_id.clone();
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(err, BuildError::LocalStateNotAuthoritative { bundle_id });
}

#[test]
fn trust_widening_field_cannot_be_applied() {
    let mut input = baseline_input();
    let bundle = input
        .scope_bundles
        .iter_mut()
        .find(|b| b.family == SyncScopeFamily::ExtensionBundles)
        .expect("extension bundle");
    bundle.remote_synced = false;
    bundle.conflicts.push(super::model::FieldConflict {
        field_path: "permissions.granted_scopes".to_owned(),
        class: ConflictClass::SameKeyDivergent,
        disposition: ConflictDisposition::RemoteAppliedAfterReview,
        local_value_ref: "aureline://value/perm-local".to_owned(),
        remote_value_ref: Some("aureline://value/perm-remote".to_owned()),
        widens_trust: Some(TrustWideningClass::ExtensionPermission),
        requires_explicit_review: true,
        detail: "Would widen extension permissions.".to_owned(),
    });
    let bundle_id = bundle.bundle_id.clone();
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::TrustWideningSilentlyApplied {
            bundle_id,
            field_path: "permissions.granted_scopes".to_owned(),
        }
    );
}

#[test]
fn trust_widening_field_must_require_review() {
    let mut input = baseline_input();
    let bundle = input
        .scope_bundles
        .iter_mut()
        .find(|b| b.family == SyncScopeFamily::ExtensionBundles)
        .expect("extension bundle");
    bundle.remote_synced = false;
    bundle.conflicts.push(super::model::FieldConflict {
        field_path: "ai.egress_hosts".to_owned(),
        class: ConflictClass::SameKeyDivergent,
        disposition: ConflictDisposition::RemoteApplyBlocked,
        local_value_ref: "aureline://value/ai-local".to_owned(),
        remote_value_ref: Some("aureline://value/ai-remote".to_owned()),
        widens_trust: Some(TrustWideningClass::AiEgress),
        requires_explicit_review: false,
        detail: "Would open AI egress.".to_owned(),
    });
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert!(matches!(
        err,
        BuildError::TrustWideningSilentlyApplied { .. }
    ));
}

#[test]
fn policy_locked_field_cannot_apply_remote() {
    let mut input = baseline_input();
    let bundle = input
        .scope_bundles
        .iter_mut()
        .find(|b| b.family == SyncScopeFamily::DataApi)
        .expect("data api bundle");
    bundle.remote_synced = false;
    bundle.conflicts.push(super::model::FieldConflict {
        field_path: "egress.allowlist".to_owned(),
        class: ConflictClass::PolicyLocked,
        disposition: ConflictDisposition::RemoteAppliedAfterReview,
        local_value_ref: "aureline://value/egress-local".to_owned(),
        remote_value_ref: Some("aureline://value/egress-remote".to_owned()),
        widens_trust: None,
        requires_explicit_review: true,
        detail: "Policy-locked egress.".to_owned(),
    });
    let bundle_id = bundle.bundle_id.clone();
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::PolicyLockedFieldApplied {
            bundle_id,
            field_path: "egress.allowlist".to_owned(),
        }
    );
}

#[test]
fn opaque_conflict_without_field_path_is_rejected() {
    let mut input = baseline_input();
    let bundle = input.scope_bundles.iter_mut().next().expect("a bundle");
    bundle.conflicts.push(super::model::FieldConflict {
        field_path: "  ".to_owned(),
        class: ConflictClass::SameKeyDivergent,
        disposition: ConflictDisposition::AwaitingFieldReview,
        local_value_ref: "aureline://value/x-local".to_owned(),
        remote_value_ref: None,
        widens_trust: None,
        requires_explicit_review: true,
        detail: "no path".to_owned(),
    });
    let bundle_id = bundle.bundle_id.clone();
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(err, BuildError::ConflictNotFieldAware { bundle_id });
}

#[test]
fn device_action_must_keep_local_state_intact() {
    let mut input = baseline_input();
    let action = input.device_actions.iter_mut().next().expect("an action");
    action.local_state_intact = false;
    let device_ref = action.device_ref.clone();
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(err, BuildError::DeviceActionWipesLocalState { device_ref });
}

#[test]
fn device_action_without_audit_is_rejected() {
    let mut input = baseline_input();
    let action = input.device_actions.iter_mut().next().expect("an action");
    action.audit_ref = String::new();
    let device_ref = action.device_ref.clone();
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(err, BuildError::DeviceActionWithoutAudit { device_ref });
}

#[test]
fn every_required_device_action_must_be_present() {
    let mut input = baseline_input();
    input
        .device_actions
        .retain(|record| record.action != DeviceAction::Rotate);
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingDeviceAction {
            action: DeviceAction::Rotate
        }
    );
}

#[test]
fn drill_must_keep_local_authoritative() {
    let mut input = baseline_input();
    let drill = input.drills.iter_mut().next().expect("a drill");
    drill.local_authoritative = false;
    let kind = drill.kind;
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(err, BuildError::DrillNotLocalAuthoritative { kind });
}

#[test]
fn every_required_drill_must_be_present() {
    let mut input = baseline_input();
    input
        .drills
        .retain(|drill| drill.kind != DrillKind::LocalOnlyFallback);
    let err = M5SyncAndDeviceReview::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingDrill {
            kind: DrillKind::LocalOnlyFallback
        }
    );
}

#[test]
fn every_scope_bundle_keeps_a_canonical_lifecycle_binding() {
    let record = M5SyncAndDeviceReview::build(baseline_input()).expect("builds");
    assert_eq!(record.lifecycle_bindings.len(), record.scope_bundles.len());
    for bundle in &record.scope_bundles {
        let binding = record
            .lifecycle_bindings
            .iter()
            .find(|binding| binding.bundle_id == bundle.bundle_id)
            .expect("binding present");
        assert!(!binding.request_case_ref.trim().is_empty());
        assert!(!binding.export_job_ref.trim().is_empty());
        assert!(!binding.delete_case_ref.trim().is_empty());
    }
}

#[test]
fn trust_widening_drill_blocks_every_widening_field() {
    let scenario = m5_sync_and_device_review_corpus()
        .into_iter()
        .find(|s| s.scenario_id == "trust_widening_blocked_drill")
        .expect("trust-widening scenario");
    let record = scenario.record();
    assert_eq!(
        record.trust_qualification.effective_trust_ceiling,
        BundleSyncTrust::ReviewBlocked
    );
    for class in TrustWideningClass::ALL {
        assert!(
            record.trust_widening_coverage.contains(&class),
            "missing trust-widening class {class:?}"
        );
    }
    for bundle in &record.scope_bundles {
        for conflict in &bundle.conflicts {
            if conflict.widens_trust.is_some() {
                assert!(conflict.requires_explicit_review);
                assert_ne!(
                    conflict.disposition,
                    ConflictDisposition::RemoteAppliedAfterReview
                );
            }
        }
    }
}

#[test]
fn drills_stay_narrowed_but_remain_sound() {
    for scenario in m5_sync_and_device_review_corpus() {
        if scenario.scenario_id == "fully_synced_baseline" {
            continue;
        }
        let record = scenario.record();
        assert_ne!(
            record.trust_qualification.effective_trust_ceiling,
            BundleSyncTrust::Synced,
            "{} should be narrowed below synced",
            scenario.scenario_id
        );
        assert_eq!(
            record.trust_qualification.claim_class,
            SyncReviewClaim::NarrowedLocalAuthoritative,
            "{} should remain a sound narrowed record",
            scenario.scenario_id
        );
    }
}

#[test]
fn corpus_covers_every_family_conflict_class_and_drill() {
    use std::collections::BTreeSet;
    let mut families = BTreeSet::new();
    let mut classes = BTreeSet::new();
    let mut widenings = BTreeSet::new();
    let mut actions = BTreeSet::new();
    let mut drills = BTreeSet::new();
    for scenario in m5_sync_and_device_review_corpus() {
        let record = scenario.record();
        families.extend(record.family_coverage);
        classes.extend(record.conflict_class_coverage);
        widenings.extend(record.trust_widening_coverage);
        actions.extend(record.device_action_coverage);
        drills.extend(record.drill_coverage);
    }
    for family in SyncScopeFamily::REQUIRED {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    for class in ConflictClass::ALL {
        assert!(classes.contains(&class), "missing conflict class {class:?}");
    }
    for class in TrustWideningClass::ALL {
        assert!(widenings.contains(&class), "missing widening {class:?}");
    }
    for action in DeviceAction::REQUIRED {
        assert!(actions.contains(&action), "missing action {action:?}");
    }
    for kind in DrillKind::REQUIRED {
        assert!(drills.contains(&kind), "missing drill {kind:?}");
    }
}
