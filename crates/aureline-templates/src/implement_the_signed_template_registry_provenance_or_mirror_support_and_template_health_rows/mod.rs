//! Signed template registry with provenance/mirror support and template-health rows.
//!
//! This module locks the canonical, export-safe packet for the signed template
//! registry. Each [`SignedTemplateRegistryRow`] binds one offerable template
//! revision to its registry origin and mirror lineage, its signing trust source
//! and signature class, its certification and support class, its declared
//! freshness, and its template-health state and cadence — so the gallery,
//! scaffold preflight, run, recovery, diagnostics, and support surfaces project
//! the same trust, provenance, and health truth instead of inferring it from a
//! mirror's location or collapsing health to a single pass/fail bit.
//!
//! The packet is metadata only. Raw signing keys, certificate material,
//! repository URLs, absolute paths, manifest bodies, hook bodies, secrets, and
//! user-authored template content never cross this boundary; rows carry opaque
//! refs, closed-vocabulary class tokens, content digests by reference, and short
//! reviewable summaries. It references the upstream
//! [`template_registry_entry`](TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF) record by id
//! rather than embedding it.
//!
//! [`SignedTemplateRegistryPacket::apply_downgrade_automation`] narrows rows whose
//! signature or trust root failed verification, whose mirror went stale, whose
//! template-health checks aged out, or whose proof or upstream dependency
//! narrowed — withholding generation and marking the blocking state rather than
//! hiding the row, so CI or release tooling narrows a stale or underqualified row
//! before it is offered.
//!
//! The boundary schema is
//! [`schemas/templates/implement-the-signed-template-registry-provenance-or-mirror-support-and-template-health-rows.schema.json`](../../../../schemas/templates/implement-the-signed-template-registry-provenance-or-mirror-support-and-template-health-rows.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows.md`](../../../../docs/frameworks/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/`](../../../../fixtures/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SignedTemplateRegistryPacket`].
pub const SIGNED_TEMPLATE_REGISTRY_RECORD_KIND: &str =
    "signed_template_registry_provenance_and_health_rows";

/// Schema version for signed template-registry packets.
pub const SIGNED_TEMPLATE_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SIGNED_TEMPLATE_REGISTRY_SCHEMA_REF: &str =
    "schemas/templates/implement-the-signed-template-registry-provenance-or-mirror-support-and-template-health-rows.schema.json";

/// Repo-relative path of the registry contract doc.
pub const SIGNED_TEMPLATE_REGISTRY_DOC_REF: &str =
    "docs/frameworks/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows.md";

/// Repo-relative path of the upstream per-entry registry contract this packet projects.
pub const TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF: &str =
    "schemas/templates/template_registry_entry.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const SIGNED_TEMPLATE_REGISTRY_FIXTURE_DIR: &str =
    "fixtures/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows";

/// Repo-relative path of the checked support-export artifact.
pub const SIGNED_TEMPLATE_REGISTRY_ARTIFACT_REF: &str =
    "artifacts/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/support_export.json";

/// Registry origin and mirror lineage class for a template row.
///
/// Mirrors the `template_registry_origin_class` vocabulary of the upstream
/// [`template_registry_entry`](TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF) contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateRegistryOriginClass {
    /// First-party origin published by the IDE vendor.
    OfficialOrigin,
    /// Vendor-operated mirror of the official origin.
    OfficialMirror,
    /// Organization-operated mirror of an upstream origin.
    OrgMirror,
    /// Community-published origin.
    CommunityOrigin,
    /// Provided by an installed extension.
    ExtensionProvided,
    /// Generator that lives inside the current repository.
    RepoLocalGenerator,
    /// Ad hoc template authored locally for this workspace only.
    AdHocLocalTemplate,
    /// Signed bundle resolved while offline.
    OfflineBundle,
}

impl TemplateRegistryOriginClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialOrigin => "official_origin",
            Self::OfficialMirror => "official_mirror",
            Self::OrgMirror => "org_mirror",
            Self::CommunityOrigin => "community_origin",
            Self::ExtensionProvided => "extension_provided",
            Self::RepoLocalGenerator => "repo_local_generator",
            Self::AdHocLocalTemplate => "ad_hoc_local_template",
            Self::OfflineBundle => "offline_bundle",
        }
    }

    /// Whether this origin preserves an upstream origin behind a mirror or bundle.
    pub const fn is_mirror(self) -> bool {
        matches!(
            self,
            Self::OfficialMirror | Self::OrgMirror | Self::OfflineBundle
        )
    }

    /// Whether this origin resolves through local-only trust.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::RepoLocalGenerator | Self::AdHocLocalTemplate)
    }
}

