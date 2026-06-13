//! Capability-envelope packets issued against the frozen M5 runtime-authority
//! matrix for every claimed M5 executing surface.
//!
//! The frozen runtime-authority matrix states, per claimed M5 executing surface,
//! what *may* be granted: a default sandbox profile, approval-ticket posture,
//! allowed capability classes, secret scope, degraded fallback, and
//! unsupported-profile behavior. That matrix is the policy. This module ships the
//! concrete runtime artifact the matrix authorizes — the **capability envelope**:
//! the export-safe record bound to one issued execution that names its actor,
//! its target identity, the exact allowed roots, sinks, or endpoints it may
//! touch, the handle-only secret references it may project, the governing policy
//! epoch, its expiry, and its full audit lineage back to the issuer and approval
//! ticket.
//!
//! Each [`M5CapabilityEnvelope`] is issued *against* a single matrix surface row
//! and may only narrow it: its granted capability classes are a subset of the
//! matrix row's allowed classes, its sandbox profile is the matrix default or a
//! fully inert fail-closed profile, and any applied downgrade marks the envelope
//! as narrowed rather than silently widening. Remote and browser-routed
//! envelopes carry the identical shape even when execution is off-device or
//! brokered by another runtime.
//!
//! The track invariant is no ambient privilege: no AI tool, extension, recipe,
//! browser route, or remote helper self-issues authority — every envelope minted
//! for an untrusted-helper actor carries an externally issued lineage and is
//! flagged `self_issued_by_executor: false`. Actor and target identity, allowed
//! scope, secret references, policy epoch, expiry, and audit lineage stay
//! inspectable and export-safe; raw secret material, credential bodies, and live
//! ticket signatures stay outside the support boundary; and if enforcement cannot
//! be honored the envelope narrows or fails closed instead of widening.
//!
//! The boundary schema is
//! [`schemas/execution-auth/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.schema.json`](../../../../schemas/execution-auth/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.md`](../../../../docs/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/`](../../../../fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/).

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

/// Stable record-kind tag carried by [`M5CapabilityEnvelopePacket`].
pub const M5_CAPABILITY_ENVELOPE_RECORD_KIND: &str = "ship_m5_capability_envelope_packets";

/// Schema version for the M5 capability-envelope packet records.
pub const M5_CAPABILITY_ENVELOPE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_CAPABILITY_ENVELOPE_SCHEMA_REF: &str =
    "schemas/execution-auth/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CAPABILITY_ENVELOPE_DOC_REF: &str =
    "docs/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CAPABILITY_ENVELOPE_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CAPABILITY_ENVELOPE_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CAPABILITY_ENVELOPE_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e";

/// Stable packet id minted by [`frozen_stable_m5_capability_envelope_packet`].
pub const M5_CAPABILITY_ENVELOPE_PACKET_ID: &str = "m5-capability-envelope-packets:stable:0001";

/// Class of actor for whom a capability envelope is minted.
///
/// Untrusted-helper actors ([`Self::is_untrusted_helper`]) — AI tools, recipes,
/// extensions, browser routes, and remote helpers — must never self-issue
/// authority; an envelope minted for one carries an externally issued lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvelopeActorClass {
    /// A human operator driving the action directly.
    HumanOperator,
    /// Trusted in-product system automation under the host policy epoch.
    SystemAutomation,
    /// An AI tool invocation.
    AiTool,
    /// A saved automation recipe.
    Recipe,
    /// A loaded extension.
    Extension,
    /// A browser-routed action.
    BrowserRoute,
    /// A remote execution helper.
    RemoteHelper,
}

impl M5EnvelopeActorClass {
    /// Stable token recorded in the envelope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanOperator => "human_operator",
            Self::SystemAutomation => "system_automation",
            Self::AiTool => "ai_tool",
            Self::Recipe => "recipe",
            Self::Extension => "extension",
            Self::BrowserRoute => "browser_route",
            Self::RemoteHelper => "remote_helper",
        }
    }

    /// Whether this actor is an untrusted helper that must never self-issue
    /// authority and must carry an externally issued envelope lineage.
    pub const fn is_untrusted_helper(self) -> bool {
        matches!(
            self,
            Self::AiTool | Self::Recipe | Self::Extension | Self::BrowserRoute | Self::RemoteHelper
        )
    }
}

/// Actor binding for a capability envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvelopeActor {
    /// Actor class.
    pub actor_class: M5EnvelopeActorClass,
    /// Export-safe actor reference (opaque id or label; never a credential).
    pub actor_ref: String,
    /// Export-safe principal this actor acts on behalf of, when delegated.
    pub on_behalf_of: Option<String>,
}

/// Class of execution target an envelope binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvelopeTargetClass {
    /// The local workspace tree.
    LocalWorkspace,
    /// A local process or kernel.
    LocalProcess,
    /// A network endpoint.
    NetworkEndpoint,
    /// A database.
    Database,
    /// A remote host or managed runtime.
    RemoteHost,
    /// A browser context.
    BrowserContext,
}

impl M5EnvelopeTargetClass {
    /// Stable token recorded in the envelope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::LocalProcess => "local_process",
            Self::NetworkEndpoint => "network_endpoint",
            Self::Database => "database",
            Self::RemoteHost => "remote_host",
            Self::BrowserContext => "browser_context",
        }
    }
}

