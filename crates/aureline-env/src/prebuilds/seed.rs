//! The checked-in prebuild-fingerprint corpus this lane freezes: the
//! per-key and per-artifact invalidation rules, the canonical evaluated
//! cases covering every outcome, the failure / recovery drills for each
//! named drift class, and the fixture corpus the engine must reproduce.

use crate::capsules::CapsuleDigest;
use crate::m5_env_governance::{
    DrillPhase, EnvironmentProfile, MaterializationClass, SourceContractRefs, WarmStartPosture,
};

use super::{
    evaluate_prebuild_reuse, ArtifactIntegrity, ArtifactInvalidationRule, ArtifactLayer,
    FingerprintKey, FingerprintKeyDigest, KeyInvalidationRule, PrebuildArtifact, PrebuildCase,
    PrebuildDrill, PrebuildDrillStep, PrebuildFingerprint, PrebuildFingerprintFixture,
    PrebuildFingerprintPacket, PrebuildReason, PrebuildSnapshot, StartOutcome,
    PREBUILD_FINGERPRINT_DOC_REF, PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF,
    PREBUILD_FINGERPRINT_FIXTURE_RECORD_KIND, PREBUILD_FINGERPRINT_PACKET_ID,
    PREBUILD_FINGERPRINT_PACKET_RECORD_KIND, PREBUILD_FINGERPRINT_PACKET_REF,
    PREBUILD_FINGERPRINT_PROOF_REF, PREBUILD_FINGERPRINT_SCHEMA_REF,
    PREBUILD_FINGERPRINT_SCHEMA_VERSION, PREBUILD_SNAPSHOT_RECORD_KIND,
};

// Upstream environment evidence packets this lane composes.
const WARM_START_CHOOSER_REF: &str = "artifacts/entry/warm_start_chooser_contract.md";
const BUILD_IDENTITY_REF: &str = "artifacts/build/build_identity.json";
const HOST_BOUNDARY_REF: &str = "artifacts/remote/host_boundary_matrix.yaml";
const STATE_ROOT_REF: &str = "artifacts/install/state_root_matrix.yaml";
const EXECUTION_SCOPE_REF: &str = "artifacts/runtime/execution_scope_matrix.yaml";
const MANAGED_LIFECYCLE_REF: &str = "artifacts/runtime/managed_workspace_lifecycle.yaml";

/// Deterministic 64-hex placeholder digest derived from a stable label.
/// These are metadata tokens standing in for real content digests, never
/// the bodies they would digest.
fn dg(label: &str) -> CapsuleDigest {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in label.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    CapsuleDigest {
        algorithm: "sha256".to_owned(),
        value: format!("{hash:016x}").repeat(4),
    }
}

fn key_summary(key: FingerprintKey) -> String {
    match key {
        FingerprintKey::SourceTreeIdentity => {
            "Identity digest of the workspace source tree the snapshot was built from."
        }
        FingerprintKey::CapsuleHash => {
            "Digest of the environment-capsule object the snapshot materializes."
        }
        FingerprintKey::PlatformArch => "The platform / architecture the snapshot was built for.",
        FingerprintKey::PolicyEpoch => "The policy / trust epoch the snapshot was built under.",
        FingerprintKey::ExtensionLockDigest => "Digest of the resolved extension lock.",
        FingerprintKey::ToolchainDigest => "Combined digest of the critical toolchain components.",
    }
    .to_owned()
}

/// Builds the current expected fingerprint: every key at its current
/// digest.
fn current_fingerprint() -> PrebuildFingerprint {
    let keys = FingerprintKey::ALL
        .into_iter()
        .map(|key| FingerprintKeyDigest {
            key,
            digest: dg(&format!("current:{}", key.as_str())),
            summary: key_summary(key),
        })
        .collect();
    PrebuildFingerprint::from_keys(keys, "Current expected prebuild fingerprint.")
}

