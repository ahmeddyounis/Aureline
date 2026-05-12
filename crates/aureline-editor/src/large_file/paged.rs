//! Bounded-memory paged reader for oversized files.
//!
//! Large-file mode deliberately avoids materialising the full file into memory.
//! Instead, consumers request ranges that are satisfied by a small LRU cache of
//! fixed-size pages.

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Default page size when callers do not supply one.
pub const DEFAULT_PAGE_SIZE: usize = 64 * 1024;

/// Default resident-page cap.
pub const DEFAULT_MAX_RESIDENT_PAGES: usize = 4;

/// Structural counters exposed by the reader. Counts only (no wall-clock
/// timing), so tests can assert deterministic behavior without host variance.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReaderMetrics {
    /// Number of distinct page reads served from disk.
    pub pages_read_from_disk: u64,
    /// Number of pages served from the in-memory cache.
    pub pages_served_from_cache: u64,
    /// Number of pages evicted from the cache.
    pub pages_evicted: u64,
    /// Total bytes pulled from disk across reads.
    pub bytes_read_from_disk: u64,
    /// High-water mark of bytes resident in the cache.
    pub bytes_resident_high_water: u64,
}

impl ReaderMetrics {
    /// Deterministic `(name, value)` pairs for structured reporting.
    pub fn entries(&self) -> [(&'static str, u64); 5] {
        [
            ("pages_read_from_disk", self.pages_read_from_disk),
            ("pages_served_from_cache", self.pages_served_from_cache),
            ("pages_evicted", self.pages_evicted),
            ("bytes_read_from_disk", self.bytes_read_from_disk),
            ("bytes_resident_high_water", self.bytes_resident_high_water),
        ]
    }
}

#[derive(Debug, Clone)]
struct CachedPage {
    index: u64,
    bytes: Vec<u8>,
}

/// Bounded-memory random-access reader over an on-disk file.
pub struct PagedReader {
    file: File,
    file_len: u64,
    page_size: usize,
    max_resident_pages: usize,
    lru: VecDeque<CachedPage>,
    bytes_resident: u64,
    metrics: ReaderMetrics,
}

impl PagedReader {
    /// Open `path` with defaults.
    pub fn open(path: &Path) -> std::io::Result<Self> {
        Self::open_with(path, DEFAULT_PAGE_SIZE, DEFAULT_MAX_RESIDENT_PAGES)
    }

    /// Open `path` with explicit tuning knobs.
    pub fn open_with(
        path: &Path,
        page_size: usize,
        max_resident_pages: usize,
    ) -> std::io::Result<Self> {
        assert!(page_size > 0, "page_size must be > 0");
        assert!(
            max_resident_pages >= 1,
            "max_resident_pages must be >= 1 so reads can always satisfy"
        );
        let file = File::open(path)?;
        let file_len = file.metadata()?.len();
        Ok(Self {
            file,
            file_len,
            page_size,
            max_resident_pages,
            lru: VecDeque::with_capacity(max_resident_pages),
            bytes_resident: 0,
            metrics: ReaderMetrics::default(),
        })
    }

    /// Returns the file length in bytes.
    pub fn file_len(&self) -> u64 {
        self.file_len
    }

    /// Returns the configured page size in bytes.
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Returns the max number of resident pages.
    pub fn max_resident_pages(&self) -> usize {
        self.max_resident_pages
    }

    /// Returns a snapshot of reader metrics.
    pub fn metrics(&self) -> &ReaderMetrics {
        &self.metrics
    }

    /// Returns the number of pages implied by `(file_len, page_size)`.
    pub fn page_count(&self) -> u64 {
        let ps = self.page_size as u64;
        self.file_len.div_ceil(ps)
    }

