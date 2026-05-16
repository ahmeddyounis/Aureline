//! Extension runtime, SDK, manifest, and bridge compatibility matrix.
//!
//! The checked matrix at
//! [`/artifacts/compat/m3/bridge_matrix.yaml`](../../../../artifacts/compat/m3/bridge_matrix.yaml)
//! is the canonical source for beta extension compatibility lanes. It
//! binds every claimed lane to four explicit windows: runtime, SDK,
//! manifest, and bridge. Marketplace rows, SDK docs, publication
//! packets, and support exports cite these row ids instead of creating
//! local bridge or shim claims.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Record-kind tag carried by [`ExtensionBridgeMatrix`] payloads.
pub const EXTENSION_BRIDGE_MATRIX_RECORD_KIND: &str = "extension_bridge_matrix";

/// Schema version for extension bridge-matrix payloads.
pub const EXTENSION_BRIDGE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked extension bridge matrix.
pub const CURRENT_EXTENSION_BRIDGE_MATRIX_PATH: &str = "artifacts/compat/m3/bridge_matrix.yaml";

const CURRENT_EXTENSION_BRIDGE_MATRIX_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/compat/m3/bridge_matrix.yaml"
));

/// Loads the checked extension bridge matrix.
///
/// # Errors
///
/// Returns a YAML parse error when the checked artifact does not match
/// [`ExtensionBridgeMatrix`].
pub fn current_extension_bridge_matrix() -> Result<ExtensionBridgeMatrix, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_EXTENSION_BRIDGE_MATRIX_YAML)
}

/// Canonical matrix for beta extension compatibility lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBridgeMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Matrix revision.
    pub matrix_revision: u32,
    /// Reviewer-facing report id.
    pub report_id: String,
    /// Release-channel scope covered by the matrix.
    pub release_channel_scope: String,
    /// Date string the matrix is valid as of.
    pub as_of: String,
    /// Generation timestamp for this revision.
    pub generated_at: String,
    /// Owning reviewer or team handle.
    pub owner: String,
    /// Upstream generated compatibility-report row.
    pub source_report_ref: String,
    /// Reviewer-facing extension compatibility report path.
    pub extension_report_ref: String,
    /// Author-facing docs projection path.
    pub docs_projection_ref: String,
    /// JSON schema path for this matrix.
    pub schema_ref: String,
    /// Surfaces that consume this matrix directly.
    pub consuming_surfaces: Vec<String>,
    /// Closed bridge-state vocabulary declared by the matrix.
    pub bridge_state_vocabulary: Vec<ExtensionBridgeStateClass>,
    /// Closed compatibility-label vocabulary declared by the matrix.
    pub compatibility_label_vocabulary: Vec<ExtensionCompatibilityLabel>,
    /// Closed parity-claim vocabulary declared by the matrix.
    pub parity_claim_vocabulary: Vec<ExtensionParityClaimClass>,
    /// Compatibility lane rows.
    pub rows: Vec<ExtensionBridgeMatrixRow>,
}

impl ExtensionBridgeMatrix {
    /// Returns a row by its stable `row_id`.
    pub fn row_by_ref(&self, row_ref: &str) -> Option<&ExtensionBridgeMatrixRow> {
        self.rows.iter().find(|row| row.row_id == row_ref)
    }
}

/// One claimed extension compatibility lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBridgeMatrixRow {
    /// Stable matrix row id.
    pub row_id: String,
    /// Human-readable lane token.
    pub claimed_lane: String,
    /// Package or lane identity covered by the row.
    pub package_or_lane_id: String,
    /// Upstream generated compatibility-report row.
    pub report_row_ref: String,
    /// Effective support class for the lane.
    pub support_class: String,
    /// Runtime compatibility window.
    pub runtime_window: ExtensionCompatibilityWindow,
    /// SDK compatibility window.
    pub sdk_window: ExtensionCompatibilityWindow,
    /// Manifest compatibility window.
    pub manifest_window: ExtensionCompatibilityWindow,
    /// Bridge compatibility window.
    pub bridge_window: ExtensionBridgeWindow,
    /// Downgrade behavior for the row.
    pub downgrade_behavior: ExtensionDowngradeBehavior,
    /// Evidence refs cited by the row.
    pub evidence_refs: Vec<String>,
    /// Marketplace row refs that consume this row.
    #[serde(default)]
    pub marketplace_surface_refs: Vec<String>,
    /// SDK docs refs that consume this row.
    #[serde(default)]
    pub sdk_doc_refs: Vec<String>,
    /// Release packet refs that consume this row.
    #[serde(default)]
    pub release_packet_refs: Vec<String>,
    /// Support export refs that consume this row.
    #[serde(default)]
    pub support_export_refs: Vec<String>,
    /// Export-safe row note.
    pub notes: String,
}

