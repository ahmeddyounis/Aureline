//! Session plans, append-only attempt-record histories, and target / toolchain /
//! env lineage normalized across local, remote, notebook, and provider-backed
//! test flows for the M5 test-intelligence lane.
//!
//! Where [`crate::scope_compatible_selection_objects_and_widened_selection_review`]
//! lands the **selection objects** that make durable discovery targets safely
//! re-runnable, this module makes the *actual execution* of those selections
//! attributable. A run is not a transient line of terminal output: it is a
//! [`SessionPlan`] (what was asked to run, in which flow, against which target
//! class, with which retry / watch policy and runtime / toolchain / env lineage)
//! and an append-only history of [`AttemptRecord`]s (each concrete execution or
//! imported join, chained to its predecessor).
//!
//! * a [`SessionPlan`] ties a stable session id to a [`SessionFlow`]
//!   (`local` / `remote` / `notebook` / `imported_provider`), a [`SessionPlanMode`]
//!   intent, the selection / snapshot it executes, an [`ExecutionLineage`] block,
//!   and a [`RetryPolicyClass`] / [`WatchPolicyClass`];
//! * an [`AttemptRecord`] is one append-only attempt in a session, carrying its
//!   own [`ExecutionLineage`] so an imported / provider-backed attempt can never
//!   masquerade as a local rerun: its [`LineageProvenanceClass`] and `imported`
//!   flag travel with the attempt, and its [`AttemptOutcome`] is drawn from the
//!   imported vocabulary rather than the local pass / fail vocabulary;
//! * local reruns, notebook-backed tests, remote targets, and imported CI failure
//!   joins all normalize onto the *same* session and attempt ledger — a single
//!   session can hold a local initial run, a local parity rerun, and an imported
//!   join attempt, with per-attempt lineage keeping each one honest.
//!
//! [`SessionAttemptLedgerPacket::validate`] refuses a packet that collapses a
//! parameterized template into a concrete invocation, lets an imported attempt
//! read as a local rerun, breaks the append-only attempt chain, hides a stale or
//! quarantine-bearing attempt behind a generic green outcome, or records a
//! session / attempt that support, notifications, or review packets cannot reopen
//! from the export alone.
//!
//! Raw test source, raw provider payloads, provider cursors, credentials, and raw
//! artifact bodies never cross this boundary; the packet carries only typed class
//! tokens, booleans, opaque ids, fingerprint digests, and redaction-aware
//! reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json`](../../../../schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json).
//! The contract doc is
//! [`docs/testing/m5/session-plans-attempt-records-and-execution-lineage.md`](../../../../docs/testing/m5/session-plans-attempt-records-and-execution-lineage.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/session-plans-attempt-records-and-execution-lineage/`](../../../../fixtures/testing/m5/session-plans-attempt-records-and-execution-lineage/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

/// Stable record-kind tag carried by [`SessionAttemptLedgerPacket`].
pub const SESSION_ATTEMPT_LEDGER_RECORD_KIND: &str = "test_session_attempt_ledger_packet";

/// Schema version for the session / attempt ledger packet.
pub const SESSION_ATTEMPT_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SESSION_ATTEMPT_LEDGER_SCHEMA_REF: &str =
    "schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json";

/// Repo-relative path of the contract doc.
pub const SESSION_ATTEMPT_LEDGER_DOC_REF: &str =
    "docs/testing/m5/session-plans-attempt-records-and-execution-lineage.md";

/// Repo-relative path of the checked support-export artifact.
pub const SESSION_ATTEMPT_LEDGER_ARTIFACT_REF: &str =
    "artifacts/testing/m5/session-plans-attempt-records-and-execution-lineage/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SESSION_ATTEMPT_LEDGER_SUMMARY_REF: &str =
    "artifacts/testing/m5/session-plans-attempt-records-and-execution-lineage.md";

/// Repo-relative path of the protected fixture directory.
pub const SESSION_ATTEMPT_LEDGER_FIXTURE_DIR: &str =
    "fixtures/testing/m5/session-plans-attempt-records-and-execution-lineage";

/// Closed vocabulary for the flow a session or attempt belongs to. The flow is
/// the attributability axis the M5 lane normalizes onto: a single ledger can hold
/// attempts from more than one flow, and per-attempt lineage keeps each honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionFlow {
    /// Executed in the local workspace.
    LocalWorkspace,
    /// Executed against a remote target / host.
    RemoteTarget,
    /// Executed inside a notebook kernel.
    NotebookKernel,
    /// Imported / provider-backed evidence (CI join); never a local execution.
    ImportedProvider,
}

impl SessionFlow {
    /// Every flow, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalWorkspace,
        Self::RemoteTarget,
        Self::NotebookKernel,
        Self::ImportedProvider,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RemoteTarget => "remote_target",
            Self::NotebookKernel => "notebook_kernel",
            Self::ImportedProvider => "imported_provider",
        }
    }

    /// Whether this flow is imported / provider-backed rather than locally
    /// executed.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedProvider)
    }
}

