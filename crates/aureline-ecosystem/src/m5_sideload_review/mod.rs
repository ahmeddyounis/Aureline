//! Canonical M5 sideload review sheets — one reviewed install model for an
//! unpacked or archive-backed package that lives on local disk.
//!
//! Where the [`install-review`](crate::m5_install_review) module reviews an install
//! or update of a package fetched through the registry, and the
//! [`author/publish preview`](crate::m5_author_and_publish_preview) module gates the
//! whole author-side lane before a package reaches the public registry, this module
//! freezes how a *side-loaded* package is reviewed before it is installed or
//! reloaded. A side-load never reaches the registry first, so the review sheet is the
//! only place its source identity, signing state, requested permissions, external
//! executables, and registry-binding decision are made explicit — and it must hold
//! the same review discipline as a public install rather than being waved through
//! because the artifact is already on disk.
//!
//! Each [`SideloadReviewSheet`] reuses the shared M5 vocabulary —
//! [`ArtifactFamily`], [`SourceClass`], [`RuntimeClass`], [`HostAbiClass`],
//! [`SignatureState`], [`TrustPosture`], and [`AntiAbuseTransparency`] — and adds the
//! sideload-specific facts: a [`SourceIdentity`] (an unpacked directory or a
//! content-addressed [`SourceKind::ArchiveBundle`]), the [`UpdateBinding`] decision
//! (stay local, bind to the registry later, or already bound to a registry
//! identity), the requested [`PermissionScope`] set, the disclosed
//! [`ExternalExecutable`] set, and — for a reload or update — an
//! [`InstalledSideloadState`] baseline to compare against.
//!
//! The sheet is honest by construction. Three published values are **recomputed**
//! from the sheet's facts, and the stored values must equal the recomputation or
//! validation fails:
//!
//! - **the rendered trust tier** is capped by *both* the signing state and the
//!   registry-binding decision, so a locally-built or side-loaded artifact can never
//!   render a [`TrustPosture::VerifiedPublisher`] or
//!   [`TrustPosture::EnterpriseApproved`] badge just because it was built or signed
//!   on a trusted machine — the registry-binding ceiling caps a side-load at
//!   [`TrustPosture::RegistryBound`] at most;
//! - **the review-trigger set** is computed from the installed baseline: a permission
//!   widening, a runtime-class change, a host/ABI rebind, a newly introduced external
//!   executable, a changed registry binding, or a changed release channel each force
//!   a [`SideloadDisposition::FreshReviewRequired`], so widening authority can never
//!   take effect through a silent hot reload; and
//! - **the disposition** is the stronger of the trigger gate and a hard
//!   [`SideloadDisposition::Blocked`] for a revoked signature or an anti-abuse
//!   quarantine.
//!
//! A reload that does not rebind to the registry must preserve the installed row's
//! limited-trust continuity: it can never silently raise the rendered badge. The
//! packet is checked in at `artifacts/ecosystem/m5/m5-sideload-review.json` and
//! embedded here, so this typed consumer and any CI gate agree on every sheet without
//! a cargo build in CI. The model is metadata-only: every field is a typed state, a
//! redacted display hint, or an opaque ref. It carries no absolute filesystem paths,
//! raw archive bytes, signing secrets, or external-executable payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::ArtifactFamily;
use crate::m5_author_and_publish_preview::{
    AntiAbuseTransparency, HostAbiClass, RuntimeClass, SignatureState, TrustPosture,
};
use crate::m5_marketplace_fact_views::SourceClass;

/// Supported M5 sideload-review schema version.
pub const M5_SIDELOAD_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SIDELOAD_REVIEW_RECORD_KIND: &str = "m5_sideload_review";

/// Repo-relative path to the checked-in packet.
pub const M5_SIDELOAD_REVIEW_PATH: &str = "artifacts/ecosystem/m5/m5-sideload-review.json";

/// Embedded checked-in packet JSON.
pub const M5_SIDELOAD_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-sideload-review.json"
));

/// Whether a side-loaded source is an unpacked directory or a content-addressed
/// archive bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// An unpacked package directory on local disk.
    UnpackedDirectory,
    /// A content-addressed archive bundle.
    ArchiveBundle,
}

impl SourceKind {
    /// Every source kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::UnpackedDirectory, Self::ArchiveBundle];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnpackedDirectory => "unpacked_directory",
            Self::ArchiveBundle => "archive_bundle",
        }
    }

    /// Whether this source kind is identified by a content address.
    pub const fn is_content_addressed(self) -> bool {
        matches!(self, Self::ArchiveBundle)
    }
}

/// Redacted class of a side-loaded source's location.
///
/// Carries the *class* of location only; the displayable hint is a redacted,
/// workspace-relative string and never an absolute machine path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePathClass {
    /// A path relative to the current workspace.
    WorkspaceRelativePath,
    /// A path relative to the user's home.
    UserHomeRelativePath,
    /// Removable media (a USB stick or external drive).
    RemovableMedia,
    /// A network mount.
    NetworkMount,
    /// A streamed import (no on-disk path).
    ProcessStream,
    /// No source-path class applies.
    NotApplicable,
}

impl SourcePathClass {
    /// Every source-path class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceRelativePath,
        Self::UserHomeRelativePath,
        Self::RemovableMedia,
        Self::NetworkMount,
        Self::ProcessStream,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRelativePath => "workspace_relative_path",
            Self::UserHomeRelativePath => "user_home_relative_path",
            Self::RemovableMedia => "removable_media",
            Self::NetworkMount => "network_mount",
            Self::ProcessStream => "process_stream",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The registry-binding decision attached to a side-loaded package.
///
/// The binding caps the [`TrustPosture`] a side-load may render: a package that
/// stays local or is only scheduled to bind later renders no inherited badge, and a
/// package already bound to a registry identity caps at [`TrustPosture::RegistryBound`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateBinding {
    /// The package stays local; it is never bound to a registry identity.
    StayLocal,
    /// The package stays local now but is marked to bind to the registry later.
    BindToRegistryLater,
    /// The package is bound to a registry release identity.
    BoundToRegistryIdentity,
}

