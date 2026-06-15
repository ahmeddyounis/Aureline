//! Canonical M5 publish-preview sheets — the reviewed publish action an author
//! drives before a package reaches the public registry.
//!
//! Where [`crate::m5_author_and_publish_preview`] freezes the whole author lane as one
//! matrix and carries a single `publish_preview_ref` per family, this module materializes
//! that reference into a first-class **publish-preview sheet** per marketed M5 artifact
//! family. Each [`PublishPreviewSheet`] makes the publish action reviewable rather than a
//! one-click upload: it names the manifest diff, the version bump, the signer and
//! namespace truth, the release channel, and the per-check state of every publish gate —
//! schema validation, the conformance kit, accessibility and performance smoke, docs
//! completeness, template/sample completeness, and registry policy — and then recomputes:
//!
//! - the **effective trust posture** the sheet may publish — capped by *both* the signing
//!   state *and* the namespace state, so a locally-built artifact, an unclaimed or
//!   mismatched namespace, or a revoked signature never inherits a verified-publisher or
//!   enterprise-approved badge just because it was built on a trusted machine;
//! - an explicit, source-tagged set of [`PublishPreviewFinding`]s split into **blockers**
//!   that hard-stop publication and **warnings** that publish with disclosure, each
//!   naming which gate it came from via [`FindingSource`] so the sheet stays a real review
//!   with registry-policy consequences instead of collapsing to a manifest linter; and
//! - a [`PublishReadiness`] verdict that withholds a quarantined family, blocks a family
//!   carrying any blocker, publishes-with-warnings a family carrying only warnings, and
//!   clears a genuinely clean family.
//!
//! The sheet keeps the structural publish facts honest. A manifest delta that widens
//! permissions, the runtime class, or an external executable requires a fresh review
//! ([`PublishPreviewSheet::widening_reviewed`]) before it can publish, and a hot reload
//! that would widen authority is gated the same way, so widening never reaches the
//! registry through a hot reload alone. The version bump must be at least as large as the
//! largest [`ChangeImpact`] in the diff, and a downgrade or an invalid version is a
//! blocker. The release channel carries its own consequences: a channel that
//! [`ReleaseChannel::requires_signed_release`] blocks an unsigned local-only artifact, and
//! one that [`ReleaseChannel::requires_clean_release`] blocks a release that still carries
//! warnings.
//!
//! [`M5PublishPreviewSheetSet::validate`] recomputes the published trust posture, the
//! readiness, and the full finding set from each sheet's facts and rejects any drift, and
//! [`M5PublishPreviewSheetSet::cross_check_matrix`] proves no sheet publishes a stronger
//! badge than the author-lane publish gate would grant the same family, so the publish
//! preview, the authoring chrome, install/update flows, diagnostics, support exports, and
//! release packets all project one publish truth.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-publish-preview.json` and
//! embedded here, so this typed consumer and any CI gate agree on every sheet without a
//! cargo build in CI. The model is metadata-only: every field is a typed state or an
//! opaque ref. It carries no credential bodies, raw provider payloads, signing secrets, or
//! manifest bodies — the `manifest_diff_ref`, `signer_identity_ref`, and `namespace_ref`
//! are opaque refs, never verbatim manifests or key material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_author_and_publish_preview::{
    AntiAbuseTransparency, ArtifactFamily, FindingSeverity, HostAbiClass, HotReloadPosture,
    M5AuthorPublishMatrix, PublishReadiness, RuntimeClass, SignatureState, TrustPosture,
};
pub use crate::m5_workspace_strip::hot_reload_widens_authority;

/// Supported M5 publish-preview sheet-set schema version.
pub const M5_PUBLISH_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_PUBLISH_PREVIEW_RECORD_KIND: &str = "m5_publish_preview_sheet_set";

/// Repo-relative path to the checked-in packet.
pub const M5_PUBLISH_PREVIEW_PATH: &str = "artifacts/ecosystem/m5/m5-publish-preview.json";

/// Embedded checked-in packet JSON.
pub const M5_PUBLISH_PREVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-publish-preview.json"
));

/// Release channel a sheet targets.
///
/// The channel carries its own publish consequences: a channel that
/// [`ReleaseChannel::requires_signed_release`] cannot accept an unsigned local-only
/// artifact, and one that [`ReleaseChannel::requires_clean_release`] cannot accept a
/// release that still carries warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannel {
    /// The default, stable public channel.
    Stable,
    /// The opt-in beta channel.
    Beta,
    /// The rolling edge channel.
    Edge,
    /// The early canary channel.
    Canary,
    /// An internal/enterprise-only channel.
    Internal,
}

impl ReleaseChannel {
    /// Every release channel, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Beta,
        Self::Edge,
        Self::Canary,
        Self::Internal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Edge => "edge",
            Self::Canary => "canary",
            Self::Internal => "internal",
        }
    }

    /// Whether this channel refuses an unsigned local-only release.
    pub const fn requires_signed_release(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }

    /// Whether this channel refuses a release that still carries warnings.
    pub const fn requires_clean_release(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Publisher namespace state backing a sheet's signer/namespace truth.
///
/// The namespace state contributes a trust ceiling alongside the signing state: a
/// mismatched or unclaimed namespace caps the rendered badge at
/// [`TrustPosture::UnsignedLocalOnly`], so a publish can never inherit a trusted badge
/// from a namespace the publisher does not own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceState {
    /// The publisher owns the namespace.
    PublisherOwned,
    /// The publisher owns a verified namespace.
    PublisherVerified,
    /// The namespace is managed by an enterprise/managed registry.
    EnterpriseManaged,
    /// Ownership of the namespace is mid-transfer.
    NamespaceTransferPending,
    /// The declared namespace does not match the signer.
    NamespaceMismatch,
    /// The namespace is not yet claimed by the publisher.
    NamespaceUnclaimed,
}

impl NamespaceState {
    /// Every namespace state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublisherOwned,
        Self::PublisherVerified,
        Self::EnterpriseManaged,
        Self::NamespaceTransferPending,
        Self::NamespaceMismatch,
        Self::NamespaceUnclaimed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublisherOwned => "publisher_owned",
            Self::PublisherVerified => "publisher_verified",
            Self::EnterpriseManaged => "enterprise_managed",
            Self::NamespaceTransferPending => "namespace_transfer_pending",
            Self::NamespaceMismatch => "namespace_mismatch",
            Self::NamespaceUnclaimed => "namespace_unclaimed",
        }
    }

    /// Highest trust posture this namespace state lets a sheet publish.
    pub const fn trust_ceiling(self) -> TrustPosture {
        match self {
            Self::EnterpriseManaged => TrustPosture::EnterpriseApproved,
            Self::PublisherVerified => TrustPosture::VerifiedPublisher,
            Self::PublisherOwned | Self::NamespaceTransferPending => TrustPosture::RegistryBound,
            Self::NamespaceMismatch | Self::NamespaceUnclaimed => TrustPosture::UnsignedLocalOnly,
        }
    }

    /// Whether this namespace state structurally caps the sheet to local-only.
    pub const fn caps_to_local_only(self) -> bool {
        matches!(self, Self::NamespaceMismatch | Self::NamespaceUnclaimed)
    }

    /// The finding this namespace state raises, if any.
    pub const fn finding(self) -> Option<FindingReason> {
        match self {
            Self::PublisherOwned | Self::PublisherVerified | Self::EnterpriseManaged => None,
            Self::NamespaceTransferPending => Some(FindingReason::NamespaceTransferPending),
            Self::NamespaceMismatch => Some(FindingReason::NamespaceMismatch),
            Self::NamespaceUnclaimed => Some(FindingReason::NamespaceUnclaimed),
        }
    }
}

/// Compatibility impact of a single manifest change.
///
/// The largest impact across a sheet's manifest diff sets the minimum version bump the
/// sheet must carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeImpact {
    /// No compatibility impact (metadata, signer, or namespace only).
    NoImpact,
    /// A backward-compatible fix.
    Fix,
    /// A backward-compatible feature addition.
    Feature,
    /// A backward-incompatible breaking change.
    Breaking,
}

