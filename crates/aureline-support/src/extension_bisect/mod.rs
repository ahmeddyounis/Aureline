//! Extension-bisect orchestration session, step, finding, restore, and support packet.
//!
//! The extension-bisect rung is the recovery posture a blocked user enters
//! when an extension regression is suspected (typically after a startup
//! crash loop or an exhausted restart budget) and the user wants to
//! attribute the offender by progressively activating cohorts rather than
//! disabling everything by hand. Every cohort activation result, suspected
//! extension set, and user-visible finding is recorded as a typed row; the
//! bisect leaves the prior extension state restorable so the user (or
//! managed policy) can always return to the state they were in before the
//! bisect began.
//!
//! This module mints four typed records that mirror the boundary schema at
//! [`/schemas/support/extension_bisect.schema.json`]:
//!
//! - [`ExtensionBisectSession`] declares the active session as a typed
//!   list of [`CandidateExtension`], [`TestedStateRow`],
//!   [`SuspectedExtensionSet`], and [`UserVisibleFinding`] rows, plus the
//!   declared [`DisabledCapabilityClass`] and [`PreservedCapabilityClass`]
//!   sets and the [`RestorePlan`] back to the prior extension state.
//! - [`ExtensionBisectStep`] records one cohort activation/verification
//!   step and pins `user_owned_state_deleted = false` and
//!   `durable_state_deleted = false`.
//! - [`ExtensionBisectFinding`] is the user-visible attribution row a
//!   bisect session produces. The evaluator refuses a finding that names
//!   single- or multi-extension suspicion without naming the suspect
//!   extension refs.
//! - [`ExtensionBisectRestore`] records the explicit return of the prior
//!   extension state. The evaluator refuses a restore record that deletes
//!   user-owned or durable state, or that drops the
//!   `user_authored_files` preservation.
//!
//! [`ExtensionBisectEvaluator::support_packet`] folds one session, its
//! steps, finding, and restore into an
//! [`ExtensionBisectSupportPacket`] that the support-export pipeline can
//! consume verbatim. The packet is metadata-only: it cites ids, refs, and
//! closed-vocabulary tokens, never raw payloads, credentials, paths, or
//! ambient authority.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for an extension-bisect session record.
pub const EXTENSION_BISECT_SESSION_RECORD_KIND: &str = "extension_bisect_session_record";

/// Stable record-kind tag for an extension-bisect step record.
pub const EXTENSION_BISECT_STEP_RECORD_KIND: &str = "extension_bisect_step_record";

/// Stable record-kind tag for an extension-bisect finding record.
pub const EXTENSION_BISECT_FINDING_RECORD_KIND: &str = "extension_bisect_finding_record";

/// Stable record-kind tag for an extension-bisect restore record.
pub const EXTENSION_BISECT_RESTORE_RECORD_KIND: &str = "extension_bisect_restore_record";

/// Stable record-kind tag for the metadata-safe support projection.
pub const EXTENSION_BISECT_SUPPORT_PACKET_RECORD_KIND: &str =
    "extension_bisect_support_packet_record";

/// Frozen schema version for extension-bisect beta records.
pub const EXTENSION_BISECT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const EXTENSION_BISECT_SCHEMA_REF: &str = "schemas/support/extension_bisect.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const EXTENSION_BISECT_DOC_REF: &str = "docs/support/m3/extension_bisect_beta.md";

/// Closed session-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BisectSessionClass {
    /// Session was entered after a startup crash loop was attributed to
    /// extensions.
    PostCrashLoopSession,
    /// Session was entered because an extension regression was suspected.
    RegressionSuspectedSession,
    /// User explicitly chose to bisect from a recovery surface.
    UserInvokedSession,
    /// Managed policy or admin override forced the bisect.
    PolicyForcedSession,
}

impl BisectSessionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostCrashLoopSession => "post_crash_loop_session",
            Self::RegressionSuspectedSession => "regression_suspected_session",
            Self::UserInvokedSession => "user_invoked_session",
            Self::PolicyForcedSession => "policy_forced_session",
        }
    }
}

/// Closed entry-reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BisectEntryReasonClass {
    /// Startup crash loop exceeded the configured strike budget.
    CrashLoopDetected,
    /// Project Doctor or runtime supervisor suspected an extension regression.
    ExtensionRegressionSuspected,
    /// Extension lane exceeded its restart budget.
    ExtensionRestartBudgetExceeded,
    /// User explicitly chose the bisect rung.
    ExplicitUserChoice,
    /// Managed policy forced the bisect.
    PolicyForced,
    /// Manual review surfaced the bisect as a required step.
    ManualReviewRequired,
}

impl BisectEntryReasonClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashLoopDetected => "crash_loop_detected",
            Self::ExtensionRegressionSuspected => "extension_regression_suspected",
            Self::ExtensionRestartBudgetExceeded => "extension_restart_budget_exceeded",
            Self::ExplicitUserChoice => "explicit_user_choice",
            Self::PolicyForced => "policy_forced",
            Self::ManualReviewRequired => "manual_review_required",
        }
    }
}

/// Closed blast-radius vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlastRadiusClass {
    /// Only a single extension lane is touched.
    SingleExtensionLane,
    /// Only the active cohort is touched.
    CohortOnly,
    /// All third-party extensions are temporarily deactivated.
    AllThirdPartyExtensions,
    /// The whole extension host is taken offline.
    FullExtensionHost,
}

impl BlastRadiusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleExtensionLane => "single_extension_lane",
            Self::CohortOnly => "cohort_only",
            Self::AllThirdPartyExtensions => "all_third_party_extensions",
            Self::FullExtensionHost => "full_extension_host",
        }
    }
}

/// Closed prior-state vocabulary describing what an extension was before
/// the bisect began.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorStateClass {
    /// Extension was enabled before the bisect.
    Enabled,
    /// Extension was disabled before the bisect.
    Disabled,
    /// Extension was quarantined before the bisect.
    Quarantined,
    /// Extension was not installed before the bisect.
    NotInstalled,
}

impl PriorStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Quarantined => "quarantined",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Closed extension-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionClass {
    /// First-party extension shipped with the repository.
    FirstPartyRepoOwned,
    /// First-party extension delivered through the marketplace.
    FirstPartyMarketplace,
    /// Third-party marketplace extension.
    ThirdPartyMarketplace,
    /// Third-party extension delivered as an offline bundle.
    ThirdPartyOfflineBundle,
}

impl ExtensionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyRepoOwned => "first_party_repo_owned",
            Self::FirstPartyMarketplace => "first_party_marketplace",
            Self::ThirdPartyMarketplace => "third_party_marketplace",
            Self::ThirdPartyOfflineBundle => "third_party_offline_bundle",
        }
    }
}

