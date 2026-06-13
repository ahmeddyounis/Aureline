//! Pre-apply policy simulation, current-versus-draft diffs, impacted-object
//! summaries, and exportable change/unchanged/expiry effects for the M5
//! policy-bearing artifact families.
//!
//! This module is the canonical *pre-apply* truth source for M5 policy
//! changes. Where [`crate::m5_records_policy`] states the legal-hold,
//! retention, and pre-action delete/export truth a user sees for the *current*
//! policy, this module lets an admin compare the current policy against a
//! proposed draft *before* publishing it. Each governed family carries a row
//! that:
//!
//! - diffs the current versus draft delete and export outcome, marking each
//!   action `changed` or `unchanged`;
//! - lists the saved objects the draft would touch ([`ImpactedObject`]);
//! - states how an expiry/retention change alters runtime behavior
//!   ([`ExpiryEffect`]) so expiry effects are visible *in the simulation*
//!   rather than implied only after publish;
//! - states the downgrade path the draft introduces ([`DowngradePath`]); and
//! - rolls everything up into a machine-readable [`ImpactSummary`] consumers
//!   can export directly.
//!
//! The simulation reuses the runtime vocabulary — [`GovernedArtifactFamily`],
//! [`RecordClassId`], and [`RecordOperationOutcome`] — so the diff packet, its
//! exported impact summary, and the runtime surfaces it affects all share one
//! set of object identities. The packet is metadata-only: it carries object
//! refs, record classes, and policy epochs but no credential bodies, raw
//! provider payloads, or durable content. A draft never implies a managed
//! (remote) hold, export, or delete for an artifact the platform only knows
//! locally.

use serde::{Deserialize, Serialize};

use crate::records_policy_simulation_matrix::{AuthorityBoundaryClass, GovernedArtifactFamily};
use crate::stabilize_record_class_registry_legal_hold_delete_honesty::RecordOperationOutcome;
use crate::{LocalVsManagedCopy, RecordClassId};

#[cfg(test)]
mod tests;

/// Schema version for the M5 policy-impact simulation packet.
pub const M5_POLICY_SIMULATION_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the top-level packet.
pub const M5_POLICY_SIMULATION_RECORD_KIND: &str = "m5_policy_impact_simulation_packet";

/// Shared contract reference for the simulation lane.
pub const M5_POLICY_SIMULATION_SHARED_CONTRACT_REF: &str = "records:m5_policy_impact_simulation:v1";

/// Reference to the runtime hold/retention contract whose identities the
/// simulation reuses.
pub const M5_POLICY_SIMULATION_RUNTIME_CONTRACT_REF: &str = "records:m5_hold_retention_truth:v1";

/// Reference to the policy exception/expiry contract this lane gates against.
pub const M5_POLICY_SIMULATION_EXCEPTION_CONTRACT_REF: &str = "policy:m5_exception_expiry_truth:v1";

/// Repo-relative doc reference for the simulation contract.
pub const M5_POLICY_SIMULATION_DOC_REF: &str = "docs/governance/m5_policy_impact_simulation.md";

/// Repo-relative artifact summary for the simulation contract.
pub const M5_POLICY_SIMULATION_ARTIFACT_REF: &str =
    "artifacts/governance/m5_policy_impact_simulation.md";

/// Repo-relative schema reference for the simulation contract.
pub const M5_POLICY_SIMULATION_SCHEMA_REF: &str =
    "schemas/governance/m5_policy_impact_simulation.schema.json";

/// Repo-relative fixture directory for the canonical packet.
pub const M5_POLICY_SIMULATION_FIXTURE_DIR: &str =
    "fixtures/governance/m5_policy_impact_simulation";

/// Action whose current-versus-draft outcome the simulation compares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulatedActionClass {
    /// Delete or destroy the artifact.
    Delete,
    /// Export the artifact off the producing surface.
    Export,
}

impl SimulatedActionClass {
    /// Every simulated action class in canonical order.
    pub const ALL: [Self; 2] = [Self::Delete, Self::Export];

    /// Returns the stable snake_case token for the action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Delete => "delete",
            Self::Export => "export",
        }
    }
}

/// How a proposed expiry/retention change alters runtime behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryEffectClass {
    /// The expiry/retention rule is unchanged by the draft.
    Unchanged,
    /// The draft lengthens the retention horizon, deferring purge eligibility.
    Extended,
    /// The draft shortens the retention horizon, bringing purge eligibility earlier.
    Shortened,
    /// The draft introduces a new expiry/retention rule where none existed.
    Introduced,
    /// The draft removes the expiry/retention rule entirely.
    Removed,
}

impl ExpiryEffectClass {
    /// Returns the stable snake_case token for the expiry effect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Extended => "extended",
            Self::Shortened => "shortened",
            Self::Introduced => "introduced",
            Self::Removed => "removed",
        }
    }

    /// Returns true when the draft changes the expiry/retention rule.
    pub const fn changes_runtime(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

/// The behavioral downgrade path a draft introduces, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePathClass {
    /// The draft introduces no downgrade.
    None,
    /// A managed control is withdrawn, leaving local-only behavior.
    ManagedToLocalOnly,
    /// A delete that completed now fails closed under a hold.
    CompletedToBlocked,
    /// A delete that completed is now deferred by retention policy.
    CompletedToPolicyRetained,
    /// An export that completed now requires a manual on-device capture step.
    ExportToManualLocalCapture,
    /// An export that completed is now partially omitted by redaction policy.
    ExportToOmittedByRedaction,
}

