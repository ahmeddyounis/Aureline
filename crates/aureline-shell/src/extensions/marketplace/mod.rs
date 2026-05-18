//! Shell marketplace row projection for trust, support, and compatibility truth.
//!
//! The extension registry produces catalog descriptors, and the compatibility
//! lane produces the generated report. This module is the first shell consumer
//! that renders those facts together on discovery rows before the user opens
//! install review. The projection is deliberately read-only: install and update
//! authority stays with the native review records.

use serde::{Deserialize, Serialize};

use aureline_extensions::{
    current_extension_bridge_matrix, evaluate_catalog_descriptor, evaluate_install_review_alpha,
    project_marketplace_fact_grid, project_marketplace_fact_grid_support_export,
    project_marketplace_truth_row, project_marketplace_truth_support_export,
    validate_extension_bridge_matrix, validate_marketplace_fact_grid,
    validate_marketplace_fact_grid_support_export, validate_marketplace_truth_row,
    validate_marketplace_truth_support_export, ActivationBudgetDisclosure, CatalogDescriptorInput,
    CatalogDescriptorRecord, CatalogLifecycleStateClass, CatalogMirrorabilityClass,
    CatalogModerationStateClass, CatalogRegistrySourceClass, CatalogRevocationSnapshotAgeClass,
    CatalogTrustBadgeInheritanceRuleClass, ClientScopeClass, CompatibilityLabelBlock,
    CompatibilityReportSnapshot, EffectivePermissionBaselineRecord, ExtensionBridgeMatrix,
    ExtensionReviewAlphaPacketRecord, InstallReviewAlphaEvaluation, InstallReviewAlphaInput,
    InstallReviewAlphaPacketRecord, InstallReviewBoundaryTruth, InstallReviewContentSourceClass,
    LockfileImpact, LockfileImpactClass, ManifestChangeClass, ManifestChangeRow,
    ManifestOriginSourceClass, MarketplaceFactGridFinding, MarketplaceFactGridInput,
    MarketplaceFactGridRecord, MarketplaceFactGridSupportExportRecord,
    MarketplaceFactGridSurfaceClass, MarketplaceTruthBadgeClass, MarketplaceTruthFinding,
    MarketplaceTruthRowInput, MarketplaceTruthRowRecord, MarketplaceTruthSupportExportRecord,
    RevocationStateClass, ScriptRiskClass, ScriptRiskDisclosure,
};
use aureline_install::InstallTopologyAlphaPacket;

#[cfg(test)]
mod tests;

/// Stable record kind for [`MarketplaceTruthPageRecord`] payloads.
pub const MARKETPLACE_TRUTH_PAGE_RECORD_KIND: &str = "shell_marketplace_truth_page_record";

/// Schema version for shell marketplace truth pages.
pub const MARKETPLACE_TRUTH_PAGE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by marketplace row, CLI, and support exports.
pub const MARKETPLACE_TRUTH_SHARED_CONTRACT_REF: &str = "shell:marketplace_truth_beta:v1";

/// Page-level projection for claimed marketplace beta rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceTruthPageRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this page.
    pub schema_version: u32,
    /// Shared contract ref consumed by all row surfaces.
    pub shared_contract_ref: String,
    /// Extension bridge matrix consumed by the marketplace page.
    pub extension_bridge_matrix_id: String,
    /// Extension bridge matrix report ref consumed by the marketplace page.
    pub extension_compatibility_report_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Page label for headless and support consumers.
    pub page_label: String,
    /// Controlled badge vocabulary the shell may render.
    pub controlled_badge_vocabulary: Vec<MarketplaceTruthBadgeClass>,
    /// Page summary counts.
    pub summary: MarketplaceTruthPageSummary,
    /// Marketplace rows shown before install review.
    pub rows: Vec<MarketplaceTruthRowRecord>,
    /// Support-export rows paired to the marketplace rows.
    pub support_rows: Vec<MarketplaceTruthSupportExportRecord>,
    /// Shared fact-grid rows paired to marketplace rows and native review.
    pub fact_grids: Vec<MarketplaceFactGridRecord>,
    /// Support-export fact-grid rows paired to the shared fact grids.
    pub fact_grid_support_rows: Vec<MarketplaceFactGridSupportExportRecord>,
}

/// Summary counts for a marketplace truth page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceTruthPageSummary {
    /// Number of marketplace rows.
    pub row_count: usize,
    /// Number of rows carrying a limited badge.
    pub limited_row_count: usize,
    /// Number of rows carrying a revoked badge.
    pub revoked_row_count: usize,
    /// Number of rows carrying a mirrored badge.
    pub mirrored_row_count: usize,
    /// Number of rows carrying a retest-pending badge.
    pub retest_pending_row_count: usize,
    /// Number of rows blocked before install or update.
    pub blocked_install_or_update_count: usize,
}

