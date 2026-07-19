//! Shared task/test, request/database, notebook/preview, AI/publish, and
//! support/export consumers for the frozen M5 execution-lifecycle components.
//!
//! This module is the M05-825 first-consumer adoption lane over the frozen M5
//! execution-lifecycle component matrix
//! ([`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`])
//! and the 821-824 primitive resolvers (run/attempt header, input-request /
//! artifact-publish interaction, rerun-comparison review, and debug-session
//! hierarchy). Where the freeze matrix defines the reusable run/attempt,
//! input-request, artifact-publish, rerun-review, and debug-hierarchy
//! primitives and 821-824 resolve their per-surface truth, this lane proves the
//! five families are reusable *primitives* rather than one task pane, one debug
//! strip, or one provider-specific run view by adopting them across the five
//! claimed M5 execution consumer classes:
//!
//! 1. a task / test consumer,
//! 2. a request or database execution lane,
//! 3. a notebook or preview lane,
//! 4. an AI / publish or remote-mutation lane, and
//! 5. a support / export lane (including docs / help and activity / history).
//!
//! Each [`ExecutionConsumerRow`] points back to exactly one canonical component
//! family (the primitive schema + release-proof packet) instead of cloning
//! surface-local run/debug vocabulary, and every consumer — even a read-only,
//! inspect-only, compare-only, or export-only one — keeps the identical label
//! families for run/attempt identity, outcome state, rerun context difference,
//! artifact lineage / retention, and debug control posture, plus the identical
//! degraded-state vocabulary. A narrower consumer discloses the reduction with a
//! reduced-capability banner (and, when it punts to another surface, a companion
//! / browser / handoff note) rather than renaming or dropping governed state.
//!
//! The packet is metadata-only: raw run logs, request bodies, artifact payloads,
//! stack frames, credentials, and provider payloads never cross this boundary;
//! the packet carries only typed class tokens, opaque summary / evidence refs,
//! booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-execution-lifecycle-component-consumer.schema.json`](../../../../schemas/ui/m5-execution-lifecycle-component-consumer.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_execution_lifecycle_component_consumer_contract.md`](../../../../docs/run-test-debug/m5_execution_lifecycle_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::implement_the_m5_debug_session_header_thread_process_tree_and_dump_crash_artifact_card_primitive::{
    M5_DEBUG_HIERARCHY_ARTIFACT_REF, M5_DEBUG_HIERARCHY_SCHEMA_REF,
};
use crate::implement_the_m5_input_request_prompt_and_artifact_publish_row_primitive::{
    M5_EXECUTION_INTERACTION_ARTIFACT_REF, M5_EXECUTION_INTERACTION_SCHEMA_REF,
};
use crate::implement_the_m5_rerun_comparison_sheet_and_retry_scope_review_primitive::{
    M5_RERUN_REVIEW_ARTIFACT_REF, M5_RERUN_REVIEW_SCHEMA_REF,
};
use crate::implement_the_m5_run_attempt_header_and_attempt_selector_primitive::{
    M5_RUN_ATTEMPT_HEADER_ARTIFACT_REF, M5_RUN_ATTEMPT_HEADER_SCHEMA_REF,
};

/// Schema version stamped on the M05-825 consumer packet.
pub const EXECUTION_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ExecutionConsumerPacket`].
pub const EXECUTION_CONSUMER_RECORD_KIND: &str = "m5_execution_lifecycle_component_consumer_packet";

/// Stable record-kind tag carried by each [`ExecutionConsumerRow`].
pub const EXECUTION_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_execution_lifecycle_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const EXECUTION_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const EXECUTION_CONSUMER_DOC_REF: &str =
    "docs/run-test-debug/m5_execution_lifecycle_component_consumer_contract.md";

/// Repo-relative path of the frozen execution-lifecycle component matrix these
/// consumers adopt.
pub const EXECUTION_CONSUMER_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const EXECUTION_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-execution-lifecycle-component-consumers";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EXECUTION_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EXECUTION_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-execution-lifecycle-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EXECUTION_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-component-consumer-proof/report.md";

/// The controlled label families a consumer must preserve identically across
/// every surface. These are the track-invariant truth pillars of the
/// execution-lifecycle components: run / attempt identity, outcome state, rerun
/// context difference, artifact lineage / retention, and debug control posture.
/// The union of every row's `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 5] = [
    "run_attempt_identity",
    "outcome_state",
    "rerun_context_difference",
    "artifact_lineage_retention",
    "debug_control_posture",
];

