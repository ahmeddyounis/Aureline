//! Root-capability envelope.
//!
//! Mirrors the ADR 0006 capability-flag set, save-mode taxonomy,
//! root-class vocabulary, and strongest / fallback identity-token
//! vocabularies. The envelope is registered at `vfs_root_attach`
//! and is the input to every save-mode and rename decision.

/// Frozen save-mode taxonomy. A save-target that cannot name one
/// of these modes MUST NOT be offered a save affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicWriteMode {
    AtomicReplace,
    InPlaceWrite,
    ConditionalRemoteWrite,
    Blocked,
}

impl AtomicWriteMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtomicReplace => "atomic_replace",
            Self::InPlaceWrite => "in_place_write",
            Self::ConditionalRemoteWrite => "conditional_remote_write",
            Self::Blocked => "blocked",
        }
    }
}

/// Frozen case-sensitivity vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaseSensitivity {
    Sensitive,
    InsensitivePreserving,
    InsensitiveNonPreserving,
}

impl CaseSensitivity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sensitive => "sensitive",
            Self::InsensitivePreserving => "insensitive_preserving",
            Self::InsensitiveNonPreserving => "insensitive_non_preserving",
        }
    }
}

/// Frozen Unicode-normalization-form vocabulary. Also used on the
/// [`crate::identity::CanonicalFilesystemObject`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NormalizationForm {
    None,
    Nfc,
    Nfd,
    MixedObserved,
}

impl NormalizationForm {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Nfc => "nfc",
            Self::Nfd => "nfd",
            Self::MixedObserved => "mixed_observed",
        }
    }
}

/// Symlink / junction escape policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymlinkEscapePolicy {
    Allow,
    Warn,
    Block,
}

impl SymlinkEscapePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Warn => "warn",
            Self::Block => "block",
        }
    }
}

/// Frozen root-class vocabulary. The class selects the
/// strongest-identity-token kind the root can provide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RootClass {
    LocalPosixLike,
    LocalWindowsLike,
    RemoteAgentMount,
    ContainerMount,
    VirtualGeneratedDocument,
    ArchiveLikeView,
}

impl RootClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalPosixLike => "local_posix_like",
            Self::LocalWindowsLike => "local_windows_like",
            Self::RemoteAgentMount => "remote_agent_mount",
            Self::ContainerMount => "container_mount",
            Self::VirtualGeneratedDocument => "virtual_generated_document",
            Self::ArchiveLikeView => "archive_like_view",
        }
    }
}

/// Frozen strongest-identity-token-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrongestIdentityTokenKind {
    FileIdGeneration,
    DeviceInodeGeneration,
    WindowsObjectId,
    ProviderObjectIdRevision,
    LogicalDocumentIdSourceRefs,
    ContentHashOnly,
}

impl StrongestIdentityTokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FileIdGeneration => "file_id_generation",
            Self::DeviceInodeGeneration => "device_inode_generation",
            Self::WindowsObjectId => "windows_object_id",
            Self::ProviderObjectIdRevision => "provider_object_id_revision",
            Self::LogicalDocumentIdSourceRefs => "logical_document_id_source_refs",
            Self::ContentHashOnly => "content_hash_only",
        }
    }
}

/// Frozen fallback-identity-token-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FallbackIdentityTokenKind {
    DeviceInode,
    InodeMtimeSize,
    ContentHash,
    RemoteRevisionToken,
}

impl FallbackIdentityTokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeviceInode => "device_inode",
            Self::InodeMtimeSize => "inode_mtime_size",
            Self::ContentHash => "content_hash",
            Self::RemoteRevisionToken => "remote_revision_token",
        }
    }
}

/// Mirror of the ADR-0006 capability-flag set. Carried on every
/// save-target token so non-VFS surfaces can explain which
/// guarantees hold without re-querying the root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityFlags {
    pub supports_atomic_replace: bool,
    pub supports_in_place_write: bool,
    pub supports_conditional_remote_write: bool,
    pub case_sensitivity: CaseSensitivity,
    pub unicode_normalization: NormalizationForm,
    pub supports_case_only_rename: bool,
    pub supports_unicode_normalization_rename: bool,
    pub symlink_escape_policy: SymlinkEscapePolicy,
    pub read_only: bool,
    pub policy_constrained: bool,
    pub review_required_before_save: bool,
    pub review_required_before_rename: bool,
    pub remote_container_adaptation: bool,
}

/// Per-root capability envelope advertised at attach time. Drives
/// save-mode selection, rename policy, and watcher bring-up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootCapabilityEnvelope {
    pub root_id: String,
    pub root_class: RootClass,
    pub capability_flags: CapabilityFlags,
    pub strongest_identity_token_kind: StrongestIdentityTokenKind,
    pub fallback_identity_token_kinds: Vec<FallbackIdentityTokenKind>,
    pub preferred_save_mode: AtomicWriteMode,
    pub permitted_save_modes: Vec<AtomicWriteMode>,
    pub watcher_source: crate::watcher::WatcherSource,
    pub mount_graph_hash: Option<String>,
}

impl RootCapabilityEnvelope {
    /// Pick the save mode the envelope advertises for the next
    /// save. Returns [`AtomicWriteMode::Blocked`] when the root is
    /// read-only or policy-constrained, regardless of which modes
    /// it permits — surfaces MUST route to review / save-as
    /// affordances in that case.
    pub fn select_save_mode(&self) -> AtomicWriteMode {
        if self.capability_flags.read_only || self.capability_flags.policy_constrained {
            return AtomicWriteMode::Blocked;
        }
        if self
            .permitted_save_modes
            .contains(&self.preferred_save_mode)
        {
            return self.preferred_save_mode;
        }
        // Fall back to the strongest permitted mode the envelope
        // still lists. Deterministic order so two identical
        // envelopes always pick the same fallback.
        for candidate in [
            AtomicWriteMode::AtomicReplace,
            AtomicWriteMode::ConditionalRemoteWrite,
            AtomicWriteMode::InPlaceWrite,
        ] {
            if self.permitted_save_modes.contains(&candidate) {
                return candidate;
            }
        }
        AtomicWriteMode::Blocked
    }
}
