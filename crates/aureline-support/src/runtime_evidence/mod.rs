//! Runtime evidence replay packs for support and partner escalation.
//!
//! A replay pack is the bounded envelope support reviewers, design partners,
//! and incident flows reopen instead of hand-built log bundles. Each pack
//! binds:
//!
//! - one canonical [`RuntimeEvidencePacket`] (task event, test attempt, debug
//!   session, or runtime trace evidence) minted by `aureline-runtime`,
//! - one [`RuntimeEvidenceReplayComparison`] returned by the runtime
//!   comparator,
//! - a closed set of opaque artefact references
//!   ([`ReplayPackArtefactClass`]) — transcript ref, runtime-log ref,
//!   artifact-blob ref, evidence-packet ref, and context-provenance ref —
//!   that point back to bytes living in the source export rather than
//!   embedding raw content,
//! - a closed [`ReplaySubjectPrivilegeClass`] that classifies whether the
//!   captured action was read-only, mutating, or privileged, and
//! - one derived [`ReplayFidelityClass`] (`exact`, `compatible`,
//!   `layout_only`, `evidence_only`) plus a closed
//!   [`ReplayReopenDecisionClass`] (`allow_replay`,
//!   `allow_inspect_no_rerun`, `allow_evidence_only_view`, `blocked`).
//!
//! The fidelity label answers *how truthfully can this pack be reopened*; the
//! reopen decision answers *what is the reviewer authorised to do with it*.
//! Mutating and privileged subjects are never advanced beyond
//! `allow_inspect_no_rerun` even when the comparator reports a fully
//! compatible replay context, so no reopen flow can silently re-run a
//! privileged or mutating action. Comparator results that resolved as
//! `unknown_requires_review` or `incompatible_redaction_class` settle the
//! pack at `blocked`.
//!
//! The boundary schema lives at
//! [`/schemas/runtime/runtime_replay_pack.schema.json`](../../../../schemas/runtime/runtime_replay_pack.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/support/m3/runtime_replay_packets.md`](../../../../docs/support/m3/runtime_replay_packets.md).
//! Closed-vocabulary tokens live alongside the artefacts at
//! [`/artifacts/runtime/m3/replay_packets/closed_vocabularies.yaml`](../../../../artifacts/runtime/m3/replay_packets/closed_vocabularies.yaml).

use std::collections::BTreeSet;

use aureline_runtime::{
    seeded_runtime_evidence_packet, ReplayCompatibilityClass, RuntimeEvidenceKind,
    RuntimeEvidenceLane, RuntimeEvidencePacket, RuntimeEvidencePacketSeededScenario,
    RuntimeEvidenceReplayComparison,
};
use serde::{Deserialize, Serialize};

