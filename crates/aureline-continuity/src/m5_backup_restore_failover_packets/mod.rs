//! Backup, restore, and failover continuity packets with typed drill evidence.
//!
//! This module turns backup/restore/failover proof into first-class continuity
//! evidence instead of implicit ops lore. Every claimed managed, self-hosted, or
//! sovereign surface that carries resilience language must point to one typed
//! [`BackupRestoreFailoverPacketEntry`] that answers the same questions
//! everywhere:
//!
//! 1. Which backup, restore, failover, or snapshot/replication packet family
//!    backs the claim, and which claim row does it back?
//! 2. What was actually exercised in the most recent drill, and — when the drill
//!    was only partial — what restored *narrower than normal* or was not
//!    exercised at all?
//! 3. What restore identity does a recovery reproduce, and what partial loss is
//!    disclosed on recovery?
//! 4. On what cadence is the packet drilled, who owns the drill now and next,
//!    when was it last drilled, and when does its evidence age out under the
//!    freshness SLO?
//!
//! The descriptor is projected identically onto every claimed surface
//! (release-center, shiproom, support-center, partner qualification, and public
//! claim-manifest generation) through a
//! [`BackupRestoreFailoverSurfaceProjection`], so the exact restore-identity and
//! partial-loss vocabulary stays byte-identical everywhere instead of drifting
//! per surface.
//!
//! Two guardrails are load-bearing:
//!
//! - Generic "DR tested" text is **not** accepted as a substitute for typed
//!   packet evidence. A packet that sets [`generic_dr_text_only`] fails closed —
//!   its claim is withdrawn — and a managed-family packet that exercised nothing
//!   narrows automatically.
//!   [`generic_dr_text_only`]: BackupRestoreFailoverPacketEntry::generic_dr_text_only
//! - A partial drill **may not omit** what restored narrower than normal or what
//!   was not exercised: a packet whose scope is partial without an explicit
//!   not-exercised disclosure narrows automatically.
//!
//! The [`DrillPacketRegistry`] is the typed consumer the release-center,
//! shiproom, support-center, partner-qualification, and public claim-manifest
//! surfaces read. It indexes packets by claim row and family and reports, per
//! claimed resilience row, whether a current packet backs the claim — so any
//! affected managed/self-hosted claim row narrows automatically when drill
//! cadence, owner, or restore-identity/partial-loss evidence is missing or stale.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, UTC timestamps, and opaque refs. Raw backup bytes, raw
//! provider payloads, raw hostnames, raw KMS handles, and secret material never
//! cross this boundary.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ContinuityClaimQualificationClass, ContinuityPacketFamilyClass, ContinuityProfileClass,
    DrillCadenceClass, DrillEvidenceStateClass, PartialLossClass, RestoreFailoverHostingClass,
    RestoreIdentityClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF: &str =
    "continuity:m5_backup_restore_failover_packets:v1";

/// Record-kind tag for [`BackupRestoreFailoverPage`] payloads.
pub const BACKUP_RESTORE_FAILOVER_PAGE_RECORD_KIND: &str = "backup_restore_failover_page_record";

/// Record-kind tag for [`BackupRestoreFailoverSummary`] payloads.
pub const BACKUP_RESTORE_FAILOVER_SUMMARY_RECORD_KIND: &str =
    "backup_restore_failover_summary_record";

/// Record-kind tag for [`BackupRestoreFailoverDescriptor`] payloads.
pub const BACKUP_RESTORE_FAILOVER_DESCRIPTOR_RECORD_KIND: &str =
    "backup_restore_failover_descriptor_record";

/// Record-kind tag for [`BackupRestoreFailoverSurfaceProjection`] payloads.
pub const BACKUP_RESTORE_FAILOVER_SURFACE_PROJECTION_RECORD_KIND: &str =
    "backup_restore_failover_surface_projection_record";

/// Record-kind tag for [`BackupRestoreFailoverOutcome`] payloads.
pub const BACKUP_RESTORE_FAILOVER_OUTCOME_RECORD_KIND: &str =
    "backup_restore_failover_outcome_record";

/// Record-kind tag for [`BackupRestoreFailoverDefect`] payloads.
pub const BACKUP_RESTORE_FAILOVER_DEFECT_RECORD_KIND: &str =
    "backup_restore_failover_defect_record";

/// Record-kind tag for [`DrillPacketRegistry`] payloads.
pub const DRILL_PACKET_REGISTRY_RECORD_KIND: &str = "drill_packet_registry_record";

/// Record-kind tag for [`ClaimCoverageRow`] payloads.
pub const CLAIM_COVERAGE_ROW_RECORD_KIND: &str = "claim_coverage_row_record";

/// Record-kind tag for [`BackupRestoreFailoverSupportExport`] payloads.
pub const BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "backup_restore_failover_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const BACKUP_RESTORE_FAILOVER_DOC_REF: &str =
    "docs/m5/continuity/backup-restore-failover-packets.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const BACKUP_RESTORE_FAILOVER_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/backup_restore_failover_packets.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const BACKUP_RESTORE_FAILOVER_SCHEMA_REF: &str =
    "schemas/continuity/backup_restore_failover_packet.schema.json";

/// One operation a backup/restore/failover drill can exercise.
///
/// The point of an explicit operation set is to make "what was exercised" a
/// typed fact rather than narrative text: a packet that exercised nothing is
/// distinguishable from one that exercised a full backup-through-failover path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreOperationClass {
    /// A backup or snapshot was captured.
    BackupCapture,
    /// A captured backup was integrity-verified.
    BackupIntegrityVerify,
    /// A restore was executed from a backup.
    RestoreExecute,
    /// A restored result was integrity-verified.
    RestoreIntegrityVerify,
    /// A failover cutover to a standby was performed.
    FailoverCutover,
    /// A failback to the primary was performed.
    Failback,
}

impl RestoreOperationClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackupCapture => "backup_capture",
            Self::BackupIntegrityVerify => "backup_integrity_verify",
            Self::RestoreExecute => "restore_execute",
            Self::RestoreIntegrityVerify => "restore_integrity_verify",
            Self::FailoverCutover => "failover_cutover",
            Self::Failback => "failback",
        }
    }

    /// Plain-language label naming the operation.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::BackupCapture => "backup capture",
            Self::BackupIntegrityVerify => "backup integrity verify",
            Self::RestoreExecute => "restore execute",
            Self::RestoreIntegrityVerify => "restore integrity verify",
            Self::FailoverCutover => "failover cutover",
            Self::Failback => "failback",
        }
    }
}

/// How completely the most recent drill exercised the packet's path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeExercisedClass {
    /// The full backup-through-restore/failover path was exercised.
    FullyExercised,
    /// Only part of the path was exercised; the rest must be disclosed.
    PartiallyExercised,
    /// Nothing was exercised in the most recent drill.
    NotExercised,
}

impl ScopeExercisedClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyExercised => "fully_exercised",
            Self::PartiallyExercised => "partially_exercised",
            Self::NotExercised => "not_exercised",
        }
    }

    /// Plain-language summary of the exercised scope.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::FullyExercised => "fully exercised",
            Self::PartiallyExercised => "partially exercised",
            Self::NotExercised => "not exercised",
        }
    }

    /// True when this scope must disclose what restored narrower than normal.
    pub const fn requires_not_exercised_disclosure(self) -> bool {
        matches!(self, Self::PartiallyExercised)
    }
}

/// Surface a backup/restore/failover descriptor is projected onto.
///
/// These are exactly the surfaces that reuse the packet family: the release
/// center, shiproom readiness dashboard, support center, partner qualification,
/// and public claim-manifest generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketSurfaceClass {
    /// The release-center readiness surface.
    ReleaseCenter,
    /// The shiproom readiness dashboard.
    Shiproom,
    /// The support-center export surface.
    SupportCenter,
    /// Partner qualification packets.
    PartnerQualification,
    /// Public claim-manifest generation.
    PublicClaimManifest,
}

