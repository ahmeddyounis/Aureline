//! Repository clone execution using the system `git` binary.
//!
//! The shell intentionally shells out to the user's installed `git` instead of
//! linking libgit2: credential helpers, proxy settings, and platform trust
//! stores already live behind that tool boundary, while the shell keeps the
//! operation in a background thread and records typed failures.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Request to materialize one Git repository at a local destination path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloneRequest {
    /// Remote URL or clone source accepted by `git clone`.
    pub remote_url: String,
    /// Full destination directory path passed as the clone target.
    pub destination_path: PathBuf,
}

impl CloneRequest {
    /// Builds a clone request from raw sheet input.
    pub fn new(remote_url: impl Into<String>, destination_path: impl Into<PathBuf>) -> Self {
        Self {
            remote_url: remote_url.into(),
            destination_path: destination_path.into(),
        }
    }

    /// Validates the request before a worker starts.
    ///
    /// # Errors
    ///
    /// Returns [`CloneErrorClass::InvalidInput`] when the URL or destination is
    /// empty, or [`CloneErrorClass::DestinationExists`] when the destination is
    /// already occupied by a non-empty directory or file.
    pub fn validate(&self) -> Result<(), CloneError> {
        if self.remote_url.trim().is_empty() {
            return Err(CloneError::new(
                CloneErrorClass::InvalidInput,
                "remote URL is required",
            ));
        }
        if self.destination_path.as_os_str().is_empty() {
            return Err(CloneError::new(
                CloneErrorClass::InvalidInput,
                "destination path is required",
            ));
        }
        if self.destination_path.is_file() {
            return Err(CloneError::new(
                CloneErrorClass::DestinationExists,
                format!(
                    "destination path is an existing file: {}",
                    self.destination_path.display()
                ),
            ));
        }
        if self.destination_path.is_dir() && !directory_is_empty(&self.destination_path)? {
            return Err(CloneError::new(
                CloneErrorClass::DestinationExists,
                format!(
                    "destination directory is not empty: {}",
                    self.destination_path.display()
                ),
            ));
        }
        Ok(())
    }
}

/// Startup probe result for the system Git executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitProbe {
    /// Human-readable version line returned by `git --version`.
    pub version_line: String,
}

/// Typed clone failure classes surfaced to sheets, notifications, and command
/// result packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloneErrorClass {
    /// The system `git` binary could not be found or launched.
    GitNotInstalled,
    /// The clone sheet supplied incomplete or malformed input.
    InvalidInput,
    /// The destination path already exists in a form `git clone` cannot use.
    DestinationExists,
    /// Authentication or authorization failed.
    Auth,
    /// The remote repository was not found or refused the requested URL.
    RemoteNotFound,
    /// Network name resolution, connection, or timeout failure.
    Network,
    /// SSH host-key verification failed.
    HostKey,
    /// The local filesystem reported a full disk or write failure.
    DiskFull,
    /// A local filesystem permission or path error blocked materialization.
    Filesystem,
    /// Git exited with a non-zero status that did not match a narrower class.
    GitExited,
    /// The shell encountered an I/O error while supervising the process.
    Io,
}

impl CloneErrorClass {
    /// Returns the stable snake_case token for this error class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitNotInstalled => "git_not_installed",
            Self::InvalidInput => "invalid_input",
            Self::DestinationExists => "destination_exists",
            Self::Auth => "auth",
            Self::RemoteNotFound => "remote_not_found",
            Self::Network => "network",
            Self::HostKey => "host_key",
            Self::DiskFull => "disk_full",
            Self::Filesystem => "filesystem",
            Self::GitExited => "git_exited",
            Self::Io => "io",
        }
    }

    /// Returns the short presentation label for this error class.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GitNotInstalled => "Git not installed",
            Self::InvalidInput => "Invalid input",
            Self::DestinationExists => "Destination exists",
            Self::Auth => "Authentication failed",
            Self::RemoteNotFound => "Remote not found",
            Self::Network => "Network failed",
            Self::HostKey => "Host key failed",
            Self::DiskFull => "Disk full",
            Self::Filesystem => "Filesystem failed",
            Self::GitExited => "Git failed",
            Self::Io => "I/O failed",
        }
    }
}

/// Error returned by clone probing or execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloneError {
    /// Typed class used by command results and retry UI.
    pub class: CloneErrorClass,
    /// Human-readable detail from validation or Git stderr.
    pub message: String,
}

impl CloneError {
    /// Builds a clone error with a stable class and display message.
    pub fn new(class: CloneErrorClass, message: impl Into<String>) -> Self {
        Self {
            class,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CloneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.class.as_str(), self.message)
    }
}

impl std::error::Error for CloneError {}

/// Progress phase emitted by a clone backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloneProgressPhase {
    /// The worker has validated input and is about to invoke Git.
    Starting,
    /// Git emitted progress text for the active clone.
    Progress,
    /// The clone process completed successfully.
    Completed,
}

