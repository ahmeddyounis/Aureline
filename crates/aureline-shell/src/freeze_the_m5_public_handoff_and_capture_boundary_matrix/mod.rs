//! Frozen M5 post-install notice/provenance, community-handoff,
//! reproduction-packet, and device-permission/auth-boundary matrix.
//!
//! This module locks the canonical M5 public-handoff and capture-boundary object
//! model into one export-safe packet. Each [`M5HandoffObjectRow`] names one
//! governed object — the post-install notice, the provenance disclosure, the
//! community-handoff route, the reproduction packet, the offline-capture
//! continuity record, the device-permission boundary, the embedded auth/webview
//! boundary, and the service-health notice — and binds it to its qualification
//! class, required fields, the controlled state vocabularies it carries, the
//! concrete vocabulary tokens it admits, evidence requirements, the proof packet
//! that keeps it current, downgrade triggers, rollback posture, source contracts,
//! and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether claimed M5 help, support,
//! ecosystem, and voice/capture surfaces may publish handoff or boundary claims.
//! About/help, marketplace, update/service-health, community handoff, repro
//! packets, and capture/auth surfaces consume this packet rather than maintaining
//! parallel dialogs: post-install provenance/notice states stay inspectable after
//! install; outbound public/community routes declare visibility and support class
//! before launch; repro packets are previewed and redacted before share; offline
//! capture survives a failed handoff; and device/mic/auth/webview boundaries never
//! impersonate native trusted product chrome.
//!
//! The controlled vocabularies mirror canonical tokens already owned by the
//! community-handoff packet, the provenance badge vocabulary, the service-health
//! destination contract, and the M3 handoff-target / repro-packet contracts; the
//! matrix freezes them in one self-describing [`M5HandoffVocabularySet`] rather
//! than minting parallel tokens. It references the upstream contracts by id. Raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics,
//! private endpoints, credentials, and user text bodies stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/help/m5-public-handoff-matrix.schema.json`](../../../../schemas/help/m5-public-handoff-matrix.schema.json).
//! The contract doc is
//! [`docs/help/m5_public_handoff_matrix_contract.md`](../../../../docs/help/m5_public_handoff_matrix_contract.md).
//! The protected fixture directory is
//! [`fixtures/help/m5-public-handoff/`](../../../../fixtures/help/m5-public-handoff/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_public_handoff_matrix, seeded_m5_public_handoff_matrix_provenance_unverified_narrowed,
    seeded_m5_public_handoff_matrix_repro_redaction_held, M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PublicHandoffMatrixPacket`].
pub const M5_PUBLIC_HANDOFF_MATRIX_RECORD_KIND: &str =
    "freeze_m5_public_handoff_and_capture_boundary_matrix";

/// Schema version for M5 public-handoff matrix records.
pub const M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_REF: &str =
    "schemas/help/m5-public-handoff-matrix.schema.json";

/// Repo-relative path of the M5 public-handoff matrix contract doc.
pub const M5_PUBLIC_HANDOFF_MATRIX_DOC_REF: &str = "docs/help/m5_public_handoff_matrix_contract.md";

/// Repo-relative path of the community-handoff packet contract this matrix
/// governs.
pub const M5_HANDOFF_COMMUNITY_PACKET_CONTRACT_REF: &str =
    "schemas/help/community-handoff-packet.schema.json";

/// Repo-relative path of the provenance badge vocabulary this matrix mirrors.
pub const M5_HANDOFF_PROVENANCE_BADGE_CONTRACT_REF: &str =
    "schemas/help/provenance_badge_vocabulary.schema.json";

/// Repo-relative path of the service-health destination contract this matrix
/// mirrors.
pub const M5_HANDOFF_SERVICE_HEALTH_CONTRACT_REF: &str =
    "schemas/help/service-health-destination.schema.json";

/// Repo-relative path of the handoff-target review contract this matrix builds on.
pub const M5_HANDOFF_TARGET_REVIEW_CONTRACT_REF: &str =
    "schemas/public/handoff_target_review.schema.json";

/// Repo-relative path of the reproduction-packet preview contract this matrix
/// builds on.
pub const M5_HANDOFF_REPRO_PACKET_CONTRACT_REF: &str =
    "schemas/public/repro_packet_preview.schema.json";

/// Repo-relative path of the product truth vocabulary register.
pub const M5_HANDOFF_PRODUCT_TRUTH_VOCABULARY_REF: &str =
    "artifacts/governance/product_truth_vocabulary.yaml";

/// Repo-relative path of the deployment-profile register that owns the
/// hosting-boundary vocabulary.
pub const M5_HANDOFF_DEPLOYMENT_PROFILES_REF: &str =
    "artifacts/governance/deployment_profiles.yaml";

/// Repo-relative path of the protected fixture directory.
pub const M5_PUBLIC_HANDOFF_MATRIX_FIXTURE_DIR: &str = "fixtures/help/m5-public-handoff";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PUBLIC_HANDOFF_MATRIX_ARTIFACT_REF: &str =
    "artifacts/help/m5-public-handoff/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_PUBLIC_HANDOFF_MATRIX_GOVERNANCE_REF: &str =
    "artifacts/help/m5-public-handoff-governance.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PUBLIC_HANDOFF_MATRIX_CSV_REF: &str = "artifacts/help/m5-public-handoff-matrix.csv";

/// One of the eight governed M5 public-handoff / capture-boundary objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffObjectKind {
    /// Post-install notice / provenance disclosure card that stays inspectable
    /// after install.
    PostInstallNotice,
    /// Provenance / source-authenticity disclosure distinguishing official,
    /// mirrored, side-loaded, and unknown sources.
    ProvenanceDisclosure,
    /// Official-versus-community outbound route descriptor that declares
    /// visibility and support class before launch.
    CommunityHandoffRoute,
    /// Redaction-safe reproduction packet that is previewed and redacted before
    /// share.
    ReproductionPacket,
    /// Offline-capture continuity record proving capture survives a failed or
    /// blocked handoff.
    OfflineCaptureContinuity,
    /// Device / microphone capture permission and capability-limit boundary.
    DevicePermissionBoundary,
    /// Embedded webview / auth boundary that never impersonates native trusted
    /// product chrome.
    EmbeddedAuthBoundary,
    /// Release / service-health communication notice.
    ServiceHealthNotice,
}

impl M5HandoffObjectKind {
    /// Every governed object, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PostInstallNotice,
        Self::ProvenanceDisclosure,
        Self::CommunityHandoffRoute,
        Self::ReproductionPacket,
        Self::OfflineCaptureContinuity,
        Self::DevicePermissionBoundary,
        Self::EmbeddedAuthBoundary,
        Self::ServiceHealthNotice,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostInstallNotice => "post_install_notice",
            Self::ProvenanceDisclosure => "provenance_disclosure",
            Self::CommunityHandoffRoute => "community_handoff_route",
            Self::ReproductionPacket => "reproduction_packet",
            Self::OfflineCaptureContinuity => "offline_capture_continuity",
            Self::DevicePermissionBoundary => "device_permission_boundary",
            Self::EmbeddedAuthBoundary => "embedded_auth_boundary",
            Self::ServiceHealthNotice => "service_health_notice",
        }
    }

    /// Controlled state vocabularies this object kind MUST declare.
    pub fn required_state_vocabularies(self) -> &'static [M5HandoffStateVocabulary] {
        use M5HandoffStateVocabulary as V;
        match self {
            Self::PostInstallNotice => &[V::ProvenanceClass, V::NoticeFreshnessState],
            Self::ProvenanceDisclosure => &[V::ProvenanceClass, V::NoticeFreshnessState],
            Self::CommunityHandoffRoute => &[V::RouteTrustClass, V::ContinuityState],
            Self::ReproductionPacket => &[V::RedactionState, V::ContinuityState],
            Self::OfflineCaptureContinuity => &[V::ContinuityState, V::RedactionState],
            Self::DevicePermissionBoundary => {
                &[V::CapturePermissionState, V::BoundaryChromeHonesty]
            }
            Self::EmbeddedAuthBoundary => &[V::BoundaryChromeHonesty, V::RouteTrustClass],
            Self::ServiceHealthNotice => &[V::RouteTrustClass, V::NoticeFreshnessState],
        }
    }
}

/// Qualification class for an M5 public-handoff / capture-boundary object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffQualificationClass {
    /// Object qualifies for the Stable claim.
    Stable,
    /// Object is narrowed to Beta.
    Beta,
    /// Object is narrowed to Preview.
    Preview,
    /// Object is experimental and not claimed.
    Experimental,
    /// Object is unavailable on this build.
    Unavailable,
    /// Object is held pending upstream resolution.
    Held,
}

impl M5HandoffQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the object may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Names one of the controlled state vocabularies a handoff object carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffStateVocabulary {
    /// Provenance / source-authenticity class.
    ProvenanceClass,
    /// Outbound-route trust class.
    RouteTrustClass,
    /// Capture permission / capability-limit state.
    CapturePermissionState,
    /// Redaction state of a shareable packet.
    RedactionState,
    /// Continuity state of an outbound or offline handoff.
    ContinuityState,
    /// Boundary chrome-honesty state (native vs embedded vs external).
    BoundaryChromeHonesty,
    /// Freshness state of a post-install or service-health notice.
    NoticeFreshnessState,
}

impl M5HandoffStateVocabulary {
    /// Every vocabulary, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProvenanceClass,
        Self::RouteTrustClass,
        Self::CapturePermissionState,
        Self::RedactionState,
        Self::ContinuityState,
        Self::BoundaryChromeHonesty,
        Self::NoticeFreshnessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceClass => "provenance_class",
            Self::RouteTrustClass => "route_trust_class",
            Self::CapturePermissionState => "capture_permission_state",
            Self::RedactionState => "redaction_state",
            Self::ContinuityState => "continuity_state",
            Self::BoundaryChromeHonesty => "boundary_chrome_honesty",
            Self::NoticeFreshnessState => "notice_freshness_state",
        }
    }
}

/// Controlled provenance / source-authenticity class for a post-install notice or
/// provenance disclosure.
///
/// Mirrors the install-mode and provenance-row tokens owned by the provenance
/// badge vocabulary so an `unknown` source can never be softened into an implied
/// official one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffProvenanceClass {
    /// Official, first-party, verifiable source.
    Official,
    /// Official build delivered through a recognized mirror.
    Mirrored,
    /// Side-loaded build installed outside an official or mirror channel.
    SideLoaded,
    /// Provenance could not be established.
    Unknown,
}

impl HandoffProvenanceClass {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Official,
        Self::Mirrored,
        Self::SideLoaded,
        Self::Unknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Mirrored => "mirrored",
            Self::SideLoaded => "side_loaded",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled outbound-route trust class for a community-handoff or embedded-auth
/// surface.
///
/// Mirrors the destination-class tokens owned by the community-handoff packet so a
/// community destination can never be presented as an official authenticated one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffRouteTrustClass {
    /// Official first-party destination.
    Official,
    /// Community destination, not vendor-verified.
    Community,
    /// Private managed / tenant destination.
    Private,
    /// Local-only posture; nothing leaves the device.
    LocalOnly,
}

impl HandoffRouteTrustClass {
    /// Every route trust class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Official,
        Self::Community,
        Self::Private,
        Self::LocalOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Community => "community",
            Self::Private => "private",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Controlled capture permission / capability-limit state for a device-permission
/// boundary.
///
/// Names the limit a capture surface stays within. A capture surface can never act
/// beyond the granted permission or capability scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffCapturePermissionState {
    /// Permission granted for the declared capability scope.
    Granted,
    /// Permission granted but limited to a narrower capability scope.
    ScopeLimited,
    /// Permission not yet requested.
    NotRequested,
    /// Permission denied.
    Denied,
    /// Previously granted permission revoked.
    Revoked,
}

impl HandoffCapturePermissionState {
    /// Every capture permission state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Granted,
        Self::ScopeLimited,
        Self::NotRequested,
        Self::Denied,
        Self::Revoked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::ScopeLimited => "scope_limited",
            Self::NotRequested => "not_requested",
            Self::Denied => "denied",
            Self::Revoked => "revoked",
        }
    }
}

/// Controlled redaction state for a shareable reproduction or offline-capture
/// packet.
///
/// Mirrors the redaction posture owned by the repro-packet preview contract so raw
/// sensitive material never leaves implicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffRedactionState {
    /// Redaction preview is required before share.
    PreviewRequired,
    /// Packet was previewed and redacted before share.
    PreviewedRedacted,
    /// Packet carries no sensitive material to redact.
    NoSensitiveMaterial,
    /// Share is blocked until the packet is previewed and redacted.
    UnredactedBlocked,
}

impl HandoffRedactionState {
    /// Every redaction state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PreviewRequired,
        Self::PreviewedRedacted,
        Self::NoSensitiveMaterial,
        Self::UnredactedBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRequired => "preview_required",
            Self::PreviewedRedacted => "previewed_redacted",
            Self::NoSensitiveMaterial => "no_sensitive_material",
            Self::UnredactedBlocked => "unredacted_blocked",
        }
    }
}

/// Controlled continuity state for an outbound or offline handoff.
///
/// Mirrors the continuity tokens owned by the community-handoff packet so a failed
/// or blocked launch always falls back to a durable local save.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffContinuityState {
    /// Ready to launch the outbound route.
    ReadyToLaunch,
    /// Launch failed; drafted material and selections are retained.
    LaunchFailedRetained,
    /// Launch blocked by policy; drafted material and selections are retained.
    BlockedRetained,
    /// Saved locally after a failed or blocked launch.
    OfflineSavedLocal,
}

impl HandoffContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReadyToLaunch,
        Self::LaunchFailedRetained,
        Self::BlockedRetained,
        Self::OfflineSavedLocal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToLaunch => "ready_to_launch",
            Self::LaunchFailedRetained => "launch_failed_retained",
            Self::BlockedRetained => "blocked_retained",
            Self::OfflineSavedLocal => "offline_saved_local",
        }
    }
}

/// Controlled boundary chrome-honesty state for a capture or embedded surface.
///
/// Distinguishes native trusted product chrome from embedded or external surfaces
/// so a webview or auth surface can never impersonate native chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBoundaryChromeHonesty {
    /// Native trusted product chrome.
    NativeTrustedChrome,
    /// Clearly disclosed embedded surface.
    ClearlyEmbedded,
    /// Labeled external / third-party surface.
    LabeledExternalSurface,
    /// An unattributed surface that would impersonate native chrome is blocked.
    UnattributedImpersonationBlocked,
}

impl HandoffBoundaryChromeHonesty {
    /// Every chrome-honesty state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NativeTrustedChrome,
        Self::ClearlyEmbedded,
        Self::LabeledExternalSurface,
        Self::UnattributedImpersonationBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeTrustedChrome => "native_trusted_chrome",
            Self::ClearlyEmbedded => "clearly_embedded",
            Self::LabeledExternalSurface => "labeled_external_surface",
            Self::UnattributedImpersonationBlocked => "unattributed_impersonation_blocked",
        }
    }
}

/// Controlled freshness state for a post-install or service-health notice.
///
/// Mirrors the freshness grammar so `stale` always means the same reserved state
/// across help, About, service-health, support, and release surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffNoticeFreshnessState {
    /// Proven current for the declared scope and freshness basis.
    ProvenCurrent,
    /// Cached notice shown with a disclosed cache posture.
    Cached,
    /// Notice is warming and not yet complete.
    Warming,
    /// Prior notice shown after its freshness floor was lost.
    Stale,
    /// Freshness could not be verified.
    Unverified,
}

impl HandoffNoticeFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProvenCurrent,
        Self::Cached,
        Self::Warming,
        Self::Stale,
        Self::Unverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenCurrent => "proven_current",
            Self::Cached => "cached",
            Self::Warming => "warming",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }
}

/// Evidence requirement level for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffEvidenceRequirement {
    /// At least one proof packet is required.
    Required,
    /// Proof is recommended but not blocking.
    Recommended,
    /// Proof is optional.
    Optional,
    /// Not applicable for this object's current qualification.
    NotApplicable,
}

impl M5HandoffEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow an object below its claimed qualification.
///
/// Each trigger names a gap the matrix must fail or narrow on rather than leave
/// implied — a stale notice, an unverified provenance, an undeclared route, a
/// missing redaction preview, a lost offline continuity, an exceeded capture
/// scope, or a native-chrome impersonation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffDowngradeTrigger {
    /// A post-install notice or provenance disclosure went stale.
    NoticeStale,
    /// Provenance could not be verified and dropped to unknown.
    ProvenanceUnverified,
    /// An outbound route did not declare visibility / support class before launch.
    RouteVisibilityUndeclared,
    /// A reproduction packet would share without a redaction preview.
    RedactionPreviewMissing,
    /// Offline capture did not survive a failed or blocked handoff.
    OfflineContinuityLost,
    /// A capture surface exceeded its granted permission or capability scope.
    CaptureScopeExceeded,
    /// An embedded / auth boundary impersonated native trusted product chrome.
    NativeChromeImpersonation,
    /// A policy or legal block applies.
    PolicyBlocked,
    /// The proof packet has gone stale.
    ProofStale,
    /// An upstream dependency object narrowed.
    UpstreamDependencyNarrowed,
}

impl M5HandoffDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::NoticeStale,
        Self::ProvenanceUnverified,
        Self::RouteVisibilityUndeclared,
        Self::RedactionPreviewMissing,
        Self::OfflineContinuityLost,
        Self::CaptureScopeExceeded,
        Self::NativeChromeImpersonation,
        Self::PolicyBlocked,
        Self::ProofStale,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoticeStale => "notice_stale",
            Self::ProvenanceUnverified => "provenance_unverified",
            Self::RouteVisibilityUndeclared => "route_visibility_undeclared",
            Self::RedactionPreviewMissing => "redaction_preview_missing",
            Self::OfflineContinuityLost => "offline_continuity_lost",
            Self::CaptureScopeExceeded => "capture_scope_exceeded",
            Self::NativeChromeImpersonation => "native_chrome_impersonation",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProofStale => "proof_stale",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffRollbackPosture {
    /// The post-install notice stays inspectable after install.
    NoticeStaysInspectableAfterInstall,
    /// Provenance stays labeled; an official source is never implied.
    ProvenanceLabeledNeverImplied,
    /// The outbound route declares visibility and support class before launch.
    RouteDeclaresVisibilityBeforeLaunch,
    /// Redaction preview is required before share.
    RedactionPreviewRequiredBeforeShare,
    /// Offline capture is saved locally after a failed or blocked handoff.
    OfflineCaptureSavedLocal,
    /// Capture stays within its granted permission and capability scope.
    CaptureStaysWithinGrantedScope,
    /// The boundary never impersonates native trusted product chrome.
    BoundaryNeverImpersonatesNativeChrome,
    /// Not applicable for the object's current qualification.
    NotApplicable,
}

impl M5HandoffRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoticeStaysInspectableAfterInstall => "notice_stays_inspectable_after_install",
            Self::ProvenanceLabeledNeverImplied => "provenance_labeled_never_implied",
            Self::RouteDeclaresVisibilityBeforeLaunch => "route_declares_visibility_before_launch",
            Self::RedactionPreviewRequiredBeforeShare => "redaction_preview_required_before_share",
            Self::OfflineCaptureSavedLocal => "offline_capture_saved_local",
            Self::CaptureStaysWithinGrantedScope => "capture_stays_within_granted_scope",
            Self::BoundaryNeverImpersonatesNativeChrome => {
                "boundary_never_impersonates_native_chrome"
            }
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project a handoff object's qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffConsumerSurface {
    /// Help / About surface.
    HelpAbout,
    /// Marketplace / extension storefront surface.
    Marketplace,
    /// Update / service-health surface.
    UpdateServiceHealth,
    /// Community-handoff surface.
    CommunityHandoff,
    /// Reproduction-packet surface.
    ReproductionPacket,
    /// Capture / auth surface.
    CaptureAuthSurface,
    /// Support / export packet.
    SupportExport,
    /// Documentation surface.
    Docs,
    /// Release notes.
    ReleaseNotes,
    /// Product UI surface.
    ProductUi,
}

impl M5HandoffConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::Marketplace => "marketplace",
            Self::UpdateServiceHealth => "update_service_health",
            Self::CommunityHandoff => "community_handoff",
            Self::ReproductionPacket => "reproduction_packet",
            Self::CaptureAuthSurface => "capture_auth_surface",
            Self::SupportExport => "support_export",
            Self::Docs => "docs",
            Self::ReleaseNotes => "release_notes",
            Self::ProductUi => "product_ui",
        }
    }
}

/// One row in the M5 public-handoff / capture-boundary matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffObjectRow {
    /// Governed handoff object.
    pub object_kind: M5HandoffObjectKind,
    /// Qualification class earned by this object.
    pub qualification: M5HandoffQualificationClass,
    /// Owner role accountable for keeping this object governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required fields the object must carry.
    pub required_fields: Vec<String>,
    /// Controlled state vocabularies this object carries.
    pub state_vocabularies: Vec<M5HandoffStateVocabulary>,
    /// Provenance classes admitted by this object.
    pub provenance_classes: Vec<HandoffProvenanceClass>,
    /// Route trust classes admitted by this object.
    pub route_trust_classes: Vec<HandoffRouteTrustClass>,
    /// Capture permission states admitted by this object.
    pub capture_permission_states: Vec<HandoffCapturePermissionState>,
    /// Redaction states admitted by this object.
    pub redaction_states: Vec<HandoffRedactionState>,
    /// Continuity states admitted by this object.
    pub continuity_states: Vec<HandoffContinuityState>,
    /// Boundary chrome-honesty states admitted by this object.
    pub boundary_chrome_states: Vec<HandoffBoundaryChromeHonesty>,
    /// Notice freshness states admitted by this object.
    pub notice_freshness_states: Vec<HandoffNoticeFreshnessState>,
    /// Evidence requirement level.
    pub evidence_requirement: M5HandoffEvidenceRequirement,
    /// Proof packet refs that keep this object current.
    pub required_proof_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this object.
    pub downgrade_triggers: Vec<M5HandoffDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5HandoffRollbackPosture,
    /// Source contract refs consumed by this object.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this object's qualification.
    pub consumer_surfaces: Vec<M5HandoffConsumerSurface>,
}

impl M5HandoffObjectRow {
    /// Returns true when the row declares the given vocabulary.
    fn declares(&self, vocab: M5HandoffStateVocabulary) -> bool {
        self.state_vocabularies.contains(&vocab)
    }

    /// Returns true when the token vec for `vocab` is non-empty.
    fn vocab_tokens_present(&self, vocab: M5HandoffStateVocabulary) -> bool {
        use M5HandoffStateVocabulary as V;
        match vocab {
            V::ProvenanceClass => !self.provenance_classes.is_empty(),
            V::RouteTrustClass => !self.route_trust_classes.is_empty(),
            V::CapturePermissionState => !self.capture_permission_states.is_empty(),
            V::RedactionState => !self.redaction_states.is_empty(),
            V::ContinuityState => !self.continuity_states.is_empty(),
            V::BoundaryChromeHonesty => !self.boundary_chrome_states.is_empty(),
            V::NoticeFreshnessState => !self.notice_freshness_states.is_empty(),
        }
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
///
/// Each field lists every canonical token for one controlled vocabulary, in
/// declaration order. The matrix validates each list against the typed `ALL`
/// arrays so the frozen vocabulary cannot silently drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffVocabularySet {
    /// Provenance-class tokens.
    pub provenance_classes: Vec<String>,
    /// Route-trust-class tokens.
    pub route_trust_classes: Vec<String>,
    /// Capture-permission-state tokens.
    pub capture_permission_states: Vec<String>,
    /// Redaction-state tokens.
    pub redaction_states: Vec<String>,
    /// Continuity-state tokens.
    pub continuity_states: Vec<String>,
    /// Boundary chrome-honesty tokens.
    pub boundary_chrome_states: Vec<String>,
    /// Notice-freshness-state tokens.
    pub notice_freshness_states: Vec<String>,
}

impl M5HandoffVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            provenance_classes: HandoffProvenanceClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            route_trust_classes: HandoffRouteTrustClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            capture_permission_states: HandoffCapturePermissionState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            redaction_states: HandoffRedactionState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            continuity_states: HandoffContinuityState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            boundary_chrome_states: HandoffBoundaryChromeHonesty::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            notice_freshness_states: HandoffNoticeFreshnessState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Trust and boundary-honesty review block.
///
/// Every flag is a hard invariant; all must hold for the matrix to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffTrustReview {
    /// Post-install provenance / notice states stay inspectable after install.
    pub post_install_provenance_inspectable_after_install: bool,
    /// Outbound routes declare visibility and support class before launch.
    pub outbound_routes_declare_visibility_and_support_class_before_launch: bool,
    /// Reproduction packets are previewed and redacted before share.
    pub repro_packets_previewed_and_redacted_before_share: bool,
    /// Offline capture survives a failed or blocked handoff.
    pub offline_capture_survives_failed_handoff: bool,
    /// Device / mic / auth / webview boundaries never impersonate native chrome.
    pub device_mic_auth_webview_never_impersonates_native_chrome: bool,
    /// Provenance states distinguish official, mirrored, side-loaded, and unknown.
    pub provenance_states_distinguish_official_mirrored_side_loaded_unknown: bool,
    /// Capture stays within its granted permission and capability scope.
    pub capture_stays_within_granted_permission_and_capability_limit: bool,
    /// Every surface points at one handoff object model, not parallel dialogs.
    pub one_handoff_object_model_not_parallel_dialogs: bool,
    /// No new community programs or capture modalities are invented.
    pub no_new_community_programs_or_capture_modalities: bool,
    /// Redaction default excludes raw sensitive material.
    pub redaction_default_excludes_raw_sensitive_material: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified objects automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffConsumerProjection {
    /// Help / About consumes the shared handoff object model.
    pub help_about_consumes_handoff_object_model: bool,
    /// Marketplace shows provenance class.
    pub marketplace_shows_provenance_class: bool,
    /// Update / service-health shows route trust and freshness.
    pub update_service_health_shows_route_trust_and_freshness: bool,
    /// Community handoff declares visibility and support class.
    pub community_handoff_declares_visibility_and_support_class: bool,
    /// Reproduction packets show the redaction preview.
    pub repro_packets_show_redaction_preview: bool,
    /// Capture / auth surfaces show the permission and chrome boundary.
    pub capture_auth_surfaces_show_permission_and_chrome_boundary: bool,
    /// Support export shows the shared handoff object model.
    pub support_export_shows_handoff_object_model: bool,
    /// Docs show provenance and redaction truth.
    pub docs_show_provenance_and_redaction_truth: bool,
    /// Release notes use the controlled vocabulary.
    pub release_notes_use_controlled_vocabulary: bool,
    /// Preview / Labs surfaces are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_objects: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the object.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the public-handoff lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet for the lane.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every object.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every object.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`M5PublicHandoffMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PublicHandoffMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5HandoffObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HandoffVocabularySet,
    /// Trust review block.
    pub trust_review: M5HandoffTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5HandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HandoffProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5HandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 public-handoff / capture-boundary matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicHandoffMatrixPacket {
    /// Record kind; must equal [`M5_PUBLIC_HANDOFF_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5HandoffObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HandoffVocabularySet,
    /// Trust review block.
    pub trust_review: M5HandoffTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5HandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HandoffProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5HandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PublicHandoffMatrixPacket {
    /// Builds an M5 public-handoff matrix packet from stable-lane input.
    pub fn new(input: M5PublicHandoffMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_PUBLIC_HANDOFF_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            object_rows: input.object_rows,
            vocabulary_set: input.vocabulary_set,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 public-handoff matrix invariants.
    pub fn validate(&self) -> Vec<M5PublicHandoffMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PUBLIC_HANDOFF_MATRIX_RECORD_KIND {
            violations.push(M5PublicHandoffMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_VERSION {
            violations.push(M5PublicHandoffMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PublicHandoffMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_object_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 public-handoff matrix packet serializes"),
        ) {
            violations.push(M5PublicHandoffMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 public-handoff matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed object,
    /// naming its qualification, owner, vocabularies, evidence, downgrade
    /// triggers, rollback posture, and consumer surfaces.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object,qualification,owner,state_vocabularies,evidence_requirement,downgrade_triggers,rollback_posture,consumer_surfaces\n",
        );
        for row in &self.object_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.object_kind.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.state_vocabularies, |v| v.as_str()),
                row.evidence_requirement.as_str(),
                join_tokens(&row.downgrade_triggers, |t| t.as_str()),
                row.rollback_posture.as_str(),
                join_tokens(&row.consumer_surfaces, |s| s.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .object_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Post-Install Notice/Provenance, Community-Handoff, Reproduction-Packet, and Device-Permission/Auth-Boundary Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Objects: {} ({} stable)\n",
            self.object_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Objects\n\n");
        for row in &self.object_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.object_kind.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Vocabularies: {}\n",
                row.state_vocabularies
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!("  - Rollback: {}\n", row.rollback_posture.as_str()));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 public-handoff matrix export.
#[derive(Debug)]
pub enum M5PublicHandoffMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PublicHandoffMatrixViolation>),
}

impl fmt::Display for M5PublicHandoffMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 public-handoff matrix export parse failed: {error}"
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
                    "m5 public-handoff matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PublicHandoffMatrixArtifactError {}

/// Validation failures emitted by [`M5PublicHandoffMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PublicHandoffMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object is missing from the matrix.
    RequiredObjectMissing,
    /// An object row is incomplete.
    ObjectRowIncomplete,
    /// An object row omits a vocabulary its kind requires.
    RequiredVocabularyMissing,
    /// A declared vocabulary has no concrete tokens.
    DeclaredVocabularyHasNoTokens,
    /// A token vec is populated for a vocabulary the row does not declare.
    UndeclaredVocabularyHasTokens,
    /// An object claiming Stable is missing required proof packet refs.
    StableObjectMissingProof,
    /// An object has no downgrade triggers.
    DowngradeTriggersMissing,
    /// An object has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5PublicHandoffMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ObjectRowIncomplete => "object_row_incomplete",
            Self::RequiredVocabularyMissing => "required_vocabulary_missing",
            Self::DeclaredVocabularyHasNoTokens => "declared_vocabulary_has_no_tokens",
            Self::UndeclaredVocabularyHasTokens => "undeclared_vocabulary_has_tokens",
            Self::StableObjectMissingProof => "stable_object_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 public-handoff matrix export.
pub fn current_stable_m5_public_handoff_matrix_export(
) -> Result<M5PublicHandoffMatrixPacket, M5PublicHandoffMatrixArtifactError> {
    let packet: M5PublicHandoffMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-public-handoff/support_export.json"
    )))
    .map_err(M5PublicHandoffMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PublicHandoffMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_REF,
        M5_PUBLIC_HANDOFF_MATRIX_DOC_REF,
        M5_HANDOFF_COMMUNITY_PACKET_CONTRACT_REF,
        M5_HANDOFF_PROVENANCE_BADGE_CONTRACT_REF,
        M5_HANDOFF_SERVICE_HEALTH_CONTRACT_REF,
        M5_HANDOFF_TARGET_REVIEW_CONTRACT_REF,
        M5_HANDOFF_REPRO_PACKET_CONTRACT_REF,
        M5_HANDOFF_PRODUCT_TRUTH_VOCABULARY_REF,
        M5_HANDOFF_DEPLOYMENT_PROFILES_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PublicHandoffMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PublicHandoffMatrixViolation::VocabularySetDrift);
    }
}

