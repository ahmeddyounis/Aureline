//! Bounded notebook alpha lane for trust, diff, repair, and export truth.
//!
//! This module is the first shell-owned projection that joins the earlier
//! notebook trust-badge wedge with notebook document identity, kernel/session
//! refs, output lineage honesty, cell-aware diff posture, structured repair
//! consequences, and export-scope separation. It does not implement a notebook
//! editor, kernel host, merge driver, or widget runtime. It emits one
//! inspectable lane record and a support-export projection that product chrome,
//! fixtures, and future runtime surfaces can quote without re-minting the
//! contract.

use std::collections::{BTreeMap, BTreeSet};

use aureline_content_safety::{BodyPosture, RepresentationClass, TrustClass};
use serde::{Deserialize, Serialize};

use crate::document_identity::{DocumentFamilyClass, DocumentIdentityDisclosure};
use crate::notebook_trust_badges::{
    CellContentClass, EscapeHatch, KernelAvailability, NotebookTrustRung, OutputTrustState,
    RepresentationState, WidgetTrustState, WorkspaceTrustState,
};

/// Stable record-kind tag for [`NotebookAlphaLaneRecord`].
pub const NOTEBOOK_ALPHA_LANE_RECORD_KIND: &str = "notebook_alpha_lane_record";

/// Schema version for the bounded notebook alpha lane.
pub const NOTEBOOK_ALPHA_LANE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`NotebookAlphaSupportExportRecord`].
pub const NOTEBOOK_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str = "notebook_alpha_support_export_record";

/// Prototype label carried on every bounded notebook alpha lane record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookAlphaPrototypeLabel {
    /// Bounded prototype covering trust, diff, repair, and export truth for one notebook lane.
    BoundedNotebookTrustDiffRepairExport,
}

impl NotebookAlphaPrototypeLabel {
    /// Returns the stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedNotebookTrustDiffRepairExport => {
                "bounded_notebook_trust_diff_repair_export"
            }
        }
    }

    /// Returns the reviewer-facing label for this prototype.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BoundedNotebookTrustDiffRepairExport => {
                "Prototype notebook trust, diff, repair, and paired export"
            }
        }
    }
}

/// Claim limits that make the prototype's supported scope explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookAlphaClaimLimit {
    /// The lane covers one bounded notebook document only.
    SingleBoundedNotebookDocumentLane,
    /// The lane records kernel/session refs but does not execute cells.
    NoKernelExecutionRuntime,
    /// The lane records widget posture but does not admit widget live binding.
    NoWidgetAdmissionRuntime,
    /// The lane records diff posture but does not install a merge driver.
    NoNotebookMergeDriver,
    /// The lane records repair consequences but does not apply repairs automatically.
    RepairPreviewOnlyUntilExplicitApply,
}

impl NotebookAlphaClaimLimit {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleBoundedNotebookDocumentLane => "single_bounded_notebook_document_lane",
            Self::NoKernelExecutionRuntime => "no_kernel_execution_runtime",
            Self::NoWidgetAdmissionRuntime => "no_widget_admission_runtime",
            Self::NoNotebookMergeDriver => "no_notebook_merge_driver",
            Self::RepairPreviewOnlyUntilExplicitApply => "repair_preview_only_until_explicit_apply",
        }
    }

    /// Returns the reviewer-facing claim-limit label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleBoundedNotebookDocumentLane => {
                "One bounded notebook document lane; not full notebook product scope."
            }
            Self::NoKernelExecutionRuntime => {
                "Kernel and session refs are inspectable, but this lane does not execute cells."
            }
            Self::NoWidgetAdmissionRuntime => {
                "Widget trust is labelled, but live widget admission is outside this lane."
            }
            Self::NoNotebookMergeDriver => {
                "Cell-aware diff posture is recorded; no notebook merge driver is claimed."
            }
            Self::RepairPreviewOnlyUntilExplicitApply => {
                "Repair consequences are previewed before any durable rewrite."
            }
        }
    }

    /// Returns the canonical scope-limiting set in stable display order.
    pub const fn canonical_set() -> [NotebookAlphaClaimLimit; 5] {
        [
            Self::SingleBoundedNotebookDocumentLane,
            Self::NoKernelExecutionRuntime,
            Self::NoWidgetAdmissionRuntime,
            Self::NoNotebookMergeDriver,
            Self::RepairPreviewOnlyUntilExplicitApply,
        ]
    }
}

/// One rendered claim-limit row on the lane record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAlphaClaimLimitRow {
    /// Stable claim-limit token.
    pub token: String,
    /// Reviewer-facing claim-limit label.
    pub label: String,
}

impl NotebookAlphaClaimLimitRow {
    fn from_limit(limit: NotebookAlphaClaimLimit) -> Self {
        Self {
            token: limit.as_str().to_owned(),
            label: limit.label().to_owned(),
        }
    }
}

/// Trust axes that must stay visually distinct on the notebook lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAlphaTrustAxes {
    /// Workspace trust posture from the shell/workspace trust vocabulary.
    pub workspace_trust_state: WorkspaceTrustState,
    /// Notebook file trust rung from the notebook trust ladder.
    pub notebook_trust_rung: NotebookTrustRung,
    /// Kernel/session availability posture.
    pub kernel_availability: KernelAvailability,
    /// Output trust posture for the current lane.
    pub output_trust_state: OutputTrustState,
    /// Widget trust posture for the current lane.
    pub widget_trust_state: WidgetTrustState,
    /// Safe-preview trust class currently used for rendered rich notebook content.
    pub rendered_output_trust_class: TrustClass,
}

impl NotebookAlphaTrustAxes {
    /// Returns the workspace trust token.
    pub fn workspace_token(&self) -> &'static str {
        self.workspace_trust_state.as_str()
    }

    /// Returns the notebook trust token.
    pub fn notebook_token(&self) -> &'static str {
        self.notebook_trust_rung.as_str()
    }

    /// Returns the kernel availability token.
    pub fn kernel_token(&self) -> &'static str {
        self.kernel_availability.as_str()
    }

    /// Returns the output trust token.
    pub fn output_token(&self) -> &'static str {
        self.output_trust_state.as_str()
    }

    /// Returns the widget trust token.
    pub fn widget_token(&self) -> &'static str {
        self.widget_trust_state.as_str()
    }
}

/// Kind of notebook metadata namespace observed in the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookMetadataNamespaceKind {
    /// Jupyter-owned metadata namespace.
    JupyterCore,
    /// Aureline-owned metadata namespace.
    AurelineOwned,
    /// Known vendor namespace such as `vscode`, `colab`, or `nteract`.
    KnownVendor,
    /// Unknown vendor namespace that must round-trip without silent loss.
    UnknownVendor,
}

impl NotebookMetadataNamespaceKind {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JupyterCore => "jupyter_core",
            Self::AurelineOwned => "aureline_owned",
            Self::KnownVendor => "known_vendor",
            Self::UnknownVendor => "unknown_vendor",
        }
    }
}

/// Round-trip survival posture for metadata namespaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataSurvivalPosture {
    /// Namespace is preserved byte-for-byte or structurally equivalent.
    PreserveVerbatim,
    /// Namespace is intentionally filtered after explicit review.
    FilteredWithReview,
    /// Namespace is metadata-only and cannot be written back.
    MetadataOnlyInspect,
    /// Namespace would be dropped.
    Dropped,
}

impl MetadataSurvivalPosture {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreserveVerbatim => "preserve_verbatim",
            Self::FilteredWithReview => "filtered_with_review",
            Self::MetadataOnlyInspect => "metadata_only_inspect",
            Self::Dropped => "dropped",
        }
    }
}