impl DowngradePathClass {
    /// Returns the stable snake_case token for the downgrade path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ManagedToLocalOnly => "managed_to_local_only",
            Self::CompletedToBlocked => "completed_to_blocked",
            Self::CompletedToPolicyRetained => "completed_to_policy_retained",
            Self::ExportToManualLocalCapture => "export_to_manual_local_capture",
            Self::ExportToOmittedByRedaction => "export_to_omitted_by_redaction",
        }
    }

    /// Returns true when the draft introduces a real downgrade.
    pub const fn is_downgrade(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// One simulated action's current-versus-draft comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionDiff {
    /// Action this diff describes.
    pub action: SimulatedActionClass,
    /// Outcome the user gets under the current policy.
    pub current_outcome: RecordOperationOutcome,
    /// Outcome the user would get under the draft policy.
    pub draft_outcome: RecordOperationOutcome,
    /// Whether the draft changes this action's outcome.
    pub changed: bool,
    /// Plain-language summary of the action's effect (changed or unchanged).
    pub effect_summary: String,
}

impl ActionDiff {
    /// Returns true when [`Self::changed`] agrees with the outcomes compared.
    pub fn changed_flag_is_consistent(&self) -> bool {
        self.changed == (self.current_outcome != self.draft_outcome)
    }
}

/// A saved object the draft policy would touch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactedObject {
    /// Stable object reference shared with the runtime surfaces.
    pub object_ref: String,
    /// Record class governing the object.
    pub record_class_id: RecordClassId,
    /// Producer record kinds that materialize the object.
    pub producer_record_kinds: Vec<String>,
    /// Whether a managed copy of the object exists.
    pub managed_copy: bool,
    /// Optional boundary note (for example, local-only scope).
    pub note: Option<String>,
}

/// How a proposed expiry/retention change alters runtime behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiryEffect {
    /// Expiry effect class.
    pub effect_class: ExpiryEffectClass,
    /// Current expiry/retention rule in plain language.
    pub current_expiry_rule: String,
    /// Draft expiry/retention rule in plain language.
    pub draft_expiry_rule: String,
    /// Runtime consequence the expiry change produces.
    pub runtime_consequence: String,
    /// When the change would take effect.
    pub effective_at: String,
}

/// The downgrade path the draft introduces for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradePath {
    /// Downgrade path class.
    pub path_class: DowngradePathClass,
    /// Behavior before the draft applies.
    pub from_behavior: String,
    /// Behavior after the draft applies.
    pub to_behavior: String,
    /// Whether the downgrade is visible in the simulation before publish (must be true).
    pub visible_before_publish: bool,
}

/// One governed family's full pre-apply simulation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationRow {
    /// Stable entry id (shared with the runtime hold/retention row).
    pub entry_id: String,
    /// Review-safe title.
    pub title: String,
    /// Governed artifact family.
    pub artifact_family: GovernedArtifactFamily,
    /// Record class for the family.
    pub record_class_id: RecordClassId,
    /// Authority boundary for the family.
    pub authority_boundary: AuthorityBoundaryClass,
    /// Where the authoritative copy lives.
    pub local_truth_authority: LocalVsManagedCopy,
    /// Policy epoch currently in force.
    pub current_policy_epoch: String,
    /// Policy epoch the draft would publish.
    pub draft_policy_epoch: String,
    /// Whether the draft claims a managed legal hold.
    pub draft_claims_managed_hold: bool,
    /// Whether the draft claims a managed export.
    pub draft_claims_managed_export: bool,
    /// Whether the draft claims a managed delete.
    pub draft_claims_managed_delete: bool,
    /// Current-versus-draft diffs for every simulated action.
    pub action_diffs: Vec<ActionDiff>,
    /// Saved objects the draft would touch.
    pub impacted_objects: Vec<ImpactedObject>,
    /// How a proposed expiry/retention change alters runtime behavior.
    pub expiry_effect: ExpiryEffect,
    /// Downgrade path the draft introduces.
    pub downgrade_path: DowngradePath,
    /// References into the policy exception/expiry lane.
    pub exception_refs: Vec<String>,
    /// Proof reference backing the row.
    pub proof_ref: String,
    /// Rationale for the simulated change.
    pub rationale: String,
}

impl PolicySimulationRow {
    /// Returns the action diffs the draft changes.
    pub fn changed_actions(&self) -> Vec<&ActionDiff> {
        self.action_diffs
            .iter()
            .filter(|diff| diff.changed)
            .collect()
    }

    /// Returns the action diffs the draft leaves unchanged.
    pub fn unchanged_actions(&self) -> Vec<&ActionDiff> {
        self.action_diffs
            .iter()
            .filter(|diff| !diff.changed)
            .collect()
    }

    /// Returns true when the draft changes any action's outcome.
    pub fn has_changes(&self) -> bool {
        self.action_diffs.iter().any(|diff| diff.changed)
    }

    /// Returns the diff for `action`, if present.
    pub fn diff_for(&self, action: SimulatedActionClass) -> Option<&ActionDiff> {
        self.action_diffs.iter().find(|diff| diff.action == action)
    }

    /// Returns true when the family's authoritative copy is local-only.
    fn is_local_only(&self) -> bool {
        matches!(self.authority_boundary, AuthorityBoundaryClass::LocalOnly)
            || matches!(
                self.local_truth_authority,
                LocalVsManagedCopy::LocalAuthoritative | LocalVsManagedCopy::LocalCacheOnly
            )
    }
}

