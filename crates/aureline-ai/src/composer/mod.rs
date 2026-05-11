//! AI composer seed for the bounded launch AI wedge.
//!
//! This module owns one canonical [`ComposerDraft`] record plus the typed
//! mention / attachment / slash-command / route-placeholder vocabularies the
//! M1 seed projects. It is intentionally narrow: the composer holds a draft
//! intent, the resolved mention set, the inspectable attachment set, the
//! resolved slash-command invocation set, and the route/provider placeholder.
//! It carries **no mutation authority**: the M1 seed never dispatches a turn,
//! never mints model output, and never silently widens the trust posture of
//! an attachment.
//!
//! ## Why one composer object
//!
//! The launch AI wedge, the shell AI-context inspector, and the support /
//! export flow all need the same answer when a user asks "what is in this
//! draft, where did it come from, and why is it blocked?". Forking copies of
//! the draft per surface would let one surface promote a tainted attachment
//! to a trusted role while another lags. This module renders one record the
//! chrome projects through typed accessors.
//!
//! ## Seed scope (M1)
//!
//! The seed is intentionally small. It models:
//!
//! - one composer draft with a free-form `intent_text` field,
//! - a list of typed mentions that resolve to stable workspace ids,
//! - a list of typed attachments each with a trust posture, a selection
//!   reason, an inspectable status, and a removable opaque id,
//! - a list of slash-command invocations that resolve to canonical
//!   [`aureline_commands::CommandId`] values from the frozen registry,
//! - a route/provider placeholder that explicitly publishes
//!   `disabled_no_dispatch_in_m1_seed` so the surface never implies live
//!   routing,
//! - a derived [`ComposerDraftState`] that names whether the draft is
//!   blocked pending resolution, ready for review-only inspection, or
//!   permanently dispatch-disabled in M1.
//!
//! Live provider routing, autonomous apply, model dispatch, evidence-packet
//! minting, and the full ordered-section composition described in the
//! prompt-composer contract are out of scope for this seed.
//!
//! ## Failure-drill posture
//!
//! When a caller adds a tainted attachment that is not placed under a
//! fenced-tainted-data role, the composer records the typed
//! [`BlockReason::TaintedAttachmentOutsideFencedSection`] block reason on
//! the draft so the inspector can render it verbatim. The drill also covers
//! out-of-scope attachments, stale attachments, over-budget attachments,
//! unresolved mentions, unresolved slash commands, and policy-blocked
//! routes. The fixture
//! [`/fixtures/ai/m1_composer_and_context_inspector_seed_cases/tainted_attachment_outside_fence.json`](../../../../fixtures/ai/m1_composer_and_context_inspector_seed_cases/tainted_attachment_outside_fence.json)
//! exercises the drill end to end.

use serde::{Deserialize, Serialize};

use aureline_commands::CommandRegistry;

/// Stable record-kind tag carried on serialized [`ComposerDraft`] payloads.
pub const COMPOSER_DRAFT_RECORD_KIND: &str = "ai_composer_draft_seed_record";

/// Schema version of the seed [`ComposerDraft`] record this crate emits.
///
/// Bumped on breaking payload changes; additive-optional fields do not bump
/// this version.
pub const COMPOSER_DRAFT_SCHEMA_VERSION: u32 = 1;

/// Closed seed vocabulary for the kinds of mentions the prototype composer
/// admits. Mirrored from the frozen
/// `prompt_composer_mention_record.mention_kind` vocabulary in
/// `docs/ai/context_assembly_contract.md`; the seed covers a small subset and
/// grows additively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionKind {
    SymbolMention,
    FileMention,
    WorksetMention,
    SearchResultMention,
    DocsAnchorMention,
    DiagnosticMention,
    ExecutionContextMention,
}

impl MentionKind {
    /// Stable string token recorded on the mention row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymbolMention => "symbol_mention",
            Self::FileMention => "file_mention",
            Self::WorksetMention => "workset_mention",
            Self::SearchResultMention => "search_result_mention",
            Self::DocsAnchorMention => "docs_anchor_mention",
            Self::DiagnosticMention => "diagnostic_mention",
            Self::ExecutionContextMention => "execution_context_mention",
        }
    }
}

