//! Canonical M5 mirror, private-registry, and side-load review packets plus
//! enterprise policy filtering — parity for sovereign and offline install lanes.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per marketed M5 artifact family and
//! [`m5_marketplace_fact_views`](crate::m5_marketplace_fact_views) projects that truth
//! into the storefront, this module proves that the *install review* a user sees does
//! not get thinner when an artifact arrives through an enterprise mirror, a private
//! registry, a manual import, or an air-gapped transfer. A [`MirrorReviewPacket`]
//! carries the same package-identity, compatibility, permission, runtime-origin,
//! lifecycle, activation, and rollback fields as a public-registry install review —
//! no reduced or special-case disclosure — and adds the continuity facts these lanes
//! make visible: publisher transfer, signing-root continuity, namespace continuity,
//! maintenance/orphan state, mirror freshness, and provenance reduction.
//!
//! The packet is honest by construction. The [`ReviewDisposition`] a packet publishes
//! is **not** stored by hand: it is recomputed from the packet's facts as the widest
//! [`ContinuitySignal::min_disposition`] over every detected [`ContinuitySignal`], and
//! the stored disposition and signal set must equal that recomputation or validation
//! fails. The lane guardrails ride that recomputation: an unreviewed permission
//! expansion, an unsupported compatibility target, an incomplete rollback, a
//! quarantine, a changed signing root, a discontinuous namespace, an unmaintained
//! package, an unverified publisher transfer, or unverifiable provenance each force
//! [`ReviewDisposition::Blocked`], so a mirrored or side-loaded artifact can never
//! bypass the checks a first-party install runs.
//!
//! Every review packet also carries the full set of backing refs — provenance,
//! permission manifest, compatibility, activation budget, rollback, publisher
//! continuity, and support export — so a manual-import or air-gapped review produces
//! an export-safe packet equivalent to a public-registry install for support and
//! audit.
//!
//! On top of the review packets, this module models **enterprise policy filtering**.
//! A [`PolicyFilter`] gates the new M5 artifact families by a stable
//! [`PolicyDimension`] — publisher, signing root, runtime origin, capability class,
//! network class, support class, bridge state, or activation-budget band — with an
//! [`PolicyEffect`] of allow, require-approval, or block. A [`PolicyEvaluation`]
//! applies the filter set to a review packet and recomputes a [`PolicyGateDecision`]
//! as the stronger of the matched filter effect and the packet's own review
//! disposition, so policy can tighten admission but never loosen it below the
//! guardrail the review already requires.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-mirror-and-sideload.json`
//! and embedded here, so this typed consumer and any CI gate agree on every record
//! without a cargo build in CI. The model is metadata-only: every field is a typed
//! state or an opaque ref. It carries no credential bodies, raw provider payloads,
//! signing secrets, or mirror tokens.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ActivationBudgetBand, ArtifactFamily, CompatibilityLabel, EvidenceFreshness, LifecycleState,
    PermissionManifestState, RollbackPosture, RuntimeOrigin, SupportClass,
};
use crate::m5_activation_budget::CapabilityClass;
use crate::m5_install_review::{NamespaceState, PublisherTransferState, SigningRootContinuity};
use crate::m5_marketplace_fact_views::{BridgeNativeState, MirrorPosture, SourceClass};

/// Supported M5 mirror-and-side-load schema version.
pub const M5_MIRROR_AND_SIDELOAD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_MIRROR_AND_SIDELOAD_RECORD_KIND: &str = "m5_mirror_and_sideload";

/// Repo-relative path to the checked-in packet.
pub const M5_MIRROR_AND_SIDELOAD_PATH: &str = "artifacts/ecosystem/m5/m5-mirror-and-sideload.json";

/// Embedded checked-in packet JSON.
pub const M5_MIRROR_AND_SIDELOAD_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-mirror-and-sideload.json"
));

/// How an artifact reached the install review.
///
/// The same review surface holds across every channel; the channel is recorded so
/// support and audit can prove an offline or manual-import review reasoned about the
/// same facts as a public-registry install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionChannel {
    /// The public registry storefront.
    PublicRegistry,
    /// An enterprise mirror of the public registry.
    EnterpriseMirror,
    /// A private registry.
    PrivateRegistry,
    /// A manual import of a downloaded artifact.
    ManualImport,
    /// An air-gapped transfer with no live registry reachability.
    AirGappedImport,
}

impl AcquisitionChannel {
    /// Every acquisition channel, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicRegistry,
        Self::EnterpriseMirror,
        Self::PrivateRegistry,
        Self::ManualImport,
        Self::AirGappedImport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::PrivateRegistry => "private_registry",
            Self::ManualImport => "manual_import",
            Self::AirGappedImport => "air_gapped_import",
        }
    }

    /// Whether this channel is a side-loaded manual or air-gapped import.
    pub const fn is_side_loaded(self) -> bool {
        matches!(self, Self::ManualImport | Self::AirGappedImport)
    }
}

/// Maintenance posture of the publisher behind a listing.
///
/// Distinct from [`LifecycleState`] (what the local install is doing) and from the
/// registry status: this names whether the upstream is still actively maintained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceState {
    /// The package is actively maintained.
    ActivelyMaintained,
    /// Maintenance is reduced but ongoing.
    MaintenanceReduced,
    /// The package is no longer maintained.
    Unmaintained,
    /// The package has no maintainer (orphaned).
    Orphaned,
}

impl MaintenanceState {
    /// Every maintenance state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ActivelyMaintained,
        Self::MaintenanceReduced,
        Self::Unmaintained,
        Self::Orphaned,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivelyMaintained => "actively_maintained",
            Self::MaintenanceReduced => "maintenance_reduced",
            Self::Unmaintained => "unmaintained",
            Self::Orphaned => "orphaned",
        }
    }

    /// Whether this state is an unmaintained or orphaned package.
    pub const fn is_unmaintained_trigger(self) -> bool {
        matches!(self, Self::Unmaintained | Self::Orphaned)
    }
}

/// Freshness of a mirror or private-registry snapshot relative to upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorFreshness {
    /// Served live from the upstream registry (no mirror in the path).
    DirectUpstream,
    /// The mirror is current with upstream.
    MirrorCurrent,
    /// The mirror lags upstream within tolerance.
    MirrorLagging,
    /// The mirror is stale beyond tolerance.
    MirrorStale,
    /// Mirror freshness cannot be established (for example, air-gapped).
    MirrorUnknown,
}

impl MirrorFreshness {
    /// Every mirror-freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DirectUpstream,
        Self::MirrorCurrent,
        Self::MirrorLagging,
        Self::MirrorStale,
        Self::MirrorUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectUpstream => "direct_upstream",
            Self::MirrorCurrent => "mirror_current",
            Self::MirrorLagging => "mirror_lagging",
            Self::MirrorStale => "mirror_stale",
            Self::MirrorUnknown => "mirror_unknown",
        }
    }

    /// Whether mirror freshness is reduced below a current snapshot.
    pub const fn is_reduced(self) -> bool {
        matches!(
            self,
            Self::MirrorLagging | Self::MirrorStale | Self::MirrorUnknown
        )
    }
}

