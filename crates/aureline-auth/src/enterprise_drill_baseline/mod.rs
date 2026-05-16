//! First backup-restore, failover, and key-rotation drill baseline for claimed
//! managed and enterprise row families.
//!
//! This module owns the auditable drill baseline that backs enterprise and
//! managed assurances during beta. Each drill packet records one rehearsal of
//! backup-restore, failover, or key-rotation on a claimed managed / enterprise
//! row family, names the recorded evidence freshness, and declares the impact
//! on the family's claim if the drill goes stale or breaks. Admin, support,
//! shell, headless, and reviewer surfaces consume the same record kind so
//! enterprise pilots see one drill baseline across connected, mirror-only,
//! offline, and enterprise-managed beta profiles.
//!
//! Guardrails: a drill packet must never widen authority on a sibling lane,
//! must never permit undeclared public-endpoint fallback, must never expose
//! raw private material, and must preserve local editing across the entire
//! rehearsal. The validator fails closed on any of these guarantees.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::trust::CapabilityAuthorityClass;

/// Beta schema version exported with every drill baseline record.
pub const ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every drill baseline record.
pub const ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF: &str =
    "security:enterprise_drill_baseline:v1";

/// Source matrix ref consumed by this drill baseline.
pub const ENTERPRISE_DRILL_BASELINE_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/backup_restore_failover_drills/enterprise_drill_baseline_matrix.yaml";

/// Stable record kind for [`EnterpriseDrillBaselinePage`] payloads.
pub const ENTERPRISE_DRILL_BASELINE_PAGE_RECORD_KIND: &str =
    "security_enterprise_drill_baseline_page_record";

/// Stable record kind for [`EnterpriseDrillPacket`] payloads.
pub const ENTERPRISE_DRILL_BASELINE_PACKET_RECORD_KIND: &str =
    "security_enterprise_drill_baseline_packet_record";

/// Stable record kind for [`EnterpriseDrillBaselineDefect`] payloads.
pub const ENTERPRISE_DRILL_BASELINE_DEFECT_RECORD_KIND: &str =
    "security_enterprise_drill_baseline_defect_record";

/// Stable record kind for [`EnterpriseDrillBaselineSummary`] payloads.
pub const ENTERPRISE_DRILL_BASELINE_SUMMARY_RECORD_KIND: &str =
    "security_enterprise_drill_baseline_summary_record";

/// Stable record kind for [`EnterpriseDrillBaselineSupportExport`] payloads.
pub const ENTERPRISE_DRILL_BASELINE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_enterprise_drill_baseline_support_export_record";

/// Drill kind exercised by an [`EnterpriseDrillPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDrillKindClass {
    /// Backup-then-restore rehearsal: recover a row family from its last
    /// signed snapshot, mirror, or air-gapped courier and replay the snapshot
    /// into the active claim.
    BackupRestore,
    /// Primary-to-secondary failover rehearsal on a row family's declared
    /// secondary path.
    Failover,
    /// Custody rotation rehearsal: rotate a row family's signing or wrapping
    /// key material and confirm the rotated material drives the claim.
    KeyRotation,
}

impl EnterpriseDrillKindClass {
    /// All required drill kinds in canonical order.
    pub const ALL: [Self; 3] = [Self::BackupRestore, Self::Failover, Self::KeyRotation];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackupRestore => "backup_restore",
            Self::Failover => "failover",
            Self::KeyRotation => "key_rotation",
        }
    }
}

/// Claimed managed / enterprise row family exercised by a drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseRowFamilyClass {
    /// Managed policy distribution rows (policy packs, admin policy push,
    /// signed mirror).
    ManagedPolicyDistribution,
    /// Managed credential / vault handle rows (secret broker, keychain, BYOK
    /// custody).
    ManagedCredentialHandle,
    /// Enterprise identity session rows (managed OIDC, passkey step-up, tenant
    /// session continuity).
    EnterpriseIdentitySession,
}

impl EnterpriseRowFamilyClass {
    /// All claimed row families in canonical order.
    pub const ALL: [Self; 3] = [
        Self::ManagedPolicyDistribution,
        Self::ManagedCredentialHandle,
        Self::EnterpriseIdentitySession,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPolicyDistribution => "managed_policy_distribution",
            Self::ManagedCredentialHandle => "managed_credential_handle",
            Self::EnterpriseIdentitySession => "enterprise_identity_session",
        }
    }
}

/// Beta profile under which a drill packet was rehearsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDrillProfileClass {
    /// Connected beta profile with live managed control plane reachable.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline profile served from a last-known-good or air-gapped snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed narrowing.
    EnterpriseManaged,
}

