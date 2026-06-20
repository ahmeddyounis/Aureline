use std::collections::BTreeSet;

use super::*;

fn snapshot(profile: EnvironmentProfile) -> PrebuildSnapshot {
    seeded_prebuild_snapshots()
        .into_iter()
        .find(|snap| snap.profile == profile)
        .unwrap_or_else(|| panic!("seeded snapshot for {} exists", profile.as_str()))
}

fn current() -> PrebuildFingerprint {
    // The seeded warm snapshot matches the current fingerprint exactly, so
    // its recorded fingerprint *is* the current fingerprint for a match.
    snapshot(EnvironmentProfile::Prebuild).recorded_fingerprint
}

/// Replaces one key's digest with a stale value to simulate drift.
fn drift(fingerprint: &PrebuildFingerprint, key: FingerprintKey) -> PrebuildFingerprint {
    let mut keys = fingerprint.keys.clone();
    for keyed in &mut keys {
        if keyed.key == key {
            keyed.digest = CapsuleDigest {
                algorithm: "sha256".to_owned(),
                value: "f".repeat(64),
            };
        }
    }
    PrebuildFingerprint::from_keys(keys, fingerprint.summary.clone())
}

#[test]
fn full_match_warm_reuses() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let decision = evaluate_prebuild_reuse(&snap, &current());
    assert_eq!(decision.outcome, StartOutcome::Warm);
    assert!(decision.reused);
    assert!(!decision.invalidated);
    assert_eq!(decision.warm_start_posture, WarmStartPosture::WarmFullReuse);
    assert_eq!(decision.headline_reason, PrebuildReason::FullMatch);
    assert!(decision.reason_tokens.is_empty());
    assert!(decision.gated_action_tokens.is_empty());
    assert!(decision
        .action_gates
        .iter()
        .all(|gate| gate.posture == ActionPosture::Available));
}

#[test]
fn platform_drift_invalidates() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let decision = evaluate_prebuild_reuse(&snap, &drift(&current(), FingerprintKey::PlatformArch));
    assert_eq!(decision.outcome, StartOutcome::Invalidated);
    assert!(!decision.reused);
    assert!(decision.invalidated);
    assert_eq!(decision.warm_start_posture, WarmStartPosture::ColdBuild);
    assert_eq!(decision.headline_reason, PrebuildReason::PlatformDrift);
    assert!(decision
        .reason_tokens
        .contains(&"platform_arch_drift".to_owned()));
    // Invalidation disables every snapshot-served action.
    assert!(decision
        .action_gates
        .iter()
        .all(|gate| gate.posture == ActionPosture::Disabled));
}

#[test]
fn policy_drift_invalidates() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let decision = evaluate_prebuild_reuse(&snap, &drift(&current(), FingerprintKey::PolicyEpoch));
    assert_eq!(decision.outcome, StartOutcome::Invalidated);
    assert_eq!(decision.headline_reason, PrebuildReason::PolicyDrift);
}

#[test]
fn source_drift_forces_cold_build() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let decision = evaluate_prebuild_reuse(
        &snap,
        &drift(&current(), FingerprintKey::SourceTreeIdentity),
    );
    assert_eq!(decision.outcome, StartOutcome::Cold);
    assert!(!decision.reused);
    assert!(!decision.invalidated);
    assert_eq!(decision.warm_start_posture, WarmStartPosture::ColdBuild);
    assert_eq!(decision.headline_reason, PrebuildReason::SourceDrift);
}

#[test]
fn extension_lock_drift_narrows_to_partial() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let decision = evaluate_prebuild_reuse(
        &snap,
        &drift(&current(), FingerprintKey::ExtensionLockDigest),
    );
    assert_eq!(decision.outcome, StartOutcome::PartiallyWarm);
    assert!(decision.reused);
    assert_eq!(
        decision.warm_start_posture,
        WarmStartPosture::WarmPartialReuse
    );
    assert_eq!(decision.headline_reason, PrebuildReason::ExtensionLockDrift);
    // Language intelligence backs the extension lock, so it degrades; build
    // and run stay available because the toolchain still matches.
    assert!(decision
        .gated_action_tokens
        .contains(&CapsuleActionClass::LanguageIntel.as_str().to_owned()));
    assert!(!decision
        .gated_action_tokens
        .contains(&CapsuleActionClass::BuildRun.as_str().to_owned()));
}

#[test]
fn invalidating_drift_dominates_layered_drift() {
    // Platform (invalidating) plus extension (layered) drift must invalidate,
    // never settle for partial reuse.
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let mut fingerprint = drift(&current(), FingerprintKey::PlatformArch);
    fingerprint = drift(&fingerprint, FingerprintKey::ExtensionLockDigest);
    let decision = evaluate_prebuild_reuse(&snap, &fingerprint);
    assert_eq!(decision.outcome, StartOutcome::Invalidated);
}

#[test]
fn partial_artifact_loss_narrows_to_partial() {
    let mut snap = snapshot(EnvironmentProfile::Prebuild);
    for artifact in &mut snap.artifacts {
        if artifact.layer == ArtifactLayer::SearchIndex {
            artifact.integrity = ArtifactIntegrity::Missing;
        }
    }
    let decision = evaluate_prebuild_reuse(&snap, &current());
    assert_eq!(decision.outcome, StartOutcome::PartiallyWarm);
    assert_eq!(
        decision.headline_reason,
        PrebuildReason::PartialArtifactLoss
    );
    assert!(decision
        .reason_tokens
        .contains(&"artifact_search_index_lost".to_owned()));
}

