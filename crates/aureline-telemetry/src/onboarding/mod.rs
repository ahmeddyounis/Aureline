//! Onboarding task-success telemetry records.
//!
//! This module owns the opt-in telemetry contract for onboarding flow
//! outcomes, first-useful-work timing, and migration funnel progress. It keeps
//! entry verbs explicit and only records privacy-safe metadata so the same
//! events can feed dashboards, design-partner proof, and support summaries.

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::trace_event::BuildIdentityRecord;

/// Schema token for onboarding task-success telemetry.
pub const SCHEMA: &str = "aureline.onboarding_task_success.v1";

/// Schema version for [`SCHEMA`].
pub const SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for one onboarding telemetry event.
pub const EVENT_RECORD_KIND: &str = "onboarding_task_success_event_record";

/// Record-kind discriminator for grouped onboarding telemetry captures.
pub const CAPTURE_RECORD_KIND: &str = "onboarding_task_success_capture_record";

/// Default generated-at value used by deterministic fixtures.
pub const ONBOARDING_TASK_SUCCESS_FIXTURE_GENERATED_AT: &str = "fixture:onboarding-task-success";

/// Distinct entry-flow families dashboards must keep separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryFlowKind {
    /// First launch with no prior local profile or selected workspace.
    FirstRun,
    /// Ordinary open of a local file, folder, or workspace.
    Open,
    /// Repository clone flow.
    Clone,
    /// Import or migration flow.
    Import,
    /// Session or crash restore flow.
    Restore,
    /// Reconnect to a previously known local, remote, or managed target.
    Reconnect,
}

impl EntryFlowKind {
    /// Flow coverage required for design-partner onboarding proof.
    pub const REQUIRED_DESIGN_PARTNER_FLOWS: [Self; 6] = [
        Self::FirstRun,
        Self::Open,
        Self::Clone,
        Self::Import,
        Self::Restore,
        Self::Reconnect,
    ];

    /// Returns the stable serialized token for this flow.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRun => "first_run",
            Self::Open => "open",
            Self::Clone => "clone",
            Self::Import => "import",
            Self::Restore => "restore",
            Self::Reconnect => "reconnect",
        }
    }
}

/// Entry verbs emitted by project-entry and onboarding surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryVerbKind {
    /// Open a local folder.
    OpenFolder,
    /// Open a workspace file or multi-root workspace.
    OpenWorkspace,
    /// Clone a repository before opening it.
    CloneRepository,
    /// Import settings, profile, handoff, or migration state.
    ImportFromExternal,
    /// Restore the last session.
    RestoreLastSession,
    /// Reconnect a known target.
    ReconnectTarget,
}

impl EntryVerbKind {
    /// Returns the stable serialized token for this verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFolder => "open_folder",
            Self::OpenWorkspace => "open_workspace",
            Self::CloneRepository => "clone_repository",
            Self::ImportFromExternal => "import_from_external",
            Self::RestoreLastSession => "restore_last_session",
            Self::ReconnectTarget => "reconnect_target",
        }
    }
}

/// Entry-route identifiers frozen by the onboarding measurement plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EntryRouteId {
    /// Start Center or first-run row activation.
    #[serde(rename = "er.start_center")]
    StartCenter,
    /// Reopen through a recent-work surface.
    #[serde(rename = "er.recent_work_reopen")]
    RecentWorkReopen,
    /// Explicit restore prompt activation.
    #[serde(rename = "er.restore_prompt")]
    RestorePrompt,
    /// Operating-system, deep-link, or companion reentry.
    #[serde(rename = "er.protocol_handler_reentry")]
    ProtocolHandlerReentry,
    /// Clone or import command route.
    #[serde(rename = "er.clone_or_import")]
    CloneOrImport,
    /// Plain open route.
    #[serde(rename = "er.plain_open")]
    PlainOpen,
    /// In-process workspace switch route.
    #[serde(rename = "er.workspace_switch")]
    WorkspaceSwitch,
    /// Warm start without a restore prompt.
    #[serde(rename = "er.warm_start")]
    WarmStart,
}

/// Measurement surfaces consumed by onboarding dashboards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementSurface {
    /// First-run measurement surface.
    SurfaceFirstRun,
    /// First-open measurement surface.
    SurfaceFirstOpen,
    /// First-useful-edit measurement surface.
    SurfaceFirstUsefulEdit,
    /// Migration review measurement surface.
    SurfaceMigrationReview,
    /// Restore success measurement surface.
    SurfaceRestoreSuccess,
    /// Optional-service opt-in boundary surface.
    SurfaceOptInBoundary,
    /// Reconnect measurement surface.
    SurfaceReconnect,
}

/// Target classes that avoid raw path, host, repository, and URL capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    /// Local folder target.
    LocalFolder,
    /// Workspace-file target.
    WorkspaceFile,
    /// Single local file target.
    LocalFile,
    /// Remote workspace target.
    RemoteWorkspace,
    /// Managed workspace target.
    ManagedWorkspace,
    /// Import packet or staged migration target.
    ImportPacket,
    /// Target kind could not be resolved.
    Unknown,
}

/// Entry-flow metadata attached to each telemetry event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryFlowDescriptor {
    /// Coarse flow family used for funnel grouping.
    pub flow_kind: EntryFlowKind,
    /// Exact entry verb chosen by the user.
    pub entry_verb: EntryVerbKind,
    /// Measurement route that admitted the flow.
    pub entry_route_id: EntryRouteId,
    /// Measurement surface that owns the event.
    pub measurement_surface: MeasurementSurface,
    /// Privacy-safe target class.
    pub target_kind: TargetKind,
    /// Optional opaque target reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    /// Deployment profile used for this entry.
    pub deployment_profile_id: String,
}