/// Signing trust source a template row resolves through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateTrustSourceClass {
    /// Core vendor signing root.
    CoreSigningRoot,
    /// Core vendor signing root reached through a mirror.
    CoreSigningRootViaMirror,
    /// Organization policy signing root.
    OrgPolicySigningRoot,
    /// Organization mirror signing root.
    OrgMirrorSigningRoot,
    /// Extension publisher signature.
    ExtensionPublisherSignature,
    /// Community channel signature.
    CommunityChannelSignature,
    /// Signed offline-bundle root.
    SignedOfflineBundleRoot,
    /// Repo-local workspace trust.
    RepoLocalWorkspaceTrust,
    /// Local user trust only.
    LocalUserTrustOnly,
    /// Unsigned; user review required before offer.
    UnsignedUserReviewRequired,
    /// Trust source unknown; user review required.
    TrustSourceUnknownReviewRequired,
}

impl TemplateTrustSourceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreSigningRoot => "core_signing_root",
            Self::CoreSigningRootViaMirror => "core_signing_root_via_mirror",
            Self::OrgPolicySigningRoot => "org_policy_signing_root",
            Self::OrgMirrorSigningRoot => "org_mirror_signing_root",
            Self::ExtensionPublisherSignature => "extension_publisher_signature",
            Self::CommunityChannelSignature => "community_channel_signature",
            Self::SignedOfflineBundleRoot => "signed_offline_bundle_root",
            Self::RepoLocalWorkspaceTrust => "repo_local_workspace_trust",
            Self::LocalUserTrustOnly => "local_user_trust_only",
            Self::UnsignedUserReviewRequired => "unsigned_user_review_required",
            Self::TrustSourceUnknownReviewRequired => "trust_source_unknown_review_required",
        }
    }
}

/// Signature posture a template row carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSignatureClass {
    /// No signature present.
    Unsigned,
    /// Signed by the template author only.
    AuthorSignature,
    /// Signed by both author and organization.
    AuthorAndOrganizationSignature,
    /// Signed by the organization only.
    OrganizationSignatureOnly,
    /// Signed only by a managed channel.
    ManagedOnlyChannelSignature,
    /// Quarantined; no usable signature.
    QuarantinedNoSignature,
}

impl TemplateSignatureClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsigned => "unsigned",
            Self::AuthorSignature => "author_signature",
            Self::AuthorAndOrganizationSignature => "author_and_organization_signature",
            Self::OrganizationSignatureOnly => "organization_signature_only",
            Self::ManagedOnlyChannelSignature => "managed_only_channel_signature",
            Self::QuarantinedNoSignature => "quarantined_no_signature",
        }
    }
}

/// Certification class claimed for a template row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCertificationClass {
    /// Core-certified by the vendor.
    CoreCertified,
    /// Officially supported by the vendor.
    OfficialSupported,
    /// Organization-certified.
    OrgCertified,
    /// Organization-approved.
    OrgApproved,
    /// Community-reviewed.
    CommunityReviewed,
    /// Community-published but unreviewed.
    CommunityUnreviewed,
    /// Claimed by an extension publisher.
    ExtensionPublisherClaimed,
    /// Repo-local and unreviewed.
    RepoLocalUnreviewed,
    /// Local-only and unreviewed.
    LocalOnlyUnreviewed,
    /// Deprecated or archived.
    DeprecatedOrArchived,
    /// Certification unknown; review required.
    CertificationUnknownReviewRequired,
}

impl TemplateCertificationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreCertified => "core_certified",
            Self::OfficialSupported => "official_supported",
            Self::OrgCertified => "org_certified",
            Self::OrgApproved => "org_approved",
            Self::CommunityReviewed => "community_reviewed",
            Self::CommunityUnreviewed => "community_unreviewed",
            Self::ExtensionPublisherClaimed => "extension_publisher_claimed",
            Self::RepoLocalUnreviewed => "repo_local_unreviewed",
            Self::LocalOnlyUnreviewed => "local_only_unreviewed",
            Self::DeprecatedOrArchived => "deprecated_or_archived",
            Self::CertificationUnknownReviewRequired => "certification_unknown_review_required",
        }
    }

    /// Whether this certification may only attach to a local-origin row.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::RepoLocalUnreviewed | Self::LocalOnlyUnreviewed)
    }
}

/// Support class communicated for a template row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSupportClass {
    /// Officially supported by the vendor.
    OfficiallySupported,
    /// Community-supported, best effort.
    CommunitySupported,
    /// Experimental; may change without notice.
    Experimental,
    /// Legacy and deprecated.
    LegacyDeprecated,
    /// Explicitly unsupported.
    Unsupported,
    /// Support class unknown.
    SupportUnknown,
}

impl TemplateSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficiallySupported => "officially_supported",
            Self::CommunitySupported => "community_supported",
            Self::Experimental => "experimental",
            Self::LegacyDeprecated => "legacy_deprecated",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }
}

/// Declared freshness of a template row's source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateFreshnessClass {
    /// Resolved live from the origin.
    LiveOrigin,
    /// Mirror is fresh against its origin.
    MirrorFresh,
    /// Mirror lags its origin but is inspectable.
    MirrorLagged,
    /// Mirror is stale against its origin.
    MirrorStale,
    /// Offline snapshot of unknown age.
    OfflineSnapshot,
    /// Signed offline bundle.
    SignedOfflineBundle,
    /// Freshness unknown.
    UnknownFreshness,
}

