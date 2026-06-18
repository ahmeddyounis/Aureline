//! Run-history and evidence-panel object and its first consumers.
//!
//! The automation run-history contract
//! ([`/docs/automation/run_history_contract.md`]) froze *what* a run-history row
//! is — the projection over a run record carrying run identity, automation layer,
//! execution mode, result class, artifact-bundle and retention/redaction posture,
//! and a rerun-under-current-policy action. This module makes the history panel
//! concrete: a live [`RunHistoryEntry`] that records one attempted dispatch as an
//! attributable evidence row, and a [`RunHistoryEntry::resolve_rerun`] that
//! resolves rerun **against current policy, trust, execution context, and secret
//! references** every time — never against a cached approval or a stale
//! environment.
//!
//! The entry never asserts that history preserved authority.
//! [`RunHistoryEntry::resolved_rerun_class`] derives the rerun action from the
//! entry's automation layer, its imported state, and the
//! [`CurrentPolicyBlocker`]s the resolver observed *now*, so yesterday's success
//! is never an admissibility argument on its own. The live entry projects onto an
//! attributable [`RunHistoryEvidenceRow`] through
//! [`RunHistoryEntry::to_evidence_row`], and
//! [`RunHistoryEntry::export`] carries the row, the resolved rerun, and the entry
//! verbatim into support packets, incident/runbook follow-up, AI evidence joins,
//! and CLI/headless inspect surfaces so the run stays comparable and explainable
//! after the panel closes.
//!
//! [`RunHistoryFirstConsumersPacket`] binds the first M5 automation families that
//! render a history panel — notebook, task/test/debug, request/API, package,
//! incident, and the AI assistant — each to a seeded panel of entries, and
//! [`RunHistoryFirstConsumersPacket::validate`] enforces the freeze mechanically:
//! every entrypoint binds a non-empty panel, every entry resolves its run identity
//! and layer, rerun resolves current policy rather than implying cached approval,
//! an imported row never offers a one-click rerun, a recorded macro never offers
//! extension/external rerun, open-as-recipe never launders a capability into a
//! recipe, and no raw secret crosses the row boundary. A dropped entrypoint, an
//! empty panel, a rerun that implies cached approval, an imported row that offers
//! rerun, a macro that offers external rerun, a laundered capability, a raw secret,
//! an inconsistent rerun projection, or a violated invariant *blocks stable*.
//!
//! The reviewer-facing landing page is
//! [`/docs/m5/automation-run-history.md`]; the cross-tool boundary schema is
//! [`/schemas/automation/run-history.schema.json`]; the frozen run-record,
//! run-history-row, and run-summary-export boundary schemas it reuses are named in
//! [`canonical_reused_contract_refs`].
//!
//! [`/docs/automation/run_history_contract.md`]: ../../../docs/automation/run_history_contract.md
//! [`/docs/m5/automation-run-history.md`]: ../../../docs/m5/automation-run-history.md
//! [`/schemas/automation/run-history.schema.json`]: ../../../schemas/automation/run-history.schema.json

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    AutomationBaselinePromotionState, RECIPE_BUILDER_SCHEMA_REF, RUN_HISTORY_ROW_SCHEMA_REF,
    RUN_RECORD_SCHEMA_REF, RUN_SUMMARY_EXPORT_SCHEMA_REF,
};
use crate::recipe_builder::RecipeBuilderEntrypoint;

/// Stable record-kind tag for [`RunHistoryFirstConsumersPacket`].
pub const RUN_HISTORY_FIRST_CONSUMERS_RECORD_KIND: &str = "m5_run_history_first_consumers_packet";

/// Stable record-kind tag for [`RunHistoryFirstConsumersSupportExport`].
pub const RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_run_history_first_consumers_support_export";

/// Stable record-kind tag for [`RunHistoryFirstConsumersCliHeadlessView`].
pub const RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_run_history_first_consumers_cli_headless";

/// Stable record-kind tag for [`RunHistoryEvidenceRow`].
pub const RUN_HISTORY_EVIDENCE_ROW_RECORD_KIND: &str = "run_history_evidence_row";

/// Stable record-kind tag for [`RunHistoryEvidenceExport`].
pub const RUN_HISTORY_EVIDENCE_EXPORT_RECORD_KIND: &str = "run_history_evidence_export_record";

/// Stable record-kind tag for [`RerunResolution`].
pub const RERUN_RESOLUTION_RECORD_KIND: &str = "run_history_rerun_resolution";

/// Integer schema version for the run-history first-consumers family.
pub const RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the first-consumers boundary schema.
pub const RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/automation/run-history.schema.json";

/// Repo-relative path of the reviewer contract doc for the run-history lane.
pub const RUN_HISTORY_DOC_REF: &str = "docs/m5/automation-run-history.md";

/// Repo-relative path of the checked-in first-consumers packet artifact.
pub const RUN_HISTORY_FIRST_CONSUMERS_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/run-history/packet.json";

/// Repo-relative root the worked-example run-history-evidence fixtures live under.
pub const RUN_HISTORY_FIXTURE_DIR: &str = "fixtures/automation/m5/run-history-evidence";

/// Stable packet id minted by the seed.
pub const RUN_HISTORY_FIRST_CONSUMERS_ID: &str = "automation:m5:run-history-first-consumers:v1";

/// Stable support-export id minted by the seed inspector.
pub const RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:run-history-first-consumers";

/// Stable CLI/headless view id minted by the seed inspector.
pub const RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:run-history-first-consumers";

// ---------------------------------------------------------------------------
// Automation layer
// ---------------------------------------------------------------------------

/// The automation layer a history row was minted under.
///
/// The layer discriminates which underlying record the row projects from and
/// which rerun subset is admissible: a recorded macro never offers extension or
/// external rerun, and an extension/external row never launders a capability into
/// a declarative recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AutomationLayerClass {
    /// A recorded macro replayed on the desktop only.
    #[serde(rename = "recorded_macro_layer")]
    RecordedMacro,
    /// A declarative recipe with the full rerun-action vocabulary.
    #[serde(rename = "declarative_recipe_layer")]
    DeclarativeRecipe,
    /// A managed-only template dispatched on the managed channel only.
    #[serde(rename = "managed_only_template_layer")]
    ManagedOnlyTemplate,
    /// An extension or external runner (or an imported provider event).
    #[serde(rename = "extension_or_external_automation_layer")]
    ExtensionOrExternalAutomation,
    /// A headless-safe run dispatched through the CLI / headless / offline lane.
    #[serde(rename = "headless_safe_run_layer")]
    HeadlessSafeRun,
}

impl AutomationLayerClass {
    /// Every automation layer in canonical order.
    pub const ALL: [AutomationLayerClass; 5] = [
        AutomationLayerClass::RecordedMacro,
        AutomationLayerClass::DeclarativeRecipe,
        AutomationLayerClass::ManagedOnlyTemplate,
        AutomationLayerClass::ExtensionOrExternalAutomation,
        AutomationLayerClass::HeadlessSafeRun,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            AutomationLayerClass::RecordedMacro => "recorded_macro_layer",
            AutomationLayerClass::DeclarativeRecipe => "declarative_recipe_layer",
            AutomationLayerClass::ManagedOnlyTemplate => "managed_only_template_layer",
            AutomationLayerClass::ExtensionOrExternalAutomation => {
                "extension_or_external_automation_layer"
            }
            AutomationLayerClass::HeadlessSafeRun => "headless_safe_run_layer",
        }
    }
}

// ---------------------------------------------------------------------------
// Execution mode
// ---------------------------------------------------------------------------

/// The surface and dispatch mode a run was attempted through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionModeClass {
    /// Dispatched from the desktop command palette.
    DesktopPaletteDispatch,
    /// Dispatched from a desktop keybinding.
    DesktopKeybindingDispatch,
    /// Dispatched from an explicit desktop action button.
    DesktopExplicitActionDispatch,
    /// Dispatched from the AI assistant.
    AiAssistantDispatch,
    /// Dispatched explicitly through the headless CLI.
    HeadlessCliExplicitDispatch,
    /// Dispatched from a headless CLI script.
    HeadlessCliScriptedDispatch,
    /// Replayed offline through the headless lane.
    HeadlessOfflineReplayDispatch,
    /// Dispatched through the run queue.
    QueuedDispatch,
    /// Dispatched on the managed-only channel.
    ManagedOnlyChannelDispatch,
    /// Dispatched by an external runner.
    ExternalRunnerDispatch,
    /// Imported from a provider event; never locally dispatched.
    ImportedProviderEvent,
}

impl ExecutionModeClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ExecutionModeClass::DesktopPaletteDispatch => "desktop_palette_dispatch",
            ExecutionModeClass::DesktopKeybindingDispatch => "desktop_keybinding_dispatch",
            ExecutionModeClass::DesktopExplicitActionDispatch => "desktop_explicit_action_dispatch",
            ExecutionModeClass::AiAssistantDispatch => "ai_assistant_dispatch",
            ExecutionModeClass::HeadlessCliExplicitDispatch => "headless_cli_explicit_dispatch",
            ExecutionModeClass::HeadlessCliScriptedDispatch => "headless_cli_scripted_dispatch",
            ExecutionModeClass::HeadlessOfflineReplayDispatch => "headless_offline_replay_dispatch",
            ExecutionModeClass::QueuedDispatch => "queued_dispatch",
            ExecutionModeClass::ManagedOnlyChannelDispatch => "managed_only_channel_dispatch",
            ExecutionModeClass::ExternalRunnerDispatch => "external_runner_dispatch",
            ExecutionModeClass::ImportedProviderEvent => "imported_provider_event",
        }
    }
}

// ---------------------------------------------------------------------------
// Result class
// ---------------------------------------------------------------------------

/// The result class a run record reports, re-export of the run-outcome axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunResultClass {
    /// Every step succeeded.
    Succeeded,
    /// Some steps succeeded and some did not.
    PartialSuccess,
    /// The run was denied at a gate before applying.
    DeniedAtGate,
    /// The run was aborted before completing.
    Aborted,
    /// The run is queued and has not completed.
    Queued,
    /// The run was a dry-run only and applied nothing.
    DryRunOnly,
}

impl RunResultClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RunResultClass::Succeeded => "succeeded",
            RunResultClass::PartialSuccess => "partial_success",
            RunResultClass::DeniedAtGate => "denied_at_gate",
            RunResultClass::Aborted => "aborted",
            RunResultClass::Queued => "queued",
            RunResultClass::DryRunOnly => "dry_run_only",
        }
    }

    /// Whether the run has completed (anything but queued).
    pub fn is_complete(self) -> bool {
        !matches!(self, RunResultClass::Queued)
    }
}

// ---------------------------------------------------------------------------
// Artifact link
// ---------------------------------------------------------------------------

/// The class of an artifact a run produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactLinkClass {
    /// A captured run / output log.
    RunLog,
    /// A produced result artifact (coverage, report, response capture).
    ResultArtifact,
    /// A local evidence / support bundle.
    EvidenceBundle,
    /// A diff or preview artifact.
    DiffArtifact,
    /// An artifact landed on an external target the run does not own.
    ExternalArtifact,
}

impl ArtifactLinkClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ArtifactLinkClass::RunLog => "run_log",
            ArtifactLinkClass::ResultArtifact => "result_artifact",
            ArtifactLinkClass::EvidenceBundle => "evidence_bundle",
            ArtifactLinkClass::DiffArtifact => "diff_artifact",
            ArtifactLinkClass::ExternalArtifact => "external_artifact",
        }
    }
}

/// One artifact a run produced, named by an opaque content-addressed reference.
///
/// The reference is opaque; a raw path, URL, or content never appears here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLink {
    /// The artifact class.
    pub link_class: ArtifactLinkClass,
    /// Opaque, content-addressed artifact reference; never a raw path or URL.
    pub artifact_ref: String,
    /// Reviewable summary of the artifact.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Retention / redaction / artifact-bundle state
// ---------------------------------------------------------------------------

/// The retention window a history row tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// User-controlled retention only.
    RetainUntilPurgedByUser,
    /// Bounded by the workspace redaction window.
    RetainUntilWorkspaceRedactionWindow,
    /// Bounded by the organization audit window.
    RetainUntilOrganizationAuditWindow,
    /// Bounded to the lifecycle of a support packet that resolved through the row.
    RetainUntilSupportExportConsumed,
    /// Bounded to the queued replay window.
    RetainUntilReplayWindowExpires,
    /// Held indefinitely under an organization audit lock.
    RetainIndefinitelyUnderAuditLock,
    /// Terminal state: purged, only the safe summary remains.
    PurgedByRetentionSummaryOnly,
}