/// Event names reserved for onboarding task-success telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingEventName {
    /// First-run surface was reached.
    FirstRunReached,
    /// First-run entry route was selected.
    FirstRunEntryRouteSelected,
    /// First-run entry was admitted.
    FirstRunAdmitted,
    /// Entry verb was resolved.
    EntryVerbResolved,
    /// Admission decision was produced.
    AdmissionDecided,
    /// First-open flow completed.
    FirstOpenCompleted,
    /// First useful navigation was reached.
    FirstUsefulNavigationReached,
    /// First useful edit became durable.
    FirstUsefulEditDurable,
    /// Migration dry-run produced a reviewable diff.
    MigrationDryRunProduced,
    /// Migration outcome row was recorded.
    MigrationOutcomeRecorded,
    /// Migration apply completed.
    MigrationApplied,
    /// Migration rollback completed.
    MigrationRolledBack,
    /// Migration rollback checkpoint was written.
    MigrationRollbackCheckpointWritten,
    /// Migration rollback checkpoint was restored.
    MigrationRollbackCheckpointRestored,
    /// Restore prompt was presented.
    RestorePromptPresented,
    /// Restore level was delivered.
    RestoreLevelDelivered,
    /// Restore flow completed.
    RestoreCompleted,
    /// Reconnect prompt was presented.
    ReconnectPromptPresented,
    /// Reconnect flow completed.
    ReconnectCompleted,
    /// Reconnect flow failed.
    ReconnectFailed,
}

/// Phase within an onboarding event stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingEventPhase {
    /// User intent or route selection.
    Intent,
    /// Admission or policy decision.
    Admission,
    /// First useful work.
    UsefulWork,
    /// Migration review, apply, or rollback.
    MigrationReview,
    /// Restore flow.
    Restore,
    /// Reconnect flow.
    Reconnect,
    /// Completion event.
    Completion,
}

/// Completion checkpoint that qualifies task success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionCheckpointClass {
    /// First useful navigation checkpoint.
    FirstUsefulNavigation,
    /// First useful edit checkpoint.
    FirstUsefulEdit,
    /// Migration committed with per-item outcomes.
    MigrationCommittedWithPerItemOutcomes,
    /// Restore level was delivered.
    RestoreLevelDelivered,
    /// Decline path continued without degradation.
    DeclineContinuedWithoutDegradation,
    /// Reconnect target was delivered.
    ReconnectDelivered,
}

/// Completion classes emitted by onboarding flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionClass {
    /// Flow completed with first useful edit.
    CompletedFirstUsefulEdit,
    /// Flow completed with first useful navigation only.
    CompletedFirstUsefulNavigationOnly,
    /// Flow completed after an advertised narrowing.
    CompletedWithAdvertisedNarrowing,
    /// Migration committed with per-item outcomes.
    CompletedMigrationCommittedPerItem,
    /// Restore delivered the advertised level.
    CompletedRestoreLevelDelivered,
    /// Optional-service decline continued without degradation.
    CompletedDeclineWithoutDegradation,
    /// Reconnect delivered the target.
    CompletedReconnectDelivered,
    /// Flow aborted before admission.
    AbortedBeforeAdmission,
    /// Flow was abandoned after admission.
    AbandonedAfterAdmission,
    /// Flow failed with a typed blocker.
    FailedWithTypedBlocker,
}

/// Coarse event outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeClass {
    /// Event is pending.
    Pending,
    /// Event completed.
    Completed,
    /// Event partially completed.
    Partial,
    /// Event is blocked.
    Blocked,
    /// Event was abandoned.
    Abandoned,
    /// Event failed.
    Failed,
    /// Event was denied.
    Denied,
    /// Restore completed.
    Restored,
    /// Rollback completed.
    RolledBack,
}

/// First-useful-work target surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstUsefulWorkTargetSurface {
    /// Editor with root-discovery cues.
    EditorWithRootDiscoveryCues,
    /// Tree plus README or changed-files view.
    TreePlusReadmeOrChangedFiles,
    /// Post-clone handoff.
    PostCloneHandoff,
    /// Review or work-item anchor.
    ReviewOrWorkItemAnchor,
    /// Prior layout with placeholders.
    PriorLayoutWithPlaceholders,
    /// Restore or compare sheet.
    RestoreOrCompareSheet,
    /// Migration center before commit.
    MigrationCenterBeforeCommit,
    /// Prebuild setup envelope.
    PrebuildSetupEnvelope,
    /// Locate-missing-target prompt.
    LocateMissingTargetPrompt,
}

/// Whether first useful work happened before semantic warm-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticWarmupState {
    /// First useful work happened before semantic warm-up completed.
    BeforeSemanticWarmup,
    /// First useful work happened after semantic warm-up completed.
    AfterSemanticWarmup,
    /// Semantic warm-up does not apply to this flow.
    SemanticWarmupNotApplicable,
}

/// Monotonic timing for a first-useful-work checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstUsefulWorkTiming {
    /// Monotonic tick at which the user intent began.
    pub intent_started_tick: u64,
    /// Monotonic tick at which first useful work was reached.
    pub reached_tick: u64,
    /// Saturating duration from intent to first useful work.
    pub duration_ticks: u64,
    /// First useful work target surface.
    pub target_surface: FirstUsefulWorkTargetSurface,
    /// Relation to semantic warm-up.
    pub semantic_warmup_state: SemanticWarmupState,
    /// Whether raw project content was captured.
    pub raw_project_content_captured: bool,
}

