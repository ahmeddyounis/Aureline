//! Canonical M5 author-side and publish-preview matrix with a non-inheriting
//! publish gate that keeps blockers and warnings explicit.
//!
//! Where the install-governance matrix in
//! [`crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix`]
//! speaks for the *end-user install* of a marketed M5 artifact family, this module
//! freezes the **author-side** lane: the local extension workspace, sideload review,
//! sandbox/runtime inspection, hot-reload/relaunch, and publish preview an author
//! drives before a package reaches the public registry. Each [`AuthorFamilyRow`]
//! reuses the shared [`ArtifactFamily`] vocabulary and names the family's runtime
//! class, host/ABI, local-workspace build state, signing state, declared trust
//! posture, hot-reload widening posture, publish-review requirement, conformance
//! output, and anti-abuse transparency state.
//!
//! The model is a publish-control gate, not a manifest linter. From the observed
//! states it recomputes:
//!
//! - the **effective trust posture** the family may publish — capped by its signing
//!   state, so a locally-built or side-loaded artifact never inherits a
//!   verified-publisher or enterprise-approved badge just because it was built on a
//!   trusted machine;
//! - an explicit, severity-tagged set of [`PublishFinding`]s — **blockers** that
//!   hard-stop publication versus **warnings** that publish with disclosure — so the
//!   publish preview keeps registry-policy consequences visible instead of collapsing
//!   to a pass/fail lint; and
//! - a [`PublishReadiness`] verdict that withholds a quarantined family, blocks a
//!   family carrying any blocker, publishes-with-warnings a family carrying only
//!   warnings, and clears a genuinely clean family.
//!
//! Hot-reload can never silently widen authority: a hot reload that widens the
//! runtime class, adds an external executable, or expands permissions raises a
//! blocking finding until a fresh review clears it. Each row's
//! [`AuthorFamilyRow::published_trust_posture`],
//! [`AuthorFamilyRow::publish_readiness`], and recomputed
//! [`AuthorFamilyRow::findings`] are validated against the gate, so local authoring
//! surfaces, package install/update flows, diagnostics, and certification packets all
//! project the same truth instead of retyping it.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-author-and-publish-preview.json` and embedded here, so
//! this typed consumer and any CI gate agree on every family without a cargo build in
//! CI. The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, signing secrets, or sideload
//! source.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::ArtifactFamily;

/// Supported M5 author-and-publish-preview matrix schema version.
pub const M5_AUTHOR_PUBLISH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_AUTHOR_PUBLISH_RECORD_KIND: &str = "m5_author_and_publish_preview_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_AUTHOR_PUBLISH_PATH: &str =
    "artifacts/ecosystem/m5/m5-author-and-publish-preview.json";

/// Embedded checked-in packet JSON.
pub const M5_AUTHOR_PUBLISH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-author-and-publish-preview.json"
));

/// Runtime class of an artifact family's executable surface.
///
/// Aligned with the stable extension runtime-class vocabulary so author surfaces and
/// install surfaces describe the same host shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeClass {
    /// No executable code; data or content only.
    PassivePackage,
    /// Runs inside the capability-scoped Wasm sandbox.
    WasmCapabilitySandbox,
    /// Declarative or host-rendered view; no untrusted code.
    DeclarativeHostRenderedView,
    /// Runs against an external host process.
    ExternalHost,
    /// Runs through a compatibility bridge adapter.
    CompatibilityBridge,
    /// Runs as a remote-side component.
    RemoteSideComponent,
}

impl RuntimeClass {
    /// Every runtime class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PassivePackage,
        Self::WasmCapabilitySandbox,
        Self::DeclarativeHostRenderedView,
        Self::ExternalHost,
        Self::CompatibilityBridge,
        Self::RemoteSideComponent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PassivePackage => "passive_package",
            Self::WasmCapabilitySandbox => "wasm_capability_sandbox",
            Self::DeclarativeHostRenderedView => "declarative_host_rendered_view",
            Self::ExternalHost => "external_host",
            Self::CompatibilityBridge => "compatibility_bridge",
            Self::RemoteSideComponent => "remote_side_component",
        }
    }
}

/// Host/ABI execution locus of an artifact family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostAbiClass {
    /// No code is executed.
    NoCodeExecution,
    /// Executes on the local machine.
    LocalMachine,
    /// Executes on a managed workspace host.
    ManagedHost,
    /// Executes on a remote target.
    RemoteTarget,
    /// Executes in a separate external process.
    ExternalProcess,
    /// Executes in a browser runtime.
    BrowserRuntime,
}

impl HostAbiClass {
    /// Every host/ABI class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoCodeExecution,
        Self::LocalMachine,
        Self::ManagedHost,
        Self::RemoteTarget,
        Self::ExternalProcess,
        Self::BrowserRuntime,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCodeExecution => "no_code_execution",
            Self::LocalMachine => "local_machine",
            Self::ManagedHost => "managed_host",
            Self::RemoteTarget => "remote_target",
            Self::ExternalProcess => "external_process",
            Self::BrowserRuntime => "browser_runtime",
        }
    }
}

/// Build/source state of the family's local extension workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceState {
    /// Source is present and a current build exists.
    SourcePresentBuilt,
    /// Source is missing; the artifact can be inspected but not rebuilt or published.
    SourceMissing,
    /// The local build failed.
    BuildFailed,
}

impl WorkspaceState {
    /// Every workspace state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SourcePresentBuilt,
        Self::SourceMissing,
        Self::BuildFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourcePresentBuilt => "source_present_built",
            Self::SourceMissing => "source_missing",
            Self::BuildFailed => "build_failed",
        }
    }

    /// The blocking finding this workspace state raises, if any.
    pub const fn finding(self) -> Option<PublishFindingCode> {
        match self {
            Self::SourcePresentBuilt => None,
            Self::SourceMissing => Some(PublishFindingCode::SourceMissing),
            Self::BuildFailed => Some(PublishFindingCode::BuildFailed),
        }
    }
}

