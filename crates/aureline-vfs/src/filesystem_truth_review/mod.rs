//! Canonical M5-era filesystem truth review surfaces.
//!
//! This module freezes one packet that packages four related filesystem-safety
//! surfaces together:
//!
//! - watch-fidelity strips,
//! - ignore-resolution drawers,
//! - external-change reviews, and
//! - cross-root move or rename reviews.
//!
//! The packet gives notebook, request-workspace, preview-output, and
//! provider-draft consumers one typed source of truth for the user-visible
//! vocabulary around degraded watch guarantees, absent or hidden items,
//! compare-first external changes, and boundary-changing moves. Later shell and
//! editor surfaces may render these records directly instead of re-deriving
//! parallel labels.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::filesystem_mutation_lineage_matrix::{MatrixRootClass, SurfaceClass};
use crate::identity::{
    ExternalChangeCompareOutcome, ExternalChangeContentKind, ExternalChangeDiffAvailability,
};

/// Schema version stamped onto packet and fixture records.
pub const FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the checked-in packet.
pub const FILESYSTEM_TRUTH_REVIEW_PACKET_RECORD_KIND: &str =
    "filesystem_truth_review_packet_record";

/// Stable record-kind tag carried by fixture records.
pub const FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND: &str =
    "filesystem_truth_review_fixture_record";

/// Repo-relative JSON schema reference.
pub const FILESYSTEM_TRUTH_REVIEW_SCHEMA_REF: &str =
    "schemas/state/filesystem_truth_review.schema.json";

/// Repo-relative reviewer doc reference.
pub const FILESYSTEM_TRUTH_REVIEW_DOC_REF: &str = "docs/state/filesystem_truth_review.md";

/// Repo-relative machine-readable artifact packet.
pub const FILESYSTEM_TRUTH_REVIEW_ARTIFACT_REF: &str =
    "artifacts/state/filesystem_truth_review.json";

/// Repo-relative reviewer artifact report.
pub const FILESYSTEM_TRUTH_REVIEW_REPORT_REF: &str =
    "artifacts/state/filesystem_truth_review.md";

/// Repo-relative fixture directory.
pub const FILESYSTEM_TRUTH_REVIEW_FIXTURE_DIR: &str = "fixtures/state/filesystem_truth_review";

/// Repo-relative fixture manifest.
pub const FILESYSTEM_TRUTH_REVIEW_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/filesystem_truth_review/manifest.yaml";

const WATCH_ACTION_IDS: &[&str] = &[
    "view_implications",
    "retry_watcher",
    "inspect_root_details",
    "continue_editing",
    "change_interval",
    "refresh_now",
];
const IGNORE_ACTION_IDS: &[&str] = &[
    "reveal_in_context",
    "inspect_ignore_sources",
    "open_policy_details",
    "expand_scope",
    "open_generated_source",
];
const EXTERNAL_CHANGE_ACTION_IDS: &[&str] = &[
    "compare_to_disk",
    "reload",
    "keep_local",
    "merge_manually",
    "save_elsewhere",
    "inspect_alias_details",
    "cancel",
];
const CROSS_ROOT_ACTION_IDS: &[&str] = &[
    "preview_plan",
    "proceed",
    "recompute_capabilities",
    "save_as_copy",
    "cancel",
];

/// Shared action row rendered by one filesystem-truth review surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionOffer {
    /// Stable action token for automation, accessibility, and support export.
    pub action_id: String,
    /// User-facing action label.
    pub label: String,
    /// True when the action is currently selectable.
    pub enabled: bool,
    /// Short summary of what selecting the action would do.
    pub outcome_summary: String,
}

/// Support-export metadata shared by every record family in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExport {
    /// Packet family token consumed by support tooling.
    pub packet_family: String,
    /// Redaction policy applied to this record family.
    pub redaction_policy: String,
    /// Stable parity signature used by fixture and export checks.
    pub parity_signature: String,
}

/// Watch-fidelity state exposed directly in the strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchMode {
    /// Native watch is active and current truth is timely.
    LiveWatch,
    /// Native watch remains attached but with narrower guarantees.
    ReducedFidelityWatch,
    /// Event watch is unavailable and interval-based polling is the floor.
    PollingFallback,
    /// No timely watch path is available; refresh is manual.
    ManualRefreshOnly,
    /// Refresh depends on an upstream provider or generator feed.
    ProviderRefreshOnly,
}

impl WatchMode {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveWatch => "live_watch",
            Self::ReducedFidelityWatch => "reduced_fidelity_watch",
            Self::PollingFallback => "polling_fallback",
            Self::ManualRefreshOnly => "manual_refresh_only",
            Self::ProviderRefreshOnly => "provider_refresh_only",
        }
    }
}

/// Reason the current watch mode is not the strongest local guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchReason {
    /// Rename pairing is missing even though the watcher still sees changes.
    RenamePairingMissing,
    /// The remote event stream is partial or intermittent.
    RemoteAgentPartialEventStream,
    /// The container mount bridge drops metadata-sensitive events.
    ContainerMountBridgeDropsMetadata,
    /// Refresh depends on a generator or provider pipeline.
    GeneratedRefreshDependsOnSource,
    /// The provider change feed is available but throttled.
    ProviderChangeFeedThrottled,
}

impl WatchReason {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenamePairingMissing => "rename_pairing_missing",
            Self::RemoteAgentPartialEventStream => "remote_agent_partial_event_stream",
            Self::ContainerMountBridgeDropsMetadata => "container_mount_bridge_drops_metadata",
            Self::GeneratedRefreshDependsOnSource => "generated_refresh_depends_on_source",
            Self::ProviderChangeFeedThrottled => "provider_change_feed_throttled",
        }
    }
}

/// Product truth that may now lag or weaken under the current watch mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchGuaranteeImpact {
    /// External changes may surface late.
    ExternalChangeDetection,
    /// Conflict timing may be less predictable.
    ConflictTiming,
    /// Generated or rendered refresh may trail authored changes.
    GeneratedRefresh,
    /// Search or query results may be stale.
    SearchFreshness,
    /// Metadata such as mode bits or ownership may lag.
    MetadataAttributeFreshness,
    /// Provider revision truth may lag behind local chrome.
    ProviderRevisionFreshness,
}

impl WatchGuaranteeImpact {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalChangeDetection => "external_change_detection",
            Self::ConflictTiming => "conflict_timing",
            Self::GeneratedRefresh => "generated_refresh",
            Self::SearchFreshness => "search_freshness",
            Self::MetadataAttributeFreshness => "metadata_attribute_freshness",
            Self::ProviderRevisionFreshness => "provider_revision_freshness",
        }
    }
}

/// Watch-fidelity strip shown before mutating one root-backed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchFidelityStripRecord {
    /// Stable strip id referenced from the scenario map and support exports.
    pub strip_id: String,
    /// Root class that owns the strip.
    pub root_class: MatrixRootClass,
    /// Consumer surface that renders the strip.
    pub surface_class: SurfaceClass,
    /// Plain-language root label rendered in the strip header.
    pub root_label: String,
    /// Current watch mode shown to the user.
    pub watch_mode: WatchMode,
    /// Reason for the current mode when one is needed to explain the downgrade.
    pub reason: Option<WatchReason>,
    /// Guarantees that may now lag or weaken.
    pub affected_guarantees: Vec<WatchGuaranteeImpact>,
    /// Short banner or strip summary.
    pub summary: String,
    /// Detail lines quoted by inspectors and support surfaces.
    pub detail_lines: Vec<String>,
    /// Ordered actions offered from the strip.
    pub actions: Vec<ActionOffer>,
    /// Export-safe metadata.
    pub support_export: SupportExport,
}

/// Ignore source that contributes to a drawer explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IgnoreSourceClass {
    /// Hidden-file naming convention such as dotfiles or checkpoints.
    HiddenFile,
    /// Tool-owned exclude or suppression rule.
    ToolIgnore,
    /// Workspace or user visibility rule.
    WorkspaceExclude,
    /// Generated overlay or derived-output suppression.
    GeneratedOverlay,
    /// Policy-managed hidden or blocked path.
    PolicyHidden,
    /// Workset or sparse-scope boundary.
    WorksetBoundary,
    /// Query or results scope that intentionally excludes rows.
    ScopeLimit,
}

impl IgnoreSourceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HiddenFile => "hidden_file",
            Self::ToolIgnore => "tool_ignore",
            Self::WorkspaceExclude => "workspace_exclude",
            Self::GeneratedOverlay => "generated_overlay",
            Self::PolicyHidden => "policy_hidden",
            Self::WorksetBoundary => "workset_boundary",
            Self::ScopeLimit => "scope_limit",
        }
    }
}

/// One ignore-source row in the drawer body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IgnoreSourceEntry {
    /// Source class contributing to the hidden or excluded state.
    pub source_class: IgnoreSourceClass,
    /// User-facing source label such as `.gitignore` or `admin rule`.
    pub source_label: String,
    /// Short explanation of the source's effect.
    pub summary: String,
}

/// Visibility class for a hidden or excluded item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IgnoreVisibilityClass {
    /// The item is hidden by default ignore rules.
    IgnoreHidden,
    /// The item is a generated overlay rather than an authored source row.
    GeneratedOverlayHidden,
    /// A stronger policy layer hides or blocks the row.
    PolicyHidden,
    /// The row exists but is outside the active slice or query scope.
    ScopeLimitedResults,
}

