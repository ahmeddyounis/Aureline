//! Fixture replay and invariant tests for profile-switch lifecycle certification.

use std::collections::BTreeSet;

use aureline_settings::stabilize_profile_switch_review_and_temporary_profile_lifecycle::{
    is_canonical_object_ref, profile_switch_lifecycle_corpus,
    validate_profile_switch_lifecycle_record, ApplyTimingClass, ArtifactExclusionClass,
    LocalAuthoritativeReason, ProfileDurabilityClass, ProfileSwitchLifecycleCertification,
    StableClaimClass, SurfaceClass, TemporaryProfileActionClass, PROFILE_SWITCH_REVIEW_RECORD_KIND,
    PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/m4/stabilize-profile-switch-review-and-temporary-profile-lifecycle",
);

fn load_record(filename: &str) -> ProfileSwitchLifecycleCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_match_in_code_projection() {
    for scenario in profile_switch_lifecycle_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        assert_eq!(
            on_disk,
            scenario.record(),
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-settings --bin aureline_settings_stabilize_profile_switch_review_and_temporary_profile_lifecycle -- emit-fixtures fixtures/settings/m4/stabilize-profile-switch-review-and-temporary-profile-lifecycle`",
            scenario.scenario_id
        );
    }
}

#[test]
fn record_identity_and_validation_are_stable() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, PROFILE_SWITCH_REVIEW_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF
        );
        assert_eq!(
            record.stable_qualification.claim_class,
            scenario.expected_claim_class
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable,
            scenario.expected_qualifies_stable
        );
        validate_profile_switch_lifecycle_record(&record)
            .unwrap_or_else(|errors| panic!("{} invalid: {errors:?}", scenario.scenario_id));
    }
}

#[test]
fn switch_review_shows_immediate_restart_exclusions_narrowing_and_checkpoint() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let review = &record.switch_review;
        assert!(is_canonical_object_ref(&review.from_profile_ref));
        assert!(is_canonical_object_ref(&review.to_profile_ref));
        assert!(is_canonical_object_ref(&review.change_summary_ref));
        assert!(is_canonical_object_ref(&review.rollback_banner_ref));
        assert!(review
            .immediate_changes
            .iter()
            .all(|row| row.apply_timing == ApplyTimingClass::Immediate));
        assert!(review
            .restart_required_changes
            .iter()
            .all(|row| row.apply_timing == ApplyTimingClass::RestartRequired));
        assert!(review.immediate_changes.len() >= 4);
        assert!(!review.restart_required_changes.is_empty());
        assert!(!review.excluded_machine_state.is_empty());
        assert!(review
            .excluded_machine_state
            .iter()
            .all(|row| row.separate_addendum_required));
        assert!(!review.narrowing_effects.is_empty());
        assert!(review.narrowing_effects.iter().all(|row| row.narrows_only));
        if review.creates_rollback_checkpoint {
            assert!(review
                .rollback_checkpoint_ref
                .as_ref()
                .is_some_and(|checkpoint| is_canonical_object_ref(checkpoint)));
        }
    }
}

#[test]
fn temporary_profiles_keep_discard_promote_and_compare_unambiguous() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let temporary = &record.temporary_profile;
        assert!(temporary.durability_class.requires_temporary_badge());
        assert!(matches!(
            temporary.durability_class,
            ProfileDurabilityClass::SessionOnly | ProfileDurabilityClass::DiscardedOnExit
        ));
        assert!(!temporary.badge_label.trim().is_empty());
        assert!(!temporary.lifetime_or_expiry.trim().is_empty());
        assert!(!temporary.restricted_persistence_rules.is_empty());
        assert!(temporary.state_boundary_visible);
        let actions: BTreeSet<_> = temporary
            .actions
            .iter()
            .map(|row| row.action_class)
            .collect();
        for required in TemporaryProfileActionClass::REQUIRED {
            assert!(actions.contains(&required), "missing {required:?}");
        }
        assert!(temporary.actions.iter().all(|row| {
            is_canonical_object_ref(&row.target_ref)
                && row.keyboard_reachable
                && row.persistence_effect_visible
        }));
    }
}

#[test]
fn profile_artifacts_are_text_diffable_and_secret_safe() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let excluded: BTreeSet<_> = record
            .artifact_boundaries
            .iter()
            .flat_map(|row| row.excluded_classes.iter().copied())
            .collect();
        for required in ArtifactExclusionClass::REQUIRED {
            assert!(excluded.contains(&required), "missing {required:?}");
        }
        assert!(record.artifact_boundaries.iter().all(|row| {
            is_canonical_object_ref(&row.artifact_ref)
                && row.text_based
                && row.diffable
                && row.exportable_without_forbidden_material
                && row.artifact_shape.ends_with(".json")
        }));
    }
}

#[test]
fn imports_and_sync_never_silently_widen_authority() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(!record.import_conflicts.is_empty());
        for conflict in &record.import_conflicts {
            assert!(conflict.field_aware && conflict.scope_aware);
            if conflict.would_widen_authority {
                assert!(
                    conflict.widening_refused,
                    "{} must refuse widening",
                    conflict.conflict_id
                );
            }
            assert!(!conflict.offered_choices.is_empty());
        }
    }
}

#[test]
fn durable_applies_have_summaries_and_rollback_checkpoints_or_narrow() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let all_checkpointed = record.apply_audit.iter().all(|row| {
            is_canonical_object_ref(&row.change_summary_ref)
                && (!row.durable_state_changed
                    || row
                        .rollback_checkpoint_ref
                        .as_ref()
                        .is_some_and(|checkpoint| {
                            is_canonical_object_ref(checkpoint) && row.rollback_inspectable
                        }))
        });
        if record.stable_qualification.qualifies_stable {
            assert!(all_checkpointed);
            assert_eq!(
                record.stable_qualification.claim_class,
                StableClaimClass::Stable
            );
        } else {
            assert!(!all_checkpointed);
            assert_ne!(
                record.stable_qualification.claim_class,
                StableClaimClass::Stable
            );
            assert!(!record.stable_qualification.narrowing_reasons.is_empty());
        }
    }
}

#[test]
fn sync_failures_degrade_to_local_authoritative_file_portability() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let reasons: BTreeSet<_> = record.sync_fallbacks.iter().map(|row| row.reason).collect();
        for required in LocalAuthoritativeReason::REQUIRED {
            assert!(reasons.contains(&required), "missing {required:?}");
        }
        assert!(record.sync_fallbacks.iter().all(|row| {
            row.local_durable_state_authoritative
                && row.file_based_portability_visible
                && row.no_hidden_cloud_authority_claim
        }));
    }
}

#[test]
fn required_surfaces_consume_shared_truth() {
    for scenario in profile_switch_lifecycle_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let surfaces: BTreeSet<_> = record
            .surface_truth
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for required in SurfaceClass::REQUIRED {
            assert!(surfaces.contains(&required), "missing {required:?}");
        }
        assert!(record.surface_truth.iter().all(|row| {
            is_canonical_object_ref(&row.record_ref)
                && row.consumes_shared_contract
                && row.shows_profile_state
                && row.shows_restart_delta_truth
                && row.shows_rollback_checkpoint
                && row.shows_local_authoritative_fallback
        }));
    }
}