impl RetentionClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RetentionClass::RetainUntilPurgedByUser => "retain_until_purged_by_user",
            RetentionClass::RetainUntilWorkspaceRedactionWindow => {
                "retain_until_workspace_redaction_window"
            }
            RetentionClass::RetainUntilOrganizationAuditWindow => {
                "retain_until_organization_audit_window"
            }
            RetentionClass::RetainUntilSupportExportConsumed => {
                "retain_until_support_export_consumed"
            }
            RetentionClass::RetainUntilReplayWindowExpires => "retain_until_replay_window_expires",
            RetentionClass::RetainIndefinitelyUnderAuditLock => {
                "retain_indefinitely_under_audit_lock"
            }
            RetentionClass::PurgedByRetentionSummaryOnly => "purged_by_retention_summary_only",
        }
    }

    /// Whether the window is bounded (requires a non-null expiry timestamp).
    pub fn is_windowed(self) -> bool {
        matches!(
            self,
            RetentionClass::RetainUntilWorkspaceRedactionWindow
                | RetentionClass::RetainUntilOrganizationAuditWindow
                | RetentionClass::RetainUntilSupportExportConsumed
                | RetentionClass::RetainUntilReplayWindowExpires
        )
    }
}

/// The redaction mode a history row's safe summary carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionModeClass {
    /// The metadata-safe default redaction floor.
    MetadataSafeDefault,
    /// Redaction is required before the row crosses an export boundary.
    RedactionRequiredOnExport,
    /// Redaction is required and secret-broker handles ride opaque.
    RedactionRequiredWithSecretBrokerHandles,
    /// Operator-only export with broader internal preservation.
    OperatorOnlyInternalPreservation,
}

impl RedactionModeClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RedactionModeClass::MetadataSafeDefault => "metadata_safe_default",
            RedactionModeClass::RedactionRequiredOnExport => "redaction_required_on_export",
            RedactionModeClass::RedactionRequiredWithSecretBrokerHandles => {
                "redaction_required_with_secret_broker_handles"
            }
            RedactionModeClass::OperatorOnlyInternalPreservation => {
                "operator_only_internal_preservation"
            }
        }
    }
}

/// The state of the artifact bundle a run produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactBundleStateClass {
    /// The artifact bundle is available and the row's ref is non-null.
    ArtifactBundleAvailable,
    /// No bundle was produced because the run was denied at a gate.
    ArtifactBundleNotProducedDeniedAtGate,
    /// No bundle was produced because the run was a macro state replay.
    ArtifactBundleNotProducedMacroStateReplay,
    /// No bundle was produced because the run ran under an external authority.
    ArtifactBundleNotProducedExternalAuthority,
    /// The bundle was purged by retention; only the safe summary remains.
    ArtifactBundlePurgedByRetentionSummaryOnly,
}

impl ArtifactBundleStateClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ArtifactBundleStateClass::ArtifactBundleAvailable => "artifact_bundle_available",
            ArtifactBundleStateClass::ArtifactBundleNotProducedDeniedAtGate => {
                "artifact_bundle_not_produced_denied_at_gate"
            }
            ArtifactBundleStateClass::ArtifactBundleNotProducedMacroStateReplay => {
                "artifact_bundle_not_produced_macro_state_replay"
            }
            ArtifactBundleStateClass::ArtifactBundleNotProducedExternalAuthority => {
                "artifact_bundle_not_produced_external_authority"
            }
            ArtifactBundleStateClass::ArtifactBundlePurgedByRetentionSummaryOnly => {
                "artifact_bundle_purged_by_retention_summary_only"
            }
        }
    }

    /// Whether this state carries a non-null artifact-bundle ref.
    pub fn carries_bundle_ref(self) -> bool {
        matches!(self, ArtifactBundleStateClass::ArtifactBundleAvailable)
    }
}

// ---------------------------------------------------------------------------
// Context summary axes
// ---------------------------------------------------------------------------

/// The workspace-trust state a run observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStateClass {
    /// A trusted local workspace.
    WorkspaceTrusted,
    /// A restricted (limited-trust) local workspace.
    WorkspaceRestricted,
    /// A trusted remote target.
    RemoteTrusted,
    /// An untrusted remote target.
    RemoteUntrusted,
}

impl TrustStateClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            TrustStateClass::WorkspaceTrusted => "workspace_trusted",
            TrustStateClass::WorkspaceRestricted => "workspace_restricted",
            TrustStateClass::RemoteTrusted => "remote_trusted",
            TrustStateClass::RemoteUntrusted => "remote_untrusted",
        }
    }
}

/// The admin-policy observation a run recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyObservationClass {
    /// Policy allowed the run.
    PolicyAllowed,
    /// Policy constrained the run.
    PolicyConstrained,
    /// Policy denied the run.
    PolicyDenied,
    /// No policy was observed.
    PolicyUnobserved,
}

impl PolicyObservationClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            PolicyObservationClass::PolicyAllowed => "policy_allowed",
            PolicyObservationClass::PolicyConstrained => "policy_constrained",
            PolicyObservationClass::PolicyDenied => "policy_denied",
            PolicyObservationClass::PolicyUnobserved => "policy_unobserved",
        }
    }
}

/// The kill-switch observation a run recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchObservationClass {
    /// The kill switch was clear.
    KillSwitchClear,
    /// The kill switch was engaged.
    KillSwitchEngaged,
    /// No kill-switch state was observed.
    KillSwitchUnobserved,
}

impl KillSwitchObservationClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            KillSwitchObservationClass::KillSwitchClear => "kill_switch_clear",
            KillSwitchObservationClass::KillSwitchEngaged => "kill_switch_engaged",
            KillSwitchObservationClass::KillSwitchUnobserved => "kill_switch_unobserved",
        }
    }
}

/// The context block a row preserves about where a run executed.
///
/// Raw env-var values, raw paths, and raw URLs never appear here; the block
/// carries opaque capsule references and closed-vocabulary observation classes
/// only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextSummary {
    /// Opaque execution-context capsule reference bounding the run.
    pub execution_context_capsule_ref: String,
    /// Opaque environment capsule reference, or `null` when non-replayable.
    pub environment_capsule_ref: Option<String>,
    /// The workspace-trust state the run observed.
    pub trust_state_class: TrustStateClass,
    /// The admin-policy observation the run recorded.
    pub policy_observation_class: PolicyObservationClass,
    /// The kill-switch observation the run recorded.
    pub kill_switch_observation_class: KillSwitchObservationClass,
    /// Reviewable sentence describing the workspace / profile / scope.
    pub context_summary_sentence: String,
}

// ---------------------------------------------------------------------------
// Run identity
// ---------------------------------------------------------------------------

/// The stable identity of one attempted dispatch.
///
/// Every history row resolves to a `run_id`, a `manifest_id`, and a
/// `manifest_revision_ref`; the optional `manifest_content_address` pins the exact
/// manifest bytes. All four are opaque, content-addressed, or revision references —
/// never a raw argv, path, or secret.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunIdentity {
    /// Opaque run id (or the external runner's opaque handle for imported rows).
    pub run_id: String,
    /// Opaque manifest id the run dispatched.
    pub manifest_id: String,
    /// Opaque manifest revision reference.
    pub manifest_revision_ref: String,
    /// Optional content address pinning the exact manifest bytes.
    pub manifest_content_address: Option<String>,
}

// ---------------------------------------------------------------------------
// Current-policy blocker
// ---------------------------------------------------------------------------

/// One blocker the rerun resolver observed at projection time.
///
/// The blocker list is the authoritative reason a rerun would not be admitted
/// *today*. [`CurrentPolicyBlocker::NoBlockerPresent`] is the only blocker that
/// pairs with [`RerunActionClass::AdmissibleNoRevalidation`]; every other rerun
/// class cites at least one non-no-blocker entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentPolicyBlocker {
    /// No blocker is present; rerun is admissible with no revalidation.
    NoBlockerPresent,
    /// The environment must be revalidated before rerun.
    EnvironmentRevalidationRequired,
    /// A fresh approval ticket must be granted before rerun.
    FreshApprovalRequired,
    /// The kill switch must be cleared before rerun.
    KillSwitchEngaged,
    /// The managed-only channel must be resolved before rerun.
    ManagedOnlyChannelUnresolved,
    /// The publisher of the automation has been revoked.
    PublisherRevoked,
    /// A required capability is disabled by policy.
    CapabilityDisabledByPolicy,
    /// The managed-only template has been retired.
    ManagedOnlyTemplateRetired,
    /// The recipe revision has been retired.
    RecipeRevisionRetired,
    /// The replay window has expired (stale authority).
    ReplayWindowExpired,
    /// The command descriptor revision has been retired.
    DescriptorRevisionRetired,
    /// The environment capsule has drifted since the run.
    EnvironmentCapsuleDriftDetected,
    /// The row is a macro recording locked to the recorded-macro capability subset.
    MacroRecordingOnly,
    /// The extension or external runner is unavailable.
    ExtensionOrExternalRunnerUnavailable,
    /// The row was imported; dispatch authority did not survive the import boundary.
    ImportedRecordNoDispatchAdmissible,
}

impl CurrentPolicyBlocker {
    /// Every blocker in canonical order.
    pub const ALL: [CurrentPolicyBlocker; 15] = [
        CurrentPolicyBlocker::NoBlockerPresent,
        CurrentPolicyBlocker::EnvironmentRevalidationRequired,
        CurrentPolicyBlocker::FreshApprovalRequired,
        CurrentPolicyBlocker::KillSwitchEngaged,
        CurrentPolicyBlocker::ManagedOnlyChannelUnresolved,
        CurrentPolicyBlocker::PublisherRevoked,
        CurrentPolicyBlocker::CapabilityDisabledByPolicy,
        CurrentPolicyBlocker::ManagedOnlyTemplateRetired,
        CurrentPolicyBlocker::RecipeRevisionRetired,
        CurrentPolicyBlocker::ReplayWindowExpired,
        CurrentPolicyBlocker::DescriptorRevisionRetired,
        CurrentPolicyBlocker::EnvironmentCapsuleDriftDetected,
        CurrentPolicyBlocker::MacroRecordingOnly,
        CurrentPolicyBlocker::ExtensionOrExternalRunnerUnavailable,
        CurrentPolicyBlocker::ImportedRecordNoDispatchAdmissible,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            CurrentPolicyBlocker::NoBlockerPresent => "no_blocker_present",
            CurrentPolicyBlocker::EnvironmentRevalidationRequired => {
                "environment_revalidation_required"
            }
            CurrentPolicyBlocker::FreshApprovalRequired => "fresh_approval_required",
            CurrentPolicyBlocker::KillSwitchEngaged => "kill_switch_engaged",
            CurrentPolicyBlocker::ManagedOnlyChannelUnresolved => "managed_only_channel_unresolved",
            CurrentPolicyBlocker::PublisherRevoked => "publisher_revoked",
            CurrentPolicyBlocker::CapabilityDisabledByPolicy => "capability_disabled_by_policy",
            CurrentPolicyBlocker::ManagedOnlyTemplateRetired => "managed_only_template_retired",
            CurrentPolicyBlocker::RecipeRevisionRetired => "recipe_revision_retired",
            CurrentPolicyBlocker::ReplayWindowExpired => "replay_window_expired",
            CurrentPolicyBlocker::DescriptorRevisionRetired => "descriptor_revision_retired",
            CurrentPolicyBlocker::EnvironmentCapsuleDriftDetected => {
                "environment_capsule_drift_detected"
            }
            CurrentPolicyBlocker::MacroRecordingOnly => "macro_recording_only",
            CurrentPolicyBlocker::ExtensionOrExternalRunnerUnavailable => {
                "extension_or_external_runner_unavailable"
            }
            CurrentPolicyBlocker::ImportedRecordNoDispatchAdmissible => {
                "imported_record_no_dispatch_admissible"
            }
        }
    }

    /// The rerun disposition this blocker forces.
    pub fn disposition(self) -> RerunDisposition {
        match self {
            CurrentPolicyBlocker::NoBlockerPresent => RerunDisposition::NoRevalidation,
            CurrentPolicyBlocker::EnvironmentRevalidationRequired
            | CurrentPolicyBlocker::FreshApprovalRequired
            | CurrentPolicyBlocker::KillSwitchEngaged
            | CurrentPolicyBlocker::ManagedOnlyChannelUnresolved => {
                RerunDisposition::RequiresRevalidation
            }
            _ => RerunDisposition::Denies,
        }
    }

    /// The rerun action class this blocker maps to one-to-one.
    pub fn rerun_action_class(self) -> RerunActionClass {
        match self {
            CurrentPolicyBlocker::NoBlockerPresent => RerunActionClass::AdmissibleNoRevalidation,
            CurrentPolicyBlocker::EnvironmentRevalidationRequired => {
                RerunActionClass::AdmissibleAfterEnvironmentRevalidation
            }
            CurrentPolicyBlocker::FreshApprovalRequired => {
                RerunActionClass::AdmissibleAfterFreshApproval
            }
            CurrentPolicyBlocker::KillSwitchEngaged => {
                RerunActionClass::AdmissibleAfterKillSwitchClear
            }
            CurrentPolicyBlocker::ManagedOnlyChannelUnresolved => {
                RerunActionClass::AdmissibleAfterManagedChannelResolved
            }
            CurrentPolicyBlocker::PublisherRevoked => RerunActionClass::BlockedPublisherRevoked,
            CurrentPolicyBlocker::CapabilityDisabledByPolicy => {
                RerunActionClass::BlockedCapabilityDisabledByPolicy
            }
            CurrentPolicyBlocker::ManagedOnlyTemplateRetired => {
                RerunActionClass::BlockedManagedOnlyTemplateRetired
            }
            CurrentPolicyBlocker::RecipeRevisionRetired => {
                RerunActionClass::BlockedRecipeRevisionRetired
            }
            CurrentPolicyBlocker::ReplayWindowExpired => {
                RerunActionClass::BlockedReplayWindowExpired
            }
            CurrentPolicyBlocker::DescriptorRevisionRetired => {
                RerunActionClass::BlockedDescriptorRevisionRetired
            }
            CurrentPolicyBlocker::EnvironmentCapsuleDriftDetected => {
                RerunActionClass::BlockedEnvironmentCapsuleDrift
            }
            CurrentPolicyBlocker::MacroRecordingOnly => RerunActionClass::BlockedMacroRecordingOnly,
            CurrentPolicyBlocker::ExtensionOrExternalRunnerUnavailable => {
                RerunActionClass::BlockedExtensionOrExternalRunnerUnavailable
            }
            CurrentPolicyBlocker::ImportedRecordNoDispatchAdmissible => {
                RerunActionClass::BlockedImportedRecord
            }
        }
    }
}