/// How the resolver settled a mention. `Resolved` is the happy path; every
/// other variant is a typed reason the composer cannot silently ignore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionResolutionState {
    /// The mention resolves to a non-empty stable target id.
    Resolved,
    /// The mention does not resolve to any known workspace object.
    UnresolvedNotFound,
    /// The mention resolves to an object outside the request workspace's
    /// scope filter; the composer MUST NOT silently include it.
    UnresolvedScopeExcluded,
    /// The mention's underlying object existed when the user typed it but
    /// has been mutated, deleted, or invalidated since.
    UnresolvedStale,
}

impl MentionResolutionState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::UnresolvedNotFound => "unresolved_not_found",
            Self::UnresolvedScopeExcluded => "unresolved_scope_excluded",
            Self::UnresolvedStale => "unresolved_stale",
        }
    }
}

/// Closed seed vocabulary for the kinds of attachments the prototype composer
/// admits. Mirrored from the frozen
/// `prompt_composer_attachment_record.attachment_kind` vocabulary; the seed
/// covers a small subset and grows additively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentKind {
    UserSuppliedText,
    UserSuppliedFile,
    WorkspaceSliceBundle,
    RetrievedDocument,
    TerminalLogCapture,
    CitationAnchorBundle,
    DiagnosticBundle,
}

impl AttachmentKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserSuppliedText => "user_supplied_text",
            Self::UserSuppliedFile => "user_supplied_file",
            Self::WorkspaceSliceBundle => "workspace_slice_bundle",
            Self::RetrievedDocument => "retrieved_document",
            Self::TerminalLogCapture => "terminal_log_capture",
            Self::CitationAnchorBundle => "citation_anchor_bundle",
            Self::DiagnosticBundle => "diagnostic_bundle",
        }
    }
}

/// Closed seed vocabulary for the byte source of an attachment. Mirrors the
/// `source_class` vocabulary frozen in `docs/ai/context_assembly_contract.md`
/// (small subset; grows additively).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    WorkspaceFileSlice,
    WorkspaceSymbol,
    WorkspaceBufferSlice,
    WorkspaceSearchResult,
    WorkspaceDiagnostics,
    DocsPackExcerpt,
    TerminalTranscriptExcerpt,
    UserSuppliedText,
    UserSuppliedFile,
    CitationAnchorQuote,
}

impl SourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceFileSlice => "workspace_file_slice",
            Self::WorkspaceSymbol => "workspace_symbol",
            Self::WorkspaceBufferSlice => "workspace_buffer_slice",
            Self::WorkspaceSearchResult => "workspace_search_result",
            Self::WorkspaceDiagnostics => "workspace_diagnostics",
            Self::DocsPackExcerpt => "docs_pack_excerpt",
            Self::TerminalTranscriptExcerpt => "terminal_transcript_excerpt",
            Self::UserSuppliedText => "user_supplied_text",
            Self::UserSuppliedFile => "user_supplied_file",
            Self::CitationAnchorQuote => "citation_anchor_quote",
        }
    }
}

/// Closed seed vocabulary for the trust posture the composer carries on an
/// attachment. Mirrored from the frozen `trust_posture` set in
/// `docs/ai/context_assembly_contract.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPosture {
    TrustedFirstParty,
    ReviewedDerived,
    UnreviewedDerived,
    UntrustedUserSupplied,
    UntrustedExternal,
    UntrustedExtensionProposed,
    PolicyQuarantined,
}

impl TrustPosture {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedFirstParty => "trusted_first_party",
            Self::ReviewedDerived => "reviewed_derived",
            Self::UnreviewedDerived => "unreviewed_derived",
            Self::UntrustedUserSupplied => "untrusted_user_supplied",
            Self::UntrustedExternal => "untrusted_external",
            Self::UntrustedExtensionProposed => "untrusted_extension_proposed",
            Self::PolicyQuarantined => "policy_quarantined",
        }
    }

    /// True when the composer MUST place the attachment under a fenced
    /// tainted-data role; the M1 seed has no such role yet so callers must
    /// either mark the attachment as fenced or accept the typed block
    /// reason on the draft.
    pub const fn requires_fence(self) -> bool {
        matches!(
            self,
            Self::UntrustedUserSupplied
                | Self::UntrustedExternal
                | Self::UntrustedExtensionProposed
                | Self::PolicyQuarantined
        )
    }
}

