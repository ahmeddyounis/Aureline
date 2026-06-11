//! Materialized effective quality-profile, save-participant order, quality-action
//! safety classes, and suppression or baseline records for the newer code-adjacent
//! artifact families.
//!
//! The stable quality lane already resolves one effective profile, orders save
//! participants, classifies fix safety, and counts release-visible debt for
//! source files through [`crate::quality`]. This module extends that exact
//! vocabulary to the artifact families that grow the quality surface — notebooks,
//! request files, scaffolded outputs, framework generators, and data-adjacent
//! artifacts — instead of inventing a parallel rule set. It publishes one
//! inspectable, serde truth packet that answers, per family:
//!
//! - **Which quality profile won?** A profile-resolution row names the effective
//!   profile, the winning source ref, the winning [`QualityProfileSourceLayer`]
//!   and [`QualityProfileSourceStateClass`], whether imported tool config was
//!   mapped, and whether policy overrode lower-layer keys.
//! - **Which save participants ran, in what order?** An ordered list of
//!   participants, each carrying its [`SaveParticipantPhaseClass`],
//!   [`QualityActionClass`], [`QualityFixSafetyClass`], preview requirement,
//!   apply posture, and mutation scope.
//! - **Which actions require preview?** Each participant declares whether it can
//!   auto-apply, must preview first, or is blocked — so no save-participant or
//!   fix-all flow becomes an invisible broad write.
//! - **Which debt is suppressed, baselined, waived, or newly introduced?** Per
//!   family debt rows keyed to a [`QualityReleaseDebtStateClass`], linked to the
//!   governed suppression or baseline record that backs them, keeping
//!   `suppressed` distinct from `baselined` everywhere.
//!
//! The governed suppression and baseline records carry scope, owner, reason,
//! expiry or review metadata, and reopen policy so they round-trip into export,
//! support, and release packets without losing scope or policy context.
//!
//! The packet is checked in at
//! `artifacts/quality/m5/materialize-artifact-family-quality-governance.json`
//! and embedded here, so this typed consumer and any CI gate agree on every row
//! without a cargo build in CI. The model is metadata-only: it carries no raw
//! source, raw tool arguments, raw paths, provider payloads, or secrets, and all
//! date arithmetic stays out of the structural invariants enforced here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::quality::{
    BaselineCompatibilityStateClass, QualityActionClass, QualityApplyPostureClass,
    QualityFixSafetyClass, QualityMutationScopeClass, QualityOwnerClass,
    QualityPolicyLockStateClass, QualityPreviewRequirementClass, QualityProfileSourceLayer,
    QualityProfileSourceStateClass, QualityReleaseDebtStateClass, QualityReopenRuleClass,
    QualityTargetScopeClass, SaveParticipantPhaseClass,
};

/// Supported artifact-family quality-governance packet schema version.
pub const ARTIFACT_FAMILY_QUALITY_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const ARTIFACT_FAMILY_QUALITY_GOVERNANCE_RECORD_KIND: &str =
    "artifact_family_quality_governance_record";

/// Repo-relative path to the checked-in packet.
pub const ARTIFACT_FAMILY_QUALITY_GOVERNANCE_PATH: &str =
    "artifacts/quality/m5/materialize-artifact-family-quality-governance.json";

/// Embedded checked-in packet JSON.
pub const ARTIFACT_FAMILY_QUALITY_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/quality/m5/materialize-artifact-family-quality-governance.json"
));

/// Artifact family whose quality governance this packet materializes.
///
/// These are the code-adjacent families that the stable source-file quality lane
/// did not previously cover end to end. Each must appear exactly once in the
/// packet so every family's profile, save order, and debt truth is explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyClass {
    /// Notebook documents and their cell/companion exports.
    Notebook,
    /// Request files such as `.http`/`.rest` request collections.
    RequestFile,
    /// Scaffolded outputs emitted by templates or new-project flows.
    ScaffoldedOutput,
    /// Framework generator outputs (routes, migrations, typed clients).
    FrameworkGenerator,
    /// Data-adjacent artifacts such as fixtures, schemas, or config bundles.
    DataAdjacentArtifact,
}

impl ArtifactFamilyClass {
    /// Every family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Notebook,
        Self::RequestFile,
        Self::ScaffoldedOutput,
        Self::FrameworkGenerator,
        Self::DataAdjacentArtifact,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::RequestFile => "request_file",
            Self::ScaffoldedOutput => "scaffolded_output",
            Self::FrameworkGenerator => "framework_generator",
            Self::DataAdjacentArtifact => "data_adjacent_artifact",
        }
    }
}