impl MarketplaceTruthPageSummary {
    fn from_rows(rows: &[MarketplaceTruthRowRecord]) -> Self {
        Self {
            row_count: rows.len(),
            limited_row_count: rows_with_badge(rows, MarketplaceTruthBadgeClass::Limited),
            revoked_row_count: rows_with_badge(rows, MarketplaceTruthBadgeClass::Revoked),
            mirrored_row_count: rows_with_badge(rows, MarketplaceTruthBadgeClass::Mirrored),
            retest_pending_row_count: rows_with_badge(
                rows,
                MarketplaceTruthBadgeClass::RetestPending,
            ),
            blocked_install_or_update_count: rows
                .iter()
                .filter(|row| row.blocks_install_or_update)
                .count(),
        }
    }
}

/// Validation error for shell marketplace truth pages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketplaceTruthPageValidationError {
    /// Page-level record kind is not the shell marketplace page kind.
    PageRecordKindWrong,
    /// Page-level schema version is not current.
    PageSchemaVersionWrong,
    /// Page-level shared contract ref is not current.
    SharedContractWrong,
    /// Extension bridge matrix did not validate.
    BridgeMatrixFinding {
        /// Validation check id.
        check_id: String,
        /// Validation message.
        message: String,
    },
    /// Required badge vocabulary is missing from the page declaration.
    ControlledBadgeMissing {
        /// Missing controlled badge class.
        badge_class: MarketplaceTruthBadgeClass,
    },
    /// A row failed the lower-level marketplace row validator.
    RowFinding {
        /// Row id that failed validation.
        row_id: String,
        /// Validation check id.
        check_id: String,
        /// Validation message.
        message: String,
    },
    /// A row has no paired support export.
    SupportExportMissing {
        /// Row id missing a support export.
        row_id: String,
    },
    /// A support export failed the lower-level validator.
    SupportExportFinding {
        /// Support export id that failed validation.
        export_id: String,
        /// Validation check id.
        check_id: String,
        /// Validation message.
        message: String,
    },
    /// A support export drifted from its source row.
    SupportExportParityDrift {
        /// Row id whose support export drifted.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// A row has no paired shared fact grid.
    FactGridMissing {
        /// Row id missing a fact grid.
        row_id: String,
    },
    /// A shared fact grid failed the lower-level validator.
    FactGridFinding {
        /// Fact-grid id that failed validation.
        fact_grid_id: String,
        /// Validation check id.
        check_id: String,
        /// Validation message.
        message: String,
    },
    /// A shared fact grid drifted from its source row.
    FactGridParityDrift {
        /// Fact-grid id whose source parity drifted.
        fact_grid_id: String,
        /// Field that drifted.
        field: String,
    },
    /// A shared fact grid has no paired support export.
    FactGridSupportExportMissing {
        /// Fact-grid id missing a support export.
        fact_grid_id: String,
    },
    /// A fact-grid support export failed the lower-level validator.
    FactGridSupportExportFinding {
        /// Support export id that failed validation.
        export_id: String,
        /// Validation check id.
        check_id: String,
        /// Validation message.
        message: String,
    },
    /// A fact-grid support export drifted from its source grid.
    FactGridSupportExportParityDrift {
        /// Fact-grid id whose support export drifted.
        fact_grid_id: String,
        /// Field that drifted.
        field: String,
    },
    /// Summary count does not match row contents.
    SummaryDrift {
        /// Field that drifted.
        field: String,
    },
}

impl std::fmt::Display for MarketplaceTruthPageValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PageRecordKindWrong => write!(f, "marketplace truth page record kind is wrong"),
            Self::PageSchemaVersionWrong => {
                write!(f, "marketplace truth page schema version is wrong")
            }
            Self::SharedContractWrong => write!(f, "marketplace truth shared contract is wrong"),
            Self::BridgeMatrixFinding { check_id, message } => {
                write!(f, "extension bridge matrix failed {check_id}: {message}")
            }
            Self::ControlledBadgeMissing { badge_class } => {
                write!(
                    f,
                    "controlled badge {:?} is missing from the page",
                    badge_class
                )
            }
            Self::RowFinding {
                row_id,
                check_id,
                message,
            } => write!(f, "row {row_id} failed {check_id}: {message}"),
            Self::SupportExportMissing { row_id } => {
                write!(f, "row {row_id} has no paired support export")
            }
            Self::SupportExportFinding {
                export_id,
                check_id,
                message,
            } => write!(f, "support export {export_id} failed {check_id}: {message}"),
            Self::SupportExportParityDrift { row_id, field } => {
                write!(f, "support export for {row_id} drifted on {field}")
            }
            Self::FactGridMissing { row_id } => {
                write!(f, "row {row_id} has no paired marketplace fact grid")
            }
            Self::FactGridFinding {
                fact_grid_id,
                check_id,
                message,
            } => write!(f, "fact grid {fact_grid_id} failed {check_id}: {message}"),
            Self::FactGridParityDrift {
                fact_grid_id,
                field,
            } => write!(f, "fact grid {fact_grid_id} drifted on {field}"),
            Self::FactGridSupportExportMissing { fact_grid_id } => {
                write!(f, "fact grid {fact_grid_id} has no paired support export")
            }
            Self::FactGridSupportExportFinding {
                export_id,
                check_id,
                message,
            } => write!(
                f,
                "fact-grid support export {export_id} failed {check_id}: {message}"
            ),
            Self::FactGridSupportExportParityDrift {
                fact_grid_id,
                field,
            } => write!(
                f,
                "fact-grid support export for {fact_grid_id} drifted on {field}"
            ),
            Self::SummaryDrift { field } => {
                write!(f, "marketplace truth summary drifted on {field}")
            }
        }
    }
}

