//! Command-parity and discoverability audit for the M5 depth surfaces.
//!
//! The M5 depth lanes (notebook, data/API, profiler, trace/replay,
//! docs/browser, template/scaffold, review/pipeline, preview, companion,
//! incident, sync, and offboarding) each ship their own panes. This
//! module carries the stable v1 promise forward into those lanes: every
//! meaningful M5 action MUST be discoverable, explainable, and
//! automation-honest through the same command graph, help anchors, and
//! disabled-state reasoning model as the v1 core — never only through a
//! pane-local icon or a browser deep link.
//!
//! The audit projects, for each registered M5 command, the canonical
//! descriptor against the projection that every stable discoverability
//! channel reports:
//!
//! - `command_palette`
//! - `keybinding_help`
//! - `help_search`
//! - `onboarding_tour`
//! - `cli_headless`
//! - `ai_automation`
//!
//! The resulting [`M5CommandParityAuditReport`] is the canonical truth
//! object for the M5 XT-12 learnability lane. It is consumed by:
//!
//! - the live shell discoverability inspector (so the in-product audit
//!   quotes the same per-row findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_command_parity`), which
//!   is the only mint-from-truth path for the JSON fixtures checked in
//!   under `fixtures/ux/m5/command-parity/`;
//! - the support-export wrapper that lets a reviewer pivot from a
//!   support case to the row that flagged a gap;
//! - the markdown audit under
//!   `artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md`
//!   (rendered from the same seed); and
//! - the XT-12 matrix, which can ingest the audit directly when
//!   qualifying or narrowing an M5 row.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 command must declare a projection row for each
//!    of the six required discoverability channels.
//! 2. Every command must carry a canonical help anchor, searchable
//!    discoverability metadata, and a promotion flag asserting the
//!    command id is part of the stable command graph; a missing anchor,
//!    missing search metadata, or unpromoted command id is a blocker.
//! 3. A high-risk command (any non-`no_preview_required` preview class or
//!    any non-reversible capability scope) with an `unknown_high_risk_gap`
//!    on a required channel, a high-risk command that declares
//!    `always_invokable` instead of a typed disabled reason, and any
//!    command reachable only through its own pane (`custom_pane_only`)
//!    are blockers.
//! 4. Disabled-reason mode, lifecycle label, preview class, automation
//!    suitability, help anchor, and aliases MUST come from the same
//!    canonical descriptor across every claimed channel; a drift is a
//!    blocker.
//! 5. At least one command must claim each of the six required channels
//!    so the audit cannot regress into a single-channel view.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/command-parity/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_m5_command_parity_audit`].

use std::collections::BTreeSet;

use aureline_commands::{registry::seeded_registry, CommandRegistryEntryRecord};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every M5 command-parity record.
pub const M5_COMMAND_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every M5 command-parity row.
pub const M5_COMMAND_PARITY_SHARED_CONTRACT_REF: &str = "shell:m5_command_parity:v1";

/// Stable record kind for [`M5CommandParityAuditReport`] payloads.
pub const M5_COMMAND_PARITY_REPORT_RECORD_KIND: &str =
    "shell_m5_command_parity_audit_report_record";

/// Stable record kind for [`M5CommandParityRow`] payloads.
pub const M5_COMMAND_PARITY_ROW_RECORD_KIND: &str = "shell_m5_command_parity_row_record";

/// Stable record kind for [`M5CommandParitySupportExport`] payloads.
pub const M5_COMMAND_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_command_parity_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_COMMAND_PARITY_REPORT_ID: &str = "shell:m5_command_parity:audit:v1";

/// Stable support-export id quoted in the published wrapper.
pub const M5_COMMAND_PARITY_SUPPORT_EXPORT_ID: &str = "support-export:m5-command-parity:001";

/// Source descriptor-diff schema ref for the canonical descriptors.
pub const M5_COMMAND_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-command-descriptor-diff.schema.json";

/// Path of the published markdown audit artifact.
pub const M5_COMMAND_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md";

/// Path of the published companion doc.
pub const M5_COMMAND_PARITY_PUBLISHED_DOC_REF: &str = "docs/ux/m5/command_parity_audit.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5 depth feature family whose commands the audit registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FeatureFamily {
    /// Notebook authoring and execution surface.
    Notebook,
    /// Data / API request and response surface.
    DataApi,
    /// Profiler capture and inspection surface.
    Profiler,
    /// Trace capture and deterministic replay surface.
    TraceReplay,
    /// Embedded docs and browser surface.
    DocsBrowser,
    /// Template and scaffold generation surface.
    TemplateScaffold,
    /// Review and pipeline orchestration surface.
    ReviewPipeline,
    /// Live preview surface.
    Preview,
    /// Companion / cross-device handoff surface.
    Companion,
    /// Incident declaration and response surface.
    Incident,
    /// Workspace sync / publish surface.
    Sync,
    /// Offboarding / export-and-wipe surface.
    Offboarding,
    /// Secret-broker review and rotation surface.
    SecretBroker,
    /// Infrastructure and managed-control surface.
    Infrastructure,
}

impl M5FeatureFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataApi => "data_api",
            Self::Profiler => "profiler",
            Self::TraceReplay => "trace_replay",
            Self::DocsBrowser => "docs_browser",
            Self::TemplateScaffold => "framework_pack",
            Self::ReviewPipeline => "review_pipeline",
            Self::Preview => "preview",
            Self::Companion => "companion",
            Self::Incident => "incident",
            Self::Sync => "sync",
            Self::Offboarding => "offboarding",
            Self::SecretBroker => "secret_broker",
            Self::Infrastructure => "infrastructure",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Notebook => "Notebook",
            Self::DataApi => "Data / API",
            Self::Profiler => "Profiler",
            Self::TraceReplay => "Trace / replay",
            Self::DocsBrowser => "Docs / browser",
            Self::TemplateScaffold => "Framework pack",
            Self::ReviewPipeline => "Review / pipeline",
            Self::Preview => "Preview",
            Self::Companion => "Companion",
            Self::Incident => "Incident",
            Self::Sync => "Sync",
            Self::Offboarding => "Offboarding",
            Self::SecretBroker => "Secret broker",
            Self::Infrastructure => "Infrastructure",
        }
    }
}

/// One of the six discoverability channels the audit covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiscoveryChannel {
    /// Command palette result rows.
    CommandPalette,
    /// Keybinding help surfaces and conflict resolution.
    KeybindingHelp,
    /// Help search and the help anchor index.
    HelpSearch,
    /// Onboarding and guided-tour references.
    OnboardingTour,
    /// CLI / headless help and dispatch rows.
    CliHeadless,
    /// AI automation surfaces invoked by stable command identity.
    AiAutomation,
}

impl M5DiscoveryChannel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::KeybindingHelp => "keybinding_help",
            Self::HelpSearch => "help_search",
            Self::OnboardingTour => "onboarding_tour",
            Self::CliHeadless => "cli_headless",
            Self::AiAutomation => "ai_automation",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::CommandPalette => "Command palette",
            Self::KeybindingHelp => "Keybinding help",
            Self::HelpSearch => "Help search",
            Self::OnboardingTour => "Onboarding / tour",
            Self::CliHeadless => "CLI / headless",
            Self::AiAutomation => "AI automation",
        }
    }

    /// Returns the six required channels in canonical order.
    pub const fn required_channels() -> [Self; 6] {
        [
            Self::CommandPalette,
            Self::KeybindingHelp,
            Self::HelpSearch,
            Self::OnboardingTour,
            Self::CliHeadless,
            Self::AiAutomation,
        ]
    }
}

/// Lifecycle label retained on the canonical command descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleLabel {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; surfaces must show the replacement command id.
    Deprecated,
}

impl M5LifecycleLabel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Preview class the canonical descriptor pins for the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PreviewClass {
    /// Command is safe to invoke without a preview.
    NoPreviewRequired,
    /// Command must show a structured diff preview before apply.
    StructuredDiffPreview,
    /// Command crosses a destructive bulk-mutation boundary.
    DestructiveBulkMutationPreview,
    /// Command writes a policy or waiver that must be authored first.
    PolicyAuthoringOrWaiverPreview,
    /// Command publishes irreversibly (push, release, share).
    IrreversiblePublishPreview,
}

impl M5PreviewClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPreviewRequired => "no_preview_required",
            Self::StructuredDiffPreview => "structured_diff_preview",
            Self::DestructiveBulkMutationPreview => "destructive_bulk_mutation_preview",
            Self::PolicyAuthoringOrWaiverPreview => "policy_authoring_or_waiver_preview",
            Self::IrreversiblePublishPreview => "irreversible_publish_preview",
        }
    }

    /// `true` when the preview class requires explicit pre-apply review.
    pub const fn is_high_risk(self) -> bool {
        !matches!(self, Self::NoPreviewRequired)
    }
}

/// Capability-scope class the canonical descriptor pins for the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityScope {
    /// Inert metadata route (no state change).
    InertMetadataOnly,
    /// Reversible local read.
    ReversibleLocalRead,
    /// Reversible local mutation the user can undo without rollback.
    ReversibleLocalMutation,
    /// Recoverable durable mutation that requires a rollback handle.
    RecoverableDurableMutation,
    /// Destructive bulk mutation (multi-file, multi-record).
    DestructiveBulkMutation,
    /// Irreversible publish / network mutation.
    IrreversiblePublish,
    /// Secret-bearing or credential-authoring action.
    CredentialOrSecretBearing,
    /// Managed workspace or control-plane mutation.
    ManagedWorkspaceControl,
}

