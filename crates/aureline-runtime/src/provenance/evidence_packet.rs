//! Runtime evidence packets carrying execution-context provenance.
//!
//! Every claimed beta runtime lane (task events, test attempts, debug
//! sessions, and free-form runtime evidence) emits artefacts that need to
//! travel through support, incident, and replay flows long after the live
//! surface is gone. This module declares one shared
//! [`RuntimeEvidencePacket`] that binds the lane's subject reference (task
//! event id, test attempt id, debug session id, or runtime evidence id) to
//! the redaction-safe [`super::ExecutionEventProvenance`] projection that
//! describes the canonical execution context, target identity, toolchain
//! lineage, capsule, trust posture, and policy epoch that produced it.
//!
//! The packet is intentionally compact: it is meant to live inside other
//! support exports (debug, test runner, task event, trace replay) and inside
//! the standalone [`RuntimeEvidencePacketSupportExport`] bundle without
//! re-deriving target truth from logs. The redaction class is pinned and
//! the included reconstruction-fields list quotes the keys a support reader
//! needs to join the packet back to source truth.
//!
//! Downstream consumers can ask whether a packet can still be replayed
//! against a freshly resolved context using
//! [`RuntimeEvidencePacket::compare_with_context`]. The comparator emits a
//! [`RuntimeEvidenceReplayComparison`] with a closed compatibility class and
//! a typed reason vocabulary so the comparison rendered in shell, CLI, and
//! support flows can never become a free-form log string.
//!
//! The boundary schema lives at
//! [`/schemas/runtime/evidence_packet.schema.json`](../../../../schemas/runtime/evidence_packet.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/evidence_packets.md`](../../../../docs/runtime/m3/evidence_packets.md).

use serde::{Deserialize, Serialize};

use super::{ExecutionEventProvenance, ExecutionProvenanceRedactionClass};
use crate::execution_context::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass, TargetClass,
    TrustState,
};