impl PacketSurfaceClass {
    /// Every surface in canonical projection order.
    pub const ALL: [PacketSurfaceClass; 5] = [
        Self::ReleaseCenter,
        Self::Shiproom,
        Self::SupportCenter,
        Self::PartnerQualification,
        Self::PublicClaimManifest,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::SupportCenter => "support_center",
            Self::PartnerQualification => "partner_qualification",
            Self::PublicClaimManifest => "public_claim_manifest",
        }
    }
}

/// Typed reason a backup/restore/failover claim narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// The packet relies on generic "DR tested" text instead of typed evidence.
    GenericDrTextOnly,
    /// A managed-family packet exercised nothing in its most recent drill.
    ScopeNotExercised,
    /// A partial drill does not disclose what restored narrower than normal.
    NotExercisedDisclosureMissing,
    /// A managed-family packet does not declare the restore identity recovery reproduces.
    RestoreIdentityUndeclared,
    /// A packet does not disclose its partial-loss behavior on recovery.
    PartialLossUndisclosed,
    /// A managed-family packet names no usable drill cadence.
    DrillCadenceMissing,
    /// A managed-family packet names no current or future drill owner.
    DrillOwnerMissing,
    /// Drill evidence is stale (or its freshness SLO is undeclared) and a fresh drill is required.
    DrillEvidenceStale,
    /// The continuity drill has never been run.
    DrillNeverRun,
    /// A self-hosted or sovereign packet hides a vendor-operated restore/failover lane.
    SovereignContinuityOverclaimed,
    /// The claimed profile is inconsistent with its restore/failover hosting.
    ProfileMismatch,
    /// A surface renders different restore-identity or partial-loss vocabulary than the descriptor.
    PacketVocabularyDrift,
    /// A packet is not projected onto every required surface.
    SurfaceReuseIncomplete,
    /// A claimed resilience row has no backup/restore/failover packet at all.
    PacketEvidenceMissing,
}

impl PacketNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::GenericDrTextOnly => "generic_dr_text_only",
            Self::ScopeNotExercised => "scope_not_exercised",
            Self::NotExercisedDisclosureMissing => "not_exercised_disclosure_missing",
            Self::RestoreIdentityUndeclared => "restore_identity_undeclared",
            Self::PartialLossUndisclosed => "partial_loss_undisclosed",
            Self::DrillCadenceMissing => "drill_cadence_missing",
            Self::DrillOwnerMissing => "drill_owner_missing",
            Self::DrillEvidenceStale => "drill_evidence_stale",
            Self::DrillNeverRun => "drill_never_run",
            Self::SovereignContinuityOverclaimed => "sovereign_continuity_overclaimed",
            Self::ProfileMismatch => "profile_mismatch",
            Self::PacketVocabularyDrift => "packet_vocabulary_drift",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::PacketEvidenceMissing => "packet_evidence_missing",
        }
    }

    /// True when this reason withdraws the claim immediately (fails closed).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::GenericDrTextOnly | Self::SovereignContinuityOverclaimed
        )
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::ScopeNotExercised
                | Self::DrillNeverRun
                | Self::ProfileMismatch
                | Self::PacketVocabularyDrift
                | Self::PacketEvidenceMissing
        )
    }
}

/// Coverage state of a claimed resilience row, derived from its packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimCoverageClass {
    /// A current packet backs the claim.
    CurrentPacket,
    /// A packet backs the claim but its evidence is stale and must be refreshed.
    StalePacketNeedsRefresh,
    /// A packet backs the claim but its claim is withheld (fails closed).
    PacketWithheld,
    /// No backup/restore/failover packet backs the claim at all.
    NoPacket,
}

impl ClaimCoverageClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentPacket => "current_packet",
            Self::StalePacketNeedsRefresh => "stale_packet_needs_refresh",
            Self::PacketWithheld => "packet_withheld",
            Self::NoPacket => "no_packet",
        }
    }

    /// True when the claim is backed by a current, drillable packet.
    pub const fn is_covered(self) -> bool {
        matches!(self, Self::CurrentPacket)
    }
}

/// Derives a qualification from the packet narrow reasons present.
fn qualification_from_reasons<'a>(
    reasons: impl IntoIterator<Item = &'a PacketNarrowReasonClass>,
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        saw_any = true;
        if reason.is_withdrawal_reason() {
            return ContinuityClaimQualificationClass::Withdrawn;
        }
        if reason.is_preview_reason() {
            saw_preview = true;
        }
    }
    if saw_preview {
        ContinuityClaimQualificationClass::Preview
    } else if saw_any {
        ContinuityClaimQualificationClass::Beta
    } else {
        ContinuityClaimQualificationClass::Stable
    }
}

/// What a drill exercised, and what it did not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreScope {
    /// How completely the path was exercised.
    pub scope_exercised: ScopeExercisedClass,
    /// Stable token for [`Self::scope_exercised`].
    pub scope_exercised_token: String,
    /// Operations exercised in the most recent drill.
    pub exercised_operations: Vec<RestoreOperationClass>,
    /// Stable tokens for [`Self::exercised_operations`].
    pub exercised_operation_tokens: Vec<String>,
    /// Export-safe disclosure of what restored narrower than normal or was not exercised.
    pub not_exercised_note: String,
}

impl RestoreScope {
    /// Builds a restore-scope record, computing its tokens.
    pub fn new(
        scope_exercised: ScopeExercisedClass,
        exercised_operations: Vec<RestoreOperationClass>,
        not_exercised_note: impl Into<String>,
    ) -> Self {
        let exercised_operation_tokens = exercised_operations
            .iter()
            .map(|op| op.as_str().to_owned())
            .collect();
        Self {
            scope_exercised,
            scope_exercised_token: scope_exercised.as_str().to_owned(),
            exercised_operations,
            exercised_operation_tokens,
            not_exercised_note: not_exercised_note.into(),
        }
    }

    /// True when an actual operation set was exercised.
    pub fn is_exercised(&self) -> bool {
        self.scope_exercised != ScopeExercisedClass::NotExercised
            && !self.exercised_operations.is_empty()
    }

    /// True when this scope must disclose what restored narrower than normal but does not.
    pub fn missing_not_exercised_disclosure(&self) -> bool {
        self.scope_exercised.requires_not_exercised_disclosure()
            && self.not_exercised_note.trim().is_empty()
    }

    /// Canonical one-line exercised/not-exercised summary.
    pub fn scope_line(&self) -> String {
        let exercised = if self.exercised_operations.is_empty() {
            "nothing".to_owned()
        } else {
            self.exercised_operations
                .iter()
                .map(|op| op.plain())
                .collect::<Vec<_>>()
                .join(", ")
        };
        let not_exercised = if self.not_exercised_note.trim().is_empty() {
            String::new()
        } else {
            format!(" Not exercised: {}", self.not_exercised_note.trim())
        };
        format!(
            "Scope {}; exercised: {}.{}",
            self.scope_exercised.plain(),
            exercised,
            not_exercised
        )
    }
}

/// Drill cadence, ownership, freshness, and evidence for a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillEvidence {
    /// Cadence at which the packet is drilled.
    pub cadence: DrillCadenceClass,
    /// Stable token for [`Self::cadence`].
    pub cadence_token: String,
    /// Freshness state of the drill evidence.
    pub evidence_state: DrillEvidenceStateClass,
    /// Stable token for [`Self::evidence_state`].
    pub evidence_state_token: String,
    /// Export-safe label naming the current drill owner.
    pub current_owner_label: String,
    /// Export-safe label naming the future or backup drill owner.
    pub future_owner_label: String,
    /// UTC timestamp of the last successful drill, empty when never run.
    pub last_drill_at: String,
    /// UTC timestamp when the evidence ages out under the freshness SLO.
    pub evidence_expires_at: String,
    /// Opaque ref to the drill evidence; never a raw backup body.
    pub drill_evidence_ref: String,
}