/// The disposition a [`CurrentPolicyBlocker`] forces on rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RerunDisposition {
    /// Rerun is admissible with no revalidation.
    NoRevalidation,
    /// Rerun is admissible only after the named revalidation.
    RequiresRevalidation,
    /// Rerun is denied until the blocker clears.
    Denies,
}

// ---------------------------------------------------------------------------
// Rerun action class
// ---------------------------------------------------------------------------

/// The resolved rerun-under-current-policy action a history row offers.
///
/// The class is derived from the entry's automation layer, imported state, and the
/// [`CurrentPolicyBlocker`]s observed now — never from a cached approval or a
/// preserved environment capsule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RerunActionClass {
    /// Rerun is admissible today with no revalidation.
    #[serde(rename = "rerun_under_current_policy_admissible_no_revalidation_required")]
    AdmissibleNoRevalidation,
    /// Rerun is admissible after the environment is revalidated.
    #[serde(rename = "rerun_under_current_policy_admissible_after_environment_revalidation")]
    AdmissibleAfterEnvironmentRevalidation,
    /// Rerun is admissible after a fresh approval is granted.
    #[serde(rename = "rerun_under_current_policy_admissible_after_fresh_approval")]
    AdmissibleAfterFreshApproval,
    /// Rerun is admissible after the kill switch is cleared.
    #[serde(rename = "rerun_under_current_policy_admissible_after_kill_switch_clear")]
    AdmissibleAfterKillSwitchClear,
    /// Rerun is admissible after the managed-only channel is resolved.
    #[serde(rename = "rerun_under_current_policy_admissible_after_managed_channel_resolved")]
    AdmissibleAfterManagedChannelResolved,
    /// Rerun is blocked: the publisher was revoked.
    #[serde(rename = "rerun_under_current_policy_blocked_publisher_revoked")]
    BlockedPublisherRevoked,
    /// Rerun is blocked: a capability is disabled by policy.
    #[serde(rename = "rerun_under_current_policy_blocked_capability_disabled_by_policy")]
    BlockedCapabilityDisabledByPolicy,
    /// Rerun is blocked: the managed-only template was retired.
    #[serde(rename = "rerun_under_current_policy_blocked_managed_only_template_retired")]
    BlockedManagedOnlyTemplateRetired,
    /// Rerun is blocked: the recipe revision was retired.
    #[serde(rename = "rerun_under_current_policy_blocked_recipe_revision_retired")]
    BlockedRecipeRevisionRetired,
    /// Rerun is blocked: the replay window expired (stale authority).
    #[serde(rename = "rerun_under_current_policy_blocked_replay_window_expired")]
    BlockedReplayWindowExpired,
    /// Rerun is blocked: the descriptor revision was retired.
    #[serde(rename = "rerun_under_current_policy_blocked_descriptor_revision_retired")]
    BlockedDescriptorRevisionRetired,
    /// Rerun is blocked: the environment capsule drifted.
    #[serde(rename = "rerun_under_current_policy_blocked_environment_capsule_drift_detected")]
    BlockedEnvironmentCapsuleDrift,
    /// Rerun is blocked: the row is a macro recording locked to its subset.
    #[serde(rename = "rerun_under_current_policy_blocked_macro_recording_only")]
    BlockedMacroRecordingOnly,
    /// Rerun is blocked: the extension or external runner is unavailable.
    #[serde(
        rename = "rerun_under_current_policy_blocked_extension_or_external_runner_unavailable"
    )]
    BlockedExtensionOrExternalRunnerUnavailable,
    /// Rerun is blocked: the row was imported and carries no dispatch authority.
    #[serde(rename = "rerun_under_current_policy_blocked_imported_record")]
    BlockedImportedRecord,
}

impl RerunActionClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RerunActionClass::AdmissibleNoRevalidation => {
                "rerun_under_current_policy_admissible_no_revalidation_required"
            }
            RerunActionClass::AdmissibleAfterEnvironmentRevalidation => {
                "rerun_under_current_policy_admissible_after_environment_revalidation"
            }
            RerunActionClass::AdmissibleAfterFreshApproval => {
                "rerun_under_current_policy_admissible_after_fresh_approval"
            }
            RerunActionClass::AdmissibleAfterKillSwitchClear => {
                "rerun_under_current_policy_admissible_after_kill_switch_clear"
            }
            RerunActionClass::AdmissibleAfterManagedChannelResolved => {
                "rerun_under_current_policy_admissible_after_managed_channel_resolved"
            }
            RerunActionClass::BlockedPublisherRevoked => {
                "rerun_under_current_policy_blocked_publisher_revoked"
            }
            RerunActionClass::BlockedCapabilityDisabledByPolicy => {
                "rerun_under_current_policy_blocked_capability_disabled_by_policy"
            }
            RerunActionClass::BlockedManagedOnlyTemplateRetired => {
                "rerun_under_current_policy_blocked_managed_only_template_retired"
            }
            RerunActionClass::BlockedRecipeRevisionRetired => {
                "rerun_under_current_policy_blocked_recipe_revision_retired"
            }
            RerunActionClass::BlockedReplayWindowExpired => {
                "rerun_under_current_policy_blocked_replay_window_expired"
            }
            RerunActionClass::BlockedDescriptorRevisionRetired => {
                "rerun_under_current_policy_blocked_descriptor_revision_retired"
            }
            RerunActionClass::BlockedEnvironmentCapsuleDrift => {
                "rerun_under_current_policy_blocked_environment_capsule_drift_detected"
            }
            RerunActionClass::BlockedMacroRecordingOnly => {
                "rerun_under_current_policy_blocked_macro_recording_only"
            }
            RerunActionClass::BlockedExtensionOrExternalRunnerUnavailable => {
                "rerun_under_current_policy_blocked_extension_or_external_runner_unavailable"
            }
            RerunActionClass::BlockedImportedRecord => {
                "rerun_under_current_policy_blocked_imported_record"
            }
        }
    }

    /// Whether the class admits rerun (after the named revalidation, if any).
    pub fn is_admissible(self) -> bool {
        matches!(
            self,
            RerunActionClass::AdmissibleNoRevalidation
                | RerunActionClass::AdmissibleAfterEnvironmentRevalidation
                | RerunActionClass::AdmissibleAfterFreshApproval
                | RerunActionClass::AdmissibleAfterKillSwitchClear
                | RerunActionClass::AdmissibleAfterManagedChannelResolved
        )
    }

    /// Whether the class is an extension/external- or imported-only rerun state.
    ///
    /// A recorded macro must never resolve to one of these.
    pub fn is_extension_or_imported_only(self) -> bool {
        matches!(
            self,
            RerunActionClass::BlockedExtensionOrExternalRunnerUnavailable
                | RerunActionClass::BlockedImportedRecord
        )
    }
}

// ---------------------------------------------------------------------------
// Open-as-recipe action class
// ---------------------------------------------------------------------------

/// The open-as-recipe affordance a history row offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OpenAsRecipeActionClass {
    /// A recorded macro promotable to a declarative recipe.
    #[serde(rename = "open_as_recipe_admissible_macro_promotable_to_declarative_recipe")]
    AdmissibleMacroPromotable,
    /// An extension/external row authored as a declarative recipe.
    #[serde(
        rename = "open_as_recipe_admissible_extension_or_external_authored_as_declarative_recipe"
    )]
    AdmissibleExtensionOrExternalAuthored,
    /// No declarative capability path is admitted; cannot author a recipe.
    #[serde(rename = "open_as_recipe_inadmissible_no_declarative_capability_path_admitted")]
    InadmissibleNoDeclarativeCapabilityPath,
    /// The row is already a declarative recipe.
    #[serde(rename = "open_as_recipe_inadmissible_already_declarative_recipe")]
    InadmissibleAlreadyDeclarativeRecipe,
    /// The row is already a managed-only template.
    #[serde(rename = "open_as_recipe_inadmissible_already_managed_only_template")]
    InadmissibleAlreadyManagedOnlyTemplate,
    /// The row requires extension/external authority.
    #[serde(rename = "open_as_recipe_inadmissible_extension_or_external_authority_required")]
    InadmissibleExtensionOrExternalAuthorityRequired,
}

impl OpenAsRecipeActionClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            OpenAsRecipeActionClass::AdmissibleMacroPromotable => {
                "open_as_recipe_admissible_macro_promotable_to_declarative_recipe"
            }
            OpenAsRecipeActionClass::AdmissibleExtensionOrExternalAuthored => {
                "open_as_recipe_admissible_extension_or_external_authored_as_declarative_recipe"
            }
            OpenAsRecipeActionClass::InadmissibleNoDeclarativeCapabilityPath => {
                "open_as_recipe_inadmissible_no_declarative_capability_path_admitted"
            }
            OpenAsRecipeActionClass::InadmissibleAlreadyDeclarativeRecipe => {
                "open_as_recipe_inadmissible_already_declarative_recipe"
            }
            OpenAsRecipeActionClass::InadmissibleAlreadyManagedOnlyTemplate => {
                "open_as_recipe_inadmissible_already_managed_only_template"
            }
            OpenAsRecipeActionClass::InadmissibleExtensionOrExternalAuthorityRequired => {
                "open_as_recipe_inadmissible_extension_or_external_authority_required"
            }
        }
    }

    /// Whether this open-as-recipe class is admissible for the given layer.
    ///
    /// This is the capability-laundering guard: a declarative recipe is already a
    /// recipe, a managed-only template is already managed, a macro is promotable or
    /// not, and an extension/external row is authored, requires authority, or has
    /// no declarative path — but never silently lifts a capability into a recipe.
    pub fn admissible_for_layer(self, layer: AutomationLayerClass) -> bool {
        match layer {
            AutomationLayerClass::RecordedMacro => matches!(
                self,
                OpenAsRecipeActionClass::AdmissibleMacroPromotable
                    | OpenAsRecipeActionClass::InadmissibleNoDeclarativeCapabilityPath
            ),
            AutomationLayerClass::DeclarativeRecipe | AutomationLayerClass::HeadlessSafeRun => {
                matches!(
                    self,
                    OpenAsRecipeActionClass::InadmissibleAlreadyDeclarativeRecipe
                )
            }
            AutomationLayerClass::ManagedOnlyTemplate => matches!(
                self,
                OpenAsRecipeActionClass::InadmissibleAlreadyManagedOnlyTemplate
            ),
            AutomationLayerClass::ExtensionOrExternalAutomation => matches!(
                self,
                OpenAsRecipeActionClass::AdmissibleExtensionOrExternalAuthored
                    | OpenAsRecipeActionClass::InadmissibleNoDeclarativeCapabilityPath
                    | OpenAsRecipeActionClass::InadmissibleExtensionOrExternalAuthorityRequired
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Run-history entry (the live evidence-panel row)
// ---------------------------------------------------------------------------

/// An error raised by a [`RunHistoryPanel`] mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunHistoryError {
    /// An entry with the given id is already present.
    DuplicateEntryId(String),
}

impl std::fmt::Display for RunHistoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunHistoryError::DuplicateEntryId(id) => {
                write!(formatter, "an entry with id {id} is already present")
            }
        }
    }
}

