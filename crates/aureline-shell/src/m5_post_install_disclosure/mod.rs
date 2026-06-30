//! Post-install notice / provenance / SBOM disclosure panels for installed and
//! generated M5 artifact families.
//!
//! This module is the in-product producer of the durable
//! [`PostInstallDisclosureRecord`] that About/help, installed-state inspectors,
//! diagnostics exports, and — for extension and framework packs — marketplace /
//! package detail views project after an artifact is installed, imported,
//! mirrored, side-loaded, or exported. A user can inspect how a build or package
//! arrived, whether its signature/attestation/checksum/revocation verify, what its
//! notice/license/SBOM inventory contains, and which provenance or notice data is
//! *missing* — without returning to the original download page or guessing from a
//! channel color.
//!
//! Each record conforms to the frozen governance contract
//! [`schemas/governance/post_install_disclosure.schema.json`](../../../../schemas/governance/post_install_disclosure.schema.json)
//! and its prose contract
//! [`docs/governance/post_install_notice_and_provenance_contract.md`](../../../../docs/governance/post_install_notice_and_provenance_contract.md).
//! The [`M5PostInstallDisclosurePanelSet`] bundles one record per governed M5
//! artifact family — desktop builds/installers, extension/framework packs,
//! mirrored/offline artifacts, and generated/exported artifacts — into one
//! export-safe panel set that the help lane checks in as proof.
//!
//! The model is faithful to two acceptance invariants:
//!
//! - **Missing data is visible.** Whenever an evidence axis (signature,
//!   attestation, SBOM, license, notice inventory, or revocation snapshot) is
//!   missing, partial, unknown, stale, or policy-hidden, a typed
//!   [`MissingOrPartialDataRow`] names it. Silence never reads as "clean".
//! - **SBOM and scope stay explicit.** Every record states its SBOM state and, when
//!   an SBOM is attached, its declared formats; the artifact subject and source
//!   class are always named, so provenance/notice completeness is never claimed for
//!   an artifact that lacks it.
//!
//! Raw artifact bytes, raw signatures, raw SBOM bodies, raw notice text, raw
//! registry URLs, raw license files, raw advisory payloads, private mirror
//! endpoints, and customer identifiers never cross this boundary: the record
//! carries only opaque refs, controlled-vocabulary tokens, and reviewable
//! sentences.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_post_install_disclosure_panel_set,
    seeded_post_install_generated_export_sbom_not_provided, seeded_post_install_panels,
    seeded_post_install_product_build_signature_revoked, M5_POST_INSTALL_DISCLOSURE_PANEL_SET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by a single disclosure record.
pub const POST_INSTALL_DISCLOSURE_RECORD_KIND: &str = "post_install_disclosure_record";

/// Schema version for the post-install disclosure record.
pub const POST_INSTALL_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the M5 panel-set bundle.
pub const M5_POST_INSTALL_DISCLOSURE_PANEL_SET_KIND: &str = "m5_post_install_disclosure_panel_set";

/// Schema version for the M5 panel-set bundle.
pub const M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the governance disclosure schema this producer projects.
pub const POST_INSTALL_DISCLOSURE_SCHEMA_REF: &str =
    "schemas/governance/post_install_disclosure.schema.json";

/// Repo-relative path of the governance disclosure prose contract.
pub const POST_INSTALL_DISCLOSURE_CONTRACT_REF: &str =
    "docs/governance/post_install_notice_and_provenance_contract.md";

/// Repo-relative path of the shared provenance-badge vocabulary contract.
pub const POST_INSTALL_PROVENANCE_BADGE_CONTRACT_REF: &str =
    "docs/governance/provenance_badge_contract.md";

/// Repo-relative path of the frozen M5 public-handoff matrix that governs whether
/// the post-install notice / provenance disclosure may publish a claim.
pub const POST_INSTALL_PUBLIC_HANDOFF_MATRIX_REF: &str =
    "schemas/help/m5-public-handoff-matrix.schema.json";

/// Repo-relative path of the help-lane panel-set bundle schema.
pub const M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_REF: &str =
    "schemas/help/m5-post-install-disclosure.schema.json";

/// Repo-relative path of the help-lane panel-set contract doc.
pub const M5_POST_INSTALL_DISCLOSURE_PANEL_SET_DOC_REF: &str =
    "docs/help/m5_post_install_disclosure_panels_contract.md";

/// Repo-relative path of the checked panel-set support export.
pub const M5_POST_INSTALL_DISCLOSURE_PANEL_SET_ARTIFACT_REF: &str =
    "artifacts/help/m5-post-install-proof/panel_set.json";

/// The four governed M5 artifact families that carry a post-install disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureArtifactFamily {
    /// Desktop builds and installer payloads shipped with the product.
    DesktopBuildInstaller,
    /// Extension packages and framework tooling packs.
    ExtensionFrameworkPack,
    /// Mirrored transport artifacts and offline bundles.
    MirroredOfflineArtifact,
    /// Generated / exported user artifacts.
    GeneratedExportArtifact,
}

impl DisclosureArtifactFamily {
    /// Every governed family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopBuildInstaller,
        Self::ExtensionFrameworkPack,
        Self::MirroredOfflineArtifact,
        Self::GeneratedExportArtifact,
    ];

    /// Stable token recorded in the panel set.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopBuildInstaller => "desktop_build_installer",
            Self::ExtensionFrameworkPack => "extension_framework_pack",
            Self::MirroredOfflineArtifact => "mirrored_offline_artifact",
            Self::GeneratedExportArtifact => "generated_export_artifact",
        }
    }
}

/// Subject class a disclosure record describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceSubjectKind {
    /// Installed desktop shell, CLI, or bundled helper.
    ProductBuild,
    /// Installer, package-manager payload, or portable bundle.
    InstallerPayload,
    /// One installed extension package.
    ExtensionPackage,
    /// First-party or third-party framework tooling pack.
    FrameworkPack,
    /// Offline / mirrored transport artifact.
    MirroredTransportArtifact,
    /// Generated user artifact / export.
    GeneratedUserArtifact,
}

