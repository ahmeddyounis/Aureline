//! Shared consumers for the reusable M5 provider-account / offline-capture components, so
//! the provider-account row, project/board mapping row, sync-behavior row, offline-capture
//! row, and privacy/redaction row keep account-state, destination-mapping, queued-draft, and
//! redaction-posture language aligned across every claimed M5 provider-backed surface where a
//! user reads a work-item detail, reviews a status transition, intakes an issue, reads Help /
//! docs, exports a support case, or completes a browser handoff.
//!
//! Aureline's frozen provider-account / offline-capture component matrix
//! (`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`)
//! names the five governed component families, and three sibling implement lanes narrow
//! those families into working primitives, each with its own canonical schema, contract
//! doc, and support-export artifact:
//!
//! * the provider-account row (`implement_provider_account_rows_...`),
//! * the project/board mapping row and sync-behavior row
//!   (`ship_project_or_board_mapping_rows_and_sync_behavior_rows_...`), and
//! * the offline-capture row and privacy/redaction row
//!   (`implement_offline_capture_rows_and_privacy_redaction_rows_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the five families
//! are reusable components — not one provider-settings page plus a few isolated export
//! objects — by binding every claimed M5 provider consumer (work-item detail, status-
//! transition review, issue intake, Help / docs, the support / export desk, and the browser-
//! handoff flow) to the same canonical component schemas and the same descriptor vocabulary.
//! Each consumer points at the primitive's canonical schema and support-export artifact
//! rather than re-wording account, mapping, sync, queue, or redaction facts in local prose,
//! and each keeps that vocabulary truthful even when provider scope is limited, a session is
//! stale, a mapping is policy-locked, or a packet remains local-only.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_provider_component_binding`] — that takes one consumer's
//!    adoption of one component family, the descriptor set it surfaces, the parity-health
//!    mode it renders under, and any export caveats, and produces one
//!    [`M5ProviderComponentResolvedBinding`] carrying the derived claim-parity state and —
//!    whenever parity is weakened — a self-contained [`M5ProviderComponentAutoNarrowBanner`]
//!    that names the exact reason (limited provider scope, a stale session, a policy-locked
//!    mapping, or a local-only packet), the descriptors that stay preserved, and the recovery
//!    action, rather than a generic "degraded" note. The resolver never lets a narrowed
//!    context drop a required descriptor and never lets cached or offline-captured state
//!    masquerade as provider-committed state.
//! 2. A parity matrix — [`M5ProviderComponentConsumerPacket`] — that binds one row per
//!    claimed M5 provider consumer to the five canonical component families, the one shared
//!    descriptor vocabulary, the same parity-health modes, export caveats, parity states,
//!    narrowing reasons, recovery actions, export fields, and non-visual accessibility
//!    routes, so account-state / destination-mapping / queued-draft / redaction-posture facts
//!    stop diverging between the product UI, the docs, and the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the five component families themselves are
//! reused verbatim from the frozen provider-account / offline-capture component matrix. This
//! module mints new vocabulary only for what the adoption lane itself needs: its provider
//! consumers, the shared descriptor vocabulary, the parity-health modes, the export caveats,
//! the claim-parity states, the narrowing reasons and recovery actions, the consumer anatomy
//! parts, and the export fields.
//!
//! Raw credentials, endpoints, tokens, and raw provider bodies stay outside the support
//! boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! `schemas/ui/m5-provider-account-offline-capture-component-consumer.schema.json` and the
//! contract doc is `docs/providers/m5_provider_account_offline_capture_component_consumers.md`.
//! The protected fixture directory is
//! `fixtures/ui/m5-provider-account-offline-capture-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed,
    seeded_m5_provider_component_consumer_issue_intake_preview_narrowed,
    seeded_m5_provider_component_consumer_packet, M5_PROVIDER_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the five component families are frozen
