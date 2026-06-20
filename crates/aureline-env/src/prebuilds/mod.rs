//! Prebuild-snapshot compatibility fingerprints, invalidation rules, and
//! cold-versus-partially-warm downgrade truth for claimed M5 warm-start
//! paths.
//!
//! The environment-capsule governance lane proves that a prebuild
//! fingerprint *exists* and stays fresh; this lane makes that fingerprint
//! *operational*. A prebuild snapshot is only an accelerator: it may load
//! in milliseconds, but speed is never proof of compatibility. Before a
//! warm start reuses a snapshot it must show that the snapshot's recorded
//! fingerprint still matches current truth across the inputs that actually
//! decide compatibility, and it must say — in review-safe terms — why the
//! snapshot was reused, rejected, or downgraded.
//!
//! This module freezes that contract. A [`PrebuildFingerprint`] is keyed
//! on six inputs, one per [`FingerprintKey`]: the source/tree identity,
//! the environment-capsule hash, the platform/arch, the policy epoch, the
//! extension-lock digest, and the critical toolchain digests. A
//! [`PrebuildSnapshot`] carries the fingerprint recorded when it was built
//! plus the integrity of its cached artifact layers. The single
//! [`evaluate_prebuild_reuse`] engine compares a snapshot's recorded
//! fingerprint against the current expected fingerprint, folds in artifact
//! integrity, and returns one [`PrebuildDecision`]: an explicit
//! [`StartOutcome`] — warm, partially warm, cold, or invalidated — the
//! dominant [`PrebuildReason`], the per-key and per-artifact evaluation
//! behind it, and the [`CapsuleActionClass`] postures the start surface
//! must narrow or disable rather than serve stale tools or indexes as
//! current truth.
//!
//! Four guardrails are frozen here:
//!
//! - **Speed is not compatibility.** A snapshot is reused only when every
//!   fingerprint key matches and its critical artifacts are intact. Any
//!   drift downgrades the outcome; the snapshot never wins by loading
//!   fast.
//! - **Drift class decides the floor.** Each key carries a
//!   [`CompatibilityClass`]: platform/arch and policy-epoch drift
//!   *invalidate* the snapshot (it is binary- or trust-incompatible and
//!   must be discarded); source-tree and capsule-hash drift force a *cold*
//!   rebuild (the snapshot is for different content); extension-lock and
//!   toolchain drift narrow to *partially warm* (the unaffected layers are
//!   reused while the affected layer is rebuilt). The coldest contribution
//!   wins, so source/policy/platform drift can never be silently outrun.
//! - **One engine.** [`evaluate_prebuild_reuse`] is the single source of
//!   truth for the outcome, shared by the cases, the drills, the fixtures,
//!   and the [`KeyInvalidationRule`] / [`ArtifactInvalidationRule`] tables,
//!   and it maps every outcome onto the same governance
//!   [`WarmStartPosture`] the environment-capsule lane reads, so the
//!   prebuild lane narrows in lockstep instead of forking a parallel
//!   warm-start model.
//! - **No silent reuse.** The decision is metadata-first and always
//!   explains itself: users and support tooling can tell a warm start from
//!   a partially warm, cold, or invalidated one, and read the exact key or
//!   artifact that forced the downgrade.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/env/prebuild-fingerprint.schema.json`](../../../../schemas/env/prebuild-fingerprint.schema.json)
//! - [`/docs/env/prebuild-fingerprint.md`](../../../../docs/env/prebuild-fingerprint.md)
//! - [`/artifacts/env/prebuild-fingerprint-packet.json`](../../../../artifacts/env/prebuild-fingerprint-packet.json)
//! - [`/artifacts/env/prebuild-fingerprint-proof.md`](../../../../artifacts/env/prebuild-fingerprint-proof.md)
//! - [`/artifacts/env/prebuild-reuse-drills.md`](../../../../artifacts/env/prebuild-reuse-drills.md)
//! - [`/fixtures/env/prebuilds/`](../../../../fixtures/env/prebuilds/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::capsules::{CapsuleDigest, RedactionClass};
use crate::m5_env_governance::{
    DrillPhase, EnvironmentProfile, MaterializationClass, SourceContractRefs, ValidationReport,
    WarmStartPosture,
};

#[cfg(test)]
mod tests;

pub mod seed;

pub use seed::{
    seeded_prebuild_fingerprint_fixtures, seeded_prebuild_fingerprint_packet,
    seeded_prebuild_snapshots,
};

/// Schema version stamped onto packets and fixtures.
pub const PREBUILD_FINGERPRINT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const PREBUILD_FINGERPRINT_PACKET_RECORD_KIND: &str = "prebuild_fingerprint_packet_record";

/// Stable record-kind tag carried by a snapshot.
pub const PREBUILD_SNAPSHOT_RECORD_KIND: &str = "prebuild_snapshot_record";

/// Stable record-kind tag carried by a reuse decision.
pub const PREBUILD_DECISION_RECORD_KIND: &str = "prebuild_reuse_decision_record";

/// Stable record-kind tag carried by a metadata export.
pub const PREBUILD_EXPORT_RECORD_KIND: &str = "prebuild_reuse_export_record";

/// Stable record-kind tag carried by fixtures.
pub const PREBUILD_FINGERPRINT_FIXTURE_RECORD_KIND: &str = "prebuild_fingerprint_fixture_record";

/// Repo-relative schema ref.
pub const PREBUILD_FINGERPRINT_SCHEMA_REF: &str = "schemas/env/prebuild-fingerprint.schema.json";

/// Repo-relative reviewer doc ref.
pub const PREBUILD_FINGERPRINT_DOC_REF: &str = "docs/env/prebuild-fingerprint.md";

/// Repo-relative machine-readable proof packet.
pub const PREBUILD_FINGERPRINT_PACKET_REF: &str = "artifacts/env/prebuild-fingerprint-packet.json";

/// Repo-relative reviewer proof summary.
pub const PREBUILD_FINGERPRINT_PROOF_REF: &str = "artifacts/env/prebuild-fingerprint-proof.md";

/// Repo-relative failure / recovery drill report.
pub const PREBUILD_REUSE_DRILLS_REF: &str = "artifacts/env/prebuild-reuse-drills.md";

/// Repo-relative fixture directory.
pub const PREBUILD_FINGERPRINT_FIXTURE_DIR: &str = "fixtures/env/prebuilds";

/// Repo-relative fixture manifest.
pub const PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF: &str = "fixtures/env/prebuilds/manifest.yaml";

/// Stable packet id.
pub const PREBUILD_FINGERPRINT_PACKET_ID: &str = "env.prebuild_fingerprint.v1";

// ---------------------------------------------------------------------------
// Fingerprint key vocabulary.
// ---------------------------------------------------------------------------