/// One metadata namespace survival row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookMetadataNamespaceRecord {
    /// Namespace key or namespace family label.
    pub namespace: String,
    /// Namespace kind.
    pub namespace_kind: NotebookMetadataNamespaceKind,
    /// Stable token for [`Self::namespace_kind`].
    pub namespace_kind_token: String,
    /// Round-trip survival posture.
    pub survival_posture: MetadataSurvivalPosture,
    /// Stable token for [`Self::survival_posture`].
    pub survival_posture_token: String,
    /// Opaque ref to the inventory or survival report entry.
    pub inventory_ref: String,
}

impl NotebookMetadataNamespaceRecord {
    fn refresh_tokens(&mut self) {
        self.namespace_kind_token = self.namespace_kind.as_str().to_owned();
        self.survival_posture_token = self.survival_posture.as_str().to_owned();
    }
}

/// Round-trip posture for a notebook attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentRoundtripPosture {
    /// Attachment payload and metadata are preserved.
    PreserveVerbatim,
    /// Attachment is preserved as a detached artifact ref.
    PreserveDetachedReference,
    /// Attachment is metadata-only because policy or size prevents body capture.
    MetadataOnlyWithReason,
    /// Attachment would be dropped.
    Dropped,
}

impl AttachmentRoundtripPosture {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreserveVerbatim => "preserve_verbatim",
            Self::PreserveDetachedReference => "preserve_detached_reference",
            Self::MetadataOnlyWithReason => "metadata_only_with_reason",
            Self::Dropped => "dropped",
        }
    }

    fn is_preserved(self) -> bool {
        matches!(
            self,
            Self::PreserveVerbatim | Self::PreserveDetachedReference
        )
    }
}

/// One attachment survival row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAttachmentRecord {
    /// Stable attachment id.
    pub attachment_id: String,
    /// Stable cell id this attachment belongs to.
    pub cell_id_ref: String,
    /// MIME family token for the attachment.
    pub mime_family: String,
    /// Opaque payload or detached-artifact ref.
    pub payload_ref: String,
    /// Attachment round-trip posture.
    pub roundtrip_posture: AttachmentRoundtripPosture,
    /// Stable token for [`Self::roundtrip_posture`].
    pub roundtrip_posture_token: String,
    /// Reviewer-facing reason when the attachment is not directly preserved.
    pub consequence_label: String,
}

impl NotebookAttachmentRecord {
    fn refresh_tokens(&mut self) {
        self.roundtrip_posture_token = self.roundtrip_posture.as_str().to_owned();
    }
}

/// Notebook document object carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentAlphaObject {
    /// Stable notebook document id.
    pub document_id: String,
    /// Display path or URI chosen by the user.
    pub presentation_uri: String,
    /// VFS canonical object ref used for exact save-target truth.
    pub canonical_object_ref: String,
    /// Route ref used to reopen or inspect this document in the shell.
    pub route_ref: String,
    /// Notebook format label such as `nbformat-4.5`.
    pub nbformat: String,
    /// Whether `.ipynb` remains the canonical source for the lane.
    pub canonical_ipynb_is_source: bool,
    /// Root/path/save-target disclosure for this notebook document.
    pub identity_disclosure: DocumentIdentityDisclosure,
    /// Metadata namespace inventory.
    pub metadata_namespaces: Vec<NotebookMetadataNamespaceRecord>,
    /// Attachment survival rows.
    pub attachments: Vec<NotebookAttachmentRecord>,
}

impl NotebookDocumentAlphaObject {
    fn refresh_tokens(&mut self) {
        self.identity_disclosure = self.identity_disclosure.clone().normalized();
        for namespace in &mut self.metadata_namespaces {
            namespace.refresh_tokens();
        }
        for attachment in &mut self.attachments {
            attachment.refresh_tokens();
        }
    }
}

/// Cell execution-state class for the alpha lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookCellExecutionState {
    /// Cell has never been run in the known lane.
    NotRun,
    /// Cell output came from the active session.
    RanCurrentSession,
    /// Cell source changed after the last output.
    SourceChangedOutputStale,
    /// No kernel is available for execution.
    ExecutionUnavailableNoKernel,
    /// Execution is blocked by trust or policy.
    ExecutionBlockedByTrustOrPolicy,
}

impl NotebookCellExecutionState {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::RanCurrentSession => "ran_current_session",
            Self::SourceChangedOutputStale => "source_changed_output_stale",
            Self::ExecutionUnavailableNoKernel => "execution_unavailable_no_kernel",
            Self::ExecutionBlockedByTrustOrPolicy => "execution_blocked_by_trust_or_policy",
        }
    }
}

/// Notebook cell object carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCellAlphaObject {
    /// Stable cell object id.
    pub cell_object_id: String,
    /// Stable notebook cell id from the `.ipynb` cell.
    pub stable_cell_id: String,
    /// Document id this cell belongs to.
    pub document_id_ref: String,
    /// Route ref for opening or diffing this cell.
    pub route_ref: String,
    /// Cell content class.
    pub content_class: CellContentClass,
    /// Stable token for [`Self::content_class`].
    pub content_class_token: String,
    /// Cell execution posture.
    pub execution_state: NotebookCellExecutionState,
    /// Stable token for [`Self::execution_state`].
    pub execution_state_token: String,
    /// Output ids attached to this cell.
    pub output_record_ids: Vec<String>,
    /// Attachment ids attached to this cell.
    pub attachment_ids: Vec<String>,
}

impl NotebookCellAlphaObject {
    fn refresh_tokens(&mut self) {
        self.content_class_token = self.content_class.as_str().to_owned();
        self.execution_state_token = self.execution_state.as_str().to_owned();
    }
}

/// Kernel session lifecycle posture shown on the alpha lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelSessionLifecycle {
    /// Kernel is ready.
    Ready,
    /// Kernel is busy.
    Busy,
    /// Kernel was detached; prior outputs are evidence only.
    Detached,
    /// Kernel is unavailable.
    Unavailable,
    /// Kernel is blocked by trust or policy.
    PolicyBlocked,
}

impl KernelSessionLifecycle {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Busy => "busy",
            Self::Detached => "detached",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Kernel session object carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookKernelSessionAlphaObject {
    /// Stable kernel session id.
    pub kernel_session_id: String,
    /// Document id this session is bound to.
    pub document_id_ref: String,
    /// Execution-context ref that admitted or denied the session.
    pub execution_context_ref: String,
    /// Route ref for inspecting the kernel/session detail.
    pub route_ref: String,
    /// Kernelspec descriptor ref.
    pub kernelspec_ref: String,
    /// Kernel lifecycle state.
    pub lifecycle: KernelSessionLifecycle,
    /// Stable token for [`Self::lifecycle`].
    pub lifecycle_token: String,
    /// Kernel/session trust label shown to the user.
    pub trust_label: String,
    /// Environment or interpreter fingerprint ref.
    pub environment_fingerprint_ref: String,
}

impl NotebookKernelSessionAlphaObject {
    fn refresh_tokens(&mut self) {
        self.lifecycle_token = self.lifecycle.as_str().to_owned();
    }
}

/// Freshness posture for a notebook output record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookOutputFreshnessClass {
    /// Output is live from the current session.
    LiveCurrentSession,
    /// Output was captured from a prior session.
    CapturedPriorSession,
    /// Output became stale after cell, kernel, environment, or data drift.
    StaleAfterDrift,
    /// Output was imported from an artifact payload or support capture.
    ImportedArtifactPayload,
    /// Output is replay evidence from captured output.
    ReplayedEvidence,
    /// Output has no resolvable kernel binding.
    OrphanedNoKernelBinding,
    /// Widget output is blocked and rendered as static fallback.
    WidgetBlockedStaticFallback,
}

impl NotebookOutputFreshnessClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCurrentSession => "live_current_session",
            Self::CapturedPriorSession => "captured_prior_session",
            Self::StaleAfterDrift => "stale_after_drift",
            Self::ImportedArtifactPayload => "imported_artifact_payload",
            Self::ReplayedEvidence => "replayed_evidence",
            Self::OrphanedNoKernelBinding => "orphaned_no_kernel_binding",
            Self::WidgetBlockedStaticFallback => "widget_blocked_static_fallback",
        }
    }

    fn is_live(self) -> bool {
        matches!(self, Self::LiveCurrentSession)
    }
}

/// Notebook output record carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputAlphaObject {
    /// Stable output record id.
    pub output_record_id: String,
    /// Stable cell id this output belongs to.
    pub cell_id_ref: String,
    /// Kernel session id that produced or last owned this output, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Cell-execution ref that produced this output, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_execution_ref: Option<String>,
    /// Execution-context ref tied to the output, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    /// Output freshness posture.
    pub freshness: NotebookOutputFreshnessClass,
    /// Stable token for [`Self::freshness`].
    pub freshness_token: String,
    /// Output trust state from the notebook trust vocabulary.
    pub output_trust_state: OutputTrustState,
    /// Stable token for [`Self::output_trust_state`].
    pub output_trust_state_token: String,
    /// Widget trust state for widget outputs or `not_applicable` otherwise.
    pub widget_trust_state: WidgetTrustState,
    /// Stable token for [`Self::widget_trust_state`].
    pub widget_trust_state_token: String,
    /// Output content class.
    pub content_class: CellContentClass,
    /// Stable token for [`Self::content_class`].
    pub content_class_token: String,
    /// Representation currently rendered to the user.
    pub representation_state: RepresentationState,
    /// Stable token for [`Self::representation_state`].
    pub representation_state_token: String,
    /// Safe-preview trust class used for the rendered output.
    pub trust_class: TrustClass,
    /// Token from the output viewer truth contract, such as `stale_output`.
    pub viewer_truth_state_token: String,
    /// Token from the kernel-output lineage contract.
    pub lineage_class_token: String,
    /// True when the row visibly marks stale/captured/imported/replayed state.
    pub honesty_marker_present: bool,
    /// Safe next actions shown next to the output.
    pub escape_hatches: Vec<EscapeHatch>,
}

impl NotebookOutputAlphaObject {
    fn refresh_tokens(&mut self) {
        self.freshness_token = self.freshness.as_str().to_owned();
        self.output_trust_state_token = self.output_trust_state.as_str().to_owned();
        self.widget_trust_state_token = self.widget_trust_state.as_str().to_owned();
        self.content_class_token = self.content_class.as_str().to_owned();
        self.representation_state_token = self.representation_state.as_str().to_owned();
    }
}

/// Cell-aware diff mode for the alpha lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookDiffMode {
    /// Cell-aware diff is available.
    CellAware,
    /// Raw JSON fallback is used.
    RawJsonFallback,
    /// Compare-only posture with no merge/apply.
    CompareOnly,
}

impl NotebookDiffMode {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CellAware => "cell_aware",
            Self::RawJsonFallback => "raw_json_fallback",
            Self::CompareOnly => "compare_only",
        }
    }
}

/// Diff or review affordance shown on the bounded lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookDiffAffordance {
    /// Open a cell-aware diff.
    OpenCellAwareDiff,
    /// Switch to raw JSON fallback.
    OpenRawJsonFallback,
    /// Toggle output include/exclude posture.
    ToggleOutputInclusion,
    /// Inspect metadata filter/survival state.
    InspectMetadataSurvival,
    /// Inspect attachment changes.
    InspectAttachmentDiff,
    /// Open repair preview.
    OpenRepairPreview,
}

impl NotebookDiffAffordance {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCellAwareDiff => "open_cell_aware_diff",
            Self::OpenRawJsonFallback => "open_raw_json_fallback",
            Self::ToggleOutputInclusion => "toggle_output_inclusion",
            Self::InspectMetadataSurvival => "inspect_metadata_survival",
            Self::InspectAttachmentDiff => "inspect_attachment_diff",
            Self::OpenRepairPreview => "open_repair_preview",
        }
    }
}

/// Diff posture record carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDiffPostureRecord {
    /// Stable diff posture id.
    pub diff_posture_id: String,
    /// Review session id this diff belongs to.
    pub review_session_id_ref: String,
    /// Diff mode.
    pub diff_mode: NotebookDiffMode,
    /// Stable token for [`Self::diff_mode`].
    pub diff_mode_token: String,
    /// Count of changed source cells.
    pub source_change_count: u32,
    /// Count of changed metadata fields.
    pub metadata_change_count: u32,
    /// Count of changed output blocks.
    pub output_change_count: u32,
    /// Count of changed attachments.
    pub attachment_change_count: u32,
    /// True when raw JSON fallback is available.
    pub raw_json_fallback_available: bool,
    /// Metadata filter state label shown to the reviewer.
    pub metadata_filter_state: String,
    /// Output include/exclude state label shown to the reviewer.
    pub output_include_state: String,
    /// Diff affordances visible in the lane.
    pub affordances: Vec<NotebookDiffAffordance>,
}

impl NotebookDiffPostureRecord {
    fn refresh_tokens(&mut self) {
        self.diff_mode_token = self.diff_mode.as_str().to_owned();
    }

    fn affordance_tokens(&self) -> BTreeSet<&'static str> {
        self.affordances
            .iter()
            .map(|affordance| affordance.as_str())
            .collect()
    }
}

/// Repair-class family from the shared repair-transaction contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairClassFamily {
    /// Observe-only row with no repair write.
    ObserveOnlyNoRepair,
    /// Rebuild or regenerate disposable/derived state.
    DisposableStateRebuild,
    /// Re-resolve an execution-context handle.
    ExecutionContextReresolve,
    /// Refresh policy or entitlement state without widening trust.
    PolicyEntitlementRefresh,
    /// Prepare an escalation/export packet instead of applying locally.
    GuidedExportEscalation,
}

impl RepairClassFamily {
    /// Returns the stable token used in repair records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObserveOnlyNoRepair => "observe_only_no_repair",
            Self::DisposableStateRebuild => "disposable_state_rebuild",
            Self::ExecutionContextReresolve => "execution_context_reresolve",
            Self::PolicyEntitlementRefresh => "policy_entitlement_refresh",
            Self::GuidedExportEscalation => "guided_export_escalation",
        }
    }
}

/// Apply mode from the shared repair-transaction contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairApplyModeClass {
    /// Preview only; no apply step runs.
    DryRunPreviewOnly,
    /// Apply requires a pre-apply checkpoint.
    ApplyWithCheckpoint,
    /// Apply requires checkpoint plus rollback on failure.
    ApplyWithRollbackOnFailure,
    /// Observe-only; no write occurs.
    ApplyObserveOnlyNoWrite,
    /// Apply is refused and only escalation/export remains.
    ApplyRefusedEscalationOnly,
}

impl RepairApplyModeClass {
    /// Returns the stable token used in repair records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DryRunPreviewOnly => "dry_run_preview_only",
            Self::ApplyWithCheckpoint => "apply_with_checkpoint",
            Self::ApplyWithRollbackOnFailure => "apply_with_rollback_on_failure",
            Self::ApplyObserveOnlyNoWrite => "apply_observe_only_no_write",
            Self::ApplyRefusedEscalationOnly => "apply_refused_escalation_only",
        }
    }

    fn requires_checkpoint(self) -> bool {
        matches!(
            self,
            Self::ApplyWithCheckpoint | Self::ApplyWithRollbackOnFailure
        )
    }
}

/// Transaction reversal class from the shared repair-transaction contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionReversalClass {
    /// Reversal restores prior state exactly.
    Exact,
    /// Reversal is compensating, not exact.
    Compensating,
    /// Reversal regenerates derived state from authoritative inputs.
    Regenerate,
    /// Reversal requires a manual user action.
    Manual,
    /// No state changed; only audit records exist.
    AuditOnly,
}