impl ChangeImpact {
    /// Every change impact, in declaration order.
    pub const ALL: [Self; 4] = [Self::NoImpact, Self::Fix, Self::Feature, Self::Breaking];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpact => "no_impact",
            Self::Fix => "fix",
            Self::Feature => "feature",
            Self::Breaking => "breaking",
        }
    }

    /// Monotonic rank; higher means a larger compatibility impact.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NoImpact => 0,
            Self::Fix => 1,
            Self::Feature => 2,
            Self::Breaking => 3,
        }
    }

    /// The larger of two impacts.
    pub const fn max(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// Semantic version bump the sheet proposes between the current and proposed revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionBump {
    /// No version change.
    NoBump,
    /// A patch bump.
    Patch,
    /// A minor bump.
    Minor,
    /// A major bump.
    Major,
    /// The proposed version is lower than the current one.
    Downgrade,
    /// The proposed version is not a valid version.
    Invalid,
}

impl VersionBump {
    /// Every version bump, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoBump,
        Self::Patch,
        Self::Minor,
        Self::Major,
        Self::Downgrade,
        Self::Invalid,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBump => "no_bump",
            Self::Patch => "patch",
            Self::Minor => "minor",
            Self::Major => "major",
            Self::Downgrade => "downgrade",
            Self::Invalid => "invalid",
        }
    }

    /// Whether this bump is at least as large as the given change impact.
    ///
    /// A [`VersionBump::Downgrade`] or [`VersionBump::Invalid`] covers nothing.
    pub const fn covers_impact(self, impact: ChangeImpact) -> bool {
        match self {
            Self::Major => true,
            Self::Minor => matches!(
                impact,
                ChangeImpact::NoImpact | ChangeImpact::Fix | ChangeImpact::Feature
            ),
            Self::Patch => matches!(impact, ChangeImpact::NoImpact | ChangeImpact::Fix),
            Self::NoBump => matches!(impact, ChangeImpact::NoImpact),
            Self::Downgrade | Self::Invalid => false,
        }
    }

    /// The finding this bump raises for a given change impact, if any.
    pub const fn finding(self, impact: ChangeImpact) -> Option<FindingReason> {
        match self {
            Self::Invalid => Some(FindingReason::VersionInvalid),
            Self::Downgrade => Some(FindingReason::VersionDowngrade),
            _ if self.covers_impact(impact) => None,
            Self::NoBump => Some(FindingReason::VersionBumpMissing),
            _ => Some(FindingReason::VersionBumpUndersized),
        }
    }
}

/// Kind of one manifest change in a sheet's diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDeltaKind {
    /// A metadata field changed (description, tags, links).
    MetadataChanged,
    /// A dependency was updated.
    DependencyUpdated,
    /// A backward-compatible API addition.
    ApiCompatibleAddition,
    /// A backward-incompatible API change.
    ApiBreakingChange,
    /// A permission was added (widening).
    PermissionAdded,
    /// A permission was removed (narrowing).
    PermissionRemoved,
    /// The runtime class changed (widening).
    RuntimeClassChanged,
    /// An external executable was added (widening).
    ExternalExecutableAdded,
    /// The publisher namespace was rebound.
    NamespaceRebind,
    /// The signing identity was rotated.
    SignerRotated,
}

impl ManifestDeltaKind {
    /// Every manifest delta kind, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::MetadataChanged,
        Self::DependencyUpdated,
        Self::ApiCompatibleAddition,
        Self::ApiBreakingChange,
        Self::PermissionAdded,
        Self::PermissionRemoved,
        Self::RuntimeClassChanged,
        Self::ExternalExecutableAdded,
        Self::NamespaceRebind,
        Self::SignerRotated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataChanged => "metadata_changed",
            Self::DependencyUpdated => "dependency_updated",
            Self::ApiCompatibleAddition => "api_compatible_addition",
            Self::ApiBreakingChange => "api_breaking_change",
            Self::PermissionAdded => "permission_added",
            Self::PermissionRemoved => "permission_removed",
            Self::RuntimeClassChanged => "runtime_class_changed",
            Self::ExternalExecutableAdded => "external_executable_added",
            Self::NamespaceRebind => "namespace_rebind",
            Self::SignerRotated => "signer_rotated",
        }
    }

    /// Compatibility impact of this change.
    pub const fn change_impact(self) -> ChangeImpact {
        match self {
            Self::MetadataChanged
            | Self::PermissionRemoved
            | Self::NamespaceRebind
            | Self::SignerRotated => ChangeImpact::NoImpact,
            Self::DependencyUpdated => ChangeImpact::Fix,
            Self::ApiCompatibleAddition | Self::PermissionAdded => ChangeImpact::Feature,
            Self::ApiBreakingChange | Self::RuntimeClassChanged | Self::ExternalExecutableAdded => {
                ChangeImpact::Breaking
            }
        }
    }

    /// Whether this change widens authority and so requires a fresh review.
    pub const fn is_widening(self) -> bool {
        matches!(
            self,
            Self::PermissionAdded | Self::RuntimeClassChanged | Self::ExternalExecutableAdded
        )
    }

    /// Whether this change narrows authority (a disclosed reduction).
    pub const fn is_narrowing(self) -> bool {
        matches!(self, Self::PermissionRemoved)
    }
}

/// One manifest change in a sheet's diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestDelta {
    /// Display path of the changed manifest section (no secrets, no verbatim bodies).
    pub path: String,
    /// Kind of change.
    pub kind: ManifestDeltaKind,
    /// Short human-readable description of the change.
    pub detail: String,
}

/// A named publish gate the sheet reports a result for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishCheck {
    /// Schema validation of the manifest and artifacts.
    SchemaValidation,
    /// The conformance kit for the artifact family.
    ConformanceKit,
    /// Accessibility smoke checks.
    AccessibilitySmoke,
    /// Performance smoke checks.
    PerformanceSmoke,
    /// Docs completeness.
    DocsCompleteness,
    /// Template/sample completeness.
    TemplateSampleCompleteness,
    /// Registry policy.
    RegistryPolicy,
}

impl PublishCheck {
    /// Every publish check, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SchemaValidation,
        Self::ConformanceKit,
        Self::AccessibilitySmoke,
        Self::PerformanceSmoke,
        Self::DocsCompleteness,
        Self::TemplateSampleCompleteness,
        Self::RegistryPolicy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaValidation => "schema_validation",
            Self::ConformanceKit => "conformance_kit",
            Self::AccessibilitySmoke => "accessibility_smoke",
            Self::PerformanceSmoke => "performance_smoke",
            Self::DocsCompleteness => "docs_completeness",
            Self::TemplateSampleCompleteness => "template_sample_completeness",
            Self::RegistryPolicy => "registry_policy",
        }
    }

    /// The finding source this check maps to.
    pub const fn source(self) -> FindingSource {
        match self {
            Self::SchemaValidation => FindingSource::SchemaValidation,
            Self::ConformanceKit => FindingSource::ConformanceKit,
            Self::AccessibilitySmoke => FindingSource::AccessibilitySmoke,
            Self::PerformanceSmoke => FindingSource::PerformanceSmoke,
            Self::DocsCompleteness => FindingSource::DocsCompleteness,
            Self::TemplateSampleCompleteness => FindingSource::TemplateSampleCompleteness,
            Self::RegistryPolicy => FindingSource::RegistryPolicy,
        }
    }
}

/// Outcome of a named publish gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckOutcome {
    /// The check passed.
    Passed,
    /// The check passed with a disclosed warning.
    Warning,
    /// The check failed and blocks publication.
    Blocked,
    /// The check does not apply to this family.
    NotApplicable,
    /// The check has not been run.
    NotRun,
}