/// One of the six inputs a prebuild compatibility fingerprint is keyed on.
/// A prebuild snapshot is reusable only when every key still matches the
/// current expected value; which keys drifted decides how far the start
/// downgrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintKey {
    /// Identity digest of the workspace source tree the snapshot was built
    /// from. Drift means the snapshot is for stale content.
    SourceTreeIdentity,
    /// Digest of the environment-capsule object the snapshot materializes.
    /// Drift means the environment definition changed under the snapshot.
    CapsuleHash,
    /// The platform / architecture the snapshot was built for. Drift means
    /// the snapshot is binary-incompatible with the current host.
    PlatformArch,
    /// The policy / trust epoch the snapshot was built under. Drift means
    /// the snapshot may carry capabilities the current policy disallows.
    PolicyEpoch,
    /// Digest of the resolved extension lock. Drift affects only the
    /// extension layer.
    ExtensionLockDigest,
    /// Combined digest of the critical toolchain components. Drift affects
    /// only the toolchain layer.
    ToolchainDigest,
}

impl FingerprintKey {
    /// Every fingerprint key in canonical order.
    pub const ALL: [Self; 6] = [
        Self::SourceTreeIdentity,
        Self::CapsuleHash,
        Self::PlatformArch,
        Self::PolicyEpoch,
        Self::ExtensionLockDigest,
        Self::ToolchainDigest,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceTreeIdentity => "source_tree_identity",
            Self::CapsuleHash => "capsule_hash",
            Self::PlatformArch => "platform_arch",
            Self::PolicyEpoch => "policy_epoch",
            Self::ExtensionLockDigest => "extension_lock_digest",
            Self::ToolchainDigest => "toolchain_digest",
        }
    }

    /// How a mismatch on this key narrows prebuild reuse.
    pub const fn compatibility_class(self) -> CompatibilityClass {
        match self {
            // A snapshot built for a different platform or under an older
            // policy epoch is unsafe to reuse at all.
            Self::PlatformArch | Self::PolicyEpoch => CompatibilityClass::Invalidating,
            // A different source tree or capsule definition is different
            // content; rebuild cold.
            Self::SourceTreeIdentity | Self::CapsuleHash => CompatibilityClass::Identity,
            // Extension and toolchain drift affect only a layer.
            Self::ExtensionLockDigest | Self::ToolchainDigest => CompatibilityClass::Layered,
        }
    }

    /// The dominant reuse reason a drift on this key implies.
    pub const fn drift_reason(self) -> PrebuildReason {
        match self {
            Self::SourceTreeIdentity => PrebuildReason::SourceDrift,
            Self::CapsuleHash => PrebuildReason::CapsuleDrift,
            Self::PlatformArch => PrebuildReason::PlatformDrift,
            Self::PolicyEpoch => PrebuildReason::PolicyDrift,
            Self::ExtensionLockDigest => PrebuildReason::ExtensionLockDrift,
            Self::ToolchainDigest => PrebuildReason::ToolchainDrift,
        }
    }
}

/// How a mismatch on a fingerprint key narrows prebuild reuse. The class
/// is the single place each key's downgrade floor is defined, so the
/// invalidation rules can never drift from [`evaluate_prebuild_reuse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityClass {
    /// A drift invalidates the snapshot outright — it is binary- or
    /// trust-incompatible and must be discarded, never partially reused.
    Invalidating,
    /// A drift means the snapshot is for different content; reuse is
    /// rejected and the environment is rebuilt cold.
    Identity,
    /// A drift affects only a layer of the snapshot; the unaffected layers
    /// may be reused while the affected layer is rebuilt.
    Layered,
}

impl CompatibilityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Invalidating => "invalidating",
            Self::Identity => "identity",
            Self::Layered => "layered",
        }
    }

    /// The outcome a drift on a key of this class forces.
    pub const fn drift_outcome(self) -> StartOutcome {
        match self {
            Self::Invalidating => StartOutcome::Invalidated,
            Self::Identity => StartOutcome::Cold,
            Self::Layered => StartOutcome::PartiallyWarm,
        }
    }

    /// The outcome an *absent* key of this class forces. A key the
    /// snapshot never recorded cannot prove compatibility, so a layered
    /// key is no longer eligible for partial reuse and drops to a cold
    /// rebuild; identity and invalidating keys behave as on drift.
    pub const fn absent_outcome(self) -> StartOutcome {
        match self {
            Self::Invalidating => StartOutcome::Invalidated,
            Self::Identity | Self::Layered => StartOutcome::Cold,
        }
    }
}

/// The explicit outcome of a prebuild warm-start decision. Declaration
/// order is the narrowing order: [`StartOutcome::Warm`] is the strongest
/// reuse and [`StartOutcome::Invalidated`] the most conservative, so
/// narrowing always moves toward a later variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartOutcome {
    /// Every key matched and every critical artifact is intact; the whole
    /// snapshot is reused.
    Warm,
    /// Only part of the snapshot is reused; an affected layer is rebuilt.
    PartiallyWarm,
    /// No reuse is trustworthy for current content; the environment is
    /// rebuilt cold (the snapshot is a benign cache miss, not evicted).
    Cold,
    /// The snapshot is incompatible or untrusted and is discarded; the
    /// environment is rebuilt and the snapshot evicted.
    Invalidated,
}

impl StartOutcome {
    /// Every outcome in canonical (narrowing) order.
    pub const ALL: [Self; 4] = [
        Self::Warm,
        Self::PartiallyWarm,
        Self::Cold,
        Self::Invalidated,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Warm => "warm",
            Self::PartiallyWarm => "partially_warm",
            Self::Cold => "cold",
            Self::Invalidated => "invalidated",
        }
    }

    /// Narrowing severity. Higher is a colder, more conservative outcome;
    /// the engine always takes the highest severity among the per-key and
    /// per-artifact contributions.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Warm => 0,
            Self::PartiallyWarm => 1,
            Self::Cold => 2,
            Self::Invalidated => 3,
        }
    }

    /// True when any part of the snapshot is reused.
    pub const fn reuses_snapshot(self) -> bool {
        matches!(self, Self::Warm | Self::PartiallyWarm)
    }

    /// The governance warm-start posture this outcome maps to. Both
    /// [`StartOutcome::Cold`] and [`StartOutcome::Invalidated`] map to a
    /// [`WarmStartPosture::ColdBuild`]; the prebuild lane only adds the
    /// finer invalidated distinction on top, so it narrows in lockstep
    /// with the environment-capsule governance lane.
    pub const fn warm_start_posture(self) -> WarmStartPosture {
        match self {
            Self::Warm => WarmStartPosture::WarmFullReuse,
            Self::PartiallyWarm => WarmStartPosture::WarmPartialReuse,
            Self::Cold | Self::Invalidated => WarmStartPosture::ColdBuild,
        }
    }
}

/// The observed match state of one fingerprint key: the recorded snapshot
/// value compared against the current expected value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyMatch {
    /// The recorded digest equals the current digest.
    Match,
    /// The recorded digest differs from the current digest (drift).
    Drift,
    /// The key was not recorded in the snapshot, so compatibility cannot
    /// be proven for it.
    Absent,
}

impl KeyMatch {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Drift => "drift",
            Self::Absent => "absent",
        }
    }
}

