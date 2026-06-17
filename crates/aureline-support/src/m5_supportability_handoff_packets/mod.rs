//! Supportability handoff packets that join finding codes, repair ids, crash artifacts, install /
//! advisory state, credential-state descriptors, environment / precedence summaries, and
//! restore-provenance records into one typed escalation object per blocked-user incident.
//!
//! Where the Support Center matrix owns *which inspectors exist*, the crash-intake registry owns *how a
//! blocked user is offered recovery*, and the support-bundle consent sheet owns *what a bundle would
//! contain*, this packet governs *how a blocked-user incident is escalated as one supportability handoff
//! object* — instead of an ad hoc pile of logs and screenshots. It is a registry of handoff packets, one
//! per escalation scenario worth carrying, each carrying the visible incident ref, the copyable
//! exact-build id, a set of typed **handoff components** (one per joined source object — a finding code,
//! a repair id, a crash artifact, the install / advisory state, a credential-state descriptor, an
//! environment summary, a precedence summary, or a restore-provenance record), and exactly one **handoff
//! mode** (local self-diagnosis, team share, or formal support handoff). Each component reuses the
//! existing finding / repair / crash / install / credential / environment / restore objects by reference:
//! it carries a `source_ref` and a `lineage_ref` projecting from those objects rather than re-deriving any
//! of them.
//!
//! The readiness analogue here is a fail-closed **handoff / share gate**. The guardrail the source set
//! treats as core supportability is that a handoff must never collapse into a monolithic export that hides
//! data-class differences or redaction posture, must never carry a data class further than its destination
//! allows, and must never present a clean "ready to share" packet that hides a redacted, withheld,
//! policy-locked, or downgraded component — and must keep the local self-diagnosis path first-class beside
//! team-share and formal-support sends. Each packet therefore publishes a [`HandoffPresentation`] derived
//! from the disposition of its components for the selected mode: a component is *carried* when it is
//! included and send-safe for the mode, *redacted* when it is carried under a redacted summary, *withheld*
//! when its data class cannot reach the mode or is policy-locked, and *blocking* when it is included but
//! cannot safely leave the machine for the mode. A packet with a blocking component is **send-blocked**; a
//! packet that had to redact or withhold a component, or that carries a downgraded lineage, is
//! **narrowed**; otherwise it is **ready to share**. Two hard rules still hold: every component keeps its
//! data class and redaction posture visible (no monolithic export), and the exact-build, finding-code, and
//! repair-id lineage is preserved on every component.
//!
//! Every packet always carries its one-step `explain_entrypoint_ref` — the inspectable "Why is this
//! escalating, and what does it carry?" answer — and its `cli_object_ref`, the CLI / headless equivalent,
//! so the same handoff object is reachable from the Support Center, the CLI / headless export, the
//! issue-report flow, the support drill packet, and the support export without forking. Every required
//! consumer surface binds to this one registry via a [`HandoffConsumerBinding`] that must ingest it,
//! preserve its handoff vocabulary, packet / component ids, and exact-build / finding-code / repair-id
//! lineage, keep data classes visible, and narrow with it.
//!
//! The packet is checked in at `artifacts/support/m5/m5-supportability-handoff-packets.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, a visible id, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, raw stack dumps, or secret-bearing payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported supportability-handoff schema version.
pub const M5_SUPPORTABILITY_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SUPPORTABILITY_HANDOFF_RECORD_KIND: &str = "m5_supportability_handoff_packets";

/// Repo-relative path to the checked-in packet.
pub const M5_SUPPORTABILITY_HANDOFF_PATH: &str =
    "artifacts/support/m5/m5-supportability-handoff-packets.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SUPPORTABILITY_HANDOFF_SCHEMA_REF: &str =
    "schemas/support/m5-supportability-handoff-packets.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SUPPORTABILITY_HANDOFF_DOC_REF: &str =
    "docs/help/support/m5-supportability-handoff-packets.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_SUPPORTABILITY_HANDOFF_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-supportability-handoff-packets.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SUPPORTABILITY_HANDOFF_FIXTURE_DIR: &str =
    "fixtures/support/m5/m5-supportability-handoff-packets";

/// Repo-relative path to the shiproom review packet that renders this registry.
pub const M5_SUPPORTABILITY_HANDOFF_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-supportability-handoff-packets-review-packet/supportability_handoff_packets_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_SUPPORTABILITY_HANDOFF_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-supportability-handoff-packets.json"
));

/// A typed handoff mode. Local self-diagnosis, team share, and formal support handoff are the three
/// escalation paths a packet can take, each with its own allowed data classes and default redaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffMode {
    /// A local-only self-diagnosis packet; nothing ever leaves the machine.
    LocalSelfDiagnosis,
    /// A share with the user's team.
    TeamShare,
    /// A formal support / vendor handoff.
    FormalSupportHandoff,
}

impl HandoffMode {
    /// Every handoff mode, least to most exposing.
    pub const ALL: [Self; 3] = [
        Self::LocalSelfDiagnosis,
        Self::TeamShare,
        Self::FormalSupportHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSelfDiagnosis => "local_self_diagnosis",
            Self::TeamShare => "team_share",
            Self::FormalSupportHandoff => "formal_support_handoff",
        }
    }

    /// Whether this is the local-only self-diagnosis mode.
    pub const fn is_local_self_diagnosis(self) -> bool {
        matches!(self, Self::LocalSelfDiagnosis)
    }

    /// Whether selecting this mode causes the packet to leave the machine.
    pub const fn leaves_machine(self) -> bool {
        !self.is_local_self_diagnosis()
    }

    /// Exposure rank; higher reaches further off the machine.
    pub const fn exposure_rank(self) -> u8 {
        match self {
            Self::LocalSelfDiagnosis => 0,
            Self::TeamShare => 1,
            Self::FormalSupportHandoff => 2,
        }
    }

    /// The default redaction posture this mode applies to its components.
    pub const fn default_redaction_posture(self) -> RedactionPosture {
        match self {
            Self::LocalSelfDiagnosis => RedactionPosture::LocalOnlyRetained,
            Self::TeamShare => RedactionPosture::RedactedSummary,
            Self::FormalSupportHandoff => RedactionPosture::MetadataSafeDefault,
        }
    }

    /// The data classes that may be carried off the machine for this mode, in canonical order.
    pub fn allowed_data_classes(self) -> Vec<HandoffDataClass> {
        HandoffDataClass::ALL
            .into_iter()
            .filter(|class| class.may_reach(self))
            .collect()
    }
}

