//! Reactive-state and materialized-view invalidation beta projection.
//!
//! This module is the canonical loader, validator, and reporter for the
//! beta cross-surface epoch-parity contract. It binds each materialized
//! view to:
//!
//! - one closed [`MaterializedViewClass`] (`ephemeral_projection`,
//!   `durable_local_materialization`, `exportable_snapshot`,
//!   `managed_replicated_view`) so support and export pipelines know
//!   whether the view is local, exportable, or replicated;
//! - one closed [`AuthorityLabel`] tying the view to its producing
//!   authority (`workspace_vfs`, `buffer_editor`, `derived_knowledge`,
//!   `execution`, `policy_entitlement`, `provider_overlay`);
//! - one monotonic `authority_epoch`;
//! - one [`SubscriberEpoch`] per required consumer surface (`shell`,
//!   `search`, `graph`, `ai`, `review`, `support`) recording the epoch
//!   each surface has observed, its freshness label, and the
//!   invalidation cause that produced its latest frame;
//! - one closed [`EpochParityState`] (`aligned`, `drift_detected`,
//!   `awaiting_resync`, `terminal_unavailable`) that the evaluator
//!   re-derives from the subscriber epochs rather than trusting prose;
//! - one [`SupportExportProjection`] declaring the export posture and
//!   the metadata-safe baseline so support bundles can preserve epoch
//!   state without re-running producers; and
//! - one closed [`DowngradeLabel`] drawn from the reactive-views
//!   vocabulary so a failing row downgrades without inventing new
//!   tokens.
//!
//! Bound to the boundary schema at
//! [`/schemas/state/materialized_view.schema.json`](../../../../schemas/state/materialized_view.schema.json),
//! the reviewer doc at
//! [`/docs/state/m3/reactive_state_beta.md`](../../../../docs/state/m3/reactive_state_beta.md),
//! and the baseline report at
//! [`/artifacts/support/m3/reactive_state_beta_report.md`](../../../../artifacts/support/m3/reactive_state_beta_report.md).

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a reactive-view case record.
pub const REACTIVE_VIEW_CASE_RECORD_KIND: &str = "materialized_view_case_record";

/// Stable record-kind tag for the reactive-view report record.
pub const REACTIVE_VIEW_REPORT_RECORD_KIND: &str = "materialized_view_report_record";

/// Frozen schema version for the reactive-view records.
pub const REACTIVE_VIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REACTIVE_VIEW_SCHEMA_REF: &str = "schemas/state/materialized_view.schema.json";

/// Repo-relative path of the reviewer doc.
pub const REACTIVE_VIEW_DOC_REF: &str = "docs/state/m3/reactive_state_beta.md";

/// Repo-relative path of the baseline report.
pub const REACTIVE_VIEW_REPORT_REF: &str = "artifacts/support/m3/reactive_state_beta_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const REACTIVE_VIEW_CORPUS_DIR: &str = "fixtures/state/reactive_views_beta";

/// Repo-relative path of the protected corpus manifest.
pub const REACTIVE_VIEW_CORPUS_MANIFEST_REF: &str =
    "fixtures/state/reactive_views_beta/manifest.yaml";

/// Closed materialized-view-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterializedViewClass {
    /// In-memory projection rebuilt on every cold start.
    EphemeralProjection,
    /// Persisted on the local device; survives restarts but is not
    /// exported by default.
    DurableLocalMaterialization,
    /// Persisted locally and bundled into support / export packets.
    ExportableSnapshot,
    /// Replicated by a managed service; the local copy may lag the
    /// remote authority epoch.
    ManagedReplicatedView,
}

impl MaterializedViewClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralProjection => "ephemeral_projection",
            Self::DurableLocalMaterialization => "durable_local_materialization",
            Self::ExportableSnapshot => "exportable_snapshot",
            Self::ManagedReplicatedView => "managed_replicated_view",
        }
    }
}

/// Closed list of materialized-view classes the corpus must cover.
pub const REQUIRED_VIEW_CLASSES: [MaterializedViewClass; 4] = [
    MaterializedViewClass::EphemeralProjection,
    MaterializedViewClass::DurableLocalMaterialization,
    MaterializedViewClass::ExportableSnapshot,
    MaterializedViewClass::ManagedReplicatedView,
];

/// Closed authority-label vocabulary. Mirrors the
/// `AuthorityClass` enum in [`crate::envelope`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLabel {
    WorkspaceVfs,
    BufferEditor,
    DerivedKnowledge,
    Execution,
    PolicyEntitlement,
    ProviderOverlay,
}

