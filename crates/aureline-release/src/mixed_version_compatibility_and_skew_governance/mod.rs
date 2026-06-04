//! Stable mixed-version compatibility and skew governance for release boundaries.
//!
//! This module consumes the checked-in stable mixed-version boundary matrix at
//! `artifacts/compat/stable/mixed-version-compatibility-and-skew-governance.json`.
//! The matrix is the release-facing source of truth for distributed and
//! file/schema boundaries: negotiated fields, supported skew window, upgrade
//! order, rollback order, downgrade posture, user-visible unsupported labels,
//! and the fail-closed behavior when skew is outside the published window.
//! Rows whose skew evidence is stale, missing, unsigned, or not bound into the
//! declared release-center/support-export consumers must narrow below Stable
//! before publication.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, StableClaimLevel,
};

/// Supported mixed-version governance schema version.
pub const MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_RECORD_KIND: &str =
    "mixed_version_compatibility_and_skew_governance";

/// Repo-relative path to the checked-in matrix.
pub const MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_PATH: &str =
    "artifacts/compat/stable/mixed-version-compatibility-and-skew-governance.json";

/// Embedded checked-in matrix JSON.
pub const MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/compat/stable/mixed-version-compatibility-and-skew-governance.json"
));

/// Boundary family governed by the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryFamily {
    /// Launcher and local helper/sidecar processes.
    LauncherAndLocalSidecars,
    /// Desktop or CLI client attached to a remote agent/helper.
    DesktopCliAndRemoteAgent,
    /// Desktop, CLI, or browser client speaking to the managed control plane.
    ManagedControlPlane,
    /// Extension host, SDK, ABI, runtime, and permission vocabulary.
    ExtensionHostAndSdk,
    /// Saved artifacts, state bundles, and schema readers/writers.
    SchemaOrStateBundle,
    /// Provider adapters and provider-linked packets.
    ProviderLinkedPacket,
    /// Audit or event producers and consumers.
    AuditOrEventProducerConsumer,
}

impl BoundaryFamily {
    /// Every required stable boundary family.
    pub const ALL: [Self; 7] = [
        Self::LauncherAndLocalSidecars,
        Self::DesktopCliAndRemoteAgent,
        Self::ManagedControlPlane,
        Self::ExtensionHostAndSdk,
        Self::SchemaOrStateBundle,
        Self::ProviderLinkedPacket,
        Self::AuditOrEventProducerConsumer,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LauncherAndLocalSidecars => "launcher_and_local_sidecars",
            Self::DesktopCliAndRemoteAgent => "desktop_cli_and_remote_agent",
            Self::ManagedControlPlane => "managed_control_plane",
            Self::ExtensionHostAndSdk => "extension_host_and_sdk",
            Self::SchemaOrStateBundle => "schema_or_state_bundle",
            Self::ProviderLinkedPacket => "provider_linked_packet",
            Self::AuditOrEventProducerConsumer => "audit_or_event_producer_consumer",
        }
    }
}

/// Supported skew-window class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkewWindowClass {
    /// Components promote and roll back as a coordinated artifact set.
    CoordinatedArtifactSetOnly,
    /// Adjacent client/agent versions are explicitly declared and drilled.
    DeclaredAdjacentWindow,
    /// Current line plus declared previous minor or LTS.
    CurrentPlusPreviousMinorOrLts,
    /// SDK support window published for host/runtime/ABI.
    PublishedSdkSupportWindow,
    /// Additive-only compatibility within one schema epoch.
    SameSchemaEpochAdditiveOnly,
}

impl SkewWindowClass {
    /// Every skew-window class.
    pub const ALL: [Self; 5] = [
        Self::CoordinatedArtifactSetOnly,
        Self::DeclaredAdjacentWindow,
        Self::CurrentPlusPreviousMinorOrLts,
        Self::PublishedSdkSupportWindow,
        Self::SameSchemaEpochAdditiveOnly,
    ];
}