/// Signing/provenance state of the family's authored artifact.
///
/// The signing state caps the [`TrustPosture`] the family may publish, so an
/// unsigned local-dev build or a side-loaded artifact can never inherit a
/// verified-publisher or enterprise-approved badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureState {
    /// Signed and the signature is verified.
    SignedVerified,
    /// Signed but the signature is not yet verified.
    SignedUnverified,
    /// Unsigned local development build.
    UnsignedLocalDev,
    /// Unsigned side-loaded artifact.
    UnsignedSideload,
    /// The signature has been revoked.
    RevokedSignature,
}

impl SignatureState {
    /// Every signing state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SignedVerified,
        Self::SignedUnverified,
        Self::UnsignedLocalDev,
        Self::UnsignedSideload,
        Self::RevokedSignature,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedVerified => "signed_verified",
            Self::SignedUnverified => "signed_unverified",
            Self::UnsignedLocalDev => "unsigned_local_dev",
            Self::UnsignedSideload => "unsigned_sideload",
            Self::RevokedSignature => "revoked_signature",
        }
    }

    /// Highest trust posture this signing state lets a family publish.
    ///
    /// An unsigned local-dev build, an unsigned side-load, and a revoked signature
    /// all cap at [`TrustPosture::UnsignedLocalOnly`]; this is the non-inheritance
    /// rule that keeps locally-built or side-loaded artifacts from inheriting a
    /// trusted publisher badge.
    pub const fn trust_ceiling(self) -> TrustPosture {
        match self {
            Self::SignedVerified => TrustPosture::EnterpriseApproved,
            Self::SignedUnverified => TrustPosture::RegistryBound,
            Self::UnsignedLocalDev | Self::UnsignedSideload | Self::RevokedSignature => {
                TrustPosture::UnsignedLocalOnly
            }
        }
    }

    /// Whether this signing state structurally forbids inheriting a trusted badge.
    pub const fn is_local_or_untrusted(self) -> bool {
        matches!(
            self,
            Self::UnsignedLocalDev | Self::UnsignedSideload | Self::RevokedSignature
        )
    }

    /// The finding this signing state raises, if any.
    pub const fn finding(self) -> Option<PublishFindingCode> {
        match self {
            Self::SignedVerified => None,
            Self::SignedUnverified | Self::UnsignedLocalDev | Self::UnsignedSideload => {
                Some(PublishFindingCode::ProvenanceUnverified)
            }
            Self::RevokedSignature => Some(PublishFindingCode::SignatureRevoked),
        }
    }
}

/// Trust posture a family may carry once published.
///
/// Ordered low-to-high by [`TrustPosture::rank`]: an
/// [`TrustPosture::UnsignedLocalOnly`] artifact carries no inherited badge, and an
/// [`TrustPosture::EnterpriseApproved`] artifact carries the strongest managed badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPosture {
    /// Local-only; carries no inherited publisher trust.
    UnsignedLocalOnly,
    /// Bound to a registry release identity.
    RegistryBound,
    /// Carries a verified-publisher badge.
    VerifiedPublisher,
    /// Carries an enterprise/managed-approved badge.
    EnterpriseApproved,
}

impl TrustPosture {
    /// Every trust posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UnsignedLocalOnly,
        Self::RegistryBound,
        Self::VerifiedPublisher,
        Self::EnterpriseApproved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsignedLocalOnly => "unsigned_local_only",
            Self::RegistryBound => "registry_bound",
            Self::VerifiedPublisher => "verified_publisher",
            Self::EnterpriseApproved => "enterprise_approved",
        }
    }

    /// Monotonic rank; higher means stronger inherited trust.
    pub const fn rank(self) -> u8 {
        match self {
            Self::UnsignedLocalOnly => 0,
            Self::RegistryBound => 1,
            Self::VerifiedPublisher => 2,
            Self::EnterpriseApproved => 3,
        }
    }

    /// The weaker (lower-rank) of two trust postures.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }

    /// Whether this posture is a trusted publisher badge that must never be inherited
    /// by a local or side-loaded artifact.
    pub const fn is_trusted_badge(self) -> bool {
        matches!(self, Self::VerifiedPublisher | Self::EnterpriseApproved)
    }
}

/// Hot-reload/relaunch posture of the family's local-dev loop.
///
/// A hot reload that widens authority — the runtime class, an external executable, or
/// permissions — forces a fresh review step rather than taking effect silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotReloadPosture {
    /// Hot reload applies without widening authority.
    NoWidening,
    /// A full relaunch only; no widening of authority.
    RelaunchOnly,
    /// Hot reload would widen the runtime class; held for a fresh review.
    RuntimeClassWidenedPendingReview,
    /// Hot reload would expand permissions; held for a fresh review.
    PermissionsWidenedPendingReview,
    /// Hot reload would add an external executable; held for a fresh review.
    ExternalExecutableAddedPendingReview,
}

impl HotReloadPosture {
    /// Every hot-reload posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoWidening,
        Self::RelaunchOnly,
        Self::RuntimeClassWidenedPendingReview,
        Self::PermissionsWidenedPendingReview,
        Self::ExternalExecutableAddedPendingReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWidening => "no_widening",
            Self::RelaunchOnly => "relaunch_only",
            Self::RuntimeClassWidenedPendingReview => "runtime_class_widened_pending_review",
            Self::PermissionsWidenedPendingReview => "permissions_widened_pending_review",
            Self::ExternalExecutableAddedPendingReview => {
                "external_executable_added_pending_review"
            }
        }
    }

    /// The blocking finding this hot-reload posture raises, if any.
    pub const fn finding(self) -> Option<PublishFindingCode> {
        match self {
            Self::NoWidening | Self::RelaunchOnly => None,
            Self::RuntimeClassWidenedPendingReview => {
                Some(PublishFindingCode::HotReloadRuntimeWidened)
            }
            Self::PermissionsWidenedPendingReview => {
                Some(PublishFindingCode::HotReloadPermissionsWidened)
            }
            Self::ExternalExecutableAddedPendingReview => {
                Some(PublishFindingCode::HotReloadExternalExecutableAdded)
            }
        }
    }
}