impl std::error::Error for MarketplaceTruthPageValidationError {}

/// Builds the seeded marketplace truth page used by the shell and headless inspector.
pub fn seeded_marketplace_truth_page() -> MarketplaceTruthPageRecord {
    let report = compatibility_report();
    let bridge_matrix = bridge_matrix();
    let specs = seeded_row_specs();
    let rows: Vec<MarketplaceTruthRowRecord> = specs
        .iter()
        .map(|spec| {
            project_seeded_row(
                spec.row_suffix,
                &spec.catalog,
                &report,
                &bridge_matrix,
                spec.bridge_matrix_row_ref,
                spec.install_review_ref,
            )
        })
        .collect();
    let support_rows = rows
        .iter()
        .map(|row| {
            project_marketplace_truth_support_export(
                row,
                &format!("marketplace_truth_support_export:{}", row.row_id),
            )
        })
        .collect();
    let install_reviews: Vec<InstallReviewAlphaPacketRecord> = rows
        .iter()
        .zip(specs.iter())
        .map(|(row, spec)| seeded_install_review_packet(row, &spec.catalog))
        .collect();
    let fact_grids: Vec<MarketplaceFactGridRecord> = rows
        .iter()
        .zip(specs.iter())
        .zip(install_reviews.iter())
        .map(|((row, spec), install_review)| project_seeded_fact_grid(row, spec, install_review))
        .collect();
    let fact_grid_support_rows = fact_grids
        .iter()
        .map(|grid| {
            project_marketplace_fact_grid_support_export(
                grid,
                &format!("marketplace_fact_grid_support_export:{}", grid.fact_grid_id),
            )
        })
        .collect();

    MarketplaceTruthPageRecord {
        record_kind: MARKETPLACE_TRUTH_PAGE_RECORD_KIND.to_string(),
        schema_version: MARKETPLACE_TRUTH_PAGE_SCHEMA_VERSION,
        shared_contract_ref: MARKETPLACE_TRUTH_SHARED_CONTRACT_REF.to_string(),
        extension_bridge_matrix_id: bridge_matrix.matrix_id.clone(),
        extension_compatibility_report_ref: bridge_matrix.extension_report_ref.clone(),
        page_id: "shell:marketplace-truth:beta:page:default".to_string(),
        page_label:
            "Marketplace truth rows: lifecycle, compatibility, support, trust, mirrorability"
                .to_string(),
        controlled_badge_vocabulary: MarketplaceTruthBadgeClass::required_acceptance_states()
            .to_vec(),
        summary: MarketplaceTruthPageSummary::from_rows(&rows),
        rows,
        support_rows,
        fact_grids,
        fact_grid_support_rows,
    }
}