impl UpdateBinding {
    /// Every update binding, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::StayLocal,
        Self::BindToRegistryLater,
        Self::BoundToRegistryIdentity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StayLocal => "stay_local",
            Self::BindToRegistryLater => "bind_to_registry_later",
            Self::BoundToRegistryIdentity => "bound_to_registry_identity",
        }
    }

    /// Whether the package is bound to a registry release identity.
    pub const fn is_registry_bound(self) -> bool {
        matches!(self, Self::BoundToRegistryIdentity)
    }

    /// Highest trust posture this binding lets a side-load render.
    ///
    /// A still-local binding caps at [`TrustPosture::UnsignedLocalOnly`]; only a
    /// package already bound to a registry identity may reach
    /// [`TrustPosture::RegistryBound`]. No binding ever permits a trusted-publisher
    /// badge, so a side-load can never inherit a verified or enterprise badge.
    pub const fn trust_ceiling(self) -> TrustPosture {
        match self {
            Self::StayLocal | Self::BindToRegistryLater => TrustPosture::UnsignedLocalOnly,
            Self::BoundToRegistryIdentity => TrustPosture::RegistryBound,
        }
    }
}

/// The release channel a side-loaded package tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannel {
    /// A local development channel.
    LocalDev,
    /// A preview channel.
    Preview,
    /// A stable channel.
    Stable,
}

impl ReleaseChannel {
    /// Every release channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::LocalDev, Self::Preview, Self::Stable];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDev => "local_dev",
            Self::Preview => "preview",
            Self::Stable => "stable",
        }
    }
}

/// The kind of permission scope a side-loaded package requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScopeKind {
    /// Read access to the filesystem.
    FilesystemRead,
    /// Write access to the filesystem.
    FilesystemWrite,
    /// Execute a shell command.
    ShellExecute,
    /// Egress to the network.
    NetworkEgress,
    /// Access to an AI provider.
    AiProviderAccess,
    /// Access to a connected provider.
    ConnectedProviderAccess,
    /// Use of a secret handle.
    SecretHandleUse,
    /// Read workspace settings.
    WorkspaceSettingsRead,
    /// Write workspace settings.
    WorkspaceSettingsWrite,
    /// Bind to an execution context.
    ExecutionContextBind,
    /// Subscribe to a subscription.
    SubscriptionSubscribe,
    /// Contribute a UI command.
    UiCommandContribute,
    /// Inherit a transitive capability.
    CapabilityInherit,
}

impl PermissionScopeKind {
    /// Every permission-scope kind, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::FilesystemRead,
        Self::FilesystemWrite,
        Self::ShellExecute,
        Self::NetworkEgress,
        Self::AiProviderAccess,
        Self::ConnectedProviderAccess,
        Self::SecretHandleUse,
        Self::WorkspaceSettingsRead,
        Self::WorkspaceSettingsWrite,
        Self::ExecutionContextBind,
        Self::SubscriptionSubscribe,
        Self::UiCommandContribute,
        Self::CapabilityInherit,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilesystemRead => "filesystem_read",
            Self::FilesystemWrite => "filesystem_write",
            Self::ShellExecute => "shell_execute",
            Self::NetworkEgress => "network_egress",
            Self::AiProviderAccess => "ai_provider_access",
            Self::ConnectedProviderAccess => "connected_provider_access",
            Self::SecretHandleUse => "secret_handle_use",
            Self::WorkspaceSettingsRead => "workspace_settings_read",
            Self::WorkspaceSettingsWrite => "workspace_settings_write",
            Self::ExecutionContextBind => "execution_context_bind",
            Self::SubscriptionSubscribe => "subscription_subscribe",
            Self::UiCommandContribute => "ui_command_contribute",
            Self::CapabilityInherit => "capability_inherit",
        }
    }
}

/// How a requested permission changes relative to the installed baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionChange {
    /// The permission is newly added.
    Added,
    /// The permission is widened (broader target or constraint).
    Widened,
    /// The permission is unchanged.
    Unchanged,
    /// The permission is narrowed.
    Narrowed,
    /// The permission is removed.
    Removed,
}

impl PermissionChange {
    /// Every permission change, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Added,
        Self::Widened,
        Self::Unchanged,
        Self::Narrowed,
        Self::Removed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Widened => "widened",
            Self::Unchanged => "unchanged",
            Self::Narrowed => "narrowed",
            Self::Removed => "removed",
        }
    }

    /// Whether this change widens the package's authority.
    pub const fn is_widening(self) -> bool {
        matches!(self, Self::Added | Self::Widened)
    }
}

/// The class of an external executable a side-loaded package discloses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalExecutableClass {
    /// An external host process the package launches.
    ExternalHostProcess,
    /// A helper binary the package ships.
    HelperBinary,
}

impl ExternalExecutableClass {
    /// Every external-executable class, in declaration order.
    pub const ALL: [Self; 2] = [Self::ExternalHostProcess, Self::HelperBinary];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalHostProcess => "external_host_process",
            Self::HelperBinary => "helper_binary",
        }
    }
}

/// How a disclosed external executable changes relative to the installed baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutableChange {
    /// The executable is newly introduced.
    Introduced,
    /// The executable is unchanged.
    Unchanged,
    /// The executable is removed.
    Removed,
}

impl ExecutableChange {
    /// Every executable change, in declaration order.
    pub const ALL: [Self; 3] = [Self::Introduced, Self::Unchanged, Self::Removed];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Introduced => "introduced",
            Self::Unchanged => "unchanged",
            Self::Removed => "removed",
        }
    }

    /// Whether this change introduces a new external executable.
    pub const fn is_introduced(self) -> bool {
        matches!(self, Self::Introduced)
    }
}

/// Whether a sheet covers a first side-load or a reload/update of an installed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideloadInstallKind {
    /// A first side-load with no installed baseline.
    FirstSideload,
    /// A reload or update of an already-installed side-load.
    ReloadOrUpdate,
}

impl SideloadInstallKind {
    /// Every install kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::FirstSideload, Self::ReloadOrUpdate];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstSideload => "first_sideload",
            Self::ReloadOrUpdate => "reload_or_update",
        }
    }

    /// Whether this install kind expects an installed baseline to compare against.
    pub const fn expects_baseline(self) -> bool {
        matches!(self, Self::ReloadOrUpdate)
    }
}

/// A computed trigger that forces a fresh review of an installed side-load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideloadReviewTrigger {
    /// The reload widens a requested permission.
    PermissionWidening,
    /// The reload changes the runtime class.
    RuntimeClassChanged,
    /// The reload rebinds to a different host or ABI.
    HostOrAbiRebound,
    /// The reload introduces a new external executable.
    ExternalExecutableIntroduced,
    /// The reload changes the registry-binding decision.
    UpdateBindingChanged,
    /// The reload changes the release channel.
    ReleaseChannelChanged,
}

