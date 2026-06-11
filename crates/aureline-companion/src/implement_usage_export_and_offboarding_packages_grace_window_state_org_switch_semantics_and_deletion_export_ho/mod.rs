//! Usage-export and offboarding packages, grace-window state, org-switch
//! semantics, and deletion/export honesty, projected as a downgrade-aware truth packet.
//!
//! This module owns the export-safe truth packet for the offboarding-depth lane. It
//! projects four sections: the **usage-export packages** that record which
//! usage-data export packages are offered, whether each is available locally now or
//! requires provider assembly, and whether the package is complete; the
//! **offboarding packages** that record the full leave-the-product export bundles per
//! data class and assert that user-owned local work is preserved; the **grace-window
//! state** that records, per scheduled deletion, its deletion scope, whether the grace
//! window is still open and reversible or already committed and irreversible, and that
//! user-owned local work is retained; and the **org-switch semantics** that record,
//! per data class, what migrates with the user, what stays local to the user, what is
//! left with the prior org, and what requires admin approval. Every section binds to
//! the frozen M5 companion-matrix [`M5CompanionMatrixLane::OffboardingContinuity`] lane
//! and gives every item an exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so opening an item always resumes the precise host context locally.
//!
//! Three invariants make this surface safe to ship. First, **read-only projection,
//! with the local core authoritative**: every section is read-only, the surface
//! *projects* the offboarding state but never applies it — an export, a deletion, or
//! an org switch is applied by the local core, never authored from this surface — and
//! a local-first path is always offered for usage-export and offboarding packages so a
//! degraded provider never strands the user. Second, **deletion/export honesty**: an
//! export package claims completeness only when it is backed by evidence; an
//! unverifiable claim narrows ([`ExportCompleteness::CompleteUnverified`]) and is
//! labeled rather than shown as proven, and an irreversible (committed) deletion is
//! always labeled rather than shown as still reversible. Third, **never strand local
//! work**: every offboarding package, grace-window row, and org-switch row preserves
//! user-owned local work — `local_work_preserved` and `user_owned_local_retained`
//! never go false — so offboarding, deletion, or an org switch never leaves
//! user-owned local work behind.
//!
//! [`UsageExportOffboardingSurfacePacket::apply_offboarding_degradation`] narrows
//! sections, narrows package availability to its local path, downgrades completeness
//! claims, holds grace windows open when deletion can no longer commit, and downgrades
//! freshness from a per-observation signal — when the export assembler is unavailable,
//! the deletion service is unavailable, proof is stale, completeness is unverified,
//! admin continuity is unavailable, the managed service is degraded, the host session
//! is inactive, or an upstream matrix lane narrowed — so CI or release tooling degrades
//! the surface honestly rather than show a package it can no longer assemble, a
//! completeness claim it can no longer verify, or a deletion it can no longer commit.
//! Degraded state is labeled, never hidden, the local path always remains, and local
//! work is always preserved.
//!
//! [`canonical_usage_export_offboarding_surface`] builds the surface and
//! [`current_stable_usage_export_offboarding_surface_export`] reads and validates the
//! checked-in support export, so the desktop companion panel, the CLI/headless surface,
//! diagnostics, support exports, and Help/About ingest the packet rather than cloning
//! status text. Credential bodies, raw account payloads, raw provider payloads, and raw
//! export record contents stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/implement-usage-export-and-offboarding-packages-grace-window-state-org-switch-semantics-and-deletion-export-ho.schema.json`](../../../../schemas/companion/implement-usage-export-and-offboarding-packages-grace-window-state-org-switch-semantics-and-deletion-export-ho.schema.json).
//! The contract doc is
//! [`docs/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho.md`](../../../../docs/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho/`](../../../../fixtures/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::{
    CompanionDesktopHandoff, CompanionHandoffResolution, CompanionHandoffTarget,
};
use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    M5CompanionConsumerSurface, M5CompanionDowngradeTrigger, M5CompanionLocalityDisclosure,
    M5CompanionMatrixLane, M5CompanionQualificationClass, M5CompanionRollbackPosture,
    M5CompanionRolloutStage, M5_COMPANION_MATRIX_SCHEMA_REF, M5_OFFBOARDING_CONTRACT_REF,
    M5_OFFBOARDING_RETENTION_MATRIX_REF,
};
use crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty::{
    CompanionFreshnessState, CompanionReadWriteScope,
};

/// Stable record-kind tag carried by [`UsageExportOffboardingSurfacePacket`].
pub const USAGE_EXPORT_OFFBOARDING_RECORD_KIND: &str =
    "implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho";

/// Schema version for usage-export/offboarding surface records.
pub const USAGE_EXPORT_OFFBOARDING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const USAGE_EXPORT_OFFBOARDING_SCHEMA_REF: &str =
    "schemas/companion/implement-usage-export-and-offboarding-packages-grace-window-state-org-switch-semantics-and-deletion-export-ho.schema.json";

/// Repo-relative path of the surface contract doc.
pub const USAGE_EXPORT_OFFBOARDING_DOC_REF: &str =
    "docs/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho.md";

/// Repo-relative path of the protected fixture directory.
pub const USAGE_EXPORT_OFFBOARDING_FIXTURE_DIR: &str =
    "fixtures/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho";

/// Repo-relative path of the checked support-export artifact.
pub const USAGE_EXPORT_OFFBOARDING_ARTIFACT_REF: &str =
    "artifacts/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const USAGE_EXPORT_OFFBOARDING_SUMMARY_REF: &str =
    "artifacts/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho.md";

/// One of the four usage-export/offboarding sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingSection {
    /// The usage-data export packages.
    UsageExportPackage,
    /// The full offboarding export bundles.
    OffboardingPackage,
    /// The grace-window state for scheduled deletions.
    GraceWindowState,
    /// The org-switch semantics per data class.
    OrgSwitchSemantics,
}

impl OffboardingSection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UsageExportPackage,
        Self::OffboardingPackage,
        Self::GraceWindowState,
        Self::OrgSwitchSemantics,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UsageExportPackage => "usage_export_package",
            Self::OffboardingPackage => "offboarding_package",
            Self::GraceWindowState => "grace_window_state",
            Self::OrgSwitchSemantics => "org_switch_semantics",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    ///
    /// Every section inherits from the
    /// [`M5CompanionMatrixLane::OffboardingContinuity`] lane.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        M5CompanionMatrixLane::OffboardingContinuity
    }

    /// Read/write scope this section is bounded to.
    ///
    /// Every section is read-only: the surface projects the offboarding state but
    /// never applies it. An export, a deletion, or an org switch is applied by the
    /// local core, never authored from this surface.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        CompanionReadWriteScope::ReadOnly
    }
}

/// A data class an export package, grace-window row, or org-switch row describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingArtifactClass {
    /// The user's local workspace and edit history.
    LocalWorkspace,
    /// The user's usage and activity history.
    UsageHistory,
    /// Managed profile and settings.
    ManagedProfile,
    /// Managed snapshot archives.
    ManagedSnapshots,
    /// The managed audit trail.
    AuditTrail,
}

impl OffboardingArtifactClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::UsageHistory => "usage_history",
            Self::ManagedProfile => "managed_profile",
            Self::ManagedSnapshots => "managed_snapshots",
            Self::AuditTrail => "audit_trail",
        }
    }
}

/// Availability of an export or offboarding package.
///
/// A usage-export or offboarding package is either ready locally now, staging locally,
/// available only via provider assembly, or unavailable. A local path
/// ([`Self::LocalReady`], [`Self::LocalStaging`]) is always offered so a degraded
/// provider never strands the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageAvailability {
    /// The package is ready locally right now.
    LocalReady,
    /// The package is assembling locally from the local core.
    LocalStaging,
    /// The package requires provider assembly.
    RequiresProviderAssembly,
    /// The package is unavailable in this tier or while the provider is down.
    Unavailable,
}