/// How much provenance evidence travels with the artifact.
///
/// Mirror and side-load lanes can strip attestation even when a signature survives;
/// this names that reduction so it is visible before install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceLevel {
    /// Full provenance: signature and build attestation present.
    FullAttested,
    /// Signed, but the build attestation did not travel with the artifact.
    SignedNoAttestation,
    /// Only a content checksum is available.
    ChecksumOnly,
    /// Provenance cannot be established.
    Unverifiable,
}

impl ProvenanceLevel {
    /// Every provenance level, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullAttested,
        Self::SignedNoAttestation,
        Self::ChecksumOnly,
        Self::Unverifiable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullAttested => "full_attested",
            Self::SignedNoAttestation => "signed_no_attestation",
            Self::ChecksumOnly => "checksum_only",
            Self::Unverifiable => "unverifiable",
        }
    }

    /// Whether provenance is reduced but still partially established.
    pub const fn is_reduced(self) -> bool {
        matches!(self, Self::SignedNoAttestation | Self::ChecksumOnly)
    }

    /// Whether provenance cannot be established at all.
    pub const fn is_unverifiable(self) -> bool {
        matches!(self, Self::Unverifiable)
    }
}

/// Network reach a package requires at runtime.
///
/// A policy-relevant axis distinct from runtime origin: it gates what an enterprise
/// or sovereign deployment will admit regardless of who signed the package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkClass {
    /// Operates fully offline.
    Offline,
    /// Reaches only the local network.
    LocalOnly,
    /// Reaches a scoped, declared set of endpoints.
    ScopedNetwork,
    /// Reaches the open network.
    OpenNetwork,
}

impl NetworkClass {
    /// Every network class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Offline,
        Self::LocalOnly,
        Self::ScopedNetwork,
        Self::OpenNetwork,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::LocalOnly => "local_only",
            Self::ScopedNetwork => "scoped_network",
            Self::OpenNetwork => "open_network",
        }
    }
}

/// A continuity or trust signal a review packet surfaces before install or update.
///
/// Each signal is recomputed from the packet's facts; the packet's stored
/// [`MirrorReviewPacket::continuity_signals`] must equal the recomputed set. Each
/// signal carries a fixed [`ContinuitySignal::min_disposition`], so the published
/// [`ReviewDisposition`] is a pure function of which signals fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuitySignal {
    /// The qualifying evidence is not current.
    EvidenceNotCurrent,
    /// The serving mirror or registry snapshot is not current.
    MirrorStale,
    /// Ownership transferred to a verified publisher.
    PublisherTransferredVerified,
    /// Maintenance is reduced but ongoing.
    MaintenanceReduced,
    /// Provenance is reduced but still partially established.
    ProvenanceReduced,
    /// The runtime is bridge-backed or local-model-hosted rather than native.
    NonNativeRuntime,
    /// The published support class is below full support.
    SupportNarrowed,
    /// The activation budget is over its ceiling.
    ActivationBudgetExceeded,
    /// Ownership transferred to an unverified or unknown publisher.
    PublisherDiscontinuous,
    /// The signing root changed to an unrelated root.
    SigningRootChanged,
    /// The namespace is orphaned or reclaimed by a different party.
    NamespaceDiscontinuous,
    /// The package is unmaintained or orphaned.
    Unmaintained,
    /// Provenance cannot be established at all.
    ProvenanceUnverifiable,
    /// Permissions were expanded without review.
    PermissionExpansionUnreviewed,
    /// The package is unsupported on the install target.
    CompatibilityUnsupported,
    /// Rollback is unverified or irreversible.
    RollbackIncomplete,
    /// The package is quarantined.
    Quarantined,
}

impl ContinuitySignal {
    /// Every continuity signal, in declaration order.
    pub const ALL: [Self; 17] = [
        Self::EvidenceNotCurrent,
        Self::MirrorStale,
        Self::PublisherTransferredVerified,
        Self::MaintenanceReduced,
        Self::ProvenanceReduced,
        Self::NonNativeRuntime,
        Self::SupportNarrowed,
        Self::ActivationBudgetExceeded,
        Self::PublisherDiscontinuous,
        Self::SigningRootChanged,
        Self::NamespaceDiscontinuous,
        Self::Unmaintained,
        Self::ProvenanceUnverifiable,
        Self::PermissionExpansionUnreviewed,
        Self::CompatibilityUnsupported,
        Self::RollbackIncomplete,
        Self::Quarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceNotCurrent => "evidence_not_current",
            Self::MirrorStale => "mirror_stale",
            Self::PublisherTransferredVerified => "publisher_transferred_verified",
            Self::MaintenanceReduced => "maintenance_reduced",
            Self::ProvenanceReduced => "provenance_reduced",
            Self::NonNativeRuntime => "non_native_runtime",
            Self::SupportNarrowed => "support_narrowed",
            Self::ActivationBudgetExceeded => "activation_budget_exceeded",
            Self::PublisherDiscontinuous => "publisher_discontinuous",
            Self::SigningRootChanged => "signing_root_changed",
            Self::NamespaceDiscontinuous => "namespace_discontinuous",
            Self::Unmaintained => "unmaintained",
            Self::ProvenanceUnverifiable => "provenance_unverifiable",
            Self::PermissionExpansionUnreviewed => "permission_expansion_unreviewed",
            Self::CompatibilityUnsupported => "compatibility_unsupported",
            Self::RollbackIncomplete => "rollback_incomplete",
            Self::Quarantined => "quarantined",
        }
    }

    /// The minimum review disposition this signal forces.
    ///
    /// The guardrail-class signals — permission expansion, unsupported
    /// compatibility, incomplete rollback, quarantine, signing-root change,
    /// discontinuous namespace, unmaintained package, unverified publisher
    /// transfer, and unverifiable provenance — force [`ReviewDisposition::Blocked`],
    /// so a mirror or side-load can never proceed past a check a first-party install
    /// would fail.
    pub const fn min_disposition(self) -> ReviewDisposition {
        match self {
            Self::EvidenceNotCurrent
            | Self::MirrorStale
            | Self::PublisherTransferredVerified
            | Self::MaintenanceReduced
            | Self::ProvenanceReduced
            | Self::NonNativeRuntime
            | Self::SupportNarrowed
            | Self::ActivationBudgetExceeded => ReviewDisposition::ReviewRequired,
            Self::PublisherDiscontinuous
            | Self::SigningRootChanged
            | Self::NamespaceDiscontinuous
            | Self::Unmaintained
            | Self::ProvenanceUnverifiable
            | Self::PermissionExpansionUnreviewed
            | Self::CompatibilityUnsupported
            | Self::RollbackIncomplete
            | Self::Quarantined => ReviewDisposition::Blocked,
        }
    }
}