impl SideloadReviewTrigger {
    /// Every review trigger, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PermissionWidening,
        Self::RuntimeClassChanged,
        Self::HostOrAbiRebound,
        Self::ExternalExecutableIntroduced,
        Self::UpdateBindingChanged,
        Self::ReleaseChannelChanged,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PermissionWidening => "permission_widening",
            Self::RuntimeClassChanged => "runtime_class_changed",
            Self::HostOrAbiRebound => "host_or_abi_rebound",
            Self::ExternalExecutableIntroduced => "external_executable_introduced",
            Self::UpdateBindingChanged => "update_binding_changed",
            Self::ReleaseChannelChanged => "release_channel_changed",
        }
    }

    /// The minimum disposition this trigger forces.
    ///
    /// Every widening or rebinding trigger forces a fresh review rather than a silent
    /// hot reload.
    pub const fn min_disposition(self) -> SideloadDisposition {
        SideloadDisposition::FreshReviewRequired
    }
}

/// The disposition a sideload review sheet publishes.
///
/// Ordered low-to-high by [`SideloadDisposition::rank`]: a
/// [`SideloadDisposition::ReviewedInstallReady`] sheet may be installed at its
/// limited-trust posture, and a [`SideloadDisposition::Blocked`] sheet must not be
/// installed at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideloadDisposition {
    /// The reviewed side-load may be installed at its limited-trust posture.
    ReviewedInstallReady,
    /// A widening or rebinding requires a fresh review before the change applies.
    FreshReviewRequired,
    /// The side-load is blocked (revoked signature or anti-abuse quarantine).
    Blocked,
}

impl SideloadDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReviewedInstallReady,
        Self::FreshReviewRequired,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewedInstallReady => "reviewed_install_ready",
            Self::FreshReviewRequired => "fresh_review_required",
            Self::Blocked => "blocked",
        }
    }

    /// Monotonic rank; higher means a stricter gate.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ReviewedInstallReady => 0,
            Self::FreshReviewRequired => 1,
            Self::Blocked => 2,
        }
    }

    /// The stricter (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// A reviewed action offered through the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideloadActionKind {
    /// Accept and install the reviewed side-load.
    AcceptSideload,
    /// Keep the package local (do not bind it to the registry).
    KeepLocal,
    /// Schedule the package to bind to the registry later.
    BindToRegistryLater,
    /// Request a fresh review of a widened or rebound reload.
    RequestFreshReview,
    /// Cancel the review without installing.
    Cancel,
}

impl SideloadActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AcceptSideload,
        Self::KeepLocal,
        Self::BindToRegistryLater,
        Self::RequestFreshReview,
        Self::Cancel,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcceptSideload => "accept_sideload",
            Self::KeepLocal => "keep_local",
            Self::BindToRegistryLater => "bind_to_registry_later",
            Self::RequestFreshReview => "request_fresh_review",
            Self::Cancel => "cancel",
        }
    }
}

/// The redacted source identity of a side-loaded package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceIdentity {
    /// Whether the source is an unpacked directory or an archive bundle.
    pub kind: SourceKind,
    /// Redacted class of the source location.
    pub path_class: SourcePathClass,
    /// Redacted, displayable hint (never an absolute machine path).
    pub display_hint: String,
    /// Opaque content-address ref for an archive bundle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_address_ref: Option<String>,
    /// Opaque ref to the local-source or import record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
}

impl SourceIdentity {
    /// Whether the display hint looks like an absolute machine path.
    ///
    /// Absolute paths must never appear in an export-safe record.
    pub fn hint_looks_absolute(&self) -> bool {
        let hint = self.display_hint.trim_start();
        hint.starts_with('/')
            || hint.starts_with('\\')
            || (hint.len() >= 3 && hint.as_bytes()[1] == b':' && hint.as_bytes()[2] == b'\\')
    }
}

/// A requested permission scope, with its change relative to the installed baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermissionScope {
    /// The kind of permission requested.
    pub kind: PermissionScopeKind,
    /// The redacted target of the permission.
    pub target: String,
    /// An optional constraint on the permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint: Option<String>,
    /// The author's rationale for the permission.
    pub rationale: String,
    /// How the permission changes relative to the baseline.
    pub change: PermissionChange,
}

impl PermissionScope {
    /// Whether this scope widens the package's authority.
    pub const fn is_widening(&self) -> bool {
        self.change.is_widening()
    }
}

/// A disclosed external executable, with its change relative to the baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalExecutable {
    /// The class of external executable.
    pub class: ExternalExecutableClass,
    /// Redacted identity label (never a raw absolute path).
    pub identity_label: String,
    /// The author's stated purpose for the executable.
    pub purpose_label: String,
    /// Opaque content-address ref for the executable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_address_ref: Option<String>,
    /// Opaque ref to a signing identity when the executable is signed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_by_ref: Option<String>,
    /// How the executable changes relative to the baseline.
    pub change: ExecutableChange,
}

impl ExternalExecutable {
    /// Whether this disclosure introduces a new external executable.
    pub const fn is_introduced(&self) -> bool {
        self.change.is_introduced()
    }
}

/// The installed baseline a reload or update is compared against.
///
/// Carried only on a [`SideloadInstallKind::ReloadOrUpdate`] sheet, so the review can
/// compute what widened and preserve the installed row's limited-trust continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstalledSideloadState {
    /// Opaque ref to the installed revision.
    pub revision_ref: String,
    /// The installed runtime class.
    pub runtime_class: RuntimeClass,
    /// The installed host/ABI class.
    pub host_abi: HostAbiClass,
    /// The installed registry-binding decision.
    pub update_binding: UpdateBinding,
    /// The installed release channel.
    pub release_channel: ReleaseChannel,
    /// The limited-trust badge the installed row currently renders.
    pub rendered_trust_tier: TrustPosture,
}

/// A scoped action offered through the sideload review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideloadAction {
    /// The kind of action.
    pub action_kind: SideloadActionKind,
    /// Opaque ref to the action.
    pub action_ref: String,
    /// Whether the action is currently enabled.
    pub enabled: bool,
}

