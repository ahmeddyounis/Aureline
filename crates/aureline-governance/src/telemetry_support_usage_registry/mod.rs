//! Stable telemetry, support-export, and usage-export schema registry.
//!
//! This module embeds the checked-in registry at
//! `artifacts/governance/telemetry_support_usage_schema_registry.json` and
//! exposes a typed access and validation surface for the M4 governance
//! dimensions that were not present in the base [`crate::schema_registry`]:
//!
//! - [`EndpointPolicyTruth`]: operative endpoint policy per deployment context,
//!   making it mechanically answerable whether data stays local, is queued for
//!   manual export, flows to a managed endpoint, or is disabled by policy or
//!   build flavor.
//! - [`DeprecatedFieldHandling`]: closed policy for what readers do when they
//!   encounter deprecated or unknown schema fields.
//! - [`PartialOutcomeMarker`]: whether packets in a family can be partial due to
//!   policy suppression, offboarding window bounds, or manual reconciliation.
//! - Per-row offboarding compatibility notes, redaction profile references, and
//!   OSS exception packet provenance.
//!
//! The registry is the canonical answer for shiproom and release gates that
//! need to confirm every stable-emitted telemetry, diagnostics, support-export,
//! and usage-export payload has an owner, schema version, consent posture,
//! endpoint policy truth, and retention note.
//!
//! # Relationship to other modules
//!
//! Each [`TelemetrySupportUsageRow`] in this registry corresponds to a row in
//! [`crate::schema_registry::GovernedSchemaRegistry`] by `entry_id`. The base
//! registry provides schema identity, consent class, downgrade rules, and
//! surface visibility; this module adds the contextual endpoint-policy truth
//! and M4 governance dimensions. Both must agree on the same `entry_id`s.
//!
//! # Validation
//!
//! [`validate_registry`] returns [`RegistryViolation`]s rather than a
//! `Result`, so callers receive the complete violation set rather than failing
//! on the first error. Release and shiproom gates should treat any non-empty
//! violation set as a promotion blocker.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported registry schema version.
pub const TELEMETRY_SUPPORT_USAGE_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for this registry.
pub const TELEMETRY_SUPPORT_USAGE_REGISTRY_RECORD_KIND: &str =
    "telemetry_support_usage_schema_registry";

/// Repo-relative path to the checked-in registry artifact.
pub const TELEMETRY_SUPPORT_USAGE_REGISTRY_PATH: &str =
    "artifacts/governance/telemetry_support_usage_schema_registry.json";

/// Embedded checked-in registry JSON.
pub const TELEMETRY_SUPPORT_USAGE_REGISTRY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/telemetry_support_usage_schema_registry.json"
));

/// Deployment contexts for which endpoint policy truth must be declared.
pub const REQUIRED_CONTEXT_CLASSES: [&str; 3] = ["oss_local", "self_hosted", "managed_enterprise"];

/// Closed set of deployment contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextClass {
    /// OSS or local-only install with no managed entitlement.
    OssLocal,
    /// Self-hosted deployment operated by the customer rather than the vendor.
    SelfHosted,
    /// Vendor-operated managed enterprise deployment.
    ManagedEnterprise,
}

impl ContextClass {
    /// Every context class, in declaration order.
    pub const ALL: [Self; 3] = [Self::OssLocal, Self::SelfHosted, Self::ManagedEnterprise];

    /// Stable token recorded in the registry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OssLocal => "oss_local",
            Self::SelfHosted => "self_hosted",
            Self::ManagedEnterprise => "managed_enterprise",
        }
    }
}

/// Operative endpoint policy for one payload family in one deployment context.
///
/// This is the *default* policy that applies before any user consent action or
/// admin policy override. It answers whether data stays on the device, is held
/// for explicit export, flows to a managed endpoint, or is not active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointPolicyTruth {
    /// Data never leaves the device in this context; no upload path is active.
    LocalOnly,
    /// Data is held locally and only leaves on explicit user or admin action.
    QueuedForManualExport,
    /// Data flows to a vendor-operated managed plane in this context.
    ManagedEndpoint,
    /// The family is not active on this lane; no data is emitted or stored.
    DisabledByPolicyOrFlavor,
}