impl DrillEvidence {
    /// Builds a drill-evidence record, computing its tokens.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cadence: DrillCadenceClass,
        evidence_state: DrillEvidenceStateClass,
        current_owner_label: impl Into<String>,
        future_owner_label: impl Into<String>,
        last_drill_at: impl Into<String>,
        evidence_expires_at: impl Into<String>,
        drill_evidence_ref: impl Into<String>,
    ) -> Self {
        Self {
            cadence,
            cadence_token: cadence.as_str().to_owned(),
            evidence_state,
            evidence_state_token: evidence_state.as_str().to_owned(),
            current_owner_label: current_owner_label.into(),
            future_owner_label: future_owner_label.into(),
            last_drill_at: last_drill_at.into(),
            evidence_expires_at: evidence_expires_at.into(),
            drill_evidence_ref: drill_evidence_ref.into(),
        }
    }

    /// True when both a current and a future drill owner are named.
    pub fn has_named_owners(&self) -> bool {
        !self.current_owner_label.trim().is_empty() && !self.future_owner_label.trim().is_empty()
    }

    /// True when current/graced evidence is missing its required freshness timestamps.
    pub fn missing_freshness_window(&self) -> bool {
        self.evidence_state.requires_last_drill_timestamp()
            && (self.last_drill_at.trim().is_empty() || self.evidence_expires_at.trim().is_empty())
    }
}

/// One claimed backup/restore/failover packet decorated with its drill facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverPacketEntry {
    /// Opaque packet identifier.
    pub packet_id: String,
    /// Opaque id of the continuity-claim row this packet backs.
    pub claim_row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// Named continuity packet family backing the claim.
    pub packet_family: ContinuityPacketFamilyClass,
    /// Stable token for [`Self::packet_family`].
    pub packet_family_token: String,
    /// True when the claim row this packet backs carries resilience language.
    pub backs_resilience_claim: bool,
    /// What the most recent drill exercised, and what it did not.
    pub restore_scope: RestoreScope,
    /// Identity a successful restore or failover reproduces.
    pub restore_identity: RestoreIdentityClass,
    /// Stable token for [`Self::restore_identity`].
    pub restore_identity_token: String,
    /// Where the restore or failover path executes.
    pub restore_failover_hosting: RestoreFailoverHostingClass,
    /// Stable token for [`Self::restore_failover_hosting`].
    pub restore_failover_hosting_token: String,
    /// True when any external restore/failover dependency is disclosed.
    pub external_dependency_disclosed: bool,
    /// Partial-loss disclosure class.
    pub partial_loss: PartialLossClass,
    /// Stable token for [`Self::partial_loss`].
    pub partial_loss_token: String,
    /// Export-safe note describing the partial-loss boundary.
    pub partial_loss_note: String,
    /// Drill cadence, ownership, freshness, and evidence.
    pub drill: DrillEvidence,
    /// True when the packet relies on generic "DR tested" text instead of typed evidence.
    pub generic_dr_text_only: bool,
    /// Surfaces this packet is projected onto.
    pub projected_surfaces: Vec<PacketSurfaceClass>,
}

impl BackupRestoreFailoverPacketEntry {
    /// Builds a backup/restore/failover packet entry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        packet_id: impl Into<String>,
        claim_row_id: impl Into<String>,
        surface_label: impl Into<String>,
        profile_class: ContinuityProfileClass,
        packet_family: ContinuityPacketFamilyClass,
        backs_resilience_claim: bool,
        restore_scope: RestoreScope,
        restore_identity: RestoreIdentityClass,
        restore_failover_hosting: RestoreFailoverHostingClass,
        external_dependency_disclosed: bool,
        partial_loss: PartialLossClass,
        partial_loss_note: impl Into<String>,
        drill: DrillEvidence,
        generic_dr_text_only: bool,
        projected_surfaces: Vec<PacketSurfaceClass>,
    ) -> Self {
        Self {
            packet_id: packet_id.into(),
            claim_row_id: claim_row_id.into(),
            surface_label: surface_label.into(),
            profile_class,
            profile_class_token: profile_class.as_str().to_owned(),
            packet_family,
            packet_family_token: packet_family.as_str().to_owned(),
            backs_resilience_claim,
            restore_scope,
            restore_identity,
            restore_identity_token: restore_identity.as_str().to_owned(),
            restore_failover_hosting,
            restore_failover_hosting_token: restore_failover_hosting.as_str().to_owned(),
            external_dependency_disclosed,
            partial_loss,
            partial_loss_token: partial_loss.as_str().to_owned(),
            partial_loss_note: partial_loss_note.into(),
            drill,
            generic_dr_text_only,
            projected_surfaces,
        }
    }

    /// Surfaces this packet is required to reach (every surface).
    pub fn required_surfaces(&self) -> &'static [PacketSurfaceClass] {
        &PacketSurfaceClass::ALL
    }

    /// True when this packet is held to managed-continuity evidence requirements.
    ///
    /// A managed continuity packet family (backup, restore, failover, or
    /// snapshot/replication) must carry typed drill evidence. A pure local-core
    /// continuity family is exempt.
    pub fn requires_managed_evidence(&self) -> bool {
        self.packet_family.is_managed_family()
    }

    /// True when a self-governed profile hides a vendor-operated restore/failover lane.
    pub fn overclaims_self_governed_continuity(&self) -> bool {
        self.profile_class.is_self_governed()
            && self.restore_failover_hosting == RestoreFailoverHostingClass::VendorOperated
            && !self.external_dependency_disclosed
    }

    /// Returns a profile-vs-hosting mismatch note when one applies.
    pub fn profile_hosting_mismatch(&self) -> Option<&'static str> {
        if self.profile_class == ContinuityProfileClass::Managed
            && self.restore_failover_hosting == RestoreFailoverHostingClass::LocalCore
        {
            Some("a managed packet cannot claim purely local-core restore or failover continuity")
        } else if self.profile_class == ContinuityProfileClass::LocalOnly
            && self.packet_family.is_managed_family()
            && self.restore_failover_hosting != RestoreFailoverHostingClass::LocalCore
        {
            Some("a local-only packet cannot host its restore or failover outside the local core")
        } else {
            None
        }
    }
}

/// Plain-language descriptor for one backup/restore/failover packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Packet this descriptor describes.
    pub packet_id: String,
    /// Claim row this packet backs.
    pub claim_row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// Plain-language claimed profile.
    pub profile_class_plain: String,
    /// Stable token for the packet family.
    pub packet_family_token: String,
    /// Plain-language packet family.
    pub packet_family_plain: String,
    /// Stable token for the exercised scope.
    pub scope_exercised_token: String,
    /// Plain-language exercised scope.
    pub scope_exercised_plain: String,
    /// Stable token for the restore identity.
    pub restore_identity_token: String,
    /// Plain-language restore identity.
    pub restore_identity_plain: String,
    /// Stable token for the partial-loss class.
    pub partial_loss_token: String,
    /// Plain-language partial-loss class.
    pub partial_loss_plain: String,
    /// Stable token for the restore/failover hosting.
    pub restore_failover_hosting_token: String,
    /// Stable token for the drill cadence.
    pub cadence_token: String,
    /// Stable token for the drill-evidence freshness state.
    pub evidence_state_token: String,
    /// True when the packet evidence is current or reconstructable.
    pub evidence_current: bool,
    /// Canonical one-line scope summary reused by every surface projection.
    pub scope_line: String,
    /// Canonical one-line restore-identity summary reused by every surface projection.
    pub restore_identity_line: String,
    /// Canonical one-line partial-loss summary reused by every surface projection.
    pub partial_loss_line: String,
    /// Canonical one-line drill summary reused by every surface projection.
    pub drill_line: String,
}