impl TransactionReversalClass {
    /// Returns the stable token used in repair records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compensating => "compensating",
            Self::Regenerate => "regenerate",
            Self::Manual => "manual",
            Self::AuditOnly => "audit_only",
        }
    }
}

/// Structured repair action or repair preview available from the lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRepairActionRecord {
    /// Stable repair action id.
    pub repair_action_id: String,
    /// Repair-class family.
    pub repair_class_family: RepairClassFamily,
    /// Stable token for [`Self::repair_class_family`].
    pub repair_class_family_token: String,
    /// Apply mode.
    pub apply_mode_class: RepairApplyModeClass,
    /// Stable token for [`Self::apply_mode_class`].
    pub apply_mode_class_token: String,
    /// Transaction reversal class.
    pub transaction_reversal_class: TransactionReversalClass,
    /// Stable token for [`Self::transaction_reversal_class`].
    pub transaction_reversal_class_token: String,
    /// True when a preview is required before apply.
    pub preview_required: bool,
    /// Opaque repair-preview record ref.
    pub repair_preview_ref: String,
    /// Pre-apply checkpoint ref when required by apply mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Affected cell ids.
    pub affected_cell_ids: Vec<String>,
    /// Affected output ids.
    pub affected_output_ids: Vec<String>,
    /// Reviewer-facing consequence label.
    pub consequence_label: String,
    /// Safe next-action labels.
    pub safe_next_actions: Vec<String>,
}

impl NotebookRepairActionRecord {
    fn refresh_tokens(&mut self) {
        self.repair_class_family_token = self.repair_class_family.as_str().to_owned();
        self.apply_mode_class_token = self.apply_mode_class.as_str().to_owned();
        self.transaction_reversal_class_token = self.transaction_reversal_class.as_str().to_owned();
    }
}

/// Export scope class used to keep `.ipynb`, rendered reports, and raw payloads separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookArtifactScopeClass {
    /// Canonical `.ipynb` document scope.
    CanonicalIpynb,
    /// Rendered report or static rich-output report scope.
    RenderedReport,
    /// Raw artifact-payload scope.
    RawArtifactPayload,
}

impl NotebookArtifactScopeClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalIpynb => "canonical_ipynb",
            Self::RenderedReport => "rendered_report",
            Self::RawArtifactPayload => "raw_artifact_payload",
        }
    }
}

/// One export-scope row on the lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookExportScopeRecord {
    /// Stable export scope id.
    pub export_scope_id: String,
    /// Scope class.
    pub scope_class: NotebookArtifactScopeClass,
    /// Stable token for [`Self::scope_class`].
    pub scope_class_token: String,
    /// Artifact or packet ref produced for this scope.
    pub artifact_ref: String,
    /// Representation class used when the scope leaves the product.
    pub representation_class: RepresentationClass,
    /// Stable token for [`Self::representation_class`].
    pub representation_class_token: String,
    /// Body posture used when the scope leaves the product.
    pub body_posture: BodyPosture,
    /// Stable token for [`Self::body_posture`].
    pub body_posture_token: String,
    /// Include-policy ref for notebook outputs.
    pub output_include_policy_ref: String,
    /// Trust class for this export scope.
    pub trust_class: TrustClass,
    /// Scope-specific stale/imported-output label.
    pub stale_or_imported_output_label: String,
}

impl NotebookExportScopeRecord {
    fn refresh_tokens(&mut self) {
        self.scope_class_token = self.scope_class.as_str().to_owned();
        self.representation_class_token = self.representation_class.as_str().to_owned();
        self.body_posture_token = self.body_posture.as_str().to_owned();
    }
}

/// State of a paired text export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PairedExportState {
    /// Paired export is derived from the canonical notebook.
    DerivedFromCanonicalNotebook,
    /// Paired export has diverged and needs review.
    DivergedNeedsReview,
    /// Export was generated once and divergence is unknown.
    GeneratedOnceUnknownDivergence,
}

impl PairedExportState {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DerivedFromCanonicalNotebook => "derived_from_canonical_notebook",
            Self::DivergedNeedsReview => "diverged_needs_review",
            Self::GeneratedOnceUnknownDivergence => "generated_once_unknown_divergence",
        }
    }
}

/// Paired export object carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookPairedExportObject {
    /// Stable paired export id.
    pub paired_export_id: String,
    /// Document id this paired export derives from.
    pub document_id_ref: String,
    /// Route ref for opening the export review sheet.
    pub route_ref: String,
    /// Target format such as `percent_script`.
    pub target_format: String,
    /// Paired export state.
    pub export_state: PairedExportState,
    /// Stable token for [`Self::export_state`].
    pub export_state_token: String,
    /// Source-of-truth scope for the paired export.
    pub source_of_truth_scope: NotebookArtifactScopeClass,
    /// Stable token for [`Self::source_of_truth_scope`].
    pub source_of_truth_scope_token: String,
    /// Canonical direction ref, such as notebook to export script.
    pub canonical_direction_ref: String,
    /// Divergence state shown before overwrite/apply.
    pub divergence_state_label: String,
}

impl NotebookPairedExportObject {
    fn refresh_tokens(&mut self) {
        self.export_state_token = self.export_state.as_str().to_owned();
        self.source_of_truth_scope_token = self.source_of_truth_scope.as_str().to_owned();
    }
}

/// Reproducibility object carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookReproducibilityObject {
    /// Stable reproducibility object id.
    pub reproducibility_id: String,
    /// Document id this object summarizes.
    pub document_id_ref: String,
    /// Kernel session id this object summarizes.
    pub kernel_session_id_ref: String,
    /// Route ref for opening full execution-context detail.
    pub route_ref: String,
    /// Environment or interpreter fingerprint ref.
    pub environment_fingerprint_ref: String,
    /// Data snapshot ref, if known.
    pub data_snapshot_ref: String,
    /// Freshness or mismatch summary label.
    pub freshness_label: String,
}

/// Review session object carried by the alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookReviewSessionObject {
    /// Stable review session id.
    pub review_session_id: String,
    /// Route ref for opening the review session.
    pub route_ref: String,
    /// Document id being reviewed.
    pub document_id_ref: String,
    /// Diff posture id used by this review session.
    pub diff_posture_id_ref: String,
    /// Local review workspace ref.
    pub review_workspace_ref: String,
    /// Provider overlay state label, or local-only label.
    pub provider_overlay_state: String,
    /// Export scope ids available from this review session.
    pub export_scope_ids: Vec<String>,
}

/// One bounded notebook alpha lane record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAlphaLaneRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable lane id.
    pub lane_id: String,
    /// Prototype label enum.
    pub prototype_label: NotebookAlphaPrototypeLabel,
    /// Stable token for [`Self::prototype_label`].
    pub prototype_label_token: String,
    /// Reviewer-facing prototype label.
    pub prototype_label_display: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Distinct trust axes for the lane.
    pub trust_axes: NotebookAlphaTrustAxes,
    /// Notebook document object.
    pub document: NotebookDocumentAlphaObject,
    /// Cell objects in the lane.
    pub cells: Vec<NotebookCellAlphaObject>,
    /// Kernel session object when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session: Option<NotebookKernelSessionAlphaObject>,
    /// Output records in the lane.
    pub outputs: Vec<NotebookOutputAlphaObject>,
    /// Review session object.
    pub review_session: NotebookReviewSessionObject,
    /// Diff posture record.
    pub diff_posture: NotebookDiffPostureRecord,
    /// Repair action records.
    pub repair_actions: Vec<NotebookRepairActionRecord>,
    /// Export scope records.
    pub export_scopes: Vec<NotebookExportScopeRecord>,
    /// Paired export objects.
    pub paired_exports: Vec<NotebookPairedExportObject>,
    /// Reproducibility objects.
    pub reproducibility: Vec<NotebookReproducibilityObject>,
    /// Explicit claim limits.
    pub claim_limits: Vec<NotebookAlphaClaimLimitRow>,
    /// Downgrade behaviors supported by the lane.
    pub downgrade_behaviors: Vec<String>,
}