/// One progress event from the clone worker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloneProgressEvent {
    /// Coarse phase of the event.
    pub phase: CloneProgressPhase,
    /// Short progress label safe for in-product activity rows.
    pub message: String,
}

impl CloneProgressEvent {
    /// Builds a progress event.
    pub fn new(phase: CloneProgressPhase, message: impl Into<String>) -> Self {
        Self {
            phase,
            message: message.into(),
        }
    }
}

/// Backend abstraction used by tests and by the live system Git adapter.
pub trait GitCloneBackend {
    /// Probes whether the backend is available.
    ///
    /// # Errors
    ///
    /// Returns a typed error when Git is missing or the backend cannot be
    /// initialized.
    fn probe(&self) -> Result<GitProbe, CloneError>;

    /// Clones `request`, emitting progress events while it runs.
    ///
    /// # Errors
    ///
    /// Returns a typed clone error for validation, process, network, auth, and
    /// filesystem failures.
    fn clone_repository(
        &self,
        request: &CloneRequest,
        progress: &mut dyn FnMut(CloneProgressEvent),
    ) -> Result<(), CloneError>;
}

/// `git` process-backed clone backend used by the live shell.
#[derive(Debug, Clone)]
pub struct SystemGitCloneBackend {
    git_binary: PathBuf,
}

impl Default for SystemGitCloneBackend {
    fn default() -> Self {
        Self {
            git_binary: PathBuf::from("git"),
        }
    }
}

impl SystemGitCloneBackend {
    /// Builds a backend that invokes the provided Git binary path.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl GitCloneBackend for SystemGitCloneBackend {
    fn probe(&self) -> Result<GitProbe, CloneError> {
        let output = Command::new(&self.git_binary)
            .arg("--version")
            .output()
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    CloneError::new(CloneErrorClass::GitNotInstalled, "git binary was not found")
                } else {
                    CloneError::new(
                        CloneErrorClass::GitNotInstalled,
                        format!("git could not be launched: {err}"),
                    )
                }
            })?;

        if !output.status.success() {
            return Err(CloneError::new(
                CloneErrorClass::GitNotInstalled,
                "git --version did not complete successfully",
            ));
        }

        let version_line = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(GitProbe { version_line })
    }

    fn clone_repository(
        &self,
        request: &CloneRequest,
        progress: &mut dyn FnMut(CloneProgressEvent),
    ) -> Result<(), CloneError> {
        request.validate()?;
        progress(CloneProgressEvent::new(
            CloneProgressPhase::Starting,
            "Starting clone",
        ));

        let mut child = Command::new(&self.git_binary)
            .arg("clone")
            .arg("--progress")
            .arg(request.remote_url.trim())
            .arg(&request.destination_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    CloneError::new(CloneErrorClass::GitNotInstalled, "git binary was not found")
                } else {
                    CloneError::new(
                        CloneErrorClass::Io,
                        format!("git clone launch failed: {err}"),
                    )
                }
            })?;

        let mut stderr_text = String::new();
        if let Some(stderr) = child.stderr.as_mut() {
            read_progress(stderr, &mut stderr_text, progress)?;
        }

        let status = child.wait().map_err(|err| {
            CloneError::new(CloneErrorClass::Io, format!("git clone wait failed: {err}"))
        })?;
        if status.success() {
            progress(CloneProgressEvent::new(
                CloneProgressPhase::Completed,
                "Clone completed",
            ));
            return Ok(());
        }

        Err(classify_git_failure(&stderr_text, status.code()))
    }
}

fn directory_is_empty(path: &Path) -> Result<bool, CloneError> {
    let mut entries = std::fs::read_dir(path).map_err(|err| {
        CloneError::new(
            CloneErrorClass::Filesystem,
            format!("destination directory cannot be read: {err}"),
        )
    })?;
    Ok(entries.next().is_none())
}

fn read_progress(
    reader: &mut dyn Read,
    stderr_text: &mut String,
    progress: &mut dyn FnMut(CloneProgressEvent),
) -> Result<(), CloneError> {
    let mut buf = [0u8; 4096];
    let mut line = Vec::<u8>::new();
    loop {
        let read = reader.read(&mut buf).map_err(|err| {
            CloneError::new(
                CloneErrorClass::Io,
                format!("git clone stderr read failed: {err}"),
            )
        })?;
        if read == 0 {
            break;
        }
        for byte in &buf[..read] {
            if *byte == b'\n' || *byte == b'\r' {
                emit_progress_line(&line, stderr_text, progress);
                line.clear();
            } else {
                line.push(*byte);
            }
        }
    }
    emit_progress_line(&line, stderr_text, progress);
    Ok(())
}

fn emit_progress_line(
    line: &[u8],
    stderr_text: &mut String,
    progress: &mut dyn FnMut(CloneProgressEvent),
) {
    let text = String::from_utf8_lossy(line).trim().to_string();
    if text.is_empty() {
        return;
    }
    if !stderr_text.is_empty() {
        stderr_text.push('\n');
    }
    stderr_text.push_str(&text);
    progress(CloneProgressEvent::new(CloneProgressPhase::Progress, text));
}