impl IgnoreVisibilityClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IgnoreHidden => "ignore_hidden",
            Self::GeneratedOverlayHidden => "generated_overlay_hidden",
            Self::PolicyHidden => "policy_hidden",
            Self::ScopeLimitedResults => "scope_limited_results",
        }
    }
}

/// Drawer explaining why a row is hidden, absent, or excluded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IgnoreResolutionDrawerRecord {
    /// Stable drawer id referenced by scenario and support records.
    pub drawer_id: String,
    /// Root class that owns the hidden or excluded item.
    pub root_class: MatrixRootClass,
    /// Consumer surface that would otherwise have shown the item.
    pub surface_class: SurfaceClass,
    /// User-facing item label.
    pub item_label: String,
    /// Top-level visibility class rendered in chrome.
    pub visibility_class: IgnoreVisibilityClass,
    /// Contributing sources that explain the exclusion.
    pub contributing_sources: Vec<IgnoreSourceEntry>,
    /// Scope effect or absence explanation.
    pub scope_impact: String,
    /// Whether the current surface may offer a direct override.
    pub override_allowed: bool,
    /// Short summary rendered in the drawer header.
    pub summary: String,
    /// Detail lines quoted by help and support surfaces.
    pub detail_lines: Vec<String>,
    /// Ordered actions offered from the drawer.
    pub actions: Vec<ActionOffer>,
    /// Export-safe metadata.
    pub support_export: SupportExport,
}

/// Metadata difference that matters during external-change review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataDeltaKind {
    /// Newline mode changed on durable bytes.
    NewlineModeChanged,
    /// Encoding changed on durable bytes.
    EncodingChanged,
    /// Execute-bit or equivalent executable posture changed.
    ExecuteBitChanged,
    /// Permission mode or ACL posture changed.
    PermissionModeChanged,
    /// Provider revision changed even though the presentation path stayed put.
    ProviderRevisionChanged,
    /// Generator or producer revision changed.
    GeneratorRevisionChanged,
    /// Ownership or authoring authority changed.
    OwnershipChanged,
}

impl MetadataDeltaKind {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewlineModeChanged => "newline_mode_changed",
            Self::EncodingChanged => "encoding_changed",
            Self::ExecuteBitChanged => "execute_bit_changed",
            Self::PermissionModeChanged => "permission_mode_changed",
            Self::ProviderRevisionChanged => "provider_revision_changed",
            Self::GeneratorRevisionChanged => "generator_revision_changed",
            Self::OwnershipChanged => "ownership_changed",
        }
    }
}

/// One metadata-delta note shown alongside the compare-first review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataDeltaNote {
    /// Closed metadata-delta class.
    pub kind: MetadataDeltaKind,
    /// User-facing note about why the metadata change matters.
    pub summary: String,
}

/// Compare-first review for an external change on one rooted surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalChangeReviewRecord {
    /// Stable review id referenced by scenario and support records.
    pub review_id: String,
    /// Root class backing the affected object.
    pub root_class: MatrixRootClass,
    /// Consumer surface that raised the review.
    pub surface_class: SurfaceClass,
    /// Presentation path or label shown to the user.
    pub presentation_path: String,
    /// Stable logical identity reference carried through the review.
    pub logical_identity_ref: String,
    /// Pinned canonical target captured before the drift.
    pub pinned_canonical_target: String,
    /// Canonical target observed at review time when resolution succeeded.
    pub observed_canonical_target: Option<String>,
    /// Identity note such as alias or same-target disclosure.
    pub identity_note: String,
    /// Best-known source of the external change.
    pub change_source: String,
    /// Compare outcome from the VFS identity layer.
    pub compare_outcome: ExternalChangeCompareOutcome,
    /// Whether a diff preview is available.
    pub diff_availability: ExternalChangeDiffAvailability,
    /// Content class for the diff.
    pub content_kind: ExternalChangeContentKind,
    /// One-line diff summary.
    pub diff_summary: String,
    /// Compact preview lines shown before any destructive choice.
    pub diff_preview_lines: Vec<String>,
    /// Metadata deltas that matter to the save choice.
    pub metadata_delta_notes: Vec<MetadataDeltaNote>,
    /// True when blind overwrite is forbidden.
    pub silent_overwrite_forbidden: bool,
    /// Ordered actions offered from the review.
    pub actions: Vec<ActionOffer>,
    /// Detail lines quoted by help and support surfaces.
    pub detail_lines: Vec<String>,
    /// Export-safe metadata.
    pub support_export: SupportExport,
}

/// Boundary crossing disclosed by a cross-root move or rename review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryCrossingKind {
    /// Root authority changes between source and target.
    RemoteAuthorityChange,
    /// Source or target crosses a container mount boundary.
    ContainerMountCrossing,
    /// The selected presentation path escapes through a symlink or mount alias.
    SymlinkOrMountCrossing,
    /// The operation detaches generated lineage into a new authored target.
    GeneratedLineageDetach,
}

impl BoundaryCrossingKind {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoteAuthorityChange => "remote_authority_change",
            Self::ContainerMountCrossing => "container_mount_crossing",
            Self::SymlinkOrMountCrossing => "symlink_or_mount_crossing",
            Self::GeneratedLineageDetach => "generated_lineage_detach",
        }
    }
}

/// Metadata or portability consequence highlighted before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataConsequence {
    /// Execute-bit or equivalent may not survive the move.
    ExecuteBitMayDrop,
    /// Ownership or uid/gid semantics may change.
    OwnershipMayChange,
    /// Permission inheritance or ACL evaluation changes.
    PermissionModelChanges,
    /// The operation becomes copy-then-review instead of in-place rename.
    SaveBecomesCopyThenReview,
    /// A remote or provider precondition must be revalidated.
    ProviderPreconditionRequired,
    /// Generated lineage is explicitly detached.
    GeneratedLineageDetached,
}

impl MetadataConsequence {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecuteBitMayDrop => "execute_bit_may_drop",
            Self::OwnershipMayChange => "ownership_may_change",
            Self::PermissionModelChanges => "permission_model_changes",
            Self::SaveBecomesCopyThenReview => "save_becomes_copy_then_review",
            Self::ProviderPreconditionRequired => "provider_precondition_required",
            Self::GeneratedLineageDetached => "generated_lineage_detached",
        }
    }
}

/// Review sheet shown before a cross-root move or rename commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossRootMoveReviewRecord {
    /// Stable review id referenced by scenario and support records.
    pub review_id: String,
    /// Consumer surface that requested the operation.
    pub surface_class: SurfaceClass,
    /// Root class for the source side of the operation.
    pub source_root_class: MatrixRootClass,
    /// Root class for the target side of the operation.
    pub target_root_class: MatrixRootClass,
    /// User-facing operation label.
    pub operation_label: String,
    /// Presentation path or label for the source side.
    pub source_path: String,
    /// Presentation path or label for the target side.
    pub target_path: String,
    /// Case-sensitivity note shown before proceed.
    pub case_sensitivity_note: String,
    /// Normalization or casing note when path spelling semantics matter.
    pub normalization_note: Option<String>,
    /// Strongest boundary-crossing class surfaced by the sheet.
    pub boundary_crossing: BoundaryCrossingKind,
    /// Portability or metadata consequences disclosed before commit.
    pub metadata_consequences: Vec<MetadataConsequence>,
    /// Short sheet summary.
    pub summary: String,
    /// Detail lines quoted by help and support surfaces.
    pub detail_lines: Vec<String>,
    /// Ordered actions offered from the review.
    pub actions: Vec<ActionOffer>,
    /// Export-safe metadata.
    pub support_export: SupportExport,
}

/// Scenario entry that binds the four review surfaces together for one lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewScenarioRecord {
    /// Stable scenario id for one seeded lane.
    pub scenario_id: String,
    /// Plain-language surface consumer reference.
    pub consumer_ref: String,
    /// Reviewer-facing scenario summary.
    pub scenario: String,
    /// Root class that anchors the scenario.
    pub root_class: MatrixRootClass,
    /// Surface class that anchors the scenario.
    pub surface_class: SurfaceClass,
    /// Referenced watch strip id.
    pub watch_strip_id: String,
    /// Referenced ignore drawer id.
    pub ignore_drawer_id: String,
    /// Referenced external-change review id.
    pub external_change_review_id: String,
    /// Referenced cross-root move review id.
    pub cross_root_move_review_id: String,
}

/// Checked-in packet covering the four filesystem-truth review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemTruthReviewPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id for this checked-in corpus.
    pub packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary sentence.
    pub summary: String,
    /// Seeded watch-fidelity strips.
    pub watch_fidelity_strips: Vec<WatchFidelityStripRecord>,
    /// Seeded ignore-resolution drawers.
    pub ignore_resolution_drawers: Vec<IgnoreResolutionDrawerRecord>,
    /// Seeded external-change reviews.
    pub external_change_reviews: Vec<ExternalChangeReviewRecord>,
    /// Seeded cross-root move or rename reviews.
    pub cross_root_move_reviews: Vec<CrossRootMoveReviewRecord>,
    /// Scenario map binding the four surfaces per lane.
    pub scenarios: Vec<ReviewScenarioRecord>,
}

