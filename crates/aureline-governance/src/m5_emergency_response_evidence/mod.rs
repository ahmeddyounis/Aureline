//! Typed register of emergency-response evidence per protected M5 ecosystem/release lane.
//!
//! The sibling [`m5_boundary_and_upstream_durability`](crate::m5_boundary_and_upstream_durability)
//! matrix records, per asset lane, the emergency signing/registry/security authority, and the
//! [`m5_release_authority_continuity`](crate::m5_release_authority_continuity) register makes each
//! protected *authority lane* inspectable. Neither records the *emergency-response evidence* a
//! protected M5 ecosystem/release lane produces when something goes wrong — the signed security
//! advisory, the extension/provider revocation packet, the emergency-disable bundle, and the
//! high-severity postmortem — nor whether that evidence actually reached the hosted, mirror, and
//! offline customers that claim it, whether the action is attributable and reversible where policy
//! allows, whether a break-glass action carried its audit markers and post-incident reconciliation,
//! and whether the evidence is linked to the release artifact-graph and support exports rather than
//! a side channel.
//!
//! This module is that emergency-response evidence layer. For every protected M5 ecosystem/release
//! lane and emergency packet it records one [`EmergencyResponseRecord`] that states, in one
//! copy-safe record:
//!
//! - the **packet template** ([`PacketTemplate`]): a signed advisory/revocation/disable/postmortem
//!   packet, bound and digested;
//! - the **distribution reach** ([`DistributionReach`]): the hosted, mirror, and offline channels,
//!   each claimed and propagated, so a mirror or offline customer is never left on a hosted-only
//!   path — the headline guardrail;
//! - the **attribution** ([`Attribution`]): the emergency action is attributable to an authorized
//!   actor;
//! - the **reversibility** ([`Reversibility`]): a reversal runbook where policy permits reversal;
//! - the **audit trail** ([`AuditTrail`]): audit markers and, for break-glass or high-severity
//!   actions, post-incident reconciliation, so a break-glass action never bypasses the audit and
//!   reconciliation rules;
//! - the **evidence linkage** ([`EvidenceLinkage`]): the release artifact-graph identity and
//!   support-export packet, not a side channel.
//!
//! Each record also carries a [`scan_posture`](EmergencyResponseRecord::scan_posture) (what the
//! response scan found) and a [`surface_posture`](EmergencyResponseRecord::surface_posture) (what
//! the service-health/release-center/support surface shows). The two **must agree**: a record may
//! never show a clean surface over a scan that found gaps, so a green emergency-response card can
//! never mask a mirror/offline customer that never received the advisory, an unattributable
//! break-glass action, or a side-channel-only disable.
//!
//! A record is [`ResponseState::Cleared`] only when the packet template is bound, every claimed
//! channel carries current evidence, the action is attributable, a reversible action has its
//! reversal runbook, the audit trail is complete (markers present and reconciliation done where
//! required), the evidence is linked to release/support, the proof is fresh, and the owner signed.
//! Otherwise it narrows on the *specific* axis that thinned out — a template gap, a
//! distribution-reach gap, an attribution gap, a reversibility gap, an audit gap, a linkage gap, or
//! stale proof — never collapsing to one global flag. A narrowed record drops its
//! [`EmergencyResponseRecord::effective_label`] below the launch cutline and may never publish an
//! effective label wider than the one it declares.
//!
//! The [`ResponseRule`] set names the closed conditions that gate promotion. An *inherited*
//! narrowing — a subject whose declared label already sits below the cutline, or a gap held by an
//! unexpired waiver — is gated upstream and does not itself hold promotion; a *response* failure on
//! a subject whose declared label is still at or above the cutline holds promotion through a
//! shiproom stop rule, recorded in [`EmergencyResponseEvidenceRegister::publication`] — a protected
//! lane whose advisory/revocation/disable evidence did not reach a claimed mirror/offline customer,
//! or whose break-glass action bypassed audit/reconciliation, cannot widen a stable claim without
//! coverage. The cross-cutting [`ScanSurfaceParity`] block summarizes scan/surface agreement over
//! every subject.
//!
//! The register is checked in at `artifacts/governance/m5-emergency-response-evidence.json` and
//! embedded here, so this typed consumer and the CI gate agree on every record without a cargo
//! build in CI. The model is metadata-only: every field is a typed state, a boolean flag, a small
//! count, a label, or an opaque ref. It carries no credential bodies, raw provider payloads, actor
//! identities beyond opaque role refs, signatures, or advisory bodies. Date arithmetic (recomputing
//! proof and waiver freshness against an `as_of` date) lives in the CI gate and the integration
//! test; this model enforces the invariants that hold regardless of the clock: scan/surface parity,
//! the no-widening ceiling, control/fact consistency, reason/state coherence, summary agreement, and
//! the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket, SupportClass, Waiver,
};
use crate::m5_versioned_boundary_manifests::M5Family;

/// Supported register schema version.
pub const M5_EMERGENCY_RESPONSE_EVIDENCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_EMERGENCY_RESPONSE_EVIDENCE_RECORD_KIND: &str =
    "m5_emergency_response_evidence_register";

/// Repo-relative path to the checked-in register.
pub const M5_EMERGENCY_RESPONSE_EVIDENCE_PATH: &str =
    "artifacts/governance/m5-emergency-response-evidence.json";

/// Embedded checked-in register JSON.
pub const M5_EMERGENCY_RESPONSE_EVIDENCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-emergency-response-evidence.json"
));

/// The kind of emergency-response packet a record governs.
///
/// The same response truth is published for advisories, extension/provider revocations,
/// emergency-disable bundles, and high-severity postmortems — so a revocation that never reached a
/// mirror customer cannot hide behind a healthy advisory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketKind {
    /// A signed security advisory.
    SecurityAdvisory,
    /// An extension/provider revocation packet.
    ExtensionProviderRevocation,
    /// An emergency-disable bundle.
    EmergencyDisableBundle,
    /// A high-severity postmortem.
    HighSeverityPostmortem,
}

impl PacketKind {
    /// Every packet kind, in declaration order. Each must be exercised by at least one record.
    pub const ALL: [Self; 4] = [
        Self::SecurityAdvisory,
        Self::ExtensionProviderRevocation,
        Self::EmergencyDisableBundle,
        Self::HighSeverityPostmortem,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecurityAdvisory => "security_advisory",
            Self::ExtensionProviderRevocation => "extension_provider_revocation",
            Self::EmergencyDisableBundle => "emergency_disable_bundle",
            Self::HighSeverityPostmortem => "high_severity_postmortem",
        }
    }
}

/// The severity grade a response packet carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Informational: no reconciliation requirement on its own.
    Informational,
    /// Moderate: a risk is present but not high-severity on its own.
    Moderate,
    /// High: post-incident reconciliation is required.
    High,
    /// Critical: post-incident reconciliation is required.
    Critical,
}

impl Severity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Informational,
        Self::Moderate,
        Self::High,
        Self::Critical,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    /// True when the severity is high (`high`/`critical`): post-incident reconciliation is required.
    pub fn is_high(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

/// A response control dimension a record must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// Packet template: a signed, bound, digested packet.
    PacketTemplate,
    /// Distribution reach: every claimed hosted/mirror/offline channel carries current evidence.
    DistributionReach,
    /// Attribution: the emergency action is attributable to an authorized actor.
    Attribution,
    /// Reversibility: a reversible action carries its reversal runbook.
    Reversibility,
    /// Audit trail: audit markers and post-incident reconciliation where required.
    AuditTrail,
    /// Evidence linkage: linked to the release artifact-graph and support export.
    EvidenceLinkage,
    /// Scan/surface parity: the response scan and the governance surface agree.
    ScanSurfaceParity,
}