impl AuthorityLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceVfs => "workspace_vfs",
            Self::BufferEditor => "buffer_editor",
            Self::DerivedKnowledge => "derived_knowledge",
            Self::Execution => "execution",
            Self::PolicyEntitlement => "policy_entitlement",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Closed consumer-surface vocabulary. Every reactive view must
/// declare one subscriber entry per surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    Shell,
    Search,
    Graph,
    Ai,
    Review,
    Support,
}

impl SurfaceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Ai => "ai",
            Self::Review => "review",
            Self::Support => "support",
        }
    }
}

/// Closed list of consumer surfaces every reactive view must wire.
pub const REQUIRED_SURFACE_KINDS: [SurfaceKind; 6] = [
    SurfaceKind::Shell,
    SurfaceKind::Search,
    SurfaceKind::Graph,
    SurfaceKind::Ai,
    SurfaceKind::Review,
    SurfaceKind::Support,
];

/// Closed freshness label for a subscriber observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberFreshness {
    /// Subscriber observed the authoritative frame at the current
    /// authority epoch.
    Authoritative,
    /// Subscriber has accepted an earlier frame and is awaiting the
    /// next refresh.
    Cached,
    /// Subscriber knows it is behind the authority epoch.
    Stale,
    /// Subscriber projected an imported / replayed bundle that is not
    /// authoritative for the live workspace.
    Imported,
    /// Subscriber has warmed up but never observed an authoritative
    /// frame.
    Warming,
    /// Subscriber cannot serve the projection right now.
    Unavailable,
}

impl SubscriberFreshness {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Imported => "imported",
            Self::Warming => "warming",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed invalidation-cause vocabulary. Records why a subscriber
/// observed its latest frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationCauseClass {
    /// Authority wrote a new frame.
    AuthorityWrite,
    /// Derived producer recomputed from upstream inputs.
    DerivedRecompute,
    /// Policy / entitlement change rebroadcast.
    PolicyChange,
    /// Provider-overlay change.
    ProviderOverlayChange,
    /// External-change notification from the filesystem watcher or
    /// remote replica.
    ExternalChange,
    /// Imported bundle swap (replay / restore).
    ImportedBundleSwap,
    /// Subscriber asked for or received a resync_required terminal.
    ResyncRequired,
}

impl InvalidationCauseClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityWrite => "authority_write",
            Self::DerivedRecompute => "derived_recompute",
            Self::PolicyChange => "policy_change",
            Self::ProviderOverlayChange => "provider_overlay_change",
            Self::ExternalChange => "external_change",
            Self::ImportedBundleSwap => "imported_bundle_swap",
            Self::ResyncRequired => "resync_required",
        }
    }
}

/// Closed epoch-parity-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochParityState {
    /// All required subscribers observe the current authority epoch
    /// authoritatively.
    Aligned,
    /// At least one subscriber lags the authority epoch or holds a
    /// non-authoritative frame.
    DriftDetected,
    /// At least one subscriber is waiting for a resync after an
    /// invalidation it could not absorb in place.
    AwaitingResync,
    /// At least one subscriber cannot serve the projection right now.
    TerminalUnavailable,
}

impl EpochParityState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::DriftDetected => "drift_detected",
            Self::AwaitingResync => "awaiting_resync",
            Self::TerminalUnavailable => "terminal_unavailable",
        }
    }

    pub const fn is_aligned(self) -> bool {
        matches!(self, Self::Aligned)
    }
}

/// Closed support-export-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportPosture {
    /// View stays on the local device; export pipelines do not include
    /// the underlying bytes.
    LocalOnly,
    /// View ships in metadata-safe export bundles (epoch state +
    /// authority label, never raw payload bytes).
    MetadataSafeExport,
    /// View is held under records governance until support release
    /// signs off; export is gated.
    HeldRecord,
}

impl SupportExportPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Closed downgrade-label vocabulary; a failing row downgrades using
/// one of these labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeLabel {
    /// No downgrade applied; the row passes outright.
    None,
    /// Red — the beta row is blocked until drift is resolved.
    RedBlocksBetaRow,
    /// Yellow — at least one surface lags but the safety invariants
    /// still hold.
    YellowSurfacePartial,
    /// Yellow — authority skew detected; the surfaces project a
    /// consistent older frame.
    YellowAuthoritySkew,
    /// View degrades to the authority-only path until replication
    /// parity ships.
    DegradedToAuthorityOnly,
    /// The protected corpus is stale; release candidate cannot
    /// promote until it is restored.
    StaleCorpusBlocksReleaseCandidate,
}

