//! Shared security-finding card contract for review, package, health,
//! companion, support, and release surfaces.
//!
//! A security-finding card is the cross-surface object that tells users what
//! kind of finding exists, how severe it is, how confident Aureline is, what
//! code or artifact scope is affected, whether a fix exists, whether a
//! suppression is active, and what the safest next step is. Package-manager
//! views, review panes, project-health centers, framework-pack health bundles,
//! companion clients, support exports, and release proof ingest this object
//! instead of reducing findings to one generic warning.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SecurityFindingCard`].
pub const M5_SECURITY_FINDING_CARD_RECORD_KIND: &str = "m5_security_finding_card";

/// Schema version for the security finding card.
pub const M5_SECURITY_FINDING_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_SECURITY_FINDING_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-security-finding-card.schema.json";

/// Repo-relative path of the canonical first-consumer fixture.
pub const M5_SECURITY_FINDING_CARD_FIXTURE_REF: &str =
    "fixtures/ui/m5-pipeline-dependency-finding-components/security_finding_card.json";

/// Reusable security-finding card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityFindingCard {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id reused by all consumers.
    pub card_id: String,
    /// Canonical finding id.
    pub finding_id: String,
    /// Package, secret, policy, code-analysis, artifact, install, or advisory class.
    pub finding_class: String,
    /// Canonical security result packet.
    pub security_result_packet_ref: String,
    /// Advisory or advisory-like source ref.
    pub advisory_ref: String,
    /// Primary affected object ref.
    pub affected_object_ref: String,
    /// Affected artifact, code, manifest, policy, and package scope.
    pub affected_scope: AffectedScope,
    /// User-visible severity.
    pub severity: String,
    /// User-visible confidence.
    pub confidence: String,
    /// User-visible freshness.
    pub freshness_state: String,
    /// Narrowed/degraded state.
    pub degraded_state: String,
    /// Fix availability, separate from remediation.
    pub fix_availability: FixAvailability,
    /// Suppression state that remains visible.
    pub suppression_state: SuppressionState,
    /// Remediation path.
    pub remediation: Remediation,
    /// Exposure summary.
    pub exposure_summary: ExposureSummary,
    /// Audit/export actions available from the card.
    pub audit_actions: Vec<AuditAction>,
    /// Source documents/schemas.
    pub source_refs: Vec<String>,
    /// Consumers that must render this same card contract.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payloads.
    pub copy_export: CopyExport,
}

/// Affected scope for a security finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedScope {
    /// Package, secret, policy, code-analysis, artifact, install, or workspace scope.
    pub scope_class: String,
    /// Export-safe scope label ref.
    pub scope_label_ref: String,
    /// Affected build/package/runtime artifact refs.
    pub affected_artifact_refs: Vec<String>,
    /// Affected code-anchor refs.
    pub code_anchor_refs: Vec<String>,
    /// Affected manifest refs.
    pub manifest_refs: Vec<String>,
    /// Affected policy refs.
    pub policy_refs: Vec<String>,
    /// Affected package refs.
    pub package_refs: Vec<String>,
}

/// Fix availability, distinct from remediation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixAvailability {
    /// Fix availability state.
    pub state: String,
    /// Fixed version ref, or `version:none`.
    pub fixed_version_ref: String,
    /// Fix action ref.
    pub fix_action_ref: String,
    /// Whether the fix can be applied automatically.
    pub can_auto_apply: bool,
    /// Whether review is required before fix or mitigation.
    pub review_required: bool,
    /// Reason ref explaining the availability state.
    pub availability_reason_ref: String,
}

/// Suppression state that remains visible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionState {
    /// Controlled suppression state.
    pub state: String,
    /// Controlled user-visible suppression label.
    pub display_label: String,
    /// Reason ref.
    pub reason_ref: String,
    /// Review ref.
    pub review_ref: String,
    /// Must remain true so support exports do not lose suppression truth.
    pub visible_in_export: bool,
}

/// Remediation path for the security finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Remediation {
    /// Safest next step.
    pub safest_next_step: String,
    /// Local validation option.
    pub local_validation: LocalValidation,
    /// Docs/help/support path.
    pub docs_help_path: DocsHelpPath,
    /// Fixed version or action.
    pub fix_version_or_action: String,
    /// Mitigation ref.
    pub mitigation_ref: String,
    /// Owner ref.
    pub owner_ref: String,
    /// Whether no fixed version or action exists yet.
    pub no_fix_yet: bool,
    /// Blocked reason, or `none`.
    pub blocked_reason: String,
}