impl TemplateFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveOrigin => "live_origin",
            Self::MirrorFresh => "mirror_fresh",
            Self::MirrorLagged => "mirror_lagged",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::SignedOfflineBundle => "signed_offline_bundle",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }
}

/// Cadence at which a template row's health is re-checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateHealthCadenceClass {
    /// Checked on every registry refresh.
    OnEveryRegistryRefresh,
    /// Checked daily.
    Daily,
    /// Checked weekly.
    Weekly,
    /// Checked per release train.
    PerReleaseTrain,
    /// Checked when the template revision changes.
    OnTemplateRevisionChange,
    /// Checked only when manually requested.
    ManualOnly,
    /// No schedule; review required.
    NotScheduledReviewRequired,
}

impl TemplateHealthCadenceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnEveryRegistryRefresh => "on_every_registry_refresh",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::PerReleaseTrain => "per_release_train",
            Self::OnTemplateRevisionChange => "on_template_revision_change",
            Self::ManualOnly => "manual_only",
            Self::NotScheduledReviewRequired => "not_scheduled_review_required",
        }
    }
}

/// Template-health state for a row — partitioned, not collapsed to pass/fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateHealthStateClass {
    /// Healthy and current.
    HealthyCurrent,
    /// Healthy from a cached check.
    HealthyCached,
    /// Stale but still inspectable.
    StaleButInspectable,
    /// A known issue is present but non-blocking.
    KnownIssueNonBlocking,
    /// A known issue blocks starter generation.
    KnownIssueBlocksStarter,
    /// Validation failed; starter generation is blocked.
    ValidationFailedBlocksStarter,
    /// Signature or trust verification failed; starter generation is blocked.
    SignatureOrTrustFailedBlocksStarter,
    /// Health unknown; review required.
    HealthUnknownReviewRequired,
}

impl TemplateHealthStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HealthyCurrent => "healthy_current",
            Self::HealthyCached => "healthy_cached",
            Self::StaleButInspectable => "stale_but_inspectable",
            Self::KnownIssueNonBlocking => "known_issue_non_blocking",
            Self::KnownIssueBlocksStarter => "known_issue_blocks_starter",
            Self::ValidationFailedBlocksStarter => "validation_failed_blocks_starter",
            Self::SignatureOrTrustFailedBlocksStarter => "signature_or_trust_failed_blocks_starter",
            Self::HealthUnknownReviewRequired => "health_unknown_review_required",
        }
    }

    /// Whether this state blocks starter generation.
    pub const fn blocks_starter(self) -> bool {
        matches!(
            self,
            Self::KnownIssueBlocksStarter
                | Self::ValidationFailedBlocksStarter
                | Self::SignatureOrTrustFailedBlocksStarter
                | Self::HealthUnknownReviewRequired
        )
    }

    /// Whether this state is one of the healthy, non-blocking states.
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::HealthyCurrent | Self::HealthyCached)
    }
}

/// Downgrade trigger that can narrow a template row below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateRegistryDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A signature failed verification or expired.
    SignatureUnverified,
    /// The trust root could not be resolved.
    TrustRootUnresolved,
    /// A mirror went stale against its origin.
    MirrorStale,
    /// A mirror's upstream origin could not be verified.
    MirrorOriginUnverifiable,
    /// The signed template revision is unavailable.
    TemplateRevisionUnavailable,
    /// Template-health checks aged out of their cadence.
    HealthCheckStale,
    /// A validation bundle failed.
    ValidationFailed,
    /// A blocking known issue applies.
    KnownIssueBlocking,
    /// The certification claim expired.
    CertificationExpired,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl TemplateRegistryDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::SignatureUnverified,
        Self::TrustRootUnresolved,
        Self::MirrorStale,
        Self::MirrorOriginUnverifiable,
        Self::TemplateRevisionUnavailable,
        Self::HealthCheckStale,
        Self::ValidationFailed,
        Self::KnownIssueBlocking,
        Self::CertificationExpired,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::SignatureUnverified => "signature_unverified",
            Self::TrustRootUnresolved => "trust_root_unresolved",
            Self::MirrorStale => "mirror_stale",
            Self::MirrorOriginUnverifiable => "mirror_origin_unverifiable",
            Self::TemplateRevisionUnavailable => "template_revision_unavailable",
            Self::HealthCheckStale => "health_check_stale",
            Self::ValidationFailed => "validation_failed",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::CertificationExpired => "certification_expired",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a template row's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateRegistryConsumerSurface {
    /// Template/starter gallery.
    Gallery,
    /// Scaffold preflight sheet.
    Preflight,
    /// Scaffold run surface.
    RunSurface,
    /// Recovery / rollback surface.
    Recovery,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl TemplateRegistryConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Gallery,
        Self::Preflight,
        Self::RunSurface,
        Self::Recovery,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gallery => "gallery",
            Self::Preflight => "preflight",
            Self::RunSurface => "run_surface",
            Self::Recovery => "recovery",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One signed-registry row: a single offerable template revision and its truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTemplateRegistryRow {
    /// Opaque stable row id.
    pub row_id: String,
    /// Opaque stable template id.
    pub template_id: String,
    /// Template revision semver.
    pub template_revision_semver: String,
    /// Opaque ref to the upstream per-entry registry record this row projects.
    pub registry_entry_ref: String,
    /// Short reviewable scope summary.
    pub scope_summary: String,
    /// Registry origin and mirror lineage class.
    pub origin_class: TemplateRegistryOriginClass,
    /// Preserved upstream origin for mirror/bundle rows.
    pub mirrored_from_origin_class: Option<TemplateRegistryOriginClass>,
    /// Opaque mirror-freshness ref for mirror/bundle rows.
    pub mirror_freshness_ref: Option<String>,
    /// Declared freshness of this row's source.
    pub declared_freshness_class: TemplateFreshnessClass,
    /// Signing trust source this row resolves through.
    pub trust_source_class: TemplateTrustSourceClass,
    /// Opaque trust-root ref for non-local rows.
    pub trust_root_ref: Option<String>,
    /// Signature posture.
    pub signature_class: TemplateSignatureClass,
    /// Certification class claimed.
    pub certification_class: TemplateCertificationClass,
    /// Support class communicated.
    pub support_class: TemplateSupportClass,
    /// Cadence at which this row's health is re-checked.
    pub health_cadence_class: TemplateHealthCadenceClass,
    /// Template-health state.
    pub health_state_class: TemplateHealthStateClass,
    /// Opaque health-check refs.
    pub health_check_refs: Vec<String>,
    /// Opaque known-issue refs disclosed before generation.
    pub known_issue_refs: Vec<String>,
    /// Whether this row is admitted for starter generation.
    pub admitted_for_generation: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<TemplateRegistryDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<TemplateRegistryConsumerSurface>,
}