impl NotebookAlphaLaneRecord {
    /// Normalizes derived token fields after fixture deserialization.
    pub fn normalized(mut self) -> Self {
        let label = self.prototype_label;
        self.record_kind = NOTEBOOK_ALPHA_LANE_RECORD_KIND.to_owned();
        self.schema_version = NOTEBOOK_ALPHA_LANE_SCHEMA_VERSION;
        self.prototype_label_token = label.as_str().to_owned();
        self.prototype_label_display = label.label().to_owned();
        self.document.refresh_tokens();
        for cell in &mut self.cells {
            cell.refresh_tokens();
        }
        if let Some(kernel_session) = &mut self.kernel_session {
            kernel_session.refresh_tokens();
        }
        for output in &mut self.outputs {
            output.refresh_tokens();
        }
        self.diff_posture.refresh_tokens();
        for repair in &mut self.repair_actions {
            repair.refresh_tokens();
        }
        for scope in &mut self.export_scopes {
            scope.refresh_tokens();
        }
        for paired_export in &mut self.paired_exports {
            paired_export.refresh_tokens();
        }
        self.claim_limits = NotebookAlphaClaimLimit::canonical_set()
            .into_iter()
            .map(NotebookAlphaClaimLimitRow::from_limit)
            .collect();
        self
    }

    /// Validates the bounded lane against the alpha acceptance contract.
    pub fn validate(&self) -> Vec<NotebookAlphaViolation> {
        let mut out = Vec::new();

        if self.record_kind != NOTEBOOK_ALPHA_LANE_RECORD_KIND {
            out.push(NotebookAlphaViolation::UnexpectedRecordKind {
                record_kind: self.record_kind.clone(),
            });
        }
        if self.schema_version != NOTEBOOK_ALPHA_LANE_SCHEMA_VERSION {
            out.push(NotebookAlphaViolation::UnexpectedSchemaVersion {
                schema_version: self.schema_version,
            });
        }

        self.validate_required_objects(&mut out);
        self.validate_stable_ids(&mut out);
        self.validate_route_and_context_refs(&mut out);
        self.validate_trust_axes(&mut out);
        self.validate_document_identity(&mut out);
        self.validate_metadata_and_attachments(&mut out);
        self.validate_outputs(&mut out);
        self.validate_diff_repair_and_exports(&mut out);
        self.validate_scope_disclosure(&mut out);

        out
    }

    /// Returns true when validation finds no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Returns true when the lane publishes all required object families.
    pub fn has_required_object_graph(&self) -> bool {
        !self.document.document_id.trim().is_empty()
            && !self.cells.is_empty()
            && self.kernel_session.is_some()
            && !self.outputs.is_empty()
            && !self.review_session.review_session_id.trim().is_empty()
            && (!self.paired_exports.is_empty() || !self.reproducibility.is_empty())
    }

    /// Returns true when unknown metadata and attachments are preserved or explicitly retained.
    pub fn preserves_unknown_metadata_and_attachments(&self) -> bool {
        self.document.metadata_namespaces.iter().all(|namespace| {
            namespace.namespace_kind != NotebookMetadataNamespaceKind::UnknownVendor
                || namespace.survival_posture == MetadataSurvivalPosture::PreserveVerbatim
        }) && self
            .document
            .attachments
            .iter()
            .all(|attachment| attachment.roundtrip_posture.is_preserved())
    }

    /// Returns true when canonical notebook, rendered report, and raw payload scopes are distinct.
    pub fn distinguishes_export_scopes(&self) -> bool {
        let scopes: BTreeSet<_> = self
            .export_scopes
            .iter()
            .map(|scope| scope.scope_class)
            .collect();
        [
            NotebookArtifactScopeClass::CanonicalIpynb,
            NotebookArtifactScopeClass::RenderedReport,
            NotebookArtifactScopeClass::RawArtifactPayload,
        ]
        .into_iter()
        .all(|scope| scopes.contains(&scope))
    }

    /// Returns true when stale, captured, imported, and replayed outputs cannot read as live.
    pub fn outputs_do_not_masquerade_as_live(&self) -> bool {
        self.outputs.iter().all(|output| {
            if output.freshness.is_live() {
                return true;
            }
            output.output_trust_state != OutputTrustState::LiveFromCurrentSession
                && output.viewer_truth_state_token != "live_output"
                && output.lineage_class_token != "live_output_from_current_session"
                && output.honesty_marker_present
        })
    }

    /// Returns true when at least one diff or repair affordance is visible.
    pub fn has_diff_or_repair_affordance(&self) -> bool {
        !self.diff_posture.affordances.is_empty() || !self.repair_actions.is_empty()
    }

    /// Projects this lane into the first inspectable support/export consumer.
    pub fn support_export(&self) -> NotebookAlphaSupportExportRecord {
        let violations = self.validate();
        NotebookAlphaSupportExportRecord {
            record_kind: NOTEBOOK_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_ALPHA_LANE_SCHEMA_VERSION,
            lane_id: self.lane_id.clone(),
            source_record_kind: self.record_kind.clone(),
            object_refs: self.object_refs(),
            trust_axis_tokens: vec![
                self.trust_axes.workspace_token().to_owned(),
                self.trust_axes.notebook_token().to_owned(),
                self.trust_axes.kernel_token().to_owned(),
                self.trust_axes.output_token().to_owned(),
                self.trust_axes.widget_token().to_owned(),
                self.trust_axes
                    .rendered_output_trust_class
                    .as_str()
                    .to_owned(),
            ],
            identity_tokens: self.document.identity_disclosure.identity_tokens(),
            export_scope_tokens: self
                .export_scopes
                .iter()
                .map(|scope| scope.scope_class.as_str().to_owned())
                .collect(),
            downgrade_behaviors: self.downgrade_behaviors.clone(),
            violation_tokens: violations
                .iter()
                .map(|violation| violation.token().to_owned())
                .collect(),
            plaintext_summary: self.render_plaintext(),
        }
    }