/// Closed seed vocabulary describing why the composer selected this
/// attachment. Mirrors the `instructional_role` / `pin` / mention-trail
/// reasons the M1 prototype currently models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionReasonClass {
    /// The user pinned this attachment explicitly.
    UserPinned,
    /// The attachment trails a typed mention the user placed in the draft.
    MentionTrail,
    /// A slash-command recipe requested the attachment.
    SlashCommandRequested,
    /// A workspace search-result packet feeds the attachment.
    SearchResultPacket,
    /// A docs / generated-reference excerpt with a citation anchor backs it.
    CitationAnchorExcerpt,
    /// A diagnostic the user is asking about backs it.
    DiagnosticContext,
    /// A terminal/log capture the user explicitly attached.
    TerminalCaptureAttached,
    /// The user typed/pasted free-form text into the draft.
    UserPastedFreeformText,
}

impl SelectionReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserPinned => "user_pinned",
            Self::MentionTrail => "mention_trail",
            Self::SlashCommandRequested => "slash_command_requested",
            Self::SearchResultPacket => "search_result_packet",
            Self::CitationAnchorExcerpt => "citation_anchor_excerpt",
            Self::DiagnosticContext => "diagnostic_context",
            Self::TerminalCaptureAttached => "terminal_capture_attached",
            Self::UserPastedFreeformText => "user_pasted_freeform_text",
        }
    }
}

/// Closed seed vocabulary for the inspectable status of an attachment.
///
/// `Live` is the happy path. Every other variant maps to a typed reason the
/// composer cannot silently include the attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentStatusClass {
    /// Attachment is admitted under the request workspace's scope, trusted
    /// or fenced appropriately, and within budget.
    Live,
    /// The underlying file / symbol / capture has mutated or been deleted
    /// since the attachment was added.
    Stale,
    /// The attachment carries an untrusted posture but is not under a
    /// fenced-tainted-data role.
    TaintedOutsideFencedSection,
    /// The estimated byte size pushes the draft over the composer's budget
    /// ceiling.
    OverBudget,
    /// Admin policy, workspace trust, or an extension permission blocks the
    /// attachment.
    PolicyBlocked,
    /// The attachment's source is outside the workspace's scope filter.
    OutOfScope,
}

impl AttachmentStatusClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
            Self::TaintedOutsideFencedSection => "tainted_outside_fenced_section",
            Self::OverBudget => "over_budget",
            Self::PolicyBlocked => "policy_blocked",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// True when the status prevents the composer from advancing past
    /// `BlockedPendingResolution`.
    pub const fn blocks_dispatch(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// How the resolver settled a slash-command invocation against the canonical
/// command registry. Mirrors the frozen
/// `aureline_commands::PreflightDecisionClass` semantics but stays narrow
/// for the M1 prototype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlashCommandResolutionState {
    /// Resolves cleanly to an admitted [`CommandId`] in the registry.
    Resolved,
    /// No registry entry matches the requested slash-command text.
    UnresolvedNoMatch,
    /// The registry entry exists but is disabled for the current surface.
    UnresolvedDisabled,
    /// Admin policy / trust posture blocks the command.
    UnresolvedPolicyBlocked,
}

impl SlashCommandResolutionState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::UnresolvedNoMatch => "unresolved_no_match",
            Self::UnresolvedDisabled => "unresolved_disabled",
            Self::UnresolvedPolicyBlocked => "unresolved_policy_blocked",
        }
    }
}

/// Closed seed vocabulary for the provider class the composer would route
/// to. The M1 seed publishes `DisabledNoProviderInM1Seed` by default so the
/// surface never implies live dispatch; the `MockedTestProvider` variant
/// exists only for fixture / smoke replays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderClass {
    /// The M1 seed deliberately has no live provider; the composer is
    /// read-only and renders this label honestly instead of pretending to
    /// dispatch.
    DisabledNoProviderInM1Seed,
    /// Reserved for fixture / smoke replays; never reachable from the live
    /// shell.
    MockedTestProvider,
}

