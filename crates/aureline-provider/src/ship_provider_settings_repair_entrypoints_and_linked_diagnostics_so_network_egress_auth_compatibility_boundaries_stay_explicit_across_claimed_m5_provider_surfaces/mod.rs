//! One reusable M5 provider primitive — the provider-settings repair-entrypoint row — so a user
//! can tell, from the row alone, *which* boundary actually failed (network/egress, auth,
//! provider compatibility, a broken mapping, or a policy block), *where* the repair entrypoint
//! is, *what* diagnostics the row links to, and — above all — that repairing the boundary never
//! loses queued work, breaks cached-read continuity, drops the reviewed export path, or forces a
//! blind re-entry of credentials.
//!
//! Aureline's frozen provider-account / mapping / offline-capture component matrix
//! ([`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`])
//! names the reusable provider-account, mapping, sync, offline-capture, and privacy/redaction
//! rows and freezes their controlled vocabulary — the account connection states, the surface
//! families, the deployment lines, the consumer surfaces, the accessibility routes, the
//! qualification classes, and the downgrade triggers. The three implement/ship lanes that
//! narrow that matrix
//! ([`crate::implement_provider_account_rows_with_signed_in_limited_scope_stale_session_offline_cached_policy_blocked_truth_and_sign_in_retry_remove_parity_across_claimed_m5_provider_surfaces`],
//! [`crate::ship_project_or_board_mapping_rows_and_sync_behavior_rows_with_inherited_local_policy_scope_read_only_comment_transition_sync_modes_and_change_reset_parity_across_claimed_m5_provider_lanes`],
//! and
//! [`crate::implement_offline_capture_rows_and_privacy_redaction_rows_with_packet_destination_queued_draft_count_export_clear_actions_and_metadata_safe_boundary_truth_across_claimed_m5_provider_workflows`])
//! resolve *what state* each row is in. This module closes the gap the acceptance criteria name:
//! it *ships* the repair entrypoint and the linked diagnostics so provider settings stop feeling
//! like an isolated sidebar divorced from the real diagnostics and export surfaces that explain
//! the failure, and so stale sessions and broken mappings stop collapsing into retry-login
//! folklore.
//!
//! The module has one resolver, [`resolve_provider_repair_entrypoint`], which takes one provider
//! row's failed boundary, its account connection state, whether queued drafts and a cached read
//! exist, and whether a policy-escalation route is available, and produces one
//! [`M5ResolvedProviderRepairEntrypoint`] carrying the derived repair posture (one per boundary
//! class), the concrete repair entrypoint the row links to, the linked diagnostics (network/egress,
//! auth, support-bundle, provider-compatibility, and export/redaction), the continuity
//! guarantees the repair preserves, and the bounded reveal / open-entrypoint / open-diagnostics /
//! export-evidence / request-escalation actions. It never loses queued work, never breaks
//! cached-read continuity, never drops the reviewed export path, and never forces a blind
//! credential re-entry.
//!
//! A single parity matrix — [`M5ProviderRepairEntrypointPacket`] — binds one row per claimed M5
//! provider-settings consumer (the provider-account row, the project/board mapping row, the
//! sync-behavior row, the privacy/redaction row, and the provider status bar) to the shared
//! repair-row anatomy, the same boundary classes, connection states, repair entrypoints, linked
//! diagnostics, continuity guarantees, bounded actions, export fields, and non-visual
//! accessibility routes, so the repair vocabulary stays identical across desktop, headless/export,
//! and support consumers.
//!
//! The account connection state ([`M5AccountConnectionState`]), surface family
//! ([`M5ProviderSurfaceFamily`]), deployment line ([`M5ProviderDeploymentLine`]), consumer
//! surface ([`M5ProviderConsumerSurface`]), accessibility route
//! ([`M5ProviderAccessibilityRoute`]), qualification class ([`M5ProviderQualificationClass`]),
//! and downgrade trigger ([`M5ProviderDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! repair row itself: the boundary class, the derived repair posture, the repair-entrypoint
//! class, the linked-diagnostic class, the continuity guarantee, the bounded actions, the anatomy
//! parts, and the export fields. No M5 provider surface invents a second repair grammar.
//!
//! Every boundary label, repair-target label, diagnostic identity, and repair identity is carried
//! only as an opaque, export-safe representation; credentials, private endpoints, and raw bodies
//! stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provider_repair_entrypoint_packet,
    seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed,
    seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed,
    M5_PROVIDER_REPAIR_ENTRYPOINT_PACKET_ID,
};