    /// Renders a deterministic support/export summary without raw notebook bodies.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display
        ));
        out.push_str(&format!(
            "lane={} workspace={} document={} canonical={}\n",
            self.lane_id,
            self.workspace_id,
            self.document.document_id,
            self.document.canonical_ipynb_is_source
        ));
        out.push_str("identity:\n");
        for line in self.document.identity_disclosure.render_plaintext_lines() {
            out.push_str(&format!("  - {line}\n"));
        }
        out.push_str(&format!(
            "trust_axes: workspace={} notebook={} kernel={} output={} widget={} trust_class={}\n",
            self.trust_axes.workspace_token(),
            self.trust_axes.notebook_token(),
            self.trust_axes.kernel_token(),
            self.trust_axes.output_token(),
            self.trust_axes.widget_token(),
            self.trust_axes.rendered_output_trust_class.as_str(),
        ));
        out.push_str("objects:\n");
        for object_ref in self.object_refs() {
            out.push_str(&format!("  - {object_ref}\n"));
        }
        out.push_str("outputs:\n");
        for output in &self.outputs {
            out.push_str(&format!(
                "  - {} cell={} freshness={} viewer={} lineage={} trust={} honesty_marker={}\n",
                output.output_record_id,
                output.cell_id_ref,
                output.freshness.as_str(),
                output.viewer_truth_state_token,
                output.lineage_class_token,
                output.output_trust_state.as_str(),
                output.honesty_marker_present,
            ));
        }
        out.push_str("diff:\n");
        let affordances = self
            .diff_posture
            .affordance_tokens()
            .into_iter()
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "  mode={} source_changes={} metadata_changes={} output_changes={} attachment_changes={} affordances=[{}]\n",
            self.diff_posture.diff_mode.as_str(),
            self.diff_posture.source_change_count,
            self.diff_posture.metadata_change_count,
            self.diff_posture.output_change_count,
            self.diff_posture.attachment_change_count,
            affordances,
        ));
        out.push_str("repairs:\n");
        for repair in &self.repair_actions {
            out.push_str(&format!(
                "  - {} family={} apply={} reversal={} preview_required={} checkpoint={} consequence={}\n",
                repair.repair_action_id,
                repair.repair_class_family.as_str(),
                repair.apply_mode_class.as_str(),
                repair.transaction_reversal_class.as_str(),
                repair.preview_required,
                repair.checkpoint_ref.as_deref().unwrap_or("(none)"),
                repair.consequence_label,
            ));
        }
        out.push_str("export_scopes:\n");
        for scope in &self.export_scopes {
            out.push_str(&format!(
                "  - {} scope={} representation={} body={} trust_class={} label={}\n",
                scope.export_scope_id,
                scope.scope_class.as_str(),
                scope.representation_class.as_str(),
                scope.body_posture.as_str(),
                scope.trust_class.as_str(),
                scope.stale_or_imported_output_label,
            ));
        }
        out.push_str("claim_limits:\n");
        for limit in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", limit.token, limit.label));
        }
        out
    }

    fn validate_required_objects(&self, out: &mut Vec<NotebookAlphaViolation>) {
        if self.document.document_id.trim().is_empty() {
            out.push(NotebookAlphaViolation::MissingObject {
                object_kind: "document".to_owned(),
            });
        }
        if self.cells.is_empty() {
            out.push(NotebookAlphaViolation::MissingObject {
                object_kind: "cell".to_owned(),
            });
        }
        if self.kernel_session.is_none() {
            out.push(NotebookAlphaViolation::MissingObject {
                object_kind: "kernel_session".to_owned(),
            });
        }
        if self.outputs.is_empty() {
            out.push(NotebookAlphaViolation::MissingObject {
                object_kind: "output_record".to_owned(),
            });
        }
        if self.review_session.review_session_id.trim().is_empty() {
            out.push(NotebookAlphaViolation::MissingObject {
                object_kind: "review_session".to_owned(),
            });
        }
        if self.paired_exports.is_empty() && self.reproducibility.is_empty() {
            out.push(NotebookAlphaViolation::MissingObject {
                object_kind: "paired_export_or_reproducibility".to_owned(),
            });
        }
    }

    fn validate_document_identity(&self, out: &mut Vec<NotebookAlphaViolation>) {
        for field in self.document.identity_disclosure.missing_fields() {
            out.push(NotebookAlphaViolation::DocumentIdentityIncomplete {
                document_id: self.document.document_id.clone(),
                field: field.to_owned(),
            });
        }

        if self.document.identity_disclosure.document_family
            != DocumentFamilyClass::NotebookDocument
        {
            out.push(NotebookAlphaViolation::DocumentIdentityFamilyMismatch {
                document_id: self.document.document_id.clone(),
                family: self
                    .document
                    .identity_disclosure
                    .document_family
                    .as_str()
                    .to_owned(),
            });
        }
    }

    fn validate_stable_ids(&self, out: &mut Vec<NotebookAlphaViolation>) {
        let mut seen: BTreeMap<String, String> = BTreeMap::new();
        push_stable_id(&mut seen, out, "lane", &self.lane_id);
        push_stable_id(&mut seen, out, "document", &self.document.document_id);
        for cell in &self.cells {
            push_stable_id(&mut seen, out, "cell", &cell.cell_object_id);
            push_stable_id(&mut seen, out, "stable_cell", &cell.stable_cell_id);
        }
        if let Some(kernel_session) = &self.kernel_session {
            push_stable_id(
                &mut seen,
                out,
                "kernel_session",
                &kernel_session.kernel_session_id,
            );
        }
        for output in &self.outputs {
            push_stable_id(&mut seen, out, "output_record", &output.output_record_id);
        }
        push_stable_id(
            &mut seen,
            out,
            "review_session",
            &self.review_session.review_session_id,
        );
        push_stable_id(
            &mut seen,
            out,
            "diff_posture",
            &self.diff_posture.diff_posture_id,
        );
        for repair in &self.repair_actions {
            push_stable_id(&mut seen, out, "repair_action", &repair.repair_action_id);
        }
        for scope in &self.export_scopes {
            push_stable_id(&mut seen, out, "export_scope", &scope.export_scope_id);
        }
        for paired_export in &self.paired_exports {
            push_stable_id(
                &mut seen,
                out,
                "paired_export",
                &paired_export.paired_export_id,
            );
        }
        for reproducibility in &self.reproducibility {
            push_stable_id(
                &mut seen,
                out,
                "reproducibility",
                &reproducibility.reproducibility_id,
            );
        }
    }

    fn validate_route_and_context_refs(&self, out: &mut Vec<NotebookAlphaViolation>) {
        require_ref(
            out,
            "document",
            &self.document.document_id,
            "route_ref",
            &self.document.route_ref,
        );
        require_ref(
            out,
            "document",
            &self.document.document_id,
            "canonical_object_ref",
            &self.document.canonical_object_ref,
        );
        for cell in &self.cells {
            require_ref(
                out,
                "cell",
                &cell.cell_object_id,
                "route_ref",
                &cell.route_ref,
            );
        }
        if let Some(kernel_session) = &self.kernel_session {
            require_ref(
                out,
                "kernel_session",
                &kernel_session.kernel_session_id,
                "execution_context_ref",
                &kernel_session.execution_context_ref,
            );
            require_ref(
                out,
                "kernel_session",
                &kernel_session.kernel_session_id,
                "route_ref",
                &kernel_session.route_ref,
            );
        }
        require_ref(
            out,
            "review_session",
            &self.review_session.review_session_id,
            "route_ref",
            &self.review_session.route_ref,
        );
        for paired_export in &self.paired_exports {
            require_ref(
                out,
                "paired_export",
                &paired_export.paired_export_id,
                "route_ref",
                &paired_export.route_ref,
            );
        }
        for reproducibility in &self.reproducibility {
            require_ref(
                out,
                "reproducibility",
                &reproducibility.reproducibility_id,
                "route_ref",
                &reproducibility.route_ref,
            );
        }
    }

    fn validate_trust_axes(&self, out: &mut Vec<NotebookAlphaViolation>) {
        if self.trust_axes.notebook_trust_rung.is_fully_trusted()
            && self.trust_axes.workspace_trust_state == WorkspaceTrustState::UnknownWorkspace
        {
            out.push(NotebookAlphaViolation::TrustAxisCollapsed {
                axis: "workspace_trust_state".to_owned(),
                reason: "fully trusted notebook cannot leave workspace trust unknown".to_owned(),
            });
        }
        if self.trust_axes.output_trust_state == OutputTrustState::LiveFromCurrentSession
            && !self.trust_axes.kernel_availability.is_available()
        {
            out.push(NotebookAlphaViolation::TrustAxisCollapsed {
                axis: "kernel_availability".to_owned(),
                reason: "live output trust requires an available kernel/session".to_owned(),
            });
        }
        let has_widget_output = self
            .outputs
            .iter()
            .any(|output| output.content_class == CellContentClass::WidgetOutput);
        if has_widget_output
            && self.trust_axes.widget_trust_state == WidgetTrustState::NotApplicable
        {
            out.push(NotebookAlphaViolation::TrustAxisCollapsed {
                axis: "widget_trust_state".to_owned(),
                reason: "widget outputs require explicit widget trust posture".to_owned(),
            });
        }
    }

    fn validate_metadata_and_attachments(&self, out: &mut Vec<NotebookAlphaViolation>) {
        if !self.document.canonical_ipynb_is_source {
            out.push(NotebookAlphaViolation::CanonicalNotebookNotSource {
                document_id: self.document.document_id.clone(),
            });
        }

        for namespace in &self.document.metadata_namespaces {
            if namespace.namespace_kind == NotebookMetadataNamespaceKind::UnknownVendor
                && namespace.survival_posture != MetadataSurvivalPosture::PreserveVerbatim
            {
                out.push(NotebookAlphaViolation::UnknownMetadataNotPreserved {
                    namespace: namespace.namespace.clone(),
                    posture: namespace.survival_posture.as_str().to_owned(),
                });
            }
        }

        for attachment in &self.document.attachments {
            if !attachment.roundtrip_posture.is_preserved() {
                out.push(NotebookAlphaViolation::AttachmentNotRoundTripped {
                    attachment_id: attachment.attachment_id.clone(),
                    posture: attachment.roundtrip_posture.as_str().to_owned(),
                });
            }
        }
    }

    fn validate_outputs(&self, out: &mut Vec<NotebookAlphaViolation>) {
        let cell_ids: BTreeSet<_> = self
            .cells
            .iter()
            .map(|cell| cell.stable_cell_id.as_str())
            .collect();
        let kernel_session_id = self
            .kernel_session
            .as_ref()
            .map(|session| session.kernel_session_id.as_str());

        for output in &self.outputs {
            if !cell_ids.contains(output.cell_id_ref.as_str()) {
                out.push(NotebookAlphaViolation::OutputMissingCellRef {
                    output_record_id: output.output_record_id.clone(),
                    cell_id_ref: output.cell_id_ref.clone(),
                });
            }

            if output.freshness.is_live() {
                if output.output_trust_state != OutputTrustState::LiveFromCurrentSession {
                    out.push(NotebookAlphaViolation::LiveOutputTrustMismatch {
                        output_record_id: output.output_record_id.clone(),
                        output_trust_state: output.output_trust_state.as_str().to_owned(),
                    });
                }
                match (&output.kernel_session_id_ref, kernel_session_id) {
                    (Some(output_session), Some(active_session))
                        if output_session == active_session => {}
                    _ => out.push(NotebookAlphaViolation::LiveOutputMissingKernelSession {
                        output_record_id: output.output_record_id.clone(),
                    }),
                }
                if output
                    .execution_context_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
                {
                    out.push(NotebookAlphaViolation::MissingRouteOrExecutionContextRef {
                        object_kind: "output_record".to_owned(),
                        object_id: output.output_record_id.clone(),
                        field: "execution_context_ref".to_owned(),
                    });
                }
            } else {
                if output.output_trust_state == OutputTrustState::LiveFromCurrentSession
                    || output.viewer_truth_state_token == "live_output"
                    || output.lineage_class_token == "live_output_from_current_session"
                {
                    out.push(
                        NotebookAlphaViolation::StaleOrImportedOutputMasqueradesLive {
                            output_record_id: output.output_record_id.clone(),
                            freshness: output.freshness.as_str().to_owned(),
                            output_trust_state: output.output_trust_state.as_str().to_owned(),
                            viewer_truth_state: output.viewer_truth_state_token.clone(),
                            lineage_class: output.lineage_class_token.clone(),
                        },
                    );
                }
                if !output.honesty_marker_present {
                    out.push(NotebookAlphaViolation::MissingStaleOutputHonestyMarker {
                        output_record_id: output.output_record_id.clone(),
                        freshness: output.freshness.as_str().to_owned(),
                    });
                }
            }

            if output.content_class == CellContentClass::WidgetOutput
                && output.widget_trust_state == WidgetTrustState::NotApplicable
            {
                out.push(NotebookAlphaViolation::TrustAxisCollapsed {
                    axis: "widget_trust_state".to_owned(),
                    reason: format!(
                        "output {} is a widget without widget trust",
                        output.output_record_id
                    ),
                });
            }
        }
    }

    fn validate_diff_repair_and_exports(&self, out: &mut Vec<NotebookAlphaViolation>) {
        if !self.has_diff_or_repair_affordance() {
            out.push(NotebookAlphaViolation::MissingDiffOrRepairAffordance);
        }
        for repair in &self.repair_actions {
            if repair.repair_preview_ref.trim().is_empty() {
                out.push(NotebookAlphaViolation::RepairMissingPreview {
                    repair_action_id: repair.repair_action_id.clone(),
                });
            }
            if !repair.preview_required
                && !matches!(
                    repair.apply_mode_class,
                    RepairApplyModeClass::ApplyObserveOnlyNoWrite
                        | RepairApplyModeClass::ApplyRefusedEscalationOnly
                )
            {
                out.push(NotebookAlphaViolation::RepairMissingPreview {
                    repair_action_id: repair.repair_action_id.clone(),
                });
            }
            if repair.apply_mode_class.requires_checkpoint()
                && repair
                    .checkpoint_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                out.push(NotebookAlphaViolation::RepairMissingCheckpoint {
                    repair_action_id: repair.repair_action_id.clone(),
                });
            }
            if repair.consequence_label.trim().is_empty() || repair.safe_next_actions.is_empty() {
                out.push(NotebookAlphaViolation::RepairConsequenceMissing {
                    repair_action_id: repair.repair_action_id.clone(),
                });
            }
        }

        for scope in &self.export_scopes {
            if scope.output_include_policy_ref.trim().is_empty() {
                out.push(NotebookAlphaViolation::ExportScopeMissingIncludePolicy {
                    scope: scope.scope_class.as_str().to_owned(),
                });
            }
        }
        if !self.distinguishes_export_scopes() {
            let present: Vec<String> = self
                .export_scopes
                .iter()
                .map(|scope| scope.scope_class.as_str().to_owned())
                .collect();
            out.push(NotebookAlphaViolation::ExportScopesCollapsed { present });
        }
        for paired_export in &self.paired_exports {
            if paired_export.source_of_truth_scope != NotebookArtifactScopeClass::CanonicalIpynb {
                out.push(NotebookAlphaViolation::PairedExportClaimsCanonical {
                    paired_export_id: paired_export.paired_export_id.clone(),
                    source_of_truth_scope: paired_export.source_of_truth_scope.as_str().to_owned(),
                });
            }
        }
    }

    fn validate_scope_disclosure(&self, out: &mut Vec<NotebookAlphaViolation>) {
        let tokens: Vec<_> = self
            .claim_limits
            .iter()
            .map(|row| row.token.as_str())
            .collect();
        let expected: Vec<_> = NotebookAlphaClaimLimit::canonical_set()
            .into_iter()
            .map(|limit| limit.as_str())
            .collect();
        if tokens != expected {
            out.push(NotebookAlphaViolation::ClaimLimitsMissingOrOutOfOrder);
        }
        if self.downgrade_behaviors.is_empty() {
            out.push(NotebookAlphaViolation::DowngradeBehaviorMissing);
        }
    }

    fn object_refs(&self) -> Vec<String> {
        let mut refs = Vec::new();
        refs.push(format!("document:{}", self.document.document_id));
        for cell in &self.cells {
            refs.push(format!("cell:{}", cell.cell_object_id));
        }
        if let Some(kernel_session) = &self.kernel_session {
            refs.push(format!(
                "kernel_session:{}",
                kernel_session.kernel_session_id
            ));
        }
        for output in &self.outputs {
            refs.push(format!("output_record:{}", output.output_record_id));
        }
        refs.push(format!(
            "review_session:{}",
            self.review_session.review_session_id
        ));
        for paired_export in &self.paired_exports {
            refs.push(format!("paired_export:{}", paired_export.paired_export_id));
        }
        for reproducibility in &self.reproducibility {
            refs.push(format!(
                "reproducibility:{}",
                reproducibility.reproducibility_id
            ));
        }
        refs
    }
}