impl ControlDimension {
    /// Every control dimension, in declaration order. Every record declares each once.
    pub const ALL: [Self; 7] = [
        Self::PacketTemplate,
        Self::DistributionReach,
        Self::Attribution,
        Self::Reversibility,
        Self::AuditTrail,
        Self::EvidenceLinkage,
        Self::ScanSurfaceParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketTemplate => "packet_template",
            Self::DistributionReach => "distribution_reach",
            Self::Attribution => "attribution",
            Self::Reversibility => "reversibility",
            Self::AuditTrail => "audit_trail",
            Self::EvidenceLinkage => "evidence_linkage",
            Self::ScanSurfaceParity => "scan_surface_parity",
        }
    }
}

/// A distribution channel an emergency packet may need to reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistributionChannel {
    /// The hosted/SaaS distribution path.
    Hosted,
    /// A mirror/proxy distribution path.
    Mirror,
    /// An air-gapped/offline import path.
    Offline,
}

impl DistributionChannel {
    /// Every channel, in declaration order. Every record declares each once.
    pub const ALL: [Self; 3] = [Self::Hosted, Self::Mirror, Self::Offline];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hosted => "hosted",
            Self::Mirror => "mirror",
            Self::Offline => "offline",
        }
    }
}

/// Propagation state of one distribution channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelState {
    /// Current evidence has propagated to this channel.
    Propagated,
    /// Propagation to this claimed channel is still pending.
    Pending,
    /// This channel's evidence has aged out of its window.
    Stale,
    /// This channel is not claimed for the subject's customer profile.
    NotClaimed,
}

impl ChannelState {
    /// Every channel state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Propagated,
        Self::Pending,
        Self::Stale,
        Self::NotClaimed,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Propagated => "propagated",
            Self::Pending => "pending",
            Self::Stale => "stale",
            Self::NotClaimed => "not_claimed",
        }
    }
}

/// State of a record's signed packet template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateState {
    /// The packet template is bound, signed, and digested.
    Bound,
    /// The packet template is not yet bound (unsigned or undigested).
    Unbound,
}

impl TemplateState {
    /// Every template state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Bound, Self::Unbound];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::Unbound => "unbound",
        }
    }
}

/// Whether the emergency action is attributable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionState {
    /// The action is attributable to a named authority.
    Attributable,
    /// The action has no attributable actor.
    Unattributable,
}

impl AttributionState {
    /// Every attribution state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Attributable, Self::Unattributable];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attributable => "attributable",
            Self::Unattributable => "unattributable",
        }
    }
}

/// The reversibility posture of an emergency action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversibilityState {
    /// Policy permits reversal and a reversal runbook is attached.
    ReversibleWithRunbook,
    /// Policy does not permit reversal; the action is final by design.
    IrreversibleByPolicy,
    /// Policy permits reversal but no reversal runbook is attached.
    ReversalRuleMissing,
}

impl ReversibilityState {
    /// Every reversibility state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReversibleWithRunbook,
        Self::IrreversibleByPolicy,
        Self::ReversalRuleMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleWithRunbook => "reversible_with_runbook",
            Self::IrreversibleByPolicy => "irreversible_by_policy",
            Self::ReversalRuleMissing => "reversal_rule_missing",
        }
    }
}

/// State of a record's post-incident reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationState {
    /// Post-incident reconciliation is complete.
    Reconciled,
    /// Reconciliation is required but still pending.
    Pending,
    /// Reconciliation is not required for this action.
    NotRequired,
}

impl ReconciliationState {
    /// Every reconciliation state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Reconciled, Self::Pending, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconciled => "reconciled",
            Self::Pending => "pending",
            Self::NotRequired => "not_required",
        }
    }
}

/// Whether the evidence is linked to release/support truth or only a side channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkageState {
    /// Linked to the release artifact-graph and a support export.
    Linked,
    /// Announced only through a side channel, not linked to release/support evidence.
    SideChannelOnly,
}

impl LinkageState {
    /// Every linkage state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Linked, Self::SideChannelOnly];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::SideChannelOnly => "side_channel_only",
        }
    }
}

/// The posture a scan or a surface reports for a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    /// No response gap found.
    Clear,
    /// One or more response gaps found.
    GapsFound,
}

impl Posture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::Clear, Self::GapsFound];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::GapsFound => "gaps_found",
        }
    }
}

/// Satisfaction state of one control binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The control holds for this record.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this record.
    NotApplicable,
}

impl ControlState {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The state a record earns after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseState {
    /// Template, distribution, attribution, reversibility, audit, linkage, and proof all hold.
    Cleared,
    /// The signed packet template is not bound.
    NarrowedTemplate,
    /// A claimed hosted/mirror/offline channel did not receive current evidence.
    NarrowedDistribution,
    /// The emergency action is not attributable.
    NarrowedAttribution,
    /// A reversible action lacks its reversal runbook.
    NarrowedReversibility,
    /// Audit markers are missing or post-incident reconciliation is pending.
    NarrowedAudit,
    /// The evidence is linked only through a side channel.
    NarrowedLinkage,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The record is withdrawn.
    Withdrawn,
}

impl ResponseState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Cleared,
        Self::NarrowedTemplate,
        Self::NarrowedDistribution,
        Self::NarrowedAttribution,
        Self::NarrowedReversibility,
        Self::NarrowedAudit,
        Self::NarrowedLinkage,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cleared => "cleared",
            Self::NarrowedTemplate => "narrowed_template",
            Self::NarrowedDistribution => "narrowed_distribution",
            Self::NarrowedAttribution => "narrowed_attribution",
            Self::NarrowedReversibility => "narrowed_reversibility",
            Self::NarrowedAudit => "narrowed_audit",
            Self::NarrowedLinkage => "narrowed_linkage",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not cleared, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Cleared | Self::Withdrawn)
    }
}

/// A reason a record narrowed. Closed vocabulary; every reason is watched by a [`ResponseRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseReason {
    /// The signed packet template is not bound.
    PacketTemplateUnbound,
    /// A claimed mirror channel has not received current evidence.
    MirrorPropagationIncomplete,
    /// A claimed offline channel has not received current evidence.
    OfflineImportResponseMissing,
    /// A claimed channel's evidence aged out of its window.
    ChannelEvidenceStale,
    /// The emergency action has no attributable actor.
    ActionUnattributable,
    /// A reversible action lacks its reversal runbook.
    ReversalRuleMissing,
    /// A break-glass or high-severity action shipped without audit markers.
    AuditMarkersMissing,
    /// Post-incident reconciliation is required but still pending.
    ReconciliationPending,
    /// The evidence is linked only through a side channel.
    EvidenceLinkageMissing,
    /// The response proof packet aged past its freshness SLO.
    ResponseProofStale,
    /// No response proof packet is captured.
    ResponseProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl ResponseReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::PacketTemplateUnbound,
        Self::MirrorPropagationIncomplete,
        Self::OfflineImportResponseMissing,
        Self::ChannelEvidenceStale,
        Self::ActionUnattributable,
        Self::ReversalRuleMissing,
        Self::AuditMarkersMissing,
        Self::ReconciliationPending,
        Self::EvidenceLinkageMissing,
        Self::ResponseProofStale,
        Self::ResponseProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketTemplateUnbound => "packet_template_unbound",
            Self::MirrorPropagationIncomplete => "mirror_propagation_incomplete",
            Self::OfflineImportResponseMissing => "offline_import_response_missing",
            Self::ChannelEvidenceStale => "channel_evidence_stale",
            Self::ActionUnattributable => "action_unattributable",
            Self::ReversalRuleMissing => "reversal_rule_missing",
            Self::AuditMarkersMissing => "audit_markers_missing",
            Self::ReconciliationPending => "reconciliation_pending",
            Self::EvidenceLinkageMissing => "evidence_linkage_missing",
            Self::ResponseProofStale => "response_proof_stale",
            Self::ResponseProofMissing => "response_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active. Distribution reach (a
    /// customer left without the advisory) is the worst, then the audit/break-glass axis, then
    /// attribution, reversibility, linkage, the template, and finally the evidence-staleness axis.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            ResponseState::NarrowedDistribution => 0,
            ResponseState::NarrowedAudit => 1,
            ResponseState::NarrowedAttribution => 2,
            ResponseState::NarrowedReversibility => 3,
            ResponseState::NarrowedLinkage => 4,
            ResponseState::NarrowedTemplate => 5,
            _ => 6,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> ResponseState {
        match self {
            Self::PacketTemplateUnbound => ResponseState::NarrowedTemplate,
            Self::MirrorPropagationIncomplete
            | Self::OfflineImportResponseMissing
            | Self::ChannelEvidenceStale => ResponseState::NarrowedDistribution,
            Self::ActionUnattributable => ResponseState::NarrowedAttribution,
            Self::ReversalRuleMissing => ResponseState::NarrowedReversibility,
            Self::AuditMarkersMissing | Self::ReconciliationPending => ResponseState::NarrowedAudit,
            Self::EvidenceLinkageMissing => ResponseState::NarrowedLinkage,
            Self::ResponseProofStale
            | Self::ResponseProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ResponseState::NarrowedStale,
        }
    }

    /// The control dimension this reason belongs to.
    pub const fn dimension(self) -> ControlDimension {
        match self {
            Self::PacketTemplateUnbound => ControlDimension::PacketTemplate,
            Self::MirrorPropagationIncomplete
            | Self::OfflineImportResponseMissing
            | Self::ChannelEvidenceStale => ControlDimension::DistributionReach,
            Self::ActionUnattributable => ControlDimension::Attribution,
            Self::ReversalRuleMissing => ControlDimension::Reversibility,
            Self::AuditMarkersMissing | Self::ReconciliationPending => ControlDimension::AuditTrail,
            Self::EvidenceLinkageMissing => ControlDimension::EvidenceLinkage,
            Self::ResponseProofStale
            | Self::ResponseProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ControlDimension::ScanSurfaceParity,
        }
    }
}