impl DowngradeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedBlocksBetaRow => "red_blocks_beta_row",
            Self::YellowSurfacePartial => "yellow_surface_partial",
            Self::YellowAuthoritySkew => "yellow_authority_skew",
            Self::DegradedToAuthorityOnly => "degraded_to_authority_only",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }

    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Closed open-gap class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenGapClass {
    None,
    /// A consumer surface implementation is pending.
    SubscriberPending,
    /// The replication backend lags; remediation is out of band.
    ReplicationPending,
    /// The support export pipeline cannot preserve this view yet.
    SupportExportPending,
    /// Drift recovery is currently a manual step.
    DriftRecoveryManual,
}

impl OpenGapClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SubscriberPending => "subscriber_pending",
            Self::ReplicationPending => "replication_pending",
            Self::SupportExportPending => "support_export_pending",
            Self::DriftRecoveryManual => "drift_recovery_manual",
        }
    }
}

/// One open-gap row attached to a reactive-view case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenGapEntry {
    pub gap_class: OpenGapClass,
    pub summary: String,
}

/// One subscriber-surface observation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberEpoch {
    pub surface_kind: SurfaceKind,
    pub observed_epoch: u64,
    pub observed_freshness: SubscriberFreshness,
    pub last_invalidation_cause: InvalidationCauseClass,
    pub observed_at: String,
}

/// Metadata-safe support-export projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportProjection {
    pub posture: SupportExportPosture,
    pub includes_view_class: bool,
    pub includes_authority_label: bool,
    pub includes_authority_epoch: bool,
    pub includes_subscriber_epochs: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub preserves_user_authored_files: bool,
}

impl SupportExportProjection {
    pub const fn metadata_safe_baseline(posture: SupportExportPosture) -> Self {
        Self {
            posture,
            includes_view_class: true,
            includes_authority_label: true,
            includes_authority_epoch: true,
            includes_subscriber_epochs: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            preserves_user_authored_files: true,
        }
    }
}

/// Safety baseline pinned on every reactive-view case and on the
/// report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSafety {
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl CaseSafety {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs quoted on each reactive-view case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseReferences {
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
}

/// One reactive-view case record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewCase {
    pub schema_version: u32,
    pub record_kind: String,
    pub view_id: String,
    pub title: String,
    pub view_class: MaterializedViewClass,
    pub authority_label: AuthorityLabel,
    pub authority_epoch: u64,
    pub subscriber_epochs: Vec<SubscriberEpoch>,
    pub parity_state: EpochParityState,
    pub support_export: SupportExportProjection,
    pub downgrade_label: DowngradeLabel,
    #[serde(default)]
    pub open_gaps: Vec<OpenGapEntry>,
    pub safety: CaseSafety,
    pub references: CaseReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// One fixture-bound entry in the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewCorpusEntry {
    pub fixture_ref: String,
    pub case: MaterializedViewCase,
}

/// Reactive-view corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewCorpus {
    pub entries: Vec<MaterializedViewCorpusEntry>,
}

/// One row in the report projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportMatrixRow {
    pub view_id: String,
    pub view_class: MaterializedViewClass,
    pub authority_label: AuthorityLabel,
    pub authority_epoch: u64,
    pub parity_state: EpochParityState,
    pub min_subscriber_epoch: u64,
    pub max_subscriber_epoch: u64,
    pub support_export_posture: SupportExportPosture,
    pub downgrade_label: DowngradeLabel,
    pub open_gap_classes: Vec<OpenGapClass>,
}

impl ReportMatrixRow {
    fn from_case(case: &MaterializedViewCase) -> Self {
        let (min_epoch, max_epoch) = subscriber_epoch_bounds(&case.subscriber_epochs)
            .unwrap_or((case.authority_epoch, case.authority_epoch));
        let mut open_gap_classes: Vec<OpenGapClass> =
            case.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(OpenGapClass::None);
        }
        Self {
            view_id: case.view_id.clone(),
            view_class: case.view_class,
            authority_label: case.authority_label,
            authority_epoch: case.authority_epoch,
            parity_state: case.parity_state,
            min_subscriber_epoch: min_epoch,
            max_subscriber_epoch: max_epoch,
            support_export_posture: case.support_export.posture,
            downgrade_label: case.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Per-view-class summary row of the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewClassSummaryRow {
    pub view_class: MaterializedViewClass,
    pub case_count: u32,
    pub aligned_count: u32,
    pub drift_detected_count: u32,
    pub awaiting_resync_count: u32,
    pub terminal_unavailable_count: u32,
    pub downgrade_required_count: u32,
}

/// Metadata-safe reactive-view report record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewReport {
    pub schema_version: u32,
    pub record_kind: String,
    pub report_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub corpus_manifest_ref: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub required_view_classes: Vec<MaterializedViewClass>,
    pub required_surface_kinds: Vec<SurfaceKind>,
    pub matrix_rows: Vec<ReportMatrixRow>,
    pub view_class_summaries: Vec<ViewClassSummaryRow>,
}

impl MaterializedViewReport {
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if self.matrix_rows.is_empty() {
            return false;
        }
        if self.view_class_summaries.is_empty() {
            return false;
        }
        true
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactiveViewViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactiveViewValidationReport {
    pub violations: Vec<ReactiveViewViolation>,
}

impl fmt::Display for ReactiveViewValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} reactive-view violation(s)",
            self.violations.len()
        )
    }
}