/// Schema version stamped on every runtime evidence packet.
pub const RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RuntimeEvidencePacket`].
pub const RUNTIME_EVIDENCE_PACKET_RECORD_KIND: &str = "runtime_evidence_packet_record";

/// Stable record-kind tag for [`RuntimeEvidenceReplayComparison`].
pub const RUNTIME_EVIDENCE_REPLAY_COMPARISON_RECORD_KIND: &str =
    "runtime_evidence_replay_comparison_record";

/// Stable record-kind tag for [`RuntimeEvidencePacketSupportExport`].
pub const RUNTIME_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "runtime_evidence_packet_support_export_record";

/// Closed runtime evidence lane vocabulary. Adding a lane is a vocabulary
/// change that MUST update the canonical schema, the reviewer doc, and the
/// checked-in fixture set together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeEvidenceLane {
    /// Canonical task-event lane (build, terminal-backed task, package,
    /// notebook, and generic task wedges).
    Task,
    /// Test-runner lane (unit, integration, watch, and imported-CI attempts).
    Test,
    /// Debug-supervisor lane (launch, attach, reconnect sessions).
    Debug,
    /// Free-form runtime evidence lane (trace replay bundles, profile
    /// captures, build/profile evidence) that needs the same provenance
    /// envelope.
    Runtime,
}

impl RuntimeEvidenceLane {
    /// Canonical enumeration order.
    pub const ALL: [Self; 4] = [Self::Task, Self::Test, Self::Debug, Self::Runtime];

    /// Stable string token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Runtime => "runtime",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Task => "Task event evidence",
            Self::Test => "Test attempt evidence",
            Self::Debug => "Debug session evidence",
            Self::Runtime => "Runtime evidence",
        }
    }
}

/// Closed evidence-kind vocabulary describing what the packet's
/// `subject_ref` points at on the source lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeEvidenceKind {
    /// Subject is a `task_event_record` id from the canonical task-event stream.
    TaskEvent,
    /// Subject is a `test_attempt_record` packet id.
    TestAttempt,
    /// Subject is a `debug_session_record` snapshot id.
    DebugSession,
    /// Subject is a `runtime_evidence_alpha_packet_record` id from the trace
    /// replay lane.
    RuntimeTraceEvidence,
}

impl RuntimeEvidenceKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskEvent => "task_event",
            Self::TestAttempt => "test_attempt",
            Self::DebugSession => "debug_session",
            Self::RuntimeTraceEvidence => "runtime_trace_evidence",
        }
    }

    /// The lane that owns this evidence kind.
    pub const fn lane(self) -> RuntimeEvidenceLane {
        match self {
            Self::TaskEvent => RuntimeEvidenceLane::Task,
            Self::TestAttempt => RuntimeEvidenceLane::Test,
            Self::DebugSession => RuntimeEvidenceLane::Debug,
            Self::RuntimeTraceEvidence => RuntimeEvidenceLane::Runtime,
        }
    }
}

/// Closed replay-compatibility class returned by the packet comparator.
///
/// Reviewers and support flows quote these tokens directly; new states are a
/// vocabulary change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayCompatibilityClass {
    /// Replay context matches the original on every required dimension. The
    /// packet can be re-played without explicit review.
    CompatibleReplay,
    /// Replay context matches target / toolchain truth but the capsule hash
    /// advanced cleanly (`in_sync`) or the policy epoch advanced cleanly. The
    /// packet can be re-played, with the drift surfaced for disclosure.
    CompatibleMinorDrift,
    /// Replay context resolves a different canonical target id.
    IncompatibleTargetIdChanged,
    /// Replay context resolves a different [`TargetClass`].
    IncompatibleTargetClassChanged,
    /// Replay context resolves a different toolchain identity or class.
    IncompatibleToolchainChanged,
    /// Replay context's environment capsule changed in a non-clean way (drift
    /// state advanced or hash diverged while still in drift).
    IncompatibleCapsuleDrift,
    /// Replay context's policy epoch regressed below the captured packet's
    /// epoch.
    IncompatiblePolicyEpochRegressed,
    /// Replay context's trust state downgraded below the captured packet's
    /// trust state (e.g. `trusted` -> `restricted`).
    IncompatibleTrustStateDowngraded,
    /// The captured packet's redaction posture is unknown to the replay
    /// reader; the comparator refuses to assert compatibility.
    IncompatibleRedactionClass,
    /// The comparator could not classify the replay context. Reviewers must
    /// inspect manually before dispatching a replay.
    UnknownRequiresReview,
}

impl ReplayCompatibilityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibleReplay => "compatible_replay",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleTargetIdChanged => "incompatible_target_id_changed",
            Self::IncompatibleTargetClassChanged => "incompatible_target_class_changed",
            Self::IncompatibleToolchainChanged => "incompatible_toolchain_changed",
            Self::IncompatibleCapsuleDrift => "incompatible_capsule_drift",
            Self::IncompatiblePolicyEpochRegressed => "incompatible_policy_epoch_regressed",
            Self::IncompatibleTrustStateDowngraded => "incompatible_trust_state_downgraded",
            Self::IncompatibleRedactionClass => "incompatible_redaction_class",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// True when the comparator authorises a replay without explicit reviewer
    /// approval. `compatible_minor_drift` still authorises replay because the
    /// drift is benign (clean capsule / policy advance); reviewers see the
    /// reason vocabulary for disclosure.
    pub const fn permits_replay_without_review(self) -> bool {
        matches!(self, Self::CompatibleReplay | Self::CompatibleMinorDrift)
    }

    /// True when the comparator definitively blocks a replay. The unknown
    /// state is included because dispatch must NOT happen against an
    /// unclassified context.
    pub const fn blocks_replay(self) -> bool {
        !self.permits_replay_without_review()
    }
}

/// Closed vocabulary of replay-incompatibility reasons surfaced alongside the
/// compatibility class. Multiple reasons may apply at once; the class records
/// the highest-priority outcome while every applicable reason is preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayIncompatibilityReason {
    /// Replay target id differs from the captured packet's target id.
    TargetIdDrift,
    /// Replay target class differs from the captured packet's target class.
    TargetClassDrift,
    /// Replay toolchain class differs.
    ToolchainClassDrift,
    /// Replay toolchain id differs (same class, different identity).
    ToolchainIdDrift,
    /// Replay capsule hash differs while drift state is not `in_sync`.
    EnvironmentCapsuleHashDrift,
    /// Replay capsule drift state advanced (e.g. `in_sync` -> `pending_review`).
    EnvironmentCapsuleDriftStateRegressed,
    /// Replay policy epoch regressed below captured packet.
    PolicyEpochRegressed,
    /// Replay trust state downgraded below captured packet.
    TrustStateDowngraded,
    /// Captured packet's redaction class is not `metadata_safe_default` and
    /// cannot be safely replayed by a downstream reader.
    RedactionClassUnsafe,
    /// Capsule advanced cleanly (`in_sync`) or policy epoch advanced cleanly.
    /// Recorded only when the comparator returns `compatible_minor_drift` so
    /// the disclosure remains explicit.
    CleanForwardDrift,
}

impl ReplayIncompatibilityReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetIdDrift => "target_id_drift",
            Self::TargetClassDrift => "target_class_drift",
            Self::ToolchainClassDrift => "toolchain_class_drift",
            Self::ToolchainIdDrift => "toolchain_id_drift",
            Self::EnvironmentCapsuleHashDrift => "environment_capsule_hash_drift",
            Self::EnvironmentCapsuleDriftStateRegressed => {
                "environment_capsule_drift_state_regressed"
            }
            Self::PolicyEpochRegressed => "policy_epoch_regressed",
            Self::TrustStateDowngraded => "trust_state_downgraded",
            Self::RedactionClassUnsafe => "redaction_class_unsafe",
            Self::CleanForwardDrift => "clean_forward_drift",
        }
    }
}

/// One runtime evidence packet binding a lane subject to the canonical
/// execution-context provenance projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidencePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub evidence_packet_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Lane that produced this packet.
    pub lane: RuntimeEvidenceLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Evidence kind owned by the lane.
    pub evidence_kind: RuntimeEvidenceKind,
    /// Stable evidence-kind token.
    pub evidence_kind_token: String,
    /// Opaque reference to the source artefact (task event id, attempt packet
    /// id, debug snapshot id, or runtime evidence packet id).
    pub subject_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Redaction-safe provenance projection of the source context.
    pub context_provenance: ExecutionEventProvenance,
    /// Redaction class for this evidence packet. Only
    /// [`ExecutionProvenanceRedactionClass::MetadataSafeDefault`] is valid.
    pub redaction_class: ExecutionProvenanceRedactionClass,
    /// Stable redaction-class token.
    pub redaction_class_token: String,
    /// Always true because the packet omits raw env, command lines, paths,
    /// and secrets.
    pub redaction_safe: bool,
    /// Reviewer-facing summary line.
    pub summary: String,
    /// Field names a support reader needs to join this packet back to source
    /// truth.
    pub reconstruction_fields: Vec<String>,
}

impl RuntimeEvidencePacket {
    /// Builds a packet from an existing context-provenance projection.
    pub fn new(
        evidence_packet_id: impl Into<String>,
        workspace_id: impl Into<String>,
        evidence_kind: RuntimeEvidenceKind,
        subject_ref: impl Into<String>,
        captured_at: impl Into<String>,
        context_provenance: ExecutionEventProvenance,
    ) -> Self {
        let lane = evidence_kind.lane();
        let subject_ref = subject_ref.into();
        let captured_at = captured_at.into();
        let summary = format!(
            "lane={}; kind={}; subject={}; {}",
            lane.as_str(),
            evidence_kind.as_str(),
            subject_ref,
            context_provenance.summary_line(),
        );
        let redaction_class = context_provenance.redaction_class;
        let redaction_safe = context_provenance.redaction_safe;
        Self {
            record_kind: RUNTIME_EVIDENCE_PACKET_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION,
            evidence_packet_id: evidence_packet_id.into(),
            workspace_id: workspace_id.into(),
            lane,
            lane_token: lane.as_str().to_owned(),
            evidence_kind,
            evidence_kind_token: evidence_kind.as_str().to_owned(),
            subject_ref,
            captured_at,
            context_provenance,
            redaction_class,
            redaction_class_token: redaction_class.as_str().to_owned(),
            redaction_safe,
            summary,
            reconstruction_fields: vec![
                "evidence_packet_id".to_owned(),
                "workspace_id".to_owned(),
                "lane_token".to_owned(),
                "evidence_kind_token".to_owned(),
                "subject_ref".to_owned(),
                "context_provenance.execution_context_ref".to_owned(),
                "context_provenance.provenance_record_ref".to_owned(),
                "context_provenance.target_id".to_owned(),
                "context_provenance.policy_epoch".to_owned(),
                "context_provenance.environment_capsule_ref".to_owned(),
            ],
        }
    }

    /// Builds a packet by projecting provenance directly from a canonical
    /// execution context.
    pub fn from_context(
        evidence_packet_id: impl Into<String>,
        workspace_id: impl Into<String>,
        evidence_kind: RuntimeEvidenceKind,
        subject_ref: impl Into<String>,
        captured_at: impl Into<String>,
        context: &ExecutionContext,
    ) -> Self {
        Self::new(
            evidence_packet_id,
            workspace_id,
            evidence_kind,
            subject_ref,
            captured_at,
            ExecutionEventProvenance::from_context(context),
        )
    }

    /// Compares this packet against a freshly resolved replay context and
    /// emits a typed compatibility decision.
    pub fn compare_with_context(
        &self,
        comparison_id: impl Into<String>,
        replay_context: &ExecutionContext,
        evaluated_at: impl Into<String>,
    ) -> RuntimeEvidenceReplayComparison {
        let replay_projection = ExecutionEventProvenance::from_context(replay_context);
        self.compare_with_provenance(comparison_id, &replay_projection, evaluated_at)
    }

    /// Compares this packet against another redaction-safe provenance
    /// projection and emits a typed compatibility decision.
    pub fn compare_with_provenance(
        &self,
        comparison_id: impl Into<String>,
        replay_context_provenance: &ExecutionEventProvenance,
        evaluated_at: impl Into<String>,
    ) -> RuntimeEvidenceReplayComparison {
        let mut reasons: Vec<ReplayIncompatibilityReason> = Vec::new();
        let original = &self.context_provenance;
        let replay = replay_context_provenance;

        if !matches!(
            original.redaction_class,
            ExecutionProvenanceRedactionClass::MetadataSafeDefault,
        ) || !original.redaction_safe
        {
            reasons.push(ReplayIncompatibilityReason::RedactionClassUnsafe);
        }
        if original.target_id != replay.target_id {
            reasons.push(ReplayIncompatibilityReason::TargetIdDrift);
        }
        if original.target_class != replay.target_class {
            reasons.push(ReplayIncompatibilityReason::TargetClassDrift);
        }
        if original.toolchain_class != replay.toolchain_class {
            reasons.push(ReplayIncompatibilityReason::ToolchainClassDrift);
        }
        if original.toolchain_id != replay.toolchain_id {
            reasons.push(ReplayIncompatibilityReason::ToolchainIdDrift);
        }
        let drift_state_regressed = drift_state_advanced(
            original.environment_capsule_drift_token.as_str(),
            replay.environment_capsule_drift_token.as_str(),
        );
        let capsule_hash_diverged =
            original.environment_capsule_hash != replay.environment_capsule_hash;
        if drift_state_regressed {
            reasons.push(ReplayIncompatibilityReason::EnvironmentCapsuleDriftStateRegressed);
        }
        if capsule_hash_diverged
            && !is_in_sync(&original.environment_capsule_drift_token)
            && !is_in_sync(&replay.environment_capsule_drift_token)
        {
            reasons.push(ReplayIncompatibilityReason::EnvironmentCapsuleHashDrift);
        }
        if replay.policy_epoch < original.policy_epoch {
            reasons.push(ReplayIncompatibilityReason::PolicyEpochRegressed);
        }
        if trust_state_downgraded(original.trust_state, replay.trust_state) {
            reasons.push(ReplayIncompatibilityReason::TrustStateDowngraded);
        }

        let capsule_clean_forward = capsule_hash_diverged
            && is_in_sync(&original.environment_capsule_drift_token)
            && is_in_sync(&replay.environment_capsule_drift_token);
        let policy_clean_forward = replay.policy_epoch > original.policy_epoch;

        let class = if reasons
            .iter()
            .any(|r| matches!(r, ReplayIncompatibilityReason::RedactionClassUnsafe))
        {
            ReplayCompatibilityClass::IncompatibleRedactionClass
        } else if reasons.iter().any(|r| {
            matches!(
                r,
                ReplayIncompatibilityReason::TargetIdDrift
                    | ReplayIncompatibilityReason::TargetClassDrift,
            )
        }) {
            if reasons
                .iter()
                .any(|r| matches!(r, ReplayIncompatibilityReason::TargetClassDrift))
            {
                ReplayCompatibilityClass::IncompatibleTargetClassChanged
            } else {
                ReplayCompatibilityClass::IncompatibleTargetIdChanged
            }
        } else if reasons.iter().any(|r| {
            matches!(
                r,
                ReplayIncompatibilityReason::ToolchainClassDrift
                    | ReplayIncompatibilityReason::ToolchainIdDrift,
            )
        }) {
            ReplayCompatibilityClass::IncompatibleToolchainChanged
        } else if reasons.iter().any(|r| {
            matches!(
                r,
                ReplayIncompatibilityReason::EnvironmentCapsuleHashDrift
                    | ReplayIncompatibilityReason::EnvironmentCapsuleDriftStateRegressed,
            )
        }) {
            ReplayCompatibilityClass::IncompatibleCapsuleDrift
        } else if reasons
            .iter()
            .any(|r| matches!(r, ReplayIncompatibilityReason::PolicyEpochRegressed))
        {
            ReplayCompatibilityClass::IncompatiblePolicyEpochRegressed
        } else if reasons
            .iter()
            .any(|r| matches!(r, ReplayIncompatibilityReason::TrustStateDowngraded))
        {
            ReplayCompatibilityClass::IncompatibleTrustStateDowngraded
        } else if capsule_clean_forward || policy_clean_forward {
            reasons.push(ReplayIncompatibilityReason::CleanForwardDrift);
            ReplayCompatibilityClass::CompatibleMinorDrift
        } else {
            ReplayCompatibilityClass::CompatibleReplay
        };

        let reason_tokens = reasons.iter().map(|r| r.as_str().to_owned()).collect();
        let summary = format!(
            "packet={}; class={}; original_target={}; replay_target={}; replay_policy_epoch={}",
            self.evidence_packet_id,
            class.as_str(),
            original.target_id,
            replay.target_id,
            replay.policy_epoch,
        );
        RuntimeEvidenceReplayComparison {
            record_kind: RUNTIME_EVIDENCE_REPLAY_COMPARISON_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION,
            comparison_id: comparison_id.into(),
            evidence_packet_ref: self.evidence_packet_id.clone(),
            original_context_provenance_id: original.context_provenance_id.clone(),
            replay_execution_context_ref: replay.execution_context_ref.clone(),
            replay_provenance_record_ref: replay.provenance_record_ref.clone(),
            evaluated_at: evaluated_at.into(),
            compatibility: class,
            compatibility_token: class.as_str().to_owned(),
            permits_replay_without_review: class.permits_replay_without_review(),
            blocks_replay: class.blocks_replay(),
            incompatibility_reasons: reasons,
            incompatibility_reason_tokens: reason_tokens,
            summary,
        }
    }
}

/// One redaction-safe replay-compatibility comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidenceReplayComparison {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable comparison id.
    pub comparison_id: String,
    /// Evidence packet this comparison refers to.
    pub evidence_packet_ref: String,
    /// Original packet's context-provenance projection id.
    pub original_context_provenance_id: String,
    /// Replay execution-context id.
    pub replay_execution_context_ref: String,
    /// Replay provenance record id.
    pub replay_provenance_record_ref: String,
    /// Comparison timestamp.
    pub evaluated_at: String,
    /// Compatibility class.
    pub compatibility: ReplayCompatibilityClass,
    /// Stable compatibility-class token.
    pub compatibility_token: String,
    /// True when the comparator authorises replay without review.
    pub permits_replay_without_review: bool,
    /// True when the comparator definitively blocks replay.
    pub blocks_replay: bool,
    /// Typed incompatibility reasons accumulated by the comparator.
    pub incompatibility_reasons: Vec<ReplayIncompatibilityReason>,
    /// Stable incompatibility-reason tokens.
    pub incompatibility_reason_tokens: Vec<String>,
    /// Reviewer-facing summary line.
    pub summary: String,
}

/// Support-export bundle binding evidence packets and replay comparisons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidencePacketSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Evidence packets bundled in the export, in canonical order.
    pub packets: Vec<RuntimeEvidencePacket>,
    /// Replay-compatibility comparisons bundled alongside the packets.
    pub comparisons: Vec<RuntimeEvidenceReplayComparison>,
    /// Redaction class. Only
    /// [`ExecutionProvenanceRedactionClass::MetadataSafeDefault`] is valid.
    pub redaction_class: ExecutionProvenanceRedactionClass,
    /// Stable redaction-class token.
    pub redaction_class_token: String,
    /// Always true because no bundled packet may relax the redaction posture.
    pub redaction_safe: bool,
    /// True when any bundled comparison blocks replay.
    pub any_comparison_blocks_replay: bool,
    /// True when any bundled comparison reports clean forward drift only.
    pub any_minor_drift: bool,
    /// Reviewer-facing summary lines (one per packet).
    pub summary_lines: Vec<String>,
}

impl RuntimeEvidencePacketSupportExport {
    /// Build a support-export bundle for a workspace's evidence packets.
    pub fn new(
        support_export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        packets: Vec<RuntimeEvidencePacket>,
        comparisons: Vec<RuntimeEvidenceReplayComparison>,
    ) -> Self {
        let summary_lines = packets.iter().map(|p| p.summary.clone()).collect();
        let any_comparison_blocks_replay = comparisons.iter().any(|c| c.blocks_replay);
        let any_minor_drift = comparisons.iter().any(|c| {
            matches!(c.compatibility, ReplayCompatibilityClass::CompatibleMinorDrift)
        });
        let redaction_safe = packets.iter().all(|p| {
            p.redaction_safe
                && matches!(
                    p.redaction_class,
                    ExecutionProvenanceRedactionClass::MetadataSafeDefault
                )
        });
        Self {
            record_kind: RUNTIME_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            packets,
            comparisons,
            redaction_class: ExecutionProvenanceRedactionClass::MetadataSafeDefault,
            redaction_class_token: ExecutionProvenanceRedactionClass::MetadataSafeDefault
                .as_str()
                .to_owned(),
            redaction_safe,
            any_comparison_blocks_replay,
            any_minor_drift,
            summary_lines,
        }
    }

    /// Returns true when the export shares one execution-context provenance
    /// id across every bundled packet (useful for join checks).
    pub fn shares_single_context_provenance(&self) -> bool {
        let mut iter = self
            .packets
            .iter()
            .map(|p| p.context_provenance.context_provenance_id.as_str());
        match iter.next() {
            None => true,
            Some(first) => iter.all(|id| id == first),
        }
    }

    /// Deterministic reviewer plaintext form. The body iterates packets and
    /// comparisons in declared order so the support clipboard, the CLI, and
    /// the shell panel agree byte-for-byte.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Runtime evidence support export: {}\n",
            self.support_export_id
        ));
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!(
            "Redaction: {} (safe={})\n",
            self.redaction_class_token, self.redaction_safe
        ));
        out.push_str(&format!(
            "Packets: {} (blocks_replay={}, minor_drift={})\n",
            self.packets.len(),
            self.any_comparison_blocks_replay,
            self.any_minor_drift
        ));
        for packet in &self.packets {
            out.push_str(&format!(
                "\n[{}] {} ({})\n",
                packet.lane_token, packet.evidence_kind_token, packet.evidence_packet_id
            ));
            out.push_str(&format!("  subject: {}\n", packet.subject_ref));
            out.push_str(&format!("  captured_at: {}\n", packet.captured_at));
            out.push_str(&format!("  summary: {}\n", packet.summary));
        }
        if !self.comparisons.is_empty() {
            out.push_str("\nReplay comparisons:\n");
            for comparison in &self.comparisons {
                out.push_str(&format!(
                    "  - packet={} class={} permits_replay={} blocks={} reasons=[{}]\n",
                    comparison.evidence_packet_ref,
                    comparison.compatibility_token,
                    comparison.permits_replay_without_review,
                    comparison.blocks_replay,
                    comparison.incompatibility_reason_tokens.join(","),
                ));
            }
        }
        out
    }
}

fn capsule_drift_state_rank(token: &str) -> u8 {
    match token {
        "in_sync" => 0,
        "stale_inputs" => 1,
        "generator_changed" => 2,
        "manually_diverged" => 3,
        "unknown_lineage" => 4,
        // Unrecognised tokens settle at the strictest rank so unclassified
        // states do not silently pass as in-sync.
        _ => u8::MAX,
    }
}

fn drift_state_advanced(original_token: &str, replay_token: &str) -> bool {
    capsule_drift_state_rank(replay_token) > capsule_drift_state_rank(original_token)
}

fn is_in_sync(token: &str) -> bool {
    token == "in_sync"
}

fn trust_state_downgraded(original: TrustState, replay: TrustState) -> bool {
    trust_state_rank(replay) < trust_state_rank(original)
}

fn trust_state_rank(state: TrustState) -> u8 {
    match state {
        TrustState::Restricted => 0,
        TrustState::PendingEvaluation => 1,
        TrustState::Trusted => 2,
    }
}

/// Identifier for the canonical seeded scenarios. Reviewer and partner
/// runs of the seed builder reproduce the same packets byte-for-byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeEvidencePacketSeededScenario {
    /// Local-host task event; the replay context resolves to the same target,
    /// toolchain, and policy epoch. The comparator returns
    /// `compatible_replay`.
    LocalTaskCompatible,
    /// Local-host test attempt; the replay context advances the workspace
    /// policy epoch cleanly. The comparator returns
    /// `compatible_minor_drift`.
    LocalTestPolicyAdvancedClean,
    /// Container debug session; the replay context resolves the same target
    /// but the workspace capsule drifted out of sync. The comparator returns
    /// `incompatible_capsule_drift`.
    ContainerDebugCapsuleDrift,
    /// Managed-workspace runtime evidence; the replay context downgrades the
    /// trust state from `trusted` to `restricted`. The comparator returns
    /// `incompatible_trust_state_downgraded`.
    ManagedRuntimeTrustDowngraded,
}

impl RuntimeEvidencePacketSeededScenario {
    /// All seeded scenarios in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalTaskCompatible,
        Self::LocalTestPolicyAdvancedClean,
        Self::ContainerDebugCapsuleDrift,
        Self::ManagedRuntimeTrustDowngraded,
    ];

    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTaskCompatible => "local_task_compatible",
            Self::LocalTestPolicyAdvancedClean => "local_test_policy_advanced_clean",
            Self::ContainerDebugCapsuleDrift => "container_debug_capsule_drift",
            Self::ManagedRuntimeTrustDowngraded => "managed_runtime_trust_downgraded",
        }
    }

    /// Closed evidence-kind for this scenario.
    pub const fn evidence_kind(self) -> RuntimeEvidenceKind {
        match self {
            Self::LocalTaskCompatible => RuntimeEvidenceKind::TaskEvent,
            Self::LocalTestPolicyAdvancedClean => RuntimeEvidenceKind::TestAttempt,
            Self::ContainerDebugCapsuleDrift => RuntimeEvidenceKind::DebugSession,
            Self::ManagedRuntimeTrustDowngraded => RuntimeEvidenceKind::RuntimeTraceEvidence,
        }
    }
}

fn seeded_resolver(policy_epoch: u64, drift_state: CapsuleDriftState, capsule_hash: &str) -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "ws-evidence-packet-beta".to_owned(),
        profile_id: Some("prof.evidence-packet-beta".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:evidence-packet-beta:seed".to_owned(),
            capsule_hash: capsule_hash.to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state,
        },
        resolver_version: "evidence-packet-beta-0".to_owned(),
    })
}

/// Build the canonical seed packet, replay context, and comparison for one
/// scenario. Reviewer and partner runs of the seed builder reproduce the
/// same outputs byte-for-byte.
pub fn seeded_runtime_evidence_packet(
    scenario: RuntimeEvidencePacketSeededScenario,
) -> (RuntimeEvidencePacket, RuntimeEvidenceReplayComparison) {
    use RuntimeEvidencePacketSeededScenario::*;
    match scenario {
        LocalTaskCompatible => {
            let mut original = seeded_resolver(1, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let original_context = original.resolve(ExecutionContextRequest::task_seed(
                "task.run",
                TrustState::Trusted,
                "2026-05-15T19:00:00Z",
            ));
            let packet = RuntimeEvidencePacket::from_context(
                format!("evpkt:{}", scenario.as_str()),
                "ws-evidence-packet-beta",
                scenario.evidence_kind(),
                "task:event:1",
                "2026-05-15T19:00:00Z",
                &original_context,
            );
            let mut replay = seeded_resolver(1, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let replay_context = replay.resolve(ExecutionContextRequest::task_seed(
                "task.run",
                TrustState::Trusted,
                "2026-05-15T19:01:00Z",
            ));
            let comparison = packet.compare_with_context(
                format!("evcmp:{}", scenario.as_str()),
                &replay_context,
                "2026-05-15T19:01:00Z",
            );
            (packet, comparison)
        }
        LocalTestPolicyAdvancedClean => {
            let mut original = seeded_resolver(2, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let original_context = original.resolve(ExecutionContextRequest::test_seed(
                "test.run",
                TrustState::Trusted,
                "2026-05-15T19:00:00Z",
            ));
            let packet = RuntimeEvidencePacket::from_context(
                format!("evpkt:{}", scenario.as_str()),
                "ws-evidence-packet-beta",
                scenario.evidence_kind(),
                "test:attempt:1",
                "2026-05-15T19:00:00Z",
                &original_context,
            );
            let mut replay = seeded_resolver(3, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let replay_context = replay.resolve(ExecutionContextRequest::test_seed(
                "test.run",
                TrustState::Trusted,
                "2026-05-15T19:01:00Z",
            ));
            let comparison = packet.compare_with_context(
                format!("evcmp:{}", scenario.as_str()),
                &replay_context,
                "2026-05-15T19:01:00Z",
            );
            (packet, comparison)
        }
        ContainerDebugCapsuleDrift => {
            let mut original = seeded_resolver(2, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let original_context = original.resolve(ExecutionContextRequest::container_task_seed(
                "debug.launch",
                TargetClass::Devcontainer,
                TrustState::Trusted,
                "2026-05-15T19:00:00Z",
            ));
            let packet = RuntimeEvidencePacket::from_context(
                format!("evpkt:{}", scenario.as_str()),
                "ws-evidence-packet-beta",
                scenario.evidence_kind(),
                "debug:session:1",
                "2026-05-15T19:00:00Z",
                &original_context,
            );
            let mut replay = seeded_resolver(
                2,
                CapsuleDriftState::ManuallyDiverged,
                "sha256:caps-drifted",
            );
            let replay_context = replay.resolve(ExecutionContextRequest::container_task_seed(
                "debug.launch",
                TargetClass::Devcontainer,
                TrustState::Trusted,
                "2026-05-15T19:01:00Z",
            ));
            let comparison = packet.compare_with_context(
                format!("evcmp:{}", scenario.as_str()),
                &replay_context,
                "2026-05-15T19:01:00Z",
            );
            (packet, comparison)
        }
        ManagedRuntimeTrustDowngraded => {
            let mut original = seeded_resolver(4, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let original_context = original.resolve(ExecutionContextRequest::request_workspace_task_seed(
                "runtime.evidence.replay",
                TargetClass::ManagedWorkspace,
                TrustState::Trusted,
                "2026-05-15T19:00:00Z",
            ));
            let packet = RuntimeEvidencePacket::from_context(
                format!("evpkt:{}", scenario.as_str()),
                "ws-evidence-packet-beta",
                scenario.evidence_kind(),
                "runtime:evidence:1",
                "2026-05-15T19:00:00Z",
                &original_context,
            );
            let mut replay = seeded_resolver(4, CapsuleDriftState::InSync, "sha256:caps-baseline");
            let replay_context = replay.resolve(ExecutionContextRequest::request_workspace_task_seed(
                "runtime.evidence.replay",
                TargetClass::ManagedWorkspace,
                TrustState::Restricted,
                "2026-05-15T19:01:00Z",
            ));
            let comparison = packet.compare_with_context(
                format!("evcmp:{}", scenario.as_str()),
                &replay_context,
                "2026-05-15T19:01:00Z",
            );
            (packet, comparison)
        }
    }
}

/// Build the canonical support-export bundle covering every seeded scenario.
pub fn seeded_runtime_evidence_packet_support_export(
    support_export_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> RuntimeEvidencePacketSupportExport {
    let mut packets = Vec::with_capacity(RuntimeEvidencePacketSeededScenario::ALL.len());
    let mut comparisons = Vec::with_capacity(RuntimeEvidencePacketSeededScenario::ALL.len());
    for scenario in RuntimeEvidencePacketSeededScenario::ALL {
        let (packet, comparison) = seeded_runtime_evidence_packet(scenario);
        packets.push(packet);
        comparisons.push(comparison);
    }
    RuntimeEvidencePacketSupportExport::new(
        support_export_id,
        "ws-evidence-packet-beta",
        generated_at,
        packets,
        comparisons,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_matches_underlying_provenance_schema() {
        assert_eq!(
            RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION,
            super::super::EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION
        );
    }

    #[test]
    fn local_task_scenario_replays_cleanly() {
        let (packet, comparison) =
            seeded_runtime_evidence_packet(RuntimeEvidencePacketSeededScenario::LocalTaskCompatible);
        assert_eq!(packet.lane, RuntimeEvidenceLane::Task);
        assert_eq!(packet.evidence_kind, RuntimeEvidenceKind::TaskEvent);
        assert!(packet.redaction_safe);
        assert_eq!(
            comparison.compatibility,
            ReplayCompatibilityClass::CompatibleReplay
        );
        assert!(comparison.permits_replay_without_review);
        assert!(comparison.incompatibility_reasons.is_empty());
    }

    #[test]
    fn policy_advanced_clean_settles_to_minor_drift() {
        let (_, comparison) = seeded_runtime_evidence_packet(
            RuntimeEvidencePacketSeededScenario::LocalTestPolicyAdvancedClean,
        );
        assert_eq!(
            comparison.compatibility,
            ReplayCompatibilityClass::CompatibleMinorDrift
        );
        assert!(comparison.permits_replay_without_review);
        assert!(comparison
            .incompatibility_reasons
            .contains(&ReplayIncompatibilityReason::CleanForwardDrift));
    }

    #[test]
    fn capsule_drift_blocks_replay() {
        let (_, comparison) = seeded_runtime_evidence_packet(
            RuntimeEvidencePacketSeededScenario::ContainerDebugCapsuleDrift,
        );
        assert_eq!(
            comparison.compatibility,
            ReplayCompatibilityClass::IncompatibleCapsuleDrift
        );
        assert!(!comparison.permits_replay_without_review);
        assert!(comparison.blocks_replay);
        assert!(comparison
            .incompatibility_reasons
            .contains(&ReplayIncompatibilityReason::EnvironmentCapsuleDriftStateRegressed));
    }

    #[test]
    fn trust_downgrade_blocks_replay() {
        let (_, comparison) = seeded_runtime_evidence_packet(
            RuntimeEvidencePacketSeededScenario::ManagedRuntimeTrustDowngraded,
        );
        assert_eq!(
            comparison.compatibility,
            ReplayCompatibilityClass::IncompatibleTrustStateDowngraded
        );
        assert!(comparison.blocks_replay);
        assert!(comparison
            .incompatibility_reasons
            .contains(&ReplayIncompatibilityReason::TrustStateDowngraded));
    }

    #[test]
    fn support_export_bundles_every_scenario() {
        let export = seeded_runtime_evidence_packet_support_export(
            "evpkt-support:test",
            "2026-05-15T19:02:00Z",
        );
        assert_eq!(
            export.packets.len(),
            RuntimeEvidencePacketSeededScenario::ALL.len()
        );
        assert_eq!(
            export.comparisons.len(),
            RuntimeEvidencePacketSeededScenario::ALL.len()
        );
        assert!(export.redaction_safe);
        assert!(export.any_comparison_blocks_replay);
        assert!(export.any_minor_drift);
        let plaintext = export.render_plaintext();
        assert!(plaintext.contains("Runtime evidence support export"));
        assert!(plaintext.contains("incompatible_capsule_drift"));
        assert!(plaintext.contains("incompatible_trust_state_downgraded"));
        assert!(plaintext.contains("compatible_minor_drift"));
    }

    #[test]
    fn support_export_redaction_class_is_pinned_and_no_secret_markers_leak() {
        let export = seeded_runtime_evidence_packet_support_export(
            "evpkt-support:redaction",
            "2026-05-15T19:02:00Z",
        );
        assert!(matches!(
            export.redaction_class,
            ExecutionProvenanceRedactionClass::MetadataSafeDefault
        ));
        assert!(export.redaction_safe);
        for packet in &export.packets {
            assert!(matches!(
                packet.redaction_class,
                ExecutionProvenanceRedactionClass::MetadataSafeDefault
            ));
            assert!(packet.redaction_safe);
            assert!(packet
                .context_provenance
                .working_directory_digest
                .is_some());
        }
        let json = serde_json::to_string(&export).expect("serialize export");
        // The projection never copies raw env bodies, raw command lines, or
        // unmanaged credential markers; assert the well-known markers stay
        // out of the export.
        assert!(!json.contains("BEARER"));
        assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
        assert!(!json.contains("SSH_PRIVATE_KEY"));
        assert!(!json.contains("LD_LIBRARY_PATH"));
    }

    #[test]
    fn seeded_outputs_are_deterministic() {
        let first = serde_json::to_string(&seeded_runtime_evidence_packet_support_export(
            "evpkt-support:deterministic",
            "2026-05-15T19:02:00Z",
        ))
        .expect("first");
        let second = serde_json::to_string(&seeded_runtime_evidence_packet_support_export(
            "evpkt-support:deterministic",
            "2026-05-15T19:02:00Z",
        ))
        .expect("second");
        assert_eq!(first, second);
    }

    #[test]
    fn compare_against_self_is_compatible_replay() {
        let mut resolver = seeded_resolver(1, CapsuleDriftState::InSync, "sha256:caps-baseline");
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "2026-05-15T19:00:00Z",
        ));
        let packet = RuntimeEvidencePacket::from_context(
            "evpkt:self",
            "ws-test",
            RuntimeEvidenceKind::TaskEvent,
            "task:event:self",
            "2026-05-15T19:00:00Z",
            &context,
        );
        let comparison = packet.compare_with_context(
            "evcmp:self",
            &context,
            "2026-05-15T19:00:01Z",
        );
        assert_eq!(
            comparison.compatibility,
            ReplayCompatibilityClass::CompatibleReplay
        );
        assert!(comparison.permits_replay_without_review);
        assert!(comparison.incompatibility_reasons.is_empty());
    }
}