impl std::error::Error for RunHistoryError {}

/// One attributable run in a history / evidence panel.
///
/// The entry records a single attempted dispatch with stable run identity, the
/// automation layer and execution mode it ran under, the result class, the
/// artifact links it produced, its retention/redaction and artifact-bundle state,
/// the context it observed, the secret-broker handles it referenced, and the
/// open-as-recipe affordance it offers. The rerun action is **derived**, never
/// stored as authority: [`RunHistoryEntry::resolved_rerun_class`] resolves it from
/// the layer, the imported state, and the observed [`CurrentPolicyBlocker`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryEntry {
    /// Opaque entry id.
    pub entry_id: String,
    /// The M5 automation family the run belongs to.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Stable run identity.
    pub run_identity: RunIdentity,
    /// The automation layer the run was minted under.
    pub automation_layer: AutomationLayerClass,
    /// Integer schema version of the underlying run record.
    pub record_schema_version: u32,
    /// The execution mode the run was dispatched through.
    pub execution_mode: ExecutionModeClass,
    /// The result class the run reported.
    pub result_class: RunResultClass,
    /// Whether the underlying record crossed the import boundary.
    pub imported: bool,
    /// The artifact links the run produced.
    pub artifact_links: Vec<ArtifactLink>,
    /// Opaque secret-broker handles the run referenced; never raw secret values.
    pub secret_reference_refs: Vec<String>,
    /// The retention window the row tracks.
    pub retention_class: RetentionClass,
    /// Non-null only for a windowed retention class.
    pub retention_window_expires_at: Option<String>,
    /// The redaction mode the row's safe summary carries.
    pub redaction_mode: RedactionModeClass,
    /// The state of the artifact bundle the run produced.
    pub artifact_bundle_state: ArtifactBundleStateClass,
    /// Opaque artifact-bundle ref, non-null only when the bundle is available.
    pub artifact_bundle_ref: Option<String>,
    /// The context block describing where the run executed.
    pub context_summary: ContextSummary,
    /// The blockers the resolver observed at projection time.
    pub current_policy_blockers: Vec<CurrentPolicyBlocker>,
    /// The open-as-recipe affordance the row offers.
    pub open_as_recipe_action_class: OpenAsRecipeActionClass,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// Monotonic dispatch timestamp.
    pub dispatched_at: String,
    /// Monotonic completion timestamp, or `null` while the run is queued.
    pub completed_at: Option<String>,
}

impl RunHistoryEntry {
    /// The resolved rerun action, derived from the layer, imported state, and blockers.
    ///
    /// An imported row always resolves to [`RerunActionClass::BlockedImportedRecord`].
    /// Otherwise a denial blocker dominates a revalidation blocker, which dominates
    /// no blocker; among same-disposition blockers the canonical order wins.
    pub fn resolved_rerun_class(&self) -> RerunActionClass {
        derive_rerun_class(self.imported, &self.current_policy_blockers)
    }

    /// Whether the resolved rerun admits a rerun today (after revalidation, if any).
    pub fn rerun_admissible(&self) -> bool {
        self.resolved_rerun_class().is_admissible()
    }

    /// Resolves rerun-under-current-policy into an explicit, attributable record.
    ///
    /// The resolution is the proof rerun resolves fresh authority: it asserts that
    /// the rerun resolves current policy, never reuses a cached approval, never
    /// reuses a stale environment, and re-resolves every secret reference.
    pub fn resolve_rerun(&self, resolved_at: impl Into<String>) -> RerunResolution {
        let rerun_action_class = self.resolved_rerun_class();
        RerunResolution {
            record_kind: RERUN_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
            resolved_at: resolved_at.into(),
            entry_id: self.entry_id.clone(),
            run_id: self.run_identity.run_id.clone(),
            rerun_action_class,
            current_policy_blockers: self.current_policy_blockers.clone(),
            admissible: rerun_action_class.is_admissible(),
            resolves_current_policy: true,
            reuses_cached_approval: false,
            reuses_stale_environment: false,
            secret_references_reresolved: true,
            summary: format!(
                "rerun resolves current policy now; {} blocker(s) observed",
                self.current_policy_blockers.len()
            ),
        }
    }

    /// Whether the entry's rerun derivation is internally consistent.
    ///
    /// The no-blocker pairing holds (admissible-no-revalidation pairs with exactly
    /// `[NoBlockerPresent]`; any other class cites a non-no-blocker entry and no
    /// `NoBlockerPresent`), an imported row resolves to the imported-blocked class,
    /// and a recorded macro never resolves to an extension/external or imported
    /// rerun class.
    pub fn rerun_consistent(&self) -> bool {
        let resolved = self.resolved_rerun_class();
        let has_no_blocker = self
            .current_policy_blockers
            .contains(&CurrentPolicyBlocker::NoBlockerPresent);
        let pairing_ok = if resolved == RerunActionClass::AdmissibleNoRevalidation {
            self.current_policy_blockers == [CurrentPolicyBlocker::NoBlockerPresent]
        } else {
            !has_no_blocker
                && self
                    .current_policy_blockers
                    .iter()
                    .any(|blocker| blocker.rerun_action_class() == resolved)
        };
        let imported_ok = !self.imported || resolved == RerunActionClass::BlockedImportedRecord;
        let macro_ok = self.automation_layer != AutomationLayerClass::RecordedMacro
            || !resolved.is_extension_or_imported_only();
        pairing_ok && imported_ok && macro_ok
    }

    /// Whether the open-as-recipe affordance is admissible for the entry's layer.
    pub fn open_as_recipe_consistent(&self) -> bool {
        self.open_as_recipe_action_class
            .admissible_for_layer(self.automation_layer)
    }

    /// Whether every secret reference is an opaque broker handle, not a raw value.
    ///
    /// A reference that does not look like an opaque handle (e.g. it embeds a raw
    /// marker) is treated as a redaction violation.
    pub fn secret_references_opaque(&self) -> bool {
        self.secret_reference_refs
            .iter()
            .all(|reference| reference_is_opaque(reference))
    }

    /// Whether the retention/artifact-bundle posture is internally consistent.
    pub fn retention_consistent(&self) -> bool {
        let window_ok =
            self.retention_class.is_windowed() == self.retention_window_expires_at.is_some();
        let bundle_ok =
            self.artifact_bundle_state.carries_bundle_ref() == self.artifact_bundle_ref.is_some();
        window_ok && bundle_ok
    }

    /// Projects the entry onto an attributable run-history evidence row.
    pub fn to_evidence_row(
        &self,
        row_id: impl Into<String>,
        recorded_at: impl Into<String>,
    ) -> RunHistoryEvidenceRow {
        let rerun_action_class = self.resolved_rerun_class();
        RunHistoryEvidenceRow {
            record_kind: RUN_HISTORY_EVIDENCE_ROW_RECORD_KIND.to_owned(),
            schema_version: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
            row_id: row_id.into(),
            recorded_at: recorded_at.into(),
            entry_id: self.entry_id.clone(),
            entrypoint: self.entrypoint,
            run_identity: self.run_identity.clone(),
            automation_layer: self.automation_layer,
            record_schema_version: self.record_schema_version,
            execution_mode: self.execution_mode,
            result_class: self.result_class,
            imported: self.imported,
            rerun_action_class,
            rerun_admissible: rerun_action_class.is_admissible(),
            current_policy_blockers: self.current_policy_blockers.clone(),
            open_as_recipe_action_class: self.open_as_recipe_action_class,
            retention_class: self.retention_class,
            retention_window_expires_at: self.retention_window_expires_at.clone(),
            redaction_mode: self.redaction_mode,
            artifact_bundle_state: self.artifact_bundle_state,
            artifact_bundle_ref: self.artifact_bundle_ref.clone(),
            trust_state_class: self.context_summary.trust_state_class,
            policy_observation_class: self.context_summary.policy_observation_class,
            kill_switch_observation_class: self.context_summary.kill_switch_observation_class,
            artifact_link_count: self.artifact_links.len() as u32,
            secret_reference_count: self.secret_reference_refs.len() as u32,
            entry_digest: self.entry_digest(),
            run_record_schema_ref: RUN_RECORD_SCHEMA_REF.to_owned(),
            run_history_row_schema_ref: RUN_HISTORY_ROW_SCHEMA_REF.to_owned(),
            run_summary_export_schema_ref: RUN_SUMMARY_EXPORT_SCHEMA_REF.to_owned(),
        }
    }

    /// Exports the entry, carrying its evidence row and resolved rerun.
    pub fn export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RunHistoryEvidenceExport {
        let exported_at = exported_at.into();
        RunHistoryEvidenceExport {
            record_kind: RUN_HISTORY_EVIDENCE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.clone(),
            evidence_row: self.to_evidence_row(
                format!("run-history:{}", self.entry_id),
                exported_at.clone(),
            ),
            rerun_resolution: self.resolve_rerun(exported_at),
            entry: self.clone(),
            export_digest: self.entry_digest(),
        }
    }

    /// Order-stable digest over the entry's identity, layer, and blockers.
    pub fn entry_digest(&self) -> String {
        fnv1a64(&self.digest_tokens())
    }

    fn digest_tokens(&self) -> Vec<String> {
        let mut tokens = vec![
            self.entry_id.clone(),
            self.run_identity.run_id.clone(),
            self.run_identity.manifest_id.clone(),
            self.run_identity.manifest_revision_ref.clone(),
            self.automation_layer.as_str().to_owned(),
            self.execution_mode.as_str().to_owned(),
            self.result_class.as_str().to_owned(),
            self.resolved_rerun_class().as_str().to_owned(),
        ];
        for blocker in &self.current_policy_blockers {
            tokens.push(blocker.as_str().to_owned());
        }
        for link in &self.artifact_links {
            tokens.push(link.link_class.as_str().to_owned());
            tokens.push(link.artifact_ref.clone());
        }
        tokens
    }
}

/// Whether a reference looks like an opaque, redaction-safe handle.
fn reference_is_opaque(reference: &str) -> bool {
    !reference.is_empty()
        && !reference.contains("raw:")
        && !reference.contains("://")
        && !reference.starts_with('/')
}

// ---------------------------------------------------------------------------
// Rerun resolution
// ---------------------------------------------------------------------------

/// An explicit rerun-under-current-policy resolution minted from an entry.
///
/// The record is the enforcement point for "history is evidence, not authority":
/// it carries the derived [`RerunActionClass`], the blockers observed now, and the
/// four assertions that rerun resolves fresh — it resolves current policy, reuses
/// no cached approval, reuses no stale environment, and re-resolves every secret
/// reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunResolution {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Monotonic resolution timestamp.
    pub resolved_at: String,
    /// Opaque entry id the resolution is for.
    pub entry_id: String,
    /// Opaque run id the resolution is for.
    pub run_id: String,
    /// The resolved rerun action class.
    pub rerun_action_class: RerunActionClass,
    /// The blockers observed at resolution time.
    pub current_policy_blockers: Vec<CurrentPolicyBlocker>,
    /// Whether rerun is admissible today (after the named revalidation, if any).
    pub admissible: bool,
    /// Always true: the rerun resolves current policy.
    pub resolves_current_policy: bool,
    /// Always false: the rerun reuses no cached approval.
    pub reuses_cached_approval: bool,
    /// Always false: the rerun reuses no stale environment.
    pub reuses_stale_environment: bool,
    /// Always true: every secret reference is re-resolved.
    pub secret_references_reresolved: bool,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl RerunResolution {
    /// Whether the resolution never implies cached approval or stale environment.
    pub fn is_fresh(&self) -> bool {
        self.resolves_current_policy
            && !self.reuses_cached_approval
            && !self.reuses_stale_environment
            && self.secret_references_reresolved
    }
}

// ---------------------------------------------------------------------------
// Run-history evidence row
// ---------------------------------------------------------------------------