impl SurfaceSubjectKind {
    /// Artifact family this subject kind rolls up to.
    pub const fn family(self) -> DisclosureArtifactFamily {
        match self {
            Self::ProductBuild | Self::InstallerPayload => {
                DisclosureArtifactFamily::DesktopBuildInstaller
            }
            Self::ExtensionPackage | Self::FrameworkPack => {
                DisclosureArtifactFamily::ExtensionFrameworkPack
            }
            Self::MirroredTransportArtifact => DisclosureArtifactFamily::MirroredOfflineArtifact,
            Self::GeneratedUserArtifact => DisclosureArtifactFamily::GeneratedExportArtifact,
        }
    }
}

/// Artifact class within a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    /// Desktop product build.
    DesktopBuild,
    /// CLI product build.
    CliBuild,
    /// Installer payload.
    Installer,
    /// Portable bundle.
    PortableBundle,
    /// Extension package.
    ExtensionPackage,
    /// Framework tooling pack.
    FrameworkPack,
    /// Mirrored transport artifact.
    MirroredTransportArtifact,
    /// Offline bundle.
    OfflineBundle,
    /// Documentation pack.
    DocsPack,
    /// Generated export.
    GeneratedExport,
    /// Generated artifact.
    GeneratedArtifact,
}

/// Source / transport posture, distinct from the artifact subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// Official, first-party, verifiable source.
    Official,
    /// Official build delivered through a recognized mirror.
    Mirrored,
    /// Side-loaded build installed outside an official or mirror channel.
    SideLoaded,
    /// Provenance could not be established.
    UnknownProvenance,
}

impl SourceClass {
    /// Stable token recorded in the panel set.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Mirrored => "mirrored",
            Self::SideLoaded => "side_loaded",
            Self::UnknownProvenance => "unknown_provenance",
        }
    }

    /// The exact human-visible label the contract reserves for this class.
    pub const fn required_label(self) -> SourceLabel {
        match self {
            Self::Official => SourceLabel::Official,
            Self::Mirrored => SourceLabel::Mirrored,
            Self::SideLoaded => SourceLabel::SideLoaded,
            Self::UnknownProvenance => SourceLabel::UnknownProvenance,
        }
    }
}

/// Human-visible source label, reserved per [`SourceClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceLabel {
    /// `Official`.
    #[serde(rename = "Official")]
    Official,
    /// `Mirrored`.
    #[serde(rename = "Mirrored")]
    Mirrored,
    /// `Side-loaded`.
    #[serde(rename = "Side-loaded")]
    SideLoaded,
    /// `Unknown provenance`.
    #[serde(rename = "Unknown provenance")]
    UnknownProvenance,
}

/// Release / acquisition channel class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelClass {
    /// Nightly channel.
    Nightly,
    /// Preview channel.
    Preview,
    /// Beta channel.
    Beta,
    /// Stable channel.
    Stable,
    /// Long-term-support channel.
    Lts,
    /// Local build / import.
    Local,
    /// External / third-party.
    External,
    /// Not applicable for this subject.
    NotApplicable,
    /// Unknown channel.
    Unknown,
}

/// Acquisition route that delivered the artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquiredVia {
    /// Official update feed.
    OfficialUpdateFeed,
    /// Official download.
    OfficialDownload,
    /// Platform package manager.
    PlatformPackageManager,
    /// Public registry.
    PublicRegistry,
    /// Private registry.
    PrivateRegistry,
    /// Enterprise mirror.
    EnterpriseMirror,
    /// Offline bundle.
    OfflineBundle,
    /// Air-gapped media.
    AirGappedMedia,
    /// Local file picker.
    LocalFilePicker,
    /// Workspace export.
    WorkspaceExport,
    /// Generated-export flow.
    GeneratedExportFlow,
    /// Unknown route.
    Unknown,
}

/// Signature verification state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureState {
    /// Signed and verified.
    SignedVerified,
    /// Signed but not verified.
    SignedUnverified,
    /// Signature missing.
    SignatureMissing,
    /// Signature revoked.
    SignatureRevoked,
    /// Signature mismatch.
    SignatureMismatch,
    /// Not applicable.
    NotApplicable,
}

/// Attestation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttestationState {
    /// Attestation verified.
    AttestationVerified,
    /// Attestation present but unverified.
    AttestationPresentUnverified,
    /// Attestation missing.
    AttestationMissing,
    /// Attestation stale.
    AttestationStale,
    /// Attestation policy-blocked.
    AttestationPolicyBlocked,
    /// Not applicable.
    NotApplicable,
}

/// Checksum state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumState {
    /// Checksum verified.
    ChecksumVerified,
    /// Checksum present but unverified.
    ChecksumPresentUnverified,
    /// Checksum missing.
    ChecksumMissing,
    /// Checksum mismatch.
    ChecksumMismatch,
    /// Not applicable.
    NotApplicable,
}

/// Revocation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationState {
    /// Revocation current.
    RevocationCurrent,
    /// Revocation snapshot current.
    RevocationSnapshotCurrent,
    /// Revocation snapshot stale.
    RevocationSnapshotStale,
    /// Revocation snapshot expired.
    RevocationSnapshotExpired,
    /// Artifact revoked or yanked.
    RevokedOrYanked,
    /// Revocation unknown.
    RevocationUnknown,
    /// Not applicable.
    NotApplicable,
}

/// Freshness class for revocation snapshots and similar cached evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Current.
    Current,
    /// Warm-cached.
    WarmCached,
    /// Stale, requires review.
    StaleRequiresReview,
    /// Expired.
    Expired,
    /// Unknown.
    Unknown,
    /// Not applicable.
    NotApplicable,
}

