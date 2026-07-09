//! Related-evidence cards and offline-handoff packet cards carrying summary-first
//! validation context, freshness, redaction state, publish-later target, and
//! copy/export/retry truth.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_work_item_component_matrix`] — the `related_evidence_card`
//! and the `offline_handoff_packet_card` — into one implemented, export-safe packet with
//! two co-equal control vectors. Together they preserve the engineering evidence tied to
//! a work item and keep publish-later recovery explicit when the provider is unavailable
//! or a write is blocked.
//!
//! A [`RelatedEvidenceCard`] summarizes one linked engineering context — a review, a
//! branch/worktree change, a failing/passing test, a CI check, an incident/runbook, or a
//! docs/ADR reference — leading with a summary and an open-detail action rather than
//! dumping raw artifacts. Its freshness ([`EvidenceFreshnessClass`]) is *derived* from
//! whether the reference is current, whether freshness is known, and whether the evidence
//! is provider-backed, so stale or local-only evidence never reads as current provider
//! truth.
//!
//! An [`OfflineHandoffPacketCard`] shows the packet type, the included metadata/evidence,
//! the redaction state ([`M5WorkItemExportBoundary`]), the publish-later target
//! ([`M5WorkItemHandoffDestination`]), and copy/export/retry actions. Its acceptance
//! class ([`PacketAcceptanceClass`]) is *derived* from the handoff destination, the
//! local-versus-provider state, and whether a prior publish failed, so a queued, held, or
//! failed packet never implies the provider accepted it, and the packet stays visible,
//! retryable, and exportable after failure rather than collapsing into a generic error
//! banner.
//!
//! The evidence kind ([`M5WorkItemEvidenceKind`]), handoff destination
//! ([`M5WorkItemHandoffDestination`]), export boundary ([`M5WorkItemExportBoundary`]),
//! local-versus-provider state ([`M5WorkItemLocalState`]), surface families
//! ([`M5WorkItemSurfaceFamily`]), deployment lines ([`M5WorkItemDeploymentLine`]),
//! consumer surfaces ([`M5WorkItemConsumerSurface`]), accessibility routes
//! ([`M5WorkItemAccessibilityRoute`]), and downgrade triggers
//! ([`M5WorkItemDowngradeTrigger`]) are reused directly from the frozen matrix, so this
//! lane never invents a parallel work-item vocabulary. It mints new vocabulary only for
//! what that matrix left implicit about these two controls: the derived evidence
//! freshness class, the evidence outcome class, the summary-first evidence actions, the
//! derived packet acceptance class, and the copy/export/retry packet actions.
//!
//! Raw work-item bodies, pasted paths, credentials, and private endpoints stay outside
//! the support boundary; canonical ids and references are carried only as opaque,
//! export-safe strings.
//!
//! The boundary schema is
//! [`schemas/ui/m5-related-evidence-offline-handoff-controls.schema.json`](../../../../schemas/ui/m5-related-evidence-offline-handoff-controls.schema.json).
//! The contract doc is
//! [`docs/team-workflows/implement_related_evidence_cards_and_offline_handoff_packet_cards.md`](../../../../docs/team-workflows/implement_related_evidence_cards_and_offline_handoff_packet_cards.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_related_evidence_offline_handoff_controls,
    seeded_related_evidence_offline_handoff_controls_offline_packet_publish_failed,
    seeded_related_evidence_offline_handoff_controls_related_evidence_summary_first,
    EVIDENCE_HANDOFF_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The evidence kind, handoff destination, export boundary, local-versus-provider state,
// surface family, deployment line, consumer surface, accessibility route, and downgrade
// triggers are frozen once, in the work-item component matrix. This lane reuses them
// verbatim so it never invents a parallel work-item vocabulary.
use crate::freeze_the_m5_work_item_component_matrix::{
    M5WorkItemAccessibilityRoute, M5WorkItemComponentFamily, M5WorkItemConsumerSurface,
    M5WorkItemDeploymentLine, M5WorkItemDowngradeTrigger, M5WorkItemEvidenceKind,
    M5WorkItemExportBoundary, M5WorkItemHandoffDestination, M5WorkItemLocalState,
    M5WorkItemSurfaceFamily, M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
    M5_RELATED_EVIDENCE_CARD_SCHEMA_REF, M5_WORK_ITEM_COMPONENT_DOC_REF,
    M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`EvidenceHandoffControlsPacket`].
pub const EVIDENCE_HANDOFF_RECORD_KIND: &str = "related_evidence_offline_handoff_controls";

/// Schema version for related-evidence-card / offline-handoff-packet-card control records.
pub const EVIDENCE_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EVIDENCE_HANDOFF_SCHEMA_REF: &str =
    "schemas/ui/m5-related-evidence-offline-handoff-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const EVIDENCE_HANDOFF_DOC_REF: &str =
    "docs/team-workflows/implement_related_evidence_cards_and_offline_handoff_packet_cards.md";