/// Closed disabled-capability vocabulary for a bisect session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisabledCapabilityClass {
    /// Third-party extension auto-activation.
    ExtensionAutoActivation,
    /// Extension host launch.
    ExtensionHostLaunch,
    /// Extension marketplace sync.
    ExtensionMarketplaceSync,
    /// Session restore auto-replay.
    SessionRestoreAutoReplay,
    /// Remote helper attach.
    RemoteHelperAttach,
    /// AI runtime access.
    AiRuntimeAccess,
    /// Background rebuild.
    BackgroundRebuild,
}

impl DisabledCapabilityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionAutoActivation => "extension_auto_activation",
            Self::ExtensionHostLaunch => "extension_host_launch",
            Self::ExtensionMarketplaceSync => "extension_marketplace_sync",
            Self::SessionRestoreAutoReplay => "session_restore_auto_replay",
            Self::RemoteHelperAttach => "remote_helper_attach",
            Self::AiRuntimeAccess => "ai_runtime_access",
            Self::BackgroundRebuild => "background_rebuild",
        }
    }
}

/// Closed preserved-capability vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedCapabilityClass {
    /// Local editing of user-authored files.
    LocalEditing,
    /// Basic navigation (file tree, quick-open, go-to-definition for local files).
    BasicNavigation,
    /// Local search.
    LocalSearch,
    /// Local Git operations.
    LocalGitOperations,
    /// Local diagnostics export and support-bundle preview.
    LocalDiagnosticsExport,
    /// Support-bundle preview surface.
    SupportBundlePreview,
    /// Project Doctor surfaces remain reachable.
    ProjectDoctorSurfaces,
    /// Explicit safe-mode exit action remains reachable.
    SafeModeExitAction,
    /// Explicit bisect exit / restore action remains reachable.
    ExtensionBisectExitAction,
}

impl PreservedCapabilityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditing => "local_editing",
            Self::BasicNavigation => "basic_navigation",
            Self::LocalSearch => "local_search",
            Self::LocalGitOperations => "local_git_operations",
            Self::LocalDiagnosticsExport => "local_diagnostics_export",
            Self::SupportBundlePreview => "support_bundle_preview",
            Self::ProjectDoctorSurfaces => "project_doctor_surfaces",
            Self::SafeModeExitAction => "safe_mode_exit_action",
            Self::ExtensionBisectExitAction => "extension_bisect_exit_action",
        }
    }
}

/// Closed preserved-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Selection, caret, and scroll state for open buffers.
    OpenBufferSelection,
    /// Durable workspace indexes that must not be deleted as collateral.
    DurableWorkspaceIndexes,
    /// Workspace trust state.
    WorkspaceTrustStore,
    /// Credential handles and stores.
    CredentialStore,
    /// Session restore records.
    SessionRestoreStore,
    /// Support export records and staging state.
    SupportExportStore,
    /// Snapshot of the prior extension state used for restore.
    ExtensionPriorStateSnapshot,
}

impl PreservedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredFiles => "user_authored_files",
            Self::OpenBufferSelection => "open_buffer_selection",
            Self::DurableWorkspaceIndexes => "durable_workspace_indexes",
            Self::WorkspaceTrustStore => "workspace_trust_store",
            Self::CredentialStore => "credential_store",
            Self::SessionRestoreStore => "session_restore_store",
            Self::SupportExportStore => "support_export_store",
            Self::ExtensionPriorStateSnapshot => "extension_prior_state_snapshot",
        }
    }
}

/// Closed step-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepClass {
    /// Baseline check with no extension cohort active.
    InitialBaselineCheck,
    /// One cohort of extensions is activated.
    CohortActivation,
    /// A repeat verification of a cohort verdict.
    CohortVerification,
    /// Exit baseline confirming the prior state has been restored.
    ExitBaselineCheck,
}

impl StepClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialBaselineCheck => "initial_baseline_check",
            Self::CohortActivation => "cohort_activation",
            Self::CohortVerification => "cohort_verification",
            Self::ExitBaselineCheck => "exit_baseline_check",
        }
    }
}

/// Closed cohort-activation-result vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortActivationResultClass {
    /// Cohort activation reproduces the failure.
    ReproducesFailure,
    /// Cohort activation does not reproduce the failure.
    NoFailureObserved,
    /// Cohort activation is inconclusive.
    InconclusiveEvidence,
    /// Cohort activation was aborted by the user.
    AbortedByUser,
    /// Cohort activation was aborted by policy.
    AbortedByPolicy,
}

impl CohortActivationResultClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReproducesFailure => "reproduces_failure",
            Self::NoFailureObserved => "no_failure_observed",
            Self::InconclusiveEvidence => "inconclusive_evidence",
            Self::AbortedByUser => "aborted_by_user",
            Self::AbortedByPolicy => "aborted_by_policy",
        }
    }
}

/// Closed step-verdict vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepVerdictClass {
    /// Cohort narrowed the suspect set further.
    CohortSuspectNarrowed,
    /// Cohort was cleared of suspicion.
    CohortCleared,
    /// Cohort outcome remained inconclusive.
    CohortInconclusive,
    /// Cohort run was aborted.
    CohortRunAborted,
}

impl StepVerdictClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CohortSuspectNarrowed => "cohort_suspect_narrowed",
            Self::CohortCleared => "cohort_cleared",
            Self::CohortInconclusive => "cohort_inconclusive",
            Self::CohortRunAborted => "cohort_run_aborted",
        }
    }
}

/// Closed suspect-confidence vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspectConfidenceClass {
    /// One extension has been isolated.
    SingleSuspectIsolated,
    /// A narrow set of extensions remains suspected.
    NarrowSetSuspected,
    /// A broad set of extensions remains suspected.
    BroadSetSuspected,
    /// No suspect could be identified.
    NoSuspectIdentified,
}

impl SuspectConfidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSuspectIsolated => "single_suspect_isolated",
            Self::NarrowSetSuspected => "narrow_set_suspected",
            Self::BroadSetSuspected => "broad_set_suspected",
            Self::NoSuspectIdentified => "no_suspect_identified",
        }
    }
}

/// Closed finding-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingClass {
    /// A single extension was attributed.
    SingleExtensionSuspected,
    /// Multiple extensions remain under suspicion.
    MultiExtensionSuspected,
    /// No extension was attributed.
    NoExtensionSuspected,
    /// Evidence points to an environmental factor outside the extension lane.
    EnvironmentalFactorSuspected,
    /// Bisect was aborted before producing a finding.
    BisectAbortedNoFinding,
}

