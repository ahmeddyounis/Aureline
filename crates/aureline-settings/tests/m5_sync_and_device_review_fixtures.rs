//! Fixture replay and invariant tests for M5 sync-and-device review.

use aureline_settings::m5_sync_and_device_review::{
    is_canonical_object_ref, m5_sync_and_device_review_corpus, BundleSyncTrust, ConflictClass,
    ConflictDisposition, DeviceAction, DrillKind, M5SyncAndDeviceReview, SurfaceClass,
    SyncReviewClaim, SyncScopeFamily, M5_SYNC_AND_DEVICE_REVIEW_RECORD_KIND,
    M5_SYNC_AND_DEVICE_REVIEW_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/m5/m5-sync-and-device-review",
);

fn load_record(filename: &str) -> M5SyncAndDeviceReview {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_match_in_code_projection() {
    for scenario in m5_sync_and_device_review_corpus() {
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
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, M5_SYNC_AND_DEVICE_REVIEW_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            M5_SYNC_AND_DEVICE_REVIEW_SHARED_CONTRACT_REF
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
fn every_family_has_a_typed_scope_bundle() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for family in SyncScopeFamily::REQUIRED {
            let bundle = record
                .scope_bundles
                .iter()
                .find(|bundle| bundle.family == family)
                .unwrap_or_else(|| panic!("{} missing {family:?}", scenario.scenario_id));
            assert!(bundle.bundle_schema_version >= 1);
            assert!(!bundle.title.trim().is_empty());
            assert!(bundle.local_authoritative);
            assert!(is_canonical_object_ref(
                &bundle.revisions.local_revision_ref
            ));
            assert!(is_canonical_object_ref(&bundle.source_device_ref));
            assert!(is_canonical_object_ref(&bundle.source_profile_ref));
        }
    }
}

#[test]
fn conflicts_are_field_aware_and_never_last_writer_wins() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for bundle in &record.scope_bundles {
            for conflict in &bundle.conflicts {
                assert!(!conflict.field_path.trim().is_empty());
                assert!(!conflict.detail.trim().is_empty());
                assert!(is_canonical_object_ref(&conflict.local_value_ref));
                // A policy-locked field never lands a remote value.
                if conflict.class == ConflictClass::PolicyLocked {
                    assert_ne!(
                        conflict.disposition,
                        ConflictDisposition::RemoteAppliedAfterReview
                    );
                }
            }
        }
    }
}

#[test]
fn trust_widening_fields_are_always_gated() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for bundle in &record.scope_bundles {
            for conflict in &bundle.conflicts {
                if conflict.widens_trust.is_some() {
                    assert!(
                        conflict.requires_explicit_review,
                        "{}: ungated widening",
                        scenario.scenario_id
                    );
                    assert_ne!(
                        conflict.disposition,
                        ConflictDisposition::RemoteAppliedAfterReview,
                        "{}: silently applied widening",
                        scenario.scenario_id
                    );
                }
            }
        }
    }
}

#[test]
fn device_actions_are_audited_and_keep_local_intact() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for action in DeviceAction::REQUIRED {
            let entry = record
                .device_actions
                .iter()
                .find(|record| record.action == action)
                .unwrap_or_else(|| panic!("{} missing {action:?}", scenario.scenario_id));
            assert!(is_canonical_object_ref(&entry.audit_ref));
            assert!(is_canonical_object_ref(&entry.device_ref));
            assert!(entry.local_state_intact);
        }
    }
}

#[test]
fn every_drill_keeps_local_authoritative_and_labeled() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for kind in DrillKind::REQUIRED {
            let drill = record
                .drills
                .iter()
                .find(|drill| drill.kind == kind)
                .unwrap_or_else(|| panic!("{} missing {kind:?}", scenario.scenario_id));
            assert!(drill.local_authoritative);
            assert!(drill.local_state_labeled);
            assert!(!drill.expected_signal.trim().is_empty());
            assert!(!drill.recovery_path.trim().is_empty());
        }
    }
}

#[test]
fn all_required_surfaces_consume_the_same_record() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for surface in SurfaceClass::REQUIRED {
            let row = record
                .surface_truth
                .iter()
                .find(|row| row.surface_class == surface)
                .unwrap_or_else(|| panic!("{} missing {surface:?}", scenario.scenario_id));
            assert!(row.consumes_shared_record);
            assert!(row.shows_scope_bundles);
            assert!(row.shows_field_conflicts);
            assert!(row.shows_device_actions);
            assert!(row.shows_local_only_fallback);
        }
    }
}

#[test]
fn baseline_is_fully_synced_and_drills_are_narrowed() {
    for scenario in m5_sync_and_device_review_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if scenario.scenario_id == "fully_synced_baseline" {
            assert_eq!(
                record.trust_qualification.claim_class,
                SyncReviewClaim::FullySynced
            );
            assert!(record.trust_qualification.qualifies_fully_synced);
            assert_eq!(
                record.trust_qualification.effective_trust_ceiling,
                BundleSyncTrust::Synced
            );
        } else {
            assert_eq!(
                record.trust_qualification.claim_class,
                SyncReviewClaim::NarrowedLocalAuthoritative
            );
            assert!(!record.trust_qualification.qualifies_fully_synced);
            assert_ne!(
                record.trust_qualification.effective_trust_ceiling,
                BundleSyncTrust::Synced
            );
        }
    }
}