/// Runtime, SDK, or manifest compatibility window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionCompatibilityWindow {
    /// Stable window id.
    pub window_id: String,
    /// Version range or explicit unsupported statement.
    pub version_range: String,
    /// Compatibility-window class.
    pub support_window_class: String,
    /// Downgrade behavior token.
    pub downgrade_behavior: String,
    /// Out-of-window posture token.
    pub out_of_window_posture: String,
    /// Rule rendered when the window is exceeded.
    pub contract_rule: String,
}

/// Bridge-specific compatibility window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBridgeWindow {
    /// Stable bridge-window id.
    pub window_id: String,
    /// Native, bridge, shimmed, partial, or unsupported bridge state.
    pub bridge_state_class: ExtensionBridgeStateClass,
    /// User-facing compatibility label.
    pub compatibility_label: ExtensionCompatibilityLabel,
    /// Explicit parity claim for this bridge state.
    pub parity_claim_class: ExtensionParityClaimClass,
    /// Source ecosystem the row translates or rejects.
    pub source_ecosystem: String,
    /// Artifact classes admitted by this bridge window.
    #[serde(default)]
    pub supported_artifact_classes: Vec<String>,
    /// Known caveats or unsupported behavior.
    #[serde(default)]
    pub known_limits: Vec<String>,
    /// Permission delta introduced by this bridge window.
    pub permission_delta: String,
    /// Performance delta introduced by this bridge window.
    pub performance_delta: String,
    /// Docs or report action ref for inspection.
    pub report_action_ref: String,
}

/// Downgrade behavior for one compatibility matrix row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDowngradeBehavior {
    /// Downgrade support class.
    pub support_class: String,
    /// Out-of-window posture token.
    pub out_of_window_posture: String,
    /// State-preservation note rendered to users and support.
    pub state_preservation_note: String,
    /// Contract rule for unsupported or narrowed cases.
    pub contract_rule: String,
    /// Default repair hints exposed by consuming surfaces.
    pub default_repair_hints: Vec<String>,
}

/// Controlled bridge-state vocabulary for extension compatibility lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionBridgeStateClass {
    /// Native Aureline contract, no bridge required for the declared lane.
    Native,
    /// Governed compatibility bridge translates a declared subset.
    Bridge,
    /// Shim approximates static assets or behavior with known limits.
    Shimmed,
    /// Only a declared subset is supported.
    Partial,
    /// No supported compatibility path exists.
    Unsupported,
}

impl ExtensionBridgeStateClass {
    /// Returns every controlled bridge-state class.
    pub const fn required_acceptance_states() -> [Self; 5] {
        [
            Self::Native,
            Self::Bridge,
            Self::Shimmed,
            Self::Partial,
            Self::Unsupported,
        ]
    }

    /// Returns true when the row must not imply exact parity.
    pub const fn requires_non_parity_disclosure(self) -> bool {
        !matches!(self, Self::Native)
    }

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bridge => "bridge",
            Self::Shimmed => "shimmed",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Controlled compatibility-label vocabulary for bridge-matrix rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionCompatibilityLabel {
    /// Native declared behavior matches the current target exactly.
    Exact,
    /// Behavior is translated through a compatibility bridge.
    Translated,
    /// Only part of the source behavior is supported.
    Partial,
    /// Behavior is shimmed with known caveats.
    Shimmed,
    /// No supported compatibility path exists.
    Unsupported,
}

/// Controlled parity claim for bridge-matrix rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionParityClaimClass {
    /// Native exact where the declared row scope applies.
    NativeExactWhereDeclared,
    /// Bridge row explicitly does not claim exact parity.
    BridgeNoExactParity,
    /// Shimmed row explicitly does not claim exact parity.
    ShimmedNoExactParity,
    /// Partial row explicitly does not claim exact parity.
    PartialNoExactParity,
    /// Unsupported row has no parity claim.
    UnsupportedNoParity,
}

/// Typed validation finding emitted by bridge-matrix validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBridgeMatrixFinding {
    /// Stable validation check id.
    pub check_id: String,
    /// Human-readable validation message.
    pub message: String,
    /// Optional row id where the finding occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_id: Option<String>,
}