/// The kind of source object a handoff component projects. The closed set is exactly the objects this row
/// joins into one escalation packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffComponentKind {
    /// A Project Doctor finding code.
    FindingCode,
    /// A guided-repair transaction id.
    RepairId,
    /// A crash envelope / symbolication artifact.
    CrashArtifact,
    /// The install / advisory state for the running build.
    InstallAdvisoryState,
    /// A credential-state descriptor (presence / scope / expiry — never the secret body).
    CredentialStateDescriptor,
    /// An execution-context / environment summary.
    EnvironmentSummary,
    /// A precedence-resolution summary.
    PrecedenceSummary,
    /// A restore-provenance record.
    RestoreProvenanceRecord,
}

impl HandoffComponentKind {
    /// Every component kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::FindingCode,
        Self::RepairId,
        Self::CrashArtifact,
        Self::InstallAdvisoryState,
        Self::CredentialStateDescriptor,
        Self::EnvironmentSummary,
        Self::PrecedenceSummary,
        Self::RestoreProvenanceRecord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingCode => "finding_code",
            Self::RepairId => "repair_id",
            Self::CrashArtifact => "crash_artifact",
            Self::InstallAdvisoryState => "install_advisory_state",
            Self::CredentialStateDescriptor => "credential_state_descriptor",
            Self::EnvironmentSummary => "environment_summary",
            Self::PrecedenceSummary => "precedence_summary",
            Self::RestoreProvenanceRecord => "restore_provenance_record",
        }
    }
}

/// The data-sensitivity class of a handoff component, kept visible so a handoff never hides data-class
/// differences. The class caps how far the component may travel off the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDataClass {
    /// Opaque ids, counts, and tokens; safe for any destination.
    Metadata,
    /// A redaction-safe diagnostic summary (finding / repair descriptions); safe for any destination.
    DiagnosticSummary,
    /// An execution-context / precedence descriptor; safe for any destination.
    EnvironmentDescriptor,
    /// A credential-state descriptor (presence / scope / expiry, never the secret); team-share at most.
    CredentialState,
    /// A reference to a crash artifact (envelope / symbolication ref); safe for any destination.
    CrashArtifactReference,
    /// An excerpt of user-owned content; never leaves the machine.
    UserContentExcerpt,
}

impl HandoffDataClass {
    /// Every data class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Metadata,
        Self::DiagnosticSummary,
        Self::EnvironmentDescriptor,
        Self::CredentialState,
        Self::CrashArtifactReference,
        Self::UserContentExcerpt,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Metadata => "metadata",
            Self::DiagnosticSummary => "diagnostic_summary",
            Self::EnvironmentDescriptor => "environment_descriptor",
            Self::CredentialState => "credential_state",
            Self::CrashArtifactReference => "crash_artifact_reference",
            Self::UserContentExcerpt => "user_content_excerpt",
        }
    }

    /// The most exposing mode this class may be carried to.
    pub const fn max_exposure_rank(self) -> u8 {
        match self {
            Self::Metadata
            | Self::DiagnosticSummary
            | Self::EnvironmentDescriptor
            | Self::CrashArtifactReference => 2,
            Self::CredentialState => 1,
            Self::UserContentExcerpt => 0,
        }
    }

    /// Whether this class may be carried off the machine for the given mode.
    pub const fn may_reach(self, mode: HandoffMode) -> bool {
        mode.exposure_rank() <= self.max_exposure_rank()
    }
}

/// How a handoff component is redacted in the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPosture {
    /// Metadata only; export-safe.
    MetadataSafeDefault,
    /// A redacted summary; export-safe.
    RedactedSummary,
    /// Retained on the machine only; not export-safe.
    LocalOnlyRetained,
    /// Withheld by policy; not export-safe.
    PolicyLocked,
    /// Content cannot be made safe; not export-safe.
    BlockedUnsafeContent,
}

impl RedactionPosture {
    /// Every redaction posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MetadataSafeDefault,
        Self::RedactedSummary,
        Self::LocalOnlyRetained,
        Self::PolicyLocked,
        Self::BlockedUnsafeContent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::RedactedSummary => "redacted_summary",
            Self::LocalOnlyRetained => "local_only_retained",
            Self::PolicyLocked => "policy_locked",
            Self::BlockedUnsafeContent => "blocked_unsafe_content",
        }
    }

    /// Whether a component handled this way may be carried off the machine.
    pub const fn is_export_safe_off_machine(self) -> bool {
        matches!(self, Self::MetadataSafeDefault | Self::RedactedSummary)
    }

    /// Whether the component is carried as a redacted summary rather than in full.
    pub const fn is_redacted(self) -> bool {
        matches!(self, Self::RedactedSummary)
    }
}

/// The effective disposition of a component for the packet's selected mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentDisposition {
    /// Included and send-safe for the mode, carried in full.
    Carried,
    /// Included and send-safe for the mode, but carried as a redacted summary.
    Redacted,
    /// Withheld: its data class cannot reach the mode, or it is policy-locked.
    Withheld,
    /// Included but cannot safely leave the machine for the mode; the send is blocked.
    Blocking,
}

impl ComponentDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Carried,
        Self::Redacted,
        Self::Withheld,
        Self::Blocking,
    ];

    /// Stable token recorded in export rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Carried => "carried",
            Self::Redacted => "redacted",
            Self::Withheld => "withheld",
            Self::Blocking => "blocking",
        }
    }
}

/// The overall handoff disposition of a packet — the headline reason it is or is not a clean, ready,
/// shareable escalation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffStatus {
    /// Every component is carried in full for the selected mode; the packet is ready to share as-is.
    ReadyToShare,
    /// A component had to be redacted or withheld to fit the mode, or carries a downgraded lineage.
    RedactionNarrowed,
    /// A component is policy-locked for the destination and is withheld.
    PolicyLocked,
    /// The selected mode would carry content that cannot safely leave the machine; the send is blocked.
    SendBlocked,
}

impl HandoffStatus {
    /// Every handoff status, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReadyToShare,
        Self::RedactionNarrowed,
        Self::PolicyLocked,
        Self::SendBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToShare => "ready_to_share",
            Self::RedactionNarrowed => "redaction_narrowed",
            Self::PolicyLocked => "policy_locked",
            Self::SendBlocked => "send_blocked",
        }
    }

    /// Highest presentation this status permits.
    pub const fn presentation_ceiling(self) -> HandoffPresentation {
        match self {
            Self::ReadyToShare => HandoffPresentation::ReadyToShare,
            Self::RedactionNarrowed | Self::PolicyLocked => HandoffPresentation::Narrowed,
            Self::SendBlocked => HandoffPresentation::SendBlocked,
        }
    }

    /// Whether this status names blockers the user must reconcile before sending.
    pub const fn requires_blockers(self) -> bool {
        matches!(self, Self::SendBlocked)
    }
}

