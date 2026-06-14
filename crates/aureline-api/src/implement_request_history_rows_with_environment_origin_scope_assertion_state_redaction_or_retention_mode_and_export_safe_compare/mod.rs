//! Request-history rows with environment, origin scope, assertion state,
//! redaction or retention mode, and export-safe compare records.
//!
//! This module owns the typed records that upgrade request history from a
//! convenience replay log into a governed object model. Each history row keeps
//! the execution timestamp, the named environment, the origin scope (local,
//! remote, container, managed-workspace, or browser-companion), the
//! status/result class, the aggregate assertion state, the retention mode, and
//! the redaction posture inspectable across the request-history panel, the
//! companion and managed history surfaces, the compare view, the retention
//! settings, CLI/headless output, support export, and Help/About surfaces.
//! Metadata-only retention is the safe default; storing redacted-replayable or
//! full payloads, results, or headers requires an explicit, reviewed retention
//! selection with a declared redaction posture rather than a convenience toggle.
//! Compare stays export-safe: it operates on what was already retained safely
//! and never widens retention toward unsafe body or header capture, and history
//! rows and export packets never drop origin or environment identity.
//!
//! These records reuse the canonical frozen vocabulary
//! ([`ContractKind`], [`RequestOriginKind`], [`RequestOriginDriftState`],
//! [`RetentionMode`]) and reference the
//! [`freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix`](crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix)
//! packet as a verified upstream truth, the named-environment vocabulary
//! ([`EnvironmentClass`]) from the request-list views lane, the
//! [`AssertionOutcome`] vocabulary from the response-viewer lane, and the
//! [`ExportRedactionClass`] vocabulary from the composer history and
//! redaction-safe export lane, rather than minting a local synonym set.
//!
//! Raw request bodies, raw response bodies, raw headers, raw cookies, raw
//! secrets, and raw credential values do not belong in these records. History
//! rows carry opaque, non-secret identity refs, closed posture vocabularies, and
//! reviewable summaries that UI, CLI, export, support, and public-proof surfaces
//! can ingest safely. Unsafe full-body retention is never the path of least
//! resistance; browser-companion and managed origins never inherit
//! desktop-local trust or naming assumptions; and origin/environment identity is
//! never dropped from a history row or an export packet.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    ContractKind, RequestOriginDriftState, RequestOriginKind, RetentionMode,
    API_MATRIX_QUALIFICATION_RECORD_KIND,
};
use crate::implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges::EnvironmentClass;
use crate::implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export::ExportRedactionClass;
use crate::ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes::AssertionOutcome;