/// Publish-review requirement the family must clear before public release.
///
/// This is the registry-policy consequence the publish preview must surface, not a
/// pass/fail of a manifest lint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReviewRequirement {
    /// Full registry-policy review required.
    FullRegistryPolicyReview,
    /// Standard registry review required.
    StandardReview,
    /// Expedited review path applies.
    ExpeditedReview,
    /// Not publishable from the local authoring lane.
    NotPublishableFromLocal,
}

impl PublishReviewRequirement {
    /// Every publish-review requirement, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullRegistryPolicyReview,
        Self::StandardReview,
        Self::ExpeditedReview,
        Self::NotPublishableFromLocal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRegistryPolicyReview => "full_registry_policy_review",
            Self::StandardReview => "standard_review",
            Self::ExpeditedReview => "expedited_review",
            Self::NotPublishableFromLocal => "not_publishable_from_local",
        }
    }
}

/// Conformance output for the family's authored artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceOutput {
    /// Fully conformant against the native conformance kit.
    Conformant,
    /// Conformant through a compatibility bridge.
    BridgeConformant,
    /// Partially conformant.
    Partial,
    /// Conformance failed.
    Failed,
    /// A retest is pending.
    RetestPending,
    /// Conformance has not been run.
    NotRun,
}

impl ConformanceOutput {
    /// Every conformance output, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Conformant,
        Self::BridgeConformant,
        Self::Partial,
        Self::Failed,
        Self::RetestPending,
        Self::NotRun,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::BridgeConformant => "bridge_conformant",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::RetestPending => "retest_pending",
            Self::NotRun => "not_run",
        }
    }

    /// The finding this conformance output raises, if any.
    pub const fn finding(self) -> Option<PublishFindingCode> {
        match self {
            Self::Conformant | Self::BridgeConformant => None,
            Self::Failed => Some(PublishFindingCode::ConformanceFailed),
            Self::Partial | Self::RetestPending | Self::NotRun => {
                Some(PublishFindingCode::ConformanceIncomplete)
            }
        }
    }
}

/// Anti-abuse transparency state of the family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AntiAbuseTransparency {
    /// Disclosed and clean; no abuse history.
    DisclosedClean,
    /// A prior publisher-loss/transfer history is disclosed.
    PublisherLossHistoryDisclosed,
    /// The anti-abuse posture is undisclosed.
    Undisclosed,
    /// The family is quarantined pending anti-abuse review.
    Quarantined,
}

impl AntiAbuseTransparency {
    /// Every anti-abuse transparency state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DisclosedClean,
        Self::PublisherLossHistoryDisclosed,
        Self::Undisclosed,
        Self::Quarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisclosedClean => "disclosed_clean",
            Self::PublisherLossHistoryDisclosed => "publisher_loss_history_disclosed",
            Self::Undisclosed => "undisclosed",
            Self::Quarantined => "quarantined",
        }
    }

    /// Whether the family is quarantined and must be withheld.
    pub const fn is_quarantined(self) -> bool {
        matches!(self, Self::Quarantined)
    }

    /// The finding this anti-abuse state raises, if any.
    pub const fn finding(self) -> Option<PublishFindingCode> {
        match self {
            Self::DisclosedClean => None,
            Self::PublisherLossHistoryDisclosed => Some(PublishFindingCode::PublisherLossHistory),
            Self::Undisclosed => Some(PublishFindingCode::AntiAbuseUndisclosed),
            Self::Quarantined => Some(PublishFindingCode::AntiAbuseQuarantined),
        }
    }
}

/// Severity of a publish finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Hard-stops publication.
    Blocker,
    /// Publishes with disclosure.
    Warning,
}

impl FindingSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 2] = [Self::Blocker, Self::Warning];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Warning => "warning",
        }
    }
}

/// Domain a publish finding belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingDomain {
    /// Local extension workspace (source/build).
    LocalWorkspace,
    /// Sideload trust / provenance.
    SideloadTrust,
    /// Hot-reload widening.
    HotReload,
    /// Conformance outputs.
    Conformance,
    /// Anti-abuse transparency.
    AntiAbuse,
}

impl FindingDomain {
    /// Every finding domain, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalWorkspace,
        Self::SideloadTrust,
        Self::HotReload,
        Self::Conformance,
        Self::AntiAbuse,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::SideloadTrust => "sideload_trust",
            Self::HotReload => "hot_reload",
            Self::Conformance => "conformance",
            Self::AntiAbuse => "anti_abuse",
        }
    }
}

/// A closed publish-finding code raised by the gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishFindingCode {
    /// The local build failed.
    BuildFailed,
    /// Source is missing.
    SourceMissing,
    /// The signature is revoked.
    SignatureRevoked,
    /// Provenance is unverified (signed-unverified or unsigned).
    ProvenanceUnverified,
    /// Hot reload would widen the runtime class without a fresh review.
    HotReloadRuntimeWidened,
    /// Hot reload would expand permissions without a fresh review.
    HotReloadPermissionsWidened,
    /// Hot reload would add an external executable without a fresh review.
    HotReloadExternalExecutableAdded,
    /// Conformance failed.
    ConformanceFailed,
    /// Conformance is incomplete (partial, retest-pending, or not run).
    ConformanceIncomplete,
    /// The anti-abuse posture is undisclosed.
    AntiAbuseUndisclosed,
    /// A publisher-loss history is disclosed.
    PublisherLossHistory,
    /// The family is quarantined.
    AntiAbuseQuarantined,
}

