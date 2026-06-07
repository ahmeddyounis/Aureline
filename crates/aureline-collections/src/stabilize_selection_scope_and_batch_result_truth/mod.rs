//! Stable selection-scope and batch-result truth for dense collections.
//!
//! This module defines the portable selection-scope object and batch review
//! packet used by dense collection surfaces before they mutate, export, or
//! delegate work. The contract preserves stable item identity, reviewed query
//! basis, hidden-member counts, stale snapshots, and mixed per-item outcomes
//! across UI, CLI/headless, export, accessibility, and support lanes.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SelectionScopePacket`].
pub const SELECTION_SCOPE_PACKET_RECORD_KIND: &str = "selection_scope_batch_truth_packet";

/// Stable record-kind tag for [`SelectionScopeSupportExport`].
pub const SELECTION_SCOPE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "selection_scope_batch_truth_support_export";

/// Integer schema version for selection-scope packets.
pub const SELECTION_SCOPE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative schema path.
pub const SELECTION_SCOPE_SCHEMA_REF: &str = "schemas/collections/selection-scope.schema.json";

/// Repo-relative reviewer doc path.
pub const SELECTION_SCOPE_DOC_REF: &str =
    "docs/m4/stabilize-selection-scope-and-batch-result-truth.md";

/// Repo-relative artifact narrative path.
pub const SELECTION_SCOPE_ARTIFACT_DOC_REF: &str =
    "artifacts/collections/m4/stabilize-selection-scope-and-batch-result-truth.md";

/// Repo-relative fixture corpus path.
pub const SELECTION_SCOPE_FIXTURE_DIR: &str =
    "fixtures/collections/m4/stabilize-selection-scope-and-batch-result-truth";

/// Repo-relative packet artifact path.
pub const SELECTION_SCOPE_PACKET_ARTIFACT_REF: &str =
    "artifacts/collections/m4/stabilize-selection-scope-and-batch-result-truth/selection_scope_packet.json";

const SEEDED_PACKET_ID: &str = "collections:selection-scope-batch-truth:stable";
const SEEDED_GENERATED_AT: &str = "2026-06-07T00:00:00Z";

/// Selection scope class allowed on stable dense collection surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopeClass {
    /// Action applies only to the current item.
    CurrentItemOnly,
    /// Action applies to a range in visible traversal order.
    VisibleRange,
    /// Action applies to materialized client rows.
    LoadedSet,
    /// Action applies to the reviewed query snapshot.
    AllMatchingQuery,
    /// Action applies to an explicit stable identity set.
    ExplicitCustomSet,
}

impl SelectionScopeClass {
    /// Stable token used in schemas, fixtures, CLI output, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentItemOnly => "current_item_only",
            Self::VisibleRange => "visible_range",
            Self::LoadedSet => "loaded_set",
            Self::AllMatchingQuery => "all_matching_query",
            Self::ExplicitCustomSet => "explicit_custom_set",
        }
    }
}

/// Surface family that must preserve selection and batch truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopeSurfaceFamily {
    /// Search result surfaces.
    Search,
    /// Provider-backed review or admin queue surfaces.
    ProviderReviewAdmin,
    /// Package, test, diagnostics, or data-grid surfaces.
    PackageTestDataGrid,
}

impl SelectionScopeSurfaceFamily {
    /// Every surface family required before the stable claim can publish.
    pub const fn required() -> [Self; 3] {
        [
            Self::Search,
            Self::ProviderReviewAdmin,
            Self::PackageTestDataGrid,
        ]
    }

    /// Stable token used in packet fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::ProviderReviewAdmin => "provider_review_admin",
            Self::PackageTestDataGrid => "package_test_data_grid",
        }
    }
}

/// Consumer projection that must carry the same selection-scope truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopeConsumerProjection {
    /// Desktop UI projection.
    DesktopUi,
    /// CLI or headless output projection.
    CliHeadless,
    /// Export packet projection.
    ExportPacket,
    /// Support capture projection.
    SupportCapture,
    /// Keyboard and screen-reader projection.
    AccessibilityTree,
}

impl SelectionScopeConsumerProjection {
    /// Every projection required before the stable claim can publish.
    pub const fn required() -> [Self; 5] {
        [
            Self::DesktopUi,
            Self::CliHeadless,
            Self::ExportPacket,
            Self::SupportCapture,
            Self::AccessibilityTree,
        ]
    }

    /// Stable token used in packet fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::ExportPacket => "export_packet",
            Self::SupportCapture => "support_capture",
            Self::AccessibilityTree => "accessibility_tree",
        }
    }
}

/// Execution origin for a selection or batch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOriginClass {
    /// Client owns both membership and execution.
    LocalClient,
    /// Provider owns execution or authoritative membership.
    ProviderAuthoritative,
    /// Client reviews membership and provider completes execution.
    MixedClientProvider,
}

impl ExecutionOriginClass {
    /// Stable token used in packets and review sheets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalClient => "local_client",
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::MixedClientProvider => "mixed_client_provider",
        }
    }
}

/// Dataset posture that affects how selection scope can be expanded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetPosture {
    /// Client has a complete exact dataset.
    ClientComplete,
    /// Client has a paged or windowed subset.
    PagedOrWindowed,
    /// Streamed rows may arrive during active review.
    StreamingLive,
    /// Provider limits or samples results.
    ProviderLimited,
    /// Selection is pinned to a captured snapshot.
    SnapshotBasis,
}