/// Closed vocabulary naming what a session plan intends. Run, watch, rerun-failed,
/// and imported-CI joins all normalize onto one session object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionPlanMode {
    /// Run the selected tests once.
    RunSelected,
    /// Keep a watch session armed over the selected scope.
    Watch,
    /// Rerun the entire originating selection.
    RerunAll,
    /// Rerun only the failed subset of the originating selection.
    RerunFailed,
    /// Join imported / provider-backed CI evidence into the ledger.
    ImportProviderJoin,
}

impl SessionPlanMode {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunSelected,
        Self::Watch,
        Self::RerunAll,
        Self::RerunFailed,
        Self::ImportProviderJoin,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunSelected => "run_selected",
            Self::Watch => "watch",
            Self::RerunAll => "rerun_all",
            Self::RerunFailed => "rerun_failed",
            Self::ImportProviderJoin => "import_provider_join",
        }
    }

    /// Whether the mode is an imported-provider join rather than a local plan.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportProviderJoin)
    }
}

/// Closed vocabulary for the kind of one append-only attempt inside a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptKind {
    /// First execution attempt of a planned session.
    Initial,
    /// A full rerun derived from a predecessor attempt.
    Rerun,
    /// A failed-only rerun derived from a predecessor attempt.
    RerunFailed,
    /// One cycle produced by a watch session.
    WatchCycle,
    /// Imported / provider-backed CI evidence joined into the session.
    ImportedJoin,
    /// A local rerun executed to compare against imported provider evidence.
    LocalParityRerun,
    /// A support / release packet reconstruction of an attempt.
    Reconstruction,
}

impl AttemptKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Initial,
        Self::Rerun,
        Self::RerunFailed,
        Self::WatchCycle,
        Self::ImportedJoin,
        Self::LocalParityRerun,
        Self::Reconstruction,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Rerun => "rerun",
            Self::RerunFailed => "rerun_failed",
            Self::WatchCycle => "watch_cycle",
            Self::ImportedJoin => "imported_join",
            Self::LocalParityRerun => "local_parity_rerun",
            Self::Reconstruction => "reconstruction",
        }
    }

    /// Whether this kind is an imported-provider join.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedJoin)
    }

    /// Whether this kind derives from a predecessor attempt (so it must record a
    /// predecessor ref to keep the append-only chain reconstructable).
    pub const fn derives_from_predecessor(self) -> bool {
        matches!(
            self,
            Self::Rerun | Self::RerunFailed | Self::LocalParityRerun
        )
    }
}

/// Closed outcome vocabulary for an attempt. The imported vocabulary
/// ([`Self::Imported`] / [`Self::ImportedStale`]) is disjoint from the local
/// pass / fail vocabulary so an imported verdict can never read as a local rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOutcome {
    /// The attempt is queued but has not started.
    Queued,
    /// The attempt is running.
    Running,
    /// The attempt passed in its declared local / remote / notebook target.
    Passed,
    /// The attempt failed in its declared local / remote / notebook target.
    Failed,
    /// The attempt errored before producing a verdict.
    Errored,
    /// Imported / provider evidence reported a result (never a local pass / fail).
    Imported,
    /// Imported evidence is stale or no longer comparable; must not roll up green.
    ImportedStale,
    /// The outcome cannot be classified; an automatic green roll-up is blocked.
    UnknownRequiresReview,
}

impl AttemptOutcome {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Errored => "errored",
            Self::Imported => "imported",
            Self::ImportedStale => "imported_stale",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether the outcome is drawn from the imported vocabulary.
    pub const fn is_imported_outcome(self) -> bool {
        matches!(self, Self::Imported | Self::ImportedStale)
    }

    /// Whether the outcome is a local execution verdict.
    pub const fn is_local_outcome(self) -> bool {
        matches!(
            self,
            Self::Queued | Self::Running | Self::Passed | Self::Failed | Self::Errored
        )
    }

    /// Whether the outcome reports current passing evidence (so it could roll up
    /// to a green state). Stale and unknown outcomes deliberately do not.
    pub const fn is_passing(self) -> bool {
        matches!(self, Self::Passed)
    }
}

/// Closed vocabulary for the kind of target an attempt dispatched against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetClass {
    /// A local process in the workspace.
    LocalProcess,
    /// A remote host / target.
    RemoteHost,
    /// A notebook kernel.
    NotebookKernel,
    /// A provider / imported backend that did not execute locally.
    ProviderBackend,
}

impl TargetClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProcess => "local_process",
            Self::RemoteHost => "remote_host",
            Self::NotebookKernel => "notebook_kernel",
            Self::ProviderBackend => "provider_backend",
        }
    }

    /// Whether the target class is a provider / imported backend.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ProviderBackend)
    }

    /// The flow a target class belongs to.
    pub const fn flow(self) -> SessionFlow {
        match self {
            Self::LocalProcess => SessionFlow::LocalWorkspace,
            Self::RemoteHost => SessionFlow::RemoteTarget,
            Self::NotebookKernel => SessionFlow::NotebookKernel,
            Self::ProviderBackend => SessionFlow::ImportedProvider,
        }
    }
}

