use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::uri_model::VfsUri;

use super::types::{VfsChangeEvent, VfsChangeKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct FileSignature {
    pub modified_nanos: u128,
    pub len: u64,
}

impl FileSignature {
    fn for_path(path: &Path) -> Option<Self> {
        let meta = std::fs::metadata(path).ok()?;
        if !meta.is_file() {
            return None;
        }
        let modified = meta.modified().ok().unwrap_or(SystemTime::UNIX_EPOCH);
        let modified_nanos = modified
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Some(Self {
            modified_nanos,
            len: meta.len(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FileEntry {
    pub uri: VfsUri,
    pub signature: FileSignature,
}

#[derive(Debug, Clone)]
pub(super) struct PollingScanOptions {
    pub interval: Duration,
    pub max_files: usize,
}

impl Default for PollingScanOptions {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(2),
            max_files: 50_000,
        }
    }
}

#[derive(Debug)]
pub(super) struct PollingScanner {
    root_id: String,
    root_path: PathBuf,
    last_snapshot: BTreeMap<PathBuf, FileEntry>,
    options: PollingScanOptions,
}

impl PollingScanner {
    pub(super) fn new(root_id: String, root_path: PathBuf, options: PollingScanOptions) -> Self {
        Self {
            root_id,
            root_path,
            last_snapshot: BTreeMap::new(),
            options,
        }
    }

    pub(super) fn interval(&self) -> Duration {
        self.options.interval
    }

    pub(super) fn seed_snapshot(&mut self) {
        self.last_snapshot = scan_files(&self.root_path, self.options.max_files);
    }

    pub(super) fn scan_delta(&mut self) -> Vec<VfsChangeEvent> {
        let next = scan_files(&self.root_path, self.options.max_files);
        let mut events = diff(&self.root_id, &self.last_snapshot, &next);
        events.sort_by(|a, b| a.root_id.cmp(&b.root_id));
        self.last_snapshot = next;
        events
    }
}

fn scan_files(root: &Path, max_files: usize) -> BTreeMap<PathBuf, FileEntry> {
    let mut out: BTreeMap<PathBuf, FileEntry> = BTreeMap::new();
    let mut queue: Vec<PathBuf> = vec![root.to_path_buf()];

    while let Some(dir) = queue.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            if out.len() >= max_files {
                return out;
            }
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_dir() {
                // Skip common build/output directories to keep the polling scan
                // bounded on typical workspaces.
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if should_skip_dir(name) {
                        continue;
                    }
                }
                queue.push(path);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            let Some(signature) = FileSignature::for_path(&path) else {
                continue;
            };
            let Some(uri) =
                VfsUri::file_url_for_path(&path).or_else(|| VfsUri::file_url_for_path_lossy(&path))
            else {
                continue;
            };
            out.insert(path, FileEntry { uri, signature });
        }
    }

    out
}

fn should_skip_dir(name: &str) -> bool {
    if name.starts_with('.') {
        return true;
    }
    matches!(
        name,
        "target" | "node_modules" | "dist" | "build" | "out" | "artifacts"
    )
}

fn diff(
    root_id: &str,
    prev: &BTreeMap<PathBuf, FileEntry>,
    next: &BTreeMap<PathBuf, FileEntry>,
) -> Vec<VfsChangeEvent> {
    let prev_keys: BTreeSet<&PathBuf> = prev.keys().collect();
    let next_keys: BTreeSet<&PathBuf> = next.keys().collect();

    let mut created: Vec<&PathBuf> = next_keys.difference(&prev_keys).copied().collect();
    let mut deleted: Vec<&PathBuf> = prev_keys.difference(&next_keys).copied().collect();

    // Heuristic rename pairing: if a single file disappeared and a single file
    // appeared with an identical signature, emit `rename` instead of
    // delete+create. Otherwise, preserve the two events.
    if created.len() == 1 && deleted.len() == 1 {
        let created_path = created[0];
        let deleted_path = deleted[0];
        if prev
            .get(deleted_path)
            .map(|entry| entry.signature)
            .zip(next.get(created_path).map(|entry| entry.signature))
            .is_some_and(|(a, b)| a == b)
        {
            let from = prev.get(deleted_path).map(|entry| entry.uri.clone());
            let to = next.get(created_path).map(|entry| entry.uri.clone());
            if let (Some(from), Some(to)) = (from, to) {
                return vec![VfsChangeEvent {
                    root_id: root_id.to_owned(),
                    kind: VfsChangeKind::Renamed { from, to },
                }];
            }
        }
    }

    let mut out = Vec::new();

    for path in created.drain(..) {
        let Some(uri) = next.get(path).map(|entry| entry.uri.clone()) else {
            out.push(VfsChangeEvent {
                root_id: root_id.to_owned(),
                kind: VfsChangeKind::Rescan,
            });
            continue;
        };
        out.push(VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Created { uri },
        });
    }

    for path in deleted.drain(..) {
        let Some(uri) = prev.get(path).map(|entry| entry.uri.clone()) else {
            out.push(VfsChangeEvent {
                root_id: root_id.to_owned(),
                kind: VfsChangeKind::Rescan,
            });
            continue;
        };
        out.push(VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Deleted { uri },
        });
    }

    for (path, next_entry) in next {
        let Some(prev_entry) = prev.get(path) else {
            continue;
        };
        if prev_entry.signature == next_entry.signature {
            continue;
        }
        let uri = next_entry.uri.clone();
        out.push(VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Modified { uri },
        });
    }

    out
}