/// Fixture entry used to lock one seeded scenario to review expectations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemTruthReviewFixture {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Referenced scenario id from the packet.
    pub expected_scenario_id: String,
    /// Root class the fixture must cover.
    pub root_class: MatrixRootClass,
    /// Surface class the fixture must cover.
    pub surface_class: SurfaceClass,
    /// Consumer reference that must match the packet scenario.
    pub consumer_ref: String,
    /// Reviewer-facing scenario summary.
    pub scenario: String,
    /// Watch mode that must be surfaced for this scenario.
    pub expected_watch_mode: WatchMode,
    /// Ignore-visibility class that must be surfaced for this scenario.
    pub expected_ignore_visibility: IgnoreVisibilityClass,
    /// External-change outcome that must be surfaced for this scenario.
    pub expected_compare_outcome: ExternalChangeCompareOutcome,
    /// Boundary crossing that must be surfaced for this scenario.
    pub expected_boundary_crossing: BoundaryCrossingKind,
    /// Action ids that must remain available across the four surfaces.
    pub required_action_ids: Vec<String>,
}

/// One validation violation reported by packet or fixture checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewSurfaceValidationViolation {
    /// Stable check id for the violated rule.
    pub check_id: String,
    /// Human-readable path to the failing field.
    pub field_path: String,
    /// Explanation of the violated rule.
    pub message: String,
}

/// Validation report returned when packet or fixture checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewSurfaceValidationReport {
    /// All violations found while validating a packet or fixture.
    pub violations: Vec<ReviewSurfaceValidationViolation>,
}

impl ReviewSurfaceValidationReport {
    fn push(
        &mut self,
        check_id: impl Into<String>,
        field_path: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.violations.push(ReviewSurfaceValidationViolation {
            check_id: check_id.into(),
            field_path: field_path.into(),
            message: message.into(),
        });
    }

