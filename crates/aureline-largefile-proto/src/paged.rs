//! Bounded-memory paged reader.
//!
//! The ADR's large-file backing store is "an mmap-backed paged
//! reader with a paged-rope-class write overlay". The prototype
//! implements the read half with stdlib `File` + `Seek` + `Read`
//! so the workspace's `unsafe_code = "deny"` lint stays honest;
//! production swaps the reader for an mmap-backed implementation
//! behind the same `read_range` surface without changing
//! consumers.
//!
//! Two invariants are tested explicitly:
//!
//! - **Bounded resident set.** At any moment the reader holds at
//!   most `max_resident_pages` page bodies. An LRU evicts the
//!   least-recently-touched page when the cache is full.
//! - **No whole-file load.** The reader never returns more than
//!   one page worth of bytes per `read_page`; `read_range` walks
//!   one page at a time and the caller chooses how much to copy
//!   into a transient buffer for rendering, search, or copy.
//!
//! Counters describe the work the reader did so the harness can
//! report deterministic structural metrics: pages read, pages
//! evicted, total bytes read from disk, and the high-water mark
//! of resident bytes.

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Default page size when callers do not supply one. 64 KiB
/// matches a common OS page-cache stride and keeps the prototype
/// out of pathological territory; production tunes per backing
/// store and per workload.
pub const DEFAULT_PAGE_SIZE: usize = 64 * 1024;

/// Default LRU resident-page cap. At the default page size this
/// caps the reader at 256 KiB resident regardless of file size,
/// so the bounded-memory test can exercise eviction on inputs an
/// order of magnitude bigger.
pub const DEFAULT_MAX_RESIDENT_PAGES: usize = 4;

/// Counters the reader exposes back to the harness. Counts only;
/// no wall-clock data so committed metric seeds stay byte-stable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReaderMetrics {
    /// Number of distinct page reads served from disk (cache misses).
    pub pages_read_from_disk: u64,
    /// Number of pages served from the LRU cache (cache hits).
    pub pages_served_from_cache: u64,
    /// Number of pages evicted from the cache.
    pub pages_evicted: u64,
    /// Total bytes pulled off disk across all page reads.
    pub bytes_read_from_disk: u64,
    /// High-water mark of bytes currently resident in the LRU.
    pub bytes_resident_high_water: u64,
}

impl ReaderMetrics {
    /// Deterministic `(name, value)` pairs for harness output.
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

/// Bounded-memory random-access reader over a file.
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
    /// Open `path` with the default page size and resident cap.
    pub fn open(path: &Path) -> std::io::Result<Self> {
        Self::open_with(path, DEFAULT_PAGE_SIZE, DEFAULT_MAX_RESIDENT_PAGES)
    }