/// Local validation route for remediation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalValidation {
    /// Whether local validation is available.
    pub available: bool,
    /// Validation action ref.
    pub validation_action_ref: String,
    /// Expected evidence ref.
    pub expected_evidence_ref: String,
}

/// Help and support path for remediation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsHelpPath {
    /// Help topic ref.
    pub help_ref: String,
    /// Documentation ref.
    pub docs_ref: String,
    /// Support path ref.
    pub support_path_ref: String,
}

/// Exposure summary for a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposureSummary {
    /// Exposure state.
    pub exposure_state: String,
    /// Affected user-facing surface refs.
    pub affected_surface_refs: Vec<String>,
    /// Exploitability label.
    pub exploitability_label: String,
    /// Affected manifest refs.
    pub affected_manifest_refs: Vec<String>,
}

/// Audit or export action available from a finding card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditAction {
    /// Stable action id.
    pub action_id: String,
    /// Export audit record, support export, evidence copy, or attach-to-review.
    pub label: String,
    /// Action target ref.
    pub target_ref: String,
    /// Whether the action is currently enabled.
    pub enabled: bool,
    /// Must remain true so support export has audit parity.
    pub included_in_support_export: bool,
}

/// Export-safe text, JSON, and Markdown copies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExport {
    /// Available formats.
    pub formats: Vec<String>,
    /// Fields intentionally preserved in export.
    pub export_fields: Vec<String>,
    /// Plain text copy.
    pub text: String,
    /// JSON copy.
    pub json: String,
    /// Markdown copy.
    pub markdown: String,
    /// Must remain true; screenshot-only reconstruction is forbidden.
    pub screenshot_only_prohibited: bool,
}

/// Minimal projection consumed by package, review, health, companion, support,
/// and release surfaces. Every projection is derived from one
/// [`SecurityFindingCard`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityFindingSurfaceProjection {
    /// Consumer surface name.
    pub consumer_surface: String,
    /// Stable card id.
    pub card_id: String,
    /// Canonical finding id.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: String,
    /// Scope class.
    pub scope_class: String,
    /// Severity.
    pub severity: String,
    /// Confidence.
    pub confidence: String,
    /// Freshness state.
    pub freshness_state: String,
    /// Fix availability state.
    pub fix_availability_state: String,
    /// Suppression state.
    pub suppression_state: String,
    /// Controlled suppression display label.
    pub suppression_display_label: String,
    /// Safest next step.
    pub safest_next_step: String,
    /// Whether local validation is available.
    pub local_validation_available: bool,
    /// First audit action label.
    pub primary_audit_action: String,
}