impl EndpointPolicyTruth {
    /// Every truth value, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::QueuedForManualExport,
        Self::ManagedEndpoint,
        Self::DisabledByPolicyOrFlavor,
    ];

    /// Stable token recorded in the registry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::QueuedForManualExport => "queued_for_manual_export",
            Self::ManagedEndpoint => "managed_endpoint",
            Self::DisabledByPolicyOrFlavor => "disabled_by_policy_or_flavor",
        }
    }
}

/// Policy for what readers do when they encounter deprecated or unknown schema fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprecatedFieldHandling {
    /// Drop the field silently on read without error.
    DropOnRead,
    /// Preserve the field value under an `unknown` marker for manual review.
    PreserveAsUnknown,
    /// Refuse to read the packet if it carries a deprecated field.
    RefuseRead,
    /// Surface the deprecated field to a human reviewer before any action is taken.
    RequireManualReview,
}

impl DeprecatedFieldHandling {
    /// Every handling policy, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DropOnRead,
        Self::PreserveAsUnknown,
        Self::RefuseRead,
        Self::RequireManualReview,
    ];

    /// Stable token recorded in the registry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DropOnRead => "drop_on_read",
            Self::PreserveAsUnknown => "preserve_as_unknown",
            Self::RefuseRead => "refuse_read",
            Self::RequireManualReview => "require_manual_review",
        }
    }
}

/// Whether packets in a family can be partial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialOutcomeMarker {
    /// Packets in this family are always complete; no partial-outcome path exists.
    None,
    /// Fields may be omitted under redaction or admin policy suppression.
    PartialSuppressedByPolicy,
    /// Completeness depends on an offboarding availability window.
    PartialOffboardingWindowBounded,
    /// Partial packets require human review to resolve completeness.
    RequiresManualReconciliation,
}

impl PartialOutcomeMarker {
    /// Every marker value, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::PartialSuppressedByPolicy,
        Self::PartialOffboardingWindowBounded,
        Self::RequiresManualReconciliation,
    ];

    /// Stable token recorded in the registry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PartialSuppressedByPolicy => "partial_suppressed_by_policy",
            Self::PartialOffboardingWindowBounded => "partial_offboarding_window_bounded",
            Self::RequiresManualReconciliation => "requires_manual_reconciliation",
        }
    }
}

/// Endpoint policy truth for one deployment context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextEndpointPolicyRow {
    /// The deployment context this row covers.
    pub context_class: ContextClass,
    /// Operative endpoint policy in this context.
    pub endpoint_policy_truth: EndpointPolicyTruth,
    /// Reviewable sentence explaining why this policy applies.
    pub note: String,
}

/// One stable telemetry, support-export, or usage-export registry row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetrySupportUsageRow {
    /// Stable id matching the corresponding entry in the base schema registry.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Payload family class.
    pub family_class: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Repo-relative schema file.
    pub schema_ref: String,
    /// Current schema version.
    pub schema_version: u32,
    /// Governed lifecycle state.
    pub lifecycle_state: String,
    /// Consent posture inherited from the base schema registry.
    pub consent_class: String,
    /// Endpoint type inherited from the base schema registry.
    pub endpoint_class: String,
    /// Reviewable retention note.
    pub retention_note: String,
    /// Reference to the redaction profile governing this family.
    pub redaction_profile_ref: String,
    /// Default posture for OSS and local builds.
    pub open_source_default_posture: String,
    /// Signed exception packet authorizing a narrower OSS posture. `None` when
    /// the default `opt_in_disabled_until_user_consent` posture applies.
    pub oss_telemetry_exception_packet_ref: Option<String>,
    /// Schema diff report reference. `None` for initial-version registrations;
    /// required before stable promotion when `schema_version` advances.
    pub schema_diff_report_ref: Option<String>,
    /// Endpoint policy truth per deployment context.
    pub endpoint_policy_truth_by_context: Vec<ContextEndpointPolicyRow>,
    /// Policy for deprecated or unknown schema fields.
    pub deprecated_field_handling: DeprecatedFieldHandling,
    /// Whether packets in this family can be partial.
    pub partial_outcome_marker: PartialOutcomeMarker,
    /// Reviewable note on offboarding behavior.
    pub offboarding_compatibility_note: String,
}