/// Validates the shell marketplace truth page.
pub fn validate_marketplace_truth_page(
    page: &MarketplaceTruthPageRecord,
) -> Result<(), Vec<MarketplaceTruthPageValidationError>> {
    let mut errors = Vec::new();

    if page.record_kind != MARKETPLACE_TRUTH_PAGE_RECORD_KIND {
        errors.push(MarketplaceTruthPageValidationError::PageRecordKindWrong);
    }
    if page.schema_version != MARKETPLACE_TRUTH_PAGE_SCHEMA_VERSION {
        errors.push(MarketplaceTruthPageValidationError::PageSchemaVersionWrong);
    }
    if page.shared_contract_ref != MARKETPLACE_TRUTH_SHARED_CONTRACT_REF {
        errors.push(MarketplaceTruthPageValidationError::SharedContractWrong);
    }
    let bridge_matrix = bridge_matrix();
    for finding in validate_extension_bridge_matrix(&bridge_matrix) {
        errors.push(MarketplaceTruthPageValidationError::BridgeMatrixFinding {
            check_id: finding.check_id,
            message: finding.message,
        });
    }
    if page.extension_bridge_matrix_id != bridge_matrix.matrix_id {
        errors.push(MarketplaceTruthPageValidationError::BridgeMatrixFinding {
            check_id: "marketplace_truth_page.bridge_matrix_id_drift".to_string(),
            message: "page bridge matrix id does not match checked matrix".to_string(),
        });
    }
    if page.extension_compatibility_report_ref != bridge_matrix.extension_report_ref {
        errors.push(MarketplaceTruthPageValidationError::BridgeMatrixFinding {
            check_id: "marketplace_truth_page.extension_report_ref_drift".to_string(),
            message: "page extension report ref does not match checked matrix".to_string(),
        });
    }
    for badge_class in MarketplaceTruthBadgeClass::required_acceptance_states() {
        if !page.controlled_badge_vocabulary.contains(&badge_class) {
            errors
                .push(MarketplaceTruthPageValidationError::ControlledBadgeMissing { badge_class });
        }
    }

    for row in &page.rows {
        for finding in validate_marketplace_truth_row(row) {
            errors.push(row_finding(row, finding));
        }
        let Some(export) = page
            .support_rows
            .iter()
            .find(|support_row| support_row.row_ref == row.row_id)
        else {
            errors.push(MarketplaceTruthPageValidationError::SupportExportMissing {
                row_id: row.row_id.clone(),
            });
            continue;
        };
        for finding in validate_marketplace_truth_support_export(export) {
            errors.push(MarketplaceTruthPageValidationError::SupportExportFinding {
                export_id: export.export_id.clone(),
                check_id: finding.check_id,
                message: finding.message,
            });
        }
        if export.compatibility_label_class != row.compatibility_label_class {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "compatibility_label_class".to_string(),
                },
            );
        }
        if export.lifecycle_badges != row.lifecycle_badges {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "lifecycle_badges".to_string(),
                },
            );
        }
        if export.support_chips != row.support_chips {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "support_chips".to_string(),
                },
            );
        }
        if export.trust_chips != row.trust_chips {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "trust_chips".to_string(),
                },
            );
        }
        if export.extension_bridge_matrix_row_id != row.extension_bridge_matrix_row_id {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "extension_bridge_matrix_row_id".to_string(),
                },
            );
        }
        if export.extension_bridge_matrix_id != row.extension_bridge_matrix_id {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "extension_bridge_matrix_id".to_string(),
                },
            );
        }
        if export.extension_bridge_state_class != row.extension_bridge_state_class {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "extension_bridge_state_class".to_string(),
                },
            );
        }
        if export.bridge_known_limits != row.bridge_known_limits {
            errors.push(
                MarketplaceTruthPageValidationError::SupportExportParityDrift {
                    row_id: row.row_id.clone(),
                    field: "bridge_known_limits".to_string(),
                },
            );
        }

        let Some(fact_grid) = page
            .fact_grids
            .iter()
            .find(|grid| grid.marketplace_row_ref == row.row_id)
        else {
            errors.push(MarketplaceTruthPageValidationError::FactGridMissing {
                row_id: row.row_id.clone(),
            });
            continue;
        };
        for finding in validate_marketplace_fact_grid(fact_grid) {
            errors.push(fact_grid_finding(fact_grid, finding));
        }
        if fact_grid.compatibility_label_class != row.compatibility_label_class {
            errors.push(MarketplaceTruthPageValidationError::FactGridParityDrift {
                fact_grid_id: fact_grid.fact_grid_id.clone(),
                field: "compatibility_label_class".to_string(),
            });
        }
        if fact_grid.registry_source_class != row.registry_source_class {
            errors.push(MarketplaceTruthPageValidationError::FactGridParityDrift {
                fact_grid_id: fact_grid.fact_grid_id.clone(),
                field: "registry_source_class".to_string(),
            });
        }
        let Some(fact_grid_export) = page
            .fact_grid_support_rows
            .iter()
            .find(|export| export.fact_grid_ref == fact_grid.fact_grid_id)
        else {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportMissing {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                },
            );
            continue;
        };
        for finding in validate_marketplace_fact_grid_support_export(fact_grid_export) {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportFinding {
                    export_id: fact_grid_export.export_id.clone(),
                    check_id: finding.check_id,
                    message: finding.message,
                },
            );
        }
        if fact_grid_export.client_scope_class != fact_grid.client_scope_class {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "client_scope_class".to_string(),
                },
            );
        }
        if fact_grid_export.registry_source_class != fact_grid.registry_source_class {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "registry_source_class".to_string(),
                },
            );
        }
        if fact_grid_export.compatibility_label_class != fact_grid.compatibility_label_class {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "compatibility_label_class".to_string(),
                },
            );
        }
        if fact_grid_export.script_risk_class != fact_grid.script_risk.script_risk_class {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "script_risk_class".to_string(),
                },
            );
        }
        if fact_grid_export.lockfile_impact_class != fact_grid.lockfile_impact.impact_class {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "lockfile_impact_class".to_string(),
                },
            );
        }
        if fact_grid_export.permission_delta_count != fact_grid.permission_delta_count {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "permission_delta_count".to_string(),
                },
            );
        }
        if fact_grid_export.blocks_install_or_update != fact_grid.blocks_install_or_update {
            errors.push(
                MarketplaceTruthPageValidationError::FactGridSupportExportParityDrift {
                    fact_grid_id: fact_grid.fact_grid_id.clone(),
                    field: "blocks_install_or_update".to_string(),
                },
            );
        }
    }

    let expected_summary = MarketplaceTruthPageSummary::from_rows(&page.rows);
    if page.summary != expected_summary {
        errors.push(MarketplaceTruthPageValidationError::SummaryDrift {
            field: "summary".to_string(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn row_finding(
    row: &MarketplaceTruthRowRecord,
    finding: MarketplaceTruthFinding,
) -> MarketplaceTruthPageValidationError {
    MarketplaceTruthPageValidationError::RowFinding {
        row_id: row.row_id.clone(),
        check_id: finding.check_id,
        message: finding.message,
    }
}

fn fact_grid_finding(
    fact_grid: &MarketplaceFactGridRecord,
    finding: MarketplaceFactGridFinding,
) -> MarketplaceTruthPageValidationError {
    MarketplaceTruthPageValidationError::FactGridFinding {
        fact_grid_id: fact_grid.fact_grid_id.clone(),
        check_id: finding.check_id,
        message: finding.message,
    }
}

fn rows_with_badge(rows: &[MarketplaceTruthRowRecord], badge: MarketplaceTruthBadgeClass) -> usize {
    rows.iter()
        .filter(|row| row.lifecycle_badges.contains(&badge))
        .count()
}

fn project_seeded_row(
    row_suffix: &str,
    catalog: &CatalogDescriptorRecord,
    report: &CompatibilityReportSnapshot,
    bridge_matrix: &ExtensionBridgeMatrix,
    bridge_matrix_row_ref: &str,
    install_review_ref: &str,
) -> MarketplaceTruthRowRecord {
    project_marketplace_truth_row(MarketplaceTruthRowInput {
        row_id: &format!("marketplace_truth_row:dev.aureline.samples/wasm-notes:{row_suffix}"),
        catalog,
        compatibility_report: report,
        compatibility_report_row_ref: "compat_row:extension_host.sdk_wit_permission_window",
        extension_bridge_matrix: bridge_matrix,
        extension_bridge_matrix_row_ref: bridge_matrix_row_ref,
        install_review_ref,
        generated_at: "2026-05-16T19:30:00Z",
    })
    .expect("seeded marketplace truth row must project")
}

fn project_seeded_fact_grid(
    row: &MarketplaceTruthRowRecord,
    spec: &SeededMarketplaceRowSpec,
    install_review: &InstallReviewAlphaPacketRecord,
) -> MarketplaceFactGridRecord {
    project_marketplace_fact_grid(MarketplaceFactGridInput {
        fact_grid_id: &format!(
            "marketplace_fact_grid:dev.aureline.samples/wasm-notes:{}",
            spec.row_suffix
        ),
        surface_class: spec.surface_class,
        marketplace_row: row,
        catalog: &spec.catalog,
        install_review,
        client_scope_class: spec.client_scope_class,
        client_scope_summary: spec.client_scope_class.label(),
        script_risk: script_risk_for(spec),
        manifest_changes: manifest_changes_for(row, &spec.catalog),
        lockfile_impact: lockfile_impact_for(spec),
        generated_at: "2026-05-16T19:35:00Z",
    })
    .expect("seeded marketplace fact grid must project")
}

fn seeded_row_specs() -> Vec<SeededMarketplaceRowSpec> {
    vec![
        SeededMarketplaceRowSpec {
            row_suffix: "public-beta",
            catalog: catalog_record(),
            bridge_matrix_row_ref: "extension_bridge_row:wasm_component_native_beta",
            install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
            surface_class: MarketplaceFactGridSurfaceClass::ResultRow,
            client_scope_class: ClientScopeClass::DesktopPlusBrowserCompanion,
            script_risk_class: ScriptRiskClass::NoScriptsOrNativeBuild,
        },
        SeededMarketplaceRowSpec {
            row_suffix: "mirror-preview",
            catalog: evaluated_catalog_fixture("staged_pending_moderation"),
            bridge_matrix_row_ref: "extension_bridge_row:wasm_component_native_beta",
            install_review_ref:
                "install_review_alpha:dev.aureline.samples/wasm-notes:mirror-staged",
            surface_class: MarketplaceFactGridSurfaceClass::ResultRow,
            client_scope_class: ClientScopeClass::Desktop,
            script_risk_class: ScriptRiskClass::LifecycleScriptsDeclared,
        },
        SeededMarketplaceRowSpec {
            row_suffix: "mirror-retest-pending",
            catalog: evaluated_catalog_fixture("limited_compatibility_catalog"),
            bridge_matrix_row_ref: "extension_bridge_row:vscode_api_bridge_beta",
            install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:limited",
            surface_class: MarketplaceFactGridSurfaceClass::ResultRow,
            client_scope_class: ClientScopeClass::BrowserCompanion,
            script_risk_class: ScriptRiskClass::NativeBuildRequired,
        },
        SeededMarketplaceRowSpec {
            row_suffix: "offline-bundle",
            catalog: catalog_source_variant(
                "offline-bundle",
                CatalogRegistrySourceClass::OfflineBundle,
                CatalogMirrorabilityClass::MirrorableCappedTrust,
                CatalogTrustBadgeInheritanceRuleClass::CappedAtCommunityOnApprovedMirror,
                CatalogRevocationSnapshotAgeClass::WarmCached,
                "offline_bundle:enterprise-airgap:dev.aureline.samples/wasm-notes",
            ),
            bridge_matrix_row_ref: "extension_bridge_row:wasm_component_native_beta",
            install_review_ref:
                "install_review_alpha:dev.aureline.samples/wasm-notes:offline-bundle",
            surface_class: MarketplaceFactGridSurfaceClass::OfflineRegistryRow,
            client_scope_class: ClientScopeClass::Desktop,
            script_risk_class: ScriptRiskClass::LifecycleScriptsDeclared,
        },
        SeededMarketplaceRowSpec {
            row_suffix: "manual-import",
            catalog: catalog_source_variant(
                "manual-import",
                CatalogRegistrySourceClass::LocalArchive,
                CatalogMirrorabilityClass::MirrorableCappedTrust,
                CatalogTrustBadgeInheritanceRuleClass::CappedAtUnverifiedOnLocalArchive,
                CatalogRevocationSnapshotAgeClass::DegradedCached,
                "local_archive:downloads:dev.aureline.samples/wasm-notes",
            ),
            bridge_matrix_row_ref: "extension_bridge_row:wasm_component_native_beta",
            install_review_ref:
                "install_review_alpha:dev.aureline.samples/wasm-notes:manual-import",
            surface_class: MarketplaceFactGridSurfaceClass::ManualImportReview,
            client_scope_class: ClientScopeClass::Desktop,
            script_risk_class: ScriptRiskClass::UnknownScriptRiskBlocked,
        },
        SeededMarketplaceRowSpec {
            row_suffix: "revoked",
            catalog: evaluated_catalog_fixture("revoked_catalog_refused"),
            bridge_matrix_row_ref: "extension_bridge_row:unsupported_webview_runtime",
            install_review_ref: "install_review_alpha:dev.aureline.samples/wasm-notes:revoked",
            surface_class: MarketplaceFactGridSurfaceClass::ResultRow,
            client_scope_class: ClientScopeClass::HeadlessOnly,
            script_risk_class: ScriptRiskClass::ExternalHelperOrHost,
        },
    ]
}

fn seeded_install_review_packet(
    row: &MarketplaceTruthRowRecord,
    catalog: &CatalogDescriptorRecord,
) -> InstallReviewAlphaPacketRecord {
    let mut fixture = install_review_fixture();
    fixture.input.review_id = row.install_review_ref.clone();
    fixture.input.subject_ref = row.extension_identity.clone();
    fixture.boundary_truth.content_source_class =
        content_source_for_registry(catalog.lifecycle.source_registry_class);
    fixture.boundary_truth.manifest_origin_source_class =
        manifest_origin_for_registry(catalog.lifecycle.source_registry_class);
    fixture.boundary_truth.canonical_native_review_ref =
        format!("native_review:{}", row.install_review_ref);
    fixture.boundary_truth.owner_origin_summary = format!(
        "{} is served from {:?}; publisher {}; catalog descriptor {}.",
        row.extension_identity,
        catalog.lifecycle.source_registry_class,
        row.publisher_display_label,
        catalog.descriptor_id
    );

    let topology = install_topology_packet();
    let topology_row = topology
        .row_by_id(&fixture.install_topology_row_id)
        .expect("install-review fixture must cite an install-topology row");

    evaluate_install_review_alpha(InstallReviewAlphaEvaluation {
        input: fixture.input,
        extension_review: &fixture.extension_review,
        effective_permission: &fixture.effective_permission,
        boundary_truth: fixture.boundary_truth,
        compatibility: fixture.compatibility,
        activation_budget: fixture.activation_budget,
        install_topology_row: topology_row,
        decided_at: "2026-05-16T19:32:00Z",
    })
}

fn install_review_fixture() -> InstallReviewFixture {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/extensions/install_review_alpha/native_marketplace_package_lane.json"
    )))
    .expect("install-review fixture must parse")
}

fn install_topology_packet() -> InstallTopologyAlphaPacket {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/topology_alpha/install_topology_alpha_packet.json"
    )))
    .expect("install-topology fixture must parse")
}