impl EnterpriseDrillProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Outcome observed during a drill rehearsal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDrillOutcomeClass {
    /// Backup-restore succeeded: the row family was rehydrated from its
    /// declared snapshot or mirror with the same authority as before.
    RestoredFromTrustedSnapshot,
    /// Failover succeeded: the row family failed over to its declared
    /// secondary without widening sibling-lane authority.
    FailedOverToDeclaredFallback,
    /// Key rotation succeeded: rotated material drove the claim and authority
    /// recovered after the rotation window closed.
    RotatedThenRecovered,
    /// Drill succeeded but authority on the affected family narrowed and
    /// remains narrowed until admin action lifts the narrowing.
    DowngradedAwaitingAdmin,
}

impl EnterpriseDrillOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoredFromTrustedSnapshot => "restored_from_trusted_snapshot",
            Self::FailedOverToDeclaredFallback => "failed_over_to_declared_fallback",
            Self::RotatedThenRecovered => "rotated_then_recovered",
            Self::DowngradedAwaitingAdmin => "downgraded_awaiting_admin",
        }
    }
}

/// Freshness state observed on the recorded drill evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDrillEvidenceFreshnessClass {
    /// Drill evidence is within the declared freshness window.
    Fresh,
    /// Drill evidence is stale but still within the soft recheck window.
    StaleWithinWindow,
    /// Drill evidence is stale beyond the hard recheck window; the claim
    /// must downgrade.
    StaleBeyondWindow,
    /// Drill evidence is missing or could not be verified; the claim must
    /// downgrade.
    Missing,
}

impl EnterpriseDrillEvidenceFreshnessClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::StaleWithinWindow => "stale_within_window",
            Self::StaleBeyondWindow => "stale_beyond_window",
            Self::Missing => "missing",
        }
    }

    /// True when this freshness state requires the claim to downgrade.
    pub const fn requires_downgrade(self) -> bool {
        matches!(self, Self::StaleBeyondWindow | Self::Missing)
    }
}

/// Claim impact applied when drill evidence goes stale or breaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDrillClaimImpactClass {
    /// No claim impact. Only valid when evidence remains fresh.
    NoImpact,
    /// Downgrade only the affected claim on the row family.
    DowngradeAffectedClaim,
    /// Downgrade every claim on this row family until the drill is refreshed.
    DowngradeFamilyClaims,
}

impl EnterpriseDrillClaimImpactClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpact => "no_impact",
            Self::DowngradeAffectedClaim => "downgrade_affected_claim",
            Self::DowngradeFamilyClaims => "downgrade_family_claims",
        }
    }

    /// True when this impact downgrades at least one claim.
    pub const fn downgrades(self) -> bool {
        matches!(
            self,
            Self::DowngradeAffectedClaim | Self::DowngradeFamilyClaims
        )
    }
}

/// One drill packet for backup-restore, failover, or key-rotation on a
/// claimed managed / enterprise row family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDrillPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable drill packet id.
    pub drill_packet_id: String,
    /// Drill kind exercised by the packet.
    pub drill_kind: EnterpriseDrillKindClass,
    /// Stable token for [`Self::drill_kind`].
    pub drill_kind_token: String,
    /// Row family on which the drill was rehearsed.
    pub row_family: EnterpriseRowFamilyClass,
    /// Stable token for [`Self::row_family`].
    pub row_family_token: String,
    /// Synthetic id of the affected claim (never carries raw private material).
    pub affected_claim_id: String,
    /// Profile under which the drill was rehearsed.
    pub profile: EnterpriseDrillProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Recorded outcome of the drill.
    pub outcome: EnterpriseDrillOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Freshness state of the recorded drill evidence.
    pub evidence_freshness: EnterpriseDrillEvidenceFreshnessClass,
    /// Stable token for [`Self::evidence_freshness`].
    pub evidence_freshness_token: String,
    /// Claim impact applied when this drill's evidence goes stale or breaks.
    pub claim_impact_if_stale: EnterpriseDrillClaimImpactClass,
    /// Stable token for [`Self::claim_impact_if_stale`].
    pub claim_impact_if_stale_token: String,
    /// Before-state label (synthetic; never carries raw private material).
    pub before_state_label: String,
    /// After-state label (synthetic; never carries raw private material).
    pub after_state_label: String,
    /// Effective authority on the affected claim before the drill.
    pub before_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::before_authority`].
    pub before_authority_token: String,
    /// Effective authority on the affected claim after the drill.
    pub after_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::after_authority`].
    pub after_authority_token: String,
    /// Plain-language explanation rendered by inspectors and exports.
    pub explanation: String,
    /// Started-at timestamp.
    pub started_at: String,
    /// Resolved-at timestamp (may equal `started_at` for instantaneous
    /// failovers).
    pub resolved_at: String,
    /// Timestamp at which this drill evidence becomes stale and must be
    /// re-rehearsed.
    pub valid_until: String,
    /// Reviewable origin ref for the drill artifact file.
    pub artifact_ref: String,
    /// True when local-only work was preserved across the entire drill.
    pub local_editing_preserved: bool,
    /// True when no sibling lane's authority widened during the drill.
    pub sibling_lanes_unwidened: bool,
    /// True when raw private material is excluded from the packet.
    pub raw_private_material_excluded: bool,
    /// True when no undeclared public-endpoint fallback was used during the
    /// drill.
    pub no_public_endpoint_fallback: bool,
}