impl M5CapabilityScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertMetadataOnly => "inert_metadata_only",
            Self::ReversibleLocalRead => "reversible_local_read",
            Self::ReversibleLocalMutation => "reversible_local_mutation",
            Self::RecoverableDurableMutation => "recoverable_durable_mutation",
            Self::DestructiveBulkMutation => "destructive_bulk_mutation",
            Self::IrreversiblePublish => "irreversible_publish",
            Self::CredentialOrSecretBearing => "credential_or_secret_bearing",
            Self::ManagedWorkspaceControl => "managed_workspace_control",
        }
    }

    /// `true` for capability scopes that contribute to high-risk status.
    pub const fn is_high_risk(self) -> bool {
        matches!(
            self,
            Self::RecoverableDurableMutation
                | Self::DestructiveBulkMutation
                | Self::IrreversiblePublish
                | Self::CredentialOrSecretBearing
                | Self::ManagedWorkspaceControl
        )
    }
}

/// Mutability class projected from the canonical descriptor and side effect lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MutabilityClass {
    /// Read-only or metadata-only action.
    ReadOnly,
    /// Local undoable mutation.
    SessionMutation,
    /// Durable but recoverable mutation.
    DurableMutation,
    /// Destructive bulk mutation.
    DestructiveMutation,
    /// External publish or provider-visible mutation.
    ExternalPublish,
    /// Secret-bearing review or rotation flow.
    SensitiveMutation,
    /// Managed control-plane mutation.
    ManagedControl,
}

impl M5MutabilityClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::SessionMutation => "session_mutation",
            Self::DurableMutation => "durable_mutation",
            Self::DestructiveMutation => "destructive_mutation",
            Self::ExternalPublish => "external_publish",
            Self::SensitiveMutation => "sensitive_mutation",
            Self::ManagedControl => "managed_control",
        }
    }
}

/// Disabled-reason mode the canonical descriptor pins for the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisabledReasonMode {
    /// Command is always invokable; no disabled-reason path is required.
    AlwaysInvokable,
    /// Command MUST surface a typed disabled reason when unavailable.
    TypedReasonRequiredWhenUnavailable,
}

impl M5DisabledReasonMode {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvokable => "always_invokable",
            Self::TypedReasonRequiredWhenUnavailable => "typed_reason_required_when_unavailable",
        }
    }
}

/// Automation suitability the canonical descriptor pins for the command.
///
/// Surfaces — especially the AI automation channel — MUST project the
/// same value so a new M5 action cannot widen authority or hide its
/// approval posture compared with the stable v1 command model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AutomationSuitability {
    /// Read-only or inert; safe to invoke fully unattended.
    FullyAutomatable,
    /// Automation may proceed but must show a preview before apply.
    PreviewThenConfirm,
    /// Automation may draft only; a human applies the result.
    DraftOnly,
    /// No automation; a human must invoke the command directly.
    HumanOnly,
}

impl M5AutomationSuitability {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyAutomatable => "fully_automatable",
            Self::PreviewThenConfirm => "preview_then_confirm",
            Self::DraftOnly => "draft_only",
            Self::HumanOnly => "human_only",
        }
    }
}

/// Coverage status reported by a discoverability channel.
///
/// Only `Claimed` rows are subject to projection-vs-descriptor drift
/// checks. `ExplicitlyNarrowed`, `DiscoverableOnly`, `BrowserHandoffOnly`,
/// `VoiceAddressable`, and `NotSurfacedOnThisClient` rows are accepted as
/// long as they carry a `narrowing_reason`. `CustomPaneOnly` (a
/// pointer-only island) and `UnknownHighRiskGap` are blocking findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageStatus {
    /// Channel claims a first-class projection of the command.
    Claimed,
    /// Channel explicitly narrows; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// Channel lists the command for discoverability only (no dispatch).
    DiscoverableOnly,
    /// Channel routes to browser handoff only.
    BrowserHandoffOnly,
    /// Channel is voice-addressable only; the real route is elsewhere.
    VoiceAddressable,
    /// Client scope excludes this channel (e.g. CLI cannot open a UI route).
    NotSurfacedOnThisClient,
    /// Command is reachable only through its own pane. Always a blocker.
    CustomPaneOnly,
    /// Channel is required but the projection is missing or unknown.
    /// Always a blocker for high-risk commands.
    UnknownHighRiskGap,
}

impl M5CoverageStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Claimed => "claimed",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::DiscoverableOnly => "discoverable_only",
            Self::BrowserHandoffOnly => "browser_handoff_only",
            Self::VoiceAddressable => "voice_addressable",
            Self::NotSurfacedOnThisClient => "not_surfaced_on_this_client",
            Self::CustomPaneOnly => "custom_pane_only",
            Self::UnknownHighRiskGap => "unknown_high_risk_gap",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::DiscoverableOnly
                | Self::BrowserHandoffOnly
                | Self::VoiceAddressable
                | Self::NotSurfacedOnThisClient
        )
    }

    /// `true` for statuses that are projected from the descriptor and
    /// therefore subject to descriptor-vs-projection drift checks.
    pub const fn projects_descriptor(self) -> bool {
        matches!(self, Self::Claimed)
    }
}

/// Canonical descriptor for one M5 depth command.
///
/// Every channel row in the audit quotes these fields verbatim; any
/// divergence is a blocking finding the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandDescriptor {
    /// Stable command id (e.g. `cmd:notebook.run_all_cells`).
    pub command_id: String,
    /// Feature family the command belongs to.
    pub feature_family: M5FeatureFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical discoverability summary text.
    pub discoverability_summary: String,
    /// Canonical help anchor ref the audit can reopen the feature from.
    pub help_anchor_ref: String,
    /// Searchable discoverability keywords surfaced in palette and help
    /// search. MUST be non-empty.
    pub search_keywords: Vec<String>,
    /// Category refs shared across palette, docs, onboarding, and support search.
    pub category_refs: Vec<String>,
    /// Pinned lifecycle label.
    pub lifecycle_label: M5LifecycleLabel,
    /// Pinned preview class.
    pub preview_class: M5PreviewClass,
    /// Pinned approval posture.
    pub approval_posture_class: String,
    /// Pinned capability scope class.
    pub capability_scope_class: M5CapabilityScope,
    /// Derived mutability class surfaces can disclose without reinterpreting the command.
    pub mutability_class: M5MutabilityClass,
    /// Pinned disabled-reason mode.
    pub disabled_reason_mode: M5DisabledReasonMode,
    /// Pinned automation suitability.
    pub automation_suitability: M5AutomationSuitability,
    /// Exact automation labels the canonical descriptor declares.
    pub automation_labels: Vec<String>,
    /// Canonical alias set the descriptor owns. Channels MUST NOT expose
    /// aliases outside this set.
    pub canonical_aliases: Vec<String>,
    /// Descriptor origin class disclosed to callers.
    pub origin_class: String,
    /// Optional origin source ref disclosed when trust depends on the source.
    pub source_ref: Option<String>,
    /// Optional origin publisher ref for packaged or bridged commands.
    pub publisher_ref: Option<String>,
    /// Invocation schema ref quoted into the descriptor packet.
    pub invocation_schema_ref: String,
    /// Result schema ref quoted into the descriptor packet.
    pub result_schema_ref: String,
    /// `true` once the command id is promoted into the stable command
    /// graph (and not pane-local only). MUST be `true`.
    pub promoted_to_stable_graph: bool,
}

impl M5CommandDescriptor {
    /// `true` when this command's pinned scope or preview class makes it
    /// high-risk for the audit.
    pub const fn is_high_risk(&self) -> bool {
        self.preview_class.is_high_risk() || self.capability_scope_class.is_high_risk()
    }
}