/// Stable phase-order index where smaller values run earlier on save.
///
/// This mirrors the declared order of [`SaveParticipantPhaseClass`] so the
/// packet can prove participants are listed in the order they would execute.
const fn save_phase_order_index(phase: SaveParticipantPhaseClass) -> u8 {
    match phase {
        SaveParticipantPhaseClass::Preflight => 0,
        SaveParticipantPhaseClass::FormatFix => 1,
        SaveParticipantPhaseClass::GeneratedArtifactUpdate => 2,
        SaveParticipantPhaseClass::Validation => 3,
        SaveParticipantPhaseClass::CompareBeforeWrite => 4,
        SaveParticipantPhaseClass::DurableWrite => 5,
        SaveParticipantPhaseClass::PostSaveIndexingRefresh => 6,
    }
}

/// Resolved effective-profile truth for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileResolution {
    /// Effective profile ref that won resolution for this family.
    pub effective_profile_ref: String,
    /// Winning source ref (settings, config, or policy ref).
    pub winning_source_ref: String,
    /// Winning source layer that determined precedence.
    pub winning_source_layer: QualityProfileSourceLayer,
    /// How the winning source participated in resolution.
    pub winning_source_state: QualityProfileSourceStateClass,
    /// True when imported tool config (formatter/linter/editorconfig) was mapped
    /// into the effective profile for this family.
    pub imported_config_mapped: bool,
    /// Imported or native config keys that could not be mapped.
    pub unmapped_imported_key_count: usize,
    /// Lower-layer keys that policy overrode.
    pub policy_overridden_key_count: usize,
    /// True when policy locks or constrains the effective profile.
    pub policy_locked: bool,
    /// Export-safe resolution summary.
    pub summary: String,
}

impl ProfileResolution {
    /// True when any imported config key could not be mapped.
    pub const fn has_unmapped_imported_config(&self) -> bool {
        self.unmapped_imported_key_count > 0
    }

    /// True when policy overrode at least one lower-layer key.
    pub const fn has_policy_overrides(&self) -> bool {
        self.policy_overridden_key_count > 0
    }
}

/// Ordered save participant materialized for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveParticipant {
    /// Participant id visible to save UI, CLI, review, support, and release exports.
    pub participant_id: String,
    /// Phase where the participant runs.
    pub phase_class: SaveParticipantPhaseClass,
    /// Order within the phase.
    pub phase_order: u16,
    /// Action class represented by this participant.
    pub action_class: QualityActionClass,
    /// Fix-safety class for this participant.
    pub fix_safety_class: QualityFixSafetyClass,
    /// Preview requirement inherited from the action proposal.
    pub preview_requirement_class: QualityPreviewRequirementClass,
    /// Apply posture inherited from the action proposal.
    pub apply_posture_class: QualityApplyPostureClass,
    /// Mutation scope claimed by the action.
    pub mutation_scope_class: QualityMutationScopeClass,
    /// True when the participant can auto-apply during hot save.
    pub auto_apply_allowed: bool,
    /// True when preview or review is required before mutation.
    pub preview_first_required: bool,
    /// True when apply is blocked pending review, policy, or trust.
    pub apply_blocked: bool,
    /// Export-safe participant summary.
    pub summary: String,
}

/// Per-family release-visible debt row keyed to a governed record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebtRow {
    /// Debt row id.
    pub debt_row_id: String,
    /// Debt state: `suppressed`, `baselined`, `waived`, or `new`.
    pub debt_state_class: QualityReleaseDebtStateClass,
    /// Suppression ref backing a suppressed/waived row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_ref: Option<String>,
    /// Baseline ref backing a baselined row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_ref: Option<String>,
    /// Finding refs represented by this row.
    #[serde(default)]
    pub finding_refs: Vec<String>,
    /// Rule refs represented by this row.
    #[serde(default)]
    pub rule_refs: Vec<String>,
    /// Owner ref responsible for the debt row.
    pub owner_ref: String,
    /// True when this row is visible in release and support export packets.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Governed suppression record retained in the packet.