// once, in the provider-account / offline-capture component matrix. This adoption lane
// reuses them verbatim so it never invents a parallel provider vocabulary.
pub use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5ProviderAccessibilityRoute, M5ProviderAccountOfflineComponentFamily, M5ProviderConsumerSurface,
    M5ProviderDeploymentLine, M5ProviderDowngradeTrigger, M5ProviderQualificationClass,
    M5ProviderSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at,
// rather than re-wording their facts in local prose.
use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_DOC_REF,
    M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_offline_capture_rows_and_privacy_redaction_rows_with_packet_destination_queued_draft_count_export_clear_actions_and_metadata_safe_boundary_truth_across_claimed_m5_provider_workflows::{
    M5_PROVIDER_OFFLINE_PRIVACY_ROW_ARTIFACT_REF, M5_PROVIDER_OFFLINE_PRIVACY_ROW_DOC_REF,
    M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF,
};
use crate::implement_provider_account_rows_with_signed_in_limited_scope_stale_session_offline_cached_policy_blocked_truth_and_sign_in_retry_remove_parity_across_claimed_m5_provider_surfaces::{
    M5_PROVIDER_ACCOUNT_ROW_ARTIFACT_REF, M5_PROVIDER_ACCOUNT_ROW_DOC_REF,
    M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF,
};
use crate::ship_project_or_board_mapping_rows_and_sync_behavior_rows_with_inherited_local_policy_scope_read_only_comment_transition_sync_modes_and_change_reset_parity_across_claimed_m5_provider_lanes::{
    M5_PROVIDER_MAPPING_SYNC_ROW_ARTIFACT_REF, M5_PROVIDER_MAPPING_SYNC_ROW_DOC_REF,
    M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderComponentConsumerPacket`].
pub const M5_PROVIDER_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_work_item_status_transition_help_support_and_export_consumers_so_provider_account_and_offline_capture_components_keep_account_mapping_sync_and_redaction_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 provider-account / offline-capture component-consumer records.
pub const M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the provider component-consumer boundary schema.
pub const M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVIDER_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/providers/m5_provider_account_offline_capture_component_consumers.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix this
/// lane adopts from.
pub const M5_PROVIDER_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_PROVIDER_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str =
    M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-account-offline-capture-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVIDER_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PROVIDER_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A
/// consumer that adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(
    family: M5ProviderAccountOfflineComponentFamily,
) -> &'static str {
    use M5ProviderAccountOfflineComponentFamily as Family;
    match family {
        Family::ProviderAccountRow => M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF,
        Family::ProjectOrBoardMappingRow | Family::SyncBehaviorRow => {
            M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF
        }
        Family::OfflineCaptureRow | Family::PrivacyRedactionRow => {
            M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(
    family: M5ProviderAccountOfflineComponentFamily,
) -> &'static str {
    use M5ProviderAccountOfflineComponentFamily as Family;
    match family {
        Family::ProviderAccountRow => M5_PROVIDER_ACCOUNT_ROW_DOC_REF,
        Family::ProjectOrBoardMappingRow | Family::SyncBehaviorRow => {
            M5_PROVIDER_MAPPING_SYNC_ROW_DOC_REF
        }
        Family::OfflineCaptureRow | Family::PrivacyRedactionRow => {
            M5_PROVIDER_OFFLINE_PRIVACY_ROW_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(
    family: M5ProviderAccountOfflineComponentFamily,
) -> &'static str {
    use M5ProviderAccountOfflineComponentFamily as Family;
    match family {
        Family::ProviderAccountRow => M5_PROVIDER_ACCOUNT_ROW_ARTIFACT_REF,
        Family::ProjectOrBoardMappingRow | Family::SyncBehaviorRow => {
            M5_PROVIDER_MAPPING_SYNC_ROW_ARTIFACT_REF
        }
        Family::OfflineCaptureRow | Family::PrivacyRedactionRow => {
            M5_PROVIDER_OFFLINE_PRIVACY_ROW_ARTIFACT_REF
        }
    }
}

/// One claimed M5 provider consumer that adopts the shared components. These are the
/// consumers the spec names — work-item detail, status-transition review, issue intake,
/// Help / docs, the support / export desk, and the browser-handoff flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderComponentConsumer {
    /// The work-item detail surface.
    WorkItemDetail,
    /// The status-transition review surface.
    StatusTransitionReview,
    /// The issue-intake surface.
    IssueIntake,
    /// The Help / docs surface.
    DocsHelp,
    /// The support / export desk and support-bundle preview.
    SupportExport,
    /// The browser / device-code handoff flow.
    BrowserHandoff,
}

impl M5ProviderComponentConsumer {
    /// Every claimed provider consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkItemDetail,
        Self::StatusTransitionReview,
        Self::IssueIntake,
        Self::DocsHelp,
        Self::SupportExport,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItemDetail => "work_item_detail",
            Self::StatusTransitionReview => "status_transition_review",
            Self::IssueIntake => "issue_intake",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::BrowserHandoff => "browser_handoff",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkItemDetail => "Work-Item Detail",
            Self::StatusTransitionReview => "Status-Transition Review",
            Self::IssueIntake => "Issue Intake",
            Self::DocsHelp => "Help / Docs",
            Self::SupportExport => "Support / Export Desk",
            Self::BrowserHandoff => "Browser Handoff",
        }
    }

    /// True when this consumer is the support / export desk — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// The one shared descriptor vocabulary every provider-account / offline-capture component
/// keeps aligned across surfaces, so no consumer invents a new grammar or stale wording. The
/// descriptors in [`M5ProviderComponentDescriptor::REQUIRED`] must be present on every
/// binding — the acceptance-criterion that account state, destination mapping, queued-draft
/// state, and redaction posture stay one truth across in-product and exported provider
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderComponentDescriptor {
    /// The account-connection-state / identity / tenant-scope descriptor.
    AccountState,
    /// The default-destination / project-or-board mapping descriptor.
    DestinationMapping,
    /// The queued-draft / offline-capture state descriptor.
    QueuedDraftState,
    /// The redaction-class / export-boundary descriptor.
    RedactionPosture,
}

impl M5ProviderComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AccountState,
        Self::DestinationMapping,
        Self::QueuedDraftState,
        Self::RedactionPosture,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountState => "account_state",
            Self::DestinationMapping => "destination_mapping",
            Self::QueuedDraftState => "queued_draft_state",
            Self::RedactionPosture => "redaction_posture",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps
/// the descriptor vocabulary — it only discloses that parity is narrowed relative to the
/// authoritative provider-settings rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerParityHealth {
    /// Full parity: the authoritative provider-settings rendering.
    FullParity,
    /// Limited provider scope weakens parity (Aureline cannot write everything here).
    ScopeLimitedNarrowed,
    /// A stale session weakens parity (only cached reads are trustworthy right now).
    SessionStaleNarrowed,
    /// A policy-locked mapping weakens parity (the destination cannot change here).
    MappingPolicyLockedNarrowed,
    /// A local-only packet weakens parity (queued work is not provider-committed yet).
    PacketLocalOnlyNarrowed,
}

impl M5ProviderConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ScopeLimitedNarrowed,
        Self::SessionStaleNarrowed,
        Self::MappingPolicyLockedNarrowed,
        Self::PacketLocalOnlyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ScopeLimitedNarrowed => "scope_limited_narrowed",
            Self::SessionStaleNarrowed => "session_stale_narrowed",
            Self::MappingPolicyLockedNarrowed => "mapping_policy_locked_narrowed",
            Self::PacketLocalOnlyNarrowed => "packet_local_only_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5ProviderConsumerNarrowingReason> {
        Some(match self {
            Self::ScopeLimitedNarrowed => M5ProviderConsumerNarrowingReason::ProviderScopeLimited,
            Self::SessionStaleNarrowed => M5ProviderConsumerNarrowingReason::SessionStale,
            Self::MappingPolicyLockedNarrowed => {
                M5ProviderConsumerNarrowingReason::MappingPolicyLocked
            }
            Self::PacketLocalOnlyNarrowed => M5ProviderConsumerNarrowingReason::PacketLocalOnly,
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow
/// banner never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerNarrowingReason {
    /// Provider scope is limited, so Aureline cannot write everything on this surface.
    ProviderScopeLimited,
    /// The session is stale, so only cached reads are trustworthy right now.
    SessionStale,
    /// The mapping is policy-locked, so the destination cannot change on this surface.
    MappingPolicyLocked,
    /// The packet remains local-only, so queued work is not provider-committed yet.
    PacketLocalOnly,
}

impl M5ProviderConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderScopeLimited,
        Self::SessionStale,
        Self::MappingPolicyLocked,
        Self::PacketLocalOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderScopeLimited => "provider_scope_limited",
            Self::SessionStale => "session_stale",
            Self::MappingPolicyLocked => "mapping_policy_locked",
            Self::PacketLocalOnly => "packet_local_only",
        }
    }

    /// True when the reason reflects cached or offline-captured state that must never
    /// masquerade as provider-committed state — the acceptance-criterion boundary for a
    /// stale session or a local-only packet.
    pub const fn is_cached_or_offline(self) -> bool {
        matches!(self, Self::SessionStale | Self::PacketLocalOnly)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ProviderScopeLimited => {
                "provider scope is limited, so Aureline cannot write everything here and the row stays read-or-limited-write"
            }
            Self::SessionStale => {
                "the session is stale, so this reflects a cached read and is not provider-committed state"
            }
            Self::MappingPolicyLocked => {
                "the mapping is policy-locked, so the default destination cannot change on this surface"
            }
            Self::PacketLocalOnly => {
                "the packet remains local-only, so queued work is captured locally and is not provider-committed yet"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5ProviderConsumerRecoveryAction {
        match self {
            Self::ProviderScopeLimited => M5ProviderConsumerRecoveryAction::ReauthorizeForFullScope,
            Self::SessionStale => M5ProviderConsumerRecoveryAction::RefreshStaleSession,
            Self::MappingPolicyLocked => {
                M5ProviderConsumerRecoveryAction::RequestMappingPolicyChangeOrUseLocal
            }
            Self::PacketLocalOnly => {
                M5ProviderConsumerRecoveryAction::PublishQueuedPacketWhenOnline
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable
/// from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerRecoveryAction {
    /// Reauthorize / sign in for full write scope before treating the row as writable.
    ReauthorizeForFullScope,
    /// Refresh the stale session before treating the read as live provider state.
    RefreshStaleSession,
    /// Request a mapping-policy change, or keep the local default, before re-pointing the
    /// destination.
    RequestMappingPolicyChangeOrUseLocal,
    /// Publish the queued packet once online before treating it as provider-committed.
    PublishQueuedPacketWhenOnline,
}

impl M5ProviderConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReauthorizeForFullScope,
        Self::RefreshStaleSession,
        Self::RequestMappingPolicyChangeOrUseLocal,
        Self::PublishQueuedPacketWhenOnline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReauthorizeForFullScope => "reauthorize_for_full_scope",
            Self::RefreshStaleSession => "refresh_stale_session",
            Self::RequestMappingPolicyChangeOrUseLocal => {
                "request_mapping_policy_change_or_use_local"
            }
            Self::PublishQueuedPacketWhenOnline => "publish_queued_packet_when_online",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the authoritative
/// provider-settings surface (limited scope, a stale session, a policy-locked mapping, or a
/// local-only packet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerExportCaveat {
    /// Provider scope is limited, so the row is read-only or limited-write.
    ScopeLimitedReadOnly,
    /// The session is stale, so this is a cached read, not provider-committed state.
    SessionStaleCachedRead,
    /// The mapping is policy-locked, so no re-pointing or publish lands here.
    MappingPolicyLockedNoPublish,
    /// The packet is local-only, so queued work is not committed to the provider yet.
    PacketLocalOnlyNotCommitted,
}

impl M5ProviderConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ScopeLimitedReadOnly,
        Self::SessionStaleCachedRead,
        Self::MappingPolicyLockedNoPublish,
        Self::PacketLocalOnlyNotCommitted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeLimitedReadOnly => "scope_limited_read_only",
            Self::SessionStaleCachedRead => "session_stale_cached_read",
            Self::MappingPolicyLockedNoPublish => "mapping_policy_locked_no_publish",
            Self::PacketLocalOnlyNotCommitted => "packet_local_only_not_committed",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is
/// preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5ProviderClaimParityState {
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
/// [`M5ProviderConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerAnatomyPart {
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

impl M5ProviderConsumerAnatomyPart {
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

/// A field the support / export packet carries so consumer parity is reconstructable from
/// the shared model. The fields in [`M5ProviderConsumerExportField::MANDATORY`] are
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerExportField {
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

impl M5ProviderConsumerExportField {
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

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay
/// preserved, the export caveats, and the recovery action, so a narrowed rendering is
/// understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5ProviderConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5ProviderConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5ProviderComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5ProviderAccountOfflineComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5ProviderComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5ProviderConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the provider component-binding resolver for one consumer/family
/// adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5ProviderComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5ProviderAccountOfflineComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so
    /// account state, destination mapping, queued-draft state, and redaction posture stay
    /// explicit.
    pub descriptor_families: Vec<M5ProviderComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5ProviderConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5ProviderConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5ProviderComponentConsumer,
    /// The component family.
    pub component_family: M5ProviderAccountOfflineComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5ProviderComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5ProviderConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5ProviderConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5ProviderClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects cached or offline-captured state (a stale session or a
    /// local-only packet). Such a binding must always be narrowed and never asserts
    /// provider-committed state.
    pub reflects_cached_or_offline_state: bool,
    /// Hard invariant: whether this binding claims provider-committed state. Only a
    /// full-parity binding may reflect a committed publish; every narrowed binding — and in
    /// particular any cached or offline-captured one — resolves this to `false`.
    pub asserts_provider_committed: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5ProviderComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_provider_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ProviderComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5ProviderComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5ProviderComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "provider component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ProviderComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that account state,
/// destination mapping, queued-draft state, and redaction posture stay explicit on every
/// surface. The claim-parity state is preserved at full parity and auto-narrowed under any
/// weakened parity-health mode, and a weakened mode always produces a self-contained banner
/// naming the exact reason and recovery action while keeping the descriptor vocabulary
/// intact. Cached or offline-captured state (a stale session or a local-only packet) always
/// narrows and never asserts provider-committed state.
pub fn resolve_provider_component_binding(
    input: &M5ProviderComponentBindingInput,
) -> Result<M5ProviderComponentResolvedBinding, M5ProviderComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5ProviderComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5ProviderComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5ProviderComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5ProviderComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5ProviderComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text
        // extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5ProviderComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_cached_or_offline_state =
        narrowing_reason.is_some_and(M5ProviderConsumerNarrowingReason::is_cached_or_offline);
    // Only a full-parity binding may reflect a committed provider publish. Every narrowed
    // binding — and every cached / offline-captured one in particular — is not committed.
    let asserts_provider_committed = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5ProviderClaimParityState::ClaimsAutoNarrowed
    } else {
        M5ProviderClaimParityState::ClaimsPreserved
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
        M5ProviderComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5ProviderComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_cached_or_offline_state,
        asserts_provider_committed,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentBindingCase {
    /// The resolver input.
    pub input: M5ProviderComponentBindingInput,
    /// The resolved truth. Must equal `resolve_provider_component_binding(&input)`.
    pub resolved: M5ProviderComponentResolvedBinding,
}

impl M5ProviderComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ProviderComponentBindingInput) -> Self {
        let resolved =
            resolve_provider_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_provider_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the
/// consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5ProviderAccountOfflineComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical
    /// schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the
    /// family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5ProviderComponentBindingCase>,
}

impl M5ProviderComponentBinding {
    /// True when the binding points at the family's canonical refs and references the
    /// canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one provider consumer bound to the canonical component
/// families, the shared descriptor vocabulary, the parity-health modes, export caveats,
/// parity states, narrowing reasons, recovery actions, export fields, and accessibility
/// routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentConsumerRow {
    /// Provider consumer.
    pub consumer: M5ProviderComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5ProviderQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 provider surface families that render / consume this projection.
    pub surface_families: Vec<M5ProviderSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5ProviderDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ProviderConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5ProviderComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5ProviderConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5ProviderConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5ProviderClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5ProviderConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5ProviderConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5ProviderConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ProviderAccessibilityRoute>,
    /// Provider subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ProviderDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5ProviderComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new provider grammar. MUST be `false`.
    pub invents_new_provider_grammar: bool,
    /// Hard invariant: this consumer never drops account, mapping, queue, or redaction truth
    /// when narrowed. MUST be `false`.
    pub drops_account_mapping_queue_or_redaction_when_narrowed: bool,
    /// Hard invariant: this consumer never shows cached or offline-captured state as
    /// provider-committed state. MUST be `false`.
    pub shows_cached_or_offline_state_as_committed: bool,
    /// Hard invariant: this consumer never inherits a stronger label from a healthier
    /// profile instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_profile: bool,
}

impl M5ProviderComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ProviderConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ProviderConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ProviderConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5ProviderConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5ProviderComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5ProviderComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5ProviderComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5ProviderAccountOfflineComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_provider_grammar
            && !self.drops_account_mapping_queue_or_redaction_when_narrowed
            && !self.shows_cached_or_offline_state_as_committed
            && !self.inherits_stronger_label_from_healthier_profile
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentConsumerVocabularySet {
    /// Provider-consumer tokens.
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

impl M5ProviderComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5ProviderComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5ProviderAccountOfflineComponentFamily::ALL, |v| {
                v.as_str()
            }),
            descriptors: tokens(&M5ProviderComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5ProviderConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5ProviderConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5ProviderConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5ProviderConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5ProviderClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ProviderConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ProviderConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ProviderAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5ProviderComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new provider grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Account state, destination mapping, queued-draft state, and redaction posture stay
    /// explicit everywhere.
    pub account_mapping_queue_redaction_explicit_on_every_surface: bool,
    /// Limited scope, stale sessions, policy-locked mappings, and local-only packets
    /// auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// Cached or offline-captured state never masquerades as provider-committed state.
    pub cached_or_offline_state_never_shown_as_committed: bool,
    /// The support / export desk presents the same account and redaction truth shown
    /// in-product.
    pub support_export_presents_same_account_and_redaction_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentConsumerProjection {
    /// Work-item detail, status-transition review, issue intake, Help / docs, the export
    /// desk, and browser handoff all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The account-state descriptor reads a single canonical source.
    pub account_state_reads_single_source: bool,
    /// The destination-mapping descriptor reads a single canonical source.
    pub destination_mapping_reads_single_source: bool,
    /// The queued-draft-state descriptor reads a single canonical source.
    pub queued_draft_state_reads_single_source: bool,
    /// The redaction-posture descriptor reads a single canonical source.
    pub redaction_posture_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting provider-account consumer audit.
    pub provider_account_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProviderComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ProviderComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 provider-account / offline-capture component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderComponentConsumerPacket {
    /// Record kind; must equal [`M5_PROVIDER_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ProviderComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderComponentConsumerPacket {
    /// Builds an M5 provider-account / offline-capture component-consumer packet from
    /// stable-lane input.
    pub fn new(input: M5ProviderComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_VERSION,
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

    /// Validates the M5 provider-account / offline-capture component-consumer invariants.
    pub fn validate(&self) -> Vec<M5ProviderComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5ProviderComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5ProviderComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_commit_honesty(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 provider component consumer packet serializes"),
        ) {
            violations.push(M5ProviderComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 provider component consumer packet serializes")
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
        out.push_str("# M5 Provider-Account / Offline-Capture Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Provider consumers: {} ({} stable)\n",
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
        out.push_str("\n## Provider consumers\n\n");
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

/// Errors emitted when reading the checked-in M5 provider-account / offline-capture
/// component-consumer export.
#[derive(Debug)]
pub enum M5ProviderComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderComponentConsumerViolation>),
}

impl fmt::Display for M5ProviderComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 provider component consumer export parse failed: {error}"
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
                    "m5 provider component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5ProviderComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderComponentConsumerViolation {
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
    /// A required provider consumer is missing from the matrix.
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
    /// A required component family is never adopted, or is adopted by only one consumer
    /// (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no banner.
    ScopePreservedUnproven,
    /// No worked binding proves that cached or offline-captured state narrows and never
    /// asserts provider-committed state, or a binding does so incorrectly.
    CommitHonestyUnproven,
    /// The support / export desk consumer does not reference the canonical component schema.
    SupportExportReferenceMissing,
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

impl M5ProviderComponentConsumerViolation {
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
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 provider-account / offline-capture
/// component-consumer export.
pub fn current_stable_m5_provider_component_consumer_export(
) -> Result<M5ProviderComponentConsumerPacket, M5ProviderComponentConsumerArtifactError> {
    let packet: M5ProviderComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-account-offline-capture-component-consumer-proof/support_export.json"
    )))
    .map_err(M5ProviderComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_PROVIDER_COMPONENT_CONSUMER_DOC_REF,
        M5_PROVIDER_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_PROVIDER_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProviderComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProviderComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let present: BTreeSet<M5ProviderComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5ProviderComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderComponentConsumerViolation::RequiredConsumerMissing);
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
            violations.push(M5ProviderComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ProviderComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5ProviderComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ProviderComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ProviderComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProviderComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProviderComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5ProviderComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5ProviderComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5ProviderComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5ProviderComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ProviderComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ProviderComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one
/// provider-settings page plus a few isolated export objects.
fn validate_family_reuse(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    for family in M5ProviderAccountOfflineComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5ProviderComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved
/// descriptors — the acceptance-criterion example that a consumer which cannot preserve
/// parity is visibly narrowed rather than inheriting stronger labels from healthier profiles.
fn validate_narrowing_disclosure(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
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
        violations.push(M5ProviderComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with
/// preserved parity and no banner — the acceptance-criterion example that full-parity
/// consumers keep the descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5ProviderClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5ProviderComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects cached or offline-captured state must be narrowed and
/// must not assert provider-committed state, and at least one such binding must be present —
/// the acceptance-criterion that cached or offline-captured state no longer masquerades as
/// provider-committed state on any claimed consumer.
fn validate_commit_honesty(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_cached_or_offline_state {
            // A cached / offline binding that claims commit, or fails to narrow, breaks AC2.
            if resolved.asserts_provider_committed
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5ProviderClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5ProviderComponentConsumerViolation::CommitHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5ProviderComponentConsumerViolation::CommitHonestyUnproven);
    }
}

/// The support / export desk consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that a support / export lane can never drift
/// from the product truth.
fn validate_support_export_reference(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5ProviderComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5ProviderComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.account_mapping_queue_redaction_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.cached_or_offline_state_never_shown_as_committed,
        review.support_export_presents_same_account_and_redaction_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ProviderComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.account_state_reads_single_source,
        projection.destination_mapping_reads_single_source,
        projection.queued_draft_state_reads_single_source,
        projection.redaction_posture_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ProviderComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProviderComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProviderComponentConsumerPacket,
    violations: &mut Vec<M5ProviderComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .provider_account_consumer_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProviderComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5ProviderComponentConsumerPacket,
) -> impl Iterator<Item = &M5ProviderComponentBindingCase> {
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