/// Privacy posture for review, export, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchPrivacyClass {
    /// Metadata-only output safe for default support export.
    MetadataSafeDefault,
    /// Operator-only labels or counts are present.
    OperatorOnlyRestricted,
    /// Internal support may inspect the packet under policy.
    InternalSupportRestricted,
}

/// Disposition of one item in the reviewed batch population.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchMemberDisposition {
    /// Item is included in the action.
    Included,
    /// Item is excluded before execution.
    Excluded,
    /// Item is blocked and cannot execute.
    Blocked,
    /// Item is hidden by policy or current filter.
    Hidden,
    /// Item is skipped as a known no-op.
    Skipped,
    /// Item came from the reviewed query basis rather than a visible row.
    QueryDerived,
}

/// Per-item execution result after a batch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchItemOutcome {
    /// Item succeeded.
    Succeeded,
    /// Item failed and remains retryable or reviewable.
    Failed,
    /// Item was skipped as a no-op.
    Skipped,
    /// Item was blocked before mutation.
    Blocked,
}

/// Count value with stable term and exactness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountTruth {
    /// Stable count term.
    pub term: String,
    /// True when the value is exact for the stated term.
    pub exact: bool,
    /// Non-negative count value.
    pub value: u64,
}

impl CountTruth {
    /// Builds a count truth value.
    pub fn new(term: impl Into<String>, exact: bool, value: u64) -> Self {
        Self {
            term: term.into(),
            exact,
            value,
        }
    }
}

/// Stable identity for one selected or reviewed item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSelectionItemRef {
    /// Stable item identity used across sort, filter, pagination, and virtualization.
    pub stable_item_id: String,
    /// Source record, provider row, test id, package id, or data-grid row ref.
    pub source_ref: String,
    /// Redaction-aware label that never carries raw row bodies or query text.
    pub review_label: String,
}

/// Scope references that define the collection basis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBasisRefs {
    /// Query session that produced the collection.
    pub query_session_id_ref: String,
    /// Query snapshot used for all-matching or provider-backed scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_snapshot_id_ref: Option<String>,
    /// Workset, workspace, provider queue, or dataset scope.
    pub scope_ref: String,
    /// Optional named workset ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_ref: Option<String>,
}

/// Range semantics for tree and virtualized collection selections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RangeTraversalContract {
    /// True when range selection uses visible traversal order.
    pub visible_traversal_order: bool,
    /// True when collapsed descendants are excluded by default.
    pub collapsed_descendants_excluded_by_default: bool,
    /// True only when the operator explicitly chose a subtree-scoped action.
    pub subtree_scope_requires_explicit_action: bool,
}

impl RangeTraversalContract {
    /// Returns the stable visible-order contract.
    pub const fn visible_order() -> Self {
        Self {
            visible_traversal_order: true,
            collapsed_descendants_excluded_by_default: true,
            subtree_scope_requires_explicit_action: true,
        }
    }

    fn valid(&self) -> bool {
        self.visible_traversal_order
            && self.collapsed_descendants_excluded_by_default
            && self.subtree_scope_requires_explicit_action
    }
}

/// Stable product object describing one selection scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionScopeObject {
    /// Stable selection id.
    pub selection_id: String,
    /// Surface family that owns the selection.
    pub surface_family: SelectionScopeSurfaceFamily,
    /// Scope class.
    pub scope_class: SelectionScopeClass,
    /// Dataset posture at the time the selection was reviewed.
    pub dataset_posture: DatasetPosture,
    /// Query, session, scope, and workset refs backing this selection.
    pub basis_refs: SelectionBasisRefs,
    /// Stable item refs explicitly selected or sampled for review.
    pub selected_items: Vec<StableSelectionItemRef>,
    /// Range anchor item id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_item_id_ref: Option<String>,
    /// Selected count for the stated scope.
    pub selected_count: u64,
    /// Hidden or policy-limited members that remain part of the explanation path.
    pub hidden_member_count: u64,
    /// Blocked members in the selected scope.
    pub blocked_member_count: u64,
    /// Filtered-out selected members outside the current visible result set.
    pub filtered_out_member_count: u64,
    /// True when query or dataset identity changed materially after selection.
    pub stale_snapshot: bool,
    /// Local, provider, or mixed execution origin.
    pub execution_origin: ExecutionOriginClass,
    /// True when expansion to all matching query required a deliberate second step.
    pub select_all_expansion_was_explicit: bool,
    /// True when stable item identity, not row position, owns preservation.
    pub preserves_stable_item_identity: bool,
    /// Tree and range-selection rules.
    pub range_traversal: RangeTraversalContract,
    /// Screen-reader and keyboard-safe summary label.
    pub accessibility_summary: String,
}

impl SelectionScopeObject {
    fn valid_count_truth(&self) -> bool {
        self.selected_count >= self.selected_items.len() as u64
    }

    fn query_backed(&self) -> bool {
        matches!(self.scope_class, SelectionScopeClass::AllMatchingQuery)
            || matches!(
                self.execution_origin,
                ExecutionOriginClass::ProviderAuthoritative
                    | ExecutionOriginClass::MixedClientProvider
            )
    }
}

/// One member shown in a batch review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewMember {
    /// Stable item ref.
    pub item: StableSelectionItemRef,
    /// Member disposition in the review sheet.
    pub disposition: BatchMemberDisposition,
    /// Redaction-aware reason label.
    pub reason_label: String,
}