impl FirstUsefulWorkTiming {
    /// Creates a timing record from start and reached ticks.
    pub fn new(
        intent_started_tick: u64,
        reached_tick: u64,
        target_surface: FirstUsefulWorkTargetSurface,
        semantic_warmup_state: SemanticWarmupState,
    ) -> Self {
        Self {
            intent_started_tick,
            reached_tick,
            duration_ticks: reached_tick.saturating_sub(intent_started_tick),
            target_surface,
            semantic_warmup_state,
            raw_project_content_captured: false,
        }
    }
}

/// Migration source families emitted by migration funnel events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationSourceKind {
    /// VS Code or Code-OSS source.
    VsCode,
    /// JetBrains-family source.
    JetbrainsFamily,
    /// Vim or Neovim source.
    VimOrNeovim,
    /// Emacs source.
    Emacs,
    /// Sublime Text or TextMate source.
    SublimeTextmate,
    /// Earlier Aureline source.
    PriorAureline,
    /// Portable profile bundle source.
    PortableProfileBundle,
    /// Support recovery bundle source.
    SupportRecoveryBundle,
    /// Workflow bundle handoff source.
    WorkflowBundleHandoff,
    /// Generic import source.
    GenericImport,
}

/// Migration funnel steps visible to dashboards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationFunnelStep {
    /// Source was selected.
    SourceSelected,
    /// Dry-run was produced.
    DryRunProduced,
    /// Per-item outcomes were recorded.
    PerItemOutcomesRecorded,
    /// Rollback checkpoint was written.
    CheckpointWritten,
    /// Migration was applied.
    Applied,
    /// Migration was rolled back.
    RolledBack,
    /// Post-import validation ran.
    PostImportValidated,
}

/// Dry-run state for migration funnel events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DryRunState {
    /// Dry-run is not applicable.
    NotApplicable,
    /// Dry-run is required.
    Required,
    /// Dry-run was produced.
    Produced,
    /// Dry-run was skipped.
    Skipped,
    /// Dry-run failed.
    Failed,
}

/// Rollback state for migration funnel events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackState {
    /// Rollback is not applicable.
    NotApplicable,
    /// Rollback is available.
    Available,
    /// Rollback was restored.
    Restored,
    /// Rollback checkpoint is missing.
    Missing,
    /// Rollback failed.
    Failed,
}

/// Importer outcome counts that preserve category-level truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationOutcomeCounts {
    /// Items imported directly.
    pub imported: u64,
    /// Items mapped to an Aureline equivalent.
    pub mapped: u64,
    /// Items skipped.
    pub skipped: u64,
    /// Items needing manual review.
    pub manual_review: u64,
    /// Items requiring a bridge.
    pub bridge_required: u64,
    /// Unsupported items.
    pub unsupported: u64,
    /// Total item count across all outcome classes.
    pub total_items: u64,
}

impl MigrationOutcomeCounts {
    /// Creates outcome counts and computes the total item count.
    pub const fn new(
        imported: u64,
        mapped: u64,
        skipped: u64,
        manual_review: u64,
        bridge_required: u64,
        unsupported: u64,
    ) -> Self {
        Self {
            imported,
            mapped,
            skipped,
            manual_review,
            bridge_required,
            unsupported,
            total_items: imported
                + mapped
                + skipped
                + manual_review
                + bridge_required
                + unsupported,
        }
    }
}

/// Migration funnel details attached to a telemetry event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationFunnelRecord {
    /// Opaque migration-session ref.
    pub migration_session_ref: String,
    /// Source family.
    pub source_kind: MigrationSourceKind,
    /// Funnel step.
    pub step: MigrationFunnelStep,
    /// Dry-run state.
    pub dry_run_state: DryRunState,
    /// Rollback state.
    pub rollback_state: RollbackState,
    /// Opaque rollback-checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Category-level outcome counts.
    pub outcome_counts: MigrationOutcomeCounts,
    /// Opaque parity-scorecard ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parity_scorecard_ref: Option<String>,
    /// Opaque migration-report ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_report_ref: Option<String>,
}

/// Typed failure categories emitted by onboarding telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureCategory {
    /// Sign-in was forced before useful local work.
    ForcedSignInBeforeUsefulLocalWork,
    /// Network was required for local entry.
    NetworkRequiredForLocalEntry,
    /// Admission was denied by policy.
    AdmissionDeniedPolicy,
    /// Admission was denied by trust posture.
    AdmissionDeniedTrust,
    /// Resulting mode was silently downgraded.
    ResultingModeSilentlyDowngraded,
    /// Editor was blocked on index warm-up.
    EditorBlockedOnIndexWarmup,
    /// Save was blocked on a service.
    SaveBlockedOnService,
    /// Migration dry-run was skipped.
    DryRunSkipped,
    /// Migration outcome was aggregated instead of per-item.
    OutcomeAggregatedNotPerItem,
    /// Rollback checkpoint was missing.
    RollbackCheckpointMissing,
    /// Restore promised a higher level than it delivered.
    RestoreLevelPromisedHigherThanDelivered,
    /// Missing target state was silently dropped.
    MissingTargetStateSilentlyDropped,
    /// Mutating command replayed silently during restore.
    SilentMutatingCommandReplay,
    /// Reconnect target was unavailable.
    ReconnectTargetUnavailable,
    /// Reconnect required reauthentication.
    ReconnectRequiresReauth,
    /// Reconnect was blocked by policy.
    ReconnectPolicyBlocked,
}

/// Telemetry consent state for onboarding events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryConsentState {
    /// No emission occurs until explicit consent exists.
    OffByDefaultNoEmissionUntilConsent,
    /// Opt-in telemetry is enabled.
    OptInEnabled,
    /// Capture remains local only.
    LocalOnlyCapture,
}

