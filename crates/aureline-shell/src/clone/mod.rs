// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Reviewed, fail-closed repository acquisition through the system Git binary.
//!
//! [`CloneRequest`] is preview-only input. It cannot start a process. A caller
//! must bind the normalized source and destination to review, checkout-plan,
//! transport-policy, and authority records with [`CloneRequest::approve`]. The
//! resulting [`ApprovedCloneExecution`] is deliberately non-cloneable and is
//! consumed by [`GitCloneBackend::clone_repository`].
//!
//! Acquisition never runs repository hooks, filters, LFS smudging, submodule
//! recursion, dependency restoration, or trust admission. Meaningful partial
//! acquisitions are preserved behind a typed recovery handle instead of being
//! silently deleted.

use serde::{Deserialize, Serialize};
use std::collections::hash_map::RandomState;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::hash::{BuildHasher, Hasher};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MAX_REMOTE_BYTES: usize = 4 * 1024;
const MAX_DESTINATION_BYTES: usize = 4 * 1024;
const MAX_REFERENCE_BYTES: usize = 512;
const MAX_RECORD_REF_BYTES: usize = 512;
const MAX_GIT_VERSION_BYTES: usize = 256;
const MAX_COMMAND_STDOUT_BYTES: usize = 4 * 1024;
const MAX_PROGRESS_EVENTS: usize = 128;
const MAX_PROGRESS_LINE_BYTES: usize = 4 * 1024;
const MAX_PUBLIC_MESSAGE_BYTES: usize = 256;
const OUTPUT_CHANNEL_SLOTS: usize = 16;
const SUPERVISOR_POLL_INTERVAL: Duration = Duration::from_millis(10);
const POST_EXIT_DRAIN_GRACE: Duration = Duration::from_millis(500);
const PROBE_OVERALL_TIMEOUT: Duration = Duration::from_secs(10);
const PROBE_IDLE_TIMEOUT: Duration = Duration::from_secs(5);
const MIN_GIT_VERSION: (u32, u32, u32) = (2, 30, 0);
const MAX_OVERALL_TIMEOUT_MS: u64 = 24 * 60 * 60 * 1_000;
const MAX_IDLE_TIMEOUT_MS: u64 = 30 * 60 * 1_000;

static GUARD_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Raw clone sheet input. This record is safe to serialize for preview but is
/// not executable authority.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneRequest {
    /// Remote locator entered by the user. Embedded credentials are rejected.
    pub remote_url: String,
    /// Requested destination presentation path.
    pub destination_path: PathBuf,
}

impl fmt::Debug for CloneRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneRequest")
            .field("remote_url", &"<redacted-locator>")
            .field("destination_path", &"<redacted-path>")
            .finish()
    }
}

impl CloneRequest {
    /// Builds preview-only input from a sheet or command argument map.
    pub fn new(remote_url: impl Into<String>, destination_path: impl Into<PathBuf>) -> Self {
        Self {
            remote_url: remote_url.into(),
            destination_path: destination_path.into(),
        }
    }

    /// Resolves immutable facts that every approving surface must render and
    /// bind before execution.
    ///
    /// # Errors
    ///
    /// Returns a typed error for unsupported locators, ambiguous paths,
    /// unavailable local sources or parents, and destination collisions.
    pub fn review_facts(&self) -> Result<CloneReviewFacts, CloneError> {
        let locator = ValidatedLocator::parse(&self.remote_url)?;
        let destination = DestinationReview::inspect(&self.destination_path)?;
        Ok(CloneReviewFacts {
            transport: locator.transport,
            normalized_source: locator.presentation,
            canonical_local_source: locator
                .local_source
                .as_ref()
                .map(|source| source.path.clone()),
            canonical_destination_parent: destination.canonical_parent,
            destination_leaf_name: destination.leaf_name,
        })
    }

    /// Validates preview input without minting execution authority.
    ///
    /// # Errors
    ///
    /// Returns the same validation failures as [`Self::review_facts`].
    pub fn validate(&self) -> Result<(), CloneError> {
        self.review_facts().map(|_| ())
    }

    /// Binds this preview-only request to reviewed records and mints one
    /// non-cloneable execution capability.
    ///
    /// # Errors
    ///
    /// Fails closed when reviewed facts no longer match the filesystem or the
    /// requested authentication, topology, policy, or ref is unsupported.
    pub fn approve(self, approval: CloneApproval) -> Result<ApprovedCloneExecution, CloneError> {
        approval.validate_record_refs()?;
        approval.reference.validate()?;
        approval.topology.validate()?;
        approval.execution_policy.validate()?;

        let locator = ValidatedLocator::parse(&self.remote_url)?;
        if locator.transport != approval.reviewed_transport
            || locator.presentation != approval.reviewed_normalized_source
        {
            return policy_denied(
                "reviewed source transport or normalized locator does not match the clone request",
            );
        }
        let destination = ReviewedDestination::capture(
            &self.destination_path,
            &approval.reviewed_destination_parent,
        )?;
        if destination.target_path.file_name()
            != Some(approval.reviewed_destination_leaf_name.as_os_str())
        {
            return policy_denied("reviewed destination leaf does not match the clone request");
        }

        match (&locator.local_source, &approval.reviewed_local_source) {
            (Some(source), Some(reviewed)) if &source.path == reviewed => {}
            (Some(_), _) => {
                return policy_denied(
                    "reviewed local source identity does not match the clone request",
                );
            }
            (None, None) => {}
            (None, Some(_)) => {
                return policy_denied("a network locator cannot carry a local source binding");
            }
        }

        let transport_decision_ref = approval
            .transport_decision_ref
            .as_deref()
            .filter(|value| !value.is_empty());
        if locator.transport.is_network() && transport_decision_ref.is_none() {
            return policy_denied("network clone requires a reviewed transport decision");
        }

        let transport_options =
            ValidatedTransportOptions::capture(&approval.transport_options, locator.transport)?;
        let authentication =
            ValidatedAuthentication::capture(&approval.authentication, locator.transport)?;

        Ok(ApprovedCloneExecution {
            locator,
            destination,
            review_record_ref: approval.review_record_ref,
            source_locator_record_ref: approval.source_locator_record_ref,
            checkout_plan_record_ref: approval.checkout_plan_record_ref,
            policy_decision_ref: approval.policy_decision_ref,
            transport_decision_ref: approval.transport_decision_ref,
            reference: approval.reference,
            topology: approval.topology,
            authentication,
            transport_options,
            execution_policy: approval.execution_policy,
            post_clone_action: approval.post_clone_action,
            cancellation: CloneCancellationToken::new(),
        })
    }
}

/// Stable source transport rendered during clone review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloneTransportClass {
    LocalFilesystem,
    Https,
    Ssh,
    GitProtocol,
}

impl CloneTransportClass {
    /// Stable contract token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFilesystem => "local_filesystem",
            Self::Https => "https",
            Self::Ssh => "ssh",
            Self::GitProtocol => "git_protocol",
        }
    }

    pub const fn is_network(self) -> bool {
        !matches!(self, Self::LocalFilesystem)
    }
}

/// Canonical facts a review surface must disclose before approval.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneReviewFacts {
    pub transport: CloneTransportClass,
    pub normalized_source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_local_source: Option<PathBuf>,
    pub canonical_destination_parent: PathBuf,
    pub destination_leaf_name: OsString,
}

impl fmt::Debug for CloneReviewFacts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneReviewFacts")
            .field("transport", &self.transport)
            .field("source", &"<redacted-locator>")
            .field("destination", &"<redacted-path>")
            .finish()
    }
}

/// Exact reviewed ref and commit identity. The OID is required so a ref move
/// between review and materialization cannot silently change acquired bytes.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneRefSelection {
    /// `HEAD`, a branch name, a tag name, or a full `refs/...` name.
    pub reference: String,
    /// Reviewed SHA-1 or SHA-256 commit object ID.
    pub expected_commit_oid: String,
}

impl fmt::Debug for CloneRefSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneRefSelection")
            .field("reference", &"<redacted-ref>")
            .field("expected_commit_oid", &"<bound-oid>")
            .finish()
    }
}

impl CloneRefSelection {
    pub fn new(reference: impl Into<String>, expected_commit_oid: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            expected_commit_oid: expected_commit_oid.into(),
        }
    }

    fn validate(&self) -> Result<(), CloneError> {
        validate_git_reference(&self.reference)?;
        validate_commit_oid(&self.expected_commit_oid)
    }
}

/// Reviewed history depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloneHistoryPolicy {
    Full,
    Shallow { depth: u32 },
}

/// Explicit topology policy. Unsupported repository-controlled expansion is
/// represented rather than silently ignored, then rejected at approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneTopologyPolicy {
    pub history: CloneHistoryPolicy,
    pub partial_clone: bool,
    pub sparse_checkout: bool,
    pub recurse_submodules: bool,
    pub hydrate_lfs: bool,
}

impl CloneTopologyPolicy {
    /// Safe full-history acquisition with all deferred expansion disabled.
    pub const fn inert_full() -> Self {
        Self {
            history: CloneHistoryPolicy::Full,
            partial_clone: false,
            sparse_checkout: false,
            recurse_submodules: false,
            hydrate_lfs: false,
        }
    }

    fn validate(&self) -> Result<(), CloneError> {
        if matches!(self.history, CloneHistoryPolicy::Shallow { depth: 0 }) {
            return policy_denied("shallow clone depth must be greater than zero");
        }
        if self.partial_clone || self.sparse_checkout || self.recurse_submodules || self.hydrate_lfs
        {
            return policy_denied(
                "partial, sparse, submodule, and LFS expansion require a later reviewed action",
            );
        }
        Ok(())
    }
}

/// Authentication projection admitted by clone execution.
///
/// HTTPS credential helpers and raw PAT/OAuth values are intentionally absent.
/// Until the secret broker provides a narrow helper projection, private HTTPS
/// acquisition must remain blocked before approval.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum CloneAuthentication {
    Anonymous,
    SshAgent {
        authority_ticket_ref: String,
        authority_expires_at_unix_seconds: u64,
        ssh_auth_sock: PathBuf,
        known_hosts_file: PathBuf,
        ssh_binary: PathBuf,
    },
}

impl fmt::Debug for CloneAuthentication {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => formatter.write_str("Anonymous"),
            Self::SshAgent { .. } => formatter.write_str("SshAgent(<redacted-projection>)"),
        }
    }
}

/// Explicit route settings admitted by transport review. Ambient proxy and CA
/// variables are never inherited.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneTransportOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_bundle_path: Option<PathBuf>,
}

impl fmt::Debug for CloneTransportOptions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneTransportOptions")
            .field("proxy", &self.proxy_url.as_ref().map(|_| "<reviewed>"))
            .field(
                "ca_bundle",
                &self.ca_bundle_path.as_ref().map(|_| "<reviewed>"),
            )
            .finish()
    }
}

/// Supervision bounds admitted by review. Milliseconds keep the serializable
/// boundary stable and avoid a platform-specific duration representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneExecutionPolicy {
    pub overall_timeout_ms: u64,
    pub idle_timeout_ms: u64,
}

impl Default for CloneExecutionPolicy {
    fn default() -> Self {
        Self {
            overall_timeout_ms: 60 * 60 * 1_000,
            idle_timeout_ms: 5 * 60 * 1_000,
        }
    }
}

impl CloneExecutionPolicy {
    fn validate(self) -> Result<(), CloneError> {
        if self.overall_timeout_ms == 0
            || self.overall_timeout_ms > MAX_OVERALL_TIMEOUT_MS
            || self.idle_timeout_ms == 0
            || self.idle_timeout_ms > MAX_IDLE_TIMEOUT_MS
            || self.idle_timeout_ms > self.overall_timeout_ms
        {
            return policy_denied("clone execution deadline is outside supported bounds");
        }
        Ok(())
    }

    fn overall(self) -> Duration {
        Duration::from_millis(self.overall_timeout_ms)
    }

    fn idle(self) -> Duration {
        Duration::from_millis(self.idle_timeout_ms)
    }
}

/// Reviewed handoff after bytes are acquired. None of these values grants
/// trust or runs setup inside this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClonePostAction {
    CloneOnly,
    ReviewTrustAndOpen,
    ReviewAndAddRoot,
}

/// Serializable record binding preview facts to owning review and policy
/// artifacts. It is evidence input, not executable authority by itself.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneApproval {
    pub review_record_ref: String,
    pub source_locator_record_ref: String,
    pub checkout_plan_record_ref: String,
    pub policy_decision_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_decision_ref: Option<String>,
    /// Transport rendered and approved by the review surface.
    pub reviewed_transport: CloneTransportClass,
    /// Credential-free normalized source rendered by the review surface.
    pub reviewed_normalized_source: String,
    pub reviewed_destination_parent: PathBuf,
    pub reviewed_destination_leaf_name: OsString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_local_source: Option<PathBuf>,
    pub reference: CloneRefSelection,
    pub topology: CloneTopologyPolicy,
    pub authentication: CloneAuthentication,
    #[serde(default)]
    pub transport_options: CloneTransportOptions,
    pub execution_policy: CloneExecutionPolicy,
    pub post_clone_action: ClonePostAction,
}

impl fmt::Debug for CloneApproval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneApproval")
            .field("review_record_ref", &"<bound>")
            .field("source_locator_record_ref", &"<bound>")
            .field("checkout_plan_record_ref", &"<bound>")
            .field("policy_decision_ref", &"<bound>")
            .field(
                "transport_decision_ref",
                &self.transport_decision_ref.as_ref().map(|_| "<bound>"),
            )
            .field("reviewed_transport", &self.reviewed_transport)
            .field("reviewed_source", &"<redacted-locator>")
            .field("reviewed_destination", &"<redacted-path>")
            .field("reference", &self.reference)
            .field("topology", &self.topology)
            .field("authentication", &self.authentication)
            .field("transport_options", &self.transport_options)
            .field("execution_policy", &self.execution_policy)
            .field("post_clone_action", &self.post_clone_action)
            .finish()
    }
}

impl CloneApproval {
    fn validate_record_refs(&self) -> Result<(), CloneError> {
        for (value, label) in [
            (&self.review_record_ref, "clone review record"),
            (&self.source_locator_record_ref, "source locator record"),
            (&self.checkout_plan_record_ref, "checkout plan record"),
            (&self.policy_decision_ref, "policy decision"),
        ] {
            validate_record_ref(value, label)?;
        }
        if let Some(value) = &self.transport_decision_ref {
            validate_record_ref(value, "transport decision")?;
        }
        Ok(())
    }
}

/// Clone cancellation capability. A UI may retain this token while moving the
/// one-shot execution value to a worker.
#[derive(Clone, Default)]
pub struct CloneCancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl fmt::Debug for CloneCancellationToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneCancellationToken")
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

impl CloneCancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// Validated, one-shot clone authority. This type intentionally implements
/// neither `Clone` nor serialization.
pub struct ApprovedCloneExecution {
    locator: ValidatedLocator,
    destination: ReviewedDestination,
    review_record_ref: String,
    source_locator_record_ref: String,
    checkout_plan_record_ref: String,
    policy_decision_ref: String,
    transport_decision_ref: Option<String>,
    reference: CloneRefSelection,
    topology: CloneTopologyPolicy,
    authentication: ValidatedAuthentication,
    transport_options: ValidatedTransportOptions,
    execution_policy: CloneExecutionPolicy,
    post_clone_action: ClonePostAction,
    cancellation: CloneCancellationToken,
}

impl fmt::Debug for ApprovedCloneExecution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovedCloneExecution")
            .field("transport", &self.locator.transport)
            .field("destination", &"<redacted-path>")
            .field("reference", &self.reference)
            .field("topology", &self.topology)
            .field("authentication", &self.authentication)
            .field("post_clone_action", &self.post_clone_action)
            .finish_non_exhaustive()
    }
}

impl ApprovedCloneExecution {
    pub fn cancellation_token(&self) -> CloneCancellationToken {
        self.cancellation.clone()
    }

    pub fn destination_path(&self) -> &Path {
        &self.destination.presentation_path
    }

    pub fn reference(&self) -> &CloneRefSelection {
        &self.reference
    }

    fn verify_current_bindings(&self) -> Result<(), CloneError> {
        self.destination.verify_parent()?;
        if let Some(source) = &self.locator.local_source {
            source.verify("reviewed local source identity changed")?;
        }
        self.authentication.verify_current()?;
        self.transport_options.verify_current()?;
        Ok(())
    }
}

/// Startup probe result for the pinned system Git executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitProbe {
    pub version_line: String,
}

/// Typed clone failure classes surfaced to command results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloneErrorClass {
    GitNotInstalled,
    GitVersionUnsupported,
    InvalidInput,
    DestinationExists,
    PolicyDenied,
    Auth,
    RemoteNotFound,
    Network,
    Tls,
    HostKey,
    DiskFull,
    Filesystem,
    RefMismatch,
    Timeout,
    Cancelled,
    OutputLimit,
    GitExited,
    Io,
}

