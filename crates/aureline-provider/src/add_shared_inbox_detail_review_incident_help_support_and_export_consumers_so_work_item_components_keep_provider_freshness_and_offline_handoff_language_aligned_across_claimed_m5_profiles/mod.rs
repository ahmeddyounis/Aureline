//! Shared consumers for the reusable M5 work-item components, so the work-item row,
//! provider-chip group, relation strip, sync-pending pill, work-item detail header,
//! status-transition sheet, related-evidence card, and offline-handoff-packet card keep
//! canonical-identity, provider-authority, local-versus-provider, linked-context,
//! side-effect, and publish-later language aligned across every claimed M5 work-item surface
//! where a user reads an issue inbox, opens a work-item detail, reviews a change, works an
//! incident, reads Help / docs, exports a support case, or hands a packet off to another
//! device.
//!
//! Aureline's frozen work-item component matrix
//! (`crate::freeze_the_m5_work_item_component_matrix`) names the eight governed component
//! families, and four sibling implement lanes narrow those families into working primitives,
//! each with its own canonical schema, contract doc, and support-export artifact:
//!
//! * the work-item row and provider-chip group
//!   (`implement_work_item_rows_and_provider_chip_groups_...`),
//! * the relation strip and sync-pending pill
//!   (`implement_relation_strips_and_sync_pending_pills_...`),
//! * the work-item detail header and status-transition sheet
//!   (`implement_work_item_detail_headers_and_status_transition_sheets_...`), and
//! * the related-evidence card and offline-handoff-packet card
//!   (`implement_related_evidence_cards_and_offline_handoff_packet_cards_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the eight families
//! are reusable components — not one tracker view plus a few isolated export objects — by
//! binding every claimed M5 work-item consumer (the issue inbox, the work-item detail, the
//! review workspace, the incident workspace, Help / docs, the support / export desk, and the
//! offline export packet) to the same canonical component schemas and the same descriptor
//! vocabulary. Each consumer points at the primitive's canonical schema and support-export
//! artifact rather than re-wording identity, authority, freshness, linked-context,
//! side-effect, or publish-later facts in local prose, and each keeps that vocabulary
//! truthful even when provider scope is limited, a change is only queued locally, an
//! offline-handoff packet stays local-only, or a linked branch/review/test relation has gone
//! stale.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_work_item_component_binding`] — that takes one consumer's
//!    adoption of one component family, the descriptor set it surfaces, the parity-health
//!    mode it renders under, and any export caveats, and produces one
//!    [`M5WorkItemComponentResolvedBinding`] carrying the derived claim-parity state and —
//!    whenever parity is weakened — a self-contained [`M5WorkItemComponentAutoNarrowBanner`]
//!    that names the exact reason (limited provider scope, a still-queued local change, a
//!    local-only offline-handoff packet, or a stale linked relation), the descriptors that
//!    stay preserved, and the recovery action, rather than a generic "degraded" note. The
//!    resolver never lets a narrowed context drop a required descriptor and never lets a
//!    queued-local or offline-captured change masquerade as provider-committed state.
//! 2. A parity matrix — [`M5WorkItemComponentConsumerPacket`] — that binds one row per
//!    claimed M5 work-item consumer to the eight canonical component families, the one shared
//!    descriptor vocabulary, the same parity-health modes, export caveats, parity states,
//!    narrowing reasons, recovery actions, export fields, and non-visual accessibility
//!    routes, so canonical-identity / provider-authority / local-versus-provider /
//!    linked-context / side-effect / publish-later facts stop diverging between the primary
//!    UX, the docs, and the support / export artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the eight component families themselves are
//! reused verbatim from the frozen work-item component matrix. This module mints new
//! vocabulary only for what the adoption lane itself needs: its work-item consumers, the
//! shared descriptor vocabulary, the parity-health modes, the export caveats, the
//! claim-parity states, the narrowing reasons and recovery actions, the consumer anatomy
//! parts, and the export fields.
//!
//! Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the
//! support boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is `schemas/ui/m5-work-item-component-consumer.schema.json` and the
//! contract doc is `docs/team-workflows/m5_work_item_component_consumers.md`. The protected
//! fixture directory is `fixtures/ui/m5-work-item-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_work_item_component_consumer_incident_beta_narrowed,
    seeded_m5_work_item_component_consumer_packet,
    seeded_m5_work_item_component_consumer_review_preview_narrowed,
    M5_WORK_ITEM_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the eight component families are frozen