impl BackupRestoreFailoverDescriptor {
    /// Builds a descriptor from a decorated packet entry.
    pub fn from_entry(entry: &BackupRestoreFailoverPacketEntry) -> Self {
        Self {
            record_kind: BACKUP_RESTORE_FAILOVER_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:brf-descriptor:{}", entry.packet_id),
            packet_id: entry.packet_id.clone(),
            claim_row_id: entry.claim_row_id.clone(),
            surface_label: entry.surface_label.clone(),
            profile_class_token: entry.profile_class_token.clone(),
            profile_class_plain: profile_plain(entry.profile_class).to_owned(),
            packet_family_token: entry.packet_family_token.clone(),
            packet_family_plain: family_plain(entry.packet_family).to_owned(),
            scope_exercised_token: entry.restore_scope.scope_exercised_token.clone(),
            scope_exercised_plain: entry.restore_scope.scope_exercised.plain().to_owned(),
            restore_identity_token: entry.restore_identity_token.clone(),
            restore_identity_plain: restore_identity_plain(entry.restore_identity).to_owned(),
            partial_loss_token: entry.partial_loss_token.clone(),
            partial_loss_plain: partial_loss_plain(entry.partial_loss).to_owned(),
            restore_failover_hosting_token: entry.restore_failover_hosting_token.clone(),
            cadence_token: entry.drill.cadence_token.clone(),
            evidence_state_token: entry.drill.evidence_state_token.clone(),
            evidence_current: entry.drill.evidence_state.is_acceptable(),
            scope_line: entry.restore_scope.scope_line(),
            restore_identity_line: restore_identity_line(entry),
            partial_loss_line: partial_loss_line(entry),
            drill_line: drill_line(entry),
        }
    }
}

/// One surface rendering of a backup/restore/failover descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this projection renders on.
    pub surface: PacketSurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Packet this projection describes.
    pub packet_id: String,
    /// Descriptor id rendered on this surface.
    pub descriptor_id: String,
    /// Scope summary line rendered on this surface.
    pub scope_line: String,
    /// Restore-identity summary line rendered on this surface.
    pub restore_identity_line: String,
    /// Partial-loss summary line rendered on this surface.
    pub partial_loss_line: String,
    /// Drill summary line rendered on this surface.
    pub drill_line: String,
}

/// Per-packet verdict joining a packet to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Packet this outcome describes.
    pub packet_id: String,
    /// Claim row this packet backs.
    pub claim_row_id: String,
    /// Stable token for the packet family.
    pub packet_family_token: String,
    /// Computed qualification token for the packet.
    pub qualification_token: String,
    /// True when the packet narrowed below stable.
    pub narrowed: bool,
    /// True when the packet's claim is withheld entirely.
    pub claim_withheld: bool,
    /// True when the packet relies on generic "DR tested" text instead of typed evidence.
    pub generic_dr_text_only: bool,
    /// Stable token for the exercised scope.
    pub scope_exercised_token: String,
    /// Stable token for the restore identity.
    pub restore_identity_token: String,
    /// Stable token for the partial-loss class.
    pub partial_loss_token: String,
    /// Stable token for the drill-evidence freshness state.
    pub evidence_state_token: String,
    /// True when the packet evidence is current or reconstructable.
    pub evidence_current: bool,
    /// Stable narrow-reason tokens that applied to the packet.
    pub narrow_reason_tokens: Vec<String>,
}

/// One claimed resilience row's coverage verdict, derived from its packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimCoverageRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Claim row this coverage row describes.
    pub claim_row_id: String,
    /// Coverage class derived from the backing packet.
    pub coverage_class: ClaimCoverageClass,
    /// Stable token for [`Self::coverage_class`].
    pub coverage_class_token: String,
    /// Packet id backing the claim, empty when none.
    pub packet_id: String,
    /// Computed qualification token for the coverage.
    pub qualification_token: String,
    /// True when a current packet backs the claim.
    pub covered: bool,
    /// True when the coverage narrowed below stable.
    pub narrowed: bool,
}

/// Typed consumer that indexes packets by claim row and family.
///
/// The release-center, shiproom, support-center, partner-qualification, and
/// public claim-manifest surfaces read this registry instead of re-deriving
/// backup/restore/failover coverage by hand. It reports, per claimed resilience
/// row, whether a current packet backs the claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillPacketRegistry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Per-claim-row coverage rows.
    pub coverage: Vec<ClaimCoverageRow>,
    /// Claim row ids that point to a current packet.
    pub covered_claim_row_ids: Vec<String>,
    /// Claim row ids that narrowed because their packet is stale, withheld, or missing.
    pub uncovered_claim_row_ids: Vec<String>,
}

impl DrillPacketRegistry {
    /// Builds a registry from a finished page's outcomes and expected rows.
    pub fn from_page(page: &BackupRestoreFailoverPage) -> Self {
        build_registry(&page.input, &page.outcomes)
    }

    /// Returns the coverage row for a claim row id, if present.
    pub fn coverage_for_claim_row(&self, claim_row_id: &str) -> Option<&ClaimCoverageRow> {
        self.coverage
            .iter()
            .find(|row| row.claim_row_id == claim_row_id)
    }

    /// True when a current packet backs the claim row.
    pub fn is_claim_row_covered(&self, claim_row_id: &str) -> bool {
        self.coverage_for_claim_row(claim_row_id)
            .is_some_and(|row| row.covered)
    }

    /// Number of claim rows backed by a current packet.
    pub fn covered_claim_count(&self) -> usize {
        self.covered_claim_row_ids.len()
    }

    /// True when every tracked claim row points to a current packet.
    pub fn all_claims_covered(&self) -> bool {
        self.uncovered_claim_row_ids.is_empty()
    }
}

/// Typed defect emitted by the backup/restore/failover audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: PacketNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source packet id or claim row that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl BackupRestoreFailoverDefect {
    fn new(
        narrow_reason: PacketNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: BACKUP_RESTORE_FAILOVER_DEFECT_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:backup-restore-failover:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Aggregate summary for a backup/restore/failover page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of packets.
    pub packet_count: usize,
    /// Number of distinct packet families covered.
    pub family_count: usize,
    /// Number of managed-family packets.
    pub managed_family_packet_count: usize,
    /// Number of fully-exercised packets.
    pub fully_exercised_count: usize,
    /// Number of partially-exercised packets.
    pub partially_exercised_count: usize,
    /// Number of packets that exercised nothing.
    pub not_exercised_count: usize,
    /// Number of managed-family packets that declare a restore identity.
    pub restore_identity_declared_count: usize,
    /// Number of packets that disclose partial-loss behavior.
    pub partial_loss_disclosed_count: usize,
    /// Number of packets whose evidence is current or reconstructable.
    pub current_evidence_count: usize,
    /// Number of packets that need a fresh drill.
    pub needs_drill_count: usize,
    /// Number of packets that narrowed below stable.
    pub narrowed_count: usize,
    /// Number of packets whose claim is withheld.
    pub withdrawn_count: usize,
    /// Number of tracked claim rows.
    pub claim_coverage_count: usize,
    /// Number of claim rows backed by a current packet.
    pub covered_claim_count: usize,
    /// Number of claim rows that narrowed for lack of a current packet.
    pub uncovered_claim_count: usize,
    /// Number of surface projections emitted.
    pub surface_projection_count: usize,
    /// True when every surface renders the same restore-identity/partial-loss vocabulary.
    pub vocabulary_consistent: bool,
    /// True when every managed-family packet declares a restore identity.
    pub all_managed_packets_disclose_restore_identity: bool,
    /// True when every packet discloses its partial-loss behavior.
    pub all_packets_disclose_partial_loss: bool,
    /// True when every tracked claim row points to a current packet.
    pub all_expected_claims_covered: bool,
    /// True when no packet relies on generic "DR tested" text.
    pub no_generic_dr_text: bool,
    /// True when restore-identity and partial-loss fields are export-safe by default.
    pub restore_identity_and_partial_loss_export_safe: bool,
    /// True when no raw provider payload is carried anywhere in the packet.
    pub raw_payloads_excluded: bool,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Full auditable input for a backup/restore/failover page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverInput {
    /// Reviewable label for the page.
    pub input_label: String,
    /// Claimed backup/restore/failover packets.
    pub packets: Vec<BackupRestoreFailoverPacketEntry>,
    /// Claim rows that carry resilience language and must point to a current packet.
    pub expected_claim_row_ids: Vec<String>,
}

/// Canonical proof packet for the backup/restore/failover continuity lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page identifier.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: BackupRestoreFailoverSummary,
    /// Typed defects for the packet.
    pub defects: Vec<BackupRestoreFailoverDefect>,
    /// Plain-language descriptors, one per packet.
    pub descriptors: Vec<BackupRestoreFailoverDescriptor>,
    /// Per-surface projections proving identical vocabulary across surfaces.
    pub surface_projections: Vec<BackupRestoreFailoverSurfaceProjection>,
    /// Per-packet verdicts joining each packet to its computed qualification.
    pub outcomes: Vec<BackupRestoreFailoverOutcome>,
    /// The typed consumer registry of claim-row coverage.
    pub registry: DrillPacketRegistry,
    /// The audited input embedded as evidence.
    pub input: BackupRestoreFailoverInput,
}