impl FindingClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleExtensionSuspected => "single_extension_suspected",
            Self::MultiExtensionSuspected => "multi_extension_suspected",
            Self::NoExtensionSuspected => "no_extension_suspected",
            Self::EnvironmentalFactorSuspected => "environmental_factor_suspected",
            Self::BisectAbortedNoFinding => "bisect_aborted_no_finding",
        }
    }

    /// Returns true when this finding class must name at least one suspect ref.
    pub fn requires_suspect_refs(self) -> bool {
        matches!(
            self,
            Self::SingleExtensionSuspected | Self::MultiExtensionSuspected
        )
    }
}

/// Closed restore-disposition vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreDispositionClass {
    /// Prior state was restored bit-for-bit.
    PriorStateRestoredExact,
    /// Prior state was restored except the attributed suspect remains
    /// quarantined.
    PriorStateRestoredWithQuarantine,
    /// Prior state never changed (no cohort activations performed).
    PriorStateUnchanged,
    /// Restore was deferred pending user or admin review.
    RestoreDeferredPendingReview,
}

impl RestoreDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorStateRestoredExact => "prior_state_restored_exact",
            Self::PriorStateRestoredWithQuarantine => "prior_state_restored_with_quarantine",
            Self::PriorStateUnchanged => "prior_state_unchanged",
            Self::RestoreDeferredPendingReview => "restore_deferred_pending_review",
        }
    }

    /// Returns true when restore is considered admissible without further
    /// review.
    pub fn is_admissible(self) -> bool {
        matches!(
            self,
            Self::PriorStateRestoredExact
                | Self::PriorStateRestoredWithQuarantine
                | Self::PriorStateUnchanged
        )
    }
}

/// Closed review-gate vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewGateClass {
    /// User confirmation is required before each cohort transition.
    UserConfirmationRequired,
    /// Admin-signed review is required before each cohort transition.
    AdminSignedReviewRequired,
    /// No explicit review gate (used only for read-only diagnostics).
    NoReviewRequired,
}

impl ReviewGateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserConfirmationRequired => "user_confirmation_required",
            Self::AdminSignedReviewRequired => "admin_signed_review_required",
            Self::NoReviewRequired => "no_review_required",
        }
    }
}

/// Closed escalation-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationActionClass {
    /// Escalate the finding to an extension-quarantine rung.
    EscalateToExtensionQuarantine,
    /// Escalate the finding to safe mode.
    EscalateToSafeMode,
    /// Escalate the finding to a rollback / reinstall candidate.
    EscalateToRollbackReinstallCandidate,
    /// Export an escalation packet for support review.
    ExportEscalationPacket,
}

impl EscalationActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EscalateToExtensionQuarantine => "escalate_to_extension_quarantine",
            Self::EscalateToSafeMode => "escalate_to_safe_mode",
            Self::EscalateToRollbackReinstallCandidate => {
                "escalate_to_rollback_reinstall_candidate"
            }
            Self::ExportEscalationPacket => "export_escalation_packet",
        }
    }
}

/// One candidate extension in the bisect session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateExtension {
    /// Opaque extension identifier safe for support and release packets.
    pub extension_id: String,
    /// Extension class.
    pub extension_class: ExtensionClass,
    /// Prior state class before the bisect started.
    pub prior_state_class: PriorStateClass,
    /// Reviewer-safe display label that excludes raw paths and private content.
    pub display_label: String,
}

/// One tested-state row in the bisect log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestedStateRow {
    /// Stable tested-state identifier.
    pub tested_state_id: String,
    /// Cohort identifier that this tested state evaluated.
    pub cohort_id: String,
    /// Cohort member refs that were active during the tested state.
    pub cohort_member_refs: Vec<String>,
    /// Outcome of the cohort activation.
    pub activation_result_class: CohortActivationResultClass,
    /// Reviewer-safe summary of what the user observed.
    pub reviewer_summary: String,
}

/// One suspected-extension-set row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspectedExtensionSet {
    /// Stable suspect-set identifier.
    pub suspect_set_id: String,
    /// Confidence class for the suspect set.
    pub confidence_class: SuspectConfidenceClass,
    /// Member extension refs that remain under suspicion (empty when no
    /// suspect was identified).
    pub member_extension_refs: Vec<String>,
    /// Reviewer-safe narrowing summary.
    pub narrowing_summary: String,
}

/// One user-visible finding row attached to the session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserVisibleFinding {
    /// Stable finding identifier.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: FindingClass,
    /// Reviewer-safe user-visible summary.
    pub user_visible_summary: String,
}

/// Action surfaced to leave the bisect and return the prior extension state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreAction {
    /// Stable action identifier.
    pub action_id: String,
    /// Whether review is required before execution.
    pub requires_review: bool,
    /// Reviewer-safe summary of what the action does.
    pub summary: String,
}

/// Restore plan that returns the user to the prior extension state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePlan {
    /// Opaque snapshot ref describing the prior extension state.
    pub prior_state_snapshot_ref: String,
    /// Explicit action used to leave the bisect.
    pub restore_action: RestoreAction,
    /// Conditions that must hold before fully restoring the prior state.
    pub restore_conditions: Vec<String>,
}

/// One restored-extension row in the restore record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoredExtensionRow {
    /// Opaque extension identifier.
    pub extension_id: String,
    /// State the extension held before the bisect.
    pub prior_state_class: PriorStateClass,
    /// State the extension holds after the restore.
    pub restored_state_class: PriorStateClass,
    /// Reviewer-safe summary of the restoration.
    pub reviewer_summary: String,
}

/// Redaction-safe evidence ref carried by sessions and findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectEvidenceRef {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Evidence kind or source role.
    pub evidence_kind: String,
    /// Reviewer-safe summary without raw private content.
    pub summary: String,
}

/// Extension-bisect session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBisectSession {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable session identifier.
    pub session_id: String,
    /// Session class.
    pub session_class: BisectSessionClass,
    /// Entry reason class.
    pub entry_reason_class: BisectEntryReasonClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Project Doctor finding that justified the bisect.
    pub doctor_finding_ref: String,
    /// Support packet ref that consumes the session.
    pub support_packet_ref: String,
    /// Declared blast radius for the bisect.
    pub blast_radius_class: BlastRadiusClass,
    /// Review gate applied at cohort transitions.
    pub review_gate_class: ReviewGateClass,
    /// Candidate extensions evaluated by the bisect.
    pub candidate_extensions: Vec<CandidateExtension>,
    /// Disabled capability classes for the duration of the bisect.
    pub disabled_capability_classes: Vec<DisabledCapabilityClass>,
    /// Preserved capability classes (must include local editing).
    pub preserved_capability_classes: Vec<PreservedCapabilityClass>,
    /// Preserved state classes (must include user-authored files).
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Log of every tested state.
    pub tested_states: Vec<TestedStateRow>,
    /// Suspected extension sets surfaced during the bisect.
    pub suspected_extension_sets: Vec<SuspectedExtensionSet>,
    /// User-visible findings produced by the bisect.
    pub findings: Vec<UserVisibleFinding>,
    /// Restore plan returning the prior extension state.
    pub restore_plan: RestorePlan,
    /// Evidence refs justifying the session.
    pub evidence: Vec<BisectEvidenceRef>,
    /// Whether the session carries any destructive reset.
    pub destructive_resets_present: bool,
    /// Whether the session deletes user-owned state.
    pub user_owned_state_deleted: bool,
    /// Whether the session deletes durable non-disposable state.
    pub durable_state_deleted: bool,
}