impl PackageAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReady => "local_ready",
            Self::LocalStaging => "local_staging",
            Self::RequiresProviderAssembly => "requires_provider_assembly",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when this package can be produced entirely from the local core.
    ///
    /// A local-ready or locally-staging package remains a fallback when the managed
    /// export assembler is unavailable, so an export degradation never strands the
    /// user.
    pub const fn is_local_path(self) -> bool {
        matches!(self, Self::LocalReady | Self::LocalStaging)
    }

    /// Narrows a provider-assembled package to [`Self::Unavailable`]; a local-path or
    /// already-unavailable package is kept.
    ///
    /// Used when the managed export assembler is lost: a provider-assembled package can
    /// no longer be produced, so it narrows to unavailable rather than continuing to
    /// look obtainable, while every local-path package keeps working.
    pub const fn narrowed_when_provider_lost(self) -> Self {
        match self {
            Self::RequiresProviderAssembly => Self::Unavailable,
            Self::LocalReady | Self::LocalStaging | Self::Unavailable => self,
        }
    }
}

/// Completeness of an export or offboarding package.
///
/// A package claims completeness only when it is backed by evidence; an unverifiable
/// claim narrows to [`Self::CompleteUnverified`] and is labeled rather than shown as
/// proven, and a known-partial package states so honestly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportCompleteness {
    /// Complete, and the completeness is verified by evidence.
    CompleteVerified,
    /// Claimed complete but the completeness could not be verified.
    CompleteUnverified,
    /// Known to be partial; stated honestly.
    Partial,
}

impl ExportCompleteness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteVerified => "complete_verified",
            Self::CompleteUnverified => "complete_unverified",
            Self::Partial => "partial",
        }
    }

    /// True when this is a verified completeness claim.
    pub const fn is_complete_claim(self) -> bool {
        matches!(self, Self::CompleteVerified)
    }

    /// Downgrades a verified completeness claim to [`Self::CompleteUnverified`]; other
    /// states are kept.
    pub const fn downgraded_to_unverified(self) -> Self {
        match self {
            Self::CompleteVerified => Self::CompleteUnverified,
            Self::CompleteUnverified | Self::Partial => self,
        }
    }
}

/// What a scheduled deletion covers.
///
/// Every offered deletion scope leaves user-owned local work intact: it deletes managed
/// state, a local cache, or both managed and derived local caches, but never the user's
/// authoritative local work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletionScope {
    /// Deletes managed state and a derived local cache; authoritative local work is retained.
    LocalAndManaged,
    /// Deletes managed state only; local work is retained.
    ManagedOnlyLocalRetained,
    /// Clears a local cache only; the managed copy and the authoritative local work are retained.
    LocalOnly,
}

impl DeletionScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAndManaged => "local_and_managed",
            Self::ManagedOnlyLocalRetained => "managed_only_local_retained",
            Self::LocalOnly => "local_only",
        }
    }
}

/// State of the grace window for a scheduled deletion.
///
/// A deletion is reversible while its grace window is open or closing; once committed it
/// is irreversible and must be labeled rather than shown as still reversible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraceWindowPosture {
    /// The grace window is open and the deletion is fully reversible.
    OpenReversible,
    /// The grace window is closing but the deletion is still reversible.
    ClosingReversible,
    /// The deletion has committed and is irreversible.
    CommittedIrreversible,
    /// No deletion is scheduled.
    NotScheduled,
}

impl GraceWindowPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenReversible => "open_reversible",
            Self::ClosingReversible => "closing_reversible",
            Self::CommittedIrreversible => "committed_irreversible",
            Self::NotScheduled => "not_scheduled",
        }
    }

    /// True when the deletion is still reversible.
    pub const fn is_reversible(self) -> bool {
        matches!(self, Self::OpenReversible | Self::ClosingReversible)
    }

    /// True when the deletion has committed and is irreversible.
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::CommittedIrreversible)
    }

    /// Holds a closing grace window open again; other states are kept.
    ///
    /// Used when the deletion service is lost: a deletion that was closing can no longer
    /// commit, so its window is held open rather than allowed to close, and an already
    /// committed or unscheduled deletion is unchanged. This only ever widens the
    /// reversible window, so it never strands the user.
    pub const fn narrowed_when_deletion_service_lost(self) -> Self {
        match self {
            Self::ClosingReversible => Self::OpenReversible,
            Self::OpenReversible | Self::CommittedIrreversible | Self::NotScheduled => self,
        }
    }
}

/// What happens to a data class when the user switches orgs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrgSwitchDisposition {
    /// The data class migrates with the user to the new org.
    MigratesWithUser,
    /// The data class stays local to the user, independent of any org.
    StaysLocalToUser,
    /// The migration requires prior-org admin approval first.
    RequiresAdminApproval,
    /// The data class stays with the prior org by policy.
    LeftWithPriorOrg,
}

impl OrgSwitchDisposition {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigratesWithUser => "migrates_with_user",
            Self::StaysLocalToUser => "stays_local_to_user",
            Self::RequiresAdminApproval => "requires_admin_approval",
            Self::LeftWithPriorOrg => "left_with_prior_org",
        }
    }

    /// True when the migration requires prior-org admin approval.
    pub const fn requires_admin(self) -> bool {
        matches!(self, Self::RequiresAdminApproval)
    }

    /// True when the data class is left with the prior org.
    pub const fn left_with_prior_org(self) -> bool {
        matches!(self, Self::LeftWithPriorOrg)
    }
}

/// A usage-data export-package row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageExportPackageItem {
    /// Stable item id.
    pub item_id: String,
    /// Data class this package exports.
    pub data_class: OffboardingArtifactClass,
    /// Availability of the package.
    pub availability: PackageAvailability,
    /// Completeness of the package, proved where claimed.
    pub completeness: ExportCompleteness,
    /// True when the completeness claim is verified by evidence.
    pub claim_verified: bool,
    /// True when an unverified claim carries a visible "claimed, not verified" label.
    pub proof_label_shown: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the export-package record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the usage-export package.
    pub handoff: CompanionDesktopHandoff,
}

/// A full offboarding export-bundle row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingPackageItem {
    /// Stable item id.
    pub item_id: String,
    /// Data class this bundle packages.
    pub data_class: OffboardingArtifactClass,
    /// Availability of the bundle.
    pub availability: PackageAvailability,
    /// Completeness of the bundle, proved where claimed.
    pub completeness: ExportCompleteness,
    /// True when the completeness claim is verified by evidence.
    pub claim_verified: bool,
    /// True when an unverified claim carries a visible "claimed, not verified" label.
    pub proof_label_shown: bool,
    /// Always true: offboarding never strands user-owned local work.
    pub local_work_preserved: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the offboarding-bundle record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the offboarding package.
    pub handoff: CompanionDesktopHandoff,
}

/// A grace-window-state row for a scheduled deletion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraceWindowStateItem {
    /// Stable item id.
    pub item_id: String,
    /// Data class this deletion covers.
    pub data_class: OffboardingArtifactClass,
    /// What the deletion covers.
    pub deletion_scope: DeletionScope,
    /// State of the grace window.
    pub grace_posture: GraceWindowPosture,
    /// True when the deletion is still reversible; mirrors [`GraceWindowPosture::is_reversible`].
    pub reversible: bool,
    /// True when an irreversible (committed) deletion carries a visible label.
    pub irreversible_labeled: bool,
    /// Always true: a scheduled deletion never strands user-owned local work.
    pub local_work_preserved: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the deletion record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the grace-window row.
    pub handoff: CompanionDesktopHandoff,
}