impl PublishFindingCode {
    /// Every publish-finding code, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::BuildFailed,
        Self::SourceMissing,
        Self::SignatureRevoked,
        Self::ProvenanceUnverified,
        Self::HotReloadRuntimeWidened,
        Self::HotReloadPermissionsWidened,
        Self::HotReloadExternalExecutableAdded,
        Self::ConformanceFailed,
        Self::ConformanceIncomplete,
        Self::AntiAbuseUndisclosed,
        Self::PublisherLossHistory,
        Self::AntiAbuseQuarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildFailed => "build_failed",
            Self::SourceMissing => "source_missing",
            Self::SignatureRevoked => "signature_revoked",
            Self::ProvenanceUnverified => "provenance_unverified",
            Self::HotReloadRuntimeWidened => "hot_reload_runtime_widened",
            Self::HotReloadPermissionsWidened => "hot_reload_permissions_widened",
            Self::HotReloadExternalExecutableAdded => "hot_reload_external_executable_added",
            Self::ConformanceFailed => "conformance_failed",
            Self::ConformanceIncomplete => "conformance_incomplete",
            Self::AntiAbuseUndisclosed => "anti_abuse_undisclosed",
            Self::PublisherLossHistory => "publisher_loss_history",
            Self::AntiAbuseQuarantined => "anti_abuse_quarantined",
        }
    }

    /// Canonical declaration rank used to order findings deterministically.
    pub const fn rank(self) -> u8 {
        match self {
            Self::BuildFailed => 0,
            Self::SourceMissing => 1,
            Self::SignatureRevoked => 2,
            Self::ProvenanceUnverified => 3,
            Self::HotReloadRuntimeWidened => 4,
            Self::HotReloadPermissionsWidened => 5,
            Self::HotReloadExternalExecutableAdded => 6,
            Self::ConformanceFailed => 7,
            Self::ConformanceIncomplete => 8,
            Self::AntiAbuseUndisclosed => 9,
            Self::PublisherLossHistory => 10,
            Self::AntiAbuseQuarantined => 11,
        }
    }

    /// Canonical severity of this finding.
    pub const fn severity(self) -> FindingSeverity {
        match self {
            Self::ProvenanceUnverified
            | Self::ConformanceIncomplete
            | Self::PublisherLossHistory => FindingSeverity::Warning,
            Self::BuildFailed
            | Self::SourceMissing
            | Self::SignatureRevoked
            | Self::HotReloadRuntimeWidened
            | Self::HotReloadPermissionsWidened
            | Self::HotReloadExternalExecutableAdded
            | Self::ConformanceFailed
            | Self::AntiAbuseUndisclosed
            | Self::AntiAbuseQuarantined => FindingSeverity::Blocker,
        }
    }

    /// Canonical domain of this finding.
    pub const fn domain(self) -> FindingDomain {
        match self {
            Self::BuildFailed | Self::SourceMissing => FindingDomain::LocalWorkspace,
            Self::SignatureRevoked | Self::ProvenanceUnverified => FindingDomain::SideloadTrust,
            Self::HotReloadRuntimeWidened
            | Self::HotReloadPermissionsWidened
            | Self::HotReloadExternalExecutableAdded => FindingDomain::HotReload,
            Self::ConformanceFailed | Self::ConformanceIncomplete => FindingDomain::Conformance,
            Self::AntiAbuseUndisclosed
            | Self::PublisherLossHistory
            | Self::AntiAbuseQuarantined => FindingDomain::AntiAbuse,
        }
    }
}

/// A severity-tagged publish finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishFinding {
    /// Closed finding code.
    pub code: PublishFindingCode,
    /// Severity; must equal the code's canonical severity.
    pub severity: FindingSeverity,
    /// Domain; must equal the code's canonical domain.
    pub domain: FindingDomain,
}

impl PublishFinding {
    /// Builds a finding from a code, filling severity and domain from the code.
    pub const fn of(code: PublishFindingCode) -> Self {
        Self {
            code,
            severity: code.severity(),
            domain: code.domain(),
        }
    }

    /// Whether the finding is a blocker.
    pub const fn is_blocker(self) -> bool {
        matches!(self.severity, FindingSeverity::Blocker)
    }
}

/// The publish verdict the gate reaches for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReadiness {
    /// No findings; the family is ready to publish.
    ReadyToPublish,
    /// Only warnings; the family publishes with disclosure.
    PublishableWithWarnings,
    /// At least one blocker; the family cannot publish.
    BlockedFromPublish,
    /// The family is quarantined and withheld entirely.
    WithheldQuarantined,
}

impl PublishReadiness {
    /// Every publish readiness, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReadyToPublish,
        Self::PublishableWithWarnings,
        Self::BlockedFromPublish,
        Self::WithheldQuarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToPublish => "ready_to_publish",
            Self::PublishableWithWarnings => "publishable_with_warnings",
            Self::BlockedFromPublish => "blocked_from_publish",
            Self::WithheldQuarantined => "withheld_quarantined",
        }
    }
}

