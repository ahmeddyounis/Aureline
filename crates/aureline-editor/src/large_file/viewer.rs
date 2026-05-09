//! Constrained large-file viewer state.
//!
//! The viewer owns the bounded-memory paged reader and exposes read-focused
//! primitives (range reads and streaming search). It does not attempt to emulate
//! the full editor buffer API; consumers should treat it as a distinct surface.

use std::ops::Range;

use super::classification::ClassificationDecision;
use super::paged::{PagedReader, ReaderMetrics, DEFAULT_MAX_RESIDENT_PAGES, DEFAULT_PAGE_SIZE};

/// Configuration knobs for [`LargeFileViewer`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeFileViewerConfig {
    /// Page size used by the paged reader.
    pub page_size: usize,
    /// Maximum number of resident pages kept in memory.
    pub max_resident_pages: usize,
}

impl Default for LargeFileViewerConfig {
    fn default() -> Self {
        Self {
            page_size: DEFAULT_PAGE_SIZE,
            max_resident_pages: DEFAULT_MAX_RESIDENT_PAGES,
        }
    }
}

/// Errors returned by large-file viewer operations.
#[derive(Debug)]
pub enum LargeFileViewerError {
    Io(std::io::Error),
}

impl std::fmt::Display for LargeFileViewerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for LargeFileViewerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for LargeFileViewerError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// Read-focused viewer for a large-file mode document.
pub struct LargeFileViewer {
    decision: ClassificationDecision,
    reader: PagedReader,
}

impl LargeFileViewer {
    /// Opens a viewer for `decision.path` using `config`.
    ///
    /// `decision` is carried so consumers can surface the exact trigger and
    /// reason alongside the constrained viewer.
    pub fn open(
        decision: ClassificationDecision,
        config: LargeFileViewerConfig,
    ) -> Result<Self, LargeFileViewerError> {
        let reader =
            PagedReader::open_with(&decision.path, config.page_size, config.max_resident_pages)?;
        Ok(Self { decision, reader })
    }

    /// Returns the at-open classification decision.
    pub fn decision(&self) -> &ClassificationDecision {
        &self.decision
    }

    /// Returns the file length in bytes.
    pub fn file_len(&self) -> u64 {
        self.reader.file_len()
    }

    /// Returns the reader metrics.
    pub fn metrics(&self) -> &ReaderMetrics {
        self.reader.metrics()
    }

    /// Reads an arbitrary byte range.
    pub fn read_range(&mut self, range: Range<u64>) -> Result<Vec<u8>, LargeFileViewerError> {
        Ok(self.reader.read_range(range)?)
    }

    /// Finds the first occurrence of `needle` as UTF-8 bytes.
    pub fn find_first(&mut self, needle: &str) -> Result<Option<u64>, LargeFileViewerError> {
        Ok(self.reader.find_first(needle.as_bytes())?)
    }

    /// Reads the first `max_bytes` bytes and returns them as UTF-8 when valid.
    pub fn read_prefix_utf8(
        &mut self,
        max_bytes: u64,
    ) -> Result<Option<String>, LargeFileViewerError> {
        let end = max_bytes.min(self.reader.file_len());
        let bytes = self.reader.read_range(0..end)?;
        match String::from_utf8(bytes) {
            Ok(s) => Ok(Some(s)),
            Err(_) => Ok(None),
        }
    }
}