/// License inventory state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseState {
    /// License allowed.
    LicenseAllowed,
    /// License allowed with notice obligations.
    LicenseAllowedWithNotice,
    /// License restricted.
    LicenseRestricted,
    /// License policy-blocked.
    LicensePolicyBlocked,
    /// License unknown.
    LicenseUnknown,
    /// Not applicable.
    NotApplicable,
}

/// Notice completeness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeState {
    /// Notices complete.
    NoticeComplete,
    /// Notices partial.
    NoticePartial,
    /// Notices missing.
    NoticeMissing,
    /// Notices not required.
    NoticeNotRequired,
    /// Notices policy-hidden.
    NoticePolicyHidden,
    /// Notices unknown.
    NoticeUnknown,
}

/// Notice-inventory availability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeInventoryState {
    /// Inventory available.
    InventoryAvailable,
    /// Inventory partial.
    InventoryPartial,
    /// Inventory missing.
    InventoryMissing,
    /// Inventory policy-hidden.
    InventoryPolicyHidden,
    /// Inventory unknown.
    InventoryUnknown,
    /// Inventory not required.
    InventoryNotRequired,
}

/// SBOM availability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SbomState {
    /// SBOM attached and verified.
    SbomAttachedVerified,
    /// SBOM attached but unverified.
    SbomAttachedUnverified,
    /// SBOM missing.
    SbomMissing,
    /// SBOM stale.
    SbomStale,
    /// SBOM policy-blocked.
    SbomPolicyBlocked,
    /// Not applicable.
    NotApplicable,
}

impl SbomState {
    /// True when this state means an SBOM is attached and must declare formats.
    pub const fn is_attached(self) -> bool {
        matches!(
            self,
            Self::SbomAttachedVerified | Self::SbomAttachedUnverified
        )
    }
}

/// SBOM format label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SbomFormat {
    /// SPDX JSON.
    SpdxJson,
    /// SPDX tag/value.
    SpdxTagValue,
    /// SPDX RDF.
    SpdxRdf,
    /// CycloneDX JSON.
    CyclonedxJson,
    /// CycloneDX XML.
    CyclonedxXml,
}

/// Durable surface from which a disclosure can be reached after install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessPointClass {
    /// About surface.
    About,
    /// Update center.
    UpdateCenter,
    /// Installed-state inspector.
    InstalledStateInspector,
    /// Diagnostics export.
    DiagnosticsExport,
    /// Review sheet.
    ReviewSheet,
    /// Extension details.
    ExtensionDetails,
    /// Installer receipt.
    InstallerReceipt,
    /// Generated-artifact viewer.
    GeneratedArtifactViewer,
    /// Export review.
    ExportReview,
    /// Marketplace / package detail.
    MarketplaceOrPackageDetail,
    /// Support bundle.
    SupportBundle,
    /// Offline review.
    OfflineReview,
}

impl AccessPointClass {
    /// Access-point classes every record MUST carry.
    pub const REQUIRED: [Self; 5] = [
        Self::About,
        Self::UpdateCenter,
        Self::InstalledStateInspector,
        Self::DiagnosticsExport,
        Self::ReviewSheet,
    ];
}

/// Reachability of an access point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReachabilityClass {
    /// Available now.
    Available,
    /// Available read-only.
    AvailableReadOnly,
    /// Unavailable in this context, with a visible reason.
    UnavailableVisible,
    /// Policy hides detail, but existence and reason stay visible.
    PolicyHiddenVisible,
    /// Not applicable for this subject, visibly.
    NotApplicableVisible,
}

/// Action class for an open/export/inspect/refresh action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionClass {
    /// Open About.
    OpenAbout,
    /// Open update center.
    OpenUpdateCenter,
    /// Open installed-state inspector.
    OpenInstalledStateInspector,
    /// Open diagnostics export.
    OpenDiagnosticsExport,
    /// Open review sheet.
    OpenReviewSheet,
    /// Open extension details.
    OpenExtensionDetails,
    /// Open installer receipt.
    OpenInstallerReceipt,
    /// Open notices.
    OpenNotices,
    /// Export notices.
    ExportNotices,
    /// Export SBOM.
    ExportSbom,
    /// Inspect provenance.
    InspectProvenance,
    /// Verify now.
    VerifyNow,
    /// Refresh revocation snapshot.
    RefreshRevocationSnapshot,
    /// Open generated lineage.
    OpenGeneratedLineage,
    /// Open source inputs.
    OpenSourceInputs,
    /// Open redistribution hint.
    OpenRedistributionHint,
    /// Copy digest.
    CopyDigest,
    /// Export disclosure packet.
    ExportDisclosurePacket,
}

/// Evidence axis that a missing/partial-data row can name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataClass {
    /// Provenance.
    Provenance,
    /// Signature.
    Signature,
    /// Attestation.
    Attestation,
    /// SBOM.
    Sbom,
    /// License.
    License,
    /// Notice inventory.
    NoticeInventory,
    /// Revocation snapshot.
    RevocationSnapshot,
    /// Mirror origin.
    MirrorOrigin,
    /// Generated lineage.
    GeneratedLineage,
    /// Redistribution terms.
    RedistributionTerms,
}

/// Why an evidence axis is incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingState {
    /// Not provided.
    NotProvided,
    /// Partial.
    Partial,
    /// Unknown.
    Unknown,
    /// Stale.
    Stale,
    /// Expired.
    Expired,
    /// Policy-hidden.
    PolicyHidden,
    /// Not applicable.
    NotApplicable,
}

/// Redistribution hint for a generated / exported artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedistributionHintClass {
    /// Not applicable.
    NotApplicable,
    /// Allowed with notice.
    AllowedWithNotice,
    /// Review before redistribution.
    ReviewBeforeRedistribution,
    /// Blocked by policy.
    BlockedByPolicy,
    /// Unknown, review required.
    UnknownReviewRequired,
}