/// Closed retry-policy vocabulary for a session plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryPolicyClass {
    /// Never auto-retry; a failed attempt stays failed until acted on.
    NoRetry,
    /// Retry the failed subset up to a bounded limit.
    RetryFailedUpToLimit,
    /// Retry until a stability verdict is reached.
    RetryUntilStable,
    /// Imported / provider-backed; never auto-retried as a local rerun.
    ImportedNoRetry,
}

impl RetryPolicyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRetry => "no_retry",
            Self::RetryFailedUpToLimit => "retry_failed_up_to_limit",
            Self::RetryUntilStable => "retry_until_stable",
            Self::ImportedNoRetry => "imported_no_retry",
        }
    }

    /// Whether the policy marks the plan as imported / never locally retried.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedNoRetry)
    }
}

/// Closed watch-policy vocabulary for a session plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchPolicyClass {
    /// Watch is not armed for this plan.
    WatchDisabled,
    /// Re-run the scope on every save.
    WatchOnSave,
    /// Re-run the scope on a debounced edit window.
    WatchDebounced,
    /// Imported / provider-backed; cannot arm a local watch controller.
    ImportedNotWatchable,
}

impl WatchPolicyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WatchDisabled => "watch_disabled",
            Self::WatchOnSave => "watch_on_save",
            Self::WatchDebounced => "watch_debounced",
            Self::ImportedNotWatchable => "imported_not_watchable",
        }
    }

    /// Whether the policy marks the plan as imported / not watchable.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedNotWatchable)
    }
}

/// Closed provenance vocabulary for an [`ExecutionLineage`]. This is the anchor
/// that keeps an imported attempt from masquerading as local truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageProvenanceClass {
    /// Executed locally; the runtime / toolchain / env tokens are local truth.
    LocalAuthoritative,
    /// Executed on a remote target; the lineage is remote-authoritative.
    RemoteAuthoritative,
    /// Executed in a notebook kernel; the lineage is notebook-authoritative.
    NotebookAuthoritative,
    /// Imported / provider-backed; read-only and never a local rerun.
    ImportedReadOnly,
}

impl LineageProvenanceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAuthoritative => "local_authoritative",
            Self::RemoteAuthoritative => "remote_authoritative",
            Self::NotebookAuthoritative => "notebook_authoritative",
            Self::ImportedReadOnly => "imported_read_only",
        }
    }

    /// Whether the provenance is imported / read-only.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedReadOnly)
    }

    /// The flow this provenance class belongs to.
    pub const fn flow(self) -> SessionFlow {
        match self {
            Self::LocalAuthoritative => SessionFlow::LocalWorkspace,
            Self::RemoteAuthoritative => SessionFlow::RemoteTarget,
            Self::NotebookAuthoritative => SessionFlow::NotebookKernel,
            Self::ImportedReadOnly => SessionFlow::ImportedProvider,
        }
    }
}

/// Runtime / toolchain / env lineage for a session plan or attempt. Carries only
/// non-display fingerprint tokens — never raw env bodies, host names, or provider
/// payloads — so the lineage survives export without leaking boundary material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLineage {
    /// Non-display runtime fingerprint token (e.g. interpreter / runner digest).
    pub runtime_token: String,
    /// Non-display toolchain identity digest.
    pub toolchain_token: String,
    /// Non-display environment-capsule digest.
    pub env_capsule_token: String,
    /// Target class this lineage describes.
    pub target_class: TargetClass,
    /// Provenance class binding the lineage to a flow and imported / local truth.
    pub provenance_class: LineageProvenanceClass,
    /// Opaque host ref when the lineage is remote (never a raw hostname).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ref: Option<String>,
    /// Opaque provider id token, present iff the lineage is imported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Whether the lineage is imported / provider-backed.
    pub imported: bool,
}

impl ExecutionLineage {
    /// The flow this lineage's target class and provenance agree on, or [`None`]
    /// when they disagree.
    pub fn agreed_flow(&self) -> Option<SessionFlow> {
        let by_target = self.target_class.flow();
        let by_provenance = self.provenance_class.flow();
        (by_target == by_provenance).then_some(by_target)
    }

    /// Whether the lineage's `imported` flag, target class, and provenance class
    /// are mutually consistent.
    pub fn imported_consistent(&self) -> bool {
        self.imported == self.target_class.is_imported()
            && self.imported == self.provenance_class.is_imported()
            && self.imported == self.provider_token.is_some()
    }