/// Machine-readable roll-up of the simulation's impact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactSummary {
    /// Total governed families simulated.
    pub total_families: usize,
    /// Families with at least one changed action.
    pub families_with_changes: usize,
    /// Families with no changed action.
    pub families_unchanged: usize,
    /// Total changed actions across all families.
    pub changed_action_count: usize,
    /// Total unchanged actions across all families.
    pub unchanged_action_count: usize,
    /// Total saved objects the draft would touch.
    pub impacted_object_count: usize,
    /// Families whose expiry/retention rule changes.
    pub families_with_expiry_change: usize,
    /// Families that incur a downgrade.
    pub families_with_downgrade: usize,
}

/// One changed-action export row in the machine-readable impact summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedActionExportRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Stable entry id.
    pub entry_id: String,
    /// Action whose outcome changes.
    pub action: SimulatedActionClass,
    /// Current outcome.
    pub current_outcome: RecordOperationOutcome,
    /// Draft outcome.
    pub draft_outcome: RecordOperationOutcome,
    /// Plain-language effect summary.
    pub effect_summary: String,
}

/// Product-surface projection row for one simulated family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductSimulationRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Whether the draft changes any action.
    pub has_changes: bool,
    /// Current delete outcome.
    pub current_delete_outcome: RecordOperationOutcome,
    /// Draft delete outcome.
    pub draft_delete_outcome: RecordOperationOutcome,
    /// Current export outcome.
    pub current_export_outcome: RecordOperationOutcome,
    /// Draft export outcome.
    pub draft_export_outcome: RecordOperationOutcome,
    /// Expiry effect class.
    pub expiry_effect: ExpiryEffectClass,
    /// Downgrade path class.
    pub downgrade_path: DowngradePathClass,
    /// Count of saved objects the draft would touch.
    pub impacted_object_count: usize,
}

/// CLI/headless projection row for one simulated family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliHeadlessSimulationRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Stable entry id.
    pub entry_id: String,
    /// Draft policy epoch.
    pub draft_policy_epoch: String,
    /// Changed action diffs.
    pub changed_actions: Vec<ActionDiff>,
    /// Unchanged action diffs.
    pub unchanged_actions: Vec<ActionDiff>,
    /// Expiry effect class.
    pub expiry_effect: ExpiryEffectClass,
    /// Downgrade path class.
    pub downgrade_path: DowngradePathClass,
}

/// Support/export projection row for one simulated family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportSimulationRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Stable entry id.
    pub entry_id: String,
    /// Current policy epoch.
    pub current_policy_epoch: String,
    /// Draft policy epoch.
    pub draft_policy_epoch: String,
    /// Changed-action export rows.
    pub changed_actions: Vec<ChangedActionExportRow>,
    /// Impacted objects.
    pub impacted_objects: Vec<ImpactedObject>,
    /// Expiry effect.
    pub expiry_effect: ExpiryEffect,
    /// Downgrade path.
    pub downgrade_path: DowngradePath,
    /// Exception/expiry references.
    pub exception_refs: Vec<String>,
}

/// Top-level canonical M5 policy-impact simulation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PolicyImpactSimulationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// UTC packet timestamp.
    pub as_of: String,
    /// Overview doc ref.
    pub overview_doc_ref: String,
    /// Artifact summary ref.
    pub artifact_summary_ref: String,
    /// Reference to the runtime hold/retention contract whose identities are reused.
    pub runtime_contract_ref: String,
    /// Reference to the policy exception/expiry contract this lane gates against.
    pub exception_expiry_contract_ref: String,
    /// Simulated family rows.
    pub rows: Vec<PolicySimulationRow>,
    /// Machine-readable impact roll-up.
    pub impact_summary: ImpactSummary,
    /// Review-safe summary.
    pub summary: String,
}

/// Validation issue emitted by the M5 policy-impact simulation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum M5PolicySimulationViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch { found: u32 },
    /// Record kind mismatch.
    RecordKindMismatch { found: String },
    /// A row's record class is not the canonical class for its family.
    RecordClassIdentityMismatch {
        entry_id: String,
        expected: RecordClassId,
        found: RecordClassId,
    },
    /// A local-only family's draft claims a managed legal hold.
    LocalOnlyDraftClaimsManagedHold { entry_id: String },
    /// A local-only family's draft claims a managed export.
    LocalOnlyDraftClaimsManagedExport { entry_id: String },
    /// A local-only family's draft claims a managed delete.
    LocalOnlyDraftClaimsManagedDelete { entry_id: String },
    /// A row omits a diff for a required action class.
    ActionCoverageMissing {
        entry_id: String,
        action: SimulatedActionClass,
    },
    /// An action diff's `changed` flag disagrees with the outcomes compared.
    ChangedFlagInconsistent {
        entry_id: String,
        action: SimulatedActionClass,
    },
    /// An action diff omits its effect summary.
    EffectSummaryMissing {
        entry_id: String,
        action: SimulatedActionClass,
    },
    /// A changing expiry effect omits its runtime consequence.
    ExpiryConsequenceMissing { entry_id: String },
    /// A draft did not advance the policy epoch.
    DraftPolicyEpochNotAdvanced { entry_id: String },
    /// A downgrade path is not visible before publish.
    DowngradeNotVisibleBeforePublish { entry_id: String },
    /// A downgrade path omits its before/after behavior.
    DowngradeBehaviorMissing { entry_id: String },
    /// The impact summary roll-up disagrees with the computed roll-up.
    ImpactSummaryMismatch { field: String },
    /// A required governed family is missing.
    FamilyCoverageMissing {
        artifact_family: GovernedArtifactFamily,
    },
}