impl RedistributionHintClass {
    /// True when this hint is valid for a generated user artifact (the contract
    /// forbids `not_applicable` there).
    pub const fn valid_for_generated_artifact(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// Redaction class for an export projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default.
    MetadataSafeDefault,
    /// Public-proof safe.
    PublicProofSafe,
    /// Support-redacted.
    SupportRedacted,
    /// Internal only.
    InternalOnly,
}

/// Names the subject class and what the surface is, and is not, describing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewContext {
    /// Subject class.
    pub surface_subject_kind: SurfaceSubjectKind,
    /// Human-readable surface title.
    pub surface_title: String,
    /// Boundary statement for the surface.
    pub boundary_statement: String,
    /// Opaque ref of the primary artifact.
    pub primary_artifact_ref: String,
}

/// Artifact identity group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    /// Artifact class.
    pub artifact_class: ArtifactClass,
    /// Display name.
    pub display_name: String,
    /// Opaque artifact identity ref.
    pub artifact_identity_ref: String,
    /// Opaque version-or-digest ref.
    pub version_or_digest_ref: String,
    /// Build id, if any.
    pub build_id: Option<String>,
    /// Channel.
    pub channel: ChannelClass,
    /// Exact-build identity ref, if any.
    pub exact_build_identity_ref: Option<String>,
    /// Installer receipt ref, if any.
    pub installer_receipt_ref: Option<String>,
    /// Generated-artifact lineage ref, if any.
    pub generated_artifact_lineage_ref: Option<String>,
}

/// Source / transport group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    /// Source class.
    pub source_class: SourceClass,
    /// Reserved source label.
    pub source_label: SourceLabel,
    /// Origin ref, if any.
    pub origin_ref: Option<String>,
    /// Upstream-origin ref, if any.
    pub upstream_origin_ref: Option<String>,
    /// Mirror ref, if any.
    pub mirror_ref: Option<String>,
    /// Side-load review ref, if any.
    pub side_load_review_ref: Option<String>,
    /// Acquisition route.
    pub acquired_via: AcquiredVia,
    /// Source disclosure sentence.
    pub source_disclosure: String,
}

/// Verification group: signature, attestation, checksum, revocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verification {
    /// Signature state.
    pub signature_state: SignatureState,
    /// Attestation state.
    pub attestation_state: AttestationState,
    /// Checksum state.
    pub checksum_state: ChecksumState,
    /// Revocation state.
    pub revocation_state: RevocationState,
    /// Revocation freshness class.
    pub revocation_freshness_class: FreshnessClass,
    /// Revocation snapshot ref, if any.
    pub revocation_snapshot_ref: Option<String>,
    /// RFC 3339 timestamp of the last check, if any.
    pub checked_at: Option<String>,
    /// Evidence refs.
    pub verification_evidence_refs: Vec<String>,
    /// Verification disclosure sentence.
    pub verification_disclosure: String,
}

/// Notice / license / SBOM inventory group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeInventory {
    /// License state.
    pub license_state: LicenseState,
    /// License-expression ref, if any.
    pub license_expression_ref: Option<String>,
    /// Notice completeness state.
    pub notice_state: NoticeState,
    /// Notice-inventory availability state.
    pub notice_inventory_state: NoticeInventoryState,
    /// Notice-inventory refs.
    pub notice_inventory_refs: Vec<String>,
    /// SBOM state.
    pub sbom_state: SbomState,
    /// Declared SBOM formats.
    pub sbom_formats: Vec<SbomFormat>,
    /// Notice disclosure sentence.
    pub notice_disclosure: String,
}

/// One typed missing/partial-data row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingOrPartialDataRow {
    /// Affected evidence axis.
    pub data_class: DataClass,
    /// Why it is incomplete.
    pub missing_state: MissingState,
    /// Visible label.
    pub visible_label: String,
    /// Disclosure sentence.
    pub disclosure: String,
    /// Resolution action ref, if any.
    pub resolution_action_ref: Option<String>,
}

/// Short user-visible cues plus typed missing/partial rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibleCues {
    /// Source cue.
    pub source_cue: String,
    /// Provenance cue.
    pub provenance_cue: String,
    /// License cue.
    pub license_cue: String,
    /// Notice cue.
    pub notice_cue: String,
    /// Revocation cue.
    pub revocation_cue: String,
    /// Typed missing/partial-data rows (required even when empty).
    pub missing_or_partial_data: Vec<MissingOrPartialDataRow>,
}

/// One durable access-point row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessPoint {
    /// Access-point class.
    pub access_point_class: AccessPointClass,
    /// Reachability.
    pub reachability_class: ReachabilityClass,
    /// Action ref, if any.
    pub action_ref: Option<String>,
    /// Disclosure sentence.
    pub disclosure: String,
}

/// One named action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    /// Opaque action ref.
    pub action_ref: String,
    /// Action class.
    pub action_class: ActionClass,
    /// Visible label.
    pub label: String,
    /// Target ref, if any.
    pub target_ref: Option<String>,
    /// Availability.
    pub availability: ReachabilityClass,
}

/// Redistribution guidance group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Redistribution {
    /// Redistribution hint class.
    pub redistribution_hint_class: RedistributionHintClass,
    /// Redistribution disclosure sentence.
    pub redistribution_disclosure: String,
    /// Required notice refs.
    pub required_notice_refs: Vec<String>,
    /// Required license refs.
    pub required_license_refs: Vec<String>,
    /// Required lineage refs.
    pub required_lineage_refs: Vec<String>,
    /// Policy refs.
    pub policy_refs: Vec<String>,
}

/// Cross-surface linkage refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Linkage {
    /// Provenance-badge refs.
    pub provenance_badge_refs: Vec<String>,
    /// Update-manifest refs.
    pub update_manifest_refs: Vec<String>,
    /// Install-review refs.
    pub install_review_refs: Vec<String>,
    /// Extension/pack-review refs.
    pub extension_or_pack_review_refs: Vec<String>,
    /// Generated-lineage refs.
    pub generated_lineage_refs: Vec<String>,
    /// Release-evidence refs.
    pub release_evidence_refs: Vec<String>,
    /// Support-bundle refs.
    pub support_bundle_refs: Vec<String>,
    /// Diagnostics-export refs.
    pub diagnostics_export_refs: Vec<String>,
    /// Mirror/offline-receipt refs.
    pub mirror_or_offline_receipt_refs: Vec<String>,
}