/// One per-item execution result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchItemResult {
    /// Stable item id.
    pub stable_item_id_ref: String,
    /// Per-item outcome.
    pub outcome: BatchItemOutcome,
    /// Redaction-aware outcome label.
    pub outcome_label: String,
    /// Retry, rollback, or follow-up action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_up_action_ref: Option<String>,
}

/// Review sheet and result truth for one batch action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewTruth {
    /// Stable batch review id.
    pub batch_review_id: String,
    /// Selection object this batch reviews.
    pub selection_id_ref: String,
    /// Action id.
    pub action_id: String,
    /// Action label.
    pub action_label: String,
    /// Execution origin.
    pub execution_origin: ExecutionOriginClass,
    /// Privacy class.
    pub privacy_class: BatchPrivacyClass,
    /// Included member count.
    pub included_count: u64,
    /// Excluded member count.
    pub excluded_count: u64,
    /// Blocked member count.
    pub blocked_count: u64,
    /// Hidden member count.
    pub hidden_count: u64,
    /// Skipped member count.
    pub skipped_count: u64,
    /// Query-derived member count.
    pub query_derived_count: u64,
    /// Count truth used by UI, CLI, export, and support lanes.
    pub counts: Vec<CountTruth>,
    /// Member rows rendered in the batch review sheet.
    pub members: Vec<BatchReviewMember>,
    /// Rollback or retry guidance.
    pub rollback_retry_guidance: String,
    /// True when destructive, provider-owned, remote, or export-bearing action requires review.
    pub review_required: bool,
    /// True when focus returns to the originating selection owner after close.
    pub focus_return_preserved: bool,
    /// Per-item result rows preserving mixed outcomes.
    pub item_results: Vec<BatchItemResult>,
    /// Summary label for mixed results.
    pub mixed_outcome_summary: String,
}

impl BatchReviewTruth {
    fn count_for(&self, disposition: BatchMemberDisposition) -> u64 {
        self.members
            .iter()
            .filter(|member| member.disposition == disposition)
            .count() as u64
    }

    fn has_mixed_outcome_truth(&self) -> bool {
        let outcomes: BTreeSet<_> = self.item_results.iter().map(|item| item.outcome).collect();
        outcomes.len() >= 2 && !self.mixed_outcome_summary.trim().is_empty()
    }
}

/// Projection proof for one consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionScopeProjectionProof {
    /// Consumer projection.
    pub consumer_projection: SelectionScopeConsumerProjection,
    /// Stable projection id.
    pub projection_id: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// True when selected count and scope class are exposed.
    pub exposes_count_and_scope: bool,
    /// True when hidden-member count is exposed.
    pub exposes_hidden_member_count: bool,
    /// True when stale snapshot state is exposed.
    pub exposes_stale_snapshot_state: bool,
    /// True when visible, loaded, and all-matching scope distinctions survive.
    pub exposes_visible_loaded_matching_scope: bool,
    /// True when mixed result summaries preserve per-item truth.
    pub preserves_mixed_outcome_results: bool,
    /// True when export/support preserve the original packet instead of recomputing counts.
    pub preserves_packet_without_recomputed_counts: bool,
}

impl SelectionScopeProjectionProof {
    fn preserves_packet(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && !self.projection_id.trim().is_empty()
            && self.exposes_count_and_scope
            && self.exposes_hidden_member_count
            && self.exposes_stale_snapshot_state
            && self.exposes_visible_loaded_matching_scope
            && self.preserves_mixed_outcome_results
            && self.preserves_packet_without_recomputed_counts
    }
}

/// Promotion state for the stable selection-scope packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopePromotionState {
    /// Packet certifies stable publication.
    Stable,
    /// Packet has warnings and must stay narrowed.
    NarrowedBelowStable,
    /// Packet has blocker findings.
    BlocksStable,
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopeFindingSeverity {
    /// Warning that narrows stable confidence.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed finding vocabulary for the selection-scope packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopeFindingKind {
    /// Packet has wrong record kind.
    WrongRecordKind,
    /// Packet has wrong schema version.
    WrongSchemaVersion,
    /// Required identity is missing.
    MissingIdentity,
    /// A required surface family is missing.
    MissingRequiredSurface,
    /// A required consumer projection is missing.
    MissingRequiredProjection,
    /// Selection count does not match or exceed explicit item refs.
    SelectionCountCollapsed,
    /// Query-backed selection lacks a query snapshot.
    QueryBackedScopeLacksSnapshot,
    /// Select-all expansion to all matching was not explicit.
    AllMatchingExpansionImplicit,
    /// Stable identity preservation is absent.
    StableIdentityNotPreserved,
    /// Hidden-member count is not preserved in the selection or review path.
    HiddenMemberTruthDropped,
    /// Stale query or dataset basis is not disclosed.
    StaleSnapshotUndisclosed,
    /// Tree range selection can silently include collapsed descendants.
    TreeRangeCanIncludeCollapsedDescendants,
    /// Protected batch review is missing.
    BatchReviewRequiredButMissing,
    /// Batch review count fields disagree with member dispositions.
    BatchReviewCountsDisagree,
    /// Mixed outcomes collapsed into generic result truth.
    MixedOutcomeTruthCollapsed,
    /// Consumer projection drops required truth.
    ProjectionDropsTruth,
    /// Promotion state disagrees with validation.
    PromotionStateMismatch,
}