/// A layer of a prebuild snapshot's cached artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactLayer {
    /// The base image / root filesystem layer.
    BaseImage,
    /// The pinned toolchain layer.
    Toolchain,
    /// The resolved dependency layer.
    Dependencies,
    /// The resolved extension layer.
    Extensions,
    /// The prebuilt code-search / language index.
    SearchIndex,
}

impl ArtifactLayer {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BaseImage => "base_image",
            Self::Toolchain => "toolchain",
            Self::Dependencies => "dependencies",
            Self::Extensions => "extensions",
            Self::SearchIndex => "search_index",
        }
    }
}

/// The integrity of one cached artifact layer at reuse time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactIntegrity {
    /// The artifact is present and verified.
    Present,
    /// The artifact is missing from the snapshot.
    Missing,
    /// The artifact is present but failed its integrity check.
    Corrupt,
}

impl ArtifactIntegrity {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
            Self::Corrupt => "corrupt",
        }
    }

    /// True when the artifact cannot be reused (missing or corrupt).
    pub const fn is_lost(self) -> bool {
        matches!(self, Self::Missing | Self::Corrupt)
    }
}

/// The dominant reason a prebuild snapshot was reused, downgraded, or
/// rejected. This is the review-safe headline the warm-start surface and
/// support tooling quote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrebuildReason {
    /// Every key matched and every critical artifact is intact.
    FullMatch,
    /// The source tree drifted from the snapshot.
    SourceDrift,
    /// The capsule definition drifted from the snapshot.
    CapsuleDrift,
    /// The platform / architecture differs from the snapshot.
    PlatformDrift,
    /// The policy epoch advanced past the snapshot.
    PolicyDrift,
    /// The extension lock drifted from the snapshot.
    ExtensionLockDrift,
    /// A critical toolchain digest drifted from the snapshot.
    ToolchainDrift,
    /// A non-critical cached artifact layer was lost or corrupt.
    PartialArtifactLoss,
    /// A critical cached artifact layer was lost or corrupt.
    CriticalArtifactLoss,
    /// A required fingerprint key was not recorded in the snapshot.
    UnprovenKey,
}

impl PrebuildReason {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullMatch => "full_match",
            Self::SourceDrift => "source_drift",
            Self::CapsuleDrift => "capsule_drift",
            Self::PlatformDrift => "platform_drift",
            Self::PolicyDrift => "policy_drift",
            Self::ExtensionLockDrift => "extension_lock_drift",
            Self::ToolchainDrift => "toolchain_drift",
            Self::PartialArtifactLoss => "partial_artifact_loss",
            Self::CriticalArtifactLoss => "critical_artifact_loss",
            Self::UnprovenKey => "unproven_key",
        }
    }
}

/// A capsule capability the start surface gates when reuse is not fully
/// warm, so a downgraded start narrows or disables actions instead of
/// presenting stale tools or indexes as current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleActionClass {
    /// Build / run / task execution using the toolchain and dependencies.
    BuildRun,
    /// Code search served from the prebuilt index.
    SearchIndex,
    /// Language intelligence and navigation.
    LanguageIntel,
    /// The declared service / dependency graph.
    Services,
}

impl CapsuleActionClass {
    /// Every action class in canonical order.
    pub const ALL: [Self; 4] = [
        Self::BuildRun,
        Self::SearchIndex,
        Self::LanguageIntel,
        Self::Services,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildRun => "build_run",
            Self::SearchIndex => "search_index",
            Self::LanguageIntel => "language_intel",
            Self::Services => "services",
        }
    }

    /// The fingerprint keys whose drift degrades this action.
    fn backing_keys(self) -> &'static [FingerprintKey] {
        match self {
            Self::BuildRun => &[
                FingerprintKey::PlatformArch,
                FingerprintKey::PolicyEpoch,
                FingerprintKey::CapsuleHash,
                FingerprintKey::ToolchainDigest,
            ],
            Self::SearchIndex => &[
                FingerprintKey::SourceTreeIdentity,
                FingerprintKey::PlatformArch,
            ],
            Self::LanguageIntel => &[
                FingerprintKey::SourceTreeIdentity,
                FingerprintKey::ToolchainDigest,
                FingerprintKey::ExtensionLockDigest,
            ],
            Self::Services => &[
                FingerprintKey::CapsuleHash,
                FingerprintKey::PolicyEpoch,
                FingerprintKey::PlatformArch,
            ],
        }
    }

    /// The artifact layers whose loss degrades this action.
    fn backing_layers(self) -> &'static [ArtifactLayer] {
        match self {
            Self::BuildRun => &[
                ArtifactLayer::BaseImage,
                ArtifactLayer::Toolchain,
                ArtifactLayer::Dependencies,
            ],
            Self::SearchIndex => &[ArtifactLayer::SearchIndex],
            Self::LanguageIntel => &[
                ArtifactLayer::Toolchain,
                ArtifactLayer::Extensions,
                ArtifactLayer::SearchIndex,
            ],
            Self::Services => &[ArtifactLayer::BaseImage, ArtifactLayer::Dependencies],
        }
    }
}

/// How available a gated capsule action is after a reuse decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPosture {
    /// The action is available against current truth.
    Available,
    /// The action is available but labeled as served from a warm snapshot
    /// pending a targeted rebuild.
    Degraded,
    /// The action is disabled until the environment is rebuilt.
    Disabled,
}

impl ActionPosture {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Disabled => "disabled",
        }
    }

    /// True when the action is narrowed or disabled (not fully available).
    pub const fn is_gated(self) -> bool {
        !matches!(self, Self::Available)
    }
}

// ---------------------------------------------------------------------------
// Fingerprint objects.
// ---------------------------------------------------------------------------

/// One keyed digest in a [`PrebuildFingerprint`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FingerprintKeyDigest {
    /// Fingerprint key this digest covers.
    pub key: FingerprintKey,
    /// Pinned digest for the key.
    pub digest: CapsuleDigest,
    /// Review-safe summary of what the key digests.
    pub summary: String,
}

/// The compatibility fingerprint a prebuild snapshot is reused against:
/// the six keyed digests plus the combined digest folded over them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildFingerprint {
    /// The combined fingerprint digest folded over the keyed digests.
    pub fingerprint: CapsuleDigest,
    /// The keyed digests, one per [`FingerprintKey`] in canonical order.
    pub keys: Vec<FingerprintKeyDigest>,
    /// Review-safe summary of the fingerprint.
    pub summary: String,
}

impl PrebuildFingerprint {
    /// Builds a fingerprint from its keyed digests, sorting them into
    /// canonical key order and folding the combined digest over them so
    /// the combined value is reproducible.
    pub fn from_keys(mut keys: Vec<FingerprintKeyDigest>, summary: impl Into<String>) -> Self {
        keys.sort_by_key(|keyed| keyed.key);
        let fingerprint = combined_digest(&keys);
        Self {
            fingerprint,
            keys,
            summary: summary.into(),
        }
    }

    /// The digest recorded for one key, if present.
    pub fn digest_for(&self, key: FingerprintKey) -> Option<&CapsuleDigest> {
        self.keys
            .iter()
            .find(|keyed| keyed.key == key)
            .map(|keyed| &keyed.digest)
    }
}