/// The presentation the handoff / share gate publishes for a packet, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPresentation {
    /// Every component is carried in full; the packet shares as-is.
    ReadyToShare,
    /// The packet is shareable but narrowed: a component was redacted, withheld, policy-locked, or carries
    /// a downgraded lineage. Data classes and redaction posture stay visible.
    Narrowed,
    /// A component cannot safely leave the machine for the selected mode; the packet warns and blocks the
    /// send before anything leaves.
    SendBlocked,
}

impl HandoffPresentation {
    /// Every presentation, highest to lowest.
    pub const ALL: [Self; 3] = [Self::ReadyToShare, Self::Narrowed, Self::SendBlocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToShare => "ready_to_share",
            Self::Narrowed => "narrowed",
            Self::SendBlocked => "send_blocked",
        }
    }

    /// Whether the gate narrowed or blocked the packet below a fully ready, shareable packet.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::ReadyToShare)
    }

    /// Whether the packet must warn and block before anything leaves the machine.
    pub const fn warns_before_send(self) -> bool {
        matches!(self, Self::SendBlocked)
    }
}

/// A headline reason the handoff / share gate narrows or blocks a packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDowngradeReason {
    /// A component is carried as a redacted summary to fit the selected mode.
    DataClassRedactedForMode,
    /// A component is excluded because its data class cannot reach the selected mode.
    ComponentExcludedForMode,
    /// A component is policy-locked for the destination.
    PolicyLockedDataClass,
    /// A component carries a downgraded / approximate lineage record.
    LineageDowngraded,
    /// A component is included but cannot safely leave the machine for the selected mode.
    SendBlockedUnsafeContent,
}

impl HandoffDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DataClassRedactedForMode,
        Self::ComponentExcludedForMode,
        Self::PolicyLockedDataClass,
        Self::LineageDowngraded,
        Self::SendBlockedUnsafeContent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataClassRedactedForMode => "data_class_redacted_for_mode",
            Self::ComponentExcludedForMode => "component_excluded_for_mode",
            Self::PolicyLockedDataClass => "policy_locked_data_class",
            Self::LineageDowngraded => "lineage_downgraded",
            Self::SendBlockedUnsafeContent => "send_blocked_unsafe_content",
        }
    }
}

/// A downstream surface that must ingest this registry and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffConsumerSurface {
    /// The Support Center escalation views.
    SupportCenter,
    /// The CLI / headless support export path.
    CliHeadless,
    /// The issue-report / crash-intake flow.
    IssueReportFlow,
    /// The shiproom / support drill packet.
    SupportDrillPacket,
    /// The support export of the handoff.
    SupportExport,
}

impl HandoffConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::SupportCenter,
        Self::CliHeadless,
        Self::IssueReportFlow,
        Self::SupportDrillPacket,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportCenter => "support_center",
            Self::CliHeadless => "cli_headless",
            Self::IssueReportFlow => "issue_report_flow",
            Self::SupportDrillPacket => "support_drill_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// One typed handoff component projecting a single joined source object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffComponent {
    /// Stable component id (kept visible in-product and in exports).
    pub component_id: String,
    /// The kind of source object this component projects.
    pub component_kind: HandoffComponentKind,
    /// The data-sensitivity class of this component; kept visible.
    pub data_class: HandoffDataClass,
    /// The redaction posture applied to this component.
    pub redaction_posture: RedactionPosture,
    /// Whether this component is included in the packet for the selected mode.
    pub included: bool,
    /// Whether a policy lock withholds this component from the destination.
    pub policy_locked: bool,
    /// Whether the projected lineage record is downgraded / approximate.
    pub lineage_downgraded: bool,
    /// Opaque ref to the source-of-truth object (finding, repair, crash, install, credential, environment,
    /// precedence, or restore object).
    pub source_ref: String,
    /// The lineage token preserved by this component (finding code, repair id, crash-envelope / build id).
    pub lineage_ref: String,
    /// Reviewer-facing label that excludes raw paths and private content.
    pub display_label: String,
    /// Plain-language summary of what this component carries.
    pub summary: String,
}

impl HandoffComponent {
    /// Whether the component carries its non-empty id, refs, label, and summary.
    pub fn is_well_formed(&self) -> bool {
        !self.component_id.trim().is_empty()
            && !self.source_ref.trim().is_empty()
            && !self.lineage_ref.trim().is_empty()
            && !self.display_label.trim().is_empty()
            && !self.summary.trim().is_empty()
    }

    /// Whether the component is carried off the machine safely for the given mode.
    pub fn is_carried_safely(&self, mode: HandoffMode) -> bool {
        if !self.included {
            return false;
        }
        if !mode.leaves_machine() {
            return true;
        }
        self.redaction_posture.is_export_safe_off_machine()
            && !self.policy_locked
            && self.data_class.may_reach(mode)
    }

    /// Whether a withheld component is withheld for a legitimate reason (policy lock or the mode cannot
    /// carry its data class). An included component is trivially justified.
    pub fn is_justified_withholding(&self, mode: HandoffMode) -> bool {
        self.included || self.policy_locked || !self.data_class.may_reach(mode)
    }

    /// The effective disposition of this component for the given mode.
    pub fn disposition(&self, mode: HandoffMode) -> ComponentDisposition {
        if !self.included {
            return ComponentDisposition::Withheld;
        }
        if mode.leaves_machine() && !self.is_carried_safely(mode) {
            return ComponentDisposition::Blocking;
        }
        if self.redaction_posture.is_redacted() {
            return ComponentDisposition::Redacted;
        }
        ComponentDisposition::Carried
    }

    /// Whether this component is excluded specifically because its data class cannot reach the mode.
    fn excluded_for_mode(&self, mode: HandoffMode) -> bool {
        !self.included && !self.policy_locked && !self.data_class.may_reach(mode)
    }
}

/// The per-mode policy declaration: which data classes a mode allows and its default redaction posture.
/// Published so a handoff never hides data-class differences between modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffModePolicy {
    /// The handoff mode this policy governs.
    pub mode: HandoffMode,
    /// Whether selecting this mode causes the packet to leave the machine; must match the mode.
    pub leaves_machine: bool,
    /// The data classes that may be carried off the machine for this mode; must equal the computed set.
    pub allowed_data_classes: Vec<HandoffDataClass>,
    /// The default redaction posture this mode applies; must equal the mode's default.
    pub default_redaction_posture: RedactionPosture,
    /// Reviewer-facing note.
    pub note: String,
}