/// Extension-bisect step record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBisectStep {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable step identifier.
    pub step_id: String,
    /// Step class.
    pub step_class: StepClass,
    /// Session id this step binds to.
    pub session_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Cohort identifier for the step.
    pub cohort_id: String,
    /// Cohort member refs active during the step.
    pub cohort_member_refs: Vec<String>,
    /// Outcome of the activation.
    pub activation_result_class: CohortActivationResultClass,
    /// Verdict the step produced for the cohort.
    pub verdict_class: StepVerdictClass,
    /// Reviewer-safe summary.
    pub reviewer_summary: String,
    /// Evidence refs cited by the step.
    pub evidence_refs: Vec<String>,
    /// Whether the step deleted any user-owned state.
    pub user_owned_state_deleted: bool,
    /// Whether the step deleted any durable non-disposable state.
    pub durable_state_deleted: bool,
}

/// Extension-bisect finding record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBisectFinding {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable finding identifier.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: FindingClass,
    /// Session id this finding binds to.
    pub session_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Suspect extension refs (must be non-empty when the class requires
    /// at least one suspect ref).
    pub suspect_extension_refs: Vec<String>,
    /// Reviewer-safe user-visible summary.
    pub user_visible_summary: String,
    /// Support packet ref that consumes the finding.
    pub support_packet_ref: String,
    /// Escalation actions the chrome surfaces for this finding.
    pub escalation_actions: Vec<EscalationActionClass>,
}

/// Extension-bisect restore record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBisectRestore {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable restore identifier.
    pub restore_id: String,
    /// Session id this restore binds to.
    pub session_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Restore disposition.
    pub restore_disposition_class: RestoreDispositionClass,
    /// Opaque snapshot ref describing the prior extension state.
    pub prior_state_snapshot_ref: String,
    /// Extensions restored by the record.
    pub restored_extensions: Vec<RestoredExtensionRow>,
    /// Preserved state classes observed at the restore boundary.
    pub preserved_state_classes_observed: Vec<PreservedStateClass>,
    /// Whether the restore deleted any user-owned state.
    pub user_owned_state_deleted: bool,
    /// Whether the restore deleted any durable non-disposable state.
    pub durable_state_deleted: bool,
    /// Support packet ref that consumes the restore.
    pub support_packet_ref: String,
}

/// Metadata-safe support projection joining one session, its steps, finding,
/// and restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBisectSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Doc ref the packet quotes.
    pub doc_ref: String,
    /// Boundary schema ref the packet mirrors.
    pub schema_ref: String,
    /// Session id projected by the packet.
    pub session_id: String,
    /// Session class projected by the packet.
    pub session_class: BisectSessionClass,
    /// Entry reason class projected by the packet.
    pub entry_reason_class: BisectEntryReasonClass,
    /// Project Doctor finding ref the packet cites.
    pub doctor_finding_ref: String,
    /// Blast-radius class for the session.
    pub blast_radius_class: BlastRadiusClass,
    /// Review-gate class for the session.
    pub review_gate_class: ReviewGateClass,
    /// Candidate extension rows (opaque identifiers and classes only).
    pub candidate_rows: Vec<BisectSupportCandidateRow>,
    /// Disabled capability classes.
    pub disabled_capability_classes: Vec<DisabledCapabilityClass>,
    /// Preserved capability classes.
    pub preserved_capability_classes: Vec<PreservedCapabilityClass>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Tested state rows.
    pub tested_state_rows: Vec<BisectSupportTestedStateRow>,
    /// Suspected extension set rows.
    pub suspect_set_rows: Vec<BisectSupportSuspectSetRow>,
    /// User-visible finding rows.
    pub user_finding_rows: Vec<BisectSupportFindingRow>,
    /// Step rows.
    pub step_rows: Vec<BisectSupportStepRow>,
    /// Restore row.
    pub restore_row: BisectSupportRestoreRow,
    /// Evidence refs cited by the packet.
    pub evidence_refs: Vec<String>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether the session declared any destructive reset.
    pub destructive_resets_present: bool,
}

impl ExtensionBisectSupportPacket {
    /// Returns true when the packet preserves the bounded bisect contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_resets_present
            && self
                .preserved_capability_classes
                .contains(&PreservedCapabilityClass::LocalEditing)
            && self
                .preserved_state_classes
                .contains(&PreservedStateClass::UserAuthoredFiles)
            && !self.candidate_rows.is_empty()
            && !self.disabled_capability_classes.is_empty()
            && !self.evidence_refs.is_empty()
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && self.restore_row.is_export_safe()
            && self
                .step_rows
                .iter()
                .all(BisectSupportStepRow::is_export_safe)
    }
}

/// One candidate-extension row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectSupportCandidateRow {
    /// Opaque extension identifier.
    pub extension_id: String,
    /// Extension class.
    pub extension_class: ExtensionClass,
    /// Prior state class.
    pub prior_state_class: PriorStateClass,
    /// Reviewer-safe display label.
    pub display_label: String,
}

/// One tested-state row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectSupportTestedStateRow {
    /// Stable tested-state identifier.
    pub tested_state_id: String,
    /// Cohort identifier.
    pub cohort_id: String,
    /// Cohort members.
    pub cohort_member_refs: Vec<String>,
    /// Activation result class.
    pub activation_result_class: CohortActivationResultClass,
    /// Reviewer-safe summary.
    pub reviewer_summary: String,
}

/// One suspect-set row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectSupportSuspectSetRow {
    /// Stable suspect-set identifier.
    pub suspect_set_id: String,
    /// Confidence class.
    pub confidence_class: SuspectConfidenceClass,
    /// Member extension refs.
    pub member_extension_refs: Vec<String>,
    /// Reviewer-safe narrowing summary.
    pub narrowing_summary: String,
}