/// Channel projection reported by one discoverability channel for one
/// command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelProjection {
    /// Channel this projection belongs to.
    pub channel: M5DiscoveryChannel,
    /// Coverage status the channel reports.
    pub coverage_status: M5CoverageStatus,
    /// Projected command id (`None` for non-claimed rows).
    pub projected_command_id: Option<String>,
    /// Projected primary label ref (`None` for non-claimed rows).
    pub projected_label_ref: Option<String>,
    /// Projected lifecycle label (`None` for non-claimed rows).
    pub projected_lifecycle_label: Option<M5LifecycleLabel>,
    /// Projected preview class (`None` for non-claimed rows).
    pub projected_preview_class: Option<M5PreviewClass>,
    /// Projected approval posture (`None` for non-claimed rows).
    pub projected_approval_posture_class: Option<String>,
    /// Projected disabled-reason mode (`None` for non-claimed rows).
    pub projected_disabled_reason_mode: Option<M5DisabledReasonMode>,
    /// Projected automation suitability (`None` for non-claimed rows).
    pub projected_automation_suitability: Option<M5AutomationSuitability>,
    /// Projected automation labels (`[]` for non-claimed rows).
    pub projected_automation_labels: Vec<String>,
    /// Projected help anchor ref (`None` for non-claimed rows).
    pub projected_help_anchor_ref: Option<String>,
    /// Aliases the channel exposes. MUST be a subset of the canonical
    /// alias set on the descriptor.
    pub projected_aliases: Vec<String>,
    /// Narrowing reason set when `coverage_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5ParityBlockingFinding {
    /// A high-risk command has no claim or explicit narrowing for a
    /// required channel.
    UnknownHighRiskGap {
        /// Command that exposes the gap.
        command_id: String,
        /// Channel that exposes the gap.
        channel: M5DiscoveryChannel,
    },
    /// A command is reachable only through its own pane on a channel.
    PointerOnlyAffordance {
        command_id: String,
        channel: M5DiscoveryChannel,
    },
    /// A claimed channel carries a command id that disagrees with the
    /// descriptor.
    CommandIdDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected command id on the channel.
        projected_command_id: String,
    },
    /// A claimed channel carries a label ref that disagrees with the
    /// descriptor.
    LabelDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected label ref.
        projected_label_ref: String,
    },
    /// A claimed channel carries a lifecycle label that disagrees with
    /// the descriptor.
    LifecycleLabelDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected lifecycle label.
        projected_lifecycle_label: M5LifecycleLabel,
    },
    /// A claimed channel carries a preview class that disagrees with the
    /// descriptor.
    PreviewClassDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected preview class.
        projected_preview_class: M5PreviewClass,
    },
    /// A claimed channel carries an approval posture that disagrees with
    /// the descriptor.
    ApprovalPostureDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected approval posture.
        projected_approval_posture_class: String,
    },
    /// A claimed channel drops typed disabled-reason disclosure.
    DisabledReasonDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected disabled-reason mode.
        projected_disabled_reason_mode: M5DisabledReasonMode,
    },
    /// A claimed channel projects an automation suitability that
    /// disagrees with the descriptor (widening or hiding authority).
    AutomationSuitabilityDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Projected automation suitability.
        projected_automation_suitability: M5AutomationSuitability,
    },
    /// A claimed channel projects automation labels that disagree with
    /// the descriptor.
    AutomationLabelDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// First automation label seen that the descriptor does not own.
        offending_automation_label: String,
    },
    /// A claimed channel cannot point back to the canonical help anchor.
    MissingHelpAnchor {
        command_id: String,
        channel: M5DiscoveryChannel,
    },
    /// A claimed channel exposes an alias outside the canonical set.
    AliasDrift {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// First alias seen that the descriptor does not own.
        offending_alias: String,
    },
    /// A non-claimed row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        command_id: String,
        channel: M5DiscoveryChannel,
        coverage_status: M5CoverageStatus,
    },
    /// A claimed row is missing a projection field it requires.
    MissingProjection {
        command_id: String,
        channel: M5DiscoveryChannel,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical help anchor.
    DescriptorMissingHelpAnchor { command_id: String },
    /// The descriptor carries no searchable discoverability metadata.
    MissingSearchMetadata { command_id: String },
    /// A high-risk command declares `always_invokable` instead of a
    /// typed disabled reason.
    MissingDisabledReasonMode { command_id: String },
    /// The command id is not promoted into the stable command graph.
    CommandNotPromoted { command_id: String },
}

impl M5ParityBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnknownHighRiskGap { .. } => "unknown_high_risk_gap",
            Self::PointerOnlyAffordance { .. } => "pointer_only_affordance",
            Self::CommandIdDrift { .. } => "command_id_drift",
            Self::LabelDrift { .. } => "label_drift",
            Self::LifecycleLabelDrift { .. } => "lifecycle_label_drift",
            Self::PreviewClassDrift { .. } => "preview_class_drift",
            Self::ApprovalPostureDrift { .. } => "approval_posture_drift",
            Self::DisabledReasonDrift { .. } => "disabled_reason_drift",
            Self::AutomationSuitabilityDrift { .. } => "automation_suitability_drift",
            Self::AutomationLabelDrift { .. } => "automation_label_drift",
            Self::MissingHelpAnchor { .. } => "missing_help_anchor",
            Self::AliasDrift { .. } => "alias_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingHelpAnchor { .. } => "descriptor_missing_help_anchor",
            Self::MissingSearchMetadata { .. } => "missing_search_metadata",
            Self::MissingDisabledReasonMode { .. } => "missing_disabled_reason_mode",
            Self::CommandNotPromoted { .. } => "command_not_promoted",
        }
    }

    /// Returns the command id this finding is attached to.
    pub fn command_id(&self) -> &str {
        match self {
            Self::UnknownHighRiskGap { command_id, .. }
            | Self::PointerOnlyAffordance { command_id, .. }
            | Self::CommandIdDrift { command_id, .. }
            | Self::LabelDrift { command_id, .. }
            | Self::LifecycleLabelDrift { command_id, .. }
            | Self::PreviewClassDrift { command_id, .. }
            | Self::ApprovalPostureDrift { command_id, .. }
            | Self::DisabledReasonDrift { command_id, .. }
            | Self::AutomationSuitabilityDrift { command_id, .. }
            | Self::AutomationLabelDrift { command_id, .. }
            | Self::MissingHelpAnchor { command_id, .. }
            | Self::AliasDrift { command_id, .. }
            | Self::MissingNarrowingReason { command_id, .. }
            | Self::MissingProjection { command_id, .. }
            | Self::DescriptorMissingHelpAnchor { command_id }
            | Self::MissingSearchMetadata { command_id }
            | Self::MissingDisabledReasonMode { command_id }
            | Self::CommandNotPromoted { command_id } => command_id,
        }
    }

    /// Returns the channel this finding is attached to, when channel-scoped.
    pub fn channel(&self) -> Option<M5DiscoveryChannel> {
        match self {
            Self::UnknownHighRiskGap { channel, .. }
            | Self::PointerOnlyAffordance { channel, .. }
            | Self::CommandIdDrift { channel, .. }
            | Self::LabelDrift { channel, .. }
            | Self::LifecycleLabelDrift { channel, .. }
            | Self::PreviewClassDrift { channel, .. }
            | Self::ApprovalPostureDrift { channel, .. }
            | Self::DisabledReasonDrift { channel, .. }
            | Self::AutomationSuitabilityDrift { channel, .. }
            | Self::AutomationLabelDrift { channel, .. }
            | Self::MissingHelpAnchor { channel, .. }
            | Self::AliasDrift { channel, .. }
            | Self::MissingNarrowingReason { channel, .. }
            | Self::MissingProjection { channel, .. } => Some(*channel),
            Self::DescriptorMissingHelpAnchor { .. }
            | Self::MissingSearchMetadata { .. }
            | Self::MissingDisabledReasonMode { .. }
            | Self::CommandNotPromoted { .. } => None,
        }
    }
}

/// One per-command parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandParityRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the command.
    pub descriptor: M5CommandDescriptor,
    /// Channel-by-channel projections, in canonical channel order.
    pub channels: Vec<M5ChannelProjection>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5ParityBlockingFinding>,
    /// `true` when the command's descriptor classifies it as high-risk.
    pub high_risk: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ParityFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unknown_high_risk_gap` findings.
    pub unknown_high_risk_gap: usize,
    /// Number of `pointer_only_affordance` findings.
    pub pointer_only_affordance: usize,
    /// Number of `command_id_drift` findings.
    pub command_id_drift: usize,
    /// Number of `label_drift` findings.
    pub label_drift: usize,
    /// Number of `lifecycle_label_drift` findings.
    pub lifecycle_label_drift: usize,
    /// Number of `preview_class_drift` findings.
    pub preview_class_drift: usize,
    /// Number of `approval_posture_drift` findings.
    pub approval_posture_drift: usize,
    /// Number of `disabled_reason_drift` findings.
    pub disabled_reason_drift: usize,
    /// Number of `automation_suitability_drift` findings.
    pub automation_suitability_drift: usize,
    /// Number of `automation_label_drift` findings.
    pub automation_label_drift: usize,
    /// Number of `missing_help_anchor` findings.
    pub missing_help_anchor: usize,
    /// Number of `alias_drift` findings.
    pub alias_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_help_anchor` findings.
    pub descriptor_missing_help_anchor: usize,
    /// Number of `missing_search_metadata` findings.
    pub missing_search_metadata: usize,
    /// Number of `missing_disabled_reason_mode` findings.
    pub missing_disabled_reason_mode: usize,
    /// Number of `command_not_promoted` findings.
    pub command_not_promoted: usize,
}

impl M5ParityFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unknown_high_risk_gap: 0,
            pointer_only_affordance: 0,
            command_id_drift: 0,
            label_drift: 0,
            lifecycle_label_drift: 0,
            preview_class_drift: 0,
            approval_posture_drift: 0,
            disabled_reason_drift: 0,
            automation_suitability_drift: 0,
            automation_label_drift: 0,
            missing_help_anchor: 0,
            alias_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_help_anchor: 0,
            missing_search_metadata: 0,
            missing_disabled_reason_mode: 0,
            command_not_promoted: 0,
        }
    }

    fn record(&mut self, finding: &M5ParityBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5ParityBlockingFinding::UnknownHighRiskGap { .. } => self.unknown_high_risk_gap += 1,
            M5ParityBlockingFinding::PointerOnlyAffordance { .. } => {
                self.pointer_only_affordance += 1
            }
            M5ParityBlockingFinding::CommandIdDrift { .. } => self.command_id_drift += 1,
            M5ParityBlockingFinding::LabelDrift { .. } => self.label_drift += 1,
            M5ParityBlockingFinding::LifecycleLabelDrift { .. } => self.lifecycle_label_drift += 1,
            M5ParityBlockingFinding::PreviewClassDrift { .. } => self.preview_class_drift += 1,
            M5ParityBlockingFinding::ApprovalPostureDrift { .. } => {
                self.approval_posture_drift += 1
            }
            M5ParityBlockingFinding::DisabledReasonDrift { .. } => self.disabled_reason_drift += 1,
            M5ParityBlockingFinding::AutomationSuitabilityDrift { .. } => {
                self.automation_suitability_drift += 1
            }
            M5ParityBlockingFinding::AutomationLabelDrift { .. } => {
                self.automation_label_drift += 1
            }
            M5ParityBlockingFinding::MissingHelpAnchor { .. } => self.missing_help_anchor += 1,
            M5ParityBlockingFinding::AliasDrift { .. } => self.alias_drift += 1,
            M5ParityBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5ParityBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5ParityBlockingFinding::DescriptorMissingHelpAnchor { .. } => {
                self.descriptor_missing_help_anchor += 1
            }
            M5ParityBlockingFinding::MissingSearchMetadata { .. } => {
                self.missing_search_metadata += 1
            }
            M5ParityBlockingFinding::MissingDisabledReasonMode { .. } => {
                self.missing_disabled_reason_mode += 1
            }
            M5ParityBlockingFinding::CommandNotPromoted { .. } => self.command_not_promoted += 1,
        }
    }
}