/// The row state earned by skew-window evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceState {
    /// Current matrix row, skew-window drill, rollback proof, owner signoff, and bindings.
    QualifiedStable,
    /// The boundary is current but intentionally coordinated-upgrade-only.
    QualifiedCoordinatedUpgradeOnly,
    /// The row narrows because its skew-window drill or rollback proof is stale.
    NarrowedStaleSkewEvidence,
    /// The row narrows because a required matrix/skew/proof row is missing.
    NarrowedMissingSkewRow,
    /// The row narrows because release-center/support-export bindings are incomplete.
    NarrowedBindingIncomplete,
    /// The row narrows because the backing public claim is already below the cutline.
    NarrowedClaimNarrowed,
}

impl GovernanceState {
    /// Every governance state.
    pub const ALL: [Self; 6] = [
        Self::QualifiedStable,
        Self::QualifiedCoordinatedUpgradeOnly,
        Self::NarrowedStaleSkewEvidence,
        Self::NarrowedMissingSkewRow,
        Self::NarrowedBindingIncomplete,
        Self::NarrowedClaimNarrowed,
    ];

    /// Returns true when the state can publish its claim at the stable cutline.
    pub const fn holds_claim(self) -> bool {
        matches!(
            self,
            Self::QualifiedStable | Self::QualifiedCoordinatedUpgradeOnly
        )
    }
}

/// Unsupported-state label surfaced by attach/open/install/restore flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedFlowLabel {
    /// A coordinated upgrade or rollback is required.
    CoordinatedUpgradeRequired,
    /// The current version combination is outside the supported skew window.
    UnsupportedSkew,
    /// The flow narrows to read-only downgrade.
    ReadOnlyDowngrade,
    /// The flow narrows to file-only fallback.
    FileOnlyFallback,
    /// The extension is disabled or quarantined.
    ExtensionQuarantined,
    /// Ingest/open fails with an attributed compatibility error.
    AttributedCompatibilityError,
}

impl UnsupportedFlowLabel {
    /// Every unsupported-flow label.
    pub const ALL: [Self; 6] = [
        Self::CoordinatedUpgradeRequired,
        Self::UnsupportedSkew,
        Self::ReadOnlyDowngrade,
        Self::FileOnlyFallback,
        Self::ExtensionQuarantined,
        Self::AttributedCompatibilityError,
    ];
}

/// Closed reason a row narrows or a governance rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The backing public claim is already below the cutline.
    ClaimLabelNarrowed,
    /// No boundary matrix row or skew-window row is current.
    BoundaryMatrixRowMissing,
    /// The skew-window drill is stale or absent.
    SkewWindowDrillStale,
    /// The rollback-order proof is stale or absent.
    RollbackProofStale,
    /// Required release-center or support-export binding is absent.
    DownstreamBindingIncomplete,
    /// Owner signoff is absent.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every gap reason.
    pub const ALL: [Self; 6] = [
        Self::ClaimLabelNarrowed,
        Self::BoundaryMatrixRowMissing,
        Self::SkewWindowDrillStale,
        Self::RollbackProofStale,
        Self::DownstreamBindingIncomplete,
        Self::OwnerSignoffMissing,
    ];
}

/// One governance rule for widening mixed-version claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// Gap reason that fires this rule.
    pub trigger_reason: GapReason,
    /// Claimed labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// Published skew window for a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportedSkewWindow {
    /// Stable skew-window ref.
    pub skew_window_ref: String,
    /// Skew-window class.
    pub window_class: SkewWindowClass,
    /// Reviewable summary rendered by docs and support exports.
    pub summary: String,
    /// True when this boundary claims rolling adjacent-version operation.
    pub rolling_upgrade_supported: bool,
}

/// Upgrade or rollback order for a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentOrder {
    /// Ordered component families.
    pub declared_order: Vec<String>,
    /// Reviewable order note.
    pub notes: String,
}

