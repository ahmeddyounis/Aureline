//! Fixture replay and invariant tests for M5 effective-settings certification.

use aureline_settings::m5_effective_settings::{
    is_canonical_object_ref, m5_effective_settings_corpus, AuditFreshnessState,
    EffectiveSettingsClaim, M5EffectiveSettingsCertification, M5SettingFamily,
    PolicyConstraintState, ProjectionMode, RowTrust, SurfaceClass, ValidationState, WriteEffect,
    M5_EFFECTIVE_SETTINGS_RECORD_KIND, M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/m5/m5-effective-settings",
);

fn load_record(filename: &str) -> M5EffectiveSettingsCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_match_in_code_projection() {
    for scenario in m5_effective_settings_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        assert_eq!(
            on_disk,
            scenario.record(),
            "{} drifted",
            scenario.scenario_id
        );
    }
}

#[test]
fn record_identity_and_rollups_are_stable() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, M5_EFFECTIVE_SETTINGS_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF
        );
        assert_eq!(
            record.trust_qualification.claim_class,
            scenario.expected_claim_class
        );
        assert_eq!(
            record.trust_qualification.effective_trust_ceiling,
            scenario.expected_trust_ceiling
        );
    }
}

#[test]
fn every_family_is_present_with_an_explicit_winning_value() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for family in M5SettingFamily::REQUIRED {
            assert!(
                record.setting_rows.iter().any(|row| row.family == family),
                "{} missing {family:?}",
                scenario.scenario_id
            );
        }
        for row in &record.setting_rows {
            assert!(is_canonical_object_ref(&row.winning_value.value_ref));
            assert!(!row.winning_value.display.trim().is_empty());
            assert!(!row.title.trim().is_empty());
            assert!(row
                .effective_value_review
                .selected_keys
                .iter()
                .any(|key| key == &row.setting_id));
            assert!(row
                .effective_value_review
                .available_projection_modes
                .contains(&row.effective_value_review.active_projection_mode));
            for shadow in &row.shadow_chain {
                assert_ne!(
                    shadow.scope, row.winning_value.scope,
                    "{}: {} shadows its own winning scope",
                    scenario.scenario_id, row.setting_id
                );
            }
        }
    }
}

#[test]
fn high_impact_rows_are_scope_explicit_and_checkpointed() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.setting_rows {
            if !row.is_high_impact() {
                continue;
            }
            let preview = row.write_preview.as_ref().unwrap_or_else(|| {
                panic!(
                    "{}: {} lacks a preview",
                    scenario.scenario_id, row.setting_id
                )
            });
            assert!(is_canonical_object_ref(&preview.rollback_checkpoint_ref));
            assert!(preview.requires_confirmation);
            if preview.effective_after_write != WriteEffect::BecomesWinningValue {
                let explanation = preview.explanation.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{}: {} lacks an explanation",
                        scenario.scenario_id, row.setting_id
                    )
                });
                assert!(!explanation.local_safe_continuation.is_empty());
            }
        }
    }
}

#[test]
fn policy_locked_rows_never_preview_a_winning_write() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.setting_rows {
            if row.policy_lock.constraint_state == PolicyConstraintState::Locked {
                assert!(row.policy_lock.policy_ref.is_some());
                assert!(
                    row.policy_lock.source_bundle_ref.is_some()
                        || row.policy_lock.source_scope_ref.is_some()
                );
                if let Some(preview) = &row.write_preview {
                    assert_ne!(
                        preview.effective_after_write,
                        WriteEffect::BecomesWinningValue,
                        "{}: locked {} previews a winning write",
                        scenario.scenario_id,
                        row.setting_id
                    );
                }
            }
        }
    }
}