impl HandoffModePolicy {
    /// Whether the policy's flags and lists agree with the mode.
    pub fn is_mode_consistent(&self) -> bool {
        self.leaves_machine == self.mode.leaves_machine()
            && self.allowed_data_classes == self.mode.allowed_data_classes()
            && self.default_redaction_posture == self.mode.default_redaction_posture()
            && !self.note.trim().is_empty()
    }
}

/// One supportability handoff packet: one blocked-user incident escalated as a single typed object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffPacket {
    /// Stable packet id (kept visible in-product and in exports).
    pub packet_id: String,
    /// Human-readable label for the packet.
    pub title: String,
    /// Visible ref to the blocked-user incident this packet escalates.
    pub incident_ref: String,
    /// Visible, copyable exact-build id (kept visible in-product and in exports).
    pub exact_build_id: String,
    /// Whether the build id is copyable; must be true.
    pub build_id_copyable: bool,
    /// The handoff mode this packet takes.
    pub mode: HandoffMode,
    /// Overall handoff status; must equal the recomputed status.
    pub status: HandoffStatus,
    /// Presentation actually published after the gate; must equal the recomputed decision.
    pub presentation: HandoffPresentation,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<HandoffDowngradeReason>,
    /// Attestation that the exact-build / finding-code / repair-id lineage is complete; must equal the
    /// recomputed value.
    pub lineage_complete: bool,
    /// Attestation that every component keeps its data class and redaction posture visible; always true.
    pub data_classes_visible: bool,
    /// True when the packet warns and blocks before anything leaves the machine; required iff send-blocked.
    pub blocked_before_send: bool,
    /// Attestation that no raw secret bodies, raw dumps, or raw payloads are carried; always true.
    pub raw_material_excluded: bool,
    /// The typed handoff components joined into this packet; at least one is required.
    #[serde(default)]
    pub components: Vec<HandoffComponent>,
    /// Caveats attached to a narrowed or blocked packet.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// The blockers the user must reconcile before sending.
    #[serde(default)]
    pub blockers: Vec<String>,
    /// Ref to the source-of-truth object family this packet projects.
    pub source_of_truth_ref: String,
    /// One-step "Why is this escalating, and what does it carry?" entrypoint; always present.
    pub explain_entrypoint_ref: String,
    /// The equivalent CLI / headless object id; always present.
    pub cli_object_ref: String,
    /// Ref to the conformance suite backing the packet.
    pub conformance_ref: String,
    /// Ref to the packet's supporting evidence.
    pub evidence_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl HandoffPacket {
    /// The component with the given id, if present.
    pub fn component(&self, component_id: &str) -> Option<&HandoffComponent> {
        self.components
            .iter()
            .find(|c| c.component_id == component_id)
    }

    /// Whether any component is included but cannot safely leave the machine for this mode.
    pub fn has_blocking_component(&self) -> bool {
        self.components
            .iter()
            .any(|c| c.disposition(self.mode) == ComponentDisposition::Blocking)
    }

    /// Whether any component is policy-locked.
    pub fn has_policy_locked_component(&self) -> bool {
        self.components.iter().any(|c| c.policy_locked)
    }

    /// Whether the packet had to redact, withhold, or downgrade any component to fit the mode.
    pub fn is_narrowed(&self) -> bool {
        self.components.iter().any(|c| {
            matches!(
                c.disposition(self.mode),
                ComponentDisposition::Redacted | ComponentDisposition::Withheld
            ) || c.lineage_downgraded
        })
    }

    /// Whether the exact-build / finding-code / repair-id lineage is complete across the packet.
    pub fn lineage_is_complete(&self) -> bool {
        !self.exact_build_id.trim().is_empty()
            && !self.components.is_empty()
            && self
                .components
                .iter()
                .all(|c| !c.lineage_ref.trim().is_empty() && !c.source_ref.trim().is_empty())
    }

    /// The handoff status recomputed from the components' dispositions.
    ///
    /// A blocking component dominates a policy lock, which dominates a redaction / exclusion / downgrade; a
    /// clean packet is ready to share.
    pub fn computed_status(&self) -> HandoffStatus {
        if self.has_blocking_component() {
            HandoffStatus::SendBlocked
        } else if self.has_policy_locked_component() {
            HandoffStatus::PolicyLocked
        } else if self.is_narrowed() {
            HandoffStatus::RedactionNarrowed
        } else {
            HandoffStatus::ReadyToShare
        }
    }

    /// The presentation the gate permits this packet to publish.
    pub fn effective_presentation(&self) -> HandoffPresentation {
        self.computed_status().presentation_ceiling()
    }

    /// Whether the packet presents as ready to share.
    pub fn is_ready_to_share(&self) -> bool {
        self.effective_presentation() == HandoffPresentation::ReadyToShare
    }

    /// The headline downgrade reasons recomputed from the components.
    pub fn computed_downgrade_reasons(&self) -> Vec<HandoffDowngradeReason> {
        HandoffDowngradeReason::ALL
            .into_iter()
            .filter(|reason| match reason {
                HandoffDowngradeReason::DataClassRedactedForMode => self
                    .components
                    .iter()
                    .any(|c| c.included && c.redaction_posture.is_redacted()),
                HandoffDowngradeReason::ComponentExcludedForMode => self
                    .components
                    .iter()
                    .any(|c| c.excluded_for_mode(self.mode)),
                HandoffDowngradeReason::PolicyLockedDataClass => {
                    self.components.iter().any(|c| c.policy_locked)
                }
                HandoffDowngradeReason::LineageDowngraded => {
                    self.components.iter().any(|c| c.lineage_downgraded)
                }
                HandoffDowngradeReason::SendBlockedUnsafeContent => self.has_blocking_component(),
            })
            .collect()
    }

    /// Whether the packet carries its own non-empty one-step explain and CLI-equivalent refs.
    pub fn has_one_step_explainability(&self) -> bool {
        !self.explain_entrypoint_ref.trim().is_empty() && !self.cli_object_ref.trim().is_empty()
    }

    /// Whether the recorded status, presentation, reasons, lineage attestation, and blocked flag agree
    /// with the gate.
    pub fn gate_consistent(&self) -> bool {
        self.status == self.computed_status()
            && self.presentation == self.effective_presentation()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.lineage_complete == self.lineage_is_complete()
            && self.blocked_before_send == self.effective_presentation().warns_before_send()
    }
}