/// Target binding for a capability envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvelopeTarget {
    /// Target class.
    pub target_class: M5EnvelopeTargetClass,
    /// Export-safe target identity (host label, path, or resource id).
    pub target_identity: String,
    /// True when execution runs off-device or is brokered by another runtime.
    pub off_device: bool,
    /// True when the target identity has been verified.
    pub identity_verified: bool,
}

/// Kind of allowed-scope entry granted inside an envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AllowedScopeKind {
    /// A filesystem root the envelope may read or write within.
    FilesystemRoot,
    /// A data sink the envelope may write to.
    DataSink,
    /// A network endpoint the envelope may reach.
    NetworkEndpoint,
}

impl M5AllowedScopeKind {
    /// Stable token recorded in the envelope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilesystemRoot => "filesystem_root",
            Self::DataSink => "data_sink",
            Self::NetworkEndpoint => "network_endpoint",
        }
    }
}

/// Access mode an envelope grants over one allowed-scope entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScopeAccessMode {
    /// Read-only access.
    ReadOnly,
    /// Append-only access.
    AppendOnly,
    /// Read and write access.
    ReadWrite,
    /// Send-only egress.
    SendOnly,
    /// Navigation-only (browser-routed) access.
    Navigate,
}

impl M5ScopeAccessMode {
    /// Stable token recorded in the envelope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::AppendOnly => "append_only",
            Self::ReadWrite => "read_write",
            Self::SendOnly => "send_only",
            Self::Navigate => "navigate",
        }
    }
}

/// One allowed root, sink, or endpoint inside a capability envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AllowedScopeEntry {
    /// Scope kind.
    pub kind: M5AllowedScopeKind,
    /// Export-safe scope label (path, sink id, or host).
    pub label: String,
    /// Access mode granted over this scope entry.
    pub access: M5ScopeAccessMode,
}

/// A handle-only secret reference projected into an envelope.
///
/// Carries a broker handle id and projection scope only; never raw secret
/// material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SecretHandleRef {
    /// Export-safe broker handle reference (never raw secret material).
    pub handle_ref: String,
    /// Secret-scope posture for this reference.
    pub scope: M5SecretScope,
    /// Repo-relative secret-broker contract ref this reference is minted under.
    pub broker_contract_ref: String,
}

/// Governing policy-epoch binding for an envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PolicyEpochBinding {
    /// Export-safe policy-epoch id.
    pub epoch_id: String,
    /// Monotonic policy-epoch sequence.
    pub epoch_sequence: u64,
    /// True when this epoch has been superseded and the envelope must re-issue.
    pub superseded: bool,
}

/// Expiry binding for an envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvelopeExpiry {
    /// RFC 3339 issuance timestamp.
    pub issued_at: String,
    /// RFC 3339 expiry timestamp.
    pub expires_at: String,
    /// Envelope time-to-live in seconds; must be non-zero.
    pub ttl_seconds: u32,
    /// True when this envelope authorizes a single action and cannot be replayed.
    pub single_use: bool,
}

/// Issuer class that minted an envelope's authority.
///
/// Every class is *external* to the executing surface: authority is minted by a
/// policy authority, an approval broker, a standing policy epoch, or a remote
/// broker runtime — never by the executor itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvelopeIssuerClass {
    /// The local policy authority.
    PolicyAuthority,
    /// The approval-ticket broker.
    ApprovalBroker,
    /// A standing policy-epoch grant.
    StandingPolicyEpoch,
    /// A remote broker runtime that issues off-device authority.
    RemoteBrokerRuntime,
}

impl M5EnvelopeIssuerClass {
    /// Stable token recorded in the envelope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyAuthority => "policy_authority",
            Self::ApprovalBroker => "approval_broker",
            Self::StandingPolicyEpoch => "standing_policy_epoch",
            Self::RemoteBrokerRuntime => "remote_broker_runtime",
        }
    }
}

/// Audit-lineage binding for an envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvelopeAuditLineage {
    /// Issuer class that minted this envelope's authority.
    pub issuer_class: M5EnvelopeIssuerClass,
    /// Export-safe issuer reference.
    pub issuer_ref: String,
    /// Export-safe approval-ticket reference (never a live ticket signature).
    pub approval_ticket_ref: String,
    /// Approval-ticket posture this envelope was issued under.
    pub approval_posture: M5ApprovalTicketPosture,
    /// Ordered export-safe lineage refs from policy epoch to issued envelope.
    pub decision_chain: Vec<String>,
    /// Always false: the executor never self-issues this envelope.
    pub self_issued_by_executor: bool,
}

