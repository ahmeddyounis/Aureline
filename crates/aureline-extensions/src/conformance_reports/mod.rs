//! Author- and reviewer-facing conformance, compatibility, and mirror/offline
//! bundle review report surfaces.
//!
//! Extension validation already produces a lot of truth: the conformance kit
//! emits passed/failed checks, the publication and registry lanes carry signer,
//! provenance, and compatibility metadata, and the mirror-import baseline keeps
//! source and trust-claim state aligned. Until now that truth lived in opaque
//! CLI logs, registry metadata, or per-lane records. This module turns it into
//! three inspectable report records that publication, review, and side-load /
//! offline decisions can be made against:
//!
//! - [`ExtensionConformanceReport`] renders passed/failed checks with one shared
//!   severity, repro / screenshot guidance, required fixes, and docs links, plus
//!   an inline [`CompatibilitySection`] that shows the target Aureline version
//!   range, deprecated APIs, required shims, removal horizons, and migration
//!   impact *before* publish or install instead of as an opaque "invalid
//!   package".
//! - [`MirrorBundleReview`] renders the side-loaded / mirrored / offline path as
//!   a first-class supported surface: artifact hashes, signing and provenance
//!   state, the source registry or mirror, the dependency graph, and
//!   reproducibility notes. Signing / provenance gaps are surfaced
//!   independently and refuse the bundle — they are never hidden behind a green
//!   compatibility check.
//! - [`ReviewExportBundle`] joins either or both reports into one attachable
//!   artifact and carries the rendered Markdown alongside the JSON record so the
//!   same report can feed issue reports, review packets, release evidence, and
//!   partner evaluations.
//!
//! All three share one [`ReviewSeverityClass`], one [`ReviewCheckStatusClass`],
//! and one [`ReviewLifecycleClass`] so authoring surfaces, install review,
//! marketplace facts, and support packets read the same vocabulary instead of
//! inventing local copies. Conversion helpers ([`ReviewSeverityClass::from_manifest_editor`],
//! [`ReviewLifecycleClass::from_catalog`], [`ReviewLifecycleClass::from_marketplace_badge`])
//! bind that vocabulary back to the existing per-lane records.
//!
//! The machine-readable boundary contracts are
//! [`/schemas/extensions/conformance_report.schema.json`](../../../schemas/extensions/conformance_report.schema.json)
//! and
//! [`/schemas/extensions/mirror_bundle_review.schema.json`](../../../schemas/extensions/mirror_bundle_review.schema.json).
//! The reviewer-facing guide is
//! [`/docs/ecosystem/m3/extension_conformance_and_bundle_review_beta.md`](../../../docs/ecosystem/m3/extension_conformance_and_bundle_review_beta.md).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::RedactionClass;
use crate::manifest_editor::ManifestEditorFindingSeverity;
use crate::marketplace_truth::MarketplaceTruthBadgeClass;
use crate::mirror_import::MirrorImportRouteClass;
use crate::publication::{
    PublicationContentAddress, PublicationProvenanceClass, PublicationSignatureClass,
};
use crate::registry::{CatalogLifecycleStateClass, CatalogRegistrySourceClass};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`ExtensionConformanceReport`] payloads.
pub const EXTENSION_CONFORMANCE_REPORT_RECORD_KIND: &str = "extension_conformance_review_report";

/// Record-kind tag carried on serialized [`MirrorBundleReview`] payloads.
pub const MIRROR_BUNDLE_REVIEW_RECORD_KIND: &str = "mirror_bundle_review_record";

/// Record-kind tag carried on serialized [`ReviewExportBundle`] payloads.
pub const REVIEW_EXPORT_BUNDLE_RECORD_KIND: &str = "extension_review_export_bundle_record";

/// Schema version shared by every report record in this module.
pub const CONFORMANCE_REPORTS_SCHEMA_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Shared review vocabulary
// ---------------------------------------------------------------------------

/// Shared severity vocabulary for review checks and compatibility findings.
///
/// This mirrors the conformance-kit and manifest-editor severities so a single
/// word means the same thing in authoring, install review, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSeverityClass {
    /// Must-fix publish or install blocker.
    Blocker,
    /// Recommended fix; does not block publication or install.
    Warning,
    /// Informational guidance.
    Info,
}

impl ReviewSeverityClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Blocker => "Blocker",
            Self::Warning => "Recommendation",
            Self::Info => "Info",
        }
    }

    /// Maps the manifest-editor / validator severity into the shared vocabulary.
    pub const fn from_manifest_editor(severity: ManifestEditorFindingSeverity) -> Self {
        match severity {
            ManifestEditorFindingSeverity::Blocker => Self::Blocker,
            ManifestEditorFindingSeverity::Warning => Self::Warning,
            ManifestEditorFindingSeverity::Info => Self::Info,
        }
    }
}

/// Shared status vocabulary for a single review check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCheckStatusClass {
    /// The check passed.
    Pass,
    /// The check failed.
    Fail,
    /// The check raised a non-blocking warning.
    Warn,
    /// The check did not apply to this subject.
    NotApplicable,
}

impl ReviewCheckStatusClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Warn => "warn",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Shared lifecycle vocabulary for an extension or one of its surfaces.
///
/// This is the same lifecycle language marketplace facts and catalog rows use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewLifecycleClass {
    /// Early, discoverable, expected-churn surface.
    Preview,
    /// Beta surface with support intent but not stable proof.
    Beta,
    /// Stable surface backed by current evidence.
    Stable,
    /// Retained for migration but on a visible sunset path.
    Deprecated,
    /// Removed from the supported surface.
    Removed,
    /// Installable or reviewable only with narrower guarantees.
    Limited,
    /// Revoked: cannot install or update.
    Revoked,
}

impl ReviewLifecycleClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::Removed => "removed",
            Self::Limited => "limited",
            Self::Revoked => "revoked",
        }
    }

    /// Returns the short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::Deprecated => "Deprecated",
            Self::Removed => "Removed",
            Self::Limited => "Limited",
            Self::Revoked => "Revoked",
        }
    }

    /// Maps a catalog lifecycle state into the shared vocabulary.
    pub const fn from_catalog(state: CatalogLifecycleStateClass) -> Self {
        match state {
            CatalogLifecycleStateClass::Staged => Self::Preview,
            CatalogLifecycleStateClass::Approved => Self::Stable,
            CatalogLifecycleStateClass::Limited => Self::Limited,
            CatalogLifecycleStateClass::Deprecated => Self::Deprecated,
            CatalogLifecycleStateClass::Revoked => Self::Revoked,
            CatalogLifecycleStateClass::Quarantined => Self::Limited,
        }
    }

    /// Maps a marketplace lifecycle badge into the shared vocabulary.
    pub const fn from_marketplace_badge(badge: MarketplaceTruthBadgeClass) -> Self {
        match badge {
            MarketplaceTruthBadgeClass::Preview => Self::Preview,
            MarketplaceTruthBadgeClass::Beta => Self::Beta,
            MarketplaceTruthBadgeClass::Stable => Self::Stable,
            MarketplaceTruthBadgeClass::Deprecated => Self::Deprecated,
            MarketplaceTruthBadgeClass::Limited => Self::Limited,
            MarketplaceTruthBadgeClass::Revoked => Self::Revoked,
            MarketplaceTruthBadgeClass::Mirrored => Self::Beta,
            MarketplaceTruthBadgeClass::RetestPending => Self::Limited,
        }
    }
}