/// One author-side and publish-preview row for a marketed M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorFamilyRow {
    /// Stable author-row id.
    pub family_id: String,
    /// Marketed M5 artifact family this row governs.
    pub artifact_family: ArtifactFamily,
    /// Runtime class of the authored artifact.
    pub runtime_class: RuntimeClass,
    /// Host/ABI execution locus.
    pub host_abi: HostAbiClass,
    /// Local-workspace build/source state.
    pub workspace_state: WorkspaceState,
    /// Signing/provenance state.
    pub signature_state: SignatureState,
    /// Trust posture the author requests, before the gate caps it.
    pub declared_trust_posture: TrustPosture,
    /// Trust posture actually published after the gate caps it.
    ///
    /// Must equal [`AuthorFamilyRow::effective_trust_posture`].
    pub published_trust_posture: TrustPosture,
    /// Hot-reload/relaunch posture.
    pub hot_reload_posture: HotReloadPosture,
    /// Publish-review requirement / registry-policy consequence.
    pub publish_review_requirement: PublishReviewRequirement,
    /// Conformance output.
    pub conformance_output: ConformanceOutput,
    /// Anti-abuse transparency state.
    pub anti_abuse_transparency: AntiAbuseTransparency,
    /// Publish verdict; must equal the recomputed readiness.
    pub publish_readiness: PublishReadiness,
    /// Severity-tagged findings; must equal the recomputed set, in canonical order.
    #[serde(default)]
    pub findings: Vec<PublishFinding>,
    /// Ref to the family's local extension workspace.
    pub workspace_ref: String,
    /// Ref to the family's sideload review record.
    pub sideload_review_ref: String,
    /// Ref to the family's sandbox/runtime inspector record.
    pub inspector_ref: String,
    /// Ref to the family's publish-preview record.
    pub publish_preview_ref: String,
    /// Ref to the family's anti-abuse transparency record.
    pub anti_abuse_ref: String,
    /// Ref to the family's conformance output.
    pub conformance_ref: String,
    /// Ref binding this row into registry, diagnostics, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl AuthorFamilyRow {
    /// The trust posture the gate lets this family publish.
    ///
    /// Lowers the author's declared posture to the ceiling implied by the signing
    /// state, so a locally-built, side-loaded, or revoked artifact can never inherit
    /// a verified-publisher or enterprise-approved badge.
    pub fn effective_trust_posture(&self) -> TrustPosture {
        self.declared_trust_posture
            .min(self.signature_state.trust_ceiling())
    }

    /// The findings recomputed from this family's observed states, in canonical order.
    pub fn computed_findings(&self) -> Vec<PublishFinding> {
        let mut codes: Vec<PublishFindingCode> = [
            self.workspace_state.finding(),
            self.signature_state.finding(),
            self.hot_reload_posture.finding(),
            self.conformance_output.finding(),
            self.anti_abuse_transparency.finding(),
        ]
        .into_iter()
        .flatten()
        .collect();
        codes.sort_by_key(|code| code.rank());
        codes.into_iter().map(PublishFinding::of).collect()
    }

    /// The publish verdict the gate must record for this family.
    pub fn computed_publish_readiness(&self) -> PublishReadiness {
        if self.anti_abuse_transparency.is_quarantined() {
            return PublishReadiness::WithheldQuarantined;
        }
        let findings = self.computed_findings();
        if findings.iter().any(|f| f.is_blocker()) {
            PublishReadiness::BlockedFromPublish
        } else if findings.is_empty() {
            PublishReadiness::ReadyToPublish
        } else {
            PublishReadiness::PublishableWithWarnings
        }
    }

    /// Whether the family is ready to publish with no findings.
    pub fn is_ready_to_publish(&self) -> bool {
        self.computed_publish_readiness() == PublishReadiness::ReadyToPublish
    }

    /// Number of blocking findings.
    pub fn blocker_count(&self) -> usize {
        self.findings.iter().filter(|f| f.is_blocker()).count()
    }

    /// Number of warning findings.
    pub fn warning_count(&self) -> usize {
        self.findings.iter().filter(|f| !f.is_blocker()).count()
    }

    /// Whether this family is published as a local-only artifact.
    pub fn is_local_only(&self) -> bool {
        self.published_trust_posture == TrustPosture::UnsignedLocalOnly
    }

    /// Whether the family carries its own non-empty author-lane refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.workspace_ref.trim().is_empty()
            && !self.sideload_review_ref.trim().is_empty()
            && !self.inspector_ref.trim().is_empty()
            && !self.publish_preview_ref.trim().is_empty()
            && !self.anti_abuse_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published trust posture, readiness, and findings all agree
    /// with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_trust_posture == self.effective_trust_posture()
            && self.publish_readiness == self.computed_publish_readiness()
            && self.findings == self.computed_findings()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AuthorPublishSummary {
    /// Total family rows.
    pub total_families: usize,
    /// Number of marketed families claimed.
    pub family_count: usize,
    /// Families ready to publish.
    pub ready_to_publish_families: usize,
    /// Families publishable with warnings.
    pub publishable_with_warnings_families: usize,
    /// Families blocked from publish.
    pub blocked_from_publish_families: usize,
    /// Families withheld as quarantined.
    pub withheld_quarantined_families: usize,
    /// Families carrying at least one blocker.
    pub families_with_blockers: usize,
    /// Families carrying at least one warning.
    pub families_with_warnings: usize,
    /// Families published as local-only (no inherited trust badge).
    pub local_only_published_families: usize,
    /// Families published with a verified-publisher or enterprise-approved badge.
    pub verified_or_enterprise_published_families: usize,
    /// Families disclosing a publisher-loss history.
    pub publisher_loss_history_families: usize,
    /// Families quarantined for anti-abuse review.
    pub quarantined_families: usize,
}

