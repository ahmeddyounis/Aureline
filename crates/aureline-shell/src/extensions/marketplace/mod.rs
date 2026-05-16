//! Shell marketplace row projection for trust, support, and compatibility truth.
//!
//! The extension registry produces catalog descriptors, and the compatibility
//! lane produces the generated report. This module is the first shell consumer
//! that renders those facts together on discovery rows before the user opens
//! install review. The projection is deliberately read-only: install and update
//! authority stays with the native review records.

use serde::{Deserialize, Serialize};

use aureline_extensions::{
    current_extension_bridge_matrix, evaluate_catalog_descriptor, project_marketplace_truth_row,
    project_marketplace_truth_support_export, validate_extension_bridge_matrix,
    validate_marketplace_truth_row, validate_marketplace_truth_support_export,
    CatalogDescriptorInput, CatalogDescriptorRecord, CompatibilityReportSnapshot,
    ExtensionBridgeMatrix, MarketplaceTruthBadgeClass, MarketplaceTruthFinding,
    MarketplaceTruthRowInput, MarketplaceTruthRowRecord, MarketplaceTruthSupportExportRecord,
};

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
    let rows = vec![
        project_seeded_row(
            "public-beta",
            &catalog_record(),
            &report,
            &bridge_matrix,
            "extension_bridge_row:wasm_component_native_beta",
            "install_review_alpha:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
        ),
        project_seeded_row(
            "mirror-preview",
            &evaluated_catalog_fixture("staged_pending_moderation"),
            &report,
            &bridge_matrix,
            "extension_bridge_row:wasm_component_native_beta",
            "install_review_alpha:dev.aureline.samples/wasm-notes:mirror-staged",
        ),
        project_seeded_row(
            "mirror-retest-pending",
            &evaluated_catalog_fixture("limited_compatibility_catalog"),
            &report,
            &bridge_matrix,
            "extension_bridge_row:vscode_api_bridge_beta",
            "install_review_alpha:dev.aureline.samples/wasm-notes:limited",
        ),
        project_seeded_row(
            "revoked",
            &evaluated_catalog_fixture("revoked_catalog_refused"),
            &report,
            &bridge_matrix,
            "extension_bridge_row:unsupported_webview_runtime",
            "install_review_alpha:dev.aureline.samples/wasm-notes:revoked",
        ),
    ];
    let support_rows = rows
        .iter()
        .map(|row| {
            project_marketplace_truth_support_export(
                row,
                &format!("marketplace_truth_support_export:{}", row.row_id),
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