/// The five reusable execution-lifecycle component families, each narrowed by a
/// sibling 821-824 primitive resolver. Consumers point at these instead of
/// inventing per-surface run/debug identity language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionComponentFamily {
    /// The run / attempt header + attempt selector (M05-821).
    RunAttemptHeader,
    /// The typed input-request prompt (M05-822).
    InputRequestPrompt,
    /// The produced-artifact publish row (M05-822).
    ArtifactPublishRow,
    /// The rerun-comparison sheet + retry-scope review (M05-823).
    RerunReview,
    /// The debug-session header + thread/process tree + dump/crash card
    /// (M05-824).
    DebugHierarchy,
}

impl M5ExecutionComponentFamily {
    /// Every execution-lifecycle component family, in declaration order.
    pub const ALL: [M5ExecutionComponentFamily; 5] = [
        M5ExecutionComponentFamily::RunAttemptHeader,
        M5ExecutionComponentFamily::InputRequestPrompt,
        M5ExecutionComponentFamily::ArtifactPublishRow,
        M5ExecutionComponentFamily::RerunReview,
        M5ExecutionComponentFamily::DebugHierarchy,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunAttemptHeader => "run_attempt_header",
            Self::InputRequestPrompt => "input_request_prompt",
            Self::ArtifactPublishRow => "artifact_publish_row",
            Self::RerunReview => "rerun_review",
            Self::DebugHierarchy => "debug_hierarchy",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RunAttemptHeader => "Run / attempt header",
            Self::InputRequestPrompt => "Input-request prompt",
            Self::ArtifactPublishRow => "Artifact-publish row",
            Self::RerunReview => "Rerun-comparison review",
            Self::DebugHierarchy => "Debug-session hierarchy",
        }
    }

    /// The canonical primitive schema that defines this family's contract.
    /// Consumers must point at this schema instead of inventing a surface-local
    /// one.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::RunAttemptHeader => M5_RUN_ATTEMPT_HEADER_SCHEMA_REF,
            // The input-request prompt and artifact-publish row are two halves
            // of the same M05-822 execution-interaction primitive.
            Self::InputRequestPrompt | Self::ArtifactPublishRow => {
                M5_EXECUTION_INTERACTION_SCHEMA_REF
            }
            Self::RerunReview => M5_RERUN_REVIEW_SCHEMA_REF,
            Self::DebugHierarchy => M5_DEBUG_HIERARCHY_SCHEMA_REF,
        }
    }

    /// The canonical release-proof packet that defines this family's first
    /// resolved truth. Consumers point back to this packet rather than cloning
    /// it.
    pub const fn canonical_packet_ref(self) -> &'static str {
        match self {
            Self::RunAttemptHeader => M5_RUN_ATTEMPT_HEADER_ARTIFACT_REF,
            Self::InputRequestPrompt | Self::ArtifactPublishRow => {
                M5_EXECUTION_INTERACTION_ARTIFACT_REF
            }
            Self::RerunReview => M5_RERUN_REVIEW_ARTIFACT_REF,
            Self::DebugHierarchy => M5_DEBUG_HIERARCHY_ARTIFACT_REF,
        }
    }
}

/// The five claimed M5 execution consumer classes that must each adopt at least
/// one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerGroup {
    /// A task / test consumer.
    TaskTest,
    /// A request or database execution lane.
    RequestDatabase,
    /// A notebook or preview lane.
    NotebookPreview,
    /// An AI / publish or remote-mutation lane.
    AiPublish,
    /// A support / export lane (including docs / help and activity / history).
    SupportExport,
}