/// An action a [`ResponseRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Bind and sign the packet template.
    BindPacketTemplate,
    /// Complete mirror propagation.
    CompleteMirrorPropagation,
    /// Complete the offline-import response.
    CompleteOfflineImportResponse,
    /// Refresh a stale channel's evidence.
    RefreshChannelEvidence,
    /// Record attribution for the action.
    RecordAttribution,
    /// Attach the reversal rule/runbook.
    AttachReversalRule,
    /// Attach the audit markers.
    AttachAuditMarkers,
    /// Complete the post-incident reconciliation.
    CompleteReconciliation,
    /// Link the evidence to the release artifact-graph and support export.
    LinkReleaseAndSupportEvidence,
    /// Refresh the response proof packet.
    RefreshResponseProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl ResponseAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::HoldPromotion,
        Self::BindPacketTemplate,
        Self::CompleteMirrorPropagation,
        Self::CompleteOfflineImportResponse,
        Self::RefreshChannelEvidence,
        Self::RecordAttribution,
        Self::AttachReversalRule,
        Self::AttachAuditMarkers,
        Self::CompleteReconciliation,
        Self::LinkReleaseAndSupportEvidence,
        Self::RefreshResponseProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::BindPacketTemplate => "bind_packet_template",
            Self::CompleteMirrorPropagation => "complete_mirror_propagation",
            Self::CompleteOfflineImportResponse => "complete_offline_import_response",
            Self::RefreshChannelEvidence => "refresh_channel_evidence",
            Self::RecordAttribution => "record_attribution",
            Self::AttachReversalRule => "attach_reversal_rule",
            Self::AttachAuditMarkers => "attach_audit_markers",
            Self::CompleteReconciliation => "complete_reconciliation",
            Self::LinkReleaseAndSupportEvidence => "link_release_and_support_evidence",
            Self::RefreshResponseProof => "refresh_response_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No response stop rule fires; promotion may proceed.
    Proceed,
    /// A response stop rule fires; hold promotion.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// The signed packet template bound to a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketTemplate {
    /// Template state.
    pub template_state: TemplateState,
    /// The packet kind (must match the record).
    pub packet_kind: PacketKind,
    /// True when the packet is signed.
    pub signed: bool,
    /// Reference to the packet template.
    pub template_ref: String,
    /// Reference to the bound packet digest (empty when unbound).
    pub digest_ref: String,
}

impl PacketTemplate {
    /// True when the template is not bound.
    pub fn is_unbound(&self) -> bool {
        self.template_state == TemplateState::Unbound
    }
}

/// One distribution channel's evidence for a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelEvidence {
    /// The distribution channel.
    pub channel: DistributionChannel,
    /// True when the channel is claimed for the subject's customer profile.
    pub claimed: bool,
    /// Propagation state of the channel.
    pub state: ChannelState,
    /// Reference to the channel evidence.
    pub evidence_ref: String,
}

/// The hosted/mirror/offline distribution reach for a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistributionReach {
    /// Per-channel evidence (one entry per channel).
    pub channels: Vec<ChannelEvidence>,
}

impl DistributionReach {
    /// Returns the entry for a channel, if present.
    pub fn channel(&self, channel: DistributionChannel) -> Option<&ChannelEvidence> {
        self.channels.iter().find(|c| c.channel == channel)
    }
}

/// The attribution of an emergency action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attribution {
    /// Attribution state.
    pub attribution_state: AttributionState,
    /// Opaque role ref of the acting authority (empty when unattributable).
    pub actor_ref: String,
    /// Reference to the authorization/break-glass approval.
    pub authorization_ref: String,
}

impl Attribution {
    /// True when the action is not attributable.
    pub fn is_unattributable(&self) -> bool {
        self.attribution_state == AttributionState::Unattributable
    }
}

/// The reversibility of an emergency action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reversibility {
    /// Reversibility state.
    pub reversibility_state: ReversibilityState,
    /// True when policy permits reversing this action class.
    pub policy_reversible: bool,
    /// Reference to the reversal runbook (empty when irreversible or missing).
    pub reversal_runbook_ref: String,
}

impl Reversibility {
    /// True when a reversible action lacks its reversal runbook.
    pub fn rule_missing(&self) -> bool {
        self.reversibility_state == ReversibilityState::ReversalRuleMissing
    }
}

/// The audit trail of an emergency action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditTrail {
    /// True when audit markers are present.
    pub audit_markers_present: bool,
    /// Reference to the audit markers (empty when absent).
    pub audit_marker_ref: String,
    /// Post-incident reconciliation state.
    pub reconciliation_state: ReconciliationState,
    /// Reference to the reconciliation record (empty when not reconciled).
    pub reconciliation_ref: String,
    /// Reference to the break-glass mutation journal.
    pub mutation_journal_ref: String,
}

/// The evidence linkage of a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceLinkage {
    /// Linkage state.
    pub linkage_state: LinkageState,
    /// Reference to the release artifact-graph identity (empty when side-channel-only).
    pub release_artifact_ref: String,
    /// Reference to the support-export packet (empty when side-channel-only).
    pub support_export_ref: String,
    /// Reference to the bundle digest.
    pub bundle_digest_ref: String,
}

impl EvidenceLinkage {
    /// True when the evidence is linked only through a side channel.
    pub fn is_side_channel_only(&self) -> bool {
        self.linkage_state == LinkageState::SideChannelOnly
    }
}