impl ProviderClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisabledNoProviderInM1Seed => "disabled_no_provider_in_m1_seed",
            Self::MockedTestProvider => "mocked_test_provider",
        }
    }

    /// Human-readable label rendered on the route row.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DisabledNoProviderInM1Seed => {
                "Disabled — M1 seed does not route or spend"
            }
            Self::MockedTestProvider => "Mocked test provider (fixtures only)",
        }
    }
}

/// Closed seed vocabulary for the route path. The M1 seed publishes
/// `DeniedByPolicyInM1Seed` by default; `OfflineCachedOnly` is reserved for
/// future cache-only flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutePathClass {
    DeniedByPolicyInM1Seed,
    OfflineCachedOnly,
}

impl RoutePathClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeniedByPolicyInM1Seed => "denied_by_policy_in_m1_seed",
            Self::OfflineCachedOnly => "offline_cached_only",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DeniedByPolicyInM1Seed => "Denied — model routing reserved for a later milestone",
            Self::OfflineCachedOnly => "Offline cached only",
        }
    }
}

/// Closed seed vocabulary for the dispatch target. The M1 seed publishes
/// `DisabledNoDispatchInM1Seed` so the surface never implies live model
/// invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DispatchTargetClass {
    DisabledNoDispatchInM1Seed,
}

impl DispatchTargetClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisabledNoDispatchInM1Seed => "disabled_no_dispatch_in_m1_seed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DisabledNoDispatchInM1Seed => {
                "Dispatch disabled — M1 prototype seed has no model dispatch path"
            }
        }
    }
}

/// Prototype label carried on every draft. The label is non-optional so the
/// chrome cannot accidentally render the wedge without the boundary marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// The M1 prototype seed. Read-only for mutation; no model dispatch.
    M1PrototypeReadOnlyNoMutation,
}

impl PrototypeLabel {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeReadOnlyNoMutation => "m1_prototype_read_only_no_mutation",
        }
    }

    /// Human-readable label rendered as a wedge chip on every surface that
    /// quotes this draft.
    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeReadOnlyNoMutation => {
                "M1 prototype seed — read-only, no model dispatch"
            }
        }
    }
}

/// Closed seed vocabulary for why the composer cannot advance the draft.
///
/// Every variant carries the opaque id of the offending mention, attachment,
/// or slash-command invocation so a consuming surface can route the user to
/// the exact addressable row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BlockReason {
    /// A mention does not resolve to a stable target.
    UnresolvedMention {
        mention_id: String,
        resolution_state: MentionResolutionState,
    },
    /// An attachment is no longer current (file mutated/deleted, capture
    /// invalidated, etc.).
    StaleAttachment { attachment_id: String },
    /// An attachment carries an untrusted posture but is not placed under
    /// a fenced-tainted-data role. The M1 seed has no fenced role yet, so
    /// the draft cannot silently include the attachment.
    TaintedAttachmentOutsideFencedSection {
        attachment_id: String,
        trust_posture: TrustPosture,
    },
    /// The aggregate byte estimate pushes the draft over budget.
    OverBudgetContext { attachment_id: String },
    /// Admin policy, workspace trust, or an extension permission blocks the
    /// attachment.
    PolicyBlockedAttachment { attachment_id: String },
    /// The attachment's source is outside the request workspace's scope
    /// filter.
    OutOfScopeAttachment { attachment_id: String },
    /// A slash-command invocation does not resolve to an admitted command.
    UnresolvedSlashCommand {
        invocation_id: String,
        resolution_state: SlashCommandResolutionState,
    },
    /// The route placeholder is denied by policy. In the M1 seed this is
    /// the canonical (and only) live route state; the variant exists so
    /// the inspector can label it honestly.
    PolicyBlockedRoute,
}

