//! Legal-hold notices, hold selector scopes, retention/archive inspectors, and
//! pre-action blocker truth for M5 managed/support surfaces.
//!
//! This module is the canonical legal-hold and retention truth source for the
//! durable artifact families introduced in M5. Each governed family carries a
//! typed [`HoldNotice`], a [`HoldSelectorScope`], a [`RetentionInspector`], an
//! [`ArchiveInspector`], and the [`PreActionTruth`] the user sees *before*
//! committing a destructive or support-sensitive action. The packet exposes
//! product, CLI/headless, and support/export projections that all read the same
//! hold/retention/blocker vocabulary so no surface can collapse a real
//! `blocked_by_hold`, `policy_retained`, or `outside_platform_scope` boundary
//! into generic denial copy.
//!
//! The packet is metadata-only: it carries hold refs, owners, and scopes but no
//! credential bodies, raw provider payloads, or durable content. Local-only
//! artifacts never claim managed hold, managed export, or managed delete; those
//! boundaries are stated explicitly and disclosed to every consumer.

use serde::{Deserialize, Serialize};

use crate::records_policy_simulation_matrix::{AuthorityBoundaryClass, GovernedArtifactFamily};
use crate::stabilize_record_class_registry_legal_hold_delete_honesty::{
    HoldScope, HoldStatus, RecordOperationOutcome,
};
use crate::{LocalVsManagedCopy, RecordClassId, RetentionLabel};

#[cfg(test)]
mod tests;

/// Schema version for the M5 records-policy hold/retention packet.
pub const M5_RECORDS_POLICY_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the top-level packet.
pub const M5_RECORDS_POLICY_RECORD_KIND: &str = "m5_records_policy_packet";

/// Shared contract reference shared with the policy exception/expiry lane.
pub const M5_RECORDS_POLICY_SHARED_CONTRACT_REF: &str = "records:m5_hold_retention_truth:v1";

/// Repo-relative doc reference for the hold/retention contract.
pub const M5_RECORDS_POLICY_DOC_REF: &str = "docs/governance/m5_records_policy_sim.md";

/// Repo-relative artifact summary for the hold/retention contract.
pub const M5_RECORDS_POLICY_ARTIFACT_REF: &str = "artifacts/governance/m5_records_policy_sim.md";

/// Repo-relative schema reference for the hold/retention contract.
pub const M5_RECORDS_POLICY_SCHEMA_REF: &str =
    "schemas/governance/m5_records_policy_sim.schema.json";

/// Repo-relative fixture directory for the canonical packet.
pub const M5_RECORDS_POLICY_FIXTURE_DIR: &str = "fixtures/governance/m5_records_policy_sim";

/// Destructive or support-sensitive action whose truth is surfaced pre-commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructiveActionClass {
    /// Delete or destroy the artifact.
    Delete,
    /// Export the artifact off the producing surface.
    Export,
}

impl DestructiveActionClass {
    /// Returns the stable snake_case token for the action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Delete => "delete",
            Self::Export => "export",
        }
    }
}

/// Inspectable archive state for a governed family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveStateClass {
    /// Live, on-device working copy.
    ActiveLocal,
    /// Archived on-device under user control.
    ArchivedLocal,
    /// Archived in a managed/control-plane store.
    ManagedArchive,
    /// No archive copy is retained.
    NoArchive,
}

impl ArchiveStateClass {
    /// Returns the stable snake_case token for the archive state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveLocal => "active_local",
            Self::ArchivedLocal => "archived_local",
            Self::ManagedArchive => "managed_archive",
            Self::NoArchive => "no_archive",
        }
    }

    /// Returns true when the archive copy lives in a managed/control-plane store.
    pub const fn is_managed(self) -> bool {
        matches!(self, Self::ManagedArchive)
    }
}

/// Legal-hold notice surfaced before a destructive or support-sensitive action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldNotice {
    /// Stable notice id.
    pub notice_id: String,
    /// Fail-closed hold status: active or indeterminate blocks destruction.
    pub hold_status: HoldStatus,
    /// Scope the hold actually covers (managed-only, local-only, or both).
    pub hold_scope: HoldScope,
    /// Hold references backing this notice.
    pub active_hold_refs: Vec<String>,
    /// User-visible notice text shown before the action commits.
    pub notice_text: String,
    /// Retention owner accountable for the hold.
    pub retention_owner_ref: String,
    /// Explicit note when the artifact is local-only and cannot be held remotely.
    pub local_only_artifact_note: Option<String>,
    /// Whether this notice blocks a destructive action (fail-closed).
    pub blocks_destructive_action: bool,
}

impl HoldNotice {
    /// Returns true when the hold status requires fail-closed blocking.
    pub const fn status_requires_block(&self) -> bool {
        matches!(
            self.hold_status,
            HoldStatus::Active | HoldStatus::UnknownIndeterminate
        )
    }
}

/// Hold selector scope describing which copies a hold actually reaches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldSelectorScope {
    /// Stable selector id.
    pub selector_id: String,
    /// Scope the selector reaches.
    pub scope: HoldScope,
    /// Human-readable selector expression.
    pub selector_expression: String,
    /// Record classes the selector covers.
    pub included_record_class_ids: Vec<RecordClassId>,
    /// Whether a managed copy is in scope.
    pub managed_copy_covered: bool,
    /// Whether a local copy is in scope.
    pub local_copy_covered: bool,
    /// Optional scope note.
    pub note: Option<String>,
}

