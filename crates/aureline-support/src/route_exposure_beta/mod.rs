//! Beta route/exposure matrix support-export consumer.
//!
//! This module consumes the checked-in route/exposure matrix and exposes the
//! metadata-safe support-export projection that Help/About, service health,
//! diagnostics, release evidence, and support bundles can share. It does not
//! open browser destinations, resolve provider callbacks, or export raw route
//! material; rows carry opaque refs and closed vocabulary tokens only.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Repository-relative path to the route/exposure matrix.
pub const ROUTE_EXPOSURE_MATRIX_PATH: &str = "artifacts/routes/m3/route_exposure_matrix.json";

/// Repository-relative path to the route/exposure matrix schema.
pub const ROUTE_EXPOSURE_MATRIX_SCHEMA_PATH: &str = "schemas/routes/exposure_matrix.schema.json";

/// Stable record kind for [`RouteExposureMatrix`] payloads.
pub const ROUTE_EXPOSURE_MATRIX_RECORD_KIND: &str = "route_exposure_matrix";

/// Stable record kind for [`RouteExposureSupportExport`] payloads.
pub const ROUTE_EXPOSURE_SUPPORT_EXPORT_RECORD_KIND: &str = "route_exposure_support_export";

/// Schema version exported with route/exposure matrix records.
pub const ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION: u32 = 1;

const ROUTE_EXPOSURE_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/routes/m3/route_exposure_matrix.json"
));

const REQUIRED_ORIGIN_CLASSES: &[&str] = &[
    "local_desktop",
    "remote_helper",
    "managed_workspace",
    "browser_companion",
    "provider_linked_context",
    "embedded_docs_help_webview",
    "headless_cli",
];

const REQUIRED_CONSUMER_SURFACES: &[&str] = &[
    "help_about",
    "service_health",
    "diagnostics",
    "support_export",
    "docs_help",
];

/// Loads the checked-in beta route/exposure matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`RouteExposureMatrix`].
pub fn current_route_exposure_matrix() -> Result<RouteExposureMatrix, serde_json::Error> {
    serde_json::from_str(ROUTE_EXPOSURE_MATRIX_JSON)
}