#[test]
fn constrained_rows_publish_explanations_and_local_safe_paths() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.setting_rows {
            if row.policy_lock.constraint_state == PolicyConstraintState::Constrained {
                assert!(row.policy_lock.policy_ref.is_some());
                assert!(
                    row.policy_lock.source_bundle_ref.is_some()
                        || row.policy_lock.source_scope_ref.is_some()
                );
                assert!(!row.policy_lock.local_safe_continuation.is_empty());
                let preview = row.write_preview.as_ref().expect("constrained row preview");
                assert_eq!(preview.effective_after_write, WriteEffect::ShadowedByPolicy);
                assert!(preview.explanation.is_some());
            }
        }
    }
}

#[test]
fn lifecycle_dependencies_are_visible_markers() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.setting_rows {
            if let Some(marker) = &row.lifecycle_dependency {
                assert!(marker.visible, "{}: hidden marker", scenario.scenario_id);
                assert!(is_canonical_object_ref(&marker.depends_on_ref));
                assert!(!marker.recovery_hint.trim().is_empty());
            }
        }
    }
}

#[test]
fn all_required_surfaces_consume_the_same_record() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for surface in SurfaceClass::REQUIRED {
            let row = record
                .surface_truth
                .iter()
                .find(|row| row.surface_class == surface)
                .unwrap_or_else(|| panic!("{} missing {surface:?}", scenario.scenario_id));
            assert!(row.consumes_shared_record);
            assert!(row.shows_winning_scope);
            assert!(row.shows_shadow_chain);
            assert!(row.shows_restart_posture);
            assert!(row.shows_projection_mode);
            assert!(row.shows_lifecycle_dependency);
            assert!(row.shows_write_preview);
            assert!(row.shows_write_explanation);
            assert!(row.shows_distribution_audit);
            assert!(row.shows_last_applied);
            assert!(row.shows_local_safe_continuation);
        }
    }
}

#[test]
fn distribution_audit_covers_every_family_with_projection_and_freshness() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for family in M5SettingFamily::REQUIRED {
            let audit = record
                .distribution_audit
                .iter()
                .find(|row| row.family == family)
                .unwrap_or_else(|| panic!("{} missing audit for {family:?}", scenario.scenario_id));
            assert!(is_canonical_object_ref(&audit.bundle_ref));
            assert!(is_canonical_object_ref(&audit.bundle_owner_ref));
            assert!(is_canonical_object_ref(&audit.policy_scope_ref));
            assert!(!audit.last_applied_at.trim().is_empty());
            assert!(!audit.last_validated_at.trim().is_empty());
            assert!(!audit.local_safe_continuation.is_empty());
        }
    }
}

#[test]
fn baseline_is_fully_active_and_drills_are_narrowed() {
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        match scenario.scenario_id {
            "fully_active_baseline" => {
                assert_eq!(
                    record.trust_qualification.claim_class,
                    EffectiveSettingsClaim::FullyActive
                );
                assert!(record.trust_qualification.qualifies_fully_active);
                assert_eq!(
                    record.trust_qualification.effective_trust_ceiling,
                    RowTrust::Active
                );
            }
            "stale_schema_drill" => {
                assert_eq!(
                    record.trust_qualification.effective_trust_ceiling,
                    RowTrust::Withheld
                );
                assert!(record
                    .setting_rows
                    .iter()
                    .any(|row| row.validation_state == ValidationState::SchemaStale));
                assert!(record
                    .distribution_audit
                    .iter()
                    .any(|row| row.freshness_state == AuditFreshnessState::Expired));
            }
            _ => {
                assert_eq!(
                    record.trust_qualification.claim_class,
                    EffectiveSettingsClaim::NarrowedActive
                );
                assert!(!record.trust_qualification.qualifies_fully_active);
            }
        }
    }
}

#[test]
fn corpus_exposes_source_effective_and_live_projection_modes() {
    let mut seen = std::collections::BTreeSet::new();
    for scenario in m5_effective_settings_corpus() {
        let record = load_record(&scenario.fixture_filename);
        seen.extend(record.projection_mode_coverage);
    }
    assert!(seen.contains(&ProjectionMode::Source));
    assert!(seen.contains(&ProjectionMode::Effective));
    assert!(seen.contains(&ProjectionMode::Live));
}