impl CheckOutcome {
    /// Every check outcome, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Passed,
        Self::Warning,
        Self::Blocked,
        Self::NotApplicable,
        Self::NotRun,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
            Self::NotRun => "not_run",
        }
    }

    /// The finding reason this outcome raises, if any.
    ///
    /// A required check that was never run is a blocker, so a publish can never skip a
    /// gate by leaving it unrun.
    pub const fn finding_reason(self) -> Option<FindingReason> {
        match self {
            Self::Passed | Self::NotApplicable => None,
            Self::Warning => Some(FindingReason::CheckWarning),
            Self::Blocked => Some(FindingReason::CheckFailed),
            Self::NotRun => Some(FindingReason::CheckNotRun),
        }
    }
}

/// One named publish gate's result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckResult {
    /// The gate this result is for.
    pub check: PublishCheck,
    /// The gate's outcome.
    pub outcome: CheckOutcome,
    /// Opaque ref to the gate's detail/evidence.
    pub detail_ref: String,
}

/// Source gate a publish finding came from.
///
/// Naming the source keeps the publish preview a real review: a reviewer can see whether
/// a blocker came from schema validation, the conformance kit, the accessibility or
/// performance smoke, docs completeness, template/sample completeness, or registry policy,
/// versus the structural manifest/version/signer/namespace/channel facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSource {
    /// Schema validation.
    SchemaValidation,
    /// The conformance kit.
    ConformanceKit,
    /// Accessibility smoke.
    AccessibilitySmoke,
    /// Performance smoke.
    PerformanceSmoke,
    /// Docs completeness.
    DocsCompleteness,
    /// Template/sample completeness.
    TemplateSampleCompleteness,
    /// Registry policy.
    RegistryPolicy,
    /// The manifest diff.
    ManifestDiff,
    /// The version bump.
    VersionBump,
    /// The signing identity.
    SignerIdentity,
    /// The publisher namespace.
    Namespace,
    /// The release-channel selection.
    ChannelSelection,
    /// The hot-reload review.
    HotReloadReview,
    /// Anti-abuse transparency.
    AntiAbuse,
}

impl FindingSource {
    /// Every finding source, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::SchemaValidation,
        Self::ConformanceKit,
        Self::AccessibilitySmoke,
        Self::PerformanceSmoke,
        Self::DocsCompleteness,
        Self::TemplateSampleCompleteness,
        Self::RegistryPolicy,
        Self::ManifestDiff,
        Self::VersionBump,
        Self::SignerIdentity,
        Self::Namespace,
        Self::ChannelSelection,
        Self::HotReloadReview,
        Self::AntiAbuse,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaValidation => "schema_validation",
            Self::ConformanceKit => "conformance_kit",
            Self::AccessibilitySmoke => "accessibility_smoke",
            Self::PerformanceSmoke => "performance_smoke",
            Self::DocsCompleteness => "docs_completeness",
            Self::TemplateSampleCompleteness => "template_sample_completeness",
            Self::RegistryPolicy => "registry_policy",
            Self::ManifestDiff => "manifest_diff",
            Self::VersionBump => "version_bump",
            Self::SignerIdentity => "signer_identity",
            Self::Namespace => "namespace",
            Self::ChannelSelection => "channel_selection",
            Self::HotReloadReview => "hot_reload_review",
            Self::AntiAbuse => "anti_abuse",
        }
    }

    /// Canonical rank used to order findings deterministically.
    pub const fn rank(self) -> u8 {
        match self {
            Self::SchemaValidation => 0,
            Self::ConformanceKit => 1,
            Self::AccessibilitySmoke => 2,
            Self::PerformanceSmoke => 3,
            Self::DocsCompleteness => 4,
            Self::TemplateSampleCompleteness => 5,
            Self::RegistryPolicy => 6,
            Self::ManifestDiff => 7,
            Self::VersionBump => 8,
            Self::SignerIdentity => 9,
            Self::Namespace => 10,
            Self::ChannelSelection => 11,
            Self::HotReloadReview => 12,
            Self::AntiAbuse => 13,
        }
    }

    /// Whether this source is one of the seven named publish-gate checks.
    pub const fn is_named_check(self) -> bool {
        matches!(
            self,
            Self::SchemaValidation
                | Self::ConformanceKit
                | Self::AccessibilitySmoke
                | Self::PerformanceSmoke
                | Self::DocsCompleteness
                | Self::TemplateSampleCompleteness
                | Self::RegistryPolicy
        )
    }
}

/// Reason a publish finding was raised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingReason {
    /// A named check failed.
    CheckFailed,
    /// A named check passed with a disclosed warning.
    CheckWarning,
    /// A required named check was not run.
    CheckNotRun,
    /// The manifest changed but the version was not bumped.
    VersionBumpMissing,
    /// The version bump is smaller than the change impact requires.
    VersionBumpUndersized,
    /// The proposed version is a downgrade.
    VersionDowngrade,
    /// The proposed version is not valid.
    VersionInvalid,
    /// A widening manifest change has not been reviewed.
    ManifestWideningUnreviewed,
    /// A permission was narrowed (a disclosed reduction).
    ManifestPermissionNarrowed,
    /// A widening hot reload has not been reviewed.
    HotReloadWideningUnreviewed,
    /// The signing provenance is unverified (signed-unverified or unsigned).
    ProvenanceUnverified,
    /// The signature is revoked.
    SignatureRevoked,
    /// The declared namespace does not match the signer.
    NamespaceMismatch,
    /// The namespace is not claimed by the publisher.
    NamespaceUnclaimed,
    /// Ownership of the namespace is mid-transfer.
    NamespaceTransferPending,
    /// The channel refuses an unsigned local-only release.
    ChannelRequiresSignedRelease,
    /// The channel refuses a release that still carries warnings.
    ChannelRequiresCleanRelease,
    /// The family is quarantined.
    AntiAbuseQuarantined,
    /// The anti-abuse posture is undisclosed.
    AntiAbuseUndisclosed,
    /// A publisher-loss/transfer history is disclosed.
    AntiAbusePublisherLossHistory,
}