/// Support/export projection for the bounded notebook alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAlphaSupportExportRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source lane id.
    pub lane_id: String,
    /// Source record kind.
    pub source_record_kind: String,
    /// Object refs included in the export.
    pub object_refs: Vec<String>,
    /// Trust-axis tokens preserved for support.
    pub trust_axis_tokens: Vec<String>,
    /// Identity tokens preserved for support.
    pub identity_tokens: Vec<String>,
    /// Export-scope tokens preserved for support.
    pub export_scope_tokens: Vec<String>,
    /// Downgrade behaviors preserved for support.
    pub downgrade_behaviors: Vec<String>,
    /// Validation violation tokens, empty for a clean lane.
    pub violation_tokens: Vec<String>,
    /// Deterministic plaintext summary.
    pub plaintext_summary: String,
}

/// Validation issue emitted by [`NotebookAlphaLaneRecord::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NotebookAlphaViolation {
    /// Record kind did not match the lane contract.
    UnexpectedRecordKind { record_kind: String },
    /// Schema version did not match the lane contract.
    UnexpectedSchemaVersion { schema_version: u32 },
    /// A required object family is missing.
    MissingObject { object_kind: String },
    /// A stable id is empty.
    MissingStableId { object_kind: String },
    /// A stable id appears on more than one object.
    DuplicateStableId {
        stable_id: String,
        first_kind: String,
        second_kind: String,
    },
    /// A route or execution-context ref is missing.
    MissingRouteOrExecutionContextRef {
        object_kind: String,
        object_id: String,
        field: String,
    },
    /// The notebook document identity disclosure is incomplete.
    DocumentIdentityIncomplete { document_id: String, field: String },
    /// The notebook document identity disclosure names the wrong family.
    DocumentIdentityFamilyMismatch { document_id: String, family: String },
    /// A trust axis collapsed or was left unavailable for a claimed object.
    TrustAxisCollapsed { axis: String, reason: String },
    /// The lane no longer treats the canonical `.ipynb` as source.
    CanonicalNotebookNotSource { document_id: String },
    /// Unknown metadata would not round-trip.
    UnknownMetadataNotPreserved { namespace: String, posture: String },
    /// A notebook attachment would not round-trip.
    AttachmentNotRoundTripped {
        attachment_id: String,
        posture: String,
    },
    /// An output references a cell that is not present in the lane.
    OutputMissingCellRef {
        output_record_id: String,
        cell_id_ref: String,
    },
    /// A live output does not cite the active kernel session.
    LiveOutputMissingKernelSession { output_record_id: String },
    /// A live output has a non-live output trust state.
    LiveOutputTrustMismatch {
        output_record_id: String,
        output_trust_state: String,
    },
    /// A stale, captured, imported, or replayed output still claims live posture.
    StaleOrImportedOutputMasqueradesLive {
        output_record_id: String,
        freshness: String,
        output_trust_state: String,
        viewer_truth_state: String,
        lineage_class: String,
    },
    /// A stale, captured, imported, or replayed output lacks an honesty marker.
    MissingStaleOutputHonestyMarker {
        output_record_id: String,
        freshness: String,
    },
    /// No diff or repair affordance is visible.
    MissingDiffOrRepairAffordance,
    /// Repair preview information is missing.
    RepairMissingPreview { repair_action_id: String },
    /// A checkpoint is required but absent.
    RepairMissingCheckpoint { repair_action_id: String },
    /// Repair consequence or safe next action is missing.
    RepairConsequenceMissing { repair_action_id: String },
    /// Export scope lacks an include-policy ref.
    ExportScopeMissingIncludePolicy { scope: String },
    /// Export scopes do not distinguish canonical, rendered, and raw payloads.
    ExportScopesCollapsed { present: Vec<String> },
    /// A paired export claims a non-notebook source of truth.
    PairedExportClaimsCanonical {
        paired_export_id: String,
        source_of_truth_scope: String,
    },
    /// Claim limits are missing or out of canonical order.
    ClaimLimitsMissingOrOutOfOrder,
    /// Supported downgrade behavior was not disclosed.
    DowngradeBehaviorMissing,
}