impl CloneErrorClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitNotInstalled => "git_not_installed",
            Self::GitVersionUnsupported => "git_version_unsupported",
            Self::InvalidInput => "invalid_input",
            Self::DestinationExists => "destination_exists",
            Self::PolicyDenied => "policy_denied",
            Self::Auth => "auth",
            Self::RemoteNotFound => "remote_not_found",
            Self::Network => "network",
            Self::Tls => "tls",
            Self::HostKey => "host_key",
            Self::DiskFull => "disk_full",
            Self::Filesystem => "filesystem",
            Self::RefMismatch => "ref_mismatch",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::OutputLimit => "output_limit",
            Self::GitExited => "git_exited",
            Self::Io => "io",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::GitNotInstalled => "Git not installed",
            Self::GitVersionUnsupported => "Git version unsupported",
            Self::InvalidInput => "Invalid input",
            Self::DestinationExists => "Destination exists",
            Self::PolicyDenied => "Policy blocked clone",
            Self::Auth => "Authentication failed",
            Self::RemoteNotFound => "Remote not found",
            Self::Network => "Network failed",
            Self::Tls => "TLS verification failed",
            Self::HostKey => "Host key failed",
            Self::DiskFull => "Disk full",
            Self::Filesystem => "Filesystem failed",
            Self::RefMismatch => "Reviewed ref changed",
            Self::Timeout => "Clone timed out",
            Self::Cancelled => "Clone cancelled",
            Self::OutputLimit => "Git output limit exceeded",
            Self::GitExited => "Git failed",
            Self::Io => "I/O failed",
        }
    }
}

/// Bounded public clone error. Construction is crate-private so raw process
/// output cannot be promoted into a presentation or audit boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloneError {
    pub(crate) class: CloneErrorClass,
    pub(crate) message: String,
}

impl CloneError {
    pub(crate) fn new(class: CloneErrorClass, message: impl AsRef<str>) -> Self {
        Self {
            class,
            message: bounded_public_message(message.as_ref()),
        }
    }

    pub const fn class(&self) -> CloneErrorClass {
        self.class
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn append_static_context(&mut self, suffix: &'static str) {
        let combined = format!("{}. {suffix}", self.message.trim_end_matches('.'));
        self.message = bounded_public_message(&combined);
    }
}

impl fmt::Display for CloneError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.class.as_str(), self.message)
    }
}

impl std::error::Error for CloneError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloneProgressPhase {
    Starting,
    Progress,
    Verifying,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloneProgressEvent {
    pub phase: CloneProgressPhase,
    pub message: String,
}

impl CloneProgressEvent {
    pub(crate) fn new(phase: CloneProgressPhase, message: &'static str) -> Self {
        debug_assert!(message.len() <= MAX_PUBLIC_MESSAGE_BYTES);
        Self {
            phase,
            message: message.to_string(),
        }
    }
}

fn emit_progress_event(
    progress: &mut dyn FnMut(CloneProgressEvent),
    event: CloneProgressEvent,
) -> Result<(), CloneError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| progress(event))).map_err(|_| {
        CloneError::new(
            CloneErrorClass::Io,
            "clone progress observer stopped unexpectedly",
        )
    })
}

/// Successful acquisition receipt. Trust and setup remain pending.
#[derive(Clone, PartialEq, Eq)]
pub struct CloneOutcome {
    pub destination_path: PathBuf,
    pub materialized_commit_oid: String,
    pub source_locator_record_ref: String,
    pub checkout_plan_record_ref: String,
    pub review_record_ref: String,
    pub policy_decision_ref: String,
    pub transport_decision_ref: Option<String>,
    pub post_clone_action: ClonePostAction,
    pub trust_and_setup_pending: bool,
}

impl fmt::Debug for CloneOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneOutcome")
            .field("destination_path", &"<redacted-path>")
            .field("materialized_commit_oid", &self.materialized_commit_oid)
            .field("trust_and_setup_pending", &self.trust_and_setup_pending)
            .finish_non_exhaustive()
    }
}

/// Stable interrupted-acquisition state used by recovery UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloneInterruptedState {
    NoPartialBytes,
    InterruptedResumable,
    InterruptedDiscardRequired,
    InterruptedOpenReadOnlyAvailable,
}

impl CloneInterruptedState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPartialBytes => "no_partial_bytes",
            Self::InterruptedResumable => "interrupted_resumable",
            Self::InterruptedDiscardRequired => "interrupted_discard_required",
            Self::InterruptedOpenReadOnlyAvailable => "interrupted_open_read_only_available",
        }
    }
}

/// Owned partial destination. It is preserved by default and can only be
/// discarded through an identity-checked, quarantine-first operation.
pub struct ClonePartialAcquisition {
    path: PathBuf,
    identity: FilesystemIdentity,
    state: CloneInterruptedState,
}

impl fmt::Debug for ClonePartialAcquisition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClonePartialAcquisition")
            .field("path", &"<redacted-path>")
            .field("state", &self.state)
            .finish()
    }
}

impl ClonePartialAcquisition {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn state(&self) -> CloneInterruptedState {
        self.state
    }

    /// Discards the preserved partial acquisition after moving the exact owned
    /// directory to a private sibling quarantine path.
    ///
    /// # Errors
    ///
    /// Fails without recursively deleting when identity changed or quarantine
    /// could not be verified.
    pub fn discard(mut self) -> Result<(), CloneError> {
        quarantine_and_remove(&mut self.path, &self.identity)
    }
}

/// Clone failure plus recovery truth. This type is not cloneable because its
/// optional partial handle is unique cleanup authority.
pub struct CloneFailure {
    pub(crate) error: CloneError,
    pub(crate) interrupted_state: CloneInterruptedState,
    partial: Option<ClonePartialAcquisition>,
}

impl fmt::Debug for CloneFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloneFailure")
            .field("error", &self.error)
            .field("interrupted_state", &self.interrupted_state)
            .field("partial", &self.partial.as_ref().map(|_| "<owned-partial>"))
            .finish()
    }
}

impl fmt::Display for CloneFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for CloneFailure {}

impl CloneFailure {
    fn without_partial(error: CloneError) -> Self {
        Self {
            error,
            interrupted_state: CloneInterruptedState::NoPartialBytes,
            partial: None,
        }
    }

    pub fn error(&self) -> &CloneError {
        &self.error
    }

    pub const fn interrupted_state(&self) -> CloneInterruptedState {
        self.interrupted_state
    }

    pub fn partial(&self) -> Option<&ClonePartialAcquisition> {
        self.partial.as_ref()
    }

    pub fn into_partial(self) -> Option<ClonePartialAcquisition> {
        self.partial
    }
}

/// Backend abstraction. Raw [`CloneRequest`] is intentionally absent from the
/// mutation method signature.
pub trait GitCloneBackend {
    fn probe(&self) -> Result<GitProbe, CloneError>;

    fn clone_repository(
        &self,
        execution: ApprovedCloneExecution,
        progress: &mut dyn FnMut(CloneProgressEvent),
    ) -> Result<CloneOutcome, CloneFailure>;
}

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
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }

    fn prepare_git(
        &self,
        overall_timeout: Duration,
        idle_timeout: Duration,
        cancellation: &CloneCancellationToken,
    ) -> Result<PinnedExecutable, CloneError> {
        let executable = PinnedExecutable::resolve(&self.git_binary, "Git binary was not found")?;
        let mut command = Command::new(&executable.path);
        configure_probe_environment(&mut command, &executable.path);
        command
            .arg("--version")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_process_group(&mut command);
        let output = run_supervised(
            &mut command,
            overall_timeout,
            idle_timeout,
            cancellation,
            MAX_GIT_VERSION_BYTES,
            None,
        )?;
        if !output.status.success() {
            return Err(CloneError::new(
                CloneErrorClass::GitNotInstalled,
                "Git version probe did not complete successfully",
            ));
        }
        let version_line = sanitize_text(&output.stdout, MAX_GIT_VERSION_BYTES);
        let version = parse_git_version(&version_line).ok_or_else(|| {
            CloneError::new(
                CloneErrorClass::GitVersionUnsupported,
                "Git version could not be verified against the supported minimum",
            )
        })?;
        if version < MIN_GIT_VERSION {
            return Err(CloneError::new(
                CloneErrorClass::GitVersionUnsupported,
                "Git version is older than the supported acquisition minimum",
            ));
        }
        executable.verify("Git executable identity changed during probe")?;
        Ok(executable)
    }
}

impl GitCloneBackend for SystemGitCloneBackend {
    fn probe(&self) -> Result<GitProbe, CloneError> {
        let cancellation = CloneCancellationToken::new();
        let executable =
            self.prepare_git(PROBE_OVERALL_TIMEOUT, PROBE_IDLE_TIMEOUT, &cancellation)?;
        let mut command = Command::new(&executable.path);
        configure_probe_environment(&mut command, &executable.path);
        command
            .arg("--version")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_process_group(&mut command);
        let output = run_supervised(
            &mut command,
            PROBE_OVERALL_TIMEOUT,
            PROBE_IDLE_TIMEOUT,
            &cancellation,
            MAX_GIT_VERSION_BYTES,
            None,
        )?;
        if !output.status.success() {
            return Err(CloneError::new(
                CloneErrorClass::GitNotInstalled,
                "Git version probe did not complete successfully",
            ));
        }
        Ok(GitProbe {
            version_line: sanitize_text(&output.stdout, MAX_GIT_VERSION_BYTES),
        })
    }

