//! Fixture-replay and invariant tests for the stable sync / device-registry
//! certification corpus.
//!
//! The records live under
//! `fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support/` and
//! are minted by the `aureline_settings_sync_device_registry_stable` emitter so
//! the checked-in JSON stays a literal projection of the in-code corpus, which is
//! itself a projection of the live settings runtime (the seeded schema registry,
//! the resolver, and the beta sync conflict path).
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-settings \
//!     --bin aureline_settings_sync_device_registry_stable -- emit-fixtures \
//!     fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support
//!   ```
//!
//! - Every device exposes identity, participation state, last successful sync,
//!   selected scope set, conflict class, retained rollback checkpoint, and
//!   local-authoritative fallback, inspectable without a mutating action.
//! - Conflict review is field-aware across exact-match, translated, partial,
//!   stale-remote, policy-locked, and local-authoritative outcomes.
//! - No sync flow overwrites a local scope without a structured change preview
//!   and a rollback checkpoint.
//! - Every snapshot class carries its included / excluded state classes,
//!   producer version, integrity hash, source provenance, and local fallback.
//! - Dirty-buffer journals and secret material never cross the sync or export
//!   lane.
//! - Profile-roaming truth keeps local launch / edit authority even when managed
//!   sync is unavailable, with temporary profiles excluded by default.
//! - The desktop UI, CLI inspect, Help/About, support export, and admin
//!   device-registry view all consume the shared record.
//! - No posture over-claims; a posture that cannot prove a pillar or sits on a
//!   below-Stable surface is narrowed below Stable with a named reason.
//! - The same record opens from the device registry, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts, available without an account or managed services.

use std::collections::BTreeSet;

use aureline_settings::sync_device_registry_stable::{
    is_canonical_object_ref, sync_device_registry_corpus, ConflictOutcomeClass,
    DeviceParticipationState, LayoutMode, RouteSurface, SnapshotClass, StableClaimClass,
    StateClass, SurfaceClass, SyncDeviceRegistryCertification, SYNC_DEVICE_REGISTRY_RECORD_KIND,
    SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support",
);

fn load_record(filename: &str) -> SyncDeviceRegistryCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in sync_device_registry_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-settings --bin aureline_settings_sync_device_registry_stable -- emit-fixtures fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.posture_id, scenario.expected_posture,
            "{} posture",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.claim_class, scenario.expected_claim_class,
            "{} claim_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable, scenario.expected_qualifies_stable,
            "{} qualifies_stable",
            scenario.scenario_id
        );
        assert_eq!(
            record.surface_lifecycle_marker, scenario.expected_surface_marker,
            "{} surface marker",
            scenario.scenario_id
        );
    }
}

#[test]
fn record_identity_and_contract_are_stable() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, SYNC_DEVICE_REGISTRY_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF
        );
        assert!(is_canonical_object_ref(&record.diagnostics_export_ref));
        assert!(is_canonical_object_ref(&record.support_export_ref));
    }
}

#[test]
fn device_participation_is_truthful_and_inspectable() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.device_participation.is_empty(),
            "{} must register at least one device",
            scenario.scenario_id
        );
        assert!(
            record
                .device_participation
                .iter()
                .any(|row| row.is_local_device),
            "{} must register the local device",
            scenario.scenario_id
        );
        for row in &record.device_participation {
            assert!(!row.device_id.is_empty());
            assert!(!row.sync_freshness.is_empty());
            assert!(!row.conflict_class.is_empty());
            assert!(
                row.local_authoritative_fallback,
                "{} device {} must keep local-authoritative fallback",
                scenario.scenario_id, row.device_id
            );
            assert!(
                row.inspectable_without_mutation,
                "{} device {} must be inspectable without a mutating action",
                scenario.scenario_id, row.device_id
            );
            // A non-active device must disclose a revocation reason.
            if !matches!(row.participation_state, DeviceParticipationState::Active) {
                assert!(
                    row.revocation_reason.is_some(),
                    "{} device {} must disclose why it is not active",
                    scenario.scenario_id,
                    row.device_id
                );
            }
            // Failed / paused sync retains a rollback checkpoint.
            if matches!(
                row.participation_state,
                DeviceParticipationState::Paused | DeviceParticipationState::Revoked
            ) {
                assert!(
                    row.rollback_checkpoint_ref.is_some(),
                    "{} paused/revoked device {} must retain a rollback checkpoint",
                    scenario.scenario_id,
                    row.device_id
                );
            }
        }
        assert!(record.pillars.device_participation_truth);
    }
}