/// Privacy class for onboarding telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryPrivacyClass {
    /// No telemetry leaves local state.
    PrivacyLocalOnlyNoEmission,
    /// Opt-in aggregate-only telemetry.
    PrivacyOptInAggregateOnly,
    /// Opt-in attributable telemetry.
    PrivacyOptInAttributable,
}

/// Redaction class for onboarding telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryRedactionClass {
    /// Metadata-safe default redaction.
    MetadataSafeDefault,
}

/// Export posture for onboarding telemetry captures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryExportPosture {
    /// Excluded by default.
    ExcludedByDefault,
    /// Included only for a design-partner opt-in proof path.
    DesignPartnerOptIn,
    /// Included only in a requested support export.
    SupportExportOnRequest,
}

/// Content classes that onboarding telemetry must not capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProhibitedContentClass {
    /// Raw project content.
    RawProjectContent,
    /// Raw repository or project name.
    RawRepoName,
    /// File path.
    FilePath,
    /// Raw URL.
    RawUrl,
    /// Prompt text.
    PromptText,
    /// Terminal text.
    TerminalText,
    /// Clipboard content.
    ClipboardContent,
    /// Credential or secret.
    CredentialOrSecret,
}

/// Privacy envelope copied onto each event and grouped capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyEnvelope {
    /// Consent state for this event family.
    pub consent_state: TelemetryConsentState,
    /// Privacy class.
    pub privacy_class: TelemetryPrivacyClass,
    /// Redaction class.
    pub redaction_class: TelemetryRedactionClass,
    /// Export posture.
    pub export_posture: TelemetryExportPosture,
    /// Prohibited content classes for this event family.
    pub prohibited_content_classes: Vec<ProhibitedContentClass>,
    /// Whether raw project content is present.
    pub contains_raw_project_content: bool,
}

impl PrivacyEnvelope {
    /// Returns the metadata-safe local default privacy envelope.
    pub fn metadata_safe_default(export_posture: TelemetryExportPosture) -> Self {
        Self {
            consent_state: TelemetryConsentState::OffByDefaultNoEmissionUntilConsent,
            privacy_class: TelemetryPrivacyClass::PrivacyLocalOnlyNoEmission,
            redaction_class: TelemetryRedactionClass::MetadataSafeDefault,
            export_posture,
            prohibited_content_classes: vec![
                ProhibitedContentClass::RawProjectContent,
                ProhibitedContentClass::RawRepoName,
                ProhibitedContentClass::FilePath,
                ProhibitedContentClass::RawUrl,
                ProhibitedContentClass::PromptText,
                ProhibitedContentClass::TerminalText,
                ProhibitedContentClass::ClipboardContent,
                ProhibitedContentClass::CredentialOrSecret,
            ],
            contains_raw_project_content: false,
        }
    }
}

/// Input used to record one onboarding telemetry event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingEventInput {
    /// Entry-flow metadata.
    pub entry: EntryFlowDescriptor,
    /// Event name.
    pub event_name: OnboardingEventName,
    /// Event phase.
    pub event_phase: OnboardingEventPhase,
    /// Optional completion checkpoint.
    pub completion_checkpoint_class: Option<CompletionCheckpointClass>,
    /// Optional completion class.
    pub completion_class: Option<CompletionClass>,
    /// Event outcome.
    pub outcome_class: OutcomeClass,
    /// Optional first-useful-work timing.
    pub first_useful_work: Option<FirstUsefulWorkTiming>,
    /// Optional migration funnel metadata.
    pub migration_funnel: Option<MigrationFunnelRecord>,
    /// Optional failure category.
    pub failure_category: Option<FailureCategory>,
    /// Event-local evidence refs.
    pub evidence_refs: Vec<String>,
    /// Event occurrence tick.
    pub occurred_tick: u64,
}

/// Context shared across onboarding telemetry events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingTelemetryContext {
    /// Trace id for this capture.
    pub trace_id: String,
    /// Session ref for this capture.
    pub session_ref: String,
    /// Build identity.
    pub build: BuildIdentityRecord,
    /// Privacy envelope.
    pub privacy: PrivacyEnvelope,
    /// Capture-level evidence refs.
    pub evidence_refs: Vec<String>,
}

impl OnboardingTelemetryContext {
    /// Creates a local-only developer context.
    pub fn developer_local(
        trace_id: impl Into<String>,
        session_ref: impl Into<String>,
        build: BuildIdentityRecord,
    ) -> Self {
        Self {
            trace_id: trace_id.into(),
            session_ref: session_ref.into(),
            build,
            privacy: PrivacyEnvelope::metadata_safe_default(
                TelemetryExportPosture::ExcludedByDefault,
            ),
            evidence_refs: vec![
                "schemas/telemetry/onboarding_task_success.schema.json".to_string(),
                "docs/product/onboarding_measurement_plan.md".to_string(),
            ],
        }
    }
}

/// One emitted onboarding task-success event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingTaskSuccessEventRecord {
    /// Schema token.
    pub schema: Cow<'static, str>,
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: Cow<'static, str>,
    /// Event id.
    pub event_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Session ref.
    pub session_ref: String,
    /// Build identity.
    pub build: BuildIdentityRecord,
    /// Event occurrence tick.
    pub occurred_tick: u64,
    /// Entry-flow metadata.
    pub entry: EntryFlowDescriptor,
    /// Event name.
    pub event_name: OnboardingEventName,
    /// Event phase.
    pub event_phase: OnboardingEventPhase,
    /// Optional completion checkpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_checkpoint_class: Option<CompletionCheckpointClass>,
    /// Optional completion class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_class: Option<CompletionClass>,
    /// Event outcome.
    pub outcome_class: OutcomeClass,
    /// Optional first-useful-work timing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_useful_work: Option<FirstUsefulWorkTiming>,
    /// Optional migration funnel metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_funnel: Option<MigrationFunnelRecord>,
    /// Optional failure category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_category: Option<FailureCategory>,
    /// Privacy envelope.
    pub privacy: PrivacyEnvelope,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
}