    fn finish(self) -> Result<(), Self> {
        if self.violations.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

fn action(action_id: &str, label: &str, enabled: bool, outcome_summary: &str) -> ActionOffer {
    ActionOffer {
        action_id: action_id.to_owned(),
        label: label.to_owned(),
        enabled,
        outcome_summary: outcome_summary.to_owned(),
    }
}

fn support_export(packet_family: &str, parity_signature: &str) -> SupportExport {
    SupportExport {
        packet_family: packet_family.to_owned(),
        redaction_policy: "support_redact_paths_default".to_owned(),
        parity_signature: parity_signature.to_owned(),
    }
}

/// Returns the checked-in filesystem-truth review packet.
pub fn seeded_filesystem_truth_review_packet() -> FilesystemTruthReviewPacket {
    FilesystemTruthReviewPacket {
        record_kind: FILESYSTEM_TRUTH_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION,
        packet_id: "filesystem_truth_review.packet.20260612".to_owned(),
        generated_at: "2026-06-12T00:00:00Z".to_owned(),
        title: "Filesystem truth review surfaces".to_owned(),
        summary: "M5 file-bearing surfaces reuse one packet for watch-fidelity strips, ignore-resolution drawers, compare-first external-change review, and cross-root move review without inventing per-surface vocabulary.".to_owned(),
        watch_fidelity_strips: vec![
            WatchFidelityStripRecord {
                strip_id: "watch.local.notebook".to_owned(),
                root_class: MatrixRootClass::LocalFilesystem,
                surface_class: SurfaceClass::NotebookDocument,
                root_label: "Local notebook root".to_owned(),
                watch_mode: WatchMode::ReducedFidelityWatch,
                reason: Some(WatchReason::RenamePairingMissing),
                affected_guarantees: vec![
                    WatchGuaranteeImpact::ExternalChangeDetection,
                    WatchGuaranteeImpact::ConflictTiming,
                    WatchGuaranteeImpact::MetadataAttributeFreshness,
                ],
                summary: "Watch degraded for this notebook root.".to_owned(),
                detail_lines: vec![
                    "Native watch is still attached, but rename pairing is incomplete on this root.".to_owned(),
                    "External-change timing and metadata-sensitive conflicts may arrive later than ordinary local edits.".to_owned(),
                ],
                actions: vec![
                    action("view_implications", "View implications", true, "Opens the affected-guarantees detail list for this root."),
                    action("retry_watcher", "Retry watcher", true, "Re-attempts full native watch registration for the notebook root."),
                    action("inspect_root_details", "Inspect root details", true, "Opens logical, canonical, and save-target detail for this root."),
                ],
                support_export: support_export("state.filesystem_truth_review.watch_strip", "sha256:watch-strip-local-notebook"),
            },
            WatchFidelityStripRecord {
                strip_id: "watch.remote.request".to_owned(),
                root_class: MatrixRootClass::RemoteAgent,
                surface_class: SurfaceClass::RequestWorkspaceDocument,
                root_label: "Remote request workspace".to_owned(),
                watch_mode: WatchMode::PollingFallback,
                reason: None,
                affected_guarantees: vec![
                    WatchGuaranteeImpact::ExternalChangeDetection,
                    WatchGuaranteeImpact::ConflictTiming,
                    WatchGuaranteeImpact::SearchFreshness,
                    WatchGuaranteeImpact::ProviderRevisionFreshness,
                ],
                summary: "Remote watch fell back to polling.".to_owned(),
                detail_lines: vec![
                    "Remote event streaming is unavailable, so change detection depends on the next polling cycle.".to_owned(),
                    "Compare-before-write remains the correctness floor before every remote save.".to_owned(),
                ],
                actions: vec![
                    action("change_interval", "Change interval", true, "Adjusts the remote polling cadence when policy allows it."),
                    action("retry_watcher", "Retry native watch", true, "Re-attempts the remote event stream."),
                    action("continue_editing", "Continue editing", true, "Keeps editing with compare-before-write protection active."),
                ],
                support_export: support_export("state.filesystem_truth_review.watch_strip", "sha256:watch-strip-remote-request"),
            },
            WatchFidelityStripRecord {
                strip_id: "watch.container.preview".to_owned(),
                root_class: MatrixRootClass::ContainerMount,
                surface_class: SurfaceClass::PreviewOutputArtifact,
                root_label: "Container preview output".to_owned(),
                watch_mode: WatchMode::ReducedFidelityWatch,
                reason: Some(WatchReason::ContainerMountBridgeDropsMetadata),
                affected_guarantees: vec![
                    WatchGuaranteeImpact::GeneratedRefresh,
                    WatchGuaranteeImpact::MetadataAttributeFreshness,
                    WatchGuaranteeImpact::ExternalChangeDetection,
                ],
                summary: "Container bridge watch is reduced.".to_owned(),
                detail_lines: vec![
                    "The container mount bridge can miss execute-bit and ownership changes between refreshes.".to_owned(),
                    "Generated preview output may lag until the next explicit refresh or compare.".to_owned(),
                ],
                actions: vec![
                    action("view_implications", "View implications", true, "Shows which freshness and metadata guarantees are weaker on the container mount."),
                    action("retry_watcher", "Retry watcher", true, "Re-initializes the container-side watch bridge."),
                    action("inspect_root_details", "Inspect root details", true, "Opens root authority, mount, and write-posture details."),
                ],
                support_export: support_export("state.filesystem_truth_review.watch_strip", "sha256:watch-strip-container-preview"),
            },
            WatchFidelityStripRecord {
                strip_id: "watch.generated.draft".to_owned(),
                root_class: MatrixRootClass::GeneratedManaged,
                surface_class: SurfaceClass::ProviderLocalDraft,
                root_label: "Generated provider draft".to_owned(),
                watch_mode: WatchMode::ProviderRefreshOnly,
                reason: Some(WatchReason::ProviderChangeFeedThrottled),
                affected_guarantees: vec![
                    WatchGuaranteeImpact::GeneratedRefresh,
                    WatchGuaranteeImpact::ProviderRevisionFreshness,
                    WatchGuaranteeImpact::ExternalChangeDetection,
                ],
                summary: "Draft freshness depends on provider refresh.".to_owned(),
                detail_lines: vec![
                    "This draft updates when the provider feed or generator refresh publishes a new revision.".to_owned(),
                    "Local chrome should not claim live external-change timing while the feed is throttled.".to_owned(),
                ],
                actions: vec![
                    action("refresh_now", "Refresh now", true, "Requests a provider-backed refresh for the generated draft."),
                    action("inspect_root_details", "Open details", true, "Opens source, provider, and save-posture details for this draft."),
                ],
                support_export: support_export("state.filesystem_truth_review.watch_strip", "sha256:watch-strip-generated-draft"),
            },
        ],
        ignore_resolution_drawers: vec![
            IgnoreResolutionDrawerRecord {
                drawer_id: "ignore.local.hidden_checkpoint".to_owned(),
                root_class: MatrixRootClass::LocalFilesystem,
                surface_class: SurfaceClass::NotebookDocument,
                item_label: ".ipynb_checkpoints/forecast-checkpoint.ipynb".to_owned(),
                visibility_class: IgnoreVisibilityClass::IgnoreHidden,
                contributing_sources: vec![
                    IgnoreSourceEntry {
                        source_class: IgnoreSourceClass::HiddenFile,
                        source_label: "checkpoint folder".to_owned(),
                        summary: "Hidden notebook checkpoints stay out of the default tree and quick-open results.".to_owned(),
                    },
                    IgnoreSourceEntry {
                        source_class: IgnoreSourceClass::ToolIgnore,
                        source_label: "notebook checkpoint filter".to_owned(),
                        summary: "Notebook tooling treats transient checkpoints as support-only recovery data.".to_owned(),
                    },
                ],
                scope_impact: "The checkpoint is absent from the default tree, but it can still be revealed for manual recovery.".to_owned(),
                override_allowed: true,
                summary: "Checkpoint file is hidden by default notebook rules.".to_owned(),
                detail_lines: vec![
                    "This row is hidden because it is a transient checkpoint, not a canonical notebook source.".to_owned(),
                    "Revealing it does not promote it to the authored save target.".to_owned(),
                ],
                actions: vec![
                    action("reveal_in_context", "Reveal in context", true, "Shows the checkpoint in the tree without changing the notebook's canonical target."),
                    action("inspect_ignore_sources", "Inspect ignore sources", true, "Lists the hide rules that excluded the checkpoint."),
                ],
                support_export: support_export("state.filesystem_truth_review.ignore_drawer", "sha256:ignore-local-hidden-checkpoint"),
            },
            IgnoreResolutionDrawerRecord {
                drawer_id: "ignore.remote.scope_limited".to_owned(),
                root_class: MatrixRootClass::RemoteAgent,
                surface_class: SurfaceClass::RequestWorkspaceDocument,
                item_label: "schemas/private/internal.yaml".to_owned(),
                visibility_class: IgnoreVisibilityClass::ScopeLimitedResults,
                contributing_sources: vec![
                    IgnoreSourceEntry {
                        source_class: IgnoreSourceClass::WorksetBoundary,
                        source_label: "request workset".to_owned(),
                        summary: "The active API workset only loads the public request slice.".to_owned(),
                    },
                    IgnoreSourceEntry {
                        source_class: IgnoreSourceClass::ScopeLimit,
                        source_label: "query scope".to_owned(),
                        summary: "The current remote query excludes files outside the selected request scope.".to_owned(),
                    },
                ],
                scope_impact: "The file exists on the remote root but is outside the current slice, so empty results must say scope-limited rather than missing.".to_owned(),
                override_allowed: true,
                summary: "Result is outside the active remote request scope.".to_owned(),
                detail_lines: vec![
                    "This path is not blocked; it is currently excluded by the selected request-workspace slice.".to_owned(),
                    "Expanding scope preserves remote authority cues before the file becomes editable.".to_owned(),
                ],
                actions: vec![
                    action("expand_scope", "Expand scope", true, "Loads the wider remote request slice before reopening the result."),
                    action("reveal_in_context", "Reveal in context", true, "Opens the scoped path with its remote-boundary cues intact."),
                ],
                support_export: support_export("state.filesystem_truth_review.ignore_drawer", "sha256:ignore-remote-scope-limited"),
            },
            IgnoreResolutionDrawerRecord {
                drawer_id: "ignore.container.generated_overlay".to_owned(),
                root_class: MatrixRootClass::ContainerMount,
                surface_class: SurfaceClass::PreviewOutputArtifact,
                item_label: "preview/.cache/rendered/index.html".to_owned(),
                visibility_class: IgnoreVisibilityClass::GeneratedOverlayHidden,
                contributing_sources: vec![
                    IgnoreSourceEntry {
                        source_class: IgnoreSourceClass::GeneratedOverlay,
                        source_label: "preview output cache".to_owned(),
                        summary: "Rendered preview cache files are derived output, not primary authored sources.".to_owned(),
                    },
                    IgnoreSourceEntry {
                        source_class: IgnoreSourceClass::WorkspaceExclude,
                        source_label: "preview cache exclude".to_owned(),
                        summary: "The workspace hides cache artifacts from ordinary navigation to prevent wrong-target edits.".to_owned(),
                    },
                ],
                scope_impact: "The cache row is absent from default explorer and search views because preview output should route through the generated-source relation first.".to_owned(),
                override_allowed: true,
                summary: "Generated overlay is hidden behind the preview source relation.".to_owned(),
                detail_lines: vec![
                    "Opening the cache file directly would bypass the generated-source relation and portability notes.".to_owned(),
                    "Use the generated-source action when you need to inspect why the overlay exists.".to_owned(),
                ],
                actions: vec![
                    action("open_generated_source", "Open generated source", true, "Opens the source-owned preview relation instead of the cache artifact."),
                    action("inspect_ignore_sources", "Inspect ignore sources", true, "Shows the generated-overlay and workspace-hide rules."),
                ],
                support_export: support_export("state.filesystem_truth_review.ignore_drawer", "sha256:ignore-container-generated-overlay"),
            },
            IgnoreResolutionDrawerRecord {
                drawer_id: "ignore.generated.policy_hidden".to_owned(),
                root_class: MatrixRootClass::GeneratedManaged,
                surface_class: SurfaceClass::ProviderLocalDraft,
                item_label: "providers/internal/review_commentary.json".to_owned(),
                visibility_class: IgnoreVisibilityClass::PolicyHidden,
                contributing_sources: vec![IgnoreSourceEntry {
                    source_class: IgnoreSourceClass::PolicyHidden,
                    source_label: "admin-hidden generated path".to_owned(),
                    summary: "Administrative policy narrows this provider-backed commentary row on the current tenant.".to_owned(),
                }],
                scope_impact: "The row is hidden by policy, not by a user override, so reopening it requires policy detail rather than a local reveal toggle.".to_owned(),
                override_allowed: false,
                summary: "Generated commentary row is policy-hidden.".to_owned(),
                detail_lines: vec![
                    "This path is excluded by an administrative rule on provider-backed generated content.".to_owned(),
                    "The draft remains editable where policy still admits the underlying authored source.".to_owned(),
                ],
                actions: vec![
                    action("open_policy_details", "Open policy details", true, "Shows the policy reason and the owner of the next step."),
                ],
                support_export: support_export("state.filesystem_truth_review.ignore_drawer", "sha256:ignore-generated-policy-hidden"),
            },
        ],
        external_change_reviews: vec![
            ExternalChangeReviewRecord {
                review_id: "external.local.notebook".to_owned(),
                root_class: MatrixRootClass::LocalFilesystem,
                surface_class: SurfaceClass::NotebookDocument,
                presentation_path: "notebooks/forecast.ipynb".to_owned(),
                logical_identity_ref: "workspace://labs/notebooks/forecast.ipynb".to_owned(),
                pinned_canonical_target: "file:///workspace/notebooks/forecast.ipynb".to_owned(),
                observed_canonical_target: Some("file:///workspace/notebooks/forecast.ipynb".to_owned()),
                identity_note: "Same canonical target, opened through the authored notebook path.".to_owned(),
                change_source: "External process".to_owned(),
                compare_outcome: ExternalChangeCompareOutcome::ExternalChangeDetected,
                diff_availability: ExternalChangeDiffAvailability::Available,
                content_kind: ExternalChangeContentKind::Text,
                diff_summary: "Notebook cell metadata changed on disk after the buffer was staged.".to_owned(),
                diff_preview_lines: vec![
                    "- \"execution_count\": 17".to_owned(),
                    "+ \"execution_count\": 18".to_owned(),
                    "+ \"metadata\": {\"last_run\": \"2026-06-12T09:14:00Z\"}".to_owned(),
                ],
                metadata_delta_notes: vec![MetadataDeltaNote {
                    kind: MetadataDeltaKind::NewlineModeChanged,
                    summary: "The notebook file now uses LF line endings, so a keep-local save would also preserve the staging buffer's normalized newline mode.".to_owned(),
                }],
                silent_overwrite_forbidden: true,
                actions: vec![
                    action("compare_to_disk", "Compare to disk", true, "Opens the compare-first diff before any write is allowed."),
                    action("reload", "Reload", true, "Replaces the local buffer with the current notebook bytes after review."),
                    action("keep_local", "Keep local", true, "Keeps the local buffer open without writing over the newer on-disk notebook."),
                    action("merge_manually", "Merge manually", true, "Opens a manual merge flow for the notebook metadata drift."),
                    action("save_elsewhere", "Save elsewhere", true, "Preserves the local buffer to a different target without overwriting the canonical notebook."),
                ],
                detail_lines: vec![
                    "This review is diff-first because the canonical notebook target changed after the local buffer staged.".to_owned(),
                    "A keep-local choice preserves the buffer state but does not claim the on-disk notebook still matches it.".to_owned(),
                ],
                support_export: support_export("state.filesystem_truth_review.external_change", "sha256:external-local-notebook"),
            },
            ExternalChangeReviewRecord {
                review_id: "external.remote.request".to_owned(),
                root_class: MatrixRootClass::RemoteAgent,
                surface_class: SurfaceClass::RequestWorkspaceDocument,
                presentation_path: "requests/payments.http".to_owned(),
                logical_identity_ref: "workspace://remote-payments/requests/payments.http".to_owned(),
                pinned_canonical_target: "agent://remote-1/object/request-file-42@rev-17".to_owned(),
                observed_canonical_target: Some("agent://remote-1/object/request-file-42@rev-18".to_owned()),
                identity_note: "The presentation path still points at the same remote object id, but the provider revision advanced.".to_owned(),
                change_source: "Remote agent".to_owned(),
                compare_outcome: ExternalChangeCompareOutcome::SaveConflict,
                diff_availability: ExternalChangeDiffAvailability::Available,
                content_kind: ExternalChangeContentKind::Text,
                diff_summary: "Remote request draft changed on revision rev-18.".to_owned(),
                diff_preview_lines: vec![
                    "- Authorization: Bearer {{token}}".to_owned(),
                    "+ Authorization: Bearer {{rotated_token}}".to_owned(),
                    "+ X-Trace-Seed: req-448".to_owned(),
                ],
                metadata_delta_notes: vec![MetadataDeltaNote {
                    kind: MetadataDeltaKind::ProviderRevisionChanged,
                    summary: "Remote compare-before-write must now target revision rev-18 rather than the staged rev-17 token.".to_owned(),
                }],
                silent_overwrite_forbidden: true,
                actions: vec![
                    action("compare_to_disk", "Compare to remote", true, "Shows the rev-17 versus rev-18 diff before any conditional write."),
                    action("reload", "Reload", true, "Adopts the current remote revision after review."),
                    action("keep_local", "Keep local", true, "Keeps the local buffer staged without discarding the remote revision evidence."),
                    action("merge_manually", "Merge manually", true, "Lets the user reconcile the remote request delta into the local draft."),
                    action("save_elsewhere", "Save elsewhere", true, "Exports the local request body without overwriting the remote canonical target."),
                ],
                detail_lines: vec![
                    "Remote conditional write safety depends on the provider revision, not only the visible path string.".to_owned(),
                    "Reload and keep-local remain distinct because the remote object is authoritative for the canonical target.".to_owned(),
                ],
                support_export: support_export("state.filesystem_truth_review.external_change", "sha256:external-remote-request"),
            },
            ExternalChangeReviewRecord {
                review_id: "external.container.preview".to_owned(),
                root_class: MatrixRootClass::ContainerMount,
                surface_class: SurfaceClass::PreviewOutputArtifact,
                presentation_path: "preview/rendered/index.html".to_owned(),
                logical_identity_ref: "workspace://preview-demo/preview/rendered/index.html".to_owned(),
                pinned_canonical_target: "container://preview-root/rendered/index.html".to_owned(),
                observed_canonical_target: Some("container://preview-root/rendered/index.html".to_owned()),
                identity_note: "Same rendered target, but the container generator published a newer render.".to_owned(),
                change_source: "Container runtime".to_owned(),
                compare_outcome: ExternalChangeCompareOutcome::ExternalChangeDetected,
                diff_availability: ExternalChangeDiffAvailability::Available,
                content_kind: ExternalChangeContentKind::Text,
                diff_summary: "Rendered preview output refreshed in the container.".to_owned(),
                diff_preview_lines: vec![
                    "- data-build-rev=\"144\"".to_owned(),
                    "+ data-build-rev=\"145\"".to_owned(),
                    "+ <meta name=\"preview-runtime\" content=\"container-44\" />".to_owned(),
                ],
                metadata_delta_notes: vec![
                    MetadataDeltaNote {
                        kind: MetadataDeltaKind::ExecuteBitChanged,
                        summary: "The container-side artifact switched executable posture for an attached helper script.".to_owned(),
                    },
                    MetadataDeltaNote {
                        kind: MetadataDeltaKind::GeneratorRevisionChanged,
                        summary: "The preview generator revision advanced, so the compare is tied to a new render identity.".to_owned(),
                    },
                ],
                silent_overwrite_forbidden: true,
                actions: vec![
                    action("compare_to_disk", "Compare to rendered output", true, "Shows the rendered output diff before any local export or reload."),
                    action("reload", "Reload", true, "Reloads the current rendered artifact from the container."),
                    action("keep_local", "Keep local", true, "Keeps the local view state without claiming the rendered artifact is current."),
                    action("merge_manually", "Merge manually", true, "Lets the user inspect the generated diff before deciding how to export or copy."),
                    action("save_elsewhere", "Save elsewhere", true, "Copies the local artifact elsewhere without overwriting the generated target."),
                ],
                detail_lines: vec![
                    "Rendered output is identity-aware: the compare notes the container generator revision as well as the visible file path.".to_owned(),
                    "The generated target should not be overwritten blindly just because the preview path matches.".to_owned(),
                ],
                support_export: support_export("state.filesystem_truth_review.external_change", "sha256:external-container-preview"),
            },
            ExternalChangeReviewRecord {
                review_id: "external.generated.draft".to_owned(),
                root_class: MatrixRootClass::GeneratedManaged,
                surface_class: SurfaceClass::ProviderLocalDraft,
                presentation_path: "providers/drafts/review.md".to_owned(),
                logical_identity_ref: "workspace://provider-drafts/review.md".to_owned(),
                pinned_canonical_target: "generated://provider-drafts/review.md?draft=91".to_owned(),
                observed_canonical_target: Some("generated://provider-drafts/review.md?draft=92".to_owned()),
                identity_note: "The visible draft label is stable, but the generated canonical draft id advanced with a provider sync.".to_owned(),
                change_source: "Provider sync".to_owned(),
                compare_outcome: ExternalChangeCompareOutcome::ExternalChangeDetected,
                diff_availability: ExternalChangeDiffAvailability::Available,
                content_kind: ExternalChangeContentKind::Text,
                diff_summary: "Provider-backed draft refreshed to a newer generated revision.".to_owned(),
                diff_preview_lines: vec![
                    "- status: awaiting-review".to_owned(),
                    "+ status: comment-added".to_owned(),
                    "+ provenance: provider-sync-92".to_owned(),
                ],
                metadata_delta_notes: vec![MetadataDeltaNote {
                    kind: MetadataDeltaKind::GeneratorRevisionChanged,
                    summary: "The generated draft id changed from 91 to 92, so the review must keep the logical draft and canonical target separate.".to_owned(),
                }],
                silent_overwrite_forbidden: true,
                actions: vec![
                    action("compare_to_disk", "Compare to draft source", true, "Shows the previous and current provider-backed draft revisions."),
                    action("reload", "Reload", true, "Adopts the latest generated draft revision."),
                    action("keep_local", "Keep local", true, "Keeps the local draft buffer without claiming the generated revision stayed fixed."),
                    action("merge_manually", "Merge manually", false, "Manual merge is unavailable on this generated projection; reopen the authored source or save elsewhere."),
                    action("save_elsewhere", "Save elsewhere", true, "Writes the local draft copy to a new authored target instead of the provider-managed projection."),
                ],
                detail_lines: vec![
                    "Generated drafts must disclose logical versus target identity before a user decides whether to reload or preserve a local copy.".to_owned(),
                    "The disabled manual-merge action remains visible so the surface does not imply a silent fallback path.".to_owned(),
                ],
                support_export: support_export("state.filesystem_truth_review.external_change", "sha256:external-generated-draft"),
            },
        ],
        cross_root_move_reviews: vec![
            CrossRootMoveReviewRecord {
                review_id: "move.local.to_container".to_owned(),
                surface_class: SurfaceClass::NotebookDocument,
                source_root_class: MatrixRootClass::LocalFilesystem,
                target_root_class: MatrixRootClass::ContainerMount,
                operation_label: "Move notebook into container scratch".to_owned(),
                source_path: "notebooks/forecast.ipynb".to_owned(),
                target_path: "container://analysis/tmp/forecast.ipynb".to_owned(),
                case_sensitivity_note: "Both roots preserve case, but the destination mount applies container ownership semantics.".to_owned(),
                normalization_note: Some("No Unicode normalization change is requested, but the target mount canonicalizes through the container path map.".to_owned()),
                boundary_crossing: BoundaryCrossingKind::ContainerMountCrossing,
                metadata_consequences: vec![
                    MetadataConsequence::OwnershipMayChange,
                    MetadataConsequence::PermissionModelChanges,
                    MetadataConsequence::SaveBecomesCopyThenReview,
                ],
                summary: "This move crosses from a local authored root into a container-owned mount.".to_owned(),
                detail_lines: vec![
                    "The operation is reviewed as copy then review rather than a hidden in-place rename.".to_owned(),
                    "Notebook permissions and ownership may differ once the file lands on the container mount.".to_owned(),
                ],
                actions: vec![
                    action("preview_plan", "Preview plan", true, "Shows the source, target, and metadata consequences before any copy occurs."),
                    action("proceed", "Proceed", true, "Stages the reviewed copy into the container target."),
                    action("cancel", "Cancel", true, "Cancels the cross-root move without changing either root."),
                ],
                support_export: support_export("state.filesystem_truth_review.cross_root_move", "sha256:move-local-to-container"),
            },
            CrossRootMoveReviewRecord {
                review_id: "move.remote.to_local".to_owned(),
                surface_class: SurfaceClass::RequestWorkspaceDocument,
                source_root_class: MatrixRootClass::RemoteAgent,
                target_root_class: MatrixRootClass::LocalFilesystem,
                operation_label: "Copy remote request draft to local workspace".to_owned(),
                source_path: "agent://remote-1/requests/payments.http".to_owned(),
                target_path: "requests/payments.http.local".to_owned(),
                case_sensitivity_note: "The local target preserves case, but the remote source remains revision-conditioned.".to_owned(),
                normalization_note: Some("Target naming is unchanged; the meaningful shift is authority, not Unicode spelling.".to_owned()),
                boundary_crossing: BoundaryCrossingKind::RemoteAuthorityChange,
                metadata_consequences: vec![
                    MetadataConsequence::ProviderPreconditionRequired,
                    MetadataConsequence::PermissionModelChanges,
                    MetadataConsequence::SaveBecomesCopyThenReview,
                ],
                summary: "This transfer crosses from remote provider authority into a local authored file.".to_owned(),
                detail_lines: vec![
                    "The remote object must keep its revision precondition even while the local copy is being prepared.".to_owned(),
                    "Deleting or replacing the remote source is a separate reviewed mutation, not part of this copy.".to_owned(),
                ],
                actions: vec![
                    action("preview_plan", "Preview plan", true, "Shows the remote revision, local target, and follow-up review posture."),
                    action("save_as_copy", "Save as copy", true, "Creates the reviewed local copy while preserving the remote canonical source."),
                    action("cancel", "Cancel", true, "Cancels the transfer without mutating the remote object."),
                ],
                support_export: support_export("state.filesystem_truth_review.cross_root_move", "sha256:move-remote-to-local"),
            },
            CrossRootMoveReviewRecord {
                review_id: "move.container.overlay_crossing".to_owned(),
                surface_class: SurfaceClass::PreviewOutputArtifact,
                source_root_class: MatrixRootClass::ContainerMount,
                target_root_class: MatrixRootClass::LocalFilesystem,
                operation_label: "Copy rendered preview artifact out of container".to_owned(),
                source_path: "container://preview-root/rendered/index.html".to_owned(),
                target_path: "preview_exports/index.html".to_owned(),
                case_sensitivity_note: "The local export path preserves case, but container ownership and execute posture may not round-trip exactly.".to_owned(),
                normalization_note: Some("No normalization change is requested; the risk is mount-boundary portability.".to_owned()),
                boundary_crossing: BoundaryCrossingKind::ContainerMountCrossing,
                metadata_consequences: vec![
                    MetadataConsequence::ExecuteBitMayDrop,
                    MetadataConsequence::OwnershipMayChange,
                    MetadataConsequence::SaveBecomesCopyThenReview,
                ],
                summary: "This operation crosses out of a container-backed generated output root.".to_owned(),
                detail_lines: vec![
                    "The exported artifact may lose execute-bit or ownership metadata when it leaves the container root.".to_owned(),
                    "The generated container artifact remains the source of truth; the local export is a reviewed copy.".to_owned(),
                ],
                actions: vec![
                    action("preview_plan", "Preview plan", true, "Shows the export path and metadata changes before copying out of the container."),
                    action("proceed", "Proceed", true, "Creates the reviewed export copy."),
                    action("cancel", "Cancel", true, "Keeps the generated artifact in place and aborts the export."),
                ],
                support_export: support_export("state.filesystem_truth_review.cross_root_move", "sha256:move-container-overlay-crossing"),
            },
            CrossRootMoveReviewRecord {
                review_id: "move.generated.detach_to_local".to_owned(),
                surface_class: SurfaceClass::ProviderLocalDraft,
                source_root_class: MatrixRootClass::GeneratedManaged,
                target_root_class: MatrixRootClass::LocalFilesystem,
                operation_label: "Detach generated draft into local authored copy".to_owned(),
                source_path: "generated://provider-drafts/review.md?draft=92".to_owned(),
                target_path: "drafts/review.detached.md".to_owned(),
                case_sensitivity_note: "The target is an authored local file; future writes no longer inherit generated-draft refresh behavior.".to_owned(),
                normalization_note: Some("The path spelling change is ordinary; the meaningful change is detaching generated lineage.".to_owned()),
                boundary_crossing: BoundaryCrossingKind::GeneratedLineageDetach,
                metadata_consequences: vec![
                    MetadataConsequence::GeneratedLineageDetached,
                    MetadataConsequence::PermissionModelChanges,
                    MetadataConsequence::SaveBecomesCopyThenReview,
                ],
                summary: "This detach changes future write authority from generated draft to authored local file.".to_owned(),
                detail_lines: vec![
                    "Detaching the draft creates a new authored target and ends provider-managed refresh for that copy.".to_owned(),
                    "The review keeps source lineage and the new local authority visible before proceed.".to_owned(),
                ],
                actions: vec![
                    action("preview_plan", "Preview plan", true, "Shows generated lineage, local destination, and post-detach write posture."),
                    action("save_as_copy", "Save as copy", true, "Creates the local authored copy without mutating the generated source."),
                    action("cancel", "Cancel", true, "Cancels the detach and keeps the provider-backed draft authoritative."),
                ],
                support_export: support_export("state.filesystem_truth_review.cross_root_move", "sha256:move-generated-detach-to-local"),
            },
        ],
        scenarios: vec![
            ReviewScenarioRecord {
                scenario_id: "scenario.local.notebook".to_owned(),
                consumer_ref: "notebook.header".to_owned(),
                scenario: "Local notebook editing shows degraded watch truth, a hidden checkpoint drawer, compare-first external-change review, and explicit copy-into-container review.".to_owned(),
                root_class: MatrixRootClass::LocalFilesystem,
                surface_class: SurfaceClass::NotebookDocument,
                watch_strip_id: "watch.local.notebook".to_owned(),
                ignore_drawer_id: "ignore.local.hidden_checkpoint".to_owned(),
                external_change_review_id: "external.local.notebook".to_owned(),
                cross_root_move_review_id: "move.local.to_container".to_owned(),
            },
            ReviewScenarioRecord {
                scenario_id: "scenario.remote.request".to_owned(),
                consumer_ref: "request.workspace.editor".to_owned(),
                scenario: "Remote request editing exposes polling fallback, scope-limited results, revision-aware external-change review, and remote-to-local copy review.".to_owned(),
                root_class: MatrixRootClass::RemoteAgent,
                surface_class: SurfaceClass::RequestWorkspaceDocument,
                watch_strip_id: "watch.remote.request".to_owned(),
                ignore_drawer_id: "ignore.remote.scope_limited".to_owned(),
                external_change_review_id: "external.remote.request".to_owned(),
                cross_root_move_review_id: "move.remote.to_local".to_owned(),
            },
            ReviewScenarioRecord {
                scenario_id: "scenario.container.preview".to_owned(),
                consumer_ref: "preview.output.header".to_owned(),
                scenario: "Container preview output stays explicit about reduced watch fidelity, generated-overlay absence, compare-first drift review, and copy-out consequences.".to_owned(),
                root_class: MatrixRootClass::ContainerMount,
                surface_class: SurfaceClass::PreviewOutputArtifact,
                watch_strip_id: "watch.container.preview".to_owned(),
                ignore_drawer_id: "ignore.container.generated_overlay".to_owned(),
                external_change_review_id: "external.container.preview".to_owned(),
                cross_root_move_review_id: "move.container.overlay_crossing".to_owned(),
            },
            ReviewScenarioRecord {
                scenario_id: "scenario.generated.draft".to_owned(),
                consumer_ref: "provider.draft.editor".to_owned(),
                scenario: "Generated provider drafts surface provider-refresh-only truth, policy-hidden commentary, identity-aware compare-first review, and lineage-detach copy review.".to_owned(),
                root_class: MatrixRootClass::GeneratedManaged,
                surface_class: SurfaceClass::ProviderLocalDraft,
                watch_strip_id: "watch.generated.draft".to_owned(),
                ignore_drawer_id: "ignore.generated.policy_hidden".to_owned(),
                external_change_review_id: "external.generated.draft".to_owned(),
                cross_root_move_review_id: "move.generated.detach_to_local".to_owned(),
            },
        ],
    }
}

/// Returns the checked-in fixture corpus for the packet.
pub fn seeded_filesystem_truth_review_fixtures() -> Vec<FilesystemTruthReviewFixture> {
    vec![
        FilesystemTruthReviewFixture {
            record_kind: FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION,
            fixture_id: "fixture.local.notebook".to_owned(),
            expected_scenario_id: "scenario.local.notebook".to_owned(),
            root_class: MatrixRootClass::LocalFilesystem,
            surface_class: SurfaceClass::NotebookDocument,
            consumer_ref: "notebook.header".to_owned(),
            scenario: "Notebook lane keeps checkpoint absence, degraded watch, compare-first review, and container copy review distinct.".to_owned(),
            expected_watch_mode: WatchMode::ReducedFidelityWatch,
            expected_ignore_visibility: IgnoreVisibilityClass::IgnoreHidden,
            expected_compare_outcome: ExternalChangeCompareOutcome::ExternalChangeDetected,
            expected_boundary_crossing: BoundaryCrossingKind::ContainerMountCrossing,
            required_action_ids: vec![
                "retry_watcher".to_owned(),
                "reveal_in_context".to_owned(),
                "compare_to_disk".to_owned(),
                "preview_plan".to_owned(),
            ],
        },
        FilesystemTruthReviewFixture {
            record_kind: FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION,
            fixture_id: "fixture.remote.request".to_owned(),
            expected_scenario_id: "scenario.remote.request".to_owned(),
            root_class: MatrixRootClass::RemoteAgent,
            surface_class: SurfaceClass::RequestWorkspaceDocument,
            consumer_ref: "request.workspace.editor".to_owned(),
            scenario: "Remote request lane keeps polling fallback, scope-limited absence, revision-aware compare, and remote-to-local copy review explicit.".to_owned(),
            expected_watch_mode: WatchMode::PollingFallback,
            expected_ignore_visibility: IgnoreVisibilityClass::ScopeLimitedResults,
            expected_compare_outcome: ExternalChangeCompareOutcome::SaveConflict,
            expected_boundary_crossing: BoundaryCrossingKind::RemoteAuthorityChange,
            required_action_ids: vec![
                "change_interval".to_owned(),
                "expand_scope".to_owned(),
                "merge_manually".to_owned(),
                "save_as_copy".to_owned(),
            ],
        },
        FilesystemTruthReviewFixture {
            record_kind: FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION,
            fixture_id: "fixture.container.preview".to_owned(),
            expected_scenario_id: "scenario.container.preview".to_owned(),
            root_class: MatrixRootClass::ContainerMount,
            surface_class: SurfaceClass::PreviewOutputArtifact,
            consumer_ref: "preview.output.header".to_owned(),
            scenario: "Container preview lane stays honest about generated overlay absence and portability consequences when copied out.".to_owned(),
            expected_watch_mode: WatchMode::ReducedFidelityWatch,
            expected_ignore_visibility: IgnoreVisibilityClass::GeneratedOverlayHidden,
            expected_compare_outcome: ExternalChangeCompareOutcome::ExternalChangeDetected,
            expected_boundary_crossing: BoundaryCrossingKind::ContainerMountCrossing,
            required_action_ids: vec![
                "view_implications".to_owned(),
                "open_generated_source".to_owned(),
                "reload".to_owned(),
                "proceed".to_owned(),
            ],
        },
        FilesystemTruthReviewFixture {
            record_kind: FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION,
            fixture_id: "fixture.generated.draft".to_owned(),
            expected_scenario_id: "scenario.generated.draft".to_owned(),
            root_class: MatrixRootClass::GeneratedManaged,
            surface_class: SurfaceClass::ProviderLocalDraft,
            consumer_ref: "provider.draft.editor".to_owned(),
            scenario: "Generated draft lane keeps provider-refresh truth, policy-hidden rows, identity-aware compare, and lineage-detach copy review explicit.".to_owned(),
            expected_watch_mode: WatchMode::ProviderRefreshOnly,
            expected_ignore_visibility: IgnoreVisibilityClass::PolicyHidden,
            expected_compare_outcome: ExternalChangeCompareOutcome::ExternalChangeDetected,
            expected_boundary_crossing: BoundaryCrossingKind::GeneratedLineageDetach,
            required_action_ids: vec![
                "refresh_now".to_owned(),
                "open_policy_details".to_owned(),
                "keep_local".to_owned(),
                "save_as_copy".to_owned(),
            ],
        },
    ]
}

/// Validates the checked-in packet.
pub fn validate_filesystem_truth_review_packet(
    packet: &FilesystemTruthReviewPacket,
) -> Result<(), ReviewSurfaceValidationReport> {
    let mut report = ReviewSurfaceValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != FILESYSTEM_TRUTH_REVIEW_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet.record_kind",
            "record_kind must match the frozen packet tag",
        );
    }
    if packet.schema_version != FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            "packet.schema_version",
            "schema_version must match the frozen packet version",
        );
    }
    if packet.watch_fidelity_strips.is_empty()
        || packet.ignore_resolution_drawers.is_empty()
        || packet.external_change_reviews.is_empty()
        || packet.cross_root_move_reviews.is_empty()
        || packet.scenarios.is_empty()
    {
        report.push(
            "packet.sections.non_empty",
            "packet",
            "all packet sections must contain at least one record",
        );
    }

    let watch_map = unique_ids(
        &packet.watch_fidelity_strips,
        |row| row.strip_id.as_str(),
        "watch_fidelity_strips",
        &mut report,
    );
    let ignore_map = unique_ids(
        &packet.ignore_resolution_drawers,
        |row| row.drawer_id.as_str(),
        "ignore_resolution_drawers",
        &mut report,
    );
    let external_map = unique_ids(
        &packet.external_change_reviews,
        |row| row.review_id.as_str(),
        "external_change_reviews",
        &mut report,
    );
    let move_map = unique_ids(
        &packet.cross_root_move_reviews,
        |row| row.review_id.as_str(),
        "cross_root_move_reviews",
        &mut report,
    );

    for strip in &packet.watch_fidelity_strips {
        validate_watch_strip(strip, &mut report);
    }
    for drawer in &packet.ignore_resolution_drawers {
        validate_ignore_drawer(drawer, &mut report);
    }
    for review in &packet.external_change_reviews {
        validate_external_change_review(review, &mut report);
    }
    for review in &packet.cross_root_move_reviews {
        validate_cross_root_review(review, &mut report);
    }

    let mut scenario_ids = BTreeSet::new();
    let mut scenario_roots = BTreeSet::new();
    for scenario in &packet.scenarios {
        if !scenario_ids.insert(scenario.scenario_id.as_str()) {
            report.push(
                "scenario.id.duplicate",
                format!("scenarios.{}", scenario.scenario_id),
                "scenario ids must be unique",
            );
        }
        scenario_roots.insert(scenario.root_class);
        if !watch_map.contains_key(scenario.watch_strip_id.as_str()) {
            report.push(
                "scenario.watch_strip.missing",
                format!("scenarios.{}.watch_strip_id", scenario.scenario_id),
                "scenario must reference an existing watch strip",
            );
        }
        if !ignore_map.contains_key(scenario.ignore_drawer_id.as_str()) {
            report.push(
                "scenario.ignore_drawer.missing",
                format!("scenarios.{}.ignore_drawer_id", scenario.scenario_id),
                "scenario must reference an existing ignore drawer",
            );
        }
        if !external_map.contains_key(scenario.external_change_review_id.as_str()) {
            report.push(
                "scenario.external_change.missing",
                format!("scenarios.{}.external_change_review_id", scenario.scenario_id),
                "scenario must reference an existing external-change review",
            );
        }
        if !move_map.contains_key(scenario.cross_root_move_review_id.as_str()) {
            report.push(
                "scenario.cross_root_move.missing",
                format!("scenarios.{}.cross_root_move_review_id", scenario.scenario_id),
                "scenario must reference an existing cross-root move review",
            );
        }
    }

    for required in [
        MatrixRootClass::LocalFilesystem,
        MatrixRootClass::RemoteAgent,
        MatrixRootClass::ContainerMount,
        MatrixRootClass::GeneratedManaged,
    ] {
        if !scenario_roots.contains(&required) {
            report.push(
                "scenario.root_coverage",
                "scenarios",
                format!("scenario coverage must include root class {}", required.as_str()),
            );
        }
    }

    report.finish()
}