/// A redaction-safe export row projected from an author family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorPublishExportRow {
    /// Author-row id.
    pub family_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Runtime-class token.
    pub runtime_class: String,
    /// Host/ABI token.
    pub host_abi: String,
    /// Workspace-state token.
    pub workspace_state: String,
    /// Signing-state token.
    pub signature_state: String,
    /// Declared trust-posture token.
    pub declared_trust_posture: String,
    /// Published trust-posture token.
    pub published_trust_posture: String,
    /// Hot-reload-posture token.
    pub hot_reload_posture: String,
    /// Publish-review-requirement token.
    pub publish_review_requirement: String,
    /// Conformance-output token.
    pub conformance_output: String,
    /// Anti-abuse-transparency token.
    pub anti_abuse_transparency: String,
    /// Publish-readiness token.
    pub publish_readiness: String,
    /// Number of blocking findings.
    pub blocker_count: usize,
    /// Number of warning findings.
    pub warning_count: usize,
    /// Finding-code tokens, in canonical order.
    pub finding_codes: Vec<String>,
    /// Whether the family is ready to publish with no findings.
    pub publish_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorPublishExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub families: Vec<M5AuthorPublishExportRow>,
    /// Whether every family's stored decision agrees with the gate.
    pub all_families_gate_consistent: bool,
    /// Families ready to publish.
    pub ready_count: usize,
    /// Families blocked or withheld.
    pub blocked_or_withheld_count: usize,
    /// Families published as local-only.
    pub local_only_count: usize,
}

/// The typed M5 author-and-publish-preview matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AuthorPublishMatrix {
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
    /// Marketed families the packet claims; one row per family.
    pub artifact_families: Vec<ArtifactFamily>,
    /// Closed runtime-class vocabulary.
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary.
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed workspace-state vocabulary.
    pub workspace_states: Vec<WorkspaceState>,
    /// Closed signing-state vocabulary.
    pub signature_states: Vec<SignatureState>,
    /// Closed trust-posture vocabulary.
    pub trust_postures: Vec<TrustPosture>,
    /// Closed hot-reload-posture vocabulary.
    pub hot_reload_postures: Vec<HotReloadPosture>,
    /// Closed publish-review-requirement vocabulary.
    pub publish_review_requirements: Vec<PublishReviewRequirement>,
    /// Closed conformance-output vocabulary.
    pub conformance_outputs: Vec<ConformanceOutput>,
    /// Closed anti-abuse-transparency vocabulary.
    pub anti_abuse_transparency_states: Vec<AntiAbuseTransparency>,
    /// Closed finding-severity vocabulary.
    pub finding_severities: Vec<FindingSeverity>,
    /// Closed finding-domain vocabulary.
    pub finding_domains: Vec<FindingDomain>,
    /// Closed finding-code vocabulary.
    pub finding_codes: Vec<PublishFindingCode>,
    /// Closed publish-readiness vocabulary.
    pub publish_readiness_states: Vec<PublishReadiness>,
    /// Author rows, one per marketed family.
    #[serde(default)]
    pub families: Vec<AuthorFamilyRow>,
    /// Summary counts.
    pub summary: M5AuthorPublishSummary,
}

impl M5AuthorPublishMatrix {
    /// Returns the row for a marketed family.
    pub fn family(&self, family: ArtifactFamily) -> Option<&AuthorFamilyRow> {
        self.families.iter().find(|f| f.artifact_family == family)
    }

    /// Families ready to publish.
    pub fn ready_families(&self) -> impl Iterator<Item = &AuthorFamilyRow> {
        self.families.iter().filter(|f| f.is_ready_to_publish())
    }

    /// Families blocked from publish or withheld.
    pub fn blocked_or_withheld_families(&self) -> impl Iterator<Item = &AuthorFamilyRow> {
        self.families.iter().filter(|f| {
            matches!(
                f.computed_publish_readiness(),
                PublishReadiness::BlockedFromPublish | PublishReadiness::WithheldQuarantined
            )
        })
    }

    /// Families published as local-only.
    pub fn local_only_families(&self) -> impl Iterator<Item = &AuthorFamilyRow> {
        self.families.iter().filter(|f| f.is_local_only())
    }