    /// Reads one page by index.
    pub fn read_page(&mut self, index: u64) -> std::io::Result<Vec<u8>> {
        if let Some(pos) = self.lru.iter().position(|p| p.index == index) {
            let page = self.lru.remove(pos).expect("present by index");
            let bytes = page.bytes.clone();
            self.lru.push_back(page);
            self.metrics.pages_served_from_cache += 1;
            return Ok(bytes);
        }

        let offset = index.saturating_mul(self.page_size as u64);
        if offset >= self.file_len {
            return Ok(Vec::new());
        }
        let remaining = (self.file_len - offset).min(self.page_size as u64) as usize;
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buf = vec![0u8; remaining];
        let mut read = 0usize;
        while read < remaining {
            match self.file.read(&mut buf[read..]) {
                Ok(0) => break,
                Ok(n) => read += n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        buf.truncate(read);

        let owned = buf.clone();
        self.evict_to_make_room_for(owned.len() as u64);
        let bytes = owned.len() as u64;
        self.lru.push_back(CachedPage {
            index,
            bytes: owned,
        });
        self.bytes_resident += bytes;
        if self.bytes_resident > self.metrics.bytes_resident_high_water {
            self.metrics.bytes_resident_high_water = self.bytes_resident;
        }
        self.metrics.pages_read_from_disk += 1;
        self.metrics.bytes_read_from_disk += bytes;
        Ok(buf)
    }

    /// Reads an arbitrary byte range.
    pub fn read_range(&mut self, range: std::ops::Range<u64>) -> std::io::Result<Vec<u8>> {
        if range.start >= range.end {
            return Ok(Vec::new());
        }
        let end = range.end.min(self.file_len);
        if range.start >= end {
            return Ok(Vec::new());
        }
        let ps = self.page_size as u64;
        let first_page = range.start / ps;
        let last_page = (end - 1) / ps;
        let mut out = Vec::with_capacity((end - range.start) as usize);
        for index in first_page..=last_page {
            let page = self.read_page(index)?;
            if page.is_empty() {
                break;
            }
            let page_offset = index * ps;
            let local_start = range.start.saturating_sub(page_offset) as usize;
            let local_end = (end - page_offset).min(page.len() as u64) as usize;
            if local_start < local_end {
                out.extend_from_slice(&page[local_start..local_end]);
            }
        }
        Ok(out)
    }

    /// Streaming search across the file. Returns the byte offset of the first
    /// match, or `None`.
    pub fn find_first(&mut self, needle: &[u8]) -> std::io::Result<Option<u64>> {
        if needle.is_empty() {
            return Ok(Some(0));
        }
        if needle.len() > self.page_size {
            return Ok(None);
        }
        let pages = self.page_count();
        let mut prev_tail: Vec<u8> = Vec::with_capacity(needle.len());
        for index in 0..pages {
            let page = self.read_page(index)?;
            if page.is_empty() {
                break;
            }
            let mut window = Vec::with_capacity(prev_tail.len() + page.len());
            window.extend_from_slice(&prev_tail);
            window.extend_from_slice(&page);
            if let Some(pos) = find_subsequence(&window, needle) {
                let page_offset = index * self.page_size as u64;
                let absolute = if pos < prev_tail.len() {
                    page_offset - prev_tail.len() as u64 + pos as u64
                } else {
                    page_offset + (pos - prev_tail.len()) as u64
                };
                return Ok(Some(absolute));
            }
            let tail_len = needle.len().saturating_sub(1);
            if tail_len == 0 {
                prev_tail.clear();
            } else if page.len() >= tail_len {
                prev_tail = page[page.len() - tail_len..].to_vec();
            } else {
                let mut combined = window.clone();
                if combined.len() > tail_len {
                    combined.drain(..combined.len() - tail_len);
                }
                prev_tail = combined;
            }
        }
        Ok(None)
    }

    fn evict_to_make_room_for(&mut self, needed_bytes: u64) {
        if self.lru.len() < self.max_resident_pages
            && self.bytes_resident + needed_bytes <= self.max_resident_bytes()
        {
            return;
        }
        while self.lru.len() >= self.max_resident_pages
            || self.bytes_resident + needed_bytes > self.max_resident_bytes()
        {
            let Some(page) = self.lru.pop_front() else {
                break;
            };
            self.bytes_resident = self.bytes_resident.saturating_sub(page.bytes.len() as u64);
            self.metrics.pages_evicted += 1;
        }
    }

    fn max_resident_bytes(&self) -> u64 {
        (self.page_size as u64).saturating_mul(self.max_resident_pages as u64)
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