fn script_risk_for(spec: &SeededMarketplaceRowSpec) -> ScriptRiskDisclosure {
    ScriptRiskDisclosure {
        script_risk_class: spec.script_risk_class,
        risk_source_refs: vec![format!("registry_manifest:script-risk:{}", spec.row_suffix)],
        native_build_requirement_refs: if spec.script_risk_class
            == ScriptRiskClass::NativeBuildRequired
        {
            vec!["toolchain:native-build-reviewed".to_string()]
        } else {
            Vec::new()
        },
        policy_block_refs: Vec::new(),
        summary: match spec.script_risk_class {
            ScriptRiskClass::NoScriptsOrNativeBuild => {
                "No lifecycle scripts or native build steps are declared.".to_string()
            }
            ScriptRiskClass::LifecycleScriptsDeclared => {
                "Lifecycle scripts are declared and must stay visible before commit.".to_string()
            }
            ScriptRiskClass::NativeBuildRequired => {
                "A native build or toolchain step is required before activation.".to_string()
            }
            ScriptRiskClass::ExternalHelperOrHost => {
                "An external helper or host process is part of the package posture.".to_string()
            }
            ScriptRiskClass::UnknownScriptRiskBlocked => {
                "Script/native-build risk is unknown and blocks mutation.".to_string()
            }
        },
    }
}