    fn clone_repository(
        &self,
        execution: ApprovedCloneExecution,
        progress: &mut dyn FnMut(CloneProgressEvent),
    ) -> Result<CloneOutcome, CloneFailure> {
        let deadline = OperationDeadline::new(execution.execution_policy.overall());
        if execution.cancellation.is_cancelled() {
            return Err(CloneFailure::without_partial(CloneError::new(
                CloneErrorClass::Cancelled,
                "Repository acquisition was cancelled before it started",
            )));
        }
        execution
            .verify_current_bindings()
            .map_err(CloneFailure::without_partial)?;
        let git = self
            .prepare_git(
                deadline
                    .remaining()
                    .map_err(CloneFailure::without_partial)?,
                execution.execution_policy.idle(),
                &execution.cancellation,
            )
            .map_err(CloneFailure::without_partial)?;
        git.verify("Git executable identity changed before acquisition")
            .map_err(CloneFailure::without_partial)?;

        let mut destination = OwnedDestination::create(&execution.destination)
            .map_err(CloneFailure::without_partial)?;
        let mut guard = match GitGuard::create(&execution.destination.canonical_parent) {
            Ok(guard) => guard,
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::Reserved,
                    destination,
                    None,
                ))
            }
        };

        if let Err(error) = emit_progress_event(
            progress,
            CloneProgressEvent::new(
                CloneProgressPhase::Starting,
                "Starting reviewed repository acquisition; trust and setup remain unchanged",
            ),
        ) {
            return Err(finish_failure(
                error,
                AcquisitionPhase::Reserved,
                destination,
                Some(guard),
            ));
        }
        let mut progress_state = ProgressState::default();

        let mut clone_command =
            match build_clone_command(&git, &execution, &destination.path, &guard) {
                Ok(command) => command,
                Err(error) => {
                    return Err(finish_failure(
                        error,
                        AcquisitionPhase::Reserved,
                        destination,
                        Some(guard),
                    ));
                }
            };
        let clone_remaining = match deadline.remaining() {
            Ok(remaining) => remaining,
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::Reserved,
                    destination,
                    Some(guard),
                ));
            }
        };
        let clone_output = run_supervised(
            &mut clone_command,
            clone_remaining,
            execution.execution_policy.idle(),
            &execution.cancellation,
            MAX_COMMAND_STDOUT_BYTES,
            Some(ProgressSink {
                state: &mut progress_state,
                callback: progress,
            }),
        );
        let clone_output = match clone_output {
            Ok(output) if output.status.success() => output,
            Ok(output) => {
                return Err(finish_failure(
                    classify_git_failure(&output.diagnostics, output.status.code()),
                    AcquisitionPhase::Fetching,
                    destination,
                    Some(guard),
                ));
            }
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::Fetching,
                    destination,
                    Some(guard),
                ));
            }
        };
        let _ = clone_output;

        if let Err(error) = verify_owned_repository(&destination, None) {
            return Err(finish_failure(
                error,
                AcquisitionPhase::Fetching,
                destination,
                Some(guard),
            ));
        }
        let git_directory_path = destination.path.join(".git");
        let git_directory_identity = match FilesystemIdentity::capture(&git_directory_path) {
            Ok(identity) if identity.kind == FilesystemObjectKind::Directory => identity,
            _ => {
                return Err(finish_failure(
                    CloneError::new(
                        CloneErrorClass::Filesystem,
                        "Git metadata directory identity could not be captured",
                    ),
                    AcquisitionPhase::Fetching,
                    destination,
                    Some(guard),
                ));
            }
        };

        if let Err(error) = emit_progress_event(
            progress,
            CloneProgressEvent::new(
                CloneProgressPhase::Verifying,
                "Verifying the reviewed commit identity",
            ),
        ) {
            return Err(finish_failure(
                error,
                AcquisitionPhase::Verifying,
                destination,
                Some(guard),
            ));
        }
        let observed_oid = match verify_materialized_oid(
            &git,
            &execution,
            &destination.path,
            &git_directory_identity,
            &guard,
            &deadline,
        ) {
            Ok(oid) => oid,
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::Verifying,
                    destination,
                    Some(guard),
                ));
            }
        };

        if let Err(error) = git_directory_identity.verify(
            &git_directory_path,
            "Git metadata directory identity changed before checkout",
        ) {
            return Err(finish_failure(
                error,
                AcquisitionPhase::Verifying,
                destination,
                Some(guard),
            ));
        }
        let mut checkout_command =
            match build_checkout_command(&git, &execution, &destination.path, &guard) {
                Ok(command) => command,
                Err(error) => {
                    return Err(finish_failure(
                        error,
                        AcquisitionPhase::Verifying,
                        destination,
                        Some(guard),
                    ));
                }
            };
        let checkout_remaining = match deadline.remaining() {
            Ok(remaining) => remaining,
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::Verifying,
                    destination,
                    Some(guard),
                ));
            }
        };
        let checkout_output = run_supervised(
            &mut checkout_command,
            checkout_remaining,
            execution.execution_policy.idle(),
            &execution.cancellation,
            MAX_COMMAND_STDOUT_BYTES,
            Some(ProgressSink {
                state: &mut progress_state,
                callback: progress,
            }),
        );
        match checkout_output {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                return Err(finish_failure(
                    classify_git_failure(&output.diagnostics, output.status.code()),
                    AcquisitionPhase::CheckingOut,
                    destination,
                    Some(guard),
                ));
            }
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::CheckingOut,
                    destination,
                    Some(guard),
                ));
            }
        }

        if let Err(error) = verify_owned_repository(&destination, Some(&git_directory_identity)) {
            return Err(finish_failure(
                error,
                AcquisitionPhase::CheckingOut,
                destination,
                Some(guard),
            ));
        }
        let post_checkout_oid = match verify_materialized_oid(
            &git,
            &execution,
            &destination.path,
            &git_directory_identity,
            &guard,
            &deadline,
        ) {
            Ok(oid) => oid,
            Err(error) => {
                return Err(finish_failure(
                    error,
                    AcquisitionPhase::CheckingOut,
                    destination,
                    Some(guard),
                ));
            }
        };
        if observed_oid != post_checkout_oid {
            return Err(finish_failure(
                CloneError::new(
                    CloneErrorClass::RefMismatch,
                    "materialized commit identity changed during checkout",
                ),
                AcquisitionPhase::CheckingOut,
                destination,
                Some(guard),
            ));
        }

        if let Err(error) = guard.cleanup() {
            return Err(finish_failure(
                error,
                AcquisitionPhase::CheckingOut,
                destination,
                None,
            ));
        }
        if destination.restore_permissions().is_err() {
            return Err(finish_failure(
                CloneError::new(
                    CloneErrorClass::Filesystem,
                    "destination permissions could not be restored safely",
                ),
                AcquisitionPhase::CheckingOut,
                destination,
                None,
            ));
        }

        if let Err(error) = emit_progress_event(
            progress,
            CloneProgressEvent::new(
                CloneProgressPhase::Completed,
                "Repository acquired; trust, setup, LFS, and submodules remain pending",
            ),
        ) {
            return Err(finish_failure(
                error,
                AcquisitionPhase::CheckingOut,
                destination,
                None,
            ));
        }
        destination.commit();
        Ok(CloneOutcome {
            destination_path: execution.destination.target_path,
            materialized_commit_oid: post_checkout_oid,
            source_locator_record_ref: execution.source_locator_record_ref,
            checkout_plan_record_ref: execution.checkout_plan_record_ref,
            review_record_ref: execution.review_record_ref,
            policy_decision_ref: execution.policy_decision_ref,
            transport_decision_ref: execution.transport_decision_ref,
            post_clone_action: execution.post_clone_action,
            trust_and_setup_pending: true,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AcquisitionPhase {
    Reserved,
    Fetching,
    Verifying,
    CheckingOut,
}

fn finish_failure(
    mut error: CloneError,
    phase: AcquisitionPhase,
    mut destination: OwnedDestination,
    mut guard: Option<GitGuard>,
) -> CloneFailure {
    if let Some(guard) = guard.as_mut() {
        if guard.cleanup().is_err() {
            error.append_static_context("Clone guard cleanup requires inspection");
        }
    }

    let meaningful = destination.has_meaningful_git_state();
    if meaningful {
        let read_only_available = matches!(phase, AcquisitionPhase::CheckingOut)
            && destination.has_worktree_materialization();
        let state = if read_only_available {
            CloneInterruptedState::InterruptedOpenReadOnlyAvailable
        } else {
            CloneInterruptedState::InterruptedResumable
        };
        let _ = destination.restore_permissions();
        let partial = destination.preserve(state);
        return CloneFailure {
            error,
            interrupted_state: state,
            partial: Some(partial),
        };
    }

    match destination.rollback() {
        Ok(()) => CloneFailure::without_partial(error),
        Err(()) => {
            error.append_static_context("Fresh destination could not be discarded safely");
            let partial = destination.preserve(CloneInterruptedState::InterruptedDiscardRequired);
            CloneFailure {
                error,
                interrupted_state: CloneInterruptedState::InterruptedDiscardRequired,
                partial: Some(partial),
            }
        }
    }
}

fn verify_owned_repository(
    destination: &OwnedDestination,
    expected_git_identity: Option<&FilesystemIdentity>,
) -> Result<(), CloneError> {
    if !destination.is_same_directory() || !is_real_directory(&destination.path.join(".git")) {
        return Err(CloneError::new(
            CloneErrorClass::Filesystem,
            "Git did not leave a safely owned repository destination",
        ));
    }
    if let Some(identity) = expected_git_identity {
        identity.verify(
            &destination.path.join(".git"),
            "Git metadata directory identity changed during acquisition",
        )?;
    }
    for borrowed_store in [
        ".git/objects/info/alternates",
        ".git/objects/info/http-alternates",
    ] {
        match fs::symlink_metadata(destination.path.join(borrowed_store)) {
            Ok(_) => {
                return policy_denied(
                    "acquired repository cannot borrow from an external object store",
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => {
                return Err(CloneError::new(
                    CloneErrorClass::Filesystem,
                    "repository object-store isolation could not be verified",
                ));
            }
        }
    }
    Ok(())
}

fn verify_materialized_oid(
    git: &PinnedExecutable,
    execution: &ApprovedCloneExecution,
    destination: &Path,
    git_directory_identity: &FilesystemIdentity,
    guard: &GitGuard,
    deadline: &OperationDeadline,
) -> Result<String, CloneError> {
    execution.verify_current_bindings()?;
    git.verify("Git executable identity changed during acquisition")?;
    git_directory_identity.verify(
        &destination.join(".git"),
        "Git metadata directory identity changed during verification",
    )?;
    let mut command = build_verify_command(git, execution, destination, guard)?;
    let output = run_supervised(
        &mut command,
        deadline.remaining()?,
        execution.execution_policy.idle(),
        &execution.cancellation,
        MAX_COMMAND_STDOUT_BYTES,
        None,
    )?;
    if !output.status.success() {
        return Err(classify_git_failure(
            &output.diagnostics,
            output.status.code(),
        ));
    }
    let observed = sanitize_text(&output.stdout, MAX_COMMAND_STDOUT_BYTES).to_ascii_lowercase();
    validate_commit_oid(&observed).map_err(|_| {
        CloneError::new(
            CloneErrorClass::RefMismatch,
            "Git did not report a valid materialized commit identity",
        )
    })?;
    if observed != execution.reference.expected_commit_oid.to_ascii_lowercase() {
        return Err(CloneError::new(
            CloneErrorClass::RefMismatch,
            "remote ref no longer matches the reviewed commit identity",
        ));
    }
    Ok(observed)
}

#[derive(Debug, Clone, Copy)]
struct OperationDeadline {
    started_at: Instant,
    overall: Duration,
}

impl OperationDeadline {
    fn new(overall: Duration) -> Self {
        Self {
            started_at: Instant::now(),
            overall,
        }
    }

    fn remaining(self) -> Result<Duration, CloneError> {
        self.overall
            .checked_sub(self.started_at.elapsed())
            .filter(|remaining| !remaining.is_zero())
            .ok_or_else(|| {
                CloneError::new(
                    CloneErrorClass::Timeout,
                    "Repository acquisition exceeded its reviewed overall deadline",
                )
            })
    }
}

// Locator and review binding -------------------------------------------------

#[derive(Clone)]
struct ValidatedLocator {
    transport: CloneTransportClass,
    argument: OsString,
    presentation: String,
    local_source: Option<PinnedFilesystemObject>,
}

impl fmt::Debug for ValidatedLocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidatedLocator")
            .field("transport", &self.transport)
            .field("argument", &"<redacted-locator>")
            .finish()
    }
}

impl ValidatedLocator {
    fn parse(remote: &str) -> Result<Self, CloneError> {
        validate_locator_text(remote)?;
        if looks_like_remote_helper_route(remote) {
            return invalid_input("Git remote-helper routes are not supported");
        }

        if let Some((scheme, remainder)) = remote.split_once("://") {
            return match scheme {
                "https" => parse_network_url(CloneTransportClass::Https, scheme, remainder),
                "ssh" | "git+ssh" | "ssh+git" => {
                    parse_network_url(CloneTransportClass::Ssh, scheme, remainder)
                }
                "git" => parse_network_url(CloneTransportClass::GitProtocol, scheme, remainder),
                "file" => parse_file_url(remainder),
                "http" => invalid_input("unencrypted HTTP clone routes are not supported"),
                _ => invalid_input("remote URL scheme is not supported"),
            };
        }

        if looks_like_scp_route(remote)? {
            validate_scp_route(remote)?;
            return Ok(Self {
                transport: CloneTransportClass::Ssh,
                argument: OsString::from(remote),
                presentation: remote.to_string(),
                local_source: None,
            });
        }

        parse_local_path(Path::new(remote))
    }
}

fn validate_locator_text(remote: &str) -> Result<(), CloneError> {
    if remote.is_empty() {
        return invalid_input("remote URL or local source is required");
    }
    if remote.trim() != remote {
        return invalid_input("remote locator cannot start or end with whitespace");
    }
    if remote.len() > MAX_REMOTE_BYTES {
        return invalid_input("remote locator is too long");
    }
    if remote.chars().any(char::is_control) {
        return invalid_input("remote locator contains unsupported control characters");
    }
    if remote.starts_with('-') {
        return invalid_input("remote locator cannot be a command-line option");
    }
    Ok(())
}

fn parse_network_url(
    transport: CloneTransportClass,
    scheme: &str,
    remainder: &str,
) -> Result<ValidatedLocator, CloneError> {
    if remainder.is_empty() || remainder.contains(['?', '#']) {
        return invalid_input("remote URL is incomplete or contains unsupported query data");
    }
    let (authority, path) = remainder.split_once('/').unwrap_or((remainder, ""));
    if authority.is_empty() || path.is_empty() {
        return invalid_input("remote URL must include a host and repository path");
    }
    if path.chars().any(char::is_whitespace) || path.contains('\\') {
        return invalid_input("remote URL repository path is not normalized");
    }
    validate_authority(authority, transport == CloneTransportClass::Ssh)?;
    if transport == CloneTransportClass::Ssh {
        validate_ssh_repository_path(path)?;
    }
    let normalized = format!("{scheme}://{authority}/{path}");
    Ok(ValidatedLocator {
        transport,
        argument: OsString::from(&normalized),
        presentation: normalized,
        local_source: None,
    })
}

fn validate_authority(authority: &str, allow_user: bool) -> Result<(), CloneError> {
    if authority
        .chars()
        .any(|character| character.is_whitespace() || "'\"`$;&|<>(){}\\%".contains(character))
    {
        return invalid_input("remote URL authority is invalid");
    }
    if authority.matches('@').count() > usize::from(allow_user) {
        return invalid_input("embedded remote credentials are not supported");
    }
    let host_port = if let Some((user, host)) = authority.split_once('@') {
        if !allow_user || user.is_empty() || user.contains(':') || user.starts_with('-') {
            return invalid_input("remote URL user is invalid");
        }
        host
    } else {
        authority
    };
    validate_host_port(host_port)
}

fn validate_host_port(host_port: &str) -> Result<(), CloneError> {
    if host_port.is_empty() || host_port.starts_with('-') {
        return invalid_input("remote URL host is invalid");
    }
    if !host_port.is_ascii() {
        return invalid_input("remote host must be normalized to ASCII IDNA before review");
    }
    if host_port.starts_with('[') {
        let Some(close) = host_port.find(']') else {
            return invalid_input("remote URL contains an unmatched IPv6 bracket");
        };
        if close == 1 || host_port[close + 1..].contains(['[', ']']) {
            return invalid_input("remote URL IPv6 host is invalid");
        }
        if host_port[1..close].parse::<std::net::Ipv6Addr>().is_err() {
            return invalid_input("remote URL IPv6 host is invalid");
        }
        let suffix = &host_port[close + 1..];
        if !suffix.is_empty() {
            validate_port_suffix(suffix)?;
        }
    } else {
        if host_port.contains(['[', ']']) || host_port.matches(':').count() > 1 {
            return invalid_input("remote URL host or port is invalid");
        }
        if let Some((host, port)) = host_port.rsplit_once(':') {
            if host.is_empty() {
                return invalid_input("remote URL host is invalid");
            }
            validate_port_suffix(&format!(":{port}"))?;
            validate_dns_or_ipv4_host(host)?;
        } else {
            validate_dns_or_ipv4_host(host_port)?;
        }
    }
    Ok(())
}

fn validate_dns_or_ipv4_host(host: &str) -> Result<(), CloneError> {
    if host.parse::<std::net::Ipv4Addr>().is_ok() {
        return Ok(());
    }
    if host.len() > 253
        || host.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
    {
        return invalid_input("remote URL host is invalid");
    }
    Ok(())
}

fn validate_port_suffix(suffix: &str) -> Result<(), CloneError> {
    let Some(port) = suffix.strip_prefix(':') else {
        return invalid_input("remote URL port is invalid");
    };
    if port.is_empty() || !port.bytes().all(|byte| byte.is_ascii_digit()) {
        return invalid_input("remote URL port is invalid");
    }
    let parsed = port.parse::<u16>().map_err(|_| {
        CloneError::new(CloneErrorClass::InvalidInput, "remote URL port is invalid")
    })?;
    if parsed == 0 {
        return invalid_input("remote URL port is invalid");
    }
    Ok(())
}

fn parse_file_url(remainder: &str) -> Result<ValidatedLocator, CloneError> {
    if remainder.contains(['?', '#', '%']) {
        return invalid_input("file URL escaping, query, and fragment data are not supported");
    }
    let path = if let Some(path) = remainder.strip_prefix("localhost/") {
        format!("/{path}")
    } else if remainder.starts_with('/') {
        remainder.to_string()
    } else {
        return invalid_input("hosted file URLs are not local-filesystem routes");
    };
    #[cfg(windows)]
    let path = path
        .strip_prefix('/')
        .filter(|candidate| looks_like_windows_path(candidate))
        .unwrap_or(&path)
        .to_string();
    parse_local_path(Path::new(&path))
}

fn parse_local_path(path: &Path) -> Result<ValidatedLocator, CloneError> {
    if !path.is_absolute() {
        return invalid_input("local clone sources must use an absolute reviewed path");
    }
    #[cfg(windows)]
    validate_windows_local_drive_path(path)?;
    validate_clean_absolute_path(path, "local clone source")?;
    if path.to_str().is_none() {
        return invalid_input("non-UTF-8 local sources require a typed filesystem locator");
    }
    let canonical = fs::canonicalize(path).map_err(|_| {
        CloneError::new(
            CloneErrorClass::Filesystem,
            "local clone source is unavailable",
        )
    })?;
    let canonical_text = canonical.to_str().ok_or_else(|| {
        CloneError::new(
            CloneErrorClass::InvalidInput,
            "canonical local source is non-UTF-8 and requires a typed filesystem locator",
        )
    })?;
    let source = PinnedFilesystemObject::capture(&canonical, None)?;
    if !matches!(
        source.identity.kind,
        FilesystemObjectKind::File | FilesystemObjectKind::Directory
    ) {
        return invalid_input("local clone source must be a repository directory or bundle file");
    }
    Ok(ValidatedLocator {
        transport: CloneTransportClass::LocalFilesystem,
        argument: canonical.as_os_str().to_owned(),
        presentation: canonical_text.to_string(),
        local_source: Some(source),
    })
}

fn looks_like_remote_helper_route(remote: &str) -> bool {
    remote.split_once("::").is_some_and(|(helper, _)| {
        !helper.is_empty()
            && helper
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
    })
}

fn looks_like_scp_route(remote: &str) -> Result<bool, CloneError> {
    if looks_like_windows_path(remote) || remote.starts_with('/') {
        return Ok(false);
    }
    let separator = scp_separator(remote)?;
    Ok(separator.is_some_and(|colon| !remote[..colon].contains('/')))
}

fn looks_like_windows_path(remote: &str) -> bool {
    let bytes = remote.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'/' | b'\\')
}

fn validate_scp_route(remote: &str) -> Result<(), CloneError> {
    let separator = scp_separator(remote)?.ok_or_else(|| {
        CloneError::new(
            CloneErrorClass::InvalidInput,
            "SCP-style remote is incomplete",
        )
    })?;
    let host_part = &remote[..separator];
    let repository_path = &remote[separator + 1..];
    if host_part.is_empty() || repository_path.is_empty() || repository_path.starts_with('-') {
        return invalid_input("SCP-style remote is incomplete");
    }
    if host_part.matches('@').count() > 1 {
        return invalid_input("SCP-style remote user or host is invalid");
    }
    if host_part
        .chars()
        .any(|character| character.is_whitespace() || "'\"`$;&|<>(){}\\%".contains(character))
    {
        return invalid_input("SCP-style remote user or host is invalid");
    }
    let host = if let Some((user, host)) = host_part.split_once('@') {
        if user.is_empty() || user.contains(':') || user.starts_with('-') {
            return invalid_input("SCP-style remote user or host is invalid");
        }
        host
    } else {
        host_part
    };
    validate_host_port(host)?;
    validate_ssh_repository_path(repository_path)
}

fn validate_ssh_repository_path(path: &str) -> Result<(), CloneError> {
    if path.is_empty()
        || path.starts_with('-')
        || !path.is_ascii()
        || path.split('/').any(|component| component == "..")
        || !path.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-' | b'+' | b'~')
        })
    {
        return invalid_input("SSH repository path contains unsupported shell-sensitive syntax");
    }
    Ok(())
}

fn scp_separator(remote: &str) -> Result<Option<usize>, CloneError> {
    let mut inside_brackets = false;
    for (index, character) in remote.char_indices() {
        match character {
            '[' if inside_brackets => {
                return invalid_input("SCP-style remote contains malformed brackets");
            }
            '[' => inside_brackets = true,
            ']' if !inside_brackets => {
                return invalid_input("SCP-style remote contains malformed brackets");
            }
            ']' => inside_brackets = false,
            ':' if !inside_brackets => return Ok(Some(index)),
            _ => {}
        }
    }
    if inside_brackets {
        return invalid_input("SCP-style remote contains malformed brackets");
    }
    Ok(None)
}

#[derive(Clone)]
struct DestinationReview {
    canonical_parent: PathBuf,
    leaf_name: OsString,
}