/// Per-channel coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelCoverageSummary {
    /// Channel this summary covers.
    pub channel: M5DiscoveryChannel,
    /// Number of `claimed` rows on this channel.
    pub claimed_rows: usize,
    /// Number of `explicitly_narrowed` rows on this channel.
    pub explicitly_narrowed_rows: usize,
    /// Number of `discoverable_only` rows on this channel.
    pub discoverable_only_rows: usize,
    /// Number of `browser_handoff_only` rows on this channel.
    pub browser_handoff_only_rows: usize,
    /// Number of `voice_addressable` rows on this channel.
    pub voice_addressable_rows: usize,
    /// Number of `not_surfaced_on_this_client` rows on this channel.
    pub not_surfaced_on_this_client_rows: usize,
    /// Number of `custom_pane_only` rows on this channel.
    pub custom_pane_only_rows: usize,
    /// Number of `unknown_high_risk_gap` rows on this channel.
    pub unknown_high_risk_gap_rows: usize,
}

impl M5ChannelCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.discoverable_only_rows
            + self.browser_handoff_only_rows
            + self.voice_addressable_rows
            + self.not_surfaced_on_this_client_rows
    }
}

/// A single help-anchor index entry the audit publishes so help search
/// and the guided tours can reopen each M5 feature by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HelpAnchorEntry {
    /// Feature family the anchor belongs to.
    pub feature_family: M5FeatureFamily,
    /// Command id the anchor reopens.
    pub command_id: String,
    /// Canonical help anchor ref.
    pub help_anchor_ref: String,
}