impl BlockReason {
    /// Stable string token for the variant tag.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnresolvedMention { .. } => "unresolved_mention",
            Self::StaleAttachment { .. } => "stale_attachment",
            Self::TaintedAttachmentOutsideFencedSection { .. } => {
                "tainted_attachment_outside_fenced_section"
            }
            Self::OverBudgetContext { .. } => "over_budget_context",
            Self::PolicyBlockedAttachment { .. } => "policy_blocked_attachment",
            Self::OutOfScopeAttachment { .. } => "out_of_scope_attachment",
            Self::UnresolvedSlashCommand { .. } => "unresolved_slash_command",
            Self::PolicyBlockedRoute => "policy_blocked_route",
        }
    }

    /// Human-readable label rendered on the block-reason row.
    pub fn label(&self) -> &'static str {
        match self {
            Self::UnresolvedMention { .. } => "Mention does not resolve to a workspace object",
            Self::StaleAttachment { .. } => "Attachment is stale",
            Self::TaintedAttachmentOutsideFencedSection { .. } => {
                "Untrusted attachment must ride a fenced-tainted-data role"
            }
            Self::OverBudgetContext { .. } => "Attachment pushes the draft over the context budget",
            Self::PolicyBlockedAttachment { .. } => "Attachment is blocked by policy or trust",
            Self::OutOfScopeAttachment { .. } => {
                "Attachment source is outside the workspace scope"
            }
            Self::UnresolvedSlashCommand { .. } => {
                "Slash command does not resolve to a registry entry"
            }
            Self::PolicyBlockedRoute => {
                "Model dispatch is disabled in the M1 prototype seed"
            }
        }
    }
}

/// Derived state of the composer draft. The M1 seed never reaches
/// `DispatchedInline` because dispatch is reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerDraftState {
    /// The user is actively editing the draft; no blocks are surfaced yet.
    Drafting,
    /// At least one block reason prevents advancing the draft.
    BlockedPendingResolution,
    /// No blocking reasons remain, but the M1 seed still cannot dispatch.
    /// The surface labels this state honestly so the user does not infer
    /// a hidden dispatch path.
    ReadyForReviewOnly,
    /// Permanent honesty-marker state: the M1 seed never dispatches.
    DispatchDisabledInM1Seed,
}

impl ComposerDraftState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Drafting => "drafting",
            Self::BlockedPendingResolution => "blocked_pending_resolution",
            Self::ReadyForReviewOnly => "ready_for_review_only",
            Self::DispatchDisabledInM1Seed => "dispatch_disabled_in_m1_seed",
        }
    }

    /// Human-readable label rendered on the draft-state row.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Drafting => "Drafting",
            Self::BlockedPendingResolution => "Blocked — pending resolution",
            Self::ReadyForReviewOnly => "Ready for review-only inspection",
            Self::DispatchDisabledInM1Seed => "Dispatch disabled in M1 prototype seed",
        }
    }
}

/// One typed mention placed in the draft. Every mention carries an opaque
/// `mention_id` so the inspector can address it individually for inspect /
/// resolve / remove actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerMention {
    pub mention_id: String,
    pub kind: MentionKind,
    /// Resolved stable id of the target workspace object. `None` when the
    /// resolver could not settle the mention.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_stable_id: Option<String>,
    pub display_label: String,
    pub resolution_state: MentionResolutionState,
}

/// One typed attachment carried on the draft. Every attachment carries an
/// opaque `attachment_id` so the inspector can address it for inspect /
/// remove actions and so the composer can attribute block reasons mechanically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerAttachment {
    pub attachment_id: String,
    pub kind: AttachmentKind,
    pub source_class: SourceClass,
    pub trust_posture: TrustPosture,
    pub selection_reason: SelectionReasonClass,
    pub status: AttachmentStatusClass,
    /// Rough byte estimate the composer uses to label budget pressure. The
    /// M1 seed does not allocate; this is presentation-only.
    pub estimated_byte_size: u64,
    pub display_label: String,
    /// True when the caller placed the attachment under a fenced
    /// tainted-data role. The M1 seed does not own the fenced role yet, so
    /// callers either set this to `true` for fixture replays or accept the
    /// typed `TaintedAttachmentOutsideFencedSection` block.
    #[serde(default)]
    pub placed_under_fenced_role: bool,
}

