use super::*;

fn packet() -> M5InstallPortabilityGovernanceMatrix {
    current_m5_install_portability_governance_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_INSTALL_PORTABILITY_GOVERNANCE_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_lane_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.lane_rows.len(), packet.lanes.len());
    for &lane in &packet.lanes {
        assert!(
            packet.lane_row(lane).is_some(),
            "missing row for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_lane_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_lanes_gate_consistent());
    for row in &packet.lane_rows {
        assert_eq!(
            row.published_assurance,
            row.effective_assurance(),
            "lane {} publishes beyond the gate",
            row.lane_id
        );
        assert_eq!(
            row.admission_outcome,
            row.required_outcome(),
            "lane {} outcome diverges from the gate",
            row.lane_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "lane {} downgrade reasons diverge from the gate",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.lane_rows {
        assert!(
            row.has_required_evidence(),
            "lane {} is missing required evidence refs",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_binds_to_its_canonical_source_packet() {
    let packet = packet();
    for row in &packet.lane_rows {
        assert_eq!(
            row.packet_ref,
            row.lane.source_packet(),
            "lane {} governs a packet other than its canonical source",
            row.lane_id
        );
    }
}

#[test]
fn install_modes_stay_distinct_per_lane() {
    // System, user, portable, managed, and marketplace installs stay pinned to their lanes.
    let packet = packet();
    for row in &packet.lane_rows {
        assert_eq!(
            row.install_mode,
            row.lane.install_mode(),
            "lane {} carries a mode other than its pinned mode",
            row.lane_id
        );
    }
}

#[test]
fn narrowed_lanes_offer_a_recovery_and_caveats() {
    let packet = packet();
    for row in &packet.lane_rows {
        if row.admission_outcome.is_narrowed() {
            assert!(
                row.downgrade_path.is_offered(),
                "narrowed lane {} must offer a recovery path",
                row.lane_id
            );
            assert!(
                !row.caveats.is_empty(),
                "narrowed lane {} must list a caveat",
                row.lane_id
            );
            assert!(
                !row.stale_or_missing_fields.is_empty(),
                "narrowed lane {} must name a stale or narrowing field",
                row.lane_id
            );
        }
    }
}

#[test]
fn every_required_consumer_surface_binds_to_the_packet() {
    let packet = packet();
    for surface in ConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "no binding ingests the packet for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.lanes.len(), packet.lane_rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_lanes_gate_consistent,
        packet.all_lanes_gate_consistent()
    );
    assert_eq!(projection.verified_count, packet.verified_lanes().count());
    assert_eq!(projection.narrowed_count, packet.narrowed_lanes().count());
    assert_eq!(projection.refused_count, packet.refused_lanes().count());
    for (row, export) in packet.lane_rows.iter().zip(projection.lanes.iter()) {
        assert_eq!(export.packet_ref, row.packet_ref);
        assert_eq!(export.install_mode, row.install_mode.as_str());
        assert_eq!(export.verified, row.is_verified());
        assert_eq!(export.downgraded, row.is_downgraded());
        assert_eq!(export.trust_sensitive, row.lane.is_trust_sensitive());
        assert_eq!(export.published_assurance, row.published_assurance.as_str());
        assert_eq!(
            export.preserves_local_work,
            row.local_continuity.preserves_local_work()
        );
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("export-1", "2026-06-11T00:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.governance_packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.governance_matrix, packet);
}

#[test]
fn published_labels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<InstallAssurance> = packet
        .lane_rows
        .iter()
        .map(|c| c.published_assurance)
        .collect();
    for label in InstallAssurance::ALL {
        assert!(
            present.contains(&label),
            "no lane publishes label {}",
            label.as_str()
        );
    }
}

#[test]
fn admission_outcomes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AdmissionOutcome> = packet
        .lane_rows
        .iter()
        .map(|c| c.admission_outcome)
        .collect();
    for outcome in AdmissionOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no lane exercises outcome {}",
            outcome.as_str()
        );
    }
}

#[test]
fn install_modes_are_exhaustive() {
    // Every distinct install mode is exercised.
    let packet = packet();
    let present: BTreeSet<InstallMode> = packet.lane_rows.iter().map(|c| c.install_mode).collect();
    for mode in InstallMode::ALL {
        assert!(
            present.contains(&mode),
            "no lane exercises install mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    // Every install/config/auth downgrade trigger is exercised by at least one lane.
    let packet = packet();
    let present: BTreeSet<DowngradeReason> = packet
        .lane_rows
        .iter()
        .flat_map(|c| c.downgrade_reasons.iter().copied())
        .collect();
    for reason in DowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no lane exercises downgrade reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn verified_lanes_are_whole() {
    let packet = packet();
    assert!(
        packet.verified_lanes().count() >= 2,
        "fixture needs at least two verified lanes to prove the gate is not a blanket downgrade"
    );
    for row in packet.verified_lanes() {
        assert_eq!(row.capability_floor(), InstallAssurance::Verified);
        assert_eq!(
            row.install_verification,
            InstallVerification::SignedVerified
        );
        assert_eq!(
            row.install_topology_support,
            InstallTopologySupport::Supported
        );
        assert_eq!(
            row.portable_state_freshness,
            PortableStateFreshness::Current
        );
        assert_eq!(row.sync_device_state, SyncDeviceState::Active);
        assert_eq!(
            row.auth_recovery_posture,
            AuthRecoveryPosture::PasskeyVerified
        );
        assert_eq!(row.local_continuity, LocalContinuity::Authoritative);
        assert!(row.downgrade_reasons.is_empty());
        assert!(row.caveats.is_empty());
        assert!(row.stale_or_missing_fields.is_empty());
        assert!(!row.downgrade_path.is_offered());
        assert!(!row.supported_scopes.is_empty());
        assert!(!row.is_downgraded());
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        InstallVerification::PlatformTrusted.assurance_ceiling(),
        InstallAssurance::Bounded
    );
    assert_eq!(
        InstallVerification::Unverified.assurance_ceiling(),
        InstallAssurance::Withheld
    );
    assert_eq!(
        InstallTopologySupport::SideBySideBounded.assurance_ceiling(),
        InstallAssurance::Bounded
    );
    assert_eq!(
        InstallTopologySupport::Unsupported.assurance_ceiling(),
        InstallAssurance::Withheld
    );
    assert_eq!(
        PortableStateFreshness::Stale.assurance_ceiling(),
        InstallAssurance::RetestPending
    );
    assert_eq!(
        PortableStateFreshness::Missing.assurance_ceiling(),
        InstallAssurance::Withheld
    );
    assert_eq!(
        SyncDeviceState::Offline.assurance_ceiling(),
        InstallAssurance::RetestPending
    );
    assert_eq!(
        SyncDeviceState::Blocked.assurance_ceiling(),
        InstallAssurance::Withheld
    );
    assert_eq!(
        AuthRecoveryPosture::SystemBrowserFallback.assurance_ceiling(),
        InstallAssurance::Bounded
    );
    assert_eq!(
        AuthRecoveryPosture::RecoveryBlocked.assurance_ceiling(),
        InstallAssurance::Withheld
    );
}

#[test]
fn preview_lane_is_bounded_not_left_verified() {
    // A signed preview install that runs side-by-side is narrowed to its slice rather than
    // inheriting the stable lane's verified label.
    let packet = packet();
    let row = packet
        .lane_row(InstallConfigLane::DesktopPreview)
        .expect("desktop-preview row");
    assert!(row.install_topology_support.is_unsupported_trigger());
    assert!(row.is_downgraded());
    assert!(row.lane.is_trust_sensitive());
    assert!(row
        .downgrade_reasons
        .contains(&DowngradeReason::UnsupportedInstallTopology));
    assert_eq!(row.published_assurance, InstallAssurance::Bounded);
}

#[test]
fn stable_lane_publishes() {
    // A signed, supported, fresh, sync-active, passkey-ready desktop install publishes verified.
    let packet = packet();
    let row = packet
        .lane_row(InstallConfigLane::DesktopStable)
        .expect("desktop-stable row");
    assert_eq!(row.published_assurance, InstallAssurance::Verified);
    assert_eq!(row.admission_outcome, AdmissionOutcome::AdmitFull);
    assert!(row.downgrade_reasons.is_empty());
}

#[test]
fn policy_limited_recovery_is_withheld() {
    // A managed lane whose recovery is policy-blocked drops to withheld rather than inheriting a
    // stronger auth claim.
    let packet = packet();
    let row = packet
        .lane_row(InstallConfigLane::ManagedFleet)
        .expect("managed-fleet row");
    assert_eq!(row.published_assurance, InstallAssurance::Withheld);
    assert_eq!(row.admission_outcome, AdmissionOutcome::Refuse);
    assert!(row.lane.is_trust_sensitive());
    assert!(row
        .downgrade_reasons
        .contains(&DowngradeReason::PolicyLimitedRecovery));
    assert_eq!(row.local_continuity, LocalContinuity::PolicyRestricted);
    assert!(row.supported_scopes.is_empty());
}

#[test]
fn blocked_sync_preserves_local_continuity() {
    // The sync-device lane is blocked from sync, but local durable state stays authoritative so
    // local-only work continues — the local-first invariant.
    let packet = packet();
    let row = packet
        .lane_row(InstallConfigLane::SyncDevice)
        .expect("sync-device row");
    assert!(row.sync_device_state.is_blocked_trigger());
    assert_eq!(row.published_assurance, InstallAssurance::Withheld);
    assert!(row
        .downgrade_reasons
        .contains(&DowngradeReason::BlockedSyncApply));
    assert_eq!(row.local_continuity, LocalContinuity::LocalOnlyFallback);
    assert!(row.local_continuity.preserves_local_work());
}

#[test]
fn trust_sensitive_lanes_never_publish_above_their_evidence() {
    // A preview, portable, managed, companion, or sync lane narrows safely instead of inheriting
    // a broader stable claim.
    let packet = packet();
    for row in &packet.lane_rows {
        if row.lane.is_trust_sensitive() {
            assert_eq!(
                row.published_assurance,
                row.effective_assurance(),
                "trust-sensitive lane {} publishes beyond its evidence",
                row.lane_id
            );
        }
    }
}

#[test]
fn local_continuity_tracks_sync_and_auth() {
    // A lane whose sync or auth degraded must not claim its local state is still fully
    // authoritative; a healthy lane must.
    let packet = packet();
    for row in &packet.lane_rows {
        if row.degrades_local() {
            assert_ne!(
                row.local_continuity,
                LocalContinuity::Authoritative,
                "degraded lane {} falsely claims authoritative local continuity",
                row.lane_id
            );
        } else {
            assert_eq!(
                row.local_continuity,
                LocalContinuity::Authoritative,
                "healthy lane {} should keep authoritative local continuity",
                row.lane_id
            );
        }
    }
}

#[test]
fn install_identities_are_namespaced() {
    let packet = packet();
    assert!(!packet.install_identity_scheme.trim().is_empty());
    for row in &packet.lane_rows {
        assert!(
            !row.install_root_namespace.trim().is_empty(),
            "lane {} has no install root namespace",
            row.lane_id
        );
        assert!(
            !row.state_root_namespace.trim().is_empty(),
            "lane {} has no state root namespace",
            row.lane_id
        );
    }
}

#[test]
fn validate_flags_overstated_claim() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.effective_assurance() != InstallAssurance::Verified)
    {
        row.published_assurance = InstallAssurance::Verified;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::OverstatedClaim { .. }
        )));
    }
}

#[test]
fn validate_flags_outcome_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.admission_outcome != AdmissionOutcome::Refuse)
    {
        row.admission_outcome = AdmissionOutcome::Refuse;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::OutcomeMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_downgrade_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lane_rows.iter_mut().find(|c| {
        !c.downgrade_reasons
            .contains(&DowngradeReason::StalePortableState)
    }) {
        row.downgrade_reasons
            .push(DowngradeReason::StalePortableState);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::DowngradeReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_source_packet_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lane_rows.first_mut() {
        row.packet_ref = "artifacts/install/not-the-source.json".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::SourcePacketMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_install_mode_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.install_mode != InstallMode::Portable)
    {
        row.install_mode = InstallMode::Portable;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::InstallModeMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_local_continuity_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lane_rows.iter_mut().find(|c| c.degrades_local()) {
        row.local_continuity = LocalContinuity::Authoritative;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::LocalContinuityMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_downgrade_path() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.admission_outcome.is_narrowed())
    {
        row.downgrade_path = DowngradePath::NoneNeeded;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::MissingDowngradePath { .. }
        )));
    }
}