// The account connection state, surface family, deployment line, consumer surface, accessibility
// route, qualification class, and downgrade triggers are frozen once, in the provider-account /
// offline-capture component matrix. This primitive reuses them verbatim so it never invents a
// parallel connection or governance vocabulary.
pub use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5AccountConnectionState, M5ProviderAccessibilityRoute, M5ProviderConsumerSurface,
    M5ProviderDeploymentLine, M5ProviderDowngradeTrigger, M5ProviderQualificationClass,
    M5ProviderSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderRepairEntrypointPacket`].
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_RECORD_KIND: &str =
    "ship_m5_provider_settings_repair_entrypoints_and_linked_diagnostics_so_network_egress_auth_compatibility_boundaries_stay_explicit_across_claimed_m5_provider_surfaces";

/// Schema version for M5 provider-settings repair-entrypoint-row records.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the repair-entrypoint-row boundary schema.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-settings-repair-entrypoint-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_DOC_REF: &str =
    "docs/providers/m5_provider_settings_repair_entrypoint_and_linked_diagnostics.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix this
/// primitive builds its repair entrypoints on top of.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json";

/// Repo-relative path of the network-egress remediation contract the network-egress repair
/// entrypoint links to.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_NETWORK_REMEDIATION_REF: &str =
    "schemas/network/network_remediation_card.schema.json";

/// Repo-relative path of the reauthentication-requirement contract the auth repair entrypoint
/// links to, so a stale session is repaired through a reviewed reauth flow rather than a blind
/// credential re-entry.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_REAUTH_REQUIREMENT_REF: &str =
    "schemas/auth/reauth_requirement.schema.json";

/// Repo-relative path of the provider sync-health contract the compatibility repair entrypoint
/// links to.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_PROVIDER_COMPAT_REF: &str =
    "schemas/providers/provider_sync_health_view.schema.json";

/// Repo-relative path of the support-bundle contract every repair row links to.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_SUPPORT_BUNDLE_REF: &str =
    "schemas/support/support_bundle.schema.json";

/// Repo-relative path of the export-redaction-profile contract that keeps the reviewed export
/// path intact while a boundary is repaired.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_EXPORT_REDACTION_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the offline-handoff-packet contract that keeps queued drafts intact
/// while a boundary is repaired.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_OFFLINE_HANDOFF_REF: &str =
    "schemas/providers/offline_handoff_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-settings-repair-entrypoint-row";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-settings-repair-entrypoint-row-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_CSV_REF: &str =
    "artifacts/release/m5-provider-settings-repair-entrypoint-row-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_REPORT_REF: &str =
    "artifacts/design/m5-provider-settings-repair-entrypoint-row.md";

/// One claimed M5 provider-settings consumer that renders the shared repair-entrypoint row. These
/// are the rows the acceptance criteria name — the provider-account row, the project/board
/// mapping row, the sync-behavior row, the privacy/redaction row, and the provider status bar —
/// so the same repair grammar works across every claimed provider surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRepairConsumerSurface {
    /// The provider-account row.
    ProviderAccountRow,
    /// The project / board mapping row.
    ProjectBoardMappingRow,
    /// The sync-behavior row.
    SyncBehaviorRow,
    /// The privacy / redaction row.
    PrivacyRedactionRow,
    /// The provider status-bar surface.
    ProviderStatusBar,
}

impl M5ProviderRepairConsumerSurface {
    /// Every claimed provider-settings consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAccountRow,
        Self::ProjectBoardMappingRow,
        Self::SyncBehaviorRow,
        Self::PrivacyRedactionRow,
        Self::ProviderStatusBar,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAccountRow => "provider_account_row",
            Self::ProjectBoardMappingRow => "project_board_mapping_row",
            Self::SyncBehaviorRow => "sync_behavior_row",
            Self::PrivacyRedactionRow => "privacy_redaction_row",
            Self::ProviderStatusBar => "provider_status_bar",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProviderAccountRow => "Provider-Account Row",
            Self::ProjectBoardMappingRow => "Project / Board Mapping Row",
            Self::SyncBehaviorRow => "Sync-Behavior Row",
            Self::PrivacyRedactionRow => "Privacy / Redaction Row",
            Self::ProviderStatusBar => "Provider Status Bar",
        }
    }
}

// ---- repair boundary vocabulary ------------------------------------------

/// The boundary that actually failed and needs repair, so a repair row names the real boundary
/// instead of collapsing every failure into "retry login". Derived from the frozen account
/// connection state plus the mapping/compatibility context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderBoundaryClass {
    /// Network / egress reachability is blocked.
    NetworkEgressBlocked,
    /// The provider session is stale and needs reauthentication.
    AuthStaleSession,
    /// The provider session is valid but the scope is too narrow to write.
    AuthScopeLimited,
    /// The project / board mapping is broken and no longer resolves a target.
    MappingBroken,
    /// The provider version or capability is incompatible with the current build.
    ProviderIncompatible,
    /// The boundary is blocked by policy and can only be repaired by a reviewed escalation.
    PolicyBlocked,
}

impl M5ProviderBoundaryClass {
    /// Every boundary class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NetworkEgressBlocked,
        Self::AuthStaleSession,
        Self::AuthScopeLimited,
        Self::MappingBroken,
        Self::ProviderIncompatible,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkEgressBlocked => "network_egress_blocked",
            Self::AuthStaleSession => "auth_stale_session",
            Self::AuthScopeLimited => "auth_scope_limited",
            Self::MappingBroken => "mapping_broken",
            Self::ProviderIncompatible => "provider_incompatible",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// True when this boundary can only be repaired by a reviewed policy escalation.
    pub const fn is_policy_blocked(self) -> bool {
        matches!(self, Self::PolicyBlocked)
    }
}

/// The derived posture of a repair row — the resolver's verdict about how the boundary is
/// repaired. Derived one-to-one from the boundary class so a stale session never reads the same
/// as a broken mapping or a policy block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRepairPosture {
    /// Repair the network / egress path.
    NetworkEgressRepairRow,
    /// Reauthenticate the stale session.
    ReauthSessionRow,
    /// Widen the too-narrow scope.
    WidenScopeRow,
    /// Remap the broken target.
    RemapTargetRow,
    /// Review the provider compatibility.
    CompatibilityReviewRow,
    /// The boundary is policy-blocked pending a reviewed escalation.
    PolicyBlockedRow,
}

impl M5ProviderRepairPosture {
    /// Every repair posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NetworkEgressRepairRow,
        Self::ReauthSessionRow,
        Self::WidenScopeRow,
        Self::RemapTargetRow,
        Self::CompatibilityReviewRow,
        Self::PolicyBlockedRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkEgressRepairRow => "network_egress_repair_row",
            Self::ReauthSessionRow => "reauth_session_row",
            Self::WidenScopeRow => "widen_scope_row",
            Self::RemapTargetRow => "remap_target_row",
            Self::CompatibilityReviewRow => "compatibility_review_row",
            Self::PolicyBlockedRow => "policy_blocked_row",
        }
    }
}

/// The concrete repair entrypoint a row links to, so provider settings stop feeling divorced from
/// the surface that actually repairs the boundary. Derived one-to-one from the boundary class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairEntrypointClass {
    /// Open the network / egress diagnostics and remediation.
    OpenNetworkEgressDiagnostics,
    /// Open the reviewed reauthentication handoff (browser / device-code), never a blind
    /// credential prompt.
    OpenReauthHandoff,
    /// Open the provider-scope review to widen scope.
    OpenScopeReview,
    /// Open the mapping repair to remap the broken target.
    OpenMappingRepair,
    /// Open the provider compatibility report.
    OpenCompatibilityReport,
    /// Open the policy review to request a reviewed escalation.
    OpenPolicyReview,
}

impl M5RepairEntrypointClass {
    /// Every repair entrypoint class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenNetworkEgressDiagnostics,
        Self::OpenReauthHandoff,
        Self::OpenScopeReview,
        Self::OpenMappingRepair,
        Self::OpenCompatibilityReport,
        Self::OpenPolicyReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenNetworkEgressDiagnostics => "open_network_egress_diagnostics",
            Self::OpenReauthHandoff => "open_reauth_handoff",
            Self::OpenScopeReview => "open_scope_review",
            Self::OpenMappingRepair => "open_mapping_repair",
            Self::OpenCompatibilityReport => "open_compatibility_report",
            Self::OpenPolicyReview => "open_policy_review",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenNetworkEgressDiagnostics => "Open Network / Egress Diagnostics",
            Self::OpenReauthHandoff => "Open Reauth Handoff",
            Self::OpenScopeReview => "Open Scope Review",
            Self::OpenMappingRepair => "Open Mapping Repair",
            Self::OpenCompatibilityReport => "Open Compatibility Report",
            Self::OpenPolicyReview => "Open Policy Review",
        }
    }

    /// Whether reaching this entrypoint forces a blind re-entry of credentials. No repair
    /// entrypoint ever does; the auth repair goes through a reviewed reauth handoff. ALWAYS
    /// `false`.
    pub const fn requires_blind_credential_reentry(self) -> bool {
        false
    }
}

/// One diagnostic surface a repair row links to, so provider settings are never an isolated
/// sidebar divorced from the diagnostics and export surfaces that explain the failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LinkedDiagnosticClass {
    /// The network / egress diagnostic.
    NetworkEgressDiagnostic,
    /// The auth / session diagnostic.
    AuthSessionDiagnostic,
    /// The support-bundle diagnostic.
    SupportBundleDiagnostic,
    /// The provider-compatibility diagnostic.
    ProviderCompatibilityDiagnostic,
    /// The export / redaction diagnostic.
    ExportRedactionDiagnostic,
}

impl M5LinkedDiagnosticClass {
    /// Every linked-diagnostic class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NetworkEgressDiagnostic,
        Self::AuthSessionDiagnostic,
        Self::SupportBundleDiagnostic,
        Self::ProviderCompatibilityDiagnostic,
        Self::ExportRedactionDiagnostic,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkEgressDiagnostic => "network_egress_diagnostic",
            Self::AuthSessionDiagnostic => "auth_session_diagnostic",
            Self::SupportBundleDiagnostic => "support_bundle_diagnostic",
            Self::ProviderCompatibilityDiagnostic => "provider_compatibility_diagnostic",
            Self::ExportRedactionDiagnostic => "export_redaction_diagnostic",
        }
    }
}

/// One continuity guarantee a repair preserves, so a user can repair the boundary without losing
/// queued work, breaking cached-read continuity, dropping the reviewed export path, or re-entering
/// credentials blindly. Every resolved repair carries all four.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairContinuityGuarantee {
    /// Queued local drafts are preserved through the repair.
    PreservesQueuedDrafts,
    /// Cached-read continuity is preserved through the repair.
    PreservesCachedReadContinuity,
    /// The reviewed export path is preserved through the repair.
    PreservesReviewedExportPath,
    /// The repair never forces a blind re-entry of credentials.
    NoBlindCredentialReentry,
}

impl M5RepairContinuityGuarantee {
    /// Every continuity guarantee, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PreservesQueuedDrafts,
        Self::PreservesCachedReadContinuity,
        Self::PreservesReviewedExportPath,
        Self::NoBlindCredentialReentry,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservesQueuedDrafts => "preserves_queued_drafts",
            Self::PreservesCachedReadContinuity => "preserves_cached_read_continuity",
            Self::PreservesReviewedExportPath => "preserves_reviewed_export_path",
            Self::NoBlindCredentialReentry => "no_blind_credential_reentry",
        }
    }
}

/// One bounded action a repair row offers, so a row never hides its reveal / open-entrypoint /
/// open-diagnostics / export-evidence / request-escalation affordances and a user can reach the
/// real repair or escalate without leaving the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRepairRowAction {
    /// Reveal the boundary class, entrypoint, linked diagnostics, and preserved work.
    RevealBoundary,
    /// Open the concrete repair entrypoint.
    OpenRepairEntrypoint,
    /// Open the linked diagnostics.
    OpenLinkedDiagnostics,
    /// Export repair evidence through the reviewed export path.
    ExportRepairEvidence,
    /// Request a reviewed policy escalation.
    RequestPolicyEscalation,
}

impl M5ProviderRepairRowAction {
    /// Every repair-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealBoundary,
        Self::OpenRepairEntrypoint,
        Self::OpenLinkedDiagnostics,
        Self::ExportRepairEvidence,
        Self::RequestPolicyEscalation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealBoundary => "reveal_boundary",
            Self::OpenRepairEntrypoint => "open_repair_entrypoint",
            Self::OpenLinkedDiagnostics => "open_linked_diagnostics",
            Self::ExportRepairEvidence => "export_repair_evidence",
            Self::RequestPolicyEscalation => "request_policy_escalation",
        }
    }
}

/// Controlled repair-row anatomy part the shared row surfaces. The parts in
/// [`M5ProviderRepairRowAnatomyPart::MANDATORY`] are required on every row so the boundary class,
/// repair entrypoint, linked diagnostics, preserved work, and repair action cue are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRepairRowAnatomyPart {
    /// The boundary-class cue.
    BoundaryClassCue,
    /// The repair-entrypoint cue.
    RepairEntrypointCue,
    /// The linked-diagnostics cue.
    LinkedDiagnosticsCue,
    /// The preserved-work cue.
    PreservedWorkCue,
    /// The credential-re-entry cue.
    CredentialReentryCue,
    /// The account-state cue.
    AccountStateCue,
    /// The repair-action cue.
    RepairActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5ProviderRepairRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BoundaryClassCue,
        Self::RepairEntrypointCue,
        Self::LinkedDiagnosticsCue,
        Self::PreservedWorkCue,
        Self::CredentialReentryCue,
        Self::AccountStateCue,
        Self::RepairActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every repair row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::BoundaryClassCue,
        Self::RepairEntrypointCue,
        Self::LinkedDiagnosticsCue,
        Self::PreservedWorkCue,
        Self::RepairActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryClassCue => "boundary_class_cue",
            Self::RepairEntrypointCue => "repair_entrypoint_cue",
            Self::LinkedDiagnosticsCue => "linked_diagnostics_cue",
            Self::PreservedWorkCue => "preserved_work_cue",
            Self::CredentialReentryCue => "credential_reentry_cue",
            Self::AccountStateCue => "account_state_cue",
            Self::RepairActionCue => "repair_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the repair-row export carries so repair-row truth is reconstructable. The fields in
/// [`M5ProviderRepairRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRepairRowExportField {
    /// The boundary class.
    BoundaryClass,
    /// The account connection state.
    AccountConnectionState,
    /// The repair entrypoint.
    RepairEntrypoint,
    /// The linked diagnostics.
    LinkedDiagnostics,
    /// The preserved continuity guarantees.
    PreservedContinuity,
    /// The derived repair-row posture.
    RowPosture,
    /// Whether the repair requires a blind credential re-entry.
    RequiresCredentialReentry,
    /// The bounded available actions.
    AvailableActions,
}

