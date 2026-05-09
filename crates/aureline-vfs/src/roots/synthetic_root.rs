//! [`crate::synthetic::SyntheticRoot`] adapter for the [`super::VfsRoot`] trait.

use crate::identity::{FallbackIdentityToken, IdentityToken};
use crate::save::{GenerationToken, GenerationTokenKind, PermissionSnapshot};
use crate::synthetic::SyntheticRoot;
use crate::uri_model::VfsUri;

use super::{RootIoError, RootResolveError, VfsRoot};

impl VfsRoot for SyntheticRoot {
    fn envelope(&self) -> &crate::capabilities::RootCapabilityEnvelope {
        self.envelope()
    }

    fn root_badge(&self) -> &str {
        self.root_badge()
    }

    fn claims_uri(&self, uri: &VfsUri) -> bool {
        self.resolve(uri.as_str()).is_ok()
    }

    fn identity_record(&self, presentation_uri: &VfsUri) -> Result<crate::identity::IdentityRecord, RootResolveError> {
        self.identity_record(presentation_uri)
            .map_err(|_| RootResolveError::UnknownPresentation(presentation_uri.clone()))
    }

    fn read_strongest_identity_token(&self, canonical_uri: &VfsUri) -> Result<IdentityToken, RootResolveError> {
        self.read_strongest_token(canonical_uri.as_str())
            .ok_or_else(|| RootResolveError::UnknownCanonical(canonical_uri.clone()))
    }

    fn read_fallback_identity_tokens(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<Vec<FallbackIdentityToken>, RootResolveError> {
        let tokens = self.fallback_tokens(canonical_uri.as_str());
        if tokens.is_empty() && self.read_strongest_token(canonical_uri.as_str()).is_none() {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        }
        Ok(tokens)
    }

    fn read_generation_token(&self, canonical_uri: &VfsUri) -> Result<GenerationToken, RootResolveError> {
        let identity = self.read_strongest_identity_token(canonical_uri)?;
        Ok(GenerationToken {
            kind: match identity.kind {
                crate::capabilities::StrongestIdentityTokenKind::FileIdGeneration => {
                    GenerationTokenKind::FileIdGeneration
                }
                crate::capabilities::StrongestIdentityTokenKind::DeviceInodeGeneration => {
                    GenerationTokenKind::DeviceInodeGeneration
                }
                crate::capabilities::StrongestIdentityTokenKind::WindowsObjectId => {
                    GenerationTokenKind::WindowsObjectId
                }
                crate::capabilities::StrongestIdentityTokenKind::ProviderObjectIdRevision => {
                    GenerationTokenKind::ProviderObjectIdRevision
                }
                crate::capabilities::StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs => {
                    GenerationTokenKind::ContentHash
                }
                crate::capabilities::StrongestIdentityTokenKind::ContentHashOnly => {
                    GenerationTokenKind::ContentHash
                }
            },
            value: identity.value,
        })
    }

    fn permission_snapshot(&self, canonical_uri: &VfsUri) -> Result<PermissionSnapshot, RootResolveError> {
        self.permission_snapshot(canonical_uri.as_str())
            .ok_or_else(|| RootResolveError::UnknownCanonical(canonical_uri.clone()))
    }

    fn read_bytes(&self, canonical_uri: &VfsUri) -> Result<Vec<u8>, RootIoError> {
        for (uri, obj) in self.canonical_objects() {
            if uri.as_str() == canonical_uri.as_str() {
                return Ok(obj.content.clone());
            }
        }
        Err(RootIoError::IoFailure {
            uri: canonical_uri.clone(),
            detail: "canonical object missing".to_owned(),
        })
    }

    fn write_bytes(&mut self, canonical_uri: &VfsUri, new_content: Vec<u8>) -> Result<(), RootIoError> {
        if self
            .apply_commit(canonical_uri.as_str(), new_content)
            .is_some()
        {
            return Ok(());
        }
        Err(RootIoError::IoFailure {
            uri: canonical_uri.clone(),
            detail: "canonical object missing".to_owned(),
        })
    }
}