/// An attributable run-history evidence row projected from an entry.
///
/// The row is the canonical object support packets, incident/runbook follow-up, AI
/// evidence joins, and CLI/headless inspect surfaces ingest. It carries the run
/// identity, automation layer, schema version, execution mode, result class,
/// resolved rerun action, retention/redaction posture, artifact-bundle state, and
/// the context observation classes — but never a raw path, URL, secret, or argv.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryEvidenceRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque run-history row id.
    pub row_id: String,
    /// Monotonic record timestamp.
    pub recorded_at: String,
    /// Opaque entry id this row records.
    pub entry_id: String,
    /// The entrypoint the run belongs to.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Stable run identity.
    pub run_identity: RunIdentity,
    /// The automation layer the run ran under.
    pub automation_layer: AutomationLayerClass,
    /// Integer schema version of the underlying run record.
    pub record_schema_version: u32,
    /// The execution mode the run was dispatched through.
    pub execution_mode: ExecutionModeClass,
    /// The result class the run reported.
    pub result_class: RunResultClass,
    /// Whether the underlying record crossed the import boundary.
    pub imported: bool,
    /// The resolved rerun action class.
    pub rerun_action_class: RerunActionClass,
    /// Whether rerun is admissible today.
    pub rerun_admissible: bool,
    /// The blockers observed at projection time.
    pub current_policy_blockers: Vec<CurrentPolicyBlocker>,
    /// The open-as-recipe affordance the row offers.
    pub open_as_recipe_action_class: OpenAsRecipeActionClass,
    /// The retention window the row tracks.
    pub retention_class: RetentionClass,
    /// Non-null only for a windowed retention class.
    pub retention_window_expires_at: Option<String>,
    /// The redaction mode the row's safe summary carries.
    pub redaction_mode: RedactionModeClass,
    /// The state of the artifact bundle.
    pub artifact_bundle_state: ArtifactBundleStateClass,
    /// Opaque artifact-bundle ref, non-null only when available.
    pub artifact_bundle_ref: Option<String>,
    /// The workspace-trust state the run observed.
    pub trust_state_class: TrustStateClass,
    /// The admin-policy observation the run recorded.
    pub policy_observation_class: PolicyObservationClass,
    /// The kill-switch observation the run recorded.
    pub kill_switch_observation_class: KillSwitchObservationClass,
    /// Count of artifact links.
    pub artifact_link_count: u32,
    /// Count of secret references (opaque broker handles only).
    pub secret_reference_count: u32,
    /// Order-stable entry digest carried for verification.
    pub entry_digest: String,
    /// Schema each dispatch mints a run record against.
    pub run_record_schema_ref: String,
    /// Schema this row conforms to in run history.
    pub run_history_row_schema_ref: String,
    /// Schema a safe-summary export of this row conforms to.
    pub run_summary_export_schema_ref: String,
}

// ---------------------------------------------------------------------------
// Run-history evidence export
// ---------------------------------------------------------------------------

/// A run-history entry exported for rerun review, comparison, or support bundles.
///
/// The export nests the whole [`RunHistoryEntry`] verbatim alongside the derived
/// evidence row and the resolved rerun, plus an order-stable digest.
/// [`RunHistoryEvidenceExport::import`] reconstructs the identical entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryEvidenceExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// The attributable evidence row consumers read.
    pub evidence_row: RunHistoryEvidenceRow,
    /// The rerun resolution resolved at export time.
    pub rerun_resolution: RerunResolution,
    /// The entry, preserved verbatim for round-trip import.
    pub entry: RunHistoryEntry,
    /// Order-stable digest over the entry.
    pub export_digest: String,
}

impl RunHistoryEvidenceExport {
    /// Reconstructs the entry from the export.
    pub fn import(&self) -> RunHistoryEntry {
        self.entry.clone()
    }

    /// Whether the export preserves identity and rerun truth across the boundary.
    ///
    /// The evidence row must carry the same run identity and resolved rerun the
    /// entry derives, the rerun resolution must be fresh, and the digests must
    /// agree — so the run stays comparable and explainable after export, history,
    /// and support without losing identity or implying cached authority.
    pub fn identity_and_rerun_preserved(&self) -> bool {
        self.evidence_row.run_identity == self.entry.run_identity
            && self.evidence_row.rerun_action_class == self.entry.resolved_rerun_class()
            && self.rerun_resolution.rerun_action_class == self.entry.resolved_rerun_class()
            && self.rerun_resolution.is_fresh()
            && self.evidence_row.entry_digest == self.export_digest
            && self.entry.rerun_consistent()
    }
}

// ---------------------------------------------------------------------------
// First-consumer binding
// ---------------------------------------------------------------------------

/// One entrypoint binding: the seeded run-history / evidence panel a consumer renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryConsumerBinding {
    /// The entrypoint this binding describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// The ordered run-history entries the panel renders (newest first).
    pub entries: Vec<RunHistoryEntry>,
    /// The attributable evidence rows the panel projects, index-aligned with entries.
    pub evidence_rows: Vec<RunHistoryEvidenceRow>,
    /// Count of entries.
    pub entry_count: u32,
    /// Count of entries whose rerun is admissible today.
    pub admissible_rerun_count: u32,
    /// Count of entries whose rerun is blocked today.
    pub blocked_rerun_count: u32,
    /// Count of imported entries.
    pub imported_count: u32,
    /// The run id of the latest entry.
    pub latest_run_id: String,
    /// The automation layer of the latest entry.
    pub latest_layer: AutomationLayerClass,
    /// The result class of the latest entry.
    pub latest_result_class: RunResultClass,
    /// The resolved rerun action of the latest entry.
    pub latest_rerun_action_class: RerunActionClass,
    /// Reviewable summary of what the consumer renders.
    pub entry_summary: String,
}

impl RunHistoryConsumerBinding {
    /// Builds a binding from a consumer's seeded panel of entries.
    ///
    /// The entries are rendered newest-first; the first entry is the latest.
    pub fn from_entries(
        entrypoint: RecipeBuilderEntrypoint,
        entries: Vec<RunHistoryEntry>,
        entry_summary: impl Into<String>,
    ) -> Self {
        let latest = entries
            .first()
            .expect("a binding must carry at least one entry");
        let evidence_rows = entries
            .iter()
            .map(|entry| {
                entry.to_evidence_row(
                    format!("run-history:{}", entry.entry_id),
                    entry.dispatched_at.clone(),
                )
            })
            .collect();
        let admissible_rerun_count = entries
            .iter()
            .filter(|entry| entry.rerun_admissible())
            .count() as u32;
        let blocked_rerun_count = entries.len() as u32 - admissible_rerun_count;
        let imported_count = entries.iter().filter(|entry| entry.imported).count() as u32;
        RunHistoryConsumerBinding {
            entrypoint,
            title: entrypoint.title().to_owned(),
            entry_count: entries.len() as u32,
            admissible_rerun_count,
            blocked_rerun_count,
            imported_count,
            latest_run_id: latest.run_identity.run_id.clone(),
            latest_layer: latest.automation_layer,
            latest_result_class: latest.result_class,
            latest_rerun_action_class: latest.resolved_rerun_class(),
            entry_summary: entry_summary.into(),
            evidence_rows,
            entries,
        }
    }
}

// ---------------------------------------------------------------------------
// Invariants and findings
// ---------------------------------------------------------------------------

/// Frozen invariants the first-consumers packet pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryInvariantsBlock {
    /// Every first-consumer entrypoint binds a non-empty panel.
    pub every_entrypoint_binds_a_panel: bool,
    /// Every entry resolves a run identity and an automation layer.
    pub every_entry_resolves_run_identity_and_layer: bool,
    /// Rerun resolves current policy, never implying cached approval.
    pub rerun_resolves_current_policy_never_cached_approval: bool,
    /// The current-policy blockers are the authoritative reason rerun is denied.
    pub current_policy_blockers_are_authoritative: bool,
    /// Imported records never offer a one-click rerun.
    pub imported_records_never_offer_rerun: bool,
    /// Recorded macros never offer extension or external rerun.
    pub macro_rows_never_offer_external_rerun: bool,
    /// Open-as-recipe never launders a capability into a recipe.
    pub open_as_recipe_never_launders_capability: bool,
    /// Raw secrets never appear in a history row.
    pub raw_secrets_never_appear_in_history: bool,
    /// History reuses the canonical run-record and run-history-row schemas.
    pub history_reuses_canonical_run_record_and_row_schema: bool,
}

impl RunHistoryInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        RunHistoryInvariantsBlock {
            every_entrypoint_binds_a_panel: true,
            every_entry_resolves_run_identity_and_layer: true,
            rerun_resolves_current_policy_never_cached_approval: true,
            current_policy_blockers_are_authoritative: true,
            imported_records_never_offer_rerun: true,
            macro_rows_never_offer_external_rerun: true,
            open_as_recipe_never_launders_capability: true,
            raw_secrets_never_appear_in_history: true,
            history_reuses_canonical_run_record_and_row_schema: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 9] {
        [
            (
                "every_entrypoint_binds_a_panel",
                self.every_entrypoint_binds_a_panel,
            ),
            (
                "every_entry_resolves_run_identity_and_layer",
                self.every_entry_resolves_run_identity_and_layer,
            ),
            (
                "rerun_resolves_current_policy_never_cached_approval",
                self.rerun_resolves_current_policy_never_cached_approval,
            ),
            (
                "current_policy_blockers_are_authoritative",
                self.current_policy_blockers_are_authoritative,
            ),
            (
                "imported_records_never_offer_rerun",
                self.imported_records_never_offer_rerun,
            ),
            (
                "macro_rows_never_offer_external_rerun",
                self.macro_rows_never_offer_external_rerun,
            ),
            (
                "open_as_recipe_never_launders_capability",
                self.open_as_recipe_never_launders_capability,
            ),
            (
                "raw_secrets_never_appear_in_history",
                self.raw_secrets_never_appear_in_history,
            ),
            (
                "history_reuses_canonical_run_record_and_row_schema",
                self.history_reuses_canonical_run_record_and_row_schema,
            ),
        ]
    }
}

/// Severity of a run-history validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunHistoryFindingSeverity {
    /// Blocks the packet from stable.
    Blocker,
    /// Narrows the packet below stable.
    Warning,
}