    /// Open `path` with explicit knobs. `page_size` MUST be > 0;
    /// `max_resident_pages` MUST be >= 1 so a `read_page` call
    /// can always satisfy itself.
    pub fn open_with(
        path: &Path,
        page_size: usize,
        max_resident_pages: usize,
    ) -> std::io::Result<Self> {
        assert!(page_size > 0, "page_size must be > 0");
        assert!(
            max_resident_pages >= 1,
            "max_resident_pages must be >= 1 so any read_page can satisfy itself"
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

    pub fn file_len(&self) -> u64 {
        self.file_len
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn max_resident_pages(&self) -> usize {
        self.max_resident_pages
    }

    pub fn metrics(&self) -> &ReaderMetrics {
        &self.metrics
    }

    pub fn page_count(&self) -> u64 {
        let ps = self.page_size as u64;
        (self.file_len + ps - 1) / ps
    }

    /// Read one page by index. Returns the page bytes (which may
    /// be shorter than `page_size` for the last page).
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
        let page_bytes = owned.len() as u64;
        self.lru.push_back(CachedPage {
            index,
            bytes: owned,
        });
        self.bytes_resident += page_bytes;
        if self.bytes_resident > self.metrics.bytes_resident_high_water {
            self.metrics.bytes_resident_high_water = self.bytes_resident;
        }
        self.metrics.pages_read_from_disk += 1;
        self.metrics.bytes_read_from_disk += page_bytes;
        Ok(buf)
    }

    /// Read an arbitrary byte range. Walks the underlying pages
    /// one at a time and concatenates only the requested slice
    /// from each, so the reader never holds more than
    /// `max_resident_pages * page_size` bytes resident even when
    /// the caller asks for a long range.
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
        for page_index in first_page..=last_page {
            let page = self.read_page(page_index)?;
            let page_offset = page_index * ps;
            let local_start = range.start.saturating_sub(page_offset) as usize;
            let local_end = (end - page_offset).min(page.len() as u64) as usize;
            if local_start < local_end {
                out.extend_from_slice(&page[local_start..local_end]);
            }
        }
        Ok(out)
    }

    /// Streaming search across the file. Returns the byte offset
    /// of the first match, or `None`. Streaming so the caller
    /// does not have to materialise the whole file. The reader
    /// scans page-by-page with a small overlap window equal to
    /// `needle.len() - 1` so a match spanning two pages is
    /// caught.
    pub fn find_first(&mut self, needle: &[u8]) -> std::io::Result<Option<u64>> {
        if needle.is_empty() {
            return Ok(Some(0));
        }
        if needle.len() > self.page_size {
            // The prototype refuses needles bigger than one page;
            // production swaps in a chunked search with a buffer
            // sized to the needle. The intent is reviewable here:
            // streaming search has a bounded window.
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
            let tail_len = needle.len() - 1;
            if page.len() >= tail_len {
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

    fn evict_to_make_room_for(&mut self, _new_page_bytes: u64) {
        while self.lru.len() >= self.max_resident_pages {
            if let Some(victim) = self.lru.pop_front() {
                self.bytes_resident = self.bytes_resident.saturating_sub(victim.bytes.len() as u64);
                self.metrics.pages_evicted += 1;
            } else {
                break;
            }
        }
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if haystack.len() < needle.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
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
            "aureline-largefile-proto-paged-{nanos}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn cleanup(p: PathBuf) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn write_synth(path: &Path, len: usize) {
        // Synthetic deterministic content so tests can cite
        // expected bytes without committing big fixtures.
        let mut bytes = Vec::with_capacity(len);
        for i in 0..len {
            bytes.push((i % 251) as u8);
        }
        std::fs::write(path, &bytes).unwrap();
    }

    #[test]
    fn page_count_rounds_up() {
        let dir = tempdir();
        let path = dir.join("synth.bin");
        write_synth(&path, 200);
        let r = PagedReader::open_with(&path, 64, 2).unwrap();
        assert_eq!(r.file_len(), 200);
        assert_eq!(r.page_count(), 4); // 64+64+64+8
        cleanup(dir);
    }

    #[test]
    fn read_range_walks_pages_and_caps_resident_bytes() {
        let dir = tempdir();
        let path = dir.join("synth.bin");
        let len = 4096usize;
        write_synth(&path, len);
        let page_size = 256usize;
        let max_pages = 2usize;
        let mut r = PagedReader::open_with(&path, page_size, max_pages).unwrap();
        let bytes = r.read_range(0..len as u64).unwrap();
        assert_eq!(bytes.len(), len);
        for (i, b) in bytes.iter().enumerate() {
            assert_eq!(*b, (i % 251) as u8, "mismatch at byte {i}");
        }
        // Bounded resident set.
        let resident_bytes_cap = (page_size * max_pages) as u64;
        assert!(
            r.metrics.bytes_resident_high_water <= resident_bytes_cap,
            "resident high water {} exceeded cap {resident_bytes_cap}",
            r.metrics.bytes_resident_high_water
        );
        // Eviction must have happened.
        let total_pages = (len / page_size) as u64;
        assert!(r.metrics.pages_evicted >= total_pages - max_pages as u64);
        cleanup(dir);
    }

    #[test]
    fn read_page_cache_hit_is_counted() {
        let dir = tempdir();
        let path = dir.join("synth.bin");
        write_synth(&path, 1024);
        let mut r = PagedReader::open_with(&path, 256, 4).unwrap();
        let _ = r.read_page(1).unwrap();
        let _ = r.read_page(1).unwrap();
        assert_eq!(r.metrics.pages_read_from_disk, 1);
        assert_eq!(r.metrics.pages_served_from_cache, 1);
        cleanup(dir);
    }

    #[test]
    fn find_first_finds_match_within_page() {
        let dir = tempdir();
        let path = dir.join("text.bin");
        std::fs::write(&path, b"hello world goodbye world").unwrap();
        let mut r = PagedReader::open_with(&path, 256, 4).unwrap();
        let pos = r.find_first(b"goodbye").unwrap();
        assert_eq!(pos, Some(12));
        cleanup(dir);
    }

    #[test]
    fn find_first_finds_match_spanning_pages() {
        let dir = tempdir();
        let path = dir.join("text.bin");
        // Page size 8, needle "needle" length 6. Place "needle"
        // straddling pages (at offset 6: "..NEEDLE..").
        let mut bytes = b"......needle......".to_vec();
        // Pad to exercise multi-page case.
        bytes.extend_from_slice(b"more bytes here");
        std::fs::write(&path, &bytes).unwrap();
        let mut r = PagedReader::open_with(&path, 8, 4).unwrap();
        let pos = r.find_first(b"needle").unwrap();
        assert_eq!(pos, Some(6));
        cleanup(dir);
    }

    #[test]
    fn find_first_refuses_oversized_needle() {
        let dir = tempdir();
        let path = dir.join("text.bin");
        std::fs::write(&path, b"hello").unwrap();
        let mut r = PagedReader::open_with(&path, 4, 2).unwrap();
        assert_eq!(r.find_first(b"hello world").unwrap(), None);
        cleanup(dir);
    }

    #[test]
    fn read_range_clamps_to_file_len() {
        let dir = tempdir();
        let path = dir.join("synth.bin");
        write_synth(&path, 100);
        let mut r = PagedReader::open_with(&path, 32, 2).unwrap();
        let bytes = r.read_range(80..200).unwrap();
        assert_eq!(bytes.len(), 20);
        assert_eq!(bytes[0], 80 % 251);
        cleanup(dir);
    }

    #[test]
    fn empty_range_yields_empty() {
        let dir = tempdir();
        let path = dir.join("synth.bin");
        write_synth(&path, 50);
        let mut r = PagedReader::open_with(&path, 16, 2).unwrap();
        assert!(r.read_range(10..10).unwrap().is_empty());
        assert!(r.read_range(60..70).unwrap().is_empty());
        cleanup(dir);
    }
}