impl ExtensionBridgeMatrixFinding {
    fn matrix(check_id: &str, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.to_string(),
            message: message.into(),
            row_id: None,
        }
    }

    fn row(row_id: &str, check_id: &str, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.to_string(),
            message: message.into(),
            row_id: Some(row_id.to_string()),
        }
    }
}

/// Validates structural and bridge-honesty invariants for the matrix.
pub fn validate_extension_bridge_matrix(
    matrix: &ExtensionBridgeMatrix,
) -> Vec<ExtensionBridgeMatrixFinding> {
    let mut findings = Vec::new();

    if matrix.record_kind != EXTENSION_BRIDGE_MATRIX_RECORD_KIND {
        findings.push(ExtensionBridgeMatrixFinding::matrix(
            "extension_bridge_matrix.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_BRIDGE_MATRIX_RECORD_KIND}'; got {:?}",
                matrix.record_kind
            ),
        ));
    }
    if matrix.schema_version != EXTENSION_BRIDGE_MATRIX_SCHEMA_VERSION {
        findings.push(ExtensionBridgeMatrixFinding::matrix(
            "extension_bridge_matrix.schema_version_wrong",
            format!(
                "schema_version must be {EXTENSION_BRIDGE_MATRIX_SCHEMA_VERSION}; got {}",
                matrix.schema_version
            ),
        ));
    }
    if !matrix.matrix_id.starts_with("extension_bridge_matrix:") {
        findings.push(ExtensionBridgeMatrixFinding::matrix(
            "extension_bridge_matrix.matrix_id_unprefixed",
            "matrix_id must start with 'extension_bridge_matrix:'",
        ));
    }
    if !matrix
        .report_id
        .starts_with("extension_compatibility_report:")
    {
        findings.push(ExtensionBridgeMatrixFinding::matrix(
            "extension_bridge_matrix.report_id_unprefixed",
            "report_id must start with 'extension_compatibility_report:'",
        ));
    }
    if matrix.rows.is_empty() {
        findings.push(ExtensionBridgeMatrixFinding::matrix(
            "extension_bridge_matrix.rows_missing",
            "matrix must contain at least one compatibility row",
        ));
    }

    for state in ExtensionBridgeStateClass::required_acceptance_states() {
        if !matrix.bridge_state_vocabulary.contains(&state) {
            findings.push(ExtensionBridgeMatrixFinding::matrix(
                "extension_bridge_matrix.bridge_state_missing",
                format!("bridge-state vocabulary missing {}", state.as_str()),
            ));
        }
    }

    let mut row_ids = BTreeSet::new();
    for row in &matrix.rows {
        validate_row(row, &mut findings);
        if !row_ids.insert(row.row_id.as_str()) {
            findings.push(ExtensionBridgeMatrixFinding::row(
                &row.row_id,
                "extension_bridge_matrix.row_id_duplicate",
                "row ids must be unique",
            ));
        }
    }

    findings
}