/// Repo-relative path of the protected fixture directory.
pub const EVIDENCE_HANDOFF_FIXTURE_DIR: &str =
    "fixtures/ui/m5-related-evidence-offline-handoff-controls";

/// Repo-relative path of the checked support-export artifact.
pub const EVIDENCE_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/release/m5-related-evidence-offline-handoff-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const EVIDENCE_HANDOFF_SUMMARY_REF: &str =
    "artifacts/release/m5-related-evidence-offline-handoff-proof/summary.md";

// ---- related-evidence-card vocabulary ------------------------------------

/// The summarized outcome a related-evidence card leads with, so a linked test, check, or
/// review is never shown without a plain summary of what it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOutcomeClass {
    /// The evidence passed (a green test, a passing check, an approved review).
    Passing,
    /// The evidence failed (a red test, a failing check, changes requested).
    Failing,
    /// The evidence is informational (a docs/ADR ref, a linked change, an attachment).
    Informational,
    /// The evidence outcome cannot currently be determined.
    UnknownOutcome,
}

impl EvidenceOutcomeClass {
    /// Every outcome class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Passing,
        Self::Failing,
        Self::Informational,
        Self::UnknownOutcome,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passing => "passing",
            Self::Failing => "failing",
            Self::Informational => "informational",
            Self::UnknownOutcome => "unknown_outcome",
        }
    }

    /// Whether this outcome requires the user's attention (a failure).
    pub const fn requires_attention(self) -> bool {
        matches!(self, Self::Failing)
    }
}

/// Derived freshness class a related-evidence card may present.
///
/// This is the evidence honesty axis: the class is derived from whether the reference is
/// current, whether freshness is known, and whether the evidence is provider-backed, never
/// asserted, so stale or local-only evidence never reads as current provider truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessClass {
    /// The evidence reflects a current, reconciled reference.
    CurrentEvidence,
    /// The evidence reflects a reference that is out of date.
    StaleEvidence,
    /// The evidence is local-only with no provider reference yet.
    LocalOnlyEvidence,
    /// The evidence's freshness cannot currently be determined.
    UnknownFreshness,
}

impl EvidenceFreshnessClass {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentEvidence,
        Self::StaleEvidence,
        Self::LocalOnlyEvidence,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentEvidence => "current_evidence",
            Self::StaleEvidence => "stale_evidence",
            Self::LocalOnlyEvidence => "local_only_evidence",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }

    /// Whether this class is current, reconciled evidence.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::CurrentEvidence)
    }
}

/// One keyboard-complete, metadata-safe action a related-evidence card offers, so the card
/// always leads with a summary-first open-detail affordance rather than dumping a raw
/// artifact, and never hides its copy affordance behind a pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCardAction {
    /// Open the full evidence detail (always available) — the summary-first escape hatch.
    OpenDetail,
    /// Copy the evidence reference (always available).
    CopyReference,
    /// Reveal the evidence provenance / source.
    RevealProvenance,
    /// Export the evidence summary as metadata-safe evidence.
    ExportEvidence,
}

impl EvidenceCardAction {
    /// Every evidence-card action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenDetail,
        Self::CopyReference,
        Self::RevealProvenance,
        Self::ExportEvidence,
    ];

    /// The open-detail / copy-reference actions every card must offer.
    pub const MANDATORY: [Self; 2] = [Self::OpenDetail, Self::CopyReference];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::CopyReference => "copy_reference",
            Self::RevealProvenance => "reveal_provenance",
            Self::ExportEvidence => "export_evidence",
        }
    }
}

/// Disclosures a related-evidence card must carry, derived from outcome and freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceDisclosure {
    /// The derived freshness class this card may present.
    pub freshness_class: EvidenceFreshnessClass,
    /// Whether the evidence is current, reconciled truth.
    pub is_current: bool,
    /// Whether the outcome requires the user's attention (a failure).
    pub requires_attention: bool,
    /// Whether the card must carry an explicit freshness note (any non-current class).
    pub needs_freshness_note: bool,
    /// Whether the card must carry an explicit failure note (a failing outcome).
    pub needs_failure_note: bool,
}

/// Resolves the freshness truth a related-evidence card may present.
///
/// Freshness is unknown when it is not known, local-only when the evidence is not
/// provider-backed, current when the reference is current, and stale otherwise.
pub fn resolve_evidence_card(
    outcome: EvidenceOutcomeClass,
    is_reference_current: bool,
    is_freshness_known: bool,
    is_provider_backed: bool,
) -> EvidenceDisclosure {
    let freshness_class = if !is_freshness_known {
        EvidenceFreshnessClass::UnknownFreshness
    } else if !is_provider_backed {
        EvidenceFreshnessClass::LocalOnlyEvidence
    } else if is_reference_current {
        EvidenceFreshnessClass::CurrentEvidence
    } else {
        EvidenceFreshnessClass::StaleEvidence
    };

    EvidenceDisclosure {
        freshness_class,
        is_current: freshness_class.is_current(),
        requires_attention: outcome.requires_attention(),
        needs_freshness_note: !freshness_class.is_current(),
        needs_failure_note: outcome.requires_attention(),
    }
}