    /// Whether the lineage carries every fingerprint token and its invariants
    /// hold.
    pub fn is_valid(&self) -> bool {
        let tokens_present = !self.runtime_token.trim().is_empty()
            && !self.toolchain_token.trim().is_empty()
            && !self.env_capsule_token.trim().is_empty();
        let host_ok = self
            .host_ref
            .as_ref()
            .map_or(true, |r| !r.trim().is_empty());
        let provider_ok = self
            .provider_token
            .as_ref()
            .map_or(true, |r| !r.trim().is_empty());
        tokens_present
            && host_ok
            && provider_ok
            && self.agreed_flow().is_some()
            && self.imported_consistent()
    }
}

/// One resolved target a session or attempt covers, addressed by a durable node
/// id and a non-display fingerprint rather than a display label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerTarget {
    /// Durable node id of the target.
    pub target_id: String,
    /// Node kind, reusing the frozen durable-discovery vocabulary so a
    /// parameterized template never collapses into a concrete invocation.
    pub node_kind: DurableTestNodeKind,
    /// Non-display fingerprint token. Must differ from
    /// [`target_id`](LedgerTarget::target_id) so neither a label nor a bare id
    /// stands in for the durable fingerprint.
    pub target_fingerprint_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
}

impl LedgerTarget {
    /// Whether this target is imported / provider-owned and read-only.
    pub fn is_imported(&self) -> bool {
        self.identity_class == TestItemIdentityClass::ImportedReadOnly
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.target_fingerprint_token.trim();
        !token.is_empty() && token != self.target_id.trim()
    }

    /// Whether the target carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.target_id.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// A durable session plan: what was asked to run, in which flow, against which
/// target class, with which retry / watch policy and execution lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionPlan {
    /// Stable session id.
    pub session_id: String,
    /// Stable session-plan id.
    pub plan_id: String,
    /// Human-readable session label. Never the identity basis.
    pub label: String,
    /// Flow the session belongs to.
    pub flow: SessionFlow,
    /// What the session plan intends.
    pub mode: SessionPlanMode,
    /// Selection object this plan executes (reconstructable elsewhere).
    pub selection_ref: String,
    /// Discovery-snapshot id the selection was resolved against.
    pub snapshot_ref: String,
    /// Execution-context ref used by the plan.
    pub execution_context_ref: String,
    /// Runtime / toolchain / env lineage for the plan.
    pub lineage: ExecutionLineage,
    /// Retry policy.
    pub retry_policy: RetryPolicyClass,
    /// Watch policy.
    pub watch_policy: WatchPolicyClass,
    /// Targets the session covers, pinned at plan time.
    pub targets: Vec<LedgerTarget>,
    /// Evidence packet refs backing this plan.
    pub evidence_refs: Vec<String>,
}

impl SessionPlan {
    /// Pinned target ids.
    pub fn target_ids(&self) -> BTreeSet<&str> {
        self.targets.iter().map(|t| t.target_id.as_str()).collect()
    }

    /// Whether every target id is unique.
    pub fn target_ids_unique(&self) -> bool {
        self.target_ids().len() == self.targets.len()
    }

    /// Whether this plan is imported / provider-backed (by flow, mode, lineage, or
    /// any covered target).
    pub fn is_imported(&self) -> bool {
        self.flow.is_imported()
            || self.mode.is_imported()
            || self.lineage.imported
            || self.targets.iter().any(LedgerTarget::is_imported)
    }

    /// Whether the plan's imported markers all agree, so an imported plan never
    /// presents a local retry / watch policy and a local plan never presents
    /// imported markers.
    pub fn imported_markers_consistent(&self) -> bool {
        if self.is_imported() {
            self.flow.is_imported()
                && self.lineage.imported
                && self.retry_policy.is_imported()
                && self.watch_policy.is_imported()
        } else {
            !self.lineage.imported
                && !self.retry_policy.is_imported()
                && !self.watch_policy.is_imported()
        }
    }

    /// Whether the plan's flow agrees with its lineage flow.
    pub fn flow_lineage_consistent(&self) -> bool {
        self.lineage.agreed_flow() == Some(self.flow)
    }

    /// Whether every field required to record this plan is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.session_id.trim().is_empty()
            && !self.plan_id.trim().is_empty()
            && !self.label.trim().is_empty()
            && !self.selection_ref.trim().is_empty()
            && !self.snapshot_ref.trim().is_empty()
            && !self.execution_context_ref.trim().is_empty()
            && self.lineage.is_valid()
            && !self.targets.is_empty()
            && self.targets.iter().all(LedgerTarget::is_valid)
            && self.target_ids_unique()
            && self.flow_lineage_consistent()
            && self.imported_markers_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// One append-only attempt in a session. Each attempt carries its own lineage so
/// an imported / provider-backed attempt can never read as a local rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptRecord {
    /// Stable attempt id.
    pub attempt_id: String,
    /// Parent session id.
    pub session_ref: String,
    /// One-based attempt index, unique and contiguous within a session.
    pub attempt_index: u32,
    /// Attempt kind.
    pub kind: AttemptKind,
    /// Attempt outcome.
    pub outcome: AttemptOutcome,
    /// Flow this attempt executed (or imported) in.
    pub flow: SessionFlow,
    /// Target class this attempt dispatched against.
    pub target_class: TargetClass,
    /// Runtime / toolchain / env lineage for the attempt.
    pub lineage: ExecutionLineage,
    /// Predecessor attempt this one derives from (the append-only retry chain).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_attempt_ref: Option<String>,
    /// Originating provider / imported ref, present iff this is an imported
    /// attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_provider_ref: Option<String>,
    /// Execution attempt ref on the generic execution rail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_attempt_ref: Option<String>,
    /// Durable target ids covered by this attempt (subset of the session targets).
    pub covered_target_ids: Vec<String>,
    /// Artifact refs retained on governed artifact rails.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_refs: Vec<String>,
    /// Evidence packet refs backing this attempt.
    pub evidence_refs: Vec<String>,
    /// Export-safe attempt summary.
    pub support_summary: String,
}

