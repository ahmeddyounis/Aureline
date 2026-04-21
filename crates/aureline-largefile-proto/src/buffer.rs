//! Large-file buffer surface.
//!
//! Ties the classification decision, the paged reader, the
//! capability table, and the hook counters together. By design
//! this surface does NOT implement the editable transaction API
//! from `aureline-buffer`; the whole point of large-file mode is
//! that most write paths are reduced or denied. Edit attempts go
//! through [`LargeFileBuffer::attempt_edit`] which returns an
//! [`EditOutcome`] tagged with the row from the limited-mode
//! capability table that gated the request.

use std::path::{Path, PathBuf};

use crate::capabilities::{
    lookup, CapabilityRow, CapabilityState, LIMITED_MODE_CAPABILITIES,
    NORMAL_MODE_CAPABILITIES,
};
use crate::classification::{
    classify_file, ClassificationDecision, ClassificationPolicy, ClassifyError, FileMode,
};
use crate::hooks::HookCounters;
use crate::paged::{PagedReader, ReaderMetrics, DEFAULT_MAX_RESIDENT_PAGES, DEFAULT_PAGE_SIZE};

/// Knobs the caller passes when opening a file through this
/// prototype. Defaults match the paged-reader defaults; tests
/// drive the cap down so eviction is observable.
#[derive(Debug, Clone)]
pub struct LargeFileConfig {
    pub policy: ClassificationPolicy,
    pub page_size: usize,
    pub max_resident_pages: usize,
}

impl Default for LargeFileConfig {
    fn default() -> Self {
        Self {
            policy: ClassificationPolicy::default(),
            page_size: DEFAULT_PAGE_SIZE,
            max_resident_pages: DEFAULT_MAX_RESIDENT_PAGES,
        }
    }
}

/// Errors the open path surfaces back to callers.
#[derive(Debug)]
pub enum OpenError {
    Classify(ClassifyError),
    Io(std::io::Error),
}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Classify(e) => write!(f, "classification: {e}"),
            Self::Io(e) => write!(f, "io: {e}"),
        }
    }
}

impl std::error::Error for OpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Classify(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}

impl From<ClassifyError> for OpenError {
    fn from(e: ClassifyError) -> Self {
        Self::Classify(e)
    }
}

impl From<std::io::Error> for OpenError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Edit-attempt vocabulary keyed to the capability table. The
/// names mirror capability ids so a row in the table is the
/// single source of truth for whether the request is accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditRequest {
    ViewportCursorInsert,
    WholeFileMultiCursor,
    ViewportMultiCursor,
    FullFileFormatOnSave,
    RangeFormatOnSave,
    FullFileAiApply,
    RangeAiApply,
    RichRefactorMultiFile,
    RichRefactorSingleFile,
    SaveParticipantWholeFile,
    SaveParticipantRangeOnly,
    Indexing,
    BackgroundAnalysis,
    DiagnosticsWholeFile,
    DiagnosticsViewport,
    SearchWholeFile,
    SearchViewport,
    Copy,
}

impl EditRequest {
    /// Capability-id this request consults.
    pub fn capability_id(self) -> &'static str {
        match self {
            Self::ViewportCursorInsert => "view",
            Self::WholeFileMultiCursor => "multi_cursor_whole_file",
            Self::ViewportMultiCursor => "multi_cursor_viewport",
            Self::FullFileFormatOnSave => "full_file_format_on_save",
            Self::RangeFormatOnSave => "range_format_on_save",
            Self::FullFileAiApply => "ai_apply_whole_file",
            Self::RangeAiApply => "ai_apply_range",
            Self::RichRefactorMultiFile => "rich_refactor_multi_file",
            Self::RichRefactorSingleFile => "rich_refactor_single_file",
            Self::SaveParticipantWholeFile => "save_participant_whole_file",
            Self::SaveParticipantRangeOnly => "save_participant_range_only",
            Self::Indexing => "indexing",
            Self::BackgroundAnalysis => "background_analysis",
            Self::DiagnosticsWholeFile => "diagnostics_whole_file",
            Self::DiagnosticsViewport => "diagnostics_viewport",
            Self::SearchWholeFile => "search_whole_file",
            Self::SearchViewport => "search_viewport",
            Self::Copy => "copy",
        }
    }
}

/// What the buffer says back when a lane attempts an edit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EditOutcome {
    Accepted {
        capability_id: &'static str,
    },
    Denied {
        capability_id: &'static str,
        reason: &'static str,
    },
    Downgraded {
        capability_id: &'static str,
        reason: &'static str,
    },
}