impl SignedTemplateRegistryRow {
    /// Whether this row preserves an upstream origin behind a mirror or bundle.
    pub const fn is_mirror(&self) -> bool {
        self.origin_class.is_mirror()
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTemplateRegistryTrustReview {
    /// Template provenance stays inspectable from gallery through recovery.
    pub template_provenance_inspectable: bool,
    /// Signing root is verified before a template is offered.
    pub signing_root_verified_before_offer: bool,
    /// Mirror rows preserve their upstream origin and carry mirror freshness.
    pub mirror_origin_and_freshness_preserved: bool,
    /// Trust is never inferred from a mirror's location.
    pub no_trust_inferred_from_mirror_location: bool,
    /// Signature, certification, and support class stay explicit on every row.
    pub signature_and_certification_class_explicit: bool,
    /// Template-health state stays partitioned from the admission decision.
    pub health_state_partitioned_from_admission: bool,
    /// Known issues are disclosed before generation.
    pub known_issues_disclosed_before_generation: bool,
    /// Blocking health states block starter generation.
    pub blocking_health_blocks_starter: bool,
    /// No credential bodies or key material cross the export boundary.
    pub no_credential_or_key_material_in_export: bool,
    /// Downgrade narrows the row's claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTemplateRegistryConsumerProjection {
    /// Gallery shows origin class and support class.
    pub gallery_shows_origin_and_support_class: bool,
    /// Gallery shows template-health state.
    pub gallery_shows_health_state: bool,
    /// Preflight shows trust source and declared freshness.
    pub preflight_shows_trust_and_freshness: bool,
    /// Run surface shows template revision and certification.
    pub run_surface_shows_revision_and_certification: bool,
    /// Recovery surface shows rollback boundary and known issues.
    pub recovery_shows_rollback_and_known_issues: bool,
    /// CLI / headless shows registry rows.
    pub cli_headless_shows_registry_rows: bool,
    /// Support export shows registry rows.
    pub support_export_shows_registry_rows: bool,
    /// Diagnostics shows template-health state.
    pub diagnostics_shows_health_state: bool,
    /// Blocked rows are visibly labeled rather than hidden.
    pub blocked_rows_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTemplateRegistryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`SignedTemplateRegistryPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTemplateRegistryRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when the row's signature currently verifies.
    pub signature_valid: bool,
    /// True when the row's trust root currently resolves.
    pub trust_root_resolved: bool,
    /// True when a mirror row is fresh against its origin (ignored for non-mirror rows).
    pub mirror_fresh: bool,
    /// True when the row's template-health checks are within cadence.
    pub health_current: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`SignedTemplateRegistryPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTemplateRegistryPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registry label.
    pub registry_label: String,
    /// Registry rows.
    pub rows: Vec<SignedTemplateRegistryRow>,
    /// Trust review block.
    pub trust_review: SignedTemplateRegistryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: SignedTemplateRegistryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: SignedTemplateRegistryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe signed template-registry packet with provenance and health rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTemplateRegistryPacket {
    /// Record kind; must equal [`SIGNED_TEMPLATE_REGISTRY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SIGNED_TEMPLATE_REGISTRY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registry label.
    pub registry_label: String,
    /// Registry rows.
    pub rows: Vec<SignedTemplateRegistryRow>,
    /// Trust review block.
    pub trust_review: SignedTemplateRegistryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: SignedTemplateRegistryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: SignedTemplateRegistryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SignedTemplateRegistryPacket {
    /// Builds a signed template-registry packet from stable-row input.
    pub fn new(input: SignedTemplateRegistryPacketInput) -> Self {
        Self {
            record_kind: SIGNED_TEMPLATE_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: SIGNED_TEMPLATE_REGISTRY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registry_label: input.registry_label,
            rows: input.rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows rows whose signature or trust failed, whose mirror went stale,
    /// whose health checks aged out, or whose proof or upstream narrowed.
    ///
    /// A failed signature or trust root is the hardest block: the row is marked
    /// signature/trust-failed, its certification and support drop to the
    /// review-required and unknown classes, and it loses admission. A stale
    /// mirror loses admission and is marked stale-but-inspectable. Aged-out
    /// health narrows a currently-healthy row to stale-but-inspectable. Stale
    /// proof or a narrowed upstream withholds admission until evidence refreshes.
    /// Rows without a matching observation are left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[SignedTemplateRegistryRowObservation],
    ) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.signature_valid || !observation.trust_root_resolved {
                row.health_state_class =
                    TemplateHealthStateClass::SignatureOrTrustFailedBlocksStarter;
                row.certification_class =
                    TemplateCertificationClass::CertificationUnknownReviewRequired;
                row.support_class = TemplateSupportClass::SupportUnknown;
                row.admitted_for_generation = false;
                let trigger = if observation.signature_valid {
                    TemplateRegistryDowngradeTrigger::TrustRootUnresolved
                } else {
                    TemplateRegistryDowngradeTrigger::SignatureUnverified
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
                continue;
            }

            if row.is_mirror() && !observation.mirror_fresh {
                row.declared_freshness_class = TemplateFreshnessClass::MirrorStale;
                if !row.health_state_class.blocks_starter() {
                    row.health_state_class = TemplateHealthStateClass::StaleButInspectable;
                }
                row.admitted_for_generation = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    TemplateRegistryDowngradeTrigger::MirrorStale,
                );
            }

            if !observation.health_current && row.health_state_class.is_healthy() {
                row.health_state_class = TemplateHealthStateClass::StaleButInspectable;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    TemplateRegistryDowngradeTrigger::HealthCheckStale,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.admitted_for_generation
            {
                row.admitted_for_generation = false;
                let trigger = if observation.proof_fresh {
                    TemplateRegistryDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    TemplateRegistryDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the signed template-registry invariants.
    pub fn validate(&self) -> Vec<SignedTemplateRegistryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SIGNED_TEMPLATE_REGISTRY_RECORD_KIND {
            violations.push(SignedTemplateRegistryViolation::WrongRecordKind);
        }
        if self.schema_version != SIGNED_TEMPLATE_REGISTRY_SCHEMA_VERSION {
            violations.push(SignedTemplateRegistryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registry_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SignedTemplateRegistryViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("signed template-registry packet serializes"),
        ) {
            violations.push(SignedTemplateRegistryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("signed template-registry packet serializes")
    }

    /// Rows currently admitted for starter generation.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &SignedTemplateRegistryRow> {
        self.rows.iter().filter(|row| row.admitted_for_generation)
    }

    /// Deterministic Markdown summary for gallery, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str("# Signed Template Registry, Provenance/Mirror, and Template-Health Rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registry_label));
        out.push_str(&format!(
            "- Rows: {} ({} admitted for generation)\n",
            self.rows.len(),
            admitted
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** `{}`: {} / {} ({})\n",
                row.template_id,
                row.template_revision_semver,
                row.origin_class.as_str(),
                row.support_class.as_str(),
                row.certification_class.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Trust: {} (signature: {})\n",
                row.trust_source_class.as_str(),
                row.signature_class.as_str()
            ));
            out.push_str(&format!(
                "  - Freshness: {}\n",
                row.declared_freshness_class.as_str()
            ));
            out.push_str(&format!(
                "  - Health: {} (cadence: {}, admitted: {})\n",
                row.health_state_class.as_str(),
                row.health_cadence_class.as_str(),
                row.admitted_for_generation
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in signed template-registry export.
#[derive(Debug)]
pub enum SignedTemplateRegistryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SignedTemplateRegistryViolation>),
}

impl fmt::Display for SignedTemplateRegistryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "signed template-registry export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "signed template-registry export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SignedTemplateRegistryArtifactError {}

/// Validation failures emitted by [`SignedTemplateRegistryPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignedTemplateRegistryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The registry carries no rows.
    RowsEmpty,
    /// A row is incomplete.
    RowIncomplete,
    /// A mirror/bundle row is missing its preserved origin or freshness ref.
    MirrorProvenanceIncomplete,
    /// A non-local row is missing its trust-root ref.
    TrustRootMissing,
    /// A row's trust source does not match its origin class.
    TrustSourceMismatch,
    /// A local-origin row inflated its certification beyond a local class.
    LocalCertificationInflated,
    /// A blocking-health row is still admitted for generation.
    BlockingHealthAdmitted,
    /// A row has no health-check refs.
    HealthCheckRefsMissing,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl SignedTemplateRegistryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::MirrorProvenanceIncomplete => "mirror_provenance_incomplete",
            Self::TrustRootMissing => "trust_root_missing",
            Self::TrustSourceMismatch => "trust_source_mismatch",
            Self::LocalCertificationInflated => "local_certification_inflated",
            Self::BlockingHealthAdmitted => "blocking_health_admitted",
            Self::HealthCheckRefsMissing => "health_check_refs_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in signed template-registry export.
///
/// This is the first real consumer of the signed registry: a gallery,
/// preflight, diagnostics, or support-export surface calls it to ingest the
/// canonical packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`SignedTemplateRegistryArtifactError`] when the checked-in support
/// export fails to parse or fails validation.
pub fn current_signed_template_registry_export(
) -> Result<SignedTemplateRegistryPacket, SignedTemplateRegistryArtifactError> {
    let packet: SignedTemplateRegistryPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/support_export.json"
    )))
    .map_err(SignedTemplateRegistryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SignedTemplateRegistryArtifactError::Validation(violations))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> SignedTemplateRegistryTrustReview {
    SignedTemplateRegistryTrustReview {
        template_provenance_inspectable: true,
        signing_root_verified_before_offer: true,
        mirror_origin_and_freshness_preserved: true,
        no_trust_inferred_from_mirror_location: true,
        signature_and_certification_class_explicit: true,
        health_state_partitioned_from_admission: true,
        known_issues_disclosed_before_generation: true,
        blocking_health_blocks_starter: true,
        no_credential_or_key_material_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting row truth.
pub fn canonical_consumer_projection() -> SignedTemplateRegistryConsumerProjection {
    SignedTemplateRegistryConsumerProjection {
        gallery_shows_origin_and_support_class: true,
        gallery_shows_health_state: true,
        preflight_shows_trust_and_freshness: true,
        run_surface_shows_revision_and_certification: true,
        recovery_shows_rollback_and_known_issues: true,
        cli_headless_shows_registry_rows: true,
        support_export_shows_registry_rows: true,
        diagnostics_shows_health_state: true,
        blocked_rows_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every registry export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        SIGNED_TEMPLATE_REGISTRY_SCHEMA_REF.to_owned(),
        SIGNED_TEMPLATE_REGISTRY_DOC_REF.to_owned(),
        TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical signed template registry from stable-row truth.
///
/// The rows mirror the checked-in support export and cover the provenance and
/// mirror spectrum: an official-origin first-party row admitted for generation,
/// a stale organization-mirror row held until refresh, a community-origin row,
/// and a repo-local generator row.
pub fn canonical_signed_template_registry(
    packet_id: String,
    registry_label: String,
    minted_at: String,
    proof_freshness: SignedTemplateRegistryProofFreshness,
) -> SignedTemplateRegistryPacket {
    SignedTemplateRegistryPacket::new(SignedTemplateRegistryPacketInput {
        packet_id,
        registry_label,
        rows: canonical_rows(),
        trust_review: canonical_trust_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical rows that match the checked-in support export.
pub fn canonical_rows() -> Vec<SignedTemplateRegistryRow> {
    use TemplateRegistryConsumerSurface as Surface;
    use TemplateRegistryDowngradeTrigger as Trigger;

    vec![
        SignedTemplateRegistryRow {
            row_id: "registry-row:official.rust.cli:2026.04".to_owned(),
            template_id: "template:first_party.rust.cli_tool:01".to_owned(),
            template_revision_semver: "0.4.2".to_owned(),
            registry_entry_ref: "registry-entry:official.rust.cli:2026.04".to_owned(),
            scope_summary: "Officially-supported Rust CLI starter anchored in the core signing root; trust, certification, and update behavior stay inspectable before generation".to_owned(),
            origin_class: TemplateRegistryOriginClass::OfficialOrigin,
            mirrored_from_origin_class: None,
            mirror_freshness_ref: None,
            declared_freshness_class: TemplateFreshnessClass::LiveOrigin,
            trust_source_class: TemplateTrustSourceClass::CoreSigningRoot,
            trust_root_ref: Some("trust-root:aureline.core.templates:2026.04".to_owned()),
            signature_class: TemplateSignatureClass::AuthorAndOrganizationSignature,
            certification_class: TemplateCertificationClass::CoreCertified,
            support_class: TemplateSupportClass::OfficiallySupported,
            health_cadence_class: TemplateHealthCadenceClass::OnEveryRegistryRefresh,
            health_state_class: TemplateHealthStateClass::HealthyCurrent,
            health_check_refs: vec![
                "health:official_rust_cli:signature_state:2026.04.20".to_owned(),
                "health:official_rust_cli:toolchain_compatibility:2026.04.20".to_owned(),
            ],
            known_issue_refs: vec![],
            admitted_for_generation: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::SignatureUnverified,
                Trigger::TrustRootUnresolved,
                Trigger::TemplateRevisionUnavailable,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::CliHeadless,
                Surface::SupportExport,
            ],
        },
        SignedTemplateRegistryRow {
            row_id: "registry-row:org_mirror.ts_web:2026.04".to_owned(),
            template_id: "template:official.ts.web_application:01".to_owned(),
            template_revision_semver: "1.8.0".to_owned(),
            registry_entry_ref: "registry-entry:org_mirror.ts_web:2026.04".to_owned(),
            scope_summary: "Organization mirror of the official TypeScript web app starter; the mirror is stale, so generation requires refresh or review while the upstream origin and mirror freshness stay inspectable".to_owned(),
            origin_class: TemplateRegistryOriginClass::OrgMirror,
            mirrored_from_origin_class: Some(TemplateRegistryOriginClass::OfficialOrigin),
            mirror_freshness_ref: Some(
                "mirror-freshness:org.templates:ts_web:2026.04.18-02".to_owned(),
            ),
            declared_freshness_class: TemplateFreshnessClass::MirrorStale,
            trust_source_class: TemplateTrustSourceClass::OrgMirrorSigningRoot,
            trust_root_ref: Some("trust-root:org.platform.templates:2026.04".to_owned()),
            signature_class: TemplateSignatureClass::OrganizationSignatureOnly,
            certification_class: TemplateCertificationClass::OrgApproved,
            support_class: TemplateSupportClass::OfficiallySupported,
            health_cadence_class: TemplateHealthCadenceClass::Daily,
            health_state_class: TemplateHealthStateClass::StaleButInspectable,
            health_check_refs: vec![
                "health:org_mirror_ts_web:signature_state:2026.04.19".to_owned(),
                "health:org_mirror_ts_web:dependency_manifest:2026.04.19".to_owned(),
            ],
            known_issue_refs: vec![
                "known-issue:org_mirror_ts_web:mirror_lag_disclosed".to_owned(),
            ],
            admitted_for_generation: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::MirrorStale,
                Trigger::MirrorOriginUnverifiable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        SignedTemplateRegistryRow {
            row_id: "registry-row:community.python.data:2026.03".to_owned(),
            template_id: "template:community.python.data_workbench:07".to_owned(),
            template_revision_semver: "2.1.0".to_owned(),
            registry_entry_ref: "registry-entry:community.python.data:2026.03".to_owned(),
            scope_summary: "Community-reviewed Python data workbench starter signed by its channel; support class and any known issues stay explicit and bridge behavior is never shown as exact first-party truth".to_owned(),
            origin_class: TemplateRegistryOriginClass::CommunityOrigin,
            mirrored_from_origin_class: None,
            mirror_freshness_ref: None,
            declared_freshness_class: TemplateFreshnessClass::LiveOrigin,
            trust_source_class: TemplateTrustSourceClass::CommunityChannelSignature,
            trust_root_ref: Some("trust-root:community.templates.channel:2026.03".to_owned()),
            signature_class: TemplateSignatureClass::AuthorSignature,
            certification_class: TemplateCertificationClass::CommunityReviewed,
            support_class: TemplateSupportClass::CommunitySupported,
            health_cadence_class: TemplateHealthCadenceClass::Weekly,
            health_state_class: TemplateHealthStateClass::KnownIssueNonBlocking,
            health_check_refs: vec![
                "health:community_python_data:dependency_manifest:2026.03.30".to_owned(),
            ],
            known_issue_refs: vec![
                "known-issue:community_python_data:optional_gpu_extra".to_owned(),
            ],
            admitted_for_generation: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::SignatureUnverified,
                Trigger::HealthCheckStale,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::RunSurface,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        SignedTemplateRegistryRow {
            row_id: "registry-row:repo_local.web.service:2026.05".to_owned(),
            template_id: "template:repo_local.node.backend_service:02".to_owned(),
            template_revision_semver: "0.1.0".to_owned(),
            registry_entry_ref: "registry-entry:repo_local.web.service:2026.05".to_owned(),
            scope_summary: "Repo-local generator scoped to this workspace; resolves through workspace trust only and cannot claim certification by location".to_owned(),
            origin_class: TemplateRegistryOriginClass::RepoLocalGenerator,
            mirrored_from_origin_class: None,
            mirror_freshness_ref: None,
            declared_freshness_class: TemplateFreshnessClass::UnknownFreshness,
            trust_source_class: TemplateTrustSourceClass::RepoLocalWorkspaceTrust,
            trust_root_ref: None,
            signature_class: TemplateSignatureClass::Unsigned,
            certification_class: TemplateCertificationClass::RepoLocalUnreviewed,
            support_class: TemplateSupportClass::SupportUnknown,
            health_cadence_class: TemplateHealthCadenceClass::OnTemplateRevisionChange,
            health_state_class: TemplateHealthStateClass::HealthyCached,
            health_check_refs: vec![
                "health:repo_local_node_service:workspace_resolve:2026.05.02".to_owned(),
            ],
            known_issue_refs: vec![],
            admitted_for_generation: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::TemplateRevisionUnavailable,
                Trigger::HealthCheckStale,
                Trigger::UpstreamDependencyNarrowed,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &SignedTemplateRegistryPacket,
    violations: &mut Vec<SignedTemplateRegistryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SIGNED_TEMPLATE_REGISTRY_SCHEMA_REF,
        SIGNED_TEMPLATE_REGISTRY_DOC_REF,
        TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SignedTemplateRegistryViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &SignedTemplateRegistryPacket,
    violations: &mut Vec<SignedTemplateRegistryViolation>,
) {
    if packet.rows.is_empty() {
        violations.push(SignedTemplateRegistryViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.template_id.trim().is_empty()
            || row.template_revision_semver.trim().is_empty()
            || row.registry_entry_ref.trim().is_empty()
            || row.scope_summary.trim().is_empty()
        {
            violations.push(SignedTemplateRegistryViolation::RowIncomplete);
        }
        if row.health_check_refs.is_empty() {
            violations.push(SignedTemplateRegistryViolation::HealthCheckRefsMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(SignedTemplateRegistryViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(SignedTemplateRegistryViolation::ConsumerSurfacesMissing);
        }

        validate_row_provenance(row, violations);

        if row.health_state_class.blocks_starter() && row.admitted_for_generation {
            violations.push(SignedTemplateRegistryViolation::BlockingHealthAdmitted);
        }
    }
}

fn validate_row_provenance(
    row: &SignedTemplateRegistryRow,
    violations: &mut Vec<SignedTemplateRegistryViolation>,
) {
    use TemplateRegistryOriginClass as Origin;
    use TemplateTrustSourceClass as Trust;

    // Mirror and bundle rows preserve their upstream origin and carry freshness.
    if row.is_mirror()
        && (row.mirrored_from_origin_class.is_none()
            || row
                .mirror_freshness_ref
                .as_deref()
                .map(str::trim)
                .map_or(true, str::is_empty))
    {
        violations.push(SignedTemplateRegistryViolation::MirrorProvenanceIncomplete);
    }

    // Non-local rows cite a trust-root ref.
    if !row.origin_class.is_local()
        && row
            .trust_root_ref
            .as_deref()
            .map(str::trim)
            .map_or(true, str::is_empty)
    {
        violations.push(SignedTemplateRegistryViolation::TrustRootMissing);
    }

    // Trust source must match the origin class for first-party, org-mirror, and local rows.
    let trust_matches = match row.origin_class {
        Origin::OfficialOrigin | Origin::OfficialMirror => matches!(
            row.trust_source_class,
            Trust::CoreSigningRoot | Trust::CoreSigningRootViaMirror
        ),
        Origin::OrgMirror => matches!(
            row.trust_source_class,
            Trust::OrgPolicySigningRoot | Trust::OrgMirrorSigningRoot
        ),
        Origin::RepoLocalGenerator | Origin::AdHocLocalTemplate => matches!(
            row.trust_source_class,
            Trust::RepoLocalWorkspaceTrust
                | Trust::LocalUserTrustOnly
                | Trust::UnsignedUserReviewRequired
        ),
        _ => true,
    };
    if !trust_matches {
        violations.push(SignedTemplateRegistryViolation::TrustSourceMismatch);
    }

    // Local-origin rows cannot inflate certification beyond a local-only class.
    if row.origin_class.is_local() && !row.certification_class.is_local_only() {
        violations.push(SignedTemplateRegistryViolation::LocalCertificationInflated);
    }
}

fn validate_trust_review(
    packet: &SignedTemplateRegistryPacket,
    violations: &mut Vec<SignedTemplateRegistryViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.template_provenance_inspectable,
        review.signing_root_verified_before_offer,
        review.mirror_origin_and_freshness_preserved,
        review.no_trust_inferred_from_mirror_location,
        review.signature_and_certification_class_explicit,
        review.health_state_partitioned_from_admission,
        review.known_issues_disclosed_before_generation,
        review.blocking_health_blocks_starter,
        review.no_credential_or_key_material_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(SignedTemplateRegistryViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &SignedTemplateRegistryPacket,
    violations: &mut Vec<SignedTemplateRegistryViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_origin_and_support_class,
        projection.gallery_shows_health_state,
        projection.preflight_shows_trust_and_freshness,
        projection.run_surface_shows_revision_and_certification,
        projection.recovery_shows_rollback_and_known_issues,
        projection.cli_headless_shows_registry_rows,
        projection.support_export_shows_registry_rows,
        projection.diagnostics_shows_health_state,
        projection.blocked_rows_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(SignedTemplateRegistryViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &SignedTemplateRegistryPacket,
    violations: &mut Vec<SignedTemplateRegistryViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(SignedTemplateRegistryViolation::ProofFreshnessIncomplete);
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<TemplateRegistryDowngradeTrigger>,
    trigger: TemplateRegistryDowngradeTrigger,
) {
    if !triggers.contains(&trigger) {
        triggers.push(trigger);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