/// One typed slash-command invocation. The composer resolves the requested
/// text against the canonical [`CommandRegistry`]; unresolved invocations
/// stay on the draft so the inspector can render them with a typed reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerSlashCommandInvocation {
    pub invocation_id: String,
    /// Resolved [`CommandId`] from the registry. Empty when
    /// `resolution_state` is anything other than `Resolved`.
    pub command_id: String,
    pub display_label: String,
    pub resolution_state: SlashCommandResolutionState,
}

impl ComposerSlashCommandInvocation {
    /// Resolve a requested command-id token against the canonical
    /// [`CommandRegistry`]. The returned invocation carries either the
    /// resolved canonical id or a typed `UnresolvedNoMatch` state; richer
    /// disablement / policy resolution is reserved for a later milestone.
    pub fn resolve_in_registry(
        invocation_id: impl Into<String>,
        requested_command_id: impl Into<String>,
        display_label: impl Into<String>,
        registry: &CommandRegistry,
    ) -> Self {
        let invocation_id = invocation_id.into();
        let requested = requested_command_id.into();
        let display_label = display_label.into();
        if registry.get(&requested).is_some() {
            Self {
                invocation_id,
                command_id: requested,
                display_label,
                resolution_state: SlashCommandResolutionState::Resolved,
            }
        } else {
            Self {
                invocation_id,
                command_id: String::new(),
                display_label,
                resolution_state: SlashCommandResolutionState::UnresolvedNoMatch,
            }
        }
    }
}

/// Route / provider placeholder carried on every draft. The M1 seed pins
/// `disabled_no_provider_in_m1_seed` + `denied_by_policy_in_m1_seed` +
/// `disabled_no_dispatch_in_m1_seed` so the surface can label the wedge
/// honestly without implying live model routing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutePlaceholder {
    pub provider_class: ProviderClass,
    pub route_path_class: RoutePathClass,
    pub dispatch_target_class: DispatchTargetClass,
    /// Human-readable note rendered next to the route placeholder so the
    /// chrome cannot accidentally hide the honesty marker behind a tooltip.
    pub seed_note: String,
}

impl RoutePlaceholder {
    /// The canonical M1 seed placeholder. Every draft uses this unless a
    /// fixture explicitly opts into the `MockedTestProvider` variant.
    pub fn m1_seed_default() -> Self {
        Self {
            provider_class: ProviderClass::DisabledNoProviderInM1Seed,
            route_path_class: RoutePathClass::DeniedByPolicyInM1Seed,
            dispatch_target_class: DispatchTargetClass::DisabledNoDispatchInM1Seed,
            seed_note:
                "Model routing, dispatch, and spend are reserved for a later milestone. \
                 The M1 prototype seed inspects context only."
                    .to_owned(),
        }
    }

    /// Fixture-only mocked variant. Reserved for replay; not reachable from
    /// the live shell.
    pub fn mocked_for_fixtures() -> Self {
        Self {
            provider_class: ProviderClass::MockedTestProvider,
            route_path_class: RoutePathClass::OfflineCachedOnly,
            dispatch_target_class: DispatchTargetClass::DisabledNoDispatchInM1Seed,
            seed_note: "Mocked test provider; fixtures only.".to_owned(),
        }
    }
}

/// Outcome of [`ComposerDraft::validate`]. The composer never silently
/// dispatches: callers read the block reasons and derived state before
/// rendering the draft.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationOutcome {
    pub state: ComposerDraftState,
    pub block_reasons: Vec<BlockReason>,
    /// Approximate aggregate byte estimate across attachments. Presentation
    /// only.
    pub aggregate_byte_estimate: u64,
    /// Budget ceiling the composer compared against.
    pub budget_byte_ceiling: u64,
}

impl ValidationOutcome {
    /// True when at least one block reason was recorded.
    pub fn is_blocked(&self) -> bool {
        !self.block_reasons.is_empty()
    }
}