impl FindingReason {
    /// Every finding reason, in declaration order.
    pub const ALL: [Self; 20] = [
        Self::CheckFailed,
        Self::CheckWarning,
        Self::CheckNotRun,
        Self::VersionBumpMissing,
        Self::VersionBumpUndersized,
        Self::VersionDowngrade,
        Self::VersionInvalid,
        Self::ManifestWideningUnreviewed,
        Self::ManifestPermissionNarrowed,
        Self::HotReloadWideningUnreviewed,
        Self::ProvenanceUnverified,
        Self::SignatureRevoked,
        Self::NamespaceMismatch,
        Self::NamespaceUnclaimed,
        Self::NamespaceTransferPending,
        Self::ChannelRequiresSignedRelease,
        Self::ChannelRequiresCleanRelease,
        Self::AntiAbuseQuarantined,
        Self::AntiAbuseUndisclosed,
        Self::AntiAbusePublisherLossHistory,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckFailed => "check_failed",
            Self::CheckWarning => "check_warning",
            Self::CheckNotRun => "check_not_run",
            Self::VersionBumpMissing => "version_bump_missing",
            Self::VersionBumpUndersized => "version_bump_undersized",
            Self::VersionDowngrade => "version_downgrade",
            Self::VersionInvalid => "version_invalid",
            Self::ManifestWideningUnreviewed => "manifest_widening_unreviewed",
            Self::ManifestPermissionNarrowed => "manifest_permission_narrowed",
            Self::HotReloadWideningUnreviewed => "hot_reload_widening_unreviewed",
            Self::ProvenanceUnverified => "provenance_unverified",
            Self::SignatureRevoked => "signature_revoked",
            Self::NamespaceMismatch => "namespace_mismatch",
            Self::NamespaceUnclaimed => "namespace_unclaimed",
            Self::NamespaceTransferPending => "namespace_transfer_pending",
            Self::ChannelRequiresSignedRelease => "channel_requires_signed_release",
            Self::ChannelRequiresCleanRelease => "channel_requires_clean_release",
            Self::AntiAbuseQuarantined => "anti_abuse_quarantined",
            Self::AntiAbuseUndisclosed => "anti_abuse_undisclosed",
            Self::AntiAbusePublisherLossHistory => "anti_abuse_publisher_loss_history",
        }
    }

    /// Canonical rank used to order findings deterministically within a source.
    pub const fn rank(self) -> u8 {
        match self {
            Self::CheckFailed => 0,
            Self::CheckWarning => 1,
            Self::CheckNotRun => 2,
            Self::VersionBumpMissing => 3,
            Self::VersionBumpUndersized => 4,
            Self::VersionDowngrade => 5,
            Self::VersionInvalid => 6,
            Self::ManifestWideningUnreviewed => 7,
            Self::ManifestPermissionNarrowed => 8,
            Self::HotReloadWideningUnreviewed => 9,
            Self::ProvenanceUnverified => 10,
            Self::SignatureRevoked => 11,
            Self::NamespaceMismatch => 12,
            Self::NamespaceUnclaimed => 13,
            Self::NamespaceTransferPending => 14,
            Self::ChannelRequiresSignedRelease => 15,
            Self::ChannelRequiresCleanRelease => 16,
            Self::AntiAbuseQuarantined => 17,
            Self::AntiAbuseUndisclosed => 18,
            Self::AntiAbusePublisherLossHistory => 19,
        }
    }

    /// Canonical severity of this reason.
    pub const fn severity(self) -> FindingSeverity {
        match self {
            Self::CheckWarning
            | Self::ManifestPermissionNarrowed
            | Self::ProvenanceUnverified
            | Self::NamespaceTransferPending
            | Self::AntiAbusePublisherLossHistory => FindingSeverity::Warning,
            Self::CheckFailed
            | Self::CheckNotRun
            | Self::VersionBumpMissing
            | Self::VersionBumpUndersized
            | Self::VersionDowngrade
            | Self::VersionInvalid
            | Self::ManifestWideningUnreviewed
            | Self::HotReloadWideningUnreviewed
            | Self::SignatureRevoked
            | Self::NamespaceMismatch
            | Self::NamespaceUnclaimed
            | Self::ChannelRequiresSignedRelease
            | Self::ChannelRequiresCleanRelease
            | Self::AntiAbuseQuarantined
            | Self::AntiAbuseUndisclosed => FindingSeverity::Blocker,
        }
    }

    /// Whether the given source is valid for this reason.
    ///
    /// Check-derived reasons accept any of the seven named-check sources; every other
    /// reason pins exactly one structural source.
    pub fn source_is_valid(self, source: FindingSource) -> bool {
        match self.fixed_source() {
            Some(expected) => source == expected,
            None => source.is_named_check(),
        }
    }

    /// The single structural source this reason pins, or `None` for check-derived reasons.
    pub const fn fixed_source(self) -> Option<FindingSource> {
        match self {
            Self::CheckFailed | Self::CheckWarning | Self::CheckNotRun => None,
            Self::VersionBumpMissing
            | Self::VersionBumpUndersized
            | Self::VersionDowngrade
            | Self::VersionInvalid => Some(FindingSource::VersionBump),
            Self::ManifestWideningUnreviewed | Self::ManifestPermissionNarrowed => {
                Some(FindingSource::ManifestDiff)
            }
            Self::HotReloadWideningUnreviewed => Some(FindingSource::HotReloadReview),
            Self::ProvenanceUnverified | Self::SignatureRevoked => {
                Some(FindingSource::SignerIdentity)
            }
            Self::NamespaceMismatch | Self::NamespaceUnclaimed | Self::NamespaceTransferPending => {
                Some(FindingSource::Namespace)
            }
            Self::ChannelRequiresSignedRelease | Self::ChannelRequiresCleanRelease => {
                Some(FindingSource::ChannelSelection)
            }
            Self::AntiAbuseQuarantined
            | Self::AntiAbuseUndisclosed
            | Self::AntiAbusePublisherLossHistory => Some(FindingSource::AntiAbuse),
        }
    }
}

/// A source- and severity-tagged publish finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishPreviewFinding {
    /// Source gate the finding came from.
    pub source: FindingSource,
    /// Reason the finding was raised.
    pub reason: FindingReason,
    /// Severity; must equal the reason's canonical severity.
    pub severity: FindingSeverity,
    /// Opaque ref to the finding's detail/evidence.
    pub detail_ref: String,
}

impl PublishPreviewFinding {
    /// Builds a finding from a source, reason, and detail ref, filling severity from the
    /// reason.
    pub fn of(source: FindingSource, reason: FindingReason, detail_ref: String) -> Self {
        Self {
            source,
            reason,
            severity: reason.severity(),
            detail_ref,
        }
    }

    /// Whether the finding is a blocker.
    pub const fn is_blocker(&self) -> bool {
        matches!(self.severity, FindingSeverity::Blocker)
    }

    /// Sort key used to order findings deterministically.
    fn order_key(&self) -> (u8, u8) {
        (self.source.rank(), self.reason.rank())
    }
}

/// The signing finding a signing state raises, if any.
const fn signer_finding(state: SignatureState) -> Option<FindingReason> {
    match state {
        SignatureState::SignedVerified => None,
        SignatureState::SignedUnverified
        | SignatureState::UnsignedLocalDev
        | SignatureState::UnsignedSideload => Some(FindingReason::ProvenanceUnverified),
        SignatureState::RevokedSignature => Some(FindingReason::SignatureRevoked),
    }
}

/// The anti-abuse finding a transparency state raises, if any.
const fn anti_abuse_finding(state: AntiAbuseTransparency) -> Option<FindingReason> {
    match state {
        AntiAbuseTransparency::DisclosedClean => None,
        AntiAbuseTransparency::PublisherLossHistoryDisclosed => {
            Some(FindingReason::AntiAbusePublisherLossHistory)
        }
        AntiAbuseTransparency::Undisclosed => Some(FindingReason::AntiAbuseUndisclosed),
        AntiAbuseTransparency::Quarantined => Some(FindingReason::AntiAbuseQuarantined),
    }
}