impl BackupRestoreFailoverPage {
    /// Builds a backup/restore/failover page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: BackupRestoreFailoverInput,
    ) -> Self {
        let descriptors: Vec<BackupRestoreFailoverDescriptor> = input
            .packets
            .iter()
            .map(BackupRestoreFailoverDescriptor::from_entry)
            .collect();
        let surface_projections = build_surface_projections(&input.packets);
        let defects = audit(&input, &surface_projections);
        let outcomes = build_outcomes(&input, &defects);
        let registry = build_registry(&input, &outcomes);
        let summary = build_summary(&input, &surface_projections, &outcomes, &registry, &defects);
        Self {
            record_kind: BACKUP_RESTORE_FAILOVER_PAGE_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            descriptors,
            surface_projections,
            outcomes,
            registry,
            input,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when every surface renders identical restore-identity/partial-loss vocabulary.
    pub fn surfaces_share_vocabulary(&self) -> bool {
        self.summary.vocabulary_consistent
    }

    /// True when every tracked claim row points to a current packet.
    pub fn every_claim_covered(&self) -> bool {
        self.summary.all_expected_claims_covered
    }

    /// Returns the descriptor for a packet id, if present.
    pub fn descriptor(&self, packet_id: &str) -> Option<&BackupRestoreFailoverDescriptor> {
        self.descriptors.iter().find(|d| d.packet_id == packet_id)
    }

    /// Returns the computed outcome for a packet id, if present.
    pub fn outcome(&self, packet_id: &str) -> Option<&BackupRestoreFailoverOutcome> {
        self.outcomes.iter().find(|o| o.packet_id == packet_id)
    }
}

/// Support-export wrapper for the backup/restore/failover page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The backup/restore/failover page embedded as evidence.
    pub page: BackupRestoreFailoverPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<PacketNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when restore-identity and partial-loss fields are export-safe by default.
    pub restore_identity_and_partial_loss_export_safe: bool,
    /// True when raw provider payloads are excluded from this export.
    pub raw_payloads_excluded: bool,
}

impl BackupRestoreFailoverSupportExport {
    /// Wraps a backup/restore/failover page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: BackupRestoreFailoverPage,
    ) -> Self {
        let mut reasons: Vec<PacketNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            restore_identity_and_partial_loss_export_safe: true,
            raw_payloads_excluded: true,
        }
    }
}

/// Re-runs the backup/restore/failover audit over a page, including its projections.
///
/// Unlike [`BackupRestoreFailoverPage::new`], this validates the page's stored
/// surface projections against freshly derived canonical lines, so a tampered
/// projection (one that renders different vocabulary than its descriptor) is
/// caught on re-validation.
pub fn audit_backup_restore_failover_page(
    page: &BackupRestoreFailoverPage,
) -> Vec<BackupRestoreFailoverDefect> {
    audit(&page.input, &page.surface_projections)
}

