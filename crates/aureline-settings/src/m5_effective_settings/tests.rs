//! Unit tests for the M5 effective-settings gate and its fail-closed guards.

use super::corpus::m5_effective_settings_corpus;
use super::model::{
    BuildError, EffectiveSettingsClaim, HighImpactClass, M5EffectiveSettingsCertification,
    M5EffectiveSettingsInput, M5SettingFamily, PolicyLockState, RowTrust, ValidationState,
    WriteEffect,
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
        locked: true,
        policy_ref: Some("aureline://policy/lock".to_owned()),
    };
    row.write_preview.as_mut().unwrap().effective_after_write = WriteEffect::BecomesWinningValue;
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::WriteEffectContradictsLock { setting_id });
}

#[test]
fn policy_locked_row_requires_a_policy_ref() {
    let mut input = baseline_input();
    let row = input.setting_rows.iter_mut().next().expect("a row");
    row.policy_lock = PolicyLockState {
        locked: true,
        policy_ref: None,
    };
    let setting_id = row.setting_id.clone();
    let err = M5EffectiveSettingsCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::PolicyLockedWithoutRef { setting_id });
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
fn corpus_covers_every_family_high_impact_class_and_dependency_kind() {
    use std::collections::BTreeSet;
    let mut families = BTreeSet::new();
    let mut impacts = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    let mut restarts = BTreeSet::new();
    for scenario in m5_effective_settings_corpus() {
        let record = scenario.record();
        families.extend(record.family_coverage);
        impacts.extend(record.high_impact_coverage);
        kinds.extend(record.lifecycle_dependency_coverage);
        restarts.extend(record.restart_posture_coverage);
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
    assert_eq!(restarts.len(), 5, "every restart posture should appear");
}