impl DestinationReview {
    fn inspect(requested: &Path) -> Result<Self, CloneError> {
        #[cfg(windows)]
        validate_windows_local_drive_path(requested)?;
        validate_clean_absolute_path(requested, "destination")?;
        if os_str_len(requested.as_os_str()) > MAX_DESTINATION_BYTES {
            return invalid_input("destination path is too long");
        }
        if os_str_has_control(requested.as_os_str()) {
            return invalid_input("destination path contains unsupported control characters");
        }
        let leaf_name = requested.file_name().ok_or_else(|| {
            CloneError::new(
                CloneErrorClass::InvalidInput,
                "destination path must name a new directory",
            )
        })?;
        let parent = requested.parent().ok_or_else(|| {
            CloneError::new(
                CloneErrorClass::InvalidInput,
                "destination path must have a reviewed parent",
            )
        })?;
        let canonical_parent = fs::canonicalize(parent).map_err(|_| {
            CloneError::new(
                CloneErrorClass::Filesystem,
                "destination parent directory is unavailable",
            )
        })?;
        let metadata = fs::symlink_metadata(&canonical_parent).map_err(|_| {
            CloneError::new(
                CloneErrorClass::Filesystem,
                "destination parent identity could not be inspected",
            )
        })?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(CloneError::new(
                CloneErrorClass::Filesystem,
                "destination parent is not a real directory",
            ));
        }
        let target = canonical_parent.join(leaf_name);
        match fs::symlink_metadata(&target) {
            Ok(_) => {
                return Err(CloneError::new(
                    CloneErrorClass::DestinationExists,
                    "destination already exists and requires collision review",
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => {
                return Err(CloneError::new(
                    CloneErrorClass::Filesystem,
                    "destination state could not be inspected safely",
                ));
            }
        }
        Ok(Self {
            canonical_parent,
            leaf_name: leaf_name.to_owned(),
        })
    }
}

#[cfg(windows)]
fn validate_windows_local_drive_path(path: &Path) -> Result<(), CloneError> {
    use std::path::Prefix;
    let is_local_drive = matches!(
        path.components().next(),
        Some(Component::Prefix(prefix)) if matches!(prefix.kind(), Prefix::Disk(_))
    );
    if is_local_drive {
        Ok(())
    } else {
        policy_denied(
            "UNC, device, and verbatim paths require reviewed network-filesystem acquisition",
        )
    }
}

#[derive(Clone)]
struct ReviewedDestination {
    presentation_path: PathBuf,
    canonical_parent: PathBuf,
    parent_identity: FilesystemIdentity,
    target_path: PathBuf,
}

impl ReviewedDestination {
    fn capture(requested: &Path, reviewed_parent: &Path) -> Result<Self, CloneError> {
        let review = DestinationReview::inspect(requested)?;
        if review.canonical_parent != reviewed_parent {
            return policy_denied(
                "reviewed canonical destination parent does not match current path resolution",
            );
        }
        let parent_identity = FilesystemIdentity::capture(&review.canonical_parent)?;
        Ok(Self {
            presentation_path: requested.to_path_buf(),
            target_path: review.canonical_parent.join(&review.leaf_name),
            canonical_parent: review.canonical_parent,
            parent_identity,
        })
    }

    fn verify_parent(&self) -> Result<(), CloneError> {
        self.parent_identity.verify(
            &self.canonical_parent,
            "destination parent identity changed",
        )
    }
}

fn validate_clean_absolute_path(path: &Path, label: &'static str) -> Result<(), CloneError> {
    if path.as_os_str().is_empty() || !path.is_absolute() {
        return invalid_input(match label {
            "destination" => "destination must use an absolute reviewed path",
            _ => "local clone source must use an absolute reviewed path",
        });
    }
    if path
        .components()
        .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        || path_has_ambiguous_lexical_syntax(path)
    {
        return invalid_input(match label {
            "destination" => "destination path cannot contain dot traversal components",
            _ => "local clone source cannot contain dot traversal components",
        });
    }
    Ok(())
}

#[cfg(unix)]
fn path_has_ambiguous_lexical_syntax(path: &Path) -> bool {
    use std::os::unix::ffi::OsStrExt;
    let bytes = path.as_os_str().as_bytes();
    (bytes.len() > 1 && bytes.ends_with(b"/"))
        || bytes.windows(2).any(|window| window == b"//")
        || bytes
            .split(|byte| *byte == b'/')
            .any(|component| matches!(component, b"." | b".."))
}

#[cfg(not(unix))]
fn path_has_ambiguous_lexical_syntax(path: &Path) -> bool {
    let value = path.to_string_lossy().replace('\\', "/");
    let without_unc_prefix = value.strip_prefix("//").unwrap_or(&value);
    value.ends_with('/')
        || without_unc_prefix.contains("//")
        || without_unc_prefix
            .split('/')
            .any(|component| matches!(component, "." | ".."))
}

#[cfg(unix)]
fn os_str_len(value: &OsStr) -> usize {
    use std::os::unix::ffi::OsStrExt;
    value.as_bytes().len()
}

#[cfg(not(unix))]
fn os_str_len(value: &OsStr) -> usize {
    value.to_string_lossy().len()
}

#[cfg(unix)]
fn os_str_has_control(value: &OsStr) -> bool {
    use std::os::unix::ffi::OsStrExt;
    value
        .as_bytes()
        .iter()
        .any(|byte| *byte < b' ' || *byte == 0x7f)
}

#[cfg(not(unix))]
fn os_str_has_control(value: &OsStr) -> bool {
    value.to_string_lossy().chars().any(char::is_control)
}

// Filesystem ownership and quarantine ---------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
struct FilesystemIdentity {
    kind: FilesystemObjectKind,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(not(unix))]
    canonical_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilesystemObjectKind {
    File,
    Directory,
    Other,
}

impl FilesystemIdentity {
    fn capture(path: &Path) -> Result<Self, CloneError> {
        let metadata = fs::symlink_metadata(path).map_err(|_| {
            CloneError::new(
                CloneErrorClass::Filesystem,
                "filesystem identity could not be captured",
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(CloneError::new(
                CloneErrorClass::Filesystem,
                "reviewed filesystem object cannot be a symbolic link",
            ));
        }
        let kind = if metadata.is_file() {
            FilesystemObjectKind::File
        } else if metadata.is_dir() {
            FilesystemObjectKind::Directory
        } else {
            FilesystemObjectKind::Other
        };
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            Ok(Self {
                kind,
                device: metadata.dev(),
                inode: metadata.ino(),
            })
        }
        #[cfg(not(unix))]
        {
            let canonical_path = fs::canonicalize(path).map_err(|_| {
                CloneError::new(
                    CloneErrorClass::Filesystem,
                    "filesystem identity could not be canonicalized",
                )
            })?;
            Ok(Self {
                kind,
                canonical_path,
            })
        }
    }

    fn matches(&self, path: &Path) -> bool {
        let Ok(metadata) = fs::symlink_metadata(path) else {
            return false;
        };
        if metadata.file_type().is_symlink() {
            return false;
        }
        let kind = if metadata.is_file() {
            FilesystemObjectKind::File
        } else if metadata.is_dir() {
            FilesystemObjectKind::Directory
        } else {
            FilesystemObjectKind::Other
        };
        if kind != self.kind {
            return false;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            metadata.dev() == self.device && metadata.ino() == self.inode
        }
        #[cfg(not(unix))]
        {
            fs::canonicalize(path)
                .map(|canonical| canonical == self.canonical_path)
                .unwrap_or(false)
        }
    }

    fn verify(&self, path: &Path, message: &'static str) -> Result<(), CloneError> {
        if self.matches(path) {
            Ok(())
        } else {
            Err(CloneError::new(CloneErrorClass::Filesystem, message))
        }
    }
}

#[derive(Clone)]
struct PinnedFilesystemObject {
    path: PathBuf,
    identity: FilesystemIdentity,
}

impl PinnedFilesystemObject {
    fn capture(
        path: &Path,
        required_kind: Option<FilesystemObjectKind>,
    ) -> Result<Self, CloneError> {
        let identity = FilesystemIdentity::capture(path)?;
        if required_kind.is_some_and(|kind| identity.kind != kind) {
            return Err(CloneError::new(
                CloneErrorClass::Filesystem,
                "reviewed filesystem object has the wrong type",
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            identity,
        })
    }

    fn verify(&self, message: &'static str) -> Result<(), CloneError> {
        self.identity.verify(&self.path, message)
    }
}

struct OwnedDestination {
    path: PathBuf,
    identity: FilesystemIdentity,
    #[cfg(unix)]
    original_mode: u32,
    active: bool,
}

impl OwnedDestination {
    fn create(reviewed: &ReviewedDestination) -> Result<Self, CloneError> {
        reviewed.verify_parent()?;
        match fs::create_dir(&reviewed.target_path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(CloneError::new(
                    CloneErrorClass::DestinationExists,
                    "destination was claimed after review",
                ));
            }
            Err(_) => {
                return Err(CloneError::new(
                    CloneErrorClass::Filesystem,
                    "destination directory could not be created",
                ));
            }
        }

        let metadata = match fs::symlink_metadata(&reviewed.target_path) {
            Ok(metadata) => metadata,
            Err(_) => {
                let _ = fs::remove_dir(&reviewed.target_path);
                return Err(CloneError::new(
                    CloneErrorClass::Filesystem,
                    "created destination could not be verified",
                ));
            }
        };
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            let _ = fs::remove_dir(&reviewed.target_path);
            return Err(CloneError::new(
                CloneErrorClass::Filesystem,
                "created destination is not a real directory",
            ));
        }
        let identity = FilesystemIdentity::capture(&reviewed.target_path)?;
        #[cfg(unix)]
        let original_mode = {
            use std::os::unix::fs::MetadataExt;
            metadata.mode() & 0o7777
        };
        let mut owned = Self {
            path: reviewed.target_path.clone(),
            identity,
            #[cfg(unix)]
            original_mode,
            active: true,
        };

        if reviewed.verify_parent().is_err() || !owned.is_same_directory() {
            let _ = owned.rollback();
            return Err(CloneError::new(
                CloneErrorClass::Filesystem,
                "destination containment changed while it was being reserved",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if fs::set_permissions(&owned.path, fs::Permissions::from_mode(0o700)).is_err() {
                let _ = owned.rollback();
                return Err(CloneError::new(
                    CloneErrorClass::Filesystem,
                    "destination permissions could not be restricted",
                ));
            }
        }
        Ok(owned)
    }

    fn is_same_directory(&self) -> bool {
        self.identity.matches(&self.path) && self.identity.kind == FilesystemObjectKind::Directory
    }

    fn has_meaningful_git_state(&self) -> bool {
        if !self.is_same_directory() {
            return false;
        }
        let git = self.path.join(".git");
        if !is_real_directory(&git) {
            return false;
        }
        ["HEAD", "config", "objects", "refs"]
            .iter()
            .any(|name| fs::symlink_metadata(git.join(name)).is_ok())
    }

    fn has_worktree_materialization(&self) -> bool {
        if !self.is_same_directory() {
            return false;
        }
        fs::read_dir(&self.path)
            .ok()
            .into_iter()
            .flatten()
            .flatten()
            .any(|entry| entry.file_name() != OsStr::new(".git"))
    }

    fn restore_permissions(&self) -> Result<(), ()> {
        if !self.is_same_directory() {
            return Err(());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.path, fs::Permissions::from_mode(self.original_mode))
                .map_err(|_| ())?;
        }
        Ok(())
    }

    fn rollback(&mut self) -> Result<(), ()> {
        if !self.active {
            return Ok(());
        }
        quarantine_and_remove(&mut self.path, &self.identity).map_err(|_| ())?;
        self.active = false;
        Ok(())
    }

    fn preserve(&mut self, state: CloneInterruptedState) -> ClonePartialAcquisition {
        self.active = false;
        ClonePartialAcquisition {
            path: self.path.clone(),
            identity: self.identity.clone(),
            state,
        }
    }

    fn commit(&mut self) {
        self.active = false;
    }
}

impl Drop for OwnedDestination {
    fn drop(&mut self) {
        let _ = self.rollback();
    }
}

fn unpredictable_path_nonce() -> String {
    let sequence = GUARD_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let mut first = RandomState::new().build_hasher();
    first.write_u64(u64::from(std::process::id()));
    first.write_u64(sequence);
    first.write(&timestamp.to_le_bytes());
    let first = first.finish();

    let mut second = RandomState::new().build_hasher();
    second.write_u64(first);
    second.write_u64(sequence.rotate_left(31));
    second.write(&timestamp.to_be_bytes());
    format!("{first:016x}{:016x}", second.finish())
}

fn quarantine_and_remove(
    path: &mut PathBuf,
    identity: &FilesystemIdentity,
) -> Result<(), CloneError> {
    if !identity.matches(path) {
        return Err(CloneError::new(
            CloneErrorClass::Filesystem,
            "owned destination identity changed; recursive cleanup was refused",
        ));
    }
    let parent = path.parent().ok_or_else(|| {
        CloneError::new(
            CloneErrorClass::Filesystem,
            "owned destination has no cleanup parent",
        )
    })?;
    for _ in 0..32 {
        let quarantine = parent.join(format!(
            ".aureline-clone-discard-{}",
            unpredictable_path_nonce()
        ));
        if fs::symlink_metadata(&quarantine).is_ok() {
            continue;
        }
        match fs::rename(&*path, &quarantine) {
            Ok(()) => {
                if !identity.matches(&quarantine) {
                    return Err(CloneError::new(
                        CloneErrorClass::Filesystem,
                        "quarantined destination identity changed; deletion was refused",
                    ));
                }
                *path = quarantine;
                fs::remove_dir_all(&*path).map_err(|_| {
                    CloneError::new(
                        CloneErrorClass::Filesystem,
                        "quarantined destination could not be removed",
                    )
                })?;
                return Ok(());
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => {
                return Err(CloneError::new(
                    CloneErrorClass::Filesystem,
                    "owned destination could not be moved into cleanup quarantine",
                ));
            }
        }
    }
    Err(CloneError::new(
        CloneErrorClass::Filesystem,
        "a unique cleanup quarantine could not be reserved",
    ))
}

fn is_real_directory(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink())
        .unwrap_or(false)
}

// Guard and reviewed transport projections ---------------------------------

struct GitGuard {
    path: PathBuf,
    identity: FilesystemIdentity,
    hooks_path: PathBuf,
    template_path: PathBuf,
    home_path: PathBuf,
    xdg_path: PathBuf,
    empty_config_path: PathBuf,
    active: bool,
}

impl GitGuard {
    fn create(parent: &Path) -> Result<Self, CloneError> {
        for _ in 0..32 {
            let path = parent.join(format!(
                ".aureline-clone-guard-{}",
                unpredictable_path_nonce()
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    let identity = match FilesystemIdentity::capture(&path) {
                        Ok(identity) => identity,
                        Err(error) => {
                            let _ = fs::remove_dir(&path);
                            return Err(error);
                        }
                    };
                    let mut guard = Self {
                        hooks_path: path.join("hooks"),
                        template_path: path.join("template"),
                        home_path: path.join("home"),
                        xdg_path: path.join("xdg"),
                        empty_config_path: path.join("empty.gitconfig"),
                        path,
                        identity,
                        active: true,
                    };
                    if let Err(error) = guard.populate() {
                        let _ = guard.cleanup();
                        return Err(error);
                    }
                    return Ok(guard);
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(_) => {
                    return Err(CloneError::new(
                        CloneErrorClass::Filesystem,
                        "clone guard directory could not be created",
                    ));
                }
            }
        }
        Err(CloneError::new(
            CloneErrorClass::Filesystem,
            "a unique clone guard directory could not be reserved",
        ))
    }

    fn populate(&self) -> Result<(), CloneError> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.path, fs::Permissions::from_mode(0o700)).map_err(|_| {
                CloneError::new(
                    CloneErrorClass::Filesystem,
                    "clone guard permissions could not be restricted",
                )
            })?;
        }
        for directory in [
            &self.hooks_path,
            &self.template_path,
            &self.home_path,
            &self.xdg_path,
        ] {
            fs::create_dir(directory).map_err(|_| {
                CloneError::new(
                    CloneErrorClass::Filesystem,
                    "clone guard subdirectory could not be created",
                )
            })?;
        }
        create_private_file(&self.empty_config_path)?;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), CloneError> {
        if !self.active {
            return Ok(());
        }
        quarantine_and_remove(&mut self.path, &self.identity)?;
        self.active = false;
        Ok(())
    }
}

impl Drop for GitGuard {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn create_private_file(path: &Path) -> Result<File, CloneError> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| {
            CloneError::new(
                CloneErrorClass::Filesystem,
                "clone guard configuration could not be created",
            )
        })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o600))
            .map_err(|_| {
                CloneError::new(
                    CloneErrorClass::Filesystem,
                    "clone guard file permissions could not be restricted",
                )
            })?;
    }
    Ok(file)
}

#[derive(Clone)]
enum ValidatedAuthentication {
    Anonymous,
    SshAgent(Box<ValidatedSshAuthentication>),
}

#[derive(Clone)]
struct ValidatedSshAuthentication {
    authority_ticket_ref: String,
    expires_at: u64,
    ssh_auth_sock: PinnedFilesystemObject,
    known_hosts_file: PinnedFilesystemObject,
    ssh_binary: PinnedExecutable,
}

impl fmt::Debug for ValidatedAuthentication {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => formatter.write_str("Anonymous"),
            Self::SshAgent(_) => formatter.write_str("SshAgent(<redacted-projection>)"),
        }
    }
}

impl ValidatedAuthentication {
    fn capture(
        authentication: &CloneAuthentication,
        transport: CloneTransportClass,
    ) -> Result<Self, CloneError> {
        match (authentication, transport) {
            (CloneAuthentication::Anonymous, CloneTransportClass::Ssh) => {
                policy_denied("SSH clone requires a reviewed SSH-agent authority projection")
            }
            (CloneAuthentication::Anonymous, _) => Ok(Self::Anonymous),
            (
                CloneAuthentication::SshAgent {
                    authority_ticket_ref,
                    authority_expires_at_unix_seconds,
                    ssh_auth_sock,
                    known_hosts_file,
                    ssh_binary,
                },
                CloneTransportClass::Ssh,
            ) => {
                validate_record_ref(authority_ticket_ref, "authority ticket")?;
                ensure_not_expired(*authority_expires_at_unix_seconds)?;
                let socket_path = require_canonical_path(ssh_auth_sock, "SSH agent socket")?;
                let known_hosts_path =
                    require_canonical_path(known_hosts_file, "known-hosts file")?;
                let ssh_path = require_canonical_path(ssh_binary, "SSH binary")?;
                Ok(Self::SshAgent(Box::new(ValidatedSshAuthentication {
                    authority_ticket_ref: authority_ticket_ref.clone(),
                    expires_at: *authority_expires_at_unix_seconds,
                    ssh_auth_sock: capture_ssh_agent_socket(&socket_path)?,
                    known_hosts_file: PinnedFilesystemObject::capture(
                        &known_hosts_path,
                        Some(FilesystemObjectKind::File),
                    )?,
                    ssh_binary: PinnedExecutable::capture(&ssh_path, "SSH binary is unavailable")?,
                })))
            }
            (CloneAuthentication::SshAgent { .. }, _) => {
                policy_denied("SSH-agent authority cannot be projected into a non-SSH clone")
            }
        }
    }

    fn verify_current(&self) -> Result<(), CloneError> {
        match self {
            Self::Anonymous => Ok(()),
            Self::SshAgent(authentication) => {
                let _ = &authentication.authority_ticket_ref;
                ensure_not_expired(authentication.expires_at)?;
                authentication
                    .ssh_auth_sock
                    .verify("reviewed SSH agent socket identity changed")?;
                authentication
                    .known_hosts_file
                    .verify("reviewed known-hosts identity changed")?;
                authentication
                    .ssh_binary
                    .verify("reviewed SSH executable identity changed")
            }
        }
    }
}