/// Validates one checked-in fixture against the packet.
pub fn validate_filesystem_truth_review_fixture(
    packet: &FilesystemTruthReviewPacket,
    fixture: &FilesystemTruthReviewFixture,
) -> Result<(), ReviewSurfaceValidationReport> {
    let mut report = ReviewSurfaceValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!("fixtures.{}", fixture.fixture_id),
            "record_kind must match the frozen fixture tag",
        );
    }
    if fixture.schema_version != FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!("fixtures.{}", fixture.fixture_id),
            "schema_version must match the frozen packet version",
        );
    }

    let scenario = packet
        .scenarios
        .iter()
        .find(|scenario| scenario.scenario_id == fixture.expected_scenario_id);
    let Some(scenario) = scenario else {
        report.push(
            "fixture.scenario.missing",
            format!("fixtures.{}.expected_scenario_id", fixture.fixture_id),
            "fixture must reference an existing scenario",
        );
        return report.finish();
    };

    if scenario.root_class != fixture.root_class {
        report.push(
            "fixture.root_class",
            format!("fixtures.{}.root_class", fixture.fixture_id),
            "fixture root_class must match the scenario root_class",
        );
    }
    if scenario.surface_class != fixture.surface_class {
        report.push(
            "fixture.surface_class",
            format!("fixtures.{}.surface_class", fixture.fixture_id),
            "fixture surface_class must match the scenario surface_class",
        );
    }
    if scenario.consumer_ref != fixture.consumer_ref {
        report.push(
            "fixture.consumer_ref",
            format!("fixtures.{}.consumer_ref", fixture.fixture_id),
            "fixture consumer_ref must match the scenario consumer_ref",
        );
    }

    let strip = packet
        .watch_fidelity_strips
        .iter()
        .find(|row| row.strip_id == scenario.watch_strip_id);
    let drawer = packet
        .ignore_resolution_drawers
        .iter()
        .find(|row| row.drawer_id == scenario.ignore_drawer_id);
    let external = packet
        .external_change_reviews
        .iter()
        .find(|row| row.review_id == scenario.external_change_review_id);
    let cross_root = packet
        .cross_root_move_reviews
        .iter()
        .find(|row| row.review_id == scenario.cross_root_move_review_id);

    match strip {
        Some(strip) if strip.watch_mode != fixture.expected_watch_mode => report.push(
            "fixture.watch_mode",
            format!("fixtures.{}.expected_watch_mode", fixture.fixture_id),
            "fixture expected_watch_mode must match the scenario watch strip",
        ),
        None => report.push(
            "fixture.watch_strip.missing",
            format!("fixtures.{}.expected_watch_mode", fixture.fixture_id),
            "scenario watch strip must exist",
        ),
        _ => {}
    }
    match drawer {
        Some(drawer) if drawer.visibility_class != fixture.expected_ignore_visibility => report.push(
            "fixture.ignore_visibility",
            format!("fixtures.{}.expected_ignore_visibility", fixture.fixture_id),
            "fixture expected_ignore_visibility must match the scenario drawer",
        ),
        None => report.push(
            "fixture.ignore_drawer.missing",
            format!("fixtures.{}.expected_ignore_visibility", fixture.fixture_id),
            "scenario ignore drawer must exist",
        ),
        _ => {}
    }
    match external {
        Some(external) if external.compare_outcome != fixture.expected_compare_outcome => report.push(
            "fixture.compare_outcome",
            format!("fixtures.{}.expected_compare_outcome", fixture.fixture_id),
            "fixture expected_compare_outcome must match the scenario external-change review",
        ),
        None => report.push(
            "fixture.external_change.missing",
            format!("fixtures.{}.expected_compare_outcome", fixture.fixture_id),
            "scenario external-change review must exist",
        ),
        _ => {}
    }
    match cross_root {
        Some(cross_root)
            if cross_root.boundary_crossing != fixture.expected_boundary_crossing =>
        {
            report.push(
                "fixture.boundary_crossing",
                format!("fixtures.{}.expected_boundary_crossing", fixture.fixture_id),
                "fixture expected_boundary_crossing must match the scenario move review",
            )
        }
        None => report.push(
            "fixture.cross_root_move.missing",
            format!("fixtures.{}.expected_boundary_crossing", fixture.fixture_id),
            "scenario cross-root move review must exist",
        ),
        _ => {}
    }

    let mut action_ids = BTreeSet::new();
    if let Some(strip) = strip {
        for action in &strip.actions {
            action_ids.insert(action.action_id.as_str());
        }
    }
    if let Some(drawer) = drawer {
        for action in &drawer.actions {
            action_ids.insert(action.action_id.as_str());
        }
    }
    if let Some(external) = external {
        for action in &external.actions {
            action_ids.insert(action.action_id.as_str());
        }
    }
    if let Some(cross_root) = cross_root {
        for action in &cross_root.actions {
            action_ids.insert(action.action_id.as_str());
        }
    }
    for action_id in &fixture.required_action_ids {
        if !action_ids.contains(action_id.as_str()) {
            report.push(
                "fixture.required_action_ids",
                format!("fixtures.{}.required_action_ids", fixture.fixture_id),
                format!("required action id {action_id} must be present on the bound scenario"),
            );
        }
    }

    report.finish()
}