impl SelectionScopeFindingKind {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingRequiredSurface => "missing_required_surface",
            Self::MissingRequiredProjection => "missing_required_projection",
            Self::SelectionCountCollapsed => "selection_count_collapsed",
            Self::QueryBackedScopeLacksSnapshot => "query_backed_scope_lacks_snapshot",
            Self::AllMatchingExpansionImplicit => "all_matching_expansion_implicit",
            Self::StableIdentityNotPreserved => "stable_identity_not_preserved",
            Self::HiddenMemberTruthDropped => "hidden_member_truth_dropped",
            Self::StaleSnapshotUndisclosed => "stale_snapshot_undisclosed",
            Self::TreeRangeCanIncludeCollapsedDescendants => {
                "tree_range_can_include_collapsed_descendants"
            }
            Self::BatchReviewRequiredButMissing => "batch_review_required_but_missing",
            Self::BatchReviewCountsDisagree => "batch_review_counts_disagree",
            Self::MixedOutcomeTruthCollapsed => "mixed_outcome_truth_collapsed",
            Self::ProjectionDropsTruth => "projection_drops_truth",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the stable validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionScopeValidationFinding {
    /// Finding kind.
    pub finding_kind: SelectionScopeFindingKind,
    /// Finding severity.
    pub severity: SelectionScopeFindingSeverity,
    /// Reviewable finding message.
    pub message: String,
}

impl SelectionScopeValidationFinding {
    fn blocker(finding_kind: SelectionScopeFindingKind, message: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: SelectionScopeFindingSeverity::Blocker,
            message: message.into(),
        }
    }
}

/// Stable selection-scope and batch-result packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionScopePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Source schema, docs, artifacts, and fixture refs.
    pub source_contract_refs: Vec<String>,
    /// Selection scope objects proving dense collection behavior.
    pub selection_scopes: Vec<SelectionScopeObject>,
    /// Batch review sheets and mixed-result truth.
    pub batch_reviews: Vec<BatchReviewTruth>,
    /// Consumer projections preserving truth.
    pub consumer_projections: Vec<SelectionScopeProjectionProof>,
    /// Derived promotion state.
    pub promotion_state: SelectionScopePromotionState,
    /// Derived validation findings.
    pub validation_findings: Vec<SelectionScopeValidationFinding>,
}