/// One sideload review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideloadReviewSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Opaque ref to the local listing under review.
    pub listing_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this listing resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Publisher-trust source class.
    pub source_class: SourceClass,
    /// Namespaced extension identity of the form `publisher/extension`.
    pub extension_identity: String,
    /// Declared version label.
    pub extension_version: String,
    /// The redacted source identity.
    pub source: SourceIdentity,
    /// The runtime class of the side-loaded artifact.
    pub runtime_class: RuntimeClass,
    /// The host/ABI class of the side-loaded artifact.
    pub host_abi: HostAbiClass,
    /// The declared host/ABI range.
    pub host_abi_range: String,
    /// The signing/provenance state.
    pub signature_state: SignatureState,
    /// Opaque ref to the signature record, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_ref: Option<String>,
    /// The trust posture the listing claims before capping.
    pub claimed_trust_tier: TrustPosture,
    /// The rendered trust posture; must equal the recomputed cap.
    pub rendered_trust_tier: TrustPosture,
    /// The anti-abuse transparency state.
    pub anti_abuse: AntiAbuseTransparency,
    /// The requested permission scopes.
    #[serde(default)]
    pub requested_permissions: Vec<PermissionScope>,
    /// The disclosed external executables.
    #[serde(default)]
    pub external_executables: Vec<ExternalExecutable>,
    /// The registry-binding decision.
    pub update_binding: UpdateBinding,
    /// The release channel the package tracks.
    pub release_channel: ReleaseChannel,
    /// Whether this is a first side-load or a reload/update.
    pub install_kind: SideloadInstallKind,
    /// The installed baseline, present for a reload/update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installed_baseline: Option<InstalledSideloadState>,
    /// Scoped actions offered through the sheet.
    #[serde(default)]
    pub actions: Vec<SideloadAction>,
    /// Review triggers; must equal the recomputed set.
    #[serde(default)]
    pub review_triggers: Vec<SideloadReviewTrigger>,
    /// Disposition; must equal the recomputed value.
    pub disposition: SideloadDisposition,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl SideloadReviewSheet {
    /// Whether any requested permission widens the package's authority.
    pub fn widens_permissions(&self) -> bool {
        self.requested_permissions
            .iter()
            .any(PermissionScope::is_widening)
    }

    /// Whether the reload introduces a new external executable.
    pub fn introduces_external_executable(&self) -> bool {
        self.external_executables
            .iter()
            .any(ExternalExecutable::is_introduced)
    }

    /// Whether the package uses any external executable.
    pub fn uses_external_executable(&self) -> bool {
        self.external_executables
            .iter()
            .any(|e| e.change != ExecutableChange::Removed)
    }

    /// Whether the runtime class changes between the baseline and the proposed.
    pub fn runtime_class_changed(&self) -> bool {
        matches!(&self.installed_baseline, Some(b) if b.runtime_class != self.runtime_class)
    }

    /// Whether the host or ABI is rebound between the baseline and the proposed.
    pub fn host_or_abi_rebound(&self) -> bool {
        matches!(&self.installed_baseline, Some(b) if b.host_abi != self.host_abi)
    }

    /// Whether the registry-binding decision changes between baseline and proposed.
    pub fn update_binding_changed(&self) -> bool {
        matches!(&self.installed_baseline, Some(b) if b.update_binding != self.update_binding)
    }

    /// Whether the release channel changes between baseline and proposed.
    pub fn release_channel_changed(&self) -> bool {
        matches!(&self.installed_baseline, Some(b) if b.release_channel != self.release_channel)
    }

    /// Whether the signing state is a hard block (revoked).
    pub fn is_signature_revoked(&self) -> bool {
        matches!(self.signature_state, SignatureState::RevokedSignature)
    }

    /// Whether the family is quarantined under anti-abuse review.
    pub fn is_quarantined(&self) -> bool {
        self.anti_abuse.is_quarantined()
    }

    /// The rendered trust tier recomputed from this sheet's facts.
    ///
    /// The rendered tier is the weakest of the claimed tier, the signing-state
    /// ceiling, and the registry-binding ceiling, so a locally-built or side-loaded
    /// artifact never inherits a trusted-publisher badge.
    pub fn computed_rendered_trust_tier(&self) -> TrustPosture {
        self.claimed_trust_tier
            .min(self.signature_state.trust_ceiling())
            .min(self.update_binding.trust_ceiling())
    }

    /// The review triggers recomputed from this sheet's facts, in canonical order.
    pub fn computed_review_triggers(&self) -> Vec<SideloadReviewTrigger> {
        let mut triggers = Vec::new();
        if self.widens_permissions() {
            triggers.push(SideloadReviewTrigger::PermissionWidening);
        }
        if self.runtime_class_changed() {
            triggers.push(SideloadReviewTrigger::RuntimeClassChanged);
        }
        if self.host_or_abi_rebound() {
            triggers.push(SideloadReviewTrigger::HostOrAbiRebound);
        }
        if self.introduces_external_executable() {
            triggers.push(SideloadReviewTrigger::ExternalExecutableIntroduced);
        }
        if self.update_binding_changed() {
            triggers.push(SideloadReviewTrigger::UpdateBindingChanged);
        }
        if self.release_channel_changed() {
            triggers.push(SideloadReviewTrigger::ReleaseChannelChanged);
        }
        triggers
    }

    /// The disposition recomputed from this sheet's triggers and hard blocks.
    pub fn computed_disposition(&self) -> SideloadDisposition {
        let mut disposition = self.computed_review_triggers().into_iter().fold(
            SideloadDisposition::ReviewedInstallReady,
            |disposition, trigger| disposition.widen(trigger.min_disposition()),
        );
        if self.is_signature_revoked() || self.is_quarantined() {
            disposition = disposition.widen(SideloadDisposition::Blocked);
        }
        disposition
    }

    /// Whether a reload preserves the installed row's limited-trust continuity.
    ///
    /// A reload that does not rebind to the registry must never raise the rendered
    /// trust badge above the installed one.
    pub fn preserves_trust_continuity(&self) -> bool {
        match &self.installed_baseline {
            Some(baseline) if !self.update_binding_changed() => {
                self.computed_rendered_trust_tier().rank() <= baseline.rendered_trust_tier.rank()
            }
            _ => true,
        }
    }

    /// Whether the stored trust tier, triggers, and disposition agree with the
    /// recomputed values.
    pub fn gate_consistent(&self) -> bool {
        self.rendered_trust_tier == self.computed_rendered_trust_tier()
            && self.review_triggers == self.computed_review_triggers()
            && self.disposition == self.computed_disposition()
    }

    /// Whether the reviewed side-load may be installed at its limited-trust posture.
    pub fn allows_local_install(&self) -> bool {
        self.disposition == SideloadDisposition::ReviewedInstallReady
    }

    /// The accept action, if any.
    pub fn accept_action(&self) -> Option<&SideloadAction> {
        self.actions
            .iter()
            .find(|a| a.action_kind == SideloadActionKind::AcceptSideload)
    }

    /// Whether the sheet offers an action of the given kind.
    pub fn offers_action(&self, kind: SideloadActionKind) -> bool {
        self.actions.iter().any(|a| a.action_kind == kind)
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SideloadReviewSummary {
    /// Total review sheets.
    pub total_sheets: usize,
    /// Sheets ready for a limited-trust install.
    pub reviewed_install_ready_sheets: usize,
    /// Sheets that require a fresh review.
    pub fresh_review_required_sheets: usize,
    /// Sheets whose install is blocked.
    pub blocked_sheets: usize,
    /// Sheets that widen permissions.
    pub permission_widening_sheets: usize,
    /// Sheets that change runtime class.
    pub runtime_class_change_sheets: usize,
    /// Sheets that rebind host or ABI.
    pub host_rebind_sheets: usize,
    /// Sheets that introduce an external executable.
    pub external_executable_introduced_sheets: usize,
    /// Sheets that change the registry binding.
    pub update_binding_change_sheets: usize,
    /// Sheets that change the release channel.
    pub release_channel_change_sheets: usize,
    /// Sheets electing to stay local.
    pub stay_local_sheets: usize,
    /// Sheets electing to bind to the registry later.
    pub bind_to_registry_later_sheets: usize,
    /// Sheets bound to a registry identity.
    pub registry_bound_sheets: usize,
    /// Sheets that use an external executable.
    pub external_executable_using_sheets: usize,
    /// Sheets whose artifact is unsigned or revoked.
    pub unsigned_or_revoked_sheets: usize,
    /// Sheets that render a local-only trust badge.
    pub local_only_trust_sheets: usize,
    /// First-sideload sheets.
    pub first_sideload_sheets: usize,
    /// Reload/update sheets.
    pub reload_or_update_sheets: usize,
    /// Distinct package kinds across sheets.
    pub distinct_package_kinds: usize,
}

/// A redaction-safe export row for one sideload review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SideloadReviewExportRow {
    /// Sheet id.
    pub sheet_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-kind token.
    pub source_kind: String,
    /// Signature-state token.
    pub signature_state: String,
    /// Rendered-trust-tier token.
    pub rendered_trust_tier: String,
    /// Update-binding token.
    pub update_binding: String,
    /// Runtime-class token.
    pub runtime_class: String,
    /// Install-kind token.
    pub install_kind: String,
    /// Disposition token.
    pub disposition: String,
    /// Review-trigger tokens.
    pub review_triggers: Vec<String>,
    /// Whether the package uses an external executable.
    pub uses_external_executable: bool,
    /// Whether the side-load may be installed at its limited-trust posture.
    pub allows_local_install: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SideloadReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5SideloadReviewExportRow>,
    /// Whether every sheet's gate is consistent with its recomputation.
    pub all_gates_consistent: bool,
    /// Sheets that require a fresh review.
    pub fresh_review_required_count: usize,
    /// Sheets whose install is blocked.
    pub blocked_count: usize,
}

/// The typed M5 sideload-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SideloadReview {
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
    /// Closed source-class vocabulary (reused from the marketplace fact-views).
    pub source_classes: Vec<SourceClass>,
    /// Closed runtime-class vocabulary (reused from the publish-preview gate).
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary (reused from the publish-preview gate).
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed signature-state vocabulary (reused from the publish-preview gate).
    pub signature_states: Vec<SignatureState>,
    /// Closed trust-posture vocabulary (reused from the publish-preview gate).
    pub trust_postures: Vec<TrustPosture>,
    /// Closed anti-abuse vocabulary (reused from the publish-preview gate).
    pub anti_abuse_states: Vec<AntiAbuseTransparency>,
    /// Closed source-kind vocabulary.
    pub source_kinds: Vec<SourceKind>,
    /// Closed source-path-class vocabulary.
    pub source_path_classes: Vec<SourcePathClass>,
    /// Closed update-binding vocabulary.
    pub update_bindings: Vec<UpdateBinding>,
    /// Closed release-channel vocabulary.
    pub release_channels: Vec<ReleaseChannel>,
    /// Closed permission-scope-kind vocabulary.
    pub permission_scope_kinds: Vec<PermissionScopeKind>,
    /// Closed permission-change vocabulary.
    pub permission_changes: Vec<PermissionChange>,
    /// Closed external-executable-class vocabulary.
    pub external_executable_classes: Vec<ExternalExecutableClass>,
    /// Closed executable-change vocabulary.
    pub executable_changes: Vec<ExecutableChange>,
    /// Closed install-kind vocabulary.
    pub install_kinds: Vec<SideloadInstallKind>,
    /// Closed review-trigger vocabulary.
    pub review_triggers: Vec<SideloadReviewTrigger>,
    /// Closed disposition vocabulary.
    pub dispositions: Vec<SideloadDisposition>,
    /// Closed action-kind vocabulary.
    pub action_kinds: Vec<SideloadActionKind>,
    /// The sideload review sheets.
    #[serde(default)]
    pub review_sheets: Vec<SideloadReviewSheet>,
    /// Summary counts.
    pub summary: M5SideloadReviewSummary,
}