/// A related-evidence card summarizing one linked engineering context, summary-first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedEvidenceCard {
    /// Frozen component this control implements; must be `related_evidence_card`.
    pub component: M5WorkItemComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Canonical id of the work item this evidence is tied to; always non-empty.
    pub canonical_id: String,
    /// Evidence kind, reused from the frozen matrix.
    pub evidence_kind: M5WorkItemEvidenceKind,
    /// Summarized outcome the card leads with.
    pub evidence_outcome: EvidenceOutcomeClass,
    /// Summary-first line — a plain summary of what the evidence means; always non-empty.
    pub summary_label: String,
    /// Source / provenance label — which review, branch, test, or ref; always non-empty.
    pub source_label: String,
    /// Whether the evidence reference is current (not out of date).
    pub is_reference_current: bool,
    /// Whether the evidence's freshness is currently known.
    pub is_freshness_known: bool,
    /// Whether the evidence is backed by a real provider reference.
    pub is_provider_backed: bool,
    /// Derived freshness class (must equal the resolved class).
    pub freshness_class: EvidenceFreshnessClass,
    /// Freshness note; required when the evidence is not current.
    pub freshness_note: String,
    /// Failure note; required when the outcome is failing.
    pub failure_note: String,
    /// Whether the card leads with a summary rather than a raw artifact. MUST be `true`.
    pub leads_with_summary: bool,
    /// Metadata-safe actions this card offers (must include open-detail and copy-reference).
    pub actions: Vec<EvidenceCardAction>,
    /// Claimed M5 work-item surface families that render this card.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals evidence provenance.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl RelatedEvidenceCard {
    /// Freshness disclosures this card must carry, derived from outcome and freshness.
    pub fn evidence_disclosure(&self) -> EvidenceDisclosure {
        resolve_evidence_card(
            self.evidence_outcome,
            self.is_reference_current,
            self.is_freshness_known,
            self.is_provider_backed,
        )
    }

    /// Whether the card offers every mandatory metadata-safe action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<EvidenceCardAction> = self.actions.iter().copied().collect();
        EvidenceCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- offline-handoff-packet-card vocabulary ------------------------------

/// Derived acceptance class an offline-handoff packet card may present.
///
/// This is the packet honesty axis: the class is derived from the handoff destination, the
/// local-versus-provider state, and whether a prior publish failed, never asserted, so a
/// held, queued, or failed packet never implies the provider accepted it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketAcceptanceClass {
    /// The packet is held in the local queue; nothing has been sent.
    HeldLocalOnly,
    /// The packet is queued for publish; the provider has not yet accepted it.
    QueuedNotYetAccepted,
    /// A prior publish failed; the packet is retryable and exportable.
    PublishFailedRetryable,
    /// The packet has been exported / handed off; the provider has not accepted it.
    ExportedForHandoff,
    /// The provider accepted the packet — the only class that implies acceptance.
    ProviderAccepted,
}

impl PacketAcceptanceClass {
    /// Every acceptance class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HeldLocalOnly,
        Self::QueuedNotYetAccepted,
        Self::PublishFailedRetryable,
        Self::ExportedForHandoff,
        Self::ProviderAccepted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeldLocalOnly => "held_local_only",
            Self::QueuedNotYetAccepted => "queued_not_yet_accepted",
            Self::PublishFailedRetryable => "publish_failed_retryable",
            Self::ExportedForHandoff => "exported_for_handoff",
            Self::ProviderAccepted => "provider_accepted",
        }
    }

    /// Whether this class implies the provider accepted the packet.
    pub const fn implies_provider_accepted(self) -> bool {
        matches!(self, Self::ProviderAccepted)
    }

    /// Whether a packet in this class can still attempt a publish (retryable).
    pub const fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::HeldLocalOnly | Self::QueuedNotYetAccepted | Self::PublishFailedRetryable
        )
    }
}

/// One action an offline-handoff packet card offers, so the packet always keeps copy and
/// export parity, offers retry after a failure, and never collapses into a generic error
/// banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketCardAction {
    /// Copy the packet contents (always available).
    CopyPacket,
    /// Export the packet as metadata-safe evidence (always available).
    ExportPacket,
    /// Retry the publish — offered whenever the packet can still reach the provider.
    RetryPublish,
    /// Open the item in the owning provider instead of publishing the packet.
    OpenInProvider,
    /// Discard the packet after review.
    DiscardPacket,
}

impl PacketCardAction {
    /// Every packet-card action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CopyPacket,
        Self::ExportPacket,
        Self::RetryPublish,
        Self::OpenInProvider,
        Self::DiscardPacket,
    ];

    /// The copy/export affordances every packet must offer.
    pub const MANDATORY: [Self; 2] = [Self::CopyPacket, Self::ExportPacket];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyPacket => "copy_packet",
            Self::ExportPacket => "export_packet",
            Self::RetryPublish => "retry_publish",
            Self::OpenInProvider => "open_in_provider",
            Self::DiscardPacket => "discard_packet",
        }
    }
}

