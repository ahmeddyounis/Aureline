//! Child-envelope derivation and nested-launch narrowing for the M5 executing
//! lanes that spawn further launches.
//!
//! The capability-envelope packet states the authority *one* issued execution
//! holds: its actor, target, allowed scope, secret handles, policy epoch,
//! expiry, and lineage. This module governs what happens when that execution
//! **spawns a child** — a notebook kernel forking a worker, a scaffold hook
//! running a generator subprocess, a request lane fanning out a follow-up call,
//! a database action opening a nested session, an AI tool invoking a sub-tool,
//! or a debug session launching a debuggee. The track invariant is that a child
//! launch may only ever derive a **narrower** envelope than its parent.
//!
//! Each [`M5ChildEnvelopeDerivation`] binds a parent-authority snapshot to a
//! derived [`M5ChildEnvelope`] and proves, dimension by dimension, that the
//! child narrows the parent: its granted capability classes are a subset of the
//! parent's, every allowed root/sink/endpoint is contained within a parent
//! scope entry at an equal-or-narrower access mode, its sandbox profile is
//! equal-or-stricter, its secret scope is equal-or-narrower, its expiry is no
//! later than the parent's, and it runs under the parent's policy epoch. A child
//! never inherits the raw OS environment and never fans out full parent
//! authority — ambient environment is projected only as allowlisted handles, and
//! [`M5ChildEnvelopeDerivation::inherits_full_parent_authority`] is always
//! false.
//!
//! Secret projection into a child defaults to handle-only references re-minted or
//! narrowed from the parent; raw secret material never crosses the derivation
//! boundary. When the platform's execution-isolation backend cannot honor the
//! child profile, the derivation either narrows to a stricter supported profile
//! or becomes visibly unsupported and fails closed — it never silently widens or
//! runs unconfined. No AI tool, recipe, extension, browser route, or remote
//! helper self-issues a child envelope: every derivation for an untrusted-helper
//! actor carries externally issued lineage and is flagged
//! `self_issued_by_executor: false`.
//!
//! The boundary schema is
//! [`schemas/execution-auth/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.schema.json`](../../../../schemas/execution-auth/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.md`](../../../../docs/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/`](../../../../fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5CapabilityClass, M5DegradedFallback,
    M5ExecutingSurface, M5RuntimeAuthorityDowngradeTrigger, M5SandboxProfile, M5SecretScope,
    M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF, M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF, M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
};
use super::ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::{
    M5AllowedScopeEntry, M5AllowedScopeKind, M5EnvelopeActorClass, M5EnvelopeIssuerClass,
    M5PolicyEpochBinding, M5ScopeAccessMode, M5SecretHandleRef, M5_CAPABILITY_ENVELOPE_DOC_REF,
    M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ChildEnvelopeDerivationPacket`].
pub const M5_CHILD_ENVELOPE_DERIVATION_RECORD_KIND: &str = "ship_m5_child_envelope_derivation";

/// Schema version for the M5 child-envelope derivation packet records.
pub const M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF: &str =
    "schemas/execution-auth/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHILD_ENVELOPE_DERIVATION_DOC_REF: &str =
    "docs/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHILD_ENVELOPE_DERIVATION_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CHILD_ENVELOPE_DERIVATION_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHILD_ENVELOPE_DERIVATION_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen";

/// Stable packet id minted by [`frozen_stable_m5_child_envelope_derivation_packet`].
pub const M5_CHILD_ENVELOPE_DERIVATION_PACKET_ID: &str = "m5-child-envelope-derivation:stable:0001";

/// One M5 executing lane that can spawn a nested child launch.
///
/// Five lanes map onto a frozen runtime-authority matrix surface
/// ([`Self::parent_surface`]); the debug lane spawns a debuggee subprocess that
/// has no standalone matrix surface, so its parent authority is bounded only by
/// the recorded parent-authority snapshot rather than a matrix row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NestedLaunchLane {
    /// Notebook execution kernel forking a worker or sub-kernel.
    Notebook,
    /// Scaffold or generator hook running a child generator subprocess.
    Scaffold,
    /// Request/API lane fanning out a derived follow-up call.
    Request,
    /// Database action opening a nested session or sub-transaction.
    Database,
    /// AI tool invoking a sub-tool or spawning a helper process.
    Ai,
    /// Debug session launching a debuggee subprocess.
    Debug,
}

impl M5NestedLaunchLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Notebook,
        Self::Scaffold,
        Self::Request,
        Self::Database,
        Self::Ai,
        Self::Debug,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::Scaffold => "scaffold",
            Self::Request => "request",
            Self::Database => "database",
            Self::Ai => "ai",
            Self::Debug => "debug",
        }
    }

    /// The frozen runtime-authority matrix surface that bounds this lane's parent
    /// authority, or [`None`] for the debug lane which has no standalone surface.
    pub const fn parent_surface(self) -> Option<M5ExecutingSurface> {
        match self {
            Self::Notebook => Some(M5ExecutingSurface::NotebookKernel),
            Self::Scaffold => Some(M5ExecutingSurface::ScaffoldHook),
            Self::Request => Some(M5ExecutingSurface::RequestApiSend),
            Self::Database => Some(M5ExecutingSurface::DatabaseAction),
            Self::Ai => Some(M5ExecutingSurface::AiTool),
            Self::Debug => None,
        }
    }
}

/// One dimension along which a child launch narrows below its baseline
/// derivation in response to a downgrade trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DerivationNarrowingDimension {
    /// One or more capability classes were dropped from the child.
    CapabilityDropped,
    /// One or more allowed roots/sinks/endpoints were removed or tightened.
    ScopeNarrowed,
    /// The child sandbox profile was tightened to a stricter profile.
    SandboxTightened,
    /// The child secret scope was narrowed (down to no secret access).
    SecretScopeNarrowed,
    /// The child expiry was shortened below the parent's remaining lifetime.
    ExpiryShortened,
    /// Ambient environment projection was stripped to nothing.
    EnvironmentStripped,
}

impl M5DerivationNarrowingDimension {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityDropped => "capability_dropped",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::SandboxTightened => "sandbox_tightened",
            Self::SecretScopeNarrowed => "secret_scope_narrowed",
            Self::ExpiryShortened => "expiry_shortened",
            Self::EnvironmentStripped => "environment_stripped",
        }
    }
}

/// Posture describing how the parent's ambient environment is projected to the
/// child.
///
/// The forbidden state — [`Self::RawOsEnvironmentInherited`] — is modeled so the
/// validator can reject any derivation that fans the raw OS environment into a
/// child; the honored states project nothing, allowlisted handles, or brokered
/// references only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AmbientEnvironmentPosture {
    /// The child inherits no environment at all.
    NoEnvironmentInherited,
    /// The child receives an explicit allowlist of environment handles.
    AllowlistedHandlesOnly,
    /// The child receives brokered environment references resolved on demand.
    BrokeredEnvironmentRefs,
    /// Forbidden: the child inherits the raw OS environment. Never honored.
    RawOsEnvironmentInherited,
}

impl M5AmbientEnvironmentPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoEnvironmentInherited => "no_environment_inherited",
            Self::AllowlistedHandlesOnly => "allowlisted_handles_only",
            Self::BrokeredEnvironmentRefs => "brokered_environment_refs",
            Self::RawOsEnvironmentInherited => "raw_os_environment_inherited",
        }
    }

    /// Whether this posture leaks ambient privilege by fanning the raw OS
    /// environment into the child.
    pub const fn is_ambient_privilege_leak(self) -> bool {
        matches!(self, Self::RawOsEnvironmentInherited)
    }
}

/// Status of the execution-isolation backend that must enforce the child
/// profile on the running platform.
///
/// The forbidden state — [`Self::SilentlyPermissiveUnsupported`] — is modeled so
/// the validator can reject any derivation that ran unconfined when its backend
/// was missing; the honored states enforce, narrow to a stricter profile, or
/// become visibly unsupported and fail closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnforcementBackendStatus {
    /// The backend enforces the child profile as derived.
    Enforced,
    /// The backend is unavailable, so the child narrowed to a stricter profile.
    NarrowedToStricterProfile,
    /// The backend is unavailable, so the child is visibly unsupported and fails closed.
    UnsupportedVisiblyDegraded,
    /// Forbidden: the backend was missing and the child ran unconfined. Never honored.
    SilentlyPermissiveUnsupported,
}

impl M5EnforcementBackendStatus {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enforced => "enforced",
            Self::NarrowedToStricterProfile => "narrowed_to_stricter_profile",
            Self::UnsupportedVisiblyDegraded => "unsupported_visibly_degraded",
            Self::SilentlyPermissiveUnsupported => "silently_permissive_unsupported",
        }
    }

    /// Whether this status silently widened authority by running unconfined when
    /// the enforcement backend could not be honored.
    pub const fn is_silently_permissive(self) -> bool {
        matches!(self, Self::SilentlyPermissiveUnsupported)
    }
}

/// Actor a child envelope is derived for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DerivationActor {
    /// Actor class.
    pub actor_class: M5EnvelopeActorClass,
    /// Export-safe actor reference (opaque id or label; never a credential).
    pub actor_ref: String,
    /// Export-safe principal this actor acts on behalf of, when delegated.
    pub on_behalf_of: Option<String>,
}

/// Snapshot of the parent authority a child is derived from.
///
/// Carries the export-safe ceiling the child may not exceed: the parent's
/// granted capability classes, allowed scope, sandbox profile, secret scope,
/// policy epoch, and expiry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ParentAuthoritySnapshot {
    /// Export-safe reference to the parent capability envelope.
    pub parent_envelope_ref: String,
    /// Capability classes the parent holds.
    pub granted_capability_classes: Vec<M5CapabilityClass>,
    /// Allowed roots, sinks, and endpoints the parent may touch.
    pub allowed_scope: Vec<M5AllowedScopeEntry>,
    /// Sandbox profile the parent ran under.
    pub sandbox_profile: M5SandboxProfile,
    /// Secret scope the parent holds.
    pub secret_scope: M5SecretScope,
    /// Governing policy-epoch binding for the parent.
    pub policy_epoch: M5PolicyEpochBinding,
    /// RFC 3339 timestamp the parent authority expires at.
    pub expires_at: String,
}

/// The derived child envelope minted for a nested launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChildEnvelope {
    /// Stable child envelope id.
    pub envelope_id: String,
    /// Export-safe child target identity (host label, path, or resource id).
    pub target_identity: String,
    /// True when the child runs off-device or is brokered by another runtime.
    pub off_device: bool,
    /// True when the child target identity has been verified.
    pub identity_verified: bool,
    /// Capability classes granted to the child (a subset of the parent's).
    pub granted_capability_classes: Vec<M5CapabilityClass>,
    /// Allowed roots, sinks, and endpoints the child may touch.
    pub allowed_scope: Vec<M5AllowedScopeEntry>,
    /// Handle-only secret references projected into the child.
    pub secret_handle_refs: Vec<M5SecretHandleRef>,
    /// Secret scope for the child (equal-or-narrower than the parent's).
    pub secret_scope: M5SecretScope,
    /// Sandbox profile the child runs under (equal-or-stricter than the parent's).
    pub sandbox_profile: M5SandboxProfile,
    /// Governing policy-epoch binding (must equal the parent's).
    pub policy_epoch: M5PolicyEpochBinding,
    /// RFC 3339 child issuance timestamp.
    pub issued_at: String,
    /// RFC 3339 child expiry timestamp (no later than the parent's).
    pub expires_at: String,
    /// Child time-to-live in seconds; must be non-zero.
    pub ttl_seconds: u32,
    /// True when the child authorizes a single action and cannot be replayed.
    pub single_use: bool,
}

/// Audit-lineage binding for a child-envelope derivation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DerivationLineage {
    /// Issuer class that minted the child's authority (always external).
    pub issuer_class: M5EnvelopeIssuerClass,
    /// Export-safe issuer reference.
    pub issuer_ref: String,
    /// Export-safe reference to the parent capability envelope.
    pub parent_envelope_ref: String,
    /// Export-safe approval-ticket reference (never a live ticket signature).
    pub approval_ticket_ref: String,
    /// Ordered export-safe lineage refs from parent envelope to child envelope.
    pub decision_chain: Vec<String>,
    /// Always false: the executor never self-issues the child envelope.
    pub self_issued_by_executor: bool,
}

/// One child-envelope derivation bound to a single nested M5 launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChildEnvelopeDerivation {
    /// Stable derivation id.
    pub derivation_id: String,
    /// Lane this nested launch belongs to.
    pub lane: M5NestedLaunchLane,
    /// Actor the child is derived for.
    pub actor: M5DerivationActor,
    /// Snapshot of the parent authority the child is derived from.
    pub parent: M5ParentAuthoritySnapshot,
    /// The derived child envelope.
    pub child: M5ChildEnvelope,
    /// Posture describing how the parent's ambient environment reaches the child.
    pub ambient_environment_posture: M5AmbientEnvironmentPosture,
    /// Always false: a child never fans out full parent authority.
    pub inherits_full_parent_authority: bool,
    /// Label of the execution-isolation backend enforcing the child profile.
    pub enforcement_backend: String,
    /// Status of the enforcement backend on the running platform.
    pub enforcement_status: M5EnforcementBackendStatus,
    /// Audit-lineage binding.
    pub audit_lineage: M5DerivationLineage,
    /// Degraded fallback applied when the child cannot run at its derived authority.
    pub degraded_fallback: M5DegradedFallback,
    /// Downgrade triggers applied to this child; empty when nominal.
    pub applied_downgrade_triggers: Vec<M5RuntimeAuthorityDowngradeTrigger>,
    /// Narrowing dimensions applied below the baseline derivation; empty when nominal.
    pub applied_narrowings: Vec<M5DerivationNarrowingDimension>,
    /// True when a downgrade trigger narrowed this child below its baseline derivation.
    pub narrowed_below_baseline: bool,
    /// Per-derivation redaction class token.
    pub redaction_class_token: String,
}

/// Trust and isolation review block for the derivation packet.
///
/// Every field encodes a hard invariant; all must hold for the packet to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChildEnvelopeDerivationTrustReview {
    /// No child derivation grants ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No child inherits the raw OS environment.
    pub no_raw_os_environment_inheritance: bool,
    /// Child envelopes only ever narrow their parent's authority.
    pub child_envelopes_only_narrow_parent: bool,
    /// No child fans out full parent authority.
    pub no_full_parent_authority_fan_out: bool,
    /// Secret projection into children is handle-only; no raw material crosses.
    pub secret_projection_handle_only_no_raw_material: bool,
    /// No untrusted-helper actor self-issues a child envelope.
    pub no_self_issued_authority_by_helpers: bool,
    /// An unsupported enforcement backend fails closed or becomes visibly unsupported.
    pub unsupported_backend_fails_closed_or_visible: bool,
    /// Parent and child identity are inspectable and export-safe.
    pub parent_and_child_identity_inspectable: bool,
    /// Policy epoch and expiry are inspectable on every derivation.
    pub policy_epoch_and_expiry_inspectable: bool,
    /// Audit lineage is inspectable on every derivation.
    pub audit_lineage_inspectable: bool,
    /// No raw secret material is exported inside derivations or support packets.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block for the derivation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChildEnvelopeDerivationConsumerProjection {
    /// Desktop shell shows the full derivation (parent, child, narrowing, lineage).
    pub desktop_shows_derivation: bool,
    /// Command palette and policy inspector reference the same derivations.
    pub command_and_policy_reference_same_derivations: bool,
    /// CLI / headless shows the full derivation.
    pub cli_headless_shows_derivation: bool,
    /// Support export shows the full derivation.
    pub support_export_shows_derivation: bool,
    /// Diagnostics shows the full derivation.
    pub diagnostics_shows_derivation: bool,
    /// Help / About shows a derivation summary.
    pub help_about_shows_derivation_summary: bool,
    /// Release evidence consumes these derivations instead of cloning per-surface prose.
    pub release_evidence_consumes_derivations: bool,
    /// Remote and browser-routed surfaces preserve derivation semantics off-device.
    pub remote_and_browser_preserve_derivation_semantics: bool,
}

/// Proof-freshness block for the derivation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChildEnvelopeDerivationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected derivations.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ChildEnvelopeDerivationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChildEnvelopeDerivationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Issued child-envelope derivations.
    pub derivations: Vec<M5ChildEnvelopeDerivation>,
    /// Trust review block.
    pub trust_review: M5ChildEnvelopeDerivationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChildEnvelopeDerivationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChildEnvelopeDerivationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 child-envelope derivation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChildEnvelopeDerivationPacket {
    /// Record kind; must equal [`M5_CHILD_ENVELOPE_DERIVATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Issued child-envelope derivations.
    pub derivations: Vec<M5ChildEnvelopeDerivation>,
    /// Trust review block.
    pub trust_review: M5ChildEnvelopeDerivationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChildEnvelopeDerivationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChildEnvelopeDerivationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChildEnvelopeDerivationPacket {
    /// Builds an M5 child-envelope derivation packet from frozen input.
    pub fn new(input: M5ChildEnvelopeDerivationPacketInput) -> Self {
        Self {
            record_kind: M5_CHILD_ENVELOPE_DERIVATION_RECORD_KIND.to_owned(),
            schema_version: M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            derivations: input.derivations,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 child-envelope derivation packet invariants.
    pub fn validate(&self) -> Vec<M5ChildEnvelopeDerivationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CHILD_ENVELOPE_DERIVATION_RECORD_KIND {
            violations.push(M5ChildEnvelopeDerivationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_VERSION {
            violations.push(M5ChildEnvelopeDerivationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ChildEnvelopeDerivationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_derivations(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 child-envelope derivation packet serializes"),
        ) {
            violations.push(M5ChildEnvelopeDerivationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 child-envelope derivation packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .derivations
            .iter()
            .filter(|derivation| derivation.narrowed_below_baseline)
            .count();
        let mut out = String::new();
        out.push_str("# M5 Child-Envelope Derivations\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Derivations: {} ({} narrowed below baseline)\n",
            self.derivations.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Nested launches\n\n");
        for derivation in &self.derivations {
            out.push_str(&format!(
                "- **{}** ({})\n",
                derivation.lane.as_str(),
                derivation.derivation_id
            ));
            out.push_str(&format!(
                "  - Actor: {} (`{}`)\n",
                derivation.actor.actor_class.as_str(),
                derivation.actor.actor_ref
            ));
            let parent_caps = derivation
                .parent
                .granted_capability_classes
                .iter()
                .map(|cap| cap.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let child_caps = derivation
                .child
                .granted_capability_classes
                .iter()
                .map(|cap| cap.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("  - Parent caps: {parent_caps}\n"));
            out.push_str(&format!("  - Child caps: {child_caps}\n"));
            out.push_str(&format!(
                "  - Sandbox: {} -> {} · Secret scope: {} -> {}\n",
                derivation.parent.sandbox_profile.as_str(),
                derivation.child.sandbox_profile.as_str(),
                derivation.parent.secret_scope.as_str(),
                derivation.child.secret_scope.as_str()
            ));
            out.push_str(&format!(
                "  - Environment: {} · Enforcement: {}\n",
                derivation.ambient_environment_posture.as_str(),
                derivation.enforcement_status.as_str()
            ));
            out.push_str(&format!(
                "  - Issuer: {} (`{}`) · Ticket: {} · Self-issued: {}\n",
                derivation.audit_lineage.issuer_class.as_str(),
                derivation.audit_lineage.issuer_ref,
                derivation.audit_lineage.approval_ticket_ref,
                derivation.audit_lineage.self_issued_by_executor
            ));
            if derivation.narrowed_below_baseline {
                let narrowings = derivation
                    .applied_narrowings
                    .iter()
                    .map(|dim| dim.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("  - Narrowed: {narrowings}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 child-envelope derivation export.
#[derive(Debug)]
pub enum M5ChildEnvelopeDerivationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChildEnvelopeDerivationViolation>),
}

impl fmt::Display for M5ChildEnvelopeDerivationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 child-envelope derivation export parse failed: {error}"
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
                    "m5 child-envelope derivation export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChildEnvelopeDerivationArtifactError {}

/// Validation failures emitted by [`M5ChildEnvelopeDerivationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChildEnvelopeDerivationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required nested-launch lane has no derivation.
    RequiredLaneMissing,
    /// A derivation is missing required identity fields.
    DerivationIncomplete,
    /// A child grants no capability classes.
    ChildCapabilityEmpty,
    /// A child grants a capability class its parent does not hold.
    ChildCapabilityWidensParent,
    /// A parent snapshot widens beyond its matrix surface row.
    ParentCapabilityWidensMatrix,
    /// A child has no allowed roots, sinks, or endpoints.
    ChildScopeMissing,
    /// A child allowed-scope entry escapes or widens the parent's scope.
    ChildScopeWidensParent,
    /// A child runs under a sandbox profile less strict than its parent's.
    ChildSandboxWidensParent,
    /// A child holds a secret scope wider than its parent's.
    ChildSecretScopeWidensParent,
    /// A child projects a secret with non-handle-only references or material.
    SecretProjectionNotHandleOnly,
    /// Secret references are inconsistent with the declared child secret scope.
    SecretScopeInconsistent,
    /// A child expiry is later than its parent's expiry.
    ChildExpiryExceedsParent,
    /// A child expiry is missing or zero.
    ExpiryMissing,
    /// A child policy epoch does not match its parent's.
    PolicyEpochMismatch,
    /// A child inherits the raw OS environment.
    RawOsEnvironmentInherited,
    /// A child fans out full parent authority.
    FullParentAuthorityFannedOut,
    /// An enforcement backend silently widened authority instead of failing closed.
    EnforcementSilentlyPermissive,
    /// An enforcement backend label is missing.
    EnforcementBackendMissing,
    /// An untrusted-helper actor self-issues a child envelope.
    SelfIssuedAuthorityForbidden,
    /// An elevated child capability is granted without an approval-ticket reference.
    ElevatedCapabilityWithoutTicket,
    /// The audit lineage is incomplete.
    AuditLineageIncomplete,
    /// An off-device child binds to an unverified target.
    OffDeviceTargetUnverified,
    /// The narrowed flag is inconsistent with the applied triggers or narrowings.
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

impl M5ChildEnvelopeDerivationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::DerivationIncomplete => "derivation_incomplete",
            Self::ChildCapabilityEmpty => "child_capability_empty",
            Self::ChildCapabilityWidensParent => "child_capability_widens_parent",
            Self::ParentCapabilityWidensMatrix => "parent_capability_widens_matrix",
            Self::ChildScopeMissing => "child_scope_missing",
            Self::ChildScopeWidensParent => "child_scope_widens_parent",
            Self::ChildSandboxWidensParent => "child_sandbox_widens_parent",
            Self::ChildSecretScopeWidensParent => "child_secret_scope_widens_parent",
            Self::SecretProjectionNotHandleOnly => "secret_projection_not_handle_only",
            Self::SecretScopeInconsistent => "secret_scope_inconsistent",
            Self::ChildExpiryExceedsParent => "child_expiry_exceeds_parent",
            Self::ExpiryMissing => "expiry_missing",
            Self::PolicyEpochMismatch => "policy_epoch_mismatch",
            Self::RawOsEnvironmentInherited => "raw_os_environment_inherited",
            Self::FullParentAuthorityFannedOut => "full_parent_authority_fanned_out",
            Self::EnforcementSilentlyPermissive => "enforcement_silently_permissive",
            Self::EnforcementBackendMissing => "enforcement_backend_missing",
            Self::SelfIssuedAuthorityForbidden => "self_issued_authority_forbidden",
            Self::ElevatedCapabilityWithoutTicket => "elevated_capability_without_ticket",
            Self::AuditLineageIncomplete => "audit_lineage_incomplete",
            Self::OffDeviceTargetUnverified => "off_device_target_unverified",
            Self::NarrowingInconsistent => "narrowing_inconsistent",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Strictness rank for a sandbox profile; a higher rank is more isolated.
///
/// A child derivation may only hold a sandbox profile with a rank greater than
/// or equal to its parent's. [`M5SandboxProfile::InertNoExecution`] is the
/// strictest because it runs no code at all.
const fn sandbox_strictness_rank(profile: M5SandboxProfile) -> u8 {
    match profile {
        M5SandboxProfile::InProcessTrustedLocal => 0,
        M5SandboxProfile::BrokeredNetworkOnly => 1,
        M5SandboxProfile::SubprocessIsolatedLocal => 2,
        M5SandboxProfile::ContainerIsolatedLocal => 3,
        M5SandboxProfile::IsolatedRemoteRuntime => 4,
        M5SandboxProfile::InertNoExecution => 5,
    }
}

/// Authority rank for a secret scope; a higher rank grants more secret access.
const fn secret_scope_rank(scope: M5SecretScope) -> u8 {
    match scope {
        M5SecretScope::NoSecretAccess => 0,
        M5SecretScope::HandleOnlyDelegated => 1,
        M5SecretScope::ScopedBrokeredSecret => 2,
    }
}

/// Authority rank for an access mode; a higher rank grants more authority.
const fn access_mode_rank(mode: M5ScopeAccessMode) -> u8 {
    match mode {
        M5ScopeAccessMode::ReadOnly | M5ScopeAccessMode::Navigate => 1,
        M5ScopeAccessMode::AppendOnly | M5ScopeAccessMode::SendOnly => 2,
        M5ScopeAccessMode::ReadWrite => 3,
    }
}

/// Whether a child scope entry is contained within some parent scope entry at an
/// equal-or-narrower access mode.
fn scope_entry_within_parent(child: &M5AllowedScopeEntry, parent: &[M5AllowedScopeEntry]) -> bool {
    parent.iter().any(|parent_entry| {
        parent_entry.kind == child.kind
            && child.label.starts_with(&parent_entry.label)
            && access_mode_rank(child.access) <= access_mode_rank(parent_entry.access)
    })
}

fn validate_source_contracts(
    packet: &M5ChildEnvelopeDerivationPacket,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF,
        M5_CHILD_ENVELOPE_DERIVATION_DOC_REF,
        M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
        M5_CAPABILITY_ENVELOPE_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ChildEnvelopeDerivationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_derivations(
    packet: &M5ChildEnvelopeDerivationPacket,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
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

    let present: BTreeSet<M5NestedLaunchLane> = packet.derivations.iter().map(|d| d.lane).collect();
    for required in M5NestedLaunchLane::ALL {
        if !present.contains(&required) {
            violations.push(M5ChildEnvelopeDerivationViolation::RequiredLaneMissing);
            return;
        }
    }

    for derivation in &packet.derivations {
        validate_one_derivation(derivation, &allowed_by_surface, violations);
    }
}

fn validate_one_derivation(
    derivation: &M5ChildEnvelopeDerivation,
    allowed_by_surface: &BTreeMap<M5ExecutingSurface, BTreeSet<M5CapabilityClass>>,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
) {
    let parent = &derivation.parent;
    let child = &derivation.child;

    if derivation.derivation_id.trim().is_empty()
        || derivation.actor.actor_ref.trim().is_empty()
        || child.envelope_id.trim().is_empty()
        || child.target_identity.trim().is_empty()
        || parent.parent_envelope_ref.trim().is_empty()
        || derivation.redaction_class_token.trim().is_empty()
    {
        violations.push(M5ChildEnvelopeDerivationViolation::DerivationIncomplete);
    }

    if child.granted_capability_classes.is_empty() {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildCapabilityEmpty);
    }

    let parent_caps: BTreeSet<M5CapabilityClass> =
        parent.granted_capability_classes.iter().copied().collect();
    if child
        .granted_capability_classes
        .iter()
        .any(|cap| !parent_caps.contains(cap))
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildCapabilityWidensParent);
    }

    if let Some(surface) = derivation.lane.parent_surface() {
        if let Some(allowed) = allowed_by_surface.get(&surface) {
            if parent
                .granted_capability_classes
                .iter()
                .any(|cap| !allowed.contains(cap))
            {
                violations.push(M5ChildEnvelopeDerivationViolation::ParentCapabilityWidensMatrix);
            }
        }
    }

    if child.allowed_scope.is_empty()
        || child
            .allowed_scope
            .iter()
            .any(|entry| entry.label.trim().is_empty())
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildScopeMissing);
    }
    if child
        .allowed_scope
        .iter()
        .any(|entry| !scope_entry_within_parent(entry, &parent.allowed_scope))
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildScopeWidensParent);
    }

    if sandbox_strictness_rank(child.sandbox_profile)
        < sandbox_strictness_rank(parent.sandbox_profile)
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildSandboxWidensParent);
    }

    if secret_scope_rank(child.secret_scope) > secret_scope_rank(parent.secret_scope) {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildSecretScopeWidensParent);
    }

    validate_child_secrets(derivation, violations);

    if child.expires_at.trim().is_empty()
        || child.issued_at.trim().is_empty()
        || child.ttl_seconds == 0
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ExpiryMissing);
    }
    if !child.expires_at.trim().is_empty()
        && !parent.expires_at.trim().is_empty()
        && child.expires_at > parent.expires_at
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ChildExpiryExceedsParent);
    }

    if child.policy_epoch != parent.policy_epoch {
        violations.push(M5ChildEnvelopeDerivationViolation::PolicyEpochMismatch);
    }

    if derivation
        .ambient_environment_posture
        .is_ambient_privilege_leak()
    {
        violations.push(M5ChildEnvelopeDerivationViolation::RawOsEnvironmentInherited);
    }
    if derivation.inherits_full_parent_authority {
        violations.push(M5ChildEnvelopeDerivationViolation::FullParentAuthorityFannedOut);
    }

    if derivation.enforcement_status.is_silently_permissive() {
        violations.push(M5ChildEnvelopeDerivationViolation::EnforcementSilentlyPermissive);
    }
    if derivation.enforcement_backend.trim().is_empty() {
        violations.push(M5ChildEnvelopeDerivationViolation::EnforcementBackendMissing);
    }

    if derivation.actor.actor_class.is_untrusted_helper()
        && derivation.audit_lineage.self_issued_by_executor
    {
        violations.push(M5ChildEnvelopeDerivationViolation::SelfIssuedAuthorityForbidden);
    }

    let has_elevated = child
        .granted_capability_classes
        .iter()
        .any(|cap| cap.is_elevated());
    if has_elevated
        && derivation
            .audit_lineage
            .approval_ticket_ref
            .trim()
            .is_empty()
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ElevatedCapabilityWithoutTicket);
    }

    if derivation.audit_lineage.issuer_ref.trim().is_empty()
        || derivation
            .audit_lineage
            .parent_envelope_ref
            .trim()
            .is_empty()
        || derivation.audit_lineage.decision_chain.is_empty()
        || derivation
            .audit_lineage
            .decision_chain
            .iter()
            .any(|ref_| ref_.trim().is_empty())
    {
        violations.push(M5ChildEnvelopeDerivationViolation::AuditLineageIncomplete);
    }

    if child.off_device && !child.identity_verified {
        violations.push(M5ChildEnvelopeDerivationViolation::OffDeviceTargetUnverified);
    }

    let triggers_present = !derivation.applied_downgrade_triggers.is_empty();
    let narrowings_present = !derivation.applied_narrowings.is_empty();
    if derivation.narrowed_below_baseline != triggers_present
        || derivation.narrowed_below_baseline != narrowings_present
    {
        violations.push(M5ChildEnvelopeDerivationViolation::NarrowingInconsistent);
    }
}