/// Kind of a run-history validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunHistoryFindingKind {
    /// A required first-consumer entrypoint is absent.
    MissingEntrypoint,
    /// An entrypoint binds a panel with no entries.
    EntrypointPanelEmpty,
    /// A rerun implies cached approval (the no-blocker pairing is violated).
    RerunImpliesCachedApproval,
    /// An imported row offers a one-click rerun.
    ImportedRowOffersRerun,
    /// A recorded macro offers extension or external rerun.
    MacroOffersExternalRerun,
    /// An open-as-recipe affordance launders a capability into a recipe.
    CapabilityLaunderedIntoRecipe,
    /// A raw secret value appears in a history row.
    RawSecretMaterialInHistory,
    /// The projected evidence row disagrees with the live entry.
    EvidenceRowProjectionInconsistent,
    /// The retention/artifact-bundle posture is inconsistent.
    RetentionPostureInconsistent,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl RunHistoryFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RunHistoryFindingKind::MissingEntrypoint => "missing_entrypoint",
            RunHistoryFindingKind::EntrypointPanelEmpty => "entrypoint_panel_empty",
            RunHistoryFindingKind::RerunImpliesCachedApproval => "rerun_implies_cached_approval",
            RunHistoryFindingKind::ImportedRowOffersRerun => "imported_row_offers_rerun",
            RunHistoryFindingKind::MacroOffersExternalRerun => "macro_offers_external_rerun",
            RunHistoryFindingKind::CapabilityLaunderedIntoRecipe => {
                "capability_laundered_into_recipe"
            }
            RunHistoryFindingKind::RawSecretMaterialInHistory => "raw_secret_material_in_history",
            RunHistoryFindingKind::EvidenceRowProjectionInconsistent => {
                "evidence_row_projection_inconsistent"
            }
            RunHistoryFindingKind::RetentionPostureInconsistent => "retention_posture_inconsistent",
            RunHistoryFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by the first-consumers gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryFinding {
    /// The finding kind.
    pub finding_kind: RunHistoryFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: RunHistoryFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl RunHistoryFinding {
    fn blocker(
        finding_kind: RunHistoryFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        RunHistoryFinding {
            finding_kind,
            severity: RunHistoryFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// First-consumers packet
// ---------------------------------------------------------------------------

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryFirstConsumersInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<RunHistoryConsumerBinding>,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: RunHistoryInvariantsBlock,
}

/// Canonical M5 run-history first-consumers packet.
///
/// The packet binds every first-consumer entrypoint to a seeded panel and pins the
/// freeze invariants. [`RunHistoryFirstConsumersPacket::validate`] recomputes the
/// findings so the fail-closed gate and the typed consumer agree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryFirstConsumersPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Boundary schema ref for this packet.
    pub schema_ref: String,
    /// Reused run-history-row boundary schema ref.
    pub run_history_row_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<RunHistoryConsumerBinding>,
    /// Frozen invariants block.
    pub invariants: RunHistoryInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<RunHistoryFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over entrypoint and entry tokens.
    pub packet_digest: String,
}

impl RunHistoryFirstConsumersPacket {
    /// Freezes an input into a packet, computing findings, promotion, and digest.
    pub fn materialize(input: RunHistoryFirstConsumersInput) -> Self {
        let findings = validate_parts(&input.consumer_bindings, &input.invariants);
        let promotion_state = promotion_state_for_findings(&findings);
        let packet_digest = packet_digest(&input.consumer_bindings);
        RunHistoryFirstConsumersPacket {
            record_kind: RUN_HISTORY_FIRST_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            schema_ref: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            run_history_row_schema_ref: RUN_HISTORY_ROW_SCHEMA_REF.to_owned(),
            doc_ref: RUN_HISTORY_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            consumer_bindings: input.consumer_bindings,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            packet_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<RunHistoryFinding> {
        validate_parts(&self.consumer_bindings, &self.invariants)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// The binding for one entrypoint, if present.
    pub fn binding(
        &self,
        entrypoint: RecipeBuilderEntrypoint,
    ) -> Option<&RunHistoryConsumerBinding> {
        self.consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
    }

    /// Entrypoint tokens in the order the packet stores them.
    pub fn entrypoint_tokens(&self) -> Vec<&'static str> {
        self.consumer_bindings
            .iter()
            .map(|binding| binding.entrypoint.as_str())
            .collect()
    }

    /// Every evidence row across every binding, for support and AI evidence joins.
    pub fn all_evidence_rows(&self) -> Vec<RunHistoryEvidenceRow> {
        self.consumer_bindings
            .iter()
            .flat_map(|binding| binding.evidence_rows.iter().cloned())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RunHistoryFirstConsumersSupportExport {
        RunHistoryFirstConsumersSupportExport {
            record_kind: RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            packet_digest: self.packet_digest.clone(),
            promotion_state: self.promotion_state,
            consumer_rows: self
                .consumer_bindings
                .iter()
                .map(RunHistorySupportConsumerRow::from_binding)
                .collect(),
            evidence_rows: self.all_evidence_rows(),
            invariants: self.invariants.clone(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind)
                .collect(),
        }
    }

    /// Builds the compact CLI / headless projection.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> RunHistoryFirstConsumersCliHeadlessView {
        RunHistoryFirstConsumersCliHeadlessView {
            record_kind: RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            consumer_lines: self
                .consumer_bindings
                .iter()
                .map(|binding| {
                    format!(
                        "{} entries={} latest_run={} layer={} result={} rerun={} admissible={} blocked={} imported={}",
                        binding.entrypoint.as_str(),
                        binding.entry_count,
                        binding.latest_run_id,
                        binding.latest_layer.as_str(),
                        binding.latest_result_class.as_str(),
                        binding.latest_rerun_action_class.as_str(),
                        binding.admissible_rerun_count,
                        binding.blocked_rerun_count,
                        binding.imported_count,
                    )
                })
                .collect(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} consumers={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.consumer_bindings.len(),
            self.packet_digest,
        )];
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} entries={} latest_run={} layer={} result={} admissible={} blocked={} imported={}",
                binding.entrypoint.as_str(),
                binding.entry_count,
                binding.latest_run_id,
                binding.latest_layer.as_str(),
                binding.latest_result_class.as_str(),
                binding.admissible_rerun_count,
                binding.blocked_rerun_count,
                binding.imported_count,
            ));
            for entry in &binding.entries {
                lines.push(format!(
                    "  entry {} run={} layer={} mode={} result={} rerun={} open_as_recipe={} retention={} bundle={} imported={}",
                    entry.entry_id,
                    entry.run_identity.run_id,
                    entry.automation_layer.as_str(),
                    entry.execution_mode.as_str(),
                    entry.result_class.as_str(),
                    entry.resolved_rerun_class().as_str(),
                    entry.open_as_recipe_action_class.as_str(),
                    entry.retention_class.as_str(),
                    entry.artifact_bundle_state.as_str(),
                    entry.imported,
                ));
            }
        }
        lines
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// One redacted support-export entry row (no raw path, URL, or content).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistorySupportEntryRow {
    /// Opaque entry id.
    pub entry_id: String,
    /// Opaque run id.
    pub run_id: String,
    /// The automation layer.
    pub automation_layer: AutomationLayerClass,
    /// The execution mode.
    pub execution_mode: ExecutionModeClass,
    /// The result class.
    pub result_class: RunResultClass,
    /// The resolved rerun action class.
    pub rerun_action_class: RerunActionClass,
    /// Whether rerun is admissible today.
    pub rerun_admissible: bool,
    /// The open-as-recipe affordance.
    pub open_as_recipe_action_class: OpenAsRecipeActionClass,
    /// The retention window.
    pub retention_class: RetentionClass,
    /// The redaction mode.
    pub redaction_mode: RedactionModeClass,
    /// The artifact-bundle state.
    pub artifact_bundle_state: ArtifactBundleStateClass,
    /// The current-policy blockers observed.
    pub current_policy_blockers: Vec<CurrentPolicyBlocker>,
    /// Whether the record was imported.
    pub imported: bool,
}

impl RunHistorySupportEntryRow {
    fn from_entry(entry: &RunHistoryEntry) -> Self {
        let rerun_action_class = entry.resolved_rerun_class();
        RunHistorySupportEntryRow {
            entry_id: entry.entry_id.clone(),
            run_id: entry.run_identity.run_id.clone(),
            automation_layer: entry.automation_layer,
            execution_mode: entry.execution_mode,
            result_class: entry.result_class,
            rerun_action_class,
            rerun_admissible: rerun_action_class.is_admissible(),
            open_as_recipe_action_class: entry.open_as_recipe_action_class,
            retention_class: entry.retention_class,
            redaction_mode: entry.redaction_mode,
            artifact_bundle_state: entry.artifact_bundle_state,
            current_policy_blockers: entry.current_policy_blockers.clone(),
            imported: entry.imported,
        }
    }
}

/// One redacted support-export consumer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistorySupportConsumerRow {
    /// The entrypoint this row describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Count of entries.
    pub entry_count: u32,
    /// The latest run id.
    pub latest_run_id: String,
    /// The latest automation layer.
    pub latest_layer: AutomationLayerClass,
    /// The latest result class.
    pub latest_result_class: RunResultClass,
    /// The latest resolved rerun action.
    pub latest_rerun_action_class: RerunActionClass,
    /// Count of entries whose rerun is admissible today.
    pub admissible_rerun_count: u32,
    /// Count of entries whose rerun is blocked today.
    pub blocked_rerun_count: u32,
    /// Count of imported entries.
    pub imported_count: u32,
    /// Per-entry redacted rows.
    pub entry_rows: Vec<RunHistorySupportEntryRow>,
}

impl RunHistorySupportConsumerRow {
    fn from_binding(binding: &RunHistoryConsumerBinding) -> Self {
        RunHistorySupportConsumerRow {
            entrypoint: binding.entrypoint,
            title: binding.title.clone(),
            entry_count: binding.entry_count,
            latest_run_id: binding.latest_run_id.clone(),
            latest_layer: binding.latest_layer,
            latest_result_class: binding.latest_result_class,
            latest_rerun_action_class: binding.latest_rerun_action_class,
            admissible_rerun_count: binding.admissible_rerun_count,
            blocked_rerun_count: binding.blocked_rerun_count,
            imported_count: binding.imported_count,
            entry_rows: binding
                .entries
                .iter()
                .map(RunHistorySupportEntryRow::from_entry)
                .collect(),
        }
    }
}

/// Redacted support-export projection of the first-consumers packet.
///
/// The export carries the per-entry layer, mode, result, rerun action, and blocker
/// classes plus the attributable evidence rows, so a run-history panel is
/// reviewable in a support bundle — and comparable across earlier runs — without a
/// raw path, URL, or secret ever crossing the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryFirstConsumersSupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// Packet id this export was minted from.
    pub packet_id: String,
    /// Packet digest carried for verification.
    pub packet_digest: String,
    /// Promotion state of the source packet.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Consumer rows.
    pub consumer_rows: Vec<RunHistorySupportConsumerRow>,
    /// Attributable evidence rows carried for support, incident, and AI joins.
    pub evidence_rows: Vec<RunHistoryEvidenceRow>,
    /// Frozen invariants block.
    pub invariants: RunHistoryInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<RunHistoryFindingKind>,
}

impl RunHistoryFirstConsumersSupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.packet_digest.is_empty()
            && !self.consumer_rows.is_empty()
            && !self.evidence_rows.is_empty()
    }
}

/// Compact CLI / headless projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistoryFirstConsumersCliHeadlessView {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Monotonic generation timestamp.
    pub generated_at: String,
    /// Packet id this view was minted from.
    pub packet_id: String,
    /// Promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// One line per consumer entrypoint.
    pub consumer_lines: Vec<String>,
}

impl RunHistoryFirstConsumersCliHeadlessView {
    /// Whether the view explains every entrypoint.
    pub fn every_entrypoint_explained(&self) -> bool {
        self.consumer_lines.len() == RecipeBuilderEntrypoint::ALL.len()
    }
}

// ---------------------------------------------------------------------------
// Derivations
// ---------------------------------------------------------------------------