///
/// Carries scope, owner, reason, and expiry-or-policy metadata so a suppressed
/// finding never reads as a permanent hidden toggle and round-trips into export,
/// support, and release packets with full context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedSuppression {
    /// Suppression id.
    pub suppression_id: String,
    /// Target scope.
    pub scope_class: QualityTargetScopeClass,
    /// Rule refs covered by the suppression.
    #[serde(default)]
    pub rule_refs: Vec<String>,
    /// Finding refs covered by the suppression.
    #[serde(default)]
    pub finding_refs: Vec<String>,
    /// Owner class.
    pub owner_class: QualityOwnerClass,
    /// Owner ref.
    pub owner_ref: String,
    /// Created-at timestamp.
    pub created_at: String,
    /// Expiry timestamp, unless policy owns expiry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Reason summary.
    pub reason: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reopen rule.
    pub reopen_rule_class: QualityReopenRuleClass,
    /// Policy edit state.
    pub policy_lock_state_class: QualityPolicyLockStateClass,
    /// Release-visible debt flag.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl GovernedSuppression {
    /// Whether the suppression would be a hidden permanent toggle.
    ///
    /// A suppression with no expiry is only governed when policy explicitly owns
    /// expiry timing; otherwise it is a hidden permanent toggle and is denied.
    pub fn is_hidden_permanent(&self) -> bool {
        self.expires_at.is_none()
            && self.policy_lock_state_class != QualityPolicyLockStateClass::ExpiryManagedByPolicy
    }
}

/// Governed baseline record retained in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedBaseline {
    /// Baseline id.
    pub baseline_id: String,
    /// Compatible profile family ref.
    pub compatible_profile_family_ref: String,
    /// Target scope class.
    pub target_scope_class: QualityTargetScopeClass,
    /// Accepted finding refs.
    #[serde(default)]
    pub accepted_finding_refs: Vec<String>,
    /// Owner class.
    pub owner_class: QualityOwnerClass,
    /// Owner ref.
    pub owner_ref: String,
    /// Created-at timestamp.
    pub created_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Review refs.
    #[serde(default)]
    pub review_refs: Vec<String>,
    /// Compatibility state.
    pub compatibility_state_class: BaselineCompatibilityStateClass,
    /// Policy edit state.
    pub policy_lock_state_class: QualityPolicyLockStateClass,
    /// Reopen rule.
    pub reopen_rule_class: QualityReopenRuleClass,
    /// Release-visible debt flag.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Materialized quality governance for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactFamilyMaterialization {
    /// Artifact family this row speaks for.
    pub family_class: ArtifactFamilyClass,
    /// Human-readable family label.
    pub family_label: String,
    /// Representative target ref the materialization was computed for.
    pub representative_target_ref: String,
    /// Resolved effective-profile truth.
    pub profile: ProfileResolution,
    /// Ordered save participants for this family.
    #[serde(default)]
    pub save_participants: Vec<SaveParticipant>,
    /// Release-visible debt rows for this family.
    #[serde(default)]
    pub debt_rows: Vec<DebtRow>,
    /// Export-safe family summary.
    pub summary: String,
}

impl ArtifactFamilyMaterialization {
    /// Participants whose apply must route through preview or review first.
    pub fn preview_first_participants(&self) -> impl Iterator<Item = &SaveParticipant> {
        self.save_participants
            .iter()
            .filter(|p| p.preview_first_required)
    }

    /// Debt rows in the given state.
    pub fn debt_rows_in_state(
        &self,
        state: QualityReleaseDebtStateClass,
    ) -> impl Iterator<Item = &DebtRow> {
        self.debt_rows
            .iter()
            .filter(move |row| row.debt_state_class == state)
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactFamilyQualityGovernanceSummary {
    /// Total artifact families materialized.
    pub total_families: usize,
    /// Total save participants across all families.
    pub total_save_participants: usize,
    /// Participants that may auto-apply on save.
    pub auto_apply_participants: usize,
    /// Participants that must preview or review before apply.
    pub preview_first_participants: usize,
    /// Participants whose apply is blocked.
    pub blocked_participants: usize,
    /// Suppressed debt rows.
    pub suppressed_debt_rows: usize,
    /// Baselined debt rows.
    pub baselined_debt_rows: usize,
    /// Waived debt rows.
    pub waived_debt_rows: usize,
    /// Newly introduced debt rows.
    pub new_debt_rows: usize,
    /// Total governed suppression records.
    pub total_suppressions: usize,
    /// Total governed baseline records.
    pub total_baselines: usize,
}

/// A redaction-safe export row projected from one family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFamilyQualityGovernanceExportRow {
    /// Family token.
    pub family: String,
    /// Effective profile ref that won.
    pub effective_profile_ref: String,
    /// Winning source layer token.
    pub winning_source_layer: String,
    /// Winning source state token.
    pub winning_source_state: String,
    /// True when policy overrode lower-layer keys.
    pub has_policy_overrides: bool,
    /// True when imported config keys could not be mapped.
    pub has_unmapped_imported_config: bool,
    /// Ordered save-participant action tokens.
    pub save_order: Vec<String>,
    /// Suppressed debt count.
    pub suppressed: usize,
    /// Baselined debt count.
    pub baselined: usize,
    /// Waived debt count.
    pub waived: usize,
    /// Newly introduced debt count.
    pub new: usize,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFamilyQualityGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected family rows.
    pub families: Vec<ArtifactFamilyQualityGovernanceExportRow>,
    /// Governed suppression refs included in the projection.
    pub suppression_refs: Vec<String>,
    /// Governed baseline refs included in the projection.
    pub baseline_refs: Vec<String>,
    /// True because no raw source, tool args, paths, or secrets are projected.
    pub redaction_safe: bool,
}

/// The typed artifact-family quality-governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactFamilyQualityGovernance {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed artifact-family vocabulary.
    pub artifact_family_classes: Vec<ArtifactFamilyClass>,
    /// Governed suppression records keyed by id.
    #[serde(default)]
    pub suppressions: Vec<GovernedSuppression>,
    /// Governed baseline records keyed by id.
    #[serde(default)]
    pub baselines: Vec<GovernedBaseline>,
    /// Per-family materializations.
    #[serde(default)]
    pub families: Vec<ArtifactFamilyMaterialization>,
    /// Summary counts.
    pub summary: ArtifactFamilyQualityGovernanceSummary,
}