/// Typed validator defect kind for the drill baseline page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseDrillBaselineDefectKind {
    /// A token field drifted from its strongly-typed class.
    TokenDrift,
    /// A row family is missing a required drill kind (backup_restore /
    /// failover / key_rotation).
    DrillKindCoverageMissingForFamily,
    /// A drill packet widened authority on a sibling lane.
    DrillSiblingLaneWidened,
    /// A drill packet did not preserve local editing.
    DrillLocalEditingNotPreserved,
    /// A drill packet would expose raw private or secret material.
    RawPrivateMaterialExposed,
    /// A drill packet permits an undeclared public endpoint fallback.
    HiddenPublicEndpointFallback,
    /// A drill packet declared stale or missing evidence but did not declare a
    /// claim downgrade.
    StaleEvidenceWithoutDowngrade,
    /// A drill packet declared fresh evidence but also declared a claim
    /// downgrade.
    FreshEvidenceWithUnexpectedDowngrade,
    /// A drill packet's outcome did not match its drill kind.
    OutcomeDoesNotMatchDrillKind,
}

impl EnterpriseDrillBaselineDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenDrift => "token_drift",
            Self::DrillKindCoverageMissingForFamily => "drill_kind_coverage_missing_for_family",
            Self::DrillSiblingLaneWidened => "drill_sibling_lane_widened",
            Self::DrillLocalEditingNotPreserved => "drill_local_editing_not_preserved",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::StaleEvidenceWithoutDowngrade => "stale_evidence_without_downgrade",
            Self::FreshEvidenceWithUnexpectedDowngrade => {
                "fresh_evidence_with_unexpected_downgrade"
            }
            Self::OutcomeDoesNotMatchDrillKind => "outcome_does_not_match_drill_kind",
        }
    }
}