#[test]
fn validate_flags_consumer_binding_drift() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.preserves_published_labels = false;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5InstallPortabilityGovernanceViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != ConsumerSurface::Diagnostics);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5InstallPortabilityGovernanceViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_missing_lane_row() {
    let mut packet = packet();
    let removed = packet.lane_rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5InstallPortabilityGovernanceViolation::MissingLaneRow { .. }
    )));
}

#[test]
fn validate_flags_unclaimed_lane_row() {
    let mut packet = packet();
    packet.lanes.retain(|l| *l != InstallConfigLane::SyncDevice);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5InstallPortabilityGovernanceViolation::UnclaimedLaneRow { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5InstallPortabilityGovernanceViolation::ClosedVocabularyMismatch { field: "lanes" }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_lanes = packet.summary.total_lanes.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5InstallPortabilityGovernanceViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(InstallConfigLane::DesktopStable.as_str(), "desktop_stable");
    assert_eq!(
        InstallConfigLane::MarketplaceCompanion.as_str(),
        "marketplace_companion"
    );
    assert_eq!(InstallMode::Marketplace.as_str(), "marketplace");
    assert_eq!(
        ChannelRing::PreviewEarlyAccess.as_str(),
        "preview_early_access"
    );
    assert_eq!(StateRootClass::PortableRoot.as_str(), "portable_root");
    assert_eq!(
        PortableExportClass::ImportedPackage.as_str(),
        "imported_package"
    );
    assert_eq!(
        EffectiveSettingScope::ManagedEnforced.as_str(),
        "managed_enforced"
    );
    assert_eq!(InstallAssurance::Verified.as_str(), "verified");
    assert_eq!(InstallAssurance::Withheld.as_str(), "withheld");
    assert_eq!(InstallVerification::SelfSigned.as_str(), "self_signed");
    assert_eq!(
        InstallTopologySupport::SideBySideBounded.as_str(),
        "side_by_side_bounded"
    );
    assert_eq!(PortableStateFreshness::Stale.as_str(), "stale");
    assert_eq!(SyncDeviceState::Blocked.as_str(), "blocked");
    assert_eq!(
        AuthRecoveryPosture::LocalOnlyContinuity.as_str(),
        "local_only_continuity"
    );
    assert_eq!(
        LocalContinuity::PolicyRestricted.as_str(),
        "policy_restricted"
    );
    assert_eq!(
        DowngradePath::RequestRecoveryPolicy.as_str(),
        "request_recovery_policy"
    );
    assert_eq!(DowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(DowngradeReason::MissingPasskey.as_str(), "missing_passkey");
    assert_eq!(AdmissionOutcome::AdmitRetest.as_str(), "admit_retest");
    assert_eq!(ConsumerSurface::AdminDocs.as_str(), "admin_docs");
}

#[test]
fn assurance_rank_orders_low_to_high() {
    assert!(InstallAssurance::Withheld.rank() < InstallAssurance::RetestPending.rank());
    assert!(InstallAssurance::RetestPending.rank() < InstallAssurance::Bounded.rank());
    assert!(InstallAssurance::Bounded.rank() < InstallAssurance::Verified.rank());
    assert_eq!(
        InstallAssurance::Verified.min(InstallAssurance::RetestPending),
        InstallAssurance::RetestPending
    );
}

#[test]
fn source_packets_are_non_empty() {
    for lane in InstallConfigLane::ALL {
        assert!(
            !lane.source_packet().trim().is_empty(),
            "lane {} does not bind to a checked-in install packet",
            lane.as_str()
        );
    }
}

#[test]
fn underqualified_rows_narrow_to_bounded_or_retest_pending() {
    // Underqualified install/config/auth rows narrow automatically to bounded or retest-pending
    // labels before publication rather than staying verified.
    let packet = packet();
    for row in &packet.lane_rows {
        if row.is_downgraded() && row.published_assurance != InstallAssurance::Withheld {
            assert!(
                matches!(
                    row.published_assurance,
                    InstallAssurance::Bounded | InstallAssurance::RetestPending
                ),
                "downgraded lane {} should narrow to bounded or retest-pending",
                row.lane_id
            );
        }
    }
}