impl TelemetrySupportUsageRow {
    /// Returns the endpoint policy truth for `context`, if declared.
    pub fn endpoint_policy_for(&self, context: ContextClass) -> Option<EndpointPolicyTruth> {
        self.endpoint_policy_truth_by_context
            .iter()
            .find(|row| row.context_class == context)
            .map(|row| row.endpoint_policy_truth)
    }

    /// Returns `true` when the row is a telemetry family with a valid OSS
    /// opt-in default or a signed exception packet reference.
    pub fn has_valid_oss_telemetry_posture(&self) -> bool {
        if self.family_class != "telemetry_payload" {
            return true;
        }
        self.open_source_default_posture == "opt_in_disabled_until_user_consent"
            || self.oss_telemetry_exception_packet_ref.is_some()
    }
}

/// Summary counts for the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetrySupportUsageSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Rows carrying all required governance dimensions.
    pub labeled_rows: usize,
    /// Telemetry rows with `opt_in_disabled_until_user_consent` as OSS default.
    pub telemetry_rows_with_oss_opt_in_default: usize,
    /// Rows whose OSS-local endpoint policy truth is `local_only`.
    pub local_only_oss_context_rows: usize,
    /// Rows whose managed-enterprise endpoint policy truth is `managed_endpoint`.
    pub managed_endpoint_rows: usize,
    /// Rows that require explicit user or admin action to emit in any context.
    pub queued_manual_export_rows: usize,
}

/// The typed stable telemetry, support-export, and usage-export registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetrySupportUsageRegistry {
    /// Registry schema version.
    pub schema_version: u32,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Lifecycle status of this registry artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Governed schema registry this registry cross-checks against.
    pub governed_schema_registry_ref: String,
    /// Consent-ledger seed this registry extends.
    pub consent_ledger_ref: String,
    /// Closed context-class vocabulary.
    pub context_classes: Vec<ContextClass>,
    /// Closed endpoint-policy-truth vocabulary.
    pub endpoint_policy_truth_classes: Vec<EndpointPolicyTruth>,
    /// Closed deprecated-field-handling vocabulary.
    pub deprecated_field_handling_classes: Vec<DeprecatedFieldHandling>,
    /// Closed partial-outcome-marker vocabulary.
    pub partial_outcome_marker_classes: Vec<PartialOutcomeMarker>,
    /// Registry rows.
    pub rows: Vec<TelemetrySupportUsageRow>,
    /// Summary counts.
    pub summary: TelemetrySupportUsageSummary,
}

impl TelemetrySupportUsageRegistry {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&TelemetrySupportUsageRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns rows whose OSS-local endpoint policy truth is `local_only`.
    pub fn local_only_oss_rows(&self) -> Vec<&TelemetrySupportUsageRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.endpoint_policy_for(ContextClass::OssLocal)
                    == Some(EndpointPolicyTruth::LocalOnly)
            })
            .collect()
    }

    /// Returns rows that flow to a managed endpoint in the managed-enterprise context.
    pub fn managed_endpoint_rows(&self) -> Vec<&TelemetrySupportUsageRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.endpoint_policy_for(ContextClass::ManagedEnterprise)
                    == Some(EndpointPolicyTruth::ManagedEndpoint)
            })
            .collect()
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> TelemetrySupportUsageSummary {
        let labeled = self
            .rows
            .iter()
            .filter(|row| {
                !row.entry_id.is_empty()
                    && !row.owner_ref.is_empty()
                    && !row.retention_note.is_empty()
                    && !row.redaction_profile_ref.is_empty()
                    && !row.offboarding_compatibility_note.is_empty()
                    && row.endpoint_policy_truth_by_context.len() >= REQUIRED_CONTEXT_CLASSES.len()
            })
            .count();

        let telemetry_opt_in = self
            .rows
            .iter()
            .filter(|row| {
                row.family_class == "telemetry_payload"
                    && row.open_source_default_posture == "opt_in_disabled_until_user_consent"
            })
            .count();

        let local_only_oss = self
            .rows
            .iter()
            .filter(|row| {
                row.endpoint_policy_for(ContextClass::OssLocal)
                    == Some(EndpointPolicyTruth::LocalOnly)
            })
            .count();

        let managed = self
            .rows
            .iter()
            .filter(|row| {
                row.endpoint_policy_for(ContextClass::ManagedEnterprise)
                    == Some(EndpointPolicyTruth::ManagedEndpoint)
            })
            .count();

        let queued = self
            .rows
            .iter()
            .filter(|row| {
                row.endpoint_policy_truth_by_context.iter().any(|ctx| {
                    ctx.endpoint_policy_truth == EndpointPolicyTruth::QueuedForManualExport
                })
            })
            .count();

        TelemetrySupportUsageSummary {
            total_rows: self.rows.len(),
            labeled_rows: labeled,
            telemetry_rows_with_oss_opt_in_default: telemetry_opt_in,
            local_only_oss_context_rows: local_only_oss,
            managed_endpoint_rows: managed,
            queued_manual_export_rows: queued,
        }
    }
}