impl SecurityFindingCard {
    /// Returns all validation violations for this card.
    #[must_use]
    pub fn validate(&self) -> Vec<SecurityFindingCardViolation> {
        use SecurityFindingCardViolation as V;

        let mut violations = Vec::new();
        if self.record_kind != M5_SECURITY_FINDING_CARD_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_SECURITY_FINDING_CARD_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if any_blank([
            &self.card_id,
            &self.finding_id,
            &self.finding_class,
            &self.security_result_packet_ref,
            &self.advisory_ref,
            &self.affected_object_ref,
            &self.severity,
            &self.confidence,
            &self.freshness_state,
            &self.degraded_state,
        ]) {
            violations.push(V::MissingIdentity);
        }
        if !allowed(
            &self.finding_class,
            [
                "package",
                "secret",
                "policy",
                "code_analysis",
                "pipeline_artifact",
                "install_surface",
                "advisory",
            ],
        ) {
            violations.push(V::UnknownFindingClass);
        }
        if any_blank([
            &self.affected_scope.scope_class,
            &self.affected_scope.scope_label_ref,
        ]) || !scope_has_any_ref(&self.affected_scope)
        {
            violations.push(V::MissingAffectedScope);
        }
        if !allowed(
            &self.fix_availability.state,
            [
                "fix_available",
                "mitigation_available",
                "no_fix_yet",
                "blocked_by_policy",
                "unknown",
            ],
        ) || any_blank([
            &self.fix_availability.fixed_version_ref,
            &self.fix_availability.fix_action_ref,
            &self.fix_availability.availability_reason_ref,
        ]) {
            violations.push(V::MissingFixAvailability);
        }
        if self.fix_availability.state == "no_fix_yet"
            && (!self.remediation.no_fix_yet
                || self.fix_availability.can_auto_apply
                || self.fix_availability.fixed_version_ref != "version:none")
        {
            violations.push(V::NoFixYetContradiction);
        }
        if self.fix_availability.state == "fix_available"
            && (self.remediation.no_fix_yet
                || self.fix_availability.fixed_version_ref == "version:none")
        {
            violations.push(V::NoFixYetContradiction);
        }
        if !self.suppression_state.visible_in_export
            || suppression_label_for(&self.suppression_state.state)
                != Some(self.suppression_state.display_label.as_str())
            || any_blank([
                &self.suppression_state.reason_ref,
                &self.suppression_state.review_ref,
            ])
        {
            violations.push(V::SuppressionLabelOrExportMissing);
        }
        if !allowed(
            &self.remediation.safest_next_step,
            [
                "apply_fix",
                "apply_mitigation",
                "open_review",
                "request_policy_exception",
                "wait_for_upstream_fix",
                "rotate_secret",
                "inspect_only",
            ],
        ) || any_blank([
            &self.remediation.fix_version_or_action,
            &self.remediation.mitigation_ref,
            &self.remediation.owner_ref,
            &self.remediation.blocked_reason,
            &self.remediation.docs_help_path.help_ref,
            &self.remediation.docs_help_path.docs_ref,
            &self.remediation.docs_help_path.support_path_ref,
        ]) {
            violations.push(V::MissingRemediationPath);
        }
        if self.remediation.local_validation.available
            && any_blank([
                &self.remediation.local_validation.validation_action_ref,
                &self.remediation.local_validation.expected_evidence_ref,
            ])
        {
            violations.push(V::MissingLocalValidation);
        }
        if self.audit_actions.is_empty()
            || self
                .audit_actions
                .iter()
                .any(|action| !action.included_in_support_export || !action.enabled)
        {
            violations.push(V::AuditActionsMissingFromExport);
        }
        if self.audit_actions.iter().any(|action| {
            any_blank([&action.action_id, &action.label, &action.target_ref])
                || !allowed(
                    &action.label,
                    [
                        "export_audit_record",
                        "open_support_export",
                        "copy_evidence",
                        "attach_to_review",
                    ],
                )
        }) {
            violations.push(V::AuditActionsMissingFromExport);
        }
        if self.source_refs.is_empty() || self.consumer_surfaces.is_empty() {
            violations.push(V::MissingSourceOrConsumer);
        }
        if !has_all(
            &self.consumer_surfaces,
            [
                "review_pane",
                "package_manager",
                "project_health_center",
                "support_export",
            ],
        ) {
            violations.push(V::CoreConsumerSurfaceMissing);
        }
        if !has_all(
            &self.consumer_surfaces,
            ["framework_pack_health", "companion_client", "release_proof"],
        ) {
            violations.push(V::ProofConsumerSurfaceMissing);
        }
        if !has_all(&self.copy_export.formats, ["text", "json", "markdown"])
            || !self.copy_export.screenshot_only_prohibited
        {
            violations.push(V::CopyExportIncomplete);
        }
        if !has_all(
            &self.copy_export.export_fields,
            [
                "finding_class",
                "affected_scope",
                "fix_availability",
                "suppression_state",
                "remediation",
                "audit_actions",
            ],
        ) {
            violations.push(V::CopyExportDropsFindingTruth);
        }
        let copy_blob = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            &self.finding_id,
            &self.finding_class,
            &self.affected_scope.scope_label_ref,
            &self.severity,
            &self.confidence,
            &self.freshness_state,
            &self.fix_availability.state,
            &self.suppression_state.display_label,
            &self.remediation.safest_next_step,
            &self.remediation.local_validation.validation_action_ref,
            &self.remediation.docs_help_path.docs_ref,
            &self.audit_actions[0].label,
        ] {
            if !copy_blob.contains(required) {
                violations.push(V::CopyExportDropsFindingTruth);
                break;
            }
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("security finding card serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Projects this card to a named consumer surface using the same field values.
    #[must_use]
    pub fn projection_for(
        &self,
        consumer_surface: &str,
    ) -> Option<SecurityFindingSurfaceProjection> {
        if !self
            .consumer_surfaces
            .iter()
            .any(|surface| surface == consumer_surface)
        {
            return None;
        }
        Some(SecurityFindingSurfaceProjection {
            consumer_surface: consumer_surface.to_owned(),
            card_id: self.card_id.clone(),
            finding_id: self.finding_id.clone(),
            finding_class: self.finding_class.clone(),
            scope_class: self.affected_scope.scope_class.clone(),
            severity: self.severity.clone(),
            confidence: self.confidence.clone(),
            freshness_state: self.freshness_state.clone(),
            fix_availability_state: self.fix_availability.state.clone(),
            suppression_state: self.suppression_state.state.clone(),
            suppression_display_label: self.suppression_state.display_label.clone(),
            safest_next_step: self.remediation.safest_next_step.clone(),
            local_validation_available: self.remediation.local_validation.available,
            primary_audit_action: self
                .audit_actions
                .first()
                .map(|action| action.label.clone())
                .unwrap_or_default(),
        })
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only card fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("security finding card serializes")
    }
}

/// Error returned when the checked-in security finding fixture fails to load.
#[derive(Debug)]
pub enum SecurityFindingCardArtifactError {
    /// The fixture could not be parsed.
    Fixture(serde_json::Error),
    /// The parsed card failed validation.
    Validation(Vec<SecurityFindingCardViolation>),
}

impl fmt::Display for SecurityFindingCardArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(err) => write!(f, "security finding fixture parse error: {err}"),
            Self::Validation(violations) => {
                write!(
                    f,
                    "security finding fixture failed validation: {violations:?}"
                )
            }
        }
    }
}