fn classify_git_failure(stderr_text: &str, status_code: Option<i32>) -> CloneError {
    let lower = stderr_text.to_ascii_lowercase();
    let class = if lower.contains("authentication failed")
        || lower.contains("could not read username")
        || lower.contains("permission denied (publickey)")
        || lower.contains("access denied")
        || lower.contains("authorization failed")
    {
        CloneErrorClass::Auth
    } else if lower.contains("repository not found")
        || lower.contains("not found")
        || lower.contains("does not appear to be a git repository")
    {
        CloneErrorClass::RemoteNotFound
    } else if lower.contains("host key verification failed")
        || lower.contains("remote host identification has changed")
    {
        CloneErrorClass::HostKey
    } else if lower.contains("no space left on device") || lower.contains("disk full") {
        CloneErrorClass::DiskFull
    } else if lower.contains("destination path")
        && (lower.contains("already exists") || lower.contains("not an empty directory"))
    {
        CloneErrorClass::DestinationExists
    } else if lower.contains("could not resolve host")
        || lower.contains("failed to connect")
        || lower.contains("network is unreachable")
        || lower.contains("connection timed out")
        || lower.contains("unable to access")
        || lower.contains("couldn't connect")
    {
        CloneErrorClass::Network
    } else if lower.contains("permission denied")
        || lower.contains("read-only file system")
        || lower.contains("input/output error")
    {
        CloneErrorClass::Filesystem
    } else {
        CloneErrorClass::GitExited
    };

    let fallback = status_code
        .map(|code| format!("git clone exited with status {code}"))
        .unwrap_or_else(|| "git clone terminated unsuccessfully".to_string());
    let message = stderr_text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .unwrap_or(fallback);
    CloneError::new(class, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct MockGitCloneBackend {
        probe: Result<GitProbe, CloneError>,
        events: Vec<CloneProgressEvent>,
        result: Result<(), CloneError>,
    }

    impl GitCloneBackend for MockGitCloneBackend {
        fn probe(&self) -> Result<GitProbe, CloneError> {
            self.probe.clone()
        }

        fn clone_repository(
            &self,
            request: &CloneRequest,
            progress: &mut dyn FnMut(CloneProgressEvent),
        ) -> Result<(), CloneError> {
            request.validate()?;
            for event in self.events.clone() {
                progress(event);
            }
            self.result.clone()
        }
    }

    #[test]
    fn mocked_backend_emits_progress_and_succeeds_without_network() {
        let destination = tempfile::tempdir().expect("temp dir");
        let target = destination.path().join("repo");
        let backend = MockGitCloneBackend {
            probe: Ok(GitProbe {
                version_line: "git version test".to_string(),
            }),
            events: vec![
                CloneProgressEvent::new(CloneProgressPhase::Starting, "Starting clone"),
                CloneProgressEvent::new(CloneProgressPhase::Progress, "Receiving objects: 50%"),
                CloneProgressEvent::new(CloneProgressPhase::Completed, "Clone completed"),
            ],
            result: Ok(()),
        };

        assert_eq!(backend.probe().unwrap().version_line, "git version test");
        let mut events = Vec::new();
        backend
            .clone_repository(
                &CloneRequest::new("https://example.invalid/repo.git", target),
                &mut |event| events.push(event),
            )
            .expect("mock clone should succeed");

        assert_eq!(events.len(), 3);
        assert_eq!(events[1].phase, CloneProgressPhase::Progress);
    }

    #[test]
    fn request_rejects_non_empty_destination_before_backend_runs() {
        let destination = tempfile::tempdir().expect("temp dir");
        std::fs::write(destination.path().join("README.md"), "occupied\n").expect("seed file");
        let request = CloneRequest::new("https://example.invalid/repo.git", destination.path());

        let err = request
            .validate()
            .expect_err("destination should be occupied");
        assert_eq!(err.class, CloneErrorClass::DestinationExists);
    }

    #[test]
    fn classifies_common_git_failure_stderr() {
        let auth = classify_git_failure("fatal: Authentication failed for url", Some(128));
        assert_eq!(auth.class, CloneErrorClass::Auth);

        let host = classify_git_failure("Host key verification failed.", Some(128));
        assert_eq!(host.class, CloneErrorClass::HostKey);

        let disk = classify_git_failure("fatal: No space left on device", Some(128));
        assert_eq!(disk.class, CloneErrorClass::DiskFull);

        let missing = classify_git_failure("ERROR: Repository not found.", Some(128));
        assert_eq!(missing.class, CloneErrorClass::RemoteNotFound);
    }

    #[test]
    fn mocked_probe_surfaces_git_not_installed() {
        let backend = MockGitCloneBackend {
            probe: Err(CloneError::new(
                CloneErrorClass::GitNotInstalled,
                "git binary was not found",
            )),
            events: Vec::new(),
            result: Ok(()),
        };

        let err = backend.probe().expect_err("probe should fail");
        assert_eq!(err.class.as_str(), "git_not_installed");
    }
}