impl EditOutcome {
    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted { .. })
    }
    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Denied { .. })
    }
    pub fn is_downgraded(&self) -> bool {
        matches!(self, Self::Downgraded { .. })
    }
    pub fn capability_id(&self) -> &'static str {
        match self {
            Self::Accepted { capability_id }
            | Self::Denied { capability_id, .. }
            | Self::Downgraded { capability_id, .. } => capability_id,
        }
    }
}

/// What kind of save the lane is trying to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveRequest {
    /// Save with no participants; the buffer writes the edited
    /// range only.
    EditedRangeOnly,
    /// Save with whole-file participants (full-file formatter,
    /// import organiser, AI apply over the whole file).
    WithWholeFileParticipants,
    /// Save with range-only participants only.
    WithRangeOnlyParticipants,
}

/// Save-attempt outcome. Save participants that touch the whole
/// file are denied in limited mode; the lane MUST pre-check via
/// `attempt_save` before kicking off the save pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SaveOutcome {
    Accepted { reason: &'static str },
    Denied { reason: &'static str },
}

/// Public buffer surface for the prototype.
pub struct LargeFileBuffer {
    decision: ClassificationDecision,
    reader: PagedReader,
    counters: HookCounters,
    capabilities_normal: &'static [CapabilityRow],
    capabilities_limited: &'static [CapabilityRow],
}

impl LargeFileBuffer {
    /// Open a file and run classification + reader bring-up. The
    /// returned buffer always uses the paged reader; a normal-mode
    /// classification still holds a paged-reader handle so the
    /// resident-bytes contract holds even when the lane chose to
    /// keep using this prototype for a small file. (Production
    /// hands normal-mode files off to `aureline-buffer` and never
    /// sees this surface.)
    pub fn open(path: &Path, config: &LargeFileConfig) -> Result<Self, OpenError> {
        let mut counters = HookCounters::default();
        let decision = classify_file(path, &config.policy)?;
        counters.classification_recorded += 1;
        let reader = PagedReader::open_with(path, config.page_size, config.max_resident_pages)?;
        counters.buffer_open += 1;
        if decision.mode == FileMode::LargeFile {
            counters.large_file_mode_enter += 1;
        }
        Ok(Self {
            decision,
            reader,
            counters,
            capabilities_normal: NORMAL_MODE_CAPABILITIES,
            capabilities_limited: LIMITED_MODE_CAPABILITIES,
        })
    }

    pub fn path(&self) -> &Path {
        &self.decision.path
    }

    pub fn len(&self) -> u64 {
        self.decision.bytes_on_disk
    }

    pub fn is_empty(&self) -> bool {
        self.decision.bytes_on_disk == 0
    }

    pub fn mode(&self) -> FileMode {
        self.decision.mode
    }

    pub fn decision(&self) -> &ClassificationDecision {
        &self.decision
    }

    pub fn hook_counters(&self) -> &HookCounters {
        &self.counters
    }

    pub fn reader_metrics(&self) -> &ReaderMetrics {
        self.reader.metrics()
    }

    pub fn page_size(&self) -> usize {
        self.reader.page_size()
    }

    pub fn max_resident_pages(&self) -> usize {
        self.reader.max_resident_pages()
    }

    pub fn capabilities(&self) -> &'static [CapabilityRow] {
        match self.decision.mode {
            FileMode::Normal => self.capabilities_normal,
            FileMode::LargeFile => self.capabilities_limited,
        }
    }

    pub fn capability_state(&self, id: &str) -> Option<CapabilityState> {
        lookup(self.capabilities(), id).map(|r| r.state)
    }

    /// Read a byte range through the paged reader. Counts a
    /// `paged_read` hook fire per page touched.
    pub fn read_range(&mut self, range: std::ops::Range<u64>) -> std::io::Result<Vec<u8>> {
        let before = self.reader.metrics().pages_read_from_disk
            + self.reader.metrics().pages_served_from_cache;
        let bytes = self.reader.read_range(range)?;
        let after = self.reader.metrics().pages_read_from_disk
            + self.reader.metrics().pages_served_from_cache;
        self.counters.paged_read += after - before;
        Ok(bytes)
    }

    /// Streaming search through the file. Returns the byte
    /// offset of the first match, or `None`.
    pub fn find_first(&mut self, needle: &[u8]) -> std::io::Result<Option<u64>> {
        let before = self.reader.metrics().pages_read_from_disk
            + self.reader.metrics().pages_served_from_cache;
        let result = self.reader.find_first(needle)?;
        let after = self.reader.metrics().pages_read_from_disk
            + self.reader.metrics().pages_served_from_cache;
        self.counters.paged_read += after - before;
        Ok(result)
    }

    /// Consult the capability table for an edit request and
    /// surface the outcome. The prototype does not commit any
    /// edit even for accepted requests; that side of the contract
    /// belongs to the production write overlay. The point of the
    /// surface here is that lanes can pre-check an action against
    /// the same table the UX banner reads.
    pub fn attempt_edit(&self, request: EditRequest) -> EditOutcome {
        let id = request.capability_id();
        let row = lookup(self.capabilities(), id).expect("capability id missing from table");
        match row.state {
            CapabilityState::Allowed => EditOutcome::Accepted { capability_id: id },
            CapabilityState::Denied => EditOutcome::Denied {
                capability_id: id,
                reason: row.note.unwrap_or("denied in this mode"),
            },
            CapabilityState::Downgraded => EditOutcome::Downgraded {
                capability_id: id,
                reason: row.note.unwrap_or("downgraded in this mode"),
            },
        }
    }

    /// Pre-check a save request against the capability table.
    /// In limited mode whole-file save participants are denied;
    /// the lane MUST drop them or fall back to a range-only save.
    pub fn attempt_save(&self, request: SaveRequest) -> SaveOutcome {
        match (self.decision.mode, request) {
            (FileMode::LargeFile, SaveRequest::WithWholeFileParticipants) => SaveOutcome::Denied {
                reason: "save participants that rewrite the whole file are denied in large-file mode",
            },
            (_, SaveRequest::EditedRangeOnly) => SaveOutcome::Accepted {
                reason: "edited-range-only save is allowed in any mode",
            },
            (_, SaveRequest::WithRangeOnlyParticipants) => SaveOutcome::Accepted {
                reason: "range-only save participants are allowed in any mode",
            },
            (FileMode::Normal, SaveRequest::WithWholeFileParticipants) => SaveOutcome::Accepted {
                reason: "whole-file save participants are allowed in normal mode",
            },
        }
    }

    /// Snapshot of the buffer's structural state, useful for the
    /// harness output. Counts only.
    pub fn structural_snapshot(&self) -> StructuralSnapshot {
        StructuralSnapshot {
            path: self.decision.path.clone(),
            len: self.decision.bytes_on_disk,
            mode: self.decision.mode,
            trigger: self
                .decision
                .trigger
                .map(|t| t.as_str())
                .unwrap_or("none"),
            page_size: self.reader.page_size() as u64,
            max_resident_pages: self.reader.max_resident_pages() as u64,
            page_count: self.reader.page_count(),
            counters: self.counters.clone(),
            reader_metrics: self.reader.metrics().clone(),
        }
    }
}