fn validate_row(row: &ExtensionBridgeMatrixRow, findings: &mut Vec<ExtensionBridgeMatrixFinding>) {
    if !row.row_id.starts_with("extension_bridge_row:") {
        findings.push(ExtensionBridgeMatrixFinding::row(
            &row.row_id,
            "extension_bridge_matrix.row_id_unprefixed",
            "row_id must start with 'extension_bridge_row:'",
        ));
    }
    if !row.report_row_ref.starts_with("compat_row:") {
        findings.push(ExtensionBridgeMatrixFinding::row(
            &row.row_id,
            "extension_bridge_matrix.report_row_unprefixed",
            "report_row_ref must start with 'compat_row:'",
        ));
    }
    validate_window(row, "runtime_window", &row.runtime_window, findings);
    validate_window(row, "sdk_window", &row.sdk_window, findings);
    validate_window(row, "manifest_window", &row.manifest_window, findings);

    if row.bridge_window.window_id.trim().is_empty() {
        findings.push(ExtensionBridgeMatrixFinding::row(
            &row.row_id,
            "extension_bridge_matrix.bridge_window_missing",
            "bridge window id must be non-empty",
        ));
    }
    if row.evidence_refs.is_empty() {
        findings.push(ExtensionBridgeMatrixFinding::row(
            &row.row_id,
            "extension_bridge_matrix.evidence_missing",
            "row must cite evidence refs",
        ));
    }
    if row.downgrade_behavior.default_repair_hints.is_empty() {
        findings.push(ExtensionBridgeMatrixFinding::row(
            &row.row_id,
            "extension_bridge_matrix.repair_hints_missing",
            "downgrade behavior must name repair hints",
        ));
    }

    if row
        .bridge_window
        .bridge_state_class
        .requires_non_parity_disclosure()
    {
        if row.bridge_window.compatibility_label == ExtensionCompatibilityLabel::Exact {
            findings.push(ExtensionBridgeMatrixFinding::row(
                &row.row_id,
                "extension_bridge_matrix.bridge_exact_label_refused",
                "bridge, shimmed, partial, and unsupported rows must not use the Exact label",
            ));
        }
        if row.bridge_window.parity_claim_class
            == ExtensionParityClaimClass::NativeExactWhereDeclared
        {
            findings.push(ExtensionBridgeMatrixFinding::row(
                &row.row_id,
                "extension_bridge_matrix.bridge_exact_parity_refused",
                "bridge, shimmed, partial, and unsupported rows must not claim native exact parity",
            ));
        }
        if row.bridge_window.known_limits.is_empty() {
            findings.push(ExtensionBridgeMatrixFinding::row(
                &row.row_id,
                "extension_bridge_matrix.bridge_limits_missing",
                "bridge, shimmed, partial, and unsupported rows must name known limits",
            ));
        }
    }

    match row.bridge_window.bridge_state_class {
        ExtensionBridgeStateClass::Native => {
            if row.bridge_window.compatibility_label != ExtensionCompatibilityLabel::Exact {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.native_label_not_exact",
                    "native rows must render Exact inside their declared scope",
                ));
            }
            if row.bridge_window.parity_claim_class
                != ExtensionParityClaimClass::NativeExactWhereDeclared
            {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.native_parity_not_exact",
                    "native rows must claim exact parity only inside their declared scope",
                ));
            }
        }
        ExtensionBridgeStateClass::Bridge => {
            if row.bridge_window.compatibility_label != ExtensionCompatibilityLabel::Translated {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.bridge_label_not_translated",
                    "bridge rows must render Translated",
                ));
            }
            if row.bridge_window.parity_claim_class
                != ExtensionParityClaimClass::BridgeNoExactParity
            {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.bridge_parity_not_bridge_no_exact",
                    "bridge rows must carry the bridge no-exact-parity claim",
                ));
            }
        }
        ExtensionBridgeStateClass::Shimmed => {
            if row.bridge_window.compatibility_label != ExtensionCompatibilityLabel::Shimmed {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.shimmed_label_not_shimmed",
                    "shimmed rows must render Shimmed",
                ));
            }
            if row.bridge_window.parity_claim_class
                != ExtensionParityClaimClass::ShimmedNoExactParity
            {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.shimmed_parity_not_shimmed_no_exact",
                    "shimmed rows must carry the shimmed no-exact-parity claim",
                ));
            }
        }
        ExtensionBridgeStateClass::Partial => {
            if row.bridge_window.compatibility_label != ExtensionCompatibilityLabel::Partial {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.partial_label_not_partial",
                    "partial rows must render Partial",
                ));
            }
            if row.bridge_window.parity_claim_class
                != ExtensionParityClaimClass::PartialNoExactParity
            {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.partial_parity_not_partial_no_exact",
                    "partial rows must carry the partial no-exact-parity claim",
                ));
            }
        }
        ExtensionBridgeStateClass::Unsupported => {
            if row.bridge_window.compatibility_label != ExtensionCompatibilityLabel::Unsupported {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.unsupported_label_not_unsupported",
                    "unsupported rows must render Unsupported",
                ));
            }
            if row.bridge_window.parity_claim_class
                != ExtensionParityClaimClass::UnsupportedNoParity
            {
                findings.push(ExtensionBridgeMatrixFinding::row(
                    &row.row_id,
                    "extension_bridge_matrix.unsupported_parity_not_unsupported",
                    "unsupported rows must carry the unsupported no-parity claim",
                ));
            }
        }
    }
}

fn validate_window(
    row: &ExtensionBridgeMatrixRow,
    field: &str,
    window: &ExtensionCompatibilityWindow,
    findings: &mut Vec<ExtensionBridgeMatrixFinding>,
) {
    if window.window_id.trim().is_empty()
        || window.version_range.trim().is_empty()
        || window.support_window_class.trim().is_empty()
        || window.downgrade_behavior.trim().is_empty()
        || window.out_of_window_posture.trim().is_empty()
        || window.contract_rule.trim().is_empty()
    {
        findings.push(ExtensionBridgeMatrixFinding::row(
            &row.row_id,
            &format!("extension_bridge_matrix.{field}_incomplete"),
            format!(
                "{field} must name id, version range, support class, downgrade, posture, and rule"
            ),
        ));
    }
}