/// Builds a recorded snapshot fingerprint: every key at its current digest
/// except `drifted` keys (pinned to a stale digest) and `absent` keys
/// (omitted entirely).
fn recorded_fingerprint(
    drifted: &[FingerprintKey],
    absent: &[FingerprintKey],
) -> PrebuildFingerprint {
    let keys = FingerprintKey::ALL
        .into_iter()
        .filter(|key| !absent.contains(key))
        .map(|key| {
            let digest = if drifted.contains(&key) {
                dg(&format!("stale:{}", key.as_str()))
            } else {
                dg(&format!("current:{}", key.as_str()))
            };
            FingerprintKeyDigest {
                key,
                digest,
                summary: key_summary(key),
            }
        })
        .collect();
    PrebuildFingerprint::from_keys(keys, "Recorded prebuild-snapshot fingerprint.")
}

struct ArtifactSpec {
    layer: ArtifactLayer,
    critical: bool,
}

const ARTIFACT_SPECS: &[ArtifactSpec] = &[
    ArtifactSpec {
        layer: ArtifactLayer::BaseImage,
        critical: true,
    },
    ArtifactSpec {
        layer: ArtifactLayer::Toolchain,
        critical: true,
    },
    ArtifactSpec {
        layer: ArtifactLayer::Dependencies,
        critical: false,
    },
    ArtifactSpec {
        layer: ArtifactLayer::Extensions,
        critical: false,
    },
    ArtifactSpec {
        layer: ArtifactLayer::SearchIndex,
        critical: false,
    },
];

/// Builds the standard artifact set, applying any integrity overrides for
/// lost layers.
fn artifacts(lost: &[(ArtifactLayer, ArtifactIntegrity)]) -> Vec<PrebuildArtifact> {
    ARTIFACT_SPECS
        .iter()
        .map(|spec| {
            let integrity = lost
                .iter()
                .find(|(layer, _)| *layer == spec.layer)
                .map(|(_, integrity)| *integrity)
                .unwrap_or(ArtifactIntegrity::Present);
            PrebuildArtifact {
                artifact_id: format!("artifact.{}", spec.layer.as_str()),
                layer: spec.layer,
                integrity,
                critical: spec.critical,
                summary: format!("The {} cached artifact layer.", spec.layer.as_str()),
            }
        })
        .collect()
}

fn materialization_for(profile: EnvironmentProfile) -> MaterializationClass {
    match profile {
        EnvironmentProfile::Starter | EnvironmentProfile::WorkspaceTemplate => {
            MaterializationClass::LocalNative
        }
        EnvironmentProfile::Prebuild => MaterializationClass::Container,
        EnvironmentProfile::Devcontainer => MaterializationClass::Devcontainer,
        EnvironmentProfile::RemoteContainer => MaterializationClass::RemoteHost,
        EnvironmentProfile::ManagedWorkspace => MaterializationClass::ManagedCloud,
    }
}

fn consumer_ref_for(profile: EnvironmentProfile) -> &'static str {
    match profile {
        EnvironmentProfile::Starter => "crates/aureline-workspace/src/entry/mod.rs",
        EnvironmentProfile::Prebuild => "crates/aureline-runtime/src/capsule_resolver/mod.rs",
        EnvironmentProfile::Devcontainer => "crates/aureline-runtime/src/execution_context/mod.rs",
        EnvironmentProfile::RemoteContainer => {
            "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs"
        }
        EnvironmentProfile::ManagedWorkspace => "crates/aureline-support/src/bundle/mod.rs",
        EnvironmentProfile::WorkspaceTemplate => "crates/aureline-runtime/src/env_inspect/mod.rs",
    }
}

/// One reusable scenario: a snapshot evaluated against the current
/// fingerprint.
struct Scenario {
    snapshot: PrebuildSnapshot,
    current: PrebuildFingerprint,
}

fn scenario(
    snapshot_id: &str,
    profile: EnvironmentProfile,
    label: &str,
    drifted: &[FingerprintKey],
    absent: &[FingerprintKey],
    lost: &[(ArtifactLayer, ArtifactIntegrity)],
) -> Scenario {
    let snapshot = PrebuildSnapshot {
        record_kind: PREBUILD_SNAPSHOT_RECORD_KIND.to_owned(),
        schema_version: PREBUILD_FINGERPRINT_SCHEMA_VERSION,
        snapshot_id: snapshot_id.to_owned(),
        profile,
        materialization_class: materialization_for(profile),
        label: label.to_owned(),
        recorded_fingerprint: recorded_fingerprint(drifted, absent),
        artifacts: artifacts(lost),
        summary: format!("Prebuild snapshot for the {} profile.", profile.as_str()),
    };
    Scenario {
        snapshot,
        current: current_fingerprint(),
    }
}