/// Typed validation defect for the drill baseline page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDrillBaselineDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: EnterpriseDrillBaselineDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (drill packet id, row family token, or "page").
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl EnterpriseDrillBaselineDefect {
    fn new(
        defect_kind: EnterpriseDrillBaselineDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: ENTERPRISE_DRILL_BASELINE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION,
            shared_contract_ref: ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the drill baseline page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDrillBaselineSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of drill packets.
    pub drill_packet_count: usize,
    /// Drill kind tokens present.
    pub drill_kinds_present: Vec<String>,
    /// Row family tokens present.
    pub row_families_present: Vec<String>,
    /// Profile tokens present.
    pub profiles_present: Vec<String>,
    /// Drill packet ids that flag claim downgrades on stale/missing evidence.
    pub claim_downgrades_present: usize,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl EnterpriseDrillBaselineSummary {
    /// Builds the summary over drill packets and defects.
    pub fn from_records(
        packets: &[EnterpriseDrillPacket],
        defects: &[EnterpriseDrillBaselineDefect],
    ) -> Self {
        let drill_kinds_present: BTreeSet<String> = packets
            .iter()
            .map(|packet| packet.drill_kind_token.clone())
            .collect();
        let row_families_present: BTreeSet<String> = packets
            .iter()
            .map(|packet| packet.row_family_token.clone())
            .collect();
        let profiles_present: BTreeSet<String> = packets
            .iter()
            .map(|packet| packet.profile_token.clone())
            .collect();
        let claim_downgrades_present = packets
            .iter()
            .filter(|packet| packet.claim_impact_if_stale.downgrades())
            .count();
        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: ENTERPRISE_DRILL_BASELINE_PAGE_RECORD_KIND.to_owned(),
            record_kind: ENTERPRISE_DRILL_BASELINE_SUMMARY_RECORD_KIND.to_owned(),
            drill_packet_count: packets.len(),
            drill_kinds_present: drill_kinds_present.into_iter().collect(),
            row_families_present: row_families_present.into_iter().collect(),
            profiles_present: profiles_present.into_iter().collect(),
            claim_downgrades_present,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level drill baseline page consumed by admin, support, shell, settings,
/// headless, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDrillBaselinePage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Drill packets covering backup-restore, failover, and key-rotation on
    /// each claimed managed / enterprise row family.
    pub drill_packets: Vec<EnterpriseDrillPacket>,
    /// Typed validation defects.
    pub defects: Vec<EnterpriseDrillBaselineDefect>,
    /// Aggregate summary.
    pub summary: EnterpriseDrillBaselineSummary,
}

/// Support-export wrapper for the drill baseline page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDrillBaselineSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported drill baseline page.
    pub page: EnterpriseDrillBaselinePage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when no undeclared public-endpoint fallback applied to any drill.
    pub no_public_endpoint_fallback_invariant: bool,
    /// True when local editing was preserved across every drill.
    pub local_editing_preserved_invariant: bool,
    /// True when sibling lanes never widened across any drill.
    pub sibling_lanes_unwidened_invariant: bool,
}

impl EnterpriseDrillBaselineSupportExport {
    /// Builds a support-export wrapper from a drill baseline page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: EnterpriseDrillBaselinePage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        let no_public_endpoint_fallback_invariant = page
            .drill_packets
            .iter()
            .all(|packet| packet.no_public_endpoint_fallback);
        let local_editing_preserved_invariant = page
            .drill_packets
            .iter()
            .all(|packet| packet.local_editing_preserved);
        let sibling_lanes_unwidened_invariant = page
            .drill_packets
            .iter()
            .all(|packet| packet.sibling_lanes_unwidened);
        Self {
            record_kind: ENTERPRISE_DRILL_BASELINE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION,
            shared_contract_ref: ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_private_material_excluded: true,
            no_public_endpoint_fallback_invariant,
            local_editing_preserved_invariant,
            sibling_lanes_unwidened_invariant,
        }
    }
}

/// Validates the drill baseline page and returns typed defects on failure.
pub fn validate_enterprise_drill_baseline_page(
    page: &EnterpriseDrillBaselinePage,
) -> Result<(), Vec<EnterpriseDrillBaselineDefect>> {
    let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for the drill baseline page given its drill packets.
pub fn audit_enterprise_drill_baseline_page(
    packets: &[EnterpriseDrillPacket],
) -> Vec<EnterpriseDrillBaselineDefect> {
    let mut defects = Vec::new();
    for packet in packets {
        audit_drill_packet(packet, &mut defects);
    }

    let mut coverage: BTreeMap<EnterpriseRowFamilyClass, BTreeSet<EnterpriseDrillKindClass>> =
        BTreeMap::new();
    for packet in packets {
        coverage
            .entry(packet.row_family)
            .or_default()
            .insert(packet.drill_kind);
    }
    for family in EnterpriseRowFamilyClass::ALL {
        let observed = coverage.get(&family);
        for required in EnterpriseDrillKindClass::ALL {
            let covered = observed.map(|kinds| kinds.contains(&required)).unwrap_or(false);
            if !covered {
                defects.push(EnterpriseDrillBaselineDefect::new(
                    EnterpriseDrillBaselineDefectKind::DrillKindCoverageMissingForFamily,
                    family.as_str(),
                    "drill_packets",
                    format!(
                        "row family {} is missing a {} drill packet",
                        family.as_str(),
                        required.as_str()
                    ),
                ));
            }
        }
    }

    defects
}

fn audit_drill_packet(
    packet: &EnterpriseDrillPacket,
    defects: &mut Vec<EnterpriseDrillBaselineDefect>,
) {
    if packet.drill_kind_token != packet.drill_kind.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "drill_kind_token",
            "drill_kind_token must match drill_kind",
        ));
    }
    if packet.row_family_token != packet.row_family.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "row_family_token",
            "row_family_token must match row_family",
        ));
    }
    if packet.profile_token != packet.profile.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if packet.outcome_token != packet.outcome.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "outcome_token",
            "outcome_token must match outcome",
        ));
    }
    if packet.evidence_freshness_token != packet.evidence_freshness.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "evidence_freshness_token",
            "evidence_freshness_token must match evidence_freshness",
        ));
    }
    if packet.claim_impact_if_stale_token != packet.claim_impact_if_stale.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "claim_impact_if_stale_token",
            "claim_impact_if_stale_token must match claim_impact_if_stale",
        ));
    }
    if packet.before_authority_token != packet.before_authority.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "before_authority_token",
            "before_authority_token must match before_authority",
        ));
    }
    if packet.after_authority_token != packet.after_authority.as_str() {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "after_authority_token",
            "after_authority_token must match after_authority",
        ));
    }

    if !outcome_matches_drill_kind(packet.drill_kind, packet.outcome) {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::OutcomeDoesNotMatchDrillKind,
            packet.drill_packet_id.clone(),
            "outcome",
            "outcome must match drill kind",
        ));
    }

    if packet.evidence_freshness.requires_downgrade()
        && !packet.claim_impact_if_stale.downgrades()
    {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::StaleEvidenceWithoutDowngrade,
            packet.drill_packet_id.clone(),
            "claim_impact_if_stale",
            "stale or missing evidence must declare a claim downgrade",
        ));
    }
    if packet.evidence_freshness == EnterpriseDrillEvidenceFreshnessClass::Fresh
        && packet.claim_impact_if_stale.downgrades()
    {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::FreshEvidenceWithUnexpectedDowngrade,
            packet.drill_packet_id.clone(),
            "claim_impact_if_stale",
            "fresh evidence must not assert a downgrade",
        ));
    }

    if !packet.sibling_lanes_unwidened {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::DrillSiblingLaneWidened,
            packet.drill_packet_id.clone(),
            "sibling_lanes_unwidened",
            "drill must not widen authority on sibling lanes",
        ));
    }
    if !packet.local_editing_preserved {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::DrillLocalEditingNotPreserved,
            packet.drill_packet_id.clone(),
            "local_editing_preserved",
            "drill must preserve local editing",
        ));
    }
    if !packet.raw_private_material_excluded {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::RawPrivateMaterialExposed,
            packet.drill_packet_id.clone(),
            "raw_private_material_excluded",
            "drill packet must exclude raw private material",
        ));
    }
    if !packet.no_public_endpoint_fallback {
        defects.push(EnterpriseDrillBaselineDefect::new(
            EnterpriseDrillBaselineDefectKind::HiddenPublicEndpointFallback,
            packet.drill_packet_id.clone(),
            "no_public_endpoint_fallback",
            "drill must not silently fall back to a public endpoint",
        ));
    }
}