impl ConsumerGroup {
    /// Every consumer group that must be present for cross-surface reuse.
    pub const ALL: [ConsumerGroup; 5] = [
        ConsumerGroup::TaskTest,
        ConsumerGroup::RequestDatabase,
        ConsumerGroup::NotebookPreview,
        ConsumerGroup::AiPublish,
        ConsumerGroup::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskTest => "task_test",
            Self::RequestDatabase => "request_database",
            Self::NotebookPreview => "notebook_preview",
            Self::AiPublish => "ai_publish",
            Self::SupportExport => "support_export",
        }
    }
}

/// The concrete M5 execution surface a component is embedded in. Each surface
/// belongs to exactly one [`ConsumerGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionConsumerSurface {
    /// The task-run pane.
    TaskRunPane,
    /// The test explorer / test-run pane.
    TestExplorer,
    /// The API / request-run pane.
    RequestRunPane,
    /// The database-query execution pane.
    DatabaseExecutionPane,
    /// A notebook execution cell.
    NotebookExecutionCell,
    /// The preview-runtime lane.
    PreviewRuntimeLane,
    /// An AI-mediated (agent-driven) run surface.
    AiMediatedRun,
    /// A publish / deploy flow.
    PublishDeployFlow,
    /// The support / export replay surface.
    SupportExportReplay,
    /// The run history / activity center.
    HistoryActivityCenter,
    /// The docs / help center.
    HelpCenterDocs,
}

impl M5ExecutionConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [M5ExecutionConsumerSurface; 11] = [
        M5ExecutionConsumerSurface::TaskRunPane,
        M5ExecutionConsumerSurface::TestExplorer,
        M5ExecutionConsumerSurface::RequestRunPane,
        M5ExecutionConsumerSurface::DatabaseExecutionPane,
        M5ExecutionConsumerSurface::NotebookExecutionCell,
        M5ExecutionConsumerSurface::PreviewRuntimeLane,
        M5ExecutionConsumerSurface::AiMediatedRun,
        M5ExecutionConsumerSurface::PublishDeployFlow,
        M5ExecutionConsumerSurface::SupportExportReplay,
        M5ExecutionConsumerSurface::HistoryActivityCenter,
        M5ExecutionConsumerSurface::HelpCenterDocs,
    ];

    /// The consumer group this surface belongs to.
    pub const fn consumer_group(self) -> ConsumerGroup {
        match self {
            Self::TaskRunPane | Self::TestExplorer => ConsumerGroup::TaskTest,
            Self::RequestRunPane | Self::DatabaseExecutionPane => ConsumerGroup::RequestDatabase,
            Self::NotebookExecutionCell | Self::PreviewRuntimeLane => {
                ConsumerGroup::NotebookPreview
            }
            Self::AiMediatedRun | Self::PublishDeployFlow => ConsumerGroup::AiPublish,
            Self::SupportExportReplay | Self::HistoryActivityCenter | Self::HelpCenterDocs => {
                ConsumerGroup::SupportExport
            }
        }
    }

    /// True when this surface is a docs / help reference surface (AC3).
    pub const fn is_docs_help(self) -> bool {
        matches!(self, Self::HelpCenterDocs)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskRunPane => "task_run_pane",
            Self::TestExplorer => "test_explorer",
            Self::RequestRunPane => "request_run_pane",
            Self::DatabaseExecutionPane => "database_execution_pane",
            Self::NotebookExecutionCell => "notebook_execution_cell",
            Self::PreviewRuntimeLane => "preview_runtime_lane",
            Self::AiMediatedRun => "ai_mediated_run",
            Self::PublishDeployFlow => "publish_deploy_flow",
            Self::SupportExportReplay => "support_export_replay",
            Self::HistoryActivityCenter => "history_activity_center",
            Self::HelpCenterDocs => "help_center_docs",
        }
    }
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, compare-only,
/// export-only, policy-blocked) but never rename or drop the governed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (drive the run, approve input, publish, debug).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Compare-only: read differences but take no action.
    CompareOnly,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::CompareOnly,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::CompareOnly => "compare_only",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot render the full