/// Counts-only structural snapshot used by the harness. No bytes,
/// no file content, so committed metric seeds stay byte-stable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralSnapshot {
    pub path: PathBuf,
    pub len: u64,
    pub mode: FileMode,
    pub trigger: &'static str,
    pub page_size: u64,
    pub max_resident_pages: u64,
    pub page_count: u64,
    pub counters: HookCounters,
    pub reader_metrics: ReaderMetrics,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tempdir() -> PathBuf {
        let mut p = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        p.push(format!(
            "aureline-largefile-proto-buffer-{nanos}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn cleanup(p: PathBuf) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn small_text_file(path: &Path) {
        std::fs::write(path, b"clean small text\nline two\n").unwrap();
    }

    #[test]
    fn open_classifies_and_fires_buffer_open() {
        let dir = tempdir();
        let path = dir.join("clean.txt");
        small_text_file(&path);
        let cfg = LargeFileConfig::default();
        let buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        assert_eq!(buf.mode(), FileMode::Normal);
        assert_eq!(buf.hook_counters().buffer_open, 1);
        assert_eq!(buf.hook_counters().classification_recorded, 1);
        assert_eq!(buf.hook_counters().large_file_mode_enter, 0);
        cleanup(dir);
    }

    #[test]
    fn open_at_size_threshold_enters_large_file_mode() {
        let dir = tempdir();
        let path = dir.join("biggish.txt");
        let bytes: Vec<u8> = std::iter::repeat(b'a').take(2048).collect();
        std::fs::write(&path, &bytes).unwrap();
        let mut cfg = LargeFileConfig::default();
        cfg.policy.large_file_size_threshold = 1024;
        cfg.page_size = 128;
        cfg.max_resident_pages = 2;
        let buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        assert_eq!(buf.mode(), FileMode::LargeFile);
        assert_eq!(buf.hook_counters().large_file_mode_enter, 1);
        cleanup(dir);
    }

    #[test]
    fn limited_mode_denies_whole_file_multi_cursor_and_accepts_viewport_search() {
        let dir = tempdir();
        let path = dir.join("biggish.bin");
        // Trip classification: NUL byte in the sniff.
        std::fs::write(&path, b"hello\0world\0and\nmore content\n").unwrap();
        let cfg = LargeFileConfig::default();
        let buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        assert_eq!(buf.mode(), FileMode::LargeFile);
        assert!(buf.attempt_edit(EditRequest::WholeFileMultiCursor).is_denied());
        assert!(buf.attempt_edit(EditRequest::ViewportMultiCursor).is_accepted());
        assert!(buf.attempt_edit(EditRequest::FullFileFormatOnSave).is_denied());
        assert!(buf.attempt_edit(EditRequest::RangeFormatOnSave).is_accepted());
        assert!(buf.attempt_edit(EditRequest::SearchViewport).is_accepted());
        let so = buf.attempt_edit(EditRequest::SearchWholeFile);
        assert!(so.is_downgraded());
        cleanup(dir);
    }

    #[test]
    fn read_range_in_limited_mode_does_not_load_whole_file_into_ram() {
        let dir = tempdir();
        let path = dir.join("biggish.bin");
        let len = 16 * 1024usize;
        let bytes: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
        std::fs::write(&path, &bytes).unwrap();
        let mut cfg = LargeFileConfig::default();
        cfg.policy.large_file_size_threshold = 1024; // force limited
        cfg.page_size = 256;
        cfg.max_resident_pages = 2;
        let mut buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        assert_eq!(buf.mode(), FileMode::LargeFile);
        // Walk the whole range; the prototype must do this through
        // the paged reader without exceeding the resident cap.
        let observed = buf.read_range(0..len as u64).unwrap();
        assert_eq!(observed.len(), len);
        let resident_cap = (cfg.page_size * cfg.max_resident_pages) as u64;
        assert!(
            buf.reader_metrics().bytes_resident_high_water <= resident_cap,
            "resident high water {} exceeds cap {resident_cap}",
            buf.reader_metrics().bytes_resident_high_water
        );
        // paged_read counter recorded at least one fire per page.
        let pages = (len / cfg.page_size) as u64;
        assert!(buf.hook_counters().paged_read >= pages);
        cleanup(dir);
    }

    #[test]
    fn save_with_whole_file_participants_is_denied_in_limited_mode() {
        let dir = tempdir();
        let path = dir.join("biggish.bin");
        std::fs::write(&path, b"hello\0world\0and\nmore content\n").unwrap();
        let cfg = LargeFileConfig::default();
        let buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        let denied = buf.attempt_save(SaveRequest::WithWholeFileParticipants);
        assert!(matches!(denied, SaveOutcome::Denied { .. }));
        let accepted = buf.attempt_save(SaveRequest::EditedRangeOnly);
        assert!(matches!(accepted, SaveOutcome::Accepted { .. }));
        let accepted_range = buf.attempt_save(SaveRequest::WithRangeOnlyParticipants);
        assert!(matches!(accepted_range, SaveOutcome::Accepted { .. }));
        cleanup(dir);
    }

    #[test]
    fn save_with_whole_file_participants_is_allowed_in_normal_mode() {
        let dir = tempdir();
        let path = dir.join("clean.txt");
        small_text_file(&path);
        let cfg = LargeFileConfig::default();
        let buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        let accepted = buf.attempt_save(SaveRequest::WithWholeFileParticipants);
        assert!(matches!(accepted, SaveOutcome::Accepted { .. }));
        cleanup(dir);
    }

    #[test]
    fn structural_snapshot_is_counts_only() {
        let dir = tempdir();
        let path = dir.join("clean.txt");
        small_text_file(&path);
        let cfg = LargeFileConfig::default();
        let buf = LargeFileBuffer::open(&path, &cfg).unwrap();
        let snap = buf.structural_snapshot();
        assert_eq!(snap.mode, FileMode::Normal);
        assert_eq!(snap.trigger, "none");
        assert_eq!(snap.counters.buffer_open, 1);
        assert_eq!(snap.counters.large_file_mode_enter, 0);
        cleanup(dir);
    }
}