fn capture_ssh_agent_socket(path: &Path) -> Result<PinnedFilesystemObject, CloneError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileTypeExt;
        let metadata = fs::symlink_metadata(path).map_err(|_| {
            CloneError::new(
                CloneErrorClass::Filesystem,
                "reviewed SSH agent socket is unavailable",
            )
        })?;
        if !metadata.file_type().is_socket() {
            return policy_denied("reviewed SSH agent endpoint is not a Unix socket");
        }
        PinnedFilesystemObject::capture(path, None)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        policy_denied(
            "SSH agent projection is unavailable until endpoint identity can be bound safely",
        )
    }
}

#[derive(Clone, Default)]
struct ValidatedTransportOptions {
    proxy_url: Option<String>,
    ca_bundle: Option<PinnedFilesystemObject>,
}

impl ValidatedTransportOptions {
    fn capture(
        options: &CloneTransportOptions,
        transport: CloneTransportClass,
    ) -> Result<Self, CloneError> {
        if (options.proxy_url.is_some() || options.ca_bundle_path.is_some())
            && transport != CloneTransportClass::Https
        {
            return policy_denied("proxy and CA options are only valid for reviewed HTTPS routes");
        }
        let proxy_url = match &options.proxy_url {
            Some(proxy) => {
                validate_proxy_url(proxy)?;
                Some(proxy.clone())
            }
            None => None,
        };
        let ca_bundle = match &options.ca_bundle_path {
            Some(path) => {
                let canonical = require_canonical_path(path, "CA bundle")?;
                Some(PinnedFilesystemObject::capture(
                    &canonical,
                    Some(FilesystemObjectKind::File),
                )?)
            }
            None => None,
        };
        Ok(Self {
            proxy_url,
            ca_bundle,
        })
    }

    fn verify_current(&self) -> Result<(), CloneError> {
        if let Some(ca_bundle) = &self.ca_bundle {
            ca_bundle.verify("reviewed CA bundle identity changed")?;
        }
        Ok(())
    }
}

fn validate_proxy_url(proxy: &str) -> Result<(), CloneError> {
    validate_locator_text(proxy)?;
    let Some((scheme, remainder)) = proxy.split_once("://") else {
        return invalid_input("reviewed proxy must use an HTTP or HTTPS URL");
    };
    if !matches!(scheme, "http" | "https") || remainder.contains(['?', '#', '@']) {
        return invalid_input("reviewed proxy URL is unsupported or contains credentials");
    }
    let authority = remainder.split('/').next().unwrap_or_default();
    validate_authority(authority, false)
}

fn require_canonical_path(path: &Path, label: &'static str) -> Result<PathBuf, CloneError> {
    if !path.is_absolute() {
        return policy_denied("reviewed authority and transport paths must be absolute");
    }
    let canonical = fs::canonicalize(path).map_err(|_| {
        CloneError::new(
            CloneErrorClass::Filesystem,
            format!("reviewed {label} is unavailable"),
        )
    })?;
    if canonical != path {
        return policy_denied("reviewed authority path must already be canonical");
    }
    Ok(canonical)
}

fn ensure_not_expired(expires_at: u64) -> Result<(), CloneError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| CloneError::new(CloneErrorClass::PolicyDenied, "system clock is unavailable"))?
        .as_secs();
    if expires_at <= now {
        policy_denied("SSH authority ticket expired before clone execution")
    } else {
        Ok(())
    }
}

// Command construction ------------------------------------------------------

#[derive(Clone)]
struct PinnedExecutable {
    path: PathBuf,
    identity: FilesystemIdentity,
}

impl PinnedExecutable {
    fn resolve(requested: &Path, missing_message: &'static str) -> Result<Self, CloneError> {
        let resolved = if requested.is_absolute() {
            fs::canonicalize(requested).ok()
        } else if requested.components().count() == 1 {
            resolve_from_path(requested)
        } else {
            None
        }
        .ok_or_else(|| CloneError::new(CloneErrorClass::GitNotInstalled, missing_message))?;
        Self::capture(&resolved, missing_message)
    }

    fn capture(path: &Path, missing_message: &'static str) -> Result<Self, CloneError> {
        let identity = FilesystemIdentity::capture(path)
            .map_err(|_| CloneError::new(CloneErrorClass::GitNotInstalled, missing_message))?;
        if identity.kind != FilesystemObjectKind::File {
            return Err(CloneError::new(
                CloneErrorClass::GitNotInstalled,
                missing_message,
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let executable = fs::metadata(path)
                .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
                .unwrap_or(false);
            if !executable {
                return Err(CloneError::new(
                    CloneErrorClass::GitNotInstalled,
                    missing_message,
                ));
            }
        }
        Ok(Self {
            path: path.to_path_buf(),
            identity,
        })
    }

    fn verify(&self, message: &'static str) -> Result<(), CloneError> {
        self.identity.verify(&self.path, message)
    }
}

fn resolve_from_path(binary: &Path) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for directory in std::env::split_paths(&paths) {
        let candidate = directory.join(binary);
        if let Ok(canonical) = fs::canonicalize(&candidate) {
            if fs::metadata(&canonical)
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
            {
                return Some(canonical);
            }
        }
        #[cfg(windows)]
        {
            let candidate = directory.join(format!("{}.exe", binary.to_string_lossy()));
            if let Ok(canonical) = fs::canonicalize(&candidate) {
                if fs::metadata(&canonical)
                    .map(|metadata| metadata.is_file())
                    .unwrap_or(false)
                {
                    return Some(canonical);
                }
            }
        }
    }
    None
}

fn configure_probe_environment(command: &mut Command, git_binary: &Path) {
    command.env_clear();
    if let Some(parent) = git_binary.parent() {
        command.env("PATH", parent);
    }
    #[cfg(windows)]
    for key in ["SystemRoot", "WINDIR"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", null_device_path())
        .env("GIT_CONFIG_SYSTEM", null_device_path())
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_ASKPASS", "")
        .env("SSH_ASKPASS", "")
        .env("SSH_ASKPASS_REQUIRE", "never");
}

fn build_clone_command(
    git: &PinnedExecutable,
    execution: &ApprovedCloneExecution,
    destination: &Path,
    guard: &GitGuard,
) -> Result<Command, CloneError> {
    execution.verify_current_bindings()?;
    git.verify("Git executable identity changed before clone")?;
    let mut command = Command::new(&git.path);
    configure_acquisition_environment(&mut command, git, execution, guard)?;
    apply_common_git_configuration(&mut command, execution, guard);
    command
        .arg("clone")
        .arg("--progress")
        .arg("--no-checkout")
        .arg("--no-recurse-submodules")
        .arg("--no-hardlinks")
        .arg(prefixed_path_argument("--template=", &guard.template_path));
    if execution.locator.transport == CloneTransportClass::LocalFilesystem {
        command.arg("--no-local");
    }
    if let Some(reference) = clone_branch_argument(&execution.reference.reference) {
        command.arg("--branch").arg(reference);
    }
    if let CloneHistoryPolicy::Shallow { depth } = execution.topology.history {
        command.arg("--depth").arg(depth.to_string());
    }
    command
        .arg("--")
        .arg(&execution.locator.argument)
        .arg(destination)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_process_group(&mut command);
    Ok(command)
}

fn build_verify_command(
    git: &PinnedExecutable,
    execution: &ApprovedCloneExecution,
    destination: &Path,
    guard: &GitGuard,
) -> Result<Command, CloneError> {
    let mut command = Command::new(&git.path);
    configure_acquisition_environment(&mut command, git, execution, guard)?;
    apply_common_git_configuration(&mut command, execution, guard);
    command
        .arg("-C")
        .arg(destination)
        .arg("rev-parse")
        .arg("--verify")
        .arg("HEAD^{commit}")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_process_group(&mut command);
    Ok(command)
}

fn build_checkout_command(
    git: &PinnedExecutable,
    execution: &ApprovedCloneExecution,
    destination: &Path,
    guard: &GitGuard,
) -> Result<Command, CloneError> {
    execution.verify_current_bindings()?;
    git.verify("Git executable identity changed before checkout")?;
    let mut command = Command::new(&git.path);
    configure_acquisition_environment(&mut command, git, execution, guard)?;
    apply_common_git_configuration(&mut command, execution, guard);
    command
        .arg("-C")
        .arg(destination)
        .arg("checkout")
        .arg("--force")
        .arg("--no-recurse-submodules");
    command
        .arg("--detach")
        .arg(&execution.reference.expected_commit_oid);
    command
        .arg("--")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_process_group(&mut command);
    Ok(command)
}

fn configure_acquisition_environment(
    command: &mut Command,
    git: &PinnedExecutable,
    execution: &ApprovedCloneExecution,
    guard: &GitGuard,
) -> Result<(), CloneError> {
    command.env_clear();
    let mut binary_directories = Vec::new();
    if let Some(parent) = git.path.parent() {
        binary_directories.push(parent.to_path_buf());
    }
    if let ValidatedAuthentication::SshAgent(authentication) = &execution.authentication {
        if let Some(parent) = authentication.ssh_binary.path.parent() {
            if !binary_directories.iter().any(|path| path == parent) {
                binary_directories.push(parent.to_path_buf());
            }
        }
    }
    let path = std::env::join_paths(binary_directories).map_err(|_| {
        CloneError::new(
            CloneErrorClass::PolicyDenied,
            "reviewed executable path could not be projected safely",
        )
    })?;
    command
        .env("PATH", path)
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .env("HOME", &guard.home_path)
        .env("XDG_CONFIG_HOME", &guard.xdg_path)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", &guard.empty_config_path)
        .env("GIT_CONFIG_SYSTEM", &guard.empty_config_path)
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_PAGER", "")
        .env("GIT_EDITOR", "false")
        .env("GIT_SEQUENCE_EDITOR", "false")
        .env("GIT_MERGE_AUTOEDIT", "no")
        .env("GIT_ASKPASS", "")
        .env("SSH_ASKPASS", "")
        .env("SSH_ASKPASS_REQUIRE", "never")
        .env("GIT_LFS_SKIP_SMUDGE", "1")
        .env("GIT_PROTOCOL_FROM_USER", "0");
    #[cfg(unix)]
    command.env("TMPDIR", &guard.path);
    #[cfg(windows)]
    {
        for key in ["SystemRoot", "WINDIR"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        command.env("TEMP", &guard.path).env("TMP", &guard.path);
    }
    if let ValidatedAuthentication::SshAgent(authentication) = &execution.authentication {
        command
            .env("SSH_AUTH_SOCK", &authentication.ssh_auth_sock.path)
            .env("GIT_SSH_VARIANT", "ssh")
            .env(
                "GIT_SSH_COMMAND",
                hardened_ssh_command(
                    &authentication.ssh_binary.path,
                    &authentication.known_hosts_file.path,
                )?,
            );
    }
    Ok(())
}

fn apply_common_git_configuration(
    command: &mut Command,
    execution: &ApprovedCloneExecution,
    guard: &GitGuard,
) {
    for setting in [
        "protocol.allow=never",
        "protocol.ext.allow=never",
        "submodule.recurse=false",
        "fetch.recurseSubmodules=false",
        "core.fsmonitor=false",
        "core.untrackedCache=false",
        "core.protectHFS=true",
        "core.protectNTFS=true",
        "filter.lfs.required=false",
        "filter.lfs.smudge=",
        "filter.lfs.process=",
        "credential.helper=",
        "credential.interactive=never",
        "http.sslVerify=true",
        "http.followRedirects=false",
        "fetch.fsckObjects=true",
        "transfer.fsckObjects=true",
    ] {
        command.arg("-c").arg(setting);
    }
    let allowed_protocol = match execution.locator.transport {
        CloneTransportClass::LocalFilesystem => "protocol.file.allow=always",
        CloneTransportClass::Https => "protocol.https.allow=always",
        CloneTransportClass::Ssh => "protocol.ssh.allow=always",
        CloneTransportClass::GitProtocol => "protocol.git.allow=always",
    };
    command.arg("-c").arg(allowed_protocol);
    if execution.locator.transport == CloneTransportClass::Ssh {
        command
            .arg("-c")
            .arg("protocol.git+ssh.allow=always")
            .arg("-c")
            .arg("protocol.ssh+git.allow=always");
    }
    command
        .arg("-c")
        .arg(prefixed_path_argument("core.hooksPath=", &guard.hooks_path))
        .arg("-c")
        .arg(prefixed_path_argument(
            "core.attributesFile=",
            Path::new(null_device_path()),
        ));
    if let Some(proxy) = &execution.transport_options.proxy_url {
        command.arg("-c").arg(format!("http.proxy={proxy}"));
    } else {
        command.arg("-c").arg("http.proxy=");
    }
    if let Some(ca_bundle) = &execution.transport_options.ca_bundle {
        command
            .arg("-c")
            .arg(prefixed_path_argument("http.sslCAInfo=", &ca_bundle.path));
    }
}

fn prefixed_path_argument(prefix: &str, path: &Path) -> OsString {
    let mut argument = OsString::from(prefix);
    argument.push(path.as_os_str());
    argument
}

fn hardened_ssh_command(ssh_binary: &Path, known_hosts: &Path) -> Result<String, CloneError> {
    let ssh = shell_quote_path(ssh_binary)?;
    let known_hosts = shell_quote_path(known_hosts)?;
    Ok(format!(
        "{ssh} -F {} -oBatchMode=yes -oClearAllForwardings=yes \
         -oPermitLocalCommand=no -oRequestTTY=no -oIdentityFile=none \
         -oIdentitiesOnly=no -oPasswordAuthentication=no \
         -oKbdInteractiveAuthentication=no -oGSSAPIAuthentication=no \
         -oHostbasedAuthentication=no -oStrictHostKeyChecking=yes \
         -oUserKnownHostsFile={known_hosts} -oGlobalKnownHostsFile={} \
         -oProxyCommand=none -oProxyJump=none -oCanonicalizeHostname=no \
         -oVerifyHostKeyDNS=no -oUpdateHostKeys=no -oNumberOfPasswordPrompts=0",
        null_device_path(),
        null_device_path()
    ))
}

fn shell_quote_path(path: &Path) -> Result<String, CloneError> {
    let value = path.to_str().ok_or_else(|| {
        CloneError::new(
            CloneErrorClass::PolicyDenied,
            "SSH authority paths must be valid UTF-8",
        )
    })?;
    if value.chars().any(char::is_control) {
        return policy_denied("SSH authority path contains unsupported control characters");
    }
    Ok(format!("'{}'", value.replace('\'', "'\\''")))
}

const fn null_device_path() -> &'static str {
    if cfg!(windows) {
        "NUL"
    } else {
        "/dev/null"
    }
}

fn configure_process_group(_command: &mut Command) {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        _command.process_group(0);
    }
}

// Bounded subprocess supervisor ---------------------------------------------

struct CommandOutput {
    status: ExitStatus,
    stdout: Vec<u8>,
    diagnostics: DiagnosticSummary,
}

struct ProgressSink<'a> {
    state: &'a mut ProgressState,
    callback: &'a mut dyn FnMut(CloneProgressEvent),
}

struct SupervisedChild {
    child: Child,
    armed: bool,
}

impl SupervisedChild {
    fn new(child: Child) -> Self {
        Self { child, armed: true }
    }