/// An org-switch-semantics row for a data class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrgSwitchSemanticsItem {
    /// Stable item id.
    pub item_id: String,
    /// Data class this row describes.
    pub data_class: OffboardingArtifactClass,
    /// What happens to the data class on an org switch.
    pub disposition: OrgSwitchDisposition,
    /// True when this data class is user-owned local work.
    pub user_owned: bool,
    /// Always true for user-owned data: the user's local work is retained across the switch.
    pub user_owned_local_retained: bool,
    /// True when the migration requires admin approval; mirrors [`OrgSwitchDisposition::requires_admin`].
    pub requires_admin: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the org-switch record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the org-switch row.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingSectionQualification {
    /// Section the row applies to.
    pub section: OffboardingSection,
    /// Qualification class earned by this section.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Read/write scope this section is bounded to.
    pub read_write_scope: CompanionReadWriteScope,
    /// Token of the frozen matrix lane this section inherits qualification from.
    pub matrix_lane_ref: String,
    /// Downgrade triggers that apply to this section.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
}

/// Read/write scope and authority contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingScopeContract {
    /// The usage-export-package section is read-only.
    pub usage_export_read_only: bool,
    /// The offboarding-package section is read-only.
    pub offboarding_package_read_only: bool,
    /// The grace-window-state section is read-only.
    pub grace_window_read_only: bool,
    /// The org-switch-semantics section is read-only.
    pub org_switch_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// An export, deletion, or org switch is applied by the local core, never authored from this surface.
    pub action_applied_by_local_core_not_surface: bool,
    /// The surface never holds an unbounded write authority.
    pub no_unbounded_workspace_write: bool,
    /// Offboarding never strands user-owned local work.
    pub offboarding_never_strands_local_work: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Deletion/export honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingHonestyContract {
    /// A local-first usage-export path is always offered.
    pub usage_export_local_path_always_available: bool,
    /// A local-first offboarding-package path is always offered.
    pub offboarding_package_local_path_always_available: bool,
    /// Export completeness is provable or labeled as unverified.
    pub export_completeness_provable_or_labeled: bool,
    /// The deletion scope is disclosed for every scheduled deletion.
    pub deletion_scope_disclosed: bool,
    /// A deletion is reversible while its grace window is open.
    pub deletion_reversible_within_grace_window: bool,
    /// An irreversible (committed) deletion is labeled, never shown as still reversible.
    pub irreversible_deletion_labeled: bool,
    /// The org-switch disposition is disclosed for every data class.
    pub org_switch_disposition_disclosed: bool,
    /// User-owned local work is never left with the prior org.
    pub user_owned_local_never_left_with_prior_org: bool,
    /// No completeness, deletion, or migration claim is made without backing evidence.
    pub no_claim_without_evidence: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingStaleStateHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Continuity contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuityContract {
    /// The local core continues when the managed provider degrades.
    pub local_core_continues_when_provider_degrades: bool,
    /// A degraded capability is labeled, not hidden.
    pub degraded_capability_labeled_not_hidden: bool,
    /// User-owned local work is never stranded.
    pub local_work_never_stranded: bool,
    /// Provider and admin continuity requirements are disclosed.
    pub provider_and_admin_continuity_disclosed: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingSecurityReview {
    /// The usage-export-package section is read-only.
    pub usage_export_read_only: bool,
    /// The offboarding-package section is read-only.
    pub offboarding_package_read_only: bool,
    /// The grace-window-state section is read-only.
    pub grace_window_read_only: bool,
    /// The org-switch-semantics section is read-only.
    pub org_switch_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// An export, deletion, or org switch is applied by the local core, never from this surface.
    pub action_applied_by_local_core_not_surface: bool,
    /// A local-first usage-export path is always offered.
    pub usage_export_local_path_always_available: bool,
    /// A local-first offboarding-package path is always offered.
    pub offboarding_package_local_path_always_available: bool,
    /// Export completeness is provable or labeled as unverified.
    pub export_completeness_provable_or_labeled: bool,
    /// The deletion scope is disclosed for every scheduled deletion.
    pub deletion_scope_disclosed: bool,
    /// A deletion is reversible within its grace window.
    pub deletion_reversible_within_grace_window: bool,
    /// An irreversible deletion is labeled, never shown as still reversible.
    pub irreversible_deletion_labeled: bool,
    /// User-owned local work is never left with the prior org.
    pub user_owned_local_never_left_with_prior_org: bool,
    /// Offboarding never strands user-owned local work.
    pub local_work_never_stranded: bool,
    /// Stale state is labeled rather than hidden.
    pub stale_state_labeled_never_hidden: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// No credential or account bodies cross the export boundary.
    pub no_credential_or_account_bodies_in_export: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the section.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every section discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingConsumerProjection {
    /// Desktop panel projects the usage-export packages.
    pub desktop_panel_shows_usage_export: bool,
    /// Desktop panel projects the offboarding packages.
    pub desktop_panel_shows_offboarding_package: bool,
    /// Desktop panel projects the grace-window state.
    pub desktop_panel_shows_grace_window: bool,
    /// Diagnostics shows the org-switch semantics.
    pub diagnostics_shows_org_switch_semantics: bool,
    /// CLI / headless shows the export and deletion state.
    pub cli_headless_shows_export_and_deletion_state: bool,
    /// Support export shows the packages and grace-window state.
    pub support_export_shows_packages_and_grace_window: bool,
    /// Help / About shows the deletion and export honesty.
    pub help_about_shows_deletion_and_export_honesty: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the section.
    pub auto_narrow_on_stale: bool,
}

/// Per-observation signal fed to
/// [`UsageExportOffboardingSurfacePacket::apply_offboarding_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffboardingDegradationObservation {
    /// True when the managed export assembler is available.
    pub export_assembler_available: bool,
    /// True when the deletion service is available (deletions can commit).
    pub deletion_service_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when export completeness claims are verified.
    pub completeness_verified: bool,
    /// True when managed-tenant admin continuity is available.
    pub admin_continuity_available: bool,
    /// True when the managed service is available (not degraded).
    pub managed_service_available: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Reason a section has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingDegradedReason {
    /// The managed export assembler is unavailable.
    ExportAssemblerUnavailable,
    /// The deletion service is unavailable.
    DeletionServiceUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// An export completeness claim could not be verified.
    CompletenessUnverified,
    /// Managed-tenant admin continuity is unavailable.
    AdminContinuityUnavailable,
    /// The managed service is degraded.
    ManagedServiceDegraded,
    /// No active desktop host session.
    HostSessionInactive,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more desktop handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
    /// One or more packages narrowed to their local path.
    PackageNarrowedToLocalPath,
    /// One or more completeness claims were downgraded to claimed-but-unverified.
    CompletenessClaimDowngraded,
    /// One or more grace windows were held open because deletion could not commit.
    GraceWindowHeldOpen,
}

impl OffboardingDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportAssemblerUnavailable => "export_assembler_unavailable",
            Self::DeletionServiceUnavailable => "deletion_service_unavailable",
            Self::ProofStale => "proof_stale",
            Self::CompletenessUnverified => "completeness_unverified",
            Self::AdminContinuityUnavailable => "admin_continuity_unavailable",
            Self::ManagedServiceDegraded => "managed_service_degraded",
            Self::HostSessionInactive => "host_session_inactive",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
            Self::PackageNarrowedToLocalPath => "package_narrowed_to_local_path",
            Self::CompletenessClaimDowngraded => "completeness_claim_downgraded",
            Self::GraceWindowHeldOpen => "grace_window_held_open",
        }
    }
}