/// Canonical seed [`ComposerDraft`] record.
///
/// One inspectable object the launch AI wedge owns and the shell context
/// inspector projects. Surfaces never re-derive its fields; they project
/// the record verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerDraft {
    pub record_kind: String,
    pub schema_version: u32,
    pub composer_draft_id: String,
    pub composer_session_id: String,
    /// Opaque id of the request workspace this draft accumulates into. The
    /// M1 seed does not own request-workspace truth; this is the typed
    /// reference the broader contract carries.
    pub request_workspace_id: String,
    pub intent_text: String,
    pub mentions: Vec<ComposerMention>,
    pub attachments: Vec<ComposerAttachment>,
    pub slash_command_invocations: Vec<ComposerSlashCommandInvocation>,
    pub route_placeholder: RoutePlaceholder,
    pub prototype_label: PrototypeLabel,
    /// Budget ceiling the composer compares aggregate attachment sizes
    /// against. The M1 seed uses a small, fixed ceiling — bumping it is
    /// presentation-only.
    pub budget_byte_ceiling: u64,
}

impl ComposerDraft {
    /// Construct an empty draft with the M1 seed defaults. Callers add
    /// mentions, attachments, and slash-command invocations through the
    /// typed builders.
    pub fn new(
        composer_draft_id: impl Into<String>,
        composer_session_id: impl Into<String>,
        request_workspace_id: impl Into<String>,
        intent_text: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: COMPOSER_DRAFT_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_DRAFT_SCHEMA_VERSION,
            composer_draft_id: composer_draft_id.into(),
            composer_session_id: composer_session_id.into(),
            request_workspace_id: request_workspace_id.into(),
            intent_text: intent_text.into(),
            mentions: Vec::new(),
            attachments: Vec::new(),
            slash_command_invocations: Vec::new(),
            route_placeholder: RoutePlaceholder::m1_seed_default(),
            prototype_label: PrototypeLabel::M1PrototypeReadOnlyNoMutation,
            budget_byte_ceiling: DEFAULT_BUDGET_BYTE_CEILING,
        }
    }

    /// Default budget ceiling used by [`ComposerDraft::new`]. Presentation-
    /// only; bumping the value never widens authority.
    pub const fn default_budget_byte_ceiling() -> u64 {
        DEFAULT_BUDGET_BYTE_CEILING
    }

    /// Append a mention. The composer does not deduplicate by display label;
    /// stable ids are the dedup key for downstream surfaces.
    pub fn add_mention(&mut self, mention: ComposerMention) -> &mut Self {
        self.mentions.push(mention);
        self
    }

    /// Append an attachment.
    pub fn add_attachment(&mut self, attachment: ComposerAttachment) -> &mut Self {
        self.attachments.push(attachment);
        self
    }

    /// Append a slash-command invocation.
    pub fn add_slash_command(&mut self, invocation: ComposerSlashCommandInvocation) -> &mut Self {
        self.slash_command_invocations.push(invocation);
        self
    }

    /// Remove an attachment by its opaque id. Returns `true` when an
    /// attachment was removed. The chrome calls this from the inspector's
    /// `remove_attachment` action.
    pub fn remove_attachment(&mut self, attachment_id: &str) -> bool {
        let before = self.attachments.len();
        self.attachments
            .retain(|attachment| attachment.attachment_id != attachment_id);
        self.attachments.len() != before
    }

    /// Derive the validation outcome for this draft.
    ///
    /// The composer never silently dispatches. Callers MUST read the
    /// returned state and block reasons before rendering the draft on a
    /// live surface. The M1 seed always concludes with at least
    /// `BlockedPendingResolution` or `ReadyForReviewOnly` — neither implies
    /// model dispatch.
    pub fn validate(&self) -> ValidationOutcome {
        let mut block_reasons: Vec<BlockReason> = Vec::new();
        let mut aggregate: u64 = 0;

        for mention in &self.mentions {
            if mention.resolution_state != MentionResolutionState::Resolved {
                block_reasons.push(BlockReason::UnresolvedMention {
                    mention_id: mention.mention_id.clone(),
                    resolution_state: mention.resolution_state,
                });
            }
        }

        for attachment in &self.attachments {
            aggregate = aggregate.saturating_add(attachment.estimated_byte_size);
            match attachment.status {
                AttachmentStatusClass::Live => {}
                AttachmentStatusClass::Stale => {
                    block_reasons.push(BlockReason::StaleAttachment {
                        attachment_id: attachment.attachment_id.clone(),
                    });
                }
                AttachmentStatusClass::TaintedOutsideFencedSection => {
                    block_reasons.push(BlockReason::TaintedAttachmentOutsideFencedSection {
                        attachment_id: attachment.attachment_id.clone(),
                        trust_posture: attachment.trust_posture,
                    });
                }
                AttachmentStatusClass::OverBudget => {
                    block_reasons.push(BlockReason::OverBudgetContext {
                        attachment_id: attachment.attachment_id.clone(),
                    });
                }
                AttachmentStatusClass::PolicyBlocked => {
                    block_reasons.push(BlockReason::PolicyBlockedAttachment {
                        attachment_id: attachment.attachment_id.clone(),
                    });
                }
                AttachmentStatusClass::OutOfScope => {
                    block_reasons.push(BlockReason::OutOfScopeAttachment {
                        attachment_id: attachment.attachment_id.clone(),
                    });
                }
            }

            // Independent invariant: an untrusted posture is required to
            // ride a fenced role. The composer surfaces this even when the
            // caller forgot to flip the attachment's status class, so a
            // surface that mis-labels its own status still gets the typed
            // block reason.
            if attachment.trust_posture.requires_fence()
                && !attachment.placed_under_fenced_role
                && !matches!(
                    attachment.status,
                    AttachmentStatusClass::TaintedOutsideFencedSection
                )
            {
                block_reasons.push(BlockReason::TaintedAttachmentOutsideFencedSection {
                    attachment_id: attachment.attachment_id.clone(),
                    trust_posture: attachment.trust_posture,
                });
            }
        }

        for invocation in &self.slash_command_invocations {
            if invocation.resolution_state != SlashCommandResolutionState::Resolved {
                block_reasons.push(BlockReason::UnresolvedSlashCommand {
                    invocation_id: invocation.invocation_id.clone(),
                    resolution_state: invocation.resolution_state,
                });
            }
        }

        // Aggregate over-budget check supplements the per-attachment
        // status, so a caller that did not pre-classify still gets a typed
        // signal.
        if aggregate > self.budget_byte_ceiling {
            // Attribute the overflow to the last attachment so the
            // inspector has a single addressable row to route the user to.
            if let Some(last) = self.attachments.last() {
                let already_reported =
                    block_reasons.iter().any(|reason| match reason {
                        BlockReason::OverBudgetContext { attachment_id } => {
                            attachment_id == &last.attachment_id
                        }
                        _ => false,
                    });
                if !already_reported {
                    block_reasons.push(BlockReason::OverBudgetContext {
                        attachment_id: last.attachment_id.clone(),
                    });
                }
            }
        }

        // Route placeholder is always denied by policy in the M1 seed; the
        // honesty-marker variant is appended as the final entry so a
        // surface that filters block reasons can split user-actionable
        // resolutions from the always-on prototype label.
        block_reasons.push(BlockReason::PolicyBlockedRoute);

        let state = if block_reasons
            .iter()
            .any(|reason| !matches!(reason, BlockReason::PolicyBlockedRoute))
        {
            ComposerDraftState::BlockedPendingResolution
        } else {
            // Only the always-on policy-blocked-route marker is present —
            // the M1 seed exits in `DispatchDisabledInM1Seed` rather than
            // pretending the draft could ever dispatch.
            ComposerDraftState::DispatchDisabledInM1Seed
        };

        ValidationOutcome {
            state,
            block_reasons,
            aggregate_byte_estimate: aggregate,
            budget_byte_ceiling: self.budget_byte_ceiling,
        }
    }

    /// True when the draft has at least one block reason other than the
    /// always-on policy-blocked-route honesty marker.
    pub fn has_actionable_block_reasons(&self) -> bool {
        self.validate()
            .block_reasons
            .iter()
            .any(|reason| !matches!(reason, BlockReason::PolicyBlockedRoute))
    }
}

const DEFAULT_BUDGET_BYTE_CEILING: u64 = 256 * 1024;

#[cfg(test)]
mod tests;