/// One inspectable review check shared by conformance and bundle reports.
///
/// A check carries the same fields a reviewer needs to act: a stable id, the
/// suite it belongs to, the human title, status, severity, the message, an
/// optional field anchor, an optional required fix, optional repro / screenshot
/// guidance, optional evidence refs, and an optional docs link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCheck {
    /// Stable check id.
    pub check_id: String,
    /// Suite the check belongs to (e.g. `manifest_shape`, `signing`).
    pub suite: String,
    /// Human title rendered on the row.
    pub title: String,
    /// Pass/fail/warn/not-applicable status.
    pub status: ReviewCheckStatusClass,
    /// Shared severity.
    pub severity: ReviewSeverityClass,
    /// Human-readable detail message.
    pub message: String,
    /// Optional dotted/JSON-pointer field anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Optional required-fix guidance for failed checks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_fix: Option<String>,
    /// Optional reproduction or screenshot guidance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repro_guidance: Option<String>,
    /// Optional evidence refs (screenshots, logs, artifacts).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// Optional docs link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,
}

impl ReviewCheck {
    /// True when this check is a failed publish/install blocker.
    pub fn is_failed_blocker(&self) -> bool {
        self.status == ReviewCheckStatusClass::Fail && self.severity == ReviewSeverityClass::Blocker
    }

    /// True when this check is a recommendation (warn status or non-blocking failure).
    pub fn is_recommendation(&self) -> bool {
        match self.status {
            ReviewCheckStatusClass::Warn => true,
            ReviewCheckStatusClass::Fail => self.severity != ReviewSeverityClass::Blocker,
            _ => false,
        }
    }
}

// ---------------------------------------------------------------------------
// Conformance + compatibility report
// ---------------------------------------------------------------------------

/// A deprecated API surfaced in the compatibility section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecatedApi {
    /// Stable API or capability id.
    pub api_id: String,
    /// Replacement API or migration path.
    pub replacement: String,
    /// Removal horizon (version or window) after which the API is removed.
    pub removal_horizon: String,
    /// Plain-language migration impact for the author.
    pub migration_impact: String,
    /// Severity of leaving this deprecation unaddressed.
    pub severity: ReviewSeverityClass,
    /// Optional migration docs link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,
}

/// A compatibility shim required before publish or install.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiredShim {
    /// Stable shim id.
    pub shim_id: String,
    /// Why the shim is required.
    pub reason: String,
    /// What the shim covers.
    pub covers: String,
    /// Target version range the shim applies to.
    pub target_version_range: String,
    /// Severity if the shim is absent.
    pub severity: ReviewSeverityClass,
    /// Optional docs link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,
}

/// Compatibility section rendered inline in the conformance report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilitySection {
    /// Minimum supported Aureline version.
    pub target_aureline_version_min: String,
    /// Maximum supported Aureline version.
    pub target_aureline_version_max: String,
    /// SDK line the extension targets.
    pub sdk_line_id: String,
    /// Bridge state quoted from the bridge matrix or publication packet.
    pub bridge_state: String,
    /// Deprecated APIs the author still depends on.
    #[serde(default)]
    pub deprecated_apis: Vec<DeprecatedApi>,
    /// Shims required before publish or install.
    #[serde(default)]
    pub required_shims: Vec<RequiredShim>,
    /// Plain-language overall migration impact summary.
    pub migration_impact_summary: String,
}

/// Aggregate counts for a conformance report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceReportSummary {
    /// Checks with `pass` status.
    pub passed: u32,
    /// Checks with `fail` status.
    pub failed: u32,
    /// Checks with `warn` status.
    pub warnings: u32,
    /// Checks with `not_applicable` status.
    pub not_applicable: u32,
    /// Publish/install blockers (failed blockers plus compatibility blockers).
    pub blockers: u32,
    /// Recommendations a reviewer may distinguish from blockers.
    pub recommendations: u32,
}

/// Overall publish/install decision for the conformance report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceDecisionClass {
    /// No blockers and no recommendations: safe to publish/install.
    PublishReady,
    /// No blockers, but recommendations remain.
    PublishReadyWithRecommendations,
    /// One or more blockers must be fixed first.
    BlockedOnConformance,
}

/// Typed reason paired with [`ConformanceDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceReasonClass {
    /// All checks passed; compatibility carries no blockers.
    ReadyAllChecksPassed,
    /// No blockers but recommendations are present.
    ReadyWithRecommendations,
    /// A conformance blocker check failed.
    BlockedConformanceBlockerFailed,
    /// A required shim or removed deprecation blocks compatibility.
    BlockedCompatibilityBlocker,
}

/// Input consumed to build one conformance report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceReportInput {
    /// Stable report id (must start with `extension_conformance_report:`).
    pub report_id: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Publisher id.
    pub publisher_id: String,
    /// Validator id that produced the underlying checks.
    pub validator_id: String,
    /// Validator version.
    pub validator_version: String,
    /// Subject manifest ref the report covers.
    pub subject_manifest_ref: String,
    /// Lifecycle class of the extension surface.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Inspectable checks.
    pub checks: Vec<ReviewCheck>,
    /// Compatibility section.
    pub compatibility: CompatibilitySection,
    /// Generation timestamp.
    pub generated_at: String,
}

/// One inspectable conformance + compatibility report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionConformanceReport {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version.
    pub conformance_report_schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Publisher id.
    pub publisher_id: String,
    /// Validator id.
    pub validator_id: String,
    /// Validator version.
    pub validator_version: String,
    /// Subject manifest ref.
    pub subject_manifest_ref: String,
    /// Lifecycle class.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Inspectable checks.
    pub checks: Vec<ReviewCheck>,
    /// Compatibility section.
    pub compatibility: CompatibilitySection,
    /// Aggregate counts.
    pub summary: ConformanceReportSummary,
    /// Overall decision.
    pub decision_class: ConformanceDecisionClass,
    /// Typed reason.
    pub reason_class: ConformanceReasonClass,
    /// Export-safe one-line decision summary.
    pub decision_summary: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

