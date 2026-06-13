//! Unit tests for the M5 effective-settings gate and its fail-closed guards.

use super::corpus::m5_effective_settings_corpus;
use super::model::{
    BuildError, EffectiveSettingsClaim, HighImpactClass, M5EffectiveSettingsCertification,
    M5EffectiveSettingsInput, M5SettingFamily, PolicyConstraintState, PolicyLockState,
    ProjectionMode, ReviewAction, RowTrust, SurfaceClass, ValidationState, WriteEffect,
};

fn baseline_input() -> M5EffectiveSettingsInput {
    let scenario = m5_effective_settings_corpus()
        .into_iter()
        .find(|s| s.scenario_id == "fully_active_baseline")
        .expect("baseline scenario exists");
    let record = scenario.record();
    M5EffectiveSettingsInput {
        record_id: record.record_id,
        as_of: record.as_of,
        summary: record.summary,
        setting_rows: record.setting_rows,
        distribution_audit: record.distribution_audit,
        surface_truth: record.surface_truth,
    }
}

#[test]
fn baseline_qualifies_for_fully_active() {
    let record = M5EffectiveSettingsCertification::build(baseline_input()).expect("builds");
    assert_eq!(
        record.trust_qualification.claim_class,
        EffectiveSettingsClaim::FullyActive
    );
    assert!(record.trust_qualification.qualifies_fully_active);
    assert_eq!(
        record.trust_qualification.effective_trust_ceiling,
        RowTrust::Active
    );
    assert!(record.trust_qualification.narrowing_reasons.is_empty());
}

#[test]
fn every_required_family_must_be_present() {
    let mut input = baseline_input();
    input
        .setting_rows
        .retain(|row| row.family != M5SettingFamily::Sync);
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingFamily {
            family: M5SettingFamily::Sync
        }
    );
}

#[test]
fn every_required_family_must_have_distribution_audit() {
    let mut input = baseline_input();
    input
        .distribution_audit
        .retain(|row| row.family != M5SettingFamily::Sync);
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingDistributionAudit {
            family: M5SettingFamily::Sync
        }
    );
}

#[test]
fn high_impact_row_without_write_preview_is_rejected() {
    let mut input = baseline_input();
    let row = input
        .setting_rows
        .iter_mut()
        .find(|row| row.high_impact_class == Some(HighImpactClass::TrustBoundary))
        .expect("companion row");
    row.write_preview = None;
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::HighImpactWithoutWritePreview { setting_id }
    );
}

#[test]
fn high_impact_write_without_checkpoint_is_rejected() {
    let mut input = baseline_input();
    let row = input
        .setting_rows
        .iter_mut()
        .find(|row| row.is_high_impact())
        .expect("a high-impact row");
    row.write_preview.as_mut().unwrap().rollback_checkpoint_ref = String::new();
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::HighImpactWriteWithoutCheckpoint { setting_id }
    );
}

#[test]
fn policy_locked_row_cannot_preview_a_winning_write() {
    let mut input = baseline_input();
    let row = input
        .setting_rows
        .iter_mut()
        .find(|row| row.is_high_impact())
        .expect("a high-impact row");
    row.policy_lock = PolicyLockState {
        constraint_state: PolicyConstraintState::Locked,
        policy_ref: Some("aureline://policy/lock".to_owned()),
        source_bundle_ref: Some("aureline://policy-bundle/lock".to_owned()),
        source_scope_ref: None,
        bundle_owner_ref: Some("aureline://owner/policy".to_owned()),
        distribution_source: None,
        last_applied_at: Some("2026-06-12T08:00:00Z".to_owned()),
        review_due_at: None,
        expires_at: None,
        constraint_summary: Some("locked".to_owned()),
        local_safe_continuation: vec!["Inspect locally".to_owned()],
    };
    row.write_preview.as_mut().unwrap().effective_after_write = WriteEffect::BecomesWinningValue;
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::WriteEffectContradictsLock { setting_id });
}

#[test]
fn constrained_row_requires_source_bundle_or_scope() {
    let mut input = baseline_input();
    let row = input.setting_rows.iter_mut().next().expect("a row");
    row.policy_lock = PolicyLockState {
        constraint_state: PolicyConstraintState::Constrained,
        policy_ref: Some("aureline://policy/constraint".to_owned()),
        source_bundle_ref: None,
        source_scope_ref: None,
        bundle_owner_ref: None,
        distribution_source: None,
        last_applied_at: None,
        review_due_at: None,
        expires_at: None,
        constraint_summary: Some("constrained".to_owned()),
        local_safe_continuation: vec!["Inspect locally".to_owned()],
    };
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::PolicyConstrainedWithoutSource { setting_id }
    );
}

#[test]
fn constrained_write_requires_explanation() {
    let mut input = baseline_input();
    let row = input
        .setting_rows
        .iter_mut()
        .find(|row| row.setting_id == "data_api.outbound_egress_allowlist")
        .expect("data api row");
    row.policy_lock = PolicyLockState {
        constraint_state: PolicyConstraintState::Constrained,
        policy_ref: Some("aureline://policy/constraint".to_owned()),
        source_bundle_ref: Some("aureline://policy-bundle/constraint".to_owned()),
        source_scope_ref: None,
        bundle_owner_ref: Some("aureline://owner/policy".to_owned()),
        distribution_source: None,
        last_applied_at: Some("2026-06-12T08:00:00Z".to_owned()),
        review_due_at: None,
        expires_at: None,
        constraint_summary: Some("constrained".to_owned()),
        local_safe_continuation: vec!["Inspect locally".to_owned()],
    };
    let preview = row.write_preview.as_mut().expect("preview");
    preview.effective_after_write = WriteEffect::ShadowedByPolicy;
    preview.explanation = None;
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::MissingWriteExplanation { setting_id });
}