const fn outcome_matches_drill_kind(
    kind: EnterpriseDrillKindClass,
    outcome: EnterpriseDrillOutcomeClass,
) -> bool {
    match kind {
        EnterpriseDrillKindClass::BackupRestore => matches!(
            outcome,
            EnterpriseDrillOutcomeClass::RestoredFromTrustedSnapshot
                | EnterpriseDrillOutcomeClass::DowngradedAwaitingAdmin
        ),
        EnterpriseDrillKindClass::Failover => matches!(
            outcome,
            EnterpriseDrillOutcomeClass::FailedOverToDeclaredFallback
                | EnterpriseDrillOutcomeClass::DowngradedAwaitingAdmin
        ),
        EnterpriseDrillKindClass::KeyRotation => matches!(
            outcome,
            EnterpriseDrillOutcomeClass::RotatedThenRecovered
                | EnterpriseDrillOutcomeClass::DowngradedAwaitingAdmin
        ),
    }
}

/// Builds the seeded drill baseline page covering backup-restore, failover,
/// and key-rotation drills on each claimed managed / enterprise row family.
pub fn seeded_enterprise_drill_baseline_page() -> EnterpriseDrillBaselinePage {
    let drill_packets = seed_drill_packets();
    let defects = audit_enterprise_drill_baseline_page(&drill_packets);
    let summary = EnterpriseDrillBaselineSummary::from_records(&drill_packets, &defects);
    EnterpriseDrillBaselinePage {
        record_kind: ENTERPRISE_DRILL_BASELINE_PAGE_RECORD_KIND.to_owned(),
        schema_version: ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION,
        shared_contract_ref: ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: ENTERPRISE_DRILL_BASELINE_SOURCE_MATRIX_REF.to_owned(),
        drill_packets,
        defects,
        summary,
    }
}