impl M5ProviderRepairRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BoundaryClass,
        Self::AccountConnectionState,
        Self::RepairEntrypoint,
        Self::LinkedDiagnostics,
        Self::PreservedContinuity,
        Self::RowPosture,
        Self::RequiresCredentialReentry,
        Self::AvailableActions,
    ];

    /// The export fields every repair row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::BoundaryClass,
        Self::RepairEntrypoint,
        Self::LinkedDiagnostics,
        Self::PreservedContinuity,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryClass => "boundary_class",
            Self::AccountConnectionState => "account_connection_state",
            Self::RepairEntrypoint => "repair_entrypoint",
            Self::LinkedDiagnostics => "linked_diagnostics",
            Self::PreservedContinuity => "preserved_continuity",
            Self::RowPosture => "row_posture",
            Self::RequiresCredentialReentry => "requires_credential_reentry",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- repair-entrypoint resolver ------------------------------------------

/// The full input to the repair-entrypoint resolver for one provider-settings row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairEntrypointResolutionInput {
    /// The boundary that failed and needs repair.
    pub boundary_class: M5ProviderBoundaryClass,
    /// The account connection state behind the row.
    pub connection_state: M5AccountConnectionState,
    /// True when local drafts remain queued behind this row.
    pub has_queued_drafts: bool,
    /// True when a cached read remains available behind this row.
    pub has_cached_read: bool,
    /// True when a reviewed policy-escalation route is available.
    pub policy_escalation_available: bool,
    /// The opaque boundary label (must be non-empty).
    pub boundary_label: String,
    /// The opaque repair-target label (must be non-empty).
    pub repair_target_label: String,
    /// The opaque stable repair identity (must be non-empty).
    pub repair_ref: String,
}