/// Validates a route/exposure matrix and returns typed findings on failure.
pub fn validate_route_exposure_matrix(
    matrix: &RouteExposureMatrix,
) -> Result<(), Vec<RouteExposureFinding>> {
    let findings = audit_route_exposure_matrix(matrix);
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Audits a route/exposure matrix without failing on findings.
pub fn audit_route_exposure_matrix(matrix: &RouteExposureMatrix) -> Vec<RouteExposureFinding> {
    let mut findings = Vec::new();
    if matrix.schema_version != ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION {
        findings.push(RouteExposureFinding::new(
            "matrix.schema_version",
            "matrix",
            "schema_version must be 1",
        ));
    }
    if matrix.record_kind != ROUTE_EXPOSURE_MATRIX_RECORD_KIND {
        findings.push(RouteExposureFinding::new(
            "matrix.record_kind",
            "matrix",
            "record_kind must be route_exposure_matrix",
        ));
    }

    let mut row_ids = BTreeSet::new();
    let mut origin_classes = BTreeSet::new();
    for row in &matrix.rows {
        if !row_ids.insert(row.row_id.as_str()) {
            findings.push(RouteExposureFinding::new(
                "row.duplicate_id",
                &row.row_id,
                "row_id must be unique",
            ));
        }
        origin_classes.insert(row.origin.origin_class.as_str());
        validate_row(matrix, row, &mut findings);
    }

    for required in REQUIRED_ORIGIN_CLASSES {
        if !origin_classes.contains(required) {
            findings.push(RouteExposureFinding::new(
                "matrix.required_origin_missing",
                *required,
                "required origin class is not covered by matrix rows",
            ));
        }
    }

    findings
}

fn validate_row(
    matrix: &RouteExposureMatrix,
    row: &RouteExposureRow,
    findings: &mut Vec<RouteExposureFinding>,
) {
    if row.promotion_guard.high_risk {
        if row.exposure.action_exposure_class == "exposure_unknown_requires_review" {
            findings.push(RouteExposureFinding::new(
                "row.high_risk_unknown_exposure",
                &row.row_id,
                "high-risk row cannot use exposure_unknown_requires_review",
            ));
        }
        if row.promotion_guard.uncategorized_high_risk_gap {
            findings.push(RouteExposureFinding::new(
                "row.high_risk_uncategorized",
                &row.row_id,
                "high-risk row cannot remain uncategorized",
            ));
        }
        if !row.promotion_guard.beta_promotion_blocking_when_unknown {
            findings.push(RouteExposureFinding::new(
                "row.high_risk_not_blocking",
                &row.row_id,
                "high-risk unknown state must block beta promotion",
            ));
        }
    }

    if !row.support_export.parity_required {
        findings.push(RouteExposureFinding::new(
            "row.support_parity_not_required",
            &row.row_id,
            "support-export parity must be required",
        ));
    }
    for required in REQUIRED_CONSUMER_SURFACES {
        if !row
            .support_export
            .consumer_surfaces
            .iter()
            .any(|surface| surface == required)
        {
            findings.push(RouteExposureFinding::new(
                "row.consumer_surface_missing",
                &row.row_id,
                format!("missing required consumer surface {required}"),
            ));
        }
    }
    if row.support_export.raw_url_export_allowed
        || row.support_export.raw_token_export_allowed
        || row.support_export.raw_provider_payload_export_allowed
    {
        findings.push(RouteExposureFinding::new(
            "row.raw_export_allowed",
            &row.row_id,
            "route/exposure support rows are metadata-only",
        ));
    }

    let needs_packet = matches!(
        row.handoff.browser_handoff_class.as_str(),
        "system_browser_required"
            | "system_browser_fallback_available"
            | "embedded_webview_boundary"
            | "browser_companion_to_desktop"
            | "provider_callback_return"
    );
    if needs_packet && row.handoff.browser_handoff_packet_ref.is_none() {
        findings.push(RouteExposureFinding::new(
            "row.handoff_packet_missing",
            &row.row_id,
            "browser/system handoff rows must carry an opaque packet ref",
        ));
    }

    validate_token(
        matrix,
        "origin_class",
        &row.origin.origin_class,
        &row.row_id,
        findings,
    );
    validate_token(
        matrix,
        "target_class",
        &row.target.target_class,
        &row.row_id,
        findings,
    );
    validate_token(
        matrix,
        "action_route_class",
        &row.route.action_route_class,
        &row.row_id,
        findings,
    );
    validate_token(
        matrix,
        "action_exposure_class",
        &row.exposure.action_exposure_class,
        &row.row_id,
        findings,
    );
    validate_token(
        matrix,
        "approval_reuse_class",
        &row.approval.approval_reuse_class,
        &row.row_id,
        findings,
    );
    validate_token(
        matrix,
        "browser_handoff_class",
        &row.handoff.browser_handoff_class,
        &row.row_id,
        findings,
    );
}

fn validate_token(
    matrix: &RouteExposureMatrix,
    vocabulary: &str,
    token: &str,
    row_id: &str,
    findings: &mut Vec<RouteExposureFinding>,
) {
    let allowed = match vocabulary {
        "origin_class" => &matrix.vocabularies.origin_class,
        "target_class" => &matrix.vocabularies.target_class,
        "action_route_class" => &matrix.vocabularies.action_route_class,
        "action_exposure_class" => &matrix.vocabularies.action_exposure_class,
        "approval_reuse_class" => &matrix.vocabularies.approval_reuse_class,
        "browser_handoff_class" => &matrix.vocabularies.browser_handoff_class,
        _ => return,
    };
    if !allowed.iter().any(|allowed| allowed == token) {
        findings.push(RouteExposureFinding::new(
            format!("row.{vocabulary}.unknown"),
            row_id,
            format!("{token} is not in the matrix vocabulary"),
        ));
    }
}

/// Machine-readable beta route/exposure matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Matrix lifecycle state.
    pub status: String,
    /// Date the matrix describes.
    pub as_of: String,
    /// Owning release/support reviewer.
    pub owner_dri: String,
    /// Repository-relative schema ref.
    pub schema_ref: String,
    /// Source artifacts the matrix consumes.
    pub source_refs: BTreeMap<String, String>,
    /// Closed vocabularies used by matrix rows.
    pub vocabularies: RouteExposureVocabularies,
    /// Route/exposure rows covered by this matrix.
    pub rows: Vec<RouteExposureRow>,
    /// Validation contract for release and support parity.
    pub validation_contract: RouteExposureValidationContract,
}