fn seed_drill_packets() -> Vec<EnterpriseDrillPacket> {
    vec![
        drill_packet(
            "enterprise-drill:managed-policy-distribution:backup-restore-001",
            EnterpriseDrillKindClass::BackupRestore,
            EnterpriseRowFamilyClass::ManagedPolicyDistribution,
            "claim:policy-pack:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::MirrorOnly,
            EnterpriseDrillOutcomeClass::RestoredFromTrustedSnapshot,
            EnterpriseDrillEvidenceFreshnessClass::Fresh,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "policy_pack:active:revision-2026-05-01",
            "policy_pack:restored:revision-2026-05-01_from_signed_mirror",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::Allowed,
            "Managed policy pack was restored on the signed mirror profile from the last signed pack snapshot. Authority on the claim recovered to the same allowed posture and sibling lanes were untouched.",
            "2026-05-15T10:00:00Z",
            "2026-05-15T10:14:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/managed_policy_distribution_backup_restore_001.json",
        ),
        drill_packet(
            "enterprise-drill:managed-policy-distribution:failover-001",
            EnterpriseDrillKindClass::Failover,
            EnterpriseRowFamilyClass::ManagedPolicyDistribution,
            "claim:policy-pack:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::Connected,
            EnterpriseDrillOutcomeClass::FailedOverToDeclaredFallback,
            EnterpriseDrillEvidenceFreshnessClass::StaleWithinWindow,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "policy_distributor:primary:us-east-1",
            "policy_distributor:declared_fallback:us-east-2",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::Allowed,
            "Primary policy distributor failed over to the declared US-East-2 secondary while the same signed manifest stayed in effect. Sibling lanes (provider credential, identity session) were not widened.",
            "2026-05-15T11:00:00Z",
            "2026-05-15T11:05:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/managed_policy_distribution_failover_001.json",
        ),
        drill_packet(
            "enterprise-drill:managed-policy-distribution:key-rotation-001",
            EnterpriseDrillKindClass::KeyRotation,
            EnterpriseRowFamilyClass::ManagedPolicyDistribution,
            "claim:policy-pack:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::EnterpriseManaged,
            EnterpriseDrillOutcomeClass::RotatedThenRecovered,
            EnterpriseDrillEvidenceFreshnessClass::StaleBeyondWindow,
            EnterpriseDrillClaimImpactClass::DowngradeFamilyClaims,
            "policy_pack_signing_key:revision-N",
            "policy_pack_signing_key:revision-N+1",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "Policy pack signing key rotated; current evidence is older than the hard recheck window, so every managed policy distribution claim is downgraded to approval-required until the rotation is re-rehearsed and the signed mirror catches up.",
            "2026-02-15T09:00:00Z",
            "2026-02-15T09:18:00Z",
            "2026-05-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/managed_policy_distribution_key_rotation_001.json",
        ),
        drill_packet(
            "enterprise-drill:managed-credential-handle:backup-restore-001",
            EnterpriseDrillKindClass::BackupRestore,
            EnterpriseRowFamilyClass::ManagedCredentialHandle,
            "claim:secret-broker:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::Offline,
            EnterpriseDrillOutcomeClass::DowngradedAwaitingAdmin,
            EnterpriseDrillEvidenceFreshnessClass::Missing,
            EnterpriseDrillClaimImpactClass::DowngradeAffectedClaim,
            "vault_handle_index:last_known_good",
            "vault_handle_index:restore_blocked_no_courier",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::BlockedPendingTrust,
            "Air-gapped offline profile could not restore the managed credential handle index because the signed courier evidence was missing. The affected claim is downgraded to blocked-pending-trust until admin imports a fresh courier; sibling lanes stayed at their declared offline authority.",
            "2026-05-15T08:00:00Z",
            "2026-05-15T08:12:00Z",
            "2026-05-22T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/managed_credential_handle_backup_restore_001.json",
        ),
        drill_packet(
            "enterprise-drill:managed-credential-handle:failover-001",
            EnterpriseDrillKindClass::Failover,
            EnterpriseRowFamilyClass::ManagedCredentialHandle,
            "claim:secret-broker:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::Connected,
            EnterpriseDrillOutcomeClass::FailedOverToDeclaredFallback,
            EnterpriseDrillEvidenceFreshnessClass::Fresh,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "vault_broker:primary",
            "vault_broker:declared_fallback",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::Allowed,
            "Primary vault broker failed over to the declared secondary broker. Handle issuance recovered without exposing raw secret material; sibling lanes (policy distribution, identity session) were not widened.",
            "2026-05-15T11:30:00Z",
            "2026-05-15T11:33:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/managed_credential_handle_failover_001.json",
        ),
        drill_packet(
            "enterprise-drill:managed-credential-handle:key-rotation-001",
            EnterpriseDrillKindClass::KeyRotation,
            EnterpriseRowFamilyClass::ManagedCredentialHandle,
            "claim:secret-broker:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::EnterpriseManaged,
            EnterpriseDrillOutcomeClass::RotatedThenRecovered,
            EnterpriseDrillEvidenceFreshnessClass::Fresh,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "vault_wrap_key:revision-N",
            "vault_wrap_key:revision-N+1",
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "Customer-managed wrap key for the vault broker rotated; every handle remained handle-only and the claim's approval-required authority was preserved.",
            "2026-05-15T12:00:00Z",
            "2026-05-15T12:21:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/managed_credential_handle_key_rotation_001.json",
        ),
        drill_packet(
            "enterprise-drill:enterprise-identity-session:backup-restore-001",
            EnterpriseDrillKindClass::BackupRestore,
            EnterpriseRowFamilyClass::EnterpriseIdentitySession,
            "claim:oidc-session:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::MirrorOnly,
            EnterpriseDrillOutcomeClass::RestoredFromTrustedSnapshot,
            EnterpriseDrillEvidenceFreshnessClass::Fresh,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "oidc_session_anchor:last_signed_snapshot",
            "oidc_session_anchor:restored_from_signed_mirror",
            CapabilityAuthorityClass::ReadOnly,
            CapabilityAuthorityClass::ReadOnly,
            "Enterprise identity session anchor was restored from the signed mirror snapshot. The claim returned to read-only posture (the mirror profile's declared authority) and sibling lanes were unaffected.",
            "2026-05-15T13:00:00Z",
            "2026-05-15T13:09:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/enterprise_identity_session_backup_restore_001.json",
        ),
        drill_packet(
            "enterprise-drill:enterprise-identity-session:failover-001",
            EnterpriseDrillKindClass::Failover,
            EnterpriseRowFamilyClass::EnterpriseIdentitySession,
            "claim:oidc-session:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::Connected,
            EnterpriseDrillOutcomeClass::FailedOverToDeclaredFallback,
            EnterpriseDrillEvidenceFreshnessClass::Fresh,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "oidc_issuer:primary",
            "oidc_issuer:declared_fallback",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::Allowed,
            "Primary OIDC issuer failed over to the declared secondary issuer (same tenant binding). The claim's allowed posture survived the failover; sibling lanes (policy distribution, credential handle) were not widened.",
            "2026-05-15T14:00:00Z",
            "2026-05-15T14:02:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/enterprise_identity_session_failover_001.json",
        ),
        drill_packet(
            "enterprise-drill:enterprise-identity-session:key-rotation-001",
            EnterpriseDrillKindClass::KeyRotation,
            EnterpriseRowFamilyClass::EnterpriseIdentitySession,
            "claim:oidc-session:enterprise-pilot-alpha",
            EnterpriseDrillProfileClass::EnterpriseManaged,
            EnterpriseDrillOutcomeClass::RotatedThenRecovered,
            EnterpriseDrillEvidenceFreshnessClass::Fresh,
            EnterpriseDrillClaimImpactClass::NoImpact,
            "oidc_signing_key:revision-N",
            "oidc_signing_key:revision-N+1",
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "Enterprise OIDC signing key rotated. The session continuity packet's signed-at moved to the rotated key and the claim's approval-required posture was preserved.",
            "2026-05-15T15:00:00Z",
            "2026-05-15T15:24:00Z",
            "2026-08-15T00:00:00Z",
            "artifacts/security/m3/backup_restore_failover_drills/enterprise_identity_session_key_rotation_001.json",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn drill_packet(
    drill_packet_id: &str,
    drill_kind: EnterpriseDrillKindClass,
    row_family: EnterpriseRowFamilyClass,
    affected_claim_id: &str,
    profile: EnterpriseDrillProfileClass,
    outcome: EnterpriseDrillOutcomeClass,
    evidence_freshness: EnterpriseDrillEvidenceFreshnessClass,
    claim_impact_if_stale: EnterpriseDrillClaimImpactClass,
    before_state_label: &str,
    after_state_label: &str,
    before_authority: CapabilityAuthorityClass,
    after_authority: CapabilityAuthorityClass,
    explanation: &str,
    started_at: &str,
    resolved_at: &str,
    valid_until: &str,
    artifact_ref: &str,
) -> EnterpriseDrillPacket {
    EnterpriseDrillPacket {
        record_kind: ENTERPRISE_DRILL_BASELINE_PACKET_RECORD_KIND.to_owned(),
        schema_version: ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION,
        shared_contract_ref: ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF.to_owned(),
        drill_packet_id: drill_packet_id.to_owned(),
        drill_kind,
        drill_kind_token: drill_kind.as_str().to_owned(),
        row_family,
        row_family_token: row_family.as_str().to_owned(),
        affected_claim_id: affected_claim_id.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        outcome,
        outcome_token: outcome.as_str().to_owned(),
        evidence_freshness,
        evidence_freshness_token: evidence_freshness.as_str().to_owned(),
        claim_impact_if_stale,
        claim_impact_if_stale_token: claim_impact_if_stale.as_str().to_owned(),
        before_state_label: before_state_label.to_owned(),
        after_state_label: after_state_label.to_owned(),
        before_authority,
        before_authority_token: before_authority.as_str().to_owned(),
        after_authority,
        after_authority_token: after_authority.as_str().to_owned(),
        explanation: explanation.to_owned(),
        started_at: started_at.to_owned(),
        resolved_at: resolved_at.to_owned(),
        valid_until: valid_until.to_owned(),
        artifact_ref: artifact_ref.to_owned(),
        local_editing_preserved: true,
        sibling_lanes_unwidened: true,
        raw_private_material_excluded: true,
        no_public_endpoint_fallback: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_enterprise_drill_baseline_page();
        validate_enterprise_drill_baseline_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        assert_eq!(
            page.drill_packets.len(),
            EnterpriseRowFamilyClass::ALL.len() * EnterpriseDrillKindClass::ALL.len()
        );
    }

    #[test]
    fn each_row_family_has_every_drill_kind() {
        let page = seeded_enterprise_drill_baseline_page();
        for family in EnterpriseRowFamilyClass::ALL {
            for kind in EnterpriseDrillKindClass::ALL {
                let count = page
                    .drill_packets
                    .iter()
                    .filter(|packet| packet.row_family == family && packet.drill_kind == kind)
                    .count();
                assert!(
                    count >= 1,
                    "row family {} is missing a {} drill packet",
                    family.as_str(),
                    kind.as_str()
                );
            }
        }
    }

    #[test]
    fn validator_flags_missing_drill_kind_on_family() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets.retain(|packet| {
            !(packet.row_family == EnterpriseRowFamilyClass::ManagedCredentialHandle
                && packet.drill_kind == EnterpriseDrillKindClass::KeyRotation)
        });
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::DrillKindCoverageMissingForFamily
            && defect.subject_id == "managed_credential_handle"));
    }

    #[test]
    fn validator_flags_stale_evidence_without_downgrade() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets[0].evidence_freshness =
            EnterpriseDrillEvidenceFreshnessClass::StaleBeyondWindow;
        page.drill_packets[0].evidence_freshness_token =
            EnterpriseDrillEvidenceFreshnessClass::StaleBeyondWindow
                .as_str()
                .to_owned();
        page.drill_packets[0].claim_impact_if_stale =
            EnterpriseDrillClaimImpactClass::NoImpact;
        page.drill_packets[0].claim_impact_if_stale_token =
            EnterpriseDrillClaimImpactClass::NoImpact.as_str().to_owned();
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::StaleEvidenceWithoutDowngrade));
    }

    #[test]
    fn validator_flags_sibling_widened_drill() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets[0].sibling_lanes_unwidened = false;
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::DrillSiblingLaneWidened));
    }

    #[test]
    fn validator_flags_public_endpoint_fallback() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets[0].no_public_endpoint_fallback = false;
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::HiddenPublicEndpointFallback));
    }

    #[test]
    fn validator_flags_raw_private_material_exposed() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets[0].raw_private_material_excluded = false;
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::RawPrivateMaterialExposed));
    }

    #[test]
    fn validator_flags_outcome_mismatch() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets[0].outcome = EnterpriseDrillOutcomeClass::FailedOverToDeclaredFallback;
        page.drill_packets[0].outcome_token = EnterpriseDrillOutcomeClass::FailedOverToDeclaredFallback
            .as_str()
            .to_owned();
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::OutcomeDoesNotMatchDrillKind));
    }

    #[test]
    fn validator_flags_fresh_evidence_with_unexpected_downgrade() {
        let mut page = seeded_enterprise_drill_baseline_page();
        page.drill_packets[0].claim_impact_if_stale =
            EnterpriseDrillClaimImpactClass::DowngradeAffectedClaim;
        page.drill_packets[0].claim_impact_if_stale_token =
            EnterpriseDrillClaimImpactClass::DowngradeAffectedClaim
                .as_str()
                .to_owned();
        let defects = audit_enterprise_drill_baseline_page(&page.drill_packets);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == EnterpriseDrillBaselineDefectKind::FreshEvidenceWithUnexpectedDowngrade));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_enterprise_drill_baseline_page();
        let export = EnterpriseDrillBaselineSupportExport::from_page(
            "enterprise-drill-baseline:support-export:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.no_public_endpoint_fallback_invariant);
        assert!(export.local_editing_preserved_invariant);
        assert!(export.sibling_lanes_unwidened_invariant);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
    }

    #[test]
    fn summary_counts_downgrades_consistently() {
        let page = seeded_enterprise_drill_baseline_page();
        let expected = page
            .drill_packets
            .iter()
            .filter(|packet| packet.claim_impact_if_stale.downgrades())
            .count();
        assert_eq!(page.summary.claim_downgrades_present, expected);
    }
}