/// Canonical warm snapshots, one per profile with a prebuild warm-start
/// path. Each matches the current fingerprint and reuses fully.
pub fn seeded_prebuild_snapshots() -> Vec<PrebuildSnapshot> {
    [
        (EnvironmentProfile::Starter, "Starter prebuild snapshot"),
        (
            EnvironmentProfile::Prebuild,
            "Prebuilt environment snapshot",
        ),
        (
            EnvironmentProfile::Devcontainer,
            "Devcontainer layer-cache snapshot",
        ),
        (
            EnvironmentProfile::RemoteContainer,
            "Remote-container snapshot",
        ),
        (
            EnvironmentProfile::ManagedWorkspace,
            "Managed-workspace snapshot",
        ),
    ]
    .into_iter()
    .map(|(profile, label)| {
        scenario(
            &format!("snapshot.{}", profile.as_str()),
            profile,
            label,
            &[],
            &[],
            &[],
        )
        .snapshot
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn case(
    case_id: &str,
    title: &str,
    profile: EnvironmentProfile,
    label: &str,
    drifted: &[FingerprintKey],
    absent: &[FingerprintKey],
    lost: &[(ArtifactLayer, ArtifactIntegrity)],
    notes: &str,
) -> PrebuildCase {
    let snapshot_id = format!("snapshot.case.{}", profile.as_str());
    let Scenario { snapshot, current } =
        scenario(&snapshot_id, profile, label, drifted, absent, lost);
    let decision = evaluate_prebuild_reuse(&snapshot, &current);
    PrebuildCase {
        case_id: case_id.to_owned(),
        title: title.to_owned(),
        snapshot,
        current_fingerprint: current,
        decision,
        consumer_refs: vec![
            consumer_ref_for(profile).to_owned(),
            WARM_START_CHOOSER_REF.to_owned(),
        ],
        notes: notes.to_owned(),
    }
}

fn key_rule(key: FingerprintKey, effect: &str, rationale: &str) -> KeyInvalidationRule {
    let class = key.compatibility_class();
    KeyInvalidationRule {
        rule_id: format!("invalidation.key.{}", key.as_str()),
        key,
        compatibility_class: class,
        drift_outcome: class.drift_outcome(),
        absent_outcome: class.absent_outcome(),
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn drill_steps(
    degraded: StartOutcome,
    posture: WarmStartPosture,
    drift: &str,
) -> Vec<PrebuildDrillStep> {
    let warm = WarmStartPosture::WarmFullReuse;
    vec![
        PrebuildDrillStep {
            phase: DrillPhase::Inject,
            observed_outcome: StartOutcome::Warm,
            observed_warm_start_posture: warm,
            narration: format!(
                "{drift} is introduced under the pinned snapshot while it still loads warm."
            ),
        },
        PrebuildDrillStep {
            phase: DrillPhase::Observe,
            observed_outcome: StartOutcome::Warm,
            observed_warm_start_posture: warm,
            narration: "The recorded fingerprint is compared key-by-key against current truth."
                .to_owned(),
        },
        PrebuildDrillStep {
            phase: DrillPhase::Narrow,
            observed_outcome: degraded,
            observed_warm_start_posture: posture,
            narration: format!(
                "The engine narrows reuse to {}; the snapshot is not served as current truth.",
                degraded.as_str()
            ),
        },
        PrebuildDrillStep {
            phase: DrillPhase::Refresh,
            observed_outcome: degraded,
            observed_warm_start_posture: posture,
            narration: "The affected layer is rebuilt and re-fingerprinted against current truth."
                .to_owned(),
        },
        PrebuildDrillStep {
            phase: DrillPhase::Recover,
            observed_outcome: StartOutcome::Warm,
            observed_warm_start_posture: warm,
            narration:
                "The recorded fingerprint matches current truth again; reuse returns to warm."
                    .to_owned(),
        },
        PrebuildDrillStep {
            phase: DrillPhase::Verify,
            observed_outcome: StartOutcome::Warm,
            observed_warm_start_posture: warm,
            narration: "The recovered outcome matches the engine for a fully current snapshot."
                .to_owned(),
        },
    ]
}

#[allow(clippy::too_many_arguments)]
fn drill(
    drill_id: &str,
    title: &str,
    injected_reason: PrebuildReason,
    profile: EnvironmentProfile,
    drift: &str,
    drifted: &[FingerprintKey],
    lost: &[(ArtifactLayer, ArtifactIntegrity)],
    notes: &str,
) -> PrebuildDrill {
    let Scenario { snapshot, current } = scenario(
        &format!("snapshot.drill.{}", profile.as_str()),
        profile,
        "Drill snapshot",
        drifted,
        &[],
        lost,
    );
    let decision = evaluate_prebuild_reuse(&snapshot, &current);
    PrebuildDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        injected_reason,
        profile,
        baseline_outcome: StartOutcome::Warm,
        degraded_outcome: decision.outcome,
        degraded_warm_start_posture: decision.warm_start_posture,
        disables_reuse: !decision.outcome.reuses_snapshot(),
        steps: drill_steps(decision.outcome, decision.warm_start_posture, drift),
        recovers_to_outcome: StartOutcome::Warm,
        asserts_reuse_blocked_under_drift: true,
        asserts_recovers_after_rebuild: true,
        notes: notes.to_owned(),
    }
}

/// Returns the checked-in prebuild-fingerprint packet this lane freezes.
pub fn seeded_prebuild_fingerprint_packet() -> PrebuildFingerprintPacket {
    let key_invalidation_rules = vec![
        key_rule(
            FingerprintKey::SourceTreeIdentity,
            "A source-tree drift rejects the snapshot for a cold build.",
            "A snapshot built from a different source tree would serve stale tools and indexes as current truth, so it is rebuilt cold.",
        ),
        key_rule(
            FingerprintKey::CapsuleHash,
            "A capsule-hash drift rejects the snapshot for a cold build.",
            "A snapshot built from a different capsule definition is for a different environment, so it is rebuilt cold rather than reused.",
        ),
        key_rule(
            FingerprintKey::PlatformArch,
            "A platform / architecture drift invalidates and evicts the snapshot.",
            "A snapshot built for a different platform is binary-incompatible and cannot be partially reused; it is invalidated.",
        ),
        key_rule(
            FingerprintKey::PolicyEpoch,
            "A policy-epoch drift invalidates and evicts the snapshot.",
            "A snapshot built under an older policy epoch may carry capabilities the current policy disallows, so it is invalidated rather than trusted.",
        ),
        key_rule(
            FingerprintKey::ExtensionLockDigest,
            "An extension-lock drift narrows reuse to a partial warm start; the extension layer is rebuilt.",
            "An extension-lock drift affects only the extension layer, so the unaffected base, toolchain, and dependency layers stay warm.",
        ),
        key_rule(
            FingerprintKey::ToolchainDigest,
            "A toolchain drift narrows reuse to a partial warm start; the toolchain layer is rebuilt.",
            "A critical toolchain drift affects only the toolchain layer, so the unaffected layers stay warm while the toolchain is rebuilt.",
        ),
    ];

    let artifact_invalidation_rules = vec![
        ArtifactInvalidationRule {
            rule_id: "invalidation.artifact.partial_loss".to_owned(),
            critical: false,
            loss_outcome: StartOutcome::PartiallyWarm,
            effect: "Losing a non-critical artifact narrows reuse to a partial warm start; the lost layer is rebuilt.".to_owned(),
            rationale: "A lost dependency, extension, or index layer can be rebuilt over the intact base and toolchain, so the start stays partially warm.".to_owned(),
        },
        ArtifactInvalidationRule {
            rule_id: "invalidation.artifact.critical_loss".to_owned(),
            critical: true,
            loss_outcome: StartOutcome::Cold,
            effect: "Losing a critical artifact rejects the snapshot for a cold build.".to_owned(),
            rationale: "A lost base image or toolchain layer cannot be rebuilt over, so the snapshot is rebuilt cold rather than served partially.".to_owned(),
        },
    ];

    let cases = vec![
        case(
            "case.prebuild.full_match",
            "A fully matching prebuild snapshot warm-reuses in full",
            EnvironmentProfile::Prebuild,
            "Prebuilt environment snapshot",
            &[],
            &[],
            &[],
            "Every key matches and every critical artifact is intact, so the whole snapshot is warm-reused.",
        ),
        case(
            "case.devcontainer.extension_lock_drift",
            "A devcontainer with extension-lock drift warm-reuses partially",
            EnvironmentProfile::Devcontainer,
            "Devcontainer layer-cache snapshot",
            &[FingerprintKey::ExtensionLockDigest],
            &[],
            &[],
            "An extension-lock drift rebuilds the extension layer while the base, toolchain, and dependency layers stay warm.",
        ),
        case(
            "case.remote_container.partial_artifact_loss",
            "A remote container with a lost index warm-reuses partially",
            EnvironmentProfile::RemoteContainer,
            "Remote-container snapshot",
            &[],
            &[],
            &[(ArtifactLayer::SearchIndex, ArtifactIntegrity::Missing)],
            "A lost search index narrows reuse to partial and rebuilds only the index, gating search until it returns.",
        ),
        case(
            "case.starter.source_drift",
            "A starter with source-tree drift is rejected for a cold build",
            EnvironmentProfile::Starter,
            "Starter prebuild snapshot",
            &[FingerprintKey::SourceTreeIdentity],
            &[],
            &[],
            "A source-tree drift rejects the snapshot for a cold build rather than serving a stale tree's tools as current.",
        ),
        case(
            "case.managed_workspace.platform_drift",
            "A managed workspace with platform drift is invalidated",
            EnvironmentProfile::ManagedWorkspace,
            "Managed-workspace snapshot",
            &[FingerprintKey::PlatformArch],
            &[],
            &[],
            "A platform / architecture drift invalidates and evicts the snapshot because it is binary-incompatible with the host.",
        ),
        case(
            "case.prebuild.policy_drift",
            "A prebuild with policy-epoch drift is invalidated",
            EnvironmentProfile::Prebuild,
            "Prebuilt environment snapshot",
            &[FingerprintKey::PolicyEpoch],
            &[],
            &[],
            "A policy-epoch drift invalidates the snapshot rather than trusting capabilities the current policy may disallow.",
        ),
    ];

    let drills = vec![
        drill(
            "drill.prebuild.source_drift",
            "Source-tree drift rejects the snapshot for a cold build",
            PrebuildReason::SourceDrift,
            EnvironmentProfile::Starter,
            "A source-tree edit",
            &[FingerprintKey::SourceTreeIdentity],
            &[],
            "Source-tree drift rejects warm reuse so a stale tree's tools and indexes are never served as current truth.",
        ),
        drill(
            "drill.prebuild.policy_drift",
            "Policy-epoch drift invalidates the snapshot",
            PrebuildReason::PolicyDrift,
            EnvironmentProfile::ManagedWorkspace,
            "A policy-epoch advance",
            &[FingerprintKey::PolicyEpoch],
            &[],
            "Policy-epoch drift invalidates the snapshot so it cannot carry capabilities the current policy disallows.",
        ),
        drill(
            "drill.prebuild.platform_drift",
            "Platform / architecture drift invalidates the snapshot",
            PrebuildReason::PlatformDrift,
            EnvironmentProfile::Prebuild,
            "A platform / architecture change",
            &[FingerprintKey::PlatformArch],
            &[],
            "Platform drift invalidates the binary-incompatible snapshot rather than reusing it on the wrong host.",
        ),
        drill(
            "drill.prebuild.extension_lock_drift",
            "Extension-lock drift narrows reuse to a partial warm start",
            PrebuildReason::ExtensionLockDrift,
            EnvironmentProfile::Devcontainer,
            "An extension-lock change",
            &[FingerprintKey::ExtensionLockDigest],
            &[],
            "Extension-lock drift rebuilds only the extension layer while the rest of the snapshot stays warm.",
        ),
        drill(
            "drill.prebuild.partial_artifact_loss",
            "Partial artifact loss narrows reuse to a partial warm start",
            PrebuildReason::PartialArtifactLoss,
            EnvironmentProfile::RemoteContainer,
            "A lost search-index artifact",
            &[],
            &[(ArtifactLayer::SearchIndex, ArtifactIntegrity::Missing)],
            "A lost index narrows reuse to partial and gates search until the index is rebuilt.",
        ),
    ];

    PrebuildFingerprintPacket {
        record_kind: PREBUILD_FINGERPRINT_PACKET_RECORD_KIND.to_owned(),
        schema_version: PREBUILD_FINGERPRINT_SCHEMA_VERSION,
        packet_id: PREBUILD_FINGERPRINT_PACKET_ID.to_owned(),
        title: "Prebuild-snapshot compatibility fingerprints, invalidation rules, and warm-start downgrade truth for claimed M5 warm-start paths".to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: PREBUILD_FINGERPRINT_DOC_REF.to_owned(),
            schema_ref: PREBUILD_FINGERPRINT_SCHEMA_REF.to_owned(),
            packet_ref: PREBUILD_FINGERPRINT_PACKET_REF.to_owned(),
            report_ref: PREBUILD_FINGERPRINT_PROOF_REF.to_owned(),
            fixture_manifest_ref: PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF.to_owned(),
        },
        fingerprint_keys: FingerprintKey::ALL.to_vec(),
        key_invalidation_rules,
        artifact_invalidation_rules,
        cases,
        drills,
        evidence_packet_refs: vec![
            WARM_START_CHOOSER_REF.to_owned(),
            BUILD_IDENTITY_REF.to_owned(),
            HOST_BOUNDARY_REF.to_owned(),
            STATE_ROOT_REF.to_owned(),
            EXECUTION_SCOPE_REF.to_owned(),
            MANAGED_LIFECYCLE_REF.to_owned(),
        ],
        invariants: vec![
            "A prebuild snapshot is reused only when every fingerprint key — source-tree identity, capsule hash, platform/arch, policy epoch, extension lock, and critical toolchain digests — matches current truth and its critical artifacts are intact.".to_owned(),
            "Drift class decides the floor: platform and policy drift invalidate the snapshot, source-tree and capsule drift force a cold build, and extension-lock and toolchain drift narrow to a partial warm start; the coldest contribution always wins.".to_owned(),
            "Prebuild speed is never proof of compatibility: a fast-loading snapshot that no longer matches the source, policy, or platform is rejected or downgraded rather than served as current truth.".to_owned(),
            "Every decision is metadata-first and self-explaining, so users and support tooling can distinguish a warm, partially warm, cold, or invalidated start and read the exact key or artifact that forced it.".to_owned(),
            "A downgraded start narrows or disables the actions whose backing layer is being rebuilt instead of presenting stale tools or indexes as current truth.".to_owned(),
            "The outcome maps onto the same governance warm-start posture the environment-capsule lane reads, so the prebuild lane narrows in lockstep instead of forking a parallel warm-start model.".to_owned(),
        ],
    }
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    fixture_id: &str,
    injected_reason: PrebuildReason,
    profile: EnvironmentProfile,
    label: &str,
    drifted: &[FingerprintKey],
    absent: &[FingerprintKey],
    lost: &[(ArtifactLayer, ArtifactIntegrity)],
    notes: &str,
) -> PrebuildFingerprintFixture {
    let Scenario { snapshot, current } = scenario(
        &format!("snapshot.fixture.{fixture_id}"),
        profile,
        label,
        drifted,
        absent,
        lost,
    );
    let decision = evaluate_prebuild_reuse(&snapshot, &current);
    PrebuildFingerprintFixture {
        record_kind: PREBUILD_FINGERPRINT_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: PREBUILD_FINGERPRINT_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        injected_reason,
        snapshot,
        current_fingerprint: current,
        expected_outcome: decision.outcome,
        expected_reused: decision.reused,
        expected_invalidated: decision.invalidated,
        expected_warm_start_posture: decision.warm_start_posture,
        expected_headline_reason: decision.headline_reason,
        expected_reason_tokens: decision.reason_tokens,
        expected_gated_action_tokens: decision.gated_action_tokens,
        consumer_ref: consumer_ref_for(profile).to_owned(),
        notes: notes.to_owned(),
    }
}

/// Returns the checked-in fixture corpus this lane freezes: a warm
/// baseline plus a degraded variant for every key-drift class, both
/// artifact-loss classes, and an unrecorded key.
pub fn seeded_prebuild_fingerprint_fixtures() -> Vec<PrebuildFingerprintFixture> {
    vec![
        fixture(
            "full_match_warm",
            PrebuildReason::FullMatch,
            EnvironmentProfile::Prebuild,
            "Prebuilt environment snapshot",
            &[],
            &[],
            &[],
            "A fully matching snapshot warm-reuses in full with no gated actions.",
        ),
        fixture(
            "source_drift_cold",
            PrebuildReason::SourceDrift,
            EnvironmentProfile::Starter,
            "Starter prebuild snapshot",
            &[FingerprintKey::SourceTreeIdentity],
            &[],
            &[],
            "Source-tree drift rejects the snapshot for a cold build.",
        ),
        fixture(
            "capsule_drift_cold",
            PrebuildReason::CapsuleDrift,
            EnvironmentProfile::Devcontainer,
            "Devcontainer layer-cache snapshot",
            &[FingerprintKey::CapsuleHash],
            &[],
            &[],
            "Capsule-hash drift rejects the snapshot for a cold build.",
        ),
        fixture(
            "policy_drift_invalidated",
            PrebuildReason::PolicyDrift,
            EnvironmentProfile::ManagedWorkspace,
            "Managed-workspace snapshot",
            &[FingerprintKey::PolicyEpoch],
            &[],
            &[],
            "Policy-epoch drift invalidates and evicts the snapshot.",
        ),
        fixture(
            "platform_drift_invalidated",
            PrebuildReason::PlatformDrift,
            EnvironmentProfile::Prebuild,
            "Prebuilt environment snapshot",
            &[FingerprintKey::PlatformArch],
            &[],
            &[],
            "Platform / architecture drift invalidates and evicts the snapshot.",
        ),
        fixture(
            "extension_lock_drift_partial",
            PrebuildReason::ExtensionLockDrift,
            EnvironmentProfile::Devcontainer,
            "Devcontainer layer-cache snapshot",
            &[FingerprintKey::ExtensionLockDigest],
            &[],
            &[],
            "Extension-lock drift narrows reuse to a partial warm start and rebuilds the extension layer.",
        ),
        fixture(
            "toolchain_drift_partial",
            PrebuildReason::ToolchainDrift,
            EnvironmentProfile::RemoteContainer,
            "Remote-container snapshot",
            &[FingerprintKey::ToolchainDigest],
            &[],
            &[],
            "Toolchain drift narrows reuse to a partial warm start and rebuilds the toolchain layer.",
        ),
        fixture(
            "partial_artifact_loss_partial",
            PrebuildReason::PartialArtifactLoss,
            EnvironmentProfile::RemoteContainer,
            "Remote-container snapshot",
            &[],
            &[],
            &[(ArtifactLayer::SearchIndex, ArtifactIntegrity::Missing)],
            "A lost search index narrows reuse to a partial warm start and gates search.",
        ),
        fixture(
            "critical_artifact_loss_cold",
            PrebuildReason::CriticalArtifactLoss,
            EnvironmentProfile::Prebuild,
            "Prebuilt environment snapshot",
            &[],
            &[],
            &[(ArtifactLayer::BaseImage, ArtifactIntegrity::Corrupt)],
            "A corrupt base image rejects the snapshot for a cold build.",
        ),
        fixture(
            "source_tree_absent_cold",
            PrebuildReason::UnprovenKey,
            EnvironmentProfile::Starter,
            "Starter prebuild snapshot",
            &[],
            &[FingerprintKey::SourceTreeIdentity],
            &[],
            "An unrecorded source-tree key cannot prove compatibility, so the snapshot is rebuilt cold.",
        ),
    ]
}