/// Disclosures an offline-handoff packet card must carry, derived from destination, state,
/// and failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketDisclosure {
    /// The derived acceptance class this card may present.
    pub acceptance_class: PacketAcceptanceClass,
    /// Whether the packet implies the provider accepted it (only the accepted class).
    pub implies_provider_accepted: bool,
    /// Whether the packet can still attempt a publish (retryable).
    pub is_retryable: bool,
    /// Whether the card must offer a retry action (retryable packets).
    pub needs_retry_action: bool,
    /// Whether the card must carry a failure-recovery note (a failed publish).
    pub needs_failure_recovery_note: bool,
}

/// Resolves the acceptance truth an offline-handoff packet card may present.
///
/// A failed publish is retryable. A publish handed off to a synced provider is accepted. A
/// local-queue packet is held locally. A packet exported to a file, support bundle, other
/// device, or discard-after-review path is exported for handoff. Otherwise the packet is
/// queued but not yet accepted.
pub fn resolve_packet_acceptance(
    handoff_destination: M5WorkItemHandoffDestination,
    local_state: M5WorkItemLocalState,
    has_publish_failed: bool,
) -> PacketDisclosure {
    use M5WorkItemHandoffDestination as Destination;
    use M5WorkItemLocalState as Local;
    use PacketAcceptanceClass as Class;

    let acceptance_class = if has_publish_failed || matches!(local_state, Local::PublishFailed) {
        Class::PublishFailedRetryable
    } else if matches!(handoff_destination, Destination::ProviderPublish)
        && matches!(local_state, Local::SyncedWithProvider)
    {
        Class::ProviderAccepted
    } else if matches!(handoff_destination, Destination::LocalQueue) {
        Class::HeldLocalOnly
    } else if matches!(
        handoff_destination,
        Destination::ExportedPacket
            | Destination::SupportBundle
            | Destination::AnotherDevice
            | Destination::DiscardAfterReview
    ) {
        Class::ExportedForHandoff
    } else {
        Class::QueuedNotYetAccepted
    };

    PacketDisclosure {
        acceptance_class,
        implies_provider_accepted: acceptance_class.implies_provider_accepted(),
        is_retryable: acceptance_class.is_retryable(),
        needs_retry_action: acceptance_class.is_retryable(),
        needs_failure_recovery_note: matches!(acceptance_class, Class::PublishFailedRetryable),
    }
}

/// An offline-handoff packet card showing type, contents, redaction, target, and recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffPacketCard {
    /// Frozen component this control implements; must be `offline_handoff_packet_card`.
    pub component: M5WorkItemComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Canonical id of the work item this packet captures; always non-empty.
    pub canonical_id: String,
    /// Packet type label — what kind of packet this is; always non-empty.
    pub packet_type_label: String,
    /// Handoff destination (publish-later target), reused from the frozen matrix.
    pub handoff_destination: M5WorkItemHandoffDestination,
    /// Publish-later target label — where the packet will land; always non-empty.
    pub publish_later_target_label: String,
    /// Local-versus-provider state, reused from the frozen matrix.
    pub local_state: M5WorkItemLocalState,
    /// Whether a prior publish attempt failed.
    pub has_publish_failed: bool,
    /// Derived acceptance class (must equal the resolved class).
    pub acceptance_class: PacketAcceptanceClass,
    /// Whether the card implies provider acceptance (must equal the derived truth).
    pub implies_provider_accepted: bool,
    /// Included-content summary — the metadata/evidence carried; always non-empty.
    pub included_content_summary: String,
    /// Export boundary (redaction state), reused from the frozen matrix.
    pub export_boundary: M5WorkItemExportBoundary,
    /// Redaction-state note — what is included or withheld; always non-empty.
    pub redaction_state_note: String,
    /// Failure-recovery note; required when a prior publish failed.
    pub failure_recovery_note: String,
    /// Whether the packet stays visible after a failure. MUST be `true`.
    pub remains_visible_after_failure: bool,
    /// Whether the packet collapses into a generic error banner. MUST be `false`.
    pub collapses_into_error_banner: bool,
    /// Copy/export/retry actions (must include the mandatory copy/export, and retry when
    /// the packet can still reach the provider).
    pub actions: Vec<PacketCardAction>,
    /// Claimed M5 work-item surface families that render this card.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals destination or boundary.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl OfflineHandoffPacketCard {
    /// Acceptance disclosures this card must carry, derived from destination, state, failure.
    pub fn packet_disclosure(&self) -> PacketDisclosure {
        resolve_packet_acceptance(
            self.handoff_destination,
            self.local_state,
            self.has_publish_failed,
        )
    }

    /// Whether the card offers every mandatory copy/export action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<PacketCardAction> = self.actions.iter().copied().collect();
        PacketCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card offers a retry action.
    fn offers_retry(&self) -> bool {
        self.actions.contains(&PacketCardAction::RetryPublish)
    }
}