#[test]
fn review_sheet_must_include_setting_id() {
    let mut input = baseline_input();
    let row = input.setting_rows.iter_mut().next().expect("a row");
    row.effective_value_review.selected_keys.clear();
    row.effective_value_review
        .selected_keys
        .push("wrong.key".to_owned());
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::ReviewSheetMissingSettingId { setting_id });
}

#[test]
fn review_sheet_must_include_active_projection_mode() {
    let mut input = baseline_input();
    let row = input.setting_rows.iter_mut().next().expect("a row");
    row.effective_value_review.available_projection_modes = vec![ProjectionMode::Source];
    row.effective_value_review.active_projection_mode = ProjectionMode::Live;
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::ReviewSheetMissingProjectionMode { setting_id }
    );
}

#[test]
fn hidden_lifecycle_marker_is_rejected() {
    let mut input = baseline_input();
    let row = input.setting_rows.iter_mut().next().expect("a row");
    row.lifecycle_dependency = Some(super::model::LifecycleDependencyMarker {
        kind: super::model::LifecycleDependencyKind::MissingCapability,
        depends_on_ref: "aureline://capability/thing".to_owned(),
        narrows_behavior: "narrowed".to_owned(),
        recovery_hint: "install it".to_owned(),
        visible: false,
    });
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::LifecycleMarkerHidden { setting_id });
}

#[test]
fn winning_scope_in_shadow_chain_is_rejected() {
    let mut input = baseline_input();
    let row = input.setting_rows.iter_mut().next().expect("a row");
    let scope = row.winning_value.scope;
    row.shadow_chain.push(super::model::ShadowedCandidate {
        scope,
        value_ref: "aureline://value/dup".to_owned(),
        reason: super::model::ShadowReason::LowerPrecedence,
    });
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::WinningScopeShadowed { setting_id });
}

#[test]
fn stale_schema_row_withholds_trust() {
    let scenario = m5_effective_settings_corpus()
        .into_iter()
        .find(|s| s.scenario_id == "stale_schema_drill")
        .expect("stale scenario");
    let record = scenario.record();
    assert_eq!(
        record.trust_qualification.effective_trust_ceiling,
        RowTrust::Withheld
    );
    assert_eq!(
        record.trust_qualification.claim_class,
        EffectiveSettingsClaim::NarrowedActive
    );
    assert!(record
        .setting_rows
        .iter()
        .any(|row| row.validation_state == ValidationState::SchemaStale));
    assert!(record
        .distribution_audit
        .iter()
        .any(|row| row.freshness_state.as_str() == "expired"));
}

#[test]
fn drills_stay_narrowed_but_remain_resolvable() {
    for scenario in m5_effective_settings_corpus() {
        if scenario.scenario_id == "fully_active_baseline" {
            continue;
        }
        let record = scenario.record();
        assert_ne!(
            record.trust_qualification.effective_trust_ceiling,
            RowTrust::Active,
            "{} should be narrowed below active",
            scenario.scenario_id
        );
        assert_eq!(
            record.trust_qualification.claim_class,
            EffectiveSettingsClaim::NarrowedActive,
            "{} should remain a sound narrowed-active record",
            scenario.scenario_id
        );
    }
}

#[test]
fn corpus_covers_every_family_high_impact_class_dependency_kind_and_projection_mode() {
    use std::collections::BTreeSet;
    let mut families = BTreeSet::new();
    let mut impacts = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    let mut restarts = BTreeSet::new();
    let mut modes = BTreeSet::new();
    let mut surfaces = BTreeSet::new();
    let mut actions = BTreeSet::new();

    for scenario in m5_effective_settings_corpus() {
        let record = scenario.record();
        families.extend(record.family_coverage);
        impacts.extend(record.high_impact_coverage);
        kinds.extend(record.lifecycle_dependency_coverage);
        restarts.extend(record.restart_posture_coverage);
        modes.extend(record.projection_mode_coverage);
        surfaces.extend(
            record
                .surface_truth
                .into_iter()
                .map(|row| row.surface_class),
        );
        actions.extend(
            record
                .setting_rows
                .into_iter()
                .flat_map(|row| row.effective_value_review.available_actions),
        );
    }

    for family in M5SettingFamily::REQUIRED {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    for impact in [
        HighImpactClass::TrustBoundary,
        HighImpactClass::AiNetworkEgress,
        HighImpactClass::ExtensionBehavior,
        HighImpactClass::RemoteExposure,
        HighImpactClass::DestructiveAutomation,
    ] {
        assert!(impacts.contains(&impact), "missing impact {impact:?}");
    }
    for kind in [
        super::model::LifecycleDependencyKind::MissingCapability,
        super::model::LifecycleDependencyKind::LabsOrPreviewDependent,
    ] {
        assert!(kinds.contains(&kind), "missing kind {kind:?}");
    }
    for mode in [
        ProjectionMode::Source,
        ProjectionMode::Effective,
        ProjectionMode::Live,
    ] {
        assert!(modes.contains(&mode), "missing mode {mode:?}");
    }
    for surface in SurfaceClass::REQUIRED {
        assert!(surfaces.contains(&surface), "missing surface {surface:?}");
    }
    assert_eq!(restarts.len(), 5, "every restart posture should appear");
    assert!(actions.contains(&ReviewAction::OpenPolicyBundle));
}