/// Validates a backup/restore/failover page and returns `Ok(())` when clean.
pub fn validate_backup_restore_failover_page(
    page: &BackupRestoreFailoverPage,
) -> Result<(), Vec<BackupRestoreFailoverDefect>> {
    let defects = audit_backup_restore_failover_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Returns the seeded stable backup/restore/failover page.
pub fn seeded_backup_restore_failover_page() -> BackupRestoreFailoverPage {
    BackupRestoreFailoverPage::new(
        "continuity:backup-restore-failover:seeded",
        "Backup, restore, and failover continuity packets",
        "2026-06-01T00:00:00Z",
        seeded_backup_restore_failover_input(),
    )
}

/// Returns the seeded input used by the canonical backup/restore/failover page.
///
/// The seeded page carries one packet for each managed continuity family —
/// backup, failover, restore, and snapshot/replication — across managed,
/// self-hosted, and sovereign profiles, plus a local-core continuity packet.
/// Every managed-family packet names a cadence, a current and future owner,
/// typed exercised operations, a restore identity, and a partial-loss
/// disclosure; the partially-exercised self-hosted restore packet discloses what
/// it did not exercise. Every claimed resilience row points to a current packet,
/// so the page qualifies stable.
pub fn seeded_backup_restore_failover_input() -> BackupRestoreFailoverInput {
    let all = PacketSurfaceClass::ALL.to_vec();
    let packets = vec![
        BackupRestoreFailoverPacketEntry::new(
            "continuity-brf:managed-workspace-backup",
            "continuity:row:managed-workspace-sync-backup",
            "Managed cloud workspace backup",
            ContinuityProfileClass::Managed,
            ContinuityPacketFamilyClass::Backup,
            true,
            RestoreScope::new(
                ScopeExercisedClass::FullyExercised,
                vec![
                    RestoreOperationClass::BackupCapture,
                    RestoreOperationClass::BackupIntegrityVerify,
                    RestoreOperationClass::RestoreExecute,
                    RestoreOperationClass::RestoreIntegrityVerify,
                ],
                "",
            ),
            RestoreIdentityClass::SameIdentityRestore,
            RestoreFailoverHostingClass::VendorOperated,
            true,
            PartialLossClass::BoundedRecentWindowLoss,
            "A bounded window of the most recent unsynced writes may be lost; older state restores exactly.",
            DrillEvidence::new(
                DrillCadenceClass::PerRelease,
                DrillEvidenceStateClass::Current,
                "Managed platform on-call",
                "Reliability guild",
                "2026-05-30T00:00:00Z",
                "2026-07-30T00:00:00Z",
                "drill-evidence:managed-workspace-backup:2026-05-30",
            ),
            false,
            all.clone(),
        ),
        BackupRestoreFailoverPacketEntry::new(
            "continuity-brf:managed-relay-failover",
            "continuity:row:managed-relay-collaboration-failover",
            "Managed relay and collaboration failover",
            ContinuityProfileClass::Managed,
            ContinuityPacketFamilyClass::Failover,
            true,
            RestoreScope::new(
                ScopeExercisedClass::FullyExercised,
                vec![
                    RestoreOperationClass::FailoverCutover,
                    RestoreOperationClass::Failback,
                    RestoreOperationClass::RestoreExecute,
                ],
                "",
            ),
            RestoreIdentityClass::SameIdentityRestore,
            RestoreFailoverHostingClass::VendorOperated,
            true,
            PartialLossClass::QueuedActionLoss,
            "Queued in-flight collaboration actions may be lost on cutover; durable documents restore exactly.",
            DrillEvidence::new(
                DrillCadenceClass::Quarterly,
                DrillEvidenceStateClass::Current,
                "Managed platform on-call",
                "Reliability guild",
                "2026-05-20T00:00:00Z",
                "2026-08-20T00:00:00Z",
                "drill-evidence:managed-relay-failover:2026-05-20",
            ),
            false,
            all.clone(),
        ),
        BackupRestoreFailoverPacketEntry::new(
            "continuity-brf:self-hosted-restore",
            "continuity:row:customer-self-hosted-restore",
            "Customer self-hosted restore and rebuild",
            ContinuityProfileClass::SelfHosted,
            ContinuityPacketFamilyClass::Restore,
            true,
            RestoreScope::new(
                ScopeExercisedClass::PartiallyExercised,
                vec![
                    RestoreOperationClass::RestoreExecute,
                    RestoreOperationClass::RestoreIntegrityVerify,
                ],
                "cross-region failover cutover; only a same-site restore was verified in this drill",
            ),
            RestoreIdentityClass::ReissuedIdentityRestore,
            RestoreFailoverHostingClass::CustomerOperated,
            true,
            PartialLossClass::BoundedRecentWindowLoss,
            "A bounded window of recent writes since the last customer snapshot may be lost.",
            DrillEvidence::new(
                DrillCadenceClass::Semiannual,
                DrillEvidenceStateClass::ReconstructableFromSnapshot,
                "Customer success SRE",
                "Field reliability owner",
                "",
                "",
                "drill-evidence:self-hosted-restore:snapshot-2026-04",
            ),
            false,
            all.clone(),
        ),
        BackupRestoreFailoverPacketEntry::new(
            "continuity-brf:sovereign-snapshot-replication",
            "continuity:row:sovereign-airgapped-snapshot",
            "Sovereign air-gapped snapshot and replication",
            ContinuityProfileClass::Sovereign,
            ContinuityPacketFamilyClass::SnapshotReplication,
            true,
            RestoreScope::new(
                ScopeExercisedClass::FullyExercised,
                vec![
                    RestoreOperationClass::BackupCapture,
                    RestoreOperationClass::RestoreExecute,
                    RestoreOperationClass::RestoreIntegrityVerify,
                ],
                "",
            ),
            RestoreIdentityClass::NewInstallRebind,
            RestoreFailoverHostingClass::OfflineSnapshot,
            true,
            PartialLossClass::CacheOnlyLoss,
            "Only derived cache state is lost on rebuild; durable records replicate exactly inside the boundary.",
            DrillEvidence::new(
                DrillCadenceClass::Annual,
                DrillEvidenceStateClass::StaleWithinGrace,
                "Sovereign operations lead",
                "Customer compliance owner",
                "2026-01-15T00:00:00Z",
                "2026-07-15T00:00:00Z",
                "drill-evidence:sovereign-snapshot-replication:2026-01-15",
            ),
            false,
            all.clone(),
        ),
        BackupRestoreFailoverPacketEntry::new(
            "continuity-brf:local-core-continuity",
            "continuity:row:local-desktop-core",
            "Local desktop core continuity",
            ContinuityProfileClass::LocalOnly,
            ContinuityPacketFamilyClass::LocalCoreContinuity,
            false,
            RestoreScope::new(
                ScopeExercisedClass::FullyExercised,
                vec![
                    RestoreOperationClass::BackupCapture,
                    RestoreOperationClass::RestoreExecute,
                ],
                "",
            ),
            RestoreIdentityClass::NotApplicable,
            RestoreFailoverHostingClass::LocalCore,
            false,
            PartialLossClass::NoPartialLoss,
            "Local editing, save, search, and version control restore from on-device history with no managed dependency.",
            DrillEvidence::new(
                DrillCadenceClass::OnDemandOnly,
                DrillEvidenceStateClass::ReconstructableFromSnapshot,
                "Local user",
                "Local user",
                "",
                "",
                "drill-evidence:local-core-continuity:on-device",
            ),
            false,
            all,
        ),
    ];
    BackupRestoreFailoverInput {
        input_label: "Backup, restore, and failover packets across managed, self-hosted, sovereign, and local-core profiles".to_owned(),
        expected_claim_row_ids: vec![
            "continuity:row:managed-workspace-sync-backup".to_owned(),
            "continuity:row:managed-relay-collaboration-failover".to_owned(),
            "continuity:row:customer-self-hosted-restore".to_owned(),
            "continuity:row:sovereign-airgapped-snapshot".to_owned(),
        ],
        packets,
    }
}

fn audit(
    input: &BackupRestoreFailoverInput,
    projections: &[BackupRestoreFailoverSurfaceProjection],
) -> Vec<BackupRestoreFailoverDefect> {
    let mut defects = Vec::new();
    for packet in &input.packets {
        audit_packet(packet, &mut defects);
    }
    audit_vocabulary(input, projections, &mut defects);
    audit_claim_coverage(input, &mut defects);
    defects
}

fn audit_packet(
    packet: &BackupRestoreFailoverPacketEntry,
    defects: &mut Vec<BackupRestoreFailoverDefect>,
) {
    // Headline guardrail: generic "DR tested" text is never a substitute for
    // typed packet evidence.
    if packet.generic_dr_text_only {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::GenericDrTextOnly,
            packet.packet_id.clone(),
            "a backup/restore/failover claim may not rely on generic 'DR tested' text; typed packet evidence is required",
        ));
    }

    // Partial-loss disclosure applies to every packet.
    if packet.partial_loss == PartialLossClass::Undisclosed {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::PartialLossUndisclosed,
            packet.packet_id.clone(),
            "every packet must disclose its partial-loss behavior on recovery",
        ));
    }

    // Hard guardrail: a self-governed boundary may not hide a vendor-operated
    // restore/failover lane.
    if packet.overclaims_self_governed_continuity() {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::SovereignContinuityOverclaimed,
            packet.packet_id.clone(),
            "a self-hosted or sovereign packet may not hide a vendor-operated restore or failover lane",
        ));
    }

    // Profile-vs-hosting mismatch.
    if let Some(note) = packet.profile_hosting_mismatch() {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::ProfileMismatch,
            packet.packet_id.clone(),
            note,
        ));
    }

    // Surface projection completeness.
    let missing = packet
        .required_surfaces()
        .iter()
        .any(|surface| !packet.projected_surfaces.contains(surface));
    if missing {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::SurfaceReuseIncomplete,
            packet.packet_id.clone(),
            "every packet must reach the release-center, shiproom, support-center, partner-qualification, and public claim-manifest surfaces",
        ));
    }

    // The managed-continuity evidence requirements only bind managed families.
    if !packet.requires_managed_evidence() {
        return;
    }

    // Scope must be exercised with typed operations.
    if !packet.restore_scope.is_exercised() {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::ScopeNotExercised,
            packet.packet_id.clone(),
            "a managed continuity packet must record which operations its most recent drill exercised",
        ));
    } else if packet.restore_scope.missing_not_exercised_disclosure() {
        // Guardrail: a partial drill may not omit what restored narrower than normal.
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::NotExercisedDisclosureMissing,
            packet.packet_id.clone(),
            "a partial drill must disclose what restored narrower than normal or was not exercised",
        ));
    }

    // Restore identity must be declared for managed continuity families.
    if packet.packet_family.requires_restore_identity()
        && packet.restore_identity == RestoreIdentityClass::NotApplicable
    {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::RestoreIdentityUndeclared,
            packet.packet_id.clone(),
            "a managed continuity packet must declare the restore identity recovery reproduces",
        ));
    }

    // Drill cadence and ownership.
    if packet.drill.cadence == DrillCadenceClass::OnDemandOnly {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::DrillCadenceMissing,
            packet.packet_id.clone(),
            "a managed continuity packet must name a recurring drill cadence, not on-demand only",
        ));
    }
    if !packet.drill.has_named_owners() {
        defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::DrillOwnerMissing,
            packet.packet_id.clone(),
            "a managed continuity packet must name both a current and a future drill owner",
        ));
    }

    // Drill evidence freshness and freshness-SLO declaration.
    match packet.drill.evidence_state {
        DrillEvidenceStateClass::NeverRun => defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::DrillNeverRun,
            packet.packet_id.clone(),
            "the continuity drill has never been run; the claim cannot exceed preview",
        )),
        DrillEvidenceStateClass::StaleNeedsDrill => defects.push(BackupRestoreFailoverDefect::new(
            PacketNarrowReasonClass::DrillEvidenceStale,
            packet.packet_id.clone(),
            "drill evidence has aged out under the freshness SLO and a fresh drill is required",
        )),
        _ if packet.drill.missing_freshness_window() => {
            defects.push(BackupRestoreFailoverDefect::new(
                PacketNarrowReasonClass::DrillEvidenceStale,
                packet.packet_id.clone(),
                "current or graced drill evidence must record a last-drill timestamp and a freshness-SLO expiry",
            ));
        }
        _ => {}
    }
}