/// One user-visible finding row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectSupportFindingRow {
    /// Stable finding identifier.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: FindingClass,
    /// Reviewer-safe user-visible summary.
    pub user_visible_summary: String,
    /// Suspect extension refs.
    pub suspect_extension_refs: Vec<String>,
    /// Escalation actions for the finding.
    pub escalation_actions: Vec<EscalationActionClass>,
}

/// One step row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectSupportStepRow {
    /// Stable step identifier.
    pub step_id: String,
    /// Step class.
    pub step_class: StepClass,
    /// Cohort identifier.
    pub cohort_id: String,
    /// Cohort members.
    pub cohort_member_refs: Vec<String>,
    /// Activation result class.
    pub activation_result_class: CohortActivationResultClass,
    /// Verdict class.
    pub verdict_class: StepVerdictClass,
    /// Reviewer-safe summary.
    pub reviewer_summary: String,
    /// Whether user-owned state was deleted.
    pub user_owned_state_deleted: bool,
    /// Whether durable state was deleted.
    pub durable_state_deleted: bool,
}

impl BisectSupportStepRow {
    /// Returns true when this step row preserves the contract.
    pub fn is_export_safe(&self) -> bool {
        !self.user_owned_state_deleted && !self.durable_state_deleted
    }
}

/// One restore row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BisectSupportRestoreRow {
    /// Stable restore identifier.
    pub restore_id: String,
    /// Restore disposition class.
    pub restore_disposition_class: RestoreDispositionClass,
    /// Opaque prior-state snapshot ref.
    pub prior_state_snapshot_ref: String,
    /// Restored extension rows.
    pub restored_extensions: Vec<RestoredExtensionRow>,
    /// Preserved state classes observed at the boundary.
    pub preserved_state_classes_observed: Vec<PreservedStateClass>,
    /// Whether user-owned state was deleted.
    pub user_owned_state_deleted: bool,
    /// Whether durable state was deleted.
    pub durable_state_deleted: bool,
}

impl BisectSupportRestoreRow {
    /// Returns true when the restore row preserves the contract.
    pub fn is_export_safe(&self) -> bool {
        !self.user_owned_state_deleted
            && !self.durable_state_deleted
            && self
                .preserved_state_classes_observed
                .contains(&PreservedStateClass::UserAuthoredFiles)
            && !self.restored_extensions.is_empty()
            && !self.prior_state_snapshot_ref.trim().is_empty()
            && self.restore_disposition_class.is_admissible()
    }
}

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionBisectViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionBisectValidationReport {
    /// Validation failures.
    pub violations: Vec<ExtensionBisectViolation>,
}

impl fmt::Display for ExtensionBisectValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} extension-bisect violation(s)", self.violations.len())
    }
}

impl Error for ExtensionBisectValidationReport {}

/// Loads an extension-bisect session from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like an
/// [`ExtensionBisectSession`].
pub fn load_extension_bisect_session(
    yaml: &str,
) -> Result<ExtensionBisectSession, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads an extension-bisect step from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like an
/// [`ExtensionBisectStep`].
pub fn load_extension_bisect_step(yaml: &str) -> Result<ExtensionBisectStep, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads an extension-bisect finding from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like an
/// [`ExtensionBisectFinding`].
pub fn load_extension_bisect_finding(
    yaml: &str,
) -> Result<ExtensionBisectFinding, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads an extension-bisect restore from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like an
/// [`ExtensionBisectRestore`].
pub fn load_extension_bisect_restore(
    yaml: &str,
) -> Result<ExtensionBisectRestore, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Extension-bisect beta evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct ExtensionBisectEvaluator;

impl ExtensionBisectEvaluator {
    /// Creates a new extension-bisect evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates an [`ExtensionBisectSession`].
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionBisectValidationReport`] when the session omits
    /// the required preservation, declares a destructive reset, fails to
    /// describe a candidate, declares an empty disabled-capability set,
    /// duplicates ids, or carries a finding whose class requires suspects
    /// without naming them.
    pub fn validate_session(
        &self,
        session: &ExtensionBisectSession,
    ) -> Result<(), ExtensionBisectValidationReport> {
        let violations = validate_session(session);
        finalize(violations)
    }

    /// Validates an [`ExtensionBisectStep`].
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionBisectValidationReport`] when the step deletes
    /// user-owned or durable state, omits required ids, or carries an
    /// inconsistent cohort shape for its step class.
    pub fn validate_step(
        &self,
        step: &ExtensionBisectStep,
    ) -> Result<(), ExtensionBisectValidationReport> {
        let violations = validate_step(step);
        finalize(violations)
    }

    /// Validates an [`ExtensionBisectFinding`].
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionBisectValidationReport`] when the finding omits
    /// suspect refs while declaring a class that requires them, or omits
    /// the support packet ref.
    pub fn validate_finding(
        &self,
        finding: &ExtensionBisectFinding,
    ) -> Result<(), ExtensionBisectValidationReport> {
        let violations = validate_finding(finding);
        finalize(violations)
    }

    /// Validates an [`ExtensionBisectRestore`].
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionBisectValidationReport`] when the restore deletes
    /// user-owned or durable state, drops the `user_authored_files`
    /// preservation observation, declares an inadmissible disposition, or
    /// omits restored extensions.
    pub fn validate_restore(
        &self,
        restore: &ExtensionBisectRestore,
    ) -> Result<(), ExtensionBisectValidationReport> {
        let violations = validate_restore(restore);
        finalize(violations)
    }

    /// Builds the metadata-safe support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionBisectValidationReport`] when the session,
    /// finding, restore, or any step fails validation, or when the
    /// `session_ref` on any bound record does not match the supplied
    /// session.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        session: &ExtensionBisectSession,
        steps: &[ExtensionBisectStep],
        finding: &ExtensionBisectFinding,
        restore: &ExtensionBisectRestore,
    ) -> Result<ExtensionBisectSupportPacket, ExtensionBisectValidationReport> {
        let mut violations = validate_session(session);

        for step in steps {
            violations.extend(validate_step(step));
            if step.session_ref != session.session_id {
                push_violation(
                    &mut violations,
                    "extension_bisect.step_session_ref_mismatch",
                    &step.step_id,
                    "step session_ref must equal the bound session_id",
                );
            }
        }

        violations.extend(validate_finding(finding));
        if finding.session_ref != session.session_id {
            push_violation(
                &mut violations,
                "extension_bisect.finding_session_ref_mismatch",
                &finding.finding_id,
                "finding session_ref must equal the bound session_id",
            );
        }

        violations.extend(validate_restore(restore));
        if restore.session_ref != session.session_id {
            push_violation(
                &mut violations,
                "extension_bisect.restore_session_ref_mismatch",
                &restore.restore_id,
                "restore session_ref must equal the bound session_id",
            );
        }
        if restore.prior_state_snapshot_ref != session.restore_plan.prior_state_snapshot_ref {
            push_violation(
                &mut violations,
                "extension_bisect.restore_snapshot_ref_mismatch",
                &restore.restore_id,
                "restore prior_state_snapshot_ref must equal the session restore plan",
            );
        }

        if !violations.is_empty() {
            return Err(ExtensionBisectValidationReport { violations });
        }

        let candidate_rows = session
            .candidate_extensions
            .iter()
            .map(BisectSupportCandidateRow::from)
            .collect::<Vec<_>>();
        let tested_state_rows = session
            .tested_states
            .iter()
            .map(BisectSupportTestedStateRow::from)
            .collect::<Vec<_>>();
        let suspect_set_rows = session
            .suspected_extension_sets
            .iter()
            .map(BisectSupportSuspectSetRow::from)
            .collect::<Vec<_>>();
        let user_finding_rows = session
            .findings
            .iter()
            .map(BisectSupportFindingRow::from_session_finding)
            .chain(std::iter::once(
                BisectSupportFindingRow::from_finding_record(finding),
            ))
            .collect::<Vec<_>>();
        let step_rows = steps
            .iter()
            .map(BisectSupportStepRow::from)
            .collect::<Vec<_>>();
        let restore_row = BisectSupportRestoreRow::from(restore);
        let evidence_refs = session
            .evidence
            .iter()
            .map(|evidence| evidence.evidence_ref.clone())
            .collect::<Vec<_>>();

        Ok(ExtensionBisectSupportPacket {
            record_kind: EXTENSION_BISECT_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_BISECT_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: EXTENSION_BISECT_DOC_REF.to_owned(),
            schema_ref: EXTENSION_BISECT_SCHEMA_REF.to_owned(),
            session_id: session.session_id.clone(),
            session_class: session.session_class,
            entry_reason_class: session.entry_reason_class,
            doctor_finding_ref: session.doctor_finding_ref.clone(),
            blast_radius_class: session.blast_radius_class,
            review_gate_class: session.review_gate_class,
            candidate_rows,
            disabled_capability_classes: session.disabled_capability_classes.clone(),
            preserved_capability_classes: session.preserved_capability_classes.clone(),
            preserved_state_classes: session.preserved_state_classes.clone(),
            tested_state_rows,
            suspect_set_rows,
            user_finding_rows,
            step_rows,
            restore_row,
            evidence_refs,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
        })
    }
}