/// One issued capability envelope bound to a single M5 execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityEnvelope {
    /// Stable envelope id.
    pub envelope_id: String,
    /// Matrix surface this envelope is issued against.
    pub surface: M5ExecutingSurface,
    /// Actor the envelope is minted for.
    pub actor: M5EnvelopeActor,
    /// Target the envelope binds to.
    pub target: M5EnvelopeTarget,
    /// Allowed roots, sinks, and endpoints this envelope may touch.
    pub allowed_scope: Vec<M5AllowedScopeEntry>,
    /// Capability classes granted by this envelope (a subset of the matrix row).
    pub granted_capability_classes: Vec<M5CapabilityClass>,
    /// Handle-only secret references projected into this envelope.
    pub secret_handle_refs: Vec<M5SecretHandleRef>,
    /// Secret scope for this envelope.
    pub secret_scope: M5SecretScope,
    /// Sandbox profile this envelope ran under.
    pub sandbox_profile: M5SandboxProfile,
    /// Governing policy-epoch binding.
    pub policy_epoch: M5PolicyEpochBinding,
    /// Expiry binding.
    pub expiry: M5EnvelopeExpiry,
    /// Audit-lineage binding.
    pub audit_lineage: M5EnvelopeAuditLineage,
    /// Degraded fallback applied when full authority cannot be honored.
    pub degraded_fallback: M5DegradedFallback,
    /// Downgrade triggers applied to this concrete envelope; empty when nominal.
    pub applied_downgrade_triggers: Vec<M5RuntimeAuthorityDowngradeTrigger>,
    /// True when this envelope was narrowed below its matrix default.
    pub narrowed_from_default: bool,
    /// Per-envelope redaction class token.
    pub redaction_class_token: String,
}

/// Trust and isolation review block for the envelope packet.
///
/// Every field encodes a hard invariant; all must hold for the packet to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityEnvelopeTrustReview {
    /// No envelope grants ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No untrusted-helper actor self-issues authority.
    pub no_self_issued_authority_by_helpers: bool,
    /// Actor and target identity are inspectable and export-safe.
    pub actor_and_target_identity_inspectable: bool,
    /// Allowed roots, sinks, and endpoints are inspectable.
    pub allowed_scope_inspectable: bool,
    /// Secret references are handle-only; no raw secret material is projected.
    pub secret_refs_handle_only_no_raw_material: bool,
    /// Policy epoch and expiry are inspectable on every envelope.
    pub policy_epoch_and_expiry_inspectable: bool,
    /// Audit lineage is inspectable on every envelope.
    pub audit_lineage_inspectable: bool,
    /// Off-device envelopes preserve the same semantics as local ones.
    pub off_device_preserves_envelope_semantics: bool,
    /// Enforcement fails closed when it cannot be honored, never widening.
    pub fail_closed_when_enforcement_unavailable: bool,
    /// No raw secret material is exported inside envelopes or support packets.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block for the envelope packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityEnvelopeConsumerProjection {
    /// Desktop shell shows the full envelope (actor, target, scope, secrets, epoch, expiry, lineage).
    pub desktop_shows_envelope: bool,
    /// Command palette and policy inspector reference the same envelopes.
    pub command_and_policy_reference_same_envelopes: bool,
    /// CLI / headless shows the full envelope.
    pub cli_headless_shows_envelope: bool,
    /// Support export shows the full envelope.
    pub support_export_shows_envelope: bool,
    /// Diagnostics shows the full envelope.
    pub diagnostics_shows_envelope: bool,
    /// Help / About shows an envelope summary.
    pub help_about_shows_envelope_summary: bool,
    /// Release evidence consumes these envelopes instead of cloning per-surface prose.
    pub release_evidence_consumes_envelopes: bool,
    /// Remote and browser-routed surfaces preserve envelope semantics off-device.
    pub remote_and_browser_preserve_envelope_semantics: bool,
}

