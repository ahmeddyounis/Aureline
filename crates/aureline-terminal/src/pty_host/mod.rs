//! PTY host abstraction and session-header truth.
//!
//! The PTY host is the single owner of terminal-session lifecycle and
//! provenance. Higher surfaces — the shell's terminal pane, the activity
//! center, the title-context bar, support export — never mint their own
//! session ids, headers, or lifecycle vocabulary; they project the records
//! defined here.
//!
//! ## Why one host abstraction
//!
//! Embedding shell launches directly in the UI thread forks session truth
//! across surfaces and produces anonymous tabs the moment a shell exits. The
//! host owns:
//!
//! - **stable identity.** Every session carries a [`PtySessionId`] derived
//!   from `(workspace_id, host_class, sequence)`; the id survives termination,
//!   restart, transport loss, and quarantine so the pane chrome can re-attach
//!   the same row to the same provenance.
//! - **provenance.** Every session carries a [`SessionHeader`] with title,
//!   cwd hint, target badge, host class, execution-context ref, trust state,
//!   and the local-vs-managed boundary cue. The header is the canonical
//!   truth a tab chip, a status mirror, a support packet, and a restore
//!   prompt all quote verbatim.
//! - **lifecycle.** A small [`SessionLifecycleState`] state machine owns the
//!   `Requested → Starting → Active → LostTransport → ReconnectedSameIdentity
//!   → Closed` walk and the `Active → Quarantined` failure branch. Surfaces
//!   never invent "loading" / "broken" euphemisms; they read the canonical
//!   token.
//!
//! ## Failure-drill posture
//!
//! Terminating or restarting a session must not erase its header. The host
//! retains the [`SessionHeader`] across `LostTransport` and
//! `ReconnectedSameIdentity` transitions, and a `Closed` session keeps its
//! provenance row available until the consumer drops it. The fixture suite
//! under `/fixtures/terminal/session_cases/*.json` exercises this contract.

use std::collections::{BTreeMap, VecDeque};
use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use portable_pty::{native_pty_system, CommandBuilder, ExitStatus, PtyPair, PtySize};
use serde::{Deserialize, Serialize};

use aureline_workspace::TrustState;

/// Default visible terminal size used when opening a fresh PTY before the UI
/// has delivered a pane geometry.
pub const DEFAULT_PTY_SIZE: PtySize = PtySize {
    rows: 24,
    cols: 80,
    pixel_width: 0,
    pixel_height: 0,
};

/// Maximum number of unread PTY output bytes retained per live session.
pub const DEFAULT_PTY_OUTPUT_RING_CAPACITY: usize = 1024 * 1024;

/// Command line used when opening a PTY-backed session for tests or tooling.
///
/// Production callers normally use [`PtyHost::open_session`], which resolves
/// the user's default shell. This type exists for deterministic smoke tests
/// and narrowly scoped tooling launches such as `/bin/echo hello`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtyCommand {
    argv: Vec<OsString>,
}

impl PtyCommand {
    /// Creates a command with the provided executable as `argv[0]`.
    pub fn new(program: impl Into<OsString>) -> Self {
        Self {
            argv: vec![program.into()],
        }
    }

    /// Appends one argument and returns the updated command.
    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.argv.push(arg.into());
        self
    }

    /// Appends multiple arguments and returns the updated command.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.argv.extend(args.into_iter().map(Into::into));
        self
    }

    /// Returns the command vector exactly as it will be passed to the PTY
    /// command builder.
    pub fn argv(&self) -> &[OsString] {
        &self.argv
    }

    fn into_builder(self) -> CommandBuilder {
        CommandBuilder::from_argv(self.argv)
    }
}

/// Typed failure stage used when a fresh PTY launch quarantines a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PtyLaunchFailureReason {
    /// The OS rejected PTY allocation.
    OpenPtyFailed,
    /// The PTY master reader could not be cloned for the output thread.
    CloneReaderFailed,
    /// The PTY master writer could not be acquired for user input.
    TakeWriterFailed,
    /// The child process could not be spawned into the slave PTY.
    SpawnCommandFailed,
    /// The reader thread could not be started.
    ReaderThreadFailed,
}

impl PtyLaunchFailureReason {
    /// Stable reason token recorded on the quarantine transition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenPtyFailed => "pty_open_failed",
            Self::CloneReaderFailed => "pty_clone_reader_failed",
            Self::TakeWriterFailed => "pty_take_writer_failed",
            Self::SpawnCommandFailed => "pty_spawn_failed",
            Self::ReaderThreadFailed => "pty_reader_thread_failed",
        }
    }
}

/// Bytes drained from a live PTY session's output ring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtyOutputDrain {
    /// Output bytes observed from the PTY master since the prior drain.
    pub bytes: Vec<u8>,
    /// Cumulative byte count dropped because the unread output ring filled.
    pub dropped_byte_count: u64,
    /// True after the reader has observed EOF or a read error.
    pub reader_closed: bool,
    /// Last reader error, when the PTY stream ended with an I/O failure.
    pub read_error: Option<String>,
}

/// Stable identifier for a single terminal session.
///
/// The id is derived from `(workspace_id, host_class, sequence)` at host time
/// and never mutates, even when transport drops, the shell exits, or the
/// session quarantines. The id is opaque on serialization boundaries; callers
/// must not parse it back into structured fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PtySessionId(String);

impl PtySessionId {
    /// Construct a session id from its host inputs.
    pub fn from_parts(workspace_id: &str, host_class: HostClass, sequence: u64) -> Self {
        Self(format!(
            "pty:{workspace_id}|{host}|{sequence}",
            host = host_class.as_str(),
        ))
    }

