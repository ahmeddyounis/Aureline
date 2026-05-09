//! Large-file detection and constrained read viewer.
//!
//! This module provides a single at-open classification step that decides
//! whether a document should enter large-file mode, along with a bounded-memory
//! paged reader that supports read-focused operations without loading an entire
//! oversized file into the normal piece-tree buffer.
//!
//! The large-file boundary is intended to stay explicit: consumers can surface
//! the activation trigger, a human-readable reason, and an explicit `Open
//! anyway` override route that opts into the heavier normal buffer path.

mod classification;
mod open;
mod paged;
mod viewer;

pub use classification::{
    classify_file, BomKind, ClassificationDecision, ClassificationPolicy, ClassifyError, FileMode,
    LargeFileTrigger, SniffHeuristics, SniffSummary,
};
pub use open::{
    open_document, DocumentOpenDisposition, DocumentOpenError, DocumentOpenOutcome,
    LargeFileDocument, LargeFileModeNotice, LargeFileOverrideInfo, NormalDocument,
};
pub use paged::{PagedReader, ReaderMetrics, DEFAULT_MAX_RESIDENT_PAGES, DEFAULT_PAGE_SIZE};
pub use viewer::{LargeFileViewer, LargeFileViewerConfig, LargeFileViewerError};
