//! Request composer, mutation-review sheets, replay and history lanes, and
//! redaction-safe export qualification records.
//!
//! This module owns the typed records that keep request composers,
//! mutation-review sheets, history lanes, replay configurations, and
//! redaction-safe exports inspectable and attributable without depending on
//! hidden shell shortcuts or ad hoc scripts. The boundary schema is
//! [`/schemas/data/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.schema.json`](../../../schemas/data/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json`](../../../artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json).
//!
//! Raw endpoint URLs, raw secrets, raw credential bodies, raw response bodies,
//! and raw cookie or token values do not belong in these records. They carry
//! stable IDs, closed posture vocabularies, and reviewable summaries that UI,
//! CLI, export, support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for composer qualification packets.
pub const COMPOSER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ComposerQualificationPacket`].
pub const COMPOSER_QUALIFICATION_RECORD_KIND: &str =
    "request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export";

/// Repo-relative path to the checked-in composer qualification packet.
pub const COMPOSER_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json";

/// Embedded checked-in packet JSON.
pub const COMPOSER_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json"
));

/// Qualification label shown on promoted composer surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerQualificationLabel {
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

impl ComposerQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Composer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerSurfaceKind {
    /// Request composer editor or viewer.
    RequestComposer,
    /// Mutation-review sheet before send.
    MutationReviewSheet,
    /// History lane preserving request outcomes.
    HistoryLane,
    /// Replay lane for rerun configurations.
    ReplayLane,
    /// Export review with redaction posture.
    ExportReview,
    /// Response viewer with stream states.
    ResponseViewer,
}

/// Request composer document kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestComposerKind {
    /// HTTP request document.
    HttpRequest,
    /// GraphQL operation document.
    GraphqlOperation,
}

/// Mutation risk class shown across composer and review surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationRiskClass {
    /// Request is constrained to safe read-only work.
    ReadOnly,
    /// Mutation is idempotent and repeatable.
    IdempotentMutation,
    /// Mutation is destructive or non-reversible.
    DestructiveMutation,
    /// Safety class cannot be determined automatically.
    Ambiguous,
    /// Policy blocks execution or export.
    PolicyBlocked,
}

/// Replay mode for history and rerun surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayMode {
    /// Exact rerun using the same captured context.
    ExactRerun,
    /// Rerun resolving current environment and connection context.
    RerunWithCurrentContext,
    /// Rerun resolving current auth context.
    RerunWithCurrentAuth,
    /// Replay from a parameterized template.
    TemplateReplay,
    /// Open for review only, no execution.
    ReviewOnly,
    /// Blocked by drift or policy.
    Blocked,
}

/// Redaction class for export and support-bundle safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRedactionClass {
    /// All sensitive fields are redacted.
    FullRedaction,
    /// Only metadata is exported.
    MetadataOnly,
    /// Safe preview with bounded values.
    SafePreview,
    /// Unredacted but constrained to local-only export.
    UnredactedLocalOnly,
}

/// History retention posture for local-first, bounded storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryRetentionPosture {
    /// Local-first storage with no remote retention.
    LocalFirst,
    /// Bounded by count and age.
    Bounded,
    /// User-pinned until explicit clear.
    Pinned,
    /// Ephemeral, discarded after session.
    Ephemeral,
}

/// Response stream state shown during and after request execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStreamState {
    /// Connection in progress.
    Connecting,
    /// Headers received, body streaming.
    HeadersReceived,
    /// Body actively streaming.
    Streaming,
    /// Response truncated by size or time limit.
    Truncated,
    /// Response complete.
    Complete,
    /// Partial response with more data available.
    Partial,
    /// Request timed out.
    TimedOut,
    /// Blocked by policy before completion.
    PolicyBlocked,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposerQualificationProof {
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

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposerSurfaceGuardSet {
    /// Composer fields (method, target, body mode, auth mode, variable source) are visible.
    pub composer_fields_visible: bool,
    /// Mutation-review sheet (side-effect class, auth scope, confirmation) is visible.
    pub mutation_review_visible: bool,
    /// History lane shows request identity, environment, auth class, and response class.
    pub history_identity_visible: bool,
    /// Replay mode and context-resolution disclosure are visible.
    pub replay_mode_visible: bool,
    /// Export redaction posture and portable format are visible.
    pub export_redaction_visible: bool,
    /// Response stream state is visible.
    pub response_stream_visible: bool,
    /// Auth scope is visible without raw secrets.
    pub auth_scope_visible: bool,
    /// Target class and endpoint identity are visible.
    pub target_class_visible: bool,
}

impl ComposerSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.composer_fields_visible
            && self.mutation_review_visible
            && self.history_identity_visible
            && self.replay_mode_visible
            && self.export_redaction_visible
            && self.response_stream_visible
            && self.auth_scope_visible
            && self.target_class_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposerSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: ComposerSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: ComposerQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: ComposerQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<ComposerQualificationProof>,
    /// Visible guard set.
    pub guards: ComposerSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One request composer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestComposerRow {
    /// Stable composer id.
    pub composer_id: String,
    /// Composer document kind.
    pub composer_kind: RequestComposerKind,
    /// Method or operation kind label.
    pub method_kind: String,
    /// Path template.
    pub path_template: String,
    /// Header refs (opaque, non-secret).
    pub header_refs: Vec<String>,
    /// Body ref (opaque, non-secret).
    pub body_ref: String,
    /// Variable refs.
    pub variable_refs: Vec<String>,
    /// Assertion refs.
    pub assertion_refs: Vec<String>,
    /// Environment set ref.
    pub environment_set_ref: String,
    /// Auth source ref.
    pub auth_source_ref: String,
    /// Mutation risk class.
    pub mutation_risk_class: MutationRiskClass,
    /// Schema snapshot ref, if any.
    pub schema_snapshot_ref: Option<String>,
    /// Whether the document is diffable and versionable.
    pub diffable: bool,
    /// Whether the document is reusable from CLI and automation.
    pub cli_reusable: bool,
}

/// One mutation-review sheet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationReviewSheetRow {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Target endpoint identity ref.
    pub target_endpoint_ref: String,
    /// Side-effect class description ref.
    pub side_effect_class: String,
    /// Auth scope ref.
    pub auth_scope_ref: String,
    /// Request body mode (template, literal, empty).
    pub request_body_mode: String,
    /// Whether explicit confirmation is required before send.
    pub confirmation_required: bool,
    /// Whether replay consequences are disclosed before send.
    pub replay_consequences_disclosed: bool,
    /// Whether the sheet is previewable before send.
    pub previewable_before_send: bool,
    /// Whether rollback or undo path is visible.
    pub rollback_path_visible: bool,
}

/// One history lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryLaneRow {
    /// Stable lane id.
    pub lane_id: String,
    /// Request identity ref.
    pub request_identity_ref: String,
    /// Environment ref.
    pub environment_ref: String,
    /// Auth class ref.
    pub auth_class_ref: String,
    /// Response class ref.
    pub response_class_ref: String,
    /// Retention posture.
    pub retention_posture: HistoryRetentionPosture,
    /// Replay mode ref.
    pub replay_mode_ref: String,
    /// Redaction-safe export ref.
    pub redaction_safe_export_ref: String,
    /// Last executed timestamp.
    pub last_executed_at: String,
    /// Whether the entry is pinned against automatic eviction.
    pub pinned: bool,
}

/// One replay configuration row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayConfigRow {
    /// Stable config id.
    pub config_id: String,
    /// Replay mode.
    pub replay_mode: ReplayMode,
    /// Whether context resolution is disclosed.
    pub context_resolution_disclosed: bool,
    /// Whether auth reuse is disclosed.
    pub auth_reuse_disclosed: bool,
    /// Whether environment reuse is disclosed.
    pub environment_reuse_disclosed: bool,
    /// Whether a fresh review is required before rerun.
    pub requires_fresh_review: bool,
    /// Exact-rerun attestation ref, if applicable.
    pub exact_rerun_attestation_ref: Option<String>,
}

/// One redaction-safe export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedactionSafeExportRow {
    /// Stable export id.
    pub export_id: String,
    /// Export redaction class.
    pub export_redaction_class: ExportRedactionClass,
    /// Whether raw secrets may be included.
    pub includes_raw_secrets: bool,
    /// Whether raw response bodies may be included.
    pub includes_raw_response_body: bool,
    /// Portable format ref.
    pub portable_format_ref: String,
    /// Redaction rationale.
    pub redaction_rationale: String,
    /// Whether the export is safe for support bundles.
    pub support_bundle_safe: bool,
}

/// Summary counts for a composer qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposerQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of request composer rows.
    pub composer_count: usize,
    /// Number of mutation-review sheet rows.
    pub mutation_review_sheet_count: usize,
    /// Number of history lane rows.
    pub history_lane_count: usize,
    /// Number of replay config rows.
    pub replay_config_count: usize,
    /// Number of redaction-safe export rows.
    pub redaction_safe_export_count: usize,
}

/// Canonical composer qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposerQualificationPacket {
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
    pub surfaces: Vec<ComposerSurfaceQualificationRow>,
    /// Request composer rows.
    pub composers: Vec<RequestComposerRow>,
    /// Mutation-review sheet rows.
    pub mutation_review_sheets: Vec<MutationReviewSheetRow>,
    /// History lane rows.
    pub history_lanes: Vec<HistoryLaneRow>,
    /// Replay config rows.
    pub replay_configs: Vec<ReplayConfigRow>,
    /// Redaction-safe export rows.
    pub redaction_safe_exports: Vec<RedactionSafeExportRow>,
    /// Summary counts.
    pub summary: ComposerQualificationSummary,
}

impl ComposerQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ComposerQualificationSummary {
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
        ComposerQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            composer_count: self.composers.len(),
            mutation_review_sheet_count: self.mutation_review_sheets.len(),
            history_lane_count: self.history_lanes.len(),
            replay_config_count: self.replay_configs.len(),
            redaction_safe_export_count: self.redaction_safe_exports.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<ComposerQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != COMPOSER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ComposerQualificationViolation::SchemaVersion {
                expected: COMPOSER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPOSER_QUALIFICATION_RECORD_KIND {
            violations.push(ComposerQualificationViolation::RecordKind {
                expected: COMPOSER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            ComposerQualificationViolationKind::Surface,
        );
        collect_ids(
            self.composers.iter().map(|row| row.composer_id.as_str()),
            &mut violations,
            ComposerQualificationViolationKind::Composer,
        );
        collect_ids(
            self.mutation_review_sheets
                .iter()
                .map(|row| row.sheet_id.as_str()),
            &mut violations,
            ComposerQualificationViolationKind::MutationReviewSheet,
        );
        collect_ids(
            self.history_lanes.iter().map(|row| row.lane_id.as_str()),
            &mut violations,
            ComposerQualificationViolationKind::HistoryLane,
        );
        collect_ids(
            self.replay_configs.iter().map(|row| row.config_id.as_str()),
            &mut violations,
            ComposerQualificationViolationKind::ReplayConfig,
        );
        collect_ids(
            self.redaction_safe_exports
                .iter()
                .map(|row| row.export_id.as_str()),
            &mut violations,
            ComposerQualificationViolationKind::RedactionSafeExport,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(ComposerQualificationViolation::StableSurfaceMissingProof {
                        surface_id: surface.surface_id.clone(),
                    });
                }
                if !surface.guards.all_visible() {
                    violations.push(ComposerQualificationViolation::StableSurfaceMissingGuard {
                        surface_id: surface.surface_id.clone(),
                    });
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    ComposerQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let composer_kinds: BTreeSet<_> =
            self.composers.iter().map(|row| row.composer_kind).collect();
        for required_kind in [RequestComposerKind::HttpRequest, RequestComposerKind::GraphqlOperation]
        {
            if !composer_kinds.contains(&required_kind) {
                violations.push(ComposerQualificationViolation::MissingComposerKind {
                    composer_kind: required_kind,
                });
            }
        }

        let mutation_risks: BTreeSet<_> =
            self.composers.iter().map(|row| row.mutation_risk_class).collect();
        for required_risk in [
            MutationRiskClass::ReadOnly,
            MutationRiskClass::IdempotentMutation,
            MutationRiskClass::DestructiveMutation,
            MutationRiskClass::PolicyBlocked,
        ] {
            if !mutation_risks.contains(&required_risk) {
                violations.push(ComposerQualificationViolation::MissingMutationRiskClass {
                    mutation_risk_class: required_risk,
                });
            }
        }

        for row in &self.composers {
            if row.path_template.is_empty()
                || row.environment_set_ref.is_empty()
                || row.auth_source_ref.is_empty()
                || !row.diffable
            {
                violations.push(ComposerQualificationViolation::IncompleteComposerProjection {
                    composer_id: row.composer_id.clone(),
                });
            }
        }

        for row in &self.mutation_review_sheets {
            if row.target_endpoint_ref.is_empty()
                || row.side_effect_class.is_empty()
                || row.auth_scope_ref.is_empty()
                || !row.previewable_before_send
            {
                violations.push(
                    ComposerQualificationViolation::IncompleteMutationReviewSheet {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            if row.confirmation_required && !row.replay_consequences_disclosed {
                violations.push(
                    ComposerQualificationViolation::MutationReviewMissingReplayConsequences {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
        }

        let replay_modes: BTreeSet<_> =
            self.replay_configs.iter().map(|row| row.replay_mode).collect();
        for required_mode in [
            ReplayMode::ExactRerun,
            ReplayMode::RerunWithCurrentContext,
            ReplayMode::ReviewOnly,
            ReplayMode::Blocked,
        ] {
            if !replay_modes.contains(&required_mode) {
                violations.push(ComposerQualificationViolation::MissingReplayMode {
                    replay_mode: required_mode,
                });
            }
        }

        for row in &self.replay_configs {
            if row.replay_mode == ReplayMode::ExactRerun && row.exact_rerun_attestation_ref.is_none()
            {
                violations.push(ComposerQualificationViolation::ExactRerunMissingAttestation {
                    config_id: row.config_id.clone(),
                });
            }
            if !(row.context_resolution_disclosed
                && row.auth_reuse_disclosed
                && row.environment_reuse_disclosed)
            {
                violations.push(ComposerQualificationViolation::IncompleteReplayDisclosure {
                    config_id: row.config_id.clone(),
                });
            }
        }

        let export_classes: BTreeSet<_> = self
            .redaction_safe_exports
            .iter()
            .map(|row| row.export_redaction_class)
            .collect();
        for required_class in [
            ExportRedactionClass::FullRedaction,
            ExportRedactionClass::MetadataOnly,
            ExportRedactionClass::SafePreview,
        ] {
            if !export_classes.contains(&required_class) {
                violations.push(ComposerQualificationViolation::MissingExportRedactionClass {
                    export_redaction_class: required_class,
                });
            }
        }

        for row in &self.redaction_safe_exports {
            if row.includes_raw_secrets || row.includes_raw_response_body {
                violations.push(ComposerQualificationViolation::ExportIncludesRawSensitiveData {
                    export_id: row.export_id.clone(),
                });
            }
            if row.portable_format_ref.is_empty() || row.redaction_rationale.is_empty() {
                violations.push(ComposerQualificationViolation::IncompleteExportProjection {
                    export_id: row.export_id.clone(),
                });
            }
        }

        let retention_postures: BTreeSet<_> =
            self.history_lanes.iter().map(|row| row.retention_posture).collect();
        for required_posture in [
            HistoryRetentionPosture::LocalFirst,
            HistoryRetentionPosture::Bounded,
            HistoryRetentionPosture::Pinned,
        ] {
            if !retention_postures.contains(&required_posture) {
                violations.push(ComposerQualificationViolation::MissingHistoryRetentionPosture {
                    retention_posture: required_posture,
                });
            }
        }

        for row in &self.history_lanes {
            if row.request_identity_ref.is_empty()
                || row.environment_ref.is_empty()
                || row.auth_class_ref.is_empty()
                || row.response_class_ref.is_empty()
            {
                violations.push(ComposerQualificationViolation::IncompleteHistoryLane {
                    lane_id: row.lane_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ComposerQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in composer qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_request_composer_qualification() -> Result<ComposerQualificationPacket, serde_json::Error> {
    serde_json::from_str(COMPOSER_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Request composer rows.
    Composer,
    /// Mutation-review sheet rows.
    MutationReviewSheet,
    /// History lane rows.
    HistoryLane,
    /// Replay config rows.
    ReplayConfig,
    /// Redaction-safe export rows.
    RedactionSafeExport,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ComposerQualificationViolation>,
    kind: ComposerQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ComposerQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for composer qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ComposerQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required request composer kind is missing.
    MissingComposerKind { composer_kind: RequestComposerKind },
    /// Required mutation risk class is missing.
    MissingMutationRiskClass { mutation_risk_class: MutationRiskClass },
    /// Composer row does not project truth everywhere.
    IncompleteComposerProjection { composer_id: String },
    /// Mutation-review sheet is incomplete.
    IncompleteMutationReviewSheet { sheet_id: String },
    /// Mutation-review sheet requires confirmation but omits replay consequences.
    MutationReviewMissingReplayConsequences { sheet_id: String },
    /// Required replay mode is missing.
    MissingReplayMode { replay_mode: ReplayMode },
    /// Exact-rerun config lacks an attestation ref.
    ExactRerunMissingAttestation { config_id: String },
    /// Replay config does not disclose context, auth, and environment reuse.
    IncompleteReplayDisclosure { config_id: String },
    /// Required export redaction class is missing.
    MissingExportRedactionClass { export_redaction_class: ExportRedactionClass },
    /// Export row includes raw secrets or raw response bodies.
    ExportIncludesRawSensitiveData { export_id: String },
    /// Export row lacks portable format or redaction rationale.
    IncompleteExportProjection { export_id: String },
    /// Required history retention posture is missing.
    MissingHistoryRetentionPosture { retention_posture: HistoryRetentionPosture },
    /// History lane lacks required identity refs.
    IncompleteHistoryLane { lane_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ComposerQualificationViolation {
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
            Self::MissingComposerKind { composer_kind } => {
                write!(f, "composer kind {composer_kind:?} is not covered")
            }
            Self::MissingMutationRiskClass { mutation_risk_class } => {
                write!(f, "mutation risk class {mutation_risk_class:?} is not covered")
            }
            Self::IncompleteComposerProjection { composer_id } => {
                write!(f, "{composer_id} does not project composer truth everywhere")
            }
            Self::IncompleteMutationReviewSheet { sheet_id } => {
                write!(f, "{sheet_id} does not project mutation-review truth everywhere")
            }
            Self::MutationReviewMissingReplayConsequences { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} requires confirmation but omits replay consequences"
                )
            }
            Self::MissingReplayMode { replay_mode } => {
                write!(f, "replay mode {replay_mode:?} is not covered")
            }
            Self::ExactRerunMissingAttestation { config_id } => {
                write!(f, "{config_id} is exact_rerun without an attestation ref")
            }
            Self::IncompleteReplayDisclosure { config_id } => {
                write!(
                    f,
                    "{config_id} does not disclose context, auth, and environment reuse"
                )
            }
            Self::MissingExportRedactionClass { export_redaction_class } => {
                write!(
                    f,
                    "export redaction class {export_redaction_class:?} is not covered"
                )
            }
            Self::ExportIncludesRawSensitiveData { export_id } => {
                write!(
                    f,
                    "{export_id} includes raw secrets or raw response bodies"
                )
            }
            Self::IncompleteExportProjection { export_id } => {
                write!(f, "{export_id} lacks portable format or redaction rationale")
            }
            Self::MissingHistoryRetentionPosture { retention_posture } => {
                write!(
                    f,
                    "history retention posture {retention_posture:?} is not covered"
                )
            }
            Self::IncompleteHistoryLane { lane_id } => {
                write!(f, "{lane_id} lacks required identity refs")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ComposerQualificationViolation {}