/// One publish-preview sheet for a marketed M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishPreviewSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Marketed M5 artifact family this sheet governs.
    pub artifact_family: ArtifactFamily,
    /// Author-facing package identity (display name/id; no secrets).
    pub package_identity: String,
    /// Current published version (display string).
    pub current_version: String,
    /// Proposed publish version (display string).
    pub proposed_version: String,
    /// Version bump between the current and proposed version.
    pub version_bump: VersionBump,
    /// Release channel the sheet targets.
    pub release_channel: ReleaseChannel,
    /// Runtime class of the authored artifact.
    pub runtime_class: RuntimeClass,
    /// Host/ABI execution locus.
    pub host_abi: HostAbiClass,
    /// Signing/provenance state.
    pub signature_state: SignatureState,
    /// Opaque ref to the signing identity (never key material).
    pub signer_identity_ref: String,
    /// Publisher namespace state.
    pub namespace_state: NamespaceState,
    /// Opaque ref to the publisher namespace.
    pub namespace_ref: String,
    /// Trust posture the author requests, before the sheet caps it.
    pub declared_trust_posture: TrustPosture,
    /// Trust posture the sheet actually publishes after capping.
    ///
    /// Must equal [`PublishPreviewSheet::effective_trust_posture`].
    pub published_trust_posture: TrustPosture,
    /// Hot-reload/relaunch posture carried from the author lane.
    pub hot_reload_posture: HotReloadPosture,
    /// Whether a fresh review has cleared the sheet's widening changes.
    pub widening_reviewed: bool,
    /// Anti-abuse transparency state.
    pub anti_abuse_transparency: AntiAbuseTransparency,
    /// The manifest diff: one entry per changed manifest section.
    #[serde(default)]
    pub manifest_deltas: Vec<ManifestDelta>,
    /// Named publish-gate results; one per [`PublishCheck`].
    #[serde(default)]
    pub checks: Vec<CheckResult>,
    /// Severity-tagged findings; must equal the recomputed set, in canonical order.
    #[serde(default)]
    pub findings: Vec<PublishPreviewFinding>,
    /// Publish verdict; must equal the recomputed readiness.
    pub publish_readiness: PublishReadiness,
    /// Opaque ref to the manifest diff.
    pub manifest_diff_ref: String,
    /// Ref to the family's author-lane publish gate row.
    pub publish_preview_ref: String,
    /// Ref binding this sheet into diagnostics and support surfaces.
    pub support_export_ref: String,
    /// Ref binding this sheet into the release packet that consumes it.
    pub release_packet_ref: String,
    /// Additional source refs backing the sheet.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl PublishPreviewSheet {
    /// The trust posture the sheet may publish for this family.
    ///
    /// Lowers the author's declared posture to the *minimum* of the signing-state ceiling
    /// and the namespace ceiling, so a locally-built, unsigned, or revoked artifact and an
    /// unclaimed or mismatched namespace can never inherit a verified-publisher or
    /// enterprise-approved badge.
    pub fn effective_trust_posture(&self) -> TrustPosture {
        self.declared_trust_posture
            .min(self.signature_state.trust_ceiling())
            .min(self.namespace_state.trust_ceiling())
    }

    /// Whether the sheet publishes as a local-only artifact (no inherited trust badge).
    pub fn is_local_only(&self) -> bool {
        self.effective_trust_posture() == TrustPosture::UnsignedLocalOnly
    }

    /// Whether the sheet carries a widening change that requires a fresh review.
    pub fn has_widening_change(&self) -> bool {
        self.manifest_deltas.iter().any(|d| d.kind.is_widening())
            || hot_reload_widens_authority(self.hot_reload_posture)
    }

    /// The largest change impact across the manifest diff.
    pub fn max_change_impact(&self) -> ChangeImpact {
        self.manifest_deltas
            .iter()
            .fold(ChangeImpact::NoImpact, |acc, delta| {
                acc.max(delta.kind.change_impact())
            })
    }

    /// Returns the result for a named publish check.
    pub fn check(&self, check: PublishCheck) -> Option<&CheckResult> {
        self.checks.iter().find(|c| c.check == check)
    }

    /// The findings recomputed from this sheet's observed facts, in canonical order.
    pub fn computed_findings(&self) -> Vec<PublishPreviewFinding> {
        let mut out: Vec<PublishPreviewFinding> = Vec::new();

        // 1. Named publish gates, in their declaration order.
        for check in PublishCheck::ALL {
            if let Some(result) = self.check(check) {
                if let Some(reason) = result.outcome.finding_reason() {
                    out.push(PublishPreviewFinding::of(
                        check.source(),
                        reason,
                        result.detail_ref.clone(),
                    ));
                }
            }
        }

        // 2. Version bump versus the largest change impact in the diff.
        if let Some(reason) = self.version_bump.finding(self.max_change_impact()) {
            out.push(PublishPreviewFinding::of(
                FindingSource::VersionBump,
                reason,
                self.manifest_diff_ref.clone(),
            ));
        }

        // 3. Manifest widening must be reviewed before it can publish.
        if self.manifest_deltas.iter().any(|d| d.kind.is_widening()) && !self.widening_reviewed {
            out.push(PublishPreviewFinding::of(
                FindingSource::ManifestDiff,
                FindingReason::ManifestWideningUnreviewed,
                self.manifest_diff_ref.clone(),
            ));
        }

        // 4. Manifest narrowing is disclosed as a warning.
        if self.manifest_deltas.iter().any(|d| d.kind.is_narrowing()) {
            out.push(PublishPreviewFinding::of(
                FindingSource::ManifestDiff,
                FindingReason::ManifestPermissionNarrowed,
                self.manifest_diff_ref.clone(),
            ));
        }

        // 5. A widening hot reload is gated the same way as a manifest widening.
        if hot_reload_widens_authority(self.hot_reload_posture) && !self.widening_reviewed {
            out.push(PublishPreviewFinding::of(
                FindingSource::HotReloadReview,
                FindingReason::HotReloadWideningUnreviewed,
                self.publish_preview_ref.clone(),
            ));
        }

        // 6. Signer truth.
        if let Some(reason) = signer_finding(self.signature_state) {
            out.push(PublishPreviewFinding::of(
                FindingSource::SignerIdentity,
                reason,
                self.signer_identity_ref.clone(),
            ));
        }

        // 7. Namespace truth.
        if let Some(reason) = self.namespace_state.finding() {
            out.push(PublishPreviewFinding::of(
                FindingSource::Namespace,
                reason,
                self.namespace_ref.clone(),
            ));
        }

        // 8. The channel refuses an unsigned local-only release.
        if self.release_channel.requires_signed_release()
            && self.effective_trust_posture().rank() < TrustPosture::RegistryBound.rank()
        {
            out.push(PublishPreviewFinding::of(
                FindingSource::ChannelSelection,
                FindingReason::ChannelRequiresSignedRelease,
                self.publish_preview_ref.clone(),
            ));
        }

        // 9. Anti-abuse transparency.
        if let Some(reason) = anti_abuse_finding(self.anti_abuse_transparency) {
            out.push(PublishPreviewFinding::of(
                FindingSource::AntiAbuse,
                reason,
                self.support_export_ref.clone(),
            ));
        }

        // 10. The channel refuses a release that still carries warnings.
        if self.release_channel.requires_clean_release() && out.iter().any(|f| !f.is_blocker()) {
            out.push(PublishPreviewFinding::of(
                FindingSource::ChannelSelection,
                FindingReason::ChannelRequiresCleanRelease,
                self.publish_preview_ref.clone(),
            ));
        }

        out.sort_by_key(PublishPreviewFinding::order_key);
        out.dedup();
        out
    }

    /// The publish verdict the sheet must record.
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

    /// Whether the sheet is ready to publish with no findings.
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

    /// Findings carried from a given source.
    pub fn findings_from(
        &self,
        source: FindingSource,
    ) -> impl Iterator<Item = &PublishPreviewFinding> {
        self.findings.iter().filter(move |f| f.source == source)
    }

    /// Whether the sheet carries a blocker from a given source.
    pub fn has_blocker_from(&self, source: FindingSource) -> bool {
        self.findings
            .iter()
            .any(|f| f.source == source && f.is_blocker())
    }

    /// Whether the sheet carries its own non-empty refs.
    pub fn has_required_refs(&self) -> bool {
        !self.signer_identity_ref.trim().is_empty()
            && !self.namespace_ref.trim().is_empty()
            && !self.manifest_diff_ref.trim().is_empty()
            && !self.publish_preview_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
            && !self.release_packet_ref.trim().is_empty()
    }

    /// Whether the stored published posture, readiness, and findings all agree with the
    /// recomputed gate decision.
    pub fn sheet_consistent(&self) -> bool {
        self.published_trust_posture == self.effective_trust_posture()
            && self.publish_readiness == self.computed_publish_readiness()
            && self.findings == self.computed_findings()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublishPreviewSummary {
    /// Total sheets.
    pub total_sheets: usize,
    /// Number of marketed families claimed.
    pub family_count: usize,
    /// Sheets ready to publish.
    pub ready_to_publish_sheets: usize,
    /// Sheets publishable with warnings.
    pub publishable_with_warnings_sheets: usize,
    /// Sheets blocked from publish.
    pub blocked_from_publish_sheets: usize,
    /// Sheets withheld as quarantined.
    pub withheld_quarantined_sheets: usize,
    /// Sheets carrying at least one blocker.
    pub sheets_with_blockers: usize,
    /// Sheets carrying at least one warning.
    pub sheets_with_warnings: usize,
    /// Sheets published as local-only (no inherited trust badge).
    pub local_only_published_sheets: usize,
    /// Sheets published with a verified-publisher or enterprise-approved badge.
    pub verified_or_enterprise_published_sheets: usize,
    /// Sheets carrying an unreviewed manifest widening.
    pub manifest_widening_unreviewed_sheets: usize,
    /// Sheets blocked by a namespace finding.
    pub namespace_blocked_sheets: usize,
    /// Sheets blocked by a channel finding.
    pub channel_blocked_sheets: usize,
    /// Sheets quarantined for anti-abuse review.
    pub quarantined_sheets: usize,
}

/// A redaction-safe export row projected from a publish-preview sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublishPreviewExportRow {
    /// Sheet id.
    pub sheet_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Author-facing package identity.
    pub package_identity: String,
    /// Current version.
    pub current_version: String,
    /// Proposed version.
    pub proposed_version: String,
    /// Version-bump token.
    pub version_bump: String,
    /// Release-channel token.
    pub release_channel: String,
    /// Signing-state token.
    pub signature_state: String,
    /// Namespace-state token.
    pub namespace_state: String,
    /// Published trust-posture token.
    pub published_trust_posture: String,
    /// Publish-readiness token.
    pub publish_readiness: String,
    /// Number of blocking findings.
    pub blocker_count: usize,
    /// Number of warning findings.
    pub warning_count: usize,
    /// Blocking finding source/reason tokens, in canonical order.
    pub blocker_sources: Vec<String>,
    /// Warning finding source/reason tokens, in canonical order.
    pub warning_sources: Vec<String>,
    /// Whether the sheet is ready to publish with no findings.
    pub publish_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublishPreviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected sheets.
    pub sheets: Vec<M5PublishPreviewExportRow>,
    /// Whether every sheet's stored decision agrees with the gate.
    pub all_sheets_consistent: bool,
    /// Sheets ready to publish.
    pub ready_count: usize,
    /// Sheets blocked or withheld.
    pub blocked_or_withheld_count: usize,
    /// Sheets published as local-only.
    pub local_only_count: usize,
}