// ---------------------------------------------------------------------------
// Mirror / offline bundle review
// ---------------------------------------------------------------------------

/// Artifact identity (hashes) for a bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleArtifactIdentity {
    /// Delivered artifact ref.
    pub artifact_ref: String,
    /// Delivered content address.
    pub content_address: PublicationContentAddress,
    /// Origin content address (for identity comparison).
    pub origin_content_address: PublicationContentAddress,
}

impl BundleArtifactIdentity {
    /// True when the delivered artifact hash matches the origin hash.
    pub fn identity_preserved(&self) -> bool {
        self.content_address == self.origin_content_address
    }
}

/// Signing and provenance state for a bundle review.
///
/// Signing/provenance state is always rendered and evaluated independently of
/// compatibility so a gap can never be hidden behind a green compatibility
/// check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleSigningProvenance {
    /// Signature posture.
    pub signature_class: PublicationSignatureClass,
    /// Provenance posture.
    pub provenance_class: PublicationProvenanceClass,
    /// Signer ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_ref: Option<String>,
    /// Signer key fingerprint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_key_fingerprint: Option<String>,
    /// Signature ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_ref: Option<String>,
    /// Provenance ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_ref: Option<String>,
    /// Optional transparency-log ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transparency_log_ref: Option<String>,
}

impl BundleSigningProvenance {
    /// True when a usable signature is present.
    pub fn signature_present(&self) -> bool {
        !matches!(
            self.signature_class,
            PublicationSignatureClass::UnsignedDeniedOnPolicy
        ) && self
            .signature_ref
            .as_deref()
            .is_some_and(|r| !r.trim().is_empty())
    }

    /// True when usable provenance is present.
    pub fn provenance_present(&self) -> bool {
        !matches!(
            self.provenance_class,
            PublicationProvenanceClass::MissingProvenance
        ) && self
            .provenance_ref
            .as_deref()
            .is_some_and(|r| !r.trim().is_empty())
    }
}

/// Source registry / mirror metadata for a bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleSource {
    /// Delivery route.
    pub route_class: MirrorImportRouteClass,
    /// Registry source class.
    pub registry_source_class: CatalogRegistrySourceClass,
    /// Human-readable source label.
    pub source_label: String,
    /// Optional mirror or offline origin ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_or_offline_origin_ref: Option<String>,
    /// True when an out-of-band manual verification receipt is attached.
    #[serde(default)]
    pub manual_verification_attached: bool,
}

/// Resolution state of one dependency-graph node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleDependencyResolutionClass {
    /// Dependency is bundled and verified.
    Resolved,
    /// Dependency is present but its identity is downgraded.
    ResolvedDowngraded,
    /// Dependency could not be resolved from the bundle.
    Unresolved,
    /// Dependency was resolved to a mismatched identity.
    Mismatched,
}

/// One dependency-graph node in a bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDependencyNode {
    /// Stable dependency id.
    pub dependency_id: String,
    /// Dependency display name.
    pub name: String,
    /// Dependency version.
    pub version: String,
    /// Resolution state.
    pub resolution_class: BundleDependencyResolutionClass,
    /// Source class for this dependency.
    pub source_class: CatalogRegistrySourceClass,
    /// Optional content address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_address: Option<PublicationContentAddress>,
    /// Optional human-readable note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Reproducibility posture for a bundle review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleReproducibilityClass {
    /// The artifact rebuilds bit-for-bit from recorded inputs.
    Reproducible,
    /// Only part of the artifact is reproducible.
    PartiallyReproducible,
    /// The artifact is not reproducible.
    NotReproducible,
    /// Reproducibility has not been verified.
    Unverified,
}

/// Reproducibility notes for a bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleReproducibility {
    /// Reproducibility posture.
    pub reproducible_class: BundleReproducibilityClass,
    /// Optional build provenance ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_provenance_ref: Option<String>,
    /// Optional rebuild-instructions ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebuild_instructions_ref: Option<String>,
    /// Plain-language notes.
    pub notes: String,
}

/// Overall decision for a mirror/offline bundle review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleReviewDecisionClass {
    /// Bundle is safe to side-load / install.
    ReadyForSideload,
    /// Bundle can install, but one or more trust claims is visibly downgraded.
    ReadyWithDowngrades,
    /// Bundle needs an admin / operator review before install.
    AwaitingAdminReview,
    /// Bundle must not install.
    Refused,
}

/// Typed reason paired with [`BundleReviewDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleReviewReasonClass {
    /// All identity, signing, provenance, and dependency truth is preserved.
    ReadyAllTrustPreserved,
    /// Trust claims, dependencies, or reproducibility are visibly downgraded.
    ReadyWithDowngradedTrustClaims,
    /// A manual artifact import needs an out-of-band verification receipt.
    AwaitingManualVerification,
    /// Delivered artifact hash does not match the origin hash.
    RefusedArtifactIdentityMismatch,
    /// A signing gap blocks the bundle.
    RefusedSigningGap,
    /// A provenance gap blocks the bundle.
    RefusedProvenanceGap,
    /// A dependency could not be resolved or is mismatched.
    RefusedDependencyUnresolved,
    /// A blocker review check failed.
    RefusedBlockerCheckFailed,
}

/// Aggregate counts for a bundle review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleReviewSummary {
    /// Total dependency-graph nodes.
    pub dependency_count: u32,
    /// Dependency nodes that are unresolved or mismatched.
    pub unresolved_dependency_count: u32,
    /// Dependency nodes resolved but downgraded.
    pub downgraded_dependency_count: u32,
    /// Failed blocker checks.
    pub blockers: u32,
    /// Recommendations.
    pub recommendations: u32,
}

/// Input consumed to build one mirror/offline bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorBundleReviewInput {
    /// Stable review id (must start with `mirror_bundle_review:`).
    pub review_id: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Lifecycle class.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Artifact identity (hashes).
    pub artifact: BundleArtifactIdentity,
    /// Signing and provenance state.
    pub signing_provenance: BundleSigningProvenance,
    /// Source registry / mirror metadata.
    pub source: BundleSource,
    /// Dependency graph.
    #[serde(default)]
    pub dependency_graph: Vec<BundleDependencyNode>,
    /// Reproducibility notes.
    pub reproducibility: BundleReproducibility,
    /// Number of trust claims that are visibly downgraded.
    #[serde(default)]
    pub downgraded_trust_claim_count: u32,
    /// Inspectable review checks.
    #[serde(default)]
    pub checks: Vec<ReviewCheck>,
    /// Generation timestamp.
    pub generated_at: String,
}