impl NotebookAlphaViolation {
    /// Returns the stable violation token.
    pub const fn token(&self) -> &'static str {
        match self {
            Self::UnexpectedRecordKind { .. } => "unexpected_record_kind",
            Self::UnexpectedSchemaVersion { .. } => "unexpected_schema_version",
            Self::MissingObject { .. } => "missing_object",
            Self::MissingStableId { .. } => "missing_stable_id",
            Self::DuplicateStableId { .. } => "duplicate_stable_id",
            Self::MissingRouteOrExecutionContextRef { .. } => {
                "missing_route_or_execution_context_ref"
            }
            Self::DocumentIdentityIncomplete { .. } => "document_identity_incomplete",
            Self::DocumentIdentityFamilyMismatch { .. } => "document_identity_family_mismatch",
            Self::TrustAxisCollapsed { .. } => "trust_axis_collapsed",
            Self::CanonicalNotebookNotSource { .. } => "canonical_notebook_not_source",
            Self::UnknownMetadataNotPreserved { .. } => "unknown_metadata_not_preserved",
            Self::AttachmentNotRoundTripped { .. } => "attachment_not_round_tripped",
            Self::OutputMissingCellRef { .. } => "output_missing_cell_ref",
            Self::LiveOutputMissingKernelSession { .. } => "live_output_missing_kernel_session",
            Self::LiveOutputTrustMismatch { .. } => "live_output_trust_mismatch",
            Self::StaleOrImportedOutputMasqueradesLive { .. } => {
                "stale_or_imported_output_masquerades_live"
            }
            Self::MissingStaleOutputHonestyMarker { .. } => "missing_stale_output_honesty_marker",
            Self::MissingDiffOrRepairAffordance => "missing_diff_or_repair_affordance",
            Self::RepairMissingPreview { .. } => "repair_missing_preview",
            Self::RepairMissingCheckpoint { .. } => "repair_missing_checkpoint",
            Self::RepairConsequenceMissing { .. } => "repair_consequence_missing",
            Self::ExportScopeMissingIncludePolicy { .. } => "export_scope_missing_include_policy",
            Self::ExportScopesCollapsed { .. } => "export_scopes_collapsed",
            Self::PairedExportClaimsCanonical { .. } => "paired_export_claims_canonical",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::DowngradeBehaviorMissing => "downgrade_behavior_missing",
        }
    }
}

fn push_stable_id(
    seen: &mut BTreeMap<String, String>,
    out: &mut Vec<NotebookAlphaViolation>,
    object_kind: &'static str,
    stable_id: &str,
) {
    if stable_id.trim().is_empty() {
        out.push(NotebookAlphaViolation::MissingStableId {
            object_kind: object_kind.to_owned(),
        });
        return;
    }
    if let Some(first_kind) = seen.insert(stable_id.to_owned(), object_kind.to_owned()) {
        out.push(NotebookAlphaViolation::DuplicateStableId {
            stable_id: stable_id.to_owned(),
            first_kind,
            second_kind: object_kind.to_owned(),
        });
    }
}

fn require_ref(
    out: &mut Vec<NotebookAlphaViolation>,
    object_kind: &'static str,
    object_id: &str,
    field: &'static str,
    value: &str,
) {
    if value.trim().is_empty() {
        out.push(NotebookAlphaViolation::MissingRouteOrExecutionContextRef {
            object_kind: object_kind.to_owned(),
            object_id: object_id.to_owned(),
            field: field.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests;