impl M5SideloadReview {
    /// Returns the review sheet with the given id.
    pub fn review_sheet(&self, sheet_id: &str) -> Option<&SideloadReviewSheet> {
        self.review_sheets.iter().find(|s| s.sheet_id == sheet_id)
    }

    /// Review sheets that require a fresh review or are blocked.
    pub fn sheets_requiring_review(&self) -> impl Iterator<Item = &SideloadReviewSheet> {
        self.review_sheets
            .iter()
            .filter(|s| !s.allows_local_install())
    }

    /// Whether every sheet's stored gate agrees with its recomputation.
    pub fn all_gates_consistent(&self) -> bool {
        self.review_sheets
            .iter()
            .all(SideloadReviewSheet::gate_consistent)
    }

    /// Whether every sheet preserves the installed row's limited-trust continuity.
    pub fn all_trust_continuity_preserved(&self) -> bool {
        self.review_sheets
            .iter()
            .all(SideloadReviewSheet::preserves_trust_continuity)
    }

    /// Recomputes the summary block from the review sheets.
    pub fn computed_summary(&self) -> M5SideloadReviewSummary {
        let count_disposition = |disposition: SideloadDisposition| {
            self.review_sheets
                .iter()
                .filter(|s| s.disposition == disposition)
                .count()
        };
        let count_binding = |binding: UpdateBinding| {
            self.review_sheets
                .iter()
                .filter(|s| s.update_binding == binding)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.review_sheets.iter().map(|s| s.package_kind).collect();
        M5SideloadReviewSummary {
            total_sheets: self.review_sheets.len(),
            reviewed_install_ready_sheets: count_disposition(
                SideloadDisposition::ReviewedInstallReady,
            ),
            fresh_review_required_sheets: count_disposition(
                SideloadDisposition::FreshReviewRequired,
            ),
            blocked_sheets: count_disposition(SideloadDisposition::Blocked),
            permission_widening_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.widens_permissions())
                .count(),
            runtime_class_change_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.runtime_class_changed())
                .count(),
            host_rebind_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.host_or_abi_rebound())
                .count(),
            external_executable_introduced_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.introduces_external_executable())
                .count(),
            update_binding_change_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.update_binding_changed())
                .count(),
            release_channel_change_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.release_channel_changed())
                .count(),
            stay_local_sheets: count_binding(UpdateBinding::StayLocal),
            bind_to_registry_later_sheets: count_binding(UpdateBinding::BindToRegistryLater),
            registry_bound_sheets: count_binding(UpdateBinding::BoundToRegistryIdentity),
            external_executable_using_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.uses_external_executable())
                .count(),
            unsigned_or_revoked_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.signature_state.is_local_or_untrusted())
                .count(),
            local_only_trust_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.rendered_trust_tier == TrustPosture::UnsignedLocalOnly)
                .count(),
            first_sideload_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.install_kind == SideloadInstallKind::FirstSideload)
                .count(),
            reload_or_update_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.install_kind == SideloadInstallKind::ReloadOrUpdate)
                .count(),
            distinct_package_kinds: package_kinds.len(),
        }
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// docs/help, and release/public-truth packets — render instead of restating
    /// sideload source, trust, and disposition text by hand.
    pub fn export_projection(&self) -> M5SideloadReviewExportProjection {
        let rows = self
            .review_sheets
            .iter()
            .map(|s| M5SideloadReviewExportRow {
                sheet_id: s.sheet_id.clone(),
                package_kind: s.package_kind.as_str().to_owned(),
                source_kind: s.source.kind.as_str().to_owned(),
                signature_state: s.signature_state.as_str().to_owned(),
                rendered_trust_tier: s.rendered_trust_tier.as_str().to_owned(),
                update_binding: s.update_binding.as_str().to_owned(),
                runtime_class: s.runtime_class.as_str().to_owned(),
                install_kind: s.install_kind.as_str().to_owned(),
                disposition: s.disposition.as_str().to_owned(),
                review_triggers: s
                    .review_triggers
                    .iter()
                    .map(|trigger| trigger.as_str().to_owned())
                    .collect(),
                uses_external_executable: s.uses_external_executable(),
                allows_local_install: s.allows_local_install(),
                summary: format!(
                    "{}: {} side-load from {}, signing {}, binding {}, renders {}, disposition {}",
                    s.package_kind.as_str(),
                    s.install_kind.as_str(),
                    s.source.kind.as_str(),
                    s.signature_state.as_str(),
                    s.update_binding.as_str(),
                    s.rendered_trust_tier.as_str(),
                    s.disposition.as_str(),
                ),
            })
            .collect();
        M5SideloadReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_gates_consistent: self.all_gates_consistent(),
            fresh_review_required_count: self
                .review_sheets
                .iter()
                .filter(|s| s.disposition == SideloadDisposition::FreshReviewRequired)
                .count(),
            blocked_count: self
                .review_sheets
                .iter()
                .filter(|s| s.disposition == SideloadDisposition::Blocked)
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5SideloadReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_sheets = BTreeSet::new();
        for sheet in &self.review_sheets {
            if !seen_sheets.insert(sheet.sheet_id.clone()) {
                violations.push(M5SideloadReviewViolation::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            self.validate_sheet(sheet, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SideloadReviewViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SideloadReviewViolation>) {
        if self.schema_version != M5_SIDELOAD_REVIEW_SCHEMA_VERSION {
            violations.push(M5SideloadReviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_SIDELOAD_REVIEW_RECORD_KIND {
            violations.push(M5SideloadReviewViolation::UnsupportedRecordKind {
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
                violations.push(M5SideloadReviewViolation::EmptyField {
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
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "anti_abuse_states",
                self.anti_abuse_states == AntiAbuseTransparency::ALL.to_vec(),
            ),
            (
                "source_kinds",
                self.source_kinds == SourceKind::ALL.to_vec(),
            ),
            (
                "source_path_classes",
                self.source_path_classes == SourcePathClass::ALL.to_vec(),
            ),
            (
                "update_bindings",
                self.update_bindings == UpdateBinding::ALL.to_vec(),
            ),
            (
                "release_channels",
                self.release_channels == ReleaseChannel::ALL.to_vec(),
            ),
            (
                "permission_scope_kinds",
                self.permission_scope_kinds == PermissionScopeKind::ALL.to_vec(),
            ),
            (
                "permission_changes",
                self.permission_changes == PermissionChange::ALL.to_vec(),
            ),
            (
                "external_executable_classes",
                self.external_executable_classes == ExternalExecutableClass::ALL.to_vec(),
            ),
            (
                "executable_changes",
                self.executable_changes == ExecutableChange::ALL.to_vec(),
            ),
            (
                "install_kinds",
                self.install_kinds == SideloadInstallKind::ALL.to_vec(),
            ),
            (
                "review_triggers",
                self.review_triggers == SideloadReviewTrigger::ALL.to_vec(),
            ),
            (
                "dispositions",
                self.dispositions == SideloadDisposition::ALL.to_vec(),
            ),
            (
                "action_kinds",
                self.action_kinds == SideloadActionKind::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5SideloadReviewViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_sheet(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        for (field, value) in [
            ("sheet_id", &sheet.sheet_id),
            ("listing_ref", &sheet.listing_ref),
            ("display_label", &sheet.display_label),
            ("governance_family_ref", &sheet.governance_family_ref),
            ("extension_identity", &sheet.extension_identity),
            ("extension_version", &sheet.extension_version),
            ("host_abi_range", &sheet.host_abi_range),
            ("summary", &sheet.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SideloadReviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: field,
                });
            }
        }

        // The extension identity must be the namespaced publisher/extension form, so a
        // side-load can never present an ambiguous unscoped id.
        if !sheet.extension_identity.trim().is_empty() && !sheet.extension_identity.contains('/') {
            violations.push(M5SideloadReviewViolation::MalformedExtensionIdentity {
                sheet_id: sheet.sheet_id.clone(),
                identity: sheet.extension_identity.clone(),
            });
        }

        self.validate_source(sheet, violations);
        self.validate_signature(sheet, violations);
        self.validate_baseline(sheet, violations);
        self.validate_permissions(sheet, violations);
        self.validate_executables(sheet, violations);
        self.validate_actions(sheet, violations);
        self.validate_gate(sheet, violations);
    }

    fn validate_source(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        let source = &sheet.source;
        if source.display_hint.trim().is_empty() {
            violations.push(M5SideloadReviewViolation::EmptyField {
                id: sheet.sheet_id.clone(),
                field_name: "source.display_hint",
            });
        }
        // An archive bundle is identified by its content address; an unpacked
        // directory must not be.
        let has_address = source.content_address_ref.is_some();
        if source.kind.is_content_addressed() != has_address {
            violations.push(M5SideloadReviewViolation::SourceIdentityInconsistent {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        // No absolute machine path ever appears in an export-safe record.
        if source.hint_looks_absolute() {
            violations.push(M5SideloadReviewViolation::SourceHintLooksAbsolute {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }

    fn validate_signature(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        // A signed or revoked artifact must name its signature record; an unsigned one
        // must not, so the signing state is never overstated or hidden.
        let expects_ref = matches!(
            sheet.signature_state,
            SignatureState::SignedVerified
                | SignatureState::SignedUnverified
                | SignatureState::RevokedSignature
        );
        if sheet.signature_ref.is_some() != expects_ref {
            violations.push(M5SideloadReviewViolation::SignatureRefInconsistent {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }

    fn validate_baseline(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        // A reload/update must carry a baseline, and a first side-load must not, so the
        // comparison model is honest about what it compares.
        match (
            &sheet.installed_baseline,
            sheet.install_kind.expects_baseline(),
        ) {
            (None, true) => {
                violations.push(M5SideloadReviewViolation::MissingInstalledBaseline {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            (Some(_), false) => {
                violations.push(M5SideloadReviewViolation::UnexpectedInstalledBaseline {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            _ => {}
        }

        // A first side-load presents its full surface as the initial review, not as a
        // delta, so no permission or executable change may be recorded.
        if sheet.install_kind == SideloadInstallKind::FirstSideload {
            let has_permission_delta = sheet
                .requested_permissions
                .iter()
                .any(|p| p.change != PermissionChange::Unchanged);
            let has_executable_delta = sheet
                .external_executables
                .iter()
                .any(|e| e.change != ExecutableChange::Unchanged);
            if has_permission_delta || has_executable_delta {
                violations.push(M5SideloadReviewViolation::UnexpectedDeltaOnFirstSideload {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
        }

        // A reload that does not rebind to the registry must never silently raise the
        // installed row's limited-trust badge.
        if !sheet.preserves_trust_continuity() {
            violations.push(M5SideloadReviewViolation::TrustContinuityElevated {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        if let Some(baseline) = &sheet.installed_baseline {
            if baseline.revision_ref.trim().is_empty() {
                violations.push(M5SideloadReviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: "installed_baseline.revision_ref",
                });
            }
        }
    }

    fn validate_permissions(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        for permission in &sheet.requested_permissions {
            for (field, value) in [
                ("permission.target", &permission.target),
                ("permission.rationale", &permission.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5SideloadReviewViolation::EmptyField {
                        id: sheet.sheet_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_executables(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        for executable in &sheet.external_executables {
            for (field, value) in [
                ("executable.identity_label", &executable.identity_label),
                ("executable.purpose_label", &executable.purpose_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5SideloadReviewViolation::EmptyField {
                        id: sheet.sheet_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_actions(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        for action in &sheet.actions {
            if action.action_ref.trim().is_empty() {
                violations.push(M5SideloadReviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: "action_ref",
                });
            }
        }

        if !sheet.offers_action(SideloadActionKind::AcceptSideload) {
            violations.push(M5SideloadReviewViolation::MissingRequiredAction {
                sheet_id: sheet.sheet_id.clone(),
                action: SideloadActionKind::AcceptSideload.as_str(),
            });
        }
        if !sheet.offers_action(SideloadActionKind::Cancel) {
            violations.push(M5SideloadReviewViolation::MissingRequiredAction {
                sheet_id: sheet.sheet_id.clone(),
                action: SideloadActionKind::Cancel.as_str(),
            });
        }

        // A blocked side-load must not expose an enabled accept action, and a
        // fresh-review-required reload must not let the user accept it without a fresh
        // review — the install-style review is never bypassed.
        if sheet.disposition != SideloadDisposition::ReviewedInstallReady {
            if let Some(accept) = sheet.accept_action() {
                if accept.enabled {
                    violations.push(M5SideloadReviewViolation::AcceptEnabledWithoutReview {
                        sheet_id: sheet.sheet_id.clone(),
                        disposition: sheet.disposition.as_str(),
                    });
                }
            }
        }
    }

    fn validate_gate(
        &self,
        sheet: &SideloadReviewSheet,
        violations: &mut Vec<M5SideloadReviewViolation>,
    ) {
        let mut seen_triggers = BTreeSet::new();
        for trigger in &sheet.review_triggers {
            if !seen_triggers.insert(*trigger) {
                violations.push(M5SideloadReviewViolation::DuplicateReviewTrigger {
                    sheet_id: sheet.sheet_id.clone(),
                    trigger: trigger.as_str(),
                });
            }
        }

        // The rendered trust tier must equal the recomputed cap, so a local or
        // side-loaded artifact can never assert a stronger badge than its signing
        // state and registry binding allow.
        let computed_tier = sheet.computed_rendered_trust_tier();
        if sheet.rendered_trust_tier != computed_tier {
            violations.push(M5SideloadReviewViolation::RenderedTrustMismatch {
                sheet_id: sheet.sheet_id.clone(),
                stored: sheet.rendered_trust_tier.as_str(),
                computed: computed_tier.as_str(),
            });
        }

        // The rendered tier of a side-load can never be a trusted-publisher badge.
        if sheet.rendered_trust_tier.is_trusted_badge() {
            violations.push(M5SideloadReviewViolation::SideloadInheritsTrustedBadge {
                sheet_id: sheet.sheet_id.clone(),
                tier: sheet.rendered_trust_tier.as_str(),
            });
        }

        // The recorded triggers must equal the recomputed set, so a widening or
        // rebinding can never be asserted or hidden by hand.
        if sheet.review_triggers != sheet.computed_review_triggers() {
            violations.push(M5SideloadReviewViolation::ReviewTriggersMismatch {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        // The published disposition must equal the recomputed gate, so a widened or
        // rebound reload can never present a narrower install path than its facts
        // warrant.
        let computed_disposition = sheet.computed_disposition();
        if sheet.disposition != computed_disposition {
            violations.push(M5SideloadReviewViolation::DispositionMismatch {
                sheet_id: sheet.sheet_id.clone(),
                stored: sheet.disposition.as_str(),
                computed: computed_disposition.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 sideload-review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SideloadReviewViolation {
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
    /// A closed vocabulary field is not the canonical value.
    ClosedVocabularyMismatch {
        /// Field name.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Owning id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// Two review sheets share an id.
    DuplicateSheetId {
        /// Duplicated sheet id.
        sheet_id: String,
    },
    /// An extension identity is not in the namespaced publisher/extension form.
    MalformedExtensionIdentity {
        /// Owning sheet id.
        sheet_id: String,
        /// The malformed identity.
        identity: String,
    },
    /// The source kind disagrees with the presence of a content address.
    SourceIdentityInconsistent {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// The source display hint looks like an absolute machine path.
    SourceHintLooksAbsolute {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// The signature ref disagrees with the signing state.
    SignatureRefInconsistent {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// A reload/update is missing its installed baseline.
    MissingInstalledBaseline {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// A first side-load carries an installed baseline.
    UnexpectedInstalledBaseline {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// A first side-load records a permission or executable delta.
    UnexpectedDeltaOnFirstSideload {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// A reload silently raised the installed row's limited-trust badge.
    TrustContinuityElevated {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// An action ref is missing or a required action is absent.
    MissingRequiredAction {
        /// Owning sheet id.
        sheet_id: String,
        /// The missing action.
        action: &'static str,
    },
    /// A non-ready sheet exposes an enabled accept action.
    AcceptEnabledWithoutReview {
        /// Owning sheet id.
        sheet_id: String,
        /// The sheet's disposition.
        disposition: &'static str,
    },
    /// A review trigger is repeated on one sheet.
    DuplicateReviewTrigger {
        /// Owning sheet id.
        sheet_id: String,
        /// The repeated trigger.
        trigger: &'static str,
    },
    /// The rendered trust tier disagrees with the recomputed cap.
    RenderedTrustMismatch {
        /// Owning sheet id.
        sheet_id: String,
        /// Stored value.
        stored: &'static str,
        /// Recomputed value.
        computed: &'static str,
    },
    /// A side-load renders a trusted-publisher badge.
    SideloadInheritsTrustedBadge {
        /// Owning sheet id.
        sheet_id: String,
        /// The rendered tier.
        tier: &'static str,
    },
    /// The stored review triggers disagree with the recomputed set.
    ReviewTriggersMismatch {
        /// Owning sheet id.
        sheet_id: String,
    },
    /// The stored disposition disagrees with the recomputed gate.
    DispositionMismatch {
        /// Owning sheet id.
        sheet_id: String,
        /// Stored value.
        stored: &'static str,
        /// Recomputed value.
        computed: &'static str,
    },
    /// The packet summary counts disagree with the review sheets.
    SummaryMismatch,
}

impl fmt::Display for M5SideloadReviewViolation {
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
                write!(f, "duplicate review sheet id {sheet_id}")
            }
            Self::MalformedExtensionIdentity { sheet_id, identity } => {
                write!(
                    f,
                    "sheet {sheet_id} extension identity {identity} is not publisher/extension form"
                )
            }
            Self::SourceIdentityInconsistent { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} source kind disagrees with the presence of a content address"
                )
            }
            Self::SourceHintLooksAbsolute { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} source display hint looks like an absolute machine path"
                )
            }
            Self::SignatureRefInconsistent { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} signature ref disagrees with its signing state"
                )
            }
            Self::MissingInstalledBaseline { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} is a reload/update but carries no installed baseline"
                )
            }
            Self::UnexpectedInstalledBaseline { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} is a first side-load but carries an installed baseline"
                )
            }
            Self::UnexpectedDeltaOnFirstSideload { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} is a first side-load but records a permission or executable delta"
                )
            }
            Self::TrustContinuityElevated { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} raises the installed row's trust badge without rebinding to the registry"
                )
            }
            Self::MissingRequiredAction { sheet_id, action } => {
                write!(f, "sheet {sheet_id} is missing required action {action}")
            }
            Self::AcceptEnabledWithoutReview {
                sheet_id,
                disposition,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} disposition {disposition} but exposes an enabled accept action"
                )
            }
            Self::DuplicateReviewTrigger { sheet_id, trigger } => {
                write!(f, "sheet {sheet_id} repeats review trigger {trigger}")
            }
            Self::RenderedTrustMismatch {
                sheet_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} renders trust tier {stored} but the recomputed cap is {computed}"
                )
            }
            Self::SideloadInheritsTrustedBadge { sheet_id, tier } => {
                write!(
                    f,
                    "sheet {sheet_id} renders trusted-publisher badge {tier} for a side-load"
                )
            }
            Self::ReviewTriggersMismatch { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} review triggers disagree with the recomputed set"
                )
            }
            Self::DispositionMismatch {
                sheet_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} publishes disposition {stored} but the recomputed gate is {computed}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the review sheets")
            }
        }
    }
}

impl Error for M5SideloadReviewViolation {}

/// Loads the embedded M5 sideload-review packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SideloadReview`].
pub fn current_m5_sideload_review() -> Result<M5SideloadReview, serde_json::Error> {
    serde_json::from_str(M5_SIDELOAD_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
