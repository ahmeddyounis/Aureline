use std::fs;
use std::path::{Path, PathBuf};

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
    pub fn begin(root_dir: impl AsRef<Path>, emitted_at: &str) -> Result<(Self, CrashMarkerOutcome), String> {
        let root_dir = root_dir.as_ref();
        let marker_path = root_dir.join("crash_marker.json");
        let prior_run_abnormal = marker_path.exists();

        if let Some(parent) = marker_path.parent() {
            fs::create_dir_all(parent).map_err(|err| format!("crash marker dir create failed: {err}"))?;
        }

        let payload = format!(
            "{{\n  \"record_kind\": \"crash_marker\",\n  \"schema_version\": 1,\n  \"emitted_at\": \"{emitted_at}\"\n}}\n",
            emitted_at = escape_json_string(emitted_at),
        );
        fs::write(&marker_path, payload)
            .map_err(|err| format!("crash marker write failed: {err}"))?;

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

fn escape_json_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