/// Retention inspector exposing the retention owner and retention rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionInspector {
    /// Stable inspector id.
    pub inspector_id: String,
    /// Retention owner accountable for the artifact.
    pub retention_owner_ref: String,
    /// Stable retention label from the record-class registry.
    pub retention_label: RetentionLabel,
    /// Human-readable retention rule.
    pub retention_rule: String,
    /// Delete action the retention rule permits.
    pub delete_action: String,
    /// Grace rule applied before destruction.
    pub grace_rule: String,
    /// Policy version backing the retention rule.
    pub policy_version: String,
    /// Local retention owner.
    pub local_owner_ref: String,
    /// Managed retention owner.
    pub managed_owner_ref: String,
}

/// Archive inspector exposing the archive state and managed/local boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveInspector {
    /// Stable inspector id.
    pub inspector_id: String,
    /// Inspectable archive state.
    pub archive_state: ArchiveStateClass,
    /// Human-readable archive location label.
    pub archive_location_label: String,
    /// Whether a managed archive copy exists.
    pub managed_archive: bool,
    /// Explicit note when the only archive is local.
    pub local_only_archive_note: Option<String>,
}

/// Pre-action truth shown before a delete or export is committed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreActionTruth {
    /// Action class this truth describes.
    pub action: DestructiveActionClass,
    /// Outcome the user would actually get if they committed the action now.
    pub projected_outcome: RecordOperationOutcome,
    /// Plain-language reason for the projected outcome.
    pub reason: String,
    /// Confirmation copy shown before committing (never a bare "done").
    pub confirmation_copy: String,
    /// Whether the artifact is outside the platform's deletion/export scope.
    pub outside_platform_scope: bool,
    /// Explicit local-only or outside-scope boundary note.
    pub local_only_boundary_note: Option<String>,
}

/// One governed M5 family row binding hold, retention, archive, and pre-action truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RecordsPolicyRow {
    /// Stable entry id.
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
    /// Whether the family claims a managed legal hold.
    pub claims_managed_hold: bool,
    /// Whether the family claims a managed export.
    pub claims_managed_export: bool,
    /// Whether the family claims a managed delete.
    pub claims_managed_delete: bool,
    /// Legal-hold notice for the family.
    pub legal_hold_notice: HoldNotice,
    /// Hold selector scope for the family.
    pub hold_selector_scope: HoldSelectorScope,
    /// Retention inspector for the family.
    pub retention_inspector: RetentionInspector,
    /// Archive inspector for the family.
    pub archive_inspector: ArchiveInspector,
    /// Pre-delete truth surfaced before a delete commits.
    pub pre_delete_truth: PreActionTruth,
    /// Pre-export truth surfaced before an export commits.
    pub pre_export_truth: PreActionTruth,
    /// References into the policy exception/expiry lane.
    pub exception_refs: Vec<String>,
    /// Proof reference backing the row.
    pub proof_ref: String,
    /// Rationale for the row.
    pub rationale: String,
}

/// Product surface projection row for one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductInspectorRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Hold status badge.
    pub hold_status: HoldStatus,
    /// Retention owner shown inline.
    pub retention_owner_ref: String,
    /// Archive state badge.
    pub archive_state: ArchiveStateClass,
    /// Pre-delete projected outcome.
    pub pre_delete_outcome: RecordOperationOutcome,
    /// Pre-export projected outcome.
    pub pre_export_outcome: RecordOperationOutcome,
    /// Whether the delete is blocked by a hold.
    pub delete_blocked_by_hold: bool,
    /// Whether either action is outside platform scope.
    pub outside_platform_scope: bool,
}

/// CLI/headless projection row for one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliHeadlessInspectorRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Hold status token.
    pub hold_status: HoldStatus,
    /// Hold scope token.
    pub hold_scope: HoldScope,
    /// Pre-delete projected outcome.
    pub pre_delete_outcome: RecordOperationOutcome,
    /// Pre-export projected outcome.
    pub pre_export_outcome: RecordOperationOutcome,
}

/// Support/export projection row for one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportInspectorRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Hold notice id.
    pub hold_notice_id: String,
    /// Hold status.
    pub hold_status: HoldStatus,
    /// Hold selector scope id.
    pub hold_selector_id: String,
    /// Retention owner ref.
    pub retention_owner_ref: String,
    /// Archive state.
    pub archive_state: ArchiveStateClass,
    /// Pre-delete projected outcome.
    pub pre_delete_outcome: RecordOperationOutcome,
    /// Pre-export projected outcome.
    pub pre_export_outcome: RecordOperationOutcome,
    /// Local-only or outside-scope boundary note when present.
    pub local_only_boundary_note: Option<String>,
    /// Exception/expiry references for the family.
    pub exception_refs: Vec<String>,
}

