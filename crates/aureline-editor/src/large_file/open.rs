//! Large-file guarded document open.
//!
//! This module wires large-file classification to a concrete open behavior:
//!
//! - eligible files open into the normal piece-tree buffer (`aureline-buffer`)
//! - oversized or hostile files open into a constrained viewer (`LargeFileViewer`)
//! - callers can explicitly override into the normal buffer via `ForceNormal`
//!   while preserving an explicit record of the override decision

use std::path::PathBuf;

use aureline_buffer::{Buffer, Snapshot};
use aureline_vfs::{IdentityRecord, RootIoError, RootResolveError, VfsRoot, VfsUri};

use super::classification::{classify_file, ClassificationDecision, ClassificationPolicy, FileMode};
use super::viewer::{LargeFileViewer, LargeFileViewerConfig, LargeFileViewerError};

/// Controls how the open path treats large-file classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentOpenDisposition {
    /// Run classification and select the normal buffer or constrained viewer
    /// accordingly.
    Auto,
    /// Enter large-file mode regardless of other triggers.
    ForceLargeFile,
    /// Open into the normal buffer even if classification would activate
    /// large-file mode. The resulting normal document retains an explicit
    /// override record.
    ForceNormal,
}

impl Default for DocumentOpenDisposition {
    fn default() -> Self {
        Self::Auto
    }
}

/// Errors returned when opening a document through [`open_document`].
#[derive(Debug)]
pub enum DocumentOpenError {
    IdentityResolveFailed { uri: VfsUri, detail: String },
    UnsupportedCanonicalUri { uri: VfsUri },
    ClassificationFailed { path: PathBuf, detail: String },
    ViewerOpenFailed { path: PathBuf, detail: String },
    ReadFailed { uri: VfsUri, detail: String },
}

impl std::fmt::Display for DocumentOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentityResolveFailed { uri, detail } => {
                write!(f, "failed to resolve identity for {uri}: {detail}")
            }
            Self::UnsupportedCanonicalUri { uri } => write!(
                f,
                "unsupported canonical uri for document open: {uri} (expected file://)"
            ),
            Self::ClassificationFailed { path, detail } => {
                write!(f, "failed to classify file {path:?}: {detail}")
            }
            Self::ViewerOpenFailed { path, detail } => {
                write!(f, "failed to open constrained viewer for {path:?}: {detail}")
            }
            Self::ReadFailed { uri, detail } => write!(f, "failed to read bytes for {uri}: {detail}"),
        }
    }
}

impl std::error::Error for DocumentOpenError {}

/// Explicit record that a large-file document was opened into the normal buffer
/// via an override.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeFileOverrideInfo {
    /// The classification decision that would have activated large-file mode.
    pub decision: ClassificationDecision,
}

/// A document opened into the normal piece-tree buffer.
pub struct NormalDocument {
    pub identity: IdentityRecord,
    pub buffer: Buffer,
    pub snapshot: Snapshot,
    pub large_file_override: Option<LargeFileOverrideInfo>,
}

/// A document opened into the constrained large-file viewer.
pub struct LargeFileDocument {
    pub identity: IdentityRecord,
    pub viewer: LargeFileViewer,
}

/// User-facing large-file mode notice content suitable for projecting into UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeFileModeNotice {
    /// Short headline.
    pub title: String,
    /// The activation trigger id, when large-file mode was entered automatically.
    pub trigger: Option<String>,
    /// Human-readable reason for entering large-file mode.
    pub reason: String,
    /// Summary of reduced or unavailable capabilities.
    pub reduced_capabilities: Vec<String>,
    /// Label for the explicit escalation action.
    pub escalation_label: String,
    /// Explanation of what escalation implies.
    pub escalation_detail: String,
}

impl LargeFileDocument {
    /// Returns a notice describing why the file is in large-file mode and what
    /// capabilities are reduced.
    pub fn notice(&self) -> LargeFileModeNotice {
        let decision = self.viewer.decision();
        LargeFileModeNotice {
            title: "Large-file mode".to_owned(),
            trigger: decision.trigger.map(|t| t.as_str().to_owned()),
            reason: decision.reason.clone(),
            reduced_capabilities: vec![
                "Editing is read-only by default.".to_owned(),
                "Decorations and analysis are reduced or disabled.".to_owned(),
                "Semantic features are unavailable in this viewer.".to_owned(),
            ],
            escalation_label: "Open anyway".to_owned(),
            escalation_detail:
                "Opens the file using the normal buffer path; this may be slow or memory intensive."
                    .to_owned(),
        }
    }