impl SelectionScopePacket {
    /// Materializes a stable packet and records validation findings.
    pub fn materialize(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        selection_scopes: Vec<SelectionScopeObject>,
        batch_reviews: Vec<BatchReviewTruth>,
        consumer_projections: Vec<SelectionScopeProjectionProof>,
    ) -> Self {
        let mut packet = Self {
            record_kind: SELECTION_SCOPE_PACKET_RECORD_KIND.to_owned(),
            schema_version: SELECTION_SCOPE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            source_contract_refs: vec![
                SELECTION_SCOPE_SCHEMA_REF.to_owned(),
                SELECTION_SCOPE_DOC_REF.to_owned(),
                SELECTION_SCOPE_ARTIFACT_DOC_REF.to_owned(),
                SELECTION_SCOPE_FIXTURE_DIR.to_owned(),
            ],
            selection_scopes,
            batch_reviews,
            consumer_projections,
            promotion_state: SelectionScopePromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates packet invariants, including serialized metadata fields.
    pub fn validate(&self) -> Vec<SelectionScopeValidationFinding> {
        self.derived_findings(true)
    }

    /// Builds a support export that preserves the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SelectionScopeSupportExport {
        SelectionScopeSupportExport {
            record_kind: SELECTION_SCOPE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SELECTION_SCOPE_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<SelectionScopeValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != SELECTION_SCOPE_PACKET_RECORD_KIND {
            findings.push(SelectionScopeValidationFinding::blocker(
                SelectionScopeFindingKind::WrongRecordKind,
                "packet record kind does not match the stable contract",
            ));
        }
        if include_record_fields && self.schema_version != SELECTION_SCOPE_SCHEMA_VERSION {
            findings.push(SelectionScopeValidationFinding::blocker(
                SelectionScopeFindingKind::WrongSchemaVersion,
                "packet schema version does not match the stable contract",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(SelectionScopeValidationFinding::blocker(
                SelectionScopeFindingKind::MissingIdentity,
                "packet id and generation timestamp are required",
            ));
        }

        let present_surfaces: BTreeSet<_> = self
            .selection_scopes
            .iter()
            .map(|scope| scope.surface_family)
            .collect();
        for required in SelectionScopeSurfaceFamily::required() {
            if !present_surfaces.contains(&required) {
                findings.push(SelectionScopeValidationFinding::blocker(
                    SelectionScopeFindingKind::MissingRequiredSurface,
                    format!("missing required surface {}", required.as_str()),
                ));
            }
        }

        for selection_scope in &self.selection_scopes {
            validate_scope(selection_scope, &mut findings);
            let reviews: Vec<_> = self
                .batch_reviews
                .iter()
                .filter(|review| review.selection_id_ref == selection_scope.selection_id)
                .collect();
            let requires_review = selection_scope.query_backed()
                || selection_scope.hidden_member_count > 0
                || selection_scope.blocked_member_count > 0
                || matches!(
                    selection_scope.scope_class,
                    SelectionScopeClass::AllMatchingQuery | SelectionScopeClass::LoadedSet
                );
            if requires_review && reviews.is_empty() {
                findings.push(SelectionScopeValidationFinding::blocker(
                    SelectionScopeFindingKind::BatchReviewRequiredButMissing,
                    format!(
                        "selection {} requires a batch-review sheet",
                        selection_scope.selection_id
                    ),
                ));
            }
            for review in reviews {
                validate_review(selection_scope, review, &mut findings);
            }
        }

        for required in SelectionScopeConsumerProjection::required() {
            if !self.consumer_projections.iter().any(|projection| {
                projection.consumer_projection == required
                    && projection.preserves_packet(&self.packet_id)
            }) {
                findings.push(SelectionScopeValidationFinding::blocker(
                    SelectionScopeFindingKind::MissingRequiredProjection,
                    format!("missing preserved {} projection", required.as_str()),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_packet(&self.packet_id) {
                findings.push(SelectionScopeValidationFinding::blocker(
                    SelectionScopeFindingKind::ProjectionDropsTruth,
                    format!(
                        "projection {} drops selection truth",
                        projection.projection_id
                    ),
                ));
            }
        }

        if include_record_fields {
            let derived = promotion_state_for(&findings);
            if self.promotion_state != derived {
                findings.push(SelectionScopeValidationFinding::blocker(
                    SelectionScopeFindingKind::PromotionStateMismatch,
                    "stored promotion state does not match validator output",
                ));
            }
        }

        findings
    }
}

fn validate_scope(
    scope: &SelectionScopeObject,
    findings: &mut Vec<SelectionScopeValidationFinding>,
) {
    if scope.selection_id.trim().is_empty()
        || scope.basis_refs.query_session_id_ref.trim().is_empty()
        || scope.basis_refs.scope_ref.trim().is_empty()
        || scope.accessibility_summary.trim().is_empty()
    {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::MissingIdentity,
            format!(
                "selection {} is missing required identity",
                scope.selection_id
            ),
        ));
    }
    if !scope.valid_count_truth() {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::SelectionCountCollapsed,
            format!(
                "selection {} count is lower than item refs",
                scope.selection_id
            ),
        ));
    }
    if scope.query_backed() && scope.basis_refs.query_snapshot_id_ref.is_none() {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::QueryBackedScopeLacksSnapshot,
            format!("selection {} lacks query snapshot", scope.selection_id),
        ));
    }
    if scope.scope_class == SelectionScopeClass::AllMatchingQuery
        && !scope.select_all_expansion_was_explicit
    {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::AllMatchingExpansionImplicit,
            format!(
                "selection {} expanded to all matching without explicit step",
                scope.selection_id
            ),
        ));
    }
    if !scope.preserves_stable_item_identity {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::StableIdentityNotPreserved,
            format!(
                "selection {} does not preserve stable identity",
                scope.selection_id
            ),
        ));
    }
    if scope.hidden_member_count > 0 && !scope.accessibility_summary.contains("hidden") {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::HiddenMemberTruthDropped,
            format!(
                "selection {} drops hidden-member summary",
                scope.selection_id
            ),
        ));
    }
    if matches!(
        scope.dataset_posture,
        DatasetPosture::StreamingLive
            | DatasetPosture::ProviderLimited
            | DatasetPosture::SnapshotBasis
    ) && !scope.stale_snapshot
        && scope.scope_class == SelectionScopeClass::AllMatchingQuery
    {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::StaleSnapshotUndisclosed,
            format!(
                "selection {} lacks stale snapshot disclosure",
                scope.selection_id
            ),
        ));
    }
    if !scope.range_traversal.valid() {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::TreeRangeCanIncludeCollapsedDescendants,
            format!(
                "selection {} has unsafe tree range semantics",
                scope.selection_id
            ),
        ));
    }
}

fn validate_review(
    scope: &SelectionScopeObject,
    review: &BatchReviewTruth,
    findings: &mut Vec<SelectionScopeValidationFinding>,
) {
    if review.review_required && review.members.is_empty() {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::BatchReviewRequiredButMissing,
            format!("batch review {} has no members", review.batch_review_id),
        ));
    }
    let counts_match = review.included_count == review.count_for(BatchMemberDisposition::Included)
        && review.excluded_count == review.count_for(BatchMemberDisposition::Excluded)
        && review.blocked_count == review.count_for(BatchMemberDisposition::Blocked)
        && review.hidden_count == review.count_for(BatchMemberDisposition::Hidden)
        && review.skipped_count == review.count_for(BatchMemberDisposition::Skipped)
        && review.query_derived_count == review.count_for(BatchMemberDisposition::QueryDerived);
    if !counts_match {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::BatchReviewCountsDisagree,
            format!(
                "batch review {} counts disagree with members",
                review.batch_review_id
            ),
        ));
    }
    if scope.hidden_member_count > 0 && review.hidden_count == 0 {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::HiddenMemberTruthDropped,
            format!(
                "batch review {} drops hidden members",
                review.batch_review_id
            ),
        ));
    }
    if !review.has_mixed_outcome_truth() {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::MixedOutcomeTruthCollapsed,
            format!(
                "batch review {} collapses mixed outcomes",
                review.batch_review_id
            ),
        ));
    }
    if !review.focus_return_preserved || review.rollback_retry_guidance.trim().is_empty() {
        findings.push(SelectionScopeValidationFinding::blocker(
            SelectionScopeFindingKind::ProjectionDropsTruth,
            format!(
                "batch review {} drops focus return or retry guidance",
                review.batch_review_id
            ),
        ));
    }
}

fn promotion_state_for(
    findings: &[SelectionScopeValidationFinding],
) -> SelectionScopePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == SelectionScopeFindingSeverity::Blocker)
    {
        SelectionScopePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == SelectionScopeFindingSeverity::Warning)
    {
        SelectionScopePromotionState::NarrowedBelowStable
    } else {
        SelectionScopePromotionState::Stable
    }
}