/// One response control binding on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseControl {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// One emergency-response evidence record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmergencyResponseRecord {
    /// Stable record id.
    pub record_id: String,
    /// The M5 family this lane serves.
    pub family: M5Family,
    /// The kind of emergency packet.
    pub packet_kind: PacketKind,
    /// Human-readable title.
    pub title: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this record declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the subject.
    pub support_class: SupportClass,
    /// The severity grade the packet carries.
    pub severity: Severity,
    /// True when the action was a break-glass action.
    pub is_break_glass: bool,
    /// Signed packet template.
    pub packet_template: PacketTemplate,
    /// Hosted/mirror/offline distribution reach.
    pub distribution_reach: DistributionReach,
    /// Action attribution.
    pub attribution: Attribution,
    /// Action reversibility.
    pub reversibility: Reversibility,
    /// Audit trail.
    pub audit_trail: AuditTrail,
    /// Evidence linkage.
    pub evidence_linkage: EvidenceLinkage,
    /// Per-dimension control bindings.
    pub controls: Vec<ResponseControl>,
    /// What the response scan found.
    pub scan_posture: Posture,
    /// What the service-health/release-center/support surface shows.
    pub surface_posture: Posture,
    /// Reference to the response scan.
    pub scan_ref: String,
    /// Reference to the governance surface.
    pub surface_ref: String,
    /// Proof packet grounding the record.
    pub proof_packet: ProofPacket,
    /// Optional waiver holding a gap provisionally.
    pub waiver: Option<Waiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// State earned after narrowing.
    pub continuity_state: ResponseState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ResponseReason>,
    /// The label the record effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this record (Help/About, service-health, release-center, support).
    pub surfaces: Vec<String>,
    /// Reviewable reason the record carries its state.
    pub rationale: String,
}

impl EmergencyResponseRecord {
    /// True when the record is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(ResponseReason::WaiverExpired)
    }

    /// True when the record carries the given active reason.
    pub fn has_active_reason(&self, reason: ResponseReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the record holds a cleared state.
    pub fn is_cleared(&self) -> bool {
        self.continuity_state == ResponseState::Cleared
    }

    /// True when the subject declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// True when the packet severity is high (`high`/`critical`).
    pub fn is_high_severity(&self) -> bool {
        self.severity.is_high()
    }

    /// True when this action requires post-incident reconciliation: break-glass or high-severity.
    pub fn requires_reconciliation(&self) -> bool {
        self.is_break_glass || self.is_high_severity()
    }

    /// True when the signed packet template is not bound.
    pub fn template_unbound(&self) -> bool {
        self.packet_template.is_unbound()
    }

    /// The distribution reasons implied by the claimed channels' propagation states.
    ///
    /// A claimed mirror channel that is still propagating yields
    /// [`ResponseReason::MirrorPropagationIncomplete`]; a claimed offline channel,
    /// [`ResponseReason::OfflineImportResponseMissing`]; any claimed channel whose evidence aged out
    /// (or a still-pending hosted channel) yields [`ResponseReason::ChannelEvidenceStale`].
    pub fn distribution_reasons(&self) -> BTreeSet<ResponseReason> {
        let mut out = BTreeSet::new();
        for c in &self.distribution_reach.channels {
            if !c.claimed {
                continue;
            }
            match (c.channel, c.state) {
                (_, ChannelState::Stale) => {
                    out.insert(ResponseReason::ChannelEvidenceStale);
                }
                (DistributionChannel::Mirror, ChannelState::Pending) => {
                    out.insert(ResponseReason::MirrorPropagationIncomplete);
                }
                (DistributionChannel::Offline, ChannelState::Pending) => {
                    out.insert(ResponseReason::OfflineImportResponseMissing);
                }
                (DistributionChannel::Hosted, ChannelState::Pending) => {
                    out.insert(ResponseReason::ChannelEvidenceStale);
                }
                _ => {}
            }
        }
        out
    }

    /// True when any claimed channel failed to receive current evidence.
    pub fn has_distribution_gap(&self) -> bool {
        !self.distribution_reasons().is_empty()
    }

    /// True when the emergency action is not attributable.
    pub fn unattributable(&self) -> bool {
        self.attribution.is_unattributable()
    }

    /// True when a reversible action lacks its reversal runbook.
    pub fn reversal_rule_missing(&self) -> bool {
        self.reversibility.rule_missing()
    }

    /// True when a break-glass or high-severity action shipped without audit markers.
    pub fn audit_markers_missing(&self) -> bool {
        !self.audit_trail.audit_markers_present
    }

    /// True when post-incident reconciliation is required but still pending.
    pub fn reconciliation_pending(&self) -> bool {
        self.requires_reconciliation()
            && self.audit_trail.reconciliation_state == ReconciliationState::Pending
    }

    /// True when the evidence is linked only through a side channel.
    pub fn linkage_missing(&self) -> bool {
        self.evidence_linkage.is_side_channel_only()
    }

    /// True when any structural response gap (other than proof/sign-off) is present.
    pub fn has_response_gap(&self) -> bool {
        self.template_unbound()
            || self.has_distribution_gap()
            || self.unattributable()
            || self.reversal_rule_missing()
            || self.audit_markers_missing()
            || self.reconciliation_pending()
            || self.linkage_missing()
    }

    /// The expected control state for a dimension, derived from the subject's facts.
    pub fn expected_control_state(&self, dimension: ControlDimension) -> ControlState {
        let unsatisfied = match dimension {
            ControlDimension::PacketTemplate => self.template_unbound(),
            ControlDimension::DistributionReach => self.has_distribution_gap(),
            ControlDimension::Attribution => self.unattributable(),
            ControlDimension::Reversibility => self.reversal_rule_missing(),
            ControlDimension::AuditTrail => {
                self.audit_markers_missing() || self.reconciliation_pending()
            }
            ControlDimension::EvidenceLinkage => self.linkage_missing(),
            ControlDimension::ScanSurfaceParity => self.scan_posture != self.surface_posture,
        };
        if unsatisfied {
            ControlState::Unsatisfied
        } else {
            ControlState::Satisfied
        }
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> ResponseState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return ResponseState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => ResponseState::Cleared,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            ResponseState::Cleared => self.declared_label,
            ResponseState::Withdrawn => LifecycleLabel::Withdrawn,
            _ => {
                // Narrowing drops the subject below the cutline: take the less-supported of the
                // declared label and beta.
                if self.declared_label.rank() <= LifecycleLabel::Beta.rank() {
                    self.declared_label
                } else {
                    LifecycleLabel::Beta
                }
            }
        }
    }

    /// The posture implied by the record's state: gaps found iff narrowed.
    pub fn computed_posture(&self) -> Posture {
        if self.continuity_state.is_narrowed() {
            Posture::GapsFound
        } else {
            Posture::Clear
        }
    }

    /// True when the record may hold promotion: a release-blocking subject, narrowed by a response
    /// gap, declaring a label at or above the cutline, and not held by an unexpired waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.continuity_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }

    /// True when the scan and the surface agree.
    pub fn scan_surface_agree(&self) -> bool {
        self.scan_posture == self.surface_posture
    }
}

/// A closed stop-rule that gates promotion on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: ResponseReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: ResponseAction,
    /// True when the rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseCutline {
    /// The cutline level (`stable`).
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Description.
    pub description: String,
}

/// Canonical source registers this register binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContractRefs {
    /// Advisory-template register.
    pub advisory_template_ref: String,
    /// Extension/provider revocation register.
    pub revocation_register_ref: String,
    /// Emergency-disable bundle register.
    pub disable_bundle_ref: String,
    /// High-severity postmortem register.
    pub postmortem_register_ref: String,
    /// Mirror-propagation register.
    pub mirror_propagation_ref: String,
    /// Offline-import response register.
    pub offline_import_ref: String,
    /// Release artifact-graph.
    pub release_graph_ref: String,
    /// Support-export index.
    pub support_export_ref: String,
    /// Break-glass mutation journal.
    pub audit_journal_ref: String,
    /// Release-authority continuity register.
    pub continuity_register_ref: String,
    /// Shiproom gate register.
    pub shiproom_register_ref: String,
    /// Canonical M5 evidence index.
    pub m5_evidence_index_ref: String,
}