/// Constructor input for [`UsageExportOffboardingSurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageExportOffboardingSurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<OffboardingSectionQualification>,
    /// Usage-export package items.
    pub usage_export_packages: Vec<UsageExportPackageItem>,
    /// Offboarding package items.
    pub offboarding_packages: Vec<OffboardingPackageItem>,
    /// Grace-window-state items.
    pub grace_window_rows: Vec<GraceWindowStateItem>,
    /// Org-switch-semantics items.
    pub org_switch_rows: Vec<OrgSwitchSemanticsItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: OffboardingScopeContract,
    /// Deletion/export honesty contract.
    pub honesty_contract: OffboardingHonestyContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: OffboardingStaleStateHonesty,
    /// Continuity contract.
    pub continuity_contract: OffboardingContinuityContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: OffboardingSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: OffboardingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: OffboardingProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe usage-export/offboarding surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageExportOffboardingSurfacePacket {
    /// Record kind; must equal [`USAGE_EXPORT_OFFBOARDING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`USAGE_EXPORT_OFFBOARDING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<OffboardingSectionQualification>,
    /// Usage-export package items.
    pub usage_export_packages: Vec<UsageExportPackageItem>,
    /// Offboarding package items.
    pub offboarding_packages: Vec<OffboardingPackageItem>,
    /// Grace-window-state items.
    pub grace_window_rows: Vec<GraceWindowStateItem>,
    /// Org-switch-semantics items.
    pub org_switch_rows: Vec<OrgSwitchSemanticsItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: OffboardingScopeContract,
    /// Deletion/export honesty contract.
    pub honesty_contract: OffboardingHonestyContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: OffboardingStaleStateHonesty,
    /// Continuity contract.
    pub continuity_contract: OffboardingContinuityContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: OffboardingSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: OffboardingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: OffboardingProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<OffboardingDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl UsageExportOffboardingSurfacePacket {
    /// Builds a usage-export/offboarding surface packet from stable-lane input.
    pub fn new(input: UsageExportOffboardingSurfacePacketInput) -> Self {
        Self {
            record_kind: USAGE_EXPORT_OFFBOARDING_RECORD_KIND.to_owned(),
            schema_version: USAGE_EXPORT_OFFBOARDING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            usage_export_packages: input.usage_export_packages,
            offboarding_packages: input.offboarding_packages,
            grace_window_rows: input.grace_window_rows,
            org_switch_rows: input.org_switch_rows,
            scope_contract: input.scope_contract,
            honesty_contract: input.honesty_contract,
            stale_state_honesty: input.stale_state_honesty,
            continuity_contract: input.continuity_contract,
            locality_disclosure: input.locality_disclosure,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            degraded_labels: Vec::new(),
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows sections, narrows package availability to its local path, downgrades
    /// completeness claims, holds grace windows open when deletion cannot commit, and
    /// downgrades freshness from a per-observation signal, recording the reasons in
    /// [`Self::degraded_labels`].
    ///
    /// A degraded managed service, stale proof, or narrowed upstream matrix lane narrows
    /// every section one step, and a degraded managed service additionally forces every
    /// live or cached item to stale and labels it. When the export assembler is
    /// unavailable, every provider-assembled package narrows to
    /// [`PackageAvailability::Unavailable`] while the local path always remains. When
    /// completeness is unverified, every verified completeness claim downgrades to
    /// [`ExportCompleteness::CompleteUnverified`] and is labeled. When the deletion
    /// service is unavailable, every closing grace window is held open again, widening
    /// the reversible window. Missing admin continuity narrows the offboarding-package,
    /// grace-window, and org-switch sections. An inactive host session downgrades the
    /// resolution of every host-dependent desktop handoff. The local path always
    /// remains, and local work is always preserved. Degraded state is labeled, never
    /// hidden.
    pub fn apply_offboarding_degradation(
        &mut self,
        observation: &OffboardingDegradationObservation,
    ) {
        let mut labels: BTreeSet<OffboardingDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let section_adverse = !observation.managed_service_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.managed_service_available {
            labels.insert(OffboardingDegradedReason::ManagedServiceDegraded);
            if self.force_all_freshness_stale() {
                labels.insert(OffboardingDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(OffboardingDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(OffboardingDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.export_assembler_available {
            labels.insert(OffboardingDegradedReason::ExportAssemblerUnavailable);
            let mut narrowed = false;
            for item in &mut self.usage_export_packages {
                let next = item.availability.narrowed_when_provider_lost();
                if next != item.availability {
                    item.availability = next;
                    narrowed = true;
                }
            }
            for item in &mut self.offboarding_packages {
                let next = item.availability.narrowed_when_provider_lost();
                if next != item.availability {
                    item.availability = next;
                    narrowed = true;
                }
            }
            if narrowed {
                labels.insert(OffboardingDegradedReason::PackageNarrowedToLocalPath);
            }
        }
        if !observation.completeness_verified {
            labels.insert(OffboardingDegradedReason::CompletenessUnverified);
            if self.force_all_completeness_unverified() {
                labels.insert(OffboardingDegradedReason::CompletenessClaimDowngraded);
            }
        }
        if !observation.deletion_service_available {
            labels.insert(OffboardingDegradedReason::DeletionServiceUnavailable);
            let mut held = false;
            for item in &mut self.grace_window_rows {
                let next = item.grace_posture.narrowed_when_deletion_service_lost();
                if next != item.grace_posture {
                    item.grace_posture = next;
                    item.reversible = next.is_reversible();
                    held = true;
                }
            }
            if held {
                labels.insert(OffboardingDegradedReason::GraceWindowHeldOpen);
            }
        }
        if !observation.admin_continuity_available {
            labels.insert(OffboardingDegradedReason::AdminContinuityUnavailable);
        }

        for row in &mut self.section_qualifications {
            let adverse = section_adverse
                || (!observation.export_assembler_available
                    && matches!(
                        row.section,
                        OffboardingSection::UsageExportPackage
                            | OffboardingSection::OffboardingPackage
                    ))
                || (!observation.completeness_verified
                    && matches!(
                        row.section,
                        OffboardingSection::UsageExportPackage
                            | OffboardingSection::OffboardingPackage
                    ))
                || (!observation.deletion_service_available
                    && row.section == OffboardingSection::GraceWindowState)
                || (!observation.admin_continuity_available
                    && matches!(
                        row.section,
                        OffboardingSection::OffboardingPackage
                            | OffboardingSection::GraceWindowState
                            | OffboardingSection::OrgSwitchSemantics
                    ));
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(OffboardingDegradedReason::HostSessionInactive);
            let mut any_unresolved = false;
            for handoff in self.handoffs_mut() {
                if handoff.requires_active_host
                    && handoff.resolution == CompanionHandoffResolution::Exact
                {
                    handoff.resolution = CompanionHandoffResolution::Unresolved;
                    any_unresolved = true;
                }
            }
            if any_unresolved {
                labels.insert(OffboardingDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Forces every live/cached item freshness to stale and labels it. Returns true
    /// when at least one item was downgraded.
    fn force_all_freshness_stale(&mut self) -> bool {
        let mut downgraded = false;
        for (state, label) in self.freshness_states_mut() {
            if *state != state.forced_stale() {
                *state = state.forced_stale();
                *label = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Downgrades every verified completeness claim (in usage-export and offboarding
    /// packages) to unverified and labels it. Returns true when at least one claim was
    /// downgraded.
    fn force_all_completeness_unverified(&mut self) -> bool {
        let mut downgraded = false;
        for item in &mut self.usage_export_packages {
            if item.completeness.is_complete_claim() {
                item.completeness = item.completeness.downgraded_to_unverified();
                item.claim_verified = false;
                item.proof_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.offboarding_packages {
            if item.completeness.is_complete_claim() {
                item.completeness = item.completeness.downgraded_to_unverified();
                item.claim_verified = false;
                item.proof_label_shown = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Mutable access to every item's freshness state and stale-label flag.
    fn freshness_states_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut CompanionFreshnessState, &mut bool)> {
        self.usage_export_packages
            .iter_mut()
            .map(|item| (&mut item.freshness, &mut item.stale_label_shown))
            .chain(
                self.offboarding_packages
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.grace_window_rows
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.org_switch_rows
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
    }

    /// Validates the surface invariants.
    pub fn validate(&self) -> Vec<OffboardingViolation> {
        let mut violations = Vec::new();

        if self.record_kind != USAGE_EXPORT_OFFBOARDING_RECORD_KIND {
            violations.push(OffboardingViolation::WrongRecordKind);
        }
        if self.schema_version != USAGE_EXPORT_OFFBOARDING_SCHEMA_VERSION {
            violations.push(OffboardingViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(OffboardingViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(OffboardingViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_section_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_honesty_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_continuity_contract(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("usage-export/offboarding packet serializes"),
        ) {
            violations.push(OffboardingViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("usage-export/offboarding packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(&self) -> impl Iterator<Item = &OffboardingSectionQualification> {
        self.section_qualifications.iter().filter(|row| {
            matches!(
                row.qualification,
                M5CompanionQualificationClass::Stable
                    | M5CompanionQualificationClass::Beta
                    | M5CompanionQualificationClass::Preview
            ) && row.rollout_stage != M5CompanionRolloutStage::Withheld
        })
    }

    /// True when every item's desktop handoff resolves to the exact location.
    pub fn all_handoffs_exact(&self) -> bool {
        self.handoffs()
            .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact)
    }

    /// True when a local-first usage-export path is offered as a fallback.
    pub fn usage_export_local_path_available(&self) -> bool {
        self.usage_export_packages
            .iter()
            .any(|item| item.availability.is_local_path())
    }

    /// True when a local-first offboarding-package path is offered as a fallback.
    pub fn offboarding_package_local_path_available(&self) -> bool {
        self.offboarding_packages
            .iter()
            .any(|item| item.availability.is_local_path())
    }

    /// True when every export/offboarding completeness claim is verified or labeled.
    pub fn export_completeness_honestly_qualified(&self) -> bool {
        let row_ok = |complete: ExportCompleteness, verified: bool, labeled: bool| {
            if complete.is_complete_claim() {
                verified
            } else if complete == ExportCompleteness::CompleteUnverified {
                labeled
            } else {
                true
            }
        };
        self.usage_export_packages.iter().all(|item| {
            row_ok(
                item.completeness,
                item.claim_verified,
                item.proof_label_shown,
            )
        }) && self.offboarding_packages.iter().all(|item| {
            row_ok(
                item.completeness,
                item.claim_verified,
                item.proof_label_shown,
            )
        })
    }

    /// True when every irreversible deletion is labeled as such.
    pub fn deletion_honestly_labeled(&self) -> bool {
        self.grace_window_rows
            .iter()
            .all(|item| !item.grace_posture.is_committed() || item.irreversible_labeled)
    }

    /// True when offboarding never strands user-owned local work.
    ///
    /// Every offboarding package and grace-window row preserves local work, and every
    /// user-owned data class is retained and never left with the prior org on a switch.
    pub fn local_work_never_stranded(&self) -> bool {
        self.offboarding_packages
            .iter()
            .all(|item| item.local_work_preserved)
            && self
                .grace_window_rows
                .iter()
                .all(|item| item.local_work_preserved)
            && self.org_switch_rows.iter().all(|item| {
                !item.user_owned
                    || (item.user_owned_local_retained && !item.disposition.left_with_prior_org())
            })
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.usage_export_packages
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .offboarding_packages
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .grace_window_rows
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .org_switch_rows
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// Iterates every desktop handoff across all four sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.usage_export_packages
            .iter()
            .map(|item| &item.handoff)
            .chain(self.offboarding_packages.iter().map(|item| &item.handoff))
            .chain(self.grace_window_rows.iter().map(|item| &item.handoff))
            .chain(self.org_switch_rows.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.usage_export_packages
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(
                self.offboarding_packages
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.grace_window_rows
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.org_switch_rows
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Usage-Export and Offboarding Packages, Grace-Window State, Org-Switch Semantics, and Deletion/Export Honesty\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Usage-export packages: {} | Offboarding packages: {} | Grace-window rows: {} | Org-switch rows: {}\n",
            self.section_qualifications.len(),
            self.usage_export_packages.len(),
            self.offboarding_packages.len(),
            self.grace_window_rows.len(),
            self.org_switch_rows.len(),
        ));
        out.push_str(&format!(
            "- Exact desktop handoff for every item: {}\n",
            yes_no(self.all_handoffs_exact())
        ));
        out.push_str(&format!(
            "- Local usage-export path available: {}\n",
            yes_no(self.usage_export_local_path_available())
        ));
        out.push_str(&format!(
            "- Local offboarding-package path available: {}\n",
            yes_no(self.offboarding_package_local_path_available())
        ));
        out.push_str(&format!(
            "- Export completeness honestly qualified: {}\n",
            yes_no(self.export_completeness_honestly_qualified())
        ));
        out.push_str(&format!(
            "- Deletion honestly labeled: {}\n",
            yes_no(self.deletion_honestly_labeled())
        ));
        out.push_str(&format!(
            "- Local work never stranded: {}\n",
            yes_no(self.local_work_never_stranded())
        ));
        out.push_str(&format!(
            "- Stale state honestly labeled: {}\n",
            yes_no(self.stale_state_honestly_labeled())
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        if self.degraded_labels.is_empty() {
            out.push_str("- Degraded: none\n");
        } else {
            let labels = self
                .degraded_labels
                .iter()
                .map(|reason| reason.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("- Degraded: {labels}\n"));
        }

        out.push_str("\n## Sections\n\n");
        for row in &self.section_qualifications {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}` [{}] (matrix lane `{}`)\n",
                row.section.as_str(),
                row.qualification.as_str(),
                row.rollout_stage.as_str(),
                row.read_write_scope.as_str(),
                row.matrix_lane_ref,
            ));
        }

        out.push_str("\n## Usage export packages\n\n");
        for item in &self.usage_export_packages {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.data_class.as_str(),
                item.availability.as_str(),
                item.completeness.as_str(),
                yes_no(item.claim_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Offboarding packages\n\n");
        for item in &self.offboarding_packages {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] (verified: {}) local_work_preserved `{}` {} ({}) → `{}` ({})\n",
                item.item_id,
                item.data_class.as_str(),
                item.availability.as_str(),
                item.completeness.as_str(),
                yes_no(item.claim_verified),
                yes_no(item.local_work_preserved),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Grace window\n\n");
        for item in &self.grace_window_rows {
            out.push_str(&format!(
                "- `{}` [{}/{}] reversible `{}` irreversible_labeled `{}` local_work_preserved `{}` {} ({}) → `{}` ({})\n",
                item.item_id,
                item.deletion_scope.as_str(),
                item.grace_posture.as_str(),
                yes_no(item.reversible),
                yes_no(item.irreversible_labeled),
                yes_no(item.local_work_preserved),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Org-switch semantics\n\n");
        for item in &self.org_switch_rows {
            out.push_str(&format!(
                "- `{}` [{}/{}] user_owned `{}` retained `{}` requires_admin `{}` {} ({}) → `{}` ({})\n",
                item.item_id,
                item.data_class.as_str(),
                item.disposition.as_str(),
                yes_no(item.user_owned),
                yes_no(item.user_owned_local_retained),
                yes_no(item.requires_admin),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

/// Errors emitted when reading the checked-in surface export.
#[derive(Debug)]
pub enum OffboardingArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<OffboardingViolation>),
}

impl fmt::Display for OffboardingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "usage-export/offboarding export parse failed: {error}"
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
                    "usage-export/offboarding export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for OffboardingArtifactError {}

/// Validation failures emitted by [`UsageExportOffboardingSurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OffboardingViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Projected surfaces list is empty.
    ProjectedSurfacesMissing,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required section qualification row is missing.
    RequiredSectionMissing,
    /// A section row's matrix lane ref does not match its section.
    SectionLaneMismatch,
    /// A section row's read/write scope does not match its bounded scope.
    SectionScopeMismatch,
    /// A section row is incomplete.
    SectionRowIncomplete,
    /// A section has no content items.
    SectionContentMissing,
    /// A read-only section item is not marked read-only.
    ReadOnlyScopeViolated,
    /// No local-first usage-export path is offered.
    LocalUsageExportPathMissing,
    /// No local-first offboarding-package path is offered.
    LocalOffboardingPathMissing,
    /// A completeness claim is verified-marked without backing verification.
    CompletenessClaimedButUnverified,
    /// An unverified completeness claim is not labeled.
    CompletenessClaimNotLabeled,
    /// A grace-window row's reversible flag does not match its posture.
    DeletionReversibilityMismatch,
    /// An irreversible (committed) deletion is not labeled.
    IrreversibleDeletionNotLabeled,
    /// An org-switch row's admin flag does not match its disposition.
    OrgSwitchAdminFlagMismatch,
    /// A user-owned data class is stranded with the prior org or not retained.
    UserLocalWorkStranded,
    /// An offboarding package or grace-window row does not preserve local work.
    LocalWorkStranded,
    /// An item is missing identity or a redacted body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// An item's desktop handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// The read/write scope contract is not fully satisfied.
    ScopeContractIncomplete,
    /// The deletion/export honesty contract is not fully satisfied.
    HonestyContractIncomplete,
    /// The stale-state honesty contract is not fully satisfied.
    StaleStateHonestyIncomplete,
    /// The continuity contract is not fully satisfied.
    ContinuityContractIncomplete,
    /// The locality disclosure is incomplete.
    LocalityDisclosureIncomplete,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl OffboardingViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ProjectedSurfacesMissing => "projected_surfaces_missing",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSectionMissing => "required_section_missing",
            Self::SectionLaneMismatch => "section_lane_mismatch",
            Self::SectionScopeMismatch => "section_scope_mismatch",
            Self::SectionRowIncomplete => "section_row_incomplete",
            Self::SectionContentMissing => "section_content_missing",
            Self::ReadOnlyScopeViolated => "read_only_scope_violated",
            Self::LocalUsageExportPathMissing => "local_usage_export_path_missing",
            Self::LocalOffboardingPathMissing => "local_offboarding_path_missing",
            Self::CompletenessClaimedButUnverified => "completeness_claimed_but_unverified",
            Self::CompletenessClaimNotLabeled => "completeness_claim_not_labeled",
            Self::DeletionReversibilityMismatch => "deletion_reversibility_mismatch",
            Self::IrreversibleDeletionNotLabeled => "irreversible_deletion_not_labeled",
            Self::OrgSwitchAdminFlagMismatch => "org_switch_admin_flag_mismatch",
            Self::UserLocalWorkStranded => "user_local_work_stranded",
            Self::LocalWorkStranded => "local_work_stranded",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::HonestyContractIncomplete => "honesty_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::ContinuityContractIncomplete => "continuity_contract_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable surface export.
///
/// This is the canonical reader: the desktop companion panel, the CLI/headless
/// surface, diagnostics, support-export, or Help/About surface calls it to ingest the
/// packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`OffboardingArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_stable_usage_export_offboarding_surface_export(
) -> Result<UsageExportOffboardingSurfacePacket, OffboardingArtifactError> {
    let packet: UsageExportOffboardingSurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho/support_export.json"
    )))
    .map_err(OffboardingArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(OffboardingArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        USAGE_EXPORT_OFFBOARDING_SCHEMA_REF.to_owned(),
        USAGE_EXPORT_OFFBOARDING_DOC_REF.to_owned(),
        M5_OFFBOARDING_CONTRACT_REF.to_owned(),
        M5_OFFBOARDING_RETENTION_MATRIX_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical read/write scope and authority contract with every guarantee met.
pub fn canonical_scope_contract() -> OffboardingScopeContract {
    OffboardingScopeContract {
        usage_export_read_only: true,
        offboarding_package_read_only: true,
        grace_window_read_only: true,
        org_switch_read_only: true,
        local_core_authoritative: true,
        action_applied_by_local_core_not_surface: true,
        no_unbounded_workspace_write: true,
        offboarding_never_strands_local_work: true,
        no_payload_bodies: true,
    }
}

/// Canonical deletion/export honesty contract with every guarantee satisfied.
pub fn canonical_honesty_contract() -> OffboardingHonestyContract {
    OffboardingHonestyContract {
        usage_export_local_path_always_available: true,
        offboarding_package_local_path_always_available: true,
        export_completeness_provable_or_labeled: true,
        deletion_scope_disclosed: true,
        deletion_reversible_within_grace_window: true,
        irreversible_deletion_labeled: true,
        org_switch_disposition_disclosed: true,
        user_owned_local_never_left_with_prior_org: true,
        no_claim_without_evidence: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> OffboardingStaleStateHonesty {
    OffboardingStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical continuity contract with every guarantee satisfied.
pub fn canonical_continuity_contract() -> OffboardingContinuityContract {
    OffboardingContinuityContract {
        local_core_continues_when_provider_degrades: true,
        degraded_capability_labeled_not_hidden: true,
        local_work_never_stranded: true,
        provider_and_admin_continuity_disclosed: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> OffboardingSecurityReview {
    OffboardingSecurityReview {
        usage_export_read_only: true,
        offboarding_package_read_only: true,
        grace_window_read_only: true,
        org_switch_read_only: true,
        local_core_authoritative: true,
        action_applied_by_local_core_not_surface: true,
        usage_export_local_path_always_available: true,
        offboarding_package_local_path_always_available: true,
        export_completeness_provable_or_labeled: true,
        deletion_scope_disclosed: true,
        deletion_reversible_within_grace_window: true,
        irreversible_deletion_labeled: true,
        user_owned_local_never_left_with_prior_org: true,
        local_work_never_stranded: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_credential_or_account_bodies_in_export: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every section projecting truth.
pub fn canonical_consumer_projection() -> OffboardingConsumerProjection {
    OffboardingConsumerProjection {
        desktop_panel_shows_usage_export: true,
        desktop_panel_shows_offboarding_package: true,
        desktop_panel_shows_grace_window: true,
        diagnostics_shows_org_switch_semantics: true,
        cli_headless_shows_export_and_deletion_state: true,
        support_export_shows_packages_and_grace_window: true,
        help_about_shows_deletion_and_export_honesty: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
///
/// All four sections inherit from the frozen M5 companion-matrix
/// `offboarding_continuity` lane. The usage-export-package and offboarding-package
/// sections earn the lane's Beta/staged-rollout qualification because a local-first
/// path is always available and local work is never stranded; the grace-window-state
/// and org-switch-semantics sections inherit the Preview/early-access qualification
/// because their managed and admin-dependent paths are less mature.
pub fn canonical_section_qualifications() -> Vec<OffboardingSectionQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;
    use OffboardingSection as Section;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        OffboardingSectionQualification {
            section: Section::UsageExportPackage,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::UsageExportPackage
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::OffboardingExportPreservesLocalWork,
        },
        OffboardingSectionQualification {
            section: Section::OffboardingPackage,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::OffboardingPackage
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::AdminContinuityRequired,
                Trigger::OffboardingStrandsLocalWork,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::OffboardingExportPreservesLocalWork,
        },
        OffboardingSectionQualification {
            section: Section::GraceWindowState,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::GraceWindowState.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::AdminContinuityRequired,
                Trigger::OffboardingStrandsLocalWork,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::OffboardingExportPreservesLocalWork,
        },
        OffboardingSectionQualification {
            section: Section::OrgSwitchSemantics,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::OrgSwitchSemantics
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::AdminContinuityRequired,
                Trigger::OffboardingStrandsLocalWork,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
    ]
}

/// Canonical locality disclosure for the surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "A local-first usage-export and offboarding-package path is always offered and always works; the local core is the authoritative source of truth, every package, grace-window row, and org-switch row stays inspectable offline, and user-owned local work is never stranded by an export, a deletion, or an org switch."
                .to_owned(),
        staged:
            "Provider-assembled export bundles, managed deletions, and managed org-switch migrations roll out per cohort and managed tenant and are visibly labeled until qualified."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Provider-assembled packages require the export assembler, committing a deletion requires the deletion service, and a managed-tenant org switch requires admin continuity; these claims are shown as proven only when verifiable. When a provider degrades the local path keeps the user working, an irreversible deletion is labeled, and the degraded capability is labeled, never hidden."
                .to_owned(),
    }
}

fn desktop_handoff(deep_link_ref: &str, requires_active_host: bool) -> CompanionDesktopHandoff {
    CompanionDesktopHandoff {
        target: CompanionHandoffTarget::ReviewPanel,
        deep_link_ref: deep_link_ref.to_owned(),
        resolution: CompanionHandoffResolution::Exact,
        requires_active_host,
    }
}

/// Canonical usage-export package items.
pub fn canonical_usage_export_packages() -> Vec<UsageExportPackageItem> {
    use CompanionFreshnessState as Fresh;
    use ExportCompleteness as Complete;
    use OffboardingArtifactClass as Class;
    use PackageAvailability as Avail;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        UsageExportPackageItem {
            item_id: "usage:0001".to_owned(),
            data_class: Class::UsageHistory,
            availability: Avail::LocalReady,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Full usage history exported locally now; complete, verified".to_owned(),
            record_ref: "export:usage-history:local".to_owned(),
            handoff: desktop_handoff("handoff:usage:0001", false),
        },
        UsageExportPackageItem {
            item_id: "usage:0002".to_owned(),
            data_class: Class::AuditTrail,
            availability: Avail::LocalReady,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Local activity-log export ready locally; complete, verified".to_owned(),
            record_ref: "export:activity-log:local".to_owned(),
            handoff: desktop_handoff("handoff:usage:0002", false),
        },
        UsageExportPackageItem {
            item_id: "usage:0003".to_owned(),
            data_class: Class::UsageHistory,
            availability: Avail::RequiresProviderAssembly,
            completeness: Complete::CompleteUnverified,
            claim_verified: false,
            proof_label_shown: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary: "Provider-assembled billing usage; completeness not yet verified; labeled"
                .to_owned(),
            record_ref: "export:billing-usage:provider".to_owned(),
            handoff: desktop_handoff("handoff:usage:0003", false),
        },
    ]
}

/// Canonical offboarding package items.
pub fn canonical_offboarding_packages() -> Vec<OffboardingPackageItem> {
    use CompanionFreshnessState as Fresh;
    use ExportCompleteness as Complete;
    use OffboardingArtifactClass as Class;
    use PackageAvailability as Avail;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        OffboardingPackageItem {
            item_id: "off:0001".to_owned(),
            data_class: Class::LocalWorkspace,
            availability: Avail::LocalReady,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            local_work_preserved: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Local workspace and edit history packaged locally now; complete, verified; local work retained"
                    .to_owned(),
            record_ref: "offboarding:local-workspace".to_owned(),
            handoff: desktop_handoff("handoff:off:0001", false),
        },
        OffboardingPackageItem {
            item_id: "off:0002".to_owned(),
            data_class: Class::ManagedSnapshots,
            availability: Avail::LocalStaging,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            local_work_preserved: true,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Snapshot archive staging locally from the local core; complete, verified; local work retained"
                    .to_owned(),
            record_ref: "offboarding:managed-snapshots".to_owned(),
            handoff: desktop_handoff("handoff:off:0002", false),
        },
        OffboardingPackageItem {
            item_id: "off:0003".to_owned(),
            data_class: Class::ManagedProfile,
            availability: Avail::RequiresProviderAssembly,
            completeness: Complete::CompleteUnverified,
            claim_verified: false,
            proof_label_shown: true,
            local_work_preserved: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary:
                "Managed profile/settings bundle requires provider assembly; completeness not yet verified; labeled; local work retained"
                    .to_owned(),
            record_ref: "offboarding:managed-profile".to_owned(),
            handoff: desktop_handoff("handoff:off:0003", false),
        },
    ]
}

/// Canonical grace-window-state items.
pub fn canonical_grace_window_rows() -> Vec<GraceWindowStateItem> {
    use CompanionFreshnessState as Fresh;
    use DeletionScope as Scope;
    use GraceWindowPosture as Posture;
    use OffboardingArtifactClass as Class;

    let rw = CompanionReadWriteScope::ReadOnly;
    vec![
        GraceWindowStateItem {
            item_id: "grace:0001".to_owned(),
            data_class: Class::ManagedProfile,
            deletion_scope: Scope::ManagedOnlyLocalRetained,
            grace_posture: Posture::OpenReversible,
            reversible: true,
            irreversible_labeled: false,
            local_work_preserved: true,
            freshness: Fresh::Live,
            read_write_scope: rw,
            stale_label_shown: false,
            summary:
                "Managed profile deletion scheduled; grace window open and reversible; local work retained"
                    .to_owned(),
            record_ref: "deletion:managed-profile".to_owned(),
            handoff: desktop_handoff("handoff:grace:0001", false),
        },
        GraceWindowStateItem {
            item_id: "grace:0002".to_owned(),
            data_class: Class::ManagedSnapshots,
            deletion_scope: Scope::LocalOnly,
            grace_posture: Posture::ClosingReversible,
            reversible: true,
            irreversible_labeled: false,
            local_work_preserved: true,
            freshness: Fresh::Cached,
            read_write_scope: rw,
            stale_label_shown: false,
            summary:
                "Local managed-mirror cache clear scheduled; window closing, still reversible; original local work retained"
                    .to_owned(),
            record_ref: "deletion:local-cache".to_owned(),
            handoff: desktop_handoff("handoff:grace:0002", false),
        },
        GraceWindowStateItem {
            item_id: "grace:0003".to_owned(),
            data_class: Class::AuditTrail,
            deletion_scope: Scope::ManagedOnlyLocalRetained,
            grace_posture: Posture::CommittedIrreversible,
            reversible: false,
            irreversible_labeled: true,
            local_work_preserved: true,
            freshness: Fresh::Live,
            read_write_scope: rw,
            stale_label_shown: false,
            summary:
                "Managed audit-trail deletion committed and irreversible; clearly labeled; local work retained"
                    .to_owned(),
            record_ref: "deletion:managed-audit-trail".to_owned(),
            handoff: desktop_handoff("handoff:grace:0003", false),
        },
    ]
}

/// Canonical org-switch-semantics items.
pub fn canonical_org_switch_rows() -> Vec<OrgSwitchSemanticsItem> {
    use CompanionFreshnessState as Fresh;
    use OffboardingArtifactClass as Class;
    use OrgSwitchDisposition as Disposition;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        OrgSwitchSemanticsItem {
            item_id: "org:0001".to_owned(),
            data_class: Class::LocalWorkspace,
            disposition: Disposition::StaysLocalToUser,
            user_owned: true,
            user_owned_local_retained: true,
            requires_admin: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Local workspace stays with the user across an org switch".to_owned(),
            record_ref: "org-switch:local-workspace".to_owned(),
            handoff: desktop_handoff("handoff:org:0001", false),
        },
        OrgSwitchSemanticsItem {
            item_id: "org:0002".to_owned(),
            data_class: Class::ManagedProfile,
            disposition: Disposition::MigratesWithUser,
            user_owned: false,
            user_owned_local_retained: true,
            requires_admin: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Managed profile migrates with the user to the new org".to_owned(),
            record_ref: "org-switch:managed-profile".to_owned(),
            handoff: desktop_handoff("handoff:org:0002", false),
        },
        OrgSwitchSemanticsItem {
            item_id: "org:0003".to_owned(),
            data_class: Class::ManagedSnapshots,
            disposition: Disposition::RequiresAdminApproval,
            user_owned: false,
            user_owned_local_retained: true,
            requires_admin: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary: "Managed snapshot migration requires prior-org admin approval; labeled"
                .to_owned(),
            record_ref: "org-switch:managed-snapshots".to_owned(),
            handoff: desktop_handoff("handoff:org:0003", false),
        },
        OrgSwitchSemanticsItem {
            item_id: "org:0004".to_owned(),
            data_class: Class::AuditTrail,
            disposition: Disposition::LeftWithPriorOrg,
            user_owned: false,
            user_owned_local_retained: true,
            requires_admin: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Prior-org audit trail stays with the prior org by policy".to_owned(),
            record_ref: "org-switch:audit-trail".to_owned(),
            handoff: desktop_handoff("handoff:org:0004", false),
        },
    ]
}

/// Builds the canonical surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export and
/// Markdown summary are generated from, so the artifact never drifts from the typed
/// section, item, scope, honesty, continuity, and freshness definitions.
pub fn canonical_usage_export_offboarding_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: OffboardingProofFreshness,
) -> UsageExportOffboardingSurfacePacket {
    UsageExportOffboardingSurfacePacket::new(UsageExportOffboardingSurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::CliHeadless,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
            M5CompanionConsumerSurface::HelpAbout,
        ],
        section_qualifications: canonical_section_qualifications(),
        usage_export_packages: canonical_usage_export_packages(),
        offboarding_packages: canonical_offboarding_packages(),
        grace_window_rows: canonical_grace_window_rows(),
        org_switch_rows: canonical_org_switch_rows(),
        scope_contract: canonical_scope_contract(),
        honesty_contract: canonical_honesty_contract(),
        stale_state_honesty: canonical_stale_state_honesty(),
        continuity_contract: canonical_continuity_contract(),
        locality_disclosure: canonical_locality_disclosure(),
        security_review: canonical_security_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

fn validate_source_contracts(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        USAGE_EXPORT_OFFBOARDING_SCHEMA_REF,
        USAGE_EXPORT_OFFBOARDING_DOC_REF,
        M5_OFFBOARDING_CONTRACT_REF,
        M5_OFFBOARDING_RETENTION_MATRIX_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(OffboardingViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let present: BTreeSet<OffboardingSection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in OffboardingSection::ALL {
        if !present.contains(&required) {
            violations.push(OffboardingViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(OffboardingViolation::SectionLaneMismatch);
        }
        if row.read_write_scope != row.section.bounded_scope() {
            violations.push(OffboardingViolation::SectionScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(OffboardingViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    if packet.usage_export_packages.is_empty()
        || packet.offboarding_packages.is_empty()
        || packet.grace_window_rows.is_empty()
        || packet.org_switch_rows.is_empty()
    {
        violations.push(OffboardingViolation::SectionContentMissing);
    }

    if !packet.usage_export_local_path_available() {
        violations.push(OffboardingViolation::LocalUsageExportPathMissing);
    }
    if !packet.offboarding_package_local_path_available() {
        violations.push(OffboardingViolation::LocalOffboardingPathMissing);
    }

    for item in &packet.usage_export_packages {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(OffboardingViolation::ReadOnlyScopeViolated);
        }
        if item.completeness.is_complete_claim() && !item.claim_verified {
            violations.push(OffboardingViolation::CompletenessClaimedButUnverified);
        }
        if item.completeness == ExportCompleteness::CompleteUnverified && !item.proof_label_shown {
            violations.push(OffboardingViolation::CompletenessClaimNotLabeled);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(OffboardingViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.offboarding_packages {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(OffboardingViolation::ReadOnlyScopeViolated);
        }
        if item.completeness.is_complete_claim() && !item.claim_verified {
            violations.push(OffboardingViolation::CompletenessClaimedButUnverified);
        }
        if item.completeness == ExportCompleteness::CompleteUnverified && !item.proof_label_shown {
            violations.push(OffboardingViolation::CompletenessClaimNotLabeled);
        }
        if !item.local_work_preserved {
            violations.push(OffboardingViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(OffboardingViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.grace_window_rows {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(OffboardingViolation::ReadOnlyScopeViolated);
        }
        if item.reversible != item.grace_posture.is_reversible() {
            violations.push(OffboardingViolation::DeletionReversibilityMismatch);
        }
        if item.grace_posture.is_committed() && !item.irreversible_labeled {
            violations.push(OffboardingViolation::IrreversibleDeletionNotLabeled);
        }
        if !item.local_work_preserved {
            violations.push(OffboardingViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(OffboardingViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.org_switch_rows {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(OffboardingViolation::ReadOnlyScopeViolated);
        }
        if item.requires_admin != item.disposition.requires_admin() {
            violations.push(OffboardingViolation::OrgSwitchAdminFlagMismatch);
        }
        if item.user_owned
            && (!item.user_owned_local_retained || item.disposition.left_with_prior_org())
        {
            violations.push(OffboardingViolation::UserLocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(OffboardingViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<OffboardingViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(OffboardingViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(handoff: &CompanionDesktopHandoff, violations: &mut Vec<OffboardingViolation>) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(OffboardingViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.usage_export_read_only,
        contract.offboarding_package_read_only,
        contract.grace_window_read_only,
        contract.org_switch_read_only,
        contract.local_core_authoritative,
        contract.action_applied_by_local_core_not_surface,
        contract.no_unbounded_workspace_write,
        contract.offboarding_never_strands_local_work,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(OffboardingViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_honesty_contract(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let contract = &packet.honesty_contract;
    for ok in [
        contract.usage_export_local_path_always_available,
        contract.offboarding_package_local_path_always_available,
        contract.export_completeness_provable_or_labeled,
        contract.deletion_scope_disclosed,
        contract.deletion_reversible_within_grace_window,
        contract.irreversible_deletion_labeled,
        contract.org_switch_disposition_disclosed,
        contract.user_owned_local_never_left_with_prior_org,
        contract.no_claim_without_evidence,
    ] {
        if !ok {
            violations.push(OffboardingViolation::HonestyContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(OffboardingViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_continuity_contract(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let contract = &packet.continuity_contract;
    for ok in [
        contract.local_core_continues_when_provider_degrades,
        contract.degraded_capability_labeled_not_hidden,
        contract.local_work_never_stranded,
        contract.provider_and_admin_continuity_disclosed,
    ] {
        if !ok {
            violations.push(OffboardingViolation::ContinuityContractIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(OffboardingViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.usage_export_read_only,
        review.offboarding_package_read_only,
        review.grace_window_read_only,
        review.org_switch_read_only,
        review.local_core_authoritative,
        review.action_applied_by_local_core_not_surface,
        review.usage_export_local_path_always_available,
        review.offboarding_package_local_path_always_available,
        review.export_completeness_provable_or_labeled,
        review.deletion_scope_disclosed,
        review.deletion_reversible_within_grace_window,
        review.irreversible_deletion_labeled,
        review.user_owned_local_never_left_with_prior_org,
        review.local_work_never_stranded,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_credential_or_account_bodies_in_export,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(OffboardingViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_panel_shows_usage_export,
        projection.desktop_panel_shows_offboarding_package,
        projection.desktop_panel_shows_grace_window,
        projection.diagnostics_shows_org_switch_semantics,
        projection.cli_headless_shows_export_and_deletion_state,
        projection.support_export_shows_packages_and_grace_window,
        projection.help_about_shows_deletion_and_export_honesty,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(OffboardingViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &UsageExportOffboardingSurfacePacket,
    violations: &mut Vec<OffboardingViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(OffboardingViolation::ProofFreshnessIncomplete);
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