/// Support-export wrapper preserving the exact stable selection-scope packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionScopeSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact product packet.
    pub packet: SelectionScopePacket,
}

impl SelectionScopeSupportExport {
    /// True when support export preserves the exact packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SELECTION_SCOPE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SELECTION_SCOPE_SCHEMA_VERSION
            && self.packet_id_ref == self.packet.packet_id
            && self.raw_private_material_excluded
            && self.packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in packet artifact.
#[derive(Debug)]
pub enum SelectionScopeArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SelectionScopeValidationFinding>),
}

impl fmt::Display for SelectionScopeArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "selection-scope packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "selection-scope packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SelectionScopeArtifactError {}

/// Returns the checked-in stable selection-scope packet.
///
/// # Errors
///
/// Returns an artifact error if the packet cannot parse or fails validation.
pub fn current_selection_scope_packet() -> Result<SelectionScopePacket, SelectionScopeArtifactError>
{
    let packet: SelectionScopePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m4/stabilize-selection-scope-and-batch-result-truth/selection_scope_packet.json"
    )))
    .map_err(SelectionScopeArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SelectionScopeArtifactError::Validation(findings))
    }
}

/// Builds the seeded stable selection-scope packet.
pub fn seeded_selection_scope_packet() -> SelectionScopePacket {
    let selection_scopes = vec![
        search_visible_range_scope(),
        provider_all_matching_scope(),
        package_loaded_set_scope(),
    ];
    let batch_reviews = vec![
        search_batch_review(),
        provider_batch_review(),
        package_batch_review(),
    ];
    SelectionScopePacket::materialize(
        SEEDED_PACKET_ID,
        SEEDED_GENERATED_AT,
        selection_scopes,
        batch_reviews,
        SelectionScopeConsumerProjection::required()
            .into_iter()
            .map(|consumer_projection| SelectionScopeProjectionProof {
                consumer_projection,
                projection_id: format!(
                    "collections:selection-scope:projection:{}",
                    consumer_projection.as_str()
                ),
                packet_id_ref: SEEDED_PACKET_ID.to_owned(),
                exposes_count_and_scope: true,
                exposes_hidden_member_count: true,
                exposes_stale_snapshot_state: true,
                exposes_visible_loaded_matching_scope: true,
                preserves_mixed_outcome_results: true,
                preserves_packet_without_recomputed_counts: true,
            })
            .collect(),
    )
}

fn search_visible_range_scope() -> SelectionScopeObject {
    SelectionScopeObject {
        selection_id: "selection:search:visible-errors".to_owned(),
        surface_family: SelectionScopeSurfaceFamily::Search,
        scope_class: SelectionScopeClass::VisibleRange,
        dataset_posture: DatasetPosture::PagedOrWindowed,
        basis_refs: SelectionBasisRefs {
            query_session_id_ref: "query-session:search:visible-errors".to_owned(),
            query_snapshot_id_ref: None,
            scope_ref: "scope:workspace-current-workset".to_owned(),
            workset_ref: Some("workset:current-reviewed-slice".to_owned()),
        },
        selected_items: vec![
            item(
                "search:item:visible:1",
                "source:search:1",
                "Search result 1",
            ),
            item(
                "search:item:visible:2",
                "source:search:2",
                "Search result 2",
            ),
            item(
                "search:item:hidden:3",
                "source:search:3",
                "Search result hidden",
            ),
        ],
        anchor_item_id_ref: Some("search:item:visible:1".to_owned()),
        selected_count: 3,
        hidden_member_count: 1,
        blocked_member_count: 1,
        filtered_out_member_count: 1,
        stale_snapshot: false,
        execution_origin: ExecutionOriginClass::LocalClient,
        select_all_expansion_was_explicit: false,
        preserves_stable_item_identity: true,
        range_traversal: RangeTraversalContract::visible_order(),
        accessibility_summary:
            "3 selected in visible range; 1 hidden; 1 blocked; visible rows only.".to_owned(),
    }
}

fn provider_all_matching_scope() -> SelectionScopeObject {
    SelectionScopeObject {
        selection_id: "selection:provider-review:all-matching".to_owned(),
        surface_family: SelectionScopeSurfaceFamily::ProviderReviewAdmin,
        scope_class: SelectionScopeClass::AllMatchingQuery,
        dataset_posture: DatasetPosture::ProviderLimited,
        basis_refs: SelectionBasisRefs {
            query_session_id_ref: "query-session:provider-review:pending".to_owned(),
            query_snapshot_id_ref: Some("query-snapshot:provider-review:pending:v7".to_owned()),
            scope_ref: "provider-scope:review-queue".to_owned(),
            workset_ref: None,
        },
        selected_items: vec![
            item("provider:item:1", "provider-row:1", "Provider row 1"),
            item("provider:item:2", "provider-row:2", "Provider row 2"),
            item(
                "provider:item:hidden",
                "provider-row:hidden",
                "Provider hidden row",
            ),
        ],
        anchor_item_id_ref: None,
        selected_count: 238,
        hidden_member_count: 19,
        blocked_member_count: 6,
        filtered_out_member_count: 23,
        stale_snapshot: true,
        execution_origin: ExecutionOriginClass::ProviderAuthoritative,
        select_all_expansion_was_explicit: true,
        preserves_stable_item_identity: true,
        range_traversal: RangeTraversalContract::visible_order(),
        accessibility_summary:
            "238 matching selected from provider query; 19 hidden; stale snapshot requires review."
                .to_owned(),
    }
}