fn manifest_changes_for(
    row: &MarketplaceTruthRowRecord,
    catalog: &CatalogDescriptorRecord,
) -> Vec<ManifestChangeRow> {
    vec![
        ManifestChangeRow {
            change_id: format!("manifest_change:{}:catalog_manifest", row.row_id),
            change_class: ManifestChangeClass::Added,
            manifest_ref: catalog.registry_manifest_ref.clone(),
            field_path: "extension_manifest".to_string(),
            before_summary: Some("not installed".to_string()),
            after_summary: Some(format!(
                "{}@{} from {:?}",
                row.extension_identity, row.extension_version, row.registry_source_class
            )),
            review_required: true,
            summary: "Install review shows the extension manifest entry before commit.".to_string(),
        },
        ManifestChangeRow {
            change_id: format!("manifest_change:{}:permission_delta", row.row_id),
            change_class: ManifestChangeClass::PermissionDelta,
            manifest_ref: catalog.permission_manifest_ref.clone(),
            field_path: "permissions".to_string(),
            before_summary: Some("not installed".to_string()),
            after_summary: Some("declared permissions plus effective policy delta".to_string()),
            review_required: true,
            summary: "Permission changes are reviewed through the native install sheet."
                .to_string(),
        },
    ]
}

fn lockfile_impact_for(spec: &SeededMarketplaceRowSpec) -> LockfileImpact {
    let (impact_class, generated_file_refs) = match spec.surface_class {
        MarketplaceFactGridSurfaceClass::ManualImportReview => (
            LockfileImpactClass::RegenerateAndReview,
            vec!["generated:manual-import-manifest-review".to_string()],
        ),
        _ if spec.script_risk_class == ScriptRiskClass::NativeBuildRequired => (
            LockfileImpactClass::GeneratedFilesExpected,
            vec!["generated:native-helper-build-plan".to_string()],
        ),
        _ => (LockfileImpactClass::LockfileChurnExpected, Vec::new()),
    };

    LockfileImpact {
        impact_class,
        resolver_ref: "resolver:aureline-extension-lock:v1".to_string(),
        affected_lockfile_refs: vec!["aureline.extensions.lock".to_string()],
        generated_file_refs,
        environment_factor_refs: vec!["platform:any".to_string(), "profile:current".to_string()],
        rollback_checkpoint_ref: Some("checkpoint:extension-lock:last-known-good".to_string()),
        summary: "Extension lockfile or generated-file churn is declared before commit."
            .to_string(),
    }
}