/// The typed M5 publish-preview sheet-set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublishPreviewSheetSet {
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
    /// Marketed families the packet claims; one sheet per family.
    pub artifact_families: Vec<ArtifactFamily>,
    /// Closed runtime-class vocabulary.
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary.
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed signing-state vocabulary.
    pub signature_states: Vec<SignatureState>,
    /// Closed namespace-state vocabulary.
    pub namespace_states: Vec<NamespaceState>,
    /// Closed release-channel vocabulary.
    pub release_channels: Vec<ReleaseChannel>,
    /// Closed trust-posture vocabulary.
    pub trust_postures: Vec<TrustPosture>,
    /// Closed change-impact vocabulary.
    pub change_impacts: Vec<ChangeImpact>,
    /// Closed version-bump vocabulary.
    pub version_bumps: Vec<VersionBump>,
    /// Closed manifest-delta-kind vocabulary.
    pub manifest_delta_kinds: Vec<ManifestDeltaKind>,
    /// Closed publish-check vocabulary.
    pub publish_checks: Vec<PublishCheck>,
    /// Closed check-outcome vocabulary.
    pub check_outcomes: Vec<CheckOutcome>,
    /// Closed hot-reload-posture vocabulary.
    pub hot_reload_postures: Vec<HotReloadPosture>,
    /// Closed finding-source vocabulary.
    pub finding_sources: Vec<FindingSource>,
    /// Closed finding-reason vocabulary.
    pub finding_reasons: Vec<FindingReason>,
    /// Closed finding-severity vocabulary.
    pub finding_severities: Vec<FindingSeverity>,
    /// Closed anti-abuse-transparency vocabulary.
    pub anti_abuse_transparency_states: Vec<AntiAbuseTransparency>,
    /// Closed publish-readiness vocabulary.
    pub publish_readiness_states: Vec<PublishReadiness>,
    /// Sheets, one per marketed family.
    #[serde(default)]
    pub sheets: Vec<PublishPreviewSheet>,
    /// Summary counts.
    pub summary: M5PublishPreviewSummary,
}

impl M5PublishPreviewSheetSet {
    /// Returns the sheet for a marketed family.
    pub fn sheet(&self, family: ArtifactFamily) -> Option<&PublishPreviewSheet> {
        self.sheets.iter().find(|s| s.artifact_family == family)
    }

    /// Sheets ready to publish.
    pub fn ready_sheets(&self) -> impl Iterator<Item = &PublishPreviewSheet> {
        self.sheets.iter().filter(|s| s.is_ready_to_publish())
    }

    /// Sheets blocked from publish or withheld.
    pub fn blocked_or_withheld_sheets(&self) -> impl Iterator<Item = &PublishPreviewSheet> {
        self.sheets.iter().filter(|s| {
            matches!(
                s.computed_publish_readiness(),
                PublishReadiness::BlockedFromPublish | PublishReadiness::WithheldQuarantined
            )
        })
    }

    /// Sheets published as local-only.
    pub fn local_only_sheets(&self) -> impl Iterator<Item = &PublishPreviewSheet> {
        self.sheets.iter().filter(|s| s.is_local_only())
    }

    /// Whether every sheet's stored decision agrees with the recomputed gate.
    pub fn all_sheets_consistent(&self) -> bool {
        self.sheets.iter().all(|s| s.sheet_consistent())
    }