/// Proof-freshness block for the envelope packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityEnvelopeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected envelopes.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5CapabilityEnvelopePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CapabilityEnvelopePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Issued envelopes.
    pub envelopes: Vec<M5CapabilityEnvelope>,
    /// Trust review block.
    pub trust_review: M5CapabilityEnvelopeTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5CapabilityEnvelopeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CapabilityEnvelopeProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 capability-envelope packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityEnvelopePacket {
    /// Record kind; must equal [`M5_CAPABILITY_ENVELOPE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CAPABILITY_ENVELOPE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Issued envelopes.
    pub envelopes: Vec<M5CapabilityEnvelope>,
    /// Trust review block.
    pub trust_review: M5CapabilityEnvelopeTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5CapabilityEnvelopeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CapabilityEnvelopeProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CapabilityEnvelopePacket {
    /// Builds an M5 capability-envelope packet from frozen input.
    pub fn new(input: M5CapabilityEnvelopePacketInput) -> Self {
        Self {
            record_kind: M5_CAPABILITY_ENVELOPE_RECORD_KIND.to_owned(),
            schema_version: M5_CAPABILITY_ENVELOPE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            envelopes: input.envelopes,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 capability-envelope packet invariants.
    pub fn validate(&self) -> Vec<M5CapabilityEnvelopeViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CAPABILITY_ENVELOPE_RECORD_KIND {
            violations.push(M5CapabilityEnvelopeViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CAPABILITY_ENVELOPE_SCHEMA_VERSION {
            violations.push(M5CapabilityEnvelopeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CapabilityEnvelopeViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_envelopes(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 capability-envelope packet serializes"),
        ) {
            violations.push(M5CapabilityEnvelopeViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 capability-envelope packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .envelopes
            .iter()
            .filter(|envelope| envelope.narrowed_from_default)
            .count();
        let mut out = String::new();
        out.push_str("# M5 Capability-Envelope Packets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Envelopes: {} ({} narrowed from default)\n",
            self.envelopes.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Issued envelopes\n\n");
        for envelope in &self.envelopes {
            out.push_str(&format!(
                "- **{}** ({})\n",
                envelope.surface.as_str(),
                envelope.envelope_id
            ));
            out.push_str(&format!(
                "  - Actor: {} (`{}`) · Target: {} (`{}`{})\n",
                envelope.actor.actor_class.as_str(),
                envelope.actor.actor_ref,
                envelope.target.target_class.as_str(),
                envelope.target.target_identity,
                if envelope.target.off_device {
                    ", off-device"
                } else {
                    ""
                }
            ));
            let scope = envelope
                .allowed_scope
                .iter()
                .map(|entry| {
                    format!(
                        "{}:{}={}",
                        entry.kind.as_str(),
                        entry.label,
                        entry.access.as_str()
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("  - Allowed scope: {scope}\n"));
            let caps = envelope
                .granted_capability_classes
                .iter()
                .map(|cap| cap.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - Capabilities: {caps} · Secret scope: {}\n",
                envelope.secret_scope.as_str()
            ));
            out.push_str(&format!(
                "  - Policy epoch: {} (seq {}) · Expires: {} (ttl {}s)\n",
                envelope.policy_epoch.epoch_id,
                envelope.policy_epoch.epoch_sequence,
                envelope.expiry.expires_at,
                envelope.expiry.ttl_seconds
            ));
            out.push_str(&format!(
                "  - Issuer: {} (`{}`) · Ticket: {} · Posture: {}\n",
                envelope.audit_lineage.issuer_class.as_str(),
                envelope.audit_lineage.issuer_ref,
                envelope.audit_lineage.approval_ticket_ref,
                envelope.audit_lineage.approval_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 capability-envelope export.
#[derive(Debug)]
pub enum M5CapabilityEnvelopeArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CapabilityEnvelopeViolation>),
}

impl fmt::Display for M5CapabilityEnvelopeArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 capability-envelope export parse failed: {error}"
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
                    "m5 capability-envelope export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CapabilityEnvelopeArtifactError {}

/// Validation failures emitted by [`M5CapabilityEnvelopePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CapabilityEnvelopeViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required executing surface has no issued envelope.
    RequiredSurfaceMissing,
    /// An envelope is missing required identity fields.
    EnvelopeIncomplete,
    /// An envelope grants no capability classes.
    CapabilityEnvelopeEmpty,
    /// An envelope grants a capability class outside its matrix surface row.
    CapabilityWidensBeyondMatrix,
    /// An envelope runs under a sandbox profile that widens its matrix default.
    SandboxProfileWidens,
    /// An envelope has no allowed roots, sinks, or endpoints.
    AllowedScopeMissing,
    /// An untrusted-helper actor self-issues authority instead of carrying external lineage.
    SelfIssuedAuthorityForbidden,
    /// An elevated capability is granted without an externally issued ticket reference.
    ElevatedCapabilityWithoutTicket,
    /// The envelope expiry is missing or zero.
    ExpiryMissing,
    /// The governing policy epoch is missing.
    PolicyEpochMissing,
    /// The audit lineage is incomplete.
    AuditLineageIncomplete,
    /// Secret references are inconsistent with the declared secret scope.
    SecretScopeInconsistent,
    /// An off-device envelope binds to an unverified target.
    OffDeviceTargetUnverified,
    /// A narrowed flag is inconsistent with the applied downgrade triggers.
    NarrowingInconsistent,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CapabilityEnvelopeViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::EnvelopeIncomplete => "envelope_incomplete",
            Self::CapabilityEnvelopeEmpty => "capability_envelope_empty",
            Self::CapabilityWidensBeyondMatrix => "capability_widens_beyond_matrix",
            Self::SandboxProfileWidens => "sandbox_profile_widens",
            Self::AllowedScopeMissing => "allowed_scope_missing",
            Self::SelfIssuedAuthorityForbidden => "self_issued_authority_forbidden",
            Self::ElevatedCapabilityWithoutTicket => "elevated_capability_without_ticket",
            Self::ExpiryMissing => "expiry_missing",
            Self::PolicyEpochMissing => "policy_epoch_missing",
            Self::AuditLineageIncomplete => "audit_lineage_incomplete",
            Self::SecretScopeInconsistent => "secret_scope_inconsistent",
            Self::OffDeviceTargetUnverified => "off_device_target_unverified",
            Self::NarrowingInconsistent => "narrowing_inconsistent",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen stable M5 capability-envelope packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_CAPABILITY_ENVELOPE_ARTIFACT_REF`]; the envelope dumper emits this
/// packet and a test asserts the checked-in artifact deserializes back to it
/// unchanged.
pub fn frozen_stable_m5_capability_envelope_packet() -> M5CapabilityEnvelopePacket {
    let envelopes = vec![
        request_api_send_envelope(),
        database_action_envelope(),
        notebook_kernel_envelope(),
        scaffold_hook_envelope(),
        preview_server_envelope(),
        ai_tool_envelope(),
        recipe_envelope(),
        browser_routed_action_envelope(),
        incident_flow_envelope(),
        remote_mutation_envelope(),
    ];

    M5CapabilityEnvelopePacket::new(M5CapabilityEnvelopePacketInput {
        packet_id: M5_CAPABILITY_ENVELOPE_PACKET_ID.to_owned(),
        packet_label: "M5 Capability-Envelope Packets".to_owned(),
        envelopes,
        trust_review: M5CapabilityEnvelopeTrustReview {
            no_ambient_machine_privilege: true,
            no_self_issued_authority_by_helpers: true,
            actor_and_target_identity_inspectable: true,
            allowed_scope_inspectable: true,
            secret_refs_handle_only_no_raw_material: true,
            policy_epoch_and_expiry_inspectable: true,
            audit_lineage_inspectable: true,
            off_device_preserves_envelope_semantics: true,
            fail_closed_when_enforcement_unavailable: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5CapabilityEnvelopeConsumerProjection {
            desktop_shows_envelope: true,
            command_and_policy_reference_same_envelopes: true,
            cli_headless_shows_envelope: true,
            support_export_shows_envelope: true,
            diagnostics_shows_envelope: true,
            help_about_shows_envelope_summary: true,
            release_evidence_consumes_envelopes: true,
            remote_and_browser_preserve_envelope_semantics: true,
        },
        proof_freshness: M5CapabilityEnvelopeProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
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

/// Reads and validates the checked-in stable M5 capability-envelope export.
pub fn current_stable_m5_capability_envelope_export(
) -> Result<M5CapabilityEnvelopePacket, M5CapabilityEnvelopeArtifactError> {
    let packet: M5CapabilityEnvelopePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/support_export.json"
    )))
    .map_err(M5CapabilityEnvelopeArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CapabilityEnvelopeArtifactError::Validation(violations))
    }
}

fn secret_handle_contract() -> String {
    M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF.to_owned()
}

fn nominal_epoch() -> M5PolicyEpochBinding {
    M5PolicyEpochBinding {
        epoch_id: "policy-epoch:m5:0007".to_owned(),
        epoch_sequence: 7,
        superseded: false,
    }
}

fn request_api_send_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:request-api-send:0001".to_owned(),
        surface: M5ExecutingSurface::RequestApiSend,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local".to_owned(),
            on_behalf_of: None,
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::NetworkEndpoint,
            target_identity: "https://api.example.test/v1/orders".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![M5AllowedScopeEntry {
            kind: M5AllowedScopeKind::NetworkEndpoint,
            label: "https://api.example.test/v1".to_owned(),
            access: M5ScopeAccessMode::SendOnly,
        }],
        granted_capability_classes: vec![
            M5CapabilityClass::NetworkEgress,
            M5CapabilityClass::SecretHandleProjection,
        ],
        secret_handle_refs: vec![M5SecretHandleRef {
            handle_ref: "broker-handle:request-api:0001".to_owned(),
            scope: M5SecretScope::HandleOnlyDelegated,
            broker_contract_ref: secret_handle_contract(),
        }],
        secret_scope: M5SecretScope::HandleOnlyDelegated,
        sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T01:00:00Z".to_owned(),
            ttl_seconds: 3600,
            single_use: false,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:request-api-send:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerScope,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:request-api-send:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::RequireFreshTicket,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn database_action_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:database-action:0001".to_owned(),
        surface: M5ExecutingSurface::DatabaseAction,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local".to_owned(),
            on_behalf_of: None,
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::Database,
            target_identity: "db://primary/orders".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::DataSink,
                label: "db://primary/orders".to_owned(),
                access: M5ScopeAccessMode::ReadWrite,
            },
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::NetworkEndpoint,
                label: "tcp://db.example.test:5432".to_owned(),
                access: M5ScopeAccessMode::SendOnly,
            },
        ],
        granted_capability_classes: vec![
            M5CapabilityClass::DatabaseRead,
            M5CapabilityClass::DatabaseWrite,
            M5CapabilityClass::NetworkEgress,
            M5CapabilityClass::SecretHandleProjection,
        ],
        secret_handle_refs: vec![M5SecretHandleRef {
            handle_ref: "broker-handle:database-action:0001".to_owned(),
            scope: M5SecretScope::HandleOnlyDelegated,
            broker_contract_ref: secret_handle_contract(),
        }],
        secret_scope: M5SecretScope::HandleOnlyDelegated,
        sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:15:00Z".to_owned(),
            ttl_seconds: 900,
            single_use: true,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:database-action:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerAction,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:database-action:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn notebook_kernel_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:notebook-kernel:0001".to_owned(),
        surface: M5ExecutingSurface::NotebookKernel,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local".to_owned(),
            on_behalf_of: None,
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::LocalProcess,
            target_identity: "kernel://project/notebook-7".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::FilesystemRoot,
                label: "workspace://project".to_owned(),
                access: M5ScopeAccessMode::ReadWrite,
            },
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::NetworkEndpoint,
                label: "broker://transport-plane".to_owned(),
                access: M5ScopeAccessMode::SendOnly,
            },
        ],
        granted_capability_classes: vec![
            M5CapabilityClass::ReadWorkspace,
            M5CapabilityClass::WriteWorkspace,
            M5CapabilityClass::ProcessSpawn,
            M5CapabilityClass::NetworkEgress,
        ],
        secret_handle_refs: vec![],
        secret_scope: M5SecretScope::NoSecretAccess,
        sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T02:00:00Z".to_owned(),
            ttl_seconds: 7200,
            single_use: false,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:notebook-kernel:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerSession,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:notebook-kernel:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn scaffold_hook_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:scaffold-hook:0001".to_owned(),
        surface: M5ExecutingSurface::ScaffoldHook,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::SystemAutomation,
            actor_ref: "actor:scaffold-runner:local".to_owned(),
            on_behalf_of: Some("actor:operator:local".to_owned()),
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::LocalWorkspace,
            target_identity: "workspace://project/templates".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![M5AllowedScopeEntry {
            kind: M5AllowedScopeKind::FilesystemRoot,
            label: "workspace://project/templates".to_owned(),
            access: M5ScopeAccessMode::ReadWrite,
        }],
        granted_capability_classes: vec![
            M5CapabilityClass::ReadWorkspace,
            M5CapabilityClass::WriteWorkspace,
            M5CapabilityClass::ProcessSpawn,
        ],
        secret_handle_refs: vec![],
        secret_scope: M5SecretScope::NoSecretAccess,
        sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:10:00Z".to_owned(),
            ttl_seconds: 600,
            single_use: true,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:scaffold-hook:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerAction,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:scaffold-hook:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToSanitizedPreview,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn preview_server_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:preview-server:0001".to_owned(),
        surface: M5ExecutingSurface::PreviewServer,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::SystemAutomation,
            actor_ref: "actor:preview-runner:local".to_owned(),
            on_behalf_of: Some("actor:operator:local".to_owned()),
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::LocalProcess,
            target_identity: "preview://project/public".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::FilesystemRoot,
                label: "workspace://project/public".to_owned(),
                access: M5ScopeAccessMode::ReadOnly,
            },
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::NetworkEndpoint,
                label: "loopback://127.0.0.1:0".to_owned(),
                access: M5ScopeAccessMode::SendOnly,
            },
        ],
        granted_capability_classes: vec![
            M5CapabilityClass::ReadWorkspace,
            M5CapabilityClass::ProcessSpawn,
            M5CapabilityClass::NetworkEgress,
        ],
        secret_handle_refs: vec![],
        secret_scope: M5SecretScope::NoSecretAccess,
        sandbox_profile: M5SandboxProfile::ContainerIsolatedLocal,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T02:00:00Z".to_owned(),
            ttl_seconds: 7200,
            single_use: false,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:preview-server:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerSession,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:preview-server:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn ai_tool_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:ai-tool:0001".to_owned(),
        surface: M5ExecutingSurface::AiTool,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::AiTool,
            actor_ref: "actor:ai-tool:composer".to_owned(),
            on_behalf_of: Some("actor:operator:local".to_owned()),
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::LocalWorkspace,
            target_identity: "workspace://project".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::FilesystemRoot,
                label: "workspace://project/src".to_owned(),
                access: M5ScopeAccessMode::ReadWrite,
            },
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::NetworkEndpoint,
                label: "broker://transport-plane".to_owned(),
                access: M5ScopeAccessMode::SendOnly,
            },
        ],
        granted_capability_classes: vec![
            M5CapabilityClass::ReadWorkspace,
            M5CapabilityClass::WriteWorkspace,
            M5CapabilityClass::NetworkEgress,
            M5CapabilityClass::SecretHandleProjection,
        ],
        secret_handle_refs: vec![M5SecretHandleRef {
            handle_ref: "broker-handle:ai-tool:0001".to_owned(),
            scope: M5SecretScope::HandleOnlyDelegated,
            broker_contract_ref: secret_handle_contract(),
        }],
        secret_scope: M5SecretScope::HandleOnlyDelegated,
        sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            single_use: true,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:ai-tool:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerAction,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:ai-tool:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToSanitizedPreview,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn recipe_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:recipe:0001".to_owned(),
        surface: M5ExecutingSurface::Recipe,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::Recipe,
            actor_ref: "actor:recipe:nightly-format".to_owned(),
            on_behalf_of: Some("actor:operator:local".to_owned()),
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::LocalWorkspace,
            target_identity: "workspace://project".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![M5AllowedScopeEntry {
            kind: M5AllowedScopeKind::FilesystemRoot,
            label: "workspace://project/src".to_owned(),
            access: M5ScopeAccessMode::ReadWrite,
        }],
        granted_capability_classes: vec![
            M5CapabilityClass::ReadWorkspace,
            M5CapabilityClass::WriteWorkspace,
            M5CapabilityClass::ProcessSpawn,
        ],
        secret_handle_refs: vec![],
        secret_scope: M5SecretScope::NoSecretAccess,
        sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:30:00Z".to_owned(),
            ttl_seconds: 1800,
            single_use: false,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:recipe:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerScope,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:recipe:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToSanitizedPreview,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn browser_routed_action_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:browser-routed-action:0001".to_owned(),
        surface: M5ExecutingSurface::BrowserRoutedAction,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::BrowserRoute,
            actor_ref: "actor:browser-route:agent".to_owned(),
            on_behalf_of: Some("actor:operator:local".to_owned()),
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::BrowserContext,
            target_identity: "https://app.example.test".to_owned(),
            off_device: true,
            identity_verified: true,
        },
        allowed_scope: vec![M5AllowedScopeEntry {
            kind: M5AllowedScopeKind::NetworkEndpoint,
            label: "https://app.example.test".to_owned(),
            access: M5ScopeAccessMode::Navigate,
        }],
        granted_capability_classes: vec![
            M5CapabilityClass::BrowserNavigation,
            M5CapabilityClass::NetworkEgress,
        ],
        secret_handle_refs: vec![],
        secret_scope: M5SecretScope::NoSecretAccess,
        sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            single_use: true,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::RemoteBrokerRuntime,
            issuer_ref: "issuer:remote-broker:managed".to_owned(),
            approval_ticket_ref: "ticket:browser-routed-action:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerAction,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:remote-broker:managed".to_owned(),
                "ticket:browser-routed-action:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn incident_flow_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:incident-flow:0001".to_owned(),
        surface: M5ExecutingSurface::IncidentFlow,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:on-call".to_owned(),
            on_behalf_of: None,
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::NetworkEndpoint,
            target_identity: "https://incident.example.test/runbook".to_owned(),
            off_device: false,
            identity_verified: true,
        },
        allowed_scope: vec![
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::FilesystemRoot,
                label: "workspace://project/runbooks".to_owned(),
                access: M5ScopeAccessMode::ReadOnly,
            },
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::NetworkEndpoint,
                label: "broker://transport-plane".to_owned(),
                access: M5ScopeAccessMode::SendOnly,
            },
        ],
        granted_capability_classes: vec![
            M5CapabilityClass::ReadWorkspace,
            M5CapabilityClass::NetworkEgress,
            M5CapabilityClass::SecretHandleProjection,
        ],
        secret_handle_refs: vec![M5SecretHandleRef {
            handle_ref: "broker-handle:incident-flow:0001".to_owned(),
            scope: M5SecretScope::HandleOnlyDelegated,
            broker_contract_ref: secret_handle_contract(),
        }],
        secret_scope: M5SecretScope::HandleOnlyDelegated,
        sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T01:00:00Z".to_owned(),
            ttl_seconds: 3600,
            single_use: false,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local".to_owned(),
            approval_ticket_ref: "ticket:incident-flow:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerScope,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:approval-broker:local".to_owned(),
                "ticket:incident-flow:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn remote_mutation_envelope() -> M5CapabilityEnvelope {
    M5CapabilityEnvelope {
        envelope_id: "envelope:remote-mutation:0001".to_owned(),
        surface: M5ExecutingSurface::RemoteMutation,
        actor: M5EnvelopeActor {
            actor_class: M5EnvelopeActorClass::RemoteHelper,
            actor_ref: "actor:remote-helper:managed".to_owned(),
            on_behalf_of: Some("actor:operator:local".to_owned()),
        },
        target: M5EnvelopeTarget {
            target_class: M5EnvelopeTargetClass::RemoteHost,
            target_identity: "remote://managed-runtime/deployment".to_owned(),
            off_device: true,
            identity_verified: true,
        },
        allowed_scope: vec![
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::DataSink,
                label: "remote://managed-runtime/deployment".to_owned(),
                access: M5ScopeAccessMode::ReadWrite,
            },
            M5AllowedScopeEntry {
                kind: M5AllowedScopeKind::NetworkEndpoint,
                label: "https://managed-runtime.example.test/mutation".to_owned(),
                access: M5ScopeAccessMode::SendOnly,
            },
        ],
        granted_capability_classes: vec![
            M5CapabilityClass::RemoteMutation,
            M5CapabilityClass::NetworkEgress,
            M5CapabilityClass::SecretHandleProjection,
        ],
        secret_handle_refs: vec![M5SecretHandleRef {
            handle_ref: "broker-handle:remote-mutation:0001".to_owned(),
            scope: M5SecretScope::ScopedBrokeredSecret,
            broker_contract_ref: secret_handle_contract(),
        }],
        secret_scope: M5SecretScope::ScopedBrokeredSecret,
        sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
        policy_epoch: nominal_epoch(),
        expiry: M5EnvelopeExpiry {
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:05:00Z".to_owned(),
            ttl_seconds: 300,
            single_use: true,
        },
        audit_lineage: M5EnvelopeAuditLineage {
            issuer_class: M5EnvelopeIssuerClass::RemoteBrokerRuntime,
            issuer_ref: "issuer:remote-broker:managed".to_owned(),
            approval_ticket_ref: "ticket:remote-mutation:0001".to_owned(),
            approval_posture: M5ApprovalTicketPosture::TicketRequiredPerAction,
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                "issuer:remote-broker:managed".to_owned(),
                "ticket:remote-mutation:0001".to_owned(),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: M5DegradedFallback::FailClosedBlock,
        applied_downgrade_triggers: vec![],
        narrowed_from_default: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn validate_source_contracts(
    packet: &M5CapabilityEnvelopePacket,
    violations: &mut Vec<M5CapabilityEnvelopeViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
        M5_CAPABILITY_ENVELOPE_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CapabilityEnvelopeViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_envelopes(
    packet: &M5CapabilityEnvelopePacket,
    violations: &mut Vec<M5CapabilityEnvelopeViolation>,
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

    let present: BTreeSet<M5ExecutingSurface> =
        packet.envelopes.iter().map(|env| env.surface).collect();
    for required in M5ExecutingSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5CapabilityEnvelopeViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for envelope in &packet.envelopes {
        if envelope.envelope_id.trim().is_empty()
            || envelope.actor.actor_ref.trim().is_empty()
            || envelope.target.target_identity.trim().is_empty()
            || envelope.redaction_class_token.trim().is_empty()
        {
            violations.push(M5CapabilityEnvelopeViolation::EnvelopeIncomplete);
        }
        if envelope.granted_capability_classes.is_empty() {
            violations.push(M5CapabilityEnvelopeViolation::CapabilityEnvelopeEmpty);
        }
        if envelope.allowed_scope.is_empty()
            || envelope
                .allowed_scope
                .iter()
                .any(|entry| entry.label.trim().is_empty())
        {
            violations.push(M5CapabilityEnvelopeViolation::AllowedScopeMissing);
        }

        if let Some(allowed) = allowed_by_surface.get(&envelope.surface) {
            if envelope
                .granted_capability_classes
                .iter()
                .any(|cap| !allowed.contains(cap))
            {
                violations.push(M5CapabilityEnvelopeViolation::CapabilityWidensBeyondMatrix);
            }
        }
        if let Some(default_profile) = default_profile_by_surface.get(&envelope.surface) {
            if envelope.sandbox_profile != *default_profile
                && envelope.sandbox_profile != M5SandboxProfile::InertNoExecution
            {
                violations.push(M5CapabilityEnvelopeViolation::SandboxProfileWidens);
            }
        }

        if envelope.actor.actor_class.is_untrusted_helper()
            && envelope.audit_lineage.self_issued_by_executor
        {
            violations.push(M5CapabilityEnvelopeViolation::SelfIssuedAuthorityForbidden);
        }

        let has_elevated = envelope
            .granted_capability_classes
            .iter()
            .any(|cap| cap.is_elevated());
        if has_elevated && envelope.audit_lineage.approval_ticket_ref.trim().is_empty() {
            violations.push(M5CapabilityEnvelopeViolation::ElevatedCapabilityWithoutTicket);
        }

        if envelope.expiry.expires_at.trim().is_empty()
            || envelope.expiry.issued_at.trim().is_empty()
            || envelope.expiry.ttl_seconds == 0
        {
            violations.push(M5CapabilityEnvelopeViolation::ExpiryMissing);
        }
        if envelope.policy_epoch.epoch_id.trim().is_empty() {
            violations.push(M5CapabilityEnvelopeViolation::PolicyEpochMissing);
        }

        if envelope.audit_lineage.issuer_ref.trim().is_empty()
            || envelope.audit_lineage.decision_chain.is_empty()
            || envelope
                .audit_lineage
                .decision_chain
                .iter()
                .any(|ref_| ref_.trim().is_empty())
        {
            violations.push(M5CapabilityEnvelopeViolation::AuditLineageIncomplete);
        }

        let projects_secret = envelope
            .granted_capability_classes
            .iter()
            .any(|cap| cap.requires_secret_scope());
        let secret_consistent = if projects_secret {
            envelope.secret_scope.grants_secret_access() && !envelope.secret_handle_refs.is_empty()
        } else {
            envelope.secret_handle_refs.is_empty()
        };
        let scope_matches_refs = envelope
            .secret_handle_refs
            .iter()
            .all(|secret_ref| secret_ref.scope == envelope.secret_scope);
        if !secret_consistent || !scope_matches_refs {
            violations.push(M5CapabilityEnvelopeViolation::SecretScopeInconsistent);
        }

        if envelope.target.off_device && !envelope.target.identity_verified {
            violations.push(M5CapabilityEnvelopeViolation::OffDeviceTargetUnverified);
        }

        if envelope.narrowed_from_default == envelope.applied_downgrade_triggers.is_empty() {
            violations.push(M5CapabilityEnvelopeViolation::NarrowingInconsistent);
        }
    }
}

fn validate_trust_review(
    packet: &M5CapabilityEnvelopePacket,
    violations: &mut Vec<M5CapabilityEnvelopeViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_self_issued_authority_by_helpers,
        review.actor_and_target_identity_inspectable,
        review.allowed_scope_inspectable,
        review.secret_refs_handle_only_no_raw_material,
        review.policy_epoch_and_expiry_inspectable,
        review.audit_lineage_inspectable,
        review.off_device_preserves_envelope_semantics,
        review.fail_closed_when_enforcement_unavailable,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5CapabilityEnvelopeViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CapabilityEnvelopePacket,
    violations: &mut Vec<M5CapabilityEnvelopeViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_envelope,
        projection.command_and_policy_reference_same_envelopes,
        projection.cli_headless_shows_envelope,
        projection.support_export_shows_envelope,
        projection.diagnostics_shows_envelope,
        projection.help_about_shows_envelope_summary,
        projection.release_evidence_consumes_envelopes,
        projection.remote_and_browser_preserve_envelope_semantics,
    ] {
        if !ok {
            violations.push(M5CapabilityEnvelopeViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CapabilityEnvelopePacket,
    violations: &mut Vec<M5CapabilityEnvelopeViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CapabilityEnvelopeViolation::ProofFreshnessIncomplete);
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