/// Supported schema version for request-history qualification packets.
pub const REQUEST_HISTORY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RequestHistoryQualificationPacket`].
pub const REQUEST_HISTORY_QUALIFICATION_RECORD_KIND: &str =
    "implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare";

/// Repo-relative path to the checked-in request-history packet.
pub const REQUEST_HISTORY_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json";

/// Embedded checked-in packet JSON.
pub const REQUEST_HISTORY_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json"
));

/// Qualification label shown on promoted request-history surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestHistoryQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl RequestHistoryQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Request-history consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestHistorySurfaceKind {
    /// Request-history panel inside a request workspace.
    RequestHistoryPanel,
    /// Browser-companion request-history surface.
    CompanionHistory,
    /// Managed-workspace request-history surface.
    ManagedHistory,
    /// Compare view that diffs two history rows.
    CompareView,
    /// Retention settings where the retention mode and redaction posture are chosen.
    RetentionSettings,
    /// CLI or headless request-history output.
    CliHeadlessOutput,
    /// Support-export bundle carrying request-history truth.
    SupportExport,
    /// Help/About surface describing the request-history contract.
    HelpAbout,
}

/// Status/result class for a single history row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryResultClass {
    /// Request completed with a success status.
    Success,
    /// Request completed with a redirection status.
    Redirected,
    /// Request completed with a client-error status.
    ClientError,
    /// Request completed with a server-error status.
    ServerError,
    /// Request failed at the transport layer (DNS, TCP, or TLS).
    TransportError,
    /// Request was blocked by policy before completion.
    Blocked,
    /// Request timed out before completion.
    TimedOut,
    /// Request was cancelled by the operator.
    Cancelled,
}

impl HistoryResultClass {
    /// Returns true when the row shows a clean success that must not mask risk
    /// behind a green status.
    pub const fn is_clean_success(self) -> bool {
        matches!(self, Self::Success)
    }
}

/// Aggregate assertion state for a history row.
///
/// This aggregate maps onto the canonical [`AssertionOutcome`] vocabulary via
/// [`AssertionStateClass::canonical_outcome`] so the history lane never mints a
/// divergent per-assertion vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionStateClass {
    /// No assertions were attached to the request.
    NoAssertions,
    /// Every attached assertion passed.
    AllPassed,
    /// Some assertions passed and some failed.
    MixedResults,
    /// At least one assertion failed and none passed.
    AnyFailed,
    /// Assertions were attached but not evaluated (skipped, blocked, or timed out).
    NotEvaluated,
}

impl AssertionStateClass {
    /// Returns true when the aggregate represents a failing or mixed result that
    /// must not be hidden behind a passing status.
    pub const fn is_failing(self) -> bool {
        matches!(self, Self::MixedResults | Self::AnyFailed)
    }

    /// Returns the canonical [`AssertionOutcome`] this aggregate resolves under,
    /// keeping the aggregate aligned with the response-viewer vocabulary.
    /// Returns [`None`] when no assertions were attached.
    pub const fn canonical_outcome(self) -> Option<AssertionOutcome> {
        match self {
            Self::NoAssertions => None,
            Self::AllPassed => Some(AssertionOutcome::Pass),
            Self::MixedResults | Self::AnyFailed => Some(AssertionOutcome::Fail),
            Self::NotEvaluated => Some(AssertionOutcome::Skipped),
        }
    }
}

/// Redaction posture applied when a history row stores more than metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPostureClass {
    /// Bodies and headers are not stored; only metadata is kept and all values
    /// are effectively redacted.
    RedactAll,
    /// Bodies are stored with secrets and sensitive fields redacted for replay.
    RedactSecrets,
    /// Bodies are stored unredacted but kept local-only and never exported.
    NoRedactionLocalOnly,
}

impl RedactionPostureClass {
    /// Returns true when an export may carry content stored under this posture.
    pub const fn permits_export(self) -> bool {
        !matches!(self, Self::NoRedactionLocalOnly)
    }
}

/// What a compare row diffs across two history rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareBasisClass {
    /// Status and timing metadata only.
    StatusAndTiming,
    /// Redacted, already-retained response bodies.
    RedactedBodies,
    /// Assertion results.
    AssertionResults,
    /// Header metadata (non-secret keys and shapes).
    HeaderMetadata,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistoryQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable request-history surfaces honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistorySurfaceGuardSet {
    /// The execution timestamp is visible.
    pub timestamp_visible: bool,
    /// The named environment is visible.
    pub environment_visible: bool,
    /// The origin scope and drift are visible.
    pub origin_scope_visible: bool,
    /// The status/result class is visible.
    pub result_class_visible: bool,
    /// The aggregate assertion state is visible.
    pub assertion_state_visible: bool,
    /// The retention mode and redaction posture are visible.
    pub retention_mode_visible: bool,
    /// The compare action is visible.
    pub compare_action_visible: bool,
    /// The export action and its redaction posture are visible.
    pub export_action_visible: bool,
    /// Metadata-only retention is the safe default.
    pub metadata_only_default: bool,
    /// Storing full payloads/results/headers requires an explicit reviewed selection.
    pub full_capture_requires_review: bool,
    /// Origin and environment identity are retained in rows and export packets.
    pub origin_environment_identity_retained: bool,
    /// Browser-companion and managed origins are isolated from desktop-local trust.
    pub origin_trust_isolated: bool,
    /// Compare never widens history toward unsafe body or header retention.
    pub no_unsafe_retention_for_compare: bool,
}

impl RequestHistorySurfaceGuardSet {
    /// Returns true when every required guard is present.
    pub const fn all_visible(&self) -> bool {
        self.timestamp_visible
            && self.environment_visible
            && self.origin_scope_visible
            && self.result_class_visible
            && self.assertion_state_visible
            && self.retention_mode_visible
            && self.compare_action_visible
            && self.export_action_visible
            && self.metadata_only_default
            && self.full_capture_requires_review
            && self.origin_environment_identity_retained
            && self.origin_trust_isolated
            && self.no_unsafe_retention_for_compare
    }
}

/// One governed request-history consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistorySurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: RequestHistorySurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: RequestHistoryQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: RequestHistoryQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<RequestHistoryQualificationProof>,
    /// Visible guard set.
    pub guards: RequestHistorySurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One request-history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistoryRow {
    /// Stable history row id.
    pub history_row_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Opaque request identity ref (not raw request text).
    pub request_identity_ref: String,
    /// Execution timestamp label.
    pub executed_at: String,
    /// Named environment the request ran in.
    pub environment: EnvironmentClass,
    /// Origin scope the request resolved to.
    pub origin_scope: RequestOriginKind,
    /// Whether the origin drifted from its last resolved target.
    pub origin_drift_state: RequestOriginDriftState,
    /// Contract family of the request.
    pub contract_kind: ContractKind,
    /// Status/result class.
    pub result_class: HistoryResultClass,
    /// Opaque status label (e.g. a non-secret status descriptor).
    pub status_label: String,
    /// Aggregate assertion state.
    pub assertion_state: AssertionStateClass,
    /// Number of passing assertions.
    pub assertion_pass_count: usize,
    /// Number of failing assertions.
    pub assertion_fail_count: usize,
    /// Retention mode for this row.
    pub retention_mode: RetentionMode,
    /// Redaction posture applied to stored content.
    pub redaction_posture: RedactionPostureClass,
    /// Whether the timestamp is visible.
    pub timestamp_visible: bool,
    /// Whether the environment is visible.
    pub environment_visible: bool,
    /// Whether the origin scope is visible.
    pub origin_scope_visible: bool,
    /// Whether the origin drift state is visible.
    pub origin_drift_visible: bool,
    /// Whether the result class is visible.
    pub result_class_visible: bool,
    /// Whether the assertion state is visible.
    pub assertion_state_visible: bool,
    /// Whether the retention mode is visible.
    pub retention_mode_visible: bool,
    /// Whether the compare action is available.
    pub compare_action_available: bool,
    /// Whether the export action is available.
    pub export_action_available: bool,
    /// Whether managed/companion origin trust is isolated from desktop-local trust.
    pub local_trust_isolated: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

impl RequestHistoryRow {
    /// Returns true when the row stores more than safe metadata and therefore
    /// requires an explicit reviewed retention selection.
    pub const fn requires_retention_review(&self) -> bool {
        matches!(
            self.retention_mode,
            RetentionMode::RedactedReplayable | RetentionMode::OptInFullCapture
        )
    }

    /// Returns true when the row captures full bodies/headers.
    pub const fn is_full_capture(&self) -> bool {
        matches!(self.retention_mode, RetentionMode::OptInFullCapture)
    }
}

/// One explicit, reviewed retention-upgrade selection for a history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionSelectionRow {
    /// Stable selection id.
    pub selection_id: String,
    /// History row this selection upgrades.
    pub history_ref: String,
    /// Requested retention mode.
    pub requested_mode: RetentionMode,
    /// Redaction posture declared for the upgrade.
    pub redaction_posture: RedactionPostureClass,
    /// Whether the upgrade requires an explicit review.
    pub requires_explicit_review: bool,
    /// Whether the operator has reviewed and acknowledged the upgrade.
    pub reviewed: bool,
    /// Whether full unredacted bodies are stored.
    pub stores_full_bodies: bool,
    /// Whether full unredacted headers are stored.
    pub stores_full_headers: bool,
    /// Whether the metadata-only base remains retained alongside the upgrade.
    pub default_safe_retained: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One export-safe compare row across two history rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryCompareRow {
    /// Stable compare id.
    pub compare_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Base history row ref.
    pub base_history_ref: String,
    /// Compared-against history row ref.
    pub against_history_ref: String,
    /// What the compare diffs.
    pub compare_basis: CompareBasisClass,
    /// Export redaction class for the compare result.
    pub export_redaction_class: ExportRedactionClass,
    /// Whether the compare requires full capture (only with a reviewed selection).
    pub requires_full_capture: bool,
    /// Whether the compare forces unsafe body/header retention (must be false).
    pub forces_unsafe_retention: bool,
    /// Whether the compare result includes raw secret values (must be false).
    pub includes_raw_secrets: bool,
    /// Whether the compare result includes raw response bodies.
    pub includes_raw_bodies: bool,
    /// Whether environment identity is retained in the compare.
    pub environment_identity_retained: bool,
    /// Whether origin identity is retained in the compare.
    pub origin_identity_retained: bool,
    /// Whether the compare is safe to export.
    pub export_safe: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One export-safe request-history export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryExportRow {
    /// Stable export id.
    pub export_id: String,
    /// History row this export covers.
    pub history_ref: String,
    /// Export redaction class.
    pub export_redaction_class: ExportRedactionClass,
    /// Whether the export includes raw secret values (must be false).
    pub includes_raw_secrets: bool,
    /// Whether the export includes raw response bodies.
    pub includes_raw_response_body: bool,
    /// Environment identity ref retained in the export (never dropped).
    pub environment_ref: String,
    /// Origin scope retained in the export (never dropped).
    pub origin_scope: RequestOriginKind,
    /// Portable format ref.
    pub portable_format_ref: String,
    /// Whether the export is safe for support bundles.
    pub support_bundle_safe: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream request-workspace packet this row consumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistoryUpstreamRefRow {
    /// Stable reference id.
    pub ref_id: String,
    /// Upstream record kind.
    pub upstream_record_kind: String,
    /// Repo-relative path to the upstream packet.
    pub upstream_packet_path: String,
    /// Repo-relative path to the upstream schema.
    pub upstream_schema_path: String,
    /// Whether integration has been verified.
    pub integration_verified: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Summary counts for a request-history qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistoryQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of history rows.
    pub history_row_count: usize,
    /// Number of history rows that capture full bodies/headers.
    pub full_capture_history_count: usize,
    /// Number of history rows whose origin must isolate desktop-local trust.
    pub trust_isolated_history_count: usize,
    /// Number of compare rows.
    pub compare_count: usize,
    /// Number of compare rows that are export-safe.
    pub export_safe_compare_count: usize,
    /// Number of retention-selection rows.
    pub retention_selection_count: usize,
    /// Number of reviewed retention upgrades.
    pub reviewed_upgrade_count: usize,
    /// Number of export rows.
    pub export_count: usize,
    /// Number of export rows safe for support bundles.
    pub support_safe_export_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical request-history qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestHistoryQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<RequestHistorySurfaceQualificationRow>,
    /// History rows.
    pub history_rows: Vec<RequestHistoryRow>,
    /// Retention-selection rows.
    pub retention_selections: Vec<RetentionSelectionRow>,
    /// Compare rows.
    pub compares: Vec<HistoryCompareRow>,
    /// Export rows.
    pub exports: Vec<HistoryExportRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<RequestHistoryUpstreamRefRow>,
    /// Summary counts.
    pub summary: RequestHistoryQualificationSummary,
}

impl RequestHistoryQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> RequestHistoryQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        let full_capture_history_count = self
            .history_rows
            .iter()
            .filter(|row| row.is_full_capture())
            .count();
        let trust_isolated_history_count = self
            .history_rows
            .iter()
            .filter(|row| row.origin_scope.must_isolate_local_trust())
            .count();
        let export_safe_compare_count = self.compares.iter().filter(|row| row.export_safe).count();
        let reviewed_upgrade_count = self
            .retention_selections
            .iter()
            .filter(|row| row.reviewed)
            .count();
        let support_safe_export_count = self
            .exports
            .iter()
            .filter(|row| row.support_bundle_safe)
            .count();
        let integration_pass_count = self
            .upstream_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        RequestHistoryQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            history_row_count: self.history_rows.len(),
            full_capture_history_count,
            trust_isolated_history_count,
            compare_count: self.compares.len(),
            export_safe_compare_count,
            retention_selection_count: self.retention_selections.len(),
            reviewed_upgrade_count,
            export_count: self.exports.len(),
            support_safe_export_count,
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of history rows that capture full bodies/headers, the
    /// rows that must be backed by an explicit reviewed retention selection.
    pub fn full_capture_history_ids(&self) -> Vec<String> {
        self.history_rows
            .iter()
            .filter(|row| row.is_full_capture())
            .map(|row| row.history_row_id.clone())
            .collect()
    }

    /// Returns the ids of history rows whose origin must isolate desktop-local
    /// trust (managed-workspace and browser-companion origins).
    pub fn trust_isolated_history_ids(&self) -> Vec<String> {
        self.history_rows
            .iter()
            .filter(|row| row.origin_scope.must_isolate_local_trust())
            .map(|row| row.history_row_id.clone())
            .collect()
    }

    /// Returns the ids of compare rows that stay export-safe.
    pub fn export_safe_compare_ids(&self) -> Vec<String> {
        self.compares
            .iter()
            .filter(|row| row.export_safe)
            .map(|row| row.compare_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<RequestHistoryQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != REQUEST_HISTORY_QUALIFICATION_SCHEMA_VERSION {
            violations.push(RequestHistoryQualificationViolation::SchemaVersion {
                expected: REQUEST_HISTORY_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REQUEST_HISTORY_QUALIFICATION_RECORD_KIND {
            violations.push(RequestHistoryQualificationViolation::RecordKind {
                expected: REQUEST_HISTORY_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let surface_ids = collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            RequestHistoryQualificationViolationKind::Surface,
        );
        let history_ids = collect_ids(
            self.history_rows
                .iter()
                .map(|row| row.history_row_id.as_str()),
            &mut violations,
            RequestHistoryQualificationViolationKind::History,
        );
        collect_ids(
            self.retention_selections
                .iter()
                .map(|row| row.selection_id.as_str()),
            &mut violations,
            RequestHistoryQualificationViolationKind::RetentionSelection,
        );
        collect_ids(
            self.compares.iter().map(|row| row.compare_id.as_str()),
            &mut violations,
            RequestHistoryQualificationViolationKind::Compare,
        );
        collect_ids(
            self.exports.iter().map(|row| row.export_id.as_str()),
            &mut violations,
            RequestHistoryQualificationViolationKind::Export,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            RequestHistoryQualificationViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_history_rows(&mut violations, &surface_ids);
        self.validate_retention_selections(&mut violations, &history_ids);
        self.validate_compares(&mut violations, &surface_ids, &history_ids);
        self.validate_exports(&mut violations, &history_ids);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(RequestHistoryQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<RequestHistoryQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        RequestHistoryQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        RequestHistoryQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    RequestHistoryQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            RequestHistorySurfaceKind::RequestHistoryPanel,
            RequestHistorySurfaceKind::CompanionHistory,
            RequestHistorySurfaceKind::ManagedHistory,
            RequestHistorySurfaceKind::CompareView,
            RequestHistorySurfaceKind::RetentionSettings,
            RequestHistorySurfaceKind::CliHeadlessOutput,
            RequestHistorySurfaceKind::SupportExport,
            RequestHistorySurfaceKind::HelpAbout,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(RequestHistoryQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_history_rows(
        &self,
        violations: &mut Vec<RequestHistoryQualificationViolation>,
        surface_ids: &BTreeSet<String>,
    ) {
        for row in &self.history_rows {
            // Every history row must project its identity columns: timestamp,
            // environment, origin scope and drift, result class, assertion
            // state, and retention mode are never hidden.
            if !surface_ids.contains(&row.surface_ref)
                || row.request_identity_ref.is_empty()
                || row.executed_at.is_empty()
                || row.status_label.is_empty()
                || !row.timestamp_visible
                || !row.environment_visible
                || !row.origin_scope_visible
                || !row.origin_drift_visible
                || !row.result_class_visible
                || !row.assertion_state_visible
                || !row.retention_mode_visible
            {
                violations.push(RequestHistoryQualificationViolation::IncompleteHistoryRow {
                    history_id: row.history_row_id.clone(),
                });
            }

            // The assertion counts must agree with the aggregate state so a
            // failing run never reads as clean.
            let counts_ok = match row.assertion_state {
                AssertionStateClass::NoAssertions => {
                    row.assertion_pass_count == 0 && row.assertion_fail_count == 0
                }
                AssertionStateClass::AllPassed => {
                    row.assertion_pass_count > 0 && row.assertion_fail_count == 0
                }
                AssertionStateClass::MixedResults => {
                    row.assertion_pass_count > 0 && row.assertion_fail_count > 0
                }
                AssertionStateClass::AnyFailed => {
                    row.assertion_pass_count == 0 && row.assertion_fail_count > 0
                }
                AssertionStateClass::NotEvaluated => {
                    row.assertion_pass_count == 0 && row.assertion_fail_count == 0
                }
            };
            if !counts_ok {
                violations.push(
                    RequestHistoryQualificationViolation::AssertionCountsMismatch {
                        history_id: row.history_row_id.clone(),
                    },
                );
            }

            // Retention mode and redaction posture must agree, and metadata-only
            // is the safe default.
            let posture_ok = match row.retention_mode {
                RetentionMode::MetadataOnly => {
                    row.redaction_posture == RedactionPostureClass::RedactAll
                }
                RetentionMode::RedactedReplayable => {
                    row.redaction_posture == RedactionPostureClass::RedactSecrets
                }
                RetentionMode::OptInFullCapture => matches!(
                    row.redaction_posture,
                    RedactionPostureClass::RedactSecrets
                        | RedactionPostureClass::NoRedactionLocalOnly
                ),
                // Text-first versioning is a collection posture, not a history
                // retention mode.
                RetentionMode::TextFirstVersioned => false,
            };
            if !posture_ok {
                violations.push(
                    RequestHistoryQualificationViolation::RetentionPostureMismatch {
                        history_id: row.history_row_id.clone(),
                    },
                );
            }

            // Any retention beyond safe metadata-only requires an explicit,
            // reviewed retention selection; full capture is never the default.
            if row.requires_retention_review() {
                let reviewed = self.retention_selections.iter().any(|selection| {
                    selection.history_ref == row.history_row_id
                        && selection.requested_mode == row.retention_mode
                        && selection.requires_explicit_review
                        && selection.reviewed
                });
                if !reviewed {
                    violations.push(
                        RequestHistoryQualificationViolation::UnreviewedRetentionUpgrade {
                            history_id: row.history_row_id.clone(),
                        },
                    );
                }
            }

            // Managed-workspace and browser-companion origins must never inherit
            // desktop-local trust.
            if row.origin_scope.must_isolate_local_trust() && !row.local_trust_isolated {
                violations.push(
                    RequestHistoryQualificationViolation::OriginTrustNotIsolated {
                        history_id: row.history_row_id.clone(),
                    },
                );
            }
        }

        // Coverage: every result class, origin kind, retention mode, and
        // assertion state must be exercised so the lane is proven, not asserted.
        let result_classes: BTreeSet<_> = self
            .history_rows
            .iter()
            .map(|row| row.result_class)
            .collect();
        for required in [
            HistoryResultClass::Success,
            HistoryResultClass::Redirected,
            HistoryResultClass::ClientError,
            HistoryResultClass::ServerError,
            HistoryResultClass::TransportError,
            HistoryResultClass::Blocked,
            HistoryResultClass::TimedOut,
            HistoryResultClass::Cancelled,
        ] {
            if !result_classes.contains(&required) {
                violations.push(RequestHistoryQualificationViolation::MissingResultClass {
                    result_class: required,
                });
            }
        }

        let origin_kinds: BTreeSet<_> = self
            .history_rows
            .iter()
            .map(|row| row.origin_scope)
            .collect();
        for required in [
            RequestOriginKind::LocalHost,
            RequestOriginKind::Remote,
            RequestOriginKind::Container,
            RequestOriginKind::Managed,
            RequestOriginKind::BrowserCompanion,
        ] {
            if !origin_kinds.contains(&required) {
                violations.push(RequestHistoryQualificationViolation::MissingOriginKind {
                    origin_kind: required,
                });
            }
        }

        let retention_modes: BTreeSet<_> = self
            .history_rows
            .iter()
            .map(|row| row.retention_mode)
            .collect();
        for required in [
            RetentionMode::MetadataOnly,
            RetentionMode::RedactedReplayable,
            RetentionMode::OptInFullCapture,
        ] {
            if !retention_modes.contains(&required) {
                violations.push(RequestHistoryQualificationViolation::MissingRetentionMode {
                    retention_mode: required,
                });
            }
        }

        let assertion_states: BTreeSet<_> = self
            .history_rows
            .iter()
            .map(|row| row.assertion_state)
            .collect();
        for required in [
            AssertionStateClass::NoAssertions,
            AssertionStateClass::AllPassed,
            AssertionStateClass::MixedResults,
            AssertionStateClass::AnyFailed,
            AssertionStateClass::NotEvaluated,
        ] {
            if !assertion_states.contains(&required) {
                violations.push(
                    RequestHistoryQualificationViolation::MissingAssertionState {
                        assertion_state: required,
                    },
                );
            }
        }

        // Companion and managed origins are claimed surfaces; at least one of
        // each must be exercised so trust isolation is proven, not asserted.
        if !self
            .history_rows
            .iter()
            .any(|row| row.origin_scope == RequestOriginKind::Managed)
            || !self
                .history_rows
                .iter()
                .any(|row| row.origin_scope == RequestOriginKind::BrowserCompanion)
        {
            violations.push(RequestHistoryQualificationViolation::NoTrustIsolatedOriginCovered);
        }
    }

    fn validate_retention_selections(
        &self,
        violations: &mut Vec<RequestHistoryQualificationViolation>,
        history_ids: &BTreeSet<String>,
    ) {
        for row in &self.retention_selections {
            if !history_ids.contains(&row.history_ref) || row.rationale.is_empty() {
                violations.push(
                    RequestHistoryQualificationViolation::IncompleteRetentionSelection {
                        selection_id: row.selection_id.clone(),
                    },
                );
            }

            // An upgrade beyond metadata-only must be an explicit, reviewed
            // selection so full capture is never the path of least resistance.
            let is_upgrade = matches!(
                row.requested_mode,
                RetentionMode::RedactedReplayable | RetentionMode::OptInFullCapture
            );
            if is_upgrade && (!row.requires_explicit_review || !row.reviewed) {
                violations.push(
                    RequestHistoryQualificationViolation::RetentionSelectionNotReviewed {
                        selection_id: row.selection_id.clone(),
                    },
                );
            }

            // Full unredacted body/header storage is only allowed under opt-in
            // full capture; redacted-replayable and metadata-only never store
            // full bodies or headers.
            let full_storage_ok = match row.requested_mode {
                RetentionMode::OptInFullCapture => true,
                _ => !row.stores_full_bodies && !row.stores_full_headers,
            };
            if !full_storage_ok {
                violations.push(
                    RequestHistoryQualificationViolation::RetentionSelectionStoresUnsafeBodies {
                        selection_id: row.selection_id.clone(),
                    },
                );
            }

            // The posture must agree with the requested mode.
            let posture_ok = match row.requested_mode {
                RetentionMode::RedactedReplayable => {
                    row.redaction_posture == RedactionPostureClass::RedactSecrets
                }
                RetentionMode::OptInFullCapture => matches!(
                    row.redaction_posture,
                    RedactionPostureClass::RedactSecrets
                        | RedactionPostureClass::NoRedactionLocalOnly
                ),
                RetentionMode::MetadataOnly => {
                    row.redaction_posture == RedactionPostureClass::RedactAll
                }
                RetentionMode::TextFirstVersioned => false,
            };
            if !posture_ok {
                violations.push(
                    RequestHistoryQualificationViolation::RetentionSelectionPostureMismatch {
                        selection_id: row.selection_id.clone(),
                    },
                );
            }
        }

        // The reviewed full-capture path must be exercised at least once so the
        // explicit-selection rule is proven rather than asserted.
        if !self.retention_selections.iter().any(|row| {
            row.requested_mode == RetentionMode::OptInFullCapture
                && row.requires_explicit_review
                && row.reviewed
        }) {
            violations.push(RequestHistoryQualificationViolation::NoReviewedFullCaptureCovered);
        }
    }

    fn validate_compares(
        &self,
        violations: &mut Vec<RequestHistoryQualificationViolation>,
        surface_ids: &BTreeSet<String>,
        history_ids: &BTreeSet<String>,
    ) {
        for row in &self.compares {
            if !surface_ids.contains(&row.surface_ref)
                || !history_ids.contains(&row.base_history_ref)
                || !history_ids.contains(&row.against_history_ref)
                || row.rationale.is_empty()
            {
                violations.push(RequestHistoryQualificationViolation::IncompleteCompare {
                    compare_id: row.compare_id.clone(),
                });
            }

            // Compare never widens history toward unsafe body/header retention
            // and never carries raw secrets.
            if row.forces_unsafe_retention || row.includes_raw_secrets {
                violations.push(
                    RequestHistoryQualificationViolation::CompareForcesUnsafeRetention {
                        compare_id: row.compare_id.clone(),
                    },
                );
            }

            // Origin and environment identity are never dropped from a compare.
            if !row.environment_identity_retained || !row.origin_identity_retained {
                violations.push(RequestHistoryQualificationViolation::CompareDropsIdentity {
                    compare_id: row.compare_id.clone(),
                });
            }

            // A compare that needs full capture must be backed by a reviewed
            // full-capture selection on both rows; otherwise it must operate on
            // already-safe retention.
            if row.requires_full_capture {
                let both_reviewed = [
                    row.base_history_ref.as_str(),
                    row.against_history_ref.as_str(),
                ]
                .iter()
                .all(|history_ref| {
                    self.retention_selections.iter().any(|selection| {
                        selection.history_ref.as_str() == *history_ref
                            && selection.requested_mode == RetentionMode::OptInFullCapture
                            && selection.requires_explicit_review
                            && selection.reviewed
                    })
                });
                if !both_reviewed {
                    violations.push(
                        RequestHistoryQualificationViolation::CompareNeedsUnreviewedCapture {
                            compare_id: row.compare_id.clone(),
                        },
                    );
                }
            }

            // An export-safe compare must not include raw bodies and must not be
            // the unredacted-local-only class.
            if row.export_safe
                && (row.includes_raw_bodies
                    || row.export_redaction_class == ExportRedactionClass::UnredactedLocalOnly)
            {
                violations.push(
                    RequestHistoryQualificationViolation::CompareExportSafeButCarriesRaw {
                        compare_id: row.compare_id.clone(),
                    },
                );
            }
        }

        let compare_bases: BTreeSet<_> =
            self.compares.iter().map(|row| row.compare_basis).collect();
        for required in [
            CompareBasisClass::StatusAndTiming,
            CompareBasisClass::RedactedBodies,
            CompareBasisClass::AssertionResults,
            CompareBasisClass::HeaderMetadata,
        ] {
            if !compare_bases.contains(&required) {
                violations.push(RequestHistoryQualificationViolation::MissingCompareBasis {
                    compare_basis: required,
                });
            }
        }
    }

    fn validate_exports(
        &self,
        violations: &mut Vec<RequestHistoryQualificationViolation>,
        history_ids: &BTreeSet<String>,
    ) {
        for row in &self.exports {
            if !history_ids.contains(&row.history_ref)
                || row.environment_ref.is_empty()
                || row.portable_format_ref.is_empty()
                || row.rationale.is_empty()
            {
                violations.push(RequestHistoryQualificationViolation::IncompleteExport {
                    export_id: row.export_id.clone(),
                });
            }

            // Exports never carry raw secret values.
            if row.includes_raw_secrets {
                violations.push(
                    RequestHistoryQualificationViolation::ExportCarriesRawSecret {
                        export_id: row.export_id.clone(),
                    },
                );
            }

            // Raw response bodies may only ride an unredacted-local-only export,
            // and such an export is never support-bundle safe.
            if row.includes_raw_response_body
                && (row.export_redaction_class != ExportRedactionClass::UnredactedLocalOnly
                    || row.support_bundle_safe)
            {
                violations.push(
                    RequestHistoryQualificationViolation::ExportRawBodyNotLocalOnly {
                        export_id: row.export_id.clone(),
                    },
                );
            }

            // A support-safe export never carries raw bodies and is never the
            // unredacted-local-only class.
            if row.support_bundle_safe
                && (row.includes_raw_response_body
                    || row.export_redaction_class == ExportRedactionClass::UnredactedLocalOnly)
            {
                violations.push(
                    RequestHistoryQualificationViolation::SupportExportCarriesRaw {
                        export_id: row.export_id.clone(),
                    },
                );
            }
        }

        let export_classes: BTreeSet<_> = self
            .exports
            .iter()
            .map(|row| row.export_redaction_class)
            .collect();
        for required in [
            ExportRedactionClass::FullRedaction,
            ExportRedactionClass::MetadataOnly,
            ExportRedactionClass::SafePreview,
            ExportRedactionClass::UnredactedLocalOnly,
        ] {
            if !export_classes.contains(&required) {
                violations.push(RequestHistoryQualificationViolation::MissingExportClass {
                    export_class: required,
                });
            }
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<RequestHistoryQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(
                    RequestHistoryQualificationViolation::IncompleteUpstreamRef {
                        ref_id: row.ref_id.clone(),
                    },
                );
            }
        }
        // The history lane must consume the frozen API-collection matrix as a
        // verified upstream packet so its origin, retention, and contract
        // vocabularies stay aligned.
        let consumes_matrix = self.upstream_refs.iter().any(|row| {
            row.upstream_record_kind == API_MATRIX_QUALIFICATION_RECORD_KIND
                && row.integration_verified
        });
        if !consumes_matrix {
            violations.push(RequestHistoryQualificationViolation::MatrixUpstreamNotIntegrated);
        }
    }
}

/// Loads the checked-in request-history qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_request_history_qualification(
) -> Result<RequestHistoryQualificationPacket, serde_json::Error> {
    serde_json::from_str(REQUEST_HISTORY_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestHistoryQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// History rows.
    History,
    /// Retention-selection rows.
    RetentionSelection,
    /// Compare rows.
    Compare,
    /// Export rows.
    Export,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<RequestHistoryQualificationViolation>,
    kind: RequestHistoryQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(RequestHistoryQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for request-history qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestHistoryQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: RequestHistoryQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required consumer surface kind is missing.
    MissingSurfaceKind {
        surface_kind: RequestHistorySurfaceKind,
    },
    /// History row does not project its identity columns.
    IncompleteHistoryRow { history_id: String },
    /// History row assertion counts disagree with the aggregate state.
    AssertionCountsMismatch { history_id: String },
    /// History row retention mode and redaction posture disagree.
    RetentionPostureMismatch { history_id: String },
    /// History row retains more than metadata without a reviewed selection.
    UnreviewedRetentionUpgrade { history_id: String },
    /// Managed or companion origin row does not isolate desktop-local trust.
    OriginTrustNotIsolated { history_id: String },
    /// Required result class is missing.
    MissingResultClass { result_class: HistoryResultClass },
    /// Required origin kind is missing.
    MissingOriginKind { origin_kind: RequestOriginKind },
    /// Required retention mode is missing.
    MissingRetentionMode { retention_mode: RetentionMode },
    /// Required assertion state is missing.
    MissingAssertionState {
        assertion_state: AssertionStateClass,
    },
    /// No managed or companion origin row is covered.
    NoTrustIsolatedOriginCovered,
    /// Retention-selection row is incomplete.
    IncompleteRetentionSelection { selection_id: String },
    /// Retention upgrade is not an explicit reviewed selection.
    RetentionSelectionNotReviewed { selection_id: String },
    /// Retention selection stores full bodies/headers outside opt-in full capture.
    RetentionSelectionStoresUnsafeBodies { selection_id: String },
    /// Retention selection posture disagrees with the requested mode.
    RetentionSelectionPostureMismatch { selection_id: String },
    /// No reviewed full-capture selection is covered.
    NoReviewedFullCaptureCovered,
    /// Compare row is incomplete.
    IncompleteCompare { compare_id: String },
    /// Compare forces unsafe retention or carries raw secrets.
    CompareForcesUnsafeRetention { compare_id: String },
    /// Compare drops origin or environment identity.
    CompareDropsIdentity { compare_id: String },
    /// Compare needs full capture without a reviewed selection on both rows.
    CompareNeedsUnreviewedCapture { compare_id: String },
    /// Export-safe compare still carries raw bodies or is local-only.
    CompareExportSafeButCarriesRaw { compare_id: String },
    /// Required compare basis is missing.
    MissingCompareBasis { compare_basis: CompareBasisClass },
    /// Export row is incomplete or drops origin/environment identity.
    IncompleteExport { export_id: String },
    /// Export carries raw secret values.
    ExportCarriesRawSecret { export_id: String },
    /// Export carries a raw body outside the unredacted-local-only class.
    ExportRawBodyNotLocalOnly { export_id: String },
    /// Support-safe export carries raw content.
    SupportExportCarriesRaw { export_id: String },
    /// Required export redaction class is missing.
    MissingExportClass { export_class: ExportRedactionClass },
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// The history lane does not consume the API-collection matrix as a verified upstream packet.
    MatrixUpstreamNotIntegrated,
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for RequestHistoryQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingSurfaceKind { surface_kind } => {
                write!(f, "consumer surface kind {surface_kind:?} is not covered")
            }
            Self::IncompleteHistoryRow { history_id } => {
                write!(
                    f,
                    "{history_id} does not project request-history truth everywhere"
                )
            }
            Self::AssertionCountsMismatch { history_id } => {
                write!(
                    f,
                    "{history_id} assertion counts disagree with its aggregate state"
                )
            }
            Self::RetentionPostureMismatch { history_id } => {
                write!(
                    f,
                    "{history_id} retention mode and redaction posture disagree"
                )
            }
            Self::UnreviewedRetentionUpgrade { history_id } => {
                write!(
                    f,
                    "{history_id} retains more than metadata without a reviewed selection"
                )
            }
            Self::OriginTrustNotIsolated { history_id } => {
                write!(
                    f,
                    "{history_id} is a managed or companion origin without isolated trust"
                )
            }
            Self::MissingResultClass { result_class } => {
                write!(f, "result class {result_class:?} is not covered")
            }
            Self::MissingOriginKind { origin_kind } => {
                write!(f, "origin kind {origin_kind:?} is not covered")
            }
            Self::MissingRetentionMode { retention_mode } => {
                write!(f, "retention mode {retention_mode:?} is not covered")
            }
            Self::MissingAssertionState { assertion_state } => {
                write!(f, "assertion state {assertion_state:?} is not covered")
            }
            Self::NoTrustIsolatedOriginCovered => {
                write!(f, "no managed or companion origin row is covered")
            }
            Self::IncompleteRetentionSelection { selection_id } => {
                write!(
                    f,
                    "{selection_id} does not project retention-selection truth everywhere"
                )
            }
            Self::RetentionSelectionNotReviewed { selection_id } => {
                write!(
                    f,
                    "{selection_id} upgrades retention without an explicit reviewed selection"
                )
            }
            Self::RetentionSelectionStoresUnsafeBodies { selection_id } => {
                write!(
                    f,
                    "{selection_id} stores full bodies/headers outside opt-in full capture"
                )
            }
            Self::RetentionSelectionPostureMismatch { selection_id } => {
                write!(
                    f,
                    "{selection_id} redaction posture disagrees with its requested mode"
                )
            }
            Self::NoReviewedFullCaptureCovered => {
                write!(f, "no reviewed full-capture selection is exercised")
            }
            Self::IncompleteCompare { compare_id } => {
                write!(f, "{compare_id} does not project compare truth everywhere")
            }
            Self::CompareForcesUnsafeRetention { compare_id } => {
                write!(
                    f,
                    "{compare_id} forces unsafe retention or carries raw secrets"
                )
            }
            Self::CompareDropsIdentity { compare_id } => {
                write!(f, "{compare_id} drops origin or environment identity")
            }
            Self::CompareNeedsUnreviewedCapture { compare_id } => {
                write!(
                    f,
                    "{compare_id} needs full capture without a reviewed selection on both rows"
                )
            }
            Self::CompareExportSafeButCarriesRaw { compare_id } => {
                write!(
                    f,
                    "{compare_id} is export-safe but carries raw bodies or is local-only"
                )
            }
            Self::MissingCompareBasis { compare_basis } => {
                write!(f, "compare basis {compare_basis:?} is not covered")
            }
            Self::IncompleteExport { export_id } => {
                write!(
                    f,
                    "{export_id} does not project export truth or drops identity"
                )
            }
            Self::ExportCarriesRawSecret { export_id } => {
                write!(f, "{export_id} carries raw secret values")
            }
            Self::ExportRawBodyNotLocalOnly { export_id } => {
                write!(
                    f,
                    "{export_id} carries a raw body outside the unredacted-local-only class"
                )
            }
            Self::SupportExportCarriesRaw { export_id } => {
                write!(f, "{export_id} is support-safe but carries raw content")
            }
            Self::MissingExportClass { export_class } => {
                write!(f, "export redaction class {export_class:?} is not covered")
            }
            Self::IncompleteUpstreamRef { ref_id } => {
                write!(
                    f,
                    "{ref_id} does not project upstream reference truth everywhere"
                )
            }
            Self::MatrixUpstreamNotIntegrated => {
                write!(f, "history lane does not consume the API-collection matrix as a verified upstream packet")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for RequestHistoryQualificationViolation {}