/// Export projection refs plus redaction and omission behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportProjection {
    /// Diagnostics-export refs.
    pub diagnostics_export_refs: Vec<String>,
    /// Support-bundle refs.
    pub support_bundle_refs: Vec<String>,
    /// Public-proof refs.
    pub public_proof_refs: Vec<String>,
    /// Offline-review refs.
    pub offline_review_refs: Vec<String>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Omission reasons.
    pub omission_reasons: Vec<String>,
}

/// One export-safe post-install disclosure record.
///
/// Conforms to
/// [`schemas/governance/post_install_disclosure.schema.json`](../../../../schemas/governance/post_install_disclosure.schema.json).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostInstallDisclosureRecord {
    /// Schema version; must equal [`POST_INSTALL_DISCLOSURE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Record kind; must equal [`POST_INSTALL_DISCLOSURE_RECORD_KIND`].
    pub record_kind: String,
    /// Stable disclosure id (`post_install_disclosure:...`).
    pub disclosure_id: String,
    /// RFC 3339 emit timestamp.
    pub emitted_at: String,
    /// Review context.
    pub review_context: ReviewContext,
    /// Artifact identity.
    pub artifact: Artifact,
    /// Source / transport.
    pub source: Source,
    /// Verification.
    pub verification: Verification,
    /// Notice / license / SBOM inventory.
    pub notice_inventory: NoticeInventory,
    /// Visible cues plus missing/partial rows.
    pub visible_cues: VisibleCues,
    /// Durable access points.
    pub access_points: Vec<AccessPoint>,
    /// Named actions.
    pub actions: Vec<Action>,
    /// Redistribution guidance.
    pub redistribution: Redistribution,
    /// Cross-surface linkage.
    pub linkage: Linkage,
    /// Export projection.
    pub export_projection: ExportProjection,
    /// Narrative refs.
    pub narrative_refs: Vec<String>,
}

impl PostInstallDisclosureRecord {
    /// Artifact family this record describes.
    pub fn artifact_family(&self) -> DisclosureArtifactFamily {
        self.review_context.surface_subject_kind.family()
    }

    /// Validates this record against the post-install disclosure contract.
    pub fn validate(&self) -> Vec<PostInstallDisclosureViolation> {
        let mut violations = Vec::new();
        self.validate_into(&mut violations);
        violations
    }