/// A validation violation for the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryViolation {
    /// The registry carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the registry.
        actual: u32,
    },
    /// The registry carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found.
        actual: String,
    },
    /// A closed vocabulary list does not match the canonical set.
    ClosedVocabularyMismatch {
        /// Field name.
        field: &'static str,
    },
    /// The registry has no rows.
    EmptyRegistry,
    /// An entry_id appears more than once.
    DuplicateEntryId {
        /// Duplicate id.
        entry_id: String,
    },
    /// A required field is empty or zero.
    EmptyField {
        /// Row id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A telemetry family row does not keep OSS builds opt-in.
    TelemetryOssDefaultNotOptIn {
        /// Row id.
        entry_id: String,
        /// Observed posture.
        posture: String,
    },
    /// A row is missing a required context entry in `endpoint_policy_truth_by_context`.
    MissingContextEndpointPolicy {
        /// Row id.
        entry_id: String,
        /// Missing context class token.
        context_class: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for RegistryViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version {actual}; expected {TELEMETRY_SUPPORT_USAGE_REGISTRY_SCHEMA_VERSION}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record_kind {actual:?}; expected {TELEMETRY_SUPPORT_USAGE_REGISTRY_RECORD_KIND:?}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch in field {field}")
            }
            Self::EmptyRegistry => write!(f, "registry has no rows"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry_id {entry_id:?}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => {
                write!(f, "row {entry_id:?} has empty required field {field_name}")
            }
            Self::TelemetryOssDefaultNotOptIn { entry_id, posture } => {
                write!(
                    f,
                    "telemetry row {entry_id:?} has non-opt-in OSS default {posture:?}; a signed exception packet is required"
                )
            }
            Self::MissingContextEndpointPolicy {
                entry_id,
                context_class,
            } => {
                write!(
                    f,
                    "row {entry_id:?} is missing endpoint policy truth for context {context_class:?}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "summary counts disagree with computed values")
            }
        }
    }
}

impl Error for RegistryViolation {}

/// Error type for registry loading failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryLoadError {
    /// JSON error message.
    pub message: String,
}

impl fmt::Display for RegistryLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to parse {TELEMETRY_SUPPORT_USAGE_REGISTRY_PATH}: {}",
            self.message
        )
    }
}

impl Error for RegistryLoadError {}

/// Loads the embedded registry.
pub fn load_registry() -> Result<TelemetrySupportUsageRegistry, RegistryLoadError> {
    serde_json::from_str(TELEMETRY_SUPPORT_USAGE_REGISTRY_JSON).map_err(|err| RegistryLoadError {
        message: err.to_string(),
    })
}

/// Returns the embedded registry, panicking if the embedded JSON is malformed.
///
/// This is infallible in correct builds: the registry JSON is validated at test
/// time by [`validate_registry`]. Call sites that need a `Result` should use
/// [`load_registry`] instead.
pub fn current_registry() -> TelemetrySupportUsageRegistry {
    load_registry().expect("embedded telemetry_support_usage_schema_registry.json is malformed")
}