impl M5PolicyImpactSimulationPacket {
    /// Validates the packet against the pre-apply simulation honesty contract.
    pub fn validate(&self) -> Vec<M5PolicySimulationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_POLICY_SIMULATION_SCHEMA_VERSION {
            violations.push(M5PolicySimulationViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_POLICY_SIMULATION_RECORD_KIND {
            violations.push(M5PolicySimulationViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        for row in &self.rows {
            let expected_class = canonical_record_class_for_family(row.artifact_family);
            if row.record_class_id != expected_class {
                violations.push(M5PolicySimulationViolation::RecordClassIdentityMismatch {
                    entry_id: row.entry_id.clone(),
                    expected: expected_class,
                    found: row.record_class_id,
                });
            }

            // Guardrail: a local-only family's draft never implies a managed
            // (remote) hold, export, or delete.
            if row.is_local_only() {
                if row.draft_claims_managed_hold {
                    violations.push(
                        M5PolicySimulationViolation::LocalOnlyDraftClaimsManagedHold {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
                if row.draft_claims_managed_export {
                    violations.push(
                        M5PolicySimulationViolation::LocalOnlyDraftClaimsManagedExport {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
                if row.draft_claims_managed_delete {
                    violations.push(
                        M5PolicySimulationViolation::LocalOnlyDraftClaimsManagedDelete {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
            }

            for action in SimulatedActionClass::ALL {
                match row.diff_for(action) {
                    None => {
                        violations.push(M5PolicySimulationViolation::ActionCoverageMissing {
                            entry_id: row.entry_id.clone(),
                            action,
                        });
                    }
                    Some(diff) => {
                        if !diff.changed_flag_is_consistent() {
                            violations.push(M5PolicySimulationViolation::ChangedFlagInconsistent {
                                entry_id: row.entry_id.clone(),
                                action,
                            });
                        }
                        if diff.effect_summary.trim().is_empty() {
                            violations.push(M5PolicySimulationViolation::EffectSummaryMissing {
                                entry_id: row.entry_id.clone(),
                                action,
                            });
                        }
                    }
                }
            }

            // Expiry effects must be visible in the simulation: a changing rule
            // must state its runtime consequence.
            if row.expiry_effect.effect_class.changes_runtime()
                && row.expiry_effect.runtime_consequence.trim().is_empty()
            {
                violations.push(M5PolicySimulationViolation::ExpiryConsequenceMissing {
                    entry_id: row.entry_id.clone(),
                });
            }

            // A draft is a new policy epoch; it must not reuse the current one.
            if row.draft_policy_epoch.trim().is_empty()
                || row.draft_policy_epoch == row.current_policy_epoch
            {
                violations.push(M5PolicySimulationViolation::DraftPolicyEpochNotAdvanced {
                    entry_id: row.entry_id.clone(),
                });
            }

            // Downgrade paths must be visible before publish.
            if row.downgrade_path.path_class.is_downgrade() {
                if !row.downgrade_path.visible_before_publish {
                    violations.push(
                        M5PolicySimulationViolation::DowngradeNotVisibleBeforePublish {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
                if row.downgrade_path.from_behavior.trim().is_empty()
                    || row.downgrade_path.to_behavior.trim().is_empty()
                {
                    violations.push(M5PolicySimulationViolation::DowngradeBehaviorMissing {
                        entry_id: row.entry_id.clone(),
                    });
                }
            }
        }

        for family in GovernedArtifactFamily::ALL {
            if !self.rows.iter().any(|row| row.artifact_family == family) {
                violations.push(M5PolicySimulationViolation::FamilyCoverageMissing {
                    artifact_family: family,
                });
            }
        }

        let computed = self.computed_impact_summary();
        for (field, ok) in [
            (
                "total_families",
                self.impact_summary.total_families == computed.total_families,
            ),
            (
                "families_with_changes",
                self.impact_summary.families_with_changes == computed.families_with_changes,
            ),
            (
                "families_unchanged",
                self.impact_summary.families_unchanged == computed.families_unchanged,
            ),
            (
                "changed_action_count",
                self.impact_summary.changed_action_count == computed.changed_action_count,
            ),
            (
                "unchanged_action_count",
                self.impact_summary.unchanged_action_count == computed.unchanged_action_count,
            ),
            (
                "impacted_object_count",
                self.impact_summary.impacted_object_count == computed.impacted_object_count,
            ),
            (
                "families_with_expiry_change",
                self.impact_summary.families_with_expiry_change
                    == computed.families_with_expiry_change,
            ),
            (
                "families_with_downgrade",
                self.impact_summary.families_with_downgrade == computed.families_with_downgrade,
            ),
        ] {
            if !ok {
                violations.push(M5PolicySimulationViolation::ImpactSummaryMismatch {
                    field: field.to_owned(),
                });
            }
        }

        violations
    }

    /// Recomputes the machine-readable impact roll-up from the rows alone.
    pub fn computed_impact_summary(&self) -> ImpactSummary {
        ImpactSummary {
            total_families: self.rows.len(),
            families_with_changes: self.rows.iter().filter(|row| row.has_changes()).count(),
            families_unchanged: self.rows.iter().filter(|row| !row.has_changes()).count(),
            changed_action_count: self
                .rows
                .iter()
                .map(|row| row.changed_actions().len())
                .sum(),
            unchanged_action_count: self
                .rows
                .iter()
                .map(|row| row.unchanged_actions().len())
                .sum(),
            impacted_object_count: self.rows.iter().map(|row| row.impacted_objects.len()).sum(),
            families_with_expiry_change: self
                .rows
                .iter()
                .filter(|row| row.expiry_effect.effect_class.changes_runtime())
                .count(),
            families_with_downgrade: self
                .rows
                .iter()
                .filter(|row| row.downgrade_path.path_class.is_downgrade())
                .count(),
        }
    }

    /// Returns the flattened machine-readable changed-action export rows.
    pub fn changed_action_export(&self) -> Vec<ChangedActionExportRow> {
        self.rows
            .iter()
            .flat_map(|row| {
                row.changed_actions()
                    .into_iter()
                    .map(|diff| ChangedActionExportRow {
                        artifact_family: row.artifact_family,
                        entry_id: row.entry_id.clone(),
                        action: diff.action,
                        current_outcome: diff.current_outcome,
                        draft_outcome: diff.draft_outcome,
                        effect_summary: diff.effect_summary.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Projects the product-surface rows.
    pub fn product_projection(&self) -> Vec<ProductSimulationRow> {
        self.rows
            .iter()
            .map(|row| {
                let delete = row.diff_for(SimulatedActionClass::Delete);
                let export = row.diff_for(SimulatedActionClass::Export);
                ProductSimulationRow {
                    artifact_family: row.artifact_family,
                    has_changes: row.has_changes(),
                    current_delete_outcome: delete
                        .map(|diff| diff.current_outcome)
                        .unwrap_or(RecordOperationOutcome::NotFound),
                    draft_delete_outcome: delete
                        .map(|diff| diff.draft_outcome)
                        .unwrap_or(RecordOperationOutcome::NotFound),
                    current_export_outcome: export
                        .map(|diff| diff.current_outcome)
                        .unwrap_or(RecordOperationOutcome::NotFound),
                    draft_export_outcome: export
                        .map(|diff| diff.draft_outcome)
                        .unwrap_or(RecordOperationOutcome::NotFound),
                    expiry_effect: row.expiry_effect.effect_class,
                    downgrade_path: row.downgrade_path.path_class,
                    impacted_object_count: row.impacted_objects.len(),
                }
            })
            .collect()
    }

    /// Projects the CLI/headless rows.
    pub fn cli_headless_projection(&self) -> Vec<CliHeadlessSimulationRow> {
        self.rows
            .iter()
            .map(|row| CliHeadlessSimulationRow {
                artifact_family: row.artifact_family,
                entry_id: row.entry_id.clone(),
                draft_policy_epoch: row.draft_policy_epoch.clone(),
                changed_actions: row.changed_actions().into_iter().cloned().collect(),
                unchanged_actions: row.unchanged_actions().into_iter().cloned().collect(),
                expiry_effect: row.expiry_effect.effect_class,
                downgrade_path: row.downgrade_path.path_class,
            })
            .collect()
    }

    /// Projects the support/export rows.
    pub fn support_export_projection(&self) -> Vec<SupportExportSimulationRow> {
        self.rows
            .iter()
            .map(|row| SupportExportSimulationRow {
                artifact_family: row.artifact_family,
                entry_id: row.entry_id.clone(),
                current_policy_epoch: row.current_policy_epoch.clone(),
                draft_policy_epoch: row.draft_policy_epoch.clone(),
                changed_actions: row
                    .changed_actions()
                    .into_iter()
                    .map(|diff| ChangedActionExportRow {
                        artifact_family: row.artifact_family,
                        entry_id: row.entry_id.clone(),
                        action: diff.action,
                        current_outcome: diff.current_outcome,
                        draft_outcome: diff.draft_outcome,
                        effect_summary: diff.effect_summary.clone(),
                    })
                    .collect(),
                impacted_objects: row.impacted_objects.clone(),
                expiry_effect: row.expiry_effect.clone(),
                downgrade_path: row.downgrade_path.clone(),
                exception_refs: row.exception_refs.clone(),
            })
            .collect()
    }

    /// Returns the set of exception refs the packet references.
    pub fn referenced_exception_ids(&self) -> Vec<String> {
        let mut refs: Vec<String> = self
            .rows
            .iter()
            .flat_map(|row| row.exception_refs.iter().cloned())
            .collect();
        refs.sort();
        refs.dedup();
        refs
    }
}

/// Maps a governed family to its canonical record class (identity contract).
const fn canonical_record_class_for_family(family: GovernedArtifactFamily) -> RecordClassId {
    match family {
        GovernedArtifactFamily::AiEvidencePacket => RecordClassId::AiRetainedEvidencePacket,
        GovernedArtifactFamily::ProviderLinkedWorkItem => {
            RecordClassId::ProviderLinkedWorkItemRecord
        }
        GovernedArtifactFamily::CompanionContinuityPacket => {
            RecordClassId::CompanionContinuityPacket
        }
        GovernedArtifactFamily::IncidentSupportPacket => RecordClassId::IncidentSupportPacket,
        GovernedArtifactFamily::SyncMirrorLedger => RecordClassId::SyncMirrorLedger,
        GovernedArtifactFamily::OffboardingRecord => RecordClassId::OffboardingExitPacket,
        GovernedArtifactFamily::BrowserHandoffManifest => RecordClassId::BrowserHandoffManifest,
        GovernedArtifactFamily::SupportExportPacket => RecordClassId::SupportExportPacket,
    }
}

/// Builds an action diff, deriving the `changed` flag from the outcomes.
fn action_diff(
    action: SimulatedActionClass,
    current: RecordOperationOutcome,
    draft: RecordOperationOutcome,
    effect_summary: &str,
) -> ActionDiff {
    ActionDiff {
        action,
        current_outcome: current,
        draft_outcome: draft,
        changed: current != draft,
        effect_summary: effect_summary.to_owned(),
    }
}

/// Returns the canonical seeded M5 policy-impact simulation packet.
///
/// The simulated rows share entry ids, families, and record classes with
/// [`crate::m5_records_policy::seeded_m5_records_policy_packet`] so the
/// simulation and the runtime surface it previews refer to the same objects.
pub fn seeded_m5_policy_simulation_packet() -> M5PolicyImpactSimulationPacket {
    let current_epoch = "policy:m5-records:v1";
    let draft_epoch = "policy:m5-records:v2-draft";

    let rows = vec![
        // AI evidence packet — draft shortens the managed retention floor so a
        // delete that was policy-retained becomes purgeable.
        PolicySimulationRow {
            entry_id: "m5-records-policy:ai-evidence".to_owned(),
            title: "AI retained evidence packet".to_owned(),
            artifact_family: GovernedArtifactFamily::AiEvidencePacket,
            record_class_id: RecordClassId::AiRetainedEvidencePacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: true,
            draft_claims_managed_export: true,
            draft_claims_managed_delete: true,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::PolicyRetained,
                    RecordOperationOutcome::Completed,
                    "Shortened retention floor makes the managed evidence copy deletable now.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::Completed,
                    RecordOperationOutcome::Completed,
                    "Export still produces a signed evidence manifest.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:ai-evidence-case-0001".to_owned(),
                record_class_id: RecordClassId::AiRetainedEvidencePacket,
                producer_record_kinds: vec!["ai_evidence_packet_finalization".to_owned()],
                managed_copy: true,
                note: None,
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Shortened,
                current_expiry_rule: "Managed evidence retained for the full policy horizon."
                    .to_owned(),
                draft_expiry_rule: "Managed evidence retained for a shortened investigation window."
                    .to_owned(),
                runtime_consequence:
                    "Delete completes immediately once the shortened window elapses instead of \
                     reporting policy_retained."
                        .to_owned(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::None,
                from_behavior: "Managed evidence is policy-retained on delete.".to_owned(),
                to_behavior: "Managed evidence is deletable after the shortened window.".to_owned(),
                visible_before_publish: true,
            },
            exception_refs: vec!["m5-exception:ai-evidence-retention-waiver".to_owned()],
            proof_ref: "proof:m5-policy-sim:ai-evidence".to_owned(),
            rationale: "Admins see the retention floor shrink and delete become possible before \
                        publishing the waiver."
                .to_owned(),
        },
        // Provider-linked work item — local-only; the draft tightens local
        // retention but neither action's outcome changes.
        PolicySimulationRow {
            entry_id: "m5-records-policy:provider-linked".to_owned(),
            title: "Provider-linked work item".to_owned(),
            artifact_family: GovernedArtifactFamily::ProviderLinkedWorkItem,
            record_class_id: RecordClassId::ProviderLinkedWorkItemRecord,
            authority_boundary: AuthorityBoundaryClass::LocalOnly,
            local_truth_authority: LocalVsManagedCopy::LocalAuthoritative,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: false,
            draft_claims_managed_export: false,
            draft_claims_managed_delete: false,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::NotFound,
                    RecordOperationOutcome::NotFound,
                    "Only local linkage metadata is removed; there is no managed copy to delete.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::OutsidePlatformScope,
                    RecordOperationOutcome::OutsidePlatformScope,
                    "The provider record stays off-platform and is not exportable here.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:provider-linked-index".to_owned(),
                record_class_id: RecordClassId::ProviderLinkedWorkItemRecord,
                producer_record_kinds: vec![
                    "ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews"
                        .to_owned(),
                ],
                managed_copy: false,
                note: Some("Only local linkage metadata is in scope.".to_owned()),
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Shortened,
                current_expiry_rule: "Local linkage metadata is user-owned until cleared."
                    .to_owned(),
                draft_expiry_rule: "Local linkage metadata auto-clears after an idle window."
                    .to_owned(),
                runtime_consequence:
                    "Local linkage metadata is purged sooner on device; no remote copy is affected."
                        .to_owned(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::None,
                from_behavior: "Local linkage metadata is kept until cleared.".to_owned(),
                to_behavior: "Local linkage metadata auto-clears after an idle window.".to_owned(),
                visible_before_publish: true,
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-policy-sim:provider-linked".to_owned(),
            rationale: "A local-only draft tightens local retention without implying any remote \
                        delete or export."
                .to_owned(),
        },
        // Companion continuity packet — draft adds export redaction; the hold
        // keeps delete blocked.
        PolicySimulationRow {
            entry_id: "m5-records-policy:companion".to_owned(),
            title: "Companion continuity packet".to_owned(),
            artifact_family: GovernedArtifactFamily::CompanionContinuityPacket,
            record_class_id: RecordClassId::CompanionContinuityPacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: true,
            draft_claims_managed_export: true,
            draft_claims_managed_delete: true,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::BlockedByHold,
                    RecordOperationOutcome::BlockedByHold,
                    "The indeterminate hold keeps deletion blocked fail-closed.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::Completed,
                    RecordOperationOutcome::OmittedByRedaction,
                    "A stricter redaction profile omits secret-bearing rows from the export.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:companion-packet-0001".to_owned(),
                record_class_id: RecordClassId::CompanionContinuityPacket,
                producer_record_kinds: vec![
                    "ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes"
                        .to_owned(),
                ],
                managed_copy: true,
                note: None,
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Unchanged,
                current_expiry_rule: "Held until the hold evaluation resolves.".to_owned(),
                draft_expiry_rule: "Held until the hold evaluation resolves.".to_owned(),
                runtime_consequence: String::new(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::ExportToOmittedByRedaction,
                from_behavior: "Export completes with the full companion packet.".to_owned(),
                to_behavior: "Export omits secret-bearing rows under the stricter profile."
                    .to_owned(),
                visible_before_publish: true,
            },
            exception_refs: vec!["m5-exception:companion-hold-review".to_owned()],
            proof_ref: "proof:m5-policy-sim:companion".to_owned(),
            rationale: "The redaction downgrade is visible in the simulation, not discovered after \
                        export."
                .to_owned(),
        },
        // Incident support packet — draft extends support-case retention,
        // deferring delete.
        PolicySimulationRow {
            entry_id: "m5-records-policy:incident".to_owned(),
            title: "Incident support packet".to_owned(),
            artifact_family: GovernedArtifactFamily::IncidentSupportPacket,
            record_class_id: RecordClassId::IncidentSupportPacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: true,
            draft_claims_managed_export: true,
            draft_claims_managed_delete: true,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::Completed,
                    RecordOperationOutcome::PolicyRetained,
                    "An extended support-case retention floor defers deletion.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::OmittedByRedaction,
                    RecordOperationOutcome::OmittedByRedaction,
                    "Secret-bearing rows remain omitted by the default redaction profile.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:incident-packet-0001".to_owned(),
                record_class_id: RecordClassId::IncidentSupportPacket,
                producer_record_kinds: vec![
                    "add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets"
                        .to_owned(),
                ],
                managed_copy: true,
                note: None,
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Extended,
                current_expiry_rule: "Retained for the standard support-case window.".to_owned(),
                draft_expiry_rule: "Retained for an extended support-case window.".to_owned(),
                runtime_consequence:
                    "Delete reports policy_retained until the extended window elapses instead of \
                     completing immediately."
                        .to_owned(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::CompletedToPolicyRetained,
                from_behavior: "Delete completes and emits a receipt.".to_owned(),
                to_behavior: "Delete is deferred until the extended retention window elapses."
                    .to_owned(),
                visible_before_publish: true,
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-policy-sim:incident".to_owned(),
            rationale: "The extended retention floor and the resulting deferred delete are shown \
                        before publish."
                .to_owned(),
        },
        // Sync mirror ledger — draft enables managed export of the mirror,
        // producing a partial export alongside local snapshots.
        PolicySimulationRow {
            entry_id: "m5-records-policy:sync-mirror".to_owned(),
            title: "Sync mirror ledger".to_owned(),
            artifact_family: GovernedArtifactFamily::SyncMirrorLedger,
            record_class_id: RecordClassId::SyncMirrorLedger,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: true,
            draft_claims_managed_export: true,
            draft_claims_managed_delete: true,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::BlockedByHold,
                    RecordOperationOutcome::BlockedByHold,
                    "The active litigation hold keeps managed-mirror deletion blocked.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::ManualLocalCaptureRequired,
                    RecordOperationOutcome::Partial,
                    "Managed export now covers the mirror; local snapshots remain a manual step, \
                     so the export is partial.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:sync-mirror-ledger-0001".to_owned(),
                record_class_id: RecordClassId::SyncMirrorLedger,
                producer_record_kinds: vec![
                    "ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage"
                        .to_owned(),
                ],
                managed_copy: true,
                note: Some("Local per-device snapshots stay outside managed export.".to_owned()),
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Unchanged,
                current_expiry_rule: "Managed mirror retained under the active hold.".to_owned(),
                draft_expiry_rule: "Managed mirror retained under the active hold.".to_owned(),
                runtime_consequence: String::new(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::None,
                from_behavior: "Export requires a manual on-device capture of the mirror."
                    .to_owned(),
                to_behavior: "Managed export covers the mirror; local snapshots stay manual."
                    .to_owned(),
                visible_before_publish: true,
            },
            exception_refs: vec!["m5-exception:sync-mirror-hold".to_owned()],
            proof_ref: "proof:m5-policy-sim:sync-mirror".to_owned(),
            rationale: "Admins see the export improve to partial while the hold still blocks delete."
                .to_owned(),
        },
        // Browser handoff manifest — local-only; draft introduces a local
        // auto-clear window without changing either action.
        PolicySimulationRow {
            entry_id: "m5-records-policy:browser-handoff".to_owned(),
            title: "Browser handoff manifest".to_owned(),
            artifact_family: GovernedArtifactFamily::BrowserHandoffManifest,
            record_class_id: RecordClassId::BrowserHandoffManifest,
            authority_boundary: AuthorityBoundaryClass::LocalOnly,
            local_truth_authority: LocalVsManagedCopy::LocalCacheOnly,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: false,
            draft_claims_managed_export: false,
            draft_claims_managed_delete: false,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::Completed,
                    RecordOperationOutcome::Completed,
                    "The local manifest is still deleted on device.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::OutsidePlatformScope,
                    RecordOperationOutcome::OutsidePlatformScope,
                    "Handoff to the system browser stays outside platform export scope.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:browser-handoff-store".to_owned(),
                record_class_id: RecordClassId::BrowserHandoffManifest,
                producer_record_kinds: vec![
                    "ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links"
                        .to_owned(),
                ],
                managed_copy: false,
                note: Some("Manifests live only on device.".to_owned()),
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Introduced,
                current_expiry_rule: "User-owned until exported or deleted on device.".to_owned(),
                draft_expiry_rule: "Auto-cleared on device after an idle window.".to_owned(),
                runtime_consequence:
                    "Stale local manifests are cleared automatically on device; nothing is held \
                     or stored remotely."
                        .to_owned(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::None,
                from_behavior: "Manifests persist on device until acted on.".to_owned(),
                to_behavior: "Manifests auto-clear on device after an idle window.".to_owned(),
                visible_before_publish: true,
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-policy-sim:browser-handoff".to_owned(),
            rationale: "A local-only auto-clear is previewed as a local expiry effect, never a \
                        remote hold."
                .to_owned(),
        },
        // Offboarding record — draft shortens the retention floor so a deferred
        // delete becomes possible.
        PolicySimulationRow {
            entry_id: "m5-records-policy:offboarding".to_owned(),
            title: "Offboarding exit packet".to_owned(),
            artifact_family: GovernedArtifactFamily::OffboardingRecord,
            record_class_id: RecordClassId::OffboardingExitPacket,
            authority_boundary: AuthorityBoundaryClass::ManagedOnly,
            local_truth_authority: LocalVsManagedCopy::ManagedAuthoritative,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: true,
            draft_claims_managed_export: true,
            draft_claims_managed_delete: true,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::PolicyRetained,
                    RecordOperationOutcome::Completed,
                    "An approved waiver shortens the retention floor so the delete completes.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::Partial,
                    RecordOperationOutcome::Partial,
                    "Export stays partial; omitted classes remain listed in the manifest.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:offboarding-packet-0001".to_owned(),
                record_class_id: RecordClassId::OffboardingExitPacket,
                producer_record_kinds: vec![
                    "implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho"
                        .to_owned(),
                ],
                managed_copy: true,
                note: None,
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Shortened,
                current_expiry_rule: "Retained until the standard retention-floor horizon elapses."
                    .to_owned(),
                draft_expiry_rule: "Retained until a shortened, waiver-approved horizon elapses."
                    .to_owned(),
                runtime_consequence:
                    "Delete completes once the shortened floor elapses instead of reporting \
                     policy_retained."
                        .to_owned(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::None,
                from_behavior: "Delete is deferred by the retention floor.".to_owned(),
                to_behavior: "Delete completes after the shortened, waiver-approved floor."
                    .to_owned(),
                visible_before_publish: true,
            },
            exception_refs: vec!["m5-exception:offboarding-retention-floor".to_owned()],
            proof_ref: "proof:m5-policy-sim:offboarding".to_owned(),
            rationale: "The shortened floor and the resulting completable delete are shown before \
                        the waiver publishes."
                .to_owned(),
        },
        // Support export packet — draft applies a stricter redaction profile to
        // the generated export.
        PolicySimulationRow {
            entry_id: "m5-records-policy:support-export".to_owned(),
            title: "Support export packet".to_owned(),
            artifact_family: GovernedArtifactFamily::SupportExportPacket,
            record_class_id: RecordClassId::SupportExportPacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::GeneratedPacketAuthoritative,
            current_policy_epoch: current_epoch.to_owned(),
            draft_policy_epoch: draft_epoch.to_owned(),
            draft_claims_managed_hold: false,
            draft_claims_managed_export: true,
            draft_claims_managed_delete: true,
            action_diffs: vec![
                action_diff(
                    SimulatedActionClass::Delete,
                    RecordOperationOutcome::Completed,
                    RecordOperationOutcome::Completed,
                    "The generated support packet is still deleted with a receipt.",
                ),
                action_diff(
                    SimulatedActionClass::Export,
                    RecordOperationOutcome::Completed,
                    RecordOperationOutcome::OmittedByRedaction,
                    "A stricter redaction profile omits secret-bearing rows from the support export.",
                ),
            ],
            impacted_objects: vec![ImpactedObject {
                object_ref: "object:support-export-packet-0001".to_owned(),
                record_class_id: RecordClassId::SupportExportPacket,
                producer_record_kinds: vec!["records_policy_governance_support_export".to_owned()],
                managed_copy: true,
                note: None,
            }],
            expiry_effect: ExpiryEffect {
                effect_class: ExpiryEffectClass::Unchanged,
                current_expiry_rule: "Retained for the support delivery window.".to_owned(),
                draft_expiry_rule: "Retained for the support delivery window.".to_owned(),
                runtime_consequence: String::new(),
                effective_at: "2026-07-01T00:00:00Z".to_owned(),
            },
            downgrade_path: DowngradePath {
                path_class: DowngradePathClass::ExportToOmittedByRedaction,
                from_behavior: "Export delivers the full support packet with its manifest."
                    .to_owned(),
                to_behavior: "Export omits secret-bearing rows under the stricter profile."
                    .to_owned(),
                visible_before_publish: true,
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-policy-sim:support-export".to_owned(),
            rationale: "The export redaction downgrade is previewed before the stricter profile \
                        publishes."
                .to_owned(),
        },
    ];

    let mut packet = M5PolicyImpactSimulationPacket {
        schema_version: M5_POLICY_SIMULATION_SCHEMA_VERSION,
        record_kind: M5_POLICY_SIMULATION_RECORD_KIND.to_owned(),
        packet_id: "m5-policy-simulation:0001".to_owned(),
        shared_contract_ref: M5_POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
        as_of: "2026-06-13T16:00:00Z".to_owned(),
        overview_doc_ref: M5_POLICY_SIMULATION_DOC_REF.to_owned(),
        artifact_summary_ref: M5_POLICY_SIMULATION_ARTIFACT_REF.to_owned(),
        runtime_contract_ref: M5_POLICY_SIMULATION_RUNTIME_CONTRACT_REF.to_owned(),
        exception_expiry_contract_ref: M5_POLICY_SIMULATION_EXCEPTION_CONTRACT_REF.to_owned(),
        rows,
        impact_summary: ImpactSummary {
            total_families: 0,
            families_with_changes: 0,
            families_unchanged: 0,
            changed_action_count: 0,
            unchanged_action_count: 0,
            impacted_object_count: 0,
            families_with_expiry_change: 0,
            families_with_downgrade: 0,
        },
        summary: "Canonical pre-apply policy simulation comparing the current and draft M5 \
                  records policy: changed and unchanged delete/export actions, impacted objects, \
                  expiry effects, and downgrade paths for the eight governed artifact families."
            .to_owned(),
    };
    packet.impact_summary = packet.computed_impact_summary();
    packet
}