/// M5 command-parity audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandParityAuditReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source descriptor-diff schema ref for the canonical descriptors.
    pub source_descriptor_schema_ref: String,
    /// Required discoverability channels, in canonical order.
    pub required_channels: Vec<M5DiscoveryChannel>,
    /// Per-command parity rows, sorted by `descriptor.command_id`.
    pub rows: Vec<M5CommandParityRow>,
    /// Per-channel coverage summary, in canonical channel order.
    pub channel_coverage: Vec<M5ChannelCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5ParityFindingSummary,
    /// Canonical help-anchor index, sorted by command id.
    pub help_anchor_index: Vec<M5HelpAnchorEntry>,
    /// Number of registered M5 commands present.
    pub registered_command_count: usize,
    /// Number of high-risk commands present.
    pub high_risk_command_count: usize,
    /// Total channel rows checked.
    pub channel_rows_checked: usize,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5CommandParityAuditReport {
    /// Returns `true` when every required channel is claimed by at least
    /// one command.
    pub fn every_required_channel_claimed(&self) -> bool {
        for channel in M5DiscoveryChannel::required_channels() {
            let any_claimed = self.rows.iter().any(|row| {
                row.channels.iter().any(|projection| {
                    projection.channel == channel
                        && projection.coverage_status == M5CoverageStatus::Claimed
                })
            });
            if !any_claimed {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: commands={}, high_risk={}, channel_rows={}, blocking={}, clean={}",
            self.registered_command_count,
            self.high_risk_command_count,
            self.channel_rows_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for channel in &self.channel_coverage {
            lines.push(format!(
                "{}: claimed={}, narrowed={}, custom_pane_only={}, unknown_high_risk={}",
                channel.channel.display_label(),
                channel.claimed_rows,
                channel.narrowed_rows(),
                channel.custom_pane_only_rows,
                channel.unknown_high_risk_gap_rows,
            ));
        }
        for row in &self.rows {
            for finding in &row.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.command_id(),
                    finding
                        .channel()
                        .map(M5DiscoveryChannel::as_str)
                        .unwrap_or("command"),
                ));
            }
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 command-parity and discoverability audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_command_registry`](../../../../crates/aureline-shell/src/m5_command_registry/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- report-md > \\\n  artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Descriptor schema ref: `{}`\n",
            self.source_descriptor_schema_ref
        ));
        out.push_str(&format!(
            "- Registered M5 commands: `{}`\n",
            self.registered_command_count
        ));
        out.push_str(&format!(
            "- High-risk commands: `{}`\n",
            self.high_risk_command_count
        ));
        out.push_str(&format!(
            "- Channel rows checked: `{}`\n",
            self.channel_rows_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Per-channel coverage\n\n");
        out.push_str(
            "| Channel | Claimed | Narrowed | Pointer-only | Unknown high-risk |\n\
             | ------- | ------: | -------: | -----------: | ----------------: |\n",
        );
        for channel in &self.channel_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                channel.channel.display_label(),
                channel.claimed_rows,
                channel.narrowed_rows(),
                channel.custom_pane_only_rows,
                channel.unknown_high_risk_gap_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unknown_high_risk_gap` | {} |\n",
            self.findings_summary.unknown_high_risk_gap
        ));
        out.push_str(&format!(
            "| `pointer_only_affordance` | {} |\n",
            self.findings_summary.pointer_only_affordance
        ));
        out.push_str(&format!(
            "| `command_id_drift` | {} |\n",
            self.findings_summary.command_id_drift
        ));
        out.push_str(&format!(
            "| `label_drift` | {} |\n",
            self.findings_summary.label_drift
        ));
        out.push_str(&format!(
            "| `lifecycle_label_drift` | {} |\n",
            self.findings_summary.lifecycle_label_drift
        ));
        out.push_str(&format!(
            "| `preview_class_drift` | {} |\n",
            self.findings_summary.preview_class_drift
        ));
        out.push_str(&format!(
            "| `approval_posture_drift` | {} |\n",
            self.findings_summary.approval_posture_drift
        ));
        out.push_str(&format!(
            "| `disabled_reason_drift` | {} |\n",
            self.findings_summary.disabled_reason_drift
        ));
        out.push_str(&format!(
            "| `automation_suitability_drift` | {} |\n",
            self.findings_summary.automation_suitability_drift
        ));
        out.push_str(&format!(
            "| `automation_label_drift` | {} |\n",
            self.findings_summary.automation_label_drift
        ));
        out.push_str(&format!(
            "| `missing_help_anchor` | {} |\n",
            self.findings_summary.missing_help_anchor
        ));
        out.push_str(&format!(
            "| `alias_drift` | {} |\n",
            self.findings_summary.alias_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_help_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_help_anchor
        ));
        out.push_str(&format!(
            "| `missing_search_metadata` | {} |\n",
            self.findings_summary.missing_search_metadata
        ));
        out.push_str(&format!(
            "| `missing_disabled_reason_mode` | {} |\n",
            self.findings_summary.missing_disabled_reason_mode
        ));
        out.push_str(&format!(
            "| `command_not_promoted` | {} |\n\n",
            self.findings_summary.command_not_promoted
        ));

        out.push_str("## Help anchor index\n\n");
        out.push_str("| Feature family | Command | Help anchor |\n| -------------- | ------- | ----------- |\n");
        for entry in &self.help_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.feature_family.display_label(),
                entry.command_id,
                entry.help_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-command rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {})\n\n",
                row.descriptor.command_id,
                row.descriptor.feature_family.as_str(),
                row.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                row.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Preview class: `{}`\n",
                row.descriptor.preview_class.as_str()
            ));
            out.push_str(&format!(
                "- Approval posture: `{}`\n",
                row.descriptor.approval_posture_class
            ));
            out.push_str(&format!(
                "- Capability scope: `{}`\n",
                row.descriptor.capability_scope_class.as_str()
            ));
            out.push_str(&format!(
                "- Mutability class: `{}`\n",
                row.descriptor.mutability_class.as_str()
            ));
            out.push_str(&format!(
                "- Disabled reason mode: `{}`\n",
                row.descriptor.disabled_reason_mode.as_str()
            ));
            out.push_str(&format!(
                "- Automation suitability: `{}`\n",
                row.descriptor.automation_suitability.as_str()
            ));
            out.push_str(&format!(
                "- Automation labels: `{}`\n",
                row.descriptor.automation_labels.join(", ")
            ));
            out.push_str(&format!("- Origin: `{}`\n", row.descriptor.origin_class));
            out.push_str(&format!(
                "- Help anchor: `{}`\n",
                row.descriptor.help_anchor_ref
            ));
            out.push_str(&format!(
                "- High-risk: `{}`\n\n",
                if row.high_risk { "yes" } else { "no" }
            ));

            out.push_str(
                "| Channel | Status | Projected preview | Disabled reason mode | Automation | Narrowing reason |\n\
                 | ------- | ------ | ----------------- | -------------------- | ---------- | ---------------- |\n",
            );
            for projection in &row.channels {
                let preview = projection
                    .projected_preview_class
                    .map(|class| class.as_str())
                    .unwrap_or("-");
                let mode = projection
                    .projected_disabled_reason_mode
                    .map(|mode| mode.as_str())
                    .unwrap_or("-");
                let automation = projection
                    .projected_automation_suitability
                    .map(|suit| suit.as_str())
                    .unwrap_or("-");
                let narrowing = projection.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    projection.channel.display_label(),
                    projection.coverage_status.as_str(),
                    preview,
                    mode,
                    automation,
                    narrowing,
                ));
            }
            out.push('\n');

            if row.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &row.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .channel()
                            .map(M5DiscoveryChannel::as_str)
                            .unwrap_or("command"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_command_parity_fixtures\n");
        out.push_str("python3 tools/ci/m5/command_parity_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 command-parity audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5CommandParityAuditReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5CommandParitySupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5CommandParityAuditReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for row in &report.rows {
            case_ids.push(row.descriptor.command_id.clone());
            case_ids.push(row.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_COMMAND_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_COMMAND_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-row blocking findings from a descriptor and its
/// channel projections.
fn compute_row_findings(
    descriptor: &M5CommandDescriptor,
    channels: &[M5ChannelProjection],
    high_risk: bool,
) -> Vec<M5ParityBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (command-scoped) findings.
    if descriptor.help_anchor_ref.trim().is_empty() {
        findings.push(M5ParityBlockingFinding::DescriptorMissingHelpAnchor {
            command_id: descriptor.command_id.clone(),
        });
    }
    if descriptor
        .search_keywords
        .iter()
        .all(|keyword| keyword.trim().is_empty())
    {
        findings.push(M5ParityBlockingFinding::MissingSearchMetadata {
            command_id: descriptor.command_id.clone(),
        });
    }
    if high_risk && descriptor.disabled_reason_mode == M5DisabledReasonMode::AlwaysInvokable {
        findings.push(M5ParityBlockingFinding::MissingDisabledReasonMode {
            command_id: descriptor.command_id.clone(),
        });
    }
    if !descriptor.promoted_to_stable_graph {
        findings.push(M5ParityBlockingFinding::CommandNotPromoted {
            command_id: descriptor.command_id.clone(),
        });
    }

    let canonical_aliases: BTreeSet<&str> = descriptor
        .canonical_aliases
        .iter()
        .map(String::as_str)
        .collect();

    for projection in channels {
        let channel = projection.channel;
        match projection.coverage_status {
            M5CoverageStatus::UnknownHighRiskGap => {
                if high_risk {
                    findings.push(M5ParityBlockingFinding::UnknownHighRiskGap {
                        command_id: descriptor.command_id.clone(),
                        channel,
                    });
                }
            }
            M5CoverageStatus::CustomPaneOnly => {
                findings.push(M5ParityBlockingFinding::PointerOnlyAffordance {
                    command_id: descriptor.command_id.clone(),
                    channel,
                });
            }
            M5CoverageStatus::Claimed => {
                match &projection.projected_command_id {
                    Some(id) if id == &descriptor.command_id => {}
                    Some(id) => findings.push(M5ParityBlockingFinding::CommandIdDrift {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        projected_command_id: id.clone(),
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_command_id".to_owned(),
                    }),
                }

                match &projection.projected_label_ref {
                    Some(label) if label == &descriptor.primary_label_ref => {}
                    Some(label) => findings.push(M5ParityBlockingFinding::LabelDrift {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        projected_label_ref: label.clone(),
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_label_ref".to_owned(),
                    }),
                }

                match projection.projected_lifecycle_label {
                    Some(lifecycle) if lifecycle == descriptor.lifecycle_label => {}
                    Some(lifecycle) => {
                        findings.push(M5ParityBlockingFinding::LifecycleLabelDrift {
                            command_id: descriptor.command_id.clone(),
                            channel,
                            projected_lifecycle_label: lifecycle,
                        })
                    }
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_lifecycle_label".to_owned(),
                    }),
                }

                match projection.projected_preview_class {
                    Some(preview) if preview == descriptor.preview_class => {}
                    Some(preview) => findings.push(M5ParityBlockingFinding::PreviewClassDrift {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        projected_preview_class: preview,
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_preview_class".to_owned(),
                    }),
                }

                match &projection.projected_approval_posture_class {
                    Some(approval) if approval == &descriptor.approval_posture_class => {}
                    Some(approval) => {
                        findings.push(M5ParityBlockingFinding::ApprovalPostureDrift {
                            command_id: descriptor.command_id.clone(),
                            channel,
                            projected_approval_posture_class: approval.clone(),
                        })
                    }
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_approval_posture_class".to_owned(),
                    }),
                }

                match projection.projected_disabled_reason_mode {
                    Some(mode) if mode == descriptor.disabled_reason_mode => {}
                    Some(mode) => findings.push(M5ParityBlockingFinding::DisabledReasonDrift {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        projected_disabled_reason_mode: mode,
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_disabled_reason_mode".to_owned(),
                    }),
                }

                match projection.projected_automation_suitability {
                    Some(suit) if suit == descriptor.automation_suitability => {}
                    Some(suit) => {
                        findings.push(M5ParityBlockingFinding::AutomationSuitabilityDrift {
                            command_id: descriptor.command_id.clone(),
                            channel,
                            projected_automation_suitability: suit,
                        })
                    }
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        field: "projected_automation_suitability".to_owned(),
                    }),
                }

                let canonical_automation_labels: BTreeSet<&str> = descriptor
                    .automation_labels
                    .iter()
                    .map(String::as_str)
                    .collect();
                for label in &projection.projected_automation_labels {
                    if !canonical_automation_labels.contains(label.as_str()) {
                        findings.push(M5ParityBlockingFinding::AutomationLabelDrift {
                            command_id: descriptor.command_id.clone(),
                            channel,
                            offending_automation_label: label.clone(),
                        });
                    }
                }
                if projection.projected_automation_labels.len()
                    != descriptor.automation_labels.len()
                {
                    let missing_label = descriptor
                        .automation_labels
                        .iter()
                        .find(|label| !projection.projected_automation_labels.contains(*label))
                        .cloned()
                        .unwrap_or_else(|| "automation_label_cardinality_mismatch".to_owned());
                    findings.push(M5ParityBlockingFinding::AutomationLabelDrift {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        offending_automation_label: missing_label,
                    });
                }

                match &projection.projected_help_anchor_ref {
                    Some(anchor) if anchor == &descriptor.help_anchor_ref => {}
                    Some(_) | None => {
                        findings.push(M5ParityBlockingFinding::MissingHelpAnchor {
                            command_id: descriptor.command_id.clone(),
                            channel,
                        });
                    }
                }

                for alias in &projection.projected_aliases {
                    if !canonical_aliases.contains(alias.as_str()) {
                        findings.push(M5ParityBlockingFinding::AliasDrift {
                            command_id: descriptor.command_id.clone(),
                            channel,
                            offending_alias: alias.clone(),
                        });
                    }
                }
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = projection
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5ParityBlockingFinding::MissingNarrowingReason {
                        command_id: descriptor.command_id.clone(),
                        channel,
                        coverage_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the per-channel and per-class summaries from finished rows.
fn summarize_report(
    rows: &[M5CommandParityRow],
) -> (Vec<M5ChannelCoverageSummary>, M5ParityFindingSummary) {
    let mut summary = M5ParityFindingSummary::empty();
    let mut coverage: Vec<M5ChannelCoverageSummary> = M5DiscoveryChannel::required_channels()
        .into_iter()
        .map(|channel| M5ChannelCoverageSummary {
            channel,
            claimed_rows: 0,
            explicitly_narrowed_rows: 0,
            discoverable_only_rows: 0,
            browser_handoff_only_rows: 0,
            voice_addressable_rows: 0,
            not_surfaced_on_this_client_rows: 0,
            custom_pane_only_rows: 0,
            unknown_high_risk_gap_rows: 0,
        })
        .collect();

    for row in rows {
        for projection in &row.channels {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|coverage| coverage.channel == projection.channel)
            {
                match projection.coverage_status {
                    M5CoverageStatus::Claimed => coverage_row.claimed_rows += 1,
                    M5CoverageStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5CoverageStatus::DiscoverableOnly => coverage_row.discoverable_only_rows += 1,
                    M5CoverageStatus::BrowserHandoffOnly => {
                        coverage_row.browser_handoff_only_rows += 1
                    }
                    M5CoverageStatus::VoiceAddressable => coverage_row.voice_addressable_rows += 1,
                    M5CoverageStatus::NotSurfacedOnThisClient => {
                        coverage_row.not_surfaced_on_this_client_rows += 1
                    }
                    M5CoverageStatus::CustomPaneOnly => coverage_row.custom_pane_only_rows += 1,
                    M5CoverageStatus::UnknownHighRiskGap => {
                        coverage_row.unknown_high_risk_gap_rows += 1
                    }
                }
            }
        }
        for finding in &row.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Builds an [`M5CommandParityRow`] from a descriptor and its channel
/// projections, computing the per-row blocking findings.
pub fn build_m5_command_parity_row(
    descriptor: M5CommandDescriptor,
    channels: Vec<M5ChannelProjection>,
) -> M5CommandParityRow {
    let high_risk = descriptor.is_high_risk();
    let blocking_findings = compute_row_findings(&descriptor, &channels, high_risk);

    M5CommandParityRow {
        record_kind: M5_COMMAND_PARITY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_COMMAND_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_COMMAND_PARITY_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        channels,
        blocking_findings,
        high_risk,
    }
}

/// Builds a full [`M5CommandParityAuditReport`] from per-command rows.
pub fn build_m5_command_parity_audit(rows: Vec<M5CommandParityRow>) -> M5CommandParityAuditReport {
    let mut rows = rows;
    rows.sort_by(|left, right| left.descriptor.command_id.cmp(&right.descriptor.command_id));

    let registered_command_count = rows.len();
    let high_risk_command_count = rows.iter().filter(|row| row.high_risk).count();
    let channel_rows_checked = rows.iter().map(|row| row.channels.len()).sum::<usize>();

    let (channel_coverage, findings_summary) = summarize_report(&rows);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut help_anchor_index: Vec<M5HelpAnchorEntry> = rows
        .iter()
        .map(|row| M5HelpAnchorEntry {
            feature_family: row.descriptor.feature_family,
            command_id: row.descriptor.command_id.clone(),
            help_anchor_ref: row.descriptor.help_anchor_ref.clone(),
        })
        .collect();
    help_anchor_index.sort_by(|left, right| left.command_id.cmp(&right.command_id));

    M5CommandParityAuditReport {
        record_kind: M5_COMMAND_PARITY_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_COMMAND_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_COMMAND_PARITY_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_COMMAND_PARITY_REPORT_ID.to_owned(),
        source_descriptor_schema_ref: M5_COMMAND_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        required_channels: M5DiscoveryChannel::required_channels().to_vec(),
        rows,
        channel_coverage,
        findings_summary,
        help_anchor_index,
        registered_command_count,
        high_risk_command_count,
        channel_rows_checked,
        report_clean,
        published_report_ref: M5_COMMAND_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_COMMAND_PARITY_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_COMMAND_PARITY_PUBLISHED_DOC_REF.to_owned(),
            "docs/ux/command_discoverability_coverage_matrix.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-command-parity".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_command_parity_audit`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5CommandParityValidationError {
    /// The audit has no registered commands.
    NoRegisteredCommands,
    /// A required channel has no claimed row.
    RequiredChannelNotClaimed { channel: String },
    /// A row is missing a required channel from its projection set.
    MissingRequiredChannel { command_id: String, channel: String },
    /// A blocking finding remains on the row.
    BlockingFindingPresent {
        command_id: String,
        channel: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A row's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { command_id: String },
}

/// Validates an audit report against the M5 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_command_parity_audit(
    report: &M5CommandParityAuditReport,
) -> Result<(), Vec<M5CommandParityValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5CommandParityValidationError::NoRegisteredCommands);
    }

    for required in M5DiscoveryChannel::required_channels() {
        let any_claimed = report.rows.iter().any(|row| {
            row.channels.iter().any(|projection| {
                projection.channel == required
                    && projection.coverage_status == M5CoverageStatus::Claimed
            })
        });
        if !any_claimed {
            errors.push(M5CommandParityValidationError::RequiredChannelNotClaimed {
                channel: required.as_str().to_owned(),
            });
        }
    }

    for row in &report.rows {
        for required in M5DiscoveryChannel::required_channels() {
            if !row
                .channels
                .iter()
                .any(|projection| projection.channel == required)
            {
                errors.push(M5CommandParityValidationError::MissingRequiredChannel {
                    command_id: row.descriptor.command_id.clone(),
                    channel: required.as_str().to_owned(),
                });
            }
        }
        if row.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                M5CommandParityValidationError::MissingDescriptorRevisionRef {
                    command_id: row.descriptor.command_id.clone(),
                },
            );
        }
        for finding in &row.blocking_findings {
            errors.push(M5CommandParityValidationError::BlockingFindingPresent {
                command_id: finding.command_id().to_owned(),
                channel: finding
                    .channel()
                    .map(|channel| channel.as_str().to_owned())
                    .unwrap_or_else(|| "command".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5CommandParityValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5CommandParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_command_parity_audit`].
#[allow(dead_code)]
struct CommandSeed {
    command_id: &'static str,
    feature_family: M5FeatureFamily,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    help_anchor_ref: &'static str,
    search_keywords: &'static [&'static str],
    lifecycle_label: M5LifecycleLabel,
    preview_class: M5PreviewClass,
    capability_scope_class: M5CapabilityScope,
    disabled_reason_mode: M5DisabledReasonMode,
    automation_suitability: M5AutomationSuitability,
    canonical_aliases: &'static [&'static str],
    channels: &'static [ChannelSeed],
}

struct ChannelSeed {
    channel: M5DiscoveryChannel,
    coverage_status: M5CoverageStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
    projected_aliases: &'static [&'static str],
}