    fn validate_into(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;

        if self.record_kind != POST_INSTALL_DISCLOSURE_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != POST_INSTALL_DISCLOSURE_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if !is_valid_disclosure_id(&self.disclosure_id) {
            violations.push(V::InvalidDisclosureId);
        }
        if self.emitted_at.trim().is_empty()
            || self.review_context.surface_title.trim().is_empty()
            || self.review_context.boundary_statement.trim().is_empty()
            || self.review_context.primary_artifact_ref.trim().is_empty()
            || self.artifact.display_name.trim().is_empty()
            || self.artifact.artifact_identity_ref.trim().is_empty()
            || self.artifact.version_or_digest_ref.trim().is_empty()
            || self.source.source_disclosure.trim().is_empty()
            || self.verification.verification_disclosure.trim().is_empty()
            || self.notice_inventory.notice_disclosure.trim().is_empty()
        {
            violations.push(V::IncompleteRecord);
        }

        self.validate_source(violations);
        self.validate_sbom(violations);
        self.validate_missing_data_visible(violations);
        self.validate_access_points(violations);
        self.validate_actions(violations);
        self.validate_generated_artifact(violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("post-install disclosure record serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }
    }

    fn validate_source(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;
        if self.source.source_label != self.source.source_class.required_label() {
            violations.push(V::SourceLabelMismatch);
        }
        let missing_required_source_ref = match self.source.source_class {
            SourceClass::Official => is_blank(&self.source.origin_ref),
            SourceClass::Mirrored => {
                is_blank(&self.source.upstream_origin_ref)
                    || is_blank(&self.source.mirror_ref)
                    || is_blank(&self.verification.revocation_snapshot_ref)
            }
            SourceClass::SideLoaded => is_blank(&self.source.side_load_review_ref),
            SourceClass::UnknownProvenance => false,
        };
        if missing_required_source_ref {
            violations.push(V::SourceEvidenceMissing);
        }
    }

    fn validate_sbom(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;
        let attached = self.notice_inventory.sbom_state.is_attached();
        let has_formats = !self.notice_inventory.sbom_formats.is_empty();
        // An attached SBOM must label its formats; a non-attached SBOM must not
        // claim formats it does not carry.
        if attached && !has_formats {
            violations.push(V::SbomFormatLabelMissing);
        }
        if !attached && has_formats {
            violations.push(V::SbomFormatLabelUnbacked);
        }
    }

    fn validate_missing_data_visible(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;
        let present: BTreeSet<DataClass> = self
            .visible_cues
            .missing_or_partial_data
            .iter()
            .map(|row| row.data_class)
            .collect();
        for axis in self.required_missing_data_axes() {
            if !present.contains(&axis) {
                violations.push(V::MissingDataRowOmitted);
                return;
            }
        }
        for row in &self.visible_cues.missing_or_partial_data {
            if row.visible_label.trim().is_empty() || row.disclosure.trim().is_empty() {
                violations.push(V::MissingDataRowIncomplete);
                return;
            }
        }
    }

    /// Evidence axes whose state requires a visible missing/partial-data row.
    fn required_missing_data_axes(&self) -> Vec<DataClass> {
        let mut axes = Vec::new();
        if matches!(
            self.verification.signature_state,
            SignatureState::SignatureMissing | SignatureState::SignatureMismatch
        ) {
            axes.push(DataClass::Signature);
        }
        if matches!(
            self.verification.attestation_state,
            AttestationState::AttestationMissing
                | AttestationState::AttestationStale
                | AttestationState::AttestationPolicyBlocked
        ) {
            axes.push(DataClass::Attestation);
        }
        if matches!(
            self.verification.revocation_state,
            RevocationState::RevocationSnapshotStale
                | RevocationState::RevocationSnapshotExpired
                | RevocationState::RevokedOrYanked
                | RevocationState::RevocationUnknown
        ) {
            axes.push(DataClass::RevocationSnapshot);
        }
        if matches!(
            self.notice_inventory.sbom_state,
            SbomState::SbomMissing | SbomState::SbomStale | SbomState::SbomPolicyBlocked
        ) {
            axes.push(DataClass::Sbom);
        }
        if matches!(
            self.notice_inventory.license_state,
            LicenseState::LicenseRestricted
                | LicenseState::LicensePolicyBlocked
                | LicenseState::LicenseUnknown
        ) {
            axes.push(DataClass::License);
        }
        if matches!(
            self.notice_inventory.notice_inventory_state,
            NoticeInventoryState::InventoryMissing
                | NoticeInventoryState::InventoryPartial
                | NoticeInventoryState::InventoryPolicyHidden
                | NoticeInventoryState::InventoryUnknown
        ) {
            axes.push(DataClass::NoticeInventory);
        }
        axes
    }

    fn validate_access_points(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;
        let present: BTreeSet<AccessPointClass> = self
            .access_points
            .iter()
            .map(|point| point.access_point_class)
            .collect();
        for required in AccessPointClass::REQUIRED {
            if !present.contains(&required) {
                violations.push(V::RequiredAccessPointMissing);
                return;
            }
        }
        for point in &self.access_points {
            if point.disclosure.trim().is_empty() {
                violations.push(V::AccessPointIncomplete);
                return;
            }
        }
    }

    fn validate_actions(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;
        if self.actions.is_empty() {
            violations.push(V::ActionsMissing);
            return;
        }
        for action in &self.actions {
            if action.action_ref.trim().is_empty() || action.label.trim().is_empty() {
                violations.push(V::ActionIncomplete);
                return;
            }
        }
    }

    fn validate_generated_artifact(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        use PostInstallDisclosureViolation as V;
        if self.review_context.surface_subject_kind != SurfaceSubjectKind::GeneratedUserArtifact {
            return;
        }
        if is_blank(&self.artifact.generated_artifact_lineage_ref) {
            violations.push(V::GeneratedLineageMissing);
        }
        if !self
            .redistribution
            .redistribution_hint_class
            .valid_for_generated_artifact()
        {
            violations.push(V::GeneratedRedistributionHintInvalid);
        }
    }
}

/// Coverage of the four governed M5 artifact families by a panel set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureCoverage {
    /// True when a desktop-build / installer panel is present.
    pub covers_desktop_build_installer: bool,
    /// True when an extension / framework-pack panel is present.
    pub covers_extension_framework_pack: bool,
    /// True when a mirrored / offline-artifact panel is present.
    pub covers_mirrored_offline_artifact: bool,
    /// True when a generated / exported-artifact panel is present.
    pub covers_generated_export_artifact: bool,
}

impl DisclosureCoverage {
    /// Derives coverage from the panel families actually present.
    pub fn from_panels(panels: &[PostInstallDisclosureRecord]) -> Self {
        let families: BTreeSet<DisclosureArtifactFamily> = panels
            .iter()
            .map(PostInstallDisclosureRecord::artifact_family)
            .collect();
        Self {
            covers_desktop_build_installer: families
                .contains(&DisclosureArtifactFamily::DesktopBuildInstaller),
            covers_extension_framework_pack: families
                .contains(&DisclosureArtifactFamily::ExtensionFrameworkPack),
            covers_mirrored_offline_artifact: families
                .contains(&DisclosureArtifactFamily::MirroredOfflineArtifact),
            covers_generated_export_artifact: families
                .contains(&DisclosureArtifactFamily::GeneratedExportArtifact),
        }
    }

    /// True when all four governed families are covered.
    pub const fn is_complete(&self) -> bool {
        self.covers_desktop_build_installer
            && self.covers_extension_framework_pack
            && self.covers_mirrored_offline_artifact
            && self.covers_generated_export_artifact
    }
}

/// Hard honesty invariants the panel set must satisfy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureHonestyInvariants {
    /// Every panel names its subject class explicitly.
    pub subject_kind_explicit: bool,
    /// Missing/partial/unknown/stale/hidden data is shown, never omitted.
    pub missing_data_visible_not_omitted: bool,
    /// Source class and subject class stay separate.
    pub source_class_and_subject_separate: bool,
    /// Trust evidence is layered into separate fields.
    pub trust_evidence_layered: bool,
    /// Disclosure stays reachable after install from durable surfaces.
    pub post_install_access_survives: bool,
    /// SBOM format labeling and artifact scope stay explicit.
    pub sbom_format_and_scope_explicit: bool,
    /// Exports preserve the same caveats as the live disclosure.
    pub exports_preserve_caveats: bool,
    /// Provenance states distinguish official / mirrored / side-loaded / unknown.
    pub provenance_states_distinguish_official_mirrored_side_loaded_unknown: bool,
    /// A stale or revoked artifact never reads as merely verified.
    pub stale_or_revoked_never_reads_as_verified: bool,
}

/// Consumer-surface projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureConsumerProjection {
    /// About/help shows the disclosure panel.
    pub about_help_shows_disclosure_panel: bool,
    /// Installed-state inspector shows the disclosure panel.
    pub installed_state_inspector_shows_disclosure_panel: bool,
    /// Diagnostics export includes the disclosure record.
    pub diagnostics_export_includes_disclosure_record: bool,
    /// Marketplace / package detail shows provenance for packs.
    pub marketplace_or_package_detail_shows_provenance_for_packs: bool,
    /// This lane exposes — does not replace — release publication artifacts.
    pub does_not_replace_release_publication_artifacts: bool,
}