// once, in the work-item component matrix. This adoption lane reuses them verbatim so it
// never invents a parallel work-item vocabulary.
pub use crate::freeze_the_m5_work_item_component_matrix::{
    M5WorkItemAccessibilityRoute, M5WorkItemComponentFamily, M5WorkItemConsumerSurface,
    M5WorkItemDeploymentLine, M5WorkItemDowngradeTrigger, M5WorkItemQualificationClass,
    M5WorkItemSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather
// than re-wording their facts in local prose.
use crate::freeze_the_m5_work_item_component_matrix::{
    M5_WORK_ITEM_COMPONENT_DOC_REF, M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_related_evidence_cards_and_offline_handoff_packet_cards_with_summary_first_evidence_redaction_state_publish_later_target_and_copy_export_retry_truth::{
    EVIDENCE_HANDOFF_ARTIFACT_REF, EVIDENCE_HANDOFF_DOC_REF, EVIDENCE_HANDOFF_SCHEMA_REF,
};
use crate::implement_relation_strips_and_sync_pending_pills_with_linked_context_stale_labeling_and_retry_or_export_continuity::{
    RELATION_STRIP_SYNC_PENDING_ARTIFACT_REF, RELATION_STRIP_SYNC_PENDING_DOC_REF,
    RELATION_STRIP_SYNC_PENDING_SCHEMA_REF,
};
use crate::implement_work_item_detail_headers_and_status_transition_sheets_with_provider_boundary_side_effect_permission_scope_and_confirm_export_cancel_truth::{
    DETAIL_HEADER_TRANSITION_ARTIFACT_REF, DETAIL_HEADER_TRANSITION_DOC_REF,
    DETAIL_HEADER_TRANSITION_SCHEMA_REF,
};
use crate::implement_work_item_rows_and_provider_chip_groups_with_canonical_id_owner_state_freshness_and_write_scope_truth::{
    WORK_ITEM_ROW_PROVIDER_CHIP_ARTIFACT_REF, WORK_ITEM_ROW_PROVIDER_CHIP_DOC_REF,
    WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5WorkItemComponentConsumerPacket`].
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_inbox_detail_review_incident_help_support_and_export_consumers_so_work_item_components_keep_provider_freshness_and_offline_handoff_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 work-item component-consumer records.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the work-item component-consumer boundary schema.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-work-item-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/team-workflows/m5_work_item_component_consumers.md";

/// Repo-relative path of the frozen work-item component matrix this lane adopts from.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_WORK_ITEM_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-work-item-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-work-item-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-work-item-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-work-item-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer
/// that adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5WorkItemComponentFamily) -> &'static str {
    use M5WorkItemComponentFamily as Family;
    match family {
        Family::WorkItemRow | Family::ProviderChipGroup => WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF,
        Family::RelationStrip | Family::SyncPendingPill => RELATION_STRIP_SYNC_PENDING_SCHEMA_REF,
        Family::WorkItemDetailHeader | Family::StatusTransitionSheet => {
            DETAIL_HEADER_TRANSITION_SCHEMA_REF
        }
        Family::RelatedEvidenceCard | Family::OfflineHandoffPacketCard => {
            EVIDENCE_HANDOFF_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5WorkItemComponentFamily) -> &'static str {
    use M5WorkItemComponentFamily as Family;
    match family {
        Family::WorkItemRow | Family::ProviderChipGroup => WORK_ITEM_ROW_PROVIDER_CHIP_DOC_REF,
        Family::RelationStrip | Family::SyncPendingPill => RELATION_STRIP_SYNC_PENDING_DOC_REF,
        Family::WorkItemDetailHeader | Family::StatusTransitionSheet => {
            DETAIL_HEADER_TRANSITION_DOC_REF
        }
        Family::RelatedEvidenceCard | Family::OfflineHandoffPacketCard => EVIDENCE_HANDOFF_DOC_REF,
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(family: M5WorkItemComponentFamily) -> &'static str {
    use M5WorkItemComponentFamily as Family;
    match family {
        Family::WorkItemRow | Family::ProviderChipGroup => WORK_ITEM_ROW_PROVIDER_CHIP_ARTIFACT_REF,
        Family::RelationStrip | Family::SyncPendingPill => RELATION_STRIP_SYNC_PENDING_ARTIFACT_REF,
        Family::WorkItemDetailHeader | Family::StatusTransitionSheet => {
            DETAIL_HEADER_TRANSITION_ARTIFACT_REF
        }
        Family::RelatedEvidenceCard | Family::OfflineHandoffPacketCard => {
            EVIDENCE_HANDOFF_ARTIFACT_REF
        }
    }
}

/// One claimed M5 work-item consumer that adopts the shared components. These are the
/// consumers the spec names — the issue inbox, the work-item detail, the review workspace, the
/// incident workspace, Help / docs, the support / export desk, and the offline export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentConsumer {
    /// The issue-inbox surface.
    Inbox,
    /// The work-item detail surface.
    Detail,
    /// The review-workspace surface.
    Review,
    /// The incident-workspace surface.
    Incident,
    /// The Help / docs surface.
    Help,
    /// The support / export desk and support-bundle preview.
    Support,
    /// The offline export packet / exported view.
    Export,
}

impl M5WorkItemComponentConsumer {
    /// Every claimed work-item consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Inbox,
        Self::Detail,
        Self::Review,
        Self::Incident,
        Self::Help,
        Self::Support,
        Self::Export,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inbox => "inbox",
            Self::Detail => "detail",
            Self::Review => "review",
            Self::Incident => "incident",
            Self::Help => "help",
            Self::Support => "support",
            Self::Export => "export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Inbox => "Issue Inbox",
            Self::Detail => "Work-Item Detail",
            Self::Review => "Review Workspace",
            Self::Incident => "Incident Workspace",
            Self::Help => "Help / Docs",
            Self::Support => "Support / Export Desk",
            Self::Export => "Offline Export Packet",
        }
    }

    /// True when this consumer is a help, support, or export surface — the surfaces singled out
    /// for a canonical-schema reference so their prose can never drift from the product truth.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(self, Self::Help | Self::Support | Self::Export)
    }
}

/// The one shared descriptor vocabulary every work-item component keeps aligned across
/// surfaces, so no consumer invents a new grammar or stale wording. The descriptors in
/// [`M5WorkItemComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that canonical identity, provider authority, local-versus-provider
/// state, linked context, the side-effect preview, and publish-later continuity stay one truth
/// across in-product and exported work-item surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentDescriptor {
    /// The canonical work-item identity / kind descriptor.
    CanonicalIdentity,
    /// The provider-authority / who-owns-the-object descriptor.
    ProviderAuthority,
    /// The local-versus-provider / sync-pending state descriptor.
    LocalVersusProviderState,
    /// The linked engineering context (branch / review / test / incident) descriptor.
    LinkedEngineeringContext,
    /// The side-effect preview descriptor.
    SideEffectPreview,
    /// The publish-later / offline-handoff continuity descriptor.
    PublishLaterContinuity,
}

impl M5WorkItemComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CanonicalIdentity,
        Self::ProviderAuthority,
        Self::LocalVersusProviderState,
        Self::LinkedEngineeringContext,
        Self::SideEffectPreview,
        Self::PublishLaterContinuity,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 6] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalIdentity => "canonical_identity",
            Self::ProviderAuthority => "provider_authority",
            Self::LocalVersusProviderState => "local_versus_provider_state",
            Self::LinkedEngineeringContext => "linked_engineering_context",
            Self::SideEffectPreview => "side_effect_preview",
            Self::PublishLaterContinuity => "publish_later_continuity",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the
/// authoritative work-item rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerParityHealth {
    /// Full parity: the authoritative work-item rendering.
    FullParity,
    /// Limited provider scope weakens parity (Aureline cannot write everything here).
    ProviderScopeLimitedNarrowed,
    /// A still-queued local change weakens parity (the change is not published yet).
    SyncPendingNarrowed,
    /// A local-only offline-handoff packet weakens parity (queued handoff is not committed).
    OfflineHandoffNarrowed,
    /// A stale linked relation weakens parity (the linked branch/review/test is not current).
    LinkedContextStaleNarrowed,
}