fn package_loaded_set_scope() -> SelectionScopeObject {
    SelectionScopeObject {
        selection_id: "selection:package-test:loaded-set".to_owned(),
        surface_family: SelectionScopeSurfaceFamily::PackageTestDataGrid,
        scope_class: SelectionScopeClass::LoadedSet,
        dataset_posture: DatasetPosture::StreamingLive,
        basis_refs: SelectionBasisRefs {
            query_session_id_ref: "query-session:package-test:failing".to_owned(),
            query_snapshot_id_ref: Some("query-snapshot:package-test:loaded:42".to_owned()),
            scope_ref: "scope:test-run:latest".to_owned(),
            workset_ref: Some("workset:package-selection".to_owned()),
        },
        selected_items: vec![
            item("test:item:1", "test-row:1", "Test row 1"),
            item("test:item:2", "test-row:2", "Test row 2"),
            item("test:item:3", "test-row:3", "Test row 3"),
            item("test:item:hidden", "test-row:hidden", "Hidden test row"),
        ],
        anchor_item_id_ref: Some("test:item:1".to_owned()),
        selected_count: 120,
        hidden_member_count: 4,
        blocked_member_count: 2,
        filtered_out_member_count: 8,
        stale_snapshot: false,
        execution_origin: ExecutionOriginClass::MixedClientProvider,
        select_all_expansion_was_explicit: false,
        preserves_stable_item_identity: true,
        range_traversal: RangeTraversalContract::visible_order(),
        accessibility_summary: "120 loaded selected; 4 hidden; mixed local and provider execution."
            .to_owned(),
    }
}

fn search_batch_review() -> BatchReviewTruth {
    batch_review(
        "batch-review:search:export-visible",
        "selection:search:visible-errors",
        "search.export_visible_selection",
        "Export visible selection",
        ExecutionOriginClass::LocalClient,
        BatchPrivacyClass::MetadataSafeDefault,
        vec![
            member("search:item:visible:1", BatchMemberDisposition::Included),
            member("search:item:visible:2", BatchMemberDisposition::Included),
            member("search:item:hidden:3", BatchMemberDisposition::Hidden),
            member("search:item:blocked", BatchMemberDisposition::Blocked),
            member("search:item:filtered", BatchMemberDisposition::Excluded),
            member("search:item:skip", BatchMemberDisposition::Skipped),
        ],
        vec![
            result("search:item:visible:1", BatchItemOutcome::Succeeded),
            result("search:item:visible:2", BatchItemOutcome::Failed),
            result("search:item:hidden:3", BatchItemOutcome::Skipped),
            result("search:item:blocked", BatchItemOutcome::Blocked),
        ],
    )
}

fn provider_batch_review() -> BatchReviewTruth {
    batch_review(
        "batch-review:provider:approve-matching",
        "selection:provider-review:all-matching",
        "provider.approve_matching_query",
        "Approve matching provider queue",
        ExecutionOriginClass::ProviderAuthoritative,
        BatchPrivacyClass::OperatorOnlyRestricted,
        vec![
            member("provider:item:1", BatchMemberDisposition::Included),
            member("provider:item:2", BatchMemberDisposition::Included),
            member("provider:item:hidden", BatchMemberDisposition::Hidden),
            member("provider:item:blocked", BatchMemberDisposition::Blocked),
            member("provider:item:excluded", BatchMemberDisposition::Excluded),
            member("provider:item:skipped", BatchMemberDisposition::Skipped),
            member(
                "provider:item:query-derived",
                BatchMemberDisposition::QueryDerived,
            ),
        ],
        vec![
            result("provider:item:1", BatchItemOutcome::Succeeded),
            result("provider:item:2", BatchItemOutcome::Failed),
            result("provider:item:hidden", BatchItemOutcome::Skipped),
            result("provider:item:blocked", BatchItemOutcome::Blocked),
            result("provider:item:query-derived", BatchItemOutcome::Succeeded),
        ],
    )
}

fn package_batch_review() -> BatchReviewTruth {
    batch_review(
        "batch-review:package-test:rerun-loaded",
        "selection:package-test:loaded-set",
        "package_test.rerun_loaded_failures",
        "Rerun loaded failures",
        ExecutionOriginClass::MixedClientProvider,
        BatchPrivacyClass::MetadataSafeDefault,
        vec![
            member("test:item:1", BatchMemberDisposition::Included),
            member("test:item:2", BatchMemberDisposition::Included),
            member("test:item:3", BatchMemberDisposition::Included),
            member("test:item:hidden", BatchMemberDisposition::Hidden),
            member("test:item:blocked", BatchMemberDisposition::Blocked),
            member("test:item:excluded", BatchMemberDisposition::Excluded),
            member("test:item:skipped", BatchMemberDisposition::Skipped),
        ],
        vec![
            result("test:item:1", BatchItemOutcome::Succeeded),
            result("test:item:2", BatchItemOutcome::Failed),
            result("test:item:3", BatchItemOutcome::Succeeded),
            result("test:item:hidden", BatchItemOutcome::Skipped),
            result("test:item:blocked", BatchItemOutcome::Blocked),
        ],
    )
}

