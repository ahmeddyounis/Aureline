//! Issue-use-revoke authority-lifecycle audit ledgers, invalidation on
//! target-or-trust-or-policy-or-network-or-sandbox drift, and support-export-safe
//! authority lineage across M5 execution families.
//!
//! The capability-envelope packet states the concrete authority issued for one
//! execution, and the approval-ticket ledger states the short-lived ticket
//! minted for one mutating action. This module is the **lifecycle side** of that
//! contract: it joins those point-in-time grants into a single inspectable audit
//! trail per authority grant — when it was **issued**, each time it was **used**
//! and with what outcome, whether it was **revoked**, and whether it was
//! **invalidated** because the world drifted out from under it.
//!
//! Each [`M5AuthorityLedgerEntry`] threads one authority grant through its whole
//! life and joins the cross-cutting identifiers an incident or support reviewer
//! needs to reason about it after the fact:
//!
//! - **linkage** — [`M5LedgerLinkage`]: the command, session, approval-ticket,
//!   and capability-envelope refs plus the originating flow family
//!   ([`M5OriginFlowClass`]) so execution, repair, AI-assisted, provider-linked,
//!   and remote flows all land in one ledger instead of per-surface spreadsheets.
//! - **issue** — [`M5IssueEvent`]: who minted the authority, under which policy
//!   epoch, sandbox profile, and secret scope, and with what expiry. Helper
//!   actors (AI, recipe, extension, browser route, remote helper) never
//!   self-issue: `self_issued_by_executor` is always false.
//! - **uses** — [`M5UseEvent`]: the monotonic sequence of spend attempts, each
//!   with an [`M5UseOutcome`] that is consistent with the grant's state at the
//!   time (a denied-by-drift use requires a recorded invalidation; a
//!   denied-by-revocation use requires a recorded revocation).
//! - **invalidation** — [`M5Invalidation`]: fires when the target identity, the
//!   trust anchor, the policy epoch, the network posture, or the sandbox profile
//!   ([`M5DriftDimension`]) drifts away from what the grant was bound to. It
//!   names the failed dimension, the downgrade trigger, the narrowed fallback,
//!   and a concrete recovery action instead of a generic permission error.
//! - **revocation** — [`M5RevokeEvent`]: an explicit, attributed revocation with
//!   its own narrowed fallback.
//!
//! The lifecycle state ([`M5AuthorityLifecycleState`]) is kept coherent with
//! those events: an `Invalidated` entry always carries an invalidation, a
//! `Revoked` entry always carries a revocation, an `Active` entry has been used
//! and carries neither, and an `Issued` entry has not yet been used.
//!
//! The track invariant holds end to end: no ambient privilege; no helper
//! self-issues authority; target identity, policy epoch, sandbox profile, secret
//! scope, and degraded fallback stay inspectable and export-safe; and if
//! enforcement cannot be honored the grant **narrows or fails closed with a named
//! drift dimension** instead of silently widening. No raw secret material,
//! credential body, or live ticket signature is ever exported.
//!
//! The boundary schema is
//! [`schemas/execution-auth/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.schema.json`](../../../../schemas/execution-auth/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.md`](../../../../docs/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/`](../../../../fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5DegradedFallback, M5ExecutingSurface,
    M5RuntimeAuthorityDowngradeTrigger, M5SandboxProfile, M5SecretScope,
    M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF, M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF, M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
};
use super::implement_approval_ticket_issuance_deny_reason_packets_replay_nonce_or_expiry_enforcement_and_local_first_verification_f::{
    M5TicketActionClass, M5TicketActor, M5TicketTarget, M5_APPROVAL_TICKET_LEDGER_DOC_REF,
    M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF,
};
use super::ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::{
    M5EnvelopeActorClass, M5EnvelopeIssuerClass, M5PolicyEpochBinding, M5_CAPABILITY_ENVELOPE_DOC_REF,
    M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5AuthorityLifecycleLedgerPacket`].
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_RECORD_KIND: &str =
    "add_m5_issue_use_revoke_authority_lifecycle_ledger";

/// Schema version for the M5 authority-lifecycle ledger packet records.
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_REF: &str =
    "schemas/execution-auth/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_DOC_REF: &str =
    "docs/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au";

/// Stable packet id minted by [`frozen_stable_m5_authority_lifecycle_ledger_packet`].
pub const M5_AUTHORITY_LIFECYCLE_LEDGER_PACKET_ID: &str =
    "m5-authority-lifecycle-ledger:stable:0001";

/// Originating flow family an authority grant was minted inside.
///
/// The ledger joins authority lineage across all of these families so runtime
/// authority can be reasoned about as one product contract rather than
/// per-surface folklore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OriginFlowClass {
    /// A direct execution flow (scaffold hook, notebook kernel, preview server).
    Execution,
    /// A repair or incident-response flow.
    Repair,
    /// An AI-assisted flow that invoked a tool on the operator's behalf.
    AiAssisted,
    /// A provider-linked flow (request/API send, database action, connector).
    ProviderLinked,
    /// A remote-execution flow brokered by another runtime.
    Remote,
}

impl M5OriginFlowClass {
    /// Every origin flow class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Execution,
        Self::Repair,
        Self::AiAssisted,
        Self::ProviderLinked,
        Self::Remote,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::Repair => "repair",
            Self::AiAssisted => "ai_assisted",
            Self::ProviderLinked => "provider_linked",
            Self::Remote => "remote",
        }
    }
}

/// The dimension along which a grant's bound world drifted, invalidating it.
///
/// Invalidation fires on exactly one named dimension instead of collapsing into
/// a generic permission error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DriftDimension {
    /// The execution target identity drifted (host, path, or resource id).
    TargetIdentityDrift,
    /// The trust anchor drifted (trust store, signer, or ambient trust change).
    TrustAnchorDrift,
    /// The governing policy epoch was superseded.
    PolicyEpochDrift,
    /// The network posture drifted (proxy, route, or egress plane change).
    NetworkPostureDrift,
    /// The sandbox profile drifted or became unavailable on this platform.
    SandboxProfileDrift,
}

impl M5DriftDimension {
    /// Every drift dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TargetIdentityDrift,
        Self::TrustAnchorDrift,
        Self::PolicyEpochDrift,
        Self::NetworkPostureDrift,
        Self::SandboxProfileDrift,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetIdentityDrift => "target_identity_drift",
            Self::TrustAnchorDrift => "trust_anchor_drift",
            Self::PolicyEpochDrift => "policy_epoch_drift",
            Self::NetworkPostureDrift => "network_posture_drift",
            Self::SandboxProfileDrift => "sandbox_profile_drift",
        }
    }

    /// The downgrade trigger this drift dimension must record.
    pub const fn trigger(self) -> M5RuntimeAuthorityDowngradeTrigger {
        match self {
            Self::TargetIdentityDrift => {
                M5RuntimeAuthorityDowngradeTrigger::TargetIdentityUnverified
            }
            Self::TrustAnchorDrift => M5RuntimeAuthorityDowngradeTrigger::AmbientPrivilegeDetected,
            Self::PolicyEpochDrift => M5RuntimeAuthorityDowngradeTrigger::PolicyEpochSuperseded,
            Self::NetworkPostureDrift => {
                M5RuntimeAuthorityDowngradeTrigger::UpstreamDependencyNarrowed
            }
            Self::SandboxProfileDrift => {
                M5RuntimeAuthorityDowngradeTrigger::SandboxProfileUnavailable
            }
        }
    }

    /// The default narrowed fallback an enforcer applies for this drift.
    pub const fn default_fallback(self) -> M5DegradedFallback {
        match self {
            Self::TargetIdentityDrift => M5DegradedFallback::RequireFreshTicket,
            Self::TrustAnchorDrift => M5DegradedFallback::FailClosedBlock,
            Self::PolicyEpochDrift => M5DegradedFallback::NarrowToSanitizedPreview,
            Self::NetworkPostureDrift => M5DegradedFallback::OfflineLocalCoreOnly,
            Self::SandboxProfileDrift => M5DegradedFallback::FailClosedBlock,
        }
    }
}

/// Lifecycle state of one authority grant in the ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthorityLifecycleState {
    /// Issued and not yet used.
    Issued,
    /// Issued and used at least once; still valid.
    Active,
    /// Explicitly revoked.
    Revoked,
    /// Invalidated by drift along one named dimension.
    Invalidated,
    /// Expired by its own time-to-live.
    Expired,
}

impl M5AuthorityLifecycleState {
    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Issued => "issued",
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::Invalidated => "invalidated",
            Self::Expired => "expired",
        }
    }

    /// Whether a grant in this state still authorizes its action.
    pub const fn is_spendable(self) -> bool {
        matches!(self, Self::Issued | Self::Active)
    }

    /// Whether a grant in this state has been terminated and must carry a reason.
    pub const fn is_terminated(self) -> bool {
        matches!(self, Self::Revoked | Self::Invalidated | Self::Expired)
    }
}

/// Class of one event recorded in the ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LedgerEventClass {
    /// The grant was issued.
    Issue,
    /// The grant was used (spend attempt).
    Use,
    /// The grant was revoked.
    Revoke,
    /// The grant was invalidated by drift.
    Invalidate,
}

impl M5LedgerEventClass {
    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Issue => "issue",
            Self::Use => "use",
            Self::Revoke => "revoke",
            Self::Invalidate => "invalidate",
        }
    }
}

/// Outcome of one recorded use (spend attempt) of an authority grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UseOutcome {
    /// The action ran at the granted authority.
    AllowedExecuted,
    /// The action ran narrowed below the granted authority.
    NarrowedExecuted,
    /// The action was denied because the grant had been invalidated by drift.
    DeniedInvalidated,
    /// The action was denied because the grant had been revoked.
    DeniedRevoked,
    /// The action was denied because the grant had expired.
    DeniedExpired,
}

impl M5UseOutcome {
    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedExecuted => "allowed_executed",
            Self::NarrowedExecuted => "narrowed_executed",
            Self::DeniedInvalidated => "denied_invalidated",
            Self::DeniedRevoked => "denied_revoked",
            Self::DeniedExpired => "denied_expired",
        }
    }

    /// Whether this outcome let the action run (possibly narrowed).
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::AllowedExecuted | Self::NarrowedExecuted)
    }
}

/// Cross-cutting linkage joining an authority grant to its command, session,
/// ticket, envelope, and originating flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LedgerLinkage {
    /// Originating flow family.
    pub origin_flow: M5OriginFlowClass,
    /// Export-safe command reference, when the grant was driven by a command.
    pub command_ref: Option<String>,
    /// Export-safe execution-session reference.
    pub session_ref: String,
    /// Export-safe approval-ticket reference this grant was bound to.
    pub approval_ticket_ref: String,
    /// Export-safe capability-envelope reference this grant was bound to.
    pub capability_envelope_ref: String,
    /// Export-safe digest of the bound capability envelope (never raw material).
    pub capability_envelope_hash: String,
}

/// The issue event for an authority grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueEvent {
    /// RFC 3339 issuance timestamp.
    pub issued_at: String,
    /// Issuer class that minted the authority.
    pub issuer_class: M5EnvelopeIssuerClass,
    /// Export-safe issuer reference.
    pub issuer_ref: String,
    /// Governing policy-epoch binding at issuance.
    pub policy_epoch: M5PolicyEpochBinding,
    /// Sandbox profile the grant authorized.
    pub sandbox_profile: M5SandboxProfile,
    /// Secret scope the grant authorized.
    pub secret_scope: M5SecretScope,
    /// RFC 3339 expiry timestamp.
    pub expires_at: String,
    /// Grant time-to-live in seconds; must be non-zero.
    pub ttl_seconds: u32,
    /// Ordered export-safe lineage refs from policy epoch to issued grant.
    pub decision_chain: Vec<String>,
    /// Always false: the executor never self-issues this grant.
    pub self_issued_by_executor: bool,
}

/// One recorded use (spend attempt) of an authority grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UseEvent {
    /// RFC 3339 use timestamp.
    pub used_at: String,
    /// Monotonic 1-based sequence number within the grant.
    pub sequence: u64,
    /// Outcome of this use.
    pub outcome: M5UseOutcome,
    /// Fallback the action was narrowed to, when it did not run at full authority.
    pub narrowed_to: Option<M5DegradedFallback>,
    /// Export-safe note describing what ran or why it was denied; never empty.
    pub note: String,
}

/// Invalidation of an authority grant triggered by drift.
///
/// Names the drifted dimension, the downgrade trigger, the narrowed fallback,
/// and a concrete recovery action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5Invalidation {
    /// RFC 3339 timestamp at which the drift was detected.
    pub detected_at: String,
    /// The dimension along which the grant's bound world drifted.
    pub drift_dimension: M5DriftDimension,
    /// Downgrade trigger recorded for this invalidation.
    pub trigger: M5RuntimeAuthorityDowngradeTrigger,
    /// Fallback the surface narrows to once invalidated.
    pub narrowed_to: M5DegradedFallback,
    /// Export-safe description of what was bound versus what was observed.
    pub explanation: String,
    /// Export-safe concrete recovery action; never empty.
    pub recovery_action: String,
}

/// Explicit revocation of an authority grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RevokeEvent {
    /// RFC 3339 revocation timestamp.
    pub revoked_at: String,
    /// Export-safe reference to the principal or authority that revoked the grant.
    pub revoked_by: String,
    /// Fallback the surface narrows to once revoked.
    pub narrowed_to: M5DegradedFallback,
    /// Export-safe reason for the revocation; never empty.
    pub reason: String,
}

/// One authority grant threaded through its whole issue-use-revoke lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorityLedgerEntry {
    /// Stable ledger-entry id.
    pub entry_id: String,
    /// Matrix surface this grant is bound to.
    pub surface: M5ExecutingSurface,
    /// The mutating or privileged action class the grant authorizes.
    pub action_class: M5TicketActionClass,
    /// Actor the grant was minted for.
    pub actor: M5TicketActor,
    /// Target the grant binds to.
    pub target: M5TicketTarget,
    /// Cross-cutting linkage joining command, session, ticket, and envelope.
    pub linkage: M5LedgerLinkage,
    /// Issue event.
    pub issue: M5IssueEvent,
    /// Ordered use events; empty exactly when the grant is `Issued`.
    pub uses: Vec<M5UseEvent>,
    /// Invalidation; present exactly when the grant is `Invalidated`.
    pub invalidation: Option<M5Invalidation>,
    /// Revocation; present exactly when the grant is `Revoked`.
    pub revocation: Option<M5RevokeEvent>,
    /// Current lifecycle state.
    pub lifecycle_state: M5AuthorityLifecycleState,
    /// Downgrade triggers applied to this grant; empty when spendable.
    pub applied_downgrade_triggers: Vec<M5RuntimeAuthorityDowngradeTrigger>,
    /// Per-entry redaction class token.
    pub redaction_class_token: String,
}

/// Trust and isolation review block for the authority-lifecycle ledger packet.
///
/// Every field encodes a hard invariant; all must hold for the packet to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorityLifecycleTrustReview {
    /// No grant confers ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No helper actor self-issues authority.
    pub no_self_issued_authority_by_helpers: bool,
    /// Every grant joins command, session, ticket, target, and envelope lineage.
    pub every_grant_joins_full_lineage: bool,
    /// Invalidation fires on a named drift dimension, never a generic error.
    pub invalidation_names_drift_dimension: bool,
    /// Invalidation and revocation carry a concrete recovery or reason.
    pub invalidation_carries_recovery: bool,
    /// Lifecycle state stays coherent with the recorded events.
    pub lifecycle_state_coherent_with_events: bool,
    /// Use outcomes are consistent with the grant's recorded state.
    pub use_outcomes_consistent_with_state: bool,
    /// Secret references are handle-only; no raw secret material is recorded.
    pub secret_refs_handle_only_no_raw_material: bool,
    /// Enforcement narrows or fails closed when it cannot be honored.
    pub fail_closed_when_enforcement_unavailable: bool,
    /// No raw secret material is exported inside ledgers or support packets.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block for the authority-lifecycle ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorityLifecycleConsumerProjection {
    /// Desktop shows the full lifecycle and any invalidation or revocation.
    pub desktop_shows_lifecycle_and_invalidation: bool,
    /// Command palette and policy inspector reference the same ledger entries.
    pub command_and_policy_reference_same_entries: bool,
    /// CLI / headless reads the same ledger offline.
    pub cli_headless_reads_ledger_offline: bool,
    /// Support export shows the full issue-use-revoke ledger.
    pub support_export_shows_full_ledger: bool,
    /// Diagnostics shows the full ledger.
    pub diagnostics_shows_full_ledger: bool,
    /// Incident review consumes the ledger for after-the-fact reasoning.
    pub incident_review_consumes_ledger: bool,
    /// Release evidence consumes the ledger instead of cloning per-surface prose.
    pub release_evidence_consumes_ledger: bool,
    /// Remote and browser-routed surfaces preserve ledger semantics off-device.
    pub remote_and_browser_preserve_ledger_semantics: bool,
}

/// Proof-freshness block for the authority-lifecycle ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorityLifecycleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected grants.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5AuthorityLifecycleLedgerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AuthorityLifecycleLedgerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Ledger entries.
    pub entries: Vec<M5AuthorityLedgerEntry>,
    /// Trust review block.
    pub trust_review: M5AuthorityLifecycleTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5AuthorityLifecycleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AuthorityLifecycleProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 authority-lifecycle ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorityLifecycleLedgerPacket {
    /// Record kind; must equal [`M5_AUTHORITY_LIFECYCLE_LEDGER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Ledger entries.
    pub entries: Vec<M5AuthorityLedgerEntry>,
    /// Trust review block.
    pub trust_review: M5AuthorityLifecycleTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5AuthorityLifecycleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AuthorityLifecycleProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AuthorityLifecycleLedgerPacket {
    /// Builds an M5 authority-lifecycle ledger packet from frozen input.
    pub fn new(input: M5AuthorityLifecycleLedgerPacketInput) -> Self {
        Self {
            record_kind: M5_AUTHORITY_LIFECYCLE_LEDGER_RECORD_KIND.to_owned(),
            schema_version: M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            entries: input.entries,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 authority-lifecycle ledger packet invariants.
    pub fn validate(&self) -> Vec<M5AuthorityLifecycleLedgerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AUTHORITY_LIFECYCLE_LEDGER_RECORD_KIND {
            violations.push(M5AuthorityLifecycleLedgerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_VERSION {
            violations.push(M5AuthorityLifecycleLedgerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AuthorityLifecycleLedgerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_entries(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 authority-lifecycle ledger packet serializes"),
        ) {
            violations.push(M5AuthorityLifecycleLedgerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 authority-lifecycle ledger packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let invalidated = self
            .entries
            .iter()
            .filter(|entry| entry.lifecycle_state == M5AuthorityLifecycleState::Invalidated)
            .count();
        let revoked = self
            .entries
            .iter()
            .filter(|entry| entry.lifecycle_state == M5AuthorityLifecycleState::Revoked)
            .count();
        let mut out = String::new();
        out.push_str("# M5 Issue-Use-Revoke Authority-Lifecycle Ledger\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Entries: {} ({} invalidated by drift, {} revoked)\n",
            self.entries.len(),
            invalidated,
            revoked
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Authority grants\n\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "- **{}** ({}) — {} on {} — state: {}\n",
                entry.action_class.as_str(),
                entry.entry_id,
                entry.surface.as_str(),
                entry.issue.sandbox_profile.as_str(),
                entry.lifecycle_state.as_str()
            ));
            out.push_str(&format!(
                "  - Flow: {} · Actor: {} (`{}`) · Issuer: {}\n",
                entry.linkage.origin_flow.as_str(),
                entry.actor.actor_class.as_str(),
                entry.actor.actor_ref,
                entry.issue.issuer_class.as_str()
            ));
            out.push_str(&format!(
                "  - Target: {} ({}) · Session: {} · Ticket: {} · Envelope: {}\n",
                entry.target.target_identity,
                if entry.target.off_device {
                    "off-device"
                } else {
                    "on-device"
                },
                entry.linkage.session_ref,
                entry.linkage.approval_ticket_ref,
                entry.linkage.capability_envelope_ref
            ));
            out.push_str(&format!(
                "  - Issued: {} (epoch {}, ttl {}s) · Uses: {}\n",
                entry.issue.issued_at,
                entry.issue.policy_epoch.epoch_id,
                entry.issue.ttl_seconds,
                entry.uses.len()
            ));
            for use_event in &entry.uses {
                out.push_str(&format!(
                    "    - Use #{} {}: {} — {}\n",
                    use_event.sequence,
                    use_event.used_at,
                    use_event.outcome.as_str(),
                    use_event.note
                ));
            }
            if let Some(invalidation) = &entry.invalidation {
                out.push_str(&format!(
                    "  - Invalidated: {} → narrows to {} — {} · Recover: {}\n",
                    invalidation.drift_dimension.as_str(),
                    invalidation.narrowed_to.as_str(),
                    invalidation.explanation,
                    invalidation.recovery_action
                ));
            }
            if let Some(revocation) = &entry.revocation {
                out.push_str(&format!(
                    "  - Revoked by {} → narrows to {} — {}\n",
                    revocation.revoked_by,
                    revocation.narrowed_to.as_str(),
                    revocation.reason
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 authority-lifecycle ledger export.
#[derive(Debug)]
pub enum M5AuthorityLifecycleLedgerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AuthorityLifecycleLedgerViolation>),
}

impl fmt::Display for M5AuthorityLifecycleLedgerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 authority-lifecycle ledger export parse failed: {error}"
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
                    "m5 authority-lifecycle ledger export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AuthorityLifecycleLedgerArtifactError {}

/// Validation failures emitted by [`M5AuthorityLifecycleLedgerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AuthorityLifecycleLedgerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required origin flow family has no ledger entry.
    RequiredOriginFlowMissing,
    /// A required drift dimension is never demonstrated by an invalidation.
    RequiredDriftDimensionMissing,
    /// An entry is missing required identity fields.
    EntryIncomplete,
    /// An entry omits required linkage (session, ticket, or envelope).
    LinkageIncomplete,
    /// An entry's grant binds a sandbox profile that widens its matrix default.
    SandboxProfileWidens,
    /// An entry omits a non-zero expiry on its issue event.
    IssueExpiryIncomplete,
    /// A helper actor self-issues authority instead of carrying external lineage.
    SelfIssuedAuthorityForbidden,
    /// A privileged grant omits its issuance decision chain.
    IssuanceLineageMissing,
    /// A valid off-device grant binds an unverified target.
    OffDeviceTargetUnverified,
    /// An entry's use sequence is not strictly increasing from 1.
    UseSequenceNotMonotonic,
    /// A use event omits its note.
    UseNoteMissing,
    /// A use outcome is inconsistent with the grant's recorded state.
    UseOutcomeInconsistent,
    /// Lifecycle state and the recorded events are incoherent.
    LifecycleStateIncoherent,
    /// An invalidated entry omits its invalidation block.
    InvalidationMissing,
    /// An invalidation's trigger is inconsistent with its drift dimension.
    InvalidationTriggerMismatch,
    /// An invalidation omits its explanation.
    InvalidationExplanationMissing,
    /// An invalidation omits its recovery action.
    InvalidationRecoveryMissing,
    /// A revoked entry omits its revocation block.
    RevocationMissing,
    /// A revocation omits its reason.
    RevocationReasonMissing,
    /// A terminated entry omits the downgrade trigger for its termination.
    TerminationTriggerMissing,
    /// A spendable entry carries an invalidation, revocation, or downgrade trigger.
    SpendableEntryCarriesTermination,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5AuthorityLifecycleLedgerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredOriginFlowMissing => "required_origin_flow_missing",
            Self::RequiredDriftDimensionMissing => "required_drift_dimension_missing",
            Self::EntryIncomplete => "entry_incomplete",
            Self::LinkageIncomplete => "linkage_incomplete",
            Self::SandboxProfileWidens => "sandbox_profile_widens",
            Self::IssueExpiryIncomplete => "issue_expiry_incomplete",
            Self::SelfIssuedAuthorityForbidden => "self_issued_authority_forbidden",
            Self::IssuanceLineageMissing => "issuance_lineage_missing",
            Self::OffDeviceTargetUnverified => "off_device_target_unverified",
            Self::UseSequenceNotMonotonic => "use_sequence_not_monotonic",
            Self::UseNoteMissing => "use_note_missing",
            Self::UseOutcomeInconsistent => "use_outcome_inconsistent",
            Self::LifecycleStateIncoherent => "lifecycle_state_incoherent",
            Self::InvalidationMissing => "invalidation_missing",
            Self::InvalidationTriggerMismatch => "invalidation_trigger_mismatch",
            Self::InvalidationExplanationMissing => "invalidation_explanation_missing",
            Self::InvalidationRecoveryMissing => "invalidation_recovery_missing",
            Self::RevocationMissing => "revocation_missing",
            Self::RevocationReasonMissing => "revocation_reason_missing",
            Self::TerminationTriggerMissing => "termination_trigger_missing",
            Self::SpendableEntryCarriesTermination => "spendable_entry_carries_termination",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen stable M5 authority-lifecycle ledger packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_AUTHORITY_LIFECYCLE_LEDGER_ARTIFACT_REF`]; the dumper emits this packet
/// and a test asserts the checked-in artifact deserializes back to it unchanged.
pub fn frozen_stable_m5_authority_lifecycle_ledger_packet() -> M5AuthorityLifecycleLedgerPacket {
    let mut entries = active_entries();
    entries.extend(invalidated_entries());
    entries.extend(revoked_entries());
    build_lifecycle_ledger_packet(
        M5_AUTHORITY_LIFECYCLE_LEDGER_PACKET_ID,
        "M5 Issue-Use-Revoke Authority-Lifecycle Ledger",
        entries,
    )
}

/// Builds a ledger packet from a set of entries, applying the frozen review,
/// projection, freshness, and source-contract blocks.
///
/// Shared by the canonical packet and the checked fixtures so they cannot drift
/// in their trust posture.
pub fn build_lifecycle_ledger_packet(
    packet_id: &str,
    packet_label: &str,
    entries: Vec<M5AuthorityLedgerEntry>,
) -> M5AuthorityLifecycleLedgerPacket {
    M5AuthorityLifecycleLedgerPacket::new(M5AuthorityLifecycleLedgerPacketInput {
        packet_id: packet_id.to_owned(),
        packet_label: packet_label.to_owned(),
        entries,
        trust_review: M5AuthorityLifecycleTrustReview {
            no_ambient_machine_privilege: true,
            no_self_issued_authority_by_helpers: true,
            every_grant_joins_full_lineage: true,
            invalidation_names_drift_dimension: true,
            invalidation_carries_recovery: true,
            lifecycle_state_coherent_with_events: true,
            use_outcomes_consistent_with_state: true,
            secret_refs_handle_only_no_raw_material: true,
            fail_closed_when_enforcement_unavailable: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5AuthorityLifecycleConsumerProjection {
            desktop_shows_lifecycle_and_invalidation: true,
            command_and_policy_reference_same_entries: true,
            cli_headless_reads_ledger_offline: true,
            support_export_shows_full_ledger: true,
            diagnostics_shows_full_ledger: true,
            incident_review_consumes_ledger: true,
            release_evidence_consumes_ledger: true,
            remote_and_browser_preserve_ledger_semantics: true,
        },
        proof_freshness: M5AuthorityLifecycleProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_REF.to_owned(),
            M5_AUTHORITY_LIFECYCLE_LEDGER_DOC_REF.to_owned(),
            M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF.to_owned(),
            M5_APPROVAL_TICKET_LEDGER_DOC_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_SCHEMA_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

/// Active (issued-and-used) grants that completed without drift or revocation.
pub fn active_entries() -> Vec<M5AuthorityLedgerEntry> {
    vec![scaffold_active_entry(), notebook_active_entry()]
}

/// Grants invalidated by drift, one per drift dimension.
pub fn invalidated_entries() -> Vec<M5AuthorityLedgerEntry> {
    vec![
        request_target_drift_entry(),
        database_policy_drift_entry(),
        remote_sandbox_drift_entry(),
        incident_trust_drift_entry(),
        browser_network_drift_entry(),
    ]
}

/// Grants terminated by an explicit revocation.
pub fn revoked_entries() -> Vec<M5AuthorityLedgerEntry> {
    vec![ai_revoked_entry()]
}

/// Grants that are issued but not yet used (lifecycle `Issued`).
///
/// Not part of the canonical packet; used by checked fixtures to exercise the
/// pre-use lifecycle state.
pub fn issued_entries() -> Vec<M5AuthorityLedgerEntry> {
    vec![preview_issued_entry()]
}

/// Grants terminated by expiry (lifecycle `Expired`).
///
/// Not part of the canonical packet; used by checked fixtures to exercise the
/// expiry lifecycle state and the denied-by-expiry use outcome.
pub fn expired_entries() -> Vec<M5AuthorityLedgerEntry> {
    vec![recipe_expired_entry()]
}

fn policy_epoch(superseded: bool) -> M5PolicyEpochBinding {
    M5PolicyEpochBinding {
        epoch_id: "policy-epoch:m5:0007".to_owned(),
        epoch_sequence: 7,
        superseded,
    }
}

fn decision_chain(issuer_ref: &str, entry_id: &str) -> Vec<String> {
    vec![
        "policy-epoch:m5:0007".to_owned(),
        issuer_ref.to_owned(),
        entry_id.to_owned(),
    ]
}

fn scaffold_active_entry() -> M5AuthorityLedgerEntry {
    M5AuthorityLedgerEntry {
        entry_id: "ledger:scaffold-hook:0001".to_owned(),
        surface: M5ExecutingSurface::ScaffoldHook,
        action_class: M5TicketActionClass::WorkspaceMutation,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::SystemAutomation,
            actor_ref: "actor:scaffold-generator".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "workspace://project/templates".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::Execution,
            command_ref: Some("command:scaffold.generate".to_owned()),
            session_ref: "session:scaffold:0001".to_owned(),
            approval_ticket_ref: "ticket:scaffold-hook:0001".to_owned(),
            capability_envelope_ref: "envelope:scaffold-hook:0001".to_owned(),
            capability_envelope_hash: "sha256:1a2b3c4d5e6f7081".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            secret_scope: M5SecretScope::NoSecretAccess,
            expires_at: "2026-06-10T00:10:00Z".to_owned(),
            ttl_seconds: 600,
            decision_chain: decision_chain(
                "issuer:approval-broker:local",
                "ledger:scaffold-hook:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![M5UseEvent {
            used_at: "2026-06-10T00:00:30Z".to_owned(),
            sequence: 1,
            outcome: M5UseOutcome::AllowedExecuted,
            narrowed_to: None,
            note: "Generated project templates under the scoped workspace root.".to_owned(),
        }],
        invalidation: None,
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Active,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn notebook_active_entry() -> M5AuthorityLedgerEntry {
    M5AuthorityLedgerEntry {
        entry_id: "ledger:notebook-kernel:0001".to_owned(),
        surface: M5ExecutingSurface::NotebookKernel,
        action_class: M5TicketActionClass::ProcessExecution,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:notebook-author".to_owned(),
            on_behalf_of: None,
        },
        target: M5TicketTarget {
            target_identity: "kernel://project/notebook-7".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::Execution,
            command_ref: Some("command:notebook.run-cell".to_owned()),
            session_ref: "session:notebook:0001".to_owned(),
            approval_ticket_ref: "ticket:notebook-kernel:0001".to_owned(),
            capability_envelope_ref: "envelope:notebook-kernel:0001".to_owned(),
            capability_envelope_hash: "sha256:2b3c4d5e6f708192".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            secret_scope: M5SecretScope::NoSecretAccess,
            expires_at: "2026-06-10T02:00:00Z".to_owned(),
            ttl_seconds: 7200,
            decision_chain: decision_chain(
                "issuer:approval-broker:local",
                "ledger:notebook-kernel:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:01:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Ran notebook cell 3 in the isolated kernel subprocess.".to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:05:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Ran notebook cell 4 in the isolated kernel subprocess.".to_owned(),
            },
        ],
        invalidation: None,
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Active,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn request_target_drift_entry() -> M5AuthorityLedgerEntry {
    let invalidation = invalidation_for(
        M5DriftDimension::TargetIdentityDrift,
        "2026-06-10T00:30:00Z",
        "The bound endpoint identity https://api.example.test/v1/orders no longer matches the verified target.",
        "Re-verify the endpoint identity and request a fresh ticket before sending again.",
    );
    M5AuthorityLedgerEntry {
        entry_id: "ledger:request-api-send:0001".to_owned(),
        surface: M5ExecutingSurface::RequestApiSend,
        action_class: M5TicketActionClass::NetworkSend,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::Extension,
            actor_ref: "actor:request-sender-extension".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "https://api.example.test/v1/orders".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::ProviderLinked,
            command_ref: Some("command:request.send".to_owned()),
            session_ref: "session:request:0001".to_owned(),
            approval_ticket_ref: "ticket:request-api-send:0001".to_owned(),
            capability_envelope_ref: "envelope:request-api-send:0001".to_owned(),
            capability_envelope_hash: "sha256:3c4d5e6f70819203".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
            secret_scope: M5SecretScope::HandleOnlyDelegated,
            expires_at: "2026-06-10T01:00:00Z".to_owned(),
            ttl_seconds: 3600,
            decision_chain: decision_chain(
                "issuer:approval-broker:local",
                "ledger:request-api-send:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:10:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Sent the first order request to the verified endpoint.".to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:31:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::DeniedInvalidated,
                narrowed_to: Some(M5DriftDimension::TargetIdentityDrift.default_fallback()),
                note: "Blocked the retry after the endpoint identity drifted; required a fresh ticket.".to_owned(),
            },
        ],
        applied_downgrade_triggers: vec![invalidation.trigger],
        invalidation: Some(invalidation),
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Invalidated,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn database_policy_drift_entry() -> M5AuthorityLedgerEntry {
    let invalidation = invalidation_for(
        M5DriftDimension::PolicyEpochDrift,
        "2026-06-10T00:12:00Z",
        "Policy epoch policy-epoch:m5:0007 was superseded after this grant was issued.",
        "Re-issue the database grant under the current policy epoch before writing again.",
    );
    M5AuthorityLedgerEntry {
        entry_id: "ledger:database-action:0001".to_owned(),
        surface: M5ExecutingSurface::DatabaseAction,
        action_class: M5TicketActionClass::DatabaseWrite,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:db-operator".to_owned(),
            on_behalf_of: None,
        },
        target: M5TicketTarget {
            target_identity: "db://project/orders".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::ProviderLinked,
            command_ref: Some("command:database.write".to_owned()),
            session_ref: "session:database:0001".to_owned(),
            approval_ticket_ref: "ticket:database-action:0001".to_owned(),
            capability_envelope_ref: "envelope:database-action:0001".to_owned(),
            capability_envelope_hash: "sha256:4d5e6f7081920314".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(true),
            sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
            secret_scope: M5SecretScope::HandleOnlyDelegated,
            expires_at: "2026-06-10T00:15:00Z".to_owned(),
            ttl_seconds: 900,
            decision_chain: decision_chain(
                "issuer:approval-broker:local",
                "ledger:database-action:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:05:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Applied the first order-status write under the issued epoch.".to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:13:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::DeniedInvalidated,
                narrowed_to: Some(M5DriftDimension::PolicyEpochDrift.default_fallback()),
                note: "Narrowed the follow-up write to a sanitized preview after the epoch was superseded.".to_owned(),
            },
        ],
        applied_downgrade_triggers: vec![invalidation.trigger],
        invalidation: Some(invalidation),
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Invalidated,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn remote_sandbox_drift_entry() -> M5AuthorityLedgerEntry {
    let invalidation = invalidation_for(
        M5DriftDimension::SandboxProfileDrift,
        "2026-06-10T00:20:00Z",
        "The isolated remote runtime profile became unavailable on the managed host.",
        "Re-issue the remote-mutation grant once a supported isolated profile is available.",
    );
    M5AuthorityLedgerEntry {
        entry_id: "ledger:remote-mutation:0001".to_owned(),
        surface: M5ExecutingSurface::RemoteMutation,
        action_class: M5TicketActionClass::RemoteMutation,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::RemoteHelper,
            actor_ref: "actor:remote-mutation-helper".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "remote://managed-host/project/deploy".to_owned(),
            off_device: true,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::Remote,
            command_ref: Some("command:remote.apply".to_owned()),
            session_ref: "session:remote:0001".to_owned(),
            approval_ticket_ref: "ticket:remote-mutation:0001".to_owned(),
            capability_envelope_ref: "envelope:remote-mutation:0001".to_owned(),
            capability_envelope_hash: "sha256:5e6f708192031425".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::RemoteBrokerRuntime,
            issuer_ref: "issuer:remote-broker:managed".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
            secret_scope: M5SecretScope::HandleOnlyDelegated,
            expires_at: "2026-06-10T00:30:00Z".to_owned(),
            ttl_seconds: 1800,
            decision_chain: decision_chain(
                "issuer:remote-broker:managed",
                "ledger:remote-mutation:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:10:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Applied the first remote deploy step in the isolated runtime.".to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:21:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::DeniedInvalidated,
                narrowed_to: Some(M5DriftDimension::SandboxProfileDrift.default_fallback()),
                note:
                    "Failed the remaining step closed after the isolated runtime profile drifted."
                        .to_owned(),
            },
        ],
        applied_downgrade_triggers: vec![invalidation.trigger],
        invalidation: Some(invalidation),
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Invalidated,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn incident_trust_drift_entry() -> M5AuthorityLedgerEntry {
    let invalidation = invalidation_for(
        M5DriftDimension::TrustAnchorDrift,
        "2026-06-10T00:08:00Z",
        "The trust-store anchor backing this repair flow changed mid-session.",
        "Re-establish the trust anchor and re-issue the repair grant before continuing.",
    );
    M5AuthorityLedgerEntry {
        entry_id: "ledger:incident-flow:0001".to_owned(),
        surface: M5ExecutingSurface::IncidentFlow,
        action_class: M5TicketActionClass::NetworkSend,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::SystemAutomation,
            actor_ref: "actor:repair-runner".to_owned(),
            on_behalf_of: Some("operator:incident-responder".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "https://incident.example.test/repair".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::Repair,
            command_ref: Some("command:incident.repair".to_owned()),
            session_ref: "session:incident:0001".to_owned(),
            approval_ticket_ref: "ticket:incident-flow:0001".to_owned(),
            capability_envelope_ref: "envelope:incident-flow:0001".to_owned(),
            capability_envelope_hash: "sha256:6f70819203142536".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::PolicyAuthority,
            issuer_ref: "issuer:policy-authority:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
            secret_scope: M5SecretScope::NoSecretAccess,
            expires_at: "2026-06-10T00:30:00Z".to_owned(),
            ttl_seconds: 1800,
            decision_chain: decision_chain(
                "issuer:policy-authority:local",
                "ledger:incident-flow:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:03:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Wrote the first recovery checkpoint under the issued trust anchor."
                    .to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:09:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::DeniedInvalidated,
                narrowed_to: Some(M5DriftDimension::TrustAnchorDrift.default_fallback()),
                note: "Failed the next repair step closed after the trust anchor drifted."
                    .to_owned(),
            },
        ],
        applied_downgrade_triggers: vec![invalidation.trigger],
        invalidation: Some(invalidation),
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Invalidated,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn browser_network_drift_entry() -> M5AuthorityLedgerEntry {
    let invalidation = invalidation_for(
        M5DriftDimension::NetworkPostureDrift,
        "2026-06-10T00:06:00Z",
        "The egress route narrowed when the enterprise proxy posture changed.",
        "Fall back to offline local-core behavior and re-issue once the route is restored.",
    );
    M5AuthorityLedgerEntry {
        entry_id: "ledger:browser-routed-action:0001".to_owned(),
        surface: M5ExecutingSurface::BrowserRoutedAction,
        action_class: M5TicketActionClass::BrowserRoutedAction,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::BrowserRoute,
            actor_ref: "actor:browser-route-helper".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "https://preview.example.test/session".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::ProviderLinked,
            command_ref: Some("command:browser.route-action".to_owned()),
            session_ref: "session:browser:0001".to_owned(),
            approval_ticket_ref: "ticket:browser-routed-action:0001".to_owned(),
            capability_envelope_ref: "envelope:browser-routed-action:0001".to_owned(),
            capability_envelope_hash: "sha256:70819203142536f4".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
            secret_scope: M5SecretScope::NoSecretAccess,
            expires_at: "2026-06-10T00:30:00Z".to_owned(),
            ttl_seconds: 1800,
            decision_chain: decision_chain(
                "issuer:approval-broker:local",
                "ledger:browser-routed-action:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![M5UseEvent {
            used_at: "2026-06-10T00:07:00Z".to_owned(),
            sequence: 1,
            outcome: M5UseOutcome::DeniedInvalidated,
            narrowed_to: Some(M5DriftDimension::NetworkPostureDrift.default_fallback()),
            note: "Blocked the routed page action and fell back to offline local-core behavior after the egress route narrowed.".to_owned(),
        }],
        applied_downgrade_triggers: vec![invalidation.trigger],
        invalidation: Some(invalidation),
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Invalidated,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn ai_revoked_entry() -> M5AuthorityLedgerEntry {
    M5AuthorityLedgerEntry {
        entry_id: "ledger:ai-tool:0001".to_owned(),
        surface: M5ExecutingSurface::AiTool,
        action_class: M5TicketActionClass::SecretProjection,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::AiTool,
            actor_ref: "actor:ai-tool:composer".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "secret-handle://project/api-token".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::AiAssisted,
            command_ref: Some("command:ai.invoke-tool".to_owned()),
            session_ref: "session:ai:0001".to_owned(),
            approval_ticket_ref: "ticket:ai-tool:0001".to_owned(),
            capability_envelope_ref: "envelope:ai-tool:0001".to_owned(),
            capability_envelope_hash: "sha256:819203142536f4a5".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            secret_scope: M5SecretScope::HandleOnlyDelegated,
            expires_at: "2026-06-10T00:20:00Z".to_owned(),
            ttl_seconds: 1200,
            decision_chain: decision_chain("issuer:approval-broker:local", "ledger:ai-tool:0001"),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:02:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Projected the handle-only token reference into one approved tool call."
                    .to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:11:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::DeniedRevoked,
                narrowed_to: Some(M5DegradedFallback::FailClosedBlock),
                note: "Blocked a further projection after the operator revoked the AI tool grant."
                    .to_owned(),
            },
        ],
        invalidation: None,
        revocation: Some(M5RevokeEvent {
            revoked_at: "2026-06-10T00:10:00Z".to_owned(),
            revoked_by: "operator:project-owner".to_owned(),
            narrowed_to: M5DegradedFallback::FailClosedBlock,
            reason: "Operator revoked the AI tool's standing secret-projection grant.".to_owned(),
        }),
        lifecycle_state: M5AuthorityLifecycleState::Revoked,
        applied_downgrade_triggers: vec![
            M5RuntimeAuthorityDowngradeTrigger::ScopeExpansionUnqualified,
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn preview_issued_entry() -> M5AuthorityLedgerEntry {
    M5AuthorityLedgerEntry {
        entry_id: "ledger:preview-server:0001".to_owned(),
        surface: M5ExecutingSurface::PreviewServer,
        action_class: M5TicketActionClass::ProcessExecution,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:preview-author".to_owned(),
            on_behalf_of: None,
        },
        target: M5TicketTarget {
            target_identity: "preview://project/dev-server".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::Execution,
            command_ref: Some("command:preview.start".to_owned()),
            session_ref: "session:preview:0001".to_owned(),
            approval_ticket_ref: "ticket:preview-server:0001".to_owned(),
            capability_envelope_ref: "envelope:preview-server:0001".to_owned(),
            capability_envelope_hash: "sha256:9203142536f4a5b6".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::ContainerIsolatedLocal,
            secret_scope: M5SecretScope::NoSecretAccess,
            expires_at: "2026-06-10T01:00:00Z".to_owned(),
            ttl_seconds: 3600,
            decision_chain: decision_chain(
                "issuer:approval-broker:local",
                "ledger:preview-server:0001",
            ),
            self_issued_by_executor: false,
        },
        uses: vec![],
        invalidation: None,
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Issued,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn recipe_expired_entry() -> M5AuthorityLedgerEntry {
    M5AuthorityLedgerEntry {
        entry_id: "ledger:recipe:0001".to_owned(),
        surface: M5ExecutingSurface::Recipe,
        action_class: M5TicketActionClass::ProcessExecution,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::Recipe,
            actor_ref: "actor:saved-recipe".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "recipe://project/nightly-build".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        linkage: M5LedgerLinkage {
            origin_flow: M5OriginFlowClass::Execution,
            command_ref: Some("command:recipe.run".to_owned()),
            session_ref: "session:recipe:0001".to_owned(),
            approval_ticket_ref: "ticket:recipe:0001".to_owned(),
            capability_envelope_ref: "envelope:recipe:0001".to_owned(),
            capability_envelope_hash: "sha256:a5b6c7d8e9f00112".to_owned(),
        },
        issue: M5IssueEvent {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            secret_scope: M5SecretScope::NoSecretAccess,
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            decision_chain: decision_chain("issuer:approval-broker:local", "ledger:recipe:0001"),
            self_issued_by_executor: false,
        },
        uses: vec![
            M5UseEvent {
                used_at: "2026-06-10T00:01:00Z".to_owned(),
                sequence: 1,
                outcome: M5UseOutcome::AllowedExecuted,
                narrowed_to: None,
                note: "Ran the recipe's first build step within the grant window.".to_owned(),
            },
            M5UseEvent {
                used_at: "2026-06-10T00:06:00Z".to_owned(),
                sequence: 2,
                outcome: M5UseOutcome::DeniedExpired,
                narrowed_to: Some(M5DegradedFallback::RequireFreshTicket),
                note: "Blocked the next step after the grant's time-to-live elapsed.".to_owned(),
            },
        ],
        invalidation: None,
        revocation: None,
        lifecycle_state: M5AuthorityLifecycleState::Expired,
        applied_downgrade_triggers: vec![M5RuntimeAuthorityDowngradeTrigger::ApprovalTicketExpired],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn invalidation_for(
    drift_dimension: M5DriftDimension,
    detected_at: &str,
    explanation: &str,
    recovery_action: &str,
) -> M5Invalidation {
    M5Invalidation {
        detected_at: detected_at.to_owned(),
        drift_dimension,
        trigger: drift_dimension.trigger(),
        narrowed_to: drift_dimension.default_fallback(),
        explanation: explanation.to_owned(),
        recovery_action: recovery_action.to_owned(),
    }
}

fn validate_source_contracts(
    packet: &M5AuthorityLifecycleLedgerPacket,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AUTHORITY_LIFECYCLE_LEDGER_SCHEMA_REF,
        M5_AUTHORITY_LIFECYCLE_LEDGER_DOC_REF,
        M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF,
        M5_APPROVAL_TICKET_LEDGER_DOC_REF,
        M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
        M5_CAPABILITY_ENVELOPE_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AuthorityLifecycleLedgerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_coverage(
    packet: &M5AuthorityLifecycleLedgerPacket,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let flows_present: BTreeSet<M5OriginFlowClass> = packet
        .entries
        .iter()
        .map(|entry| entry.linkage.origin_flow)
        .collect();
    for required in M5OriginFlowClass::ALL {
        if !flows_present.contains(&required) {
            violations.push(M5AuthorityLifecycleLedgerViolation::RequiredOriginFlowMissing);
            break;
        }
    }

    let drift_present: BTreeSet<M5DriftDimension> = packet
        .entries
        .iter()
        .filter_map(|entry| entry.invalidation.as_ref().map(|inv| inv.drift_dimension))
        .collect();
    for required in M5DriftDimension::ALL {
        if !drift_present.contains(&required) {
            violations.push(M5AuthorityLifecycleLedgerViolation::RequiredDriftDimensionMissing);
            break;
        }
    }
}

fn validate_entries(
    packet: &M5AuthorityLifecycleLedgerPacket,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let default_profile_by_surface: BTreeMap<M5ExecutingSurface, M5SandboxProfile> =
        frozen_stable_m5_runtime_authority_matrix_packet()
            .surface_rows
            .iter()
            .map(|row| (row.surface, row.default_sandbox_profile))
            .collect();

    for entry in &packet.entries {
        validate_entry_identity(entry, violations);
        validate_entry_issue(entry, &default_profile_by_surface, violations);
        validate_entry_uses(entry, violations);
        validate_entry_lifecycle(entry, violations);
    }
}

fn validate_entry_identity(
    entry: &M5AuthorityLedgerEntry,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    if entry.entry_id.trim().is_empty()
        || entry.redaction_class_token.trim().is_empty()
        || entry.target.target_identity.trim().is_empty()
        || entry.actor.actor_ref.trim().is_empty()
    {
        violations.push(M5AuthorityLifecycleLedgerViolation::EntryIncomplete);
    }

    let linkage = &entry.linkage;
    if linkage.session_ref.trim().is_empty()
        || linkage.approval_ticket_ref.trim().is_empty()
        || linkage.capability_envelope_ref.trim().is_empty()
        || linkage.capability_envelope_hash.trim().is_empty()
    {
        violations.push(M5AuthorityLifecycleLedgerViolation::LinkageIncomplete);
    }

    // A valid (spendable) off-device grant must bind a verified target; a
    // terminated grant may bind an unverified target precisely because it failed.
    if entry.lifecycle_state.is_spendable()
        && entry.target.off_device
        && !entry.target.identity_verified
    {
        violations.push(M5AuthorityLifecycleLedgerViolation::OffDeviceTargetUnverified);
    }
}

fn validate_entry_issue(
    entry: &M5AuthorityLedgerEntry,
    default_profile_by_surface: &BTreeMap<M5ExecutingSurface, M5SandboxProfile>,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let issue = &entry.issue;

    // No executor self-issues authority; helper actors (AI, recipe, extension,
    // browser route, remote helper) are the case this most guards against.
    if issue.self_issued_by_executor {
        violations.push(M5AuthorityLifecycleLedgerViolation::SelfIssuedAuthorityForbidden);
    }

    if issue.issuer_ref.trim().is_empty()
        || issue.decision_chain.is_empty()
        || issue
            .decision_chain
            .iter()
            .any(|ref_| ref_.trim().is_empty())
    {
        violations.push(M5AuthorityLifecycleLedgerViolation::IssuanceLineageMissing);
    }

    if issue.ttl_seconds == 0
        || issue.issued_at.trim().is_empty()
        || issue.expires_at.trim().is_empty()
        || issue.policy_epoch.epoch_id.trim().is_empty()
    {
        violations.push(M5AuthorityLifecycleLedgerViolation::IssueExpiryIncomplete);
    }

    if let Some(default_profile) = default_profile_by_surface.get(&entry.surface) {
        if issue.sandbox_profile != *default_profile
            && issue.sandbox_profile != M5SandboxProfile::InertNoExecution
        {
            violations.push(M5AuthorityLifecycleLedgerViolation::SandboxProfileWidens);
        }
    }
}

fn validate_entry_uses(
    entry: &M5AuthorityLedgerEntry,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let mut expected_sequence = 1u64;
    for use_event in &entry.uses {
        if use_event.sequence != expected_sequence {
            violations.push(M5AuthorityLifecycleLedgerViolation::UseSequenceNotMonotonic);
        }
        expected_sequence = use_event.sequence.saturating_add(1);

        if use_event.note.trim().is_empty() {
            violations.push(M5AuthorityLifecycleLedgerViolation::UseNoteMissing);
        }

        let consistent = match use_event.outcome {
            M5UseOutcome::DeniedInvalidated => entry.invalidation.is_some(),
            M5UseOutcome::DeniedRevoked => entry.revocation.is_some(),
            M5UseOutcome::DeniedExpired => {
                entry.lifecycle_state == M5AuthorityLifecycleState::Expired
            }
            M5UseOutcome::AllowedExecuted | M5UseOutcome::NarrowedExecuted => true,
        };
        if !consistent {
            violations.push(M5AuthorityLifecycleLedgerViolation::UseOutcomeInconsistent);
        }
    }
}

fn validate_entry_lifecycle(
    entry: &M5AuthorityLedgerEntry,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    // Termination evidence (invalidation / revocation) must match the state, and
    // spendable grants must carry none of it.
    match entry.lifecycle_state {
        M5AuthorityLifecycleState::Invalidated => {
            if entry.invalidation.is_none() {
                violations.push(M5AuthorityLifecycleLedgerViolation::InvalidationMissing);
            }
            if entry.revocation.is_some() {
                violations.push(M5AuthorityLifecycleLedgerViolation::LifecycleStateIncoherent);
            }
        }
        M5AuthorityLifecycleState::Revoked => {
            if entry.revocation.is_none() {
                violations.push(M5AuthorityLifecycleLedgerViolation::RevocationMissing);
            }
            if entry.invalidation.is_some() {
                violations.push(M5AuthorityLifecycleLedgerViolation::LifecycleStateIncoherent);
            }
        }
        M5AuthorityLifecycleState::Expired => {
            if entry.invalidation.is_some() || entry.revocation.is_some() {
                violations.push(M5AuthorityLifecycleLedgerViolation::LifecycleStateIncoherent);
            }
        }
        M5AuthorityLifecycleState::Issued => {
            if !entry.uses.is_empty() {
                violations.push(M5AuthorityLifecycleLedgerViolation::LifecycleStateIncoherent);
            }
            if entry.invalidation.is_some() || entry.revocation.is_some() {
                violations
                    .push(M5AuthorityLifecycleLedgerViolation::SpendableEntryCarriesTermination);
            }
        }
        M5AuthorityLifecycleState::Active => {
            if entry.uses.is_empty() {
                violations.push(M5AuthorityLifecycleLedgerViolation::LifecycleStateIncoherent);
            }
            if entry.invalidation.is_some() || entry.revocation.is_some() {
                violations
                    .push(M5AuthorityLifecycleLedgerViolation::SpendableEntryCarriesTermination);
            }
        }
    }

    // A spendable grant must not carry downgrade triggers; a terminated grant
    // must carry one, and an invalidation's trigger must match its drift.
    if entry.lifecycle_state.is_spendable() {
        if !entry.applied_downgrade_triggers.is_empty() {
            violations.push(M5AuthorityLifecycleLedgerViolation::SpendableEntryCarriesTermination);
        }
    } else if entry.applied_downgrade_triggers.is_empty() {
        violations.push(M5AuthorityLifecycleLedgerViolation::TerminationTriggerMissing);
    }

    if let Some(invalidation) = &entry.invalidation {
        if invalidation.trigger != invalidation.drift_dimension.trigger() {
            violations.push(M5AuthorityLifecycleLedgerViolation::InvalidationTriggerMismatch);
        }
        if invalidation.explanation.trim().is_empty() {
            violations.push(M5AuthorityLifecycleLedgerViolation::InvalidationExplanationMissing);
        }
        if invalidation.recovery_action.trim().is_empty() {
            violations.push(M5AuthorityLifecycleLedgerViolation::InvalidationRecoveryMissing);
        }
        if !entry
            .applied_downgrade_triggers
            .contains(&invalidation.trigger)
        {
            violations.push(M5AuthorityLifecycleLedgerViolation::TerminationTriggerMissing);
        }
    }

    if let Some(revocation) = &entry.revocation {
        if revocation.reason.trim().is_empty() || revocation.revoked_by.trim().is_empty() {
            violations.push(M5AuthorityLifecycleLedgerViolation::RevocationReasonMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5AuthorityLifecycleLedgerPacket,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_self_issued_authority_by_helpers,
        review.every_grant_joins_full_lineage,
        review.invalidation_names_drift_dimension,
        review.invalidation_carries_recovery,
        review.lifecycle_state_coherent_with_events,
        review.use_outcomes_consistent_with_state,
        review.secret_refs_handle_only_no_raw_material,
        review.fail_closed_when_enforcement_unavailable,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5AuthorityLifecycleLedgerViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AuthorityLifecycleLedgerPacket,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_lifecycle_and_invalidation,
        projection.command_and_policy_reference_same_entries,
        projection.cli_headless_reads_ledger_offline,
        projection.support_export_shows_full_ledger,
        projection.diagnostics_shows_full_ledger,
        projection.incident_review_consumes_ledger,
        projection.release_evidence_consumes_ledger,
        projection.remote_and_browser_preserve_ledger_semantics,
    ] {
        if !ok {
            violations.push(M5AuthorityLifecycleLedgerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AuthorityLifecycleLedgerPacket,
    violations: &mut Vec<M5AuthorityLifecycleLedgerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AuthorityLifecycleLedgerViolation::ProofFreshnessIncomplete);
    }
}

/// Reads and validates the checked-in stable M5 authority-lifecycle ledger export.
pub fn current_stable_m5_authority_lifecycle_ledger_export(
) -> Result<M5AuthorityLifecycleLedgerPacket, M5AuthorityLifecycleLedgerArtifactError> {
    let packet: M5AuthorityLifecycleLedgerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/support_export.json"
    )))
    .map_err(M5AuthorityLifecycleLedgerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AuthorityLifecycleLedgerArtifactError::Validation(
            violations,
        ))
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