fn validate_child_secrets(
    derivation: &M5ChildEnvelopeDerivation,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
) {
    let child = &derivation.child;
    let projects_secret = child
        .granted_capability_classes
        .iter()
        .any(|cap| cap.requires_secret_scope());

    let secret_consistent = if projects_secret {
        child.secret_scope.grants_secret_access() && !child.secret_handle_refs.is_empty()
    } else {
        child.secret_handle_refs.is_empty()
    };
    let scope_matches_refs = child
        .secret_handle_refs
        .iter()
        .all(|secret_ref| secret_ref.scope == child.secret_scope);
    if !secret_consistent || !scope_matches_refs {
        violations.push(M5ChildEnvelopeDerivationViolation::SecretScopeInconsistent);
    }

    if child
        .secret_handle_refs
        .iter()
        .any(|secret_ref| secret_ref.handle_ref.trim().is_empty())
    {
        violations.push(M5ChildEnvelopeDerivationViolation::SecretProjectionNotHandleOnly);
    }
}

fn validate_trust_review(
    packet: &M5ChildEnvelopeDerivationPacket,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_raw_os_environment_inheritance,
        review.child_envelopes_only_narrow_parent,
        review.no_full_parent_authority_fan_out,
        review.secret_projection_handle_only_no_raw_material,
        review.no_self_issued_authority_by_helpers,
        review.unsupported_backend_fails_closed_or_visible,
        review.parent_and_child_identity_inspectable,
        review.policy_epoch_and_expiry_inspectable,
        review.audit_lineage_inspectable,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5ChildEnvelopeDerivationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ChildEnvelopeDerivationPacket,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_derivation,
        projection.command_and_policy_reference_same_derivations,
        projection.cli_headless_shows_derivation,
        projection.support_export_shows_derivation,
        projection.diagnostics_shows_derivation,
        projection.help_about_shows_derivation_summary,
        projection.release_evidence_consumes_derivations,
        projection.remote_and_browser_preserve_derivation_semantics,
    ] {
        if !ok {
            violations.push(M5ChildEnvelopeDerivationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ChildEnvelopeDerivationPacket,
    violations: &mut Vec<M5ChildEnvelopeDerivationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ChildEnvelopeDerivationViolation::ProofFreshnessIncomplete);
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

fn nominal_epoch() -> M5PolicyEpochBinding {
    M5PolicyEpochBinding {
        epoch_id: "policy-epoch:m5:0007".to_owned(),
        epoch_sequence: 7,
        superseded: false,
    }
}

fn secret_handle_contract() -> String {
    M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF.to_owned()
}

/// Per-lane fixed parameters used to build deterministic derivations.
struct LaneSpec {
    actor_class: M5EnvelopeActorClass,
    actor_ref: &'static str,
    on_behalf_of: Option<&'static str>,
    issuer_class: M5EnvelopeIssuerClass,
    issuer_ref: &'static str,
    parent_envelope_ref: &'static str,
    parent_caps: &'static [M5CapabilityClass],
    parent_sandbox: M5SandboxProfile,
    parent_secret_scope: M5SecretScope,
    parent_root: &'static str,
    parent_root_kind: M5AllowedScopeKind,
    parent_root_access: M5ScopeAccessMode,
    child_caps: &'static [M5CapabilityClass],
    child_target: &'static str,
    child_off_device: bool,
    child_scope_label: &'static str,
    child_scope_kind: M5AllowedScopeKind,
    child_scope_access: M5ScopeAccessMode,
    child_secret_scope: M5SecretScope,
    child_sandbox: M5SandboxProfile,
    handle_ref: &'static str,
    approval_ticket_ref: &'static str,
    enforcement_backend: &'static str,
    degraded_fallback: M5DegradedFallback,
}

fn lane_spec(lane: M5NestedLaunchLane) -> LaneSpec {
    match lane {
        M5NestedLaunchLane::Notebook => LaneSpec {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local",
            on_behalf_of: None,
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local",
            parent_envelope_ref: "envelope:notebook-kernel:0001",
            parent_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
                M5CapabilityClass::NetworkEgress,
            ],
            parent_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            parent_secret_scope: M5SecretScope::NoSecretAccess,
            parent_root: "workspace://project",
            parent_root_kind: M5AllowedScopeKind::FilesystemRoot,
            parent_root_access: M5ScopeAccessMode::ReadWrite,
            child_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
            ],
            child_target: "kernel://project/notebook-7/worker-1",
            child_off_device: false,
            child_scope_label: "workspace://project/notebooks",
            child_scope_kind: M5AllowedScopeKind::FilesystemRoot,
            child_scope_access: M5ScopeAccessMode::ReadWrite,
            child_secret_scope: M5SecretScope::NoSecretAccess,
            child_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            handle_ref: "",
            approval_ticket_ref: "ticket:notebook-kernel:0001",
            enforcement_backend: "seatbelt-sandbox",
            degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        },
        M5NestedLaunchLane::Scaffold => LaneSpec {
            actor_class: M5EnvelopeActorClass::SystemAutomation,
            actor_ref: "actor:scaffold-runner:local",
            on_behalf_of: Some("actor:operator:local"),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local",
            parent_envelope_ref: "envelope:scaffold-hook:0001",
            parent_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
            ],
            parent_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            parent_secret_scope: M5SecretScope::NoSecretAccess,
            parent_root: "workspace://project/templates",
            parent_root_kind: M5AllowedScopeKind::FilesystemRoot,
            parent_root_access: M5ScopeAccessMode::ReadWrite,
            child_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
            ],
            child_target: "scaffold://project/templates/generator-2",
            child_off_device: false,
            child_scope_label: "workspace://project/templates/out",
            child_scope_kind: M5AllowedScopeKind::FilesystemRoot,
            child_scope_access: M5ScopeAccessMode::ReadWrite,
            child_secret_scope: M5SecretScope::NoSecretAccess,
            child_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            handle_ref: "",
            approval_ticket_ref: "ticket:scaffold-hook:0001",
            enforcement_backend: "seatbelt-sandbox",
            degraded_fallback: M5DegradedFallback::NarrowToSanitizedPreview,
        },
        M5NestedLaunchLane::Request => LaneSpec {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local",
            on_behalf_of: None,
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local",
            parent_envelope_ref: "envelope:request-api-send:0001",
            parent_caps: &[
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            parent_sandbox: M5SandboxProfile::BrokeredNetworkOnly,
            parent_secret_scope: M5SecretScope::HandleOnlyDelegated,
            parent_root: "https://api.example.test/v1",
            parent_root_kind: M5AllowedScopeKind::NetworkEndpoint,
            parent_root_access: M5ScopeAccessMode::SendOnly,
            child_caps: &[M5CapabilityClass::NetworkEgress],
            child_target: "https://api.example.test/v1/orders/follow-up",
            child_off_device: false,
            child_scope_label: "https://api.example.test/v1/orders",
            child_scope_kind: M5AllowedScopeKind::NetworkEndpoint,
            child_scope_access: M5ScopeAccessMode::SendOnly,
            child_secret_scope: M5SecretScope::NoSecretAccess,
            child_sandbox: M5SandboxProfile::BrokeredNetworkOnly,
            handle_ref: "",
            approval_ticket_ref: "ticket:request-api-send:0001",
            enforcement_backend: "transport-plane-broker",
            degraded_fallback: M5DegradedFallback::RequireFreshTicket,
        },
        M5NestedLaunchLane::Database => LaneSpec {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local",
            on_behalf_of: None,
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local",
            parent_envelope_ref: "envelope:database-action:0001",
            parent_caps: &[
                M5CapabilityClass::DatabaseRead,
                M5CapabilityClass::DatabaseWrite,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            parent_sandbox: M5SandboxProfile::BrokeredNetworkOnly,
            parent_secret_scope: M5SecretScope::HandleOnlyDelegated,
            parent_root: "db://primary/orders",
            parent_root_kind: M5AllowedScopeKind::DataSink,
            parent_root_access: M5ScopeAccessMode::ReadWrite,
            child_caps: &[
                M5CapabilityClass::DatabaseRead,
                M5CapabilityClass::SecretHandleProjection,
            ],
            child_target: "db://primary/orders/read-replica",
            child_off_device: false,
            child_scope_label: "db://primary/orders/items",
            child_scope_kind: M5AllowedScopeKind::DataSink,
            child_scope_access: M5ScopeAccessMode::ReadOnly,
            child_secret_scope: M5SecretScope::HandleOnlyDelegated,
            child_sandbox: M5SandboxProfile::BrokeredNetworkOnly,
            handle_ref: "broker-handle:database-child:0001",
            approval_ticket_ref: "ticket:database-action:0001",
            enforcement_backend: "transport-plane-broker",
            degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        },
        M5NestedLaunchLane::Ai => LaneSpec {
            actor_class: M5EnvelopeActorClass::AiTool,
            actor_ref: "actor:ai-tool:composer",
            on_behalf_of: Some("actor:operator:local"),
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local",
            parent_envelope_ref: "envelope:ai-tool:0001",
            parent_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            parent_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            parent_secret_scope: M5SecretScope::HandleOnlyDelegated,
            parent_root: "workspace://project/src",
            parent_root_kind: M5AllowedScopeKind::FilesystemRoot,
            parent_root_access: M5ScopeAccessMode::ReadWrite,
            child_caps: &[M5CapabilityClass::ReadWorkspace],
            child_target: "ai://project/composer/sub-tool-3",
            child_off_device: false,
            child_scope_label: "workspace://project/src/lib",
            child_scope_kind: M5AllowedScopeKind::FilesystemRoot,
            child_scope_access: M5ScopeAccessMode::ReadOnly,
            child_secret_scope: M5SecretScope::NoSecretAccess,
            child_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            handle_ref: "",
            approval_ticket_ref: "ticket:ai-tool:0001",
            enforcement_backend: "seatbelt-sandbox",
            degraded_fallback: M5DegradedFallback::NarrowToSanitizedPreview,
        },
        M5NestedLaunchLane::Debug => LaneSpec {
            actor_class: M5EnvelopeActorClass::HumanOperator,
            actor_ref: "actor:operator:local",
            on_behalf_of: None,
            issuer_class: M5EnvelopeIssuerClass::ApprovalBroker,
            issuer_ref: "issuer:approval-broker:local",
            parent_envelope_ref: "envelope:debug-session:0001",
            parent_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::ProcessSpawn,
                M5CapabilityClass::NetworkEgress,
            ],
            parent_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            parent_secret_scope: M5SecretScope::NoSecretAccess,
            parent_root: "workspace://project/build",
            parent_root_kind: M5AllowedScopeKind::FilesystemRoot,
            parent_root_access: M5ScopeAccessMode::ReadWrite,
            child_caps: &[
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::ProcessSpawn,
            ],
            child_target: "debug://project/debuggee-1",
            child_off_device: false,
            child_scope_label: "workspace://project/build/target",
            child_scope_kind: M5AllowedScopeKind::FilesystemRoot,
            child_scope_access: M5ScopeAccessMode::ReadOnly,
            child_secret_scope: M5SecretScope::NoSecretAccess,
            child_sandbox: M5SandboxProfile::SubprocessIsolatedLocal,
            handle_ref: "",
            approval_ticket_ref: "ticket:debug-session:0001",
            enforcement_backend: "seatbelt-sandbox",
            degraded_fallback: M5DegradedFallback::NarrowToReadOnly,
        },
    }
}