/// Helper: the five non-AI channels all claimed, with an optional
/// keybinding narrowing. Used by most read-only commands.
const fn claimed(channel: M5DiscoveryChannel) -> ChannelSeed {
    ChannelSeed {
        channel,
        coverage_status: M5CoverageStatus::Claimed,
        narrowing_reason: None,
        note: None,
        projected_aliases: &[],
    }
}

const KEYBINDING_NARROWED: ChannelSeed = ChannelSeed {
    channel: M5DiscoveryChannel::KeybindingHelp,
    coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
    narrowing_reason: Some("keybinding_unassigned_at_beta"),
    note: Some("Keybinding help lists the command without a default chord."),
    projected_aliases: &[],
};

const COMMAND_SEEDS: &[CommandSeed] = &[
    // Notebook: run all cells. Reversible local mutation; AI drafts only.
    CommandSeed {
        command_id: "cmd:notebook.run_all_cells",
        feature_family: M5FeatureFamily::Notebook,
        descriptor_revision_ref: "cmd-rev:notebook.run_all_cells:2026.06.01-01",
        primary_label_ref: "label:notebook.run_all_cells:primary",
        help_anchor_ref: "help:anchor:notebook:run_all_cells",
        search_keywords: &["notebook", "run", "execute", "cells"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        capability_scope_class: M5CapabilityScope::ReversibleLocalMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::DraftOnly,
        canonical_aliases: &["alias:notebook.run_all_cells:cli_run"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            claimed(M5DiscoveryChannel::KeybindingHelp),
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:notebook.run_all_cells:cli_run"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("draft_only_human_runs_cells"),
                note: Some("AI may draft cells; a human runs them."),
                projected_aliases: &[],
            },
        ],
    },
    // Data / API: send request. Irreversible network publish; human only.
    CommandSeed {
        command_id: "cmd:data_api.send_request",
        feature_family: M5FeatureFamily::DataApi,
        descriptor_revision_ref: "cmd-rev:data_api.send_request:2026.06.01-01",
        primary_label_ref: "label:data_api.send_request:primary",
        help_anchor_ref: "help:anchor:data_api:send_request",
        search_keywords: &["data", "api", "request", "send", "http"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::IrreversiblePublishPreview,
        capability_scope_class: M5CapabilityScope::IrreversiblePublish,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        canonical_aliases: &["alias:data_api.send_request:cli_send"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:data_api.send_request:cli_send"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required_network_publish"),
                note: Some("AI may compose a request but cannot send without confirmation."),
                projected_aliases: &[],
            },
        ],
    },
    // Profiler: start capture. Reversible local read; fully automatable.
    CommandSeed {
        command_id: "cmd:profiler.start_capture",
        feature_family: M5FeatureFamily::Profiler,
        descriptor_revision_ref: "cmd-rev:profiler.start_capture:2026.06.01-01",
        primary_label_ref: "label:profiler.start_capture:primary",
        help_anchor_ref: "help:anchor:profiler:start_capture",
        search_keywords: &["profiler", "capture", "performance", "trace"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        capability_scope_class: M5CapabilityScope::ReversibleLocalRead,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::FullyAutomatable,
        canonical_aliases: &["alias:profiler.start_capture:cli_profile"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            claimed(M5DiscoveryChannel::KeybindingHelp),
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:profiler.start_capture:cli_profile"],
            },
            claimed(M5DiscoveryChannel::AiAutomation),
        ],
    },
    // Trace / replay: replay session. Recoverable durable mutation.
    CommandSeed {
        command_id: "cmd:trace_replay.replay_session",
        feature_family: M5FeatureFamily::TraceReplay,
        descriptor_revision_ref: "cmd-rev:trace_replay.replay_session:2026.06.01-01",
        primary_label_ref: "label:trace_replay.replay_session:primary",
        help_anchor_ref: "help:anchor:trace_replay:replay_session",
        search_keywords: &["trace", "replay", "session", "deterministic"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::StructuredDiffPreview,
        capability_scope_class: M5CapabilityScope::RecoverableDurableMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::PreviewThenConfirm,
        canonical_aliases: &["alias:trace_replay.replay_session:cli_replay"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:trace_replay.replay_session:cli_replay"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: Some("AI replays behind the same structured-diff preview."),
                projected_aliases: &[],
            },
        ],
    },
    // Docs / browser: open external. Reversible local read; browser handoff.
    CommandSeed {
        command_id: "cmd:docs_browser.open_external",
        feature_family: M5FeatureFamily::DocsBrowser,
        descriptor_revision_ref: "cmd-rev:docs_browser.open_external:2026.06.01-01",
        primary_label_ref: "label:docs_browser.open_external:primary",
        help_anchor_ref: "help:anchor:docs_browser:open_external",
        search_keywords: &["docs", "browser", "open", "external", "link"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        capability_scope_class: M5CapabilityScope::ReversibleLocalRead,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::FullyAutomatable,
        canonical_aliases: &["alias:docs_browser.open_external:cli_open_docs"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:docs_browser.open_external:cli_open_docs"],
            },
            claimed(M5DiscoveryChannel::AiAutomation),
        ],
    },
    // Template / scaffold: scaffold project. Recoverable durable mutation.
    CommandSeed {
        command_id: "cmd:template_scaffold.scaffold_project",
        feature_family: M5FeatureFamily::TemplateScaffold,
        descriptor_revision_ref: "cmd-rev:template_scaffold.scaffold_project:2026.06.01-01",
        primary_label_ref: "label:template_scaffold.scaffold_project:primary",
        help_anchor_ref: "help:anchor:template_scaffold:scaffold_project",
        search_keywords: &["template", "scaffold", "generate", "new", "project"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::StructuredDiffPreview,
        capability_scope_class: M5CapabilityScope::RecoverableDurableMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::PreviewThenConfirm,
        canonical_aliases: &["alias:template_scaffold.scaffold_project:cli_scaffold"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:template_scaffold.scaffold_project:cli_scaffold"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: Some("AI scaffolds behind the same structured-diff preview."),
                projected_aliases: &[],
            },
        ],
    },
    // Review / pipeline: run pipeline. Recoverable durable mutation; draft only.
    CommandSeed {
        command_id: "cmd:review_pipeline.run_pipeline",
        feature_family: M5FeatureFamily::ReviewPipeline,
        descriptor_revision_ref: "cmd-rev:review_pipeline.run_pipeline:2026.06.01-01",
        primary_label_ref: "label:review_pipeline.run_pipeline:primary",
        help_anchor_ref: "help:anchor:review_pipeline:run_pipeline",
        search_keywords: &["review", "pipeline", "run", "ci", "checks"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::StructuredDiffPreview,
        capability_scope_class: M5CapabilityScope::RecoverableDurableMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::DraftOnly,
        canonical_aliases: &["alias:review_pipeline.run_pipeline:cli_pipeline"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:review_pipeline.run_pipeline:cli_pipeline"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("draft_only_human_dispatches_pipeline"),
                note: Some("AI may draft pipeline runs; a human dispatches them."),
                projected_aliases: &[],
            },
        ],
    },
    // Preview: open live preview. Reversible local read; fully automatable.
    CommandSeed {
        command_id: "cmd:preview.open_live_preview",
        feature_family: M5FeatureFamily::Preview,
        descriptor_revision_ref: "cmd-rev:preview.open_live_preview:2026.06.01-01",
        primary_label_ref: "label:preview.open_live_preview:primary",
        help_anchor_ref: "help:anchor:preview:open_live_preview",
        search_keywords: &["preview", "live", "open", "render"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        capability_scope_class: M5CapabilityScope::ReversibleLocalRead,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::FullyAutomatable,
        canonical_aliases: &["alias:preview.open_live_preview:cli_preview"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            claimed(M5DiscoveryChannel::KeybindingHelp),
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:preview.open_live_preview:cli_preview"],
            },
            claimed(M5DiscoveryChannel::AiAutomation),
        ],
    },
    // Companion: hand off session. Reversible local mutation; draft only.
    CommandSeed {
        command_id: "cmd:companion.handoff_session",
        feature_family: M5FeatureFamily::Companion,
        descriptor_revision_ref: "cmd-rev:companion.handoff_session:2026.06.01-01",
        primary_label_ref: "label:companion.handoff_session:primary",
        help_anchor_ref: "help:anchor:companion:handoff_session",
        search_keywords: &["companion", "handoff", "device", "transfer"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        capability_scope_class: M5CapabilityScope::ReversibleLocalMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::DraftOnly,
        canonical_aliases: &["alias:companion.handoff_session:cli_handoff"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("ui_only_handoff_requires_local_session"),
                note: Some(
                    "Handoff is surfaced for discoverability; the route needs a live UI session.",
                ),
                projected_aliases: &[],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("draft_only_human_confirms_handoff"),
                note: Some("AI may stage a handoff; a human confirms it."),
                projected_aliases: &[],
            },
        ],
    },
    // Incident: open incident. Policy authoring; recoverable durable; human only.
    CommandSeed {
        command_id: "cmd:incident.open_incident",
        feature_family: M5FeatureFamily::Incident,
        descriptor_revision_ref: "cmd-rev:incident.open_incident:2026.06.01-01",
        primary_label_ref: "label:incident.open_incident:primary",
        help_anchor_ref: "help:anchor:incident:open_incident",
        search_keywords: &["incident", "declare", "open", "escalate"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::PolicyAuthoringOrWaiverPreview,
        capability_scope_class: M5CapabilityScope::RecoverableDurableMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        canonical_aliases: &["alias:incident.open_incident:cli_incident"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:incident.open_incident:cli_incident"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required_human_declares_incident"),
                note: Some("AI may prepare an incident draft; a human declares it."),
                projected_aliases: &[],
            },
        ],
    },
    // Sync: push workspace state. Irreversible publish; human only.
    CommandSeed {
        command_id: "cmd:sync.push_workspace_state",
        feature_family: M5FeatureFamily::Sync,
        descriptor_revision_ref: "cmd-rev:sync.push_workspace_state:2026.06.01-01",
        primary_label_ref: "label:sync.push_workspace_state:primary",
        help_anchor_ref: "help:anchor:sync:push_workspace_state",
        search_keywords: &["sync", "push", "publish", "workspace", "upload"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::IrreversiblePublishPreview,
        capability_scope_class: M5CapabilityScope::IrreversiblePublish,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        canonical_aliases: &["alias:sync.push_workspace_state:cli_push"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:sync.push_workspace_state:cli_push"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required_irreversible_publish"),
                note: Some("AI may stage a push; a human confirms the irreversible publish."),
                projected_aliases: &[],
            },
        ],
    },
    // Offboarding: export and wipe. Destructive bulk mutation; human only.
    CommandSeed {
        command_id: "cmd:offboarding.export_and_wipe",
        feature_family: M5FeatureFamily::Offboarding,
        descriptor_revision_ref: "cmd-rev:offboarding.export_and_wipe:2026.06.01-01",
        primary_label_ref: "label:offboarding.export_and_wipe:primary",
        help_anchor_ref: "help:anchor:offboarding:export_and_wipe",
        search_keywords: &["offboarding", "export", "wipe", "delete", "leave"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::DestructiveBulkMutationPreview,
        capability_scope_class: M5CapabilityScope::DestructiveBulkMutation,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        canonical_aliases: &["alias:offboarding.export_and_wipe:cli_offboard"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:offboarding.export_and_wipe:cli_offboard"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required_destructive_bulk_wipe"),
                note: Some("AI may stage the export; a human confirms the destructive wipe."),
                projected_aliases: &[],
            },
        ],
    },
    // Secret broker: credential review. Sensitive review lane; human only.
    CommandSeed {
        command_id: "cmd:secret_broker.open_credential_review",
        feature_family: M5FeatureFamily::SecretBroker,
        descriptor_revision_ref: "cmd-rev:secret_broker.open_credential_review:2026.06.12-01",
        primary_label_ref: "label:secret_broker.open_credential_review:primary",
        help_anchor_ref: "help:anchor:secret_broker:open_credential_review",
        search_keywords: &["secret", "credential", "review", "broker"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        capability_scope_class: M5CapabilityScope::CredentialOrSecretBearing,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        canonical_aliases: &["alias:secret_broker.open_credential_review:cli_secret_review"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("ui_only_secret_review_requires_local_session"),
                note: Some(
                    "The review command is discoverable on CLI, but the sensitive sheet stays in the local session.",
                ),
                projected_aliases: &[],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required_sensitive_review"),
                note: Some("AI may reference the review step; a human opens the sensitive route."),
                projected_aliases: &[],
            },
        ],
    },
    // Secret broker: credential rotation. Sensitive preview lane; human approves.
    CommandSeed {
        command_id: "cmd:secret_broker.open_credential_rotation",
        feature_family: M5FeatureFamily::SecretBroker,
        descriptor_revision_ref: "cmd-rev:secret_broker.open_credential_rotation:2026.06.12-01",
        primary_label_ref: "label:secret_broker.open_credential_rotation:primary",
        help_anchor_ref: "help:anchor:secret_broker:open_credential_rotation",
        search_keywords: &["secret", "credential", "rotation", "broker"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::StructuredDiffPreview,
        capability_scope_class: M5CapabilityScope::CredentialOrSecretBearing,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        canonical_aliases: &["alias:secret_broker.open_credential_rotation:cli_secret_rotate"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:secret_broker.open_credential_rotation:cli_secret_rotate"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::ExplicitlyNarrowed,
                narrowing_reason: Some("approval_required_sensitive_rotation"),
                note: Some("AI may prepare rotation details; a human approves the sensitive change."),
                projected_aliases: &[],
            },
        ],
    },
    // Infrastructure: reconcile workspace. Managed control lane; preview and approval preserved.
    CommandSeed {
        command_id: "cmd:infrastructure.reconcile_workspace",
        feature_family: M5FeatureFamily::Infrastructure,
        descriptor_revision_ref: "cmd-rev:infrastructure.reconcile_workspace:2026.06.12-01",
        primary_label_ref: "label:infrastructure.reconcile_workspace:primary",
        help_anchor_ref: "help:anchor:infrastructure:reconcile_workspace",
        search_keywords: &["infrastructure", "reconcile", "workspace", "control", "plane"],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::StructuredDiffPreview,
        capability_scope_class: M5CapabilityScope::ManagedWorkspaceControl,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::PreviewThenConfirm,
        canonical_aliases: &["alias:infrastructure.reconcile_workspace:cli_reconcile"],
        channels: &[
            claimed(M5DiscoveryChannel::CommandPalette),
            KEYBINDING_NARROWED,
            claimed(M5DiscoveryChannel::HelpSearch),
            claimed(M5DiscoveryChannel::OnboardingTour),
            ChannelSeed {
                channel: M5DiscoveryChannel::CliHeadless,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: None,
                projected_aliases: &["alias:infrastructure.reconcile_workspace:cli_reconcile"],
            },
            ChannelSeed {
                channel: M5DiscoveryChannel::AiAutomation,
                coverage_status: M5CoverageStatus::Claimed,
                narrowing_reason: None,
                note: Some("AI uses the same preview and approval boundary as desktop and CLI."),
                projected_aliases: &[],
            },
        ],
    },
];