/// Validates the registry, returning every violation found.
///
/// Returns an empty `Vec` when the registry is fully compliant. Release and
/// shiproom gates should treat any non-empty result as a promotion blocker.
pub fn validate_registry(registry: &TelemetrySupportUsageRegistry) -> Vec<RegistryViolation> {
    let mut violations = Vec::new();

    if registry.schema_version != TELEMETRY_SUPPORT_USAGE_REGISTRY_SCHEMA_VERSION {
        violations.push(RegistryViolation::UnsupportedSchemaVersion {
            actual: registry.schema_version,
        });
    }
    if registry.record_kind != TELEMETRY_SUPPORT_USAGE_REGISTRY_RECORD_KIND {
        violations.push(RegistryViolation::UnsupportedRecordKind {
            actual: registry.record_kind.clone(),
        });
    }
    if registry.governed_schema_registry_ref
        != crate::schema_registry::GOVERNED_SCHEMA_REGISTRY_PATH
    {
        violations.push(RegistryViolation::ClosedVocabularyMismatch {
            field: "governed_schema_registry_ref",
        });
    }

    if registry.context_classes != ContextClass::ALL.to_vec() {
        violations.push(RegistryViolation::ClosedVocabularyMismatch {
            field: "context_classes",
        });
    }
    if registry.endpoint_policy_truth_classes != EndpointPolicyTruth::ALL.to_vec() {
        violations.push(RegistryViolation::ClosedVocabularyMismatch {
            field: "endpoint_policy_truth_classes",
        });
    }
    if registry.deprecated_field_handling_classes != DeprecatedFieldHandling::ALL.to_vec() {
        violations.push(RegistryViolation::ClosedVocabularyMismatch {
            field: "deprecated_field_handling_classes",
        });
    }
    if registry.partial_outcome_marker_classes != PartialOutcomeMarker::ALL.to_vec() {
        violations.push(RegistryViolation::ClosedVocabularyMismatch {
            field: "partial_outcome_marker_classes",
        });
    }

    if registry.rows.is_empty() {
        violations.push(RegistryViolation::EmptyRegistry);
        return violations;
    }

    let mut seen_ids = BTreeSet::new();
    for row in &registry.rows {
        for (field, value) in [
            ("entry_id", row.entry_id.as_str()),
            ("title", row.title.as_str()),
            ("family_class", row.family_class.as_str()),
            ("owner_ref", row.owner_ref.as_str()),
            ("schema_ref", row.schema_ref.as_str()),
            ("lifecycle_state", row.lifecycle_state.as_str()),
            ("consent_class", row.consent_class.as_str()),
            ("endpoint_class", row.endpoint_class.as_str()),
            ("retention_note", row.retention_note.as_str()),
            ("redaction_profile_ref", row.redaction_profile_ref.as_str()),
            (
                "open_source_default_posture",
                row.open_source_default_posture.as_str(),
            ),
            (
                "offboarding_compatibility_note",
                row.offboarding_compatibility_note.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(RegistryViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.schema_version == 0 {
            violations.push(RegistryViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "schema_version",
            });
        }

        if !seen_ids.insert(row.entry_id.clone()) {
            violations.push(RegistryViolation::DuplicateEntryId {
                entry_id: row.entry_id.clone(),
            });
        }

        if !row.has_valid_oss_telemetry_posture() {
            violations.push(RegistryViolation::TelemetryOssDefaultNotOptIn {
                entry_id: row.entry_id.clone(),
                posture: row.open_source_default_posture.clone(),
            });
        }

        for context in REQUIRED_CONTEXT_CLASSES {
            let covered = row
                .endpoint_policy_truth_by_context
                .iter()
                .any(|ctx| ctx.context_class.as_str() == context);
            if !covered {
                violations.push(RegistryViolation::MissingContextEndpointPolicy {
                    entry_id: row.entry_id.clone(),
                    context_class: context,
                });
            }
        }
    }

    if registry.summary != registry.computed_summary() {
        violations.push(RegistryViolation::SummaryMismatch);
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_parses_and_validates() {
        let registry = current_registry();
        let violations = validate_registry(&registry);
        assert!(
            violations.is_empty(),
            "embedded registry has violations: {violations:#?}"
        );
    }

    #[test]
    fn all_rows_have_required_governance_dimensions() {
        let registry = current_registry();
        for row in &registry.rows {
            assert!(
                !row.owner_ref.is_empty(),
                "row {} missing owner_ref",
                row.entry_id
            );
            assert!(
                row.schema_version > 0,
                "row {} has zero schema_version",
                row.entry_id
            );
            assert!(
                !row.consent_class.is_empty(),
                "row {} missing consent_class",
                row.entry_id
            );
            assert!(
                !row.endpoint_class.is_empty(),
                "row {} missing endpoint_class",
                row.entry_id
            );
            assert!(
                !row.retention_note.is_empty(),
                "row {} missing retention_note",
                row.entry_id
            );
            assert!(
                !row.redaction_profile_ref.is_empty(),
                "row {} missing redaction_profile_ref",
                row.entry_id
            );
            assert!(
                !row.offboarding_compatibility_note.is_empty(),
                "row {} missing offboarding_compatibility_note",
                row.entry_id
            );
        }
    }

    #[test]
    fn all_rows_have_full_context_coverage() {
        let registry = current_registry();
        for row in &registry.rows {
            for context in ContextClass::ALL {
                assert!(
                    row.endpoint_policy_for(context).is_some(),
                    "row {} missing endpoint policy for context {:?}",
                    row.entry_id,
                    context
                );
            }
        }
    }

    #[test]
    fn telemetry_rows_stay_opt_in_on_oss() {
        let registry = current_registry();
        for row in registry
            .rows
            .iter()
            .filter(|r| r.family_class == "telemetry_payload")
        {
            assert!(
                row.has_valid_oss_telemetry_posture(),
                "telemetry row {} does not maintain opt-in OSS posture",
                row.entry_id
            );
        }
    }

    #[test]
    fn summary_matches_computed_values() {
        let registry = current_registry();
        assert_eq!(
            registry.summary,
            registry.computed_summary(),
            "embedded summary does not match computed summary"
        );
    }

    #[test]
    fn telemetry_oss_local_is_local_only() {
        let registry = current_registry();
        let row = registry
            .row("telemetry.ux_product_event")
            .expect("telemetry row missing");
        assert_eq!(
            row.endpoint_policy_for(ContextClass::OssLocal),
            Some(EndpointPolicyTruth::LocalOnly),
            "telemetry OSS-local endpoint policy must be local_only"
        );
    }

    #[test]
    fn usage_export_managed_is_managed_endpoint() {
        let registry = current_registry();
        let row = registry
            .row("usage.metering_export_packet")
            .expect("usage export row missing");
        assert_eq!(
            row.endpoint_policy_for(ContextClass::ManagedEnterprise),
            Some(EndpointPolicyTruth::ManagedEndpoint),
            "usage export managed-enterprise endpoint policy must be managed_endpoint"
        );
        assert_eq!(
            row.endpoint_policy_for(ContextClass::OssLocal),
            Some(EndpointPolicyTruth::DisabledByPolicyOrFlavor),
            "usage export OSS-local endpoint policy must be disabled"
        );
    }

    #[test]
    fn support_bundle_is_queued_for_manual_export_in_all_contexts() {
        let registry = current_registry();
        let row = registry
            .row("support.bundle_manifest")
            .expect("support row missing");
        for context in ContextClass::ALL {
            assert_eq!(
                row.endpoint_policy_for(context),
                Some(EndpointPolicyTruth::QueuedForManualExport),
                "support bundle must be queued_for_manual_export in context {:?}",
                context
            );
        }
    }

    #[test]
    fn cli_diagnostic_is_local_only_in_all_contexts() {
        let registry = current_registry();
        let row = registry
            .row("cli.headless_diagnostic_payload")
            .expect("cli row missing");
        for context in ContextClass::ALL {
            assert_eq!(
                row.endpoint_policy_for(context),
                Some(EndpointPolicyTruth::LocalOnly),
                "CLI diagnostic must be local_only in context {:?}",
                context
            );
        }
    }
}