/// Summary derived from a grouped onboarding capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingTaskSuccessSummary {
    /// Flow families observed in the capture.
    pub observed_flow_kinds: Vec<EntryFlowKind>,
    /// Entry verbs observed in the capture.
    pub distinct_entry_verbs: Vec<EntryVerbKind>,
    /// Number of events carrying first-useful-work timing.
    pub first_useful_work_timing_count: u64,
    /// Number of events carrying migration funnel metadata.
    pub migration_funnel_event_count: u64,
    /// Whether the capture has enough structure for design-partner proof.
    pub design_partner_proof_ready: bool,
}

impl OnboardingTaskSuccessSummary {
    fn from_events(events: &[OnboardingTaskSuccessEventRecord]) -> Self {
        let observed_flow_kinds = events
            .iter()
            .map(|event| event.entry.flow_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let distinct_entry_verbs = events
            .iter()
            .map(|event| event.entry.entry_verb)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let first_useful_work_timing_count = events
            .iter()
            .filter(|event| event.first_useful_work.is_some())
            .count() as u64;
        let migration_funnel_event_count = events
            .iter()
            .filter(|event| event.migration_funnel.is_some())
            .count() as u64;
        let required_flows = EntryFlowKind::REQUIRED_DESIGN_PARTNER_FLOWS
            .into_iter()
            .collect::<BTreeSet<_>>();
        let observed_flows = observed_flow_kinds.iter().copied().collect::<BTreeSet<_>>();
        let design_partner_proof_ready = required_flows.is_subset(&observed_flows)
            && first_useful_work_timing_count > 0
            && migration_funnel_event_count > 0;

        Self {
            observed_flow_kinds,
            distinct_entry_verbs,
            first_useful_work_timing_count,
            migration_funnel_event_count,
            design_partner_proof_ready,
        }
    }
}

/// Grouped onboarding task-success capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingTaskSuccessCaptureRecord {
    /// Schema token.
    pub schema: Cow<'static, str>,
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: Cow<'static, str>,
    /// Capture id.
    pub capture_id: String,
    /// Capture generated-at timestamp or fixture token.
    pub generated_at: String,
    /// Build identity.
    pub build: BuildIdentityRecord,
    /// Capture-level privacy envelope.
    pub privacy: PrivacyEnvelope,
    /// Event records in this capture.
    pub events: Vec<OnboardingTaskSuccessEventRecord>,
    /// Derived capture summary.
    pub summary: OnboardingTaskSuccessSummary,
    /// Capture-level evidence refs.
    pub evidence_refs: Vec<String>,
}

impl OnboardingTaskSuccessCaptureRecord {
    /// Validates that the capture can back design-partner onboarding proof.
    pub fn validate_design_partner_proof(&self) -> Result<(), OnboardingTelemetryValidationError> {
        self.privacy.validate()?;
        for event in &self.events {
            event.validate()?;
        }

        let recomputed_summary = OnboardingTaskSuccessSummary::from_events(&self.events);
        let observed_flows = self
            .events
            .iter()
            .map(|event| event.entry.flow_kind)
            .collect::<BTreeSet<_>>();
        for required in EntryFlowKind::REQUIRED_DESIGN_PARTNER_FLOWS {
            if !observed_flows.contains(&required) {
                return Err(OnboardingTelemetryValidationError::new(
                    "onboarding_task_success.flow_coverage.missing",
                    format!("missing required entry flow '{}'", required.as_str()),
                ));
            }
        }

        if recomputed_summary.first_useful_work_timing_count == 0 {
            return Err(OnboardingTelemetryValidationError::new(
                "onboarding_task_success.first_useful_work_timing.missing",
                "capture does not include first-useful-work timing",
            ));
        }

        if recomputed_summary.migration_funnel_event_count == 0 {
            return Err(OnboardingTelemetryValidationError::new(
                "onboarding_task_success.migration_funnel.missing",
                "capture does not include migration funnel metadata",
            ));
        }

        if !recomputed_summary.design_partner_proof_ready {
            return Err(OnboardingTelemetryValidationError::new(
                "onboarding_task_success.design_partner_proof.not_ready",
                "capture summary is not marked design-partner proof ready",
            ));
        }

        if self.summary != recomputed_summary {
            return Err(OnboardingTelemetryValidationError::new(
                "onboarding_task_success.capture_summary.stale",
                "capture summary does not match events",
            ));
        }

        Ok(())
    }
}

/// Validation error for onboarding telemetry records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingTelemetryValidationError {
    /// Stable validation check id.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
}

impl OnboardingTelemetryValidationError {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for OnboardingTelemetryValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.check_id, self.message)
    }
}

impl std::error::Error for OnboardingTelemetryValidationError {}

/// In-memory recorder for onboarding task-success telemetry.
#[derive(Debug)]
pub struct OnboardingTaskSuccessRecorder {
    context: OnboardingTelemetryContext,
    next_seq: u32,
    events: Vec<OnboardingTaskSuccessEventRecord>,
}

impl OnboardingTaskSuccessRecorder {
    /// Creates an empty recorder.
    pub fn new(context: OnboardingTelemetryContext) -> Self {
        Self {
            context,
            next_seq: 0,
            events: Vec::new(),
        }
    }