/// Closed vocabularies used by route/exposure rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureVocabularies {
    /// Row-level origin classes.
    pub origin_class: Vec<String>,
    /// Route-truth action origin classes.
    pub action_origin_class: Vec<String>,
    /// Route-truth target classes.
    pub target_class: Vec<String>,
    /// Route-truth route classes.
    pub action_route_class: Vec<String>,
    /// Route-truth exposure classes.
    pub action_exposure_class: Vec<String>,
    /// Route-change reason codes.
    pub route_change_reason_code: Vec<String>,
    /// Approval reuse posture classes.
    pub approval_reuse_class: Vec<String>,
    /// Reapproval trigger classes.
    pub reapproval_trigger_class: Vec<String>,
    /// Privacy consequence classes.
    pub privacy_consequence_class: Vec<String>,
    /// Browser/system handoff classes.
    pub browser_handoff_class: Vec<String>,
    /// Consumer surfaces that must quote row truth.
    pub consumer_surface_class: Vec<String>,
}

/// One route/exposure matrix row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureRow {
    /// Stable row id.
    pub row_id: String,
    /// Reviewer-facing row title.
    pub title: String,
    /// Claimed beta surfaces governed by this row.
    pub claimed_beta_surface_refs: Vec<String>,
    /// Handoff, callback, command, or route refs covered by this row.
    pub route_refs: Vec<String>,
    /// Provider route-resolution row refs covered by this row.
    #[serde(default)]
    pub provider_route_resolution_row_refs: Vec<String>,
    /// Source artifacts this row consumes.
    pub source_artifact_refs: Vec<String>,
    /// Acting origin block.
    pub origin: RouteExposureOrigin,
    /// Target identity block.
    pub target: RouteExposureTarget,
    /// Route class block.
    pub route: RouteExposureRoute,
    /// Exposure and privacy block.
    pub exposure: RouteExposure,
    /// Approval reuse and reapproval block.
    pub approval: RouteExposureApproval,
    /// Browser/system handoff block.
    pub handoff: RouteExposureHandoff,
    /// Support-export parity block.
    pub support_export: RouteExposureSupportProjection,
    /// Beta promotion guard block.
    pub promotion_guard: RouteExposurePromotionGuard,
}

/// Origin block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureOrigin {
    /// Matrix origin class.
    pub origin_class: String,
    /// Route-truth action origin class.
    pub action_origin_class: String,
    /// Export-safe traffic-origin label.
    pub traffic_origin_label: String,
}

/// Target block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureTarget {
    /// Route-truth target class.
    pub target_class: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque workspace identity ref.
    pub workspace_identity_ref: String,
    /// Opaque environment identity ref.
    pub environment_identity_ref: String,
}

/// Route block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureRoute {
    /// Route-truth route class.
    pub action_route_class: String,
    /// Route-choice token from the owning lane.
    pub route_choice: String,
    /// Route-change reason code.
    pub route_change_reason_code: String,
    /// Opaque authority source ref.
    pub authority_source_ref: String,
}

/// Exposure and privacy block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposure {
    /// Route-truth exposure class.
    pub action_exposure_class: String,
    /// Export-safe exposure label.
    pub exposure_label: String,
    /// Privacy consequence class.
    pub privacy_consequence_class: String,
    /// Export-safe privacy consequence summary.
    pub privacy_consequence_summary: String,
}

/// Approval reuse and reapproval block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureApproval {
    /// Approval reuse posture class.
    pub approval_reuse_class: String,
    /// Whether approval may be reused while the row remains stable.
    pub approval_reuse_allowed: bool,
    /// Drift classes that force reapproval.
    pub reapproval_trigger_classes: Vec<String>,
    /// Optional opaque approval ticket ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Export-safe reapproval summary.
    pub reapproval_summary: String,
}

/// Browser/system handoff block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureHandoff {
    /// Browser/system handoff class.
    pub browser_handoff_class: String,
    /// Optional opaque handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Whether the row requires the system browser.
    pub system_browser_required: bool,
    /// Export-safe destination class.
    pub destination_class: String,
    /// Optional opaque return anchor ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_anchor_ref: Option<String>,
    /// Export-safe handoff summary.
    pub handoff_summary: String,
}