/// One inspectable mirror/offline bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorBundleReview {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version.
    pub mirror_bundle_review_schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Lifecycle class.
    pub lifecycle_class: ReviewLifecycleClass,
    /// Artifact identity (hashes).
    pub artifact: BundleArtifactIdentity,
    /// Signing and provenance state.
    pub signing_provenance: BundleSigningProvenance,
    /// Source registry / mirror metadata.
    pub source: BundleSource,
    /// Dependency graph.
    pub dependency_graph: Vec<BundleDependencyNode>,
    /// Reproducibility notes.
    pub reproducibility: BundleReproducibility,
    /// Number of trust claims that are visibly downgraded.
    pub downgraded_trust_claim_count: u32,
    /// Inspectable review checks.
    pub checks: Vec<ReviewCheck>,
    /// Aggregate counts.
    pub summary: BundleReviewSummary,
    /// True when the delivered artifact hash matches the origin hash.
    pub artifact_identity_preserved: bool,
    /// True when a usable signature is present.
    pub signature_present: bool,
    /// True when usable provenance is present.
    pub provenance_present: bool,
    /// True when install/side-load may continue.
    pub install_lane_continues: bool,
    /// Overall decision.
    pub decision_class: BundleReviewDecisionClass,
    /// Typed reason.
    pub reason_class: BundleReviewReasonClass,
    /// Export-safe one-line decision summary.
    pub decision_summary: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

// ---------------------------------------------------------------------------
// Export bundle
// ---------------------------------------------------------------------------

/// One attachable export bundle joining the JSON reports and their Markdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewExportBundle {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version.
    pub review_export_bundle_schema_version: u32,
    /// Stable export id (must start with `extension_review_export_bundle:`).
    pub export_id: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Optional conformance report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_report: Option<ExtensionConformanceReport>,
    /// Optional bundle review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_review: Option<MirrorBundleReview>,
    /// Rendered Markdown for the combined report.
    pub markdown: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

// ---------------------------------------------------------------------------
// Findings
// ---------------------------------------------------------------------------

/// Typed validation finding emitted by report validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceReportFinding {
    /// Stable validation check id.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
}

impl ConformanceReportFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Conformance report: build / validate / render
// ---------------------------------------------------------------------------

/// Build one conformance + compatibility report from inspectable truth.
pub fn build_conformance_report(input: ConformanceReportInput) -> ExtensionConformanceReport {
    let passed = count_status(&input.checks, ReviewCheckStatusClass::Pass);
    let failed = count_status(&input.checks, ReviewCheckStatusClass::Fail);
    let warnings = count_status(&input.checks, ReviewCheckStatusClass::Warn);
    let not_applicable = count_status(&input.checks, ReviewCheckStatusClass::NotApplicable);

    let check_blockers = input
        .checks
        .iter()
        .filter(|c| c.is_failed_blocker())
        .count() as u32;
    let compat_blockers = compatibility_blocker_count(&input.compatibility);
    let blockers = check_blockers + compat_blockers;

    let check_recommendations = input
        .checks
        .iter()
        .filter(|c| c.is_recommendation())
        .count() as u32;
    let compat_recommendations = compatibility_recommendation_count(&input.compatibility);
    let recommendations = check_recommendations + compat_recommendations;

    let (decision_class, reason_class) = if blockers > 0 {
        if check_blockers > 0 {
            (
                ConformanceDecisionClass::BlockedOnConformance,
                ConformanceReasonClass::BlockedConformanceBlockerFailed,
            )
        } else {
            (
                ConformanceDecisionClass::BlockedOnConformance,
                ConformanceReasonClass::BlockedCompatibilityBlocker,
            )
        }
    } else if recommendations > 0 {
        (
            ConformanceDecisionClass::PublishReadyWithRecommendations,
            ConformanceReasonClass::ReadyWithRecommendations,
        )
    } else {
        (
            ConformanceDecisionClass::PublishReady,
            ConformanceReasonClass::ReadyAllChecksPassed,
        )
    };

    let decision_summary = format!(
        "{} {}: {} ({} blockers, {} recommendations across {} checks); target Aureline {}–{}.",
        input.extension_identity,
        input.extension_version,
        decision_label(decision_class),
        blockers,
        recommendations,
        input.checks.len(),
        input.compatibility.target_aureline_version_min,
        input.compatibility.target_aureline_version_max,
    );

    ExtensionConformanceReport {
        record_kind: EXTENSION_CONFORMANCE_REPORT_RECORD_KIND.to_string(),
        conformance_report_schema_version: CONFORMANCE_REPORTS_SCHEMA_VERSION,
        report_id: input.report_id,
        extension_identity: input.extension_identity,
        extension_version: input.extension_version,
        package_id: input.package_id,
        publisher_id: input.publisher_id,
        validator_id: input.validator_id,
        validator_version: input.validator_version,
        subject_manifest_ref: input.subject_manifest_ref,
        lifecycle_class: input.lifecycle_class,
        checks: input.checks,
        compatibility: input.compatibility,
        summary: ConformanceReportSummary {
            passed,
            failed,
            warnings,
            not_applicable,
            blockers,
            recommendations,
        },
        decision_class,
        reason_class,
        decision_summary,
        generated_at: input.generated_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a conformance report.
pub fn validate_conformance_report(
    report: &ExtensionConformanceReport,
) -> Vec<ConformanceReportFinding> {
    let mut findings = Vec::new();

    if report.record_kind != EXTENSION_CONFORMANCE_REPORT_RECORD_KIND {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_CONFORMANCE_REPORT_RECORD_KIND}'; got {:?}",
                report.record_kind
            ),
        ));
    }
    if report.conformance_report_schema_version != CONFORMANCE_REPORTS_SCHEMA_VERSION {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.schema_version_wrong",
            format!(
                "conformance_report_schema_version must be {CONFORMANCE_REPORTS_SCHEMA_VERSION}; got {}",
                report.conformance_report_schema_version
            ),
        ));
    }
    if !report
        .report_id
        .starts_with("extension_conformance_report:")
    {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.id_unprefixed",
            "report_id must start with 'extension_conformance_report:'",
        ));
    }
    if report.checks.is_empty() {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.checks_empty",
            "a conformance report must carry at least one check",
        ));
    }
    for check in &report.checks {
        validate_check(check, "conformance_report", &mut findings);
    }
    for api in &report.compatibility.deprecated_apis {
        if api.removal_horizon.trim().is_empty() {
            findings.push(ConformanceReportFinding::new(
                "conformance_report.deprecated_api_missing_removal_horizon",
                format!(
                    "deprecated api '{}' must carry a removal horizon",
                    api.api_id
                ),
            ));
        }
        if api.replacement.trim().is_empty() {
            findings.push(ConformanceReportFinding::new(
                "conformance_report.deprecated_api_missing_replacement",
                format!(
                    "deprecated api '{}' must carry a replacement path",
                    api.api_id
                ),
            ));
        }
    }
    for shim in &report.compatibility.required_shims {
        if shim.covers.trim().is_empty() || shim.target_version_range.trim().is_empty() {
            findings.push(ConformanceReportFinding::new(
                "conformance_report.required_shim_incomplete",
                format!(
                    "required shim '{}' must declare what it covers and its target version range",
                    shim.shim_id
                ),
            ));
        }
    }

    let recomputed = build_conformance_report(report_to_input(report));
    if recomputed.summary != report.summary {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.summary_inconsistent",
            "summary counts must reflect the checks and compatibility section",
        ));
    }
    if recomputed.decision_class != report.decision_class
        || recomputed.reason_class != report.reason_class
    {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.decision_inconsistent",
            "decision_class/reason_class must reflect the blocker and recommendation counts",
        ));
    }
    if report.summary.blockers > 0
        && report.decision_class != ConformanceDecisionClass::BlockedOnConformance
    {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.blockers_not_blocked",
            "reports with blockers must render the BlockedOnConformance decision",
        ));
    }
    if report.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ConformanceReportFinding::new(
            "conformance_report.redaction_not_metadata_safe",
            "conformance reports must be metadata-safe by default",
        ));
    }

    findings
}