fn unique_ids<'a, T>(
    rows: &'a [T],
    id: impl Fn(&'a T) -> &'a str,
    section: &str,
    report: &mut ReviewSurfaceValidationReport,
) -> BTreeMap<&'a str, &'a T> {
    let mut map = BTreeMap::new();
    for row in rows {
        let key = id(row);
        if map.insert(key, row).is_some() {
            report.push(
                "record.id.duplicate",
                format!("{section}.{key}"),
                "record ids must be unique within their section",
            );
        }
    }
    map
}

fn validate_actions(
    actions: &[ActionOffer],
    allowed: &[&str],
    field_path: &str,
    report: &mut ReviewSurfaceValidationReport,
) {
    if actions.is_empty() {
        report.push(
            "actions.non_empty",
            field_path,
            "at least one action must be offered",
        );
        return;
    }
    let allowed: BTreeSet<_> = allowed.iter().copied().collect();
    let mut seen = BTreeSet::new();
    for (index, action) in actions.iter().enumerate() {
        let base = format!("{field_path}.{index}");
        if !allowed.contains(action.action_id.as_str()) {
            report.push(
                "actions.allowed_set",
                format!("{base}.action_id"),
                format!("action_id {} is outside the closed vocabulary", action.action_id),
            );
        }
        if !seen.insert(action.action_id.as_str()) {
            report.push(
                "actions.duplicate",
                format!("{base}.action_id"),
                "action ids must not repeat within one surface record",
            );
        }
        if action.label.trim().is_empty() || action.outcome_summary.trim().is_empty() {
            report.push(
                "actions.label_or_summary",
                base,
                "action labels and summaries must be non-empty",
            );
        }
    }
}