/// One binding wiring a downstream surface to this registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: HandoffConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet-family id this surface ingests.
    pub packet_id_ref: String,
    /// True when the surface ingests this registry rather than a parallel list.
    pub ingests_registry: bool,
    /// True when the surface preserves the handoff vocabulary verbatim.
    pub preserves_handoff_vocabulary: bool,
    /// True when the surface preserves the packet and component ids rather than reminting them.
    pub preserves_object_ids: bool,
    /// True when the surface preserves the exact-build / finding-code / repair-id lineage by reference.
    pub preserves_lineage: bool,
    /// True when the surface keeps each component's data class and redaction posture visible.
    pub keeps_data_classes_visible: bool,
    /// True when the surface narrows automatically as packets are narrowed or blocked.
    pub narrows_on_downgrade: bool,
    /// True when raw secret, dump, or payload material is excluded from the binding.
    pub raw_material_excluded: bool,
}

impl HandoffConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.ingests_registry
            && self.preserves_handoff_vocabulary
            && self.preserves_object_ids
            && self.preserves_lineage
            && self.keeps_data_classes_visible
            && self.narrows_on_downgrade
            && self.raw_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportabilityHandoffSummary {
    /// Total handoff packets.
    pub total_packets: usize,
    /// Packets that present as ready to share.
    pub ready_to_share_packets: usize,
    /// Packets the gate narrowed.
    pub narrowed_packets: usize,
    /// Packets the gate blocked from sending.
    pub send_blocked_packets: usize,
    /// Packets whose headline status is policy-locked.
    pub policy_locked_packets: usize,
    /// Packets that take the local self-diagnosis path.
    pub local_self_diagnosis_packets: usize,
    /// Total handoff components across all packets.
    pub total_components: usize,
    /// Components carried as redacted summaries.
    pub redacted_components: usize,
    /// Components withheld from their packet's mode.
    pub withheld_components: usize,
    /// Components withheld by a policy lock.
    pub policy_locked_components: usize,
    /// Components that block their packet's send.
    pub blocking_components: usize,
}

/// A redaction-safe export row projected from a handoff component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportabilityHandoffComponentRow {
    /// Component id.
    pub component_id: String,
    /// Component-kind token.
    pub component_kind: String,
    /// Data-class token.
    pub data_class: String,
    /// Redaction-posture token.
    pub redaction_posture: String,
    /// Disposition token for the packet's mode.
    pub disposition: String,
    /// Lineage token preserved by the component.
    pub lineage_ref: String,
    /// Whether the component is policy-locked.
    pub policy_locked: bool,
    /// Whether the component carries a downgraded lineage.
    pub lineage_downgraded: bool,
}

/// A redaction-safe export row projected from a handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportabilityHandoffExportRow {
    /// Packet id.
    pub packet_id: String,
    /// Visible incident ref.
    pub incident_ref: String,
    /// Visible, copyable exact-build id.
    pub exact_build_id: String,
    /// Handoff-mode token.
    pub mode: String,
    /// Handoff-status token.
    pub status: String,
    /// Published-presentation token.
    pub presentation: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Whether the exact-build / finding-code / repair-id lineage is complete.
    pub lineage_complete: bool,
    /// Whether each component's data class and redaction posture stays visible.
    pub data_classes_visible: bool,
    /// Whether the packet warns and blocks before sending.
    pub blocked_before_send: bool,
    /// Projected component rows, in order.
    pub components: Vec<M5SupportabilityHandoffComponentRow>,
    /// One-step explain entrypoint ref.
    pub explain_entrypoint_ref: String,
    /// CLI / headless equivalent object id.
    pub cli_object_ref: String,
    /// Source-of-truth ref.
    pub source_of_truth_ref: String,
    /// Whether the packet presents as ready to share.
    pub ready_to_share: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the registry — the canonical handoff index downstream surfaces
/// render instead of restating each escalation by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportabilityHandoffExportProjection {
    /// Packet-family id this projection was produced from.
    pub packet_id: String,
    /// Packet family as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5SupportabilityHandoffExportRow>,
    /// Whether every packet's published decision agrees with the gate.
    pub all_packets_gate_consistent: bool,
    /// Whether every packet keeps its component data classes visible.
    pub all_data_classes_visible: bool,
    /// Packets that present as ready to share.
    pub ready_to_share_count: usize,
    /// Packets the gate narrowed.
    pub narrowed_count: usize,
    /// Packets the gate blocked from sending.
    pub send_blocked_count: usize,
}

/// The typed supportability-handoff-packets registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportabilityHandoffPackets {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet-family identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet family.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed handoff-mode vocabulary.
    pub handoff_modes: Vec<HandoffMode>,
    /// Closed component-kind vocabulary.
    pub component_kinds: Vec<HandoffComponentKind>,
    /// Closed data-class vocabulary.
    pub data_classes: Vec<HandoffDataClass>,
    /// Closed redaction-posture vocabulary.
    pub redaction_postures: Vec<RedactionPosture>,
    /// Closed handoff-status vocabulary.
    pub handoff_statuses: Vec<HandoffStatus>,
    /// Closed presentation vocabulary.
    pub presentations: Vec<HandoffPresentation>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<HandoffDowngradeReason>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<HandoffConsumerSurface>,
    /// Per-mode policy declarations, one per mode.
    #[serde(default)]
    pub mode_policies: Vec<HandoffModePolicy>,
    /// Handoff packets, one per blocked-user escalation scenario.
    #[serde(default)]
    pub packets: Vec<HandoffPacket>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<HandoffConsumerBinding>,
    /// Summary counts.
    pub summary: M5SupportabilityHandoffSummary,
}

impl M5SupportabilityHandoffPackets {
    /// Returns the packet with the given id.
    pub fn packet(&self, packet_id: &str) -> Option<&HandoffPacket> {
        self.packets.iter().find(|p| p.packet_id == packet_id)
    }

    /// The per-mode policy for the given mode, if present.
    pub fn mode_policy(&self, mode: HandoffMode) -> Option<&HandoffModePolicy> {
        self.mode_policies.iter().find(|p| p.mode == mode)
    }

    /// Packets that present as ready to share.
    pub fn ready_to_share_packets(&self) -> impl Iterator<Item = &HandoffPacket> {
        self.packets
            .iter()
            .filter(|p| p.effective_presentation() == HandoffPresentation::ReadyToShare)
    }