#[test]
fn conflict_review_is_field_aware() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.conflict_review.is_empty(),
            "{} must exercise at least one conflict-review row",
            scenario.scenario_id
        );
        let exposed: BTreeSet<ConflictOutcomeClass> = record
            .conflict_review
            .iter()
            .map(|row| row.outcome_class)
            .collect();
        for required in ConflictOutcomeClass::REQUIRED_COVERAGE {
            assert!(
                exposed.contains(&required),
                "{} conflict review must distinguish outcome {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        let rollup: BTreeSet<ConflictOutcomeClass> =
            record.outcome_coverage.iter().copied().collect();
        assert_eq!(rollup, exposed, "{} outcome rollup", scenario.scenario_id);
        for row in &record.conflict_review {
            assert!(is_canonical_object_ref(&row.diagnostics_entry_ref));
            assert!(
                row.inspectable_before_apply,
                "{} conflict {} must be inspectable before apply",
                scenario.scenario_id, row.setting_id
            );
        }
        assert!(record.pillars.conflict_review_field_aware);
    }
}

#[test]
fn overwrites_are_protected_or_narrowed() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.conflict_review {
            if row.overwrites_local && row.conforms {
                // A conforming overwrite must carry a change preview and a
                // rollback checkpoint before apply.
                assert!(
                    row.change_preview_ref.is_some() && row.rollback_checkpoint_ref.is_some(),
                    "{} overwrite {} must carry a change preview and a rollback checkpoint",
                    scenario.scenario_id,
                    row.setting_id
                );
                assert!(row.protected_before_overwrite);
            }
            // No conflict row may widen trust / egress / permissions / managed
            // authority and still conform.
            if row.conforms {
                assert!(
                    !row.widens_authority,
                    "{} conflict {} must not widen authority",
                    scenario.scenario_id, row.setting_id
                );
            }
            // A non-conforming row must carry a bounded waiver.
            if !row.conforms {
                assert!(
                    row.waiver_ref.is_some(),
                    "{} non-conforming conflict {} must carry a waiver",
                    scenario.scenario_id,
                    row.setting_id
                );
            }
        }
        // At least one posture must demonstrate a protected overwrite.
        if scenario.scenario_id == "nominal" {
            assert!(
                record
                    .conflict_review
                    .iter()
                    .any(|row| row.overwrites_local && row.protected_before_overwrite),
                "nominal must demonstrate a protected overwrite"
            );
        }
    }
}

#[test]
fn snapshots_carry_provenance_and_hold_the_secret_boundary() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<SnapshotClass> = record
            .snapshots
            .iter()
            .map(|row| row.snapshot_class)
            .collect();
        for required in SnapshotClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing snapshot class {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.snapshots {
            assert!(is_canonical_object_ref(&row.snapshot_ref));
            assert!(!row.producer_aureline_version.is_empty());
            assert!(!row.producer_schema_version.is_empty());
            assert!(!row.integrity_hash.is_empty());
            assert!(!row.source_provenance.is_empty());
            // A snapshot that crosses a sync / export lane must never include a
            // forbidden state class unless the posture is narrowed for it.
            if row.snapshot_class.crosses_sync_or_export_lane() {
                let leaks = row
                    .included_state_classes
                    .iter()
                    .any(|class| class.is_secret_or_volatile());
                assert_eq!(
                    leaks,
                    row.carries_forbidden_state_class,
                    "{} snapshot {} forbidden-class flag must match its contents",
                    scenario.scenario_id,
                    row.snapshot_class.as_str()
                );
                if leaks {
                    assert!(
                        row.waiver_ref.is_some(),
                        "{} leaking snapshot {} must carry a waiver",
                        scenario.scenario_id,
                        row.snapshot_class.as_str()
                    );
                }
            }
        }
        assert!(record.pillars.snapshot_provenance_complete);
    }
}