    /// Whether every family's stored decision agrees with the recomputed gate.
    pub fn all_families_gate_consistent(&self) -> bool {
        self.families.iter().all(|f| f.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5AuthorPublishSummary {
        let count_readiness = |readiness: PublishReadiness| {
            self.families
                .iter()
                .filter(|f| f.publish_readiness == readiness)
                .count()
        };
        M5AuthorPublishSummary {
            total_families: self.families.len(),
            family_count: self.artifact_families.len(),
            ready_to_publish_families: count_readiness(PublishReadiness::ReadyToPublish),
            publishable_with_warnings_families: count_readiness(
                PublishReadiness::PublishableWithWarnings,
            ),
            blocked_from_publish_families: count_readiness(PublishReadiness::BlockedFromPublish),
            withheld_quarantined_families: count_readiness(PublishReadiness::WithheldQuarantined),
            families_with_blockers: self
                .families
                .iter()
                .filter(|f| f.blocker_count() > 0)
                .count(),
            families_with_warnings: self
                .families
                .iter()
                .filter(|f| f.warning_count() > 0)
                .count(),
            local_only_published_families: self.local_only_families().count(),
            verified_or_enterprise_published_families: self
                .families
                .iter()
                .filter(|f| f.published_trust_posture.is_trusted_badge())
                .count(),
            publisher_loss_history_families: self
                .families
                .iter()
                .filter(|f| {
                    f.anti_abuse_transparency
                        == AntiAbuseTransparency::PublisherLossHistoryDisclosed
                })
                .count(),
            quarantined_families: self
                .families
                .iter()
                .filter(|f| f.anti_abuse_transparency.is_quarantined())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — local authoring
    /// surfaces, package install/update flows, diagnostics, and certification
    /// packets — render instead of restating author/publish status text by hand.
    pub fn export_projection(&self) -> M5AuthorPublishExportProjection {
        let families = self
            .families
            .iter()
            .map(|f| M5AuthorPublishExportRow {
                family_id: f.family_id.clone(),
                artifact_family: f.artifact_family.as_str().to_owned(),
                runtime_class: f.runtime_class.as_str().to_owned(),
                host_abi: f.host_abi.as_str().to_owned(),
                workspace_state: f.workspace_state.as_str().to_owned(),
                signature_state: f.signature_state.as_str().to_owned(),
                declared_trust_posture: f.declared_trust_posture.as_str().to_owned(),
                published_trust_posture: f.published_trust_posture.as_str().to_owned(),
                hot_reload_posture: f.hot_reload_posture.as_str().to_owned(),
                publish_review_requirement: f.publish_review_requirement.as_str().to_owned(),
                conformance_output: f.conformance_output.as_str().to_owned(),
                anti_abuse_transparency: f.anti_abuse_transparency.as_str().to_owned(),
                publish_readiness: f.publish_readiness.as_str().to_owned(),
                blocker_count: f.blocker_count(),
                warning_count: f.warning_count(),
                finding_codes: f
                    .findings
                    .iter()
                    .map(|finding| finding.code.as_str().to_owned())
                    .collect(),
                publish_ready: f.is_ready_to_publish(),
                summary: format!(
                    "{}: runtime {}, host {}, workspace {}, signing {}, declared {}, published {} ({}), hot-reload {}, review {}, conformance {}, anti-abuse {} [{} blockers, {} warnings]",
                    f.artifact_family.as_str(),
                    f.runtime_class.as_str(),
                    f.host_abi.as_str(),
                    f.workspace_state.as_str(),
                    f.signature_state.as_str(),
                    f.declared_trust_posture.as_str(),
                    f.published_trust_posture.as_str(),
                    f.publish_readiness.as_str(),
                    f.hot_reload_posture.as_str(),
                    f.publish_review_requirement.as_str(),
                    f.conformance_output.as_str(),
                    f.anti_abuse_transparency.as_str(),
                    f.blocker_count(),
                    f.warning_count()
                ),
            })
            .collect();
        M5AuthorPublishExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            families,
            all_families_gate_consistent: self.all_families_gate_consistent(),
            ready_count: self.ready_families().count(),
            blocked_or_withheld_count: self.blocked_or_withheld_families().count(),
            local_only_count: self.local_only_families().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5AuthorPublishViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.families {
            if !seen_ids.insert(row.family_id.clone()) {
                violations.push(M5AuthorPublishViolation::DuplicateFamilyId {
                    family_id: row.family_id.clone(),
                });
            }
            if !seen_families.insert(row.artifact_family) {
                violations.push(M5AuthorPublishViolation::DuplicateFamilyRow {
                    family: row.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&row.artifact_family) {
                violations.push(M5AuthorPublishViolation::UnclaimedFamilyRow {
                    family_id: row.family_id.clone(),
                    family: row.artifact_family.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed family must carry its own row, so a family never inherits
        // an author-lane posture from an adjacent one.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5AuthorPublishViolation::MissingFamilyRow {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5AuthorPublishViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5AuthorPublishViolation>) {
        if self.schema_version != M5_AUTHOR_PUBLISH_SCHEMA_VERSION {
            violations.push(M5AuthorPublishViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_AUTHOR_PUBLISH_RECORD_KIND {
            violations.push(M5AuthorPublishViolation::UnsupportedRecordKind {
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
                violations.push(M5AuthorPublishViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_families",
                self.artifact_families == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "host_abi_classes",
                self.host_abi_classes == HostAbiClass::ALL.to_vec(),
            ),
            (
                "workspace_states",
                self.workspace_states == WorkspaceState::ALL.to_vec(),
            ),
            (
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "hot_reload_postures",
                self.hot_reload_postures == HotReloadPosture::ALL.to_vec(),
            ),
            (
                "publish_review_requirements",
                self.publish_review_requirements == PublishReviewRequirement::ALL.to_vec(),
            ),
            (
                "conformance_outputs",
                self.conformance_outputs == ConformanceOutput::ALL.to_vec(),
            ),
            (
                "anti_abuse_transparency_states",
                self.anti_abuse_transparency_states == AntiAbuseTransparency::ALL.to_vec(),
            ),
            (
                "finding_severities",
                self.finding_severities == FindingSeverity::ALL.to_vec(),
            ),
            (
                "finding_domains",
                self.finding_domains == FindingDomain::ALL.to_vec(),
            ),
            (
                "finding_codes",
                self.finding_codes == PublishFindingCode::ALL.to_vec(),
            ),
            (
                "publish_readiness_states",
                self.publish_readiness_states == PublishReadiness::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5AuthorPublishViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(&self, row: &AuthorFamilyRow, violations: &mut Vec<M5AuthorPublishViolation>) {
        for (field, value) in [
            ("family_id", &row.family_id),
            ("workspace_ref", &row.workspace_ref),
            ("sideload_review_ref", &row.sideload_review_ref),
            ("inspector_ref", &row.inspector_ref),
            ("publish_preview_ref", &row.publish_preview_ref),
            ("anti_abuse_ref", &row.anti_abuse_ref),
            ("conformance_ref", &row.conformance_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AuthorPublishViolation::EmptyField {
                    id: row.family_id.clone(),
                    field_name: field,
                });
            }
        }

        // Each stored finding's severity and domain must agree with its code, so a
        // blocker can never be relabeled as a warning by hand.
        for finding in &row.findings {
            if finding.severity != finding.code.severity()
                || finding.domain != finding.code.domain()
            {
                violations.push(M5AuthorPublishViolation::FindingSeverityMismatch {
                    family_id: row.family_id.clone(),
                    code: finding.code.as_str(),
                });
            }
        }

        // The published trust posture must equal the gate's recomputed posture, so a
        // family can never publish a stronger badge than its signing state supports.
        let effective = row.effective_trust_posture();
        if row.published_trust_posture != effective {
            violations.push(M5AuthorPublishViolation::OverstatedTrustPosture {
                family_id: row.family_id.clone(),
                published: row.published_trust_posture.as_str(),
                computed: effective.as_str(),
            });
        }

        // Non-inheritance: a local-dev, side-loaded, or revoked artifact must publish
        // as local-only and may never inherit a verified/enterprise badge.
        if row.signature_state.is_local_or_untrusted()
            && row.published_trust_posture != TrustPosture::UnsignedLocalOnly
        {
            violations.push(M5AuthorPublishViolation::LocalArtifactInheritedTrust {
                family_id: row.family_id.clone(),
                signature_state: row.signature_state.as_str(),
                published: row.published_trust_posture.as_str(),
            });
        }

        // The recorded readiness must match the recomputed verdict.
        let required = row.computed_publish_readiness();
        if row.publish_readiness != required {
            violations.push(M5AuthorPublishViolation::ReadinessMismatch {
                family_id: row.family_id.clone(),
                declared: row.publish_readiness.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded findings must equal the findings recomputed from the observed
        // states, in canonical order, so a blocker or warning can never be asserted
        // or hidden by hand.
        let computed = row.computed_findings();
        if row.findings != computed {
            violations.push(M5AuthorPublishViolation::FindingsMismatch {
                family_id: row.family_id.clone(),
            });
        }

        // A ready-to-publish family must be genuinely clean: built source, a verified
        // signature, no hot-reload widening, a passing conformance output, a clean
        // anti-abuse posture, and no findings.
        if row.is_ready_to_publish()
            && (row.workspace_state != WorkspaceState::SourcePresentBuilt
                || row.signature_state != SignatureState::SignedVerified
                || !matches!(
                    row.hot_reload_posture,
                    HotReloadPosture::NoWidening | HotReloadPosture::RelaunchOnly
                )
                || !matches!(
                    row.conformance_output,
                    ConformanceOutput::Conformant | ConformanceOutput::BridgeConformant
                )
                || row.anti_abuse_transparency != AntiAbuseTransparency::DisclosedClean
                || !row.findings.is_empty())
        {
            violations.push(M5AuthorPublishViolation::ReadyFamilyNotClean {
                family_id: row.family_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 author-and-publish-preview packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AuthorPublishViolation {
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
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An author-row id appears more than once.
    DuplicateFamilyId {
        /// Duplicate row id.
        family_id: String,
    },
    /// A marketed family carries more than one row.
    DuplicateFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A claimed marketed family has no row.
    MissingFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A row covers a family the packet does not claim.
    UnclaimedFamilyRow {
        /// Row id.
        family_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A finding's severity or domain disagrees with its code.
    FindingSeverityMismatch {
        /// Row id.
        family_id: String,
        /// Finding code token.
        code: &'static str,
    },
    /// A family publishes a trust posture beyond what its signing state supports.
    OverstatedTrustPosture {
        /// Row id.
        family_id: String,
        /// Published trust-posture token.
        published: &'static str,
        /// Computed effective trust-posture token.
        computed: &'static str,
    },
    /// A local/side-loaded/revoked artifact inherited a trusted publisher badge.
    LocalArtifactInheritedTrust {
        /// Row id.
        family_id: String,
        /// Signing-state token.
        signature_state: &'static str,
        /// Published trust-posture token.
        published: &'static str,
    },
    /// A family's readiness disagrees with the recomputed verdict.
    ReadinessMismatch {
        /// Row id.
        family_id: String,
        /// Declared readiness token.
        declared: &'static str,
        /// Required readiness token.
        required: &'static str,
    },
    /// A family's findings disagree with the recomputed findings.
    FindingsMismatch {
        /// Row id.
        family_id: String,
    },
    /// A ready-to-publish family still carries a finding or a non-clean state.
    ReadyFamilyNotClean {
        /// Row id.
        family_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5AuthorPublishViolation {
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
            Self::DuplicateFamilyId { family_id } => {
                write!(f, "duplicate family id {family_id}")
            }
            Self::DuplicateFamilyRow { family } => {
                write!(f, "duplicate row for family {family}")
            }
            Self::MissingFamilyRow { family } => {
                write!(f, "missing row for claimed family {family}")
            }
            Self::UnclaimedFamilyRow { family_id, family } => {
                write!(f, "row {family_id} covers unclaimed family {family}")
            }
            Self::FindingSeverityMismatch { family_id, code } => {
                write!(
                    f,
                    "row {family_id} finding {code} carries a non-canonical severity or domain"
                )
            }
            Self::OverstatedTrustPosture {
                family_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {family_id} publishes trust posture {published} but the gate computes {computed}"
                )
            }
            Self::LocalArtifactInheritedTrust {
                family_id,
                signature_state,
                published,
            } => {
                write!(
                    f,
                    "row {family_id} is {signature_state} but publishes {published}; local artifacts must publish unsigned_local_only"
                )
            }
            Self::ReadinessMismatch {
                family_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {family_id} records readiness {declared} but the gate requires {required}"
                )
            }
            Self::FindingsMismatch { family_id } => {
                write!(f, "row {family_id} findings disagree with the gate")
            }
            Self::ReadyFamilyNotClean { family_id } => {
                write!(
                    f,
                    "row {family_id} is ready to publish but carries a finding or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5AuthorPublishViolation {}

/// Loads the embedded M5 author-and-publish-preview matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5AuthorPublishMatrix`].
pub fn current_m5_author_publish_matrix() -> Result<M5AuthorPublishMatrix, serde_json::Error> {
    serde_json::from_str(M5_AUTHOR_PUBLISH_JSON)
}

#[cfg(test)]
mod tests;
