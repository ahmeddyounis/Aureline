//! Large-file path prototype.
//!
//! Validates the reduced-capability large-file mode frozen in
//! `docs/adr/0003-buffer-undo-large-file.md` without touching the
//! normal piece-tree buffer in [`aureline-buffer`]. The goal is to
//! prove that a file that trips any of the ADR's switch conditions
//! can be opened, inspected, searched, and partially saved through
//! a paged reader with bounded memory, while every full-file
//! capability the ADR denies in large-file mode is reviewable in
//! one place.
//!
//! Three pieces sit behind this crate's public surface:
//!
//! - [`classification`] — at-open classification that maps an
//!   on-disk file to either [`FileMode::Normal`] or
//!   [`FileMode::LargeFile`], records which switch trigger fired,
//!   and explains the decision in human-readable text. Triggers
//!   are evaluated in the ADR-frozen order (size threshold →
//!   resource pressure → classification → decode posture →
//!   operator override) and the first match wins.
//! - [`paged`] — a small paged reader that slices the file into
//!   fixed-size pages, holds at most a configured number of pages
//!   in an LRU cache, and surfaces metrics (bytes resident high
//!   water, pages read, pages evicted, total bytes read). The
//!   reader never loads the whole file into RAM; the
//!   limited-mode buffer asks the reader for the ranges it needs
//!   to render, search, or copy, and lets older pages drop out.
//! - [`capabilities`] — the frozen capability split between the
//!   normal-mode path (everything) and the limited-mode path
//!   (allowed / denied / downgraded rows). The split is a
//!   reviewable constant, not branching scattered across the
//!   buffer; later UX work consumes the same constant when it
//!   builds banners that explain a denied operation.
//!
//! [`LargeFileBuffer`] ties them together: it owns the open file
//! handle, the classification decision, the paged reader, the
//! capability table, and a small set of hook counters that mirror
//! the ADR's frozen large-file hook names. It deliberately does
//! NOT implement the editable transaction surface from
//! `aureline-buffer`: an editable transaction is the contract the
//! large-file mode reduces or denies, and the prototype keeps
//! that boundary visible by routing every edit attempt through
//! [`LargeFileBuffer::attempt_edit`] rather than offering a
//! piece-tree at all.
//!
//! Known holes (durable journal, structural-share large-file
//! overlay, real mmap, decode recovery, reflow, accessibility
//! tree) live in [`prototypes/large_file/README.md`](https://github.com/ahmeddyounis/Aureline/blob/main/prototypes/large_file/README.md)
//! and are tracked as carry-forward items, not silent capabilities
//! of this prototype.

#![doc(html_root_url = "https://docs.rs/aureline-largefile-proto/0.0.0")]

pub mod capabilities;
pub mod classification;
pub mod harness;
pub mod hooks;
pub mod paged;

mod buffer;

pub use buffer::{
    EditOutcome, EditRequest, LargeFileBuffer, LargeFileConfig, OpenError, SaveOutcome, SaveRequest,
};
pub use capabilities::{
    CapabilityRow, CapabilityState, LIMITED_MODE_CAPABILITIES, NORMAL_MODE_CAPABILITIES,
};
pub use classification::{
    BomKind, ClassificationDecision, ClassificationPolicy, FileMode, LargeFileTrigger, SniffSummary,
};
pub use hooks::HookCounters;
pub use paged::{PagedReader, ReaderMetrics};
