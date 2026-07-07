//! Frozen M5 prompt-composer-header, context-attachment-pill, mention-resolver,
//! slash-command-row, budget/size-strip, tainted-context-warning, draft-state-row,
//! attachment-stale-banner, and split-send / review-control component matrix.
//!
//! This module locks Aureline's reusable pre-send prompt-composition components
//! into one export-safe packet. Every composer subcomponent M5 claims that still
//! drifts too easily by inline composer, composer panel, patch-review sheet,
//! branch-agent console, help composer, or companion surface — the composer header,
//! the context-attachment pill, the mention resolver, the slash-command row, the
//! budget / size strip, the tainted-context warning, the draft-state row, the
//! attachment-stale banner, and the split-send / review-before-send control — is
//! named once here and constrained by the same prompt-mode, scope, route/provider/
//! model, attachment-identity, freshness/trust/taint, omitted-context, draft-
//! locality, and review-before-send rules regardless of the surface family that
//! renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the composer modes, scopes, and route
//! classes, the attachment kinds and trust states, the mention-resolution states,
//! the slash-command states, the budget postures and omitted-context reasons, the
//! taint sources and severities, the draft localities, the attachment staleness
//! reasons, the send postures and review requirements, the deployment lines every
//! component must survive, the non-visual accessibility routes, and the mandatory
//! labels every component must be able to show. It does not re-architect model
//! routing, evidence storage, or branch-agent lifecycle that already own those
//! records — it is the shared pre-send composition contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 composer,
//! patch-review, branch-agent, help, or companion surface may publish a mode,
//! scope, route, attachment, trust/taint, omitted-context, draft-locality, or
//! send-review claim. Inline, panel, patch-review, branch-agent, help, and
//! companion consumers all read this packet so one composer header names the mode,
//! scope, and route it is composing under, one attachment pill names the object it
//! attached and its trust state, one mention resolver names whether a mention
//! resolved, one slash-command row names its availability and approval gate, one
//! budget strip names what was omitted or truncated, one tainted-context warning
//! names the taint source and severity, one draft-state row names whether the draft
//! is local-only, one attachment-stale banner names why an attachment is stale, and
//! one send-review control names whether the request needs review before it leaves
//! the shell. No M5 lane invents a second composer grammar, masks a mode or route,
//! hides a taint or trust state, or bypasses the send-review gate.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5PromptComposerComponentVocabularySet`] rather than minted per surface. Raw
//! prompts, pasted bodies, attachment contents, raw URLs, credentials, and private
//! endpoints stay outside the support boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_prompt_composer_component_matrix,
    seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed,
    seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed,
    M5_PROMPT_COMPOSER_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PromptComposerComponentMatrixPacket`].
pub const M5_PROMPT_COMPOSER_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_size_strip_tainted_context_warning_draft_state_row_attachment_stale_banner_and_send_review_control_component_matrix";

/// Schema version for M5 prompt-composer-component-matrix records.
pub const M5_PROMPT_COMPOSER_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the prompt-composer-components boundary schema.
pub const M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF: &str =
    "schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROMPT_COMPOSER_COMPONENT_DOC_REF: &str =
    "docs/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md";

/// Repo-relative path of the prompt-composer-draft contract this matrix binds
/// against.
pub const M5_PROMPT_COMPOSER_COMPONENT_DRAFT_REF: &str =
    "schemas/ai/prompt_composer_draft.schema.json";

/// Repo-relative path of the prompt-context-attachment contract this matrix binds
/// against.
pub const M5_PROMPT_COMPOSER_COMPONENT_ATTACHMENT_REF: &str =
    "schemas/ai/prompt_context_attachment.schema.json";

/// Repo-relative path of the tainted-context contract this matrix binds against.
pub const M5_PROMPT_COMPOSER_COMPONENT_TAINT_REF: &str = "schemas/ai/tainted_context.schema.json";

/// Repo-relative path of the context-assembly contract this matrix binds against.
pub const M5_PROMPT_COMPOSER_COMPONENT_ASSEMBLY_REF: &str =
    "schemas/ai/context_assembly.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROMPT_COMPOSER_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROMPT_COMPOSER_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROMPT_COMPOSER_COMPONENT_CSV_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PROMPT_COMPOSER_COMPONENT_REPORT_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md";

/// One of the nine governed prompt-composer component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromptComposerComponentFamily {
    /// A prompt-composer header carrying the mode, scope, and route/provider/model.
    PromptComposerHeader,
    /// A context-attachment pill carrying an attached object identity and its trust
    /// state.
    ContextAttachmentPill,
    /// A mention resolver carrying whether an `@`-mention resolved to an object.
    MentionResolver,
    /// A slash-command row carrying its availability and approval gate.
    SlashCommandRow,
    /// A budget / size strip carrying the budget posture and omitted-context reason.
    BudgetSizeStrip,
    /// A tainted-context warning carrying the taint source and severity.
    TaintedContextWarning,
    /// A draft-state row carrying the draft locality and retention posture.
    DraftStateRow,
    /// An attachment-stale banner carrying why an attachment is stale.
    AttachmentStaleBanner,
    /// A split-send / review-before-send control carrying the send posture and
    /// review requirement.
    SendReviewControl,
}

impl M5PromptComposerComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PromptComposerHeader,
        Self::ContextAttachmentPill,
        Self::MentionResolver,
        Self::SlashCommandRow,
        Self::BudgetSizeStrip,
        Self::TaintedContextWarning,
        Self::DraftStateRow,
        Self::AttachmentStaleBanner,
        Self::SendReviewControl,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptComposerHeader => "prompt_composer_header",
            Self::ContextAttachmentPill => "context_attachment_pill",
            Self::MentionResolver => "mention_resolver",
            Self::SlashCommandRow => "slash_command_row",
            Self::BudgetSizeStrip => "budget_size_strip",
            Self::TaintedContextWarning => "tainted_context_warning",
            Self::DraftStateRow => "draft_state_row",
            Self::AttachmentStaleBanner => "attachment_stale_banner",
            Self::SendReviewControl => "send_review_control",
        }
    }

    /// `true` when this family is a composer header and must therefore declare its
    /// composer modes, scopes, and route classes.
    pub const fn is_composer_header(self) -> bool {
        matches!(self, Self::PromptComposerHeader)
    }

    /// `true` when this family is a context-attachment pill and must therefore
    /// declare its attachment kinds and trust states.
    pub const fn is_attachment_pill(self) -> bool {
        matches!(self, Self::ContextAttachmentPill)
    }

    /// `true` when this family is a mention resolver and must therefore declare its
    /// mention-resolution states.
    pub const fn is_mention_resolver(self) -> bool {
        matches!(self, Self::MentionResolver)
    }

    /// `true` when this family is a slash-command row and must therefore declare its
    /// slash-command states.
    pub const fn is_slash_command_row(self) -> bool {
        matches!(self, Self::SlashCommandRow)
    }

    /// `true` when this family is a budget / size strip and must therefore declare
    /// its budget postures and omitted-context reasons.
    pub const fn is_budget_strip(self) -> bool {
        matches!(self, Self::BudgetSizeStrip)
    }

    /// `true` when this family is a tainted-context warning and must therefore
    /// declare its taint sources and severities.
    pub const fn is_tainted_warning(self) -> bool {
        matches!(self, Self::TaintedContextWarning)
    }

    /// `true` when this family is a draft-state row and must therefore declare its
    /// draft localities.
    pub const fn is_draft_state_row(self) -> bool {
        matches!(self, Self::DraftStateRow)
    }

    /// `true` when this family is an attachment-stale banner and must therefore
    /// declare its staleness reasons.
    pub const fn is_attachment_stale_banner(self) -> bool {
        matches!(self, Self::AttachmentStaleBanner)
    }

    /// `true` when this family is a send / review control and must therefore declare
    /// its send postures and review requirements.
    pub const fn is_send_review_control(self) -> bool {
        matches!(self, Self::SendReviewControl)
    }
}

/// Controlled composer mode — the intent a composer header is composing under, so a
/// header never leaves the prompt mode implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerMode {
    /// Chat / ask mode.
    ChatAsk,
    /// Inline edit mode.
    InlineEdit,
    /// Guided-patch mode.
    GuidedPatch,
    /// Background branch / worktree agent mode.
    BackgroundAgent,
    /// Review-first placement mode.
    ReviewFirst,
    /// Headless / automation mode.
    HeadlessAutomation,
}

impl M5ComposerMode {
    /// Every composer mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChatAsk,
        Self::InlineEdit,
        Self::GuidedPatch,
        Self::BackgroundAgent,
        Self::ReviewFirst,
        Self::HeadlessAutomation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChatAsk => "chat_ask",
            Self::InlineEdit => "inline_edit",
            Self::GuidedPatch => "guided_patch",
            Self::BackgroundAgent => "background_agent",
            Self::ReviewFirst => "review_first",
            Self::HeadlessAutomation => "headless_automation",
        }
    }
}

/// Controlled composer scope — how wide a composer header's request reaches, so a
/// header never leaves the blast radius of the request ambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerScope {
    /// The current selection only.
    Selection,
    /// The active file.
    ActiveFile,
    /// The set of open files.
    OpenFiles,
    /// The workspace.
    Workspace,
    /// The repository.
    Repository,
    /// A managed-org scope.
    ManagedOrg,
}

impl M5ComposerScope {
    /// Every composer scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Selection,
        Self::ActiveFile,
        Self::OpenFiles,
        Self::Workspace,
        Self::Repository,
        Self::ManagedOrg,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selection => "selection",
            Self::ActiveFile => "active_file",
            Self::OpenFiles => "open_files",
            Self::Workspace => "workspace",
            Self::Repository => "repository",
            Self::ManagedOrg => "managed_org",
        }
    }
}

/// Controlled route class — where a composer header's request will actually route,
/// so a header never masks whether the request stays local, is byok, or crosses a
/// managed boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerRouteClass {
    /// A local model.
    LocalModel,
    /// A bring-your-own-key direct route.
    ByokDirect,
    /// A managed route.
    ManagedRoute,
    /// A self-hosted route.
    SelfHostedRoute,
    /// A mirrored / offline-safe route.
    MirroredRoute,
    /// A policy-pinned route.
    PolicyPinnedRoute,
}

impl M5ComposerRouteClass {
    /// Every route class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalModel,
        Self::ByokDirect,
        Self::ManagedRoute,
        Self::SelfHostedRoute,
        Self::MirroredRoute,
        Self::PolicyPinnedRoute,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalModel => "local_model",
            Self::ByokDirect => "byok_direct",
            Self::ManagedRoute => "managed_route",
            Self::SelfHostedRoute => "self_hosted_route",
            Self::MirroredRoute => "mirrored_route",
            Self::PolicyPinnedRoute => "policy_pinned_route",
        }
    }
}

/// Controlled attachment kind — what object a context-attachment pill represents, so
/// a pill never leaves the attached object's identity implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttachmentKind {
    /// A file.
    File,
    /// A symbol.
    Symbol,
    /// A selection range.
    SelectionRange,
    /// An evidence packet.
    EvidencePacket,
    /// Externally pasted text.
    ExternalPaste,
    /// A URL reference.
    UrlReference,
}

impl M5AttachmentKind {
    /// Every attachment kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::File,
        Self::Symbol,
        Self::SelectionRange,
        Self::EvidencePacket,
        Self::ExternalPaste,
        Self::UrlReference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::SelectionRange => "selection_range",
            Self::EvidencePacket => "evidence_packet",
            Self::ExternalPaste => "external_paste",
            Self::UrlReference => "url_reference",
        }
    }
}

/// Controlled attachment trust state — how much an attached object can be trusted, so
/// a pill never shows a stale, unverified, or tainted attachment as trusted-fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttachmentTrustState {
    /// Trusted and fresh.
    TrustedFresh,
    /// Trusted but stale.
    TrustedStale,
    /// From an unverified source.
    UnverifiedSource,
    /// Tainted external content.
    TaintedExternal,
    /// Redacted to a narrower scope.
    RedactedScope,
    /// Out of the current scope.
    OutOfScope,
}

impl M5AttachmentTrustState {
    /// Every attachment trust state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrustedFresh,
        Self::TrustedStale,
        Self::UnverifiedSource,
        Self::TaintedExternal,
        Self::RedactedScope,
        Self::OutOfScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedFresh => "trusted_fresh",
            Self::TrustedStale => "trusted_stale",
            Self::UnverifiedSource => "unverified_source",
            Self::TaintedExternal => "tainted_external",
            Self::RedactedScope => "redacted_scope",
            Self::OutOfScope => "out_of_scope",
        }
    }
}

/// Controlled mention-resolution state — whether an `@`-mention resolved to a
/// governed object, so a mention resolver never sends an unresolved or ambiguous
/// mention as if it bound cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MentionResolution {
    /// Resolved to a unique object.
    ResolvedUnique,
    /// Resolved to a pinned object.
    ResolvedPinned,
    /// Ambiguous across several candidates.
    AmbiguousCandidates,
    /// Unresolved / missing target.
    UnresolvedMissing,
    /// Out of scope and denied.
    OutOfScopeDenied,
    /// Deferred pending resolution.
    DeferredPending,
}

impl M5MentionResolution {
    /// Every mention-resolution state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResolvedUnique,
        Self::ResolvedPinned,
        Self::AmbiguousCandidates,
        Self::UnresolvedMissing,
        Self::OutOfScopeDenied,
        Self::DeferredPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedUnique => "resolved_unique",
            Self::ResolvedPinned => "resolved_pinned",
            Self::AmbiguousCandidates => "ambiguous_candidates",
            Self::UnresolvedMissing => "unresolved_missing",
            Self::OutOfScopeDenied => "out_of_scope_denied",
            Self::DeferredPending => "deferred_pending",
        }
    }
}

/// Controlled slash-command state — the availability posture of a slash-command row,
/// so a row never presents a disabled, deprecated, or approval-gated command as a
/// plain ready action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SlashCommandState {
    /// Available.
    Available,
    /// Disabled by an unmet precondition.
    DisabledUnmetPrecondition,
    /// Requires approval.
    RequiresApproval,
    /// Deprecated / aliased.
    DeprecatedAliased,
    /// Hidden by policy.
    PolicyHidden,
    /// Unknown command.
    UnknownCommand,
}

impl M5SlashCommandState {
    /// Every slash-command state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Available,
        Self::DisabledUnmetPrecondition,
        Self::RequiresApproval,
        Self::DeprecatedAliased,
        Self::PolicyHidden,
        Self::UnknownCommand,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::DisabledUnmetPrecondition => "disabled_unmet_precondition",
            Self::RequiresApproval => "requires_approval",
            Self::DeprecatedAliased => "deprecated_aliased",
            Self::PolicyHidden => "policy_hidden",
            Self::UnknownCommand => "unknown_command",
        }
    }
}

/// Controlled budget posture — the spend / size posture of a budget-or-size strip, so
/// a strip never shows an over-budget or hard-blocked request as within budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetPosture {
    /// Within budget.
    WithinBudget,
    /// Near the limit.
    NearLimit,
    /// Over budget.
    OverBudget,
    /// Truncation is pending to fit.
    TruncationPending,
    /// Hard-blocked by the ceiling.
    HardBlocked,
    /// Unmetered local execution.
    UnmeteredLocal,
}

impl M5BudgetPosture {
    /// Every budget posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WithinBudget,
        Self::NearLimit,
        Self::OverBudget,
        Self::TruncationPending,
        Self::HardBlocked,
        Self::UnmeteredLocal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::NearLimit => "near_limit",
            Self::OverBudget => "over_budget",
            Self::TruncationPending => "truncation_pending",
            Self::HardBlocked => "hard_blocked",
            Self::UnmeteredLocal => "unmetered_local",
        }
    }
}

/// Controlled omitted-context reason — why part of the context was left out or
/// truncated, so a budget strip never silently drops context without naming why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OmittedContextReason {
    /// Nothing omitted.
    NoneOmitted,
    /// Truncated for size.
    SizeTruncated,
    /// Capped by budget.
    BudgetCapped,
    /// Excluded by policy.
    PolicyExcluded,
    /// Collapsed as a duplicate.
    DedupCollapsed,
    /// Dropped as stale.
    StaleDropped,
}

impl M5OmittedContextReason {
    /// Every omitted-context reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoneOmitted,
        Self::SizeTruncated,
        Self::BudgetCapped,
        Self::PolicyExcluded,
        Self::DedupCollapsed,
        Self::StaleDropped,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneOmitted => "none_omitted",
            Self::SizeTruncated => "size_truncated",
            Self::BudgetCapped => "budget_capped",
            Self::PolicyExcluded => "policy_excluded",
            Self::DedupCollapsed => "dedup_collapsed",
            Self::StaleDropped => "stale_dropped",
        }
    }
}

/// Controlled taint source — where tainted context originated, so a tainted-context
/// warning never leaves the origin of untrusted text implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TaintSource {
    /// Pasted external text.
    PastedExternalText,
    /// Tool output.
    ToolOutput,
    /// Fetched URL content.
    FetchedUrlContent,
    /// An untrusted file.
    UntrustedFile,
    /// A third-party connector.
    ThirdPartyConnector,
    /// Prior model output.
    PriorModelOutput,
}

impl M5TaintSource {
    /// Every taint source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PastedExternalText,
        Self::ToolOutput,
        Self::FetchedUrlContent,
        Self::UntrustedFile,
        Self::ThirdPartyConnector,
        Self::PriorModelOutput,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PastedExternalText => "pasted_external_text",
            Self::ToolOutput => "tool_output",
            Self::FetchedUrlContent => "fetched_url_content",
            Self::UntrustedFile => "untrusted_file",
            Self::ThirdPartyConnector => "third_party_connector",
            Self::PriorModelOutput => "prior_model_output",
        }
    }
}

/// Controlled taint severity — how dangerous the tainted context is, so a
/// tainted-context warning never downplays an injection-suspected or
/// quarantine-required taint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TaintSeverity {
    /// No taint.
    None,
    /// Informational.
    Informational,
    /// Elevated.
    Elevated,
    /// Quarantine required.
    QuarantineRequired,
    /// Injection suspected.
    InjectionSuspected,
}

impl M5TaintSeverity {
    /// Every taint severity, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::Informational,
        Self::Elevated,
        Self::QuarantineRequired,
        Self::InjectionSuspected,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Informational => "informational",
            Self::Elevated => "elevated",
            Self::QuarantineRequired => "quarantine_required",
            Self::InjectionSuspected => "injection_suspected",
        }
    }
}

/// Controlled draft locality — where a composer draft actually lives, so a draft-state
/// row never shows a local-only draft as synced or a retained draft as purged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftLocality {
    /// Local-only.
    LocalOnly,
    /// Workspace-synced.
    WorkspaceSynced,
    /// Account-synced.
    AccountSynced,
    /// Shared to a thread.
    SharedThread,
    /// Ephemeral / unsaved.
    EphemeralUnsaved,
    /// Retained pending purge.
    RetentionPendingPurge,
}

impl M5DraftLocality {
    /// Every draft locality, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnly,
        Self::WorkspaceSynced,
        Self::AccountSynced,
        Self::SharedThread,
        Self::EphemeralUnsaved,
        Self::RetentionPendingPurge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::WorkspaceSynced => "workspace_synced",
            Self::AccountSynced => "account_synced",
            Self::SharedThread => "shared_thread",
            Self::EphemeralUnsaved => "ephemeral_unsaved",
            Self::RetentionPendingPurge => "retention_pending_purge",
        }
    }
}

/// Controlled staleness reason — why an attachment is stale, so an attachment-stale
/// banner never leaves a moved, deleted, or revoked attachment silently attached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StalenessReason {
    /// The source was edited.
    SourceEdited,
    /// The source was moved.
    SourceMoved,
    /// The source was deleted.
    SourceDeleted,
    /// A newer revision superseded it.
    RevisionSuperseded,
    /// Permission was revoked.
    PermissionRevoked,
    /// The index was rebuilt.
    IndexReindexed,
}

impl M5StalenessReason {
    /// Every staleness reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceEdited,
        Self::SourceMoved,
        Self::SourceDeleted,
        Self::RevisionSuperseded,
        Self::PermissionRevoked,
        Self::IndexReindexed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceEdited => "source_edited",
            Self::SourceMoved => "source_moved",
            Self::SourceDeleted => "source_deleted",
            Self::RevisionSuperseded => "revision_superseded",
            Self::PermissionRevoked => "permission_revoked",
            Self::IndexReindexed => "index_reindexed",
        }
    }
}

/// Controlled send posture — whether a request can leave the shell, so a
/// send-review control never lets an over-budget, tainted, or policy-blocked request
/// send as a plain ready action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SendPosture {
    /// Ready to send.
    ReadyToSend,
    /// Split-send after review.
    SplitSendReview,
    /// Review required before send.
    ReviewBeforeSend,
    /// Blocked by policy.
    PolicyBlocked,
    /// Blocked because over budget.
    OverBudgetBlocked,
    /// Blocked because context is tainted.
    TaintBlocked,
}

impl M5SendPosture {
    /// Every send posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadyToSend,
        Self::SplitSendReview,
        Self::ReviewBeforeSend,
        Self::PolicyBlocked,
        Self::OverBudgetBlocked,
        Self::TaintBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToSend => "ready_to_send",
            Self::SplitSendReview => "split_send_review",
            Self::ReviewBeforeSend => "review_before_send",
            Self::PolicyBlocked => "policy_blocked",
            Self::OverBudgetBlocked => "over_budget_blocked",
            Self::TaintBlocked => "taint_blocked",
        }
    }
}

/// Controlled review requirement — the acknowledgement a send-review control demands
/// before send, so a control never sends past an unresolved review requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewRequirement {
    /// No review required.
    None,
    /// Attachment review required.
    AttachmentReview,
    /// Taint acknowledgement required.
    TaintAck,
    /// Budget acknowledgement required.
    BudgetAck,
    /// Route-change acknowledgement required.
    RouteChangeAck,
}

impl M5ReviewRequirement {
    /// Every review requirement, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::AttachmentReview,
        Self::TaintAck,
        Self::BudgetAck,
        Self::RouteChangeAck,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AttachmentReview => "attachment_review",
            Self::TaintAck => "taint_ack",
            Self::BudgetAck => "budget_ack",
            Self::RouteChangeAck => "route_change_ack",
        }
    }
}

/// Claimed M5 composer surface family that renders / consumes a prompt-composer
/// component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerSurfaceFamily {
    /// The inline composer surface.
    InlineComposer,
    /// The composer-panel surface.
    ComposerPanel,
    /// The patch-review surface.
    PatchReview,
    /// The branch-agent console surface.
    BranchAgentConsole,
    /// The help-composer surface.
    HelpComposer,
    /// The companion-composer surface.
    CompanionComposer,
    /// The support-desk surface.
    SupportDesk,
}

impl M5ComposerSurfaceFamily {
    /// Every composer surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InlineComposer,
        Self::ComposerPanel,
        Self::PatchReview,
        Self::BranchAgentConsole,
        Self::HelpComposer,
        Self::CompanionComposer,
        Self::SupportDesk,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineComposer => "inline_composer",
            Self::ComposerPanel => "composer_panel",
            Self::PatchReview => "patch_review",
            Self::BranchAgentConsole => "branch_agent_console",
            Self::HelpComposer => "help_composer",
            Self::CompanionComposer => "companion_composer",
            Self::SupportDesk => "support_desk",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// mode, route, or trust state never silently narrows or widens between deployment
/// shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5ComposerDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Composer subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerConsumerSurface {
    /// The inline-composer UI.
    InlineComposerUi,
    /// The composer-panel UI.
    ComposerPanelUi,
    /// The patch-review UI.
    PatchReviewUi,
    /// The branch-agent console UI.
    BranchAgentConsoleUi,
    /// The help-composer UI.
    HelpComposerUi,
    /// The companion-composer UI.
    CompanionComposerUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5ComposerConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::InlineComposerUi,
        Self::ComposerPanelUi,
        Self::PatchReviewUi,
        Self::BranchAgentConsoleUi,
        Self::HelpComposerUi,
        Self::CompanionComposerUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineComposerUi => "inline_composer_ui",
            Self::ComposerPanelUi => "composer_panel_ui",
            Self::PatchReviewUi => "patch_review_ui",
            Self::BranchAgentConsoleUi => "branch_agent_console_ui",
            Self::HelpComposerUi => "help_composer_ui",
            Self::CompanionComposerUi => "companion_composer_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no composer truth
/// is hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5ComposerAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed prompt-composer component must be able to show. The
/// first three are hard requirements on every component; the remaining three close
/// the acceptance-criteria ambiguity about composer mode, route, and trust/taint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerRequiredLabel {
    /// The component's stable identity / what composer object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The composer mode behind the component.
    ComposerMode,
    /// The route / provider / model behind the component.
    RouteProviderModel,
    /// The trust / taint state behind the component's context.
    TrustOrTaint,
}

impl M5ComposerRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ComposerMode,
        Self::RouteProviderModel,
        Self::TrustOrTaint,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ComposerMode => "composer_mode",
            Self::RouteProviderModel => "route_provider_model",
            Self::TrustOrTaint => "trust_or_taint",
        }
    }
}

/// Qualification class for an M5 prompt-composer-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5ComposerQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a prompt-composer component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerDowngradeTrigger {
    /// A component left its composer mode unstated.
    ComposerModeUnstated,
    /// A component masked its route or provider.
    RouteOrProviderMasked,
    /// An attachment pill left its object identity unstated.
    AttachmentIdentityUnstated,
    /// An attachment pill masked its freshness.
    AttachmentFreshnessMasked,
    /// A component hid a taint state.
    TaintStateHidden,
    /// A budget strip left omitted context undisclosed.
    OmittedContextUndisclosed,
    /// A mention resolver left a mention unresolved without saying so.
    MentionLeftUnresolved,
    /// A budget strip hid a budget overrun.
    BudgetOverrunHidden,
    /// A draft-state row masked the draft locality.
    DraftLocalityMasked,
    /// A send-review control bypassed the send-review gate.
    SendReviewGateBypassed,
    /// An attachment-stale banner left staleness undisclosed.
    AttachmentStalenessUndisclosed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ComposerDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ComposerModeUnstated,
        Self::RouteOrProviderMasked,
        Self::AttachmentIdentityUnstated,
        Self::AttachmentFreshnessMasked,
        Self::TaintStateHidden,
        Self::OmittedContextUndisclosed,
        Self::MentionLeftUnresolved,
        Self::BudgetOverrunHidden,
        Self::DraftLocalityMasked,
        Self::SendReviewGateBypassed,
        Self::AttachmentStalenessUndisclosed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerModeUnstated => "composer_mode_unstated",
            Self::RouteOrProviderMasked => "route_or_provider_masked",
            Self::AttachmentIdentityUnstated => "attachment_identity_unstated",
            Self::AttachmentFreshnessMasked => "attachment_freshness_masked",
            Self::TaintStateHidden => "taint_state_hidden",
            Self::OmittedContextUndisclosed => "omitted_context_undisclosed",
            Self::MentionLeftUnresolved => "mention_left_unresolved",
            Self::BudgetOverrunHidden => "budget_overrun_hidden",
            Self::DraftLocalityMasked => "draft_locality_masked",
            Self::SendReviewGateBypassed => "send_review_gate_bypassed",
            Self::AttachmentStalenessUndisclosed => "attachment_staleness_undisclosed",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed prompt-composer component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentRow {
    /// Governed component family.
    pub component_family: M5PromptComposerComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5ComposerQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 composer surface families that render / consume this component.
    pub surface_families: Vec<M5ComposerSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5ComposerDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5ComposerRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ComposerRequiredLabel>,
    /// Composer modes this component names (composer-header only).
    pub composer_modes: Vec<M5ComposerMode>,
    /// Composer scopes this component names (composer-header only).
    pub composer_scopes: Vec<M5ComposerScope>,
    /// Route classes this component names (composer-header only).
    pub route_classes: Vec<M5ComposerRouteClass>,
    /// Attachment kinds this component distinguishes (attachment-pill only).
    pub attachment_kinds: Vec<M5AttachmentKind>,
    /// Attachment trust states this component distinguishes (attachment-pill only).
    pub attachment_trust_states: Vec<M5AttachmentTrustState>,
    /// Mention-resolution states this component distinguishes (mention-resolver
    /// only).
    pub mention_resolutions: Vec<M5MentionResolution>,
    /// Slash-command states this component distinguishes (slash-command-row only).
    pub slash_command_states: Vec<M5SlashCommandState>,
    /// Budget postures this component distinguishes (budget-strip only).
    pub budget_postures: Vec<M5BudgetPosture>,
    /// Omitted-context reasons this component discloses (budget-strip only).
    pub omitted_context_reasons: Vec<M5OmittedContextReason>,
    /// Taint sources this component names (tainted-warning only).
    pub taint_sources: Vec<M5TaintSource>,
    /// Taint severities this component distinguishes (tainted-warning only).
    pub taint_severities: Vec<M5TaintSeverity>,
    /// Draft localities this component distinguishes (draft-state-row only).
    pub draft_localities: Vec<M5DraftLocality>,
    /// Staleness reasons this component discloses (attachment-stale-banner only).
    pub staleness_reasons: Vec<M5StalenessReason>,
    /// Send postures this component distinguishes (send-review-control only).
    pub send_postures: Vec<M5SendPosture>,
    /// Review requirements this component demands (send-review-control only).
    pub review_requirements: Vec<M5ReviewRequirement>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5ComposerAccessibilityRoute>,
    /// Composer subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ComposerConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ComposerDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its composer mode or route /
    /// provider. MUST be `false`.
    pub masks_mode_or_route: bool,
    /// Hard invariant: this component never hides a taint or trust state. MUST be
    /// `false`.
    pub hides_taint_or_trust_state: bool,
    /// Hard invariant: this component never invents a private composer grammar. MUST
    /// be `false`.
    pub invents_private_composer_grammar: bool,
    /// Hard invariant: this component never bypasses the send-review gate. MUST be
    /// `false`.
    pub bypasses_send_review_gate: bool,
}

impl M5PromptComposerComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ComposerRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ComposerRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_mode_or_route
            && !self.hides_taint_or_trust_state
            && !self.invents_private_composer_grammar
            && !self.bypasses_send_review_gate
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Composer-mode tokens.
    pub composer_modes: Vec<String>,
    /// Composer-scope tokens.
    pub composer_scopes: Vec<String>,
    /// Route-class tokens.
    pub route_classes: Vec<String>,
    /// Attachment-kind tokens.
    pub attachment_kinds: Vec<String>,
    /// Attachment-trust-state tokens.
    pub attachment_trust_states: Vec<String>,
    /// Mention-resolution tokens.
    pub mention_resolutions: Vec<String>,
    /// Slash-command-state tokens.
    pub slash_command_states: Vec<String>,
    /// Budget-posture tokens.
    pub budget_postures: Vec<String>,
    /// Omitted-context-reason tokens.
    pub omitted_context_reasons: Vec<String>,
    /// Taint-source tokens.
    pub taint_sources: Vec<String>,
    /// Taint-severity tokens.
    pub taint_severities: Vec<String>,
    /// Draft-locality tokens.
    pub draft_localities: Vec<String>,
    /// Staleness-reason tokens.
    pub staleness_reasons: Vec<String>,
    /// Send-posture tokens.
    pub send_postures: Vec<String>,
    /// Review-requirement tokens.
    pub review_requirements: Vec<String>,
    /// Composer-surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5PromptComposerComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5PromptComposerComponentFamily::ALL, |v| v.as_str()),
            composer_modes: tokens(&M5ComposerMode::ALL, |v| v.as_str()),
            composer_scopes: tokens(&M5ComposerScope::ALL, |v| v.as_str()),
            route_classes: tokens(&M5ComposerRouteClass::ALL, |v| v.as_str()),
            attachment_kinds: tokens(&M5AttachmentKind::ALL, |v| v.as_str()),
            attachment_trust_states: tokens(&M5AttachmentTrustState::ALL, |v| v.as_str()),
            mention_resolutions: tokens(&M5MentionResolution::ALL, |v| v.as_str()),
            slash_command_states: tokens(&M5SlashCommandState::ALL, |v| v.as_str()),
            budget_postures: tokens(&M5BudgetPosture::ALL, |v| v.as_str()),
            omitted_context_reasons: tokens(&M5OmittedContextReason::ALL, |v| v.as_str()),
            taint_sources: tokens(&M5TaintSource::ALL, |v| v.as_str()),
            taint_severities: tokens(&M5TaintSeverity::ALL, |v| v.as_str()),
            draft_localities: tokens(&M5DraftLocality::ALL, |v| v.as_str()),
            staleness_reasons: tokens(&M5StalenessReason::ALL, |v| v.as_str()),
            send_postures: tokens(&M5SendPosture::ALL, |v| v.as_str()),
            review_requirements: tokens(&M5ReviewRequirement::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ComposerSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ComposerDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ComposerConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ComposerAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ComposerRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentGovernanceReview {
    /// The composer header shows its mode, scope, and route.
    pub header_shows_mode_scope_and_route: bool,
    /// The attachment pill shows its object identity and trust state.
    pub attachment_pill_shows_identity_and_trust: bool,
    /// The mention resolver shows its resolution state.
    pub mention_resolver_shows_resolution_state: bool,
    /// The slash-command row shows its availability and approval gate.
    pub slash_command_row_shows_availability_and_gate: bool,
    /// The budget strip shows its budget posture and omitted context.
    pub budget_strip_shows_budget_and_omitted_context: bool,
    /// The tainted-context warning shows its source and severity.
    pub tainted_warning_shows_source_and_severity: bool,
    /// The draft-state row shows its locality and retention posture.
    pub draft_state_row_shows_locality_and_retention: bool,
    /// The attachment-stale banner shows its staleness reason.
    pub attachment_stale_banner_shows_staleness_reason: bool,
    /// The send-review control shows its send posture and review requirement.
    pub send_review_control_shows_posture_and_review: bool,
    /// A local-only draft is never shown as synced.
    pub local_only_draft_never_shown_as_synced: bool,
    /// Tainted context is never shown as trusted.
    pub tainted_context_never_shown_as_trusted: bool,
    /// No component invents a second composer grammar.
    pub no_component_invents_second_composer_grammar: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel composer vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentConsumerProjection {
    /// Inline and panel surfaces consume the shared mode / scope vocabulary.
    pub inline_and_panel_surfaces_consume_mode_vocabulary: bool,
    /// Attachment and mention surfaces consume the trust / taint vocabulary.
    pub attachment_and_mention_surfaces_consume_trust_vocabulary: bool,
    /// Budget surfaces consume the omitted-context vocabulary.
    pub budget_surfaces_consume_omitted_context_vocabulary: bool,
    /// Send surfaces consume the send / review vocabulary.
    pub send_surfaces_consume_review_gate_vocabulary: bool,
    /// Support / export reads a single canonical composer source.
    pub support_export_reads_single_source: bool,
    /// Help and companion surfaces read a single canonical composer source.
    pub help_and_companion_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the prompt-composer-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting composer audit for the lane.
    pub ai_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PromptComposerComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PromptComposerComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5PromptComposerComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PromptComposerComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PromptComposerComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PromptComposerComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PromptComposerComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PromptComposerComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 prompt-composer-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerComponentMatrixPacket {
    /// Record kind; must equal [`M5_PROMPT_COMPOSER_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_PROMPT_COMPOSER_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5PromptComposerComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PromptComposerComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PromptComposerComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PromptComposerComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PromptComposerComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PromptComposerComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PromptComposerComponentMatrixPacket {
    /// Builds an M5 prompt-composer-component matrix packet from stable-lane input.
    pub fn new(input: M5PromptComposerComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_PROMPT_COMPOSER_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_PROMPT_COMPOSER_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 prompt-composer-component matrix invariants.
    pub fn validate(&self) -> Vec<M5PromptComposerComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROMPT_COMPOSER_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5PromptComposerComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROMPT_COMPOSER_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5PromptComposerComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PromptComposerComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 prompt composer component matrix packet serializes"),
        ) {
            violations.push(M5PromptComposerComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 prompt composer component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Prompt-Composer-Header, Context-Attachment-Pill, Mention-Resolver, Slash-Command-Row, Budget-Strip, Tainted-Context-Warning, Draft-State-Row, Attachment-Stale-Banner, and Send-Review-Control Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Composer modes: {}\n",
            self.vocabulary_set.composer_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Send postures: {}\n",
            self.vocabulary_set.send_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 prompt-composer matrix export.
#[derive(Debug)]
pub enum M5PromptComposerComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PromptComposerComponentMatrixViolation>),
}

impl fmt::Display for M5PromptComposerComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 prompt composer component matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 prompt composer component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PromptComposerComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5PromptComposerComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PromptComposerComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A composer-header component declares no composer modes.
    ComposerModeMissing,
    /// A composer-header component declares no composer scopes.
    ComposerScopeMissing,
    /// A composer-header component declares no route classes.
    RouteClassMissing,
    /// An attachment-pill component declares no attachment kinds.
    AttachmentKindMissing,
    /// An attachment-pill component declares no attachment trust states.
    AttachmentTrustStateMissing,
    /// A mention-resolver component declares no mention-resolution states.
    MentionResolutionMissing,
    /// A slash-command-row component declares no slash-command states.
    SlashCommandStateMissing,
    /// A budget-strip component declares no budget postures.
    BudgetPostureMissing,
    /// A budget-strip component declares no omitted-context reasons.
    OmittedContextReasonMissing,
    /// A tainted-warning component declares no taint sources.
    TaintSourceMissing,
    /// A tainted-warning component declares no taint severities.
    TaintSeverityMissing,
    /// A draft-state-row component declares no draft localities.
    DraftLocalityMissing,
    /// An attachment-stale-banner component declares no staleness reasons.
    StalenessReasonMissing,
    /// A send-review-control component declares no send postures.
    SendPostureMissing,
    /// A send-review-control component declares no review requirements.
    ReviewRequirementMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked mode/route, hidden taint/trust,
    /// private composer grammar, or bypassed send-review gate).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PromptComposerComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComposerModeMissing => "composer_mode_missing",
            Self::ComposerScopeMissing => "composer_scope_missing",
            Self::RouteClassMissing => "route_class_missing",
            Self::AttachmentKindMissing => "attachment_kind_missing",
            Self::AttachmentTrustStateMissing => "attachment_trust_state_missing",
            Self::MentionResolutionMissing => "mention_resolution_missing",
            Self::SlashCommandStateMissing => "slash_command_state_missing",
            Self::BudgetPostureMissing => "budget_posture_missing",
            Self::OmittedContextReasonMissing => "omitted_context_reason_missing",
            Self::TaintSourceMissing => "taint_source_missing",
            Self::TaintSeverityMissing => "taint_severity_missing",
            Self::DraftLocalityMissing => "draft_locality_missing",
            Self::StalenessReasonMissing => "staleness_reason_missing",
            Self::SendPostureMissing => "send_posture_missing",
            Self::ReviewRequirementMissing => "review_requirement_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 prompt-composer matrix export.
pub fn current_stable_m5_prompt_composer_component_matrix_export(
) -> Result<M5PromptComposerComponentMatrixPacket, M5PromptComposerComponentMatrixArtifactError> {
    let packet: M5PromptComposerComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/support_export.json"
    )))
    .map_err(M5PromptComposerComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PromptComposerComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
        M5_PROMPT_COMPOSER_COMPONENT_DOC_REF,
        M5_PROMPT_COMPOSER_COMPONENT_DRAFT_REF,
        M5_PROMPT_COMPOSER_COMPONENT_ATTACHMENT_REF,
        M5_PROMPT_COMPOSER_COMPONENT_TAINT_REF,
        M5_PROMPT_COMPOSER_COMPONENT_ASSEMBLY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PromptComposerComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PromptComposerComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    let present: BTreeSet<M5PromptComposerComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5PromptComposerComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5PromptComposerComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5PromptComposerComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5PromptComposerComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_composer_header() && row.composer_modes.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::ComposerModeMissing);
        }
        if family.is_composer_header() && row.composer_scopes.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::ComposerScopeMissing);
        }
        if family.is_composer_header() && row.route_classes.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::RouteClassMissing);
        }
        if family.is_attachment_pill() && row.attachment_kinds.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::AttachmentKindMissing);
        }
        if family.is_attachment_pill() && row.attachment_trust_states.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::AttachmentTrustStateMissing);
        }
        if family.is_mention_resolver() && row.mention_resolutions.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::MentionResolutionMissing);
        }
        if family.is_slash_command_row() && row.slash_command_states.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::SlashCommandStateMissing);
        }
        if family.is_budget_strip() && row.budget_postures.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::BudgetPostureMissing);
        }
        if family.is_budget_strip() && row.omitted_context_reasons.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::OmittedContextReasonMissing);
        }
        if family.is_tainted_warning() && row.taint_sources.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::TaintSourceMissing);
        }
        if family.is_tainted_warning() && row.taint_severities.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::TaintSeverityMissing);
        }
        if family.is_draft_state_row() && row.draft_localities.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::DraftLocalityMissing);
        }
        if family.is_attachment_stale_banner() && row.staleness_reasons.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::StalenessReasonMissing);
        }
        if family.is_send_review_control() && row.send_postures.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::SendPostureMissing);
        }
        if family.is_send_review_control() && row.review_requirements.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::ReviewRequirementMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PromptComposerComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5PromptComposerComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.header_shows_mode_scope_and_route,
        review.attachment_pill_shows_identity_and_trust,
        review.mention_resolver_shows_resolution_state,
        review.slash_command_row_shows_availability_and_gate,
        review.budget_strip_shows_budget_and_omitted_context,
        review.tainted_warning_shows_source_and_severity,
        review.draft_state_row_shows_locality_and_retention,
        review.attachment_stale_banner_shows_staleness_reason,
        review.send_review_control_shows_posture_and_review,
        review.local_only_draft_never_shown_as_synced,
        review.tainted_context_never_shown_as_trusted,
        review.no_component_invents_second_composer_grammar,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5PromptComposerComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.inline_and_panel_surfaces_consume_mode_vocabulary,
        projection.attachment_and_mention_surfaces_consume_trust_vocabulary,
        projection.budget_surfaces_consume_omitted_context_vocabulary,
        projection.send_surfaces_consume_review_gate_vocabulary,
        projection.support_export_reads_single_source,
        projection.help_and_companion_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(M5PromptComposerComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PromptComposerComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PromptComposerComponentMatrixPacket,
    violations: &mut Vec<M5PromptComposerComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PromptComposerComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