    /// Stable string form. Safe to log and to ship in support bundles.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Consume into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for PtySessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Closed vocabulary for the session host class.
///
/// The class names whether the session's PTY runs on the local desktop or a
/// managed/remote target. Surfaces use the class to render the local-vs-
/// managed boundary cue without re-deriving truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostClass {
    /// PTY backed by the user's local desktop.
    HostDesktop,
    /// PTY backed by a managed remote agent.
    RemoteAgentPrimary,
    /// PTY backed by a local container or sandbox.
    LocalContainer,
}

impl HostClass {
    /// Stable string token used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostDesktop => "host_desktop",
            Self::RemoteAgentPrimary => "remote_agent_primary",
            Self::LocalContainer => "local_container",
        }
    }

    /// Short human-readable badge label, e.g. for the bottom-panel tab chip.
    pub const fn target_badge(self) -> &'static str {
        match self {
            Self::HostDesktop => "Local",
            Self::RemoteAgentPrimary => "Remote",
            Self::LocalContainer => "Container",
        }
    }

    /// True when the session's target is not the local desktop and the chrome
    /// MUST render a visible boundary cue.
    pub const fn needs_boundary_cue(self) -> bool {
        !matches!(self, Self::HostDesktop)
    }

    /// Stable boundary-cue token, suitable for chrome and support exports.
    pub const fn boundary_cue_token(self) -> &'static str {
        match self {
            Self::HostDesktop => "boundary_cue_local_session",
            Self::RemoteAgentPrimary => "boundary_cue_remote_session",
            Self::LocalContainer => "boundary_cue_container_session",
        }
    }
}

/// Trust posture projected onto the terminal session.
///
/// Mirrors [`aureline_workspace::TrustState`]; we re-export the projection so
/// callers can consume the typed enum without forking the trust vocabulary.
pub type TerminalTrustState = TrustState;

/// Canonical lifecycle state for a terminal session.
///
/// The state machine is intentionally small. It models the contract that
/// downstream terminal, task, debug, provider, and tracing surfaces can build
/// on without re-deriving session truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleState {
    /// User requested a fresh session through the command-dispatch boundary.
    Requested,
    /// Host is preparing the PTY (allocating, attaching, warming).
    Starting,
    /// PTY is attached and the session is interactive.
    Active,
    /// Transport dropped; the session is detached but its provenance is
    /// preserved for an explicit reconnect or fresh-session decision.
    LostTransport,
    /// Transport recovered against the same target identity. Read-only state
    /// resumes; in-flight mutations are NOT replayed.
    ReconnectedSameIdentity,
    /// Session is shutting down or has shut down. The header remains
    /// addressable until the consumer drops the row.
    Closed,
    /// Supervisor revoked the session because it exceeded a protocol-violation
    /// budget for the current trust tier. Re-admission requires the user to
    /// open a fresh session.
    Quarantined,
}

impl SessionLifecycleState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "session_requested",
            Self::Starting => "session_starting",
            Self::Active => "session_active",
            Self::LostTransport => "session_lost_transport",
            Self::ReconnectedSameIdentity => "session_reconnected_same_identity",
            Self::Closed => "session_closed",
            Self::Quarantined => "session_quarantined",
        }
    }

    /// True when the chrome should render the session as taking input today.
    pub const fn is_interactive(self) -> bool {
        matches!(self, Self::Active | Self::ReconnectedSameIdentity)
    }

    /// True when the chrome should render a degraded-state cue alongside the
    /// header (the row is still addressable, but not currently usable).
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::LostTransport | Self::Closed | Self::Quarantined)
    }
}

/// One transition frame emitted by the host.
///
/// Frames carry the same lifecycle vocabulary the header exports so audit
/// surfaces, support packets, and the activity center can replay the walk
/// without inventing local timestamps or reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionLifecycleTransition {
    pub session_id: PtySessionId,
    pub from_state: SessionLifecycleState,
    pub to_state: SessionLifecycleState,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

/// Canonical session-header record.
///
/// The header is the single truth a shell tab chip, a status-bar mirror, a
/// restore prompt, and a support packet all consume. Surfaces never compute a
/// title or cwd hint locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionHeader {
    pub session_id: PtySessionId,
    pub workspace_id: String,
    pub host_class: HostClass,
    pub target_badge: String,
    pub boundary_cue_token: String,
    pub display_title: String,
    /// Optional cwd hint. Absent when the session has not yet observed a cwd
    /// (e.g. baseline shell with no shell-integration signal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    /// Stable reference to the execution-context object the session runs
    /// under. Surfaces consume this verbatim to wire context inspectors.
    pub execution_context_ref: String,
    pub trust_state: TerminalTrustState,
    pub trust_state_token: String,
    pub lifecycle_state: SessionLifecycleState,
    pub lifecycle_state_token: String,
    /// Sequence within the host. Stable across renames; bumps only when a new
    /// fresh session is opened.
    pub sequence: u64,
    pub created_at: String,
    pub last_observed_at: String,
}

impl SessionHeader {
    /// True when the chrome MUST render the local-vs-managed boundary cue.
    pub const fn needs_boundary_cue(&self) -> bool {
        self.host_class.needs_boundary_cue()
    }

    /// True when the chrome should render a degraded chip on this row.
    pub const fn is_degraded(&self) -> bool {
        self.lifecycle_state.is_degraded()
    }
}

/// Inputs accepted when opening a fresh session on the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSessionRequest<'a> {
    pub workspace_id: &'a str,
    pub host_class: HostClass,
    pub display_title: &'a str,
    pub cwd_hint: Option<&'a str>,
    pub execution_context_ref: &'a str,
    pub trust_state: TerminalTrustState,
    pub observed_at: &'a str,
}