/// The resolved repair-entrypoint truth for one provider-settings row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedProviderRepairEntrypoint {
    /// The boundary that failed and needs repair.
    pub boundary_class: M5ProviderBoundaryClass,
    /// The account connection state behind the row.
    pub connection_state: M5AccountConnectionState,
    /// True when local drafts remain queued behind this row.
    pub has_queued_drafts: bool,
    /// True when a cached read remains available behind this row.
    pub has_cached_read: bool,
    /// The opaque boundary label, preserved exactly from the input.
    pub boundary_label: String,
    /// The opaque repair-target label, preserved exactly from the input.
    pub repair_target_label: String,
    /// The opaque stable repair identity, preserved exactly from the input.
    pub repair_ref: String,
    /// The derived repair-row posture.
    pub row_posture: M5ProviderRepairPosture,
    /// The concrete repair entrypoint this row links to.
    pub repair_entrypoint: M5RepairEntrypointClass,
    /// The diagnostics this row links to.
    pub linked_diagnostics: Vec<M5LinkedDiagnosticClass>,
    /// The continuity guarantees the repair preserves.
    pub continuity_guarantees: Vec<M5RepairContinuityGuarantee>,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5ProviderRepairRowAction>,
    /// True when the row links to at least one diagnostic. ALWAYS `true`.
    pub links_to_diagnostics: bool,
    /// The repair preserves queued work. ALWAYS `true`.
    pub preserves_queued_work: bool,
    /// The repair preserves cached-read continuity. ALWAYS `true`.
    pub preserves_cached_read_continuity: bool,
    /// The repair preserves the reviewed export path. ALWAYS `true`.
    pub preserves_reviewed_export_path: bool,
    /// The repair never forces a blind credential re-entry. ALWAYS `false`.
    pub requires_blind_credential_reentry: bool,
    /// The row is never an isolated sidebar divorced from diagnostics. ALWAYS `false`.
    pub isolated_from_diagnostics: bool,
}

/// Errors returned by [`resolve_provider_repair_entrypoint`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ProviderRepairEntrypointResolutionError {
    /// The boundary label was empty.
    EmptyBoundaryLabel,
    /// The repair-target label was empty.
    EmptyRepairTargetLabel,
    /// The repair ref was empty.
    EmptyRepairRef,
    /// A policy-blocked boundary carried no reviewed escalation route.
    PolicyBlockedWithoutEscalationRoute,
    /// A repair descriptor carried forbidden material.
    ForbiddenRepairMaterial,
}

impl M5ProviderRepairEntrypointResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBoundaryLabel => "empty_boundary_label",
            Self::EmptyRepairTargetLabel => "empty_repair_target_label",
            Self::EmptyRepairRef => "empty_repair_ref",
            Self::PolicyBlockedWithoutEscalationRoute => "policy_blocked_without_escalation_route",
            Self::ForbiddenRepairMaterial => "forbidden_repair_material",
        }
    }
}

impl fmt::Display for M5ProviderRepairEntrypointResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "provider repair entrypoint resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ProviderRepairEntrypointResolutionError {}