#[test]
fn secret_boundary_excludes_forbidden_classes_on_every_lane() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for class in StateClass::FORBIDDEN_ON_LANE {
            for lane in ["sync", "export"] {
                assert!(
                    record.secret_boundary.iter().any(|row| {
                        row.state_class == class && row.lane == lane && row.excluded
                    }),
                    "{} secret boundary must exclude {} on {} lane",
                    scenario.scenario_id,
                    class.as_str(),
                    lane
                );
            }
        }
    }
}

#[test]
fn profile_roaming_keeps_local_authority() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let roaming = &record.profile_roaming;
        assert!(
            roaming.local_launch_edit_authority_retained,
            "{} must retain local launch/edit authority",
            scenario.scenario_id
        );
        assert!(
            roaming.temporary_profiles_excluded,
            "{} must exclude temporary profiles by default",
            scenario.scenario_id
        );
        assert!(!roaming.summary.trim().is_empty());
        assert!(!roaming.originating_profile_revision.trim().is_empty());
        assert!(roaming.conforms);
        assert!(record.pillars.profile_roaming_truth);
        // The managed-unavailable posture must still keep local authority.
        if scenario.scenario_id == "managed_sync_unavailable" {
            assert!(!roaming.managed_sync_available);
            assert!(roaming.local_launch_edit_authority_retained);
        }
    }
}

#[test]
fn every_required_surface_shares_one_truth() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<SurfaceClass> = record
            .surface_parity
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for required in SurfaceClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing surface {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.surface_parity {
            assert!(is_canonical_object_ref(&row.record_ref));
        }
    }
}

#[test]
fn stable_postures_qualify_and_narrowed_postures_name_a_reason() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.qualifies_stable {
            assert_eq!(
                record.stable_qualification.claim_class,
                StableClaimClass::Stable
            );
            assert!(record.stable_qualification.narrowing_reasons.is_empty());
        } else {
            assert_ne!(
                record.stable_qualification.claim_class,
                StableClaimClass::Stable
            );
            assert!(
                !record.stable_qualification.narrowing_reasons.is_empty(),
                "{} narrowed posture must name a reason",
                scenario.scenario_id
            );
            assert!(record.honesty_marker_present);
        }
    }
}

#[test]
fn at_least_one_posture_qualifies_and_one_narrows() {
    let records: Vec<_> = sync_device_registry_corpus()
        .into_iter()
        .map(|scenario| scenario.record())
        .collect();
    assert!(
        records
            .iter()
            .any(|record| record.stable_qualification.qualifies_stable),
        "matrix must include a Stable posture"
    );
    assert!(
        records
            .iter()
            .any(|record| !record.stable_qualification.qualifies_stable),
        "matrix must exercise the narrowing path"
    );
}

#[test]
fn record_is_reachable_keyboard_first_across_layouts() {
    for scenario in sync_device_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<RouteSurface> =
            record.routes.iter().map(|route| route.surface).collect();
        for required in RouteSurface::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing entry surface {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for route in &record.routes {
            assert!(route.keyboard_reachable && route.activates_same_record);
        }
        for required in LayoutMode::REQUIRED {
            let disclosure = record
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                disclosure.row_narration_available && disclosure.recovery_affordances_reachable
            );
        }
        assert!(record.available_without_account && record.available_without_managed_services);
    }
}