fn build_projection_from_seed(
    descriptor: &M5CommandDescriptor,
    seed: &ChannelSeed,
) -> M5ChannelProjection {
    let projects_descriptor = seed.coverage_status.projects_descriptor();
    M5ChannelProjection {
        channel: seed.channel,
        coverage_status: seed.coverage_status,
        projected_command_id: projects_descriptor.then(|| descriptor.command_id.clone()),
        projected_label_ref: projects_descriptor.then(|| descriptor.primary_label_ref.clone()),
        projected_lifecycle_label: projects_descriptor.then_some(descriptor.lifecycle_label),
        projected_preview_class: projects_descriptor.then_some(descriptor.preview_class),
        projected_approval_posture_class: projects_descriptor
            .then(|| descriptor.approval_posture_class.clone()),
        projected_disabled_reason_mode: projects_descriptor
            .then_some(descriptor.disabled_reason_mode),
        projected_automation_suitability: projects_descriptor
            .then_some(descriptor.automation_suitability),
        projected_automation_labels: projects_descriptor
            .then(|| descriptor.automation_labels.clone())
            .unwrap_or_default(),
        projected_help_anchor_ref: projects_descriptor.then(|| descriptor.help_anchor_ref.clone()),
        projected_aliases: seed
            .projected_aliases
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrowing_reason: seed.narrowing_reason.map(str::to_owned),
        note: seed.note.map(str::to_owned),
    }
}