fn content_source_for_registry(
    registry_source_class: CatalogRegistrySourceClass,
) -> InstallReviewContentSourceClass {
    match registry_source_class {
        CatalogRegistrySourceClass::PublicRegistry => InstallReviewContentSourceClass::FirstParty,
        CatalogRegistrySourceClass::ApprovedMirror | CatalogRegistrySourceClass::OfflineBundle => {
            InstallReviewContentSourceClass::Mirrored
        }
        CatalogRegistrySourceClass::PrivateRegistry => {
            InstallReviewContentSourceClass::AccountOwned
        }
        CatalogRegistrySourceClass::LocalArchive
        | CatalogRegistrySourceClass::QuarantinedLocalCopy => {
            InstallReviewContentSourceClass::Community
        }
    }
}

fn manifest_origin_for_registry(
    registry_source_class: CatalogRegistrySourceClass,
) -> ManifestOriginSourceClass {
    match registry_source_class {
        CatalogRegistrySourceClass::PublicRegistry => ManifestOriginSourceClass::PublicRegistry,
        CatalogRegistrySourceClass::ApprovedMirror => ManifestOriginSourceClass::Mirror,
        CatalogRegistrySourceClass::PrivateRegistry => ManifestOriginSourceClass::PrivateRegistry,
        CatalogRegistrySourceClass::OfflineBundle => ManifestOriginSourceClass::OfflineBundle,
        CatalogRegistrySourceClass::LocalArchive
        | CatalogRegistrySourceClass::QuarantinedLocalCopy => {
            ManifestOriginSourceClass::VendoredLocal
        }
    }
}

fn compatibility_report() -> CompatibilityReportSnapshot {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/compat/m3/compatibility_report.json"
    )))
    .expect("compatibility report artifact must parse")
}

/// Loads the checked extension bridge matrix consumed by the marketplace page.
pub fn bridge_matrix() -> ExtensionBridgeMatrix {
    current_extension_bridge_matrix().expect("extension bridge matrix artifact must parse")
}

fn catalog_record() -> CatalogDescriptorRecord {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/extensions/m3/registry_moderation/catalog_descriptor_record.json"
    )))
    .expect("catalog descriptor artifact must parse")
}