/// Render one conformance report as reviewer-facing Markdown.
pub fn render_conformance_report_markdown(report: &ExtensionConformanceReport) -> String {
    let mut out = String::new();
    out.push_str("# Extension conformance report\n\n");
    out.push_str(&format!(
        "- **Extension:** {} {}\n",
        report.extension_identity, report.extension_version
    ));
    out.push_str(&format!("- **Package:** {}\n", report.package_id));
    out.push_str(&format!("- **Publisher:** {}\n", report.publisher_id));
    out.push_str(&format!(
        "- **Lifecycle:** {}\n",
        report.lifecycle_class.label()
    ));
    out.push_str(&format!(
        "- **Validator:** {} {}\n",
        report.validator_id, report.validator_version
    ));
    out.push_str(&format!(
        "- **Subject manifest:** {}\n",
        report.subject_manifest_ref
    ));
    out.push_str(&format!(
        "- **Decision:** {} — {}\n\n",
        decision_label(report.decision_class),
        report.decision_summary
    ));

    out.push_str("## Summary\n\n");
    out.push_str("| Result | Count |\n| --- | --- |\n");
    out.push_str(&format!("| Passed | {} |\n", report.summary.passed));
    out.push_str(&format!("| Failed | {} |\n", report.summary.failed));
    out.push_str(&format!("| Warnings | {} |\n", report.summary.warnings));
    out.push_str(&format!(
        "| Not applicable | {} |\n",
        report.summary.not_applicable
    ));
    out.push_str(&format!("| **Blockers** | {} |\n", report.summary.blockers));
    out.push_str(&format!(
        "| Recommendations | {} |\n\n",
        report.summary.recommendations
    ));

    out.push_str("## Checks\n\n");
    for check in &report.checks {
        out.push_str(&format!(
            "### {} {} — {} (`{}`)\n",
            status_glyph(check.status),
            check.severity.label(),
            check.title,
            check.check_id
        ));
        out.push_str(&format!(
            "- Status: {} · Suite: {}\n",
            check.status.as_str(),
            check.suite
        ));
        out.push_str(&format!("- {}\n", check.message));
        if let Some(field) = &check.field {
            out.push_str(&format!("- Field: `{field}`\n"));
        }
        if let Some(fix) = &check.required_fix {
            out.push_str(&format!("- Required fix: {fix}\n"));
        }
        if let Some(repro) = &check.repro_guidance {
            out.push_str(&format!("- Repro: {repro}\n"));
        }
        if !check.evidence_refs.is_empty() {
            out.push_str(&format!("- Evidence: {}\n", check.evidence_refs.join(", ")));
        }
        if let Some(docs) = &check.docs_url {
            out.push_str(&format!("- Docs: {docs}\n"));
        }
        out.push('\n');
    }

    out.push_str("## Compatibility\n\n");
    out.push_str(&format!(
        "- **Target Aureline version:** {} – {}\n",
        report.compatibility.target_aureline_version_min,
        report.compatibility.target_aureline_version_max
    ));
    out.push_str(&format!(
        "- **SDK line:** {}\n",
        report.compatibility.sdk_line_id
    ));
    out.push_str(&format!(
        "- **Bridge state:** {}\n",
        report.compatibility.bridge_state
    ));
    out.push_str(&format!(
        "- **Migration impact:** {}\n\n",
        report.compatibility.migration_impact_summary
    ));
    if !report.compatibility.deprecated_apis.is_empty() {
        out.push_str("### Deprecated APIs\n\n");
        for api in &report.compatibility.deprecated_apis {
            out.push_str(&format!(
                "- {} `{}` → {} (removal: {}) — {}\n",
                api.severity.label(),
                api.api_id,
                api.replacement,
                api.removal_horizon,
                api.migration_impact
            ));
        }
        out.push('\n');
    }
    if !report.compatibility.required_shims.is_empty() {
        out.push_str("### Required shims\n\n");
        for shim in &report.compatibility.required_shims {
            out.push_str(&format!(
                "- {} `{}` covers {} ({}); {}\n",
                shim.severity.label(),
                shim.shim_id,
                shim.covers,
                shim.target_version_range,
                shim.reason
            ));
        }
        out.push('\n');
    }

    out
}

// ---------------------------------------------------------------------------
// Bundle review: build / validate / render
// ---------------------------------------------------------------------------