    fn terminate(&mut self) {
        if self.armed {
            terminate_child_tree(&mut self.child);
            self.armed = false;
        }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for SupervisedChild {
    fn drop(&mut self) {
        self.terminate();
    }
}

fn run_supervised(
    command: &mut Command,
    overall_timeout: Duration,
    idle_timeout: Duration,
    cancellation: &CloneCancellationToken,
    stdout_limit: usize,
    mut progress: Option<ProgressSink<'_>>,
) -> Result<CommandOutput, CloneError> {
    let child = command.spawn().map_err(map_git_launch_error)?;
    let mut child = SupervisedChild::new(child);
    let stdout = child.child.stdout.take().ok_or_else(|| {
        child.terminate();
        CloneError::new(CloneErrorClass::Io, "Git stdout pipe was unavailable")
    })?;
    let stderr = child.child.stderr.take().ok_or_else(|| {
        child.terminate();
        CloneError::new(CloneErrorClass::Io, "Git diagnostic pipe was unavailable")
    })?;

    let (sender, receiver) = mpsc::sync_channel(OUTPUT_CHANNEL_SLOTS);
    spawn_output_reader(OutputStream::Stdout, stdout, sender.clone());
    spawn_output_reader(OutputStream::Stderr, stderr, sender);

    let started = Instant::now();
    let mut last_activity = started;
    let mut exited_at = None;
    let mut status = None;
    let mut stdout = Vec::with_capacity(stdout_limit.min(1024));
    let mut stdout_overflow = false;
    let mut stdout_eof = false;
    let mut stderr_eof = false;
    let mut diagnostics = DiagnosticSummary::default();

    loop {
        loop {
            match receiver.try_recv() {
                Ok(OutputMessage::Data(stream, bytes)) => {
                    last_activity = Instant::now();
                    match stream {
                        OutputStream::Stdout => {
                            let remaining = stdout_limit.saturating_sub(stdout.len());
                            stdout.extend_from_slice(&bytes[..bytes.len().min(remaining)]);
                            stdout_overflow |= bytes.len() > remaining;
                        }
                        OutputStream::Stderr => {
                            diagnostics.ingest(&bytes);
                            if let Some(sink) = progress.as_mut() {
                                sink.state.ingest(&bytes, sink.callback)?;
                            }
                        }
                    }
                }
                Ok(OutputMessage::Eof(OutputStream::Stdout)) => stdout_eof = true,
                Ok(OutputMessage::Eof(OutputStream::Stderr)) => {
                    stderr_eof = true;
                    diagnostics.finish();
                    if let Some(sink) = progress.as_mut() {
                        sink.state.finish(sink.callback)?;
                    }
                }
                Ok(OutputMessage::ReadFailed) => {
                    child.terminate();
                    return Err(CloneError::new(
                        CloneErrorClass::Io,
                        "Git process output could not be read",
                    ));
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    stdout_eof = true;
                    stderr_eof = true;
                    break;
                }
            }
        }

        if stdout_overflow {
            child.terminate();
            return Err(CloneError::new(
                CloneErrorClass::OutputLimit,
                "Git command output exceeded the safe capture limit",
            ));
        }
        if cancellation.is_cancelled() {
            child.terminate();
            return Err(CloneError::new(
                CloneErrorClass::Cancelled,
                "Repository acquisition was cancelled",
            ));
        }
        let now = Instant::now();
        if now.duration_since(started) >= overall_timeout {
            child.terminate();
            return Err(CloneError::new(
                CloneErrorClass::Timeout,
                "Repository acquisition exceeded its reviewed overall deadline",
            ));
        }
        if now.duration_since(last_activity) >= idle_timeout {
            child.terminate();
            return Err(CloneError::new(
                CloneErrorClass::Timeout,
                "Repository acquisition exceeded its reviewed idle deadline",
            ));
        }

        if status.is_none() {
            match child.child.try_wait() {
                Ok(Some(observed)) => {
                    status = Some(observed);
                    exited_at = Some(now);
                    last_activity = now;
                }
                Ok(None) => {}
                Err(_) => {
                    child.terminate();
                    return Err(CloneError::new(
                        CloneErrorClass::Io,
                        "Git process status could not be observed",
                    ));
                }
            }
        }

        if let Some(status) = status {
            if stdout_eof && stderr_eof {
                child.disarm();
                return Ok(CommandOutput {
                    status,
                    stdout,
                    diagnostics,
                });
            }
            if exited_at.is_some_and(|exited| now.duration_since(exited) >= POST_EXIT_DRAIN_GRACE) {
                child.terminate();
                return Err(CloneError::new(
                    CloneErrorClass::Io,
                    "Git descendant retained an output pipe after process exit",
                ));
            }
        }
        thread::sleep(SUPERVISOR_POLL_INTERVAL);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputStream {
    Stdout,
    Stderr,
}

enum OutputMessage {
    Data(OutputStream, Vec<u8>),
    Eof(OutputStream),
    ReadFailed,
}

fn spawn_output_reader(
    stream_kind: OutputStream,
    mut stream: impl Read + Send + 'static,
    sender: mpsc::SyncSender<OutputMessage>,
) {
    let _ = thread::Builder::new()
        .name("aureline-clone-output".to_string())
        .spawn(move || {
            let mut buffer = [0_u8; 8192];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        let _ = sender.send(OutputMessage::Eof(stream_kind));
                        return;
                    }
                    Ok(read) => {
                        if sender
                            .send(OutputMessage::Data(stream_kind, buffer[..read].to_vec()))
                            .is_err()
                        {
                            return;
                        }
                    }
                    Err(_) => {
                        let _ = sender.send(OutputMessage::ReadFailed);
                        return;
                    }
                }
            }
        });
}

fn terminate_child_tree(child: &mut Child) {
    #[cfg(unix)]
    {
        let process_group = format!("-{}", child.id());
        let _ = Command::new("/bin/kill")
            .env_clear()
            .args(["-KILL", "--", process_group.as_str()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn map_git_launch_error(error: std::io::Error) -> CloneError {
    if error.kind() == std::io::ErrorKind::NotFound {
        CloneError::new(
            CloneErrorClass::GitNotInstalled,
            "Git executable was not found",
        )
    } else {
        CloneError::new(CloneErrorClass::Io, "Git process could not be launched")
    }
}

#[derive(Default)]
struct ProgressState {
    event_count: usize,
    last_message: Option<&'static str>,
    line: Vec<u8>,
}

impl ProgressState {
    fn ingest(
        &mut self,
        bytes: &[u8],
        progress: &mut dyn FnMut(CloneProgressEvent),
    ) -> Result<(), CloneError> {
        for byte in bytes {
            if matches!(*byte, b'\n' | b'\r') {
                self.emit(progress)?;
                self.line.clear();
            } else if self.line.len() < MAX_PROGRESS_LINE_BYTES {
                self.line.push(*byte);
            }
        }
        Ok(())
    }

    fn finish(&mut self, progress: &mut dyn FnMut(CloneProgressEvent)) -> Result<(), CloneError> {
        self.emit(progress)?;
        self.line.clear();
        Ok(())
    }

    fn emit(&mut self, progress: &mut dyn FnMut(CloneProgressEvent)) -> Result<(), CloneError> {
        if self.event_count >= MAX_PROGRESS_EVENTS {
            return Ok(());
        }
        let line = sanitize_text(&self.line, MAX_PROGRESS_LINE_BYTES).to_ascii_lowercase();
        let message = if line.contains("cloning into") {
            Some("Preparing destination")
        } else if line.contains("enumerating objects") {
            Some("Enumerating repository objects")
        } else if line.contains("counting objects") {
            Some("Counting repository objects")
        } else if line.contains("compressing objects") {
            Some("Compressing repository objects")
        } else if line.contains("receiving objects") {
            Some("Receiving repository objects")
        } else if line.contains("resolving deltas") {
            Some("Resolving repository deltas")
        } else if line.contains("updating files") || line.contains("checking out files") {
            Some("Materializing working tree")
        } else {
            None
        };
        if let Some(message) = message {
            if self.last_message != Some(message) {
                self.last_message = Some(message);
                self.event_count += 1;
                emit_progress_event(
                    progress,
                    CloneProgressEvent::new(CloneProgressPhase::Progress, message),
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct DiagnosticSummary {
    auth: bool,
    remote_not_found: bool,
    host_key: bool,
    tls: bool,
    disk_full: bool,
    destination_exists: bool,
    network: bool,
    filesystem: bool,
    carry: String,
}

impl DiagnosticSummary {
    fn ingest(&mut self, bytes: &[u8]) {
        let chunk = sanitize_text(bytes, bytes.len().min(8192)).to_ascii_lowercase();
        let combined = format!("{}{}", self.carry, chunk);
        self.auth |= contains_any(
            &combined,
            &[
                "authentication failed",
                "could not read username",
                "permission denied (publickey)",
                "access denied",
                "authorization failed",
                "http 401",
                "http 403",
                "requested url returned error: 401",
                "requested url returned error: 403",
            ],
        );
        self.remote_not_found |= contains_any(
            &combined,
            &[
                "repository not found",
                "project not found",
                "does not appear to be a git repository",
                "does not exist",
            ],
        );
        self.host_key |= contains_any(
            &combined,
            &[
                "host key verification failed",
                "remote host identification has changed",
            ],
        );
        self.tls |= contains_any(
            &combined,
            &[
                "ssl certificate problem",
                "certificate verify failed",
                "server certificate verification failed",
                "unable to get local issuer certificate",
            ],
        );
        self.disk_full |= contains_any(&combined, &["no space left on device", "disk full"]);
        self.destination_exists |= combined.contains("destination path")
            && (combined.contains("already exists") || combined.contains("not an empty directory"));
        self.network |= contains_any(
            &combined,
            &[
                "could not resolve host",
                "failed to connect",
                "network is unreachable",
                "connection timed out",
                "unable to access",
                "couldn't connect",
            ],
        );
        self.filesystem |= contains_any(
            &combined,
            &[
                "permission denied",
                "read-only file system",
                "input/output error",
            ],
        );
        self.carry = combined
            .chars()
            .rev()
            .take(256)
            .collect::<String>()
            .chars()
            .rev()
            .collect();
    }

    fn finish(&mut self) {
        self.carry.clear();
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn classify_git_failure(summary: &DiagnosticSummary, status_code: Option<i32>) -> CloneError {
    let class = if summary.auth {
        CloneErrorClass::Auth
    } else if summary.remote_not_found {
        CloneErrorClass::RemoteNotFound
    } else if summary.host_key {
        CloneErrorClass::HostKey
    } else if summary.tls {
        CloneErrorClass::Tls
    } else if summary.disk_full {
        CloneErrorClass::DiskFull
    } else if summary.destination_exists {
        CloneErrorClass::DestinationExists
    } else if summary.network {
        CloneErrorClass::Network
    } else if summary.filesystem {
        CloneErrorClass::Filesystem
    } else {
        CloneErrorClass::GitExited
    };
    let message = match class {
        CloneErrorClass::Auth => "Git could not authenticate with the reviewed remote",
        CloneErrorClass::RemoteNotFound => "The reviewed repository or ref was not found",
        CloneErrorClass::HostKey => "SSH host-key verification failed",
        CloneErrorClass::Tls => "TLS certificate verification failed",
        CloneErrorClass::DiskFull => "The destination filesystem does not have enough free space",
        CloneErrorClass::DestinationExists => "The destination became occupied during acquisition",
        CloneErrorClass::Network => "Git could not reach the reviewed remote",
        CloneErrorClass::Filesystem => "The destination filesystem refused repository acquisition",
        _ => {
            return CloneError::new(
                class,
                status_code.map_or_else(
                    || "Git terminated before repository acquisition completed".to_string(),
                    |code| {
                        format!(
                            "Git exited before repository acquisition completed (status {code})"
                        )
                    },
                ),
            );
        }
    };
    CloneError::new(class, message)
}

// Validation and safe text --------------------------------------------------

fn validate_record_ref(value: &str, label: &'static str) -> Result<(), CloneError> {
    if value.is_empty()
        || value.len() > MAX_RECORD_REF_BYTES
        || value.chars().any(char::is_control)
        || value.trim() != value
        || value.contains("://")
    {
        return policy_denied(match label {
            "clone review record" => "clone review record reference is missing or invalid",
            "source locator record" => "source locator record reference is missing or invalid",
            "checkout plan record" => "checkout plan record reference is missing or invalid",
            "policy decision" => "policy decision reference is missing or invalid",
            "transport decision" => "transport decision reference is missing or invalid",
            "authority ticket" => "authority ticket reference is missing or invalid",
            _ => "reviewed record reference is missing or invalid",
        });
    }
    Ok(())
}

fn validate_git_reference(reference: &str) -> Result<(), CloneError> {
    if reference.is_empty()
        || reference.len() > MAX_REFERENCE_BYTES
        || reference.trim() != reference
        || reference == "@"
        || reference.starts_with('-')
        || reference.starts_with('/')
        || reference.ends_with('/')
        || reference.ends_with('.')
        || reference.contains("..")
        || reference.contains("@{")
        || reference.contains("//")
        || reference
            .chars()
            .any(|character| character.is_control() || " ~^:?*[\\".contains(character))
    {
        return invalid_input("reviewed Git ref is invalid");
    }
    if reference.starts_with("refs/")
        && !reference.starts_with("refs/heads/")
        && !reference.starts_with("refs/tags/")
    {
        return invalid_input("reviewed Git ref must name HEAD, a branch, or a tag");
    }
    if reference != "HEAD"
        && reference.split('/').any(|component| {
            component.is_empty() || component.starts_with('.') || component.ends_with(".lock")
        })
    {
        return invalid_input("reviewed Git ref is invalid");
    }
    Ok(())
}

fn clone_branch_argument(reference: &str) -> Option<&str> {
    if reference == "HEAD" {
        None
    } else {
        Some(
            reference
                .strip_prefix("refs/heads/")
                .or_else(|| reference.strip_prefix("refs/tags/"))
                .unwrap_or(reference),
        )
    }
}

fn validate_commit_oid(oid: &str) -> Result<(), CloneError> {
    if !matches!(oid.len(), 40 | 64) || !oid.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return invalid_input("reviewed commit identity must be a full SHA-1 or SHA-256 OID");
    }
    Ok(())
}

fn parse_git_version(version_line: &str) -> Option<(u32, u32, u32)> {
    let version = version_line.strip_prefix("git version ")?;
    let mut components = version.split('.');
    let major = components.next()?.parse().ok()?;
    let minor = components.next()?.parse().ok()?;
    let patch_text = components.next().unwrap_or("0");
    let patch_digits = patch_text
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    let patch = patch_digits.parse().ok()?;
    Some((major, minor, patch))
}

fn invalid_input<T>(message: &'static str) -> Result<T, CloneError> {
    Err(CloneError::new(CloneErrorClass::InvalidInput, message))
}

fn policy_denied<T>(message: &'static str) -> Result<T, CloneError> {
    Err(CloneError::new(CloneErrorClass::PolicyDenied, message))
}

fn bounded_public_message(message: &str) -> String {
    let sanitized = sanitize_text(message.as_bytes(), MAX_PUBLIC_MESSAGE_BYTES * 4);
    let mut redacted = String::new();
    let mut redact_next_secret = false;
    for token in sanitized.split_whitespace() {
        if !redacted.is_empty() {
            redacted.push(' ');
        }
        let lower = token.to_ascii_lowercase();
        if redact_next_secret {
            redacted.push_str("<redacted-value>");
            redact_next_secret = lower.contains("bearer");
            continue;
        }
        if token.contains("://") {
            redacted.push_str("<redacted-locator>");
            continue;
        }
        let looks_like_secret = [
            "password=",
            "passwd=",
            "token=",
            "secret=",
            "authorization:",
            "bearer",
            "private-token",
            "oauth",
        ]
        .iter()
        .any(|marker| lower.contains(marker));
        let path_candidate = token
            .split_once('=')
            .map_or(token, |(_, value)| value)
            .trim_start_matches(['(', '[', '{', '\'', '"']);
        let bytes = path_candidate.as_bytes();
        let looks_like_drive_path = bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && matches!(bytes[2], b'/' | b'\\');
        if looks_like_secret {
            redacted.push_str("<redacted-value>");
            redact_next_secret = lower.contains("authorization:") || lower.contains("bearer");
        } else if path_candidate.starts_with('/')
            || path_candidate.starts_with("~/")
            || path_candidate.starts_with("\\\\")
            || looks_like_drive_path
        {
            redacted.push_str("<redacted-path>");
        } else {
            redacted.push_str(token);
        }
        if redacted.len() >= MAX_PUBLIC_MESSAGE_BYTES {
            break;
        }
    }
    sanitize_text(redacted.as_bytes(), MAX_PUBLIC_MESSAGE_BYTES)
}

fn sanitize_text(bytes: &[u8], maximum: usize) -> String {
    let text = String::from_utf8_lossy(&bytes[..bytes.len().min(maximum)]);
    let mut sanitized = String::with_capacity(text.len().min(maximum));
    let mut escape_state = EscapeState::Plain;
    for character in text.chars() {
        if sanitized.len() >= maximum {
            break;
        }
        match escape_state {
            EscapeState::AfterEscape => {
                escape_state = match character {
                    '[' => EscapeState::ControlSequence,
                    ']' => EscapeState::OperatingSystemCommand,
                    _ => EscapeState::Plain,
                };
                continue;
            }
            EscapeState::ControlSequence => {
                if ('@'..='~').contains(&character) {
                    escape_state = EscapeState::Plain;
                }
                continue;
            }
            EscapeState::OperatingSystemCommand => {
                if character == '\u{7}' {
                    escape_state = EscapeState::Plain;
                } else if character == '\u{1b}' {
                    escape_state = EscapeState::OperatingSystemCommandEscape;
                }
                continue;
            }
            EscapeState::OperatingSystemCommandEscape => {
                escape_state = if character == '\\' {
                    EscapeState::Plain
                } else {
                    EscapeState::OperatingSystemCommand
                };
                continue;
            }
            EscapeState::Plain => {}
        }
        if character == '\u{1b}' {
            escape_state = EscapeState::AfterEscape;
            continue;
        }
        if character.is_control() && !matches!(character, '\n' | '\r' | '\t') {
            continue;
        }
        if sanitized.len() + character.len_utf8() > maximum {
            break;
        }
        sanitized.push(character);
    }
    sanitized.trim().to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EscapeState {
    Plain,
    AfterEscape,
    ControlSequence,
    OperatingSystemCommand,
    OperatingSystemCommandEscape,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::{Mutex, MutexGuard};

    const TEST_OID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    static PROCESS_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn process_test_lock() -> MutexGuard<'static, ()> {
        PROCESS_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn approval_for(request: &CloneRequest, oid: &str) -> CloneApproval {
        let facts = request.review_facts().expect("review facts");
        CloneApproval {
            review_record_ref: "clone-review:test".to_string(),
            source_locator_record_ref: "source-locator:test".to_string(),
            checkout_plan_record_ref: "checkout-plan:test".to_string(),
            policy_decision_ref: "policy-decision:test".to_string(),
            transport_decision_ref: facts
                .transport
                .is_network()
                .then(|| "transport:test".to_string()),
            reviewed_transport: facts.transport,
            reviewed_normalized_source: facts.normalized_source,
            reviewed_destination_parent: facts.canonical_destination_parent,
            reviewed_destination_leaf_name: facts.destination_leaf_name,
            reviewed_local_source: facts.canonical_local_source,
            reference: CloneRefSelection::new("HEAD", oid),
            topology: CloneTopologyPolicy::inert_full(),
            authentication: CloneAuthentication::Anonymous,
            transport_options: CloneTransportOptions::default(),
            execution_policy: CloneExecutionPolicy {
                overall_timeout_ms: 5_000,
                idle_timeout_ms: 2_000,
            },
            post_clone_action: ClonePostAction::ReviewTrustAndOpen,
        }
    }

    #[test]
    fn locator_allowlist_is_explicit_and_rejects_ambiguous_routes() {
        for (locator, transport) in [
            (
                "https://example.invalid/repo.git",
                CloneTransportClass::Https,
            ),
            (
                "ssh://git@example.invalid/repo.git",
                CloneTransportClass::Ssh,
            ),
            (
                "git+ssh://git@example.invalid/repo.git",
                CloneTransportClass::Ssh,
            ),
            (
                "ssh+git://git@example.invalid/repo.git",
                CloneTransportClass::Ssh,
            ),
            ("ssh://git@[::1]/repo.git", CloneTransportClass::Ssh),
            ("git@example.invalid:repo.git", CloneTransportClass::Ssh),
            ("example.invalid:repo.git", CloneTransportClass::Ssh),
            (
                "git://example.invalid/repo.git",
                CloneTransportClass::GitProtocol,
            ),
        ] {
            assert_eq!(
                ValidatedLocator::parse(locator).expect(locator).transport,
                transport
            );
        }

        for locator in [
            "http://example.invalid/repo.git",
            "file://example.invalid/repo.git",
            "../repo",
            "repo",
            "ext::sh -c payload",
            "helper::payload",
            "ssh://@example.invalid/repo.git",
            "ssh://git@@example.invalid/repo.git",
            "ssh://git@[::1/repo.git",
            "ssh://git@[not-ipv6]/repo.git",
            "ssh://git@example.invalid:0/repo.git",
            "https://user:secret@example.invalid/repo.git",
            "https://user%40example.invalid/repo.git",
            "https://example.invalid/repo.git?token=secret",
            "https://bad..example/repo.git",
            "https://bad_host.example/repo.git",
            "https://example.invalid/repo path.git",
            "https://example.invalid/repo\\path.git",
            "https://exämple.invalid/repo.git",
            "git@example.invalid:-oProxyCommand=payload",
            "git@example.invalid:repo;payload",
        ] {
            let error = ValidatedLocator::parse(locator).expect_err(locator);
            assert_eq!(error.class(), CloneErrorClass::InvalidInput, "{locator}");
            assert!(!error.message().contains("secret"));
            assert!(!error.message().contains("payload"));
        }
    }

    #[test]
    fn local_and_file_routes_bind_one_canonical_source() {
        let parent = tempfile::tempdir().expect("temp dir");
        let source = parent.path().join("source");
        fs::create_dir(&source).expect("source");
        let canonical = fs::canonicalize(&source).expect("canonical source");

        let local = ValidatedLocator::parse(source.to_str().expect("utf8 path")).expect("local");
        assert_eq!(local.transport, CloneTransportClass::LocalFilesystem);
        assert_eq!(local.local_source.expect("binding").path, canonical);

        #[cfg(unix)]
        {
            let file_url = format!("file://{}", source.display());
            let file = ValidatedLocator::parse(&file_url).expect("file URL");
            assert_eq!(file.transport, CloneTransportClass::LocalFilesystem);
            assert_eq!(file.local_source.expect("binding").path, canonical);
        }
    }

    #[test]
    fn request_is_preview_only_and_approval_binds_reviewed_parent_and_records() {
        let parent = tempfile::tempdir().expect("temp dir");
        let request = CloneRequest::new(
            "https://example.invalid/repo.git",
            parent.path().join("repo"),
        );
        let facts = request.review_facts().expect("review facts");
        assert_eq!(facts.transport, CloneTransportClass::Https);

        let mut approval = approval_for(&request, TEST_OID);
        approval.transport_decision_ref = None;
        assert_eq!(
            request.clone().approve(approval).unwrap_err().class(),
            CloneErrorClass::PolicyDenied
        );

        let mut approval = approval_for(&request, TEST_OID);
        approval.reviewed_destination_parent = parent.path().join("not-reviewed");
        assert_eq!(
            request.clone().approve(approval).unwrap_err().class(),
            CloneErrorClass::PolicyDenied
        );

        let mut approval = approval_for(&request, TEST_OID);
        approval.reviewed_transport = CloneTransportClass::Ssh;
        assert_eq!(
            request.clone().approve(approval).unwrap_err().class(),
            CloneErrorClass::PolicyDenied
        );

        let mut approval = approval_for(&request, TEST_OID);
        approval.reviewed_normalized_source = "https://other.invalid/different.git".to_string();
        let error = request.clone().approve(approval).unwrap_err();
        assert_eq!(error.class(), CloneErrorClass::PolicyDenied);
        assert!(!error.message().contains("other.invalid"));

        let mut approval = approval_for(&request, TEST_OID);
        approval.reviewed_destination_leaf_name = OsString::from("different-repo");
        assert_eq!(
            request.clone().approve(approval).unwrap_err().class(),
            CloneErrorClass::PolicyDenied
        );

        let mut approval = approval_for(&request, TEST_OID);
        approval.topology.recurse_submodules = true;
        assert_eq!(
            request.clone().approve(approval).unwrap_err().class(),
            CloneErrorClass::PolicyDenied
        );

        let approval = approval_for(&request, TEST_OID);
        let serialized = serde_json::to_string(&approval).expect("approval serializes");
        let decoded: CloneApproval = serde_json::from_str(&serialized).expect("approval decodes");
        assert_eq!(decoded, approval);

        let execution = request.approve(approval).expect("approval");
        let debug = format!("{execution:?}");
        assert!(!debug.contains("example.invalid"));
        assert!(!debug.contains(parent.path().to_string_lossy().as_ref()));
    }

    #[test]
    fn request_rejects_all_existing_destination_kinds() {
        let parent = tempfile::tempdir().expect("temp dir");
        for name in ["empty", "occupied", "file"] {
            let target = parent.path().join(name);
            if name == "file" {
                fs::write(&target, "occupied").expect("file");
            } else {
                fs::create_dir(&target).expect("directory");
                if name == "occupied" {
                    fs::write(target.join("README"), "occupied").expect("content");
                }
            }
            let error = CloneRequest::new("https://example.invalid/repo.git", &target)
                .review_facts()
                .unwrap_err();
            assert_eq!(error.class(), CloneErrorClass::DestinationExists);
            assert!(!error.message().contains(target.to_string_lossy().as_ref()));
        }
    }

    #[test]
    fn review_rejects_lexically_ambiguous_paths_before_canonicalization() {
        let parent = tempfile::tempdir().expect("temp dir");
        let destination = PathBuf::from(format!("{}/./repo", parent.path().display()));
        let error = CloneRequest::new("https://example.invalid/repo.git", destination)
            .review_facts()
            .unwrap_err();
        assert_eq!(error.class(), CloneErrorClass::InvalidInput);

        let source = parent.path().join("source");
        fs::create_dir(&source).expect("source");
        let ambiguous_source = format!("{}/./source", parent.path().display());
        let error = ValidatedLocator::parse(&ambiguous_source).unwrap_err();
        assert_eq!(error.class(), CloneErrorClass::InvalidInput);
    }

    #[cfg(unix)]
    #[test]
    fn destination_collision_does_not_follow_symlinks() {
        use std::os::unix::fs::symlink;

        let parent = tempfile::tempdir().expect("temp dir");
        let victim = parent.path().join("victim");
        let destination = parent.path().join("repo");
        fs::create_dir(&victim).expect("victim");
        symlink(&victim, &destination).expect("symlink");

        let error = CloneRequest::new("https://example.invalid/repo.git", &destination)
            .review_facts()
            .unwrap_err();
        assert_eq!(error.class(), CloneErrorClass::DestinationExists);
        assert!(victim.is_dir());
    }

    #[test]
    fn ref_version_and_deadline_validation_are_closed() {
        for reference in [
            "",
            "@",
            "-branch",
            "refs/heads/a..b",
            "refs/heads/a b",
            "refs/heads/.hidden",
            "refs/heads/main.lock",
            "a@{1}",
        ] {
            assert!(validate_git_reference(reference).is_err(), "{reference}");
        }
        assert!(validate_git_reference("HEAD").is_ok());
        assert!(validate_git_reference("refs/heads/main").is_ok());
        assert!(validate_git_reference("refs/pull/1/head").is_err());
        assert_eq!(clone_branch_argument("refs/heads/main"), Some("main"));
        assert_eq!(clone_branch_argument("refs/tags/v1"), Some("v1"));
        assert!(validate_commit_oid(TEST_OID).is_ok());
        assert!(validate_commit_oid("abc").is_err());
        assert_eq!(
            parse_git_version("git version 2.39.5 (Apple Git-154)"),
            Some((2, 39, 5))
        );
        assert_eq!(parse_git_version("git version 2.30.0"), Some((2, 30, 0)));
        assert!(CloneExecutionPolicy {
            overall_timeout_ms: 100,
            idle_timeout_ms: 101,
        }
        .validate()
        .is_err());
    }

    #[cfg(unix)]
    fn write_executable(path: &Path, body: &str) {
        use std::os::unix::fs::PermissionsExt;
        fs::write(path, body).expect("write executable");
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod executable");
    }

    #[cfg(unix)]
    fn successful_fake_git(root: &Path) -> PathBuf {
        let fake = root.join("fake-git");
        write_executable(
            &fake,
            &format!(
                r#"#!/bin/sh
case " $* " in
  *" --version "*|*"fake-git --version"*) printf 'git version 2.39.5\n'; exit 0 ;;
esac
command_name=''
destination=''
previous=''
for argument in "$@"; do
  if [ "$previous" = '-C' ]; then destination="$argument"; fi
  case "$argument" in clone|rev-parse|checkout) command_name="$argument" ;; esac
  previous="$argument"
done
case "$command_name" in
  clone)
    eval "destination=\${{$#}}"
    /bin/mkdir -p "$destination/.git/objects" "$destination/.git/refs"
    printf 'ref: refs/heads/main\n' > "$destination/.git/HEAD"
    printf '[core]\n' > "$destination/.git/config"
    printf 'Receiving objects: 50%%\n' >&2
    ;;
  rev-parse) printf '{}\n' ;;
  checkout)
    printf 'safe materialization\n' > "$destination/README.md"
    printf 'Checking out files: 100%%\n' >&2
    ;;
esac
"#,
                TEST_OID
            ),
        );
        fake
    }

    #[cfg(unix)]
    #[test]
    fn approved_execution_uses_isolated_environment_and_exact_oid() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = successful_fake_git(parent.path());
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let approval = approval_for(&request, TEST_OID);
        let execution = request.approve(approval).expect("approval");
        let backend = SystemGitCloneBackend::new(&fake_git);
        let mut events = Vec::new();
        let outcome = backend
            .clone_repository(execution, &mut |event| events.push(event))
            .expect("clone succeeds");

        assert_eq!(outcome.materialized_commit_oid, TEST_OID);
        assert!(outcome.trust_and_setup_pending);
        assert!(destination.join("README.md").is_file());
        assert_eq!(events.first().unwrap().phase, CloneProgressPhase::Starting);
        assert_eq!(events.last().unwrap().phase, CloneProgressPhase::Completed);
        assert!(events.last().unwrap().message.contains("pending"));
    }

    #[cfg(unix)]
    #[test]
    fn progress_observer_failure_preserves_completed_bytes_as_partial() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = successful_fake_git(parent.path());
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let execution = request
            .clone()
            .approve(approval_for(&request, TEST_OID))
            .expect("approval");
        let failure = SystemGitCloneBackend::new(fake_git)
            .clone_repository(execution, &mut |event| {
                if event.phase == CloneProgressPhase::Completed {
                    panic!("simulated completed-event observer failure");
                }
            })
            .unwrap_err();

        assert_eq!(failure.error().class(), CloneErrorClass::Io);
        assert_eq!(
            failure.interrupted_state(),
            CloneInterruptedState::InterruptedOpenReadOnlyAvailable
        );
        assert!(destination.join("README.md").is_file());
        failure
            .into_partial()
            .expect("partial")
            .discard()
            .expect("discard");
    }

    #[cfg(unix)]
    #[test]
    fn command_environment_has_no_ambient_config_or_credentials() {
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = successful_fake_git(parent.path());
        let request = CloneRequest::new(
            "https://example.invalid/repo.git",
            parent.path().join("repo"),
        );
        let execution = request
            .clone()
            .approve(approval_for(&request, TEST_OID))
            .expect("approval");
        let git = PinnedExecutable::resolve(&fake_git, "missing").expect("git");
        let guard = GitGuard::create(&execution.destination.canonical_parent).expect("guard");
        let command =
            build_clone_command(&git, &execution, &execution.destination.target_path, &guard)
                .expect("command");
        let environment = command
            .get_envs()
            .map(|(key, value)| {
                (
                    key.to_string_lossy().into_owned(),
                    value.map(OsStr::to_owned),
                )
            })
            .collect::<BTreeMap<_, _>>();

        for absent in [
            "GIT_SSL_NO_VERIFY",
            "HTTPS_PROXY",
            "HTTP_PROXY",
            "ALL_PROXY",
            "GIT_CONFIG_PARAMETERS",
            "GIT_CONFIG_COUNT",
            "GIT_SSH_COMMAND",
            "SSH_AUTH_SOCK",
        ] {
            assert!(!environment.contains_key(absent), "{absent}");
        }
        assert_eq!(environment.get("LC_ALL"), Some(&Some("C".into())));
        assert_eq!(
            environment.get("GIT_TERMINAL_PROMPT"),
            Some(&Some("0".into()))
        );
        assert_eq!(
            environment.get("GIT_CONFIG_NOSYSTEM"),
            Some(&Some("1".into()))
        );
        let arguments = command
            .get_args()
            .map(|value| value.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert!(arguments.iter().any(|value| value == "credential.helper="));
        assert!(arguments.iter().any(|value| value == "http.sslVerify=true"));
        assert!(arguments
            .iter()
            .any(|value| value == "http.followRedirects=false"));
        assert!(arguments
            .iter()
            .any(|value| value == "protocol.allow=never"));
    }

    #[cfg(unix)]
    #[test]
    fn ssh_projection_is_current_canonical_and_strict() {
        use std::os::unix::net::UnixListener;

        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = successful_fake_git(parent.path());
        let fake_ssh = parent.path().join("ssh");
        write_executable(&fake_ssh, "#!/bin/sh\nexit 1\n");
        let agent = parent.path().join("agent.sock");
        let known_hosts = parent.path().join("known_hosts");
        let _agent_listener = UnixListener::bind(&agent).expect("agent fixture");
        fs::write(&known_hosts, "example.invalid test-key\n").expect("known hosts");
        let fake_ssh = fs::canonicalize(fake_ssh).expect("canonical ssh");
        let agent = fs::canonicalize(agent).expect("canonical agent");
        let known_hosts = fs::canonicalize(known_hosts).expect("canonical known hosts");

        let request = CloneRequest::new("git@example.invalid:repo.git", parent.path().join("repo"));
        let mut approval = approval_for(&request, TEST_OID);
        approval.authentication = CloneAuthentication::SshAgent {
            authority_ticket_ref: "authority:test".to_string(),
            authority_expires_at_unix_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_secs()
                + 60,
            ssh_auth_sock: agent.clone(),
            known_hosts_file: known_hosts.clone(),
            ssh_binary: fake_ssh,
        };
        let execution = request.approve(approval).expect("SSH approval");
        let git = PinnedExecutable::resolve(&fake_git, "missing").expect("git");
        let guard = GitGuard::create(&execution.destination.canonical_parent).expect("guard");
        let command =
            build_clone_command(&git, &execution, &execution.destination.target_path, &guard)
                .expect("command");
        let environment = command
            .get_envs()
            .map(|(key, value)| {
                (
                    key.to_string_lossy().into_owned(),
                    value.map(OsStr::to_owned),
                )
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            environment.get("SSH_AUTH_SOCK"),
            Some(&Some(agent.into_os_string()))
        );
        let ssh_command = environment
            .get("GIT_SSH_COMMAND")
            .and_then(Option::as_ref)
            .expect("SSH command")
            .to_string_lossy();
        for required in [
            "BatchMode=yes",
            "RequestTTY=no",
            "PasswordAuthentication=no",
            "KbdInteractiveAuthentication=no",
            "StrictHostKeyChecking=yes",
            "NumberOfPasswordPrompts=0",
        ] {
            assert!(ssh_command.contains(required), "{required}");
        }
        assert!(ssh_command.contains(known_hosts.to_string_lossy().as_ref()));
    }

    #[cfg(unix)]
    #[test]
    fn failure_before_useful_git_state_is_removed() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = parent.path().join("fake-git");
        write_executable(
            &fake_git,
            "#!/bin/sh\ncase \" $* \" in *\" --version \"*|*\"fake-git --version\"*) printf 'git version 2.39.5\\n'; exit 0;; esac\nprintf 'fatal: Authentication failed\\n' >&2\nexit 128\n",
        );
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let execution = request
            .clone()
            .approve(approval_for(&request, TEST_OID))
            .expect("approval");
        let failure = SystemGitCloneBackend::new(fake_git)
            .clone_repository(execution, &mut |_| {})
            .unwrap_err();
        assert_eq!(failure.error().class(), CloneErrorClass::Auth);
        assert_eq!(
            failure.interrupted_state(),
            CloneInterruptedState::NoPartialBytes
        );
        assert!(!destination.exists());
    }

    #[cfg(unix)]
    #[test]
    fn reviewed_oid_drift_preserves_partial_and_blocks_checkout() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = parent.path().join("fake-git");
        let observed_oid = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        write_executable(
            &fake_git,
            &format!(
                r#"#!/bin/sh
case " $* " in *" --version "*|*"fake-git --version"*) printf 'git version 2.39.5\n'; exit 0;; esac
command_name=''; destination=''
for argument in "$@"; do case "$argument" in clone|rev-parse|checkout) command_name="$argument" ;; esac; done
case "$command_name" in
 clone) eval "destination=\${{$#}}"; /bin/mkdir -p "$destination/.git/objects"; printf x > "$destination/.git/HEAD" ;;
 rev-parse) printf '{}\n' ;;
 checkout) exit 99 ;;
esac
"#,
                observed_oid
            ),
        );
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let execution = request
            .clone()
            .approve(approval_for(&request, TEST_OID))
            .expect("approval");
        let failure = SystemGitCloneBackend::new(fake_git)
            .clone_repository(execution, &mut |_| {})
            .unwrap_err();
        assert_eq!(failure.error().class(), CloneErrorClass::RefMismatch);
        assert_eq!(
            failure.interrupted_state(),
            CloneInterruptedState::InterruptedResumable
        );
        assert!(destination.join(".git/HEAD").is_file());
        failure
            .into_partial()
            .expect("partial")
            .discard()
            .expect("discard");
    }