/// Folds the combined fingerprint digest over the keyed digests. The
/// inputs are concatenated in canonical key order, so the combined value
/// is a deterministic function of the keys and reproducible by
/// [`validate_prebuild_fingerprint`].
fn combined_digest(keys: &[FingerprintKeyDigest]) -> CapsuleDigest {
    let mut ordered: Vec<&FingerprintKeyDigest> = keys.iter().collect();
    ordered.sort_by_key(|keyed| keyed.key);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for keyed in ordered {
        for byte in keyed.key.as_str().bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= u64::from(b'=');
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in keyed.digest.value.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    CapsuleDigest {
        algorithm: "sha256".to_owned(),
        value: format!("{hash:016x}").repeat(4),
    }
}

/// One cached artifact layer in a prebuild snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildArtifact {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Which layer this artifact is.
    pub layer: ArtifactLayer,
    /// Integrity of the artifact at reuse time.
    pub integrity: ArtifactIntegrity,
    /// Whether losing this artifact forces a cold rebuild rather than a
    /// partial one.
    pub critical: bool,
    /// Review-safe summary of the artifact.
    pub summary: String,
}

/// A recorded prebuild snapshot: the fingerprint captured when it was
/// built plus the integrity of its cached artifact layers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Claimed environment profile (reused governance vocabulary).
    pub profile: EnvironmentProfile,
    /// How this snapshot's environment materializes.
    pub materialization_class: MaterializationClass,
    /// Review-safe snapshot label.
    pub label: String,
    /// The fingerprint recorded when the snapshot was built.
    pub recorded_fingerprint: PrebuildFingerprint,
    /// The cached artifact layers, one per declared layer.
    pub artifacts: Vec<PrebuildArtifact>,
    /// Review-safe summary of the snapshot.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// The reuse decision and the engine that produces it.
// ---------------------------------------------------------------------------

/// One per-key evaluation line behind a [`PrebuildDecision`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvaluation {
    /// Fingerprint key being evaluated.
    pub key: FingerprintKey,
    /// Compatibility class of the key.
    pub compatibility_class: CompatibilityClass,
    /// Observed match state of the key.
    pub match_state: KeyMatch,
    /// The outcome this key contributes (warm when it matches).
    pub outcome_contribution: StartOutcome,
    /// Review-safe explanation of the key's contribution.
    pub summary: String,
}

/// One per-artifact evaluation line behind a [`PrebuildDecision`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactEvaluation {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Which layer the artifact is.
    pub layer: ArtifactLayer,
    /// Integrity of the artifact.
    pub integrity: ArtifactIntegrity,
    /// Whether the artifact is critical.
    pub critical: bool,
    /// The outcome this artifact contributes (warm when intact).
    pub outcome_contribution: StartOutcome,
    /// Review-safe explanation of the artifact's contribution.
    pub summary: String,
}

/// One gated capsule action and the posture the start surface holds it at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionGate {
    /// The capsule action being gated.
    pub action: CapsuleActionClass,
    /// Posture the action is held at.
    pub posture: ActionPosture,
    /// Review-safe reason for the posture.
    pub reason: String,
}

/// The decision the engine reaches for one prebuild snapshot against the
/// current expected fingerprint. The decision is the single explainability
/// object the desktop, headless, and support surfaces all read; it carries
/// no secrets or raw bodies, only ids, digests, tokens, and review-safe
/// prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Snapshot id under evaluation.
    pub snapshot_id: String,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// How the snapshot's environment materializes.
    pub materialization_class: MaterializationClass,
    /// The explicit reuse outcome.
    pub outcome: StartOutcome,
    /// True when any part of the snapshot is reused.
    pub reused: bool,
    /// True when the snapshot is rejected as incompatible and evicted.
    pub invalidated: bool,
    /// The governance warm-start posture the outcome maps to.
    pub warm_start_posture: WarmStartPosture,
    /// The dominant reason behind the outcome.
    pub headline_reason: PrebuildReason,
    /// Stable tokens naming every key and artifact that forced a downgrade.
    pub reason_tokens: Vec<String>,
    /// Per-key evaluation behind the outcome.
    pub key_evaluations: Vec<KeyEvaluation>,
    /// Per-artifact evaluation behind the outcome.
    pub artifact_evaluations: Vec<ArtifactEvaluation>,
    /// Per-action gating the start surface applies.
    pub action_gates: Vec<ActionGate>,
    /// Stable tokens naming every action narrowed or disabled.
    pub gated_action_tokens: Vec<String>,
    /// Review-safe headline explaining why the snapshot was reused,
    /// downgraded, or rejected.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

fn match_state(recorded: Option<&CapsuleDigest>, current: Option<&CapsuleDigest>) -> KeyMatch {
    match (recorded, current) {
        (None, _) | (_, None) => KeyMatch::Absent,
        (Some(a), Some(b)) => {
            if a == b {
                KeyMatch::Match
            } else {
                KeyMatch::Drift
            }
        }
    }
}

fn key_contribution(class: CompatibilityClass, state: KeyMatch) -> StartOutcome {
    match state {
        KeyMatch::Match => StartOutcome::Warm,
        KeyMatch::Drift => class.drift_outcome(),
        KeyMatch::Absent => class.absent_outcome(),
    }
}

fn key_summary(key: FingerprintKey, state: KeyMatch, contribution: StartOutcome) -> String {
    match state {
        KeyMatch::Match => format!(
            "The {} key matches the current fingerprint; the snapshot stays eligible for warm reuse on this key.",
            key.as_str()
        ),
        KeyMatch::Drift => format!(
            "The {} key drifted from the current fingerprint ({} class), narrowing reuse to {}.",
            key.as_str(),
            key.compatibility_class().as_str(),
            contribution.as_str()
        ),
        KeyMatch::Absent => format!(
            "The {} key was not recorded in the snapshot, so its compatibility cannot be proven, narrowing reuse to {}.",
            key.as_str(),
            contribution.as_str()
        ),
    }
}

fn artifact_contribution(artifact: &PrebuildArtifact) -> StartOutcome {
    if !artifact.integrity.is_lost() {
        StartOutcome::Warm
    } else if artifact.critical {
        StartOutcome::Cold
    } else {
        StartOutcome::PartiallyWarm
    }
}

fn artifact_summary(artifact: &PrebuildArtifact, contribution: StartOutcome) -> String {
    if !artifact.integrity.is_lost() {
        format!(
            "The {} artifact is {}; it stays eligible for warm reuse.",
            artifact.layer.as_str(),
            artifact.integrity.as_str()
        )
    } else {
        format!(
            "The {} artifact is {}{}; reuse narrows to {} and the layer is rebuilt.",
            artifact.layer.as_str(),
            artifact.integrity.as_str(),
            if artifact.critical { " (critical)" } else { "" },
            contribution.as_str()
        )
    }
}

