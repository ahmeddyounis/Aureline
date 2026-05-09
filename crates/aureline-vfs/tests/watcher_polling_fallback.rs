use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use aureline_vfs::{
    VfsChangeKind, WatcherEvent, WatcherHealth, WatcherService, WatcherServiceOptions,
    WatcherSource,
};

fn unique_temp_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("aureline-vfs-{label}-{nonce}"))
}

fn wait_for(
    service: &WatcherService,
    timeout: Duration,
    predicate: impl Fn(&WatcherEvent) -> bool,
) -> Option<WatcherEvent> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Some(event) = service.try_recv() {
            if predicate(&event) {
                return Some(event);
            }
            continue;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    None
}

#[test]
fn polling_fallback_emits_health_and_change_events() {
    let root = unique_temp_dir("watcher-polling");
    std::fs::create_dir_all(&root).expect("temp dir create");

    let service = WatcherService::spawn_local(
        "root-test",
        root.clone(),
        WatcherServiceOptions {
            force_polling: true,
            polling_interval: Duration::from_millis(50),
        },
    )
    .expect("watcher service spawn");

    let _health = wait_for(&service, Duration::from_secs(2), |event| {
        matches!(
            event,
            WatcherEvent::Health(frame)
                if frame.watcher_health == WatcherHealth::FallbackPolling
                    && frame.watcher_source == WatcherSource::PollingFallback
        )
    })
    .expect("expected fallback_polling health frame");

    let a_path = root.join("a.txt");
    std::fs::write(&a_path, b"hello\n").expect("file create");

    let a_uri = aureline_vfs::VfsUri::file_url_for_path(&a_path).expect("file uri");
    let _created = wait_for(&service, Duration::from_secs(2), |event| {
        matches!(
            event,
            WatcherEvent::Change(change)
                if change.kind == VfsChangeKind::Created { uri: a_uri.clone() }
        )
    })
    .expect("expected create event");

    std::fs::write(&a_path, b"hello world\n").expect("file modify");
    let _modified = wait_for(&service, Duration::from_secs(2), |event| {
        matches!(
            event,
            WatcherEvent::Change(change)
                if change.kind == VfsChangeKind::Modified { uri: a_uri.clone() }
        )
    })
    .expect("expected modify event");

    let b_path = root.join("b.txt");
    std::fs::rename(&a_path, &b_path).expect("rename");
    let b_uri = aureline_vfs::VfsUri::file_url_for_path(&b_path).expect("renamed uri");
    let rename_observed = wait_for(&service, Duration::from_secs(2), |event| match event {
        WatcherEvent::Change(change) => match &change.kind {
            VfsChangeKind::Renamed { from, to } => from == &a_uri && to == &b_uri,
            VfsChangeKind::Created { uri } => uri == &b_uri,
            _ => false,
        },
        _ => false,
    })
    .is_some();
    assert!(rename_observed, "expected rename (or created) event");

    std::fs::remove_file(&b_path).expect("delete");
    let _deleted = wait_for(&service, Duration::from_secs(2), |event| {
        matches!(
            event,
            WatcherEvent::Change(change)
                if change.kind == VfsChangeKind::Deleted { uri: b_uri.clone() }
        )
    })
    .expect("expected delete event");

    drop(service);
    let _ = std::fs::remove_dir_all(&root);
}