impl Error for SecurityFindingCardArtifactError {}

/// A validation invariant a security finding card can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityFindingCardViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// Card identity is incomplete.
    MissingIdentity,
    /// Finding class is outside the controlled vocabulary.
    UnknownFindingClass,
    /// Affected scope is incomplete.
    MissingAffectedScope,
    /// Fix availability is incomplete.
    MissingFixAvailability,
    /// No-fix-yet state contradicts fix/remediation fields.
    NoFixYetContradiction,
    /// Suppression label is wrong or hidden from export.
    SuppressionLabelOrExportMissing,
    /// Remediation path is incomplete.
    MissingRemediationPath,
    /// Local validation route is incomplete.
    MissingLocalValidation,
    /// Audit actions are missing or not included in support export.
    AuditActionsMissingFromExport,
    /// Source refs or consumer surfaces are missing.
    MissingSourceOrConsumer,
    /// Review, package, project-health, or support export consumer is missing.
    CoreConsumerSurfaceMissing,
    /// Framework-pack, companion, or release proof consumer is missing.
    ProofConsumerSurfaceMissing,
    /// Copy/export formats are incomplete.
    CopyExportIncomplete,
    /// Copy/export drops finding class, scope, fix, suppression, remediation, or audit truth.
    CopyExportDropsFindingTruth,
    /// Raw credential, provider body, advisory body, or local path crossed the export boundary.
    RawBoundaryMaterialInExport,
}

/// Loads and validates the checked-in canonical security finding fixture.
///
/// # Errors
///
/// Returns an error if the checked-in fixture cannot be parsed or fails
/// validation.
pub fn current_m5_security_finding_card(
) -> Result<SecurityFindingCard, SecurityFindingCardArtifactError> {
    let card: SecurityFindingCard = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-pipeline-dependency-finding-components/security_finding_card.json"
    )))
    .map_err(SecurityFindingCardArtifactError::Fixture)?;
    let violations = card.validate();
    if violations.is_empty() {
        Ok(card)
    } else {
        Err(SecurityFindingCardArtifactError::Validation(violations))
    }
}

fn any_blank<'a>(values: impl IntoIterator<Item = &'a String>) -> bool {
    values.into_iter().any(|value| value.trim().is_empty())
}

fn allowed<'a>(value: &str, allowed_values: impl IntoIterator<Item = &'a str>) -> bool {
    allowed_values.into_iter().any(|allowed| allowed == value)
}

fn has_all<'a>(actual: &[String], required: impl IntoIterator<Item = &'a str>) -> bool {
    let actual: BTreeSet<&str> = actual.iter().map(String::as_str).collect();
    required.into_iter().all(|value| actual.contains(value))
}

fn scope_has_any_ref(scope: &AffectedScope) -> bool {
    [
        &scope.affected_artifact_refs,
        &scope.code_anchor_refs,
        &scope.manifest_refs,
        &scope.policy_refs,
        &scope.package_refs,
    ]
    .iter()
    .any(|refs| refs.iter().any(|value| !value.trim().is_empty()))
}

fn suppression_label_for(state: &str) -> Option<&'static str> {
    match state {
        "unsuppressed" => Some("Unsuppressed"),
        "suppressed_until_review" => Some("Suppressed until review"),
        "suppressed_by_policy" => Some("Suppressed by policy"),
        "exception_expired" => Some("Exception expired"),
        _ => None,
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("raw advisory")
                || lower.contains("raw provider")
                || lower.contains("raw exploit")
                || lower.contains("raw secret")
                || lower.contains("/users/")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