impl Error for ReactiveViewValidationReport {}

/// Reactive-view evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct ReactiveViewEvaluator;

impl ReactiveViewEvaluator {
    pub const fn new() -> Self {
        Self
    }

    pub fn validate_case(
        &self,
        case: &MaterializedViewCase,
    ) -> Result<(), ReactiveViewValidationReport> {
        let violations = validate_case(case);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ReactiveViewValidationReport { violations })
        }
    }

    pub fn validate_corpus(
        &self,
        corpus: &MaterializedViewCorpus,
    ) -> Result<(), ReactiveViewValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ReactiveViewValidationReport { violations })
        }
    }

    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &MaterializedViewCorpus,
    ) -> Result<MaterializedViewReport, ReactiveViewValidationReport> {
        self.validate_corpus(corpus)?;
        let mut matrix_rows: Vec<ReportMatrixRow> = corpus
            .entries
            .iter()
            .map(|entry| ReportMatrixRow::from_case(&entry.case))
            .collect();
        matrix_rows.sort_by(|a, b| a.view_id.cmp(&b.view_id));

        let view_class_summaries = REQUIRED_VIEW_CLASSES
            .iter()
            .map(|view_class| summarize_view_class(corpus, *view_class))
            .collect();

        Ok(MaterializedViewReport {
            schema_version: REACTIVE_VIEW_SCHEMA_VERSION,
            record_kind: REACTIVE_VIEW_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REACTIVE_VIEW_DOC_REF.to_owned(),
            schema_ref: REACTIVE_VIEW_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: REACTIVE_VIEW_CORPUS_MANIFEST_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_view_classes: REQUIRED_VIEW_CLASSES.to_vec(),
            required_surface_kinds: REQUIRED_SURFACE_KINDS.to_vec(),
            matrix_rows,
            view_class_summaries,
        })
    }
}

fn subscriber_epoch_bounds(subscribers: &[SubscriberEpoch]) -> Option<(u64, u64)> {
    let mut iter = subscribers.iter().map(|s| s.observed_epoch);
    let first = iter.next()?;
    let mut min = first;
    let mut max = first;
    for value in iter {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    Some((min, max))
}

fn summarize_view_class(
    corpus: &MaterializedViewCorpus,
    view_class: MaterializedViewClass,
) -> ViewClassSummaryRow {
    let mut row = ViewClassSummaryRow {
        view_class,
        case_count: 0,
        aligned_count: 0,
        drift_detected_count: 0,
        awaiting_resync_count: 0,
        terminal_unavailable_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.case.view_class != view_class {
            continue;
        }
        row.case_count += 1;
        match entry.case.parity_state {
            EpochParityState::Aligned => row.aligned_count += 1,
            EpochParityState::DriftDetected => row.drift_detected_count += 1,
            EpochParityState::AwaitingResync => row.awaiting_resync_count += 1,
            EpochParityState::TerminalUnavailable => row.terminal_unavailable_count += 1,
        }
        if !entry.case.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn validate_corpus(corpus: &MaterializedViewCorpus) -> Vec<ReactiveViewViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            REACTIVE_VIEW_CORPUS_DIR,
            "corpus must contain at least one reactive-view case",
        );
        return violations;
    }

    let mut view_ids = BTreeSet::new();
    let mut fixture_refs = BTreeSet::new();
    let mut seen_view_classes: BTreeSet<MaterializedViewClass> = BTreeSet::new();
    let mut seen_drift = false;

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let case = &entry.case;
        if !view_ids.insert(case.view_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_view_id",
                &case.view_id,
                "view_id must be unique within the corpus",
            );
        }
        seen_view_classes.insert(case.view_class);
        if matches!(
            case.parity_state,
            EpochParityState::DriftDetected | EpochParityState::AwaitingResync
        ) {
            seen_drift = true;
        }
        violations.extend(validate_case(case));
    }

    for view_class in REQUIRED_VIEW_CLASSES {
        if !seen_view_classes.contains(&view_class) {
            push_violation(
                &mut violations,
                "corpus.required_view_class_missing",
                view_class.as_str(),
                format!(
                    "corpus must seed at least one case for view_class = {}",
                    view_class.as_str()
                ),
            );
        }
    }

    if !seen_drift {
        push_violation(
            &mut violations,
            "corpus.drift_or_resync_case_missing",
            REACTIVE_VIEW_CORPUS_DIR,
            "corpus must seed at least one case with parity_state in {drift_detected, awaiting_resync} so cross-surface drift is observable",
        );
    }

    violations
}