    /// Packets the gate narrowed.
    pub fn narrowed_packets(&self) -> impl Iterator<Item = &HandoffPacket> {
        self.packets
            .iter()
            .filter(|p| p.effective_presentation() == HandoffPresentation::Narrowed)
    }

    /// Packets the gate blocked from sending.
    pub fn send_blocked_packets(&self) -> impl Iterator<Item = &HandoffPacket> {
        self.packets
            .iter()
            .filter(|p| p.effective_presentation() == HandoffPresentation::SendBlocked)
    }

    /// Whether a consumer binding preserves this registry for the given surface.
    pub fn has_binding_for(&self, surface: HandoffConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every packet's recorded decision agrees with the gate.
    pub fn all_packets_gate_consistent(&self) -> bool {
        self.packets.iter().all(HandoffPacket::gate_consistent)
    }

    /// Whether every packet keeps its component data classes visible.
    pub fn all_data_classes_visible(&self) -> bool {
        self.packets.iter().all(|p| p.data_classes_visible)
    }

    /// Recomputes the summary block from the packets.
    pub fn computed_summary(&self) -> M5SupportabilityHandoffSummary {
        let count_presentation = |decision: HandoffPresentation| {
            self.packets
                .iter()
                .filter(|p| p.effective_presentation() == decision)
                .count()
        };
        let mut total_components = 0usize;
        let mut redacted = 0usize;
        let mut withheld = 0usize;
        let mut policy_locked_components = 0usize;
        let mut blocking = 0usize;
        for packet in &self.packets {
            total_components += packet.components.len();
            for component in &packet.components {
                if component.policy_locked {
                    policy_locked_components += 1;
                }
                match component.disposition(packet.mode) {
                    ComponentDisposition::Redacted => redacted += 1,
                    ComponentDisposition::Withheld => withheld += 1,
                    ComponentDisposition::Blocking => blocking += 1,
                    ComponentDisposition::Carried => {}
                }
            }
        }
        M5SupportabilityHandoffSummary {
            total_packets: self.packets.len(),
            ready_to_share_packets: count_presentation(HandoffPresentation::ReadyToShare),
            narrowed_packets: count_presentation(HandoffPresentation::Narrowed),
            send_blocked_packets: count_presentation(HandoffPresentation::SendBlocked),
            policy_locked_packets: self
                .packets
                .iter()
                .filter(|p| p.computed_status() == HandoffStatus::PolicyLocked)
                .count(),
            local_self_diagnosis_packets: self
                .packets
                .iter()
                .filter(|p| p.mode.is_local_self_diagnosis())
                .count(),
            total_components,
            redacted_components: redacted,
            withheld_components: withheld,
            policy_locked_components,
            blocking_components: blocking,
        }
    }

    /// Produces the handoff index downstream surfaces render instead of restating each escalation by hand.
    pub fn export_projection(&self) -> M5SupportabilityHandoffExportProjection {
        let rows = self
            .packets
            .iter()
            .map(|p| M5SupportabilityHandoffExportRow {
                packet_id: p.packet_id.clone(),
                incident_ref: p.incident_ref.clone(),
                exact_build_id: p.exact_build_id.clone(),
                mode: p.mode.as_str().to_owned(),
                status: p.status.as_str().to_owned(),
                presentation: p.presentation.as_str().to_owned(),
                downgrade_reasons: p
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                lineage_complete: p.lineage_complete,
                data_classes_visible: p.data_classes_visible,
                blocked_before_send: p.blocked_before_send,
                components: p
                    .components
                    .iter()
                    .map(|c| M5SupportabilityHandoffComponentRow {
                        component_id: c.component_id.clone(),
                        component_kind: c.component_kind.as_str().to_owned(),
                        data_class: c.data_class.as_str().to_owned(),
                        redaction_posture: c.redaction_posture.as_str().to_owned(),
                        disposition: c.disposition(p.mode).as_str().to_owned(),
                        lineage_ref: c.lineage_ref.clone(),
                        policy_locked: c.policy_locked,
                        lineage_downgraded: c.lineage_downgraded,
                    })
                    .collect(),
                explain_entrypoint_ref: p.explain_entrypoint_ref.clone(),
                cli_object_ref: p.cli_object_ref.clone(),
                source_of_truth_ref: p.source_of_truth_ref.clone(),
                ready_to_share: p.is_ready_to_share(),
                summary: format!(
                    "{}: incident {} on build {} via {} ({}), {} components, presentation {}",
                    p.packet_id,
                    p.incident_ref,
                    p.exact_build_id,
                    p.mode.as_str(),
                    p.status.as_str(),
                    p.components.len(),
                    p.presentation.as_str()
                ),
            })
            .collect();
        M5SupportabilityHandoffExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_packets_gate_consistent: self.all_packets_gate_consistent(),
            all_data_classes_visible: self.all_data_classes_visible(),
            ready_to_share_count: self.ready_to_share_packets().count(),
            narrowed_count: self.narrowed_packets().count(),
            send_blocked_count: self.send_blocked_packets().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact handoff registry.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SupportabilityHandoffSupportExport {
        M5SupportabilityHandoffSupportExport {
            record_kind: M5_SUPPORTABILITY_HANDOFF_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORTABILITY_HANDOFF_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_material_excluded: true,
            registry: self.clone(),
        }
    }

    /// Validates the packet family, returning every violation found.
    pub fn validate(&self) -> Vec<M5SupportabilityHandoffViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        for mode in HandoffMode::ALL {
            match self.mode_policy(mode) {
                None => violations.push(M5SupportabilityHandoffViolation::ModePolicyMissing {
                    mode: mode.as_str(),
                }),
                Some(policy) if !policy.is_mode_consistent() => {
                    violations.push(M5SupportabilityHandoffViolation::ModePolicyDrift {
                        mode: mode.as_str(),
                    });
                }
                Some(_) => {}
            }
        }

        let mut seen_ids = BTreeSet::new();
        for packet in &self.packets {
            if !seen_ids.insert(packet.packet_id.clone()) {
                violations.push(M5SupportabilityHandoffViolation::DuplicatePacket {
                    packet_id: packet.packet_id.clone(),
                });
            }
            self.validate_packet(packet, &mut violations);
        }

        for surface in HandoffConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5SupportabilityHandoffViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5SupportabilityHandoffViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SupportabilityHandoffViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SupportabilityHandoffViolation>) {
        if self.schema_version != M5_SUPPORTABILITY_HANDOFF_SCHEMA_VERSION {
            violations.push(M5SupportabilityHandoffViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_SUPPORTABILITY_HANDOFF_RECORD_KIND {
            violations.push(M5SupportabilityHandoffViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportabilityHandoffViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "handoff_modes",
                self.handoff_modes == HandoffMode::ALL.to_vec(),
            ),
            (
                "component_kinds",
                self.component_kinds == HandoffComponentKind::ALL.to_vec(),
            ),
            (
                "data_classes",
                self.data_classes == HandoffDataClass::ALL.to_vec(),
            ),
            (
                "redaction_postures",
                self.redaction_postures == RedactionPosture::ALL.to_vec(),
            ),
            (
                "handoff_statuses",
                self.handoff_statuses == HandoffStatus::ALL.to_vec(),
            ),
            (
                "presentations",
                self.presentations == HandoffPresentation::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == HandoffDowngradeReason::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == HandoffConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5SupportabilityHandoffViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_packet(
        &self,
        packet: &HandoffPacket,
        violations: &mut Vec<M5SupportabilityHandoffViolation>,
    ) {
        for (field, value) in [
            ("packet_id", &packet.packet_id),
            ("title", &packet.title),
            ("incident_ref", &packet.incident_ref),
            ("exact_build_id", &packet.exact_build_id),
            ("source_of_truth_ref", &packet.source_of_truth_ref),
            ("explain_entrypoint_ref", &packet.explain_entrypoint_ref),
            ("cli_object_ref", &packet.cli_object_ref),
            ("conformance_ref", &packet.conformance_ref),
            ("evidence_ref", &packet.evidence_ref),
            ("note", &packet.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportabilityHandoffViolation::EmptyField {
                    id: packet.packet_id.clone(),
                    field_name: field,
                });
            }
        }

        // The exact-build id is always visible and copyable, so a blocked user can quote it to support.
        if !packet.build_id_copyable {
            violations.push(M5SupportabilityHandoffViolation::BuildIdNotCopyable {
                packet_id: packet.packet_id.clone(),
            });
        }

        // Every component keeps its data class and redaction posture visible — never a monolithic export.
        if !packet.data_classes_visible {
            violations.push(M5SupportabilityHandoffViolation::DataClassesNotVisible {
                packet_id: packet.packet_id.clone(),
            });
        }

        // No raw secret bodies, raw dumps, or raw payloads may be carried, ever.
        if !packet.raw_material_excluded {
            violations.push(M5SupportabilityHandoffViolation::RawMaterialNotExcluded {
                packet_id: packet.packet_id.clone(),
            });
        }

        // Every packet must carry its one-step explain entry and its CLI / headless equivalent.
        if !packet.has_one_step_explainability() {
            violations.push(
                M5SupportabilityHandoffViolation::MissingOneStepExplainability {
                    packet_id: packet.packet_id.clone(),
                },
            );
        }

        // A handoff packet is never empty: it always joins at least one source object.
        if packet.components.is_empty() {
            violations.push(M5SupportabilityHandoffViolation::NoComponents {
                packet_id: packet.packet_id.clone(),
            });
        }

        self.validate_components(packet, violations);
        self.validate_gate(packet, violations);
    }

    fn validate_components(
        &self,
        packet: &HandoffPacket,
        violations: &mut Vec<M5SupportabilityHandoffViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for component in &packet.components {
            if !seen.insert(component.component_id.clone()) {
                violations.push(M5SupportabilityHandoffViolation::DuplicateComponent {
                    packet_id: packet.packet_id.clone(),
                    component_id: component.component_id.clone(),
                });
            }
            if !component.is_well_formed() {
                violations.push(M5SupportabilityHandoffViolation::ComponentIncomplete {
                    packet_id: packet.packet_id.clone(),
                    component_id: component.component_id.clone(),
                });
            }
            // A withheld component must be withheld for a legitimate reason — never a silent drop.
            if !component.is_justified_withholding(packet.mode) {
                violations.push(
                    M5SupportabilityHandoffViolation::UnjustifiedWithheldComponent {
                        packet_id: packet.packet_id.clone(),
                        component_id: component.component_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_gate(
        &self,
        packet: &HandoffPacket,
        violations: &mut Vec<M5SupportabilityHandoffViolation>,
    ) {
        // The recorded handoff status must equal the recomputed status.
        let computed_status = packet.computed_status();
        if packet.status != computed_status {
            violations.push(M5SupportabilityHandoffViolation::StatusMismatch {
                packet_id: packet.packet_id.clone(),
                declared: packet.status.as_str(),
                computed: computed_status.as_str(),
            });
        }

        // The published presentation must equal the gate's recomputed decision.
        let effective = packet.effective_presentation();
        if packet.presentation != effective {
            violations.push(M5SupportabilityHandoffViolation::OverstatedPresentation {
                packet_id: packet.packet_id.clone(),
                published: packet.presentation.as_str(),
                computed: effective.as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &packet.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5SupportabilityHandoffViolation::DuplicateDowngradeReason {
                    packet_id: packet.packet_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }
        if packet.downgrade_reasons != packet.computed_downgrade_reasons() {
            violations.push(M5SupportabilityHandoffViolation::DowngradeReasonsMismatch {
                packet_id: packet.packet_id.clone(),
            });
        }

        // The lineage-complete attestation must equal the recomputed value, so the exact-build /
        // finding-code / repair-id lineage is never silently dropped.
        if packet.lineage_complete != packet.lineage_is_complete() {
            violations.push(
                M5SupportabilityHandoffViolation::LineageCompleteAttestationMismatch {
                    packet_id: packet.packet_id.clone(),
                },
            );
        }

        // A send-blocked packet must warn before anything leaves; a non-blocked one must not claim it.
        if packet.blocked_before_send != effective.warns_before_send() {
            violations.push(
                M5SupportabilityHandoffViolation::BlockedBeforeSendMismatch {
                    packet_id: packet.packet_id.clone(),
                },
            );
        }

        // A narrowed or blocked packet always carries a caveat naming why it is not cleanly ready.
        if effective.requires_attention() && packet.caveats.is_empty() {
            violations.push(M5SupportabilityHandoffViolation::EmptyField {
                id: packet.packet_id.clone(),
                field_name: "caveats",
            });
        }

        // A send-blocked packet always names the blockers the user must reconcile.
        if computed_status.requires_blockers() && packet.blockers.is_empty() {
            violations.push(M5SupportabilityHandoffViolation::EmptyField {
                id: packet.packet_id.clone(),
                field_name: "blockers",
            });
        }
    }
}

/// A validation violation for the supportability-handoff-packets registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SupportabilityHandoffViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Packet or family id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A packet id appears more than once.
    DuplicatePacket {
        /// Duplicate packet id.
        packet_id: String,
    },
    /// A component id appears more than once within a packet.
    DuplicateComponent {
        /// Packet id.
        packet_id: String,
        /// Duplicate component id.
        component_id: String,
    },
    /// A component is missing its id, refs, label, or summary.
    ComponentIncomplete {
        /// Packet id.
        packet_id: String,
        /// Component id.
        component_id: String,
    },
    /// A component is withheld without a legitimate reason.
    UnjustifiedWithheldComponent {
        /// Packet id.
        packet_id: String,
        /// Component id.
        component_id: String,
    },
    /// A packet's build id is not marked copyable.
    BuildIdNotCopyable {
        /// Packet id.
        packet_id: String,
    },
    /// A packet does not attest that data classes stay visible.
    DataClassesNotVisible {
        /// Packet id.
        packet_id: String,
    },
    /// A packet does not attest that raw secret / dump / payload material is excluded.
    RawMaterialNotExcluded {
        /// Packet id.
        packet_id: String,
    },
    /// A packet is missing its one-step explain entry or CLI-equivalent object id.
    MissingOneStepExplainability {
        /// Packet id.
        packet_id: String,
    },
    /// A packet joins no source objects.
    NoComponents {
        /// Packet id.
        packet_id: String,
    },
    /// The recorded handoff status disagrees with the recomputed status.
    StatusMismatch {
        /// Packet id.
        packet_id: String,
        /// Declared status token.
        declared: &'static str,
        /// Computed status token.
        computed: &'static str,
    },
    /// A packet publishes a presentation cleaner than the gate computes.
    OverstatedPresentation {
        /// Packet id.
        packet_id: String,
        /// Published presentation token.
        published: &'static str,
        /// Computed effective presentation token.
        computed: &'static str,
    },
    /// A packet lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Packet id.
        packet_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A packet's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Packet id.
        packet_id: String,
    },
    /// A packet's lineage-complete attestation disagrees with the recomputed value.
    LineageCompleteAttestationMismatch {
        /// Packet id.
        packet_id: String,
    },
    /// A packet's blocked-before-send flag disagrees with the gate.
    BlockedBeforeSendMismatch {
        /// Packet id.
        packet_id: String,
    },
    /// A required per-mode policy is missing.
    ModePolicyMissing {
        /// Mode token.
        mode: &'static str,
    },
    /// A per-mode policy disagrees with the mode's allowed classes or default redaction.
    ModePolicyDrift {
        /// Mode token.
        mode: &'static str,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints registry truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the packets.
    SummaryMismatch,
}

impl fmt::Display for M5SupportabilityHandoffViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicatePacket { packet_id } => write!(f, "duplicate packet id {packet_id}"),
            Self::DuplicateComponent {
                packet_id,
                component_id,
            } => write!(
                f,
                "packet {packet_id} lists component {component_id} more than once"
            ),
            Self::ComponentIncomplete {
                packet_id,
                component_id,
            } => write!(
                f,
                "packet {packet_id} component {component_id} is missing its id, refs, label, or summary"
            ),
            Self::UnjustifiedWithheldComponent {
                packet_id,
                component_id,
            } => write!(
                f,
                "packet {packet_id} component {component_id} is withheld without a policy lock or a data-class limit"
            ),
            Self::BuildIdNotCopyable { packet_id } => {
                write!(f, "packet {packet_id} build id is not copyable")
            }
            Self::DataClassesNotVisible { packet_id } => write!(
                f,
                "packet {packet_id} does not keep component data classes and redaction posture visible"
            ),
            Self::RawMaterialNotExcluded { packet_id } => write!(
                f,
                "packet {packet_id} does not attest raw secret/dump/payload material is excluded"
            ),
            Self::MissingOneStepExplainability { packet_id } => write!(
                f,
                "packet {packet_id} is missing its one-step explain entry or CLI-equivalent object id"
            ),
            Self::NoComponents { packet_id } => {
                write!(f, "packet {packet_id} joins no source objects")
            }
            Self::StatusMismatch {
                packet_id,
                declared,
                computed,
            } => write!(
                f,
                "packet {packet_id} records status {declared} but the gate computes {computed}"
            ),
            Self::OverstatedPresentation {
                packet_id,
                published,
                computed,
            } => write!(
                f,
                "packet {packet_id} publishes presentation {published} but the gate computes {computed}"
            ),
            Self::DuplicateDowngradeReason { packet_id, reason } => {
                write!(f, "packet {packet_id} repeats downgrade reason {reason}")
            }
            Self::DowngradeReasonsMismatch { packet_id } => {
                write!(f, "packet {packet_id} downgrade reasons disagree with the gate")
            }
            Self::LineageCompleteAttestationMismatch { packet_id } => write!(
                f,
                "packet {packet_id} lineage-complete attestation disagrees with the recomputed value"
            ),
            Self::BlockedBeforeSendMismatch { packet_id } => write!(
                f,
                "packet {packet_id} blocked-before-send flag disagrees with the gate"
            ),
            Self::ModePolicyMissing { mode } => {
                write!(f, "missing per-mode policy for mode {mode}")
            }
            Self::ModePolicyDrift { mode } => write!(
                f,
                "per-mode policy for mode {mode} disagrees with its allowed classes or default redaction"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve registry truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the packets"),
        }
    }
}

impl Error for M5SupportabilityHandoffViolation {}

/// Stable record-kind tag for [`M5SupportabilityHandoffSupportExport`].
pub const M5_SUPPORTABILITY_HANDOFF_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_supportability_handoff_packets_support_export";

/// Support-export wrapper preserving the registry verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportabilityHandoffSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet-family id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw secret, dump, or payload material is excluded.
    pub raw_material_excluded: bool,
    /// Exact registry preserved by the export.
    pub registry: M5SupportabilityHandoffPackets,
}

impl M5SupportabilityHandoffSupportExport {
    /// Whether the export preserves the same packet-family id and a clean registry.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_SUPPORTABILITY_HANDOFF_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_SUPPORTABILITY_HANDOFF_SCHEMA_VERSION
            && self.packet_id_ref == self.registry.packet_id
            && self.raw_material_excluded
            && self.registry.validate().is_empty()
    }
}

/// Loads the embedded supportability-handoff-packets registry.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SupportabilityHandoffPackets`].
pub fn current_m5_supportability_handoff_packets(
) -> Result<M5SupportabilityHandoffPackets, serde_json::Error> {
    serde_json::from_str(M5_SUPPORTABILITY_HANDOFF_JSON)
}

#[cfg(test)]
mod tests;