    #[cfg(unix)]
    #[test]
    fn probe_enforces_the_documented_git_version_floor() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = parent.path().join("fake-git");
        write_executable(&fake_git, "#!/bin/sh\nprintf 'git version 2.29.9\\n'\n");
        let error = SystemGitCloneBackend::new(fake_git)
            .probe()
            .expect_err("old Git is rejected");
        assert_eq!(error.class(), CloneErrorClass::GitVersionUnsupported);
    }

    #[cfg(unix)]
    #[test]
    fn meaningful_timeout_is_preserved_and_discard_is_identity_safe() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = parent.path().join("fake-git");
        write_executable(
            &fake_git,
            "#!/bin/sh\ncase \" $* \" in *\" --version \"*|*\"fake-git --version\"*) printf 'git version 2.39.5\\n'; exit 0;; esac\neval \"destination=\\${$#}\"\n/bin/mkdir -p \"$destination/.git/objects\"\nprintf 'ref: refs/heads/main\\n' > \"$destination/.git/HEAD\"\nprintf 'Receiving objects: 1%%\\n' >&2\n/bin/sleep 5\n",
        );
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let mut approval = approval_for(&request, TEST_OID);
        approval.execution_policy = CloneExecutionPolicy {
            overall_timeout_ms: 3_000,
            idle_timeout_ms: 2_000,
        };
        let execution = request.approve(approval).expect("approval");
        let failure = SystemGitCloneBackend::new(fake_git)
            .clone_repository(execution, &mut |_| {})
            .unwrap_err();
        assert_eq!(failure.error().class(), CloneErrorClass::Timeout);
        assert_eq!(
            failure.interrupted_state(),
            CloneInterruptedState::InterruptedResumable,
            "{failure:?}"
        );
        assert!(destination.join(".git/HEAD").is_file());
        failure
            .into_partial()
            .expect("partial")
            .discard()
            .expect("discard");
        assert!(!destination.exists());
    }

    #[cfg(unix)]
    #[test]
    fn checkout_failure_exposes_read_only_partial_without_raw_stderr() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = parent.path().join("fake-git");
        write_executable(
            &fake_git,
            &format!(
                r#"#!/bin/sh
case " $* " in *" --version "*|*"fake-git --version"*) printf 'git version 2.39.5\n'; exit 0;; esac
command_name=''; destination=''; previous=''
for argument in "$@"; do
  if [ "$previous" = '-C' ]; then destination="$argument"; fi
  case "$argument" in clone|rev-parse|checkout) command_name="$argument" ;; esac
  previous="$argument"