fn audit_vocabulary(
    input: &BackupRestoreFailoverInput,
    projections: &[BackupRestoreFailoverSurfaceProjection],
    defects: &mut Vec<BackupRestoreFailoverDefect>,
) {
    for packet in &input.packets {
        let canonical_scope = packet.restore_scope.scope_line();
        let canonical_restore = restore_identity_line(packet);
        let canonical_partial = partial_loss_line(packet);
        let canonical_drill = drill_line(packet);
        let drifted = projections
            .iter()
            .filter(|projection| projection.packet_id == packet.packet_id)
            .any(|projection| {
                projection.scope_line != canonical_scope
                    || projection.restore_identity_line != canonical_restore
                    || projection.partial_loss_line != canonical_partial
                    || projection.drill_line != canonical_drill
            });
        if drifted {
            defects.push(BackupRestoreFailoverDefect::new(
                PacketNarrowReasonClass::PacketVocabularyDrift,
                packet.packet_id.clone(),
                "a surface renders different restore-identity, partial-loss, scope, or drill vocabulary than the descriptor",
            ));
        }
    }
}

fn audit_claim_coverage(
    input: &BackupRestoreFailoverInput,
    defects: &mut Vec<BackupRestoreFailoverDefect>,
) {
    for claim_row_id in &input.expected_claim_row_ids {
        let has_packet = input
            .packets
            .iter()
            .any(|packet| &packet.claim_row_id == claim_row_id);
        if !has_packet {
            defects.push(BackupRestoreFailoverDefect::new(
                PacketNarrowReasonClass::PacketEvidenceMissing,
                claim_row_id.clone(),
                "a claimed resilience row carries no backup/restore/failover packet; the claim narrows",
            ));
        }
    }
}

fn build_surface_projections(
    packets: &[BackupRestoreFailoverPacketEntry],
) -> Vec<BackupRestoreFailoverSurfaceProjection> {
    let mut projections = Vec::new();
    for packet in packets {
        let scope_line = packet.restore_scope.scope_line();
        let restore_identity_line = restore_identity_line(packet);
        let partial_loss_line = partial_loss_line(packet);
        let drill_line = drill_line(packet);
        let descriptor_id = format!("continuity:brf-descriptor:{}", packet.packet_id);
        for surface in PacketSurfaceClass::ALL {
            if !packet.projected_surfaces.contains(&surface) {
                continue;
            }
            projections.push(BackupRestoreFailoverSurfaceProjection {
                record_kind: BACKUP_RESTORE_FAILOVER_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
                schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
                shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
                surface,
                surface_token: surface.as_str().to_owned(),
                packet_id: packet.packet_id.clone(),
                descriptor_id: descriptor_id.clone(),
                scope_line: scope_line.clone(),
                restore_identity_line: restore_identity_line.clone(),
                partial_loss_line: partial_loss_line.clone(),
                drill_line: drill_line.clone(),
            });
        }
    }
    projections
}