/// Build one mirror/offline bundle review from inspectable truth.
pub fn build_mirror_bundle_review(input: MirrorBundleReviewInput) -> MirrorBundleReview {
    let artifact_identity_preserved = input.artifact.identity_preserved();
    let signature_present = input.signing_provenance.signature_present();
    let provenance_present = input.signing_provenance.provenance_present();

    let dependency_count = input.dependency_graph.len() as u32;
    let unresolved_dependency_count = input
        .dependency_graph
        .iter()
        .filter(|d| {
            matches!(
                d.resolution_class,
                BundleDependencyResolutionClass::Unresolved
                    | BundleDependencyResolutionClass::Mismatched
            )
        })
        .count() as u32;
    let downgraded_dependency_count = input
        .dependency_graph
        .iter()
        .filter(|d| d.resolution_class == BundleDependencyResolutionClass::ResolvedDowngraded)
        .count() as u32;

    let blockers = input
        .checks
        .iter()
        .filter(|c| c.is_failed_blocker())
        .count() as u32;
    let recommendations = input
        .checks
        .iter()
        .filter(|c| c.is_recommendation())
        .count() as u32;

    let (decision_class, reason_class) = decide_bundle(
        &input,
        artifact_identity_preserved,
        signature_present,
        provenance_present,
        unresolved_dependency_count,
        downgraded_dependency_count,
        blockers,
    );
    let install_lane_continues = matches!(
        decision_class,
        BundleReviewDecisionClass::ReadyForSideload
            | BundleReviewDecisionClass::ReadyWithDowngrades
    );

    let decision_summary = format!(
        "{} {} from {} ({:?}): {:?}; signature_present={}; provenance_present={}; downgraded_claims={}; deps={} ({} unresolved).",
        input.extension_identity,
        input.extension_version,
        input.source.source_label,
        input.source.route_class,
        decision_class,
        signature_present,
        provenance_present,
        input.downgraded_trust_claim_count,
        dependency_count,
        unresolved_dependency_count,
    );

    MirrorBundleReview {
        record_kind: MIRROR_BUNDLE_REVIEW_RECORD_KIND.to_string(),
        mirror_bundle_review_schema_version: CONFORMANCE_REPORTS_SCHEMA_VERSION,
        review_id: input.review_id,
        extension_identity: input.extension_identity,
        extension_version: input.extension_version,
        package_id: input.package_id,
        lifecycle_class: input.lifecycle_class,
        artifact: input.artifact,
        signing_provenance: input.signing_provenance,
        source: input.source,
        dependency_graph: input.dependency_graph,
        reproducibility: input.reproducibility,
        downgraded_trust_claim_count: input.downgraded_trust_claim_count,
        checks: input.checks,
        summary: BundleReviewSummary {
            dependency_count,
            unresolved_dependency_count,
            downgraded_dependency_count,
            blockers,
            recommendations,
        },
        artifact_identity_preserved,
        signature_present,
        provenance_present,
        install_lane_continues,
        decision_class,
        reason_class,
        decision_summary,
        generated_at: input.generated_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a mirror/offline bundle review.
pub fn validate_mirror_bundle_review(review: &MirrorBundleReview) -> Vec<ConformanceReportFinding> {
    let mut findings = Vec::new();

    if review.record_kind != MIRROR_BUNDLE_REVIEW_RECORD_KIND {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.record_kind_wrong",
            format!(
                "record_kind must be '{MIRROR_BUNDLE_REVIEW_RECORD_KIND}'; got {:?}",
                review.record_kind
            ),
        ));
    }
    if review.mirror_bundle_review_schema_version != CONFORMANCE_REPORTS_SCHEMA_VERSION {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.schema_version_wrong",
            format!(
                "mirror_bundle_review_schema_version must be {CONFORMANCE_REPORTS_SCHEMA_VERSION}; got {}",
                review.mirror_bundle_review_schema_version
            ),
        ));
    }
    if !review.review_id.starts_with("mirror_bundle_review:") {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.id_unprefixed",
            "review_id must start with 'mirror_bundle_review:'",
        ));
    }
    if review.source.source_label.trim().is_empty() {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.source_label_required",
            "bundle review must render a source label",
        ));
    }
    if !route_matches_source(
        review.source.route_class,
        review.source.registry_source_class,
    ) {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.route_source_mismatch",
            "route_class must match registry_source_class",
        ));
    }
    if review.reproducibility.notes.trim().is_empty() {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.reproducibility_notes_required",
            "bundle review must carry reproducibility notes",
        ));
    }
    for check in &review.checks {
        validate_check(check, "mirror_bundle_review", &mut findings);
    }

    // Guardrail: signing/provenance gaps must never be hidden behind a green
    // compatibility/install decision.
    if (!review.signature_present || !review.provenance_present) && review.install_lane_continues {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.signing_gap_hidden",
            "a signing or provenance gap must refuse the bundle, not continue the install lane",
        ));
    }
    if !review.artifact_identity_preserved
        && review.decision_class != BundleReviewDecisionClass::Refused
    {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.identity_mismatch_not_refused",
            "an artifact identity mismatch must refuse the bundle",
        ));
    }

    let recomputed = build_mirror_bundle_review(review_to_input(review));
    if recomputed.summary != review.summary {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.summary_inconsistent",
            "summary counts must reflect dependencies and checks",
        ));
    }
    if recomputed.decision_class != review.decision_class
        || recomputed.reason_class != review.reason_class
    {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.decision_inconsistent",
            "decision_class/reason_class must reflect identity, signing, provenance, and dependencies",
        ));
    }
    if review.artifact_identity_preserved != review.artifact.identity_preserved() {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.identity_flag_inconsistent",
            "artifact_identity_preserved must compare delivered and origin hashes",
        ));
    }
    if review.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ConformanceReportFinding::new(
            "mirror_bundle_review.redaction_not_metadata_safe",
            "bundle reviews must be metadata-safe by default",
        ));
    }

    findings
}