    /// Recomputes the summary block from the sheets.
    pub fn computed_summary(&self) -> M5PublishPreviewSummary {
        let count_readiness = |readiness: PublishReadiness| {
            self.sheets
                .iter()
                .filter(|s| s.publish_readiness == readiness)
                .count()
        };
        M5PublishPreviewSummary {
            total_sheets: self.sheets.len(),
            family_count: self.artifact_families.len(),
            ready_to_publish_sheets: count_readiness(PublishReadiness::ReadyToPublish),
            publishable_with_warnings_sheets: count_readiness(
                PublishReadiness::PublishableWithWarnings,
            ),
            blocked_from_publish_sheets: count_readiness(PublishReadiness::BlockedFromPublish),
            withheld_quarantined_sheets: count_readiness(PublishReadiness::WithheldQuarantined),
            sheets_with_blockers: self.sheets.iter().filter(|s| s.blocker_count() > 0).count(),
            sheets_with_warnings: self.sheets.iter().filter(|s| s.warning_count() > 0).count(),
            local_only_published_sheets: self.local_only_sheets().count(),
            verified_or_enterprise_published_sheets: self
                .sheets
                .iter()
                .filter(|s| s.published_trust_posture.is_trusted_badge())
                .count(),
            manifest_widening_unreviewed_sheets: self
                .sheets
                .iter()
                .filter(|s| {
                    s.findings
                        .iter()
                        .any(|f| f.reason == FindingReason::ManifestWideningUnreviewed)
                })
                .count(),
            namespace_blocked_sheets: self
                .sheets
                .iter()
                .filter(|s| s.has_blocker_from(FindingSource::Namespace))
                .count(),
            channel_blocked_sheets: self
                .sheets
                .iter()
                .filter(|s| s.has_blocker_from(FindingSource::ChannelSelection))
                .count(),
            quarantined_sheets: self
                .sheets
                .iter()
                .filter(|s| s.anti_abuse_transparency.is_quarantined())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — authoring chrome,
    /// install/update flows, diagnostics, support, and release packets — render instead of
    /// restating publish-preview status text by hand.
    pub fn export_projection(&self) -> M5PublishPreviewExportProjection {
        let sheets = self
            .sheets
            .iter()
            .map(|s| {
                let token = |f: &PublishPreviewFinding| {
                    format!("{}:{}", f.source.as_str(), f.reason.as_str())
                };
                M5PublishPreviewExportRow {
                    sheet_id: s.sheet_id.clone(),
                    artifact_family: s.artifact_family.as_str().to_owned(),
                    package_identity: s.package_identity.clone(),
                    current_version: s.current_version.clone(),
                    proposed_version: s.proposed_version.clone(),
                    version_bump: s.version_bump.as_str().to_owned(),
                    release_channel: s.release_channel.as_str().to_owned(),
                    signature_state: s.signature_state.as_str().to_owned(),
                    namespace_state: s.namespace_state.as_str().to_owned(),
                    published_trust_posture: s.published_trust_posture.as_str().to_owned(),
                    publish_readiness: s.publish_readiness.as_str().to_owned(),
                    blocker_count: s.blocker_count(),
                    warning_count: s.warning_count(),
                    blocker_sources: s
                        .findings
                        .iter()
                        .filter(|f| f.is_blocker())
                        .map(token)
                        .collect(),
                    warning_sources: s
                        .findings
                        .iter()
                        .filter(|f| !f.is_blocker())
                        .map(token)
                        .collect(),
                    publish_ready: s.is_ready_to_publish(),
                    summary: format!(
                        "{}: {} -> {} ({}), channel {}, signer {}, namespace {}, published {} [{}] {} blockers, {} warnings",
                        s.artifact_family.as_str(),
                        s.current_version,
                        s.proposed_version,
                        s.version_bump.as_str(),
                        s.release_channel.as_str(),
                        s.signature_state.as_str(),
                        s.namespace_state.as_str(),
                        s.published_trust_posture.as_str(),
                        s.publish_readiness.as_str(),
                        s.blocker_count(),
                        s.warning_count(),
                    ),
                }
            })
            .collect();
        M5PublishPreviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            sheets,
            all_sheets_consistent: self.all_sheets_consistent(),
            ready_count: self.ready_sheets().count(),
            blocked_or_withheld_count: self.blocked_or_withheld_sheets().count(),
            local_only_count: self.local_only_sheets().count(),
        }
    }

    /// Cross-checks the sheets against the author-lane publish gate.
    ///
    /// Proves no sheet publishes a *stronger* trust badge than the publish gate would grant
    /// the same family, so the publish preview and the author lane project one trust truth.
    pub fn cross_check_matrix(
        &self,
        matrix: &M5AuthorPublishMatrix,
    ) -> Vec<M5PublishPreviewViolation> {
        let mut violations = Vec::new();
        for sheet in &self.sheets {
            match matrix.family(sheet.artifact_family) {
                None => violations.push(M5PublishPreviewViolation::MissingMatrixRow {
                    sheet_id: sheet.sheet_id.clone(),
                    family: sheet.artifact_family.as_str(),
                }),
                Some(row) => {
                    if sheet.published_trust_posture.rank() > row.published_trust_posture.rank() {
                        violations.push(M5PublishPreviewViolation::SheetExceedsPublishGate {
                            sheet_id: sheet.sheet_id.clone(),
                            published: sheet.published_trust_posture.as_str(),
                            gate: row.published_trust_posture.as_str(),
                        });
                    }
                }
            }
        }
        violations
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5PublishPreviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for sheet in &self.sheets {
            if !seen_ids.insert(sheet.sheet_id.clone()) {
                violations.push(M5PublishPreviewViolation::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            if !seen_families.insert(sheet.artifact_family) {
                violations.push(M5PublishPreviewViolation::DuplicateFamilySheet {
                    family: sheet.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&sheet.artifact_family) {
                violations.push(M5PublishPreviewViolation::UnclaimedFamilySheet {
                    sheet_id: sheet.sheet_id.clone(),
                    family: sheet.artifact_family.as_str(),
                });
            }
            self.validate_sheet(sheet, &mut violations);
        }

        // Every claimed family must carry its own sheet, so a family never inherits a
        // publish decision from an adjacent one.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5PublishPreviewViolation::MissingFamilySheet {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5PublishPreviewViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5PublishPreviewViolation>) {
        if self.schema_version != M5_PUBLISH_PREVIEW_SCHEMA_VERSION {
            violations.push(M5PublishPreviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PUBLISH_PREVIEW_RECORD_KIND {
            violations.push(M5PublishPreviewViolation::UnsupportedRecordKind {
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
                violations.push(M5PublishPreviewViolation::EmptyField {
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
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "namespace_states",
                self.namespace_states == NamespaceState::ALL.to_vec(),
            ),
            (
                "release_channels",
                self.release_channels == ReleaseChannel::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "change_impacts",
                self.change_impacts == ChangeImpact::ALL.to_vec(),
            ),
            (
                "version_bumps",
                self.version_bumps == VersionBump::ALL.to_vec(),
            ),
            (
                "manifest_delta_kinds",
                self.manifest_delta_kinds == ManifestDeltaKind::ALL.to_vec(),
            ),
            (
                "publish_checks",
                self.publish_checks == PublishCheck::ALL.to_vec(),
            ),
            (
                "check_outcomes",
                self.check_outcomes == CheckOutcome::ALL.to_vec(),
            ),
            (
                "hot_reload_postures",
                self.hot_reload_postures == HotReloadPosture::ALL.to_vec(),
            ),
            (
                "finding_sources",
                self.finding_sources == FindingSource::ALL.to_vec(),
            ),
            (
                "finding_reasons",
                self.finding_reasons == FindingReason::ALL.to_vec(),
            ),
            (
                "finding_severities",
                self.finding_severities == FindingSeverity::ALL.to_vec(),
            ),
            (
                "anti_abuse_transparency_states",
                self.anti_abuse_transparency_states == AntiAbuseTransparency::ALL.to_vec(),
            ),
            (
                "publish_readiness_states",
                self.publish_readiness_states == PublishReadiness::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5PublishPreviewViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_sheet(
        &self,
        sheet: &PublishPreviewSheet,
        violations: &mut Vec<M5PublishPreviewViolation>,
    ) {
        for (field, value) in [
            ("sheet_id", &sheet.sheet_id),
            ("package_identity", &sheet.package_identity),
            ("current_version", &sheet.current_version),
            ("proposed_version", &sheet.proposed_version),
            ("signer_identity_ref", &sheet.signer_identity_ref),
            ("namespace_ref", &sheet.namespace_ref),
            ("manifest_diff_ref", &sheet.manifest_diff_ref),
            ("publish_preview_ref", &sheet.publish_preview_ref),
            ("support_export_ref", &sheet.support_export_ref),
            ("release_packet_ref", &sheet.release_packet_ref),
            ("note", &sheet.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublishPreviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every named publish gate must report exactly one result, so a publish can never
        // hide a gate by omitting it.
        let mut seen_checks = BTreeSet::new();
        for result in &sheet.checks {
            if result.detail_ref.trim().is_empty() {
                violations.push(M5PublishPreviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: "checks.detail_ref",
                });
            }
            if !seen_checks.insert(result.check) {
                violations.push(M5PublishPreviewViolation::DuplicateCheck {
                    sheet_id: sheet.sheet_id.clone(),
                    check: result.check.as_str(),
                });
            }
        }
        for check in PublishCheck::ALL {
            if !seen_checks.contains(&check) {
                violations.push(M5PublishPreviewViolation::MissingCheck {
                    sheet_id: sheet.sheet_id.clone(),
                    check: check.as_str(),
                });
            }
        }

        for delta in &sheet.manifest_deltas {
            if delta.path.trim().is_empty() || delta.detail.trim().is_empty() {
                violations.push(M5PublishPreviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: "manifest_deltas",
                });
            }
        }
        for evidence in &sheet.evidence_refs {
            if evidence.trim().is_empty() {
                violations.push(M5PublishPreviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: "evidence_refs",
                });
            }
        }

        // Each stored finding's severity and source must agree with its reason, so a
        // blocker can never be relabeled a warning, or a source mis-attributed, by hand.
        for finding in &sheet.findings {
            if finding.severity != finding.reason.severity() {
                violations.push(M5PublishPreviewViolation::FindingSeverityMismatch {
                    sheet_id: sheet.sheet_id.clone(),
                    reason: finding.reason.as_str(),
                });
            }
            if !finding.reason.source_is_valid(finding.source) {
                violations.push(M5PublishPreviewViolation::FindingSourceMismatch {
                    sheet_id: sheet.sheet_id.clone(),
                    source: finding.source.as_str(),
                    reason: finding.reason.as_str(),
                });
            }
        }

        // The published trust posture must equal the gate's recomputed posture, so a sheet
        // can never publish a stronger badge than its signing or namespace truth supports.
        let effective = sheet.effective_trust_posture();
        if sheet.published_trust_posture != effective {
            violations.push(M5PublishPreviewViolation::OverstatedTrustPosture {
                sheet_id: sheet.sheet_id.clone(),
                published: sheet.published_trust_posture.as_str(),
                computed: effective.as_str(),
            });
        }

        // Non-inheritance: a local-dev, side-loaded, or revoked artifact, or an unclaimed
        // or mismatched namespace, must publish local-only and never inherit a trusted
        // badge from the build machine.
        if (sheet.signature_state.is_local_or_untrusted()
            || sheet.namespace_state.caps_to_local_only())
            && sheet.published_trust_posture != TrustPosture::UnsignedLocalOnly
        {
            violations.push(M5PublishPreviewViolation::LocalArtifactInheritedTrust {
                sheet_id: sheet.sheet_id.clone(),
                signature_state: sheet.signature_state.as_str(),
                namespace_state: sheet.namespace_state.as_str(),
                published: sheet.published_trust_posture.as_str(),
            });
        }

        // The recorded readiness must match the recomputed verdict.
        let required = sheet.computed_publish_readiness();
        if sheet.publish_readiness != required {
            violations.push(M5PublishPreviewViolation::ReadinessMismatch {
                sheet_id: sheet.sheet_id.clone(),
                declared: sheet.publish_readiness.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded findings must equal the findings recomputed from the observed facts,
        // in canonical order, so a blocker or warning can never be asserted or hidden by
        // hand.
        if sheet.findings != sheet.computed_findings() {
            violations.push(M5PublishPreviewViolation::FindingsMismatch {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        // A ready-to-publish sheet must be genuinely clean: a verified signature, an owned
        // namespace, a clean anti-abuse posture, no unreviewed widening, and no findings.
        if sheet.is_ready_to_publish()
            && (sheet.signature_state != SignatureState::SignedVerified
                || sheet.namespace_state.caps_to_local_only()
                || sheet.anti_abuse_transparency != AntiAbuseTransparency::DisclosedClean
                || (sheet.has_widening_change() && !sheet.widening_reviewed)
                || !sheet.findings.is_empty())
        {
            violations.push(M5PublishPreviewViolation::ReadySheetNotClean {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 publish-preview sheet-set packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PublishPreviewViolation {
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
        /// Sheet or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A sheet id appears more than once.
    DuplicateSheetId {
        /// Duplicate sheet id.
        sheet_id: String,
    },
    /// A marketed family carries more than one sheet.
    DuplicateFamilySheet {
        /// Family token.
        family: &'static str,
    },
    /// A claimed marketed family has no sheet.
    MissingFamilySheet {
        /// Family token.
        family: &'static str,
    },
    /// A sheet covers a family the packet does not claim.
    UnclaimedFamilySheet {
        /// Sheet id.
        sheet_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A named publish gate is reported more than once.
    DuplicateCheck {
        /// Sheet id.
        sheet_id: String,
        /// Check token.
        check: &'static str,
    },
    /// A named publish gate is missing a result.
    MissingCheck {
        /// Sheet id.
        sheet_id: String,
        /// Check token.
        check: &'static str,
    },
    /// A finding's severity disagrees with its reason.
    FindingSeverityMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Finding-reason token.
        reason: &'static str,
    },
    /// A finding's source is not valid for its reason.
    FindingSourceMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Finding-source token.
        source: &'static str,
        /// Finding-reason token.
        reason: &'static str,
    },
    /// A sheet publishes a trust posture beyond what its signing/namespace truth supports.
    OverstatedTrustPosture {
        /// Sheet id.
        sheet_id: String,
        /// Published trust-posture token.
        published: &'static str,
        /// Computed effective trust-posture token.
        computed: &'static str,
    },
    /// A local/side-loaded/revoked artifact or unclaimed namespace inherited a trusted badge.
    LocalArtifactInheritedTrust {
        /// Sheet id.
        sheet_id: String,
        /// Signing-state token.
        signature_state: &'static str,
        /// Namespace-state token.
        namespace_state: &'static str,
        /// Published trust-posture token.
        published: &'static str,
    },
    /// A sheet's readiness disagrees with the recomputed verdict.
    ReadinessMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Declared readiness token.
        declared: &'static str,
        /// Required readiness token.
        required: &'static str,
    },
    /// A sheet's findings disagree with the recomputed findings.
    FindingsMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A ready-to-publish sheet still carries a finding or a non-clean state.
    ReadySheetNotClean {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet covers a family the publish gate does not.
    MissingMatrixRow {
        /// Sheet id.
        sheet_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A sheet publishes a stronger badge than the publish gate would grant.
    SheetExceedsPublishGate {
        /// Sheet id.
        sheet_id: String,
        /// Published trust-posture token.
        published: &'static str,
        /// Publish-gate trust-posture token.
        gate: &'static str,
    },
    /// The summary counts disagree with the sheets.
    SummaryMismatch,
}

impl fmt::Display for M5PublishPreviewViolation {
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
            Self::DuplicateSheetId { sheet_id } => {
                write!(f, "duplicate sheet id {sheet_id}")
            }
            Self::DuplicateFamilySheet { family } => {
                write!(f, "duplicate sheet for family {family}")
            }
            Self::MissingFamilySheet { family } => {
                write!(f, "missing sheet for claimed family {family}")
            }
            Self::UnclaimedFamilySheet { sheet_id, family } => {
                write!(f, "sheet {sheet_id} covers unclaimed family {family}")
            }
            Self::DuplicateCheck { sheet_id, check } => {
                write!(f, "sheet {sheet_id} reports check {check} more than once")
            }
            Self::MissingCheck { sheet_id, check } => {
                write!(f, "sheet {sheet_id} is missing check {check}")
            }
            Self::FindingSeverityMismatch { sheet_id, reason } => {
                write!(
                    f,
                    "sheet {sheet_id} finding {reason} carries a non-canonical severity"
                )
            }
            Self::FindingSourceMismatch {
                sheet_id,
                source,
                reason,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} finding {reason} is attributed to invalid source {source}"
                )
            }
            Self::OverstatedTrustPosture {
                sheet_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} publishes trust posture {published} but the gate computes {computed}"
                )
            }
            Self::LocalArtifactInheritedTrust {
                sheet_id,
                signature_state,
                namespace_state,
                published,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} is {signature_state}/{namespace_state} but publishes {published}; local artifacts must publish unsigned_local_only"
                )
            }
            Self::ReadinessMismatch {
                sheet_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} records readiness {declared} but the gate requires {required}"
                )
            }
            Self::FindingsMismatch { sheet_id } => {
                write!(f, "sheet {sheet_id} findings disagree with the gate")
            }
            Self::ReadySheetNotClean { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} is ready to publish but carries a finding or non-clean state"
                )
            }
            Self::MissingMatrixRow { sheet_id, family } => {
                write!(
                    f,
                    "sheet {sheet_id} covers family {family} but the publish gate has no row for it"
                )
            }
            Self::SheetExceedsPublishGate {
                sheet_id,
                published,
                gate,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} publishes {published} but the publish gate grants only {gate}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the sheets")
            }
        }
    }
}

impl Error for M5PublishPreviewViolation {}

/// Loads the embedded M5 publish-preview sheet-set packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5PublishPreviewSheetSet`].
pub fn current_m5_publish_preview_sheet_set() -> Result<M5PublishPreviewSheetSet, serde_json::Error>
{
    serde_json::from_str(M5_PUBLISH_PREVIEW_JSON)
}

#[cfg(test)]
mod tests;