/// Constructor input for [`M5PostInstallDisclosurePanelSet::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PostInstallDisclosurePanelSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable panel-set label.
    pub panel_set_label: String,
    /// Disclosure panels, one per governed family.
    pub panels: Vec<PostInstallDisclosureRecord>,
    /// Honesty invariants.
    pub honesty_invariants: DisclosureHonestyInvariants,
    /// Consumer projection.
    pub consumer_projection: DisclosureConsumerProjection,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Export-safe bundle of post-install disclosure panels for the four governed M5
/// artifact families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PostInstallDisclosurePanelSet {
    /// Record kind; must equal [`M5_POST_INSTALL_DISCLOSURE_PANEL_SET_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable panel-set label.
    pub panel_set_label: String,
    /// Disclosure panels, one per governed family.
    pub panels: Vec<PostInstallDisclosureRecord>,
    /// Coverage of the four governed families.
    pub coverage: DisclosureCoverage,
    /// Honesty invariants.
    pub honesty_invariants: DisclosureHonestyInvariants,
    /// Consumer projection.
    pub consumer_projection: DisclosureConsumerProjection,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5PostInstallDisclosurePanelSet {
    /// Builds a panel set, deriving coverage from the supplied panels.
    pub fn new(input: M5PostInstallDisclosurePanelSetInput) -> Self {
        let coverage = DisclosureCoverage::from_panels(&input.panels);
        Self {
            record_kind: M5_POST_INSTALL_DISCLOSURE_PANEL_SET_KIND.to_owned(),
            schema_version: M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            panel_set_label: input.panel_set_label,
            panels: input.panels,
            coverage,
            honesty_invariants: input.honesty_invariants,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the panel set and every nested panel.
    pub fn validate(&self) -> Vec<PostInstallDisclosureViolation> {
        use PostInstallDisclosureViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_POST_INSTALL_DISCLOSURE_PANEL_SET_KIND {
            violations.push(V::WrongPanelSetKind);
        }
        if self.schema_version != M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.panel_set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }

        for panel in &self.panels {
            panel.validate_into(&mut violations);
        }

        let derived = DisclosureCoverage::from_panels(&self.panels);
        if derived != self.coverage {
            violations.push(V::CoverageDrift);
        }
        if !self.coverage.is_complete() {
            violations.push(V::FamilyCoverageIncomplete);
        }

        self.validate_honesty_invariants(&mut violations);
        self.validate_consumer_projection(&mut violations);
        self.validate_source_contracts(&mut violations);

        violations
    }

    fn validate_honesty_invariants(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        let invariants = &self.honesty_invariants;
        for ok in [
            invariants.subject_kind_explicit,
            invariants.missing_data_visible_not_omitted,
            invariants.source_class_and_subject_separate,
            invariants.trust_evidence_layered,
            invariants.post_install_access_survives,
            invariants.sbom_format_and_scope_explicit,
            invariants.exports_preserve_caveats,
            invariants.provenance_states_distinguish_official_mirrored_side_loaded_unknown,
            invariants.stale_or_revoked_never_reads_as_verified,
        ] {
            if !ok {
                violations.push(PostInstallDisclosureViolation::HonestyInvariantUnmet);
                return;
            }
        }
    }

    fn validate_consumer_projection(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        let projection = &self.consumer_projection;
        for ok in [
            projection.about_help_shows_disclosure_panel,
            projection.installed_state_inspector_shows_disclosure_panel,
            projection.diagnostics_export_includes_disclosure_record,
            projection.marketplace_or_package_detail_shows_provenance_for_packs,
            projection.does_not_replace_release_publication_artifacts,
        ] {
            if !ok {
                violations.push(PostInstallDisclosureViolation::ConsumerProjectionIncomplete);
                return;
            }
        }
    }

    fn validate_source_contracts(&self, violations: &mut Vec<PostInstallDisclosureViolation>) {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            POST_INSTALL_DISCLOSURE_SCHEMA_REF,
            POST_INSTALL_DISCLOSURE_CONTRACT_REF,
            POST_INSTALL_PROVENANCE_BADGE_CONTRACT_REF,
            POST_INSTALL_PUBLIC_HANDOFF_MATRIX_REF,
            M5_POST_INSTALL_DISCLOSURE_PANEL_SET_SCHEMA_REF,
            M5_POST_INSTALL_DISCLOSURE_PANEL_SET_DOC_REF,
        ] {
            if !refs.contains(required) {
                violations.push(PostInstallDisclosureViolation::MissingSourceContracts);
                return;
            }
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("post-install disclosure panel set serializes")
    }

    /// Deterministic, machine-readable panel CSV: one row per artifact family,
    /// naming its source class, signature/SBOM/notice/revocation states, missing-row
    /// count, and access-point count.
    pub fn render_panel_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,subject_kind,source_class,signature_state,sbom_state,notice_inventory_state,revocation_state,missing_data_rows,access_points\n",
        );
        for panel in &self.panels {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                panel.artifact_family().as_str(),
                token(&panel.review_context.surface_subject_kind),
                panel.source.source_class.as_str(),
                token(&panel.verification.signature_state),
                token(&panel.notice_inventory.sbom_state),
                token(&panel.notice_inventory.notice_inventory_state),
                token(&panel.verification.revocation_state),
                panel.visible_cues.missing_or_partial_data.len(),
                panel.access_points.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Post-Install Notice/Provenance/SBOM Disclosure Panels\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.panel_set_label));
        out.push_str(&format!(
            "- Panels: {} ({} families covered)\n",
            self.panels.len(),
            DisclosureArtifactFamily::ALL
                .iter()
                .filter(|family| self.panels.iter().any(|p| p.artifact_family() == **family))
                .count()
        ));
        out.push_str("\n## Panels\n\n");
        for panel in &self.panels {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                panel.artifact_family().as_str(),
                panel.disclosure_id
            ));
            out.push_str(&format!(
                "  - Subject: {} ({})\n",
                token(&panel.review_context.surface_subject_kind),
                panel.review_context.surface_title
            ));
            out.push_str(&format!(
                "  - Source: {} / signature {} / sbom {}\n",
                panel.source.source_class.as_str(),
                token(&panel.verification.signature_state),
                token(&panel.notice_inventory.sbom_state)
            ));
            out.push_str(&format!(
                "  - Notice inventory: {} / revocation {}\n",
                token(&panel.notice_inventory.notice_inventory_state),
                token(&panel.verification.revocation_state)
            ));
            out.push_str(&format!(
                "  - Visible missing/partial-data rows: {}\n",
                panel.visible_cues.missing_or_partial_data.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in panel-set export.
#[derive(Debug)]
pub enum PostInstallDisclosureArtifactError {
    /// Panel-set export failed to parse.
    Parse(serde_json::Error),
    /// Panel-set export failed validation.
    Validation(Vec<PostInstallDisclosureViolation>),
}

impl fmt::Display for PostInstallDisclosureArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(
                    formatter,
                    "post-install disclosure export parse failed: {error}"
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
                    "post-install disclosure export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PostInstallDisclosureArtifactError {}

/// Validation failures emitted by the disclosure validators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostInstallDisclosureViolation {
    /// Record record-kind is wrong.
    WrongRecordKind,
    /// Panel-set record-kind is wrong.
    WrongPanelSetKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Disclosure id does not match the reserved pattern.
    InvalidDisclosureId,
    /// A required identity field is missing.
    MissingIdentity,
    /// A required record field is empty.
    IncompleteRecord,
    /// The source label does not match its source class.
    SourceLabelMismatch,
    /// Required source evidence for the source class is missing.
    SourceEvidenceMissing,
    /// An attached SBOM does not declare its formats.
    SbomFormatLabelMissing,
    /// SBOM formats are declared without an attached SBOM.
    SbomFormatLabelUnbacked,
    /// A missing/partial evidence axis has no visible row.
    MissingDataRowOmitted,
    /// A missing/partial-data row is incomplete.
    MissingDataRowIncomplete,
    /// A required access point is missing.
    RequiredAccessPointMissing,
    /// An access-point row is incomplete.
    AccessPointIncomplete,
    /// No actions are present.
    ActionsMissing,
    /// An action row is incomplete.
    ActionIncomplete,
    /// A generated user artifact omits its lineage ref.
    GeneratedLineageMissing,
    /// A generated user artifact carries an invalid redistribution hint.
    GeneratedRedistributionHintInvalid,
    /// Export carries raw boundary material.
    RawBoundaryMaterialInExport,
    /// Derived coverage drifted from the recorded coverage.
    CoverageDrift,
    /// One of the four governed families is not covered.
    FamilyCoverageIncomplete,
    /// A honesty invariant flag is unset.
    HonestyInvariantUnmet,
    /// A consumer-projection flag is unset.
    ConsumerProjectionIncomplete,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
}

impl PostInstallDisclosureViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongPanelSetKind => "wrong_panel_set_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::InvalidDisclosureId => "invalid_disclosure_id",
            Self::MissingIdentity => "missing_identity",
            Self::IncompleteRecord => "incomplete_record",
            Self::SourceLabelMismatch => "source_label_mismatch",
            Self::SourceEvidenceMissing => "source_evidence_missing",
            Self::SbomFormatLabelMissing => "sbom_format_label_missing",
            Self::SbomFormatLabelUnbacked => "sbom_format_label_unbacked",
            Self::MissingDataRowOmitted => "missing_data_row_omitted",
            Self::MissingDataRowIncomplete => "missing_data_row_incomplete",
            Self::RequiredAccessPointMissing => "required_access_point_missing",
            Self::AccessPointIncomplete => "access_point_incomplete",
            Self::ActionsMissing => "actions_missing",
            Self::ActionIncomplete => "action_incomplete",
            Self::GeneratedLineageMissing => "generated_lineage_missing",
            Self::GeneratedRedistributionHintInvalid => "generated_redistribution_hint_invalid",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
            Self::CoverageDrift => "coverage_drift",
            Self::FamilyCoverageIncomplete => "family_coverage_incomplete",
            Self::HonestyInvariantUnmet => "honesty_invariant_unmet",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::MissingSourceContracts => "missing_source_contracts",
        }
    }
}