/// Downgrade posture for a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradePosture {
    /// Closed support-class token from the compatibility seed.
    pub support_class: String,
    /// State-preservation note rendered in support/export contexts.
    pub state_preservation_note: String,
}

/// Unsupported behavior for out-of-window skew.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnsupportedBehavior {
    /// User/support-visible controlled label.
    pub label: UnsupportedFlowLabel,
    /// Closed posture token from compatibility seeds.
    pub out_of_window_posture: String,
    /// Exact contract rule rendered by release/docs/support consumers.
    pub fail_closed_behavior: String,
}

/// Downstream bindings that must all consume this row rather than clone text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownstreamBindings {
    /// Release-center row ref that gates claim widening.
    pub release_center_ref: String,
    /// Compatibility packet ref that carries drill evidence.
    pub compatibility_packet_ref: String,
    /// Support export ref that quotes this row.
    pub support_export_ref: String,
    /// Help/About ref that shows user-inspectable truth.
    pub help_about_ref: String,
}

impl DownstreamBindings {
    /// True when every binding required by the task is present.
    pub fn is_complete(&self) -> bool {
        !self.release_center_ref.trim().is_empty()
            && !self.compatibility_packet_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
            && !self.help_about_ref.trim().is_empty()
    }
}

/// One boundary row in the mixed-version matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryRow {
    /// Stable boundary row id.
    pub boundary_row_id: String,
    /// Human-readable title.
    pub title: String,
    /// Boundary family.
    pub boundary_family: BoundaryFamily,
    /// Producer side of the boundary.
    pub producer_surface: String,
    /// Consumer side of the boundary.
    pub consumer_surface: String,
    /// The stable-claim-manifest entry this row backs.
    pub claim_ref: String,
    /// Canonical public claim label; this is a hard ceiling.
    pub claim_label: StableClaimLevel,
    /// Effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Governance state earned by the row.
    pub governance_state: GovernanceState,
    /// Qualification row ref.
    pub qualification_row_ref: String,
    /// Version-skew register ref.
    pub version_skew_register_ref: String,
    /// Current supported skew case ref.
    pub current_skew_case_ref: String,
    /// Unsupported skew case ref used by fail-closed drills.
    pub unsupported_skew_case_ref: String,
    /// Negotiated fields on this boundary.
    pub negotiated_fields: Vec<String>,
    /// Published skew window.
    pub supported_skew_window: SupportedSkewWindow,
    /// Upgrade order.
    pub upgrade_order: ComponentOrder,
    /// Rollback order.
    pub rollback_order: ComponentOrder,
    /// Downgrade posture.
    pub downgrade_posture: DowngradePosture,
    /// Unsupported behavior for out-of-window skew.
    pub unsupported_behavior: UnsupportedBehavior,
    /// Proof packet for skew-window drill.
    pub skew_window_drill_packet: ProofPacket,
    /// Proof packet for rollback-order drill.
    pub rollback_order_packet: ProofPacket,
    /// Downstream bindings.
    pub downstream_bindings: DownstreamBindings,
    /// Owner signoff.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// Reviewable row rationale.
    pub rationale: String,
}

impl BoundaryRow {
    /// True when the row effectively publishes at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the backing claim is at or above the stable cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when all proof packets required for a mixed-version claim are current.
    pub fn proof_packets_current(&self) -> bool {
        self.skew_window_drill_packet.slo_state == FreshnessSloState::Current
            && self.rollback_order_packet.slo_state == FreshnessSloState::Current
    }

    /// True when the row can carry its claim label.
    pub fn holds_claim(&self) -> bool {
        self.governance_state.holds_claim()
    }
}

/// Publication verdict for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernancePublication {
    /// Gate name.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Firing blocking rule ids.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Boundary ids causing the hold.
    #[serde(default)]
    pub blocking_boundary_row_ids: Vec<String>,
    /// Reviewable summary.
    pub rationale: String,
}