fn validate_case(case: &MaterializedViewCase) -> Vec<ReactiveViewViolation> {
    let mut violations = Vec::new();
    let target = case.view_id.as_str();

    if case.schema_version != REACTIVE_VIEW_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "case.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if case.record_kind != REACTIVE_VIEW_CASE_RECORD_KIND {
        push_violation(
            &mut violations,
            "case.record_kind",
            target,
            format!("record_kind must be {REACTIVE_VIEW_CASE_RECORD_KIND}"),
        );
    }
    if case.view_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.view_id",
            target,
            "view_id must be non-empty",
        );
    }
    if case.title.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.title",
            target,
            "title must be non-empty",
        );
    }
    if case.captured_at.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.captured_at",
            target,
            "captured_at must be non-empty",
        );
    }

    validate_subscribers(&mut violations, target, case);
    validate_parity(&mut violations, target, case);
    validate_support_export(&mut violations, target, case);
    validate_outcome_and_downgrade(&mut violations, target, case);
    validate_open_gaps(&mut violations, target, &case.open_gaps);
    validate_safety(&mut violations, target, &case.safety);
    validate_references(&mut violations, target, &case.references);

    violations
}

fn validate_subscribers(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    case: &MaterializedViewCase,
) {
    let mut seen: BTreeSet<SurfaceKind> = BTreeSet::new();
    for sub in &case.subscriber_epochs {
        if !seen.insert(sub.surface_kind) {
            push_violation(
                violations,
                "case.subscriber.duplicate_surface_kind",
                target,
                format!(
                    "subscriber_epochs must not declare duplicate surface_kind = {}",
                    sub.surface_kind.as_str()
                ),
            );
        }
        if sub.observed_at.trim().is_empty() {
            push_violation(
                violations,
                "case.subscriber.observed_at",
                target,
                format!(
                    "subscriber_epochs[{}].observed_at must be non-empty",
                    sub.surface_kind.as_str()
                ),
            );
        }
        if sub.observed_epoch > case.authority_epoch {
            push_violation(
                violations,
                "case.subscriber.epoch_exceeds_authority",
                target,
                format!(
                    "subscriber_epochs[{}].observed_epoch ({}) must not exceed authority_epoch ({})",
                    sub.surface_kind.as_str(),
                    sub.observed_epoch,
                    case.authority_epoch
                ),
            );
        }
    }
    for required in REQUIRED_SURFACE_KINDS {
        if !seen.contains(&required) {
            push_violation(
                violations,
                "case.subscriber.required_surface_missing",
                target,
                format!(
                    "subscriber_epochs must declare an entry for required surface_kind = {}",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_parity(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    case: &MaterializedViewCase,
) {
    let subscribers = &case.subscriber_epochs;
    if subscribers.is_empty() {
        return;
    }
    let any_unavailable = subscribers
        .iter()
        .any(|s| s.observed_freshness == SubscriberFreshness::Unavailable);
    let any_resync_required = subscribers
        .iter()
        .any(|s| s.last_invalidation_cause == InvalidationCauseClass::ResyncRequired);
    let any_stale = subscribers
        .iter()
        .any(|s| matches!(s.observed_freshness, SubscriberFreshness::Stale));
    let any_warming = subscribers
        .iter()
        .any(|s| s.observed_freshness == SubscriberFreshness::Warming);
    let any_lag = subscribers
        .iter()
        .any(|s| s.observed_epoch < case.authority_epoch);
    let all_at_authority = subscribers.iter().all(|s| {
        s.observed_epoch == case.authority_epoch
            && matches!(
                s.observed_freshness,
                SubscriberFreshness::Authoritative | SubscriberFreshness::Imported
            )
    });
    let all_authoritative = subscribers
        .iter()
        .all(|s| s.observed_freshness == SubscriberFreshness::Authoritative);

    match case.parity_state {
        EpochParityState::Aligned => {
            if !all_at_authority || !all_authoritative {
                push_violation(
                    violations,
                    "case.parity.aligned_requires_authoritative_at_authority_epoch",
                    target,
                    "parity_state = aligned requires every subscriber.observed_epoch == authority_epoch and observed_freshness = authoritative",
                );
            }
            if any_unavailable {
                push_violation(
                    violations,
                    "case.parity.aligned_must_not_have_unavailable",
                    target,
                    "parity_state = aligned must not declare any subscriber with observed_freshness = unavailable",
                );
            }
            if any_resync_required {
                push_violation(
                    violations,
                    "case.parity.aligned_must_not_require_resync",
                    target,
                    "parity_state = aligned must not declare any subscriber with last_invalidation_cause = resync_required",
                );
            }
        }
        EpochParityState::DriftDetected => {
            if !any_lag {
                push_violation(
                    violations,
                    "case.parity.drift_requires_epoch_lag",
                    target,
                    "parity_state = drift_detected requires at least one subscriber.observed_epoch < authority_epoch",
                );
            }
            if any_unavailable {
                push_violation(
                    violations,
                    "case.parity.drift_must_not_have_unavailable",
                    target,
                    "parity_state = drift_detected must not declare any subscriber with observed_freshness = unavailable",
                );
            }
        }
        EpochParityState::AwaitingResync => {
            if !any_resync_required && !any_stale && !any_warming {
                push_violation(
                    violations,
                    "case.parity.awaiting_resync_requires_signal",
                    target,
                    "parity_state = awaiting_resync requires at least one subscriber with last_invalidation_cause = resync_required or observed_freshness in {stale, warming}",
                );
            }
            if any_unavailable {
                push_violation(
                    violations,
                    "case.parity.awaiting_resync_must_not_have_unavailable",
                    target,
                    "parity_state = awaiting_resync must not declare any subscriber with observed_freshness = unavailable",
                );
            }
        }
        EpochParityState::TerminalUnavailable => {
            if !any_unavailable {
                push_violation(
                    violations,
                    "case.parity.terminal_requires_unavailable",
                    target,
                    "parity_state = terminal_unavailable requires at least one subscriber with observed_freshness = unavailable",
                );
            }
        }
    }
}

fn validate_support_export(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    case: &MaterializedViewCase,
) {
    let support = &case.support_export;
    if !support.raw_private_material_excluded {
        push_violation(
            violations,
            "case.support_export.raw_private_material_excluded",
            target,
            "support_export.raw_private_material_excluded must be true",
        );
    }
    if !support.ambient_authority_excluded {
        push_violation(
            violations,
            "case.support_export.ambient_authority_excluded",
            target,
            "support_export.ambient_authority_excluded must be true",
        );
    }
    if !support.preserves_user_authored_files {
        push_violation(
            violations,
            "case.support_export.preserves_user_authored_files",
            target,
            "support_export.preserves_user_authored_files must be true",
        );
    }
    if !support.includes_view_class
        || !support.includes_authority_label
        || !support.includes_authority_epoch
        || !support.includes_subscriber_epochs
    {
        push_violation(
            violations,
            "case.support_export.must_preserve_epoch_state",
            target,
            "support_export must include view_class, authority_label, authority_epoch, and subscriber_epochs so support bundles preserve epoch state",
        );
    }
    match (case.view_class, support.posture) {
        (MaterializedViewClass::ExportableSnapshot, SupportExportPosture::LocalOnly) => {
            push_violation(
                violations,
                "case.support_export.exportable_snapshot_posture",
                target,
                "exportable_snapshot view_class must declare posture = metadata_safe_export or held_record",
            );
        }
        (MaterializedViewClass::ManagedReplicatedView, SupportExportPosture::LocalOnly) => {
            push_violation(
                violations,
                "case.support_export.managed_replicated_posture",
                target,
                "managed_replicated_view view_class must declare posture = metadata_safe_export or held_record",
            );
        }
        _ => {}
    }
}

fn validate_outcome_and_downgrade(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    case: &MaterializedViewCase,
) {
    let healthy = case.downgrade_label.is_healthy();
    match (case.parity_state, healthy) {
        (EpochParityState::Aligned, false) => {
            push_violation(
                violations,
                "case.outcome.aligned_must_not_carry_downgrade",
                target,
                "parity_state = aligned must declare downgrade_label = none",
            );
        }
        (
            EpochParityState::DriftDetected
            | EpochParityState::AwaitingResync
            | EpochParityState::TerminalUnavailable,
            true,
        ) => {
            push_violation(
                violations,
                "case.outcome.non_aligned_must_declare_downgrade",
                target,
                "non-aligned parity_state must declare a non-none downgrade_label",
            );
        }
        _ => {}
    }
    if !healthy {
        let has_open_gap = case
            .open_gaps
            .iter()
            .any(|gap| gap.gap_class != OpenGapClass::None);
        if !has_open_gap {
            push_violation(
                violations,
                "case.outcome.non_aligned_must_record_open_gap",
                target,
                "downgraded rows must record at least one open_gap with a non-none gap_class",
            );
        }
    } else if case
        .open_gaps
        .iter()
        .any(|gap| gap.gap_class != OpenGapClass::None)
    {
        push_violation(
            violations,
            "case.outcome.aligned_must_not_record_open_gap",
            target,
            "aligned rows must not declare any open_gap with a non-none gap_class",
        );
    }
    if case.parity_state == EpochParityState::TerminalUnavailable
        && case.downgrade_label != DowngradeLabel::RedBlocksBetaRow
        && case.downgrade_label != DowngradeLabel::DegradedToAuthorityOnly
    {
        push_violation(
            violations,
            "case.outcome.terminal_label_class",
            target,
            "parity_state = terminal_unavailable must downgrade with red_blocks_beta_row or degraded_to_authority_only",
        );
    }
}

fn validate_open_gaps(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    gaps: &[OpenGapEntry],
) {
    let mut seen = BTreeSet::new();
    for gap in gaps {
        if gap.summary.trim().is_empty() {
            push_violation(
                violations,
                "case.open_gaps.summary",
                target,
                "open_gaps.summary must be non-empty",
            );
        }
        if !seen.insert(gap.gap_class) {
            push_violation(
                violations,
                "case.open_gaps.duplicate_gap_class",
                target,
                format!("duplicate open_gap_class {}", gap.gap_class.as_str()),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    safety: &CaseSafety,
) {
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "case.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "case.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "case.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "case.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<ReactiveViewViolation>,
    target: &str,
    refs: &CaseReferences,
) {
    if refs.doc_ref != REACTIVE_VIEW_DOC_REF {
        push_violation(
            violations,
            "case.references.doc_ref",
            target,
            format!("references.doc_ref must pin {REACTIVE_VIEW_DOC_REF}"),
        );
    }
    if refs.schema_ref != REACTIVE_VIEW_SCHEMA_REF {
        push_violation(
            violations,
            "case.references.schema_ref",
            target,
            format!("references.schema_ref must pin {REACTIVE_VIEW_SCHEMA_REF}"),
        );
    }
    if refs.report_ref != REACTIVE_VIEW_REPORT_REF {
        push_violation(
            violations,
            "case.references.report_ref",
            target,
            format!("references.report_ref must pin {REACTIVE_VIEW_REPORT_REF}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<ReactiveViewViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ReactiveViewViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Loads a YAML-encoded [`MaterializedViewCase`].
pub fn load_materialized_view_case(yaml: &str) -> Result<MaterializedViewCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in reactive-views beta corpus.
pub fn current_materialized_view_corpus() -> Result<MaterializedViewCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<MaterializedViewCase>(yaml).map(|case| {
                MaterializedViewCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(MaterializedViewCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration
/// order.
pub fn current_materialized_view_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/state/reactive_views_beta/ephemeral_shell_status_aligned_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/reactive_views_beta/ephemeral_shell_status_aligned_case.yaml"
        )),
    ),
    (
        "fixtures/state/reactive_views_beta/durable_workspace_index_aligned_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/reactive_views_beta/durable_workspace_index_aligned_case.yaml"
        )),
    ),
    (
        "fixtures/state/reactive_views_beta/exportable_support_snapshot_aligned_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/reactive_views_beta/exportable_support_snapshot_aligned_case.yaml"
        )),
    ),
    (
        "fixtures/state/reactive_views_beta/managed_review_state_drift_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/reactive_views_beta/managed_review_state_drift_case.yaml"
        )),
    ),
    (
        "fixtures/state/reactive_views_beta/derived_graph_neighborhood_resync_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/reactive_views_beta/derived_graph_neighborhood_resync_case.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn aligned_case() -> MaterializedViewCase {
        let subs = REQUIRED_SURFACE_KINDS
            .iter()
            .map(|surface| SubscriberEpoch {
                surface_kind: *surface,
                observed_epoch: 7,
                observed_freshness: SubscriberFreshness::Authoritative,
                last_invalidation_cause: InvalidationCauseClass::AuthorityWrite,
                observed_at: "2026-05-16T00:00:00Z".to_owned(),
            })
            .collect();
        MaterializedViewCase {
            schema_version: REACTIVE_VIEW_SCHEMA_VERSION,
            record_kind: REACTIVE_VIEW_CASE_RECORD_KIND.to_owned(),
            view_id: "view:test:aligned".to_owned(),
            title: "Aligned test view".to_owned(),
            view_class: MaterializedViewClass::EphemeralProjection,
            authority_label: AuthorityLabel::WorkspaceVfs,
            authority_epoch: 7,
            subscriber_epochs: subs,
            parity_state: EpochParityState::Aligned,
            support_export: SupportExportProjection::metadata_safe_baseline(
                SupportExportPosture::LocalOnly,
            ),
            downgrade_label: DowngradeLabel::None,
            open_gaps: vec![],
            safety: CaseSafety::metadata_safe_baseline(),
            references: CaseReferences {
                doc_ref: REACTIVE_VIEW_DOC_REF.to_owned(),
                schema_ref: REACTIVE_VIEW_SCHEMA_REF.to_owned(),
                report_ref: REACTIVE_VIEW_REPORT_REF.to_owned(),
            },
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn aligned_case_validates() {
        ReactiveViewEvaluator::new()
            .validate_case(&aligned_case())
            .expect("aligned test case must validate");
    }

    #[test]
    fn refuses_aligned_with_downgrade_label() {
        let mut case = aligned_case();
        case.downgrade_label = DowngradeLabel::YellowSurfacePartial;
        let err = ReactiveViewEvaluator::new()
            .validate_case(&case)
            .expect_err("aligned with downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.aligned_must_not_carry_downgrade"));
    }

    #[test]
    fn refuses_drift_without_epoch_lag() {
        let mut case = aligned_case();
        case.parity_state = EpochParityState::DriftDetected;
        case.downgrade_label = DowngradeLabel::YellowSurfacePartial;
        case.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::SubscriberPending,
            summary: "subscriber lag".into(),
        });
        let err = ReactiveViewEvaluator::new()
            .validate_case(&case)
            .expect_err("drift without lag must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.parity.drift_requires_epoch_lag"));
    }

    #[test]
    fn refuses_subscriber_epoch_above_authority() {
        let mut case = aligned_case();
        case.subscriber_epochs[0].observed_epoch = case.authority_epoch + 1;
        let err = ReactiveViewEvaluator::new()
            .validate_case(&case)
            .expect_err("subscriber epoch above authority must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.subscriber.epoch_exceeds_authority"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut case = aligned_case();
        case.safety.destructive_resets_present = true;
        let err = ReactiveViewEvaluator::new()
            .validate_case(&case)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.destructive_resets_present"));
    }

    #[test]
    fn refuses_exportable_snapshot_with_local_only_posture() {
        let mut case = aligned_case();
        case.view_class = MaterializedViewClass::ExportableSnapshot;
        let err = ReactiveViewEvaluator::new()
            .validate_case(&case)
            .expect_err("exportable_snapshot with local_only must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.support_export.exportable_snapshot_posture"));
    }

    #[test]
    fn refuses_support_export_dropping_epoch_fields() {
        let mut case = aligned_case();
        case.support_export.includes_subscriber_epochs = false;
        let err = ReactiveViewEvaluator::new()
            .validate_case(&case)
            .expect_err("dropped epoch fields must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.support_export.must_preserve_epoch_state"));
    }

    #[test]
    fn refuses_corpus_missing_required_view_class() {
        let mut corpus = MaterializedViewCorpus {
            entries: vec![MaterializedViewCorpusEntry {
                fixture_ref: "fixtures/state/reactive_views_beta/only_one.yaml".into(),
                case: aligned_case(),
            }],
        };
        let drift_case_id = "view:test:drift".to_owned();
        let mut drift = aligned_case();
        drift.view_id = drift_case_id;
        drift.view_class = MaterializedViewClass::DurableLocalMaterialization;
        drift.parity_state = EpochParityState::DriftDetected;
        drift.downgrade_label = DowngradeLabel::YellowSurfacePartial;
        drift.subscriber_epochs[0].observed_epoch = drift.authority_epoch - 1;
        drift.subscriber_epochs[0].observed_freshness = SubscriberFreshness::Cached;
        drift.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::DriftRecoveryManual,
            summary: "manual rerun".into(),
        });
        corpus.entries.push(MaterializedViewCorpusEntry {
            fixture_ref: "fixtures/state/reactive_views_beta/drift.yaml".into(),
            case: drift,
        });
        let err = ReactiveViewEvaluator::new()
            .validate_corpus(&corpus)
            .expect_err("missing required view classes must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "corpus.required_view_class_missing"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus = current_materialized_view_corpus().expect("checked-in corpus must parse");
        ReactiveViewEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for required in REQUIRED_VIEW_CLASSES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.case.view_class == required),
                "checked-in corpus must seed a case for view_class = {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn checked_in_report_is_export_safe() {
        let corpus = current_materialized_view_corpus().unwrap();
        let report = ReactiveViewEvaluator::new()
            .report("report:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("report must build");
        assert!(report.is_export_safe());
        assert_eq!(report.matrix_rows.len(), corpus.entries.len());
        assert_eq!(
            report.view_class_summaries.len(),
            REQUIRED_VIEW_CLASSES.len()
        );
        let total: u32 = report
            .view_class_summaries
            .iter()
            .map(|s| s.case_count)
            .sum();
        assert_eq!(total as usize, corpus.entries.len());
    }
}