/// The disposition a review packet publishes.
///
/// Ordered low-to-high by [`ReviewDisposition::rank`]: a [`ReviewDisposition::Proceed`]
/// packet may install directly, and a [`ReviewDisposition::Blocked`] packet must clear
/// a guardrail before it can proceed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisposition {
    /// No continuity signal applies; install may proceed.
    Proceed,
    /// At least one review-class signal applies; review is required.
    ReviewRequired,
    /// At least one guardrail-class signal applies; install is blocked.
    Blocked,
}

impl ReviewDisposition {
    /// Every review disposition, in declaration order.
    pub const ALL: [Self; 3] = [Self::Proceed, Self::ReviewRequired, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::ReviewRequired => "review_required",
            Self::Blocked => "blocked",
        }
    }

    /// Monotonic rank; higher means a stronger gate.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Proceed => 0,
            Self::ReviewRequired => 1,
            Self::Blocked => 2,
        }
    }

    /// The stronger (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    /// The gate decision this disposition maps to before policy is applied.
    pub const fn as_gate_decision(self) -> PolicyGateDecision {
        match self {
            Self::Proceed => PolicyGateDecision::Allowed,
            Self::ReviewRequired => PolicyGateDecision::ApprovalRequired,
            Self::Blocked => PolicyGateDecision::Blocked,
        }
    }
}

/// An export-safe install/update review packet for one acquisition.
///
/// The packet reproduces the full public-registry install-review fact set — package
/// identity, compatibility, permission, runtime origin, lifecycle, activation, and
/// rollback — and adds the continuity facts a mirror, private-registry, or side-load
/// lane makes visible. No field is dropped because the source is reduced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorReviewPacket {
    /// Stable review id.
    pub review_id: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this listing resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust origin.
    pub source_class: SourceClass,
    /// Channel the artifact was acquired through.
    pub acquisition_channel: AcquisitionChannel,
    /// Distribution posture of the served copy.
    pub mirror_posture: MirrorPosture,
    /// Opaque ref to the publisher identity.
    pub publisher_ref: String,
    /// Opaque ref to the signing root.
    pub signing_root_ref: String,
    /// Opaque ref to the namespace.
    pub namespace_ref: String,
    /// Lifecycle state of the local install.
    pub lifecycle_state: LifecycleState,
    /// Published support class.
    pub support_class: SupportClass,
    /// Evidence freshness.
    pub evidence_freshness: EvidenceFreshness,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Bridge/native state.
    pub bridge_native_state: BridgeNativeState,
    /// Network class the package requires.
    pub network_class: NetworkClass,
    /// Compatibility label against the install target.
    pub compatibility_label: CompatibilityLabel,
    /// Permission-manifest state relative to the prior install.
    pub permission_manifest_state: PermissionManifestState,
    /// Activation-budget band.
    pub activation_budget_band: ActivationBudgetBand,
    /// Rollback posture.
    pub rollback_posture: RollbackPosture,
    /// Publisher-transfer continuity.
    pub publisher_transfer_state: PublisherTransferState,
    /// Signing-root continuity.
    pub signing_root_continuity: SigningRootContinuity,
    /// Namespace continuity.
    pub namespace_state: NamespaceState,
    /// Upstream maintenance posture.
    pub maintenance_state: MaintenanceState,
    /// Freshness of the serving mirror snapshot.
    pub mirror_freshness: MirrorFreshness,
    /// Provenance evidence level travelling with the artifact.
    pub provenance_level: ProvenanceLevel,
    /// Capability classes the package declares.
    #[serde(default)]
    pub capability_classes: Vec<CapabilityClass>,
    /// Recomputed review disposition; must equal the recomputed value.
    pub review_disposition: ReviewDisposition,
    /// Recomputed continuity signals; must equal the recomputed set.
    #[serde(default)]
    pub continuity_signals: Vec<ContinuitySignal>,
    /// Ref to the provenance/signature record.
    pub provenance_ref: String,
    /// Ref to the permission manifest.
    pub permission_manifest_ref: String,
    /// Ref to the compatibility/downgrade story.
    pub compatibility_ref: String,
    /// Ref to the activation-budget record.
    pub activation_budget_ref: String,
    /// Ref to the durable rollback path.
    pub rollback_ref: String,
    /// Ref to the publisher-continuity record.
    pub publisher_continuity_ref: String,
    /// Ref binding this packet into support and release surfaces.
    pub support_export_ref: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl MirrorReviewPacket {
    /// The continuity signals recomputed from this packet's facts, in canonical
    /// order.
    pub fn computed_continuity_signals(&self) -> Vec<ContinuitySignal> {
        ContinuitySignal::ALL
            .into_iter()
            .filter(|signal| self.signal_detected(*signal))
            .collect()
    }

    fn signal_detected(&self, signal: ContinuitySignal) -> bool {
        match signal {
            ContinuitySignal::EvidenceNotCurrent => !self.evidence_freshness.is_current(),
            ContinuitySignal::MirrorStale => self.mirror_freshness.is_reduced(),
            ContinuitySignal::PublisherTransferredVerified => {
                self.publisher_transfer_state == PublisherTransferState::TransferredVerified
            }
            ContinuitySignal::MaintenanceReduced => {
                self.maintenance_state == MaintenanceState::MaintenanceReduced
            }
            ContinuitySignal::ProvenanceReduced => {
                self.provenance_level.is_reduced()
                    || self.source_class.is_reduced_provenance()
                    || self.acquisition_channel.is_side_loaded()
            }
            ContinuitySignal::NonNativeRuntime => self.bridge_native_state.is_non_native(),
            ContinuitySignal::SupportNarrowed => self.support_class != SupportClass::FullySupported,
            ContinuitySignal::ActivationBudgetExceeded => {
                self.activation_budget_band.is_exceeded_trigger()
            }
            ContinuitySignal::PublisherDiscontinuous => matches!(
                self.publisher_transfer_state,
                PublisherTransferState::TransferredUnverified
                    | PublisherTransferState::PublisherUnknown
            ),
            ContinuitySignal::SigningRootChanged => self.signing_root_continuity.is_discontinuity(),
            ContinuitySignal::NamespaceDiscontinuous => self.namespace_state.is_discontinuity(),
            ContinuitySignal::Unmaintained => self.maintenance_state.is_unmaintained_trigger(),
            ContinuitySignal::ProvenanceUnverifiable => {
                self.provenance_level.is_unverifiable()
                    || self.runtime_origin == RuntimeOrigin::UnsignedSideLoaded
                    || self.signing_root_continuity == SigningRootContinuity::Unsigned
            }
            ContinuitySignal::PermissionExpansionUnreviewed => self
                .permission_manifest_state
                .is_expansion_unreviewed_trigger(),
            ContinuitySignal::CompatibilityUnsupported => {
                self.compatibility_label.is_unsupported_trigger()
            }
            ContinuitySignal::RollbackIncomplete => self.rollback_posture.is_incomplete_trigger(),
            ContinuitySignal::Quarantined => self.lifecycle_state.is_quarantined_trigger(),
        }
    }

    /// The review disposition recomputed from this packet's facts.
    pub fn computed_review_disposition(&self) -> ReviewDisposition {
        self.computed_continuity_signals()
            .into_iter()
            .fold(ReviewDisposition::Proceed, |disposition, signal| {
                disposition.widen(signal.min_disposition())
            })
    }

    /// Whether the stored disposition and signals agree with the recomputed values.
    pub fn disposition_consistent(&self) -> bool {
        self.review_disposition == self.computed_review_disposition()
            && self.continuity_signals == self.computed_continuity_signals()
    }

    /// Whether the packet carries every backing ref a public-registry review carries.
    ///
    /// A mirror, private-registry, manual-import, or air-gapped review must carry the
    /// same evidence as a first-party install; a reduced source widens disclosure but
    /// must never drop a backing ref.
    pub fn is_export_safe(&self) -> bool {
        !self.provenance_ref.trim().is_empty()
            && !self.permission_manifest_ref.trim().is_empty()
            && !self.compatibility_ref.trim().is_empty()
            && !self.activation_budget_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
            && !self.publisher_continuity_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// The packet's value(s) for a policy dimension, as stable tokens.
    pub fn dimension_values(&self, dimension: PolicyDimension) -> Vec<String> {
        match dimension {
            PolicyDimension::Publisher => vec![self.publisher_ref.clone()],
            PolicyDimension::SigningRoot => vec![self.signing_root_ref.clone()],
            PolicyDimension::RuntimeOrigin => vec![self.runtime_origin.as_str().to_owned()],
            PolicyDimension::CapabilityClass => self
                .capability_classes
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            PolicyDimension::NetworkClass => vec![self.network_class.as_str().to_owned()],
            PolicyDimension::SupportClass => vec![self.support_class.as_str().to_owned()],
            PolicyDimension::BridgeState => vec![self.bridge_native_state.as_str().to_owned()],
            PolicyDimension::ActivationBudgetBand => {
                vec![self.activation_budget_band.as_str().to_owned()]
            }
        }
    }
}

/// A stable field an enterprise policy filter gates on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDimension {
    /// Match on the publisher identity.
    Publisher,
    /// Match on the signing root.
    SigningRoot,
    /// Match on the runtime origin.
    RuntimeOrigin,
    /// Match on a declared capability class.
    CapabilityClass,
    /// Match on the required network class.
    NetworkClass,
    /// Match on the published support class.
    SupportClass,
    /// Match on the bridge/native state.
    BridgeState,
    /// Match on the activation-budget band.
    ActivationBudgetBand,
}