impl From<&CandidateExtension> for BisectSupportCandidateRow {
    fn from(candidate: &CandidateExtension) -> Self {
        Self {
            extension_id: candidate.extension_id.clone(),
            extension_class: candidate.extension_class,
            prior_state_class: candidate.prior_state_class,
            display_label: candidate.display_label.clone(),
        }
    }
}

impl From<&TestedStateRow> for BisectSupportTestedStateRow {
    fn from(row: &TestedStateRow) -> Self {
        Self {
            tested_state_id: row.tested_state_id.clone(),
            cohort_id: row.cohort_id.clone(),
            cohort_member_refs: row.cohort_member_refs.clone(),
            activation_result_class: row.activation_result_class,
            reviewer_summary: row.reviewer_summary.clone(),
        }
    }
}

impl From<&SuspectedExtensionSet> for BisectSupportSuspectSetRow {
    fn from(row: &SuspectedExtensionSet) -> Self {
        Self {
            suspect_set_id: row.suspect_set_id.clone(),
            confidence_class: row.confidence_class,
            member_extension_refs: row.member_extension_refs.clone(),
            narrowing_summary: row.narrowing_summary.clone(),
        }
    }
}

impl BisectSupportFindingRow {
    fn from_session_finding(finding: &UserVisibleFinding) -> Self {
        Self {
            finding_id: finding.finding_id.clone(),
            finding_class: finding.finding_class,
            user_visible_summary: finding.user_visible_summary.clone(),
            suspect_extension_refs: Vec::new(),
            escalation_actions: Vec::new(),
        }
    }

    fn from_finding_record(finding: &ExtensionBisectFinding) -> Self {
        Self {
            finding_id: finding.finding_id.clone(),
            finding_class: finding.finding_class,
            user_visible_summary: finding.user_visible_summary.clone(),
            suspect_extension_refs: finding.suspect_extension_refs.clone(),
            escalation_actions: finding.escalation_actions.clone(),
        }
    }
}

impl From<&ExtensionBisectStep> for BisectSupportStepRow {
    fn from(step: &ExtensionBisectStep) -> Self {
        Self {
            step_id: step.step_id.clone(),
            step_class: step.step_class,
            cohort_id: step.cohort_id.clone(),
            cohort_member_refs: step.cohort_member_refs.clone(),
            activation_result_class: step.activation_result_class,
            verdict_class: step.verdict_class,
            reviewer_summary: step.reviewer_summary.clone(),
            user_owned_state_deleted: step.user_owned_state_deleted,
            durable_state_deleted: step.durable_state_deleted,
        }
    }
}

impl From<&ExtensionBisectRestore> for BisectSupportRestoreRow {
    fn from(restore: &ExtensionBisectRestore) -> Self {
        Self {
            restore_id: restore.restore_id.clone(),
            restore_disposition_class: restore.restore_disposition_class,
            prior_state_snapshot_ref: restore.prior_state_snapshot_ref.clone(),
            restored_extensions: restore.restored_extensions.clone(),
            preserved_state_classes_observed: restore.preserved_state_classes_observed.clone(),
            user_owned_state_deleted: restore.user_owned_state_deleted,
            durable_state_deleted: restore.durable_state_deleted,
        }
    }
}

fn finalize(
    violations: Vec<ExtensionBisectViolation>,
) -> Result<(), ExtensionBisectValidationReport> {
    if violations.is_empty() {
        Ok(())
    } else {
        Err(ExtensionBisectValidationReport { violations })
    }
}