fn validate_support_export(
    export: &SupportExport,
    field_path: &str,
    report: &mut ReviewSurfaceValidationReport,
) {
    if export.packet_family.trim().is_empty()
        || export.redaction_policy.trim().is_empty()
        || export.parity_signature.trim().is_empty()
    {
        report.push(
            "support_export.non_empty",
            field_path,
            "support export metadata fields must be non-empty",
        );
    }
}

fn validate_watch_strip(
    strip: &WatchFidelityStripRecord,
    report: &mut ReviewSurfaceValidationReport,
) {
    let base = format!("watch_fidelity_strips.{}", strip.strip_id);
    if strip.summary.trim().is_empty() || strip.detail_lines.is_empty() {
        report.push(
            "watch_strip.summary_or_details",
            &base,
            "watch strips must carry a summary and at least one detail line",
        );
    }
    match strip.watch_mode {
        WatchMode::LiveWatch => {
            if strip.reason.is_some() || !strip.affected_guarantees.is_empty() {
                report.push(
                    "watch_strip.live_mode",
                    &base,
                    "live_watch must not claim a downgrade reason or affected guarantees",
                );
            }
        }
        _ => {
            if strip.reason.is_none() && strip.watch_mode != WatchMode::PollingFallback {
                report.push(
                    "watch_strip.reason.required",
                    &base,
                    "non-live, non-polling watch strips must carry a reason",
                );
            }
            if strip.affected_guarantees.is_empty() {
                report.push(
                    "watch_strip.affected_guarantees",
                    &base,
                    "degraded watch strips must cite at least one affected guarantee",
                );
            }
        }
    }
    validate_actions(&strip.actions, WATCH_ACTION_IDS, &format!("{base}.actions"), report);
    validate_support_export(&strip.support_export, &format!("{base}.support_export"), report);
}