/// Support-export projection block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureSupportProjection {
    /// Stable support item id.
    pub support_item_id: String,
    /// Support packet projection ref.
    pub projection_ref: String,
    /// Surfaces that must quote the same row truth.
    pub consumer_surfaces: Vec<String>,
    /// Whether support-export parity is required.
    pub parity_required: bool,
    /// Whether raw browser destinations may be exported.
    pub raw_url_export_allowed: bool,
    /// Whether raw token material may be exported.
    pub raw_token_export_allowed: bool,
    /// Whether raw provider payloads may be exported.
    pub raw_provider_payload_export_allowed: bool,
}

/// Beta promotion guard block for a route/exposure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposurePromotionGuard {
    /// Whether the row is high risk for beta promotion.
    pub high_risk: bool,
    /// Whether the high-risk row still has an uncategorized gap.
    pub uncategorized_high_risk_gap: bool,
    /// Whether an unknown state blocks beta promotion.
    pub beta_promotion_blocking_when_unknown: bool,
    /// Subject refs checked for drift by release validation.
    pub drift_check_subject_refs: Vec<String>,
}

/// Matrix validation contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureValidationContract {
    /// Origin classes that must be covered by rows.
    pub required_origin_classes: Vec<String>,
    /// Consumer surfaces that must receive support parity.
    pub required_consumer_surfaces: Vec<String>,
    /// Claimed-surface register ref.
    pub claimed_handoff_register_ref: String,
    /// Provider route-resolution support export ref.
    pub provider_route_support_export_ref: String,
    /// Release matrix Markdown ref.
    pub release_matrix_ref: String,
    /// Support audit Markdown ref.
    pub support_audit_ref: String,
    /// UX contract Markdown ref.
    pub ux_doc_ref: String,
}

/// Typed route/exposure validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureFinding {
    /// Finding check id.
    pub check_id: String,
    /// Subject row id or matrix token.
    pub subject_ref: String,
    /// Export-safe finding message.
    pub message: String,
}

impl RouteExposureFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Metadata-only support export for the route/exposure matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExposureSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Export schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Source matrix id.
    pub matrix_id: String,
    /// Number of matrix rows exported.
    pub row_count: usize,
    /// Number of high-risk rows exported.
    pub high_risk_row_count: usize,
    /// Row counts by origin class.
    pub rows_by_origin_class: BTreeMap<String, usize>,
    /// Row counts by exposure class.
    pub rows_by_exposure_class: BTreeMap<String, usize>,
    /// Row counts by browser/system handoff class.
    pub rows_by_handoff_class: BTreeMap<String, usize>,
    /// Exported matrix payload.
    pub matrix: RouteExposureMatrix,
    /// Typed validation findings present at export time.
    pub findings: Vec<RouteExposureFinding>,
    /// Whether raw URLs are excluded.
    pub raw_urls_excluded: bool,
    /// Whether raw tokens are excluded.
    pub raw_tokens_excluded: bool,
    /// Whether raw provider payloads are excluded.
    pub raw_provider_payloads_excluded: bool,
}

impl RouteExposureSupportExport {
    /// Builds a metadata-only support export from a route/exposure matrix.
    pub fn from_matrix(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        matrix: RouteExposureMatrix,
    ) -> Self {
        let findings = audit_route_exposure_matrix(&matrix);
        let mut rows_by_origin_class = BTreeMap::new();
        let mut rows_by_exposure_class = BTreeMap::new();
        let mut rows_by_handoff_class = BTreeMap::new();
        let mut high_risk_row_count = 0usize;
        for row in &matrix.rows {
            if row.promotion_guard.high_risk {
                high_risk_row_count += 1;
            }
            *rows_by_origin_class
                .entry(row.origin.origin_class.clone())
                .or_insert(0) += 1;
            *rows_by_exposure_class
                .entry(row.exposure.action_exposure_class.clone())
                .or_insert(0) += 1;
            *rows_by_handoff_class
                .entry(row.handoff.browser_handoff_class.clone())
                .or_insert(0) += 1;
        }
        Self {
            record_kind: ROUTE_EXPOSURE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            matrix_id: matrix.matrix_id.clone(),
            row_count: matrix.rows.len(),
            high_risk_row_count,
            rows_by_origin_class,
            rows_by_exposure_class,
            rows_by_handoff_class,
            matrix,
            findings,
            raw_urls_excluded: true,
            raw_tokens_excluded: true,
            raw_provider_payloads_excluded: true,
        }
    }
}