fn validate_session(session: &ExtensionBisectSession) -> Vec<ExtensionBisectViolation> {
    let mut violations = Vec::new();

    if session.schema_version != EXTENSION_BISECT_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "extension_bisect.session_schema_version",
            &session.session_id,
            "session schema_version must be 1",
        );
    }
    if session.record_kind != EXTENSION_BISECT_SESSION_RECORD_KIND {
        push_violation(
            &mut violations,
            "extension_bisect.session_record_kind",
            &session.session_id,
            "session record_kind must be extension_bisect_session_record",
        );
    }
    if session.session_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.session_id_empty",
            &session.session_id,
            "session_id must be non-empty",
        );
    }
    if !session.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "extension_bisect.doctor_finding_ref_missing",
            &session.session_id,
            "session must cite a Project Doctor finding ref",
        );
    }
    if session.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.support_packet_ref_missing",
            &session.session_id,
            "session must cite a support_packet_ref",
        );
    }
    if session.candidate_extensions.is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.candidates_missing",
            &session.session_id,
            "session must declare at least one candidate extension",
        );
    }
    if session.disabled_capability_classes.is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.disabled_capabilities_missing",
            &session.session_id,
            "session must declare at least one disabled capability class",
        );
    }
    if !session
        .disabled_capability_classes
        .contains(&DisabledCapabilityClass::ExtensionAutoActivation)
    {
        push_violation(
            &mut violations,
            "extension_bisect.extension_auto_activation_must_be_disabled",
            &session.session_id,
            "session must disable extension_auto_activation while the bisect is active",
        );
    }
    if !session
        .preserved_capability_classes
        .contains(&PreservedCapabilityClass::LocalEditing)
    {
        push_violation(
            &mut violations,
            "extension_bisect.local_editing_must_be_preserved",
            &session.session_id,
            "session must preserve local editing",
        );
    }
    if !session
        .preserved_capability_classes
        .contains(&PreservedCapabilityClass::ExtensionBisectExitAction)
    {
        push_violation(
            &mut violations,
            "extension_bisect.exit_action_must_be_preserved",
            &session.session_id,
            "session must preserve the explicit bisect exit action",
        );
    }
    if !session
        .preserved_state_classes
        .contains(&PreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "extension_bisect.user_authored_files_must_be_preserved",
            &session.session_id,
            "session must preserve user-authored files",
        );
    }
    if !session
        .preserved_state_classes
        .contains(&PreservedStateClass::ExtensionPriorStateSnapshot)
    {
        push_violation(
            &mut violations,
            "extension_bisect.prior_state_snapshot_must_be_preserved",
            &session.session_id,
            "session must preserve the extension prior-state snapshot",
        );
    }
    if session.destructive_resets_present {
        push_violation(
            &mut violations,
            "extension_bisect.destructive_reset_declared",
            &session.session_id,
            "extension-bisect sessions must not declare a destructive reset",
        );
    }
    if session.user_owned_state_deleted {
        push_violation(
            &mut violations,
            "extension_bisect.session_deletes_user_owned_state",
            &session.session_id,
            "session must not delete user-owned state",
        );
    }
    if session.durable_state_deleted {
        push_violation(
            &mut violations,
            "extension_bisect.session_deletes_durable_state",
            &session.session_id,
            "session must not delete durable non-disposable state",
        );
    }
    if session
        .restore_plan
        .prior_state_snapshot_ref
        .trim()
        .is_empty()
        || session
            .restore_plan
            .restore_action
            .action_id
            .trim()
            .is_empty()
        || session
            .restore_plan
            .restore_action
            .summary
            .trim()
            .is_empty()
        || session.restore_plan.restore_conditions.is_empty()
    {
        push_violation(
            &mut violations,
            "extension_bisect.restore_plan_missing",
            &session.session_id,
            "session must name a prior-state snapshot, restore action, summary, and conditions",
        );
    }
    if session.evidence.is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.evidence_missing",
            &session.session_id,
            "session must cite at least one evidence ref",
        );
    }

    let mut candidate_ids: BTreeSet<&str> = BTreeSet::new();
    for candidate in &session.candidate_extensions {
        if candidate.extension_id.trim().is_empty() || candidate.display_label.trim().is_empty() {
            push_violation(
                &mut violations,
                "extension_bisect.candidate_field_empty",
                &candidate.extension_id,
                "candidate extension_id and display_label must be non-empty",
            );
        }
        if !candidate_ids.insert(candidate.extension_id.as_str()) {
            push_violation(
                &mut violations,
                "extension_bisect.duplicate_candidate_id",
                &candidate.extension_id,
                "duplicate candidate extension_id is forbidden",
            );
        }
    }

    let mut tested_state_ids: BTreeSet<&str> = BTreeSet::new();
    for tested in &session.tested_states {
        if !tested_state_ids.insert(tested.tested_state_id.as_str()) {
            push_violation(
                &mut violations,
                "extension_bisect.duplicate_tested_state_id",
                &tested.tested_state_id,
                "duplicate tested_state_id is forbidden",
            );
        }
        if tested.cohort_id.trim().is_empty() || tested.reviewer_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "extension_bisect.tested_state_field_empty",
                &tested.tested_state_id,
                "tested-state cohort_id and reviewer_summary must be non-empty",
            );
        }
        for member in &tested.cohort_member_refs {
            if !candidate_ids.contains(member.as_str()) {
                push_violation(
                    &mut violations,
                    "extension_bisect.tested_state_member_unknown",
                    &tested.tested_state_id,
                    "tested-state cohort member must reference a declared candidate",
                );
            }
        }
    }

    let mut suspect_set_ids: BTreeSet<&str> = BTreeSet::new();
    for suspect in &session.suspected_extension_sets {
        if !suspect_set_ids.insert(suspect.suspect_set_id.as_str()) {
            push_violation(
                &mut violations,
                "extension_bisect.duplicate_suspect_set_id",
                &suspect.suspect_set_id,
                "duplicate suspect_set_id is forbidden",
            );
        }
        if suspect.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "extension_bisect.suspect_set_summary_empty",
                &suspect.suspect_set_id,
                "suspect-set narrowing_summary must be non-empty",
            );
        }
        let requires_members = matches!(
            suspect.confidence_class,
            SuspectConfidenceClass::SingleSuspectIsolated
                | SuspectConfidenceClass::NarrowSetSuspected
                | SuspectConfidenceClass::BroadSetSuspected
        );
        if requires_members && suspect.member_extension_refs.is_empty() {
            push_violation(
                &mut violations,
                "extension_bisect.suspect_set_members_missing",
                &suspect.suspect_set_id,
                "suspect set with non-empty confidence must name member extension refs",
            );
        }
        for member in &suspect.member_extension_refs {
            if !candidate_ids.contains(member.as_str()) {
                push_violation(
                    &mut violations,
                    "extension_bisect.suspect_member_unknown",
                    &suspect.suspect_set_id,
                    "suspect-set member must reference a declared candidate",
                );
            }
        }
    }

    let mut finding_ids: BTreeSet<&str> = BTreeSet::new();
    for finding in &session.findings {
        if !finding_ids.insert(finding.finding_id.as_str()) {
            push_violation(
                &mut violations,
                "extension_bisect.duplicate_session_finding_id",
                &finding.finding_id,
                "duplicate finding_id within the session is forbidden",
            );
        }
        if finding.user_visible_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "extension_bisect.session_finding_summary_empty",
                &finding.finding_id,
                "user_visible_summary must be non-empty",
            );
        }
    }

    violations
}