/// Errors emitted by the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtyHostError {
    UnknownSession(PtySessionId),
    /// The session has no live PTY runtime attached.
    NoLivePty(PtySessionId),
    /// The requested operation requires an interactive session.
    SessionNotInteractive {
        session_id: PtySessionId,
        lifecycle_state: SessionLifecycleState,
    },
    /// Writing bytes to the PTY master failed.
    WriteFailed {
        session_id: PtySessionId,
        reason: String,
    },
    /// Resizing the PTY master failed.
    ResizeFailed {
        session_id: PtySessionId,
        reason: String,
    },
    /// Closing the PTY child process failed.
    CloseFailed {
        session_id: PtySessionId,
        reason: String,
    },
    /// The host refused a transition that would erase a session's identity
    /// (e.g. moving directly from `Requested` to `Closed` without recording a
    /// header).
    InvalidTransition {
        session_id: PtySessionId,
        from_state: SessionLifecycleState,
        to_state: SessionLifecycleState,
    },
}

impl std::fmt::Display for PtyHostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSession(id) => write!(f, "unknown terminal session: {id}"),
            Self::NoLivePty(id) => write!(f, "terminal session has no live PTY: {id}"),
            Self::SessionNotInteractive {
                session_id,
                lifecycle_state,
            } => write!(
                f,
                "terminal session is not interactive: {session_id} ({state})",
                state = lifecycle_state.as_str(),
            ),
            Self::WriteFailed { session_id, reason } => {
                write!(
                    f,
                    "failed to write to terminal session {session_id}: {reason}"
                )
            }
            Self::ResizeFailed { session_id, reason } => {
                write!(
                    f,
                    "failed to resize terminal session {session_id}: {reason}"
                )
            }
            Self::CloseFailed { session_id, reason } => {
                write!(f, "failed to close terminal session {session_id}: {reason}")
            }
            Self::InvalidTransition {
                session_id,
                from_state,
                to_state,
            } => write!(
                f,
                "invalid lifecycle transition for {session_id}: {from} -> {to}",
                from = from_state.as_str(),
                to = to_state.as_str(),
            ),
        }
    }
}

impl std::error::Error for PtyHostError {}

/// One terminal session as the host knows it.
#[derive(Clone, Serialize, Deserialize)]
pub struct PtySession {
    pub header: SessionHeader,
    #[serde(skip)]
    runtime: Option<Arc<Mutex<PtyRuntime>>>,
}

impl std::fmt::Debug for PtySession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtySession")
            .field("header", &self.header)
            .field("has_live_pty", &self.runtime.is_some())
            .finish()
    }
}

impl PartialEq for PtySession {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}

impl Eq for PtySession {}

impl PtySession {
    /// Returns the session id.
    pub fn session_id(&self) -> &PtySessionId {
        &self.header.session_id
    }

    /// Returns the canonical session header.
    pub const fn header(&self) -> &SessionHeader {
        &self.header
    }

    /// Returns the current lifecycle state.
    pub const fn lifecycle_state(&self) -> SessionLifecycleState {
        self.header.lifecycle_state
    }

    /// True when this row has a live PTY runtime attached.
    pub fn has_live_pty(&self) -> bool {
        self.runtime.is_some()
    }
}

#[derive(Debug)]
struct PtyOutputRing {
    capacity: usize,
    inner: Mutex<PtyOutputRingInner>,
}

#[derive(Debug, Default)]
struct PtyOutputRingInner {
    bytes: VecDeque<u8>,
    dropped_byte_count: u64,
    reader_closed: bool,
    read_error: Option<String>,
}

impl PtyOutputRing {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            inner: Mutex::new(PtyOutputRingInner::default()),
        }
    }

    fn push(&self, bytes: &[u8]) {
        if bytes.is_empty() || self.capacity == 0 {
            return;
        }

        let mut inner = self.lock_inner();
        if bytes.len() >= self.capacity {
            let dropped_existing = inner.bytes.len() as u64;
            let dropped_incoming = (bytes.len() - self.capacity) as u64;
            inner.bytes.clear();
            inner
                .bytes
                .extend(bytes[bytes.len() - self.capacity..].iter().copied());
            inner.dropped_byte_count = inner
                .dropped_byte_count
                .saturating_add(dropped_existing)
                .saturating_add(dropped_incoming);
            return;
        }

        while inner.bytes.len() + bytes.len() > self.capacity {
            inner.bytes.pop_front();
            inner.dropped_byte_count = inner.dropped_byte_count.saturating_add(1);
        }
        inner.bytes.extend(bytes.iter().copied());
    }

    fn mark_reader_closed(&self, read_error: Option<String>) {
        let mut inner = self.lock_inner();
        inner.reader_closed = true;
        inner.read_error = read_error;
    }

    fn drain(&self) -> PtyOutputDrain {
        let mut inner = self.lock_inner();
        let bytes = inner.bytes.drain(..).collect();
        PtyOutputDrain {
            bytes,
            dropped_byte_count: inner.dropped_byte_count,
            reader_closed: inner.reader_closed,
            read_error: inner.read_error.clone(),
        }
    }

    fn lock_inner(&self) -> std::sync::MutexGuard<'_, PtyOutputRingInner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

struct PtyRuntime {
    pair: PtyPair,
    child: Box<dyn portable_pty::Child + Send + Sync>,
    writer: Option<Box<dyn Write + Send>>,
    output: Arc<PtyOutputRing>,
    reader_thread: Option<JoinHandle<()>>,
    last_size: PtySize,
}

impl std::fmt::Debug for PtyRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtyRuntime")
            .field("child_process_id", &self.child.process_id())
            .field("has_writer", &self.writer.is_some())
            .field("last_size", &self.last_size)
            .finish()
    }
}