/// Resolves one repair-entrypoint row from its failed boundary.
///
/// The derived row posture and repair entrypoint are taken one-to-one from the boundary class so a
/// stale session, a broken mapping, and a policy block never collapse into one "retry login"; the
/// linked diagnostics tie the row to the real surfaces that explain the failure. A policy-blocked
/// boundary must carry a reviewed escalation route. Every repair preserves queued work,
/// cached-read continuity, and the reviewed export path, and never forces a blind credential
/// re-entry.
pub fn resolve_provider_repair_entrypoint(
    input: &M5ProviderRepairEntrypointResolutionInput,
) -> Result<M5ResolvedProviderRepairEntrypoint, M5ProviderRepairEntrypointResolutionError> {
    if input.boundary_label.trim().is_empty() {
        return Err(M5ProviderRepairEntrypointResolutionError::EmptyBoundaryLabel);
    }
    if input.repair_target_label.trim().is_empty() {
        return Err(M5ProviderRepairEntrypointResolutionError::EmptyRepairTargetLabel);
    }
    if input.repair_ref.trim().is_empty() {
        return Err(M5ProviderRepairEntrypointResolutionError::EmptyRepairRef);
    }
    if input.boundary_class.is_policy_blocked() && !input.policy_escalation_available {
        return Err(M5ProviderRepairEntrypointResolutionError::PolicyBlockedWithoutEscalationRoute);
    }
    if value_repr_is_forbidden(&input.boundary_label)
        || value_repr_is_forbidden(&input.repair_target_label)
        || value_repr_is_forbidden(&input.repair_ref)
    {
        return Err(M5ProviderRepairEntrypointResolutionError::ForbiddenRepairMaterial);
    }

    let row_posture = derive_repair_posture(input.boundary_class);
    let repair_entrypoint = derive_repair_entrypoint(input.boundary_class);
    let linked_diagnostics = derive_linked_diagnostics(input.boundary_class);
    let available_actions =
        derive_repair_actions(input.boundary_class, input.policy_escalation_available);

    Ok(M5ResolvedProviderRepairEntrypoint {
        boundary_class: input.boundary_class,
        connection_state: input.connection_state,
        has_queued_drafts: input.has_queued_drafts,
        has_cached_read: input.has_cached_read,
        boundary_label: input.boundary_label.clone(),
        repair_target_label: input.repair_target_label.clone(),
        repair_ref: input.repair_ref.clone(),
        row_posture,
        repair_entrypoint,
        linked_diagnostics,
        continuity_guarantees: M5RepairContinuityGuarantee::ALL.to_vec(),
        available_actions,
        links_to_diagnostics: true,
        // The acceptance criterion: a boundary is repaired without losing queued work, breaking
        // cached-read continuity, dropping the reviewed export path, or re-entering credentials
        // blindly — and provider settings are never an isolated sidebar.
        preserves_queued_work: true,
        preserves_cached_read_continuity: true,
        preserves_reviewed_export_path: true,
        requires_blind_credential_reentry: repair_entrypoint.requires_blind_credential_reentry(),
        isolated_from_diagnostics: false,
    })
}

/// Derives the repair-row posture one-to-one from the boundary class.
fn derive_repair_posture(boundary: M5ProviderBoundaryClass) -> M5ProviderRepairPosture {
    use M5ProviderBoundaryClass as Boundary;
    use M5ProviderRepairPosture as Posture;
    match boundary {
        Boundary::NetworkEgressBlocked => Posture::NetworkEgressRepairRow,
        Boundary::AuthStaleSession => Posture::ReauthSessionRow,
        Boundary::AuthScopeLimited => Posture::WidenScopeRow,
        Boundary::MappingBroken => Posture::RemapTargetRow,
        Boundary::ProviderIncompatible => Posture::CompatibilityReviewRow,
        Boundary::PolicyBlocked => Posture::PolicyBlockedRow,
    }
}

/// Derives the concrete repair entrypoint one-to-one from the boundary class.
fn derive_repair_entrypoint(boundary: M5ProviderBoundaryClass) -> M5RepairEntrypointClass {
    use M5ProviderBoundaryClass as Boundary;
    use M5RepairEntrypointClass as Entrypoint;
    match boundary {
        Boundary::NetworkEgressBlocked => Entrypoint::OpenNetworkEgressDiagnostics,
        Boundary::AuthStaleSession => Entrypoint::OpenReauthHandoff,
        Boundary::AuthScopeLimited => Entrypoint::OpenScopeReview,
        Boundary::MappingBroken => Entrypoint::OpenMappingRepair,
        Boundary::ProviderIncompatible => Entrypoint::OpenCompatibilityReport,
        Boundary::PolicyBlocked => Entrypoint::OpenPolicyReview,
    }
}

/// Derives the diagnostics a repair row links to from the boundary class. Every row links to the
/// support-bundle and export/redaction diagnostics plus the boundary-specific diagnostic, so no
/// repair row is ever divorced from the diagnostics and export surfaces that explain the failure.
fn derive_linked_diagnostics(boundary: M5ProviderBoundaryClass) -> Vec<M5LinkedDiagnosticClass> {
    use M5LinkedDiagnosticClass as Diagnostic;
    use M5ProviderBoundaryClass as Boundary;
    let boundary_specific = match boundary {
        Boundary::NetworkEgressBlocked => Diagnostic::NetworkEgressDiagnostic,
        Boundary::AuthStaleSession | Boundary::AuthScopeLimited | Boundary::PolicyBlocked => {
            Diagnostic::AuthSessionDiagnostic
        }
        Boundary::MappingBroken | Boundary::ProviderIncompatible => {
            Diagnostic::ProviderCompatibilityDiagnostic
        }
    };
    vec![
        boundary_specific,
        Diagnostic::SupportBundleDiagnostic,
        Diagnostic::ExportRedactionDiagnostic,
    ]
}