impl PolicyDimension {
    /// Every policy dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Publisher,
        Self::SigningRoot,
        Self::RuntimeOrigin,
        Self::CapabilityClass,
        Self::NetworkClass,
        Self::SupportClass,
        Self::BridgeState,
        Self::ActivationBudgetBand,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publisher => "publisher",
            Self::SigningRoot => "signing_root",
            Self::RuntimeOrigin => "runtime_origin",
            Self::CapabilityClass => "capability_class",
            Self::NetworkClass => "network_class",
            Self::SupportClass => "support_class",
            Self::BridgeState => "bridge_state",
            Self::ActivationBudgetBand => "activation_budget_band",
        }
    }
}

/// The effect an enterprise policy filter applies to a matching listing.
///
/// Ordered low-to-high by [`PolicyEffect::rank`]; the strongest matching effect wins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEffect {
    /// Admit the listing.
    Allow,
    /// Admit only after explicit approval.
    RequireApproval,
    /// Block the listing.
    Block,
}

impl PolicyEffect {
    /// Every policy effect, in declaration order.
    pub const ALL: [Self; 3] = [Self::Allow, Self::RequireApproval, Self::Block];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::RequireApproval => "require_approval",
            Self::Block => "block",
        }
    }

    /// Monotonic rank; higher means a stronger restriction.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Allow => 0,
            Self::RequireApproval => 1,
            Self::Block => 2,
        }
    }

    /// The stronger (higher-rank) of two effects.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    /// The gate decision this effect maps to.
    pub const fn as_gate_decision(self) -> PolicyGateDecision {
        match self {
            Self::Allow => PolicyGateDecision::Allowed,
            Self::RequireApproval => PolicyGateDecision::ApprovalRequired,
            Self::Block => PolicyGateDecision::Blocked,
        }
    }
}

/// The recomputed admission decision for a listing under policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyGateDecision {
    /// Admitted with no further action.
    Allowed,
    /// Admitted only after explicit approval.
    ApprovalRequired,
    /// Blocked from admission.
    Blocked,
}

impl PolicyGateDecision {
    /// Every gate decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::Allowed, Self::ApprovalRequired, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::ApprovalRequired => "approval_required",
            Self::Blocked => "blocked",
        }
    }

    /// Monotonic rank; higher means a stronger gate.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Allowed => 0,
            Self::ApprovalRequired => 1,
            Self::Blocked => 2,
        }
    }

    /// The stronger (higher-rank) of two decisions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// One enterprise policy filter over the new M5 artifact families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyFilter {
    /// Stable filter id.
    pub filter_id: String,
    /// Human-readable filter label.
    pub display_label: String,
    /// Dimension this filter gates on.
    pub dimension: PolicyDimension,
    /// Stable token values this filter matches.
    pub match_values: Vec<String>,
    /// Effect applied to a matching listing.
    pub effect: PolicyEffect,
    /// Reviewer-facing rationale.
    pub rationale: String,
}

impl PolicyFilter {
    /// Whether this filter matches the given review packet.
    ///
    /// A filter matches when any of the packet's values for the filter dimension is
    /// among the filter's [`PolicyFilter::match_values`].
    pub fn matches(&self, packet: &MirrorReviewPacket) -> bool {
        let values: BTreeSet<&str> = self.match_values.iter().map(String::as_str).collect();
        packet
            .dimension_values(self.dimension)
            .iter()
            .any(|value| values.contains(value.as_str()))
    }
}