fn build_outcomes(
    input: &BackupRestoreFailoverInput,
    defects: &[BackupRestoreFailoverDefect],
) -> Vec<BackupRestoreFailoverOutcome> {
    input
        .packets
        .iter()
        .map(|packet| {
            let reasons: Vec<PacketNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == packet.packet_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = qualification_from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            BackupRestoreFailoverOutcome {
                record_kind: BACKUP_RESTORE_FAILOVER_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
                shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
                packet_id: packet.packet_id.clone(),
                claim_row_id: packet.claim_row_id.clone(),
                packet_family_token: packet.packet_family_token.clone(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                generic_dr_text_only: packet.generic_dr_text_only,
                scope_exercised_token: packet.restore_scope.scope_exercised_token.clone(),
                restore_identity_token: packet.restore_identity_token.clone(),
                partial_loss_token: packet.partial_loss_token.clone(),
                evidence_state_token: packet.drill.evidence_state_token.clone(),
                evidence_current: packet.drill.evidence_state.is_acceptable(),
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_registry(
    input: &BackupRestoreFailoverInput,
    outcomes: &[BackupRestoreFailoverOutcome],
) -> DrillPacketRegistry {
    // The tracked claim rows are the declared resilience rows plus every row a
    // packet actually backs, in stable sorted order.
    let mut claim_row_ids: Vec<String> = input.expected_claim_row_ids.clone();
    for packet in &input.packets {
        claim_row_ids.push(packet.claim_row_id.clone());
    }
    claim_row_ids.sort();
    claim_row_ids.dedup();

    let mut coverage = Vec::new();
    let mut covered = Vec::new();
    let mut uncovered = Vec::new();
    for claim_row_id in claim_row_ids {
        let outcome = outcomes
            .iter()
            .find(|outcome| outcome.claim_row_id == claim_row_id);
        let (coverage_class, qualification_token, packet_id) = match outcome {
            None => (
                ClaimCoverageClass::NoPacket,
                ContinuityClaimQualificationClass::Preview
                    .as_str()
                    .to_owned(),
                String::new(),
            ),
            Some(outcome) if outcome.claim_withheld => (
                ClaimCoverageClass::PacketWithheld,
                outcome.qualification_token.clone(),
                outcome.packet_id.clone(),
            ),
            Some(outcome) if outcome.narrowed || !outcome.evidence_current => (
                ClaimCoverageClass::StalePacketNeedsRefresh,
                outcome.qualification_token.clone(),
                outcome.packet_id.clone(),
            ),
            Some(outcome) => (
                ClaimCoverageClass::CurrentPacket,
                outcome.qualification_token.clone(),
                outcome.packet_id.clone(),
            ),
        };
        let covered_now = coverage_class.is_covered();
        if covered_now {
            covered.push(claim_row_id.clone());
        } else {
            uncovered.push(claim_row_id.clone());
        }
        coverage.push(ClaimCoverageRow {
            record_kind: CLAIM_COVERAGE_ROW_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            claim_row_id,
            coverage_class,
            coverage_class_token: coverage_class.as_str().to_owned(),
            packet_id,
            qualification_token,
            covered: covered_now,
            narrowed: !covered_now,
        });
    }

    DrillPacketRegistry {
        record_kind: DRILL_PACKET_REGISTRY_RECORD_KIND.to_owned(),
        schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
        shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
        registry_id: "continuity:drill-packet-registry".to_owned(),
        coverage,
        covered_claim_row_ids: covered,
        uncovered_claim_row_ids: uncovered,
    }
}

fn build_summary(
    input: &BackupRestoreFailoverInput,
    projections: &[BackupRestoreFailoverSurfaceProjection],
    outcomes: &[BackupRestoreFailoverOutcome],
    registry: &DrillPacketRegistry,
    defects: &[BackupRestoreFailoverDefect],
) -> BackupRestoreFailoverSummary {
    let overall = if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        ContinuityClaimQualificationClass::Withdrawn
    } else if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason())
    {
        ContinuityClaimQualificationClass::Preview
    } else if defects.is_empty() {
        ContinuityClaimQualificationClass::Stable
    } else {
        ContinuityClaimQualificationClass::Beta
    };

    let vocabulary_consistent = !defects
        .iter()
        .any(|defect| defect.narrow_reason == PacketNarrowReasonClass::PacketVocabularyDrift);

    let mut families: Vec<ContinuityPacketFamilyClass> = input
        .packets
        .iter()
        .map(|packet| packet.packet_family)
        .collect();
    families.sort();
    families.dedup();

    let managed_packets: Vec<&BackupRestoreFailoverPacketEntry> = input
        .packets
        .iter()
        .filter(|packet| packet.requires_managed_evidence())
        .collect();

    BackupRestoreFailoverSummary {
        record_kind: BACKUP_RESTORE_FAILOVER_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
        shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        packet_count: input.packets.len(),
        family_count: families.len(),
        managed_family_packet_count: managed_packets.len(),
        fully_exercised_count: scope_count(input, ScopeExercisedClass::FullyExercised),
        partially_exercised_count: scope_count(input, ScopeExercisedClass::PartiallyExercised),
        not_exercised_count: scope_count(input, ScopeExercisedClass::NotExercised),
        restore_identity_declared_count: managed_packets
            .iter()
            .filter(|packet| packet.restore_identity != RestoreIdentityClass::NotApplicable)
            .count(),
        partial_loss_disclosed_count: input
            .packets
            .iter()
            .filter(|packet| packet.partial_loss != PartialLossClass::Undisclosed)
            .count(),
        current_evidence_count: input
            .packets
            .iter()
            .filter(|packet| packet.drill.evidence_state.is_acceptable())
            .count(),
        needs_drill_count: input
            .packets
            .iter()
            .filter(|packet| packet.drill.evidence_state.needs_drill())
            .count(),
        narrowed_count: outcomes.iter().filter(|outcome| outcome.narrowed).count(),
        withdrawn_count: outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        claim_coverage_count: registry.coverage.len(),
        covered_claim_count: registry.covered_claim_row_ids.len(),
        uncovered_claim_count: registry.uncovered_claim_row_ids.len(),
        surface_projection_count: projections.len(),
        vocabulary_consistent,
        all_managed_packets_disclose_restore_identity: managed_packets.iter().all(|packet| {
            !packet.packet_family.requires_restore_identity()
                || packet.restore_identity != RestoreIdentityClass::NotApplicable
        }),
        all_packets_disclose_partial_loss: input
            .packets
            .iter()
            .all(|packet| packet.partial_loss != PartialLossClass::Undisclosed),
        all_expected_claims_covered: input
            .expected_claim_row_ids
            .iter()
            .all(|claim_row_id| registry.is_claim_row_covered(claim_row_id)),
        no_generic_dr_text: !input
            .packets
            .iter()
            .any(|packet| packet.generic_dr_text_only),
        restore_identity_and_partial_loss_export_safe: true,
        raw_payloads_excluded: true,
        defect_count: defects.len(),
    }
}

fn scope_count(input: &BackupRestoreFailoverInput, scope: ScopeExercisedClass) -> usize {
    input
        .packets
        .iter()
        .filter(|packet| packet.restore_scope.scope_exercised == scope)
        .count()
}

fn restore_identity_line(entry: &BackupRestoreFailoverPacketEntry) -> String {
    format!(
        "Restore identity: {}.",
        restore_identity_plain(entry.restore_identity)
    )
}

fn partial_loss_line(entry: &BackupRestoreFailoverPacketEntry) -> String {
    let note = entry.partial_loss_note.trim();
    if note.is_empty() {
        format!("Partial loss: {}.", partial_loss_plain(entry.partial_loss))
    } else {
        format!(
            "Partial loss: {} — {}",
            partial_loss_plain(entry.partial_loss),
            note
        )
    }
}

fn drill_line(entry: &BackupRestoreFailoverPacketEntry) -> String {
    format!(
        "Drilled {}; evidence {}; owners {} (now) / {} (next).",
        cadence_plain(entry.drill.cadence),
        evidence_plain(entry.drill.evidence_state),
        owner_or_unnamed(&entry.drill.current_owner_label),
        owner_or_unnamed(&entry.drill.future_owner_label),
    )
}

fn owner_or_unnamed(label: &str) -> &str {
    if label.trim().is_empty() {
        "unnamed"
    } else {
        label
    }
}

fn profile_plain(class: ContinuityProfileClass) -> &'static str {
    match class {
        ContinuityProfileClass::Managed => "managed cloud",
        ContinuityProfileClass::SelfHosted => "self-hosted",
        ContinuityProfileClass::Sovereign => "sovereign",
        ContinuityProfileClass::LocalOnly => "local-only",
    }
}

fn family_plain(class: ContinuityPacketFamilyClass) -> &'static str {
    match class {
        ContinuityPacketFamilyClass::Backup => "backup",
        ContinuityPacketFamilyClass::Restore => "restore",
        ContinuityPacketFamilyClass::Failover => "failover",
        ContinuityPacketFamilyClass::SnapshotReplication => "snapshot and replication",
        ContinuityPacketFamilyClass::LocalCoreContinuity => "local-core continuity",
    }
}

fn restore_identity_plain(class: RestoreIdentityClass) -> &'static str {
    match class {
        RestoreIdentityClass::SameIdentityRestore => {
            "recovery reproduces the same durable identity"
        }
        RestoreIdentityClass::ReissuedIdentityRestore => {
            "recovery reissues a derived identity that must be re-trusted"
        }
        RestoreIdentityClass::NewInstallRebind => "recovery requires a new install rebind",
        RestoreIdentityClass::NotApplicable => "not applicable to this packet",
    }
}

fn partial_loss_plain(class: PartialLossClass) -> &'static str {
    match class {
        PartialLossClass::NoPartialLoss => "no partial loss",
        PartialLossClass::BoundedRecentWindowLoss => {
            "a bounded recent window of writes may be lost"
        }
        PartialLossClass::QueuedActionLoss => "queued or in-flight actions may be lost",
        PartialLossClass::CacheOnlyLoss => "only cache or derived state may be lost",
        PartialLossClass::Undisclosed => "not disclosed",
    }
}

fn cadence_plain(class: DrillCadenceClass) -> &'static str {
    match class {
        DrillCadenceClass::PerRelease => "per release",
        DrillCadenceClass::Monthly => "monthly",
        DrillCadenceClass::Quarterly => "quarterly",
        DrillCadenceClass::Semiannual => "twice a year",
        DrillCadenceClass::Annual => "annually",
        DrillCadenceClass::OnDemandOnly => "on demand only",
    }
}

fn evidence_plain(class: DrillEvidenceStateClass) -> &'static str {
    match class {
        DrillEvidenceStateClass::Current => "current",
        DrillEvidenceStateClass::StaleWithinGrace => "stale within grace",
        DrillEvidenceStateClass::StaleNeedsDrill => "stale, needs a fresh drill",
        DrillEvidenceStateClass::NeverRun => "never run",
        DrillEvidenceStateClass::ReconstructableFromSnapshot => {
            "reconstructable from a verified snapshot"
        }
    }
}
