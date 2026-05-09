//! URI model for VFS roots and documents.
//!
//! Aureline uses URIs as the canonical cross-surface address for documents
//! (local, generated, virtual, remote). This module provides a minimal,
//! dependency-free URI wrapper plus helpers for the URI shapes used in the VFS
//! identity model (ADR 0006).

use std::path::{Path, PathBuf};

/// Error returned when parsing or constructing a [`VfsUri`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriError {
    Empty,
    MissingSchemeSeparator,
    InvalidScheme(String),
}

impl std::fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "uri is empty"),
            Self::MissingSchemeSeparator => write!(f, "uri is missing ':' scheme separator"),
            Self::InvalidScheme(scheme) => write!(f, "invalid uri scheme: {scheme}"),
        }
    }
}

impl std::error::Error for UriError {}

/// Parsed hierarchical URI components for `scheme://authority/path` shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierarchicalUriRef<'a> {
    pub scheme: &'a str,
    pub authority: &'a str,
    pub path: &'a str,
}

impl<'a> HierarchicalUriRef<'a> {
    /// Iterates over non-empty path segments, without leading `/`.
    pub fn path_segments(self) -> impl Iterator<Item = &'a str> {
        self.path
            .trim_start_matches('/')
            .split('/')
            .filter(|segment| !segment.is_empty())
    }
}

/// A validated URI string used by the VFS identity model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VfsUri {
    raw: String,
    scheme_end: usize,
}

impl VfsUri {
    /// Parses and validates a URI.
    pub fn parse(uri: impl Into<String>) -> Result<Self, UriError> {
        let raw = uri.into();
        if raw.is_empty() {
            return Err(UriError::Empty);
        }
        let (scheme, _) = raw
            .split_once(':')
            .ok_or(UriError::MissingSchemeSeparator)?;
        if !is_valid_scheme(scheme) {
            return Err(UriError::InvalidScheme(scheme.to_owned()));
        }
        Ok(Self {
            scheme_end: scheme.len(),
            raw,
        })
    }

    /// Returns the URI scheme (the substring before the first `:`).
    pub fn scheme(&self) -> &str {
        &self.raw[..self.scheme_end]
    }

    /// Returns the full URI string.
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Converts this URI into its owned string.
    pub fn into_string(self) -> String {
        self.raw
    }

    /// Splits `scheme://authority/path` URIs into components.
    ///
    /// Returns `None` if the URI does not contain a hierarchical part.
    pub fn split_hierarchical(&self) -> Option<HierarchicalUriRef<'_>> {
        let raw = self.as_str();
        let scheme = self.scheme();
        let rest = raw.strip_prefix(&format!("{scheme}://"))?;
        let (authority, path) = match rest.find('/') {
            Some(slash) => (&rest[..slash], &rest[slash..]),
            None => (rest, ""),
        };
        let path = if path.is_empty() { "/" } else { path };
        Some(HierarchicalUriRef {
            scheme,
            authority,
            path,
        })
    }

    /// Builds a `file://` URI for a canonicalized filesystem path.
    pub fn file_url_for_path(path: &Path) -> Option<Self> {
        let canonical = path.canonicalize().ok()?;
        let raw = canonical.to_string_lossy();
        let url = if cfg!(windows) {
            format!("file:///{}", raw.replace('\\', "/"))
        } else {
            format!("file://{raw}")
        };
        Self::parse(url).ok()
    }

    /// Builds a `file://` URI for `path` without canonicalizing or touching the
    /// filesystem.
    ///
    /// This is intended for watcher events where the path may no longer exist
    /// (for example, a `delete` event) but the last observed path string still
    /// needs to be reported.
    pub fn file_url_for_path_lossy(path: &Path) -> Option<Self> {
        if !path.is_absolute() {
            return None;
        }
        let raw = path.to_string_lossy();
        let url = if cfg!(windows) {
            format!("file:///{}", raw.replace('\\', "/"))
        } else {
            format!("file://{raw}")
        };
        Self::parse(url).ok()
    }

    /// Converts a `file://` URI into an OS path, when possible.
    pub fn file_path(&self) -> Option<PathBuf> {
        if self.scheme() != "file" {
            return None;
        }
        let raw = self.as_str();
        let path_part = raw.strip_prefix("file://")?;
        if cfg!(windows) {
            let path_part = path_part.strip_prefix('/')?;
            Some(PathBuf::from(path_part.replace('/', "\\")))
        } else {
            Some(PathBuf::from(path_part))
        }
    }

    /// Builds an `aureline-ws://` logical workspace URI.
    pub fn workspace_logical_uri(
        workspace_id: &str,
        root_id: &str,
        logical_path: &str,
    ) -> Result<Self, UriError> {
        let logical_path = logical_path.trim_start_matches('/');
        Self::parse(format!(
            "aureline-ws://{workspace_id}/{root_id}/{logical_path}"
        ))
    }

    /// Builds a `virtual://` URI identifying a virtual document under a root.
    pub fn virtual_document_uri(
        workspace_id: &str,
        root_id: &str,
        document_id: &str,
    ) -> Result<Self, UriError> {
        let document_id = document_id.trim_start_matches('/');
        Self::parse(format!("virtual://{workspace_id}/{root_id}/{document_id}"))
    }

    /// Builds a `generated://` URI identifying a generated document under a root.
    pub fn generated_document_uri(
        workspace_id: &str,
        root_id: &str,
        document_id: &str,
    ) -> Result<Self, UriError> {
        let document_id = document_id.trim_start_matches('/');
        Self::parse(format!(
            "generated://{workspace_id}/{root_id}/{document_id}"
        ))
    }
}

impl std::fmt::Display for VfsUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for VfsUri {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

fn is_valid_scheme(scheme: &str) -> bool {
    let mut chars = scheme.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rejects_empty_and_invalid_scheme() {
        assert_eq!(VfsUri::parse("").unwrap_err(), UriError::Empty);
        assert!(matches!(
            VfsUri::parse("1bad://x").unwrap_err(),
            UriError::InvalidScheme(_)
        ));
    }

    #[test]
    fn split_hierarchical_extracts_authority_and_segments() {
        let uri = VfsUri::parse("aureline-ws://ws-test/root-1/src/main.rs").expect("valid uri");
        let parts = uri.split_hierarchical().expect("expected hierarchical uri");
        assert_eq!(parts.scheme, "aureline-ws");
        assert_eq!(parts.authority, "ws-test");
        let segments: Vec<&str> = parts.path_segments().collect();
        assert_eq!(segments, vec!["root-1", "src", "main.rs"]);
    }
}
