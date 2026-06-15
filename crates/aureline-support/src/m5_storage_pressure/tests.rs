use super::*;

use crate::m5_storage_governance::current_m5_artifact_family_storage_matrix;

fn corpus() -> StoragePressureBannerCorpus {
    current_storage_pressure_banner_corpus().expect("corpus parses")
}

fn matrix() -> M5ArtifactFamilyStorageMatrix {
    current_m5_artifact_family_storage_matrix().expect("matrix parses")
}

#[test]
fn corpus_parses_every_fixture() {
    let corpus = corpus();
    assert_eq!(corpus.banners.len(), BANNER_FIXTURES.len());
}

#[test]
fn corpus_validates_against_the_safety_contract() {
    let corpus = corpus();
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn every_banner_is_export_safe() {
    let corpus = corpus();
    for entry in &corpus.banners {
        assert!(
            entry.banner.is_export_safe(),
            "{} must be export-safe",
            entry.banner.banner_id
        );
        assert!(!entry.banner.authoritative_state_loss);
    }
}

#[test]
fn every_banner_lists_the_full_frozen_ladder_in_order() {
    let corpus = corpus();
    for entry in &corpus.banners {
        let banner = &entry.banner;
        assert_eq!(
            banner.eviction_order.len(),
            FROZEN_LADDER.len(),
            "{}",
            banner.banner_id
        );
        for (index, step) in banner.eviction_order.iter().enumerate() {
            assert_eq!(
                step.ladder_step, FROZEN_LADDER[index],
                "{}",
                banner.banner_id
            );
            assert_eq!(step.ladder_order, (index as u32) + 1);
        }
    }
}

#[test]
fn user_owned_recovery_step_is_never_auto_applied() {
    let corpus = corpus();
    for entry in &corpus.banners {
        let step = entry
            .banner
            .step(LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview)
            .expect("recovery step present");
        assert!(
            !step.applied,
            "{} auto-applied recovery",
            entry.banner.banner_id
        );
        assert!(step.requires_reviewed_escalation);
        assert!(step.protected);
    }
}

#[test]
fn user_owned_recovery_state_always_reclaims_zero_bytes() {
    let corpus = corpus();
    for entry in &corpus.banners {
        let guard = entry
            .banner
            .guard(StorageClassId::UserOwnedRecoveryState)
            .expect("recovery guard present");
        assert_eq!(
            guard.reclaimed_bytes, 0,
            "{} reclaimed recovery bytes",
            entry.banner.banner_id
        );
        assert!(guard.holds);
        assert!(guard.guard_class.protects_authoritative_state());
    }
}

#[test]
fn user_owned_recovery_state_is_always_protected_not_trimmed() {
    let corpus = corpus();
    for entry in &corpus.banners {
        assert!(
            entry
                .banner
                .protected_class_ids_not_trimmed
                .contains(&StorageClassId::UserOwnedRecoveryState),
            "{}",
            entry.banner.banner_id
        );
    }
}

#[test]
fn evidence_only_expires_unpinned_past_retention_at_protect_core() {
    let corpus = corpus();
    for entry in &corpus.banners {
        let banner = &entry.banner;
        let guard = banner
            .guard(StorageClassId::EvidenceSupportCache)
            .expect("evidence guard present");
        if banner.pressure_class == PressureClass::ProtectCore {
            assert_eq!(
                guard.guard_class,
                StateLossGuardClass::UnpinnedEvidenceExpiredPinnedAndInWindowRetained
            );
        } else {
            assert_eq!(
                guard.guard_class,
                StateLossGuardClass::ProtectedEvidenceFullyRetained
            );
            assert_eq!(guard.reclaimed_bytes, 0, "{}", banner.banner_id);
        }
    }
}

#[test]
fn every_banner_discloses_the_two_pause_steps_and_open_inspector_action() {
    let corpus = corpus();
    for entry in &corpus.banners {
        let banner = &entry.banner;
        assert!(banner
            .paused_work
            .contains(&PausedWorkClass::SpeculativeFetchAndPrefetch));
        assert!(banner
            .paused_work
            .contains(&PausedWorkClass::ManagedReplicationAndPackRefresh));
        assert_eq!(
            banner.open_inspector_action_ref,
            OPEN_STORAGE_INSPECTOR_ACTION_REF
        );
    }
}

#[test]
fn pending_escalation_never_reclaims_protected_bytes() {
    let corpus = corpus();
    for entry in &corpus.banners {
        let banner = &entry.banner;
        if banner.escalation_state != EscalationStateClass::ReviewedEscalationRequiredNotYetApproved
        {
            continue;
        }
        for guard in &banner.state_loss_guards {
            if guard.is_protected_class() {
                assert_eq!(
                    guard.reclaimed_bytes, 0,
                    "{} reclaimed protected bytes under pending escalation",
                    banner.banner_id
                );
            }
        }
    }
}

#[test]
fn composer_matches_the_seeded_constrained_banner() {
    let matrix = matrix();
    let signal = PressureSignal {
        banner_id: "storage_pressure.low_disk_constrained.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        pressure_class: PressureClass::Constrained,
        pressure_source: PressureSourceClass::LowDiskFloor,
        scope_ref: "ws.alpha".to_owned(),
        scope_label: "Project Alpha".to_owned(),
        observations: vec![
            ClassObservation {
                class_id: StorageClassId::InteractiveHotCache,
                reclaimable_unpinned_bytes: 1_200_000_000,
                pinned_or_in_window_bytes: 0,
                unpinned_past_retention_bytes: 0,
            },
            ClassObservation {
                class_id: StorageClassId::KnowledgeCache,
                reclaimable_unpinned_bytes: 3_500_000_000,
                pinned_or_in_window_bytes: 800_000_000,
                unpinned_past_retention_bytes: 0,
            },
            ClassObservation {
                class_id: StorageClassId::ArtifactCache,
                reclaimable_unpinned_bytes: 2_000_000_000,
                pinned_or_in_window_bytes: 1_000_000_000,
                unpinned_past_retention_bytes: 0,
            },
            ClassObservation {
                class_id: StorageClassId::PrebuildEnvironmentCache,
                reclaimable_unpinned_bytes: 4_000_000_000,
                pinned_or_in_window_bytes: 2_000_000_000,
                unpinned_past_retention_bytes: 0,
            },
            ClassObservation {
                class_id: StorageClassId::EvidenceSupportCache,
                reclaimable_unpinned_bytes: 0,
                pinned_or_in_window_bytes: 900_000_000,
                unpinned_past_retention_bytes: 0,
            },
            ClassObservation {
                class_id: StorageClassId::UserOwnedRecoveryState,
                reclaimable_unpinned_bytes: 0,
                pinned_or_in_window_bytes: 1_500_000_000,
                unpinned_past_retention_bytes: 0,
            },
        ],
        only_protected_over_ceiling: false,
    };
    let composed = compose_banner(&matrix, &signal);
    let seeded = corpus()
        .banner("storage_pressure.low_disk_constrained.v1")
        .expect("seeded banner present")
        .clone();
    assert_eq!(composed, seeded);
}

#[test]
fn composer_never_auto_trims_recovery_even_at_protect_core() {
    let matrix = matrix();
    let signal = PressureSignal {
        banner_id: "composed.protect_core".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        pressure_class: PressureClass::ProtectCore,
        pressure_source: PressureSourceClass::LowDiskFloor,
        scope_ref: "ws.gamma".to_owned(),
        scope_label: "Project Gamma".to_owned(),
        observations: vec![ClassObservation {
            class_id: StorageClassId::UserOwnedRecoveryState,
            reclaimable_unpinned_bytes: 0,
            pinned_or_in_window_bytes: 9_000_000_000,
            unpinned_past_retention_bytes: 0,
        }],
        only_protected_over_ceiling: false,
    };
    let banner = compose_banner(&matrix, &signal);
    assert!(banner.validate_into_is_clean());
    let recovery = banner
        .guard(StorageClassId::UserOwnedRecoveryState)
        .expect("recovery guard");
    assert_eq!(recovery.reclaimed_bytes, 0);
    assert_eq!(recovery.retained_bytes, 9_000_000_000);
    // The recovery step is present but never applied.
    let step = banner
        .step(LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview)
        .expect("recovery step");
    assert!(!step.applied);
}

#[test]
fn support_export_matches_checked_in_golden() {
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_storage_pressure.v1",
        "2026-06-14T00:00:00Z",
    );
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_storage_pressure/support_export.golden.json"
    ));
    let golden: StoragePressureBannerSupportExport =
        serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support --example \
         dump_m5_storage_pressure_support_export`"
    );
    assert!(export.is_export_safe());
}

#[test]
fn support_export_is_metadata_safe_and_reports_no_loss() {
    let corpus = corpus();
    let export = corpus.support_export("envelope.test", "2026-06-14T00:00:00Z");
    assert!(!export.raw_content_exported);
    assert_eq!(export.redaction_class, METADATA_SAFE_DEFAULT);
    assert_eq!(export.authoritative_state_loss_count, 0);
    assert_eq!(export.pressure_event_count, corpus.banners.len() as u32);
    assert_eq!(export.escalation_pending_count, 1);
}

// Test helper: a banner composed by `compose_banner` must validate cleanly.
impl StoragePressureBanner {
    fn validate_into_is_clean(&self) -> bool {
        let mut violations = Vec::new();
        self.validate_into(&mut violations, "composed");
        violations.is_empty()
    }
}