impl ArtifactFamilyQualityGovernance {
    /// Returns the materialization for `family`, when present.
    pub fn family(&self, family: ArtifactFamilyClass) -> Option<&ArtifactFamilyMaterialization> {
        self.families.iter().find(|f| f.family_class == family)
    }

    /// Returns the governed suppression for `id`, when present.
    pub fn suppression(&self, id: &str) -> Option<&GovernedSuppression> {
        self.suppressions.iter().find(|s| s.suppression_id == id)
    }

    /// Returns the governed baseline for `id`, when present.
    pub fn baseline(&self, id: &str) -> Option<&GovernedBaseline> {
        self.baselines.iter().find(|b| b.baseline_id == id)
    }

    /// Recomputes the summary block from the materializations and records.
    pub fn computed_summary(&self) -> ArtifactFamilyQualityGovernanceSummary {
        let participants = || {
            self.families
                .iter()
                .flat_map(|f| f.save_participants.iter())
        };
        let debt_in = |state: QualityReleaseDebtStateClass| {
            self.families
                .iter()
                .flat_map(|f| f.debt_rows.iter())
                .filter(move |row| row.debt_state_class == state)
                .count()
        };
        ArtifactFamilyQualityGovernanceSummary {
            total_families: self.families.len(),
            total_save_participants: participants().count(),
            auto_apply_participants: participants().filter(|p| p.auto_apply_allowed).count(),
            preview_first_participants: participants().filter(|p| p.preview_first_required).count(),
            blocked_participants: participants().filter(|p| p.apply_blocked).count(),
            suppressed_debt_rows: debt_in(QualityReleaseDebtStateClass::Suppressed),
            baselined_debt_rows: debt_in(QualityReleaseDebtStateClass::Baselined),
            waived_debt_rows: debt_in(QualityReleaseDebtStateClass::Waived),
            new_debt_rows: debt_in(QualityReleaseDebtStateClass::New),
            total_suppressions: self.suppressions.len(),
            total_baselines: self.baselines.len(),
        }
    }