/// Derives the resolved rerun class from the imported state and observed blockers.
fn derive_rerun_class(imported: bool, blockers: &[CurrentPolicyBlocker]) -> RerunActionClass {
    if imported {
        return RerunActionClass::BlockedImportedRecord;
    }
    // A denial dominates a revalidation, which dominates no blocker; among
    // same-disposition blockers the canonical CurrentPolicyBlocker::ALL order wins.
    let mut denial: Option<CurrentPolicyBlocker> = None;
    let mut revalidation: Option<CurrentPolicyBlocker> = None;
    for candidate in CurrentPolicyBlocker::ALL {
        if !blockers.contains(&candidate) {
            continue;
        }
        match candidate.disposition() {
            RerunDisposition::Denies if denial.is_none() => denial = Some(candidate),
            RerunDisposition::RequiresRevalidation if revalidation.is_none() => {
                revalidation = Some(candidate)
            }
            _ => {}
        }
    }
    if let Some(blocker) = denial {
        return blocker.rerun_action_class();
    }
    if let Some(blocker) = revalidation {
        return blocker.rerun_action_class();
    }
    RerunActionClass::AdmissibleNoRevalidation
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_parts(
    consumer_bindings: &[RunHistoryConsumerBinding],
    invariants: &RunHistoryInvariantsBlock,
) -> Vec<RunHistoryFinding> {
    let mut findings = Vec::new();

    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let Some(binding) = consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
        else {
            findings.push(RunHistoryFinding::blocker(
                RunHistoryFindingKind::MissingEntrypoint,
                Some(entrypoint.as_str().to_owned()),
                format!(
                    "the {} entrypoint binds no run-history panel",
                    entrypoint.as_str()
                ),
            ));
            continue;
        };
        validate_binding(binding, &mut findings);
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(RunHistoryFinding::blocker(
                RunHistoryFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn validate_binding(binding: &RunHistoryConsumerBinding, findings: &mut Vec<RunHistoryFinding>) {
    let entrypoint = binding.entrypoint.as_str();
    let entries = &binding.entries;

    if entries.is_empty() {
        findings.push(RunHistoryFinding::blocker(
            RunHistoryFindingKind::EntrypointPanelEmpty,
            Some(entrypoint.to_owned()),
            format!("the {entrypoint} entrypoint binds a panel with no entries"),
        ));
        return;
    }

    // The panel must project one evidence row per entry.
    if binding.evidence_rows.len() != entries.len() {
        findings.push(RunHistoryFinding::blocker(
            RunHistoryFindingKind::EvidenceRowProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} panel projects {} rows for {} entries",
                binding.evidence_rows.len(),
                entries.len()
            ),
        ));
    }

    for (index, entry) in entries.iter().enumerate() {
        let subject = format!("{entrypoint}:{}", entry.entry_id);
        let resolved = entry.resolved_rerun_class();

        // Rerun must resolve current policy, never implying cached approval.
        if !entry.rerun_consistent() {
            let has_no_blocker = entry
                .current_policy_blockers
                .contains(&CurrentPolicyBlocker::NoBlockerPresent);
            let pairing_ok = if resolved == RerunActionClass::AdmissibleNoRevalidation {
                entry.current_policy_blockers == [CurrentPolicyBlocker::NoBlockerPresent]
            } else {
                !has_no_blocker
            };
            if !pairing_ok {
                findings.push(RunHistoryFinding::blocker(
                    RunHistoryFindingKind::RerunImpliesCachedApproval,
                    Some(subject.clone()),
                    format!(
                        "entry {} on {entrypoint} resolves {} but its blockers imply cached approval",
                        entry.entry_id,
                        resolved.as_str()
                    ),
                ));
            }
            if entry.imported && resolved != RerunActionClass::BlockedImportedRecord {
                findings.push(RunHistoryFinding::blocker(
                    RunHistoryFindingKind::ImportedRowOffersRerun,
                    Some(subject.clone()),
                    format!(
                        "imported entry {} on {entrypoint} offers a rerun",
                        entry.entry_id
                    ),
                ));
            }
            if entry.automation_layer == AutomationLayerClass::RecordedMacro
                && resolved.is_extension_or_imported_only()
            {
                findings.push(RunHistoryFinding::blocker(
                    RunHistoryFindingKind::MacroOffersExternalRerun,
                    Some(subject.clone()),
                    format!(
                        "macro entry {} on {entrypoint} offers extension/external rerun",
                        entry.entry_id
                    ),
                ));
            }
        }

        // Open-as-recipe must not launder a capability into a recipe.
        if !entry.open_as_recipe_consistent() {
            findings.push(RunHistoryFinding::blocker(
                RunHistoryFindingKind::CapabilityLaunderedIntoRecipe,
                Some(subject.clone()),
                format!(
                    "entry {} on {entrypoint} offers open-as-recipe {} inadmissible for its layer",
                    entry.entry_id,
                    entry.open_as_recipe_action_class.as_str()
                ),
            ));
        }

        // No raw secret may appear in a history row.
        if !entry.secret_references_opaque() {
            findings.push(RunHistoryFinding::blocker(
                RunHistoryFindingKind::RawSecretMaterialInHistory,
                Some(subject.clone()),
                format!(
                    "entry {} on {entrypoint} carries a non-opaque secret reference",
                    entry.entry_id
                ),
            ));
        }

        // The retention/artifact-bundle posture must be consistent.
        if !entry.retention_consistent() {
            findings.push(RunHistoryFinding::blocker(
                RunHistoryFindingKind::RetentionPostureInconsistent,
                Some(subject.clone()),
                format!(
                    "entry {} on {entrypoint} has an inconsistent retention or bundle posture",
                    entry.entry_id
                ),
            ));
        }

        // The projected evidence row must quote the same resolution as the entry.
        if let Some(row) = binding.evidence_rows.get(index) {
            let expected = entry.to_evidence_row(row.row_id.clone(), row.recorded_at.clone());
            if row != &expected {
                findings.push(RunHistoryFinding::blocker(
                    RunHistoryFindingKind::EvidenceRowProjectionInconsistent,
                    Some(subject.clone()),
                    format!(
                        "the projected evidence row for {} on {entrypoint} disagrees with the entry",
                        entry.entry_id
                    ),
                ));
            }
        }
    }
}

fn promotion_state_for_findings(
    findings: &[RunHistoryFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == RunHistoryFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == RunHistoryFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn packet_digest(consumer_bindings: &[RunHistoryConsumerBinding]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    for binding in consumer_bindings {
        tokens.push(binding.entrypoint.as_str().to_owned());
        for entry in &binding.entries {
            tokens.push(entry.entry_id.clone());
        }
    }
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[String]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

// ---------------------------------------------------------------------------
// Seeds
// ---------------------------------------------------------------------------

fn s(value: &str) -> String {
    value.to_owned()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn link(link_class: ArtifactLinkClass, artifact_ref: &str, summary: &str) -> ArtifactLink {
    ArtifactLink {
        link_class,
        artifact_ref: s(artifact_ref),
        summary: s(summary),
    }
}

fn identity(
    run_id: &str,
    manifest_id: &str,
    manifest_revision_ref: &str,
    content_address: Option<&str>,
) -> RunIdentity {
    RunIdentity {
        run_id: s(run_id),
        manifest_id: s(manifest_id),
        manifest_revision_ref: s(manifest_revision_ref),
        manifest_content_address: content_address.map(s),
    }
}

fn context(
    trust: TrustStateClass,
    policy: PolicyObservationClass,
    kill_switch: KillSwitchObservationClass,
    environment_capsule_ref: Option<&str>,
    sentence: &str,
) -> ContextSummary {
    ContextSummary {
        execution_context_capsule_ref: s("capsule:execution-context"),
        environment_capsule_ref: environment_capsule_ref.map(s),
        trust_state_class: trust,
        policy_observation_class: policy,
        kill_switch_observation_class: kill_switch,
        context_summary_sentence: s(sentence),
    }
}

/// Existing contracts the first-consumers packet reuses instead of re-deciding.
pub fn canonical_reused_contract_refs() -> Vec<String> {
    strings(&[
        RUN_HISTORY_ROW_SCHEMA_REF,
        RUN_RECORD_SCHEMA_REF,
        RUN_SUMMARY_EXPORT_SCHEMA_REF,
        RECIPE_BUILDER_SCHEMA_REF,
        "schemas/automation/automation-contract-baseline.schema.json",
        "schemas/automation/dry-run-explain.schema.json",
        "docs/automation/run_history_contract.md",
        "docs/m5/recipe-builder-and-macro-contract.md",
    ])
}

/// Builds the seeded run-history panel one first consumer renders.
///
/// Each panel carries the latest run first. Across the six panels every automation
/// layer appears, the four admissible rerun states and a representative set of
/// blocked states appear, and the imported, macro-promotion, and managed-channel
/// paths are exercised, so the freeze covers the cross-surface vocabulary.
pub fn seeded_consumer_panel(entrypoint: RecipeBuilderEntrypoint) -> Vec<RunHistoryEntry> {
    use ArtifactBundleStateClass::{
        ArtifactBundleAvailable, ArtifactBundleNotProducedExternalAuthority,
    };
    use ArtifactLinkClass::{EvidenceBundle, ExternalArtifact, ResultArtifact, RunLog};
    use AutomationLayerClass::{
        DeclarativeRecipe, ExtensionOrExternalAutomation, HeadlessSafeRun, ManagedOnlyTemplate,
        RecordedMacro,
    };
    use CurrentPolicyBlocker::{
        ExtensionOrExternalRunnerUnavailable, FreshApprovalRequired,
        ImportedRecordNoDispatchAdmissible, KillSwitchEngaged, ManagedOnlyChannelUnresolved,
        NoBlockerPresent,
    };
    use ExecutionModeClass::{
        AiAssistantDispatch, DesktopExplicitActionDispatch, ExternalRunnerDispatch,
        HeadlessCliExplicitDispatch, ImportedProviderEvent, ManagedOnlyChannelDispatch,
    };
    use KillSwitchObservationClass::{KillSwitchClear, KillSwitchEngaged as KsEngaged};
    use OpenAsRecipeActionClass::{
        AdmissibleMacroPromotable, InadmissibleAlreadyDeclarativeRecipe,
        InadmissibleAlreadyManagedOnlyTemplate, InadmissibleNoDeclarativeCapabilityPath,
    };
    use PolicyObservationClass::{PolicyAllowed, PolicyConstrained};
    use RedactionModeClass::{MetadataSafeDefault, RedactionRequiredWithSecretBrokerHandles};
    use RetentionClass::{
        RetainUntilOrganizationAuditWindow, RetainUntilPurgedByUser,
        RetainUntilWorkspaceRedactionWindow,
    };
    use RunResultClass::{PartialSuccess, Succeeded};
    use TrustStateClass::{RemoteUntrusted, WorkspaceTrusted};

    match entrypoint {
        // Notebook: a declarative recipe run plus a recorded-macro replay, so the
        // panel demonstrates comparison across earlier runs and the macro-promotion
        // path. Both reruns are admissible with no revalidation.
        RecipeBuilderEntrypoint::Notebook => vec![
            RunHistoryEntry {
                entry_id: s("run-history:notebook-run-and-export:2"),
                entrypoint,
                run_identity: identity(
                    "run:notebook-run-and-export:2",
                    "manifest:notebook-run-and-export",
                    "recipe-rev:notebook-run-and-export:2",
                    Some("content-address:notebook-run-and-export:2"),
                ),
                automation_layer: DeclarativeRecipe,
                record_schema_version: 1,
                execution_mode: DesktopExplicitActionDispatch,
                result_class: Succeeded,
                imported: false,
                artifact_links: vec![
                    link(RunLog, "artifact:notebook-run-log:2", "the notebook run log"),
                    link(
                        ResultArtifact,
                        "artifact:notebook-export:2",
                        "the rendered notebook export",
                    ),
                ],
                secret_reference_refs: vec![],
                retention_class: RetainUntilPurgedByUser,
                retention_window_expires_at: None,
                redaction_mode: MetadataSafeDefault,
                artifact_bundle_state: ArtifactBundleAvailable,
                artifact_bundle_ref: Some(s("bundle:notebook-run-and-export:2")),
                context_summary: context(
                    WorkspaceTrusted,
                    PolicyAllowed,
                    KillSwitchClear,
                    Some("capsule:env:notebook:2"),
                    "the local workspace under an allowing policy",
                ),
                current_policy_blockers: vec![NoBlockerPresent],
                open_as_recipe_action_class: InadmissibleAlreadyDeclarativeRecipe,
                title: s("Run and export the notebook"),
                summary: s("Ran every cell and wrote the rendered export; rerun resolves clean."),
                dispatched_at: s("2026-06-18T00:00:00Z"),
                completed_at: Some(s("2026-06-18T00:00:05Z")),
            },
            RunHistoryEntry {
                entry_id: s("run-history:notebook-macro-replay:1"),
                entrypoint,
                run_identity: identity(
                    "run:notebook-macro-replay:1",
                    "macro:notebook-tidy",
                    "macro-rev:notebook-tidy:1",
                    None,
                ),
                automation_layer: RecordedMacro,
                record_schema_version: 1,
                execution_mode: DesktopExplicitActionDispatch,
                result_class: Succeeded,
                imported: false,
                artifact_links: vec![link(
                    RunLog,
                    "artifact:notebook-macro-log:1",
                    "the macro replay log",
                )],
                secret_reference_refs: vec![],
                retention_class: RetainUntilPurgedByUser,
                retention_window_expires_at: None,
                redaction_mode: MetadataSafeDefault,
                artifact_bundle_state: ArtifactBundleAvailable,
                artifact_bundle_ref: Some(s("bundle:notebook-macro-replay:1")),
                context_summary: context(
                    WorkspaceTrusted,
                    PolicyAllowed,
                    KillSwitchClear,
                    Some("capsule:env:notebook:1"),
                    "the local workspace under an allowing policy",
                ),
                current_policy_blockers: vec![NoBlockerPresent],
                open_as_recipe_action_class: AdmissibleMacroPromotable,
                title: s("Replay the notebook tidy macro"),
                summary: s("A recorded macro promotable to a declarative recipe; rerun resolves clean."),
                dispatched_at: s("2026-06-17T00:00:00Z"),
                completed_at: Some(s("2026-06-17T00:00:03Z")),
            },
        ],
        // Task/test/debug: a headless-safe run. Rerun admissible with no revalidation.
        RecipeBuilderEntrypoint::TaskTestDebug => vec![RunHistoryEntry {
            entry_id: s("run-history:run-tests-and-report:1"),
            entrypoint,
            run_identity: identity(
                "run:run-tests-and-report:1",
                "manifest:run-tests-and-report",
                "recipe-rev:run-tests-and-report:1",
                Some("content-address:run-tests-and-report:1"),
            ),
            automation_layer: HeadlessSafeRun,
            record_schema_version: 1,
            execution_mode: HeadlessCliExplicitDispatch,
            result_class: Succeeded,
            imported: false,
            artifact_links: vec![
                link(RunLog, "artifact:test-run-events:1", "the test-event stream"),
                link(
                    ResultArtifact,
                    "artifact:coverage-report:1",
                    "the coverage report",
                ),
            ],
            secret_reference_refs: vec![],
            retention_class: RetainUntilPurgedByUser,
            retention_window_expires_at: None,
            redaction_mode: MetadataSafeDefault,
            artifact_bundle_state: ArtifactBundleAvailable,
            artifact_bundle_ref: Some(s("bundle:run-tests-and-report:1")),
            context_summary: context(
                WorkspaceTrusted,
                PolicyAllowed,
                KillSwitchClear,
                Some("capsule:env:tests:1"),
                "the headless CLI on the workspace toolchain",
            ),
            current_policy_blockers: vec![NoBlockerPresent],
            open_as_recipe_action_class: InadmissibleAlreadyDeclarativeRecipe,
            title: s("Run tests and write the coverage report"),
            summary: s("A headless-safe run; rerun resolves clean under current policy."),
            dispatched_at: s("2026-06-18T00:00:00Z"),
            completed_at: Some(s("2026-06-18T00:00:30Z")),
        }],
        // Request/API: a declarative recipe whose rerun needs a fresh approval, so
        // the row proves rerun never reuses yesterday's approval ticket.
        RecipeBuilderEntrypoint::RequestApi => vec![RunHistoryEntry {
            entry_id: s("run-history:send-request-and-save:1"),
            entrypoint,
            run_identity: identity(
                "run:send-request-and-save:1",
                "manifest:send-request-and-save",
                "recipe-rev:send-request-and-save:1",
                Some("content-address:send-request-and-save:1"),
            ),
            automation_layer: DeclarativeRecipe,
            record_schema_version: 1,
            execution_mode: DesktopExplicitActionDispatch,
            result_class: PartialSuccess,
            imported: false,
            artifact_links: vec![link(
                ResultArtifact,
                "artifact:saved-response:1",
                "the saved response capture",
            )],
            secret_reference_refs: vec![s("secret-broker:request-bearer-token")],
            retention_class: RetainUntilWorkspaceRedactionWindow,
            retention_window_expires_at: Some(s("2026-07-18T00:00:00Z")),
            redaction_mode: RedactionRequiredWithSecretBrokerHandles,
            artifact_bundle_state: ArtifactBundleAvailable,
            artifact_bundle_ref: Some(s("bundle:send-request-and-save:1")),
            context_summary: context(
                WorkspaceTrusted,
                PolicyConstrained,
                KillSwitchClear,
                Some("capsule:env:request:1"),
                "the local workspace against the resolved environment endpoint",
            ),
            current_policy_blockers: vec![FreshApprovalRequired],
            open_as_recipe_action_class: InadmissibleAlreadyDeclarativeRecipe,
            title: s("Send the request and save the response"),
            summary: s("Rerun is admissible only after a fresh approval; the row never reuses the prior ticket."),
            dispatched_at: s("2026-06-18T00:00:00Z"),
            completed_at: Some(s("2026-06-18T00:00:02Z")),
        }],
        // Package: a managed-only template whose rerun needs the managed channel
        // resolved, so the row proves managed authority is re-resolved.
        RecipeBuilderEntrypoint::Package => vec![RunHistoryEntry {
            entry_id: s("run-history:update-and-publish:1"),
            entrypoint,
            run_identity: identity(
                "run:update-and-publish:1",
                "managed-template:update-and-publish",
                "template-rev:update-and-publish:1",
                Some("content-address:update-and-publish:1"),
            ),
            automation_layer: ManagedOnlyTemplate,
            record_schema_version: 1,
            execution_mode: ManagedOnlyChannelDispatch,
            result_class: Succeeded,
            imported: false,
            artifact_links: vec![link(
                ResultArtifact,
                "artifact:lockfile-update:1",
                "the lockfile update",
            )],
            secret_reference_refs: vec![s("secret-broker:registry-publish-token")],
            retention_class: RetainUntilOrganizationAuditWindow,
            retention_window_expires_at: Some(s("2026-12-18T00:00:00Z")),
            redaction_mode: RedactionRequiredWithSecretBrokerHandles,
            artifact_bundle_state: ArtifactBundleAvailable,
            artifact_bundle_ref: Some(s("bundle:update-and-publish:1")),
            context_summary: context(
                WorkspaceTrusted,
                PolicyConstrained,
                KillSwitchClear,
                Some("capsule:env:package:1"),
                "the managed channel under a constraining policy",
            ),
            current_policy_blockers: vec![ManagedOnlyChannelUnresolved],
            open_as_recipe_action_class: InadmissibleAlreadyManagedOnlyTemplate,
            title: s("Update dependencies and publish the package"),
            summary: s("Rerun is admissible only after the managed-only channel is resolved."),
            dispatched_at: s("2026-06-18T00:00:00Z"),
            completed_at: Some(s("2026-06-18T00:00:20Z")),
        }],
        // Incident: an imported extension/external row plus a non-imported external
        // row whose runner is unavailable, so the panel proves imported rows never
        // offer rerun and an extension row blocks on runner availability.
        RecipeBuilderEntrypoint::Incident => vec![
            RunHistoryEntry {
                entry_id: s("run-history:imported-runbook:1"),
                entrypoint,
                run_identity: identity(
                    "external-handle:imported-runbook:1",
                    "external:incident-runbook",
                    "external-rev:incident-runbook:1",
                    None,
                ),
                automation_layer: ExtensionOrExternalAutomation,
                record_schema_version: 1,
                execution_mode: ImportedProviderEvent,
                result_class: Succeeded,
                imported: true,
                artifact_links: vec![link(
                    ExternalArtifact,
                    "external-artifact:imported-runbook:1",
                    "an external runbook artifact reference",
                )],
                secret_reference_refs: vec![],
                retention_class: RetainUntilOrganizationAuditWindow,
                retention_window_expires_at: Some(s("2026-12-18T00:00:00Z")),
                redaction_mode: MetadataSafeDefault,
                artifact_bundle_state: ArtifactBundleNotProducedExternalAuthority,
                artifact_bundle_ref: None,
                context_summary: context(
                    RemoteUntrusted,
                    PolicyConstrained,
                    KillSwitchClear,
                    None,
                    "an imported provider event from an untrusted remote",
                ),
                current_policy_blockers: vec![ImportedRecordNoDispatchAdmissible],
                open_as_recipe_action_class: InadmissibleNoDeclarativeCapabilityPath,
                title: s("Imported incident runbook event"),
                summary: s("An imported row; dispatch authority did not survive the import boundary."),
                dispatched_at: s("2026-06-18T00:00:00Z"),
                completed_at: Some(s("2026-06-18T00:00:01Z")),
            },
            RunHistoryEntry {
                entry_id: s("run-history:external-runbook:1"),
                entrypoint,
                run_identity: identity(
                    "external-handle:external-runbook:1",
                    "external:incident-runbook",
                    "external-rev:incident-runbook:1",
                    None,
                ),
                automation_layer: ExtensionOrExternalAutomation,
                record_schema_version: 1,
                execution_mode: ExternalRunnerDispatch,
                result_class: PartialSuccess,
                imported: false,
                artifact_links: vec![link(
                    EvidenceBundle,
                    "artifact:incident-bundle:1",
                    "the local incident evidence bundle",
                )],
                secret_reference_refs: vec![],
                retention_class: RetainUntilWorkspaceRedactionWindow,
                retention_window_expires_at: Some(s("2026-07-18T00:00:00Z")),
                redaction_mode: MetadataSafeDefault,
                artifact_bundle_state: ArtifactBundleAvailable,
                artifact_bundle_ref: Some(s("bundle:external-runbook:1")),
                context_summary: context(
                    RemoteUntrusted,
                    PolicyConstrained,
                    KillSwitchClear,
                    Some("capsule:env:incident:1"),
                    "an external runner against an untrusted remote",
                ),
                current_policy_blockers: vec![ExtensionOrExternalRunnerUnavailable],
                open_as_recipe_action_class: InadmissibleNoDeclarativeCapabilityPath,
                title: s("Run the incident runbook on the external runner"),
                summary: s("Rerun is blocked until the external runner is available again."),
                dispatched_at: s("2026-06-17T00:00:00Z"),
                completed_at: Some(s("2026-06-17T00:00:10Z")),
            },
        ],
        // AI assistant: a declarative recipe whose rerun needs the kill switch
        // cleared, demonstrating the AI evidence join and kill-switch revalidation.
        RecipeBuilderEntrypoint::AiAssistant => vec![RunHistoryEntry {
            entry_id: s("run-history:apply-proposed-fix:1"),
            entrypoint,
            run_identity: identity(
                "run:apply-proposed-fix:1",
                "manifest:apply-proposed-fix",
                "recipe-rev:apply-proposed-fix:1",
                Some("content-address:apply-proposed-fix:1"),
            ),
            automation_layer: DeclarativeRecipe,
            record_schema_version: 1,
            execution_mode: AiAssistantDispatch,
            result_class: PartialSuccess,
            imported: false,
            artifact_links: vec![
                link(EvidenceBundle, "artifact:ai-evidence:1", "the AI evidence bundle"),
                link(
                    ArtifactLinkClass::DiffArtifact,
                    "artifact:proposed-diff:1",
                    "the applied diff preview",
                ),
            ],
            secret_reference_refs: vec![],
            retention_class: RetainUntilWorkspaceRedactionWindow,
            retention_window_expires_at: Some(s("2026-07-18T00:00:00Z")),
            redaction_mode: MetadataSafeDefault,
            artifact_bundle_state: ArtifactBundleAvailable,
            artifact_bundle_ref: Some(s("bundle:apply-proposed-fix:1")),
            context_summary: context(
                WorkspaceTrusted,
                PolicyConstrained,
                KsEngaged,
                Some("capsule:env:ai:1"),
                "the AI assistant under a constraining policy with the kill switch engaged",
            ),
            current_policy_blockers: vec![KillSwitchEngaged],
            open_as_recipe_action_class: InadmissibleAlreadyDeclarativeRecipe,
            title: s("Apply the AI-proposed fix"),
            summary: s("Rerun is admissible only after the kill switch is cleared."),
            dispatched_at: s("2026-06-18T00:00:00Z"),
            completed_at: Some(s("2026-06-18T00:00:04Z")),
        }],
    }
}

/// The reviewable summary one consumer's panel carries.
fn panel_summary(entrypoint: RecipeBuilderEntrypoint) -> &'static str {
    match entrypoint {
        RecipeBuilderEntrypoint::Notebook => {
            "A recipe run and a promotable macro replay; both reruns resolve clean."
        }
        RecipeBuilderEntrypoint::TaskTestDebug => {
            "A headless-safe test run whose rerun resolves clean under current policy."
        }
        RecipeBuilderEntrypoint::RequestApi => {
            "A request run whose rerun is admissible only after a fresh approval."
        }
        RecipeBuilderEntrypoint::Package => {
            "A managed-only template run whose rerun needs the managed channel resolved."
        }
        RecipeBuilderEntrypoint::Incident => {
            "An imported runbook row that offers no rerun and an external row blocked on its runner."
        }
        RecipeBuilderEntrypoint::AiAssistant => {
            "An AI-assistant run whose rerun is admissible only after the kill switch clears."
        }
    }
}

/// Builds the canonical stable first-consumers input.
pub fn current_run_history_first_consumers_input() -> RunHistoryFirstConsumersInput {
    let consumer_bindings = RecipeBuilderEntrypoint::ALL
        .into_iter()
        .map(|entrypoint| {
            RunHistoryConsumerBinding::from_entries(
                entrypoint,
                seeded_consumer_panel(entrypoint),
                panel_summary(entrypoint),
            )
        })
        .collect();
    RunHistoryFirstConsumersInput {
        packet_id: RUN_HISTORY_FIRST_CONSUMERS_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        consumer_bindings,
        reused_contract_refs: canonical_reused_contract_refs(),
        invariants: RunHistoryInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable first-consumers packet.
pub fn seeded_run_history_first_consumers_packet() -> RunHistoryFirstConsumersPacket {
    RunHistoryFirstConsumersPacket::materialize(current_run_history_first_consumers_input())
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_run_history_first_consumers_packet(
    packet: &RunHistoryFirstConsumersPacket,
) -> Result<(), Vec<RunHistoryFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Worked example: the latest notebook run exported for round-trip review.
///
/// The notebook recipe run mixes a workspace artifact bundle with a clean rerun, so
/// the round-trip proves identity and rerun truth survive export, history, and
/// support.
pub fn seeded_run_history_export_roundtrip() -> RunHistoryEvidenceExport {
    seeded_consumer_panel(RecipeBuilderEntrypoint::Notebook)
        .into_iter()
        .next()
        .expect("notebook panel has at least one entry")
        .export("export:notebook-run-and-export:v1", "2026-06-18T00:01:00Z")
}

/// Worked example: the imported incident row whose rerun is blocked.
pub fn seeded_imported_entry() -> RunHistoryEntry {
    seeded_consumer_panel(RecipeBuilderEntrypoint::Incident)
        .into_iter()
        .next()
        .expect("incident panel has at least one entry")
}

/// Convenience accessor for a single seeded entry by entrypoint (latest entry).
pub fn seeded_consumer_entry(entrypoint: RecipeBuilderEntrypoint) -> RunHistoryEntry {
    seeded_consumer_panel(entrypoint)
        .into_iter()
        .next()
        .expect("panel has at least one entry")
}