fn catalog_source_variant(
    row_suffix: &str,
    registry_source_class: CatalogRegistrySourceClass,
    mirrorability_class: CatalogMirrorabilityClass,
    trust_rule_class: CatalogTrustBadgeInheritanceRuleClass,
    snapshot_age_class: CatalogRevocationSnapshotAgeClass,
    source_endpoint_ref: &str,
) -> CatalogDescriptorRecord {
    let mut input = catalog_input_from_record(catalog_record());
    let package_suffix = format!("dev.aureline.samples/wasm-notes:{row_suffix}");

    input.descriptor_id = format!("catalog_descriptor:{package_suffix}");
    input.package_id = format!("dev.aureline.samples.wasm-notes.{row_suffix}");
    input.publication_ref = format!("publication:{package_suffix}");
    input.registry_manifest_ref = format!("registry_manifest:{package_suffix}");
    input.permission_manifest_ref = format!("permission_manifest:{package_suffix}");
    input.lifecycle.lifecycle_state_class = CatalogLifecycleStateClass::Approved;
    input.lifecycle.source_registry_class = registry_source_class;
    input.lifecycle.source_endpoint_ref = source_endpoint_ref.to_string();
    input.lifecycle.support_class = format!("{registry_source_class:?}");
    input.lifecycle.lifecycle_event_refs = vec![format!("catalog_lifecycle:{package_suffix}")];
    input.moderation.moderation_state_class = CatalogModerationStateClass::Admitted;
    input.moderation.moderation_review_ref = format!("moderation_review:{package_suffix}");
    input.revocation.revocation_state_class = RevocationStateClass::NoKnownRevocation;
    input.revocation.revocation_snapshot_ref = format!("revocation_snapshot:{package_suffix}");
    input.revocation.revocation_snapshot_age_class = snapshot_age_class;
    input.revocation.rollback_manifest_ref = format!("rollback_manifest:{package_suffix}");
    input.mirror.mirrorability_class = mirrorability_class;
    input.mirror.mirror_descriptor_ref = format!("mirror_descriptor:{package_suffix}");
    input.mirror.mirror_registry_source_class = registry_source_class;
    input.mirror.signature_ref = format!("signature:{package_suffix}");
    input.mirror.trust_badge_inheritance_rule_class = trust_rule_class;
    input.mirror.parity_assertion_refs = vec![format!("parity_assertion:{package_suffix}")];
    input.generated_at = "2026-05-16T19:29:00Z".to_string();

    evaluate_catalog_descriptor(input)
}

fn catalog_input_from_record(record: CatalogDescriptorRecord) -> CatalogDescriptorInput {
    CatalogDescriptorInput {
        descriptor_id: record.descriptor_id,
        extension_identity: record.extension_identity,
        extension_version: record.extension_version,
        package_id: record.package_id,
        display_name: record.display_name,
        publication_ref: record.publication_ref,
        registry_manifest_ref: record.registry_manifest_ref,
        permission_manifest_ref: record.permission_manifest_ref,
        runtime_contract_ref: record.runtime_contract_ref,
        compatibility_report_ref: record.compatibility_report_ref,
        publisher: record.publisher,
        lifecycle: record.lifecycle,
        moderation: record.moderation,
        revocation: record.revocation,
        mirror: record.mirror,
        rendered_disclosures: record.rendered_disclosures,
        generated_at: record.generated_at,
    }
}

fn evaluated_catalog_fixture(name: &str) -> CatalogDescriptorRecord {
    let raw = match name {
        "staged_pending_moderation" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/extensions/m3/registry_moderation/staged_pending_moderation.json"
        )),
        "limited_compatibility_catalog" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/extensions/m3/registry_moderation/limited_compatibility_catalog.json"
        )),
        "revoked_catalog_refused" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/extensions/m3/registry_moderation/revoked_catalog_refused.json"
        )),
        other => panic!("unknown catalog fixture {other}"),
    };
    let fixture: CatalogFixture = serde_json::from_str(raw).expect("catalog fixture must parse");
    evaluate_catalog_descriptor(fixture.input)
}

#[derive(Debug, Deserialize)]
struct CatalogFixture {
    input: CatalogDescriptorInput,
}

struct SeededMarketplaceRowSpec {
    row_suffix: &'static str,
    catalog: CatalogDescriptorRecord,
    bridge_matrix_row_ref: &'static str,
    install_review_ref: &'static str,
    surface_class: MarketplaceFactGridSurfaceClass,
    client_scope_class: ClientScopeClass,
    script_risk_class: ScriptRiskClass,
}

#[derive(Debug, Deserialize)]
struct InstallReviewFixture {
    input: InstallReviewAlphaInput,
    extension_review: ExtensionReviewAlphaPacketRecord,
    effective_permission: EffectivePermissionBaselineRecord,
    boundary_truth: InstallReviewBoundaryTruth,
    compatibility: CompatibilityLabelBlock,
    activation_budget: ActivationBudgetDisclosure,
    install_topology_row_id: String,
}