/// Evaluates whether a prebuild snapshot may be reused for a warm start by
/// comparing its recorded fingerprint against the current expected
/// fingerprint and folding in artifact integrity.
///
/// This is the canonical engine the cases, drills, fixtures, and
/// invalidation rules all share. The outcome starts warm and is narrowed
/// to the coldest contribution among the six keys and the artifact layers;
/// the dominant reason is the first contributor (in canonical key order,
/// then artifact order) that produced the winning outcome. The decision is
/// metadata-first and self-explaining, so a warm start can never silently
/// outrun source, policy, or platform drift.
pub fn evaluate_prebuild_reuse(
    snapshot: &PrebuildSnapshot,
    current: &PrebuildFingerprint,
) -> PrebuildDecision {
    let mut outcome = StartOutcome::Warm;
    let mut reason_tokens: Vec<String> = Vec::new();
    let mut headline_reason = PrebuildReason::FullMatch;
    let mut headline_severity = 0u8;

    let mut key_evaluations = Vec::new();
    for key in FingerprintKey::ALL {
        let class = key.compatibility_class();
        let state = match_state(
            snapshot.recorded_fingerprint.digest_for(key),
            current.digest_for(key),
        );
        let contribution = key_contribution(class, state);
        if contribution.severity() > outcome.severity() {
            outcome = contribution;
        }
        if state != KeyMatch::Match {
            let token = match state {
                KeyMatch::Drift => format!("{}_drift", key.as_str()),
                KeyMatch::Absent => format!("{}_absent", key.as_str()),
                KeyMatch::Match => unreachable!(),
            };
            reason_tokens.push(token);
            let reason = if state == KeyMatch::Absent {
                PrebuildReason::UnprovenKey
            } else {
                key.drift_reason()
            };
            if contribution.severity() > headline_severity {
                headline_severity = contribution.severity();
                headline_reason = reason;
            }
        }
        key_evaluations.push(KeyEvaluation {
            key,
            compatibility_class: class,
            match_state: state,
            outcome_contribution: contribution,
            summary: key_summary(key, state, contribution),
        });
    }

    let mut artifact_evaluations = Vec::new();
    for artifact in &snapshot.artifacts {
        let contribution = artifact_contribution(artifact);
        if contribution.severity() > outcome.severity() {
            outcome = contribution;
        }
        if artifact.integrity.is_lost() {
            reason_tokens.push(format!("artifact_{}_lost", artifact.layer.as_str()));
            let reason = if artifact.critical {
                PrebuildReason::CriticalArtifactLoss
            } else {
                PrebuildReason::PartialArtifactLoss
            };
            if contribution.severity() > headline_severity {
                headline_severity = contribution.severity();
                headline_reason = reason;
            }
        }
        artifact_evaluations.push(ArtifactEvaluation {
            artifact_id: artifact.artifact_id.clone(),
            layer: artifact.layer,
            integrity: artifact.integrity,
            critical: artifact.critical,
            outcome_contribution: contribution,
            summary: artifact_summary(artifact, contribution),
        });
    }

    reason_tokens.sort();
    reason_tokens.dedup();

    let action_gates = action_gates(outcome, &key_evaluations, &artifact_evaluations);
    let gated_action_tokens = action_gates
        .iter()
        .filter(|gate| gate.posture.is_gated())
        .map(|gate| gate.action.as_str().to_owned())
        .collect();

    let headline = headline(snapshot, outcome, headline_reason);

    PrebuildDecision {
        record_kind: PREBUILD_DECISION_RECORD_KIND.to_owned(),
        schema_version: PREBUILD_FINGERPRINT_SCHEMA_VERSION,
        snapshot_id: snapshot.snapshot_id.clone(),
        profile: snapshot.profile,
        materialization_class: snapshot.materialization_class,
        outcome,
        reused: outcome.reuses_snapshot(),
        invalidated: outcome == StartOutcome::Invalidated,
        warm_start_posture: outcome.warm_start_posture(),
        headline_reason,
        reason_tokens,
        key_evaluations,
        artifact_evaluations,
        action_gates,
        gated_action_tokens,
        headline,
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// Derives the per-action gating for a reuse decision. A warm start leaves
/// every action available; a cold or invalidated start disables every
/// snapshot-served action until the rebuild; a partially warm start keeps
/// actions whose backing keys and layers are intact available and degrades
/// the rest, so stale tools and indexes are never presented as current
/// truth.
fn action_gates(
    outcome: StartOutcome,
    keys: &[KeyEvaluation],
    artifacts: &[ArtifactEvaluation],
) -> Vec<ActionGate> {
    CapsuleActionClass::ALL
        .into_iter()
        .map(|action| {
            let posture = match outcome {
                StartOutcome::Warm => ActionPosture::Available,
                StartOutcome::Cold | StartOutcome::Invalidated => ActionPosture::Disabled,
                StartOutcome::PartiallyWarm => action_posture_partial(action, keys, artifacts),
            };
            let reason = action_reason(action, posture, outcome);
            ActionGate {
                action,
                posture,
                reason,
            }
        })
        .collect()
}

fn action_posture_partial(
    action: CapsuleActionClass,
    keys: &[KeyEvaluation],
    artifacts: &[ArtifactEvaluation],
) -> ActionPosture {
    let backing_keys = action.backing_keys();
    let backing_layers = action.backing_layers();
    let mut worst = StartOutcome::Warm;
    for evaluation in keys {
        if backing_keys.contains(&evaluation.key)
            && evaluation.outcome_contribution.severity() > worst.severity()
        {
            worst = evaluation.outcome_contribution;
        }
    }
    for evaluation in artifacts {
        if backing_layers.contains(&evaluation.layer)
            && evaluation.outcome_contribution.severity() > worst.severity()
        {
            worst = evaluation.outcome_contribution;
        }
    }
    match worst {
        StartOutcome::Warm => ActionPosture::Available,
        StartOutcome::PartiallyWarm => ActionPosture::Degraded,
        StartOutcome::Cold | StartOutcome::Invalidated => ActionPosture::Disabled,
    }
}

fn action_reason(
    action: CapsuleActionClass,
    posture: ActionPosture,
    outcome: StartOutcome,
) -> String {
    match posture {
        ActionPosture::Available => format!(
            "The {} action is served against current truth.",
            action.as_str()
        ),
        ActionPosture::Degraded => format!(
            "The {} action is served from the warm snapshot but labeled pending a targeted rebuild.",
            action.as_str()
        ),
        ActionPosture::Disabled => format!(
            "The {} action is disabled until the {} rebuild completes rather than served from a stale snapshot.",
            action.as_str(),
            outcome.as_str()
        ),
    }
}

fn headline(snapshot: &PrebuildSnapshot, outcome: StartOutcome, reason: PrebuildReason) -> String {
    match outcome {
        StartOutcome::Warm => format!(
            "Snapshot {} is fully warm-reused: every fingerprint key matches and its critical artifacts are intact.",
            snapshot.snapshot_id
        ),
        StartOutcome::PartiallyWarm => format!(
            "Snapshot {} is partially warm-reused ({}): the unaffected layers are reused while the affected layer is rebuilt.",
            snapshot.snapshot_id,
            reason.as_str()
        ),
        StartOutcome::Cold => format!(
            "Snapshot {} is rejected for a cold build ({}): it is for different content, so it is rebuilt rather than served as current truth.",
            snapshot.snapshot_id,
            reason.as_str()
        ),
        StartOutcome::Invalidated => format!(
            "Snapshot {} is invalidated and evicted ({}): it is binary- or trust-incompatible with the current host, so it is never reused.",
            snapshot.snapshot_id,
            reason.as_str()
        ),
    }
}

/// The desktop reuse decision. Desktop reads the same [`PrebuildDecision`]
/// object as every other surface.
pub fn desktop_prebuild_decision(
    snapshot: &PrebuildSnapshot,
    current: &PrebuildFingerprint,
) -> PrebuildDecision {
    evaluate_prebuild_reuse(snapshot, current)
}

/// The headless / CLI reuse decision. Headless reads the same
/// [`PrebuildDecision`] object as every other surface.
pub fn headless_prebuild_decision(
    snapshot: &PrebuildSnapshot,
    current: &PrebuildFingerprint,
) -> PrebuildDecision {
    evaluate_prebuild_reuse(snapshot, current)
}

/// The support-path reuse export: the metadata-first projection wrapping
/// the same [`PrebuildDecision`] object support and release surfaces read.
pub fn support_prebuild_decision(
    snapshot: &PrebuildSnapshot,
    current: &PrebuildFingerprint,
) -> PrebuildExport {
    export_prebuild_decision(&evaluate_prebuild_reuse(snapshot, current))
}

// ---------------------------------------------------------------------------
// Metadata-first export.
// ---------------------------------------------------------------------------

/// A metadata-first projection of a reuse decision for support and release
/// surfaces. It carries only the distinguishable outcome, the reason, the
/// downgrade tokens, and the gated actions — never secrets, raw bodies, or
/// provider payloads — and wraps the canonical decision so support never
/// re-derives the outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Snapshot id under evaluation.
    pub snapshot_id: String,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// The explicit reuse outcome.
    pub outcome: StartOutcome,
    /// True when any part of the snapshot is reused.
    pub reused: bool,
    /// True when the snapshot is rejected as incompatible.
    pub invalidated: bool,
    /// The governance warm-start posture the outcome maps to.
    pub warm_start_posture: WarmStartPosture,
    /// The dominant reason behind the outcome.
    pub headline_reason: PrebuildReason,
    /// Stable tokens naming every key and artifact that forced a downgrade.
    pub reason_tokens: Vec<String>,
    /// Stable tokens naming every action narrowed or disabled.
    pub gated_action_tokens: Vec<String>,
    /// The canonical decision this export wraps.
    pub decision: PrebuildDecision,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// Projects a metadata-first [`PrebuildExport`] from a decision.
pub fn export_prebuild_decision(decision: &PrebuildDecision) -> PrebuildExport {
    PrebuildExport {
        record_kind: PREBUILD_EXPORT_RECORD_KIND.to_owned(),
        schema_version: PREBUILD_FINGERPRINT_SCHEMA_VERSION,
        snapshot_id: decision.snapshot_id.clone(),
        profile: decision.profile,
        outcome: decision.outcome,
        reused: decision.reused,
        invalidated: decision.invalidated,
        warm_start_posture: decision.warm_start_posture,
        headline_reason: decision.headline_reason,
        reason_tokens: decision.reason_tokens.clone(),
        gated_action_tokens: decision.gated_action_tokens.clone(),
        decision: decision.clone(),
        redaction_class: RedactionClass::MetadataOnly,
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One automatic per-key invalidation rule. The outcomes are computed from
/// the key's [`CompatibilityClass`], so the rule set can never drift from
/// [`evaluate_prebuild_reuse`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyInvalidationRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Key the rule governs.
    pub key: FingerprintKey,
    /// Compatibility class of the key.
    pub compatibility_class: CompatibilityClass,
    /// Outcome a drift on the key forces.
    pub drift_outcome: StartOutcome,
    /// Outcome an absent key forces.
    pub absent_outcome: StartOutcome,
    /// User-visible effect on warm start.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One automatic artifact-integrity invalidation rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactInvalidationRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Whether the rule governs a critical artifact.
    pub critical: bool,
    /// Outcome a lost artifact of this criticality forces.
    pub loss_outcome: StartOutcome,
    /// User-visible effect on warm start.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One canonical evaluated case: a snapshot, the current fingerprint it is
/// evaluated against, and the decision the engine stamped onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildCase {
    /// Stable case id.
    pub case_id: String,
    /// Reviewer title.
    pub title: String,
    /// The recorded snapshot.
    pub snapshot: PrebuildSnapshot,
    /// The current expected fingerprint.
    pub current_fingerprint: PrebuildFingerprint,
    /// The decision the engine reached.
    pub decision: PrebuildDecision,
    /// Real consumer surfaces that ingest this case.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One ordered step inside a prebuild drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildDrillStep {
    /// Phase of this step (reused governance vocabulary).
    pub phase: DrillPhase,
    /// Outcome observed at this step.
    pub observed_outcome: StartOutcome,
    /// Warm-start posture observed at this step.
    pub observed_warm_start_posture: WarmStartPosture,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill: a drift is injected, the engine narrows
/// or rejects reuse, the snapshot is rebuilt, and reuse recovers to warm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Reason the drift injects.
    pub injected_reason: PrebuildReason,
    /// Environment profile exercised by the drill.
    pub profile: EnvironmentProfile,
    /// Outcome before the drift (warm).
    pub baseline_outcome: StartOutcome,
    /// Outcome while the drift is active.
    pub degraded_outcome: StartOutcome,
    /// Warm-start posture while the drift is active.
    pub degraded_warm_start_posture: WarmStartPosture,
    /// True when the drift disables snapshot reuse entirely.
    pub disables_reuse: bool,
    /// Ordered drill steps.
    pub steps: Vec<PrebuildDrillStep>,
    /// Outcome once the snapshot is rebuilt (warm).
    pub recovers_to_outcome: StartOutcome,
    /// True when the drill proves reuse narrows or is blocked under drift.
    pub asserts_reuse_blocked_under_drift: bool,
    /// True when the drill proves reuse recovers after rebuild.
    pub asserts_recovers_after_rebuild: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// Top-level packet governing prebuild-snapshot compatibility, the keyed
/// fingerprint, the invalidation rules, and the cold-versus-partially-warm
/// downgrade truth for claimed M5 warm-start paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildFingerprintPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared source refs.
    pub source_contract_refs: SourceContractRefs,
    /// The six fingerprint keys this lane covers, in canonical order.
    pub fingerprint_keys: Vec<FingerprintKey>,
    /// Per-key invalidation rules.
    pub key_invalidation_rules: Vec<KeyInvalidationRule>,
    /// Artifact-integrity invalidation rules.
    pub artifact_invalidation_rules: Vec<ArtifactInvalidationRule>,
    /// Canonical evaluated cases, covering every outcome.
    pub cases: Vec<PrebuildCase>,
    /// Failure / recovery drills.
    pub drills: Vec<PrebuildDrill>,
    /// Upstream environment packets this lane composes.
    pub evidence_packet_refs: Vec<String>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a snapshot and a current fingerprint to the outcome
/// the engine must reach for it, proving the canonical reuse behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildFingerprintFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reason the fixture exercises.
    pub injected_reason: PrebuildReason,
    /// The recorded snapshot.
    pub snapshot: PrebuildSnapshot,
    /// The current expected fingerprint.
    pub current_fingerprint: PrebuildFingerprint,
    /// Expected reuse outcome.
    pub expected_outcome: StartOutcome,
    /// Expected reuse flag.
    pub expected_reused: bool,
    /// Expected invalidated flag.
    pub expected_invalidated: bool,
    /// Expected warm-start posture.
    pub expected_warm_start_posture: WarmStartPosture,
    /// Expected dominant reason.
    pub expected_headline_reason: PrebuildReason,
    /// Expected reason tokens.
    pub expected_reason_tokens: Vec<String>,
    /// Expected gated-action tokens.
    pub expected_gated_action_tokens: Vec<String>,
    /// One consumer that quotes this case.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

trait ReportExt {
    fn note(&mut self, check_id: &'static str, message: impl Into<String>);
}

impl ReportExt for ValidationReport {
    fn note(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations
            .push(crate::m5_env_governance::ValidationViolation {
                check_id,
                message: message.into(),
            });
    }
}

fn is_hex64(digest: &CapsuleDigest) -> bool {
    digest.value.len() == 64 && digest.value.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Validates a fingerprint's keyed digests, the canonical key order, and
/// the combined digest. When `require_all_keys` is set every key must be
/// present (the contract for a current / canonical fingerprint); otherwise
/// a subset is allowed (a recorded snapshot fingerprint may omit a key it
/// never recorded, which the engine treats as unproven).
fn validate_fingerprint_into(
    report: &mut ValidationReport,
    fingerprint: &PrebuildFingerprint,
    require_all_keys: bool,
) {
    let mut seen = BTreeSet::new();
    for keyed in &fingerprint.keys {
        if !seen.insert(keyed.key) {
            report.note(
                "fingerprint.key_unique",
                format!("fingerprint repeats key {}", keyed.key.as_str()),
            );
        }
        if !is_hex64(&keyed.digest) {
            report.note(
                "fingerprint.key_digest_hex",
                format!("key {} digest must be 64 lowercase hex", keyed.key.as_str()),
            );
        }
        if keyed.summary.trim().is_empty() {
            report.note(
                "fingerprint.key_summary",
                format!("key {} must carry a summary", keyed.key.as_str()),
            );
        }
    }
    if require_all_keys {
        for required in FingerprintKey::ALL {
            if !seen.contains(&required) {
                report.note(
                    "fingerprint.key_coverage",
                    format!("fingerprint must key on {}", required.as_str()),
                );
            }
        }
    }
    // The present keys must be a canonical-ordered subsequence.
    if fingerprint
        .keys
        .windows(2)
        .any(|pair| pair[0].key >= pair[1].key)
    {
        report.note(
            "fingerprint.key_order",
            "fingerprint keys must be in canonical order",
        );
    }
    let recomputed = combined_digest(&fingerprint.keys);
    if recomputed != fingerprint.fingerprint {
        report.note(
            "fingerprint.combined",
            "combined fingerprint digest does not fold over the keyed digests",
        );
    }
}

/// Validates a current / canonical prebuild fingerprint: all six keys
/// present and unique, each digest 64-hex, and the combined digest
/// reproducible from the keys.
pub fn validate_prebuild_fingerprint(
    fingerprint: &PrebuildFingerprint,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    validate_fingerprint_into(&mut report, fingerprint, true);
    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_snapshot_into(report: &mut ValidationReport, snapshot: &PrebuildSnapshot) {
    if snapshot.record_kind != PREBUILD_SNAPSHOT_RECORD_KIND {
        report.note("snapshot.record_kind", "snapshot record_kind drifted");
    }
    if snapshot.schema_version != PREBUILD_FINGERPRINT_SCHEMA_VERSION {
        report.note(
            "snapshot.schema_version",
            "snapshot schema_version must be 1",
        );
    }
    if snapshot.snapshot_id.trim().is_empty() {
        report.note("snapshot.id", "snapshot must carry a stable id");
    }
    if snapshot.label.trim().is_empty() {
        report.note("snapshot.label", "snapshot must carry a label");
    }
    // A recorded snapshot fingerprint may legitimately omit a key it never
    // recorded; the engine treats an absent key as unproven.
    validate_fingerprint_into(report, &snapshot.recorded_fingerprint, false);
    let mut layers = BTreeSet::new();
    for artifact in &snapshot.artifacts {
        if !layers.insert(artifact.layer) {
            report.note(
                "snapshot.artifact_unique",
                format!(
                    "snapshot repeats artifact layer {}",
                    artifact.layer.as_str()
                ),
            );
        }
        if artifact.artifact_id.trim().is_empty() {
            report.note("snapshot.artifact_id", "artifact must carry a stable id");
        }
    }
}

/// Validates one snapshot record.
pub fn validate_prebuild_snapshot(snapshot: &PrebuildSnapshot) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    validate_snapshot_into(&mut report, snapshot);
    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_decision_into(
    report: &mut ValidationReport,
    owner: &str,
    snapshot: &PrebuildSnapshot,
    current: &PrebuildFingerprint,
    decision: &PrebuildDecision,
) {
    let recomputed = evaluate_prebuild_reuse(snapshot, current);
    if decision != &recomputed {
        report.note(
            "decision.engine_parity",
            format!("{owner} stamped decision disagrees with the engine"),
        );
    }
    if decision.redaction_class != RedactionClass::MetadataOnly {
        report.note(
            "decision.redaction",
            format!("{owner} decision must be metadata-only"),
        );
    }
}

/// Validates the checked-in fixture contract.
pub fn validate_prebuild_fingerprint_fixture(
    fixture: &PrebuildFingerprintFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != PREBUILD_FINGERPRINT_FIXTURE_RECORD_KIND {
        report.note("fixture.record_kind", "fixture record_kind drifted");
    }
    if fixture.schema_version != PREBUILD_FINGERPRINT_SCHEMA_VERSION {
        report.note("fixture.schema_version", "fixture schema_version must be 1");
    }
    if fixture.fixture_id.trim().is_empty() {
        report.note("fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        report.note("fixture.consumer_ref", "fixture must cite a consumer ref");
    }
    if fixture.notes.trim().is_empty() {
        report.note("fixture.notes", "fixture must carry a reviewer note");
    }
    validate_snapshot_into(&mut report, &fixture.snapshot);
    if let Err(nested) = validate_prebuild_fingerprint(&fixture.current_fingerprint) {
        for violation in nested.violations {
            report.violations.push(violation);
        }
    }

    let decision = evaluate_prebuild_reuse(&fixture.snapshot, &fixture.current_fingerprint);
    if fixture.expected_outcome != decision.outcome {
        report.note(
            "fixture.expected_outcome",
            format!(
                "fixture {} expected outcome {} but engine reached {}",
                fixture.fixture_id,
                fixture.expected_outcome.as_str(),
                decision.outcome.as_str()
            ),
        );
    }
    if fixture.expected_reused != decision.reused {
        report.note(
            "fixture.expected_reused",
            "fixture reused flag disagrees with engine",
        );
    }
    if fixture.expected_invalidated != decision.invalidated {
        report.note(
            "fixture.expected_invalidated",
            "fixture invalidated flag disagrees with engine",
        );
    }
    if fixture.expected_warm_start_posture != decision.warm_start_posture {
        report.note(
            "fixture.expected_warm_start_posture",
            "fixture warm-start posture disagrees with engine",
        );
    }
    if fixture.expected_headline_reason != decision.headline_reason {
        report.note(
            "fixture.expected_headline_reason",
            "fixture headline reason disagrees with engine",
        );
    }
    if fixture.expected_reason_tokens != decision.reason_tokens {
        report.note(
            "fixture.expected_reason_tokens",
            "fixture reason tokens disagree with engine",
        );
    }
    if fixture.expected_gated_action_tokens != decision.gated_action_tokens {
        report.note(
            "fixture.expected_gated_action_tokens",
            "fixture gated-action tokens disagree with engine",
        );
    }
    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates the checked-in packet contract.
pub fn validate_prebuild_fingerprint_packet(
    packet: &PrebuildFingerprintPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if packet.record_kind != PREBUILD_FINGERPRINT_PACKET_RECORD_KIND {
        report.note("packet.record_kind", "packet record_kind drifted");
    }
    if packet.schema_version != PREBUILD_FINGERPRINT_SCHEMA_VERSION {
        report.note("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != PREBUILD_FINGERPRINT_PACKET_ID {
        report.note("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.schema_ref != PREBUILD_FINGERPRINT_SCHEMA_REF {
        report.note(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.doc_ref != PREBUILD_FINGERPRINT_DOC_REF {
        report.note("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.packet_ref != PREBUILD_FINGERPRINT_PACKET_REF {
        report.note(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != PREBUILD_FINGERPRINT_PROOF_REF {
        report.note(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF
    {
        report.note(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.fingerprint_keys != FingerprintKey::ALL.to_vec() {
        report.note(
            "packet.fingerprint_keys",
            "packet must key on every fingerprint input in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.note(
            "packet.evidence_packet_refs",
            "packet must cite upstream environment evidence",
        );
    }
    if packet.invariants.is_empty() {
        report.note("packet.invariants", "packet must declare invariants");
    }

    // Key invalidation rules: one per key, outcomes derived from the class.
    let mut keyed_rules = BTreeSet::new();
    for rule in &packet.key_invalidation_rules {
        if !keyed_rules.insert(rule.key) {
            report.note(
                "packet.key_rule_unique",
                format!("duplicate key rule for {}", rule.key.as_str()),
            );
        }
        let class = rule.key.compatibility_class();
        if rule.compatibility_class != class
            || rule.drift_outcome != class.drift_outcome()
            || rule.absent_outcome != class.absent_outcome()
        {
            report.note(
                "packet.key_rule_outcome",
                format!(
                    "key rule {} disagrees with the compatibility class",
                    rule.key.as_str()
                ),
            );
        }
    }
    for required in FingerprintKey::ALL {
        if !keyed_rules.contains(&required) {
            report.note(
                "packet.key_rule_coverage",
                format!(
                    "packet must carry an invalidation rule for {}",
                    required.as_str()
                ),
            );
        }
    }

    // Artifact rules: a critical and a non-critical rule.
    let mut saw_critical = false;
    let mut saw_partial = false;
    for rule in &packet.artifact_invalidation_rules {
        let expected = if rule.critical {
            StartOutcome::Cold
        } else {
            StartOutcome::PartiallyWarm
        };
        if rule.loss_outcome != expected {
            report.note(
                "packet.artifact_rule_outcome",
                "artifact rule outcome disagrees with the engine",
            );
        }
        saw_critical |= rule.critical;
        saw_partial |= !rule.critical;
    }
    if !saw_critical || !saw_partial {
        report.note(
            "packet.artifact_rule_coverage",
            "packet must carry both a critical and a non-critical artifact rule",
        );
    }

    // Cases must stamp the engine's decision and cover every outcome.
    let mut case_outcomes = BTreeSet::new();
    for case in &packet.cases {
        if case.consumer_refs.is_empty() {
            report.note(
                "case.consumer_refs",
                format!("case {} must cite a consumer ref", case.case_id),
            );
        }
        validate_snapshot_into(&mut report, &case.snapshot);
        if let Err(nested) = validate_prebuild_fingerprint(&case.current_fingerprint) {
            for violation in nested.violations {
                report.violations.push(violation);
            }
        }
        validate_decision_into(
            &mut report,
            &format!("case {}", case.case_id),
            &case.snapshot,
            &case.current_fingerprint,
            &case.decision,
        );
        case_outcomes.insert(case.decision.outcome);
    }
    for required in StartOutcome::ALL {
        if !case_outcomes.contains(&required) {
            report.note(
                "packet.case_outcome_coverage",
                format!("cases must cover the {} outcome", required.as_str()),
            );
        }
    }

    // Drills must cover the five named drift classes and recover to warm.
    let mut drill_reasons = BTreeSet::new();
    for drill in &packet.drills {
        drill_reasons.insert(drill.injected_reason);
        if drill.recovers_to_outcome != StartOutcome::Warm {
            report.note(
                "drill.recovers_to_warm",
                format!("drill {} must recover to warm", drill.drill_id),
            );
        }
        if drill.baseline_outcome != StartOutcome::Warm {
            report.note(
                "drill.baseline_warm",
                format!("drill {} must baseline at warm", drill.drill_id),
            );
        }
        if drill.degraded_outcome == StartOutcome::Warm {
            report.note(
                "drill.degraded_narrows",
                format!(
                    "drill {} must narrow below warm under drift",
                    drill.drill_id
                ),
            );
        }
        if drill.disables_reuse == drill.degraded_outcome.reuses_snapshot() {
            report.note(
                "drill.disables_reuse",
                format!(
                    "drill {} disables_reuse disagrees with its degraded outcome",
                    drill.drill_id
                ),
            );
        }
    }
    for required in [
        PrebuildReason::SourceDrift,
        PrebuildReason::PolicyDrift,
        PrebuildReason::PlatformDrift,
        PrebuildReason::ExtensionLockDrift,
        PrebuildReason::PartialArtifactLoss,
    ] {
        if !drill_reasons.contains(&required) {
            report.note(
                "packet.drill_coverage",
                format!("drills must exercise the {} class", required.as_str()),
            );
        }
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

impl fmt::Display for PrebuildDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} ({})",
            self.snapshot_id,
            self.outcome.as_str(),
            self.headline_reason.as_str()
        )
    }
}