/// Matrix summary counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceSummary {
    /// Total boundary rows.
    pub total_boundaries: usize,
    /// Rows publishing at or above the cutline.
    pub boundaries_stable: usize,
    /// Rows narrowed below the cutline.
    pub boundaries_narrowed: usize,
    /// Rows with rolling-upgrade support.
    pub rolling_upgrade_supported: usize,
    /// Coordinated-upgrade-only rows.
    pub coordinated_upgrade_only: usize,
    /// Current skew-window drill packets.
    pub skew_drills_current: usize,
    /// Stale or missing skew-window drill packets.
    pub skew_drills_not_current: usize,
    /// Current rollback-order packets.
    pub rollback_proofs_current: usize,
    /// Stale or missing rollback-order packets.
    pub rollback_proofs_not_current: usize,
    /// Governance rules currently firing.
    pub governance_rules_firing: usize,
}

/// The typed mixed-version governance matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MixedVersionCompatibilityAndSkewGovernance {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix id.
    pub governance_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this matrix ingests as its ceiling.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed boundary-family vocabulary.
    pub boundary_families: Vec<BoundaryFamily>,
    /// Closed skew-window class vocabulary.
    pub skew_window_classes: Vec<SkewWindowClass>,
    /// Closed governance-state vocabulary.
    pub governance_states: Vec<GovernanceState>,
    /// Closed unsupported-label vocabulary.
    pub unsupported_flow_labels: Vec<UnsupportedFlowLabel>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Stable launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Boundary families that must be covered before broad stable release.
    pub required_boundary_families: Vec<BoundaryFamily>,
    /// Governance rules.
    pub governance_rules: Vec<GovernanceRule>,
    /// Boundary rows.
    pub boundary_rows: Vec<BoundaryRow>,
    /// Publication verdict.
    pub publication: GovernancePublication,
    /// Summary counts.
    pub summary: GovernanceSummary,
}

impl MixedVersionCompatibilityAndSkewGovernance {
    /// Returns rows for one boundary family.
    pub fn rows_for_family(&self, family: BoundaryFamily) -> Vec<&BoundaryRow> {
        self.boundary_rows
            .iter()
            .filter(|row| row.boundary_family == family)
            .collect()
    }

    /// Returns stable-published rows.
    pub fn rows_publishing_stable(&self) -> Vec<&BoundaryRow> {
        self.boundary_rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns rows narrowed below the stable cutline.
    pub fn rows_narrowed(&self) -> Vec<&BoundaryRow> {
        self.boundary_rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// True when `rule` fires on any watched row.
    pub fn governance_rule_fires(&self, rule: &GovernanceRule) -> bool {
        self.boundary_rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.active_gap_reasons.contains(&rule.trigger_reason)
        })
    }

    /// Recomputes the publication decision from firing governance rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .governance_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.governance_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Firing blocking rule ids, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .governance_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.governance_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Boundary row ids that trigger a blocking rule, sorted.
    pub fn computed_blocking_boundary_row_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .governance_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.governance_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids = BTreeSet::new();
        for row in &self.boundary_rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.boundary_row_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes summary counts.
    pub fn computed_summary(&self) -> GovernanceSummary {
        GovernanceSummary {
            total_boundaries: self.boundary_rows.len(),
            boundaries_stable: self
                .boundary_rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            boundaries_narrowed: self
                .boundary_rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            rolling_upgrade_supported: self
                .boundary_rows
                .iter()
                .filter(|row| row.supported_skew_window.rolling_upgrade_supported)
                .count(),
            coordinated_upgrade_only: self
                .boundary_rows
                .iter()
                .filter(|row| {
                    !row.supported_skew_window.rolling_upgrade_supported
                        || row.supported_skew_window.window_class
                            == SkewWindowClass::CoordinatedArtifactSetOnly
                })
                .count(),
            skew_drills_current: self
                .boundary_rows
                .iter()
                .filter(|row| row.skew_window_drill_packet.slo_state == FreshnessSloState::Current)
                .count(),
            skew_drills_not_current: self
                .boundary_rows
                .iter()
                .filter(|row| row.skew_window_drill_packet.slo_state != FreshnessSloState::Current)
                .count(),
            rollback_proofs_current: self
                .boundary_rows
                .iter()
                .filter(|row| row.rollback_order_packet.slo_state == FreshnessSloState::Current)
                .count(),
            rollback_proofs_not_current: self
                .boundary_rows
                .iter()
                .filter(|row| row.rollback_order_packet.slo_state != FreshnessSloState::Current)
                .count(),
            governance_rules_firing: self
                .governance_rules
                .iter()
                .filter(|rule| self.governance_rule_fires(rule))
                .count(),
        }
    }