/// Derives the bounded repair action set from the boundary class and escalation availability.
///
/// Reveal, open-linked-diagnostics, and export-repair-evidence are always offered. A self-serve
/// open-repair-entrypoint is offered for every non-policy-blocked boundary; a policy-blocked
/// boundary offers a reviewed policy escalation instead.
fn derive_repair_actions(
    boundary: M5ProviderBoundaryClass,
    policy_escalation_available: bool,
) -> Vec<M5ProviderRepairRowAction> {
    use M5ProviderRepairRowAction as Action;
    let mut actions = vec![Action::RevealBoundary];
    if !boundary.is_policy_blocked() {
        actions.push(Action::OpenRepairEntrypoint);
    }
    actions.push(Action::OpenLinkedDiagnostics);
    actions.push(Action::ExportRepairEvidence);
    if boundary.is_policy_blocked() && policy_escalation_available {
        actions.push(Action::RequestPolicyEscalation);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked repair-entrypoint resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairEntrypointResolutionCase {
    /// The resolver input.
    pub input: M5ProviderRepairEntrypointResolutionInput,
    /// The resolved truth. Must equal `resolve_provider_repair_entrypoint(&input)`.
    pub resolved: M5ResolvedProviderRepairEntrypoint,
}

impl M5ProviderRepairEntrypointResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ProviderRepairEntrypointResolutionInput) -> Self {
        let resolved = resolve_provider_repair_entrypoint(&input)
            .expect("seed provider repair entrypoint case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_provider_repair_entrypoint(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved repair identity preserves the input identity exactly.
    pub fn preserves_repair_identity(&self) -> bool {
        self.resolved.repair_ref == self.input.repair_ref
            && self.resolved.repair_target_label == self.input.repair_target_label
            && self.resolved.boundary_label == self.input.boundary_label
    }

    /// True when the case preserves queued work, cached-read continuity, and the reviewed export
    /// path, never forces a blind credential re-entry, and is never divorced from diagnostics.
    pub fn preserves_continuity(&self) -> bool {
        self.resolved.preserves_queued_work
            && self.resolved.preserves_cached_read_continuity
            && self.resolved.preserves_reviewed_export_path
            && !self.resolved.requires_blind_credential_reentry
            && !self.resolved.isolated_from_diagnostics
            && self.resolved.links_to_diagnostics
    }
}

/// One row in the primitive matrix: one provider-settings consumer bound to the shared repair-row
/// anatomy, the boundary classes, connection states, repair postures, entrypoints, linked
/// diagnostics, continuity guarantees, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairConsumerRow {
    /// Provider-settings consumer family.
    pub consumer_surface: M5ProviderRepairConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ProviderQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 provider surface families that render / consume this row.
    pub surface_families: Vec<M5ProviderSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ProviderDeploymentLine>,
    /// Repair-row anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ProviderRepairRowAnatomyPart>,
    /// Boundary classes this consumer distinguishes.
    pub boundary_classes: Vec<M5ProviderBoundaryClass>,
    /// Account connection states this consumer distinguishes.
    pub connection_states: Vec<M5AccountConnectionState>,
    /// Repair postures this consumer distinguishes.
    pub repair_postures: Vec<M5ProviderRepairPosture>,
    /// Repair entrypoints this consumer links to.
    pub repair_entrypoints: Vec<M5RepairEntrypointClass>,
    /// Linked-diagnostic classes this consumer links to.
    pub linked_diagnostics: Vec<M5LinkedDiagnosticClass>,
    /// Continuity guarantees this consumer preserves.
    pub continuity_guarantees: Vec<M5RepairContinuityGuarantee>,
    /// Bounded repair-row actions this consumer offers.
    pub row_actions: Vec<M5ProviderRepairRowAction>,
    /// Repair-row export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5ProviderRepairRowExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ProviderAccessibilityRoute>,
    /// Provider subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ProviderDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked repair-entrypoint resolutions proving the resolver on this consumer.
    pub examples: Vec<M5ProviderRepairEntrypointResolutionCase>,
    /// Hard invariant: this consumer is never an isolated sidebar divorced from diagnostics. MUST
    /// be `false`.
    pub isolates_settings_from_diagnostics: bool,
    /// Hard invariant: this consumer never loses queued work while repairing. MUST be `false`.
    pub loses_queued_work: bool,
    /// Hard invariant: this consumer never forces a blind credential re-entry. MUST be `false`.
    pub requires_blind_credential_reentry: bool,
    /// Hard invariant: this consumer never breaks cached-read continuity. MUST be `false`.
    pub breaks_cached_read_continuity: bool,
    /// Hard invariant: this consumer never drops the reviewed export path. MUST be `false`.
    pub breaks_reviewed_export_path: bool,
}

impl M5ProviderRepairConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ProviderRepairRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ProviderRepairRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5ProviderRepairRowExportField> =
            self.export_fields.iter().copied().collect();
        M5ProviderRepairRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.isolates_settings_from_diagnostics
            && !self.loses_queued_work
            && !self.requires_blind_credential_reentry
            && !self.breaks_cached_read_continuity
            && !self.breaks_reviewed_export_path
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairVocabularySet {
    /// Provider-settings-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Boundary-class tokens.
    pub boundary_classes: Vec<String>,
    /// Repair-posture tokens.
    pub repair_postures: Vec<String>,
    /// Repair-entrypoint tokens.
    pub repair_entrypoints: Vec<String>,
    /// Linked-diagnostic tokens.
    pub linked_diagnostics: Vec<String>,
    /// Continuity-guarantee tokens.
    pub continuity_guarantees: Vec<String>,
    /// Repair-row-action tokens.
    pub row_actions: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Account-connection-state tokens (reused from the frozen matrix).
    pub connection_states: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ProviderRepairVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ProviderRepairConsumerSurface::ALL, |v| v.as_str()),
            boundary_classes: tokens(&M5ProviderBoundaryClass::ALL, |v| v.as_str()),
            repair_postures: tokens(&M5ProviderRepairPosture::ALL, |v| v.as_str()),
            repair_entrypoints: tokens(&M5RepairEntrypointClass::ALL, |v| v.as_str()),
            linked_diagnostics: tokens(&M5LinkedDiagnosticClass::ALL, |v| v.as_str()),
            continuity_guarantees: tokens(&M5RepairContinuityGuarantee::ALL, |v| v.as_str()),
            row_actions: tokens(&M5ProviderRepairRowAction::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ProviderRepairRowAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ProviderRepairRowExportField::ALL, |v| v.as_str()),
            connection_states: tokens(&M5AccountConnectionState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ProviderSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ProviderDeploymentLine::ALL, |v| v.as_str()),
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
pub struct M5ProviderRepairGovernanceReview {
    /// Rows link to the network / egress diagnostics.
    pub rows_link_to_network_egress_diagnostics: bool,
    /// Rows link to the auth / session diagnostics.
    pub rows_link_to_auth_diagnostics: bool,
    /// Rows link to the support-bundle diagnostics.
    pub rows_link_to_support_bundle_diagnostics: bool,
    /// Rows link to the provider-compatibility diagnostics.
    pub rows_link_to_provider_compatibility_diagnostics: bool,
    /// The repair preserves queued drafts.
    pub repair_preserves_queued_drafts: bool,
    /// The repair preserves cached-read continuity.
    pub repair_preserves_cached_read_continuity: bool,
    /// The repair preserves the reviewed export path.
    pub repair_preserves_reviewed_export_path: bool,
    /// The repair never forces a blind credential re-entry.
    pub repair_never_requires_blind_credential_reentry: bool,
    /// Provider settings are never an isolated sidebar divorced from diagnostics.
    pub settings_never_isolated_from_diagnostics: bool,
    /// Every boundary class names a concrete repair entrypoint.
    pub every_boundary_names_a_repair_entrypoint: bool,
    /// Rows keep the same truth across every deployment line.
    pub rows_stable_across_deployment_lines: bool,
    /// Rows keep the same truth across desktop, headless/export, and support consumers.
    pub rows_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs repair truth.
    pub support_export_reconstructs_repair_truth: bool,
    /// Later M5 rows cannot invent parallel repair vocabulary.
    pub later_rows_cannot_invent_parallel_repair_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairConsumerProjection {
    /// Provider surfaces consume the shared repair vocabulary.
    pub provider_surfaces_consume_repair_vocabulary: bool,
    /// The repair-posture resolver reads a single canonical source.
    pub repair_posture_reads_single_source: bool,
    /// The linked-diagnostics derivation reads a single canonical source.
    pub linked_diagnostics_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop rows read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the repair rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting provider repair / diagnostics audit.
    pub provider_repair_diagnostics_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProviderRepairEntrypointPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderRepairEntrypointPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-settings rows.
    pub rows: Vec<M5ProviderRepairConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderRepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderRepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderRepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderRepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderRepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 provider-settings repair-entrypoint-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderRepairEntrypointPacket {
    /// Record kind; must equal [`M5_PROVIDER_REPAIR_ENTRYPOINT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-settings rows.
    pub rows: Vec<M5ProviderRepairConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderRepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderRepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderRepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderRepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderRepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderRepairEntrypointPacket {
    /// Builds an M5 repair-entrypoint-row-primitive packet from stable-lane input.
    pub fn new(input: M5ProviderRepairEntrypointPacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_REPAIR_ENTRYPOINT_RECORD_KIND.to_owned(),
            schema_version: M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 repair-entrypoint-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5ProviderRepairEntrypointViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_REPAIR_ENTRYPOINT_RECORD_KIND {
            violations.push(M5ProviderRepairEntrypointViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_VERSION {
            violations.push(M5ProviderRepairEntrypointViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderRepairEntrypointViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_boundary_class_coverage(self, &mut violations);
        validate_repair_entrypoint_coverage(self, &mut violations);
        validate_linked_diagnostics_coverage(self, &mut violations);
        validate_continuity_preservation(self, &mut violations);
        validate_no_blind_credential_reentry(self, &mut violations);
        validate_policy_escalation_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 repair entrypoint row primitive packet serializes"),
        ) {
            violations.push(M5ProviderRepairEntrypointViolation::RawMaterialInExport);
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
            .expect("m5 repair entrypoint row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per provider-settings consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,boundary_classes,repair_entrypoints,linked_diagnostics,row_actions,examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.boundary_classes, |v| v.as_str()),
                join_tokens(&row.repair_entrypoints, |v| v.as_str()),
                join_tokens(&row.linked_diagnostics, |v| v.as_str()),
                join_tokens(&row.row_actions, |v| v.as_str()),
                row.examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Provider-Settings Repair-Entrypoint Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Provider-settings consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Boundary classes: {}\n",
            self.vocabulary_set.boundary_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Repair entrypoints: {}\n",
            self.vocabulary_set.repair_entrypoints.join(", ")
        ));
        out.push_str(&format!(
            "- Linked diagnostics: {}\n",
            self.vocabulary_set.linked_diagnostics.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Provider-settings consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str("  - Repair entrypoints:\n");
            for case in &row.examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` via `{}` (diagnostics `{}`, blind-reentry `{}`)\n",
                    case.resolved.repair_ref,
                    case.resolved.boundary_class.as_str(),
                    case.resolved.connection_state.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.repair_entrypoint.as_str(),
                    case.resolved.linked_diagnostics.len(),
                    case.resolved.requires_blind_credential_reentry,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 repair-entrypoint-row-primitive export.
#[derive(Debug)]
pub enum M5ProviderRepairEntrypointArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderRepairEntrypointViolation>),
}

impl fmt::Display for M5ProviderRepairEntrypointArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 repair entrypoint row primitive export parse failed: {error}"
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
                    "m5 repair entrypoint row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderRepairEntrypointArtifactError {}

/// Validation failures emitted by [`M5ProviderRepairEntrypointPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderRepairEntrypointViolation {
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
    /// A required provider-settings consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A provider-settings row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every boundary class.
    BoundaryClassCoverageUnproven,
    /// The worked resolutions do not exercise every repair entrypoint.
    RepairEntrypointCoverageUnproven,
    /// The worked resolutions do not exercise every linked diagnostic, or a row links to none.
    LinkedDiagnosticsCoverageUnproven,
    /// The worked resolutions do not preserve queued work, cached-read continuity, and the
    /// reviewed export path on every row.
    ContinuityPreservationUnproven,
    /// A worked resolution requires a blind credential re-entry, or no reviewed reauth entrypoint
    /// is proven.
    BlindCredentialReentryPresent,
    /// The worked resolutions do not prove a policy-blocked escalation and a self-serve repair.
    PolicyEscalationCoverageUnproven,
    /// A worked resolution does not preserve its exact repair identity.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
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

impl M5ProviderRepairEntrypointViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::BoundaryClassCoverageUnproven => "boundary_class_coverage_unproven",
            Self::RepairEntrypointCoverageUnproven => "repair_entrypoint_coverage_unproven",
            Self::LinkedDiagnosticsCoverageUnproven => "linked_diagnostics_coverage_unproven",
            Self::ContinuityPreservationUnproven => "continuity_preservation_unproven",
            Self::BlindCredentialReentryPresent => "blind_credential_reentry_present",
            Self::PolicyEscalationCoverageUnproven => "policy_escalation_coverage_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 repair-entrypoint-row-primitive export.
pub fn current_stable_m5_provider_repair_entrypoint_export(
) -> Result<M5ProviderRepairEntrypointPacket, M5ProviderRepairEntrypointArtifactError> {
    let packet: M5ProviderRepairEntrypointPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-settings-repair-entrypoint-row-proof/support_export.json"
    )))
    .map_err(M5ProviderRepairEntrypointArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderRepairEntrypointArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_DOC_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_COMPONENT_MATRIX_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_NETWORK_REMEDIATION_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_REAUTH_REQUIREMENT_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_PROVIDER_COMPAT_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_SUPPORT_BUNDLE_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_EXPORT_REDACTION_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_OFFLINE_HANDOFF_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProviderRepairEntrypointViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProviderRepairEntrypointViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let present: BTreeSet<M5ProviderRepairConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5ProviderRepairConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderRepairEntrypointViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.boundary_classes.is_empty()
            || row.connection_states.is_empty()
            || row.repair_postures.is_empty()
            || row.repair_entrypoints.is_empty()
            || row.linked_diagnostics.is_empty()
            || row.continuity_guarantees.is_empty()
            || row.row_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5ProviderRepairEntrypointViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ProviderRepairEntrypointViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5ProviderRepairEntrypointViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ProviderRepairEntrypointViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProviderRepairEntrypointViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProviderRepairEntrypointViolation::DowngradeTriggersMissing);
        }
        if row.examples.is_empty() {
            violations.push(M5ProviderRepairEntrypointViolation::ExampleMissing);
        }
        if row.examples.iter().any(|case| !case.is_self_consistent()) {
            violations.push(M5ProviderRepairEntrypointViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ProviderRepairEntrypointViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ProviderRepairEntrypointViolation::RowInvariantViolated);
        }
    }
}

/// Every boundary class must be exercised by some worked resolution — the acceptance criterion
/// that a user can repair any of the auth, network, mapping, or compatibility boundaries rather
/// than have them all collapse into "retry login".
fn validate_boundary_class_coverage(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let exercised: BTreeSet<M5ProviderBoundaryClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.examples.iter())
        .map(|case| case.resolved.boundary_class)
        .collect();
    if !M5ProviderBoundaryClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5ProviderRepairEntrypointViolation::BoundaryClassCoverageUnproven);
    }
}

/// Every repair entrypoint must be exercised by some worked resolution — the implementation
/// requirement that every boundary names a concrete repair entrypoint.
fn validate_repair_entrypoint_coverage(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let exercised: BTreeSet<M5RepairEntrypointClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.examples.iter())
        .map(|case| case.resolved.repair_entrypoint)
        .collect();
    if !M5RepairEntrypointClass::ALL
        .iter()
        .all(|entry| exercised.contains(entry))
    {
        violations.push(M5ProviderRepairEntrypointViolation::RepairEntrypointCoverageUnproven);
    }
}

/// Every linked-diagnostic class must be exercised, and every worked resolution must link to at
/// least one diagnostic — the acceptance criterion that provider settings stop feeling like an
/// isolated sidebar divorced from the diagnostics and export surfaces that explain the failure.
fn validate_linked_diagnostics_coverage(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.examples.iter());
    let exercised: BTreeSet<M5LinkedDiagnosticClass> = cases()
        .flat_map(|case| case.resolved.linked_diagnostics.iter().copied())
        .collect();
    let all_link = cases().all(|case| {
        case.resolved.links_to_diagnostics
            && !case.resolved.linked_diagnostics.is_empty()
            && !case.resolved.isolated_from_diagnostics
    });
    let all_classes = M5LinkedDiagnosticClass::ALL
        .iter()
        .all(|class| exercised.contains(class));
    if !(all_link && all_classes) {
        violations.push(M5ProviderRepairEntrypointViolation::LinkedDiagnosticsCoverageUnproven);
    }
}

/// Every worked resolution must preserve queued work, cached-read continuity, and the reviewed
/// export path — the acceptance criterion that a user repairs a boundary without losing queued
/// work. The set must also prove a repair with queued drafts still present.
fn validate_continuity_preservation(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.examples.iter());
    let all_preserve = cases().all(|case| case.preserves_continuity());
    let has_queued = cases().any(|case| case.resolved.has_queued_drafts);
    let has_cached = cases().any(|case| case.resolved.has_cached_read);
    if !(all_preserve && has_queued && has_cached) {
        violations.push(M5ProviderRepairEntrypointViolation::ContinuityPreservationUnproven);
    }
}

/// No worked resolution may require a blind credential re-entry, and the set must prove a reviewed
/// reauth entrypoint — the acceptance criterion that a user never re-enters credentials blindly.
fn validate_no_blind_credential_reentry(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.examples.iter());
    let none_blind = cases().all(|case| !case.resolved.requires_blind_credential_reentry);
    let has_reauth = cases().any(|case| {
        matches!(
            case.resolved.repair_entrypoint,
            M5RepairEntrypointClass::OpenReauthHandoff
        )
    });
    if !(none_blind && has_reauth) {
        violations.push(M5ProviderRepairEntrypointViolation::BlindCredentialReentryPresent);
    }
}