fn validate_ignore_drawer(
    drawer: &IgnoreResolutionDrawerRecord,
    report: &mut ReviewSurfaceValidationReport,
) {
    let base = format!("ignore_resolution_drawers.{}", drawer.drawer_id);
    if drawer.contributing_sources.is_empty()
        || drawer.scope_impact.trim().is_empty()
        || drawer.summary.trim().is_empty()
        || drawer.detail_lines.is_empty()
    {
        report.push(
            "ignore_drawer.shape",
            &base,
            "ignore drawers must carry sources, scope impact, summary, and detail lines",
        );
    }
    if matches!(drawer.visibility_class, IgnoreVisibilityClass::PolicyHidden)
        && drawer.override_allowed
    {
        report.push(
            "ignore_drawer.policy_hidden_override",
            &base,
            "policy-hidden rows may not claim a direct override path",
        );
    }
    validate_actions(&drawer.actions, IGNORE_ACTION_IDS, &format!("{base}.actions"), report);
    validate_support_export(&drawer.support_export, &format!("{base}.support_export"), report);
}

fn validate_external_change_review(
    review: &ExternalChangeReviewRecord,
    report: &mut ReviewSurfaceValidationReport,
) {
    let base = format!("external_change_reviews.{}", review.review_id);
    if review.diff_summary.trim().is_empty()
        || review.identity_note.trim().is_empty()
        || review.change_source.trim().is_empty()
        || review.detail_lines.is_empty()
    {
        report.push(
            "external_change_review.shape",
            &base,
            "external-change reviews must carry diff, identity, source, and detail text",
        );
    }
    if !review.silent_overwrite_forbidden {
        report.push(
            "external_change_review.silent_overwrite_forbidden",
            &base,
            "external-change reviews must forbid silent overwrite",
        );
    }
    if !review
        .actions
        .iter()
        .any(|action| action.action_id == "compare_to_disk")
    {
        report.push(
            "external_change_review.compare_action",
            &base,
            "external-change review must keep a compare-first action visible",
        );
    }
    validate_actions(
        &review.actions,
        EXTERNAL_CHANGE_ACTION_IDS,
        &format!("{base}.actions"),
        report,
    );
    validate_support_export(&review.support_export, &format!("{base}.support_export"), report);
}

fn validate_cross_root_review(
    review: &CrossRootMoveReviewRecord,
    report: &mut ReviewSurfaceValidationReport,
) {
    let base = format!("cross_root_move_reviews.{}", review.review_id);
    if review.metadata_consequences.is_empty()
        || review.summary.trim().is_empty()
        || review.case_sensitivity_note.trim().is_empty()
        || review.detail_lines.is_empty()
    {
        report.push(
            "cross_root_review.shape",
            &base,
            "cross-root reviews must carry consequences, summary, path notes, and detail lines",
        );
    }
    if review.source_root_class == review.target_root_class
        && review.boundary_crossing == BoundaryCrossingKind::RemoteAuthorityChange
    {
        report.push(
            "cross_root_review.boundary",
            &base,
            "remote authority change must not claim identical source and target root classes",
        );
    }
    if !review
        .actions
        .iter()
        .any(|action| action.action_id == "preview_plan")
    {
        report.push(
            "cross_root_review.preview_action",
            &base,
            "cross-root reviews must expose a preview-plan action before commit",
        );
    }
    validate_actions(
        &review.actions,
        CROSS_ROOT_ACTION_IDS,
        &format!("{base}.actions"),
        report,
    );
    validate_support_export(&review.support_export, &format!("{base}.support_export"), report);
}
