//! Fixture replay and invariant tests for M5 managed auth-and-recovery.

use aureline_auth::m5_auth_and_recovery::{
    is_canonical_object_ref, m5_auth_and_recovery_corpus, AuthEventKind, AuthRecoveryClaim,
    AuthSurface, ContinuityCeiling, CredentialClass, DrillCategory, DrillKind, M5AuthAndRecovery,
    SurfaceClass, M5_AUTH_AND_RECOVERY_RECORD_KIND, M5_AUTH_AND_RECOVERY_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/auth/m5_auth_and_recovery",
);

fn load_record(filename: &str) -> M5AuthAndRecovery {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_match_in_code_projection() {
    for scenario in m5_auth_and_recovery_corpus() {
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
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, M5_AUTH_AND_RECOVERY_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            M5_AUTH_AND_RECOVERY_SHARED_CONTRACT_REF
        );
        assert_eq!(
            record.trust_qualification.claim_class,
            scenario.expected_claim_class
        );
        assert_eq!(
            record.trust_qualification.effective_continuity_ceiling,
            scenario.expected_continuity_ceiling
        );
    }
}

#[test]
fn every_event_discloses_provider_and_handoff() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for kind in AuthEventKind::REQUIRED {
            let event = record
                .events
                .iter()
                .find(|event| event.kind == kind)
                .unwrap_or_else(|| panic!("{} missing {kind:?}", scenario.scenario_id));
            assert!(!event.provider_label.trim().is_empty());
            assert!(is_canonical_object_ref(&event.issuer_ref));
            assert!(is_canonical_object_ref(&event.handoff.return_route_ref));
            assert!(event.handoff.keyboard_complete_fallback);
        }
    }
}

#[test]
fn every_required_surface_is_represented() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for surface in AuthSurface::REQUIRED {
            assert!(
                record.surface_coverage.contains(&surface),
                "{} missing surface {surface:?}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn local_work_is_never_threatened_without_governance() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for event in &record.events {
            assert!(event.local_continuity.fully_preserved());
            if let Some(condition) = &event.active_condition {
                if condition.local_work_threatened {
                    assert!(
                        condition.governed_policy_exception_ref.is_some(),
                        "{}: ungoverned local-work threat",
                        scenario.scenario_id
                    );
                }
                assert!(
                    !condition.paused_capabilities.is_empty(),
                    "{}: opaque condition pauses nothing",
                    scenario.scenario_id
                );
                assert!(condition.keyboard_complete_fallback);
            }
        }
    }
}

#[test]
fn fallback_posture_is_never_embedded_or_captcha_on_stable() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if !record.profile_channel.forbids_embedded_recovery() {
            continue;
        }
        for event in &record.events {
            if let Some(condition) = &event.active_condition {
                assert!(
                    !condition.fallback_posture.is_embedded_or_captcha(),
                    "{}: embedded/CAPTCHA recovery on stable",
                    scenario.scenario_id
                );
            }
        }
    }
}

#[test]
fn credentials_are_export_excluded_and_in_protected_stores() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for class in [
            CredentialClass::RefreshToken,
            CredentialClass::DelegatedHandle,
            CredentialClass::SessionBroker,
        ] {
            let store = record
                .credential_stores
                .iter()
                .find(|store| store.credential_class == class)
                .unwrap_or_else(|| panic!("{} missing {class:?}", scenario.scenario_id));
            assert!(store.fully_export_excluded());
            assert!(store.store_class.is_protected_store());
        }
    }
}

#[test]
fn every_drill_is_local_preserving_keyboard_complete_and_categorized() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for kind in DrillKind::REQUIRED {
            let drill = record
                .drills
                .iter()
                .find(|drill| drill.kind == kind)
                .unwrap_or_else(|| panic!("{} missing {kind:?}", scenario.scenario_id));
            assert!(drill.local_preserved);
            assert!(drill.local_labeled);
            assert!(drill.keyboard_complete);
            assert!(!drill.categories.is_empty());
            assert!(!drill.expected_signal.trim().is_empty());
            assert!(!drill.recovery_path.trim().is_empty());
        }
        for category in DrillCategory::REQUIRED {
            assert!(
                record.drill_category_coverage.contains(&category),
                "{} missing category {category:?}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn all_required_surfaces_consume_the_same_record() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for surface in SurfaceClass::REQUIRED {
            let row = record
                .surface_truth
                .iter()
                .find(|row| row.surface_class == surface)
                .unwrap_or_else(|| panic!("{} missing {surface:?}", scenario.scenario_id));
            assert!(row.consumes_shared_record);
            assert!(row.shows_provider_disclosure);
            assert!(row.shows_paused_capabilities);
            assert!(row.shows_local_continuity);
            assert!(row.shows_fallback_posture);
            assert!(row.shows_drills);
        }
    }
}

#[test]
fn baseline_is_calm_and_drills_are_narrowed() {
    for scenario in m5_auth_and_recovery_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if scenario.scenario_id == "calm_managed_baseline" {
            assert_eq!(
                record.trust_qualification.claim_class,
                AuthRecoveryClaim::LocalFirstManagedSafe
            );
            assert!(record.trust_qualification.qualifies_local_first_safe);
            assert_eq!(
                record.trust_qualification.effective_continuity_ceiling,
                ContinuityCeiling::LocalFirstFull
            );
        } else {
            assert_eq!(
                record.trust_qualification.claim_class,
                AuthRecoveryClaim::NarrowedManagedDegraded
            );
            assert!(!record.trust_qualification.qualifies_local_first_safe);
            assert_eq!(
                record.trust_qualification.effective_continuity_ceiling,
                ContinuityCeiling::ManagedNarrowedLocalIntact
            );
        }
    }
}