/// Promotion verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// Stable promotion-gate id.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PublicationDecision,
    /// Firing rule ids.
    pub blocking_rule_ids: Vec<String>,
    /// Offending record ids.
    pub blocking_record_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Cross-cutting scan/surface parity summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanSurfaceParity {
    /// Stable parity-gate id.
    pub parity_gate: String,
    /// Total subjects.
    pub subjects_total: usize,
    /// Subjects whose scan and surface agree.
    pub subjects_in_agreement: usize,
    /// Subjects whose scan and surface disagree.
    pub subjects_in_disagreement: usize,
    /// Subjects whose surface reports gaps found.
    pub subjects_with_gaps: usize,
    /// True when every subject's scan and surface agree.
    pub all_subjects_agree: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseSummary {
    /// Total records.
    pub total_records: usize,
    /// Cleared records.
    pub records_cleared: usize,
    /// Narrowed records.
    pub records_narrowed: usize,
    /// Records in the `cleared` state.
    pub state_cleared: usize,
    /// Records in the `narrowed_template` state.
    pub state_narrowed_template: usize,
    /// Records in the `narrowed_distribution` state.
    pub state_narrowed_distribution: usize,
    /// Records in the `narrowed_attribution` state.
    pub state_narrowed_attribution: usize,
    /// Records in the `narrowed_reversibility` state.
    pub state_narrowed_reversibility: usize,
    /// Records in the `narrowed_audit` state.
    pub state_narrowed_audit: usize,
    /// Records in the `narrowed_linkage` state.
    pub state_narrowed_linkage: usize,
    /// Records in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Records in the `withdrawn` state.
    pub state_withdrawn: usize,
    /// Release-blocking records.
    pub release_blocking_total: usize,
    /// Release-blocking records that are narrowed.
    pub release_blocking_narrowed: usize,
    /// Records held by an active waiver.
    pub records_on_active_waiver: usize,
    /// Records carrying a packet-template gap.
    pub template_gaps: usize,
    /// Records carrying a distribution-reach gap.
    pub distribution_gaps: usize,
    /// Records carrying an attribution gap.
    pub attribution_gaps: usize,
    /// Records carrying a reversibility gap.
    pub reversibility_gaps: usize,
    /// Records carrying an audit gap.
    pub audit_gaps: usize,
    /// Records carrying a linkage gap.
    pub linkage_gaps: usize,
    /// Records whose claimed mirror channel did not receive current evidence.
    pub mirror_reach_gaps: usize,
    /// Records whose claimed offline channel did not receive current evidence.
    pub offline_reach_gaps: usize,
    /// Break-glass records.
    pub break_glass_total: usize,
    /// Records that require post-incident reconciliation.
    pub reconciliation_required: usize,
    /// Records whose reconciliation is complete.
    pub reconciliation_complete: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of emergency-response evidence records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmergencyResponseEvidenceRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register id.
    pub register_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Date the register was last reconciled.
    pub as_of: String,
    /// Canonical source registers.
    pub source_contract_refs: SourceContractRefs,
    /// Launch cutline.
    pub response_cutline: ResponseCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed packet-kind vocabulary.
    pub packet_kinds: Vec<PacketKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed severity vocabulary.
    pub severities: Vec<Severity>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed distribution-channel vocabulary.
    pub distribution_channels: Vec<DistributionChannel>,
    /// Closed channel-state vocabulary.
    pub channel_states: Vec<ChannelState>,
    /// Closed template-state vocabulary.
    pub template_states: Vec<TemplateState>,
    /// Closed attribution-state vocabulary.
    pub attribution_states: Vec<AttributionState>,
    /// Closed reversibility-state vocabulary.
    pub reversibility_states: Vec<ReversibilityState>,
    /// Closed reconciliation-state vocabulary.
    pub reconciliation_states: Vec<ReconciliationState>,
    /// Closed linkage-state vocabulary.
    pub linkage_states: Vec<LinkageState>,
    /// Closed posture vocabulary.
    pub postures: Vec<Posture>,
    /// Closed response-state vocabulary.
    pub response_states: Vec<ResponseState>,
    /// Closed response-reason vocabulary.
    pub response_reasons: Vec<ResponseReason>,
    /// Closed response-action vocabulary.
    pub response_actions: Vec<ResponseAction>,
    /// Stop rules.
    pub rules: Vec<ResponseRule>,
    /// Per-packet records.
    pub records: Vec<EmergencyResponseRecord>,
    /// Cross-cutting scan/surface parity summary.
    pub scan_surface_parity: ScanSurfaceParity,
    /// Promotion verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: ResponseSummary,
}

impl EmergencyResponseEvidenceRegister {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&EmergencyResponseRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Returns the cleared records.
    pub fn records_cleared(&self) -> Vec<&EmergencyResponseRecord> {
        self.records.iter().filter(|r| r.is_cleared()).collect()
    }

    /// Returns the narrowed records.
    pub fn records_narrowed(&self) -> Vec<&EmergencyResponseRecord> {
        self.records
            .iter()
            .filter(|r| r.continuity_state.is_narrowed())
            .collect()
    }

    /// Returns the records of a given packet kind.
    pub fn records_of_kind(&self, kind: PacketKind) -> Vec<&EmergencyResponseRecord> {
        self.records
            .iter()
            .filter(|r| r.packet_kind == kind)
            .collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: ResponseReason) -> Option<&ResponseRule> {
        self.rules.iter().find(|rule| rule.trigger_reason == reason)
    }

    /// Recomputes the firing rule ids: a blocking rule fires when a promotion-holding record carries
    /// its trigger reason at an applicable label.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if !rule.blocks_promotion {
                continue;
            }
            let fires = self.records.iter().any(|r| {
                r.holds_promotion()
                    && r.has_active_reason(rule.trigger_reason)
                    && rule.applies_to_labels.contains(&r.declared_label)
            });
            if fires {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the offending record ids: promotion-holding records carrying a reason watched by a
    /// firing blocking rule.
    pub fn computed_blocking_record_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for r in &self.records {
            if !r.holds_promotion() {
                continue;
            }
            let blocked = r.active_reasons.iter().any(|reason| {
                self.rule_for(*reason).is_some_and(|rule| {
                    rule.blocks_promotion && rule.applies_to_labels.contains(&r.declared_label)
                })
            });
            if blocked {
                ids.insert(r.record_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the promotion decision.
    pub fn computed_decision(&self) -> PublicationDecision {
        if self.computed_blocking_record_ids().is_empty() {
            PublicationDecision::Proceed
        } else {
            PublicationDecision::Hold
        }
    }

    /// Recomputes the cross-cutting scan/surface parity summary.
    pub fn computed_scan_surface_parity(&self) -> ScanSurfaceParity {
        ScanSurfaceParity {
            parity_gate: self.scan_surface_parity.parity_gate.clone(),
            subjects_total: self.records.len(),
            subjects_in_agreement: self
                .records
                .iter()
                .filter(|r| r.scan_surface_agree())
                .count(),
            subjects_in_disagreement: self
                .records
                .iter()
                .filter(|r| !r.scan_surface_agree())
                .count(),
            subjects_with_gaps: self
                .records
                .iter()
                .filter(|r| r.surface_posture == Posture::GapsFound)
                .count(),
            all_subjects_agree: self.records.iter().all(|r| r.scan_surface_agree()),
            rationale: self.scan_surface_parity.rationale.clone(),
        }
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> ResponseSummary {
        let count_state = |state: ResponseState| {
            self.records
                .iter()
                .filter(|r| r.continuity_state == state)
                .count()
        };
        ResponseSummary {
            total_records: self.records.len(),
            records_cleared: self.records_cleared().len(),
            records_narrowed: self.records_narrowed().len(),
            state_cleared: count_state(ResponseState::Cleared),
            state_narrowed_template: count_state(ResponseState::NarrowedTemplate),
            state_narrowed_distribution: count_state(ResponseState::NarrowedDistribution),
            state_narrowed_attribution: count_state(ResponseState::NarrowedAttribution),
            state_narrowed_reversibility: count_state(ResponseState::NarrowedReversibility),
            state_narrowed_audit: count_state(ResponseState::NarrowedAudit),
            state_narrowed_linkage: count_state(ResponseState::NarrowedLinkage),
            state_narrowed_stale: count_state(ResponseState::NarrowedStale),
            state_withdrawn: count_state(ResponseState::Withdrawn),
            release_blocking_total: self.records.iter().filter(|r| r.release_blocking).count(),
            release_blocking_narrowed: self
                .records
                .iter()
                .filter(|r| r.release_blocking && r.continuity_state.is_narrowed())
                .count(),
            records_on_active_waiver: self.records.iter().filter(|r| r.is_waived()).count(),
            template_gaps: self.records.iter().filter(|r| r.template_unbound()).count(),
            distribution_gaps: self
                .records
                .iter()
                .filter(|r| r.has_distribution_gap())
                .count(),
            attribution_gaps: self.records.iter().filter(|r| r.unattributable()).count(),
            reversibility_gaps: self
                .records
                .iter()
                .filter(|r| r.reversal_rule_missing())
                .count(),
            audit_gaps: self
                .records
                .iter()
                .filter(|r| r.audit_markers_missing() || r.reconciliation_pending())
                .count(),
            linkage_gaps: self.records.iter().filter(|r| r.linkage_missing()).count(),
            mirror_reach_gaps: self
                .records
                .iter()
                .filter(|r| r.has_active_reason(ResponseReason::MirrorPropagationIncomplete))
                .count(),
            offline_reach_gaps: self
                .records
                .iter()
                .filter(|r| r.has_active_reason(ResponseReason::OfflineImportResponseMissing))
                .count(),
            break_glass_total: self.records.iter().filter(|r| r.is_break_glass).count(),
            reconciliation_required: self
                .records
                .iter()
                .filter(|r| r.requires_reconciliation())
                .count(),
            reconciliation_complete: self
                .records
                .iter()
                .filter(|r| r.audit_trail.reconciliation_state == ReconciliationState::Reconciled)
                .count(),
            total_active_reasons: self.records.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, service-health, release-center publication,
    /// support exports, and shiproom panels. It carries only the family, packet kind, declared and
    /// effective labels, severity, state, the scan/surface-agreement flag, the
    /// template/distribution/attribution/reversibility/audit/linkage summary, active reasons, and
    /// surfaces — never the detailed channel, audit, and proof internals.
    pub fn reuse_projection(&self) -> Vec<EmergencyResponseReuseRow> {
        self.records
            .iter()
            .map(|r| EmergencyResponseReuseRow {
                record_id: r.record_id.clone(),
                family: r.family,
                packet_kind: r.packet_kind,
                declared_label: r.declared_label,
                effective_label: r.effective_label,
                support_class: r.support_class,
                severity: r.severity,
                continuity_state: r.continuity_state,
                release_blocking: r.release_blocking,
                is_break_glass: r.is_break_glass,
                scan_surface_agree: r.scan_surface_agree(),
                template_state: r.packet_template.template_state,
                attribution_state: r.attribution.attribution_state,
                reversibility_state: r.reversibility.reversibility_state,
                reconciliation_state: r.audit_trail.reconciliation_state,
                linkage_state: r.evidence_linkage.linkage_state,
                active_reasons: r.active_reasons.clone(),
                surfaces: r.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_EMERGENCY_RESPONSE_EVIDENCE_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_EMERGENCY_RESPONSE_EVIDENCE_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.records.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every packet kind must be exercised by at least one record.
        for kind in PacketKind::ALL {
            if !self.records.iter().any(|r| r.packet_kind == kind) {
                v.push(RegisterViolation::PacketKindUncovered { kind });
            }
        }

        // Every reason must have a stop rule.
        for reason in ResponseReason::ALL {
            if self.rule_for(reason).is_none() {
                v.push(RegisterViolation::ReasonUncoveredByRule { reason });
            }
        }

        let mut seen = BTreeSet::new();
        for r in &self.records {
            self.validate_record(r, &mut seen, &mut v);
        }

        // Verdict, parity, and summary coherence.
        if self.publication.decision != self.computed_decision() {
            v.push(RegisterViolation::PublicationDecisionInconsistent);
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            v.push(RegisterViolation::PublicationBlockingRulesMismatch);
        }
        if self.publication.blocking_record_ids != self.computed_blocking_record_ids() {
            v.push(RegisterViolation::PublicationBlockingRecordsMismatch);
        }
        if self.scan_surface_parity != self.computed_scan_surface_parity() {
            v.push(RegisterViolation::ScanSurfaceParityMismatch);
        }
        if self.summary != self.computed_summary() {
            v.push(RegisterViolation::SummaryMismatch);
        }

        v
    }

    fn validate_vocabularies(&self, v: &mut Vec<RegisterViolation>) {
        if self.families != M5Family::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "families" });
        }
        if self.packet_kinds != PacketKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "packet_kinds",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.severities != Severity::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "severities",
            });
        }
        if self.control_dimensions != ControlDimension::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "control_dimensions",
            });
        }
        if self.distribution_channels != DistributionChannel::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "distribution_channels",
            });
        }
        if self.channel_states != ChannelState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "channel_states",
            });
        }
        if self.template_states != TemplateState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "template_states",
            });
        }
        if self.attribution_states != AttributionState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "attribution_states",
            });
        }
        if self.reversibility_states != ReversibilityState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "reversibility_states",
            });
        }
        if self.reconciliation_states != ReconciliationState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "reconciliation_states",
            });
        }
        if self.linkage_states != LinkageState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "linkage_states",
            });
        }
        if self.postures != Posture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "postures" });
        }
        if self.response_states != ResponseState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "response_states",
            });
        }
        if self.response_reasons != ResponseReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "response_reasons",
            });
        }
        if self.response_actions != ResponseAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "response_actions",
            });
        }
        if self.response_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "response_cutline",
            });
        }
    }

    fn validate_record(
        &self,
        r: &EmergencyResponseRecord,
        seen: &mut BTreeSet<String>,
        v: &mut Vec<RegisterViolation>,
    ) {
        for (field, value) in [
            ("record_id", &r.record_id),
            ("title", &r.title),
            ("subject_ref", &r.subject_ref),
            ("subject_summary", &r.subject_summary),
            ("rationale", &r.rationale),
        ] {
            if value.trim().is_empty() {
                v.push(RegisterViolation::EmptyField {
                    record_id: r.record_id.clone(),
                    field_name: field,
                });
            }
        }
        if !seen.insert(r.record_id.clone()) {
            v.push(RegisterViolation::DuplicateRecordId {
                record_id: r.record_id.clone(),
            });
        }
        if r.surfaces.is_empty() {
            v.push(RegisterViolation::RecordMissingSurfaces {
                record_id: r.record_id.clone(),
            });
        }

        self.validate_fact_consistency(r, v);
        self.validate_controls(r, v);
        self.validate_reason_evidence(r, v);
        self.validate_scan_surface(r, v);
        self.validate_state_and_label(r, v);
    }

    /// Each fact block must be internally consistent — so a state token can never sit over a
    /// contradicting fact (a "bound" template that is not signed/digested, a packet kind that does
    /// not match the record, a "claimed" channel marked `not_claimed`, an "attributable" action
    /// with no actor ref, a reversibility state inconsistent with its policy flag, an audit-markers
    /// flag that disagrees with its ref, a reconciliation that does not apply to its requirement, or
    /// a "linked" evidence block missing its refs).
    fn validate_fact_consistency(
        &self,
        r: &EmergencyResponseRecord,
        v: &mut Vec<RegisterViolation>,
    ) {
        // the template kind must match the record, and bound ⟺ signed && digest present.
        if r.packet_template.packet_kind != r.packet_kind {
            v.push(RegisterViolation::PacketKindMismatch {
                record_id: r.record_id.clone(),
            });
        }
        let bound = r.packet_template.template_state == TemplateState::Bound;
        let signed_and_digested =
            r.packet_template.signed && !r.packet_template.digest_ref.trim().is_empty();
        if bound != signed_and_digested {
            v.push(RegisterViolation::TemplateFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }

        // each channel must be declared exactly once, and claimed ⟺ state != not_claimed, with
        // hosted always claimed.
        for channel in DistributionChannel::ALL {
            let matches: Vec<&ChannelEvidence> = r
                .distribution_reach
                .channels
                .iter()
                .filter(|c| c.channel == channel)
                .collect();
            if matches.len() != 1 {
                v.push(RegisterViolation::ChannelNotDeclaredOnce {
                    record_id: r.record_id.clone(),
                    channel,
                });
                continue;
            }
            let c = matches[0];
            let claimed_consistent = c.claimed == (c.state != ChannelState::NotClaimed);
            let hosted_claimed = channel != DistributionChannel::Hosted || c.claimed;
            if !claimed_consistent || !hosted_claimed {
                v.push(RegisterViolation::ChannelFactInconsistent {
                    record_id: r.record_id.clone(),
                    channel,
                });
            }
        }

        // attributable ⟺ actor ref present.
        let attributable = r.attribution.attribution_state == AttributionState::Attributable;
        if attributable == r.attribution.actor_ref.trim().is_empty() {
            v.push(RegisterViolation::AttributionFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }

        // reversibility state must agree with the policy flag and the runbook ref.
        let revers_ok = match r.reversibility.reversibility_state {
            ReversibilityState::ReversibleWithRunbook => {
                r.reversibility.policy_reversible
                    && !r.reversibility.reversal_runbook_ref.trim().is_empty()
            }
            ReversibilityState::ReversalRuleMissing => {
                r.reversibility.policy_reversible
                    && r.reversibility.reversal_runbook_ref.trim().is_empty()
            }
            ReversibilityState::IrreversibleByPolicy => !r.reversibility.policy_reversible,
        };
        if !revers_ok {
            v.push(RegisterViolation::ReversibilityFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }

        // audit markers flag ⟺ marker ref present.
        if r.audit_trail.audit_markers_present == r.audit_trail.audit_marker_ref.trim().is_empty() {
            v.push(RegisterViolation::AuditMarkerFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // reconciliation applies (state != not_required) ⟺ the action requires reconciliation.
        let reconciliation_applies =
            r.audit_trail.reconciliation_state != ReconciliationState::NotRequired;
        if reconciliation_applies != r.requires_reconciliation() {
            v.push(RegisterViolation::ReconciliationApplicabilityInconsistent {
                record_id: r.record_id.clone(),
            });
        }

        // linked ⟺ both release-artifact and support-export refs present.
        let linked = r.evidence_linkage.linkage_state == LinkageState::Linked;
        let linkage_refs_present = !r.evidence_linkage.release_artifact_ref.trim().is_empty()
            && !r.evidence_linkage.support_export_ref.trim().is_empty();
        if linked != linkage_refs_present {
            v.push(RegisterViolation::LinkageFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_controls(&self, r: &EmergencyResponseRecord, v: &mut Vec<RegisterViolation>) {
        // Every control dimension must be declared exactly once, and its declared state must equal
        // the state its facts imply — so a control can never assert "satisfied" over a gap.
        for dimension in ControlDimension::ALL {
            let matches: Vec<&ResponseControl> = r
                .controls
                .iter()
                .filter(|c| c.dimension == dimension)
                .collect();
            if matches.len() != 1 {
                v.push(RegisterViolation::ControlDimensionNotDeclaredOnce {
                    record_id: r.record_id.clone(),
                    dimension,
                });
                continue;
            }
            let expected = r.expected_control_state(dimension);
            if matches[0].state != expected {
                v.push(RegisterViolation::ControlStateInconsistent {
                    record_id: r.record_id.clone(),
                    dimension,
                });
            }
        }
    }

    /// Every active reason must be justified by the record's own facts, and every structural gap
    /// must surface its reason.
    fn validate_reason_evidence(
        &self,
        r: &EmergencyResponseRecord,
        v: &mut Vec<RegisterViolation>,
    ) {
        let dist_reasons = r.distribution_reasons();
        let proof_stale = r.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = r.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !r.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &r.active_reasons {
            let justified = match reason {
                ResponseReason::PacketTemplateUnbound => r.template_unbound(),
                ResponseReason::MirrorPropagationIncomplete
                | ResponseReason::OfflineImportResponseMissing
                | ResponseReason::ChannelEvidenceStale => dist_reasons.contains(reason),
                ResponseReason::ActionUnattributable => r.unattributable(),
                ResponseReason::ReversalRuleMissing => r.reversal_rule_missing(),
                ResponseReason::AuditMarkersMissing => r.audit_markers_missing(),
                ResponseReason::ReconciliationPending => r.reconciliation_pending(),
                ResponseReason::EvidenceLinkageMissing => r.linkage_missing(),
                ResponseReason::ResponseProofStale => proof_stale,
                ResponseReason::ResponseProofMissing => proof_missing,
                ResponseReason::OwnerSignoffMissing => signoff_missing,
                ResponseReason::WaiverExpired => r.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    record_id: r.record_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: ResponseReason, v: &mut Vec<RegisterViolation>| {
            if present && !r.has_active_reason(reason) {
                v.push(RegisterViolation::GapWithoutReason {
                    record_id: r.record_id.clone(),
                    reason,
                });
            }
        };
        require(
            r.template_unbound(),
            ResponseReason::PacketTemplateUnbound,
            v,
        );
        for reason in &dist_reasons {
            require(true, *reason, v);
        }
        require(r.unattributable(), ResponseReason::ActionUnattributable, v);
        require(
            r.reversal_rule_missing(),
            ResponseReason::ReversalRuleMissing,
            v,
        );
        require(
            r.audit_markers_missing(),
            ResponseReason::AuditMarkersMissing,
            v,
        );
        require(
            r.reconciliation_pending(),
            ResponseReason::ReconciliationPending,
            v,
        );
        require(
            r.linkage_missing(),
            ResponseReason::EvidenceLinkageMissing,
            v,
        );
        require(proof_stale, ResponseReason::ResponseProofStale, v);
        require(proof_missing, ResponseReason::ResponseProofMissing, v);
        require(signoff_missing, ResponseReason::OwnerSignoffMissing, v);
    }

    /// The scan and the surface must agree, and the posture must reflect the gaps — a green surface
    /// may never sit over a scan that found a mirror/offline reach gap, an unattributable action, or
    /// a side-channel-only disable.
    fn validate_scan_surface(&self, r: &EmergencyResponseRecord, v: &mut Vec<RegisterViolation>) {
        if r.scan_posture != r.surface_posture {
            v.push(RegisterViolation::ScanSurfaceDisagreement {
                record_id: r.record_id.clone(),
            });
        }
        let computed = r.computed_posture();
        if r.surface_posture != computed || r.scan_posture != computed {
            v.push(RegisterViolation::PostureMismatch {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_state_and_label(
        &self,
        r: &EmergencyResponseRecord,
        v: &mut Vec<RegisterViolation>,
    ) {
        // cleared ⇒ no reasons; narrowed ⇒ at least one reason.
        if r.is_cleared() && !r.active_reasons.is_empty() {
            v.push(RegisterViolation::ClearedWithActiveReason {
                record_id: r.record_id.clone(),
            });
        }
        if r.continuity_state.is_narrowed() && r.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                record_id: r.record_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if r.continuity_state != r.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                record_id: r.record_id.clone(),
                declared: r.continuity_state,
                computed: r.computed_state(),
            });
        }
        // never widen: effective may not rank above declared.
        if r.effective_label.rank() > r.declared_label.rank() {
            v.push(RegisterViolation::EffectiveLabelExceedsDeclared {
                record_id: r.record_id.clone(),
            });
        }
        // effective must equal the computed effective label.
        if r.effective_label != r.computed_effective_label() {
            v.push(RegisterViolation::EffectiveLabelMismatch {
                record_id: r.record_id.clone(),
            });
        }
        // a narrowed record must drop below the cutline.
        if r.continuity_state.is_narrowed() && r.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                record_id: r.record_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmergencyResponseReuseRow {
    /// Record id.
    pub record_id: String,
    /// Family.
    pub family: M5Family,
    /// Packet kind.
    pub packet_kind: PacketKind,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Severity grade.
    pub severity: Severity,
    /// Response state.
    pub continuity_state: ResponseState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// Break-glass flag.
    pub is_break_glass: bool,
    /// True when the scan and the surface agree.
    pub scan_surface_agree: bool,
    /// Packet-template posture.
    pub template_state: TemplateState,
    /// Attribution posture.
    pub attribution_state: AttributionState,
    /// Reversibility posture.
    pub reversibility_state: ReversibilityState,
    /// Reconciliation posture.
    pub reconciliation_state: ReconciliationState,
    /// Evidence-linkage posture.
    pub linkage_state: LinkageState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ResponseReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the emergency-response evidence register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found.
        actual: u32,
    },
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no records.
    EmptyRegister,
    /// A packet kind has no record.
    PacketKindUncovered {
        /// Uncovered kind.
        kind: PacketKind,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: ResponseReason,
    },
    /// A record id appears more than once.
    DuplicateRecordId {
        /// Duplicate id.
        record_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Record id.
        record_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A record lists no reuse surfaces.
    RecordMissingSurfaces {
        /// Record id.
        record_id: String,
    },
    /// A record's packet template kind does not match its packet kind.
    PacketKindMismatch {
        /// Record id.
        record_id: String,
    },
    /// A record's template state disagrees with its signed/digest facts.
    TemplateFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A distribution channel is not declared exactly once.
    ChannelNotDeclaredOnce {
        /// Record id.
        record_id: String,
        /// Offending channel.
        channel: DistributionChannel,
    },
    /// A channel's claimed flag disagrees with its state (or hosted is not claimed).
    ChannelFactInconsistent {
        /// Record id.
        record_id: String,
        /// Offending channel.
        channel: DistributionChannel,
    },
    /// A record's attribution state disagrees with its actor ref.
    AttributionFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's reversibility state disagrees with its policy flag or runbook ref.
    ReversibilityFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's audit-markers flag disagrees with its marker ref.
    AuditMarkerFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's reconciliation applicability disagrees with its severity/break-glass facts.
    ReconciliationApplicabilityInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's linkage state disagrees with its release/support refs.
    LinkageFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A control dimension is not declared exactly once.
    ControlDimensionNotDeclaredOnce {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// A control's declared state disagrees with the facts it governs.
    ControlStateInconsistent {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// An active reason is not justified by the record's fields.
    ReasonNotJustified {
        /// Record id.
        record_id: String,
        /// Offending reason.
        reason: ResponseReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Record id.
        record_id: String,
        /// Missing reason.
        reason: ResponseReason,
    },
    /// A record's scan and surface postures disagree.
    ScanSurfaceDisagreement {
        /// Record id.
        record_id: String,
    },
    /// A record's posture disagrees with the gaps its state implies.
    PostureMismatch {
        /// Record id.
        record_id: String,
    },
    /// A cleared record carries an active reason.
    ClearedWithActiveReason {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record carries no reason.
    NarrowedWithoutReason {
        /// Record id.
        record_id: String,
    },
    /// The record state disagrees with the active reasons.
    StateReasonMismatch {
        /// Record id.
        record_id: String,
        /// Declared state.
        declared: ResponseState,
        /// Computed state.
        computed: ResponseState,
    },
    /// The effective label ranks above the declared label.
    EffectiveLabelExceedsDeclared {
        /// Record id.
        record_id: String,
    },
    /// The effective label disagrees with the computed effective label.
    EffectiveLabelMismatch {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record did not drop below the cutline.
    NarrowedAboveCutline {
        /// Record id.
        record_id: String,
    },
    /// The promotion decision disagrees with the firing rules.
    PublicationDecisionInconsistent,
    /// The recorded blocking rule ids disagree with the computed set.
    PublicationBlockingRulesMismatch,
    /// The recorded blocking record ids disagree with the computed set.
    PublicationBlockingRecordsMismatch,
    /// The recorded scan/surface parity disagrees with the computed summary.
    ScanSurfaceParityMismatch,
    /// The summary counts disagree with the records.
    SummaryMismatch,
}

impl fmt::Display for RegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no records"),
            Self::PacketKindUncovered { kind } => {
                write!(f, "packet kind {} has no record", kind.as_str())
            }
            Self::ReasonUncoveredByRule { reason } => {
                write!(f, "reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateRecordId { record_id } => {
                write!(f, "duplicate record id {record_id}")
            }
            Self::EmptyField {
                record_id,
                field_name,
            } => write!(f, "record {record_id} has empty field {field_name}"),
            Self::RecordMissingSurfaces { record_id } => {
                write!(f, "record {record_id} lists no reuse surfaces")
            }
            Self::PacketKindMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} packet template kind does not match its packet kind"
                )
            }
            Self::TemplateFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} template state disagrees with its signed/digest facts"
                )
            }
            Self::ChannelNotDeclaredOnce { record_id, channel } => write!(
                f,
                "record {record_id} does not declare channel {} exactly once",
                channel.as_str()
            ),
            Self::ChannelFactInconsistent { record_id, channel } => write!(
                f,
                "record {record_id} channel {} claimed flag disagrees with its state",
                channel.as_str()
            ),
            Self::AttributionFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} attribution state disagrees with its actor ref"
                )
            }
            Self::ReversibilityFactInconsistent { record_id } => write!(
                f,
                "record {record_id} reversibility state disagrees with its policy flag or runbook ref"
            ),
            Self::AuditMarkerFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} audit-markers flag disagrees with its marker ref"
                )
            }
            Self::ReconciliationApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} reconciliation applicability disagrees with its severity/break-glass facts"
            ),
            Self::LinkageFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} linkage state disagrees with its release/support refs"
                )
            }
            Self::ControlDimensionNotDeclaredOnce {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} does not declare control {} exactly once",
                dimension.as_str()
            ),
            Self::ControlStateInconsistent {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} control {} state disagrees with its facts",
                dimension.as_str()
            ),
            Self::ReasonNotJustified { record_id, reason } => write!(
                f,
                "record {record_id} names reason {} which its fields do not justify",
                reason.as_str()
            ),
            Self::GapWithoutReason { record_id, reason } => write!(
                f,
                "record {record_id} has a structural gap but does not name reason {}",
                reason.as_str()
            ),
            Self::ScanSurfaceDisagreement { record_id } => {
                write!(f, "record {record_id} scan and surface postures disagree")
            }
            Self::PostureMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} posture disagrees with the gaps its state implies"
                )
            }
            Self::ClearedWithActiveReason { record_id } => {
                write!(
                    f,
                    "cleared record {record_id} carries an active narrowing reason"
                )
            }
            Self::NarrowedWithoutReason { record_id } => {
                write!(f, "narrowed record {record_id} names no reason")
            }
            Self::StateReasonMismatch {
                record_id,
                declared,
                computed,
            } => write!(
                f,
                "record {record_id} records state {} but its reasons imply {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::EffectiveLabelExceedsDeclared { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label is wider than its declared label"
                )
            }
            Self::EffectiveLabelMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label disagrees with its state"
                )
            }
            Self::NarrowedAboveCutline { record_id } => {
                write!(
                    f,
                    "narrowed record {record_id} did not drop below the cutline"
                )
            }
            Self::PublicationDecisionInconsistent => {
                write!(f, "promotion decision disagrees with the firing rules")
            }
            Self::PublicationBlockingRulesMismatch => {
                write!(
                    f,
                    "publication blocking_rule_ids disagree with the computed set"
                )
            }
            Self::PublicationBlockingRecordsMismatch => {
                write!(
                    f,
                    "publication blocking_record_ids disagree with the computed set"
                )
            }
            Self::ScanSurfaceParityMismatch => {
                write!(f, "scan_surface_parity disagrees with the computed summary")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the records"),
        }
    }
}

impl Error for RegisterViolation {}

/// Loads the embedded emergency-response evidence register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`EmergencyResponseEvidenceRegister`] — including when a record carries a token outside any closed
/// vocabulary.
pub fn current_m5_emergency_response_evidence(
) -> Result<EmergencyResponseEvidenceRegister, serde_json::Error> {
    serde_json::from_str(M5_EMERGENCY_RESPONSE_EVIDENCE_JSON)
}

#[cfg(test)]
mod tests;