impl M5WorkItemConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ProviderScopeLimitedNarrowed,
        Self::SyncPendingNarrowed,
        Self::OfflineHandoffNarrowed,
        Self::LinkedContextStaleNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ProviderScopeLimitedNarrowed => "provider_scope_limited_narrowed",
            Self::SyncPendingNarrowed => "sync_pending_narrowed",
            Self::OfflineHandoffNarrowed => "offline_handoff_narrowed",
            Self::LinkedContextStaleNarrowed => "linked_context_stale_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5WorkItemConsumerNarrowingReason> {
        Some(match self {
            Self::ProviderScopeLimitedNarrowed => {
                M5WorkItemConsumerNarrowingReason::ProviderScopeLimited
            }
            Self::SyncPendingNarrowed => M5WorkItemConsumerNarrowingReason::SyncPending,
            Self::OfflineHandoffNarrowed => {
                M5WorkItemConsumerNarrowingReason::OfflineHandoffLocalOnly
            }
            Self::LinkedContextStaleNarrowed => {
                M5WorkItemConsumerNarrowingReason::LinkedContextStale
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner
/// never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerNarrowingReason {
    /// Provider scope is limited, so Aureline cannot write everything on this surface.
    ProviderScopeLimited,
    /// The change is still queued locally, so it is not published to the provider yet.
    SyncPending,
    /// The offline-handoff packet remains local-only, so queued handoff is not committed.
    OfflineHandoffLocalOnly,
    /// The linked branch/review/test relation is stale, so it is not current provider context.
    LinkedContextStale,
}

impl M5WorkItemConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderScopeLimited,
        Self::SyncPending,
        Self::OfflineHandoffLocalOnly,
        Self::LinkedContextStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderScopeLimited => "provider_scope_limited",
            Self::SyncPending => "sync_pending",
            Self::OfflineHandoffLocalOnly => "offline_handoff_local_only",
            Self::LinkedContextStale => "linked_context_stale",
        }
    }

    /// True when the reason reflects queued-local or offline-captured state that must never
    /// masquerade as provider-committed state — the acceptance-criterion boundary for a
    /// still-queued change or a local-only offline-handoff packet.
    pub const fn is_queued_or_offline(self) -> bool {
        matches!(self, Self::SyncPending | Self::OfflineHandoffLocalOnly)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ProviderScopeLimited => {
                "provider scope is limited, so Aureline cannot write everything here and the component stays read-or-limited-write"
            }
            Self::SyncPending => {
                "the change is still queued locally, so it reflects a local draft and is not provider-committed state"
            }
            Self::OfflineHandoffLocalOnly => {
                "the offline-handoff packet remains local-only, so queued handoff is captured locally and is not provider-committed yet"
            }
            Self::LinkedContextStale => {
                "the linked branch, review, or test relation is stale, so it is not current provider context"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5WorkItemConsumerRecoveryAction {
        match self {
            Self::ProviderScopeLimited => M5WorkItemConsumerRecoveryAction::ReauthorizeForFullScope,
            Self::SyncPending => M5WorkItemConsumerRecoveryAction::PublishOrRetryQueuedWhenOnline,
            Self::OfflineHandoffLocalOnly => {
                M5WorkItemConsumerRecoveryAction::ExportOrPublishHandoffPacket
            }
            Self::LinkedContextStale => M5WorkItemConsumerRecoveryAction::RelinkOrRefreshContext,
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable
/// from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerRecoveryAction {
    /// Reauthorize / sign in for full write scope before treating the component as writable.
    ReauthorizeForFullScope,
    /// Publish or retry the queued change once online before treating it as provider-committed.
    PublishOrRetryQueuedWhenOnline,
    /// Export or publish the offline-handoff packet before treating it as provider-committed.
    ExportOrPublishHandoffPacket,
    /// Relink or refresh the stale relation before treating it as current provider context.
    RelinkOrRefreshContext,
}

impl M5WorkItemConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReauthorizeForFullScope,
        Self::PublishOrRetryQueuedWhenOnline,
        Self::ExportOrPublishHandoffPacket,
        Self::RelinkOrRefreshContext,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReauthorizeForFullScope => "reauthorize_for_full_scope",
            Self::PublishOrRetryQueuedWhenOnline => "publish_or_retry_queued_when_online",
            Self::ExportOrPublishHandoffPacket => "export_or_publish_handoff_packet",
            Self::RelinkOrRefreshContext => "relink_or_refresh_context",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the authoritative
/// work-item surface (limited scope, a still-queued change, a local-only offline-handoff
/// packet, or a stale linked relation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerExportCaveat {
    /// Provider scope is limited, so the component is read-only or limited-write.
    ScopeLimitedReadOnly,
    /// The change is still queued locally, so it is not committed to the provider yet.
    SyncPendingNotCommitted,
    /// The offline-handoff packet is local-only, so queued handoff is not committed yet.
    OfflineHandoffLocalOnly,
    /// The linked relation is stale, so it is not authoritative provider context.
    LinkedContextStaleNotAuthoritative,
}

impl M5WorkItemConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ScopeLimitedReadOnly,
        Self::SyncPendingNotCommitted,
        Self::OfflineHandoffLocalOnly,
        Self::LinkedContextStaleNotAuthoritative,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeLimitedReadOnly => "scope_limited_read_only",
            Self::SyncPendingNotCommitted => "sync_pending_not_committed",
            Self::OfflineHandoffLocalOnly => "offline_handoff_local_only",
            Self::LinkedContextStaleNotAuthoritative => "linked_context_stale_not_authoritative",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is
/// preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5WorkItemClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5WorkItemConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5WorkItemConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable from the
/// shared model. The fields in [`M5WorkItemConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5WorkItemConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay preserved,
/// the export caveats, and the recovery action, so a narrowed rendering is understood from the
/// banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5WorkItemConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5WorkItemConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5WorkItemComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5WorkItemComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5WorkItemComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5WorkItemConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors,
    /// and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the work-item component-binding resolver for one consumer/family
/// adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5WorkItemComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5WorkItemComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so
    /// canonical identity, provider authority, local-versus-provider state, linked context, the
    /// side-effect preview, and publish-later continuity stay explicit.
    pub descriptor_families: Vec<M5WorkItemComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5WorkItemConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5WorkItemConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5WorkItemComponentConsumer,
    /// The component family.
    pub component_family: M5WorkItemComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5WorkItemComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5WorkItemConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5WorkItemConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5WorkItemClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects queued-local or offline-captured state (a still-queued
    /// change or a local-only offline-handoff packet). Such a binding must always be narrowed
    /// and never asserts provider-committed state.
    pub reflects_queued_or_offline_state: bool,
    /// Hard invariant: whether this binding claims provider-committed state. Only a full-parity
    /// binding may reflect a committed publish; every narrowed binding — and in particular any
    /// queued or offline-captured one — resolves this to `false`.
    pub asserts_provider_committed: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5WorkItemComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_work_item_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WorkItemComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5WorkItemComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5WorkItemComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "work-item component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WorkItemComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that canonical
/// identity, provider authority, local-versus-provider state, linked context, the side-effect
/// preview, and publish-later continuity stay explicit on every surface. The claim-parity state
/// is preserved at full parity and auto-narrowed under any weakened parity-health mode, and a
/// weakened mode always produces a self-contained banner naming the exact reason and recovery
/// action while keeping the descriptor vocabulary intact. Queued-local or offline-captured
/// state (a still-queued change or a local-only offline-handoff packet) always narrows and
/// never asserts provider-committed state.
pub fn resolve_work_item_component_binding(
    input: &M5WorkItemComponentBindingInput,
) -> Result<M5WorkItemComponentResolvedBinding, M5WorkItemComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5WorkItemComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5WorkItemComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5WorkItemComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5WorkItemComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5WorkItemComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text
        // extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5WorkItemComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_queued_or_offline_state =
        narrowing_reason.is_some_and(M5WorkItemConsumerNarrowingReason::is_queued_or_offline);
    // Only a full-parity binding may reflect a committed provider publish. Every narrowed
    // binding — and every queued / offline-captured one in particular — is not committed.
    let asserts_provider_committed = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5WorkItemClaimParityState::ClaimsAutoNarrowed
    } else {
        M5WorkItemClaimParityState::ClaimsPreserved
    };

    let auto_narrow_banner = narrowing_reason.map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5WorkItemComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5WorkItemComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_queued_or_offline_state,
        asserts_provider_committed,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentBindingCase {
    /// The resolver input.
    pub input: M5WorkItemComponentBindingInput,
    /// The resolved truth. Must equal `resolve_work_item_component_binding(&input)`.
    pub resolved: M5WorkItemComponentResolvedBinding,
}

impl M5WorkItemComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5WorkItemComponentBindingInput) -> Self {
        let resolved =
            resolve_work_item_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_work_item_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer
/// points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5WorkItemComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical schema
    /// ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the family's
    /// canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description
    /// of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5WorkItemComponentBindingCase>,
}

impl M5WorkItemComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical
    /// family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one work-item consumer bound to the canonical component
/// families, the shared descriptor vocabulary, the parity-health modes, export caveats, parity
/// states, narrowing reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerRow {
    /// Work-item consumer.
    pub consumer: M5WorkItemComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5WorkItemQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 work-item surface families that render / consume this projection.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5WorkItemConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5WorkItemComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5WorkItemConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5WorkItemConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5WorkItemClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5WorkItemConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5WorkItemConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5WorkItemConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5WorkItemComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new work-item grammar. MUST be `false`.
    pub invents_new_work_item_grammar: bool,
    /// Hard invariant: this consumer never drops identity, authority, freshness, linked-context,
    /// side-effect, or publish-later truth when narrowed. MUST be `false`.
    pub drops_identity_authority_freshness_or_publish_later_when_narrowed: bool,
    /// Hard invariant: this consumer never shows queued-local or offline-captured state as
    /// provider-committed state. MUST be `false`.
    pub shows_queued_or_offline_state_as_committed: bool,
    /// Hard invariant: this consumer never inherits a stronger label from a healthier profile
    /// instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_profile: bool,
    /// Hard invariant: this consumer never lets generic ticket / task wording conceal provider
    /// ownership, queued state, offline capture, or linked context. MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl M5WorkItemComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5WorkItemConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5WorkItemConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5WorkItemConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5WorkItemConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5WorkItemComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5WorkItemComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5WorkItemComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5WorkItemComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_work_item_grammar
            && !self.drops_identity_authority_freshness_or_publish_later_when_narrowed
            && !self.shows_queued_or_offline_state_as_committed
            && !self.inherits_stronger_label_from_healthier_profile
            && !self.uses_generic_ticket_wording
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerVocabularySet {
    /// Work-item-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5WorkItemComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5WorkItemComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5WorkItemComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5WorkItemComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5WorkItemConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5WorkItemConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5WorkItemConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5WorkItemConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5WorkItemClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5WorkItemConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5WorkItemConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5WorkItemAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new work-item grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Canonical identity, provider authority, local-versus-provider state, linked context, the
    /// side-effect preview, and publish-later continuity stay explicit everywhere.
    pub identity_authority_state_context_side_effect_publish_later_explicit_on_every_surface: bool,
    /// Limited scope, still-queued changes, local-only offline-handoff packets, and stale
    /// relations auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// Queued-local or offline-captured state never masquerades as provider-committed state.
    pub queued_or_offline_state_never_shown_as_committed: bool,
    /// Generic ticket / task wording never conceals provider ownership, queued state, offline
    /// capture, or linked context.
    pub no_generic_ticket_wording_conceals_provider_or_queued_state: bool,
    /// The help / support / export surfaces present the same work-item truth shown in-product.
    pub help_support_export_present_same_work_item_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerProjection {
    /// The inbox, detail, review, incident, Help / docs, the support / export desk, and the
    /// offline export packet all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The canonical-identity descriptor reads a single canonical source.
    pub canonical_identity_reads_single_source: bool,
    /// The provider-authority descriptor reads a single canonical source.
    pub provider_authority_reads_single_source: bool,
    /// The local-versus-provider-state descriptor reads a single canonical source.
    pub local_versus_provider_state_reads_single_source: bool,
    /// The linked-engineering-context descriptor reads a single canonical source.
    pub linked_engineering_context_reads_single_source: bool,
    /// The publish-later-continuity descriptor reads a single canonical source.
    pub publish_later_continuity_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting work-item consumer audit.
    pub work_item_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WorkItemComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkItemComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5WorkItemComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkItemComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkItemComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkItemComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkItemComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkItemComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 work-item component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerPacket {
    /// Record kind; must equal [`M5_WORK_ITEM_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5WorkItemComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkItemComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkItemComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkItemComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkItemComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkItemComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WorkItemComponentConsumerPacket {
    /// Builds an M5 work-item component-consumer packet from stable-lane input.
    pub fn new(input: M5WorkItemComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_WORK_ITEM_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 work-item component-consumer invariants.
    pub fn validate(&self) -> Vec<M5WorkItemComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WORK_ITEM_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5WorkItemComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5WorkItemComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WorkItemComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_commit_honesty(self, &mut violations);
        validate_help_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 work-item component consumer packet serializes"),
        ) {
            violations.push(M5WorkItemComponentConsumerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 work-item component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Work-Item Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Work-item consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Work-item consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 work-item component-consumer export.
#[derive(Debug)]
pub enum M5WorkItemComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WorkItemComponentConsumerViolation>),
}

impl fmt::Display for M5WorkItemComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 work-item component consumer export parse failed: {error}"
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
                    "m5 work-item component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WorkItemComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5WorkItemComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WorkItemComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required work-item consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer (reuse
    /// across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no banner.
    ScopePreservedUnproven,
    /// No worked binding proves that queued-local or offline-captured state narrows and never
    /// asserts provider-committed state, or a binding does so incorrectly.
    CommitHonestyUnproven,
    /// A help / support / export consumer does not reference the canonical component schema.
    HelpSupportExportReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5WorkItemComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::CommitHonestyUnproven => "commit_honesty_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 work-item component-consumer export.
pub fn current_stable_m5_work_item_component_consumer_export(
) -> Result<M5WorkItemComponentConsumerPacket, M5WorkItemComponentConsumerArtifactError> {
    let packet: M5WorkItemComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-work-item-component-consumer-proof/support_export.json"
    )))
    .map_err(M5WorkItemComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WorkItemComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_CONSUMER_DOC_REF,
        M5_WORK_ITEM_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_WORK_ITEM_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF,
        RELATION_STRIP_SYNC_PENDING_SCHEMA_REF,
        DETAIL_HEADER_TRANSITION_SCHEMA_REF,
        EVIDENCE_HANDOFF_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WorkItemComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5WorkItemComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let present: BTreeSet<M5WorkItemComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5WorkItemComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5WorkItemComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5WorkItemComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5WorkItemComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5WorkItemComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5WorkItemComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5WorkItemComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5WorkItemComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5WorkItemComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5WorkItemComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5WorkItemComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5WorkItemComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5WorkItemComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5WorkItemComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5WorkItemComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one tracker
/// view plus a few isolated export objects.
fn validate_family_reuse(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    for family in M5WorkItemComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5WorkItemComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved descriptors —
/// the acceptance-criterion example that a consumer which cannot preserve parity is visibly
/// narrowed rather than inheriting stronger labels from healthier profiles.
fn validate_narrowing_disclosure(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5WorkItemComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with
/// preserved parity and no banner — the acceptance-criterion example that full-parity consumers
/// keep the descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5WorkItemClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5WorkItemComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects queued-local or offline-captured state must be narrowed
/// and must not assert provider-committed state, and at least one such binding must be present —
/// the acceptance-criterion that queued-local or offline-captured state no longer masquerades as
/// provider-committed state on any claimed consumer.
fn validate_commit_honesty(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_queued_or_offline_state {
            // A queued / offline binding that claims commit, or fails to narrow, breaks AC2.
            if resolved.asserts_provider_committed
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5WorkItemClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5WorkItemComponentConsumerViolation::CommitHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5WorkItemComponentConsumerViolation::CommitHonestyUnproven);
    }
}

/// The help / support / export consumers must reference the canonical component schema for each
/// family they adopt — the acceptance-criterion that a help, support, or export lane can never
/// drift from the product truth.
fn validate_help_support_export_reference(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_help_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5WorkItemComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations
                .push(M5WorkItemComponentConsumerViolation::HelpSupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.identity_authority_state_context_side_effect_publish_later_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.queued_or_offline_state_never_shown_as_committed,
        review.no_generic_ticket_wording_conceals_provider_or_queued_state,
        review.help_support_export_present_same_work_item_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5WorkItemComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.canonical_identity_reads_single_source,
        projection.provider_authority_reads_single_source,
        projection.local_versus_provider_state_reads_single_source,
        projection.linked_engineering_context_reads_single_source,
        projection.publish_later_continuity_reads_single_source,
    ] {
        if !ok {
            violations.push(M5WorkItemComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WorkItemComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WorkItemComponentConsumerPacket,
    violations: &mut Vec<M5WorkItemComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.work_item_consumer_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WorkItemComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5WorkItemComponentConsumerPacket,
) -> impl Iterator<Item = &M5WorkItemComponentBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
