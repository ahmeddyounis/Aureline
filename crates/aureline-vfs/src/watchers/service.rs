use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::watcher::{WatcherHealth, WatcherRegistry, WatcherSource};

use super::polling::{PollingScanOptions, PollingScanner};
use super::types::{VfsChangeEvent, VfsChangeKind, WatcherEvent};

/// Options controlling watcher bring-up and fallback polling.
#[derive(Debug, Clone)]
pub struct WatcherServiceOptions {
    /// Forces polling fallback even when an OS-native watcher is available.
    pub force_polling: bool,
    /// Polling interval used when `force_polling = true` or when the native
    /// watcher cannot start.
    pub polling_interval: Duration,
}

impl Default for WatcherServiceOptions {
    fn default() -> Self {
        Self {
            force_polling: false,
            polling_interval: Duration::from_secs(2),
        }
    }
}

/// Errors returned when spawning a [`WatcherService`].
#[derive(Debug)]
pub enum WatcherServiceError {
    RootPathMissing(PathBuf),
    ThreadSpawnFailed(std::io::Error),
}

impl std::fmt::Display for WatcherServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RootPathMissing(path) => write!(f, "watch root does not exist: {path:?}"),
            Self::ThreadSpawnFailed(err) => write!(f, "failed to spawn watcher thread: {err}"),
        }
    }
}

impl std::error::Error for WatcherServiceError {}

/// Background watcher service for one root.
///
/// The service emits watcher-health frames plus normalized change events.
pub struct WatcherService {
    root_id: String,
    event_rx: mpsc::Receiver<WatcherEvent>,
    shutdown: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
    latest_health: Arc<Mutex<WatcherHealth>>,
}

impl std::fmt::Debug for WatcherService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WatcherService")
            .field("root_id", &self.root_id)
            .field("latest_health", &self.latest_health())
            .finish()
    }
}

impl WatcherService {
    /// Spawns a watcher service rooted at a local filesystem directory.
    ///
    /// The service prefers an OS-native watcher backend. When that cannot be
    /// started (or when `options.force_polling = true`), it falls back to a
    /// bounded polling scanner that emits the same [`WatcherEvent`] envelope.
    pub fn spawn_local(
        root_id: impl Into<String>,
        root_path: PathBuf,
        options: WatcherServiceOptions,
    ) -> Result<Self, WatcherServiceError> {
        if !root_path.exists() {
            return Err(WatcherServiceError::RootPathMissing(root_path));
        }
        let root_id = root_id.into();
        let (tx, rx) = mpsc::channel();
        let shutdown = Arc::new(AtomicBool::new(false));
        let latest_health = Arc::new(Mutex::new(WatcherHealth::Warming));

        let worker_shutdown = shutdown.clone();
        let worker_root_id = root_id.clone();
        let worker_latest_health = latest_health.clone();

        let worker = std::thread::Builder::new()
            .name("aureline_vfs_watcher".to_owned())
            .spawn(move || {
                run_worker(
                    worker_root_id,
                    root_path,
                    options,
                    tx,
                    worker_shutdown,
                    worker_latest_health,
                );
            })
            .map_err(WatcherServiceError::ThreadSpawnFailed)?;

        Ok(Self {
            root_id,
            event_rx: rx,
            shutdown,
            worker: Some(worker),
            latest_health,
        })
    }

    /// Returns the root identifier the service is attached to.
    pub fn root_id(&self) -> &str {
        &self.root_id
    }

    /// Returns the latest watcher-health state observed for this root.
    pub fn latest_health(&self) -> WatcherHealth {
        *self
            .latest_health
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Reads the next watcher event if one is ready.
    pub fn try_recv(&self) -> Option<WatcherEvent> {
        self.event_rx.try_recv().ok()
    }
}

impl Drop for WatcherService {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(handle) = self.worker.take() {
            let _ = handle.join();
        }
    }
}