    /// Records one event after validating privacy-sensitive refs.
    pub fn record_event(
        &mut self,
        input: OnboardingEventInput,
    ) -> Result<(), OnboardingTelemetryValidationError> {
        validate_entry_refs(&input.entry)?;
        validate_migration_refs(input.migration_funnel.as_ref())?;
        validate_first_useful_work(input.first_useful_work.as_ref())?;

        let event = OnboardingTaskSuccessEventRecord {
            schema: Cow::Borrowed(SCHEMA),
            schema_version: SCHEMA_VERSION,
            record_kind: Cow::Borrowed(EVENT_RECORD_KIND),
            event_id: format!("event:onboarding-task-success-{}", self.next_seq),
            trace_id: self.context.trace_id.clone(),
            session_ref: self.context.session_ref.clone(),
            build: self.context.build.clone(),
            occurred_tick: input.occurred_tick,
            entry: input.entry,
            event_name: input.event_name,
            event_phase: input.event_phase,
            completion_checkpoint_class: input.completion_checkpoint_class,
            completion_class: input.completion_class,
            outcome_class: input.outcome_class,
            first_useful_work: input.first_useful_work,
            migration_funnel: input.migration_funnel,
            failure_category: input.failure_category,
            privacy: self.context.privacy.clone(),
            evidence_refs: input.evidence_refs,
        };
        event.validate()?;
        self.next_seq = self.next_seq.saturating_add(1);
        self.events.push(event);
        Ok(())
    }

    /// Returns a grouped capture over the recorded events.
    pub fn capture(
        &self,
        capture_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> OnboardingTaskSuccessCaptureRecord {
        let events = self.events.clone();
        let summary = OnboardingTaskSuccessSummary::from_events(&events);
        OnboardingTaskSuccessCaptureRecord {
            schema: Cow::Borrowed(SCHEMA),
            schema_version: SCHEMA_VERSION,
            record_kind: Cow::Borrowed(CAPTURE_RECORD_KIND),
            capture_id: capture_id.into(),
            generated_at: generated_at.into(),
            build: self.context.build.clone(),
            privacy: self.context.privacy.clone(),
            events,
            summary,
            evidence_refs: self.context.evidence_refs.clone(),
        }
    }

    /// Writes a grouped capture to disk.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the parent directory cannot be created, the
    /// capture cannot be serialized, or the destination cannot be written.
    pub fn write_capture(
        &self,
        path: impl AsRef<Path>,
        capture_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> io::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.capture(capture_id, generated_at))
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
        fs::write(path, format!("{json}\n"))
    }
}

impl OnboardingTaskSuccessEventRecord {
    fn validate(&self) -> Result<(), OnboardingTelemetryValidationError> {
        self.privacy.validate()?;
        validate_entry_refs(&self.entry)?;
        validate_migration_refs(self.migration_funnel.as_ref())?;
        validate_first_useful_work(self.first_useful_work.as_ref())?;
        Ok(())
    }
}

impl PrivacyEnvelope {
    fn validate(&self) -> Result<(), OnboardingTelemetryValidationError> {
        if self.contains_raw_project_content {
            return Err(OnboardingTelemetryValidationError::new(
                "onboarding_task_success.privacy.raw_project_content_present",
                "privacy envelope reports raw project content",
            ));
        }
        if !self
            .prohibited_content_classes
            .contains(&ProhibitedContentClass::RawProjectContent)
        {
            return Err(OnboardingTelemetryValidationError::new(
                "onboarding_task_success.privacy.raw_project_content_not_prohibited",
                "privacy envelope does not prohibit raw project content",
            ));
        }
        Ok(())
    }
}

fn validate_entry_refs(
    entry: &EntryFlowDescriptor,
) -> Result<(), OnboardingTelemetryValidationError> {
    if let Some(target_ref) = entry.target_ref.as_deref() {
        validate_opaque_ref(
            target_ref,
            "onboarding_task_success.privacy.target_ref_not_opaque",
            "entry target ref is not opaque",
        )?;
    }
    Ok(())
}

fn validate_migration_refs(
    migration: Option<&MigrationFunnelRecord>,
) -> Result<(), OnboardingTelemetryValidationError> {
    let Some(migration) = migration else {
        return Ok(());
    };
    validate_opaque_ref(
        &migration.migration_session_ref,
        "onboarding_task_success.privacy.migration_session_ref_not_opaque",
        "migration session ref is not opaque",
    )?;
    if let Some(checkpoint_ref) = migration.checkpoint_ref.as_deref() {
        validate_opaque_ref(
            checkpoint_ref,
            "onboarding_task_success.privacy.checkpoint_ref_not_opaque",
            "checkpoint ref is not opaque",
        )?;
    }
    if let Some(scorecard_ref) = migration.parity_scorecard_ref.as_deref() {
        validate_opaque_ref(
            scorecard_ref,
            "onboarding_task_success.privacy.parity_scorecard_ref_not_opaque",
            "parity scorecard ref is not opaque",
        )?;
    }
    if let Some(report_ref) = migration.migration_report_ref.as_deref() {
        validate_opaque_ref(
            report_ref,
            "onboarding_task_success.privacy.migration_report_ref_not_opaque",
            "migration report ref is not opaque",
        )?;
    }

    let total = migration.outcome_counts.imported
        + migration.outcome_counts.mapped
        + migration.outcome_counts.skipped
        + migration.outcome_counts.manual_review
        + migration.outcome_counts.bridge_required
        + migration.outcome_counts.unsupported;
    if total != migration.outcome_counts.total_items {
        return Err(OnboardingTelemetryValidationError::new(
            "onboarding_task_success.migration_funnel.total_mismatch",
            "migration outcome total does not match category counts",
        ));
    }
    if migration.step == MigrationFunnelStep::Applied
        && migration.rollback_state == RollbackState::Missing
    {
        return Err(OnboardingTelemetryValidationError::new(
            "onboarding_task_success.migration_funnel.apply_without_checkpoint",
            "migration apply event cannot report missing rollback state",
        ));
    }
    Ok(())
}