// ---- review blocks ------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffTrustReview {
    /// Evidence cards lead with a summary rather than dumping raw artifacts first.
    pub evidence_card_leads_with_summary: bool,
    /// Evidence freshness is derived, so stale or local-only evidence never reads as current.
    pub evidence_freshness_derived: bool,
    /// Evidence cards name their provenance / source.
    pub evidence_names_provenance: bool,
    /// Evidence cards always offer an open-detail action.
    pub evidence_offers_open_detail: bool,
    /// Packet cards name the packet type and its publish-later target.
    pub packet_names_type_and_target: bool,
    /// Packet cards disclose their redaction state.
    pub packet_discloses_redaction_state: bool,
    /// A held, queued, or failed packet never implies the provider accepted it.
    pub offline_packet_never_implies_acceptance: bool,
    /// Offline packets stay retryable after a failure.
    pub offline_packet_retryable_after_failure: bool,
    /// Offline packets stay exportable after a failure.
    pub offline_packet_exportable_after_failure: bool,
    /// Offline packets stay visible and never collapse into a generic error banner.
    pub offline_packet_stays_visible_not_error_banner: bool,
    /// No generic ticket/task wording conceals provenance, destination, or boundary.
    pub no_generic_ticket_wording_conceals_truth: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl EvidenceHandoffTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.evidence_card_leads_with_summary
            && self.evidence_freshness_derived
            && self.evidence_names_provenance
            && self.evidence_offers_open_detail
            && self.packet_names_type_and_target
            && self.packet_discloses_redaction_state
            && self.offline_packet_never_implies_acceptance
            && self.offline_packet_retryable_after_failure
            && self.offline_packet_exportable_after_failure
            && self.offline_packet_stays_visible_not_error_banner
            && self.no_generic_ticket_wording_conceals_truth
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffConsumerProjection {
    /// The detail surface renders summary-first evidence with derived freshness.
    pub detail_surface_renders_summary_first_evidence: bool,
    /// The offline surface keeps the packet visible and retryable after failure.
    pub offline_surface_keeps_packet_visible_and_retryable: bool,
    /// The copy/export/retry and open-detail paths are reachable headless.
    pub copy_export_retry_reachable_headless: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl EvidenceHandoffConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.detail_surface_renders_summary_first_evidence
            && self.offline_surface_keeps_packet_visible_and_retryable
            && self.copy_export_retry_reachable_headless
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`EvidenceHandoffControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceHandoffControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Related-evidence cards.
    pub related_evidence_cards: Vec<RelatedEvidenceCard>,
    /// Offline-handoff packet cards.
    pub offline_handoff_packet_cards: Vec<OfflineHandoffPacketCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: EvidenceHandoffTrustReview,
    /// Consumer projection block.
    pub consumer_projection: EvidenceHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: EvidenceHandoffProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe related-evidence-card / offline-handoff-packet-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffControlsPacket {
    /// Record kind; must equal [`EVIDENCE_HANDOFF_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EVIDENCE_HANDOFF_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Related-evidence cards.
    pub related_evidence_cards: Vec<RelatedEvidenceCard>,
    /// Offline-handoff packet cards.
    pub offline_handoff_packet_cards: Vec<OfflineHandoffPacketCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: EvidenceHandoffTrustReview,
    /// Consumer projection block.
    pub consumer_projection: EvidenceHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: EvidenceHandoffProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl EvidenceHandoffControlsPacket {
    /// Builds a related-evidence / offline-handoff controls packet from stable-lane input.
    pub fn new(input: EvidenceHandoffControlsPacketInput) -> Self {
        Self {
            record_kind: EVIDENCE_HANDOFF_RECORD_KIND.to_owned(),
            schema_version: EVIDENCE_HANDOFF_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            related_evidence_cards: input.related_evidence_cards,
            offline_handoff_packet_cards: input.offline_handoff_packet_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the related-evidence / offline-handoff control invariants.
    pub fn validate(&self) -> Vec<EvidenceHandoffViolation> {
        let mut violations = Vec::new();

        if self.record_kind != EVIDENCE_HANDOFF_RECORD_KIND {
            violations.push(EvidenceHandoffViolation::WrongRecordKind);
        }
        if self.schema_version != EVIDENCE_HANDOFF_SCHEMA_VERSION {
            violations.push(EvidenceHandoffViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(EvidenceHandoffViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(EvidenceHandoffViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(EvidenceHandoffViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_evidence_cards(self, &mut violations);
        validate_packet_cards(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(EvidenceHandoffViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(EvidenceHandoffViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(EvidenceHandoffViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("evidence handoff packet serializes"),
        ) {
            violations.push(EvidenceHandoffViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("evidence handoff packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,kind_or_destination,outcome_or_boundary,derived,attention_or_accepted\n",
        );
        for card in &self.related_evidence_cards {
            let disclosure = card.evidence_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                "related_evidence_card",
                csv_field(&card.card_id),
                card.evidence_kind.as_str(),
                card.evidence_outcome.as_str(),
                disclosure.freshness_class.as_str(),
                disclosure.requires_attention,
            ));
        }
        for card in &self.offline_handoff_packet_cards {
            let disclosure = card.packet_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                "offline_handoff_packet_card",
                csv_field(&card.card_id),
                card.handoff_destination.as_str(),
                card.export_boundary.as_str(),
                disclosure.acceptance_class.as_str(),
                disclosure.implies_provider_accepted,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let failing_evidence = self
            .related_evidence_cards
            .iter()
            .filter(|card| card.evidence_disclosure().requires_attention)
            .count();
        let accepted_packets = self
            .offline_handoff_packet_cards
            .iter()
            .filter(|card| card.packet_disclosure().implies_provider_accepted)
            .count();

        let mut out = String::new();
        out.push_str("# Related-evidence cards and offline-handoff packet cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Related-evidence cards: {} ({} need attention)\n",
            self.related_evidence_cards.len(),
            failing_evidence
        ));
        out.push_str(&format!(
            "- Offline-handoff packet cards: {} ({} provider-accepted)\n",
            self.offline_handoff_packet_cards.len(),
            accepted_packets
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Related-evidence cards\n\n");
        for card in &self.related_evidence_cards {
            let disclosure = card.evidence_disclosure();
            out.push_str(&format!(
                "- **{}** ({}) [{} / {}] → {}\n",
                card.card_id,
                card.evidence_kind.as_str(),
                card.evidence_outcome.as_str(),
                disclosure.freshness_class.as_str(),
                card.summary_label,
            ));
        }

        out.push_str("\n## Offline-handoff packet cards\n\n");
        for card in &self.offline_handoff_packet_cards {
            out.push_str(&format!(
                "- **{}** [{}] → `{}` boundary: {}\n",
                card.card_id,
                card.packet_disclosure().acceptance_class.as_str(),
                card.handoff_destination.as_str(),
                card.export_boundary.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in related-evidence / offline-handoff export.
#[derive(Debug)]
pub enum EvidenceHandoffArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EvidenceHandoffViolation>),
}

impl fmt::Display for EvidenceHandoffArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "evidence handoff export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "evidence handoff export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for EvidenceHandoffArtifactError {}

/// Validation failures emitted by [`EvidenceHandoffControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceHandoffViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No related-evidence cards are present.
    RelatedEvidenceCardsMissing,
    /// A related-evidence card is incomplete.
    RelatedEvidenceCardIncomplete,
    /// A related-evidence card carries the wrong frozen component class.
    RelatedEvidenceCardWrongComponentClass,
    /// A related-evidence card misrepresents its derived freshness class.
    EvidenceFreshnessMisrepresented,
    /// A related-evidence card leads with a raw artifact instead of a summary.
    RawArtifactDumpedBeforeSummary,
    /// A related-evidence card does not carry a summary line.
    EvidenceSummaryMissing,
    /// A non-current evidence card does not name its freshness.
    EvidenceFreshnessNoteMissing,
    /// A failing evidence card does not name its failure.
    FailingEvidenceNoteMissing,
    /// A related-evidence card omits the mandatory open-detail / copy-reference actions.
    EvidenceOpenDetailMissing,
    /// The evidence cards do not cover every evidence kind.
    EvidenceKindCoverageMissing,
    /// The evidence cards do not cover every outcome class.
    EvidenceOutcomeCoverageMissing,
    /// The evidence cards do not cover every derived freshness class.
    EvidenceFreshnessCoverageMissing,
    /// No offline-handoff packet cards are present.
    OfflineHandoffCardsMissing,
    /// An offline-handoff packet card is incomplete.
    OfflineHandoffCardIncomplete,
    /// An offline-handoff packet card carries the wrong frozen component class.
    OfflineHandoffCardWrongComponentClass,
    /// An offline-handoff packet card misrepresents its derived acceptance class.
    PacketAcceptanceClassMisrepresented,
    /// A held, queued, or failed packet implies, or an accepted packet denies, acceptance.
    ProviderAcceptanceMisrepresented,
    /// An offline-handoff packet card does not name its packet type.
    PacketTypeLabelMissing,
    /// An offline-handoff packet card does not summarize its included content.
    IncludedContentSummaryMissing,
    /// An offline-handoff packet card does not name its redaction state.
    RedactionStateNoteMissing,
    /// An offline-handoff packet card does not name its publish-later target.
    PublishLaterTargetMissing,
    /// A failed packet does not name its failure recovery.
    FailureRecoveryNoteMissing,
    /// A retryable packet omits its retry action.
    PacketRetryActionMissing,
    /// An offline-handoff packet card omits a mandatory copy/export action.
    CopyExportActionMissing,
    /// A packet collapses into a generic error banner instead of staying visible.
    PacketCollapsedIntoErrorBanner,
    /// The packet cards do not cover every derived acceptance class.
    PacketAcceptanceClassCoverageMissing,
    /// The packet cards do not cover every handoff destination.
    HandoffDestinationCoverageMissing,
    /// The packet cards do not cover every export boundary.
    ExportBoundaryCoverageMissing,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control lets generic ticket/task wording conceal provenance, destination, or boundary.
    GenericTicketWordingUsed,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl EvidenceHandoffViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RelatedEvidenceCardsMissing => "related_evidence_cards_missing",
            Self::RelatedEvidenceCardIncomplete => "related_evidence_card_incomplete",
            Self::RelatedEvidenceCardWrongComponentClass => {
                "related_evidence_card_wrong_component_class"
            }
            Self::EvidenceFreshnessMisrepresented => "evidence_freshness_misrepresented",
            Self::RawArtifactDumpedBeforeSummary => "raw_artifact_dumped_before_summary",
            Self::EvidenceSummaryMissing => "evidence_summary_missing",
            Self::EvidenceFreshnessNoteMissing => "evidence_freshness_note_missing",
            Self::FailingEvidenceNoteMissing => "failing_evidence_note_missing",
            Self::EvidenceOpenDetailMissing => "evidence_open_detail_missing",
            Self::EvidenceKindCoverageMissing => "evidence_kind_coverage_missing",
            Self::EvidenceOutcomeCoverageMissing => "evidence_outcome_coverage_missing",
            Self::EvidenceFreshnessCoverageMissing => "evidence_freshness_coverage_missing",
            Self::OfflineHandoffCardsMissing => "offline_handoff_cards_missing",
            Self::OfflineHandoffCardIncomplete => "offline_handoff_card_incomplete",
            Self::OfflineHandoffCardWrongComponentClass => {
                "offline_handoff_card_wrong_component_class"
            }
            Self::PacketAcceptanceClassMisrepresented => "packet_acceptance_class_misrepresented",
            Self::ProviderAcceptanceMisrepresented => "provider_acceptance_misrepresented",
            Self::PacketTypeLabelMissing => "packet_type_label_missing",
            Self::IncludedContentSummaryMissing => "included_content_summary_missing",
            Self::RedactionStateNoteMissing => "redaction_state_note_missing",
            Self::PublishLaterTargetMissing => "publish_later_target_missing",
            Self::FailureRecoveryNoteMissing => "failure_recovery_note_missing",
            Self::PacketRetryActionMissing => "packet_retry_action_missing",
            Self::CopyExportActionMissing => "copy_export_action_missing",
            Self::PacketCollapsedIntoErrorBanner => "packet_collapsed_into_error_banner",
            Self::PacketAcceptanceClassCoverageMissing => {
                "packet_acceptance_class_coverage_missing"
            }
            Self::HandoffDestinationCoverageMissing => "handoff_destination_coverage_missing",
            Self::ExportBoundaryCoverageMissing => "export_boundary_coverage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::GenericTicketWordingUsed => "generic_ticket_wording_used",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable related-evidence / offline-handoff export.
pub fn current_evidence_handoff_export(
) -> Result<EvidenceHandoffControlsPacket, EvidenceHandoffArtifactError> {
    let packet: EvidenceHandoffControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-related-evidence-offline-handoff-proof/support_export.json"
    )))
    .map_err(EvidenceHandoffArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EvidenceHandoffArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &EvidenceHandoffControlsPacket,
    violations: &mut Vec<EvidenceHandoffViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        EVIDENCE_HANDOFF_SCHEMA_REF,
        EVIDENCE_HANDOFF_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_RELATED_EVIDENCE_CARD_SCHEMA_REF,
        M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(EvidenceHandoffViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_evidence_cards(
    packet: &EvidenceHandoffControlsPacket,
    violations: &mut Vec<EvidenceHandoffViolation>,
) {
    if packet.related_evidence_cards.is_empty() {
        violations.push(EvidenceHandoffViolation::RelatedEvidenceCardsMissing);
        return;
    }

    let mut evidence_kinds: BTreeSet<M5WorkItemEvidenceKind> = BTreeSet::new();
    let mut outcomes: BTreeSet<EvidenceOutcomeClass> = BTreeSet::new();
    let mut freshness_classes: BTreeSet<EvidenceFreshnessClass> = BTreeSet::new();

    for card in &packet.related_evidence_cards {
        let disclosure = card.evidence_disclosure();
        evidence_kinds.insert(card.evidence_kind);
        outcomes.insert(card.evidence_outcome);
        freshness_classes.insert(disclosure.freshness_class);

        if card.card_id.trim().is_empty()
            || card.canonical_id.trim().is_empty()
            || card.source_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(EvidenceHandoffViolation::RelatedEvidenceCardIncomplete);
        }
        if card.component != M5WorkItemComponentFamily::RelatedEvidenceCard {
            violations.push(EvidenceHandoffViolation::RelatedEvidenceCardWrongComponentClass);
        }
        if card.freshness_class != disclosure.freshness_class {
            violations.push(EvidenceHandoffViolation::EvidenceFreshnessMisrepresented);
        }
        // AC1: summary-first evidence instead of dumping raw artifacts first.
        if !card.leads_with_summary {
            violations.push(EvidenceHandoffViolation::RawArtifactDumpedBeforeSummary);
        }
        if card.summary_label.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::EvidenceSummaryMissing);
        }
        if disclosure.needs_freshness_note && card.freshness_note.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::EvidenceFreshnessNoteMissing);
        }
        if disclosure.needs_failure_note && card.failure_note.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::FailingEvidenceNoteMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(EvidenceHandoffViolation::EvidenceOpenDetailMissing);
        }
        if card.accessibility_routes.is_empty()
            || !card
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(EvidenceHandoffViolation::AccessibilityRouteMissing);
        }
        if card.uses_generic_ticket_wording {
            violations.push(EvidenceHandoffViolation::GenericTicketWordingUsed);
        }
    }

    for required in M5WorkItemEvidenceKind::ALL {
        if !evidence_kinds.contains(&required) {
            violations.push(EvidenceHandoffViolation::EvidenceKindCoverageMissing);
            break;
        }
    }
    for required in EvidenceOutcomeClass::ALL {
        if !outcomes.contains(&required) {
            violations.push(EvidenceHandoffViolation::EvidenceOutcomeCoverageMissing);
            break;
        }
    }
    for required in EvidenceFreshnessClass::ALL {
        if !freshness_classes.contains(&required) {
            violations.push(EvidenceHandoffViolation::EvidenceFreshnessCoverageMissing);
            break;
        }
    }
}

fn validate_packet_cards(
    packet: &EvidenceHandoffControlsPacket,
    violations: &mut Vec<EvidenceHandoffViolation>,
) {
    if packet.offline_handoff_packet_cards.is_empty() {
        violations.push(EvidenceHandoffViolation::OfflineHandoffCardsMissing);
        return;
    }

    let mut acceptance_classes: BTreeSet<PacketAcceptanceClass> = BTreeSet::new();
    let mut destinations: BTreeSet<M5WorkItemHandoffDestination> = BTreeSet::new();
    let mut boundaries: BTreeSet<M5WorkItemExportBoundary> = BTreeSet::new();

    for card in &packet.offline_handoff_packet_cards {
        let disclosure = card.packet_disclosure();
        acceptance_classes.insert(disclosure.acceptance_class);
        destinations.insert(card.handoff_destination);
        boundaries.insert(card.export_boundary);

        if card.card_id.trim().is_empty()
            || card.canonical_id.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(EvidenceHandoffViolation::OfflineHandoffCardIncomplete);
        }
        if card.component != M5WorkItemComponentFamily::OfflineHandoffPacketCard {
            violations.push(EvidenceHandoffViolation::OfflineHandoffCardWrongComponentClass);
        }
        if card.acceptance_class != disclosure.acceptance_class {
            violations.push(EvidenceHandoffViolation::PacketAcceptanceClassMisrepresented);
        }
        // AC: a held, queued, or failed packet never implies provider acceptance.
        if card.implies_provider_accepted != disclosure.implies_provider_accepted {
            violations.push(EvidenceHandoffViolation::ProviderAcceptanceMisrepresented);
        }
        if card.packet_type_label.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::PacketTypeLabelMissing);
        }
        if card.included_content_summary.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::IncludedContentSummaryMissing);
        }
        if card.redaction_state_note.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::RedactionStateNoteMissing);
        }
        if card.publish_later_target_label.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::PublishLaterTargetMissing);
        }
        if disclosure.needs_failure_recovery_note && card.failure_recovery_note.trim().is_empty() {
            violations.push(EvidenceHandoffViolation::FailureRecoveryNoteMissing);
        }
        // AC2: offline packets remain retryable after failure.
        if disclosure.needs_retry_action && !card.offers_retry() {
            violations.push(EvidenceHandoffViolation::PacketRetryActionMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(EvidenceHandoffViolation::CopyExportActionMissing);
        }
        // AC2: offline packets stay visible rather than collapsing into a generic banner.
        if !card.remains_visible_after_failure || card.collapses_into_error_banner {
            violations.push(EvidenceHandoffViolation::PacketCollapsedIntoErrorBanner);
        }
        if card.accessibility_routes.is_empty()
            || !card
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(EvidenceHandoffViolation::AccessibilityRouteMissing);
        }
        if card.uses_generic_ticket_wording {
            violations.push(EvidenceHandoffViolation::GenericTicketWordingUsed);
        }
    }

    for required in PacketAcceptanceClass::ALL {
        if !acceptance_classes.contains(&required) {
            violations.push(EvidenceHandoffViolation::PacketAcceptanceClassCoverageMissing);
            break;
        }
    }
    for required in M5WorkItemHandoffDestination::ALL {
        if !destinations.contains(&required) {
            violations.push(EvidenceHandoffViolation::HandoffDestinationCoverageMissing);
            break;
        }
    }
    for required in M5WorkItemExportBoundary::ALL {
        if !boundaries.contains(&required) {
            violations.push(EvidenceHandoffViolation::ExportBoundaryCoverageMissing);
            break;
        }
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
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
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