/// Schema version stamped on every replay pack record.
pub const RUNTIME_REPLAY_PACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RuntimeReplayPack`].
pub const RUNTIME_REPLAY_PACK_RECORD_KIND: &str = "runtime_replay_pack_record";

/// Stable record-kind tag for [`RuntimeReplayPackSupportExport`].
pub const RUNTIME_REPLAY_PACK_SUPPORT_EXPORT_RECORD_KIND: &str =
    "runtime_replay_pack_support_export_record";

/// Stable record-kind tag for the checked-in fixture case payloads.
pub const RUNTIME_REPLAY_PACK_CASE_RECORD_KIND: &str = "runtime_replay_pack_case";

/// Repository-relative path to the closed-vocabulary artefact bundled
/// alongside this module.
pub const RUNTIME_REPLAY_PACK_CLOSED_VOCABULARIES_PATH: &str =
    "artifacts/runtime/m3/replay_packets/closed_vocabularies.yaml";

/// Closed fidelity vocabulary returned by the replay-pack builder.
///
/// `exact` and `compatible` permit replay (subject to privilege gating).
/// `layout_only` and `evidence_only` never permit replay; reviewers can only
/// inspect the captured artefacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFidelityClass {
    /// Comparator returned `compatible_replay` and the replay target,
    /// toolchain, capsule, policy epoch, and trust state all match.
    /// Transcripts, logs, artefacts, and context can be replayed against
    /// the live target subject to privilege class.
    Exact,
    /// Comparator returned `compatible_minor_drift` (capsule or policy
    /// epoch advanced cleanly). Replay permitted with disclosure.
    Compatible,
    /// Comparator blocked replay on capsule drift, policy-epoch regression,
    /// toolchain identity drift, or trust-state downgrade. The pack still
    /// preserves transcript / log / artefact / context layout for reviewer
    /// inspection but the live target must not be re-fired.
    LayoutOnly,
    /// Comparator blocked replay on target identity, target class,
    /// unsafe redaction class, or returned `unknown_requires_review`.
    /// The pack carries the evidence record only; transcripts/logs/artefacts
    /// MAY render but they cannot be reopened on the live target.
    EvidenceOnly,
}

impl ReplayFidelityClass {
    /// All fidelity classes in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Compatible,
        Self::LayoutOnly,
        Self::EvidenceOnly,
    ];

    /// Stable string token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact replay",
            Self::Compatible => "Compatible replay",
            Self::LayoutOnly => "Layout only",
            Self::EvidenceOnly => "Evidence only",
        }
    }

    /// True when the pack's runtime comparator authorises replay on the live
    /// target without explicit reviewer approval. Mutating and privileged
    /// subjects still downgrade the reopen decision separately.
    pub const fn permits_runtime_replay(self) -> bool {
        matches!(self, Self::Exact | Self::Compatible)
    }

    /// Derive the fidelity class from a runtime comparator outcome.
    pub fn from_comparison(comparison: &RuntimeEvidenceReplayComparison) -> Self {
        match comparison.compatibility {
            ReplayCompatibilityClass::CompatibleReplay => Self::Exact,
            ReplayCompatibilityClass::CompatibleMinorDrift => Self::Compatible,
            ReplayCompatibilityClass::IncompatibleCapsuleDrift
            | ReplayCompatibilityClass::IncompatiblePolicyEpochRegressed
            | ReplayCompatibilityClass::IncompatibleToolchainChanged
            | ReplayCompatibilityClass::IncompatibleTrustStateDowngraded => Self::LayoutOnly,
            ReplayCompatibilityClass::IncompatibleTargetIdChanged
            | ReplayCompatibilityClass::IncompatibleTargetClassChanged
            | ReplayCompatibilityClass::IncompatibleRedactionClass
            | ReplayCompatibilityClass::UnknownRequiresReview => Self::EvidenceOnly,
        }
    }
}

/// Closed privilege vocabulary for the captured subject. The captured action
/// is the action that produced the evidence (e.g. `cargo build`, `pytest`,
/// `debug-launch`, or `request_workspace.send`). Mutating and privileged
/// subjects MUST NOT be silently re-run; the reopen decision is downgraded
/// regardless of replay fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySubjectPrivilegeClass {
    /// The captured action neither mutates state nor requires elevated
    /// credentials (e.g. read-only inspection, status reads, unit tests
    /// against an isolated workspace).
    ReadOnly,
    /// The captured action mutates user-visible state (e.g. mutating
    /// HTTP request, file write, package install, destructive build).
    Mutating,
    /// The captured action mutates state behind an explicit privilege
    /// boundary (e.g. managed-workspace dispatch, credentialed deploy,
    /// admin/root supervisor action).
    Privileged,
}

impl ReplaySubjectPrivilegeClass {
    /// All privilege classes in canonical order.
    pub const ALL: [Self; 3] = [Self::ReadOnly, Self::Mutating, Self::Privileged];

    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::Mutating => "mutating",
            Self::Privileged => "privileged",
        }
    }

    /// True when reopen MUST never silently fire the captured action.
    pub const fn forbids_silent_rerun(self) -> bool {
        matches!(self, Self::Mutating | Self::Privileged)
    }
}

/// Closed reopen-decision vocabulary returned by the replay-pack builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayReopenDecisionClass {
    /// Reviewer may replay the pack on the live target.
    AllowReplay,
    /// Reviewer may reopen the pack to inspect transcripts, logs, artefacts,
    /// and context — but the runtime MUST NOT silently re-fire the captured
    /// action.
    AllowInspectNoRerun,
    /// Reviewer may view the evidence record (lane, target, comparator
    /// outcome, summary). Transcripts and logs MAY render but cannot be
    /// reopened on the live target.
    AllowEvidenceOnlyView,
    /// Reviewer may not open the pack. The original capture must be
    /// re-triaged by a privileged escalation owner before any replay or
    /// reopen flow.
    Blocked,
}

impl ReplayReopenDecisionClass {
    /// All decisions in canonical order.
    pub const ALL: [Self; 4] = [
        Self::AllowReplay,
        Self::AllowInspectNoRerun,
        Self::AllowEvidenceOnlyView,
        Self::Blocked,
    ];

    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowReplay => "allow_replay",
            Self::AllowInspectNoRerun => "allow_inspect_no_rerun",
            Self::AllowEvidenceOnlyView => "allow_evidence_only_view",
            Self::Blocked => "blocked",
        }
    }

    /// True when the decision authorises a live replay against the target.
    pub const fn permits_runtime_replay(self) -> bool {
        matches!(self, Self::AllowReplay)
    }

    /// True when the decision authorises reopening (inspect-only or replay).
    pub const fn permits_reopen(self) -> bool {
        matches!(
            self,
            Self::AllowReplay | Self::AllowInspectNoRerun | Self::AllowEvidenceOnlyView
        )
    }

    /// Derive the reopen decision from fidelity + privilege.
    pub fn derive(
        fidelity: ReplayFidelityClass,
        privilege: ReplaySubjectPrivilegeClass,
        comparison: &RuntimeEvidenceReplayComparison,
    ) -> Self {
        if matches!(
            comparison.compatibility,
            ReplayCompatibilityClass::UnknownRequiresReview
                | ReplayCompatibilityClass::IncompatibleRedactionClass
        ) {
            return Self::Blocked;
        }
        match (fidelity, privilege) {
            (ReplayFidelityClass::Exact, ReplaySubjectPrivilegeClass::ReadOnly)
            | (ReplayFidelityClass::Compatible, ReplaySubjectPrivilegeClass::ReadOnly) => {
                Self::AllowReplay
            }
            (ReplayFidelityClass::Exact, _) | (ReplayFidelityClass::Compatible, _) => {
                Self::AllowInspectNoRerun
            }
            (ReplayFidelityClass::LayoutOnly, _) => Self::AllowInspectNoRerun,
            (ReplayFidelityClass::EvidenceOnly, _) => Self::AllowEvidenceOnlyView,
        }
    }
}

/// Closed artefact-class vocabulary. Every artefact reference is opaque: the
/// pack never embeds raw bytes; the support-export reader joins back to the
/// source corpus through the carried `artefact_ref`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPackArtefactClass {
    /// Terminal transcript (run output, test runner output, debug console).
    TranscriptRef,
    /// Structured runtime log (task event stream slice, supervisor events,
    /// test attempt rows).
    RuntimeLogRef,
    /// Opaque artefact blob (built binary digest, profiling capture digest,
    /// recorded HTTP response digest).
    ArtifactBlobRef,
    /// The originating runtime evidence packet.
    EvidencePacketRef,
    /// The redaction-safe execution-context provenance projection.
    ContextProvenanceRef,
}

impl ReplayPackArtefactClass {
    /// All artefact classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::TranscriptRef,
        Self::RuntimeLogRef,
        Self::ArtifactBlobRef,
        Self::EvidencePacketRef,
        Self::ContextProvenanceRef,
    ];

    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranscriptRef => "transcript_ref",
            Self::RuntimeLogRef => "runtime_log_ref",
            Self::ArtifactBlobRef => "artifact_blob_ref",
            Self::EvidencePacketRef => "evidence_packet_ref",
            Self::ContextProvenanceRef => "context_provenance_ref",
        }
    }
}

/// One opaque artefact reference attached to a replay pack.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReplayPackArtefact {
    /// Artefact class.
    pub artefact_class: ReplayPackArtefactClass,
    /// Stable artefact-class token.
    pub artefact_class_token: String,
    /// Stable, opaque reference (id, hash digest, or URI) that the source
    /// export resolves back to bytes. The pack never carries raw content.
    pub artefact_ref: String,
    /// Reviewer-facing label.
    pub label: String,
}

impl ReplayPackArtefact {
    /// Build a new artefact entry.
    pub fn new(
        artefact_class: ReplayPackArtefactClass,
        artefact_ref: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            artefact_class,
            artefact_class_token: artefact_class.as_str().to_owned(),
            artefact_ref: artefact_ref.into(),
            label: label.into(),
        }
    }
}

/// One runtime replay pack bundling evidence, comparator outcome, artefacts,
/// fidelity, privilege, and a derived reopen decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReplayPack {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable pack id.
    pub replay_pack_id: String,
    /// Workspace id (mirrors the underlying evidence packet).
    pub workspace_id: String,
    /// Pack creation timestamp.
    pub created_at: String,
    /// Underlying runtime evidence packet.
    pub evidence_packet: RuntimeEvidencePacket,
    /// Runtime comparator outcome paired with the evidence packet.
    pub replay_comparison: RuntimeEvidenceReplayComparison,
    /// Captured subject privilege class.
    pub subject_privilege: ReplaySubjectPrivilegeClass,
    /// Stable privilege-class token.
    pub subject_privilege_token: String,
    /// Derived fidelity class.
    pub fidelity: ReplayFidelityClass,
    /// Stable fidelity-class token.
    pub fidelity_token: String,
    /// Derived reopen decision.
    pub reopen_decision: ReplayReopenDecisionClass,
    /// Stable reopen-decision token.
    pub reopen_decision_token: String,
    /// Opaque artefact references attached to this pack.
    pub artefacts: Vec<ReplayPackArtefact>,
    /// True when no live replay is permitted (either by fidelity or privilege).
    pub forbids_live_rerun: bool,
    /// True when the runtime comparator definitively blocks replay.
    pub comparator_blocks_replay: bool,
    /// Reviewer-facing summary line.
    pub summary: String,
}

impl RuntimeReplayPack {
    /// Build a replay pack from the runtime evidence packet, comparator
    /// outcome, privilege class, and artefact references.
    pub fn new(
        replay_pack_id: impl Into<String>,
        created_at: impl Into<String>,
        evidence_packet: RuntimeEvidencePacket,
        replay_comparison: RuntimeEvidenceReplayComparison,
        subject_privilege: ReplaySubjectPrivilegeClass,
        artefacts: Vec<ReplayPackArtefact>,
    ) -> Self {
        let fidelity = ReplayFidelityClass::from_comparison(&replay_comparison);
        let reopen_decision =
            ReplayReopenDecisionClass::derive(fidelity, subject_privilege, &replay_comparison);
        let workspace_id = evidence_packet.workspace_id.clone();
        let comparator_blocks_replay = replay_comparison.blocks_replay;
        let forbids_live_rerun = !reopen_decision.permits_runtime_replay();
        let summary = format!(
            "pack={}; lane={}; kind={}; subject={}; fidelity={}; privilege={}; reopen={}",
            evidence_packet.evidence_packet_id,
            evidence_packet.lane_token,
            evidence_packet.evidence_kind_token,
            evidence_packet.subject_ref,
            fidelity.as_str(),
            subject_privilege.as_str(),
            reopen_decision.as_str(),
        );
        Self {
            record_kind: RUNTIME_REPLAY_PACK_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_REPLAY_PACK_SCHEMA_VERSION,
            replay_pack_id: replay_pack_id.into(),
            workspace_id,
            created_at: created_at.into(),
            evidence_packet,
            replay_comparison,
            subject_privilege,
            subject_privilege_token: subject_privilege.as_str().to_owned(),
            fidelity,
            fidelity_token: fidelity.as_str().to_owned(),
            reopen_decision,
            reopen_decision_token: reopen_decision.as_str().to_owned(),
            artefacts,
            forbids_live_rerun,
            comparator_blocks_replay,
            summary,
        }
    }

    /// Convenience accessor for the underlying lane.
    pub fn lane(&self) -> RuntimeEvidenceLane {
        self.evidence_packet.lane
    }

    /// Convenience accessor for the underlying evidence kind.
    pub fn evidence_kind(&self) -> RuntimeEvidenceKind {
        self.evidence_packet.evidence_kind
    }

    /// True when the pack carries every artefact class except blob refs.
    /// Blob refs are optional because some lanes (e.g. unit tests against an
    /// empty target) produce no artefact blob.
    pub fn covers_required_artefact_classes(&self) -> bool {
        let mut seen: BTreeSet<ReplayPackArtefactClass> = BTreeSet::new();
        for artefact in &self.artefacts {
            seen.insert(artefact.artefact_class);
        }
        seen.contains(&ReplayPackArtefactClass::TranscriptRef)
            && seen.contains(&ReplayPackArtefactClass::RuntimeLogRef)
            && seen.contains(&ReplayPackArtefactClass::EvidencePacketRef)
            && seen.contains(&ReplayPackArtefactClass::ContextProvenanceRef)
    }
}

/// Support-export bundle binding multiple replay packs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReplayPackSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Replay packs bundled in the export, in canonical order.
    pub packs: Vec<RuntimeReplayPack>,
    /// True when any bundled pack forbids live rerun.
    pub any_pack_forbids_live_rerun: bool,
    /// True when any bundled pack's comparator blocks replay.
    pub any_pack_comparator_blocks_replay: bool,
    /// True when every bundled pack carries the required artefact classes.
    pub every_pack_covers_required_artefact_classes: bool,
    /// Tokens for the distinct fidelity classes present in the export, in
    /// canonical order. Surfaced for reviewers and dashboards.
    pub fidelity_class_tokens_present: Vec<String>,
    /// Tokens for the distinct reopen decisions present in the export.
    pub reopen_decision_tokens_present: Vec<String>,
    /// Reviewer-facing summary lines, one per pack.
    pub summary_lines: Vec<String>,
}

impl RuntimeReplayPackSupportExport {
    /// Build a support-export bundle from replay packs.
    pub fn new(
        support_export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        packs: Vec<RuntimeReplayPack>,
    ) -> Self {
        let summary_lines = packs.iter().map(|p| p.summary.clone()).collect();
        let any_pack_forbids_live_rerun = packs.iter().any(|p| p.forbids_live_rerun);
        let any_pack_comparator_blocks_replay = packs.iter().any(|p| p.comparator_blocks_replay);
        let every_pack_covers_required_artefact_classes =
            packs.iter().all(|p| p.covers_required_artefact_classes());

        let mut fidelity_set: BTreeSet<ReplayFidelityClass> = BTreeSet::new();
        let mut reopen_set: BTreeSet<ReplayReopenDecisionClass> = BTreeSet::new();
        for pack in &packs {
            fidelity_set.insert(pack.fidelity);
            reopen_set.insert(pack.reopen_decision);
        }
        let fidelity_class_tokens_present = ReplayFidelityClass::ALL
            .iter()
            .filter(|f| fidelity_set.contains(f))
            .map(|f| f.as_str().to_owned())
            .collect();
        let reopen_decision_tokens_present = ReplayReopenDecisionClass::ALL
            .iter()
            .filter(|d| reopen_set.contains(d))
            .map(|d| d.as_str().to_owned())
            .collect();

        Self {
            record_kind: RUNTIME_REPLAY_PACK_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_REPLAY_PACK_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            packs,
            any_pack_forbids_live_rerun,
            any_pack_comparator_blocks_replay,
            every_pack_covers_required_artefact_classes,
            fidelity_class_tokens_present,
            reopen_decision_tokens_present,
            summary_lines,
        }
    }

    /// Deterministic reviewer plaintext form. Packs render in their declared
    /// order so the support clipboard, the CLI, and the shell panel agree
    /// byte-for-byte.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Runtime replay packs support export: {}\n",
            self.support_export_id
        ));
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!(
            "Packs: {} (forbids_live_rerun={}, comparator_blocks_replay={}, artefact_coverage={})\n",
            self.packs.len(),
            self.any_pack_forbids_live_rerun,
            self.any_pack_comparator_blocks_replay,
            self.every_pack_covers_required_artefact_classes,
        ));
        out.push_str(&format!(
            "Fidelity classes: {}\n",
            self.fidelity_class_tokens_present.join(",")
        ));
        out.push_str(&format!(
            "Reopen decisions: {}\n",
            self.reopen_decision_tokens_present.join(",")
        ));
        for pack in &self.packs {
            out.push_str(&format!(
                "\n[{}] {} ({}) lane={} kind={}\n",
                pack.fidelity_token,
                pack.reopen_decision_token,
                pack.replay_pack_id,
                pack.evidence_packet.lane_token,
                pack.evidence_packet.evidence_kind_token,
            ));
            out.push_str(&format!(
                "  subject: {}\n",
                pack.evidence_packet.subject_ref
            ));
            out.push_str(&format!(
                "  privilege: {} | forbids_live_rerun: {}\n",
                pack.subject_privilege_token, pack.forbids_live_rerun,
            ));
            out.push_str(&format!(
                "  evidence_packet: {} | comparator: {}\n",
                pack.evidence_packet.evidence_packet_id, pack.replay_comparison.compatibility_token,
            ));
            if !pack
                .replay_comparison
                .incompatibility_reason_tokens
                .is_empty()
            {
                out.push_str(&format!(
                    "  comparator_reasons: {}\n",
                    pack.replay_comparison
                        .incompatibility_reason_tokens
                        .join(","),
                ));
            }
            for artefact in &pack.artefacts {
                out.push_str(&format!(
                    "  artefact[{}]: {} ({})\n",
                    artefact.artefact_class_token, artefact.artefact_ref, artefact.label,
                ));
            }
            out.push_str(&format!("  summary: {}\n", pack.summary));
        }
        out
    }
}

/// Identifier for the canonical seeded replay-pack scenarios. Each scenario
/// covers one closed fidelity class. Reviewer and partner runs of the seed
/// builder reproduce the same packs byte-for-byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeReplayPackSeededScenario {
    /// Local task event captured against a matching replay context.
    /// Fidelity `exact`, privilege `read_only`, reopen `allow_replay`.
    LocalTaskExactReadOnly,
    /// Local test attempt; replay context advances policy epoch cleanly.
    /// Fidelity `compatible`, privilege `read_only`, reopen `allow_replay`.
    LocalTestCompatibleReadOnly,
    /// Container debug session captured before capsule drift. Fidelity
    /// `layout_only`, privilege `mutating`, reopen `allow_inspect_no_rerun`.
    ContainerDebugLayoutOnlyMutating,
    /// Managed-workspace runtime evidence; replay context downgrades trust.
    /// Fidelity `layout_only`, privilege `privileged`, reopen
    /// `allow_inspect_no_rerun`.
    ManagedRuntimeLayoutOnlyPrivileged,
}

impl RuntimeReplayPackSeededScenario {
    /// All seeded scenarios in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalTaskExactReadOnly,
        Self::LocalTestCompatibleReadOnly,
        Self::ContainerDebugLayoutOnlyMutating,
        Self::ManagedRuntimeLayoutOnlyPrivileged,
    ];

    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTaskExactReadOnly => "local_task_exact_read_only",
            Self::LocalTestCompatibleReadOnly => "local_test_compatible_read_only",
            Self::ContainerDebugLayoutOnlyMutating => "container_debug_layout_only_mutating",
            Self::ManagedRuntimeLayoutOnlyPrivileged => "managed_runtime_layout_only_privileged",
        }
    }

    /// The runtime evidence-packet scenario this replay-pack scenario joins to.
    pub const fn runtime_scenario(self) -> RuntimeEvidencePacketSeededScenario {
        match self {
            Self::LocalTaskExactReadOnly => {
                RuntimeEvidencePacketSeededScenario::LocalTaskCompatible
            }
            Self::LocalTestCompatibleReadOnly => {
                RuntimeEvidencePacketSeededScenario::LocalTestPolicyAdvancedClean
            }
            Self::ContainerDebugLayoutOnlyMutating => {
                RuntimeEvidencePacketSeededScenario::ContainerDebugCapsuleDrift
            }
            Self::ManagedRuntimeLayoutOnlyPrivileged => {
                RuntimeEvidencePacketSeededScenario::ManagedRuntimeTrustDowngraded
            }
        }
    }

    /// Privilege class applied to the pack.
    pub const fn subject_privilege(self) -> ReplaySubjectPrivilegeClass {
        match self {
            Self::LocalTaskExactReadOnly => ReplaySubjectPrivilegeClass::ReadOnly,
            Self::LocalTestCompatibleReadOnly => ReplaySubjectPrivilegeClass::ReadOnly,
            Self::ContainerDebugLayoutOnlyMutating => ReplaySubjectPrivilegeClass::Mutating,
            Self::ManagedRuntimeLayoutOnlyPrivileged => ReplaySubjectPrivilegeClass::Privileged,
        }
    }

    /// Expected fidelity class.
    pub const fn expected_fidelity(self) -> ReplayFidelityClass {
        match self {
            Self::LocalTaskExactReadOnly => ReplayFidelityClass::Exact,
            Self::LocalTestCompatibleReadOnly => ReplayFidelityClass::Compatible,
            Self::ContainerDebugLayoutOnlyMutating => ReplayFidelityClass::LayoutOnly,
            Self::ManagedRuntimeLayoutOnlyPrivileged => ReplayFidelityClass::LayoutOnly,
        }
    }

    /// Expected reopen decision.
    pub const fn expected_reopen_decision(self) -> ReplayReopenDecisionClass {
        match self {
            Self::LocalTaskExactReadOnly => ReplayReopenDecisionClass::AllowReplay,
            Self::LocalTestCompatibleReadOnly => ReplayReopenDecisionClass::AllowReplay,
            Self::ContainerDebugLayoutOnlyMutating => {
                ReplayReopenDecisionClass::AllowInspectNoRerun
            }
            Self::ManagedRuntimeLayoutOnlyPrivileged => {
                ReplayReopenDecisionClass::AllowInspectNoRerun
            }
        }
    }
}

fn seeded_artefacts(scenario: RuntimeReplayPackSeededScenario) -> Vec<ReplayPackArtefact> {
    let runtime_scenario = scenario.runtime_scenario();
    let key = runtime_scenario.as_str();
    vec![
        ReplayPackArtefact::new(
            ReplayPackArtefactClass::TranscriptRef,
            format!("transcript:{key}:seed"),
            "Captured terminal transcript",
        ),
        ReplayPackArtefact::new(
            ReplayPackArtefactClass::RuntimeLogRef,
            format!("runtime-log:{key}:seed"),
            "Structured runtime log slice",
        ),
        ReplayPackArtefact::new(
            ReplayPackArtefactClass::ArtifactBlobRef,
            format!("artifact-blob:{key}:seed"),
            "Opaque captured artefact digest",
        ),
        ReplayPackArtefact::new(
            ReplayPackArtefactClass::EvidencePacketRef,
            format!("evpkt:{key}"),
            "Runtime evidence packet",
        ),
        ReplayPackArtefact::new(
            ReplayPackArtefactClass::ContextProvenanceRef,
            format!("provenance:{key}:seed"),
            "Execution-context provenance projection",
        ),
    ]
}

/// Build the canonical seeded replay pack for one scenario.
pub fn seeded_runtime_replay_pack(scenario: RuntimeReplayPackSeededScenario) -> RuntimeReplayPack {
    let (packet, comparison) = seeded_runtime_evidence_packet(scenario.runtime_scenario());
    let artefacts = seeded_artefacts(scenario);
    RuntimeReplayPack::new(
        format!("replay-pack:{}", scenario.as_str()),
        "2026-05-15T19:02:00Z",
        packet,
        comparison,
        scenario.subject_privilege(),
        artefacts,
    )
}

/// Build the canonical seeded support-export bundle covering every scenario.
pub fn seeded_runtime_replay_pack_support_export(
    support_export_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> RuntimeReplayPackSupportExport {
    let packs = RuntimeReplayPackSeededScenario::ALL
        .into_iter()
        .map(seeded_runtime_replay_pack)
        .collect();
    RuntimeReplayPackSupportExport::new(
        support_export_id,
        "ws-evidence-packet-beta",
        generated_at,
        packs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_runtime::ReplayIncompatibilityReason;

    #[test]
    fn fidelity_class_tokens_match_closed_vocabulary() {
        let tokens: Vec<&str> = ReplayFidelityClass::ALL
            .iter()
            .map(|f| f.as_str())
            .collect();
        assert_eq!(
            tokens,
            vec!["exact", "compatible", "layout_only", "evidence_only"],
        );
    }

    #[test]
    fn reopen_decision_tokens_match_closed_vocabulary() {
        let tokens: Vec<&str> = ReplayReopenDecisionClass::ALL
            .iter()
            .map(|d| d.as_str())
            .collect();
        assert_eq!(
            tokens,
            vec![
                "allow_replay",
                "allow_inspect_no_rerun",
                "allow_evidence_only_view",
                "blocked",
            ],
        );
    }

    #[test]
    fn artefact_class_tokens_match_closed_vocabulary() {
        let tokens: Vec<&str> = ReplayPackArtefactClass::ALL
            .iter()
            .map(|c| c.as_str())
            .collect();
        assert_eq!(
            tokens,
            vec![
                "transcript_ref",
                "runtime_log_ref",
                "artifact_blob_ref",
                "evidence_packet_ref",
                "context_provenance_ref",
            ],
        );
    }

    #[test]
    fn local_task_scenario_resolves_to_exact_read_only_replay() {
        let pack =
            seeded_runtime_replay_pack(RuntimeReplayPackSeededScenario::LocalTaskExactReadOnly);
        assert_eq!(pack.fidelity, ReplayFidelityClass::Exact);
        assert_eq!(
            pack.subject_privilege,
            ReplaySubjectPrivilegeClass::ReadOnly
        );
        assert_eq!(pack.reopen_decision, ReplayReopenDecisionClass::AllowReplay);
        assert!(!pack.forbids_live_rerun);
        assert!(!pack.comparator_blocks_replay);
        assert!(pack.covers_required_artefact_classes());
    }

    #[test]
    fn local_test_scenario_resolves_to_compatible_read_only_replay() {
        let pack = seeded_runtime_replay_pack(
            RuntimeReplayPackSeededScenario::LocalTestCompatibleReadOnly,
        );
        assert_eq!(pack.fidelity, ReplayFidelityClass::Compatible);
        assert_eq!(pack.reopen_decision, ReplayReopenDecisionClass::AllowReplay);
        assert!(pack.replay_comparison.permits_replay_without_review);
        assert!(pack
            .replay_comparison
            .incompatibility_reasons
            .contains(&ReplayIncompatibilityReason::CleanForwardDrift));
    }

    #[test]
    fn mutating_subject_never_silently_reruns_on_layout_only_capsule_drift() {
        let pack = seeded_runtime_replay_pack(
            RuntimeReplayPackSeededScenario::ContainerDebugLayoutOnlyMutating,
        );
        assert_eq!(pack.fidelity, ReplayFidelityClass::LayoutOnly);
        assert_eq!(
            pack.subject_privilege,
            ReplaySubjectPrivilegeClass::Mutating
        );
        assert_eq!(
            pack.reopen_decision,
            ReplayReopenDecisionClass::AllowInspectNoRerun,
        );
        assert!(pack.forbids_live_rerun);
        assert!(pack.comparator_blocks_replay);
    }

    #[test]
    fn privileged_subject_never_silently_reruns_on_trust_downgrade() {
        let pack = seeded_runtime_replay_pack(
            RuntimeReplayPackSeededScenario::ManagedRuntimeLayoutOnlyPrivileged,
        );
        assert_eq!(pack.fidelity, ReplayFidelityClass::LayoutOnly);
        assert_eq!(
            pack.subject_privilege,
            ReplaySubjectPrivilegeClass::Privileged
        );
        assert_eq!(
            pack.reopen_decision,
            ReplayReopenDecisionClass::AllowInspectNoRerun,
        );
        assert!(pack.forbids_live_rerun);
        assert!(pack.comparator_blocks_replay);
    }

    #[test]
    fn even_an_exact_pack_with_mutating_privilege_drops_to_inspect_only() {
        let (packet, comparison) = seeded_runtime_evidence_packet(
            RuntimeEvidencePacketSeededScenario::LocalTaskCompatible,
        );
        let pack = RuntimeReplayPack::new(
            "pack:exact-mutating",
            "2026-05-15T19:02:00Z",
            packet,
            comparison,
            ReplaySubjectPrivilegeClass::Mutating,
            Vec::new(),
        );
        assert_eq!(pack.fidelity, ReplayFidelityClass::Exact);
        assert_eq!(
            pack.reopen_decision,
            ReplayReopenDecisionClass::AllowInspectNoRerun,
        );
        assert!(pack.forbids_live_rerun);
    }

    #[test]
    fn support_export_bundles_every_scenario_with_no_secret_markers() {
        let export = seeded_runtime_replay_pack_support_export(
            "replay-pack-support:test",
            "2026-05-15T19:03:00Z",
        );
        assert_eq!(
            export.packs.len(),
            RuntimeReplayPackSeededScenario::ALL.len(),
        );
        assert!(export.every_pack_covers_required_artefact_classes);
        assert!(export.any_pack_forbids_live_rerun);
        assert!(export.any_pack_comparator_blocks_replay);
        for token in ["exact", "compatible", "layout_only"] {
            assert!(
                export
                    .fidelity_class_tokens_present
                    .contains(&token.to_owned()),
                "missing fidelity token '{token}'",
            );
        }
        for token in ["allow_replay", "allow_inspect_no_rerun"] {
            assert!(
                export
                    .reopen_decision_tokens_present
                    .contains(&token.to_owned()),
                "missing reopen token '{token}'",
            );
        }

        let json = serde_json::to_string(&export).expect("serialize export");
        // Replay packs must NOT carry raw env, raw command lines, or
        // credential markers anywhere in their JSON form. The bundled
        // evidence packets pin metadata_safe_default and the artefact refs
        // are opaque digests; this assertion guards against accidental
        // additions that would leak.
        assert!(!json.contains("BEARER"));
        assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
        assert!(!json.contains("SSH_PRIVATE_KEY"));
        assert!(!json.contains("LD_LIBRARY_PATH"));
    }

    #[test]
    fn support_export_round_trips_through_serde() {
        let export = seeded_runtime_replay_pack_support_export(
            "replay-pack-support:round-trip",
            "2026-05-15T19:03:00Z",
        );
        let json = serde_json::to_string(&export).expect("serialize");
        let round: RuntimeReplayPackSupportExport =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, export);
    }

    #[test]
    fn seeded_support_export_is_deterministic_across_calls() {
        let first = serde_json::to_string(&seeded_runtime_replay_pack_support_export(
            "replay-pack-support:deterministic",
            "2026-05-15T19:03:00Z",
        ))
        .expect("first");
        let second = serde_json::to_string(&seeded_runtime_replay_pack_support_export(
            "replay-pack-support:deterministic",
            "2026-05-15T19:03:00Z",
        ))
        .expect("second");
        assert_eq!(first, second);
    }

    #[test]
    fn plaintext_quotes_closed_tokens() {
        let export = seeded_runtime_replay_pack_support_export(
            "replay-pack-support:plaintext",
            "2026-05-15T19:03:00Z",
        );
        let text = export.render_plaintext();
        for token in [
            "exact",
            "compatible",
            "layout_only",
            "allow_replay",
            "allow_inspect_no_rerun",
            "read_only",
            "mutating",
            "privileged",
            "transcript_ref",
            "runtime_log_ref",
            "evidence_packet_ref",
            "context_provenance_ref",
            "incompatible_capsule_drift",
            "incompatible_trust_state_downgraded",
        ] {
            assert!(
                text.contains(token),
                "plaintext must quote closed token '{token}'",
            );
        }
    }
}