impl AttemptRecord {
    /// Whether this attempt is imported / provider-backed by any marker.
    pub fn is_imported_attempt(&self) -> bool {
        self.flow.is_imported()
            || self.kind.is_imported()
            || self.lineage.imported
            || self.outcome.is_imported_outcome()
    }

    /// Whether the attempt's flow agrees with its lineage and declared target
    /// class.
    pub fn flow_lineage_consistent(&self) -> bool {
        self.lineage.target_class == self.target_class
            && self.lineage.agreed_flow() == Some(self.flow)
    }

    /// Whether the attempt's imported markers agree across kind, flow, lineage,
    /// outcome, and origin ref — so an imported verdict never reads as a local
    /// rerun and a local attempt never carries an imported origin.
    pub fn imported_markers_consistent(&self) -> bool {
        if self.is_imported_attempt() {
            self.flow.is_imported()
                && self.lineage.imported
                && self.outcome.is_imported_outcome()
                && self.origin_provider_ref.is_some()
        } else {
            !self.flow.is_imported()
                && !self.lineage.imported
                && self.outcome.is_local_outcome()
                && self.origin_provider_ref.is_none()
                && !self.kind.is_imported()
        }
    }

    /// Whether the predecessor ref is present when the attempt kind requires it.
    pub fn predecessor_present_if_required(&self) -> bool {
        !self.kind.derives_from_predecessor() || self.predecessor_attempt_ref.is_some()
    }

    /// Whether every field required to record this attempt is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.attempt_id.trim().is_empty()
            && !self.session_ref.trim().is_empty()
            && self.attempt_index >= 1
            && self.lineage.is_valid()
            && !self.covered_target_ids.is_empty()
            && self.covered_target_ids.iter().all(|t| !t.trim().is_empty())
            && self.artifact_refs.iter().all(|r| !r.trim().is_empty())
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.support_summary.trim().is_empty()
            && self.flow_lineage_consistent()
            && self.imported_markers_consistent()
            && self.predecessor_present_if_required()
            && self
                .origin_provider_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
            && self
                .predecessor_attempt_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerGuardrails {
    /// Parameterized templates stay distinct from their concrete invocations.
    pub templates_distinct_from_invocations: bool,
    /// Imported / provider-backed attempts never read as local reruns.
    pub imported_never_local_rerun: bool,
    /// Every attempt carries its own runtime / toolchain / env lineage.
    pub lineage_preserved_per_attempt: bool,
    /// Attempt history is append-only and reconstructable.
    pub attempt_history_append_only: bool,
    /// No stale / quarantine-bearing attempt hides behind a generic green state.
    pub no_green_over_quarantine_or_stale: bool,
    /// Sessions and attempts can be reopened from the export alone.
    pub session_reopenable_from_export: bool,
}

impl LedgerGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.templates_distinct_from_invocations
            && self.imported_never_local_rerun
            && self.lineage_preserved_per_attempt
            && self.attempt_history_append_only
            && self.no_green_over_quarantine_or_stale
            && self.session_reopenable_from_export
    }
}

/// Consumer projection block: the surfaces that reopen the same session / attempt
/// objects without replaying the work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerConsumerProjection {
    /// Notifications reopen the authoritative session / attempt.
    pub notifications_reopen_session: bool,
    /// Support export reopens the authoritative attempt.
    pub support_export_reopens_attempt: bool,
    /// Review packets reopen the authoritative session.
    pub review_packet_reopens_session: bool,
    /// Release gating reads the ledger instead of scraping UI text.
    pub release_gate_reads_ledger: bool,
}

impl LedgerConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.notifications_reopen_session
            && self.support_export_reopens_attempt
            && self.review_packet_reopens_session
            && self.release_gate_reads_ledger
    }
}

