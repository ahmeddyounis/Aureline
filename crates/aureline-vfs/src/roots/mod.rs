//! VFS root abstraction.
//!
//! A root is the unit of filesystem truth and capability disclosure. Every
//! document opened through the VFS resolves against exactly one root, yielding
//! the five-layer identity model (ADR 0006).

mod local_filesystem;
mod synthetic_root;
mod virtual_documents;

use crate::capabilities::RootCapabilityEnvelope;
use crate::identity::{FallbackIdentityToken, IdentityRecord, IdentityToken};
use crate::save::{GenerationToken, PermissionSnapshot};
use crate::uri_model::VfsUri;

pub use local_filesystem::{LocalFilesystemRoot, LocalFilesystemRootError};
pub use virtual_documents::{
    VirtualDocumentKind, VirtualDocumentRoot, VirtualDocumentRootError, VirtualDocumentSpec,
};

/// Root-side errors while resolving identity or reading root metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootResolveError {
    NotInRoot(VfsUri),
    UnknownPresentation(VfsUri),
    UnknownCanonical(VfsUri),
    IoFailure { uri: VfsUri, detail: String },
    UriInvalid { uri: String, detail: String },
}

impl std::fmt::Display for RootResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInRoot(uri) => write!(f, "uri not in root scope: {uri}"),
            Self::UnknownPresentation(uri) => write!(f, "unknown presentation uri: {uri}"),
            Self::UnknownCanonical(uri) => write!(f, "unknown canonical uri: {uri}"),
            Self::IoFailure { uri, detail } => write!(f, "io failure for {uri}: {detail}"),
            Self::UriInvalid { uri, detail } => write!(f, "invalid uri {uri}: {detail}"),
        }
    }
}

impl std::error::Error for RootResolveError {}

/// Root-side IO error for content reads/writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootIoError {
    NotSupported {
        uri: VfsUri,
        operation: &'static str,
    },
    IoFailure {
        uri: VfsUri,
        detail: String,
    },
}

impl std::fmt::Display for RootIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSupported { uri, operation } => {
                write!(f, "{operation} not supported for {uri}")
            }
            Self::IoFailure { uri, detail } => write!(f, "io failure for {uri}: {detail}"),
        }
    }
}

impl std::error::Error for RootIoError {}

/// Root interface required for VFS identity resolution and save coordination.
pub trait VfsRoot {
    /// Returns the root capability envelope registered at attach time.
    fn envelope(&self) -> &RootCapabilityEnvelope;

    /// Returns the short badge used in chrome (`local`, `remote`, `container`,
    /// `virtual`, `archive`).
    fn root_badge(&self) -> &str;

    /// Returns true when `uri` belongs to this root.
    fn claims_uri(&self, uri: &VfsUri) -> bool;

    /// Resolves a presentation URI into layers 1–4 of the identity model.
    fn identity_record(
        &self,
        presentation_uri: &VfsUri,
    ) -> Result<IdentityRecord, RootResolveError>;

    /// Reads the strongest identity token for the canonical object.
    fn read_strongest_identity_token(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<IdentityToken, RootResolveError>;

    /// Reads any fallback identity tokens for the canonical object.
    fn read_fallback_identity_tokens(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<Vec<FallbackIdentityToken>, RootResolveError>;

    /// Reads the compare-before-write generation token for the canonical object.
    fn read_generation_token(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<GenerationToken, RootResolveError>;

    /// Reads the permission snapshot for the canonical object.
    fn permission_snapshot(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<PermissionSnapshot, RootResolveError>;

    /// Reads the raw bytes for the canonical object.
    fn read_bytes(&self, canonical_uri: &VfsUri) -> Result<Vec<u8>, RootIoError>;

    /// Writes raw bytes to the canonical object.
    fn write_bytes(
        &mut self,
        canonical_uri: &VfsUri,
        new_content: Vec<u8>,
    ) -> Result<(), RootIoError>;
}