/// Top-level canonical M5 hold/retention truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RecordsPolicyPacket {
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
    /// Reference to the policy exception/expiry contract.
    pub exception_expiry_contract_ref: String,
    /// Governed family rows.
    pub rows: Vec<M5RecordsPolicyRow>,
    /// Review-safe summary.
    pub summary: String,
}

/// Validation issue emitted by the M5 hold/retention packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum M5RecordsPolicyViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch { found: u32 },
    /// Record kind mismatch.
    RecordKindMismatch { found: String },
    /// Local-only family claims a managed legal hold.
    LocalOnlyClaimsManagedHold { entry_id: String },
    /// Local-only family claims a managed export.
    LocalOnlyClaimsManagedExport { entry_id: String },
    /// Local-only family claims a managed delete.
    LocalOnlyClaimsManagedDelete { entry_id: String },
    /// Active/indeterminate hold did not fail closed on delete.
    HoldNotFailClosed { entry_id: String },
    /// A pre-action truth omitted its reason.
    PreActionReasonMissing {
        entry_id: String,
        action: DestructiveActionClass,
    },
    /// A pre-action truth omitted its confirmation copy.
    PreActionConfirmationMissing {
        entry_id: String,
        action: DestructiveActionClass,
    },
    /// Outside-platform-scope flag disagrees with the projected outcome.
    OutsideScopeOutcomeMismatch {
        entry_id: String,
        action: DestructiveActionClass,
    },
    /// Retention inspector omitted a retention owner.
    RetentionOwnerMissing { entry_id: String },
    /// Family claims a managed hold but the selector cannot reach a managed copy.
    ManagedHoldWithoutManagedScope { entry_id: String },
    /// Family advertises a managed archive while authority is local-only.
    ManagedArchiveLocalOnly { entry_id: String },
    /// A required governed family is missing.
    FamilyCoverageMissing {
        artifact_family: GovernedArtifactFamily,
    },
}

impl M5RecordsPolicyPacket {
    /// Returns true when the family's authoritative copy is local-only.
    fn is_local_only(authority: AuthorityBoundaryClass, copy: LocalVsManagedCopy) -> bool {
        matches!(authority, AuthorityBoundaryClass::LocalOnly)
            || matches!(
                copy,
                LocalVsManagedCopy::LocalAuthoritative | LocalVsManagedCopy::LocalCacheOnly
            )
    }