/// Constructor input for [`SessionAttemptLedgerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionAttemptLedgerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Session plans across flows and modes.
    pub sessions: Vec<SessionPlan>,
    /// Append-only attempt records referencing the sessions.
    pub attempts: Vec<AttemptRecord>,
    /// Guardrail invariants block.
    pub guardrails: LedgerGuardrails,
    /// Consumer projection block.
    pub consumer_projection: LedgerConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe session / attempt ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionAttemptLedgerPacket {
    /// Record kind; must equal [`SESSION_ATTEMPT_LEDGER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SESSION_ATTEMPT_LEDGER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Session plans across flows and modes.
    pub sessions: Vec<SessionPlan>,
    /// Append-only attempt records referencing the sessions.
    pub attempts: Vec<AttemptRecord>,
    /// Guardrail invariants block.
    pub guardrails: LedgerGuardrails,
    /// Consumer projection block.
    pub consumer_projection: LedgerConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SessionAttemptLedgerPacket {
    /// Builds a session / attempt ledger packet.
    pub fn new(input: SessionAttemptLedgerPacketInput) -> Self {
        Self {
            record_kind: SESSION_ATTEMPT_LEDGER_RECORD_KIND.to_owned(),
            schema_version: SESSION_ATTEMPT_LEDGER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            sessions: input.sessions,
            attempts: input.attempts,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Flows represented by some session in this packet.
    pub fn represented_flows(&self) -> BTreeSet<SessionFlow> {
        self.sessions.iter().map(|s| s.flow).collect()
    }

    /// Modes represented by some session in this packet.
    pub fn represented_modes(&self) -> BTreeSet<SessionPlanMode> {
        self.sessions.iter().map(|s| s.mode).collect()
    }

    /// Attempt kinds represented by some attempt in this packet.
    pub fn represented_attempt_kinds(&self) -> BTreeSet<AttemptKind> {
        self.attempts.iter().map(|a| a.kind).collect()
    }

    /// Target node kinds represented across every session.
    pub fn represented_target_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.sessions
            .iter()
            .flat_map(|s| s.targets.iter().map(|t| t.node_kind))
            .collect()
    }

    /// Resolves a session by its id.
    pub fn session(&self, session_id: &str) -> Option<&SessionPlan> {
        self.sessions.iter().find(|s| s.session_id == session_id)
    }

    /// Resolves an attempt by its id.
    pub fn attempt(&self, attempt_id: &str) -> Option<&AttemptRecord> {
        self.attempts.iter().find(|a| a.attempt_id == attempt_id)
    }

    /// Attempts belonging to a session, ordered by attempt index.
    pub fn session_attempts(&self, session_id: &str) -> Vec<&AttemptRecord> {
        let mut attempts: Vec<&AttemptRecord> = self
            .attempts
            .iter()
            .filter(|a| a.session_ref == session_id)
            .collect();
        attempts.sort_by_key(|a| a.attempt_index);
        attempts
    }

    /// Count of attempts that derive from a predecessor (the retry / parity
    /// history).
    pub fn retry_chain_count(&self) -> usize {
        self.attempts
            .iter()
            .filter(|a| a.predecessor_attempt_ref.is_some())
            .count()
    }

    /// Count of imported / provider-backed attempts.
    pub fn imported_attempt_count(&self) -> usize {
        self.attempts
            .iter()
            .filter(|a| a.is_imported_attempt())
            .count()
    }

    /// Validates the session / attempt ledger invariants.
    pub fn validate(&self) -> Vec<SessionAttemptLedgerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SESSION_ATTEMPT_LEDGER_RECORD_KIND {
            violations.push(SessionAttemptLedgerViolation::WrongRecordKind);
        }
        if self.schema_version != SESSION_ATTEMPT_LEDGER_SCHEMA_VERSION {
            violations.push(SessionAttemptLedgerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SessionAttemptLedgerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_sessions(self, &mut violations);
        validate_attempts(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(SessionAttemptLedgerViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(SessionAttemptLedgerViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("session attempt ledger packet serializes"),
        ) {
            violations.push(SessionAttemptLedgerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("session attempt ledger packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Session Plans And Attempt-Record Ledger\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Sessions: {} across {} / {} flows\n",
            self.sessions.len(),
            self.represented_flows().len(),
            SessionFlow::ALL.len()
        ));
        out.push_str(&format!(
            "- Attempts: {} ({} imported, {} derive from a predecessor)\n",
            self.attempts.len(),
            self.imported_attempt_count(),
            self.retry_chain_count()
        ));
        out.push_str("\n## Sessions\n\n");
        for session in &self.sessions {
            out.push_str(&format!(
                "- **{}** ({} / {}): retry `{}`, watch `{}`\n",
                session.session_id,
                session.flow.as_str(),
                session.mode.as_str(),
                session.retry_policy.as_str(),
                session.watch_policy.as_str()
            ));
            out.push_str(&format!("  - {}\n", session.label));
            out.push_str(&format!(
                "  - selection `{}` snapshot `{}` ({} targets)\n",
                session.selection_ref,
                session.snapshot_ref,
                session.targets.len()
            ));
            out.push_str(&format!(
                "  - lineage: runtime `{}` toolchain `{}` env `{}` ({})\n",
                session.lineage.runtime_token,
                session.lineage.toolchain_token,
                session.lineage.env_capsule_token,
                session.lineage.provenance_class.as_str()
            ));
            for attempt in self.session_attempts(&session.session_id) {
                out.push_str(&format!(
                    "  - attempt #{} `{}` [{}] → `{}`{}\n",
                    attempt.attempt_index,
                    attempt.attempt_id,
                    attempt.kind.as_str(),
                    attempt.outcome.as_str(),
                    match &attempt.predecessor_attempt_ref {
                        Some(pred) => format!(" (from `{pred}`)"),
                        None => String::new(),
                    }
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in ledger export.
#[derive(Debug)]
pub enum SessionAttemptLedgerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SessionAttemptLedgerViolation>),
}

impl fmt::Display for SessionAttemptLedgerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "session attempt ledger export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "session attempt ledger export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SessionAttemptLedgerArtifactError {}

/// Validation failures emitted by [`SessionAttemptLedgerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionAttemptLedgerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required session flow is represented by no session.
    FlowCoverageMissing,
    /// A required session mode is represented by no session.
    ModeCoverageMissing,
    /// A parameterized template was collapsed into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// No attempt demonstrates an imported join held read-only.
    ImportedAttemptCaseMissing,
    /// No attempt demonstrates a local parity rerun distinct from imported truth.
    LocalParityCaseMissing,
    /// No attempt demonstrates an append-only retry chain.
    RetryHistoryCaseMissing,
    /// A session is incomplete.
    SessionInvalid,
    /// Session target ids are not unique.
    TargetIdsNotUnique,
    /// A target's fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A session's imported markers disagree (imported plan with a local policy or
    /// a local plan with imported markers).
    SessionImportedMarkersInconsistent,
    /// An attempt is incomplete.
    AttemptInvalid,
    /// An attempt references a session absent from the packet.
    AttemptSessionUnresolved,
    /// An attempt's flow / target-class / lineage disagree.
    AttemptLineageInconsistent,
    /// An imported attempt reads as a local rerun (or a local attempt carries
    /// imported markers).
    ImportedAttemptReadsAsLocal,
    /// An attempt's covered target is not part of its session.
    AttemptTargetUnresolved,
    /// A session's attempt indices are not unique and contiguous from one.
    AttemptIndicesNotContiguous,
    /// An attempt's predecessor ref does not resolve to an earlier attempt in the
    /// same session.
    PredecessorChainInvalid,
    /// A stale or unknown attempt hides behind a passing outcome.
    GreenOverStaleOrUnknown,
    /// A session or attempt lacks evidence refs.
    EvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl SessionAttemptLedgerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::FlowCoverageMissing => "flow_coverage_missing",
            Self::ModeCoverageMissing => "mode_coverage_missing",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::ImportedAttemptCaseMissing => "imported_attempt_case_missing",
            Self::LocalParityCaseMissing => "local_parity_case_missing",
            Self::RetryHistoryCaseMissing => "retry_history_case_missing",
            Self::SessionInvalid => "session_invalid",
            Self::TargetIdsNotUnique => "target_ids_not_unique",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::SessionImportedMarkersInconsistent => "session_imported_markers_inconsistent",
            Self::AttemptInvalid => "attempt_invalid",
            Self::AttemptSessionUnresolved => "attempt_session_unresolved",
            Self::AttemptLineageInconsistent => "attempt_lineage_inconsistent",
            Self::ImportedAttemptReadsAsLocal => "imported_attempt_reads_as_local",
            Self::AttemptTargetUnresolved => "attempt_target_unresolved",
            Self::AttemptIndicesNotContiguous => "attempt_indices_not_contiguous",
            Self::PredecessorChainInvalid => "predecessor_chain_invalid",
            Self::GreenOverStaleOrUnknown => "green_over_stale_or_unknown",
            Self::EvidenceMissing => "evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable ledger export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_session_attempt_ledger_export(
) -> Result<SessionAttemptLedgerPacket, SessionAttemptLedgerArtifactError> {
    let packet: SessionAttemptLedgerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/session-plans-attempt-records-and-execution-lineage/support_export.json"
    )))
    .map_err(SessionAttemptLedgerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SessionAttemptLedgerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &SessionAttemptLedgerPacket,
    violations: &mut Vec<SessionAttemptLedgerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SESSION_ATTEMPT_LEDGER_SCHEMA_REF,
        SESSION_ATTEMPT_LEDGER_DOC_REF,
        SESSION_ATTEMPT_LEDGER_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SessionAttemptLedgerViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &SessionAttemptLedgerPacket,
    violations: &mut Vec<SessionAttemptLedgerViolation>,
) {
    let flows = packet.represented_flows();
    for required in SessionFlow::ALL {
        if !flows.contains(&required) {
            violations.push(SessionAttemptLedgerViolation::FlowCoverageMissing);
            break;
        }
    }

    let modes = packet.represented_modes();
    for required in [
        SessionPlanMode::RunSelected,
        SessionPlanMode::RerunFailed,
        SessionPlanMode::ImportProviderJoin,
    ] {
        if !modes.contains(&required) {
            violations.push(SessionAttemptLedgerViolation::ModeCoverageMissing);
            break;
        }
    }

    let target_kinds = packet.represented_target_kinds();
    if !(target_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && target_kinds.contains(&DurableTestNodeKind::ConcreteInvocation))
    {
        violations.push(SessionAttemptLedgerViolation::TemplateCollapsedWithInvocation);
    }

    if !packet.attempts.iter().any(|a| {
        a.is_imported_attempt()
            && a.lineage.provenance_class == LineageProvenanceClass::ImportedReadOnly
    }) {
        violations.push(SessionAttemptLedgerViolation::ImportedAttemptCaseMissing);
    }

    if !packet
        .attempts
        .iter()
        .any(|a| a.kind == AttemptKind::LocalParityRerun && !a.is_imported_attempt())
    {
        violations.push(SessionAttemptLedgerViolation::LocalParityCaseMissing);
    }

    if packet.retry_chain_count() == 0 {
        violations.push(SessionAttemptLedgerViolation::RetryHistoryCaseMissing);
    }
}

fn validate_sessions(
    packet: &SessionAttemptLedgerPacket,
    violations: &mut Vec<SessionAttemptLedgerViolation>,
) {
    for session in &packet.sessions {
        if !session.is_valid() {
            violations.push(SessionAttemptLedgerViolation::SessionInvalid);
        }
        if !session.target_ids_unique() {
            violations.push(SessionAttemptLedgerViolation::TargetIdsNotUnique);
        }
        if !session.imported_markers_consistent() {
            violations.push(SessionAttemptLedgerViolation::SessionImportedMarkersInconsistent);
        }
        if session.evidence_refs.is_empty()
            || session.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(SessionAttemptLedgerViolation::EvidenceMissing);
        }
        for target in &session.targets {
            if !target.fingerprint_independent_of_id() {
                violations.push(SessionAttemptLedgerViolation::FingerprintSubstitutesIdentity);
            }
        }
    }
}

fn validate_attempts(
    packet: &SessionAttemptLedgerPacket,
    violations: &mut Vec<SessionAttemptLedgerViolation>,
) {
    for attempt in &packet.attempts {
        if !attempt.is_valid() {
            violations.push(SessionAttemptLedgerViolation::AttemptInvalid);
        }
        if !attempt.flow_lineage_consistent() {
            violations.push(SessionAttemptLedgerViolation::AttemptLineageInconsistent);
        }
        if !attempt.imported_markers_consistent() {
            violations.push(SessionAttemptLedgerViolation::ImportedAttemptReadsAsLocal);
        }
        if attempt.evidence_refs.is_empty()
            || attempt.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(SessionAttemptLedgerViolation::EvidenceMissing);
        }

        // A stale or unknown attempt that presents a passing outcome would let a
        // generic green state hide quarantine / stale evidence.
        if attempt.outcome.is_passing()
            && (attempt.is_imported_attempt() || attempt.kind == AttemptKind::ImportedJoin)
        {
            violations.push(SessionAttemptLedgerViolation::GreenOverStaleOrUnknown);
        }

        match packet.session(&attempt.session_ref) {
            None => {
                violations.push(SessionAttemptLedgerViolation::AttemptSessionUnresolved);
            }
            Some(session) => {
                let session_target_ids = session.target_ids();
                if attempt
                    .covered_target_ids
                    .iter()
                    .any(|id| !session_target_ids.contains(id.as_str()))
                {
                    violations.push(SessionAttemptLedgerViolation::AttemptTargetUnresolved);
                }
            }
        }
    }

    validate_attempt_chains(packet, violations);
}

/// Validates per-session append-only attempt history: indices are unique and
/// contiguous from one, and every predecessor ref resolves to an earlier attempt
/// in the same session.
fn validate_attempt_chains(
    packet: &SessionAttemptLedgerPacket,
    violations: &mut Vec<SessionAttemptLedgerViolation>,
) {
    for session in &packet.sessions {
        let attempts = packet.session_attempts(&session.session_id);
        if attempts.is_empty() {
            continue;
        }

        let indices: Vec<u32> = attempts.iter().map(|a| a.attempt_index).collect();
        let unique: BTreeSet<u32> = indices.iter().copied().collect();
        let contiguous = unique.len() == attempts.len()
            && indices.first() == Some(&1)
            && indices.windows(2).all(|pair| pair[1] == pair[0] + 1);
        if !contiguous {
            violations.push(SessionAttemptLedgerViolation::AttemptIndicesNotContiguous);
        }

        for attempt in &attempts {
            let Some(pred_ref) = &attempt.predecessor_attempt_ref else {
                continue;
            };
            match packet.attempt(pred_ref) {
                Some(pred)
                    if pred.session_ref == session.session_id
                        && pred.attempt_index < attempt.attempt_index => {}
                _ => violations.push(SessionAttemptLedgerViolation::PredecessorChainInvalid),
            }
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