/// Render one bundle review as reviewer-facing Markdown.
pub fn render_mirror_bundle_review_markdown(review: &MirrorBundleReview) -> String {
    let mut out = String::new();
    out.push_str("# Mirror / offline bundle review\n\n");
    out.push_str(&format!(
        "- **Extension:** {} {}\n",
        review.extension_identity, review.extension_version
    ));
    out.push_str(&format!("- **Package:** {}\n", review.package_id));
    out.push_str(&format!(
        "- **Lifecycle:** {}\n",
        review.lifecycle_class.label()
    ));
    out.push_str(&format!(
        "- **Source:** {} ({:?} · {:?})\n",
        review.source.source_label, review.source.route_class, review.source.registry_source_class
    ));
    if let Some(origin) = &review.source.mirror_or_offline_origin_ref {
        out.push_str(&format!("- **Origin ref:** {origin}\n"));
    }
    out.push_str(&format!(
        "- **Decision:** {:?} — {}\n\n",
        review.decision_class, review.decision_summary
    ));

    out.push_str("## Artifact identity\n\n");
    out.push_str(&format!(
        "- **Artifact:** {}\n",
        review.artifact.artifact_ref
    ));
    out.push_str(&format!(
        "- **Hash:** {}:{} ({} bytes)\n",
        review.artifact.content_address.digest_algorithm,
        review.artifact.content_address.digest_hex,
        review.artifact.content_address.digest_size_bytes
    ));
    out.push_str(&format!(
        "- **Identity preserved:** {}\n\n",
        yes_no(review.artifact_identity_preserved)
    ));

    out.push_str("## Signing & provenance\n\n");
    out.push_str(&format!(
        "- **Signature:** {:?} ({})\n",
        review.signing_provenance.signature_class,
        if review.signature_present {
            "present"
        } else {
            "**missing — blocks install**"
        }
    ));
    out.push_str(&format!(
        "- **Provenance:** {:?} ({})\n",
        review.signing_provenance.provenance_class,
        if review.provenance_present {
            "present"
        } else {
            "**missing — blocks install**"
        }
    ));
    if let Some(signer) = &review.signing_provenance.signer_ref {
        out.push_str(&format!("- **Signer:** {signer}\n"));
    }
    if let Some(log) = &review.signing_provenance.transparency_log_ref {
        out.push_str(&format!("- **Transparency log:** {log}\n"));
    }
    out.push('\n');

    out.push_str("## Dependency graph\n\n");
    if review.dependency_graph.is_empty() {
        out.push_str("- No bundled dependencies.\n\n");
    } else {
        for dep in &review.dependency_graph {
            out.push_str(&format!(
                "- `{}` {} ({:?} · {:?})",
                dep.name, dep.version, dep.resolution_class, dep.source_class
            ));
            if let Some(note) = &dep.notes {
                out.push_str(&format!(" — {note}"));
            }
            out.push('\n');
        }
        out.push('\n');
    }

    out.push_str("## Reproducibility\n\n");
    out.push_str(&format!(
        "- **Posture:** {:?}\n",
        review.reproducibility.reproducible_class
    ));
    if let Some(provenance) = &review.reproducibility.build_provenance_ref {
        out.push_str(&format!("- **Build provenance:** {provenance}\n"));
    }
    if let Some(rebuild) = &review.reproducibility.rebuild_instructions_ref {
        out.push_str(&format!("- **Rebuild instructions:** {rebuild}\n"));
    }
    out.push_str(&format!(
        "- **Notes:** {}\n\n",
        review.reproducibility.notes
    ));

    if !review.checks.is_empty() {
        out.push_str("## Review checks\n\n");
        for check in &review.checks {
            out.push_str(&format!(
                "### {} {} — {} (`{}`)\n",
                status_glyph(check.status),
                check.severity.label(),
                check.title,
                check.check_id
            ));
            out.push_str(&format!("- {}\n", check.message));
            if let Some(fix) = &check.required_fix {
                out.push_str(&format!("- Required fix: {fix}\n"));
            }
            if let Some(docs) = &check.docs_url {
                out.push_str(&format!("- Docs: {docs}\n"));
            }
            out.push('\n');
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Export bundle: build / validate / render
// ---------------------------------------------------------------------------

/// Build one attachable export bundle joining the JSON reports and their Markdown.
///
/// At least one of `conformance_report` or `bundle_review` must be present.
pub fn build_review_export_bundle(
    export_id: &str,
    extension_identity: &str,
    extension_version: &str,
    conformance_report: Option<ExtensionConformanceReport>,
    bundle_review: Option<MirrorBundleReview>,
    generated_at: &str,
) -> ReviewExportBundle {
    let markdown =
        render_review_export_bundle_markdown(conformance_report.as_ref(), bundle_review.as_ref());
    ReviewExportBundle {
        record_kind: REVIEW_EXPORT_BUNDLE_RECORD_KIND.to_string(),
        review_export_bundle_schema_version: CONFORMANCE_REPORTS_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        extension_identity: extension_identity.to_string(),
        extension_version: extension_version.to_string(),
        conformance_report,
        bundle_review,
        markdown,
        generated_at: generated_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Render the combined Markdown for an export bundle.
pub fn render_review_export_bundle_markdown(
    conformance_report: Option<&ExtensionConformanceReport>,
    bundle_review: Option<&MirrorBundleReview>,
) -> String {
    let mut out = String::new();
    if let Some(report) = conformance_report {
        out.push_str(&render_conformance_report_markdown(report));
    }
    if conformance_report.is_some() && bundle_review.is_some() {
        out.push_str("\n---\n\n");
    }
    if let Some(review) = bundle_review {
        out.push_str(&render_mirror_bundle_review_markdown(review));
    }
    out
}

/// Validate structural invariants for an export bundle.
pub fn validate_review_export_bundle(bundle: &ReviewExportBundle) -> Vec<ConformanceReportFinding> {
    let mut findings = Vec::new();

    if bundle.record_kind != REVIEW_EXPORT_BUNDLE_RECORD_KIND {
        findings.push(ConformanceReportFinding::new(
            "review_export_bundle.record_kind_wrong",
            format!(
                "record_kind must be '{REVIEW_EXPORT_BUNDLE_RECORD_KIND}'; got {:?}",
                bundle.record_kind
            ),
        ));
    }
    if bundle.review_export_bundle_schema_version != CONFORMANCE_REPORTS_SCHEMA_VERSION {
        findings.push(ConformanceReportFinding::new(
            "review_export_bundle.schema_version_wrong",
            format!(
                "review_export_bundle_schema_version must be {CONFORMANCE_REPORTS_SCHEMA_VERSION}; got {}",
                bundle.review_export_bundle_schema_version
            ),
        ));
    }
    if !bundle
        .export_id
        .starts_with("extension_review_export_bundle:")
    {
        findings.push(ConformanceReportFinding::new(
            "review_export_bundle.id_unprefixed",
            "export_id must start with 'extension_review_export_bundle:'",
        ));
    }
    if bundle.conformance_report.is_none() && bundle.bundle_review.is_none() {
        findings.push(ConformanceReportFinding::new(
            "review_export_bundle.empty",
            "export bundle must carry a conformance report, a bundle review, or both",
        ));
    }
    if bundle.markdown.trim().is_empty() {
        findings.push(ConformanceReportFinding::new(
            "review_export_bundle.markdown_empty",
            "export bundle must carry rendered Markdown",
        ));
    }
    if let Some(report) = &bundle.conformance_report {
        findings.extend(validate_conformance_report(report));
    }
    if let Some(review) = &bundle.bundle_review {
        findings.extend(validate_mirror_bundle_review(review));
    }
    if bundle.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ConformanceReportFinding::new(
            "review_export_bundle.redaction_not_metadata_safe",
            "export bundles must be metadata-safe by default",
        ));
    }

    findings
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn count_status(checks: &[ReviewCheck], status: ReviewCheckStatusClass) -> u32 {
    checks.iter().filter(|c| c.status == status).count() as u32
}

fn compatibility_blocker_count(compat: &CompatibilitySection) -> u32 {
    let deprecated = compat
        .deprecated_apis
        .iter()
        .filter(|a| a.severity == ReviewSeverityClass::Blocker)
        .count() as u32;
    let shims = compat
        .required_shims
        .iter()
        .filter(|s| s.severity == ReviewSeverityClass::Blocker)
        .count() as u32;
    deprecated + shims
}

fn compatibility_recommendation_count(compat: &CompatibilitySection) -> u32 {
    let deprecated = compat
        .deprecated_apis
        .iter()
        .filter(|a| a.severity != ReviewSeverityClass::Blocker)
        .count() as u32;
    let shims = compat
        .required_shims
        .iter()
        .filter(|s| s.severity != ReviewSeverityClass::Blocker)
        .count() as u32;
    deprecated + shims
}

#[allow(clippy::too_many_arguments)]
fn decide_bundle(
    input: &MirrorBundleReviewInput,
    artifact_identity_preserved: bool,
    signature_present: bool,
    provenance_present: bool,
    unresolved_dependency_count: u32,
    downgraded_dependency_count: u32,
    blockers: u32,
) -> (BundleReviewDecisionClass, BundleReviewReasonClass) {
    if !artifact_identity_preserved {
        return (
            BundleReviewDecisionClass::Refused,
            BundleReviewReasonClass::RefusedArtifactIdentityMismatch,
        );
    }
    if !signature_present {
        return (
            BundleReviewDecisionClass::Refused,
            BundleReviewReasonClass::RefusedSigningGap,
        );
    }
    if !provenance_present {
        return (
            BundleReviewDecisionClass::Refused,
            BundleReviewReasonClass::RefusedProvenanceGap,
        );
    }
    if unresolved_dependency_count > 0 {
        return (
            BundleReviewDecisionClass::Refused,
            BundleReviewReasonClass::RefusedDependencyUnresolved,
        );
    }
    if blockers > 0 {
        return (
            BundleReviewDecisionClass::Refused,
            BundleReviewReasonClass::RefusedBlockerCheckFailed,
        );
    }
    if input.source.route_class == MirrorImportRouteClass::ManualArtifact
        && !input.source.manual_verification_attached
    {
        return (
            BundleReviewDecisionClass::AwaitingAdminReview,
            BundleReviewReasonClass::AwaitingManualVerification,
        );
    }
    let reproducibility_downgraded = !matches!(
        input.reproducibility.reproducible_class,
        BundleReproducibilityClass::Reproducible
    );
    if input.downgraded_trust_claim_count > 0
        || downgraded_dependency_count > 0
        || reproducibility_downgraded
    {
        return (
            BundleReviewDecisionClass::ReadyWithDowngrades,
            BundleReviewReasonClass::ReadyWithDowngradedTrustClaims,
        );
    }
    (
        BundleReviewDecisionClass::ReadyForSideload,
        BundleReviewReasonClass::ReadyAllTrustPreserved,
    )
}

fn validate_check(
    check: &ReviewCheck,
    prefix: &'static str,
    findings: &mut Vec<ConformanceReportFinding>,
) {
    if check.check_id.trim().is_empty() {
        findings.push(ConformanceReportFinding::new(
            check_id_missing(prefix),
            "every review check must carry a stable check_id",
        ));
    }
    if check.title.trim().is_empty() || check.message.trim().is_empty() {
        findings.push(ConformanceReportFinding::new(
            check_title_missing(prefix),
            format!(
                "review check '{}' must carry a title and message",
                check.check_id
            ),
        ));
    }
    if check.is_failed_blocker()
        && check
            .required_fix
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        findings.push(ConformanceReportFinding::new(
            check_fix_missing(prefix),
            format!(
                "failed blocker check '{}' must carry a required fix",
                check.check_id
            ),
        ));
    }
}

fn check_id_missing(prefix: &str) -> &'static str {
    match prefix {
        "conformance_report" => "conformance_report.check_id_missing",
        _ => "mirror_bundle_review.check_id_missing",
    }
}

fn check_title_missing(prefix: &str) -> &'static str {
    match prefix {
        "conformance_report" => "conformance_report.check_title_missing",
        _ => "mirror_bundle_review.check_title_missing",
    }
}

fn check_fix_missing(prefix: &str) -> &'static str {
    match prefix {
        "conformance_report" => "conformance_report.check_fix_missing",
        _ => "mirror_bundle_review.check_fix_missing",
    }
}