/// One policy evaluation of a review packet against the filter set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyEvaluation {
    /// Stable evaluation id.
    pub evaluation_id: String,
    /// Review packet this evaluation gates.
    pub review_ref: String,
    /// Filters that matched, in sorted id order; must equal the recomputed set.
    #[serde(default)]
    pub matched_filter_refs: Vec<String>,
    /// Strongest matching filter effect; must equal the recomputed value.
    pub strongest_effect: PolicyEffect,
    /// Recomputed gate decision; must equal the recomputed value.
    pub gate_decision: PolicyGateDecision,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MirrorAndSideloadSummary {
    /// Total review packets.
    pub total_review_packets: usize,
    /// Total policy filters.
    pub total_policy_filters: usize,
    /// Total policy evaluations.
    pub total_policy_evaluations: usize,
    /// Packets that may proceed directly.
    pub proceed_packets: usize,
    /// Packets that require review.
    pub review_required_packets: usize,
    /// Packets that are blocked.
    pub blocked_packets: usize,
    /// Packets served from a mirror or private registry.
    pub mirrored_or_private_packets: usize,
    /// Packets side-loaded by manual or air-gapped import.
    pub side_loaded_packets: usize,
    /// Packets carrying a publisher discontinuity.
    pub publisher_discontinuity_packets: usize,
    /// Packets carrying reduced or unverifiable provenance.
    pub provenance_reduced_packets: usize,
    /// Evaluations whose gate decision is allowed.
    pub allowed_evaluations: usize,
    /// Evaluations whose gate decision requires approval.
    pub approval_required_evaluations: usize,
    /// Evaluations whose gate decision is blocked.
    pub blocked_evaluations: usize,
    /// Distinct package kinds across packets.
    pub distinct_package_kinds: usize,
    /// Distinct acquisition channels across packets.
    pub distinct_acquisition_channels: usize,
}

/// A redaction-safe export row projected from a review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorAndSideloadExportRow {
    /// Review id.
    pub review_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-class token.
    pub source_class: String,
    /// Acquisition-channel token.
    pub acquisition_channel: String,
    /// Mirror-posture token.
    pub mirror_posture: String,
    /// Runtime-origin token.
    pub runtime_origin: String,
    /// Network-class token.
    pub network_class: String,
    /// Mirror-freshness token.
    pub mirror_freshness: String,
    /// Provenance-level token.
    pub provenance_level: String,
    /// Publisher-transfer-state token.
    pub publisher_transfer_state: String,
    /// Review-disposition token.
    pub review_disposition: String,
    /// Continuity-signal tokens.
    pub continuity_signals: Vec<String>,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Whether the packet carries every backing ref.
    pub export_safe: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorAndSideloadExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5MirrorAndSideloadExportRow>,
    /// Whether every packet is disposition-consistent and export-safe.
    pub all_packets_consistent: bool,
    /// Packets that are blocked.
    pub blocked_count: usize,
    /// Packets side-loaded by manual or air-gapped import.
    pub side_loaded_count: usize,
}

/// The typed M5 mirror-and-side-load packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MirrorAndSideload {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed source-class vocabulary (reused from marketplace fact-views).
    pub source_classes: Vec<SourceClass>,
    /// Closed acquisition-channel vocabulary.
    pub acquisition_channels: Vec<AcquisitionChannel>,
    /// Closed mirror-posture vocabulary (reused from marketplace fact-views).
    pub mirror_postures: Vec<MirrorPosture>,
    /// Closed support-class vocabulary (reused from the governance matrix).
    pub support_classes: Vec<SupportClass>,
    /// Closed runtime-origin vocabulary (reused from the governance matrix).
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed bridge/native-state vocabulary (reused from marketplace fact-views).
    pub bridge_native_states: Vec<BridgeNativeState>,
    /// Closed network-class vocabulary.
    pub network_classes: Vec<NetworkClass>,
    /// Closed lifecycle-state vocabulary (reused from the governance matrix).
    pub lifecycle_states: Vec<LifecycleState>,
    /// Closed evidence-freshness vocabulary (reused from the governance matrix).
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed compatibility-label vocabulary (reused from the governance matrix).
    pub compatibility_labels: Vec<CompatibilityLabel>,
    /// Closed permission-manifest-state vocabulary (reused from the governance matrix).
    pub permission_manifest_states: Vec<PermissionManifestState>,
    /// Closed activation-budget-band vocabulary (reused from the governance matrix).
    pub activation_budget_bands: Vec<ActivationBudgetBand>,
    /// Closed rollback-posture vocabulary (reused from the governance matrix).
    pub rollback_postures: Vec<RollbackPosture>,
    /// Closed publisher-transfer-state vocabulary (reused from install review).
    pub publisher_transfer_states: Vec<PublisherTransferState>,
    /// Closed signing-root-continuity vocabulary (reused from install review).
    pub signing_root_continuities: Vec<SigningRootContinuity>,
    /// Closed namespace-state vocabulary (reused from install review).
    pub namespace_states: Vec<NamespaceState>,
    /// Closed maintenance-state vocabulary.
    pub maintenance_states: Vec<MaintenanceState>,
    /// Closed mirror-freshness vocabulary.
    pub mirror_freshness_classes: Vec<MirrorFreshness>,
    /// Closed provenance-level vocabulary.
    pub provenance_levels: Vec<ProvenanceLevel>,
    /// Closed capability-class vocabulary (reused from activation budget).
    pub capability_classes: Vec<CapabilityClass>,
    /// Closed continuity-signal vocabulary.
    pub continuity_signals: Vec<ContinuitySignal>,
    /// Closed review-disposition vocabulary.
    pub review_dispositions: Vec<ReviewDisposition>,
    /// Closed policy-dimension vocabulary.
    pub policy_dimensions: Vec<PolicyDimension>,
    /// Closed policy-effect vocabulary.
    pub policy_effects: Vec<PolicyEffect>,
    /// Closed policy-gate-decision vocabulary.
    pub policy_gate_decisions: Vec<PolicyGateDecision>,
    /// Export-safe install/update review packets.
    #[serde(default)]
    pub review_packets: Vec<MirrorReviewPacket>,
    /// Enterprise policy filters.
    #[serde(default)]
    pub policy_filters: Vec<PolicyFilter>,
    /// Policy evaluations of review packets against the filter set.
    #[serde(default)]
    pub policy_evaluations: Vec<PolicyEvaluation>,
    /// Summary counts.
    pub summary: M5MirrorAndSideloadSummary,
}

impl M5MirrorAndSideload {
    /// Returns the review packet with the given id.
    pub fn review_packet(&self, review_id: &str) -> Option<&MirrorReviewPacket> {
        self.review_packets
            .iter()
            .find(|p| p.review_id == review_id)
    }

    /// Returns the policy filter with the given id.
    pub fn policy_filter(&self, filter_id: &str) -> Option<&PolicyFilter> {
        self.policy_filters
            .iter()
            .find(|f| f.filter_id == filter_id)
    }