/// component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders the component in-place.
    None,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a read-only browser surface.
    BrowserReadonly,
    /// Punt to a portable handoff packet.
    HandoffPacket,
    /// Punt to the desktop primary execution UI.
    DesktopPrimary,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore
    /// must carry a companion / browser / handoff note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::HandoffPacket => "handoff_packet",
            Self::DesktopPrimary => "desktop_primary",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full identity / state / lineage / posture label parity.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// identity and state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// control it drops relative to the full execution surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical execution-lifecycle component family on
/// one M5 execution surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionConsumerRow {
    /// Record kind; must equal [`EXECUTION_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EXECUTION_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_group: ConsumerGroup,
    /// The concrete execution surface; must belong to `consumer_group`.
    pub consumer_surface: M5ExecutionConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5ExecutionComponentFamily,
    /// The canonical primitive schema for the family. Must equal
    /// `component_family.canonical_schema_ref()`.
    pub canonical_family_schema_ref: String,
    /// The canonical release-proof packet(s) this consumer points back to. Must
    /// contain `component_family.canonical_packet_ref()`.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local run/debug prose.
    pub references_canonical_not_local_prose: bool,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The degraded-state vocabulary the consumer keeps visible even when
    /// narrowed.
    #[serde(default)]
    pub degraded_state_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The companion / browser / handoff note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ExecutionConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared group matches the row's declared group.
    pub fn surface_group_consistent(&self) -> bool {
        self.consumer_surface.consumer_group() == self.consumer_group
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared schema matches the family, a release-proof packet is
    /// referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == self.component_family.canonical_schema_ref()
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == self.component_family.canonical_packet_ref())
            && self.references_canonical_not_local_prose
    }

    /// AC2 (parity): the consumer preserves the family's controlled label
    /// families and degraded-state vocabulary rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.degraded_state_vocab.is_empty()
    }

    /// AC2 (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a companion / browser / handoff note whenever it punts to another
    /// surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EXECUTION_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == EXECUTION_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_packet_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} group={group} family={family} authority={authority} \
label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            group = self.consumer_group.as_str(),
            family = self.component_family.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-825 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionConsumerSummary {
    pub row_count: usize,
    pub consumer_group_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub task_test_consumer_present: bool,
    pub request_database_consumer_present: bool,
    pub notebook_preview_consumer_present: bool,
    pub ai_publish_consumer_present: bool,
    pub support_export_consumer_present: bool,
    pub docs_help_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub families_reused_across_groups: usize,
}

/// Constructor input for [`ExecutionConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ExecutionConsumerRow>,
}

/// Checked-in M05-825 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ExecutionConsumerRow>,
    pub summary: ExecutionConsumerSummary,
}