fn command_entry(command_id: &str) -> &'static CommandRegistryEntryRecord {
    seeded_registry()
        .get(command_id)
        .unwrap_or_else(|| panic!("M5 audit command {command_id} must exist in canonical registry"))
}

fn discoverability_string_array(
    entry: &CommandRegistryEntryRecord,
    field_name: &str,
) -> Vec<String> {
    entry
        .discoverability_record
        .get(field_name)
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn lifecycle_label(entry: &CommandRegistryEntryRecord) -> M5LifecycleLabel {
    match entry.descriptor.lifecycle_state.as_str() {
        "stable" => M5LifecycleLabel::Stable,
        "beta" => M5LifecycleLabel::Beta,
        "deprecated" => M5LifecycleLabel::Deprecated,
        other => panic!(
            "M5 audit command {} has unsupported lifecycle state {other}",
            entry.descriptor.command_id
        ),
    }
}

fn preview_class(entry: &CommandRegistryEntryRecord) -> M5PreviewClass {
    match entry.descriptor.preview_class.as_str() {
        "no_preview_required" => M5PreviewClass::NoPreviewRequired,
        "structured_diff_preview" => M5PreviewClass::StructuredDiffPreview,
        "destructive_bulk_mutation_preview" => M5PreviewClass::DestructiveBulkMutationPreview,
        "policy_authoring_or_waiver_preview" => M5PreviewClass::PolicyAuthoringOrWaiverPreview,
        "irreversible_publish_preview" => M5PreviewClass::IrreversiblePublishPreview,
        other => panic!(
            "M5 audit command {} has unsupported preview class {other}",
            entry.descriptor.command_id
        ),
    }
}

fn capability_scope(entry: &CommandRegistryEntryRecord) -> M5CapabilityScope {
    match entry.descriptor.capability_scope_class.as_str() {
        "inert_metadata_only" => M5CapabilityScope::InertMetadataOnly,
        "reversible_local_read" => M5CapabilityScope::ReversibleLocalRead,
        "reversible_local_mutation" => M5CapabilityScope::ReversibleLocalMutation,
        "recoverable_durable_mutation" => M5CapabilityScope::RecoverableDurableMutation,
        "destructive_bulk_mutation" => M5CapabilityScope::DestructiveBulkMutation,
        "irreversible_publish" => M5CapabilityScope::IrreversiblePublish,
        "credential_or_secret_bearing" => M5CapabilityScope::CredentialOrSecretBearing,
        "managed_workspace_control" => M5CapabilityScope::ManagedWorkspaceControl,
        other => panic!(
            "M5 audit command {} has unsupported capability scope {other}",
            entry.descriptor.command_id
        ),
    }
}

fn mutability_class(capability_scope: M5CapabilityScope) -> M5MutabilityClass {
    match capability_scope {
        M5CapabilityScope::InertMetadataOnly | M5CapabilityScope::ReversibleLocalRead => {
            M5MutabilityClass::ReadOnly
        }
        M5CapabilityScope::ReversibleLocalMutation => M5MutabilityClass::SessionMutation,
        M5CapabilityScope::RecoverableDurableMutation => M5MutabilityClass::DurableMutation,
        M5CapabilityScope::DestructiveBulkMutation => M5MutabilityClass::DestructiveMutation,
        M5CapabilityScope::IrreversiblePublish => M5MutabilityClass::ExternalPublish,
        M5CapabilityScope::CredentialOrSecretBearing => M5MutabilityClass::SensitiveMutation,
        M5CapabilityScope::ManagedWorkspaceControl => M5MutabilityClass::ManagedControl,
    }
}

fn disabled_reason_mode(entry: &CommandRegistryEntryRecord) -> M5DisabledReasonMode {
    if entry.disabled_reason_records.is_empty() {
        M5DisabledReasonMode::AlwaysInvokable
    } else {
        M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable
    }
}

fn automation_labels(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    if entry.descriptor.automation_labels.is_empty() {
        entry.automation_labels.clone()
    } else {
        entry.descriptor.automation_labels.clone()
    }
}

fn build_descriptor_from_registry(seed: &CommandSeed) -> M5CommandDescriptor {
    let entry = command_entry(seed.command_id);
    let capability_scope_class = capability_scope(entry);
    let origin = entry.descriptor.origin.as_ref();
    let canonical_aliases = entry
        .descriptor
        .aliases
        .iter()
        .map(|alias| alias.alias_id.clone())
        .collect();

    M5CommandDescriptor {
        command_id: entry.descriptor.command_id.clone(),
        feature_family: seed.feature_family,
        descriptor_revision_ref: entry.descriptor.command_revision_ref.clone(),
        primary_label_ref: entry.descriptor.primary_label_ref.clone(),
        discoverability_summary: entry.summary.clone(),
        help_anchor_ref: entry.descriptor.docs_help_anchor_ref.anchor_id.clone(),
        search_keywords: discoverability_string_array(entry, "search_keywords"),
        category_refs: if entry.descriptor.category_refs.is_empty() {
            discoverability_string_array(entry, "category_refs")
        } else {
            entry.descriptor.category_refs.clone()
        },
        lifecycle_label: lifecycle_label(entry),
        preview_class: preview_class(entry),
        approval_posture_class: entry.descriptor.approval_posture_class.clone(),
        capability_scope_class,
        mutability_class: mutability_class(capability_scope_class),
        disabled_reason_mode: disabled_reason_mode(entry),
        automation_suitability: seed.automation_suitability,
        automation_labels: automation_labels(entry),
        canonical_aliases,
        origin_class: origin
            .map(|origin| origin.origin_class.clone())
            .unwrap_or_else(|| entry.namespace_class.clone()),
        source_ref: origin.and_then(|origin| origin.source_ref.clone()),
        publisher_ref: origin.and_then(|origin| origin.publisher_ref.clone()),
        invocation_schema_ref: entry
            .descriptor
            .invocation_schema_ref
            .clone()
            .unwrap_or_default(),
        result_schema_ref: entry
            .descriptor
            .result_schema_ref
            .clone()
            .unwrap_or_default(),
        promoted_to_stable_graph: true,
    }
}

fn build_row_from_seed(seed: &CommandSeed) -> M5CommandParityRow {
    let descriptor = build_descriptor_from_registry(seed);
    let channels: Vec<M5ChannelProjection> = seed
        .channels
        .iter()
        .map(|channel_seed| build_projection_from_seed(&descriptor, channel_seed))
        .collect();
    build_m5_command_parity_row(descriptor, channels)
}

/// Seeded audit builder used by the headless inspector and the
/// integration test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5/command-parity/`.
pub fn seeded_m5_command_parity_audit() -> M5CommandParityAuditReport {
    let rows = COMMAND_SEEDS.iter().map(build_row_from_seed).collect();
    build_m5_command_parity_audit(rows)
}