fn validate_object_rows(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    let present: BTreeSet<M5HandoffObjectKind> = packet
        .object_rows
        .iter()
        .map(|row| row.object_kind)
        .collect();
    for required in M5HandoffObjectKind::ALL {
        if !present.contains(&required) {
            violations.push(M5PublicHandoffMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.object_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.required_fields.is_empty()
            || row.state_vocabularies.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5PublicHandoffMatrixViolation::ObjectRowIncomplete);
        }

        for required_vocab in row.object_kind.required_state_vocabularies() {
            if !row.declares(*required_vocab) {
                violations.push(M5PublicHandoffMatrixViolation::RequiredVocabularyMissing);
            }
        }

        for vocab in M5HandoffStateVocabulary::ALL {
            let declared = row.declares(vocab);
            let has_tokens = row.vocab_tokens_present(vocab);
            if declared && !has_tokens {
                violations.push(M5PublicHandoffMatrixViolation::DeclaredVocabularyHasNoTokens);
            }
            if !declared && has_tokens {
                violations.push(M5PublicHandoffMatrixViolation::UndeclaredVocabularyHasTokens);
            }
        }

        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PublicHandoffMatrixViolation::StableObjectMissingProof);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PublicHandoffMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PublicHandoffMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.post_install_provenance_inspectable_after_install,
        review.outbound_routes_declare_visibility_and_support_class_before_launch,
        review.repro_packets_previewed_and_redacted_before_share,
        review.offline_capture_survives_failed_handoff,
        review.device_mic_auth_webview_never_impersonates_native_chrome,
        review.provenance_states_distinguish_official_mirrored_side_loaded_unknown,
        review.capture_stays_within_granted_permission_and_capability_limit,
        review.one_handoff_object_model_not_parallel_dialogs,
        review.no_new_community_programs_or_capture_modalities,
        review.redaction_default_excludes_raw_sensitive_material,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5PublicHandoffMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.help_about_consumes_handoff_object_model,
        projection.marketplace_shows_provenance_class,
        projection.update_service_health_shows_route_trust_and_freshness,
        projection.community_handoff_declares_visibility_and_support_class,
        projection.repro_packets_show_redaction_preview,
        projection.capture_auth_surfaces_show_permission_and_chrome_boundary,
        projection.support_export_shows_handoff_object_model,
        projection.docs_show_provenance_and_redaction_truth,
        projection.release_notes_use_controlled_vocabulary,
        projection.preview_labs_label_for_unqualified_objects,
    ] {
        if !ok {
            violations.push(M5PublicHandoffMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PublicHandoffMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PublicHandoffMatrixPacket,
    violations: &mut Vec<M5PublicHandoffMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(M5PublicHandoffMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
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