    /// Produces a redaction-safe export projection for support/release ingest.
    pub fn export_projection(&self) -> ArtifactFamilyQualityGovernanceExportProjection {
        let families = self
            .families
            .iter()
            .map(|family| ArtifactFamilyQualityGovernanceExportRow {
                family: family.family_class.as_str().to_owned(),
                effective_profile_ref: family.profile.effective_profile_ref.clone(),
                winning_source_layer: family.profile.winning_source_layer.as_str().to_owned(),
                winning_source_state: family.profile.winning_source_state.as_str().to_owned(),
                has_policy_overrides: family.profile.has_policy_overrides(),
                has_unmapped_imported_config: family.profile.has_unmapped_imported_config(),
                save_order: family
                    .save_participants
                    .iter()
                    .map(|p| p.action_class.as_str().to_owned())
                    .collect(),
                suppressed: family
                    .debt_rows_in_state(QualityReleaseDebtStateClass::Suppressed)
                    .count(),
                baselined: family
                    .debt_rows_in_state(QualityReleaseDebtStateClass::Baselined)
                    .count(),
                waived: family
                    .debt_rows_in_state(QualityReleaseDebtStateClass::Waived)
                    .count(),
                new: family
                    .debt_rows_in_state(QualityReleaseDebtStateClass::New)
                    .count(),
                summary: family.summary.clone(),
            })
            .collect();
        ArtifactFamilyQualityGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            families,
            suppression_refs: self
                .suppressions
                .iter()
                .map(|s| s.suppression_id.clone())
                .collect(),
            baseline_refs: self
                .baselines
                .iter()
                .map(|b| b.baseline_id.clone())
                .collect(),
            redaction_safe: true,
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ArtifactFamilyQualityGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_governed_records(&mut violations);

        let mut seen_families = BTreeSet::new();
        for family in &self.families {
            if !seen_families.insert(family.family_class) {
                violations.push(ArtifactFamilyQualityGovernanceViolation::DuplicateFamily {
                    family: family.family_class.as_str(),
                });
            }
            self.validate_family(family, &mut violations);
        }
        for family in ArtifactFamilyClass::ALL {
            if !seen_families.contains(&family) {
                violations.push(ArtifactFamilyQualityGovernanceViolation::MissingFamily {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ArtifactFamilyQualityGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ArtifactFamilyQualityGovernanceViolation>) {
        if self.schema_version != ARTIFACT_FAMILY_QUALITY_GOVERNANCE_SCHEMA_VERSION {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != ARTIFACT_FAMILY_QUALITY_GOVERNANCE_RECORD_KIND {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.artifact_family_classes != ArtifactFamilyClass::ALL.to_vec() {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::ClosedVocabularyMismatch {
                    field: "artifact_family_classes",
                },
            );
        }
    }

    fn validate_governed_records(
        &self,
        violations: &mut Vec<ArtifactFamilyQualityGovernanceViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for suppression in &self.suppressions {
            if !seen.insert(suppression.suppression_id.clone()) {
                violations.push(
                    ArtifactFamilyQualityGovernanceViolation::DuplicateSuppressionId {
                        id: suppression.suppression_id.clone(),
                    },
                );
            }
            for (field, value) in [
                ("owner_ref", &suppression.owner_ref),
                ("reason", &suppression.reason),
                ("created_at", &suppression.created_at),
            ] {
                if value.trim().is_empty() {
                    violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyField {
                        id: suppression.suppression_id.clone(),
                        field_name: field,
                    });
                }
            }
            if suppression.evidence_refs.is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::MissingEvidence {
                    id: suppression.suppression_id.clone(),
                });
            }
            if suppression.is_hidden_permanent() {
                violations.push(
                    ArtifactFamilyQualityGovernanceViolation::HiddenPermanentSuppression {
                        id: suppression.suppression_id.clone(),
                    },
                );
            }
        }

        let mut seen_baselines = BTreeSet::new();
        for baseline in &self.baselines {
            if !seen_baselines.insert(baseline.baseline_id.clone()) {
                violations.push(
                    ArtifactFamilyQualityGovernanceViolation::DuplicateBaselineId {
                        id: baseline.baseline_id.clone(),
                    },
                );
            }
            if baseline.owner_ref.trim().is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyField {
                    id: baseline.baseline_id.clone(),
                    field_name: "owner_ref",
                });
            }
            if baseline.accepted_finding_refs.is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyBaseline {
                    id: baseline.baseline_id.clone(),
                });
            }
            if baseline.evidence_refs.is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::MissingEvidence {
                    id: baseline.baseline_id.clone(),
                });
            }
        }
    }

    fn validate_family(
        &self,
        family: &ArtifactFamilyMaterialization,
        violations: &mut Vec<ArtifactFamilyQualityGovernanceViolation>,
    ) {
        let id = family.family_class.as_str();
        for (field, value) in [
            ("family_label", &family.family_label),
            (
                "representative_target_ref",
                &family.representative_target_ref,
            ),
            ("summary", &family.summary),
            (
                "effective_profile_ref",
                &family.profile.effective_profile_ref,
            ),
            ("winning_source_ref", &family.profile.winning_source_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyField {
                    id: id.to_owned(),
                    field_name: field,
                });
            }
        }

        // A policy-overridden profile must surface a policy-bearing winner state
        // so overridden values are never silently masked.
        if family.profile.has_policy_overrides()
            && !matches!(
                family.profile.winning_source_state,
                QualityProfileSourceStateClass::SelectedWinner
                    | QualityProfileSourceStateClass::PolicyOverridden
            )
        {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::MaskedPolicyOverride { family: id },
            );
        }

        self.validate_save_order(family, violations);
        self.validate_debt_rows(family, violations);
    }

    fn validate_save_order(
        &self,
        family: &ArtifactFamilyMaterialization,
        violations: &mut Vec<ArtifactFamilyQualityGovernanceViolation>,
    ) {
        let id = family.family_class.as_str();
        if family.save_participants.is_empty() {
            violations
                .push(ArtifactFamilyQualityGovernanceViolation::NoSaveParticipants { family: id });
            return;
        }

        let mut seen = BTreeSet::new();
        let mut prev_key: Option<(u8, u16)> = None;
        for participant in &family.save_participants {
            if !seen.insert(participant.participant_id.clone()) {
                violations.push(
                    ArtifactFamilyQualityGovernanceViolation::DuplicateParticipantId {
                        family: id,
                        participant_id: participant.participant_id.clone(),
                    },
                );
            }
            // The list must be ordered by (phase, phase_order) so save order is
            // explicit and reconstructable.
            let key = (
                save_phase_order_index(participant.phase_class),
                participant.phase_order,
            );
            if let Some(previous) = prev_key {
                if key < previous {
                    violations.push(
                        ArtifactFamilyQualityGovernanceViolation::SaveParticipantsOutOfOrder {
                            family: id,
                            participant_id: participant.participant_id.clone(),
                        },
                    );
                }
            }
            prev_key = Some(key);

            self.validate_participant(family.family_class, participant, violations);
        }
    }

    fn validate_participant(
        &self,
        family: ArtifactFamilyClass,
        participant: &SaveParticipant,
        violations: &mut Vec<ArtifactFamilyQualityGovernanceViolation>,
    ) {
        let id = family.as_str();
        if participant.participant_id.trim().is_empty() || participant.summary.trim().is_empty() {
            violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyField {
                id: id.to_owned(),
                field_name: "participant",
            });
        }

        // Guardrail: a save participant that is not a safe local edit must never
        // auto-apply, and a broad mutation must never auto-apply — otherwise it
        // is an invisible broad write.
        let unsafe_class = !participant.fix_safety_class.allows_auto_apply();
        let broad_scope = participant.mutation_scope_class.is_broad();
        if participant.auto_apply_allowed && (unsafe_class || broad_scope) {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::InvisibleBroadWrite {
                    family: id,
                    participant_id: participant.participant_id.clone(),
                },
            );
        }

        // A non-safe or broad participant must route through preview or be
        // blocked; it can never silently apply.
        if (unsafe_class || broad_scope)
            && !participant.preview_first_required
            && !participant.apply_blocked
        {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::UnsafeWriteWithoutPreview {
                    family: id,
                    participant_id: participant.participant_id.clone(),
                },
            );
        }

        // Generated-companion safety must run in the generated-artifact phase so
        // companion updates are never folded into a silent local format pass.
        if participant.fix_safety_class == QualityFixSafetyClass::GeneratedCompanionUpdate
            && participant.phase_class != SaveParticipantPhaseClass::GeneratedArtifactUpdate
        {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::GeneratedPhaseMismatch {
                    family: id,
                    participant_id: participant.participant_id.clone(),
                },
            );
        }

        // The declared apply posture and preview requirement must agree with the
        // boolean flags downstream surfaces render.
        if participant.apply_posture_class.blocks_apply() != participant.apply_blocked {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::ApplyPostureInconsistent {
                    family: id,
                    participant_id: participant.participant_id.clone(),
                },
            );
        }
        if participant
            .preview_requirement_class
            .requires_preview_first()
            && !participant.preview_first_required
        {
            violations.push(
                ArtifactFamilyQualityGovernanceViolation::PreviewFlagInconsistent {
                    family: id,
                    participant_id: participant.participant_id.clone(),
                },
            );
        }
    }

    fn validate_debt_rows(
        &self,
        family: &ArtifactFamilyMaterialization,
        violations: &mut Vec<ArtifactFamilyQualityGovernanceViolation>,
    ) {
        let id = family.family_class.as_str();
        let mut seen = BTreeSet::new();
        for row in &family.debt_rows {
            if !seen.insert(row.debt_row_id.clone()) {
                violations.push(
                    ArtifactFamilyQualityGovernanceViolation::DuplicateDebtRowId {
                        family: id,
                        debt_row_id: row.debt_row_id.clone(),
                    },
                );
            }
            if row.owner_ref.trim().is_empty() {
                violations.push(ArtifactFamilyQualityGovernanceViolation::EmptyField {
                    id: row.debt_row_id.clone(),
                    field_name: "owner_ref",
                });
            }

            // `suppressed` is distinct from `baselined`: a row may never carry
            // both a suppression and a baseline ref.
            if row.suppression_ref.is_some() && row.baseline_ref.is_some() {
                violations.push(
                    ArtifactFamilyQualityGovernanceViolation::DebtRowCarriesBothRefs {
                        family: id,
                        debt_row_id: row.debt_row_id.clone(),
                    },
                );
            }

            match row.debt_state_class {
                QualityReleaseDebtStateClass::Suppressed | QualityReleaseDebtStateClass::Waived => {
                    match &row.suppression_ref {
                        Some(reference) if self.suppression(reference).is_some() => {}
                        Some(reference) => violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DanglingDebtRef {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                reference: reference.clone(),
                            },
                        ),
                        None => violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DebtStateRefMismatch {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                state: row.debt_state_class.as_str(),
                            },
                        ),
                    }
                    if row.baseline_ref.is_some() {
                        violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DebtStateRefMismatch {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                state: row.debt_state_class.as_str(),
                            },
                        );
                    }
                }
                QualityReleaseDebtStateClass::Baselined => {
                    match &row.baseline_ref {
                        Some(reference) if self.baseline(reference).is_some() => {}
                        Some(reference) => violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DanglingDebtRef {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                reference: reference.clone(),
                            },
                        ),
                        None => violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DebtStateRefMismatch {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                state: row.debt_state_class.as_str(),
                            },
                        ),
                    }
                    if row.suppression_ref.is_some() {
                        violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DebtStateRefMismatch {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                state: row.debt_state_class.as_str(),
                            },
                        );
                    }
                }
                QualityReleaseDebtStateClass::New => {
                    if row.suppression_ref.is_some() || row.baseline_ref.is_some() {
                        violations.push(
                            ArtifactFamilyQualityGovernanceViolation::DebtStateRefMismatch {
                                family: id,
                                debt_row_id: row.debt_row_id.clone(),
                                state: row.debt_state_class.as_str(),
                            },
                        );
                    }
                }
                QualityReleaseDebtStateClass::Resolved | QualityReleaseDebtStateClass::Unmapped => {
                }
            }
        }
    }
}