fn item(id: &str, source_ref: &str, review_label: &str) -> StableSelectionItemRef {
    StableSelectionItemRef {
        stable_item_id: id.to_owned(),
        source_ref: source_ref.to_owned(),
        review_label: review_label.to_owned(),
    }
}

fn member(id: &str, disposition: BatchMemberDisposition) -> BatchReviewMember {
    BatchReviewMember {
        item: item(id, &format!("source:{id}"), id),
        disposition,
        reason_label: format!(
            "{} member preserved by stable item identity.",
            disposition_token(disposition)
        ),
    }
}

fn result(id: &str, outcome: BatchItemOutcome) -> BatchItemResult {
    BatchItemResult {
        stable_item_id_ref: id.to_owned(),
        outcome,
        outcome_label: format!("{} result for {id}.", outcome_token(outcome)),
        follow_up_action_ref: matches!(outcome, BatchItemOutcome::Failed)
            .then(|| format!("retry:{id}")),
    }
}

#[allow(clippy::too_many_arguments)]
fn batch_review(
    batch_review_id: &str,
    selection_id_ref: &str,
    action_id: &str,
    action_label: &str,
    execution_origin: ExecutionOriginClass,
    privacy_class: BatchPrivacyClass,
    members: Vec<BatchReviewMember>,
    item_results: Vec<BatchItemResult>,
) -> BatchReviewTruth {
    let included_count = count(&members, BatchMemberDisposition::Included);
    let excluded_count = count(&members, BatchMemberDisposition::Excluded);
    let blocked_count = count(&members, BatchMemberDisposition::Blocked);
    let hidden_count = count(&members, BatchMemberDisposition::Hidden);
    let skipped_count = count(&members, BatchMemberDisposition::Skipped);
    let query_derived_count = count(&members, BatchMemberDisposition::QueryDerived);
    BatchReviewTruth {
        batch_review_id: batch_review_id.to_owned(),
        selection_id_ref: selection_id_ref.to_owned(),
        action_id: action_id.to_owned(),
        action_label: action_label.to_owned(),
        execution_origin,
        privacy_class,
        included_count,
        excluded_count,
        blocked_count,
        hidden_count,
        skipped_count,
        query_derived_count,
        counts: vec![
            CountTruth::new("included", true, included_count),
            CountTruth::new("excluded", true, excluded_count),
            CountTruth::new("blocked", true, blocked_count),
            CountTruth::new("hidden", true, hidden_count),
            CountTruth::new("skipped", true, skipped_count),
            CountTruth::new("query_derived", execution_origin == ExecutionOriginClass::LocalClient, query_derived_count),
        ],
        members,
        rollback_retry_guidance:
            "Retry failed members from the result list; blocked and hidden members remain reviewable before replay."
                .to_owned(),
        review_required: true,
        focus_return_preserved: true,
        item_results,
        mixed_outcome_summary:
            "Mixed result preserved per item with succeeded, failed, skipped, and blocked rows."
                .to_owned(),
    }
}

fn count(members: &[BatchReviewMember], disposition: BatchMemberDisposition) -> u64 {
    members
        .iter()
        .filter(|member| member.disposition == disposition)
        .count() as u64
}

fn disposition_token(disposition: BatchMemberDisposition) -> &'static str {
    match disposition {
        BatchMemberDisposition::Included => "included",
        BatchMemberDisposition::Excluded => "excluded",
        BatchMemberDisposition::Blocked => "blocked",
        BatchMemberDisposition::Hidden => "hidden",
        BatchMemberDisposition::Skipped => "skipped",
        BatchMemberDisposition::QueryDerived => "query-derived",
    }
}

fn outcome_token(outcome: BatchItemOutcome) -> &'static str {
    match outcome {
        BatchItemOutcome::Succeeded => "succeeded",
        BatchItemOutcome::Failed => "failed",
        BatchItemOutcome::Skipped => "skipped",
        BatchItemOutcome::Blocked => "blocked",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_is_stable() {
        let packet = seeded_selection_scope_packet();
        assert!(packet.validate().is_empty());
        assert_eq!(packet.promotion_state, SelectionScopePromotionState::Stable);
    }

    #[test]
    fn checked_in_packet_is_valid() {
        let packet =
            current_selection_scope_packet().expect("checked-in selection-scope packet validates");
        assert!(packet
            .support_export("export:test", "2026-06-07T00:00:00Z")
            .is_export_safe());
    }

    #[test]
    fn all_matching_without_explicit_expansion_blocks_stable() {
        let mut packet = seeded_selection_scope_packet();
        let scope = packet
            .selection_scopes
            .iter_mut()
            .find(|scope| scope.scope_class == SelectionScopeClass::AllMatchingQuery)
            .expect("seed includes all matching query");
        scope.select_all_expansion_was_explicit = false;
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == SelectionScopeFindingKind::AllMatchingExpansionImplicit
        }));
    }

    #[test]
    fn collapsed_tree_descendant_inclusion_blocks_stable() {
        let mut packet = seeded_selection_scope_packet();
        packet.selection_scopes[0]
            .range_traversal
            .collapsed_descendants_excluded_by_default = false;
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind
                == SelectionScopeFindingKind::TreeRangeCanIncludeCollapsedDescendants
        }));
    }

    #[test]
    fn generic_result_summary_blocks_stable() {
        let mut packet = seeded_selection_scope_packet();
        packet.batch_reviews[0].item_results =
            vec![result("search:item:visible:1", BatchItemOutcome::Succeeded)];
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == SelectionScopeFindingKind::MixedOutcomeTruthCollapsed
        }));
    }
}
