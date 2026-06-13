//! Approval-ticket issuance, deny-reason packets, replay-nonce-or-expiry
//! enforcement, and local-first verification for M5 mutating or privileged
//! actions.
//!
//! The frozen runtime-authority matrix states *what posture* each claimed M5
//! executing surface requires; the capability-envelope packet states the
//! concrete authority issued for one execution. This module is the **verb
//! side** of that contract: the short-lived approval tickets actually minted
//! for mutating or privileged actions, the deny-reason packets emitted when a
//! ticket cannot be honored, and the local-first verification descriptor that
//! lets an allowed local action be checked offline without widening authority.
//!
//! Each [`M5ApprovalTicket`] binds the seven dimensions an enforcer must check
//! before letting a mutating action run:
//!
//! - **actor** — [`M5TicketActor`]: who is acting, and on whose behalf. Helper
//!   actors (AI, recipe, extension, browser route, remote helper) never
//!   self-issue authority.
//! - **action class** — [`M5TicketActionClass`]: the one mutating or privileged
//!   action the ticket authorizes, which pins a required capability class.
//! - **target identity** — [`M5TicketTarget`]: the export-safe resource the
//!   action touches, whether it is off-device, and whether it is verified.
//! - **policy epoch** — [`super::ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::M5PolicyEpochBinding`]:
//!   the governing epoch the ticket was minted under.
//! - **sandbox or capability hash** — [`M5TicketBinding`]: the sandbox profile,
//!   capability-envelope hash, and granted capability classes the ticket is
//!   pinned to, so it cannot be replayed for wider authority.
//! - **expiry** — [`M5TicketValidity`]: a non-zero TTL and a single-use marker.
//! - **replay protection** — [`M5ReplayProtection`]: a one-time nonce, a
//!   monotonic counter, and a bounded replay window.
//!
//! A denied or expired ticket does not collapse into a generic permission
//! error: it carries an [`M5TicketDenyReason`] that names the failed binding
//! dimension ([`M5TicketDenyDimension`]) and a concrete recovery action. A
//! [`M5LocalFirstVerification`] descriptor proves that an allowed local action
//! verifies offline ([`M5LocalFirstVerificationMethod::is_offline_capable`])
//! without depending on live control-plane reachability, without skipping audit
//! lineage, and without widening authority.
//!
//! The track invariant holds end to end: no ambient privilege; no helper
//! self-issues authority; every binding dimension stays inspectable and
//! export-safe; and if enforcement cannot be honored the ticket **narrows or
//! fails closed with a named deny reason** instead of silently widening. No raw
//! secret material, credential body, or live ticket signature is ever exported.
//!
//! The boundary schema is
//! [`schemas/execution-auth/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.schema.json`](../../../../schemas/execution-auth/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.md`](../../../../docs/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/`](../../../../fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5ApprovalTicketPosture, M5CapabilityClass,
    M5DegradedFallback, M5ExecutingSurface, M5RuntimeAuthorityDowngradeTrigger, M5SandboxProfile,
    M5SecretScope, M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF, M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF, M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
};
use super::ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::{
    M5EnvelopeActorClass, M5EnvelopeIssuerClass, M5PolicyEpochBinding, M5_CAPABILITY_ENVELOPE_DOC_REF,
    M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ApprovalTicketLedgerPacket`].
pub const M5_APPROVAL_TICKET_LEDGER_RECORD_KIND: &str =
    "implement_m5_approval_ticket_issuance_and_local_first_verification";

/// Schema version for the M5 approval-ticket ledger packet records.
pub const M5_APPROVAL_TICKET_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF: &str =
    "schemas/execution-auth/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_APPROVAL_TICKET_LEDGER_DOC_REF: &str =
    "docs/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_APPROVAL_TICKET_LEDGER_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_APPROVAL_TICKET_LEDGER_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_APPROVAL_TICKET_LEDGER_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f";

/// Stable packet id minted by [`frozen_stable_m5_approval_ticket_ledger_packet`].
pub const M5_APPROVAL_TICKET_LEDGER_PACKET_ID: &str = "m5-approval-ticket-ledger:stable:0001";

/// Class of mutating or privileged action an approval ticket authorizes.
///
/// Every action class pins exactly one [`M5CapabilityClass`] that must appear in
/// the ticket's bound capability classes ([`Self::required_capability`]); the
/// ticket cannot be reused to exercise a capability it was not minted for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TicketActionClass {
    /// A mutation of the local workspace tree.
    WorkspaceMutation,
    /// Spawning or executing a local process or kernel.
    ProcessExecution,
    /// An outbound network send / request.
    NetworkSend,
    /// A database write / mutation.
    DatabaseWrite,
    /// A remote-resource mutation.
    RemoteMutation,
    /// A browser-routed page action.
    BrowserRoutedAction,
    /// A handle-only secret projection into an action.
    SecretProjection,
}

impl M5TicketActionClass {
    /// Every action class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::WorkspaceMutation,
        Self::ProcessExecution,
        Self::NetworkSend,
        Self::DatabaseWrite,
        Self::RemoteMutation,
        Self::BrowserRoutedAction,
        Self::SecretProjection,
    ];

    /// Stable token recorded in the ticket.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceMutation => "workspace_mutation",
            Self::ProcessExecution => "process_execution",
            Self::NetworkSend => "network_send",
            Self::DatabaseWrite => "database_write",
            Self::RemoteMutation => "remote_mutation",
            Self::BrowserRoutedAction => "browser_routed_action",
            Self::SecretProjection => "secret_projection",
        }
    }

    /// The capability class this action class requires the ticket to bind.
    pub const fn required_capability(self) -> M5CapabilityClass {
        match self {
            Self::WorkspaceMutation => M5CapabilityClass::WriteWorkspace,
            Self::ProcessExecution => M5CapabilityClass::ProcessSpawn,
            Self::NetworkSend => M5CapabilityClass::NetworkEgress,
            Self::DatabaseWrite => M5CapabilityClass::DatabaseWrite,
            Self::RemoteMutation => M5CapabilityClass::RemoteMutation,
            Self::BrowserRoutedAction => M5CapabilityClass::BrowserNavigation,
            Self::SecretProjection => M5CapabilityClass::SecretHandleProjection,
        }
    }
}

/// Current verification state of an approval ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TicketVerificationState {
    /// The ticket verified, including offline for allowed local actions.
    ValidLocalFirst,
    /// The ticket's expiry has elapsed.
    DeniedExpired,
    /// A replayed ticket was detected (nonce consumed or window exceeded).
    DeniedReplayDetected,
    /// The governing policy epoch was superseded.
    DeniedEpochSuperseded,
    /// A bound dimension (capability hash, sandbox, target, or actor) mismatched.
    DeniedBindingMismatch,
    /// The ticket was explicitly revoked.
    DeniedRevoked,
}

impl M5TicketVerificationState {
    /// Stable token recorded in the ticket.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidLocalFirst => "valid_local_first",
            Self::DeniedExpired => "denied_expired",
            Self::DeniedReplayDetected => "denied_replay_detected",
            Self::DeniedEpochSuperseded => "denied_epoch_superseded",
            Self::DeniedBindingMismatch => "denied_binding_mismatch",
            Self::DeniedRevoked => "denied_revoked",
        }
    }

    /// Whether the ticket currently authorizes its action.
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::ValidLocalFirst)
    }

    /// Whether the ticket is denied and therefore must carry a deny reason.
    pub const fn is_denied(self) -> bool {
        !self.is_valid()
    }
}

/// The binding dimension that failed for a denied ticket.
///
/// A deny-reason packet names exactly which dimension failed instead of
/// collapsing into a generic permission error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TicketDenyDimension {
    /// The ticket's expiry timestamp has elapsed.
    ExpiryElapsed,
    /// The one-time replay nonce was already consumed.
    ReplayNonceConsumed,
    /// The replay window was exceeded.
    ReplayWindowExceeded,
    /// The governing policy epoch was superseded.
    PolicyEpochSuperseded,
    /// The bound capability-envelope hash did not match.
    CapabilityHashMismatch,
    /// The bound sandbox profile did not match.
    SandboxProfileMismatch,
    /// The bound target identity did not match.
    TargetIdentityMismatch,
    /// The bound actor did not match.
    ActorBindingMismatch,
    /// The ticket was revoked.
    TicketRevoked,
}

impl M5TicketDenyDimension {
    /// Stable token recorded in the deny reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpiryElapsed => "expiry_elapsed",
            Self::ReplayNonceConsumed => "replay_nonce_consumed",
            Self::ReplayWindowExceeded => "replay_window_exceeded",
            Self::PolicyEpochSuperseded => "policy_epoch_superseded",
            Self::CapabilityHashMismatch => "capability_hash_mismatch",
            Self::SandboxProfileMismatch => "sandbox_profile_mismatch",
            Self::TargetIdentityMismatch => "target_identity_mismatch",
            Self::ActorBindingMismatch => "actor_binding_mismatch",
            Self::TicketRevoked => "ticket_revoked",
        }
    }

    /// The verification state a ticket denied for this dimension must carry.
    pub const fn denies_as(self) -> M5TicketVerificationState {
        match self {
            Self::ExpiryElapsed => M5TicketVerificationState::DeniedExpired,
            Self::ReplayNonceConsumed | Self::ReplayWindowExceeded => {
                M5TicketVerificationState::DeniedReplayDetected
            }
            Self::PolicyEpochSuperseded => M5TicketVerificationState::DeniedEpochSuperseded,
            Self::CapabilityHashMismatch
            | Self::SandboxProfileMismatch
            | Self::TargetIdentityMismatch
            | Self::ActorBindingMismatch => M5TicketVerificationState::DeniedBindingMismatch,
            Self::TicketRevoked => M5TicketVerificationState::DeniedRevoked,
        }
    }
}

/// Method by which a ticket's authority is verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalFirstVerificationMethod {
    /// Verified against a locally held signature chain; works fully offline.
    LocalSignatureChain,
    /// Verified against a cached signed policy bundle; works fully offline.
    CachedPolicyBundle,
    /// Verified through a remote broker attestation; required only off-device.
    RemoteBrokerAttestation,
}

impl M5LocalFirstVerificationMethod {
    /// Stable token recorded in the verification descriptor.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSignatureChain => "local_signature_chain",
            Self::CachedPolicyBundle => "cached_policy_bundle",
            Self::RemoteBrokerAttestation => "remote_broker_attestation",
        }
    }

    /// Whether this method can verify a ticket without live control-plane reach.
    pub const fn is_offline_capable(self) -> bool {
        matches!(self, Self::LocalSignatureChain | Self::CachedPolicyBundle)
    }
}

/// Actor an approval ticket is minted for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TicketActor {
    /// Actor class.
    pub actor_class: M5EnvelopeActorClass,
    /// Export-safe actor reference (opaque id or label; never a credential).
    pub actor_ref: String,
    /// Export-safe principal this actor acts on behalf of, when delegated.
    pub on_behalf_of: Option<String>,
}

/// Target an approval ticket binds its action to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TicketTarget {
    /// Export-safe target identity (host label, path, or resource id).
    pub target_identity: String,
    /// True when the action runs off-device or is brokered by another runtime.
    pub off_device: bool,
    /// True when the target identity has been verified.
    pub identity_verified: bool,
}

/// Sandbox and capability binding an approval ticket is pinned to.
///
/// The `capability_envelope_hash` binds the ticket to one concrete capability
/// envelope, so authority cannot be widened by replaying the ticket against a
/// different envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TicketBinding {
    /// Governing policy-epoch binding.
    pub policy_epoch: M5PolicyEpochBinding,
    /// Sandbox profile the action runs under.
    pub sandbox_profile: M5SandboxProfile,
    /// Export-safe digest of the bound capability envelope (never raw material).
    pub capability_envelope_hash: String,
    /// Export-safe reference to the bound capability envelope.
    pub capability_envelope_ref: String,
    /// Capability classes the ticket grants (a subset of the matrix row).
    pub bound_capability_classes: Vec<M5CapabilityClass>,
    /// Secret scope for the bound action.
    pub secret_scope: M5SecretScope,
}

/// Expiry binding for an approval ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TicketValidity {
    /// RFC 3339 issuance timestamp.
    pub issued_at: String,
    /// RFC 3339 expiry timestamp.
    pub expires_at: String,
    /// Ticket time-to-live in seconds; must be non-zero.
    pub ttl_seconds: u32,
    /// True when this ticket authorizes a single action and cannot be replayed.
    pub single_use: bool,
}

/// Replay-protection binding for an approval ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReplayProtection {
    /// Export-safe one-time nonce (opaque; never raw secret material).
    pub nonce: String,
    /// Monotonic issuance counter the enforcer checks against.
    pub monotonic_counter: u64,
    /// Replay acceptance window in seconds; must be non-zero.
    pub replay_window_seconds: u32,
    /// True when the nonce has already been consumed (a replay was attempted).
    pub nonce_consumed: bool,
}

/// Issuance-lineage binding for an approval ticket.
///
/// Every issuer class is external to the executing surface: authority is minted
/// by a policy authority, an approval broker, a standing policy epoch, or a
/// remote broker — never by the executor itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TicketIssuanceLineage {
    /// Issuer class that minted this ticket's authority.
    pub issuer_class: M5EnvelopeIssuerClass,
    /// Export-safe issuer reference.
    pub issuer_ref: String,
    /// Approval-ticket posture this ticket was issued under.
    pub approval_posture: M5ApprovalTicketPosture,
    /// Ordered export-safe lineage refs from policy epoch to issued ticket.
    pub decision_chain: Vec<String>,
    /// Always false: the executor never self-issues this ticket.
    pub self_issued_by_executor: bool,
}

/// Local-first verification descriptor for an approval ticket.
///
/// Encodes the invariant that an allowed local action verifies offline without
/// depending on live control-plane reachability, skipping audit lineage, or
/// widening authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalFirstVerification {
    /// Verification method used.
    pub method: M5LocalFirstVerificationMethod,
    /// True when this ticket can be verified offline.
    pub verifiable_offline: bool,
    /// True when verification depends on live control-plane reachability.
    pub requires_live_control_plane: bool,
    /// Always true: verification never drops audit lineage.
    pub audit_lineage_preserved: bool,
    /// Always false: offline verification never widens authority.
    pub authority_widened_offline: bool,
}

/// Deny-reason packet for a denied or expired ticket.
///
/// Names exactly which binding dimension failed and how to recover, instead of
/// collapsing into a generic permission error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TicketDenyReason {
    /// The binding dimension that failed.
    pub dimension: M5TicketDenyDimension,
    /// What the action narrows to when the ticket cannot be honored.
    pub narrowed_to: M5DegradedFallback,
    /// Export-safe human explanation; never empty.
    pub explanation: String,
    /// Export-safe concrete recovery action; never empty.
    pub recovery_action: String,
}

/// One issued approval ticket bound to a single mutating or privileged M5 action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovalTicket {
    /// Stable ticket id.
    pub ticket_id: String,
    /// Matrix surface this ticket is issued against.
    pub surface: M5ExecutingSurface,
    /// The mutating or privileged action class this ticket authorizes.
    pub action_class: M5TicketActionClass,
    /// Actor the ticket is minted for.
    pub actor: M5TicketActor,
    /// Target the ticket binds to.
    pub target: M5TicketTarget,
    /// Sandbox and capability binding.
    pub binding: M5TicketBinding,
    /// Expiry binding.
    pub validity: M5TicketValidity,
    /// Replay-protection binding.
    pub replay_protection: M5ReplayProtection,
    /// Issuance-lineage binding.
    pub issuance_lineage: M5TicketIssuanceLineage,
    /// Local-first verification descriptor.
    pub local_first_verification: M5LocalFirstVerification,
    /// Current verification state.
    pub verification_state: M5TicketVerificationState,
    /// Deny reason; present exactly when `verification_state` is denied.
    pub deny_reason: Option<M5TicketDenyReason>,
    /// Downgrade triggers applied to this ticket; empty when valid.
    pub applied_downgrade_triggers: Vec<M5RuntimeAuthorityDowngradeTrigger>,
    /// Per-ticket redaction class token.
    pub redaction_class_token: String,
}

/// Trust and isolation review block for the approval-ticket ledger packet.
///
/// Every field encodes a hard invariant; all must hold for the packet to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovalTicketTrustReview {
    /// No ticket grants ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No helper actor self-issues authority.
    pub no_self_issued_authority_by_helpers: bool,
    /// Every ticket binds actor, action, target, epoch, hash, expiry, and replay.
    pub every_ticket_binds_all_dimensions: bool,
    /// Deny reasons name the failed binding dimension, never a generic error.
    pub deny_reasons_name_failed_dimension: bool,
    /// Deny reasons carry a concrete recovery action.
    pub deny_reasons_carry_recovery: bool,
    /// Replay protection is present on every ticket.
    pub replay_protection_present_on_every_ticket: bool,
    /// Allowed local actions verify offline without live control-plane reach.
    pub local_first_verifies_without_control_plane: bool,
    /// Offline verification never widens authority.
    pub local_first_never_widens_authority: bool,
    /// Audit lineage is preserved even during offline verification.
    pub audit_lineage_preserved_offline: bool,
    /// Secret references are handle-only; no raw secret material is bound.
    pub secret_refs_handle_only_no_raw_material: bool,
    /// Enforcement fails closed or narrows when it cannot be honored.
    pub fail_closed_when_enforcement_unavailable: bool,
    /// No raw secret material is exported inside tickets or support packets.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block for the approval-ticket ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovalTicketConsumerProjection {
    /// Desktop shows the full ticket and any deny reason.
    pub desktop_shows_ticket_and_deny_reason: bool,
    /// Command palette and policy inspector reference the same tickets.
    pub command_and_policy_reference_same_tickets: bool,
    /// CLI / headless verifies tickets offline.
    pub cli_headless_verifies_offline: bool,
    /// Support export shows the full ticket ledger.
    pub support_export_shows_ticket_ledger: bool,
    /// Diagnostics shows the full ticket ledger.
    pub diagnostics_shows_ticket_ledger: bool,
    /// Help / About shows a ticket-ledger summary.
    pub help_about_shows_ticket_summary: bool,
    /// Release evidence consumes the ledger instead of cloning per-surface prose.
    pub release_evidence_consumes_ledger: bool,
    /// Remote and browser-routed surfaces preserve ticket semantics off-device.
    pub remote_and_browser_preserve_ticket_semantics: bool,
}

/// Proof-freshness block for the approval-ticket ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovalTicketProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected tickets.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ApprovalTicketLedgerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ApprovalTicketLedgerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Issued approval tickets.
    pub tickets: Vec<M5ApprovalTicket>,
    /// Trust review block.
    pub trust_review: M5ApprovalTicketTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ApprovalTicketConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ApprovalTicketProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 approval-ticket ledger packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovalTicketLedgerPacket {
    /// Record kind; must equal [`M5_APPROVAL_TICKET_LEDGER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_APPROVAL_TICKET_LEDGER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Issued approval tickets.
    pub tickets: Vec<M5ApprovalTicket>,
    /// Trust review block.
    pub trust_review: M5ApprovalTicketTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ApprovalTicketConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ApprovalTicketProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ApprovalTicketLedgerPacket {
    /// Builds an M5 approval-ticket ledger packet from frozen input.
    pub fn new(input: M5ApprovalTicketLedgerPacketInput) -> Self {
        Self {
            record_kind: M5_APPROVAL_TICKET_LEDGER_RECORD_KIND.to_owned(),
            schema_version: M5_APPROVAL_TICKET_LEDGER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            tickets: input.tickets,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 approval-ticket ledger packet invariants.
    pub fn validate(&self) -> Vec<M5ApprovalTicketLedgerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_APPROVAL_TICKET_LEDGER_RECORD_KIND {
            violations.push(M5ApprovalTicketLedgerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_APPROVAL_TICKET_LEDGER_SCHEMA_VERSION {
            violations.push(M5ApprovalTicketLedgerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ApprovalTicketLedgerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_tickets(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 approval-ticket ledger packet serializes"),
        ) {
            violations.push(M5ApprovalTicketLedgerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 approval-ticket ledger packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let denied = self
            .tickets
            .iter()
            .filter(|ticket| ticket.verification_state.is_denied())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Approval-Ticket Issuance And Local-First Verification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Tickets: {} ({} denied or expired)\n",
            self.tickets.len(),
            denied
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Tickets\n\n");
        for ticket in &self.tickets {
            out.push_str(&format!(
                "- **{}** ({}) — {} on {} — state: {}\n",
                ticket.action_class.as_str(),
                ticket.ticket_id,
                ticket.surface.as_str(),
                ticket.binding.sandbox_profile.as_str(),
                ticket.verification_state.as_str()
            ));
            out.push_str(&format!(
                "  - Actor: {} (`{}`) · Posture: {} · Issuer: {}\n",
                ticket.actor.actor_class.as_str(),
                ticket.actor.actor_ref,
                ticket.issuance_lineage.approval_posture.as_str(),
                ticket.issuance_lineage.issuer_class.as_str()
            ));
            out.push_str(&format!(
                "  - Target: {} ({}) · Epoch: {} · Hash: {}\n",
                ticket.target.target_identity,
                if ticket.target.off_device {
                    "off-device"
                } else {
                    "on-device"
                },
                ticket.binding.policy_epoch.epoch_id,
                ticket.binding.capability_envelope_hash
            ));
            out.push_str(&format!(
                "  - Expiry: {} (ttl {}s, single-use {}) · Replay nonce: {} (window {}s)\n",
                ticket.validity.expires_at,
                ticket.validity.ttl_seconds,
                ticket.validity.single_use,
                ticket.replay_protection.nonce,
                ticket.replay_protection.replay_window_seconds
            ));
            out.push_str(&format!(
                "  - Local-first: {} (offline {}, control-plane {})\n",
                ticket.local_first_verification.method.as_str(),
                ticket.local_first_verification.verifiable_offline,
                ticket.local_first_verification.requires_live_control_plane
            ));
            if let Some(deny) = &ticket.deny_reason {
                out.push_str(&format!(
                    "  - Denied: {} → narrows to {} — {} · Recover: {}\n",
                    deny.dimension.as_str(),
                    deny.narrowed_to.as_str(),
                    deny.explanation,
                    deny.recovery_action
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 approval-ticket ledger export.
#[derive(Debug)]
pub enum M5ApprovalTicketLedgerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ApprovalTicketLedgerViolation>),
}

impl fmt::Display for M5ApprovalTicketLedgerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 approval-ticket ledger export parse failed: {error}"
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
                    "m5 approval-ticket ledger export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ApprovalTicketLedgerArtifactError {}

/// Validation failures emitted by [`M5ApprovalTicketLedgerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ApprovalTicketLedgerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required mutating action class has no issued ticket.
    RequiredActionClassMissing,
    /// A ticket is missing required identity fields.
    TicketIncomplete,
    /// A ticket does not bind the capability class its action class requires.
    ActionClassCapabilityUnbound,
    /// A ticket grants a capability class outside its matrix surface row.
    CapabilityWidensBeyondMatrix,
    /// A ticket binds a sandbox profile that widens its matrix default.
    SandboxProfileWidens,
    /// A ticket omits its sandbox or capability-envelope binding hash.
    BindingHashMissing,
    /// A ticket omits a non-zero expiry.
    ValidityIncomplete,
    /// A ticket omits replay protection (nonce or window).
    ReplayProtectionMissing,
    /// A helper actor self-issues authority instead of carrying external lineage.
    SelfIssuedAuthorityForbidden,
    /// A ticket's posture is not externally issued.
    TicketPostureNotExternallyIssued,
    /// A privileged ticket omits its issuance lineage.
    PrivilegedActionWithoutLineage,
    /// The secret scope and bound capabilities are inconsistent.
    SecretScopeInconsistent,
    /// A valid ticket binds an off-device target that is unverified.
    OffDeviceTargetUnverified,
    /// A denied ticket omits its deny reason.
    DenyReasonMissing,
    /// A valid ticket carries a deny reason or downgrade triggers.
    DenyReasonOnValidTicket,
    /// A deny reason's dimension is inconsistent with the verification state.
    DenyDimensionStateMismatch,
    /// A deny reason omits its explanation.
    DenyExplanationMissing,
    /// A deny reason omits its recovery action.
    DenyRecoveryMissing,
    /// An allowed local action requires live control-plane reachability.
    LocalFirstRequiresControlPlane,
    /// Offline verification widens authority.
    LocalFirstWidensAuthority,
    /// Verification drops audit lineage.
    AuditLineageDropped,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ApprovalTicketLedgerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredActionClassMissing => "required_action_class_missing",
            Self::TicketIncomplete => "ticket_incomplete",
            Self::ActionClassCapabilityUnbound => "action_class_capability_unbound",
            Self::CapabilityWidensBeyondMatrix => "capability_widens_beyond_matrix",
            Self::SandboxProfileWidens => "sandbox_profile_widens",
            Self::BindingHashMissing => "binding_hash_missing",
            Self::ValidityIncomplete => "validity_incomplete",
            Self::ReplayProtectionMissing => "replay_protection_missing",
            Self::SelfIssuedAuthorityForbidden => "self_issued_authority_forbidden",
            Self::TicketPostureNotExternallyIssued => "ticket_posture_not_externally_issued",
            Self::PrivilegedActionWithoutLineage => "privileged_action_without_lineage",
            Self::SecretScopeInconsistent => "secret_scope_inconsistent",
            Self::OffDeviceTargetUnverified => "off_device_target_unverified",
            Self::DenyReasonMissing => "deny_reason_missing",
            Self::DenyReasonOnValidTicket => "deny_reason_on_valid_ticket",
            Self::DenyDimensionStateMismatch => "deny_dimension_state_mismatch",
            Self::DenyExplanationMissing => "deny_explanation_missing",
            Self::DenyRecoveryMissing => "deny_recovery_missing",
            Self::LocalFirstRequiresControlPlane => "local_first_requires_control_plane",
            Self::LocalFirstWidensAuthority => "local_first_widens_authority",
            Self::AuditLineageDropped => "audit_lineage_dropped",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen stable M5 approval-ticket ledger packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_APPROVAL_TICKET_LEDGER_ARTIFACT_REF`]; the dumper emits this packet
/// and a test asserts the checked-in artifact deserializes back to it unchanged.
pub fn frozen_stable_m5_approval_ticket_ledger_packet() -> M5ApprovalTicketLedgerPacket {
    let mut tickets = valid_tickets();
    tickets.extend(denied_tickets());
    build_ledger_packet(
        M5_APPROVAL_TICKET_LEDGER_PACKET_ID,
        "M5 Approval-Ticket Issuance And Local-First Verification",
        tickets,
    )
}

/// Builds a ledger packet from a set of tickets, applying the frozen review,
/// projection, freshness, and source-contract blocks.
///
/// Shared by the canonical packet and the checked narrowed fixtures so they
/// cannot drift in their trust posture.
pub fn build_ledger_packet(
    packet_id: &str,
    packet_label: &str,
    tickets: Vec<M5ApprovalTicket>,
) -> M5ApprovalTicketLedgerPacket {
    M5ApprovalTicketLedgerPacket::new(M5ApprovalTicketLedgerPacketInput {
        packet_id: packet_id.to_owned(),
        packet_label: packet_label.to_owned(),
        tickets,
        trust_review: M5ApprovalTicketTrustReview {
            no_ambient_machine_privilege: true,
            no_self_issued_authority_by_helpers: true,
            every_ticket_binds_all_dimensions: true,
            deny_reasons_name_failed_dimension: true,
            deny_reasons_carry_recovery: true,
            replay_protection_present_on_every_ticket: true,
            local_first_verifies_without_control_plane: true,
            local_first_never_widens_authority: true,
            audit_lineage_preserved_offline: true,
            secret_refs_handle_only_no_raw_material: true,
            fail_closed_when_enforcement_unavailable: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5ApprovalTicketConsumerProjection {
            desktop_shows_ticket_and_deny_reason: true,
            command_and_policy_reference_same_tickets: true,
            cli_headless_verifies_offline: true,
            support_export_shows_ticket_ledger: true,
            diagnostics_shows_ticket_ledger: true,
            help_about_shows_ticket_summary: true,
            release_evidence_consumes_ledger: true,
            remote_and_browser_preserve_ticket_semantics: true,
        },
        proof_freshness: M5ApprovalTicketProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
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

/// The set of issued, currently-valid local-first tickets, one per action class.
pub fn valid_tickets() -> Vec<M5ApprovalTicket> {
    vec![
        scaffold_workspace_mutation_ticket(),
        notebook_process_execution_ticket(),
        request_network_send_ticket(),
        database_write_ticket(),
        ai_secret_projection_ticket(),
        remote_mutation_ticket(),
        browser_routed_action_ticket(),
    ]
}

/// The set of denied tickets demonstrating each deny dimension and recovery.
pub fn denied_tickets() -> Vec<M5ApprovalTicket> {
    vec![
        request_send_expired_denied_ticket(),
        database_write_replay_denied_ticket(),
        ai_epoch_superseded_denied_ticket(),
        remote_mutation_binding_mismatch_denied_ticket(),
    ]
}

fn local_first(
    method: M5LocalFirstVerificationMethod,
    off_device: bool,
) -> M5LocalFirstVerification {
    M5LocalFirstVerification {
        method,
        verifiable_offline: method.is_offline_capable(),
        requires_live_control_plane: off_device && !method.is_offline_capable(),
        audit_lineage_preserved: true,
        authority_widened_offline: false,
    }
}

fn lineage(
    issuer_class: M5EnvelopeIssuerClass,
    issuer_ref: &str,
    posture: M5ApprovalTicketPosture,
    ticket_id: &str,
) -> M5TicketIssuanceLineage {
    M5TicketIssuanceLineage {
        issuer_class,
        issuer_ref: issuer_ref.to_owned(),
        approval_posture: posture,
        decision_chain: vec![
            "policy-epoch:m5:0007".to_owned(),
            issuer_ref.to_owned(),
            ticket_id.to_owned(),
        ],
        self_issued_by_executor: false,
    }
}

fn policy_epoch(superseded: bool) -> M5PolicyEpochBinding {
    M5PolicyEpochBinding {
        epoch_id: "policy-epoch:m5:0007".to_owned(),
        epoch_sequence: 7,
        superseded,
    }
}

fn scaffold_workspace_mutation_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:scaffold-hook:0001".to_owned(),
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
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            capability_envelope_hash: "sha256:1a2b3c4d5e6f7081".to_owned(),
            capability_envelope_ref: "envelope:scaffold-hook:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:10:00Z".to_owned(),
            ttl_seconds: 600,
            single_use: true,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:scaffold-hook:0001:7f3a".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 600,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::ApprovalBroker,
            "issuer:approval-broker:local",
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:scaffold-hook:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::LocalSignatureChain,
            false,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn notebook_process_execution_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:notebook-kernel:0001".to_owned(),
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
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            capability_envelope_hash: "sha256:2b3c4d5e6f708192".to_owned(),
            capability_envelope_ref: "envelope:notebook-kernel:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
                M5CapabilityClass::NetworkEgress,
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T02:00:00Z".to_owned(),
            ttl_seconds: 7200,
            single_use: false,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:notebook-kernel:0001:8c41".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 7200,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::ApprovalBroker,
            "issuer:approval-broker:local",
            M5ApprovalTicketPosture::TicketRequiredPerSession,
            "ticket:notebook-kernel:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::CachedPolicyBundle,
            false,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn request_network_send_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:request-api-send:0001".to_owned(),
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
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
            capability_envelope_hash: "sha256:3c4d5e6f70819203".to_owned(),
            capability_envelope_ref: "envelope:request-api-send:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            secret_scope: M5SecretScope::HandleOnlyDelegated,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T01:00:00Z".to_owned(),
            ttl_seconds: 3600,
            single_use: false,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:request-api-send:0001:9d52".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 3600,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::ApprovalBroker,
            "issuer:approval-broker:local",
            M5ApprovalTicketPosture::TicketRequiredPerScope,
            "ticket:request-api-send:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::LocalSignatureChain,
            false,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn database_write_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:database-action:0001".to_owned(),
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
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
            capability_envelope_hash: "sha256:4d5e6f7081920314".to_owned(),
            capability_envelope_ref: "envelope:database-action:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::DatabaseRead,
                M5CapabilityClass::DatabaseWrite,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            secret_scope: M5SecretScope::HandleOnlyDelegated,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:15:00Z".to_owned(),
            ttl_seconds: 900,
            single_use: true,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:database-action:0001:ae63".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 900,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::ApprovalBroker,
            "issuer:approval-broker:local",
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:database-action:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::CachedPolicyBundle,
            false,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn ai_secret_projection_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:ai-tool:0001".to_owned(),
        surface: M5ExecutingSurface::AiTool,
        action_class: M5TicketActionClass::SecretProjection,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::AiTool,
            actor_ref: "actor:ai-tool-runtime".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "workspace://project/src".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            capability_envelope_hash: "sha256:5e6f708192031425".to_owned(),
            capability_envelope_ref: "envelope:ai-tool:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            secret_scope: M5SecretScope::HandleOnlyDelegated,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            single_use: true,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:ai-tool:0001:bf74".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 300,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::ApprovalBroker,
            "issuer:approval-broker:local",
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:ai-tool:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::LocalSignatureChain,
            false,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn remote_mutation_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:remote-mutation:0001".to_owned(),
        surface: M5ExecutingSurface::RemoteMutation,
        action_class: M5TicketActionClass::RemoteMutation,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::RemoteHelper,
            actor_ref: "actor:remote-helper".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "remote://managed-runtime/deployment".to_owned(),
            off_device: true,
            identity_verified: true,
        },
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
            capability_envelope_hash: "sha256:6f70819203142536".to_owned(),
            capability_envelope_ref: "envelope:remote-mutation:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::RemoteMutation,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            secret_scope: M5SecretScope::ScopedBrokeredSecret,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            single_use: true,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:remote-mutation:0001:c085".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 120,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::RemoteBrokerRuntime,
            "issuer:remote-broker:managed",
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:remote-mutation:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::RemoteBrokerAttestation,
            true,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn browser_routed_action_ticket() -> M5ApprovalTicket {
    M5ApprovalTicket {
        ticket_id: "ticket:browser-routed-action:0001".to_owned(),
        surface: M5ExecutingSurface::BrowserRoutedAction,
        action_class: M5TicketActionClass::BrowserRoutedAction,
        actor: M5TicketActor {
            actor_class: M5EnvelopeActorClass::BrowserRoute,
            actor_ref: "actor:browser-route".to_owned(),
            on_behalf_of: Some("operator:project-owner".to_owned()),
        },
        target: M5TicketTarget {
            target_identity: "https://app.example.test".to_owned(),
            off_device: true,
            identity_verified: true,
        },
        binding: M5TicketBinding {
            policy_epoch: policy_epoch(false),
            sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
            capability_envelope_hash: "sha256:70819203142536f7".to_owned(),
            capability_envelope_ref: "envelope:browser-routed-action:0001".to_owned(),
            bound_capability_classes: vec![
                M5CapabilityClass::BrowserNavigation,
                M5CapabilityClass::NetworkEgress,
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
        },
        validity: M5TicketValidity {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            single_use: true,
        },
        replay_protection: M5ReplayProtection {
            nonce: "nonce:browser-routed-action:0001:d196".to_owned(),
            monotonic_counter: 1,
            replay_window_seconds: 120,
            nonce_consumed: false,
        },
        issuance_lineage: lineage(
            M5EnvelopeIssuerClass::RemoteBrokerRuntime,
            "issuer:remote-broker:managed",
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:browser-routed-action:0001",
        ),
        local_first_verification: local_first(
            M5LocalFirstVerificationMethod::RemoteBrokerAttestation,
            true,
        ),
        verification_state: M5TicketVerificationState::ValidLocalFirst,
        deny_reason: None,
        applied_downgrade_triggers: vec![],
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn request_send_expired_denied_ticket() -> M5ApprovalTicket {
    let mut ticket = request_network_send_ticket();
    ticket.ticket_id = "ticket:request-api-send:0002".to_owned();
    ticket.replay_protection.nonce = "nonce:request-api-send:0002:e207".to_owned();
    ticket.validity.expires_at = "2026-06-09T23:00:00Z".to_owned();
    ticket.issuance_lineage.decision_chain = vec![
        "policy-epoch:m5:0007".to_owned(),
        "issuer:approval-broker:local".to_owned(),
        "ticket:request-api-send:0002".to_owned(),
    ];
    ticket.local_first_verification =
        local_first(M5LocalFirstVerificationMethod::LocalSignatureChain, false);
    ticket.verification_state = M5TicketVerificationState::DeniedExpired;
    ticket.deny_reason = Some(M5TicketDenyReason {
        dimension: M5TicketDenyDimension::ExpiryElapsed,
        narrowed_to: M5DegradedFallback::RequireFreshTicket,
        explanation: "The approval ticket expired at 2026-06-09T23:00:00Z, before this send."
            .to_owned(),
        recovery_action: "Request a fresh per-scope approval ticket for this endpoint and resend."
            .to_owned(),
    });
    ticket.applied_downgrade_triggers =
        vec![M5RuntimeAuthorityDowngradeTrigger::ApprovalTicketExpired];
    ticket
}

fn database_write_replay_denied_ticket() -> M5ApprovalTicket {
    let mut ticket = database_write_ticket();
    ticket.ticket_id = "ticket:database-action:0002".to_owned();
    ticket.replay_protection.nonce = "nonce:database-action:0002:f318".to_owned();
    ticket.replay_protection.nonce_consumed = true;
    ticket.replay_protection.monotonic_counter = 2;
    ticket.issuance_lineage.decision_chain = vec![
        "policy-epoch:m5:0007".to_owned(),
        "issuer:approval-broker:local".to_owned(),
        "ticket:database-action:0002".to_owned(),
    ];
    ticket.verification_state = M5TicketVerificationState::DeniedReplayDetected;
    ticket.deny_reason = Some(M5TicketDenyReason {
        dimension: M5TicketDenyDimension::ReplayNonceConsumed,
        narrowed_to: M5DegradedFallback::NarrowToReadOnly,
        explanation: "The single-use replay nonce was already consumed; this is a replay attempt."
            .to_owned(),
        recovery_action: "Re-run the write as a fresh per-action request to mint a new nonce."
            .to_owned(),
    });
    ticket.applied_downgrade_triggers =
        vec![M5RuntimeAuthorityDowngradeTrigger::ApprovalTicketReplayed];
    ticket
}

fn ai_epoch_superseded_denied_ticket() -> M5ApprovalTicket {
    let mut ticket = ai_secret_projection_ticket();
    ticket.ticket_id = "ticket:ai-tool:0002".to_owned();
    ticket.replay_protection.nonce = "nonce:ai-tool:0002:0429".to_owned();
    ticket.binding.policy_epoch = policy_epoch(true);
    ticket.issuance_lineage.decision_chain = vec![
        "policy-epoch:m5:0007".to_owned(),
        "issuer:approval-broker:local".to_owned(),
        "ticket:ai-tool:0002".to_owned(),
    ];
    ticket.verification_state = M5TicketVerificationState::DeniedEpochSuperseded;
    ticket.deny_reason = Some(M5TicketDenyReason {
        dimension: M5TicketDenyDimension::PolicyEpochSuperseded,
        narrowed_to: M5DegradedFallback::NarrowToSanitizedPreview,
        explanation:
            "Policy epoch policy-epoch:m5:0007 was superseded after this ticket was minted."
                .to_owned(),
        recovery_action:
            "Re-issue the ticket under the current policy epoch before acting on workspace files."
                .to_owned(),
    });
    ticket.applied_downgrade_triggers =
        vec![M5RuntimeAuthorityDowngradeTrigger::PolicyEpochSuperseded];
    ticket
}

fn remote_mutation_binding_mismatch_denied_ticket() -> M5ApprovalTicket {
    let mut ticket = remote_mutation_ticket();
    ticket.ticket_id = "ticket:remote-mutation:0002".to_owned();
    ticket.replay_protection.nonce = "nonce:remote-mutation:0002:153a".to_owned();
    ticket.issuance_lineage.decision_chain = vec![
        "policy-epoch:m5:0007".to_owned(),
        "issuer:remote-broker:managed".to_owned(),
        "ticket:remote-mutation:0002".to_owned(),
    ];
    ticket.verification_state = M5TicketVerificationState::DeniedBindingMismatch;
    ticket.deny_reason = Some(M5TicketDenyReason {
        dimension: M5TicketDenyDimension::CapabilityHashMismatch,
        narrowed_to: M5DegradedFallback::FailClosedBlock,
        explanation:
            "The presented capability-envelope hash did not match the envelope bound at issuance."
                .to_owned(),
        recovery_action:
            "Re-issue the remote-mutation ticket against the current capability envelope and retry."
                .to_owned(),
    });
    ticket.applied_downgrade_triggers =
        vec![M5RuntimeAuthorityDowngradeTrigger::ScopeExpansionUnqualified];
    ticket
}

fn validate_source_contracts(
    packet: &M5ApprovalTicketLedgerPacket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
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
            violations.push(M5ApprovalTicketLedgerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_tickets(
    packet: &M5ApprovalTicketLedgerPacket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();
    let allowed_by_surface: BTreeMap<M5ExecutingSurface, BTreeSet<M5CapabilityClass>> = matrix
        .surface_rows
        .iter()
        .map(|row| {
            (
                row.surface,
                row.allowed_capability_classes.iter().copied().collect(),
            )
        })
        .collect();
    let default_profile_by_surface: BTreeMap<M5ExecutingSurface, M5SandboxProfile> = matrix
        .surface_rows
        .iter()
        .map(|row| (row.surface, row.default_sandbox_profile))
        .collect();

    let action_classes_present: BTreeSet<M5TicketActionClass> =
        packet.tickets.iter().map(|t| t.action_class).collect();
    for required in M5TicketActionClass::ALL {
        if !action_classes_present.contains(&required) {
            violations.push(M5ApprovalTicketLedgerViolation::RequiredActionClassMissing);
            return;
        }
    }

    for ticket in &packet.tickets {
        validate_ticket_identity(ticket, violations);
        validate_ticket_binding(
            ticket,
            &allowed_by_surface,
            &default_profile_by_surface,
            violations,
        );
        validate_ticket_issuance(ticket, violations);
        validate_ticket_local_first(ticket, violations);
        validate_ticket_verification(ticket, violations);
    }
}

fn validate_ticket_identity(
    ticket: &M5ApprovalTicket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    if ticket.ticket_id.trim().is_empty()
        || ticket.redaction_class_token.trim().is_empty()
        || ticket.target.target_identity.trim().is_empty()
        || ticket.actor.actor_ref.trim().is_empty()
    {
        violations.push(M5ApprovalTicketLedgerViolation::TicketIncomplete);
    }
}

fn validate_ticket_binding(
    ticket: &M5ApprovalTicket,
    allowed_by_surface: &BTreeMap<M5ExecutingSurface, BTreeSet<M5CapabilityClass>>,
    default_profile_by_surface: &BTreeMap<M5ExecutingSurface, M5SandboxProfile>,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let binding = &ticket.binding;

    if !binding
        .bound_capability_classes
        .contains(&ticket.action_class.required_capability())
    {
        violations.push(M5ApprovalTicketLedgerViolation::ActionClassCapabilityUnbound);
    }

    if let Some(allowed) = allowed_by_surface.get(&ticket.surface) {
        if binding
            .bound_capability_classes
            .iter()
            .any(|cap| !allowed.contains(cap))
        {
            violations.push(M5ApprovalTicketLedgerViolation::CapabilityWidensBeyondMatrix);
        }
    }

    if let Some(default_profile) = default_profile_by_surface.get(&ticket.surface) {
        if binding.sandbox_profile != *default_profile
            && binding.sandbox_profile != M5SandboxProfile::InertNoExecution
        {
            violations.push(M5ApprovalTicketLedgerViolation::SandboxProfileWidens);
        }
    }

    if binding.capability_envelope_hash.trim().is_empty()
        || binding.capability_envelope_ref.trim().is_empty()
        || binding.policy_epoch.epoch_id.trim().is_empty()
        || binding.bound_capability_classes.is_empty()
    {
        violations.push(M5ApprovalTicketLedgerViolation::BindingHashMissing);
    }

    if ticket.validity.ttl_seconds == 0
        || ticket.validity.issued_at.trim().is_empty()
        || ticket.validity.expires_at.trim().is_empty()
    {
        violations.push(M5ApprovalTicketLedgerViolation::ValidityIncomplete);
    }

    if ticket.replay_protection.nonce.trim().is_empty()
        || ticket.replay_protection.replay_window_seconds == 0
    {
        violations.push(M5ApprovalTicketLedgerViolation::ReplayProtectionMissing);
    }

    let projects_secret = binding
        .bound_capability_classes
        .iter()
        .any(|cap| cap.requires_secret_scope());
    let secret_consistent = if projects_secret {
        binding.secret_scope.grants_secret_access()
    } else {
        !binding.secret_scope.grants_secret_access()
    };
    if !secret_consistent {
        violations.push(M5ApprovalTicketLedgerViolation::SecretScopeInconsistent);
    }

    // A valid off-device ticket must carry a verified target identity; a denied
    // ticket may bind an unverified target precisely because it failed.
    if ticket.verification_state.is_valid()
        && ticket.target.off_device
        && !ticket.target.identity_verified
    {
        violations.push(M5ApprovalTicketLedgerViolation::OffDeviceTargetUnverified);
    }
}

fn validate_ticket_issuance(
    ticket: &M5ApprovalTicket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let lineage = &ticket.issuance_lineage;

    // No executor self-issues authority; helper actors (AI, recipe, extension,
    // browser route, remote helper) are the case this most guards against.
    if lineage.self_issued_by_executor {
        violations.push(M5ApprovalTicketLedgerViolation::SelfIssuedAuthorityForbidden);
    }

    if !lineage.approval_posture.is_externally_issued() {
        violations.push(M5ApprovalTicketLedgerViolation::TicketPostureNotExternallyIssued);
    }

    if lineage.issuer_ref.trim().is_empty()
        || lineage.decision_chain.is_empty()
        || lineage
            .decision_chain
            .iter()
            .any(|ref_| ref_.trim().is_empty())
    {
        violations.push(M5ApprovalTicketLedgerViolation::PrivilegedActionWithoutLineage);
    }
}

fn validate_ticket_local_first(
    ticket: &M5ApprovalTicket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let verification = &ticket.local_first_verification;

    if verification.authority_widened_offline {
        violations.push(M5ApprovalTicketLedgerViolation::LocalFirstWidensAuthority);
    }
    if !verification.audit_lineage_preserved {
        violations.push(M5ApprovalTicketLedgerViolation::AuditLineageDropped);
    }

    // An allowed local (on-device) action must verify offline without depending
    // on live control-plane reachability.
    if ticket.verification_state.is_valid()
        && !ticket.target.off_device
        && (verification.requires_live_control_plane || !verification.verifiable_offline)
    {
        violations.push(M5ApprovalTicketLedgerViolation::LocalFirstRequiresControlPlane);
    }
}

fn validate_ticket_verification(
    ticket: &M5ApprovalTicket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    match (&ticket.verification_state, &ticket.deny_reason) {
        (state, None) if state.is_denied() => {
            violations.push(M5ApprovalTicketLedgerViolation::DenyReasonMissing);
        }
        (state, Some(_)) if state.is_valid() => {
            violations.push(M5ApprovalTicketLedgerViolation::DenyReasonOnValidTicket);
        }
        (state, Some(deny)) => {
            if deny.dimension.denies_as() != *state {
                violations.push(M5ApprovalTicketLedgerViolation::DenyDimensionStateMismatch);
            }
            if deny.explanation.trim().is_empty() {
                violations.push(M5ApprovalTicketLedgerViolation::DenyExplanationMissing);
            }
            if deny.recovery_action.trim().is_empty() {
                violations.push(M5ApprovalTicketLedgerViolation::DenyRecoveryMissing);
            }
        }
        _ => {}
    }

    if ticket.verification_state.is_valid() && !ticket.applied_downgrade_triggers.is_empty() {
        violations.push(M5ApprovalTicketLedgerViolation::DenyReasonOnValidTicket);
    }
}

fn validate_trust_review(
    packet: &M5ApprovalTicketLedgerPacket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_self_issued_authority_by_helpers,
        review.every_ticket_binds_all_dimensions,
        review.deny_reasons_name_failed_dimension,
        review.deny_reasons_carry_recovery,
        review.replay_protection_present_on_every_ticket,
        review.local_first_verifies_without_control_plane,
        review.local_first_never_widens_authority,
        review.audit_lineage_preserved_offline,
        review.secret_refs_handle_only_no_raw_material,
        review.fail_closed_when_enforcement_unavailable,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5ApprovalTicketLedgerViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ApprovalTicketLedgerPacket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_ticket_and_deny_reason,
        projection.command_and_policy_reference_same_tickets,
        projection.cli_headless_verifies_offline,
        projection.support_export_shows_ticket_ledger,
        projection.diagnostics_shows_ticket_ledger,
        projection.help_about_shows_ticket_summary,
        projection.release_evidence_consumes_ledger,
        projection.remote_and_browser_preserve_ticket_semantics,
    ] {
        if !ok {
            violations.push(M5ApprovalTicketLedgerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ApprovalTicketLedgerPacket,
    violations: &mut Vec<M5ApprovalTicketLedgerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ApprovalTicketLedgerViolation::ProofFreshnessIncomplete);
    }
}

/// Reads and validates the checked-in stable M5 approval-ticket ledger export.
pub fn current_stable_m5_approval_ticket_ledger_export(
) -> Result<M5ApprovalTicketLedgerPacket, M5ApprovalTicketLedgerArtifactError> {
    let packet: M5ApprovalTicketLedgerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/support_export.json"
    )))
    .map_err(M5ApprovalTicketLedgerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ApprovalTicketLedgerArtifactError::Validation(violations))
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