impl PtyRuntime {
    fn spawn(
        cwd_hint: Option<&str>,
        size: PtySize,
        launch: PtyLaunchCommand,
    ) -> Result<Self, PtyLaunchFailure> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(size)
            .map_err(|err| PtyLaunchFailure::new(PtyLaunchFailureReason::OpenPtyFailed, err))?;
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|err| PtyLaunchFailure::new(PtyLaunchFailureReason::CloneReaderFailed, err))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|err| PtyLaunchFailure::new(PtyLaunchFailureReason::TakeWriterFailed, err))?;
        let mut command = launch.into_builder();
        command.cwd(resolve_launch_cwd(cwd_hint));
        let mut child = pair.slave.spawn_command(command).map_err(|err| {
            PtyLaunchFailure::new(PtyLaunchFailureReason::SpawnCommandFailed, err)
        })?;
        let output = Arc::new(PtyOutputRing::new(DEFAULT_PTY_OUTPUT_RING_CAPACITY));
        let reader_thread = spawn_reader_thread(reader, Arc::clone(&output)).map_err(|err| {
            let _ = child.kill();
            PtyLaunchFailure::new(PtyLaunchFailureReason::ReaderThreadFailed, err)
        })?;

        Ok(Self {
            pair,
            child,
            writer: Some(writer),
            output,
            reader_thread: Some(reader_thread),
            last_size: size,
        })
    }

    fn write_input(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        let writer = self.writer.as_mut().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, "PTY writer is closed")
        })?;
        writer.write_all(bytes)?;
        writer.flush()
    }

    fn resize(&mut self, size: PtySize) -> Result<(), String> {
        self.pair
            .master
            .resize(size)
            .map_err(|err| err.to_string())?;
        self.last_size = size;
        Ok(())
    }

    fn close(&mut self) -> std::io::Result<()> {
        self.writer.take();
        for _ in 0..5 {
            if self.child.try_wait()?.is_some() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(20));
        }
        self.child.kill()
    }

    fn try_wait(&mut self) -> std::io::Result<Option<ExitStatus>> {
        let status = self.child.try_wait()?;
        if status.is_some() {
            self.writer.take();
        }
        Ok(status)
    }
}

impl Drop for PtyRuntime {
    fn drop(&mut self) {
        self.writer.take();
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
        }
        self.reader_thread.take();
    }
}

#[derive(Debug, Clone)]
enum PtyLaunchCommand {
    DefaultShell,
    Command(PtyCommand),
}

impl PtyLaunchCommand {
    fn into_builder(self) -> CommandBuilder {
        match self {
            Self::DefaultShell => CommandBuilder::new_default_prog(),
            Self::Command(command) => command.into_builder(),
        }
    }
}

#[derive(Debug)]
struct PtyLaunchFailure {
    reason: PtyLaunchFailureReason,
}

impl PtyLaunchFailure {
    fn new(reason: PtyLaunchFailureReason, _err: impl std::fmt::Display) -> Self {
        Self { reason }
    }
}

fn spawn_reader_thread(
    mut reader: Box<dyn Read + Send>,
    output: Arc<PtyOutputRing>,
) -> std::io::Result<JoinHandle<()>> {
    thread::Builder::new()
        .name("aureline-pty-reader".to_owned())
        .spawn(move || {
            let mut buf = [0_u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => {
                        output.mark_reader_closed(None);
                        break;
                    }
                    Ok(read) => output.push(&buf[..read]),
                    Err(err) => {
                        output.mark_reader_closed(Some(err.to_string()));
                        break;
                    }
                }
            }
        })
}