    /// Validates the packet against the hold/retention honesty contract.
    pub fn validate(&self) -> Vec<M5RecordsPolicyViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_RECORDS_POLICY_SCHEMA_VERSION {
            violations.push(M5RecordsPolicyViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_RECORDS_POLICY_RECORD_KIND {
            violations.push(M5RecordsPolicyViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        for row in &self.rows {
            let local_only = Self::is_local_only(row.authority_boundary, row.local_truth_authority);

            if local_only {
                if row.claims_managed_hold {
                    violations.push(M5RecordsPolicyViolation::LocalOnlyClaimsManagedHold {
                        entry_id: row.entry_id.clone(),
                    });
                }
                if row.claims_managed_export {
                    violations.push(M5RecordsPolicyViolation::LocalOnlyClaimsManagedExport {
                        entry_id: row.entry_id.clone(),
                    });
                }
                if row.claims_managed_delete {
                    violations.push(M5RecordsPolicyViolation::LocalOnlyClaimsManagedDelete {
                        entry_id: row.entry_id.clone(),
                    });
                }
            }

            // Fail-closed: an active or indeterminate hold must block destruction and
            // surface `blocked_by_hold` on the pre-delete truth.
            if row.legal_hold_notice.status_requires_block()
                && (!row.legal_hold_notice.blocks_destructive_action
                    || row.pre_delete_truth.projected_outcome
                        != RecordOperationOutcome::BlockedByHold)
            {
                violations.push(M5RecordsPolicyViolation::HoldNotFailClosed {
                    entry_id: row.entry_id.clone(),
                });
            }

            for truth in [&row.pre_delete_truth, &row.pre_export_truth] {
                if truth.reason.trim().is_empty() {
                    violations.push(M5RecordsPolicyViolation::PreActionReasonMissing {
                        entry_id: row.entry_id.clone(),
                        action: truth.action,
                    });
                }
                if truth.confirmation_copy.trim().is_empty() {
                    violations.push(M5RecordsPolicyViolation::PreActionConfirmationMissing {
                        entry_id: row.entry_id.clone(),
                        action: truth.action,
                    });
                }
                let outcome_is_outside =
                    truth.projected_outcome == RecordOperationOutcome::OutsidePlatformScope;
                if truth.outside_platform_scope != outcome_is_outside {
                    violations.push(M5RecordsPolicyViolation::OutsideScopeOutcomeMismatch {
                        entry_id: row.entry_id.clone(),
                        action: truth.action,
                    });
                }
            }

            if row
                .retention_inspector
                .retention_owner_ref
                .trim()
                .is_empty()
            {
                violations.push(M5RecordsPolicyViolation::RetentionOwnerMissing {
                    entry_id: row.entry_id.clone(),
                });
            }

            if row.claims_managed_hold
                && !matches!(
                    row.hold_selector_scope.scope,
                    HoldScope::ManagedOnly | HoldScope::Both
                )
            {
                violations.push(M5RecordsPolicyViolation::ManagedHoldWithoutManagedScope {
                    entry_id: row.entry_id.clone(),
                });
            }

            if row.archive_inspector.managed_archive && local_only {
                violations.push(M5RecordsPolicyViolation::ManagedArchiveLocalOnly {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        for family in GovernedArtifactFamily::ALL {
            if !self.rows.iter().any(|row| row.artifact_family == family) {
                violations.push(M5RecordsPolicyViolation::FamilyCoverageMissing {
                    artifact_family: family,
                });
            }
        }

        violations
    }

    /// Projects the product surface rows.
    pub fn product_projection(&self) -> Vec<ProductInspectorRow> {
        self.rows
            .iter()
            .map(|row| ProductInspectorRow {
                artifact_family: row.artifact_family,
                hold_status: row.legal_hold_notice.hold_status,
                retention_owner_ref: row.retention_inspector.retention_owner_ref.clone(),
                archive_state: row.archive_inspector.archive_state,
                pre_delete_outcome: row.pre_delete_truth.projected_outcome,
                pre_export_outcome: row.pre_export_truth.projected_outcome,
                delete_blocked_by_hold: row.pre_delete_truth.projected_outcome
                    == RecordOperationOutcome::BlockedByHold,
                outside_platform_scope: row.pre_delete_truth.outside_platform_scope
                    || row.pre_export_truth.outside_platform_scope,
            })
            .collect()
    }

    /// Projects the CLI/headless rows.
    pub fn cli_headless_projection(&self) -> Vec<CliHeadlessInspectorRow> {
        self.rows
            .iter()
            .map(|row| CliHeadlessInspectorRow {
                artifact_family: row.artifact_family,
                hold_status: row.legal_hold_notice.hold_status,
                hold_scope: row.legal_hold_notice.hold_scope,
                pre_delete_outcome: row.pre_delete_truth.projected_outcome,
                pre_export_outcome: row.pre_export_truth.projected_outcome,
            })
            .collect()
    }

    /// Projects the support/export rows.
    pub fn support_export_projection(&self) -> Vec<SupportExportInspectorRow> {
        self.rows
            .iter()
            .map(|row| {
                let local_only_boundary_note = row
                    .pre_delete_truth
                    .local_only_boundary_note
                    .clone()
                    .or_else(|| row.pre_export_truth.local_only_boundary_note.clone())
                    .or_else(|| row.legal_hold_notice.local_only_artifact_note.clone());
                SupportExportInspectorRow {
                    artifact_family: row.artifact_family,
                    hold_notice_id: row.legal_hold_notice.notice_id.clone(),
                    hold_status: row.legal_hold_notice.hold_status,
                    hold_selector_id: row.hold_selector_scope.selector_id.clone(),
                    retention_owner_ref: row.retention_inspector.retention_owner_ref.clone(),
                    archive_state: row.archive_inspector.archive_state,
                    pre_delete_outcome: row.pre_delete_truth.projected_outcome,
                    pre_export_outcome: row.pre_export_truth.projected_outcome,
                    local_only_boundary_note,
                    exception_refs: row.exception_refs.clone(),
                }
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

/// Builds a hold notice with no active hold (cleared, fail-open for destruction).
fn cleared_hold(notice_id: &str, scope: HoldScope, owner: &str) -> HoldNotice {
    HoldNotice {
        notice_id: notice_id.to_owned(),
        hold_status: HoldStatus::Cleared,
        hold_scope: scope,
        active_hold_refs: Vec::new(),
        notice_text: "No active legal hold blocks this action.".to_owned(),
        retention_owner_ref: owner.to_owned(),
        local_only_artifact_note: None,
        blocks_destructive_action: false,
    }
}

/// Returns the canonical seeded M5 hold/retention truth packet.
pub fn seeded_m5_records_policy_packet() -> M5RecordsPolicyPacket {
    let rows = vec![
        // AI evidence packet — managed copy retains policy-held evidence.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:ai-evidence".to_owned(),
            title: "AI retained evidence packet".to_owned(),
            artifact_family: GovernedArtifactFamily::AiEvidencePacket,
            record_class_id: RecordClassId::AiRetainedEvidencePacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            claims_managed_hold: true,
            claims_managed_export: true,
            claims_managed_delete: true,
            legal_hold_notice: cleared_hold(
                "hold-notice:ai-evidence",
                HoldScope::Both,
                "owner:records-governance",
            ),
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:ai-evidence".to_owned(),
                scope: HoldScope::Both,
                selector_expression: "class=ai_retained_evidence_packet".to_owned(),
                included_record_class_ids: vec![RecordClassId::AiRetainedEvidencePacket],
                managed_copy_covered: true,
                local_copy_covered: true,
                note: None,
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:ai-evidence".to_owned(),
                retention_owner_ref: "owner:records-governance".to_owned(),
                retention_label: RetentionLabel::ManagedPolicyRetained,
                retention_rule: "Managed evidence copies retained for the policy horizon."
                    .to_owned(),
                delete_action: "managed_delete_with_receipt".to_owned(),
                grace_rule: "No grace window; retention floor applies.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:org-admin".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:ai-evidence".to_owned(),
                archive_state: ArchiveStateClass::ManagedArchive,
                archive_location_label: "Managed evidence archive".to_owned(),
                managed_archive: true,
                local_only_archive_note: None,
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::PolicyRetained,
                reason: "Managed evidence copies are policy-retained and survive a local delete."
                    .to_owned(),
                confirmation_copy:
                    "Deleting removes local caches; managed evidence copies remain policy-retained."
                        .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: Some(
                    "Local prompt/result caches are deleted on device.".to_owned(),
                ),
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::Completed,
                reason: "Evidence packet exports with a signed manifest.".to_owned(),
                confirmation_copy: "Export produces a signed evidence manifest.".to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            exception_refs: vec!["m5-exception:ai-evidence-retention-waiver".to_owned()],
            proof_ref: "proof:m5-records-policy:ai-evidence".to_owned(),
            rationale: "AI evidence keeps managed retention truth visible before delete."
                .to_owned(),
        },
        // Provider-linked work item — only local linkage metadata exists.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:provider-linked".to_owned(),
            title: "Provider-linked work item".to_owned(),
            artifact_family: GovernedArtifactFamily::ProviderLinkedWorkItem,
            record_class_id: RecordClassId::ProviderLinkedWorkItemRecord,
            authority_boundary: AuthorityBoundaryClass::LocalOnly,
            local_truth_authority: LocalVsManagedCopy::LocalAuthoritative,
            claims_managed_hold: false,
            claims_managed_export: false,
            claims_managed_delete: false,
            legal_hold_notice: HoldNotice {
                notice_id: "hold-notice:provider-linked".to_owned(),
                hold_status: HoldStatus::Cleared,
                hold_scope: HoldScope::LocalOnly,
                active_hold_refs: Vec::new(),
                notice_text:
                    "The platform only holds local linkage metadata; no remote hold applies."
                        .to_owned(),
                retention_owner_ref: "owner:local-user".to_owned(),
                local_only_artifact_note: Some(
                    "Only local linkage metadata was ever possessed.".to_owned(),
                ),
                blocks_destructive_action: false,
            },
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:provider-linked".to_owned(),
                scope: HoldScope::LocalOnly,
                selector_expression: "class=provider_linked_work_item_record".to_owned(),
                included_record_class_ids: vec![RecordClassId::ProviderLinkedWorkItemRecord],
                managed_copy_covered: false,
                local_copy_covered: true,
                note: Some("No managed copy exists to hold.".to_owned()),
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:provider-linked".to_owned(),
                retention_owner_ref: "owner:local-user".to_owned(),
                retention_label: RetentionLabel::LocalUserOwnedUntilCleared,
                retention_rule: "Local linkage metadata is user-owned until cleared.".to_owned(),
                delete_action: "local_delete_only".to_owned(),
                grace_rule: "Cleared immediately on user request.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:none".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:provider-linked".to_owned(),
                archive_state: ArchiveStateClass::ActiveLocal,
                archive_location_label: "On-device linkage index".to_owned(),
                managed_archive: false,
                local_only_archive_note: Some("Linkage metadata lives only on device.".to_owned()),
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::NotFound,
                reason: "No managed copy exists; only local linkage metadata is removed."
                    .to_owned(),
                confirmation_copy:
                    "Deleting clears local linkage metadata; the provider's record is untouched."
                        .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: Some(
                    "The provider-side item is outside this platform's control.".to_owned(),
                ),
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::OutsidePlatformScope,
                reason: "The provider record lives off-platform and cannot be exported here."
                    .to_owned(),
                confirmation_copy:
                    "Export covers local linkage metadata only; the provider record is off-platform."
                        .to_owned(),
                outside_platform_scope: true,
                local_only_boundary_note: Some(
                    "Provider-side content is outside platform scope.".to_owned(),
                ),
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-records-policy:provider-linked".to_owned(),
            rationale: "Provider linkage never implies remote export or remote delete.".to_owned(),
        },
        // Companion continuity packet — indeterminate hold fails closed.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:companion".to_owned(),
            title: "Companion continuity packet".to_owned(),
            artifact_family: GovernedArtifactFamily::CompanionContinuityPacket,
            record_class_id: RecordClassId::CompanionContinuityPacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            claims_managed_hold: true,
            claims_managed_export: true,
            claims_managed_delete: true,
            legal_hold_notice: HoldNotice {
                notice_id: "hold-notice:companion".to_owned(),
                hold_status: HoldStatus::UnknownIndeterminate,
                hold_scope: HoldScope::Both,
                active_hold_refs: vec!["hold:companion:pending-confirmation".to_owned()],
                notice_text:
                    "Hold status is indeterminate; destruction is blocked until it resolves."
                        .to_owned(),
                retention_owner_ref: "owner:org-admin".to_owned(),
                local_only_artifact_note: None,
                blocks_destructive_action: true,
            },
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:companion".to_owned(),
                scope: HoldScope::Both,
                selector_expression: "class=companion_continuity_packet".to_owned(),
                included_record_class_ids: vec![RecordClassId::CompanionContinuityPacket],
                managed_copy_covered: true,
                local_copy_covered: true,
                note: None,
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:companion".to_owned(),
                retention_owner_ref: "owner:org-admin".to_owned(),
                retention_label: RetentionLabel::ManagedPolicyRetained,
                retention_rule: "Held until the hold evaluation resolves.".to_owned(),
                delete_action: "hold_blocks_completion".to_owned(),
                grace_rule: "No destruction while hold is indeterminate.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:org-admin".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:companion".to_owned(),
                archive_state: ArchiveStateClass::ManagedArchive,
                archive_location_label: "Managed companion archive".to_owned(),
                managed_archive: true,
                local_only_archive_note: None,
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::BlockedByHold,
                reason: "Hold status is indeterminate; deletion is blocked fail-closed.".to_owned(),
                confirmation_copy:
                    "Delete is blocked: the legal-hold status is still being confirmed.".to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::Completed,
                reason: "Export is read-only and unaffected by the delete hold.".to_owned(),
                confirmation_copy: "Export produces a manifest; the hold does not block reads."
                    .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            exception_refs: vec!["m5-exception:companion-hold-review".to_owned()],
            proof_ref: "proof:m5-records-policy:companion".to_owned(),
            rationale: "Indeterminate holds block destruction fail-closed.".to_owned(),
        },
        // Incident support packet — export omitted by redaction; delete completes.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:incident".to_owned(),
            title: "Incident support packet".to_owned(),
            artifact_family: GovernedArtifactFamily::IncidentSupportPacket,
            record_class_id: RecordClassId::IncidentSupportPacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            claims_managed_hold: true,
            claims_managed_export: true,
            claims_managed_delete: true,
            legal_hold_notice: cleared_hold(
                "hold-notice:incident",
                HoldScope::Both,
                "owner:support-governance",
            ),
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:incident".to_owned(),
                scope: HoldScope::Both,
                selector_expression: "class=incident_support_packet".to_owned(),
                included_record_class_ids: vec![RecordClassId::IncidentSupportPacket],
                managed_copy_covered: true,
                local_copy_covered: true,
                note: None,
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:incident".to_owned(),
                retention_owner_ref: "owner:support-governance".to_owned(),
                retention_label: RetentionLabel::SupportCaseRetention,
                retention_rule: "Retained for the support-case window.".to_owned(),
                delete_action: "managed_delete_with_receipt".to_owned(),
                grace_rule: "Standard support-case grace window.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:support-governance".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:incident".to_owned(),
                archive_state: ArchiveStateClass::ManagedArchive,
                archive_location_label: "Managed incident archive".to_owned(),
                managed_archive: true,
                local_only_archive_note: None,
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::Completed,
                reason: "Incident packet is deleted and a destruction receipt is emitted."
                    .to_owned(),
                confirmation_copy: "Delete removes the incident packet and emits a receipt."
                    .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::OmittedByRedaction,
                reason: "Secret-bearing rows are omitted by the default redaction profile."
                    .to_owned(),
                confirmation_copy:
                    "Export omits secret-bearing rows under the default redaction profile."
                        .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-records-policy:incident".to_owned(),
            rationale: "Incident exports disclose redaction omissions explicitly.".to_owned(),
        },
        // Sync mirror ledger — active hold blocks delete; export needs local capture.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:sync-mirror".to_owned(),
            title: "Sync mirror ledger".to_owned(),
            artifact_family: GovernedArtifactFamily::SyncMirrorLedger,
            record_class_id: RecordClassId::SyncMirrorLedger,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::MixedLocalManaged,
            claims_managed_hold: true,
            claims_managed_export: false,
            claims_managed_delete: true,
            legal_hold_notice: HoldNotice {
                notice_id: "hold-notice:sync-mirror".to_owned(),
                hold_status: HoldStatus::Active,
                hold_scope: HoldScope::ManagedOnly,
                active_hold_refs: vec!["hold:sync-mirror:litigation-2026-04".to_owned()],
                notice_text: "An active legal hold blocks deletion of the managed mirror."
                    .to_owned(),
                retention_owner_ref: "owner:org-admin".to_owned(),
                local_only_artifact_note: Some(
                    "Per-device local snapshots are outside the managed hold.".to_owned(),
                ),
                blocks_destructive_action: true,
            },
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:sync-mirror".to_owned(),
                scope: HoldScope::ManagedOnly,
                selector_expression: "class=sync_mirror_ledger AND copy=managed".to_owned(),
                included_record_class_ids: vec![RecordClassId::SyncMirrorLedger],
                managed_copy_covered: true,
                local_copy_covered: false,
                note: Some("Local per-device snapshots are not covered by the hold.".to_owned()),
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:sync-mirror".to_owned(),
                retention_owner_ref: "owner:org-admin".to_owned(),
                retention_label: RetentionLabel::ManagedPolicyRetained,
                retention_rule: "Managed mirror retained under active hold.".to_owned(),
                delete_action: "hold_blocks_completion".to_owned(),
                grace_rule: "No deletion while the hold is active.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:org-admin".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:sync-mirror".to_owned(),
                archive_state: ArchiveStateClass::ManagedArchive,
                archive_location_label: "Managed sync mirror".to_owned(),
                managed_archive: true,
                local_only_archive_note: Some(
                    "Local per-device snapshots remain on each device.".to_owned(),
                ),
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::BlockedByHold,
                reason: "An active legal hold blocks deletion of the managed mirror.".to_owned(),
                confirmation_copy:
                    "Delete is blocked by an active legal hold on the managed mirror.".to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: Some(
                    "Local snapshots can be cleared on device independently.".to_owned(),
                ),
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::ManualLocalCaptureRequired,
                reason: "Per-device local snapshots must be captured locally to export.".to_owned(),
                confirmation_copy:
                    "Export of local snapshots requires a manual on-device capture step.".to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: Some(
                    "Local snapshots are not collected by managed export.".to_owned(),
                ),
            },
            exception_refs: vec!["m5-exception:sync-mirror-hold".to_owned()],
            proof_ref: "proof:m5-records-policy:sync-mirror".to_owned(),
            rationale: "Active holds and local snapshot boundaries stay visible before delete."
                .to_owned(),
        },
        // Browser handoff manifest — local-only; export is outside platform scope.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:browser-handoff".to_owned(),
            title: "Browser handoff manifest".to_owned(),
            artifact_family: GovernedArtifactFamily::BrowserHandoffManifest,
            record_class_id: RecordClassId::BrowserHandoffManifest,
            authority_boundary: AuthorityBoundaryClass::LocalOnly,
            local_truth_authority: LocalVsManagedCopy::LocalCacheOnly,
            claims_managed_hold: false,
            claims_managed_export: false,
            claims_managed_delete: false,
            legal_hold_notice: HoldNotice {
                notice_id: "hold-notice:browser-handoff".to_owned(),
                hold_status: HoldStatus::Cleared,
                hold_scope: HoldScope::LocalOnly,
                active_hold_refs: Vec::new(),
                notice_text: "Handoff manifests are local-only and carry no remote hold."
                    .to_owned(),
                retention_owner_ref: "owner:local-user".to_owned(),
                local_only_artifact_note: Some(
                    "The manifest is created and stored only on device.".to_owned(),
                ),
                blocks_destructive_action: false,
            },
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:browser-handoff".to_owned(),
                scope: HoldScope::LocalOnly,
                selector_expression: "class=browser_handoff_manifest".to_owned(),
                included_record_class_ids: vec![RecordClassId::BrowserHandoffManifest],
                managed_copy_covered: false,
                local_copy_covered: true,
                note: Some("No managed copy exists.".to_owned()),
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:browser-handoff".to_owned(),
                retention_owner_ref: "owner:local-user".to_owned(),
                retention_label: RetentionLabel::LocalUserOwnedUntilExportOrDelete,
                retention_rule: "User-owned until exported or deleted on device.".to_owned(),
                delete_action: "local_delete_only".to_owned(),
                grace_rule: "Cleared immediately on user request.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:none".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:browser-handoff".to_owned(),
                archive_state: ArchiveStateClass::ActiveLocal,
                archive_location_label: "On-device handoff store".to_owned(),
                managed_archive: false,
                local_only_archive_note: Some("Handoff manifests live only on device.".to_owned()),
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::Completed,
                reason: "The local manifest is deleted on device.".to_owned(),
                confirmation_copy: "Delete removes the local handoff manifest from this device."
                    .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: Some("Nothing is held or stored remotely.".to_owned()),
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::OutsidePlatformScope,
                reason: "Handoff to the system browser is outside this platform's export scope."
                    .to_owned(),
                confirmation_copy:
                    "The handoff target is the system browser, outside platform export scope."
                        .to_owned(),
                outside_platform_scope: true,
                local_only_boundary_note: Some(
                    "The receiving browser is outside platform scope.".to_owned(),
                ),
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-records-policy:browser-handoff".to_owned(),
            rationale: "Local-only handoff never implies remote hold or remote export.".to_owned(),
        },
        // Offboarding record — delete is policy-retained at the retention floor.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:offboarding".to_owned(),
            title: "Offboarding exit packet".to_owned(),
            artifact_family: GovernedArtifactFamily::OffboardingRecord,
            record_class_id: RecordClassId::OffboardingExitPacket,
            authority_boundary: AuthorityBoundaryClass::ManagedOnly,
            local_truth_authority: LocalVsManagedCopy::ManagedAuthoritative,
            claims_managed_hold: true,
            claims_managed_export: true,
            claims_managed_delete: true,
            legal_hold_notice: cleared_hold(
                "hold-notice:offboarding",
                HoldScope::ManagedOnly,
                "owner:org-admin",
            ),
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:offboarding".to_owned(),
                scope: HoldScope::ManagedOnly,
                selector_expression: "class=offboarding_exit_packet".to_owned(),
                included_record_class_ids: vec![RecordClassId::OffboardingExitPacket],
                managed_copy_covered: true,
                local_copy_covered: false,
                note: None,
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:offboarding".to_owned(),
                retention_owner_ref: "owner:org-admin".to_owned(),
                retention_label: RetentionLabel::ManagedPolicyRetained,
                retention_rule: "Retained until the retention-floor horizon elapses.".to_owned(),
                delete_action: "managed_delete_with_receipt".to_owned(),
                grace_rule: "Retention floor blocks early destruction.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:org-admin".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:offboarding".to_owned(),
                archive_state: ArchiveStateClass::ManagedArchive,
                archive_location_label: "Managed offboarding archive".to_owned(),
                managed_archive: true,
                local_only_archive_note: Some(
                    "Downloaded local exports stay under user control.".to_owned(),
                ),
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::PolicyRetained,
                reason: "The retention-floor horizon blocks destruction until it elapses."
                    .to_owned(),
                confirmation_copy:
                    "Delete is deferred: the offboarding record is policy-retained until its floor."
                        .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: Some(
                    "Local downloaded exports remain user-controlled.".to_owned(),
                ),
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::Partial,
                reason: "Some managed classes are omitted; an explicit partial manifest is emitted."
                    .to_owned(),
                confirmation_copy: "Export is partial; omitted classes are listed in the manifest."
                    .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            exception_refs: vec!["m5-exception:offboarding-retention-floor".to_owned()],
            proof_ref: "proof:m5-records-policy:offboarding".to_owned(),
            rationale: "Offboarding deletes cite the retention floor before destruction."
                .to_owned(),
        },
        // Support export packet — the artifact is itself an export.
        M5RecordsPolicyRow {
            entry_id: "m5-records-policy:support-export".to_owned(),
            title: "Support export packet".to_owned(),
            artifact_family: GovernedArtifactFamily::SupportExportPacket,
            record_class_id: RecordClassId::SupportExportPacket,
            authority_boundary: AuthorityBoundaryClass::LocalAndManaged,
            local_truth_authority: LocalVsManagedCopy::GeneratedPacketAuthoritative,
            claims_managed_hold: false,
            claims_managed_export: true,
            claims_managed_delete: true,
            legal_hold_notice: cleared_hold(
                "hold-notice:support-export",
                HoldScope::Both,
                "owner:support-governance",
            ),
            hold_selector_scope: HoldSelectorScope {
                selector_id: "hold-selector:support-export".to_owned(),
                scope: HoldScope::Both,
                selector_expression: "class=support_export_packet".to_owned(),
                included_record_class_ids: vec![RecordClassId::SupportExportPacket],
                managed_copy_covered: true,
                local_copy_covered: true,
                note: None,
            },
            retention_inspector: RetentionInspector {
                inspector_id: "retention:support-export".to_owned(),
                retention_owner_ref: "owner:support-governance".to_owned(),
                retention_label: RetentionLabel::GeneratedPacketDeliveryWindow,
                retention_rule: "Retained for the support delivery window.".to_owned(),
                delete_action: "managed_delete_with_receipt".to_owned(),
                grace_rule: "Cleared after the delivery window.".to_owned(),
                policy_version: "policy:m5-records:v1".to_owned(),
                local_owner_ref: "owner:local-user".to_owned(),
                managed_owner_ref: "owner:support-governance".to_owned(),
            },
            archive_inspector: ArchiveInspector {
                inspector_id: "archive:support-export".to_owned(),
                archive_state: ArchiveStateClass::ArchivedLocal,
                archive_location_label: "Local support export archive".to_owned(),
                managed_archive: false,
                local_only_archive_note: Some(
                    "The generated packet is held locally until delivered.".to_owned(),
                ),
            },
            pre_delete_truth: PreActionTruth {
                action: DestructiveActionClass::Delete,
                projected_outcome: RecordOperationOutcome::Completed,
                reason: "The generated support packet is deleted with a receipt.".to_owned(),
                confirmation_copy: "Delete removes the generated support packet and emits a receipt."
                    .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            pre_export_truth: PreActionTruth {
                action: DestructiveActionClass::Export,
                projected_outcome: RecordOperationOutcome::Completed,
                reason: "The packet is itself the export and includes a manifest.".to_owned(),
                confirmation_copy: "Export delivers the support packet with its manifest."
                    .to_owned(),
                outside_platform_scope: false,
                local_only_boundary_note: None,
            },
            exception_refs: Vec::new(),
            proof_ref: "proof:m5-records-policy:support-export".to_owned(),
            rationale: "Support exports keep delete/export honesty even for generated packets."
                .to_owned(),
        },
    ];

    M5RecordsPolicyPacket {
        schema_version: M5_RECORDS_POLICY_SCHEMA_VERSION,
        record_kind: M5_RECORDS_POLICY_RECORD_KIND.to_owned(),
        packet_id: "m5-records-policy:hold-retention:0001".to_owned(),
        shared_contract_ref: M5_RECORDS_POLICY_SHARED_CONTRACT_REF.to_owned(),
        as_of: "2026-06-13T16:00:00Z".to_owned(),
        overview_doc_ref: M5_RECORDS_POLICY_DOC_REF.to_owned(),
        artifact_summary_ref: M5_RECORDS_POLICY_ARTIFACT_REF.to_owned(),
        exception_expiry_contract_ref: "policy:m5_exception_expiry_truth:v1".to_owned(),
        rows,
        summary: "Canonical legal-hold notices, hold selector scopes, retention/archive \
                  inspectors, and pre-action delete/export truth for the M5 governed artifact \
                  families."
            .to_owned(),
    }
}