fn validate_first_useful_work(
    timing: Option<&FirstUsefulWorkTiming>,
) -> Result<(), OnboardingTelemetryValidationError> {
    let Some(timing) = timing else {
        return Ok(());
    };
    if timing.raw_project_content_captured {
        return Err(OnboardingTelemetryValidationError::new(
            "onboarding_task_success.first_useful_work.raw_content_present",
            "first-useful-work timing reports raw project content capture",
        ));
    }
    let expected = timing
        .reached_tick
        .saturating_sub(timing.intent_started_tick);
    if timing.duration_ticks != expected {
        return Err(OnboardingTelemetryValidationError::new(
            "onboarding_task_success.first_useful_work.duration_mismatch",
            "first-useful-work duration does not match monotonic ticks",
        ));
    }
    Ok(())
}

fn validate_opaque_ref(
    value: &str,
    check_id: &'static str,
    message: &'static str,
) -> Result<(), OnboardingTelemetryValidationError> {
    if is_opaque_ref(value) {
        Ok(())
    } else {
        Err(OnboardingTelemetryValidationError::new(
            check_id,
            format!("{message}: {value}"),
        ))
    }
}

fn is_opaque_ref(value: &str) -> bool {
    let Some((prefix, rest)) = value.split_once(':') else {
        return false;
    };
    if prefix.is_empty() || rest.is_empty() {
        return false;
    }
    if !prefix.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
    }) {
        return false;
    }
    rest.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'.'
            || byte == b'_'
            || byte == b'-'
    })
}