/// A validation violation for the artifact-family quality-governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactFamilyQualityGovernanceViolation {
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
    /// A closed vocabulary is not the canonical value.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, family, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An artifact family appears more than once.
    DuplicateFamily {
        /// Duplicate family token.
        family: &'static str,
    },
    /// An artifact family is missing from the packet.
    MissingFamily {
        /// Missing family token.
        family: &'static str,
    },
    /// A suppression id appears more than once.
    DuplicateSuppressionId {
        /// Duplicate suppression id.
        id: String,
    },
    /// A baseline id appears more than once.
    DuplicateBaselineId {
        /// Duplicate baseline id.
        id: String,
    },
    /// A governed record carries no evidence refs.
    MissingEvidence {
        /// Record id missing evidence.
        id: String,
    },
    /// A suppression with no expiry is not policy-managed, so it would be a
    /// hidden permanent toggle.
    HiddenPermanentSuppression {
        /// Suppression id.
        id: String,
    },
    /// A baseline accepts no findings.
    EmptyBaseline {
        /// Baseline id.
        id: String,
    },
    /// A policy-overridden profile hides the override behind a non-policy winner.
    MaskedPolicyOverride {
        /// Family token.
        family: &'static str,
    },
    /// A family materializes no save participants.
    NoSaveParticipants {
        /// Family token.
        family: &'static str,
    },
    /// A participant id appears more than once within a family.
    DuplicateParticipantId {
        /// Family token.
        family: &'static str,
        /// Duplicate participant id.
        participant_id: String,
    },
    /// Save participants are not listed in execution order.
    SaveParticipantsOutOfOrder {
        /// Family token.
        family: &'static str,
        /// Out-of-order participant id.
        participant_id: String,
    },
    /// A non-safe or broad participant is marked auto-apply.
    InvisibleBroadWrite {
        /// Family token.
        family: &'static str,
        /// Offending participant id.
        participant_id: String,
    },
    /// A non-safe or broad participant neither previews nor blocks before apply.
    UnsafeWriteWithoutPreview {
        /// Family token.
        family: &'static str,
        /// Offending participant id.
        participant_id: String,
    },
    /// A generated-companion participant runs outside the generated-artifact phase.
    GeneratedPhaseMismatch {
        /// Family token.
        family: &'static str,
        /// Offending participant id.
        participant_id: String,
    },
    /// The apply posture and apply-blocked flag disagree.
    ApplyPostureInconsistent {
        /// Family token.
        family: &'static str,
        /// Offending participant id.
        participant_id: String,
    },
    /// A preview-required participant does not set the preview-first flag.
    PreviewFlagInconsistent {
        /// Family token.
        family: &'static str,
        /// Offending participant id.
        participant_id: String,
    },
    /// A debt row id appears more than once within a family.
    DuplicateDebtRowId {
        /// Family token.
        family: &'static str,
        /// Duplicate debt row id.
        debt_row_id: String,
    },
    /// A debt row carries both a suppression and a baseline ref.
    DebtRowCarriesBothRefs {
        /// Family token.
        family: &'static str,
        /// Offending debt row id.
        debt_row_id: String,
    },
    /// A debt row's state disagrees with its suppression/baseline refs.
    DebtStateRefMismatch {
        /// Family token.
        family: &'static str,
        /// Offending debt row id.
        debt_row_id: String,
        /// Debt state token.
        state: &'static str,
    },
    /// A debt row references a suppression or baseline the packet does not carry.
    DanglingDebtRef {
        /// Family token.
        family: &'static str,
        /// Offending debt row id.
        debt_row_id: String,
        /// Dangling reference.
        reference: String,
    },
    /// The summary counts disagree with the materializations and records.
    SummaryMismatch,
}

