use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_MARKER_TIMESTAMP_BYTES: usize = 1_024;
static MARKER_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(1);

/// Outcome reported when starting a crash-marker guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashMarkerOutcome {
    /// True when the previous run did not clear the marker file.
    pub prior_run_abnormal: bool,
}

/// Crash-marker guard used to detect abnormal termination between launches.
///
/// The marker file is created at startup and must be cleared explicitly on a
/// clean shutdown. If the marker is still present on the next launch,
/// recovery surfaces treat the previous run as an abnormal termination.
#[derive(Debug, Clone)]
pub struct CrashMarkerGuard {
    marker_path: PathBuf,
    cleared: bool,
}

impl CrashMarkerGuard {
    /// Creates (or refreshes) the crash marker under `root_dir`.
    pub fn begin(
        root_dir: impl AsRef<Path>,
        emitted_at: &str,
    ) -> Result<(Self, CrashMarkerOutcome), String> {
        let root_dir = root_dir.as_ref();
        if root_dir.as_os_str().is_empty()
            || root_dir
                .components()
                .any(|component| matches!(component, Component::ParentDir))
        {
            return Err("crash marker root is invalid".to_string());
        }
        if emitted_at.is_empty()
            || emitted_at.len() > MAX_MARKER_TIMESTAMP_BYTES
            || emitted_at.bytes().any(|byte| byte == 0)
        {
            return Err("crash marker timestamp is invalid".to_string());
        }
        let marker_path = root_dir.join("crash_marker.json");
        ensure_regular_root(root_dir)?;

        let prior_run_abnormal = match fs::symlink_metadata(&marker_path) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
                return Err("crash marker is not a regular file".to_string());
            }
            Ok(_) => true,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => false,
            Err(err) => return Err(format!("crash marker inspect failed: {err}")),
        };

        let payload = format!(
            "{{\n  \"record_kind\": \"crash_marker\",\n  \"schema_version\": 1,\n  \"emitted_at\": \"{emitted_at}\"\n}}\n",
            emitted_at = escape_json_string(emitted_at),
        );
        if !prior_run_abnormal {
            publish_new_marker(root_dir, &marker_path, payload.as_bytes())?;
        }

        Ok((
            Self {
                marker_path,
                cleared: false,
            },
            CrashMarkerOutcome { prior_run_abnormal },
        ))
    }

    /// Clears the crash marker to indicate a clean shutdown.
    pub fn mark_clean_shutdown(&mut self) -> Result<(), String> {
        if self.cleared {
            return Ok(());
        }
        match fs::symlink_metadata(&self.marker_path) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
                return Err("crash marker changed to a non-regular file".to_string());
            }
            Ok(_) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                self.cleared = true;
                return Ok(());
            }
            Err(err) => return Err(format!("crash marker inspect failed: {err}")),
        }
        match fs::remove_file(&self.marker_path) {
            Ok(()) => {
                self.cleared = true;
                Ok(())
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                self.cleared = true;
                Ok(())
            }
            Err(err) => Err(format!("crash marker remove failed: {err}")),
        }
    }
}

fn ensure_regular_root(root: &Path) -> Result<(), String> {
    match fs::symlink_metadata(root) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err("crash marker root is not a regular directory".to_string())
        }
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(root)
                .map_err(|err| format!("crash marker dir create failed: {err}"))?;
            let metadata = fs::symlink_metadata(root)
                .map_err(|err| format!("crash marker dir inspect failed: {err}"))?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err("crash marker root creation was redirected".to_string());
            }
            Ok(())
        }
        Err(err) => Err(format!("crash marker dir inspect failed: {err}")),
    }
}

fn publish_new_marker(root: &Path, marker_path: &Path, bytes: &[u8]) -> Result<(), String> {
    let sequence = MARKER_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temporary = root.join(format!(
        ".crash_marker.tmp-{}-{sequence}-{stamp}",
        std::process::id()
    ));
    let result = (|| -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|err| format!("crash marker temp create failed: {err}"))?;
        file.write_all(bytes)
            .map_err(|err| format!("crash marker temp write failed: {err}"))?;
        file.sync_all()
            .map_err(|err| format!("crash marker temp sync failed: {err}"))?;
        fs::hard_link(&temporary, marker_path)
            .map_err(|err| format!("crash marker publish failed: {err}"))?;
        sync_directory(root)?;
        Ok(())
    })();
    let _ = fs::remove_file(&temporary);
    result
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), String> {
    fs::File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(|err| format!("crash marker dir sync failed: {err}"))
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), String> {
    Ok(())
}

fn escape_json_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_marker_is_preserved_until_clean_shutdown() {
        let dir = tempfile::tempdir().expect("tempdir");
        let marker = dir.path().join("crash_marker.json");
        fs::write(&marker, b"prior-run-evidence").expect("seed marker");

        let (mut guard, outcome) = CrashMarkerGuard::begin(dir.path(), "mono:new")
            .expect("existing regular marker accepted");
        assert!(outcome.prior_run_abnormal);
        assert_eq!(
            fs::read(&marker).expect("read marker"),
            b"prior-run-evidence"
        );

        guard.mark_clean_shutdown().expect("clean shutdown");
        assert!(!marker.exists());
    }

    #[cfg(unix)]
    #[test]
    fn symlink_marker_is_rejected_without_touching_target() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().expect("tempdir");
        let victim = dir.path().join("victim.json");
        fs::write(&victim, b"do-not-touch").expect("seed victim");
        symlink(&victim, dir.path().join("crash_marker.json")).expect("symlink marker");

        let error = CrashMarkerGuard::begin(dir.path(), "mono:new")
            .expect_err("symlink marker must fail closed");
        assert!(error.contains("not a regular file"));
        assert_eq!(fs::read(&victim).expect("read victim"), b"do-not-touch");
    }
}