/// Builds the deterministic onboarding task-success design-partner fixture.
///
/// # Panics
///
/// Panics if a static fixture ref violates the privacy-safe opaque-ref rules.
pub fn seeded_design_partner_capture(
    generated_at: impl Into<String>,
) -> OnboardingTaskSuccessCaptureRecord {
    let build = BuildIdentityRecord {
        crate_name: "aureline-telemetry".to_string(),
        crate_version: "0.0.0".to_string(),
        rustc_target_triple: "fixture-target".to_string(),
    };
    let mut context = OnboardingTelemetryContext::developer_local(
        "trace:onboarding-task-success-design-partner",
        "session:onboarding-task-success-alpha",
        build,
    );
    context.privacy.export_posture = TelemetryExportPosture::DesignPartnerOptIn;
    let mut recorder = OnboardingTaskSuccessRecorder::new(context);

    recorder
        .record_event(OnboardingEventInput {
            entry: entry(
                EntryFlowKind::FirstRun,
                EntryVerbKind::OpenFolder,
                EntryRouteId::StartCenter,
                MeasurementSurface::SurfaceFirstRun,
                TargetKind::LocalFolder,
                Some("target:first-run-local-folder"),
            ),
            event_name: OnboardingEventName::FirstUsefulNavigationReached,
            event_phase: OnboardingEventPhase::UsefulWork,
            completion_checkpoint_class: Some(CompletionCheckpointClass::FirstUsefulNavigation),
            completion_class: Some(CompletionClass::CompletedFirstUsefulNavigationOnly),
            outcome_class: OutcomeClass::Completed,
            first_useful_work: Some(FirstUsefulWorkTiming::new(
                0,
                42,
                FirstUsefulWorkTargetSurface::TreePlusReadmeOrChangedFiles,
                SemanticWarmupState::BeforeSemanticWarmup,
            )),
            migration_funnel: None,
            failure_category: None,
            evidence_refs: vec![
                "fixtures/ux/first_useful_work_cases/first_run_start_center_local_folder.yaml"
                    .to_string(),
            ],
            occurred_tick: 42,
        })
        .expect("fixture first-run event is valid");

    recorder
        .record_event(OnboardingEventInput {
            entry: entry(
                EntryFlowKind::Open,
                EntryVerbKind::OpenWorkspace,
                EntryRouteId::PlainOpen,
                MeasurementSurface::SurfaceFirstOpen,
                TargetKind::WorkspaceFile,
                Some("target:plain-open-workspace"),
            ),
            event_name: OnboardingEventName::FirstOpenCompleted,
            event_phase: OnboardingEventPhase::Completion,
            completion_checkpoint_class: Some(CompletionCheckpointClass::FirstUsefulEdit),
            completion_class: Some(CompletionClass::CompletedFirstUsefulEdit),
            outcome_class: OutcomeClass::Completed,
            first_useful_work: Some(FirstUsefulWorkTiming::new(
                50,
                88,
                FirstUsefulWorkTargetSurface::TreePlusReadmeOrChangedFiles,
                SemanticWarmupState::BeforeSemanticWarmup,
            )),
            migration_funnel: None,
            failure_category: None,
            evidence_refs: vec![
                "fixtures/ux/first_useful_work_cases/plain_open_unknown_archetype.yaml".to_string(),
            ],
            occurred_tick: 88,
        })
        .expect("fixture open event is valid");

    recorder
        .record_event(OnboardingEventInput {
            entry: entry(
                EntryFlowKind::Clone,
                EntryVerbKind::CloneRepository,
                EntryRouteId::CloneOrImport,
                MeasurementSurface::SurfaceFirstOpen,
                TargetKind::RemoteWorkspace,
                Some("target:clone-remote-repository"),
            ),
            event_name: OnboardingEventName::FirstOpenCompleted,
            event_phase: OnboardingEventPhase::Completion,
            completion_checkpoint_class: Some(CompletionCheckpointClass::FirstUsefulNavigation),
            completion_class: Some(CompletionClass::CompletedFirstUsefulNavigationOnly),
            outcome_class: OutcomeClass::Completed,
            first_useful_work: Some(FirstUsefulWorkTiming::new(
                90,
                141,
                FirstUsefulWorkTargetSurface::PostCloneHandoff,
                SemanticWarmupState::BeforeSemanticWarmup,
            )),
            migration_funnel: None,
            failure_category: None,
            evidence_refs: vec![
                "fixtures/ux/first_useful_work_cases/clone_then_review_remote_repo.yaml"
                    .to_string(),
            ],
            occurred_tick: 141,
        })
        .expect("fixture clone event is valid");

    let migration_counts = MigrationOutcomeCounts::new(3, 6, 1, 2, 1, 1);
    recorder
        .record_event(OnboardingEventInput {
            entry: entry(
                EntryFlowKind::Import,
                EntryVerbKind::ImportFromExternal,
                EntryRouteId::CloneOrImport,
                MeasurementSurface::SurfaceMigrationReview,
                TargetKind::ImportPacket,
                Some("target:vs-code-settings-import"),
            ),
            event_name: OnboardingEventName::MigrationOutcomeRecorded,
            event_phase: OnboardingEventPhase::MigrationReview,
            completion_checkpoint_class: Some(
                CompletionCheckpointClass::MigrationCommittedWithPerItemOutcomes,
            ),
            completion_class: Some(CompletionClass::CompletedMigrationCommittedPerItem),
            outcome_class: OutcomeClass::Completed,
            first_useful_work: Some(FirstUsefulWorkTiming::new(
                150,
                177,
                FirstUsefulWorkTargetSurface::MigrationCenterBeforeCommit,
                SemanticWarmupState::SemanticWarmupNotApplicable,
            )),
            migration_funnel: Some(MigrationFunnelRecord {
                migration_session_ref: "migration-session:vs-code-settings-alpha".to_string(),
                source_kind: MigrationSourceKind::VsCode,
                step: MigrationFunnelStep::PerItemOutcomesRecorded,
                dry_run_state: DryRunState::Produced,
                rollback_state: RollbackState::Available,
                checkpoint_ref: Some("checkpoint:vs-code-settings-pre-apply".to_string()),
                outcome_counts: migration_counts,
                parity_scorecard_ref: Some("parity-scorecard:vs-code-settings-alpha".to_string()),
                migration_report_ref: Some("migration-report:vs-code-settings-alpha".to_string()),
            }),
            failure_category: None,
            evidence_refs: vec![
                "fixtures/ux/first_useful_work_cases/import_vs_code_settings_dry_run.yaml"
                    .to_string(),
                "schemas/migration/importer_outcome.schema.json".to_string(),
            ],
            occurred_tick: 177,
        })
        .expect("fixture import event is valid");

    recorder
        .record_event(OnboardingEventInput {
            entry: entry(
                EntryFlowKind::Restore,
                EntryVerbKind::RestoreLastSession,
                EntryRouteId::RestorePrompt,
                MeasurementSurface::SurfaceRestoreSuccess,
                TargetKind::WorkspaceFile,
                Some("target:restore-compatible-session"),
            ),
            event_name: OnboardingEventName::RestoreCompleted,
            event_phase: OnboardingEventPhase::Restore,
            completion_checkpoint_class: Some(CompletionCheckpointClass::RestoreLevelDelivered),
            completion_class: Some(CompletionClass::CompletedRestoreLevelDelivered),
            outcome_class: OutcomeClass::Restored,
            first_useful_work: Some(FirstUsefulWorkTiming::new(
                180,
                211,
                FirstUsefulWorkTargetSurface::PriorLayoutWithPlaceholders,
                SemanticWarmupState::SemanticWarmupNotApplicable,
            )),
            migration_funnel: None,
            failure_category: None,
            evidence_refs: vec![
                "fixtures/ux/first_useful_work_cases/compatible_restore_after_crash.yaml"
                    .to_string(),
            ],
            occurred_tick: 211,
        })
        .expect("fixture restore event is valid");

    recorder
        .record_event(OnboardingEventInput {
            entry: entry(
                EntryFlowKind::Reconnect,
                EntryVerbKind::ReconnectTarget,
                EntryRouteId::WarmStart,
                MeasurementSurface::SurfaceReconnect,
                TargetKind::ManagedWorkspace,
                Some("target:managed-workspace-reconnect"),
            ),
            event_name: OnboardingEventName::ReconnectCompleted,
            event_phase: OnboardingEventPhase::Reconnect,
            completion_checkpoint_class: Some(CompletionCheckpointClass::ReconnectDelivered),
            completion_class: Some(CompletionClass::CompletedReconnectDelivered),
            outcome_class: OutcomeClass::Completed,
            first_useful_work: Some(FirstUsefulWorkTiming::new(
                220,
                249,
                FirstUsefulWorkTargetSurface::ReviewOrWorkItemAnchor,
                SemanticWarmupState::SemanticWarmupNotApplicable,
            )),
            migration_funnel: None,
            failure_category: None,
            evidence_refs: vec![
                "fixtures/ux/first_useful_work_cases/managed_cloud_resume_reauth_required_declined.yaml"
                    .to_string(),
            ],
            occurred_tick: 249,
        })
        .expect("fixture reconnect event is valid");

    recorder.capture(
        "capture:onboarding-task-success-design-partner",
        generated_at,
    )
}

fn entry(
    flow_kind: EntryFlowKind,
    entry_verb: EntryVerbKind,
    entry_route_id: EntryRouteId,
    measurement_surface: MeasurementSurface,
    target_kind: TargetKind,
    target_ref: Option<&str>,
) -> EntryFlowDescriptor {
    EntryFlowDescriptor {
        flow_kind,
        entry_verb,
        entry_route_id,
        measurement_surface,
        target_kind,
        target_ref: target_ref.map(str::to_string),
        deployment_profile_id: "deployment_profile:individual-local-alpha".to_string(),
    }
}