fn resolve_launch_cwd(cwd_hint: Option<&str>) -> PathBuf {
    cwd_hint
        .map(Path::new)
        .filter(|path| path.is_dir())
        .map(Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Single, inspectable PTY host.
///
/// The host owns the canonical session map, the lifecycle state machine, the
/// live PTY runtime for local sessions, and the transition log so shell
/// surfaces can read one truth.
#[derive(Debug, Clone, Default)]
pub struct PtyHost {
    next_sequence: u64,
    sessions: BTreeMap<PtySessionId, PtySession>,
    order: Vec<PtySessionId>,
    transitions: Vec<SessionLifecycleTransition>,
}

impl PtyHost {
    /// Construct an empty host.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of sessions currently tracked (including closed sessions whose
    /// rows are still pinned for provenance).
    pub fn session_count(&self) -> usize {
        self.order.len()
    }

    /// Iterate sessions in insertion order. The order is stable across
    /// lifecycle transitions, so a tab strip never reshuffles when transport
    /// drops.
    pub fn sessions(&self) -> impl Iterator<Item = &PtySession> {
        self.order
            .iter()
            .filter_map(move |id| self.sessions.get(id))
    }

    /// Look up a session by id.
    pub fn session(&self, id: &PtySessionId) -> Option<&PtySession> {
        self.sessions.get(id)
    }

    /// Drain the transition log.
    pub fn drain_transitions(&mut self) -> Vec<SessionLifecycleTransition> {
        std::mem::take(&mut self.transitions)
    }

    /// Open a fresh session backed by the user's default shell.
    ///
    /// The host mints a stable [`PtySessionId`], builds the canonical header,
    /// records the `Requested → Starting → Active` walk, and attaches a
    /// [`portable_pty`] runtime for local desktop sessions. If PTY allocation
    /// or process spawn fails, the same row transitions to
    /// [`SessionLifecycleState::Quarantined`] with a typed reason token.
    pub fn open_session(&mut self, request: OpenSessionRequest<'_>) -> PtySessionId {
        self.open_session_with_launch(request, PtyLaunchCommand::DefaultShell)
    }

    /// Open a fresh session backed by a specific command.
    ///
    /// This is primarily for deterministic terminal smoke tests and narrow
    /// tooling launches. The same lifecycle, output ring, input writer, and
    /// quarantine behavior as [`Self::open_session`] apply.
    pub fn open_command_session(
        &mut self,
        request: OpenSessionRequest<'_>,
        command: PtyCommand,
    ) -> PtySessionId {
        self.open_session_with_launch(request, PtyLaunchCommand::Command(command))
    }

    fn open_session_with_launch(
        &mut self,
        request: OpenSessionRequest<'_>,
        launch: PtyLaunchCommand,
    ) -> PtySessionId {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);

        let session_id =
            PtySessionId::from_parts(request.workspace_id, request.host_class, sequence);
        let header = SessionHeader {
            session_id: session_id.clone(),
            workspace_id: request.workspace_id.to_owned(),
            host_class: request.host_class,
            target_badge: request.host_class.target_badge().to_owned(),
            boundary_cue_token: request.host_class.boundary_cue_token().to_owned(),
            display_title: request.display_title.to_owned(),
            cwd_hint: request.cwd_hint.map(str::to_owned),
            execution_context_ref: request.execution_context_ref.to_owned(),
            trust_state: request.trust_state,
            trust_state_token: request.trust_state.as_str().to_owned(),
            lifecycle_state: SessionLifecycleState::Requested,
            lifecycle_state_token: SessionLifecycleState::Requested.as_str().to_owned(),
            sequence,
            created_at: request.observed_at.to_owned(),
            last_observed_at: request.observed_at.to_owned(),
        };

        self.sessions.insert(
            session_id.clone(),
            PtySession {
                header: header.clone(),
                runtime: None,
            },
        );
        self.order.push(session_id.clone());
        self.transitions.push(SessionLifecycleTransition {
            session_id: session_id.clone(),
            from_state: SessionLifecycleState::Requested,
            to_state: SessionLifecycleState::Requested,
            observed_at: request.observed_at.to_owned(),
            reason_code: Some("session_opened".to_owned()),
        });
        self.start_session_runtime(&session_id, &request, launch);
        session_id
    }

    fn start_session_runtime(
        &mut self,
        session_id: &PtySessionId,
        request: &OpenSessionRequest<'_>,
        launch: PtyLaunchCommand,
    ) {
        let _ = self.mark_starting(session_id, request.observed_at);
        if !matches!(request.host_class, HostClass::HostDesktop) {
            let _ = self.mark_active(session_id, request.observed_at);
            return;
        }

        match PtyRuntime::spawn(request.cwd_hint, DEFAULT_PTY_SIZE, launch) {
            Ok(runtime) => {
                if let Some(session) = self.sessions.get_mut(session_id) {
                    session.runtime = Some(Arc::new(Mutex::new(runtime)));
                }
                let _ = self.mark_active(session_id, request.observed_at);
            }
            Err(err) => {
                let _ = self.transition(
                    session_id,
                    SessionLifecycleState::Quarantined,
                    request.observed_at,
                    Some(err.reason.as_str()),
                    |from| {
                        matches!(
                            from,
                            SessionLifecycleState::Requested | SessionLifecycleState::Starting
                        )
                    },
                );
            }
        }
    }

    /// Mark a session as starting (host is preparing the PTY).
    pub fn mark_starting(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let state = self.lifecycle_state(session_id)?;
        if matches!(
            state,
            SessionLifecycleState::Starting
                | SessionLifecycleState::Active
                | SessionLifecycleState::ReconnectedSameIdentity
        ) {
            self.touch(session_id, observed_at)?;
            return Ok(());
        }
        self.transition(
            session_id,
            SessionLifecycleState::Starting,
            observed_at,
            Some("starting"),
            |from| matches!(from, SessionLifecycleState::Requested),
        )
    }

    /// Mark a session as active (PTY attached and interactive).
    pub fn mark_active(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let state = self.lifecycle_state(session_id)?;
        if matches!(state, SessionLifecycleState::Active) {
            self.touch(session_id, observed_at)?;
            return Ok(());
        }
        self.transition(
            session_id,
            SessionLifecycleState::Active,
            observed_at,
            Some("attached"),
            |from| {
                matches!(
                    from,
                    SessionLifecycleState::Requested
                        | SessionLifecycleState::Starting
                        | SessionLifecycleState::ReconnectedSameIdentity
                )
            },
        )
    }

    /// Update the cwd hint as the host observes a new working directory. The
    /// hint is the canonical truth quoted by every header consumer.
    pub fn update_cwd_hint(
        &mut self,
        session_id: &PtySessionId,
        cwd_hint: Option<&str>,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        session.header.cwd_hint = cwd_hint.map(str::to_owned);
        session.header.last_observed_at = observed_at.to_owned();
        Ok(())
    }

    /// Update the display title (e.g. command name or window title escape).
    pub fn update_display_title(
        &mut self,
        session_id: &PtySessionId,
        display_title: &str,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        session.header.display_title = display_title.to_owned();
        session.header.last_observed_at = observed_at.to_owned();
        Ok(())
    }

    /// Record that transport dropped. The header is preserved verbatim so the
    /// pane chip continues to disclose target, cwd, and context after the
    /// drop.
    pub fn mark_lost_transport(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<(), PtyHostError> {
        let state = self.lifecycle_state(session_id)?;
        if matches!(state, SessionLifecycleState::LostTransport) {
            self.touch(session_id, observed_at)?;
            return Ok(());
        }
        self.transition(
            session_id,
            SessionLifecycleState::LostTransport,
            observed_at,
            reason_code.or(Some("transport_dropped")),
            |from| {
                matches!(
                    from,
                    SessionLifecycleState::Active | SessionLifecycleState::ReconnectedSameIdentity
                )
            },
        )
    }

    /// Record a successful reconnect against the same target identity. The
    /// session id, header, and sequence are unchanged so downstream surfaces
    /// can match the prior provenance row.
    pub fn mark_reconnected_same_identity(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let state = self.lifecycle_state(session_id)?;
        if matches!(state, SessionLifecycleState::ReconnectedSameIdentity) {
            self.touch(session_id, observed_at)?;
            return Ok(());
        }
        self.transition(
            session_id,
            SessionLifecycleState::ReconnectedSameIdentity,
            observed_at,
            Some("reconnected_same_identity"),
            |from| matches!(from, SessionLifecycleState::LostTransport),
        )
    }

    /// Quarantine a session because the supervisor revoked it.
    pub fn quarantine(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::Quarantined,
            observed_at,
            Some(reason_code),
            |from| {
                matches!(
                    from,
                    SessionLifecycleState::Active
                        | SessionLifecycleState::Starting
                        | SessionLifecycleState::Requested
                        | SessionLifecycleState::ReconnectedSameIdentity
                        | SessionLifecycleState::LostTransport
                )
            },
        )
    }

    /// Close a live session and transition its retained header to `Closed`.
    ///
    /// The host first closes the writer, waits briefly for a graceful child
    /// exit, and then asks the PTY child handle to terminate if it is still
    /// running. The session header remains queryable after close.
    pub fn close_session(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<(), PtyHostError> {
        let state = self.lifecycle_state(session_id)?;
        if matches!(state, SessionLifecycleState::Closed) {
            self.touch(session_id, observed_at)?;
            return Ok(());
        }

        if let Some(runtime) = self.session_runtime(session_id)? {
            runtime
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .close()
                .map_err(|err| PtyHostError::CloseFailed {
                    session_id: session_id.clone(),
                    reason: err.to_string(),
                })?;
        }

        self.transition(
            session_id,
            SessionLifecycleState::Closed,
            observed_at,
            reason_code.or(Some("closed")),
            |from| !matches!(from, SessionLifecycleState::Closed),
        )
    }

    /// Close a session.
    pub fn close(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<(), PtyHostError> {
        self.close_session(session_id, observed_at, reason_code)
    }

    /// Resize the PTY backing a live session.
    ///
    /// The `size` values are forwarded to the OS PTY layer. On Unix this
    /// causes `SIGWINCH` delivery to the foreground process group when the
    /// platform supports it.
    pub fn resize(
        &mut self,
        session_id: &PtySessionId,
        size: PtySize,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        self.require_interactive(session_id)?;
        let runtime = self
            .session_runtime(session_id)?
            .ok_or_else(|| PtyHostError::NoLivePty(session_id.clone()))?;
        runtime
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .resize(size)
            .map_err(|reason| PtyHostError::ResizeFailed {
                session_id: session_id.clone(),
                reason,
            })?;
        self.touch(session_id, observed_at)
    }

    /// Write user input bytes to the PTY backing a live session.
    ///
    /// Callers pass bytes exactly as they should reach the shell, including
    /// carriage returns or newlines appropriate to the target shell.
    pub fn write_input(
        &mut self,
        session_id: &PtySessionId,
        bytes: &[u8],
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        self.require_interactive(session_id)?;
        let runtime = self
            .session_runtime(session_id)?
            .ok_or_else(|| PtyHostError::NoLivePty(session_id.clone()))?;
        let write_result = runtime
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .write_input(bytes);
        match write_result {
            Ok(()) => self.touch(session_id, observed_at),
            Err(err) => {
                let _ = self.refresh_session_status(session_id, observed_at);
                if self
                    .session(session_id)
                    .map(|session| session.lifecycle_state().is_interactive())
                    .unwrap_or(false)
                {
                    let _ =
                        self.mark_lost_transport(session_id, observed_at, Some("pty_write_failed"));
                }
                Err(PtyHostError::WriteFailed {
                    session_id: session_id.clone(),
                    reason: err.to_string(),
                })
            }
        }
    }

    /// Drain bytes accumulated by the session's PTY reader thread.
    ///
    /// Draining also polls the child process and records a clean `Closed`
    /// transition when the PTY command has exited.
    pub fn drain_output(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<PtyOutputDrain, PtyHostError> {
        let runtime = self
            .session_runtime(session_id)?
            .ok_or_else(|| PtyHostError::NoLivePty(session_id.clone()))?;
        let drain = runtime
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .output
            .drain();
        self.refresh_session_status(session_id, observed_at)?;
        Ok(drain)
    }

    /// Poll the PTY child process and close the retained session row when the
    /// child has exited.
    pub fn refresh_session_status(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let state = self.lifecycle_state(session_id)?;
        if matches!(
            state,
            SessionLifecycleState::Closed | SessionLifecycleState::Quarantined
        ) {
            return Ok(());
        }
        let Some(runtime) = self.session_runtime(session_id)? else {
            return Ok(());
        };
        let status = runtime
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .try_wait()
            .map_err(|err| PtyHostError::CloseFailed {
                session_id: session_id.clone(),
                reason: err.to_string(),
            })?;
        if let Some(status) = status {
            let reason = if status.success() {
                "process_exited_success"
            } else {
                "process_exited_failure"
            };
            self.transition(
                session_id,
                SessionLifecycleState::Closed,
                observed_at,
                Some(reason),
                |from| !matches!(from, SessionLifecycleState::Closed),
            )?;
        }
        Ok(())
    }

    fn transition(
        &mut self,
        session_id: &PtySessionId,
        to: SessionLifecycleState,
        observed_at: &str,
        reason_code: Option<&str>,
        guard: impl FnOnce(SessionLifecycleState) -> bool,
    ) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        let from = session.header.lifecycle_state;
        if !guard(from) {
            return Err(PtyHostError::InvalidTransition {
                session_id: session_id.clone(),
                from_state: from,
                to_state: to,
            });
        }
        session.header.lifecycle_state = to;
        session.header.lifecycle_state_token = to.as_str().to_owned();
        session.header.last_observed_at = observed_at.to_owned();
        self.transitions.push(SessionLifecycleTransition {
            session_id: session_id.clone(),
            from_state: from,
            to_state: to,
            observed_at: observed_at.to_owned(),
            reason_code: reason_code.map(str::to_owned),
        });
        Ok(())
    }

    fn lifecycle_state(
        &self,
        session_id: &PtySessionId,
    ) -> Result<SessionLifecycleState, PtyHostError> {
        self.sessions
            .get(session_id)
            .map(PtySession::lifecycle_state)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))
    }

    fn touch(&mut self, session_id: &PtySessionId, observed_at: &str) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        session.header.last_observed_at = observed_at.to_owned();
        Ok(())
    }

    fn session_runtime(
        &self,
        session_id: &PtySessionId,
    ) -> Result<Option<Arc<Mutex<PtyRuntime>>>, PtyHostError> {
        self.sessions
            .get(session_id)
            .map(|session| session.runtime.clone())
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))
    }

    fn require_interactive(&self, session_id: &PtySessionId) -> Result<(), PtyHostError> {
        let lifecycle_state = self.lifecycle_state(session_id)?;
        if lifecycle_state.is_interactive() {
            Ok(())
        } else {
            Err(PtyHostError::SessionNotInteractive {
                session_id: session_id.clone(),
                lifecycle_state,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{Duration, Instant};

    fn open_local(host: &mut PtyHost) -> PtySessionId {
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::HostDesktop,
            display_title: "zsh",
            cwd_hint: Some("~/code/aureline"),
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        })
    }

    fn command_request<'a>(display_title: &'a str) -> OpenSessionRequest<'a> {
        OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::HostDesktop,
            display_title,
            cwd_hint: Some(env!("CARGO_MANIFEST_DIR")),
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        }
    }

    #[cfg(unix)]
    fn echo_command() -> PtyCommand {
        PtyCommand::new("/bin/echo").arg("hello")
    }

    #[cfg(windows)]
    fn echo_command() -> PtyCommand {
        PtyCommand::new("cmd.exe").args(["/C", "echo hello"])
    }

    #[cfg(unix)]
    fn interactive_shell_command() -> PtyCommand {
        PtyCommand::new("/bin/sh")
    }

    #[cfg(windows)]
    fn interactive_shell_command() -> PtyCommand {
        PtyCommand::new("cmd.exe").arg("/Q")
    }

    fn collect_until(
        host: &mut PtyHost,
        id: &PtySessionId,
        needle: &[u8],
    ) -> (Vec<u8>, SessionLifecycleState) {
        let started = Instant::now();
        let mut output = Vec::new();
        let mut state = host.session(id).unwrap().lifecycle_state();
        while started.elapsed() < Duration::from_secs(3) {
            let drained = host.drain_output(id, "mono:poll").unwrap();
            output.extend(drained.bytes);
            state = host.session(id).unwrap().lifecycle_state();
            if output.windows(needle.len()).any(|window| window == needle)
                && matches!(state, SessionLifecycleState::Closed)
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        (output, state)
    }

    #[test]
    fn open_session_records_stable_header_and_id() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        assert_eq!(id.as_str(), "pty:ws-test|host_desktop|0");

        let session = host.session(&id).expect("session must exist");
        let header = session.header();
        assert_eq!(header.session_id, id);
        assert_eq!(header.workspace_id, "ws-test");
        assert_eq!(header.host_class, HostClass::HostDesktop);
        assert_eq!(header.target_badge, "Local");
        assert_eq!(header.boundary_cue_token, "boundary_cue_local_session");
        assert_eq!(header.display_title, "zsh");
        assert_eq!(header.cwd_hint.as_deref(), Some("~/code/aureline"));
        assert_eq!(
            header.execution_context_ref,
            "execution_context.local_desktop.workspace_root"
        );
        assert_eq!(header.trust_state_token, "trusted");
        assert_eq!(header.lifecycle_state, SessionLifecycleState::Active);
        assert!(session.has_live_pty());
        assert!(!header.needs_boundary_cue());
    }

    #[test]
    fn command_session_drains_echo_output_and_closes_on_clean_exit() {
        let mut host = PtyHost::new();
        let id = host.open_command_session(command_request("echo"), echo_command());
        assert_eq!(
            host.session(&id).unwrap().lifecycle_state(),
            SessionLifecycleState::Active
        );

        let (output, state) = collect_until(&mut host, &id, b"hello");
        assert!(
            output
                .windows(b"hello".len())
                .any(|window| window == b"hello"),
            "expected echo output, got {:?}",
            String::from_utf8_lossy(&output)
        );
        assert_eq!(state, SessionLifecycleState::Closed);
    }

    #[test]
    fn write_input_reaches_spawned_shell_and_exit_closes_session() {
        let mut host = PtyHost::new();
        let id = host.open_command_session(command_request("shell"), interactive_shell_command());

        #[cfg(unix)]
        let input = b"printf 'aureline-input-ok'\nexit\n";
        #[cfg(windows)]
        let input = b"echo aureline-input-ok\r\nexit\r\n";

        host.write_input(&id, input, "mono:input").unwrap();
        let (output, state) = collect_until(&mut host, &id, b"aureline-input-ok");
        assert!(
            output
                .windows(b"aureline-input-ok".len())
                .any(|window| window == b"aureline-input-ok"),
            "expected shell output, got {:?}",
            String::from_utf8_lossy(&output)
        );
        assert_eq!(state, SessionLifecycleState::Closed);
    }

    #[test]
    fn resize_live_session_updates_the_pty_size() {
        let mut host = PtyHost::new();
        let id = host.open_command_session(command_request("shell"), interactive_shell_command());
        host.resize(
            &id,
            PtySize {
                rows: 30,
                cols: 100,
                pixel_width: 0,
                pixel_height: 0,
            },
            "mono:resize",
        )
        .unwrap();
        host.close_session(&id, "mono:close", Some("test_closed"))
            .unwrap();
        assert_eq!(
            host.session(&id).unwrap().lifecycle_state(),
            SessionLifecycleState::Closed
        );
    }

    #[test]
    fn spawn_failure_quarantines_session_with_typed_reason() {
        let mut host = PtyHost::new();
        let id = host.open_command_session(
            command_request("missing"),
            PtyCommand::new("aureline-terminal-command-that-does-not-exist"),
        );
        let session = host.session(&id).unwrap();
        assert_eq!(
            session.lifecycle_state(),
            SessionLifecycleState::Quarantined
        );
        let transitions = host.drain_transitions();
        assert!(transitions.iter().any(|transition| {
            transition.to_state == SessionLifecycleState::Quarantined
                && transition
                    .reason_code
                    .as_deref()
                    .is_some_and(|reason| reason == "pty_spawn_failed")
        }));
    }

    #[test]
    fn lifecycle_walks_through_starting_active_lost_reconnect() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.update_cwd_hint(&id, Some("~/code/aureline/crates"), "mono:3")
            .unwrap();
        let before_loss = host.session(&id).unwrap().header().clone();
        host.mark_lost_transport(&id, "mono:4", Some("network_drop"))
            .unwrap();
        host.mark_reconnected_same_identity(&id, "mono:5").unwrap();
        host.mark_active(&id, "mono:6").unwrap();
        host.close(&id, "mono:7", Some("user_closed")).unwrap();

        let session = host.session(&id).expect("session must exist");
        assert_eq!(session.lifecycle_state(), SessionLifecycleState::Closed);
        assert_eq!(session.header().session_id, before_loss.session_id);
        assert_eq!(session.header().workspace_id, before_loss.workspace_id);
        assert_eq!(
            session.header().execution_context_ref,
            before_loss.execution_context_ref
        );
        assert_eq!(session.header().target_badge, before_loss.target_badge);
        assert_eq!(
            session.header().boundary_cue_token,
            before_loss.boundary_cue_token
        );
        assert_eq!(
            session.header().cwd_hint.as_deref(),
            Some("~/code/aureline/crates"),
            "cwd hint preserved across transitions"
        );

        let transitions = host.drain_transitions();
        let walks: Vec<_> = transitions
            .iter()
            .map(|t| (t.from_state, t.to_state))
            .collect();
        assert!(walks.contains(&(
            SessionLifecycleState::Active,
            SessionLifecycleState::LostTransport
        )));
        assert!(walks.contains(&(
            SessionLifecycleState::LostTransport,
            SessionLifecycleState::ReconnectedSameIdentity
        )));
    }

    #[test]
    fn lost_transport_preserves_provenance_for_failure_drill() {
        // Failure drill: terminate transport unexpectedly. The header MUST
        // remain attached so the pane never collapses to an anonymous tab.
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.mark_lost_transport(&id, "mono:3", Some("network_drop"))
            .unwrap();

        let session = host.session(&id).expect("session must exist");
        assert_eq!(
            session.lifecycle_state(),
            SessionLifecycleState::LostTransport
        );
        assert_eq!(session.header().display_title, "zsh");
        assert_eq!(
            session.header().cwd_hint.as_deref(),
            Some("~/code/aureline")
        );
        assert_eq!(session.header().target_badge, "Local");
        assert_eq!(
            session.header().boundary_cue_token,
            "boundary_cue_local_session"
        );
        assert!(session.header().is_degraded());
    }

    #[test]
    fn quarantine_keeps_header_and_blocks_silent_recovery() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.quarantine(&id, "mono:3", "terminal_protocol_violation_budget_exceeded")
            .unwrap();

        let session = host.session(&id).expect("session must exist");
        assert_eq!(
            session.lifecycle_state(),
            SessionLifecycleState::Quarantined
        );
        assert!(session.header().is_degraded());

        // A quarantined session refuses to silently re-attach; the user must
        // open a fresh session through the command-dispatch boundary.
        let err = host.mark_active(&id, "mono:4").unwrap_err();
        assert!(matches!(err, PtyHostError::InvalidTransition { .. }));
    }

    #[test]
    fn remote_session_emits_visible_boundary_cue() {
        let mut host = PtyHost::new();
        let id = host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::RemoteAgentPrimary,
            display_title: "agent shell",
            cwd_hint: Some("/srv/code"),
            execution_context_ref: "execution_context.remote_agent.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        });
        let session = host.session(&id).expect("session must exist");
        assert_eq!(session.header().target_badge, "Remote");
        assert_eq!(
            session.header().boundary_cue_token,
            "boundary_cue_remote_session"
        );
        assert!(session.header().needs_boundary_cue());
    }

    #[test]
    fn ordering_is_stable_across_lifecycle_transitions() {
        let mut host = PtyHost::new();
        let a = open_local(&mut host);
        let b = host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::HostDesktop,
            display_title: "bash",
            cwd_hint: None,
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:1",
        });
        host.mark_starting(&a, "mono:2").unwrap();
        host.mark_active(&a, "mono:3").unwrap();
        host.mark_lost_transport(&a, "mono:4", None).unwrap();

        let order: Vec<_> = host.sessions().map(|s| s.session_id().clone()).collect();
        assert_eq!(order, vec![a, b]);
    }

    #[test]
    fn unknown_session_returns_error() {
        let mut host = PtyHost::new();
        let bogus = PtySessionId::from_parts("ws-other", HostClass::HostDesktop, 99);
        let err = host.mark_active(&bogus, "mono:0").unwrap_err();
        assert_eq!(err, PtyHostError::UnknownSession(bogus));
    }
}