done
case "$command_name" in
 clone) eval "destination=\${{$#}}"; /bin/mkdir -p "$destination/.git/objects"; printf x > "$destination/.git/HEAD" ;;
 rev-parse) printf '{}\n' ;;
 checkout) printf partial > "$destination/PARTIAL"; printf 'fatal: token=TOP-SECRET\n' >&2; exit 128 ;;
esac
"#,
                TEST_OID
            ),
        );
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let execution = request
            .clone()
            .approve(approval_for(&request, TEST_OID))
            .expect("approval");
        let failure = SystemGitCloneBackend::new(fake_git)
            .clone_repository(execution, &mut |_| {})
            .unwrap_err();
        assert_eq!(
            failure.interrupted_state(),
            CloneInterruptedState::InterruptedOpenReadOnlyAvailable
        );
        assert!(!failure.error().message().contains("TOP-SECRET"));
        assert!(destination.join("PARTIAL").is_file());
        failure
            .into_partial()
            .expect("partial")
            .discard()
            .expect("discard");
    }

    #[cfg(unix)]
    #[test]
    fn cancellation_kills_and_reaps_the_clone_process_group() {
        let _process_lock = process_test_lock();
        let parent = tempfile::tempdir().expect("temp dir");
        let fake_git = parent.path().join("fake-git");
        write_executable(
            &fake_git,
            "#!/bin/sh\ncase \" $* \" in *\" --version \"*|*\"fake-git --version\"*) printf 'git version 2.39.5\\n'; exit 0;; esac\neval \"destination=\\${$#}\"\n/bin/mkdir -p \"$destination/.git/objects\"\nprintf x > \"$destination/.git/HEAD\"\nprintf 'Receiving objects: 1%%\\n' >&2\n/bin/sleep 5\n",
        );
        let destination = parent.path().join("repo");
        let request = CloneRequest::new("https://example.invalid/repo.git", &destination);
        let execution = request
            .clone()
            .approve(approval_for(&request, TEST_OID))
            .expect("approval");
        let cancellation = execution.cancellation_token();
        let callback_cancellation = cancellation.clone();
        let failure = SystemGitCloneBackend::new(fake_git)
            .clone_repository(execution, &mut |event| {
                if event.phase == CloneProgressPhase::Progress {
                    callback_cancellation.cancel();
                }
            })
            .unwrap_err();
        assert_eq!(failure.error().class(), CloneErrorClass::Cancelled);
        failure
            .into_partial()
            .expect("partial")
            .discard()
            .expect("discard");
    }

    #[cfg(unix)]
    #[test]
    fn supervisor_enforces_output_and_descendant_pipe_bounds() {
        let parent = tempfile::tempdir().expect("temp dir");
        let noisy = parent.path().join("noisy");
        write_executable(
            &noisy,
            "#!/bin/sh\ni=0\nwhile [ \"$i\" -lt 512 ]; do printf x; i=$((i + 1)); done\n",
        );
        let mut noisy_command = Command::new(noisy);
        noisy_command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_process_group(&mut noisy_command);
        let error = match run_supervised(
            &mut noisy_command,
            Duration::from_secs(2),
            Duration::from_secs(1),
            &CloneCancellationToken::new(),
            64,
            None,
        ) {
            Ok(_) => panic!("bounded stdout must reject overflow"),
            Err(error) => error,
        };
        assert_eq!(error.class(), CloneErrorClass::OutputLimit);

        let retaining = parent.path().join("retaining");
        write_executable(&retaining, "#!/bin/sh\n/bin/sleep 5 &\nexit 0\n");
        let mut retaining_command = Command::new(retaining);
        retaining_command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_process_group(&mut retaining_command);
        let started = Instant::now();
        let error = match run_supervised(
            &mut retaining_command,
            Duration::from_secs(3),
            Duration::from_secs(2),
            &CloneCancellationToken::new(),
            64,
            None,
        ) {
            Ok(_) => panic!("a descendant retaining pipes must fail closed"),
            Err(error) => error,
        };
        assert_eq!(error.class(), CloneErrorClass::Io);
        assert!(started.elapsed() < Duration::from_secs(2));

        let callback_failure = parent.path().join("callback-failure");
        write_executable(
            &callback_failure,
            "#!/bin/sh\nprintf 'Receiving objects: 1%%\\n' >&2\n/bin/sleep 5\n",
        );
        let mut callback_command = Command::new(callback_failure);
        callback_command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_process_group(&mut callback_command);
        let mut state = ProgressState::default();
        let mut callback = |_| panic!("simulated UI callback panic");
        let started = Instant::now();
        let error = match run_supervised(
            &mut callback_command,
            Duration::from_secs(3),
            Duration::from_secs(2),
            &CloneCancellationToken::new(),
            64,
            Some(ProgressSink {
                state: &mut state,
                callback: &mut callback,
            }),
        ) {
            Ok(_) => panic!("a failed progress observer must stop supervision"),
            Err(error) => error,
        };
        assert_eq!(error.class(), CloneErrorClass::Io);
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[cfg(unix)]
    #[test]
    fn quarantine_refuses_to_delete_a_replacement_symlink() {
        use std::os::unix::fs::symlink;

        let parent = tempfile::tempdir().expect("temp dir");
        let requested = parent.path().join("repo");
        let reviewed = ReviewedDestination::capture(
            &requested,
            &fs::canonicalize(parent.path()).expect("parent"),
        )
        .expect("reviewed");
        let mut owned = OwnedDestination::create(&reviewed).expect("owned destination");
        let victim = parent.path().join("victim");
        fs::create_dir(&victim).expect("victim");
        fs::write(victim.join("KEEP"), "keep").expect("marker");
        fs::remove_dir(&owned.path).expect("remove owned");
        symlink(&victim, &owned.path).expect("replacement");

        assert!(owned.rollback().is_err());
        owned.active = false;
        assert!(victim.join("KEEP").is_file());
        assert!(fs::symlink_metadata(&requested)
            .expect("replacement retained")
            .file_type()
            .is_symlink());
    }

    #[test]
    fn repository_verification_detects_git_directory_identity_replacement() {
        let parent = tempfile::tempdir().expect("temp dir");
        let requested = parent.path().join("repo");
        let reviewed = ReviewedDestination::capture(
            &requested,
            &fs::canonicalize(parent.path()).expect("parent"),
        )
        .expect("reviewed");
        let mut owned = OwnedDestination::create(&reviewed).expect("owned destination");
        let git = owned.path.join(".git");
        fs::create_dir(&git).expect("git directory");
        let identity = FilesystemIdentity::capture(&git).expect("git identity");
        fs::remove_dir(&git).expect("remove git directory");
        fs::create_dir(&git).expect("replace git directory");

        let error = verify_owned_repository(&owned, Some(&identity)).unwrap_err();
        assert_eq!(error.class(), CloneErrorClass::Filesystem);
        owned.rollback().expect("cleanup");
    }

    #[test]
    fn repository_verification_rejects_borrowed_object_stores() {
        let parent = tempfile::tempdir().expect("temp dir");
        let requested = parent.path().join("repo");
        let reviewed = ReviewedDestination::capture(
            &requested,
            &fs::canonicalize(parent.path()).expect("parent"),
        )
        .expect("reviewed");
        let mut owned = OwnedDestination::create(&reviewed).expect("owned destination");
        let objects_info = owned.path.join(".git/objects/info");
        fs::create_dir_all(&objects_info).expect("object info");
        fs::write(
            objects_info.join("alternates"),
            "/unreviewed/object/store\n",
        )
        .expect("alternate");

        let error = verify_owned_repository(&owned, None).unwrap_err();
        assert_eq!(error.class(), CloneErrorClass::PolicyDenied);
        assert!(!error.message().contains("unreviewed"));
        owned.rollback().expect("cleanup");
    }

    #[test]
    fn errors_progress_and_request_debug_are_bounded_and_redacted() {
        let request = CloneRequest::new(
            "https://user:secret@example.invalid/repo.git?token=secret",
            "/sensitive/destination",
        );
        let debug = format!("{request:?}");
        assert!(!debug.contains("secret"));
        assert!(!debug.contains("sensitive"));
        let reference = CloneRefSelection::new(
            "refs/heads/private-customer-branch",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        );
        let debug = format!("{reference:?}");
        assert!(!debug.contains("private-customer"));
        assert!(!debug.contains("bbbbbbbb"));

        let message = format!(
            "\u{1b}[31m{}\u{1b}[0m",
            "x".repeat(MAX_PUBLIC_MESSAGE_BYTES * 2)
        );
        let error = CloneError::new(CloneErrorClass::Io, message);
        assert!(error.message().len() <= MAX_PUBLIC_MESSAGE_BYTES);
        assert!(!error.message().contains('\u{1b}'));
        let error = CloneError::new(
            CloneErrorClass::Io,
            "failed at https://user:secret@example.invalid/repo?token=secret /private/path token=secret",
        );
        assert!(!error.message().contains("user"));
        assert!(!error.message().contains("private"));
        assert!(!error.message().contains("secret"));
        let error = CloneError::new(
            CloneErrorClass::Io,
            "failure at (https://example.invalid/private) path=/customer/repository Authorization: Bearer opaque-value",
        );
        assert!(!error.message().contains("example.invalid"));
        assert!(!error.message().contains("customer"));
        assert!(!error.message().contains("opaque-value"));
    }

    #[test]
    fn diagnostic_classifier_keeps_only_typed_flags() {
        for (diagnostic, class) in [
            (
                "fatal: Authentication failed for SECRET",
                CloneErrorClass::Auth,
            ),
            (
                "requested URL returned error: 403 SECRET",
                CloneErrorClass::Auth,
            ),
            ("SSL certificate problem: SECRET", CloneErrorClass::Tls),
            (
                "Host key verification failed SECRET",
                CloneErrorClass::HostKey,
            ),
            ("No space left on device SECRET", CloneErrorClass::DiskFull),
        ] {
            let mut summary = DiagnosticSummary::default();
            summary.ingest(diagnostic.as_bytes());
            summary.finish();
            let error = classify_git_failure(&summary, Some(128));
            assert_eq!(error.class(), class);
            assert!(!error.message().contains("SECRET"));
        }
    }

    #[test]
    fn public_execution_signature_consumes_approved_authority() {
        fn compile_signature<B: GitCloneBackend>(
            backend: &B,
            approved: ApprovedCloneExecution,
        ) -> Result<CloneOutcome, CloneFailure> {
            backend.clone_repository(approved, &mut |_| {})
        }
        let _ = compile_signature::<SystemGitCloneBackend>;
    }

    #[test]
    fn system_git_local_clone_materializes_without_filters_or_lfs() {
        let _process_lock = process_test_lock();
        let backend = SystemGitCloneBackend::default();
        if backend.probe().is_err() {
            return;
        }
        let parent = tempfile::tempdir().expect("temp dir");
        let source = parent.path().join("source");
        let destination = parent.path().join("destination");
        fs::create_dir(&source).expect("source");

        let run = |arguments: &[&str]| {
            let status = Command::new("git")
                .args(arguments)
                .env_clear()
                .env("PATH", std::env::var_os("PATH").unwrap_or_default())
                .env("LC_ALL", "C")
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .env("GIT_CONFIG_GLOBAL", null_device_path())
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("run seed git");
            assert!(status.success(), "seed git command failed: {arguments:?}");
        };
        let source_text = source.to_str().expect("source utf8");
        run(&["init", "--quiet", "--template=", source_text]);
        fs::write(
            source.join(".gitattributes"),
            "payload.txt filter=untrusted-required\nlarge.bin filter=lfs diff=lfs merge=lfs -text\n",
        )
        .expect("attributes");
        fs::write(source.join("payload.txt"), "repository bytes\n").expect("payload");
        let lfs_pointer = "version https://git-lfs.github.com/spec/v1\noid sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\nsize 123\n";
        fs::write(source.join("large.bin"), lfs_pointer).expect("LFS pointer");
        run(&["-C", source_text, "add", "--", "."]);
        run(&[
            "-C",
            source_text,
            "-c",
            "user.name=Aureline Test",
            "-c",
            "user.email=aureline-test@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "seed",
        ]);
        let oid_output = Command::new("git")
            .args(["-C", source_text, "rev-parse", "HEAD^{commit}"])
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("LC_ALL", "C")
            .output()
            .expect("resolve oid");
        assert!(oid_output.status.success());
        let oid = String::from_utf8(oid_output.stdout)
            .expect("oid utf8")
            .trim()
            .to_string();

        let request = CloneRequest::new(source_text, &destination);
        let execution = request
            .clone()
            .approve(approval_for(&request, &oid))
            .expect("approval");
        let outcome = backend
            .clone_repository(execution, &mut |_| {})
            .expect("safe local clone");
        assert_eq!(outcome.materialized_commit_oid, oid);
        assert_eq!(
            fs::read_to_string(destination.join("payload.txt")).expect("payload"),
            "repository bytes\n"
        );
        assert_eq!(
            fs::read_to_string(destination.join("large.bin")).expect("pointer"),
            lfs_pointer
        );
    }
}