fn validate_step(step: &ExtensionBisectStep) -> Vec<ExtensionBisectViolation> {
    let mut violations = Vec::new();

    if step.schema_version != EXTENSION_BISECT_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "extension_bisect.step_schema_version",
            &step.step_id,
            "step schema_version must be 1",
        );
    }
    if step.record_kind != EXTENSION_BISECT_STEP_RECORD_KIND {
        push_violation(
            &mut violations,
            "extension_bisect.step_record_kind",
            &step.step_id,
            "step record_kind must be extension_bisect_step_record",
        );
    }
    if step.step_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.step_id_empty",
            &step.step_id,
            "step_id must be non-empty",
        );
    }
    if step.session_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.step_session_ref_empty",
            &step.step_id,
            "step session_ref must be non-empty",
        );
    }
    if step.cohort_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.step_cohort_id_empty",
            &step.step_id,
            "step cohort_id must be non-empty",
        );
    }
    if step.reviewer_summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.step_reviewer_summary_empty",
            &step.step_id,
            "step reviewer_summary must be non-empty",
        );
    }
    if step.user_owned_state_deleted {
        push_violation(
            &mut violations,
            "extension_bisect.step_deletes_user_owned_state",
            &step.step_id,
            "step must not delete user-owned state",
        );
    }
    if step.durable_state_deleted {
        push_violation(
            &mut violations,
            "extension_bisect.step_deletes_durable_state",
            &step.step_id,
            "step must not delete durable non-disposable state",
        );
    }

    match step.step_class {
        StepClass::InitialBaselineCheck | StepClass::ExitBaselineCheck => {
            if !step.cohort_member_refs.is_empty() {
                push_violation(
                    &mut violations,
                    "extension_bisect.baseline_step_has_members",
                    &step.step_id,
                    "baseline steps must not declare cohort members",
                );
            }
        }
        StepClass::CohortActivation | StepClass::CohortVerification => {
            if step.cohort_member_refs.is_empty() {
                push_violation(
                    &mut violations,
                    "extension_bisect.cohort_step_missing_members",
                    &step.step_id,
                    "cohort steps must declare at least one cohort member",
                );
            }
        }
    }

    if matches!(
        step.activation_result_class,
        CohortActivationResultClass::AbortedByUser | CohortActivationResultClass::AbortedByPolicy
    ) && step.verdict_class != StepVerdictClass::CohortRunAborted
    {
        push_violation(
            &mut violations,
            "extension_bisect.aborted_step_verdict_mismatch",
            &step.step_id,
            "aborted activations must record verdict_class cohort_run_aborted",
        );
    }

    violations
}

fn validate_finding(finding: &ExtensionBisectFinding) -> Vec<ExtensionBisectViolation> {
    let mut violations = Vec::new();

    if finding.schema_version != EXTENSION_BISECT_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "extension_bisect.finding_schema_version",
            &finding.finding_id,
            "finding schema_version must be 1",
        );
    }
    if finding.record_kind != EXTENSION_BISECT_FINDING_RECORD_KIND {
        push_violation(
            &mut violations,
            "extension_bisect.finding_record_kind",
            &finding.finding_id,
            "finding record_kind must be extension_bisect_finding_record",
        );
    }
    if finding.finding_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.finding_id_empty",
            &finding.finding_id,
            "finding_id must be non-empty",
        );
    }
    if finding.session_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.finding_session_ref_empty",
            &finding.finding_id,
            "finding session_ref must be non-empty",
        );
    }
    if finding.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.finding_support_packet_ref_empty",
            &finding.finding_id,
            "finding support_packet_ref must be non-empty",
        );
    }
    if finding.user_visible_summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.finding_summary_empty",
            &finding.finding_id,
            "finding user_visible_summary must be non-empty",
        );
    }
    if finding.finding_class.requires_suspect_refs() && finding.suspect_extension_refs.is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.finding_missing_suspect_refs",
            &finding.finding_id,
            "single/multi-extension findings must name at least one suspect ref",
        );
    }

    violations
}

fn validate_restore(restore: &ExtensionBisectRestore) -> Vec<ExtensionBisectViolation> {
    let mut violations = Vec::new();

    if restore.schema_version != EXTENSION_BISECT_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "extension_bisect.restore_schema_version",
            &restore.restore_id,
            "restore schema_version must be 1",
        );
    }
    if restore.record_kind != EXTENSION_BISECT_RESTORE_RECORD_KIND {
        push_violation(
            &mut violations,
            "extension_bisect.restore_record_kind",
            &restore.restore_id,
            "restore record_kind must be extension_bisect_restore_record",
        );
    }
    if restore.restore_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.restore_id_empty",
            &restore.restore_id,
            "restore_id must be non-empty",
        );
    }
    if restore.session_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.restore_session_ref_empty",
            &restore.restore_id,
            "restore session_ref must be non-empty",
        );
    }
    if restore.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.restore_support_packet_ref_empty",
            &restore.restore_id,
            "restore support_packet_ref must be non-empty",
        );
    }
    if restore.prior_state_snapshot_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.restore_snapshot_ref_empty",
            &restore.restore_id,
            "restore prior_state_snapshot_ref must be non-empty",
        );
    }
    if restore.restored_extensions.is_empty() {
        push_violation(
            &mut violations,
            "extension_bisect.restore_no_extensions",
            &restore.restore_id,
            "restore must name at least one restored extension",
        );
    }
    if !restore
        .preserved_state_classes_observed
        .contains(&PreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "extension_bisect.restore_must_preserve_user_authored_files",
            &restore.restore_id,
            "restore must observe preservation of user-authored files",
        );
    }
    if restore.user_owned_state_deleted {
        push_violation(
            &mut violations,
            "extension_bisect.restore_deletes_user_owned_state",
            &restore.restore_id,
            "restore must not delete user-owned state",
        );
    }
    if restore.durable_state_deleted {
        push_violation(
            &mut violations,
            "extension_bisect.restore_deletes_durable_state",
            &restore.restore_id,
            "restore must not delete durable non-disposable state",
        );
    }
    if !restore.restore_disposition_class.is_admissible() {
        push_violation(
            &mut violations,
            "extension_bisect.restore_disposition_pending_review",
            &restore.restore_id,
            "restore_disposition_class must be admissible at packet time",
        );
    }
    for row in &restore.restored_extensions {
        if row.extension_id.trim().is_empty() || row.reviewer_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "extension_bisect.restored_row_field_empty",
                &row.extension_id,
                "restored extension_id and reviewer_summary must be non-empty",
            );
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<ExtensionBisectViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ExtensionBisectViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