    /// Opens the same document into the normal piece-tree buffer, recording
    /// that the caller explicitly chose to override large-file mode.
    pub fn open_anyway(&self, root: &dyn VfsRoot) -> Result<NormalDocument, DocumentOpenError> {
        let canonical_uri = &self.identity.canonical_filesystem_object.canonical_uri;
        let bytes = root
            .read_bytes(canonical_uri)
            .map_err(|err| read_err(canonical_uri, err))?;
        let mut buffer = Buffer::from_bytes(&bytes);
        let snapshot = buffer.snapshot();
        Ok(NormalDocument {
            identity: self.identity.clone(),
            buffer,
            snapshot,
            large_file_override: Some(LargeFileOverrideInfo {
                decision: self.viewer.decision().clone(),
            }),
        })
    }
}

/// Result of opening a document with large-file protection.
pub enum DocumentOpenOutcome {
    Normal(NormalDocument),
    LargeFile(LargeFileDocument),
}

/// Opens `presentation_uri` through the given VFS `root` and selects between the
/// normal piece-tree buffer and the constrained large-file viewer.
pub fn open_document(
    root: &dyn VfsRoot,
    presentation_uri: &VfsUri,
    policy: &ClassificationPolicy,
    viewer_config: LargeFileViewerConfig,
    disposition: DocumentOpenDisposition,
) -> Result<DocumentOpenOutcome, DocumentOpenError> {
    let identity = root
        .identity_record(presentation_uri)
        .map_err(|err| identity_err(presentation_uri, err))?;
    let canonical_uri = identity.canonical_filesystem_object.canonical_uri.clone();
    let Some(path) = canonical_uri.file_path() else {
        return Err(DocumentOpenError::UnsupportedCanonicalUri { uri: canonical_uri });
    };

    let mut policy = policy.clone();
    if disposition == DocumentOpenDisposition::ForceLargeFile {
        policy.operator_override = true;
    }
    let decision = classify_file(&path, &policy).map_err(|err| DocumentOpenError::ClassificationFailed {
        path: path.clone(),
        detail: err.to_string(),
    })?;

    match (decision.mode, disposition) {
        (FileMode::LargeFile, DocumentOpenDisposition::ForceNormal) => {
            open_normal_with_override(root, identity, &canonical_uri, decision)
        }
        (FileMode::LargeFile, _) => open_large_file(identity, decision, viewer_config),
        (FileMode::Normal, _) => open_normal(root, identity, &canonical_uri, None),
    }
}

fn open_large_file(
    identity: IdentityRecord,
    decision: ClassificationDecision,
    viewer_config: LargeFileViewerConfig,
) -> Result<DocumentOpenOutcome, DocumentOpenError> {
    let path = decision.path.clone();
    let viewer = LargeFileViewer::open(decision, viewer_config).map_err(|err| {
        DocumentOpenError::ViewerOpenFailed {
            path,
            detail: err.to_string(),
        }
    })?;
    Ok(DocumentOpenOutcome::LargeFile(LargeFileDocument { identity, viewer }))
}

fn open_normal(
    root: &dyn VfsRoot,
    identity: IdentityRecord,
    canonical_uri: &VfsUri,
    override_info: Option<LargeFileOverrideInfo>,
) -> Result<DocumentOpenOutcome, DocumentOpenError> {
    let bytes = root
        .read_bytes(canonical_uri)
        .map_err(|err| read_err(canonical_uri, err))?;
    let mut buffer = Buffer::from_bytes(&bytes);
    let snapshot = buffer.snapshot();
    Ok(DocumentOpenOutcome::Normal(NormalDocument {
        identity,
        buffer,
        snapshot,
        large_file_override: override_info,
    }))
}

fn open_normal_with_override(
    root: &dyn VfsRoot,
    identity: IdentityRecord,
    canonical_uri: &VfsUri,
    decision: ClassificationDecision,
) -> Result<DocumentOpenOutcome, DocumentOpenError> {
    open_normal(
        root,
        identity,
        canonical_uri,
        Some(LargeFileOverrideInfo { decision }),
    )
}

fn identity_err(uri: &VfsUri, err: RootResolveError) -> DocumentOpenError {
    DocumentOpenError::IdentityResolveFailed {
        uri: uri.clone(),
        detail: err.to_string(),
    }
}

fn read_err(uri: &VfsUri, err: RootIoError) -> DocumentOpenError {
    DocumentOpenError::ReadFailed {
        uri: uri.clone(),
        detail: err.to_string(),
    }
}

// Keep the mapping deterministic and future-proof by explicitly touching the
// error type. The current surface stores stringified details only.
#[allow(dead_code)]
fn _viewer_err_detail(err: &LargeFileViewerError) -> &str {
    match err {
        LargeFileViewerError::Io(_) => "io",
    }
}