fn run_worker(
    root_id: String,
    root_path: PathBuf,
    options: WatcherServiceOptions,
    tx: mpsc::Sender<WatcherEvent>,
    shutdown: Arc<AtomicBool>,
    latest_health: Arc<Mutex<WatcherHealth>>,
) {
    let mut registry = WatcherRegistry::new();
    let start_observed_at = observed_at();

    let initial_source = if options.force_polling {
        WatcherSource::PollingFallback
    } else {
        WatcherSource::OsNativeWatcher
    };
    registry.register(
        root_id.clone(),
        initial_source,
        start_observed_at,
        Some("watcher_start".to_owned()),
    );
    let _ = tx.send(WatcherEvent::Health(
        registry.frames().last().cloned().unwrap(),
    ));

    if !options.force_polling {
        if let Some(watcher) = try_spawn_native_watcher(
            root_id.clone(),
            root_path.clone(),
            tx.clone(),
            shutdown.clone(),
        ) {
            registry.transition(
                &root_id,
                WatcherHealth::Healthy,
                observed_at(),
                Some("watcher_primed".to_owned()),
            );
            update_health(&registry, &tx, &latest_health);
            // Keep the worker alive while the native watcher runs; the notify
            // backend invokes our callback on its own threads.
            while !shutdown.load(Ordering::Relaxed) {
                sleep_interruptible(&shutdown, Duration::from_millis(200));
            }
            drop(watcher);
            return;
        }
    }

    // Polling fallback.
    if options.force_polling {
        registry.transition(
            &root_id,
            WatcherHealth::FallbackPolling,
            observed_at(),
            Some("polling_fallback_active".to_owned()),
        );
    } else {
        registry.rebind(
            &root_id,
            WatcherSource::PollingFallback,
            WatcherHealth::FallbackPolling,
            observed_at(),
            Some("polling_fallback_active".to_owned()),
        );
    }
    update_health(&registry, &tx, &latest_health);

    let mut scanner = PollingScanner::new(
        root_id.clone(),
        root_path,
        PollingScanOptions {
            interval: options.polling_interval,
            ..PollingScanOptions::default()
        },
    );
    scanner.seed_snapshot();

    while !shutdown.load(Ordering::Relaxed) {
        sleep_interruptible(&shutdown, scanner.interval());
        if shutdown.load(Ordering::Relaxed) {
            break;
        }
        for event in scanner.scan_delta() {
            let _ = tx.send(WatcherEvent::Change(event));
        }
    }
}

fn update_health(
    registry: &WatcherRegistry,
    tx: &mpsc::Sender<WatcherEvent>,
    latest_health: &Arc<Mutex<WatcherHealth>>,
) {
    if let Some(frame) = registry.frames().last() {
        if let Ok(mut guard) = latest_health.lock() {
            *guard = frame.watcher_health;
        }
        let _ = tx.send(WatcherEvent::Health(frame.clone()));
    }
}

fn observed_at() -> String {
    let now = SystemTime::now();
    let millis = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("unix_ms:{millis}")
}

fn sleep_interruptible(shutdown: &AtomicBool, duration: Duration) {
    let step = Duration::from_millis(50);
    let mut remaining = duration;
    while remaining > Duration::ZERO && !shutdown.load(Ordering::Relaxed) {
        let chunk = if remaining > step { step } else { remaining };
        std::thread::sleep(chunk);
        remaining = remaining.saturating_sub(chunk);
    }
}

fn try_spawn_native_watcher(
    root_id: String,
    root_path: PathBuf,
    tx: mpsc::Sender<WatcherEvent>,
    shutdown: Arc<AtomicBool>,
) -> Option<notify::RecommendedWatcher> {
    use notify::Watcher as _;

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if shutdown.load(Ordering::Relaxed) {
                return;
            }
            let event = match res {
                Ok(ev) => ev,
                Err(_) => {
                    let _ = tx.send(WatcherEvent::Change(VfsChangeEvent {
                        root_id: root_id.clone(),
                        kind: VfsChangeKind::Rescan,
                    }));
                    return;
                }
            };
            for normalized in normalize_notify_event(&root_id, &event) {
                let _ = tx.send(WatcherEvent::Change(normalized));
            }
        })
        .ok()?;

    watcher
        .watch(&root_path, notify::RecursiveMode::Recursive)
        .ok()?;

    Some(watcher)
}