    /// The filters that match a given review packet, in sorted id order.
    pub fn matching_filter_ids(&self, packet: &MirrorReviewPacket) -> Vec<String> {
        let mut ids: Vec<String> = self
            .policy_filters
            .iter()
            .filter(|f| f.matches(packet))
            .map(|f| f.filter_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// The strongest filter effect that applies to a review packet.
    pub fn strongest_effect(&self, packet: &MirrorReviewPacket) -> PolicyEffect {
        self.policy_filters
            .iter()
            .filter(|f| f.matches(packet))
            .fold(PolicyEffect::Allow, |effect, f| effect.widen(f.effect))
    }

    /// The recomputed gate decision for a review packet under the filter set.
    ///
    /// The decision is the stronger of the matched filter effect and the packet's own
    /// review disposition, so policy can tighten admission but never loosen it below
    /// the guardrail the review already requires.
    pub fn computed_gate_decision(&self, packet: &MirrorReviewPacket) -> PolicyGateDecision {
        self.strongest_effect(packet)
            .as_gate_decision()
            .widen(packet.review_disposition.as_gate_decision())
    }

    /// Recomputes the summary block from the packets, filters, and evaluations.
    pub fn computed_summary(&self) -> M5MirrorAndSideloadSummary {
        let count_disposition = |d: ReviewDisposition| {
            self.review_packets
                .iter()
                .filter(|p| p.review_disposition == d)
                .count()
        };
        let count_gate = |g: PolicyGateDecision| {
            self.policy_evaluations
                .iter()
                .filter(|e| e.gate_decision == g)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.review_packets.iter().map(|p| p.package_kind).collect();
        let channels: BTreeSet<AcquisitionChannel> = self
            .review_packets
            .iter()
            .map(|p| p.acquisition_channel)
            .collect();
        M5MirrorAndSideloadSummary {
            total_review_packets: self.review_packets.len(),
            total_policy_filters: self.policy_filters.len(),
            total_policy_evaluations: self.policy_evaluations.len(),
            proceed_packets: count_disposition(ReviewDisposition::Proceed),
            review_required_packets: count_disposition(ReviewDisposition::ReviewRequired),
            blocked_packets: count_disposition(ReviewDisposition::Blocked),
            mirrored_or_private_packets: self
                .review_packets
                .iter()
                .filter(|p| p.mirror_posture.is_mirrored_or_private())
                .count(),
            side_loaded_packets: self
                .review_packets
                .iter()
                .filter(|p| p.acquisition_channel.is_side_loaded())
                .count(),
            publisher_discontinuity_packets: self
                .review_packets
                .iter()
                .filter(|p| {
                    p.continuity_signals
                        .contains(&ContinuitySignal::PublisherDiscontinuous)
                })
                .count(),
            provenance_reduced_packets: self
                .review_packets
                .iter()
                .filter(|p| {
                    p.continuity_signals
                        .contains(&ContinuitySignal::ProvenanceReduced)
                        || p.continuity_signals
                            .contains(&ContinuitySignal::ProvenanceUnverifiable)
                })
                .count(),
            allowed_evaluations: count_gate(PolicyGateDecision::Allowed),
            approval_required_evaluations: count_gate(PolicyGateDecision::ApprovalRequired),
            blocked_evaluations: count_gate(PolicyGateDecision::Blocked),
            distinct_package_kinds: package_kinds.len(),
            distinct_acquisition_channels: channels.len(),
        }
    }

    /// Whether every packet is disposition-consistent and export-safe, and every
    /// evaluation matches its recomputation.
    pub fn all_records_consistent(&self) -> bool {
        if !self
            .review_packets
            .iter()
            .all(|p| p.disposition_consistent() && p.is_export_safe())
        {
            return false;
        }
        self.policy_evaluations
            .iter()
            .all(|e| match self.review_packet(&e.review_ref) {
                Some(packet) => {
                    e.matched_filter_refs == self.matching_filter_ids(packet)
                        && e.strongest_effect == self.strongest_effect(packet)
                        && e.gate_decision == self.computed_gate_decision(packet)
                }
                None => false,
            })
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// docs/help, and release/audit packets — render instead of restating mirror,
    /// side-load, provenance, and continuity status by hand.
    pub fn export_projection(&self) -> M5MirrorAndSideloadExportProjection {
        let rows = self
            .review_packets
            .iter()
            .map(|p| M5MirrorAndSideloadExportRow {
                review_id: p.review_id.clone(),
                package_kind: p.package_kind.as_str().to_owned(),
                source_class: p.source_class.as_str().to_owned(),
                acquisition_channel: p.acquisition_channel.as_str().to_owned(),
                mirror_posture: p.mirror_posture.as_str().to_owned(),
                runtime_origin: p.runtime_origin.as_str().to_owned(),
                network_class: p.network_class.as_str().to_owned(),
                mirror_freshness: p.mirror_freshness.as_str().to_owned(),
                provenance_level: p.provenance_level.as_str().to_owned(),
                publisher_transfer_state: p.publisher_transfer_state.as_str().to_owned(),
                review_disposition: p.review_disposition.as_str().to_owned(),
                continuity_signals: p
                    .continuity_signals
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                governance_family_ref: p.governance_family_ref.clone(),
                export_safe: p.is_export_safe(),
                summary: format!(
                    "{}: source {}, channel {}, posture {}, runtime {}, provenance {}, mirror {}, publisher {}, disposition {}",
                    p.package_kind.as_str(),
                    p.source_class.as_str(),
                    p.acquisition_channel.as_str(),
                    p.mirror_posture.as_str(),
                    p.runtime_origin.as_str(),
                    p.provenance_level.as_str(),
                    p.mirror_freshness.as_str(),
                    p.publisher_transfer_state.as_str(),
                    p.review_disposition.as_str(),
                ),
            })
            .collect();
        M5MirrorAndSideloadExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_packets_consistent: self.all_records_consistent(),
            blocked_count: self
                .review_packets
                .iter()
                .filter(|p| p.review_disposition == ReviewDisposition::Blocked)
                .count(),
            side_loaded_count: self
                .review_packets
                .iter()
                .filter(|p| p.acquisition_channel.is_side_loaded())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5MirrorAndSideloadViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_packets = BTreeSet::new();
        for packet in &self.review_packets {
            if !seen_packets.insert(packet.review_id.clone()) {
                violations.push(M5MirrorAndSideloadViolation::DuplicateReviewId {
                    review_id: packet.review_id.clone(),
                });
            }
            self.validate_packet(packet, &mut violations);
        }

        let mut seen_filters = BTreeSet::new();
        for filter in &self.policy_filters {
            if !seen_filters.insert(filter.filter_id.clone()) {
                violations.push(M5MirrorAndSideloadViolation::DuplicateFilterId {
                    filter_id: filter.filter_id.clone(),
                });
            }
            self.validate_filter(filter, &mut violations);
        }

        let mut seen_evals = BTreeSet::new();
        for evaluation in &self.policy_evaluations {
            if !seen_evals.insert(evaluation.evaluation_id.clone()) {
                violations.push(M5MirrorAndSideloadViolation::DuplicateEvaluationId {
                    evaluation_id: evaluation.evaluation_id.clone(),
                });
            }
            self.validate_evaluation(evaluation, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5MirrorAndSideloadViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5MirrorAndSideloadViolation>) {
        if self.schema_version != M5_MIRROR_AND_SIDELOAD_SCHEMA_VERSION {
            violations.push(M5MirrorAndSideloadViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_MIRROR_AND_SIDELOAD_RECORD_KIND {
            violations.push(M5MirrorAndSideloadViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MirrorAndSideloadViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "acquisition_channels",
                self.acquisition_channels == AcquisitionChannel::ALL.to_vec(),
            ),
            (
                "mirror_postures",
                self.mirror_postures == MirrorPosture::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
            ),
            (
                "bridge_native_states",
                self.bridge_native_states == BridgeNativeState::ALL.to_vec(),
            ),
            (
                "network_classes",
                self.network_classes == NetworkClass::ALL.to_vec(),
            ),
            (
                "lifecycle_states",
                self.lifecycle_states == LifecycleState::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "compatibility_labels",
                self.compatibility_labels == CompatibilityLabel::ALL.to_vec(),
            ),
            (
                "permission_manifest_states",
                self.permission_manifest_states == PermissionManifestState::ALL.to_vec(),
            ),
            (
                "activation_budget_bands",
                self.activation_budget_bands == ActivationBudgetBand::ALL.to_vec(),
            ),
            (
                "rollback_postures",
                self.rollback_postures == RollbackPosture::ALL.to_vec(),
            ),
            (
                "publisher_transfer_states",
                self.publisher_transfer_states == PublisherTransferState::ALL.to_vec(),
            ),
            (
                "signing_root_continuities",
                self.signing_root_continuities == SigningRootContinuity::ALL.to_vec(),
            ),
            (
                "namespace_states",
                self.namespace_states == NamespaceState::ALL.to_vec(),
            ),
            (
                "maintenance_states",
                self.maintenance_states == MaintenanceState::ALL.to_vec(),
            ),
            (
                "mirror_freshness_classes",
                self.mirror_freshness_classes == MirrorFreshness::ALL.to_vec(),
            ),
            (
                "provenance_levels",
                self.provenance_levels == ProvenanceLevel::ALL.to_vec(),
            ),
            (
                "capability_classes",
                self.capability_classes == CapabilityClass::ALL.to_vec(),
            ),
            (
                "continuity_signals",
                self.continuity_signals == ContinuitySignal::ALL.to_vec(),
            ),
            (
                "review_dispositions",
                self.review_dispositions == ReviewDisposition::ALL.to_vec(),
            ),
            (
                "policy_dimensions",
                self.policy_dimensions == PolicyDimension::ALL.to_vec(),
            ),
            (
                "policy_effects",
                self.policy_effects == PolicyEffect::ALL.to_vec(),
            ),
            (
                "policy_gate_decisions",
                self.policy_gate_decisions == PolicyGateDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5MirrorAndSideloadViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_packet(
        &self,
        packet: &MirrorReviewPacket,
        violations: &mut Vec<M5MirrorAndSideloadViolation>,
    ) {
        for (field, value) in [
            ("review_id", &packet.review_id),
            ("display_label", &packet.display_label),
            ("governance_family_ref", &packet.governance_family_ref),
            ("publisher_ref", &packet.publisher_ref),
            ("signing_root_ref", &packet.signing_root_ref),
            ("namespace_ref", &packet.namespace_ref),
            ("provenance_ref", &packet.provenance_ref),
            ("permission_manifest_ref", &packet.permission_manifest_ref),
            ("compatibility_ref", &packet.compatibility_ref),
            ("activation_budget_ref", &packet.activation_budget_ref),
            ("rollback_ref", &packet.rollback_ref),
            ("publisher_continuity_ref", &packet.publisher_continuity_ref),
            ("support_export_ref", &packet.support_export_ref),
            ("summary", &packet.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MirrorAndSideloadViolation::EmptyField {
                    id: packet.review_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_signals = BTreeSet::new();
        for signal in &packet.continuity_signals {
            if !seen_signals.insert(*signal) {
                violations.push(M5MirrorAndSideloadViolation::DuplicateContinuitySignal {
                    id: packet.review_id.clone(),
                    signal: signal.as_str(),
                });
            }
        }
        let mut seen_caps = BTreeSet::new();
        for capability in &packet.capability_classes {
            if !seen_caps.insert(*capability) {
                violations.push(M5MirrorAndSideloadViolation::DuplicateCapabilityClass {
                    id: packet.review_id.clone(),
                    capability: capability.as_str(),
                });
            }
        }

        // The published signals must equal the recomputed set, so a widening can
        // never be asserted or hidden by hand.
        if packet.continuity_signals != packet.computed_continuity_signals() {
            violations.push(M5MirrorAndSideloadViolation::ContinuitySignalsMismatch {
                id: packet.review_id.clone(),
            });
        }

        // The published disposition must equal the recomputed disposition, so a
        // mirror, private, side-loaded, or air-gapped review can never present a
        // weaker gate than its facts warrant.
        let computed = packet.computed_review_disposition();
        if packet.review_disposition != computed {
            violations.push(M5MirrorAndSideloadViolation::ReviewDispositionMismatch {
                id: packet.review_id.clone(),
                stored: packet.review_disposition.as_str(),
                computed: computed.as_str(),
            });
        }

        // The lane guardrail: every review packet, regardless of source, must carry
        // the full backing-ref set so support and audit can reason about it.
        if !packet.is_export_safe() {
            violations.push(M5MirrorAndSideloadViolation::NotExportSafe {
                id: packet.review_id.clone(),
            });
        }
    }

    fn validate_filter(
        &self,
        filter: &PolicyFilter,
        violations: &mut Vec<M5MirrorAndSideloadViolation>,
    ) {
        for (field, value) in [
            ("filter_id", &filter.filter_id),
            ("display_label", &filter.display_label),
            ("rationale", &filter.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MirrorAndSideloadViolation::EmptyField {
                    id: filter.filter_id.clone(),
                    field_name: field,
                });
            }
        }
        if filter.match_values.is_empty() {
            violations.push(M5MirrorAndSideloadViolation::EmptyFilterMatchValues {
                filter_id: filter.filter_id.clone(),
            });
        }
        if filter.match_values.iter().any(|v| v.trim().is_empty()) {
            violations.push(M5MirrorAndSideloadViolation::EmptyField {
                id: filter.filter_id.clone(),
                field_name: "match_values",
            });
        }
    }

    fn validate_evaluation(
        &self,
        evaluation: &PolicyEvaluation,
        violations: &mut Vec<M5MirrorAndSideloadViolation>,
    ) {
        for (field, value) in [
            ("evaluation_id", &evaluation.evaluation_id),
            ("review_ref", &evaluation.review_ref),
            ("summary", &evaluation.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MirrorAndSideloadViolation::EmptyField {
                    id: evaluation.evaluation_id.clone(),
                    field_name: field,
                });
            }
        }

        match self.review_packet(&evaluation.review_ref) {
            Some(packet) => {
                let matched = self.matching_filter_ids(packet);
                if evaluation.matched_filter_refs != matched {
                    violations.push(M5MirrorAndSideloadViolation::MatchedFiltersMismatch {
                        evaluation_id: evaluation.evaluation_id.clone(),
                    });
                }
                if evaluation.strongest_effect != self.strongest_effect(packet) {
                    violations.push(M5MirrorAndSideloadViolation::StrongestEffectMismatch {
                        evaluation_id: evaluation.evaluation_id.clone(),
                    });
                }
                let computed = self.computed_gate_decision(packet);
                if evaluation.gate_decision != computed {
                    violations.push(M5MirrorAndSideloadViolation::GateDecisionMismatch {
                        evaluation_id: evaluation.evaluation_id.clone(),
                        stored: evaluation.gate_decision.as_str(),
                        computed: computed.as_str(),
                    });
                }
            }
            None => violations.push(M5MirrorAndSideloadViolation::DanglingReviewRef {
                evaluation_id: evaluation.evaluation_id.clone(),
                review_ref: evaluation.review_ref.clone(),
            }),
        }

        for filter_ref in &evaluation.matched_filter_refs {
            if self.policy_filter(filter_ref).is_none() {
                violations.push(M5MirrorAndSideloadViolation::DanglingFilterRef {
                    evaluation_id: evaluation.evaluation_id.clone(),
                    filter_ref: filter_ref.clone(),
                });
            }
        }
    }
}

/// A validation violation for the M5 mirror-and-side-load packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MirrorAndSideloadViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Packet, filter, evaluation, or packet-envelope id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A review-packet id appears more than once.
    DuplicateReviewId {
        /// Duplicate review id.
        review_id: String,
    },
    /// A policy-filter id appears more than once.
    DuplicateFilterId {
        /// Duplicate filter id.
        filter_id: String,
    },
    /// A policy-evaluation id appears more than once.
    DuplicateEvaluationId {
        /// Duplicate evaluation id.
        evaluation_id: String,
    },
    /// A packet lists a continuity signal more than once.
    DuplicateContinuitySignal {
        /// Review id.
        id: String,
        /// Signal token.
        signal: &'static str,
    },
    /// A packet lists a capability class more than once.
    DuplicateCapabilityClass {
        /// Review id.
        id: String,
        /// Capability token.
        capability: &'static str,
    },
    /// A packet's continuity signals disagree with the recomputed set.
    ContinuitySignalsMismatch {
        /// Review id.
        id: String,
    },
    /// A packet's stored review disposition disagrees with the recomputed value.
    ReviewDispositionMismatch {
        /// Review id.
        id: String,
        /// Stored disposition token.
        stored: &'static str,
        /// Recomputed disposition token.
        computed: &'static str,
    },
    /// A packet is missing a required backing ref.
    NotExportSafe {
        /// Review id.
        id: String,
    },
    /// A policy filter carries no match values.
    EmptyFilterMatchValues {
        /// Filter id.
        filter_id: String,
    },
    /// An evaluation references a review packet that does not exist.
    DanglingReviewRef {
        /// Evaluation id.
        evaluation_id: String,
        /// Missing review ref.
        review_ref: String,
    },
    /// An evaluation references a policy filter that does not exist.
    DanglingFilterRef {
        /// Evaluation id.
        evaluation_id: String,
        /// Missing filter ref.
        filter_ref: String,
    },
    /// An evaluation's matched filters disagree with the recomputed set.
    MatchedFiltersMismatch {
        /// Evaluation id.
        evaluation_id: String,
    },
    /// An evaluation's strongest effect disagrees with the recomputed value.
    StrongestEffectMismatch {
        /// Evaluation id.
        evaluation_id: String,
    },
    /// An evaluation's gate decision disagrees with the recomputed value.
    GateDecisionMismatch {
        /// Evaluation id.
        evaluation_id: String,
        /// Stored decision token.
        stored: &'static str,
        /// Recomputed decision token.
        computed: &'static str,
    },
    /// The summary counts disagree with the packets, filters, and evaluations.
    SummaryMismatch,
}

impl fmt::Display for M5MirrorAndSideloadViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateReviewId { review_id } => {
                write!(f, "duplicate review packet id {review_id}")
            }
            Self::DuplicateFilterId { filter_id } => {
                write!(f, "duplicate policy filter id {filter_id}")
            }
            Self::DuplicateEvaluationId { evaluation_id } => {
                write!(f, "duplicate policy evaluation id {evaluation_id}")
            }
            Self::DuplicateContinuitySignal { id, signal } => {
                write!(f, "packet {id} repeats continuity signal {signal}")
            }
            Self::DuplicateCapabilityClass { id, capability } => {
                write!(f, "packet {id} repeats capability class {capability}")
            }
            Self::ContinuitySignalsMismatch { id } => {
                write!(
                    f,
                    "packet {id} continuity signals disagree with the recomputed set"
                )
            }
            Self::ReviewDispositionMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "packet {id} publishes review disposition {stored} but the recomputed disposition is {computed}"
                )
            }
            Self::NotExportSafe { id } => {
                write!(f, "packet {id} is missing a required backing ref")
            }
            Self::EmptyFilterMatchValues { filter_id } => {
                write!(f, "policy filter {filter_id} carries no match values")
            }
            Self::DanglingReviewRef {
                evaluation_id,
                review_ref,
            } => {
                write!(
                    f,
                    "evaluation {evaluation_id} references missing review packet {review_ref}"
                )
            }
            Self::DanglingFilterRef {
                evaluation_id,
                filter_ref,
            } => {
                write!(
                    f,
                    "evaluation {evaluation_id} references missing policy filter {filter_ref}"
                )
            }
            Self::MatchedFiltersMismatch { evaluation_id } => {
                write!(
                    f,
                    "evaluation {evaluation_id} matched filters disagree with the recomputed set"
                )
            }
            Self::StrongestEffectMismatch { evaluation_id } => {
                write!(
                    f,
                    "evaluation {evaluation_id} strongest effect disagrees with the recomputed value"
                )
            }
            Self::GateDecisionMismatch {
                evaluation_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "evaluation {evaluation_id} publishes gate decision {stored} but the recomputed decision is {computed}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the records")
            }
        }
    }
}

impl Error for M5MirrorAndSideloadViolation {}

/// Loads the embedded M5 mirror-and-side-load packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5MirrorAndSideload`].
pub fn current_m5_mirror_and_sideload() -> Result<M5MirrorAndSideload, serde_json::Error> {
    serde_json::from_str(M5_MIRROR_AND_SIDELOAD_JSON)
}

#[cfg(test)]
mod tests;