impl fmt::Display for ArtifactFamilyQualityGovernanceViolation {
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
            Self::DuplicateFamily { family } => {
                write!(f, "duplicate artifact family {family}")
            }
            Self::MissingFamily { family } => {
                write!(f, "missing artifact family {family}")
            }
            Self::DuplicateSuppressionId { id } => {
                write!(f, "duplicate suppression id {id}")
            }
            Self::DuplicateBaselineId { id } => {
                write!(f, "duplicate baseline id {id}")
            }
            Self::MissingEvidence { id } => {
                write!(f, "governed record {id} carries no evidence refs")
            }
            Self::HiddenPermanentSuppression { id } => {
                write!(
                    f,
                    "suppression {id} has no expiry and is not policy-managed (hidden permanent toggle denied)"
                )
            }
            Self::EmptyBaseline { id } => {
                write!(f, "baseline {id} accepts no findings")
            }
            Self::MaskedPolicyOverride { family } => {
                write!(
                    f,
                    "family {family} reports policy overrides behind a non-policy winner state"
                )
            }
            Self::NoSaveParticipants { family } => {
                write!(f, "family {family} materializes no save participants")
            }
            Self::DuplicateParticipantId {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} has duplicate save participant {participant_id}"
                )
            }
            Self::SaveParticipantsOutOfOrder {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} save participant {participant_id} is out of execution order"
                )
            }
            Self::InvisibleBroadWrite {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} participant {participant_id} auto-applies a non-safe or broad write"
                )
            }
            Self::UnsafeWriteWithoutPreview {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} participant {participant_id} neither previews nor blocks a non-safe or broad write"
                )
            }
            Self::GeneratedPhaseMismatch {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} participant {participant_id} updates a generated companion outside the generated-artifact phase"
                )
            }
            Self::ApplyPostureInconsistent {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} participant {participant_id} apply posture disagrees with the apply-blocked flag"
                )
            }
            Self::PreviewFlagInconsistent {
                family,
                participant_id,
            } => {
                write!(
                    f,
                    "family {family} participant {participant_id} requires preview but the preview-first flag is unset"
                )
            }
            Self::DuplicateDebtRowId {
                family,
                debt_row_id,
            } => {
                write!(f, "family {family} has duplicate debt row {debt_row_id}")
            }
            Self::DebtRowCarriesBothRefs {
                family,
                debt_row_id,
            } => {
                write!(
                    f,
                    "family {family} debt row {debt_row_id} carries both a suppression and a baseline ref"
                )
            }
            Self::DebtStateRefMismatch {
                family,
                debt_row_id,
                state,
            } => {
                write!(
                    f,
                    "family {family} debt row {debt_row_id} state {state} disagrees with its governed refs"
                )
            }
            Self::DanglingDebtRef {
                family,
                debt_row_id,
                reference,
            } => {
                write!(
                    f,
                    "family {family} debt row {debt_row_id} references unknown governed record {reference}"
                )
            }
            Self::SummaryMismatch => {
                write!(
                    f,
                    "packet summary counts disagree with the materializations"
                )
            }
        }
    }
}

impl Error for ArtifactFamilyQualityGovernanceViolation {}

/// Loads the embedded artifact-family quality-governance packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ArtifactFamilyQualityGovernance`].
pub fn current_artifact_family_quality_governance(
) -> Result<ArtifactFamilyQualityGovernance, serde_json::Error> {
    serde_json::from_str(ARTIFACT_FAMILY_QUALITY_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