/// The set must prove a policy-blocked row that offers a reviewed escalation (and no self-serve
/// repair entrypoint) and at least one non-policy row that offers a self-serve repair entrypoint —
/// the implementation requirement that a policy block is repaired only through review.
fn validate_policy_escalation_coverage(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.examples.iter());
    let has_policy_escalation = cases().any(|case| {
        case.resolved.boundary_class.is_policy_blocked()
            && case
                .resolved
                .available_actions
                .contains(&M5ProviderRepairRowAction::RequestPolicyEscalation)
            && !case
                .resolved
                .available_actions
                .contains(&M5ProviderRepairRowAction::OpenRepairEntrypoint)
    });
    let has_self_serve = cases().any(|case| {
        !case.resolved.boundary_class.is_policy_blocked()
            && case
                .resolved
                .available_actions
                .contains(&M5ProviderRepairRowAction::OpenRepairEntrypoint)
    });
    if !(has_policy_escalation && has_self_serve) {
        violations.push(M5ProviderRepairEntrypointViolation::PolicyEscalationCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact repair identity — the invariant that the row
/// never rewrites the user's boundary, target, or repair identity.
fn validate_identity_preservation(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let ok = packet
        .rows
        .iter()
        .flat_map(|row| row.examples.iter())
        .all(|case| case.preserves_repair_identity());
    if !ok {
        violations.push(M5ProviderRepairEntrypointViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.rows_link_to_network_egress_diagnostics,
        review.rows_link_to_auth_diagnostics,
        review.rows_link_to_support_bundle_diagnostics,
        review.rows_link_to_provider_compatibility_diagnostics,
        review.repair_preserves_queued_drafts,
        review.repair_preserves_cached_read_continuity,
        review.repair_preserves_reviewed_export_path,
        review.repair_never_requires_blind_credential_reentry,
        review.settings_never_isolated_from_diagnostics,
        review.every_boundary_names_a_repair_entrypoint,
        review.rows_stable_across_deployment_lines,
        review.rows_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_repair_truth,
        review.later_rows_cannot_invent_parallel_repair_vocabulary,
    ] {
        if !ok {
            violations.push(M5ProviderRepairEntrypointViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.provider_surfaces_consume_repair_vocabulary,
        projection.repair_posture_reads_single_source,
        projection.linked_diagnostics_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5ProviderRepairEntrypointViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProviderRepairEntrypointViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProviderRepairEntrypointPacket,
    violations: &mut Vec<M5ProviderRepairEntrypointViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .provider_repair_diagnostics_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProviderRepairEntrypointViolation::ReleasePostureIncomplete);
    }
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