fn parent_snapshot(spec: &LaneSpec) -> M5ParentAuthoritySnapshot {
    M5ParentAuthoritySnapshot {
        parent_envelope_ref: spec.parent_envelope_ref.to_owned(),
        granted_capability_classes: spec.parent_caps.to_vec(),
        allowed_scope: vec![M5AllowedScopeEntry {
            kind: spec.parent_root_kind,
            label: spec.parent_root.to_owned(),
            access: spec.parent_root_access,
        }],
        sandbox_profile: spec.parent_sandbox,
        secret_scope: spec.parent_secret_scope,
        policy_epoch: nominal_epoch(),
        expires_at: "2026-06-10T02:00:00Z".to_owned(),
    }
}

fn child_secret_refs(spec: &LaneSpec) -> Vec<M5SecretHandleRef> {
    if spec.handle_ref.is_empty() {
        Vec::new()
    } else {
        vec![M5SecretHandleRef {
            handle_ref: spec.handle_ref.to_owned(),
            scope: spec.child_secret_scope,
            broker_contract_ref: secret_handle_contract(),
        }]
    }
}

/// Builds a nominal (non-narrowed) child-envelope derivation for one lane.
fn nominal_derivation(lane: M5NestedLaunchLane) -> M5ChildEnvelopeDerivation {
    let spec = lane_spec(lane);
    M5ChildEnvelopeDerivation {
        derivation_id: format!("derivation:{}:0001", lane.as_str()),
        lane,
        actor: M5DerivationActor {
            actor_class: spec.actor_class,
            actor_ref: spec.actor_ref.to_owned(),
            on_behalf_of: spec.on_behalf_of.map(str::to_owned),
        },
        parent: parent_snapshot(&spec),
        child: M5ChildEnvelope {
            envelope_id: format!("envelope:{}-child:0001", lane.as_str()),
            target_identity: spec.child_target.to_owned(),
            off_device: spec.child_off_device,
            identity_verified: true,
            granted_capability_classes: spec.child_caps.to_vec(),
            allowed_scope: vec![M5AllowedScopeEntry {
                kind: spec.child_scope_kind,
                label: spec.child_scope_label.to_owned(),
                access: spec.child_scope_access,
            }],
            secret_handle_refs: child_secret_refs(&spec),
            secret_scope: spec.child_secret_scope,
            sandbox_profile: spec.child_sandbox,
            policy_epoch: nominal_epoch(),
            issued_at: "2026-06-10T00:00:00Z".to_owned(),
            expires_at: "2026-06-10T00:30:00Z".to_owned(),
            ttl_seconds: 1800,
            single_use: true,
        },
        ambient_environment_posture: M5AmbientEnvironmentPosture::AllowlistedHandlesOnly,
        inherits_full_parent_authority: false,
        enforcement_backend: spec.enforcement_backend.to_owned(),
        enforcement_status: M5EnforcementBackendStatus::Enforced,
        audit_lineage: M5DerivationLineage {
            issuer_class: spec.issuer_class,
            issuer_ref: spec.issuer_ref.to_owned(),
            parent_envelope_ref: spec.parent_envelope_ref.to_owned(),
            approval_ticket_ref: spec.approval_ticket_ref.to_owned(),
            decision_chain: vec![
                "policy-epoch:m5:0007".to_owned(),
                spec.parent_envelope_ref.to_owned(),
                spec.issuer_ref.to_owned(),
                format!("envelope:{}-child:0001", lane.as_str()),
            ],
            self_issued_by_executor: false,
        },
        degraded_fallback: spec.degraded_fallback,
        applied_downgrade_triggers: Vec::new(),
        applied_narrowings: Vec::new(),
        narrowed_below_baseline: false,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

/// Builds a narrowed child-envelope derivation for one lane: a downgrade trigger
/// has tightened the child to a fail-closed, secretless profile.
fn narrowed_derivation(lane: M5NestedLaunchLane) -> M5ChildEnvelopeDerivation {
    let mut derivation = nominal_derivation(lane);
    derivation.derivation_id = format!("derivation:{}:narrowed:0001", lane.as_str());
    // The enforcement backend could not honor the child profile, so the child
    // narrows to a strictly inert, secretless, environment-free envelope.
    derivation.child.sandbox_profile = M5SandboxProfile::InertNoExecution;
    derivation.child.secret_scope = M5SecretScope::NoSecretAccess;
    derivation.child.secret_handle_refs = Vec::new();
    derivation.child.granted_capability_classes = derivation
        .child
        .granted_capability_classes
        .iter()
        .copied()
        .filter(|cap| !cap.requires_secret_scope())
        .collect();
    if derivation.child.granted_capability_classes.is_empty() {
        derivation.child.granted_capability_classes = vec![M5CapabilityClass::ReadWorkspace];
    }
    derivation.ambient_environment_posture = M5AmbientEnvironmentPosture::NoEnvironmentInherited;
    derivation.enforcement_status = M5EnforcementBackendStatus::NarrowedToStricterProfile;
    derivation.degraded_fallback = M5DegradedFallback::FailClosedBlock;
    derivation.applied_downgrade_triggers = vec![
        M5RuntimeAuthorityDowngradeTrigger::EnforcementBackendMissing,
        M5RuntimeAuthorityDowngradeTrigger::SandboxProfileUnavailable,
    ];
    derivation.applied_narrowings = vec![
        M5DerivationNarrowingDimension::SandboxTightened,
        M5DerivationNarrowingDimension::CapabilityDropped,
        M5DerivationNarrowingDimension::SecretScopeNarrowed,
        M5DerivationNarrowingDimension::EnvironmentStripped,
    ];
    derivation.narrowed_below_baseline = true;
    derivation
}

/// Every lane's nominal derivation, in declaration order.
pub fn nominal_derivations() -> Vec<M5ChildEnvelopeDerivation> {
    M5NestedLaunchLane::ALL
        .into_iter()
        .map(nominal_derivation)
        .collect()
}

/// Every lane's narrowed (downgrade-driven) derivation, in declaration order.
pub fn narrowed_derivations() -> Vec<M5ChildEnvelopeDerivation> {
    M5NestedLaunchLane::ALL
        .into_iter()
        .map(narrowed_derivation)
        .collect()
}

/// Builds a child-envelope derivation packet from a derivation set.
///
/// Shared by [`frozen_stable_m5_child_envelope_derivation_packet`] and the
/// fixture dumper so every packet carries identical trust, consumer, freshness,
/// and source-contract blocks.
pub fn build_derivation_packet(
    packet_id: &str,
    packet_label: &str,
    derivations: Vec<M5ChildEnvelopeDerivation>,
) -> M5ChildEnvelopeDerivationPacket {
    M5ChildEnvelopeDerivationPacket::new(M5ChildEnvelopeDerivationPacketInput {
        packet_id: packet_id.to_owned(),
        packet_label: packet_label.to_owned(),
        derivations,
        trust_review: M5ChildEnvelopeDerivationTrustReview {
            no_ambient_machine_privilege: true,
            no_raw_os_environment_inheritance: true,
            child_envelopes_only_narrow_parent: true,
            no_full_parent_authority_fan_out: true,
            secret_projection_handle_only_no_raw_material: true,
            no_self_issued_authority_by_helpers: true,
            unsupported_backend_fails_closed_or_visible: true,
            parent_and_child_identity_inspectable: true,
            policy_epoch_and_expiry_inspectable: true,
            audit_lineage_inspectable: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5ChildEnvelopeDerivationConsumerProjection {
            desktop_shows_derivation: true,
            command_and_policy_reference_same_derivations: true,
            cli_headless_shows_derivation: true,
            support_export_shows_derivation: true,
            diagnostics_shows_derivation: true,
            help_about_shows_derivation_summary: true,
            release_evidence_consumes_derivations: true,
            remote_and_browser_preserve_derivation_semantics: true,
        },
        proof_freshness: M5ChildEnvelopeDerivationProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF.to_owned(),
            M5_CHILD_ENVELOPE_DERIVATION_DOC_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_SCHEMA_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

/// Builds the canonical frozen stable M5 child-envelope derivation packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_CHILD_ENVELOPE_DERIVATION_ARTIFACT_REF`]; the derivation dumper emits
/// this packet and a test asserts the checked-in artifact deserializes back to
/// it unchanged. It pairs a nominal derivation for every lane with a
/// downgrade-narrowed derivation for the untrusted AI lane and the debug lane to
/// exercise the fail-closed narrowing path.
pub fn frozen_stable_m5_child_envelope_derivation_packet() -> M5ChildEnvelopeDerivationPacket {
    let mut derivations = nominal_derivations();
    derivations.push(narrowed_derivation(M5NestedLaunchLane::Ai));
    derivations.push(narrowed_derivation(M5NestedLaunchLane::Debug));
    build_derivation_packet(
        M5_CHILD_ENVELOPE_DERIVATION_PACKET_ID,
        "M5 Child-Envelope Derivations",
        derivations,
    )
}

/// Reads and validates the checked-in stable M5 child-envelope derivation export.
pub fn current_stable_m5_child_envelope_derivation_export(
) -> Result<M5ChildEnvelopeDerivationPacket, M5ChildEnvelopeDerivationArtifactError> {
    let packet: M5ChildEnvelopeDerivationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/support_export.json"
    )))
    .map_err(M5ChildEnvelopeDerivationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChildEnvelopeDerivationArtifactError::Validation(
            violations,
        ))
    }
}