#[test]
fn critical_artifact_loss_forces_cold() {
    let mut snap = snapshot(EnvironmentProfile::Prebuild);
    for artifact in &mut snap.artifacts {
        if artifact.layer == ArtifactLayer::BaseImage {
            artifact.integrity = ArtifactIntegrity::Corrupt;
        }
    }
    let decision = evaluate_prebuild_reuse(&snap, &current());
    assert_eq!(decision.outcome, StartOutcome::Cold);
    assert_eq!(
        decision.headline_reason,
        PrebuildReason::CriticalArtifactLoss
    );
}

#[test]
fn absent_key_cannot_prove_compatibility() {
    let mut snap = snapshot(EnvironmentProfile::Prebuild);
    snap.recorded_fingerprint = PrebuildFingerprint::from_keys(
        snap.recorded_fingerprint
            .keys
            .into_iter()
            .filter(|keyed| keyed.key != FingerprintKey::SourceTreeIdentity)
            .collect(),
        "missing source key",
    );
    let decision = evaluate_prebuild_reuse(&snap, &current());
    assert_eq!(decision.outcome, StartOutcome::Cold);
    assert_eq!(decision.headline_reason, PrebuildReason::UnprovenKey);
    assert!(decision
        .reason_tokens
        .contains(&"source_tree_identity_absent".to_owned()));
}

#[test]
fn outcome_maps_to_governance_warm_start_posture() {
    assert_eq!(
        StartOutcome::Warm.warm_start_posture(),
        WarmStartPosture::WarmFullReuse
    );
    assert_eq!(
        StartOutcome::PartiallyWarm.warm_start_posture(),
        WarmStartPosture::WarmPartialReuse
    );
    assert_eq!(
        StartOutcome::Cold.warm_start_posture(),
        WarmStartPosture::ColdBuild
    );
    assert_eq!(
        StartOutcome::Invalidated.warm_start_posture(),
        WarmStartPosture::ColdBuild
    );
}

#[test]
fn every_seeded_snapshot_validates_and_warm_reuses() {
    let current = current_fingerprint_for_validation();
    for snap in seeded_prebuild_snapshots() {
        validate_prebuild_snapshot(&snap)
            .unwrap_or_else(|err| panic!("snapshot {} must validate: {err}", snap.snapshot_id));
        let decision = evaluate_prebuild_reuse(&snap, &current);
        assert_eq!(
            decision.outcome,
            StartOutcome::Warm,
            "snapshot {} must warm-reuse on matching truth",
            snap.snapshot_id
        );
    }
}

fn current_fingerprint_for_validation() -> PrebuildFingerprint {
    snapshot(EnvironmentProfile::Starter).recorded_fingerprint
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_prebuild_fingerprint_packet();
    validate_prebuild_fingerprint_packet(&packet).expect("seeded packet must validate");
}

#[test]
fn packet_cases_cover_every_outcome() {
    let packet = seeded_prebuild_fingerprint_packet();
    let outcomes: BTreeSet<_> = packet.cases.iter().map(|c| c.decision.outcome).collect();
    for required in StartOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "cases must cover {}",
            required.as_str()
        );
    }
}

#[test]
fn drills_cover_every_named_drift_class_and_recover() {
    let packet = seeded_prebuild_fingerprint_packet();
    let reasons: BTreeSet<_> = packet.drills.iter().map(|d| d.injected_reason).collect();
    for required in [
        PrebuildReason::SourceDrift,
        PrebuildReason::PolicyDrift,
        PrebuildReason::PlatformDrift,
        PrebuildReason::ExtensionLockDrift,
        PrebuildReason::PartialArtifactLoss,
    ] {
        assert!(
            reasons.contains(&required),
            "drills must cover {}",
            required.as_str()
        );
    }
    for drill in &packet.drills {
        assert_eq!(drill.recovers_to_outcome, StartOutcome::Warm);
        assert!(drill.degraded_outcome != StartOutcome::Warm);
    }
}

#[test]
fn every_seeded_fixture_validates_and_covers_all_outcomes() {
    let fixtures = seeded_prebuild_fingerprint_fixtures();
    let mut outcomes = BTreeSet::new();
    for fixture in &fixtures {
        validate_prebuild_fingerprint_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        outcomes.insert(fixture.expected_outcome);
    }
    for required in StartOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "fixtures must cover {}",
            required.as_str()
        );
    }
}

#[test]
fn fingerprint_combined_digest_is_reproducible() {
    let fingerprint = snapshot(EnvironmentProfile::Prebuild).recorded_fingerprint;
    validate_prebuild_fingerprint(&fingerprint).expect("fingerprint must validate");
}

#[test]
fn decision_round_trips_through_json() {
    let snap = snapshot(EnvironmentProfile::Devcontainer);
    let decision =
        evaluate_prebuild_reuse(&snap, &drift(&current(), FingerprintKey::ToolchainDigest));
    let json = serde_json::to_string(&decision).expect("decision serializes");
    let back: PrebuildDecision = serde_json::from_str(&json).expect("decision deserializes");
    assert_eq!(decision, back);
}

#[test]
fn export_is_metadata_first() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let decision = evaluate_prebuild_reuse(&snap, &current());
    let export = export_prebuild_decision(&decision);
    assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
    assert_eq!(export.outcome, decision.outcome);
    assert_eq!(export.decision, decision);
}

#[test]
fn desktop_headless_and_support_share_one_decision() {
    let snap = snapshot(EnvironmentProfile::Prebuild);
    let cur = drift(&current(), FingerprintKey::ExtensionLockDigest);
    let desktop = desktop_prebuild_decision(&snap, &cur);
    let headless = headless_prebuild_decision(&snap, &cur);
    let support = support_prebuild_decision(&snap, &cur);
    assert_eq!(desktop, headless);
    assert_eq!(support.decision, desktop);
}