    /// Produces a Help/About and support-export-safe projection.
    pub fn support_export_projection(&self) -> GovernanceExportProjection {
        GovernanceExportProjection {
            governance_id: self.governance_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .boundary_rows
                .iter()
                .map(|row| GovernanceExportRow {
                    boundary_row_id: row.boundary_row_id.clone(),
                    boundary_family: row.boundary_family,
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    governance_state: row.governance_state,
                    negotiated_fields: row.negotiated_fields.clone(),
                    skew_window_ref: row.supported_skew_window.skew_window_ref.clone(),
                    skew_window_class: row.supported_skew_window.window_class,
                    rolling_upgrade_supported: row.supported_skew_window.rolling_upgrade_supported,
                    upgrade_order: row.upgrade_order.declared_order.clone(),
                    rollback_order: row.rollback_order.declared_order.clone(),
                    unsupported_label: row.unsupported_behavior.label,
                    fail_closed_behavior: row.unsupported_behavior.fail_closed_behavior.clone(),
                    release_center_ref: row.downstream_bindings.release_center_ref.clone(),
                    support_export_ref: row.downstream_bindings.support_export_ref.clone(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<GovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.boundary_rows {
            if !seen.insert(row.boundary_row_id.clone()) {
                violations.push(GovernanceViolation::DuplicateBoundaryRowId {
                    boundary_row_id: row.boundary_row_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.boundary_rows.is_empty() {
            violations.push(GovernanceViolation::EmptyMatrix);
        }
        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(GovernanceViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<GovernanceViolation>) {
        if self.schema_version != MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_SCHEMA_VERSION {
            violations.push(GovernanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_RECORD_KIND {
            violations.push(GovernanceViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("governance_id", &self.governance_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(GovernanceViolation::EmptyField {
                    boundary_row_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.boundary_families != BoundaryFamily::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "boundary_families",
            });
        }
        if self.skew_window_classes != SkewWindowClass::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "skew_window_classes",
            });
        }
        if self.governance_states != GovernanceState::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "governance_states",
            });
        }
        if self.unsupported_flow_labels != UnsupportedFlowLabel::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "unsupported_flow_labels",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.launch_cutline.cutline_level != StableClaimLevel::Stable
            || self.launch_cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec()
            || self.launch_cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec()
            || self.launch_cutline.description.trim().is_empty()
        {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "launch_cutline",
            });
        }
        if self.required_boundary_families != BoundaryFamily::ALL.to_vec() {
            violations.push(GovernanceViolation::ClosedVocabularyMismatch {
                field: "required_boundary_families",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<GovernanceViolation>) {
        if self.governance_rules.is_empty() {
            violations.push(GovernanceViolation::NoGovernanceRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.governance_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(GovernanceViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
            if rule.rule_id.trim().is_empty()
                || rule.title.trim().is_empty()
                || rule.rationale.trim().is_empty()
                || rule.applies_to_labels.is_empty()
            {
                violations.push(GovernanceViolation::IncompleteRule {
                    rule_id: rule.rule_id.clone(),
                });
            }
        }
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(GovernanceViolation::RuleMissingForGapReason { reason });
            }
        }
    }

    fn validate_row(&self, row: &BoundaryRow, violations: &mut Vec<GovernanceViolation>) {
        for (field, value) in [
            ("boundary_row_id", &row.boundary_row_id),
            ("title", &row.title),
            ("producer_surface", &row.producer_surface),
            ("consumer_surface", &row.consumer_surface),
            ("claim_ref", &row.claim_ref),
            ("qualification_row_ref", &row.qualification_row_ref),
            ("version_skew_register_ref", &row.version_skew_register_ref),
            ("current_skew_case_ref", &row.current_skew_case_ref),
            ("unsupported_skew_case_ref", &row.unsupported_skew_case_ref),
            (
                "skew_window_ref",
                &row.supported_skew_window.skew_window_ref,
            ),
            (
                "unsupported_behavior.fail_closed_behavior",
                &row.unsupported_behavior.fail_closed_behavior,
            ),
            ("rationale", &row.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(GovernanceViolation::EmptyField {
                    boundary_row_id: row.boundary_row_id.clone(),
                    field_name: field,
                });
            }
        }
        if row.negotiated_fields.is_empty()
            || row.upgrade_order.declared_order.is_empty()
            || row.rollback_order.declared_order.is_empty()
            || row.supported_skew_window.summary.trim().is_empty()
            || row.upgrade_order.notes.trim().is_empty()
            || row.rollback_order.notes.trim().is_empty()
            || row.downgrade_posture.support_class.trim().is_empty()
            || row
                .downgrade_posture
                .state_preservation_note
                .trim()
                .is_empty()
        {
            violations.push(GovernanceViolation::IncompleteBoundaryRow {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(GovernanceViolation::EffectiveLabelWiderThanClaim {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if !row.holds_claim() && row.claim_holds_stable() && row.publishes_stable() {
            violations.push(GovernanceViolation::NarrowingRowNotNarrowed {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if row.holds_claim() && !row.proof_packets_current() {
            violations.push(GovernanceViolation::HeldOnStaleProof {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if row.holds_claim() && !row.downstream_bindings.is_complete() {
            violations.push(GovernanceViolation::HeldWithoutDownstreamBindings {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if row.holds_claim() && !row.owner_signoff.signed_off {
            violations.push(GovernanceViolation::HeldWithoutOwnerSignoff {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if row.claim_holds_stable() && !row.holds_claim() && row.active_gap_reasons.is_empty() {
            violations.push(GovernanceViolation::NarrowedWithoutGapReason {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
        if !row.supported_skew_window.rolling_upgrade_supported
            && row.supported_skew_window.window_class != SkewWindowClass::CoordinatedArtifactSetOnly
            && row.unsupported_behavior.label != UnsupportedFlowLabel::CoordinatedUpgradeRequired
        {
            violations.push(GovernanceViolation::CoordinatedOnlyWithoutVisibleLabel {
                boundary_row_id: row.boundary_row_id.clone(),
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<GovernanceViolation>) {
        for family in &self.required_boundary_families {
            if self.rows_for_family(*family).is_empty() {
                violations.push(GovernanceViolation::RequiredBoundaryMissing { family: *family });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<GovernanceViolation>) {
        if self.publication.decision != self.computed_publication_decision()
            || self.publication.blocking_rule_ids != self.computed_blocking_rule_ids()
            || self.publication.blocking_boundary_row_ids
                != self.computed_blocking_boundary_row_ids()
        {
            violations.push(GovernanceViolation::PublicationDecisionInconsistent);
        }
    }
}

/// Help/About and support-export projection of the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceExportProjection {
    /// Stable matrix id.
    pub governance_id: String,
    /// Snapshot date.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<GovernanceExportRow>,
}

/// One export row safe for user/support inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceExportRow {
    /// Boundary row id.
    pub boundary_row_id: String,
    /// Boundary family.
    pub boundary_family: BoundaryFamily,
    /// Canonical claim label.
    pub claim_label: StableClaimLevel,
    /// Effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Governance state.
    pub governance_state: GovernanceState,
    /// Negotiated fields.
    pub negotiated_fields: Vec<String>,
    /// Skew-window ref.
    pub skew_window_ref: String,
    /// Skew-window class.
    pub skew_window_class: SkewWindowClass,
    /// Whether rolling upgrade is supported.
    pub rolling_upgrade_supported: bool,
    /// Upgrade order.
    pub upgrade_order: Vec<String>,
    /// Rollback order.
    pub rollback_order: Vec<String>,
    /// Unsupported-flow label.
    pub unsupported_label: UnsupportedFlowLabel,
    /// Fail-closed behavior.
    pub fail_closed_behavior: String,
    /// Release-center binding.
    pub release_center_ref: String,
    /// Support-export binding.
    pub support_export_ref: String,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// Validation failures for the mixed-version matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion { actual: u32 },
    /// Unsupported record kind.
    UnsupportedRecordKind { actual: String },
    /// Required field is empty.
    EmptyField {
        boundary_row_id: String,
        field_name: &'static str,
    },
    /// Closed vocabulary does not match the typed model.
    ClosedVocabularyMismatch { field: &'static str },
    /// No governance rows exist.
    EmptyMatrix,
    /// Duplicate boundary row id.
    DuplicateBoundaryRowId { boundary_row_id: String },
    /// Duplicate rule id.
    DuplicateRuleId { rule_id: String },
    /// Rule is incomplete.
    IncompleteRule { rule_id: String },
    /// A gap reason lacks a rule.
    RuleMissingForGapReason { reason: GapReason },
    /// No governance rules exist.
    NoGovernanceRules,
    /// Boundary row lacks required content.
    IncompleteBoundaryRow { boundary_row_id: String },
    /// Effective label is wider than the claim ceiling.
    EffectiveLabelWiderThanClaim { boundary_row_id: String },
    /// A row that must narrow still publishes at or above the cutline.
    NarrowingRowNotNarrowed { boundary_row_id: String },
    /// A held row relies on stale or missing proof.
    HeldOnStaleProof { boundary_row_id: String },
    /// A held row lacks release/support/help bindings.
    HeldWithoutDownstreamBindings { boundary_row_id: String },
    /// A held row lacks owner signoff.
    HeldWithoutOwnerSignoff { boundary_row_id: String },
    /// A narrowed row does not explain why it narrowed.
    NarrowedWithoutGapReason { boundary_row_id: String },
    /// Coordinated-only boundary lacks the visible label.
    CoordinatedOnlyWithoutVisibleLabel { boundary_row_id: String },
    /// Required boundary family is missing.
    RequiredBoundaryMissing { family: BoundaryFamily },
    /// Publication verdict disagrees with firing rules.
    PublicationDecisionInconsistent,
    /// Summary counts disagree with rows.
    SummaryMismatch,
}

/// Parse the checked-in mixed-version governance matrix.
pub fn current_mixed_version_compatibility_and_skew_governance(
) -> Result<MixedVersionCompatibilityAndSkewGovernance, Box<dyn Error>> {
    parse_mixed_version_compatibility_and_skew_governance(
        MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_JSON,
    )
}

/// Parse a mixed-version governance matrix from JSON.
pub fn parse_mixed_version_compatibility_and_skew_governance(
    raw: &str,
) -> Result<MixedVersionCompatibilityAndSkewGovernance, Box<dyn Error>> {
    let matrix = serde_json::from_str(raw)?;
    Ok(matrix)
}

impl fmt::Display for GovernanceViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for GovernanceViolation {}