impl ExecutionConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: ExecutionConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EXECUTION_CONSUMER_SCHEMA_VERSION,
            record_kind: EXECUTION_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ExecutionConsumerSummary {
                row_count: 0,
                consumer_group_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                task_test_consumer_present: false,
                request_database_consumer_present: false,
                notebook_preview_consumer_present: false,
                ai_publish_consumer_present: false,
                support_export_consumer_present: false,
                docs_help_reference_present: false,
                label_family_coverage_complete: false,
                families_reused_across_groups: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ExecutionComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// groups — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_groups(&self) -> usize {
        M5ExecutionComponentFamily::ALL
            .iter()
            .filter(|family| {
                let groups: BTreeSet<ConsumerGroup> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_group)
                    .collect();
                groups.len() >= 2
            })
            .count()
    }

    /// Whether some docs / help surface references the canonical families (AC3).
    pub fn has_docs_help_reference(&self) -> bool {
        self.rows
            .iter()
            .any(|r| r.consumer_surface.is_docs_help() && r.references_canonical_not_local_prose)
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ExecutionConsumerSummary {
        let mut groups = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            groups.insert(row.consumer_group);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_group = |g: ConsumerGroup| groups.contains(&g);
        let covered = self.covered_label_families();

        ExecutionConsumerSummary {
            row_count: self.rows.len(),
            consumer_group_count: groups.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(ExecutionConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(ExecutionConsumerRow::preserves_labels),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(ExecutionConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            task_test_consumer_present: has_group(ConsumerGroup::TaskTest),
            request_database_consumer_present: has_group(ConsumerGroup::RequestDatabase),
            notebook_preview_consumer_present: has_group(ConsumerGroup::NotebookPreview),
            ai_publish_consumer_present: has_group(ConsumerGroup::AiPublish),
            support_export_consumer_present: has_group(ConsumerGroup::SupportExport),
            docs_help_reference_present: self.has_docs_help_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            families_reused_across_groups: self.families_reused_across_groups(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ExecutionConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EXECUTION_CONSUMER_SCHEMA_VERSION {
            violations.push(ExecutionConsumerViolation::SchemaVersion {
                expected: EXECUTION_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXECUTION_CONSUMER_RECORD_KIND {
            violations.push(ExecutionConsumerViolation::RecordKind {
                expected: EXECUTION_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ExecutionConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_groups = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ExecutionConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_groups.insert(row.consumer_group);

            if !row.is_complete() {
                violations.push(ExecutionConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer group.
            if !row.surface_group_consistent() {
                violations.push(ExecutionConsumerViolation::SurfaceGroupMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(ExecutionConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC2: controlled label families / degraded vocab preserved.
            if !row.preserves_labels() {
                violations.push(ExecutionConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC2: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(ExecutionConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(ExecutionConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // Cross-surface reuse spans all five claimed consumer classes.
        for group in ConsumerGroup::ALL {
            if !seen_groups.contains(&group) {
                violations.push(ExecutionConsumerViolation::MissingConsumerGroup { group });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5ExecutionComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(ExecutionConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer groups
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_groups() == 0 {
            violations.push(ExecutionConsumerViolation::NoFamilyReusedAcrossGroups);
        }

        // AC2: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(ExecutionConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC3: a docs / help consumer references the canonical components rather
        // than cloning local run/debug semantics.
        if !self.has_docs_help_reference() {
            violations.push(ExecutionConsumerViolation::MissingDocsHelpReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(ExecutionConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(ExecutionConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_group,consumer_surface,component_family,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{group},{surface},{family},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                group = row.consumer_group.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Execution-Lifecycle Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer groups and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_group_count,
            self.represented_families().len(),
            M5ExecutionComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across groups: {}\n",
            self.summary.families_reused_across_groups,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_execution_lifecycle_component_consumers_export(
) -> Result<ExecutionConsumerPacket, ExecutionConsumerArtifactError> {
    let packet: ExecutionConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-execution-lifecycle-component-consumer-proof/support_export.json"
    )))
    .map_err(ExecutionConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExecutionConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum ExecutionConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExecutionConsumerViolation>),
}

impl fmt::Display for ExecutionConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ExecutionConsumerArtifactError {}

/// Validation failure for M05-825 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionConsumerViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    SurfaceGroupMismatch { id: String },
    NotCanonicalFamily { id: String },
    LabelParityBroken { id: String },
    NarrowedWithoutDisclosure { id: String },
    MissingCopyExportParity { id: String },
    MissingConsumerGroup { group: ConsumerGroup },
    MissingFamilyCoverage { family: M5ExecutionComponentFamily },
    NoFamilyReusedAcrossGroups,
    MissingLabelFamily { family: String },
    MissingDocsHelpReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for ExecutionConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceGroupMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer group"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical run/attempt-identity, outcome-state, \
rerun-context, artifact-lineage, or debug-control-posture label"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::MissingConsumerGroup { group } => {
                write!(f, "consumer group {group:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossGroups => write!(
                f,
                "no component family is adopted across two or more consumer groups"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingDocsHelpReference => write!(
                f,
                "no docs / help consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for ExecutionConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "read only"
            | "read-only"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests, the example dump, and the on-disk support export
/// so all three stay byte-aligned.
pub fn seeded_m5_execution_lifecycle_component_consumers_packet() -> ExecutionConsumerPacket {
    ExecutionConsumerPacket::new(ExecutionConsumerPacketInput {
        packet_id: "m5-execution-lifecycle-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: EXECUTION_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:execution-lifecycle-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn degraded_vocab() -> Vec<String> {
    vec![
        "queued".to_owned(),
        "waiting_input".to_owned(),
        "partially_complete".to_owned(),
        "stale_output".to_owned(),
        "cancelled".to_owned(),
    ]
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5ExecutionConsumerSurface,
    component_family: M5ExecutionComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> ExecutionConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    ExecutionConsumerRow {
        record_kind: EXECUTION_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: EXECUTION_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_group: consumer_surface.consumer_group(),
        consumer_surface,
        component_family,
        canonical_family_schema_ref: component_family.canonical_schema_ref().to_owned(),
        canonical_packet_refs: vec![component_family.canonical_packet_ref().to_owned()],
        references_canonical_not_local_prose: true,
        authority_mode,
        preserved_label_families: labels(label_families),
        degraded_state_vocab: degraded_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        source_refs: vec![EXECUTION_CONSUMER_MATRIX_REF.to_owned()],
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<ExecutionConsumerRow> {
    use AuthorityMode::*;
    use M5ExecutionComponentFamily::*;
    use M5ExecutionConsumerSurface::*;

    vec![
        // --- Task / test consumer ------------------------------------------
        // Task-run pane driving the run/attempt header full-interactive.
        row(
            "consumer:task-test:run-attempt-header",
            TaskRunPane,
            RunAttemptHeader,
            FullInteractive,
            &["run_attempt_identity", "outcome_state"],
            &["run_id", "attempt_id", "outcome_state"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Test explorer driving the rerun-comparison review full-interactive.
        row(
            "consumer:task-test:rerun-review",
            TestExplorer,
            RerunReview,
            FullInteractive,
            &["rerun_context_difference", "outcome_state"],
            &["rerun_mode", "changed_context", "prior_attempt_ref"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Task-run pane hosting the debug-session hierarchy full-interactive.
        row(
            "consumer:task-test:debug-hierarchy",
            TaskRunPane,
            DebugHierarchy,
            FullInteractive,
            &["debug_control_posture"],
            &["session_id", "control_posture", "dump_ref"],
            HandoffTarget::None,
            "",
            None,
        ),
        // --- Request / database execution lane -----------------------------
        // Request-run pane reusing the run/attempt header read-only (2nd group).
        row(
            "consumer:request-database:run-attempt-header",
            RequestRunPane,
            RunAttemptHeader,
            ReadOnly,
            &["run_attempt_identity", "outcome_state"],
            &["run_id", "attempt_id", "outcome_state"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:request-database:run-attempt-header",
                "Read-only request run header: read run-versus-attempt identity and outcome; re-dispatch stays on the task-run pane",
                ReadOnly,
                &["dispatch_rerun", "cancel_run"],
            )),
        ),
        // Database execution pane driving the input-request prompt.
        row(
            "consumer:request-database:input-request-prompt",
            DatabaseExecutionPane,
            InputRequestPrompt,
            FullInteractive,
            &["run_attempt_identity", "outcome_state"],
            &["prompt_id", "consequence", "timeout_state"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Request-run pane publishing produced artifacts full-interactive.
        row(
            "consumer:request-database:artifact-publish-row",
            RequestRunPane,
            ArtifactPublishRow,
            FullInteractive,
            &["artifact_lineage_retention", "outcome_state"],
            &["artifact_id", "producing_run_ref", "retention_class"],
            HandoffTarget::None,
            "",
            None,
        ),
        // --- Notebook / preview lane ---------------------------------------
        // Notebook cell reusing the run/attempt header full-interactive (3rd group).
        row(
            "consumer:notebook-preview:run-attempt-header",
            NotebookExecutionCell,
            RunAttemptHeader,
            FullInteractive,
            &["run_attempt_identity", "outcome_state"],
            &["run_id", "attempt_id", "outcome_state"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Notebook cell driving the input-request prompt (2nd group for family).
        row(
            "consumer:notebook-preview:input-request-prompt",
            NotebookExecutionCell,
            InputRequestPrompt,
            FullInteractive,
            &["run_attempt_identity", "outcome_state"],
            &["prompt_id", "consequence", "timeout_state"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Preview-runtime lane reusing the artifact-publish row read-only (2nd group).
        row(
            "consumer:notebook-preview:artifact-publish-row",
            PreviewRuntimeLane,
            ArtifactPublishRow,
            ReadOnly,
            &["artifact_lineage_retention", "outcome_state"],
            &["artifact_id", "producing_run_ref", "retention_class"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:notebook-preview:artifact-publish-row",
                "Read-only preview artifact row: read the producing-run lineage and retention truth; republishing stays on the request pane",
                ReadOnly,
                &["republish_artifact", "delete_artifact"],
            )),
        ),
        // --- AI / publish or remote-mutation lane --------------------------
        // AI-mediated run reusing the rerun-review inspect-only (2nd group).
        row(
            "consumer:ai-publish:rerun-review",
            AiMediatedRun,
            RerunReview,
            InspectOnly,
            &["rerun_context_difference", "outcome_state"],
            &["rerun_mode", "changed_context", "prior_attempt_ref"],
            HandoffTarget::CompanionApp,
            "handoff:ai-publish:rerun-review-open-in-task-pane",
            Some(banner(
                "banner:ai-publish:rerun-review",
                "Inspect-only agent rerun review: read exact-versus-current-context differences before the agent proposes a retry; dispatch stays with the user",
                InspectOnly,
                &["dispatch_rerun", "edit_rerun_context"],
            )),
        ),
        // Publish / deploy flow reusing the artifact-publish row full-interactive (3rd group).
        row(
            "consumer:ai-publish:artifact-publish-row",
            PublishDeployFlow,
            ArtifactPublishRow,
            FullInteractive,
            &["artifact_lineage_retention", "outcome_state"],
            &["artifact_id", "producing_run_ref", "retention_class"],
            HandoffTarget::None,
            "",
            None,
        ),
        // AI-mediated run reusing the debug-session hierarchy inspect-only (2nd group).
        row(
            "consumer:ai-publish:debug-hierarchy",
            AiMediatedRun,
            DebugHierarchy,
            InspectOnly,
            &["debug_control_posture"],
            &["session_id", "control_posture", "dump_ref"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:ai-publish:debug-hierarchy",
                "Inspect-only agent debug view: read live-attached-control versus captured-analysis posture; the agent never claims live control it does not hold",
                InspectOnly,
                &["continue_execution", "pause_execution", "detach"],
            )),
        ),
        // --- Support / export lane -----------------------------------------
        // Support / export replay reconstructing the run/attempt header export-only (4th group).
        row(
            "consumer:support-export:run-attempt-header",
            SupportExportReplay,
            RunAttemptHeader,
            ExportOnly,
            &["run_attempt_identity", "outcome_state"],
            &["run_id", "attempt_id", "outcome_state"],
            HandoffTarget::HandoffPacket,
            "handoff:support-export:run-attempt-header-packet",
            Some(banner(
                "banner:support-export:run-attempt-header",
                "Export-only support replay: reconstruct run-versus-attempt identity and outcome state from the support packet; open the desktop app to act",
                ExportOnly,
                &["dispatch_rerun", "cancel_run"],
            )),
        ),
        // Run history / activity center reusing the rerun-review read-only (3rd group).
        row(
            "consumer:support-export:rerun-review",
            HistoryActivityCenter,
            RerunReview,
            ReadOnly,
            &["rerun_context_difference", "outcome_state"],
            &["rerun_mode", "changed_context", "prior_attempt_ref"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:support-export:rerun-review",
                "Read-only history rerun review: read the retry scope and changed context of a prior attempt; re-dispatch opens the task-run pane",
                ReadOnly,
                &["dispatch_rerun"],
            )),
        ),
        // Docs / help center referencing the debug-session hierarchy read-only (AC3, 3rd group).
        row(
            "consumer:support-export:debug-hierarchy-docs",
            HelpCenterDocs,
            DebugHierarchy,
            ReadOnly,
            &["debug_control_posture"],
            &["session_id", "control_posture", "dump_ref"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:support-export:debug-hierarchy-docs",
                "Read-only help reference: explains launch / attach / core / replay / inspect-only posture and live-versus-captured control for each debug surface",
                ReadOnly,
                &["continue_execution", "detach"],
            )),
        ),
    ]
}