fn route_matches_source(
    route_class: MirrorImportRouteClass,
    source_class: CatalogRegistrySourceClass,
) -> bool {
    match route_class {
        MirrorImportRouteClass::PrimaryCatalog => matches!(
            source_class,
            CatalogRegistrySourceClass::PublicRegistry
                | CatalogRegistrySourceClass::PrivateRegistry
        ),
        MirrorImportRouteClass::ApprovedMirror => {
            source_class == CatalogRegistrySourceClass::ApprovedMirror
        }
        MirrorImportRouteClass::OfflineBundle => {
            source_class == CatalogRegistrySourceClass::OfflineBundle
        }
        MirrorImportRouteClass::ManualArtifact => matches!(
            source_class,
            CatalogRegistrySourceClass::LocalArchive
                | CatalogRegistrySourceClass::QuarantinedLocalCopy
        ),
    }
}

fn decision_label(decision: ConformanceDecisionClass) -> &'static str {
    match decision {
        ConformanceDecisionClass::PublishReady => "Publish ready",
        ConformanceDecisionClass::PublishReadyWithRecommendations => {
            "Publish ready (recommendations)"
        }
        ConformanceDecisionClass::BlockedOnConformance => "Blocked on conformance",
    }
}

fn status_glyph(status: ReviewCheckStatusClass) -> &'static str {
    match status {
        ReviewCheckStatusClass::Pass => "✅",
        ReviewCheckStatusClass::Fail => "❌",
        ReviewCheckStatusClass::Warn => "⚠️",
        ReviewCheckStatusClass::NotApplicable => "➖",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn report_to_input(report: &ExtensionConformanceReport) -> ConformanceReportInput {
    ConformanceReportInput {
        report_id: report.report_id.clone(),
        extension_identity: report.extension_identity.clone(),
        extension_version: report.extension_version.clone(),
        package_id: report.package_id.clone(),
        publisher_id: report.publisher_id.clone(),
        validator_id: report.validator_id.clone(),
        validator_version: report.validator_version.clone(),
        subject_manifest_ref: report.subject_manifest_ref.clone(),
        lifecycle_class: report.lifecycle_class,
        checks: report.checks.clone(),
        compatibility: report.compatibility.clone(),
        generated_at: report.generated_at.clone(),
    }
}

fn review_to_input(review: &MirrorBundleReview) -> MirrorBundleReviewInput {
    MirrorBundleReviewInput {
        review_id: review.review_id.clone(),
        extension_identity: review.extension_identity.clone(),
        extension_version: review.extension_version.clone(),
        package_id: review.package_id.clone(),
        lifecycle_class: review.lifecycle_class,
        artifact: review.artifact.clone(),
        signing_provenance: review.signing_provenance.clone(),
        source: review.source.clone(),
        dependency_graph: review.dependency_graph.clone(),
        reproducibility: review.reproducibility.clone(),
        downgraded_trust_claim_count: review.downgraded_trust_claim_count,
        checks: review.checks.clone(),
        generated_at: review.generated_at.clone(),
    }
}