fn normalize_notify_event(root_id: &str, event: &notify::Event) -> Vec<VfsChangeEvent> {
    use notify::event::{EventKind, ModifyKind, RenameMode};

    if event.need_rescan() {
        return vec![VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Rescan,
        }];
    }

    match &event.kind {
        EventKind::Create(_) => {
            normalize_paths(root_id, &event.paths, |uri| VfsChangeKind::Created { uri })
        }
        EventKind::Remove(_) => {
            normalize_removed_paths(root_id, &event.paths, |uri| VfsChangeKind::Deleted { uri })
        }
        EventKind::Modify(ModifyKind::Data(_)) | EventKind::Modify(ModifyKind::Metadata(_)) => {
            normalize_paths(root_id, &event.paths, |uri| VfsChangeKind::Modified { uri })
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) if event.paths.len() >= 2 => {
            let from = crate::uri_model::VfsUri::file_url_for_path_lossy(&event.paths[0]);
            let to = crate::uri_model::VfsUri::file_url_for_path(&event.paths[1])
                .or_else(|| crate::uri_model::VfsUri::file_url_for_path_lossy(&event.paths[1]));
            if let (Some(from), Some(to)) = (from, to) {
                vec![VfsChangeEvent {
                    root_id: root_id.to_owned(),
                    kind: VfsChangeKind::Renamed { from, to },
                }]
            } else {
                vec![VfsChangeEvent {
                    root_id: root_id.to_owned(),
                    kind: VfsChangeKind::Rescan,
                }]
            }
        }
        EventKind::Modify(ModifyKind::Name(_)) => vec![VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Rescan,
        }],
        _ => vec![VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Rescan,
        }],
    }
}

fn normalize_paths(
    root_id: &str,
    paths: &[PathBuf],
    mk_kind: impl Fn(crate::uri_model::VfsUri) -> VfsChangeKind,
) -> Vec<VfsChangeEvent> {
    if paths.is_empty() {
        return vec![VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Rescan,
        }];
    }
    let mut out = Vec::new();
    for path in paths {
        let Some(uri) = crate::uri_model::VfsUri::file_url_for_path(path)
            .or_else(|| crate::uri_model::VfsUri::file_url_for_path_lossy(path))
        else {
            return vec![VfsChangeEvent {
                root_id: root_id.to_owned(),
                kind: VfsChangeKind::Rescan,
            }];
        };
        out.push(VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: mk_kind(uri),
        });
    }
    out
}

fn normalize_removed_paths(
    root_id: &str,
    paths: &[PathBuf],
    mk_kind: impl Fn(crate::uri_model::VfsUri) -> VfsChangeKind,
) -> Vec<VfsChangeEvent> {
    if paths.is_empty() {
        return vec![VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: VfsChangeKind::Rescan,
        }];
    }
    let mut out = Vec::new();
    for path in paths {
        let Some(uri) = crate::uri_model::VfsUri::file_url_for_path_lossy(path) else {
            return vec![VfsChangeEvent {
                root_id: root_id.to_owned(),
                kind: VfsChangeKind::Rescan,
            }];
        };
        out.push(VfsChangeEvent {
            root_id: root_id.to_owned(),
            kind: mk_kind(uri),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_notify_event_maps_common_kinds() {
        use notify::event::{
            CreateKind, DataChange, EventAttributes, EventKind, ModifyKind, RemoveKind, RenameMode,
        };

        let root_id = "root-test";
        let tmp = std::env::temp_dir();

        let created_path = tmp.join("aureline-watcher-created.txt");
        let create = notify::Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![created_path.clone()],
            attrs: EventAttributes::default(),
        };
        let out = normalize_notify_event(root_id, &create);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].kind, VfsChangeKind::Created { .. }));

        let removed_path = tmp.join("aureline-watcher-removed.txt");
        let remove = notify::Event {
            kind: EventKind::Remove(RemoveKind::File),
            paths: vec![removed_path.clone()],
            attrs: EventAttributes::default(),
        };
        let out = normalize_notify_event(root_id, &remove);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].kind, VfsChangeKind::Deleted { .. }));

        let modified_path = tmp.join("aureline-watcher-modified.txt");
        let modify = notify::Event {
            kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
            paths: vec![modified_path.clone()],
            attrs: EventAttributes::default(),
        };
        let out = normalize_notify_event(root_id, &modify);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].kind, VfsChangeKind::Modified { .. }));

        let from_path = tmp.join("aureline-watcher-from.txt");
        let to_path = tmp.join("aureline-watcher-to.txt");
        let rename = notify::Event {
            kind: EventKind::Modify(ModifyKind::Name(RenameMode::Both)),
            paths: vec![from_path.clone(), to_path.clone()],
            attrs: EventAttributes::default(),
        };
        let out = normalize_notify_event(root_id, &rename);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].kind, VfsChangeKind::Renamed { .. }));
    }
}