/// Reads and validates the checked-in stable M5 post-install disclosure panel set.
pub fn current_stable_m5_post_install_disclosure_panel_set(
) -> Result<M5PostInstallDisclosurePanelSet, PostInstallDisclosureArtifactError> {
    let packet: M5PostInstallDisclosurePanelSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-post-install-proof/panel_set.json"
    )))
    .map_err(PostInstallDisclosureArtifactError::Parse)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PostInstallDisclosureArtifactError::Validation(violations))
    }
}

/// Returns the serde token for any controlled-vocabulary value.
fn token<T: Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(serde_json::Value::String(token)) => token,
        _ => String::new(),
    }
}

/// True when an optional ref is `None` or blank.
fn is_blank(value: &Option<String>) -> bool {
    value.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true)
}

/// Validates the reserved `post_install_disclosure:` id pattern.
fn is_valid_disclosure_id(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("post_install_disclosure:") else {
        return false;
    };
    if rest.is_empty() {
        return false;
    }
    // Lowercase alphanumeric segments separated by single `.`, `_`, or `-`.
    let bytes = rest.as_bytes();
    let mut prev_sep = true; // leading separator is not allowed
    for &b in bytes {
        let is_alnum = b.is_ascii_lowercase() || b.is_ascii_digit();
        let is_sep = matches!(b, b'.' | b'_' | b'-');
        if is_alnum {
            prev_sep = false;
        } else if is_sep {
            if prev_sep {
                return false;
            }
            prev_sep = true;
        } else {
            return false;
        }
    }
    !prev_sep
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
