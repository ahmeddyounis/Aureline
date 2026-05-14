//! AI composer and context-inspector projection for the bounded launch AI
//! wedge.
//!
//! The inspector is the protected-row surface a user opens when they need to
//! answer "what is in this draft, where did each attachment come from, and
//! why is it blocked?" without trusting the composer's own chrome to be
//! truthful. It is a thin projection over [`aureline_ai::ComposerDraft`]:
//! every value comes verbatim from the draft, every block reason is
//! mechanically derived from [`aureline_ai::ComposerDraft::validate`], and
//! every section / action carries a stable id so the chrome cannot reshape
//! the snapshot per surface.
//!
//! ## Why one projection
//!
//! Forking copies of the inspector per surface would let the launch AI wedge
//! drift its vocabulary while a support / export flow lags. This module
//! renders one snapshot type the chrome quotes verbatim.
//!
//! ## Seed scope (M1)
//!
//! The projection covers:
//!
//! - the prototype label chip and the route placeholder honesty marker,
//! - one section per addressable axis (mentions, attachments, slash-command
//!   invocations, route placeholder, block reasons),
//! - typed per-row addresses so the chrome's inspect / remove / resolve
//!   actions route to the right row,
//! - a deterministic plaintext render the copy-context action quotes
//!   verbatim.
//!
//! Live model dispatch, agentic apply, and route/spend receipts remain out
//! of scope — the inspector renders the typed honesty marker rather than
//! pretending those rows exist.

use serde::{Deserialize, Serialize};

use aureline_ai::{
    BlockReason, ComposerAttachment, ComposerContextAlphaSnapshot, ComposerDraft,
    ComposerDraftState, ComposerMention, ComposerSlashCommandInvocation, PrototypeLabel,
    ValidationOutcome,
};

/// Stable record-kind tag carried in serialized inspector snapshots.
pub const AI_CONTEXT_INSPECTOR_RECORD_KIND: &str = "ai_context_inspector_snapshot_record";

/// Schema version for the [`AiContextInspectorSnapshot`] payload shape.
pub const AI_CONTEXT_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Stable section ids the inspector renders. The order is the canonical
/// reading order: prototype label -> intent -> mentions -> attachments ->
/// slash commands -> route placeholder -> block reasons -> draft state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorSectionId {
    PrototypeLabel,
    Intent,
    Mentions,
    Attachments,
    SlashCommands,
    RoutePlaceholder,
    BlockReasons,
    DraftState,
}

impl InspectorSectionId {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrototypeLabel => "prototype_label",
            Self::Intent => "intent",
            Self::Mentions => "mentions",
            Self::Attachments => "attachments",
            Self::SlashCommands => "slash_commands",
            Self::RoutePlaceholder => "route_placeholder",
            Self::BlockReasons => "block_reasons",
            Self::DraftState => "draft_state",
        }
    }

    /// Human-readable section heading.
    pub const fn heading(self) -> &'static str {
        match self {
            Self::PrototypeLabel => "Prototype wedge",
            Self::Intent => "Intent",
            Self::Mentions => "Mentions",
            Self::Attachments => "Attachments",
            Self::SlashCommands => "Slash commands",
            Self::RoutePlaceholder => "Route placeholder",
            Self::BlockReasons => "Block reasons",
            Self::DraftState => "Draft state",
        }
    }
}

/// Stable inspector actions. These are the only addressable actions the
/// inspector exposes in the M1 seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorAction {
    /// Copy the deterministic plaintext render of the draft for support /
    /// review exports.
    CopyDraft,
    /// Inspect a single attachment (focused row).
    InspectAttachment,
    /// Remove a single attachment. The chrome routes the user to the typed
    /// remove API on the composer; the inspector itself is read-only.
    RemoveAttachment,
    /// Resolve a mention or slash-command invocation (focused row).
    ResolveAddressable,
    /// Return to the launch AI wedge surface that opened the inspector.
    ReturnToComposer,
}

impl InspectorAction {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyDraft => "copy_draft",
            Self::InspectAttachment => "inspect_attachment",
            Self::RemoveAttachment => "remove_attachment",
            Self::ResolveAddressable => "resolve_addressable",
            Self::ReturnToComposer => "return_to_composer",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CopyDraft => "Copy draft",
            Self::InspectAttachment => "Inspect attachment",
            Self::RemoveAttachment => "Remove attachment",
            Self::ResolveAddressable => "Resolve",
            Self::ReturnToComposer => "Return to composer",
        }
    }
}

/// Stable per-row honesty marker the chrome quotes verbatim when a row is
/// blocked, tainted, or otherwise non-actionable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorRowStatusClass {
    /// Row is live; no honesty marker needed.
    Live,
    /// Row is blocked by a typed reason quoted on the row.
    Blocked,
    /// Row is informational only (e.g. prototype-label chip).
    Informational,
    /// Row is the always-on M1 honesty marker for dispatch being disabled.
    DispatchDisabled,
}

impl InspectorRowStatusClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Blocked => "blocked",
            Self::Informational => "informational",
            Self::DispatchDisabled => "dispatch_disabled",
        }
    }
}

/// Addressable target of a row. The chrome reads this so an `Inspect` /
/// `Resolve` / `Remove` action routes to the matching composer object
/// without the inspector having to fork addressing schemes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InspectorRowAddress {
    /// Row addresses a composer mention by id.
    Mention { mention_id: String },
    /// Row addresses a composer attachment by id.
    Attachment { attachment_id: String },
    /// Row addresses a composer slash-command invocation by id.
    SlashCommandInvocation { invocation_id: String },
    /// Row addresses the always-present route placeholder.
    RoutePlaceholder,
    /// Row is a descriptive / chip row with no addressable target.
    None,
}

/// One inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorRow {
    /// Stable row id within the section. Combine with the section id for a
    /// fully addressable row identity.
    pub row_id: String,
    /// Human-readable row label (e.g. "Trust posture").
    pub label: String,
    /// Resolved value the chrome quotes verbatim.
    pub value: String,
    /// Stable token form of the value when the row is value-bearing. Null
    /// when the row is purely descriptive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
    pub status: InspectorRowStatusClass,
    pub address: InspectorRowAddress,
    /// Block-reason string when the row's status is `Blocked`. The
    /// inspector quotes the typed reason verbatim so the chrome cannot
    /// hide it behind a generic warning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason_token: Option<String>,
}

impl InspectorRow {
    fn descriptive(row_id: &str, label: &str, value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            row_id: row_id.to_owned(),
            label: label.to_owned(),
            value_token: Some(value.clone()),
            value,
            status: InspectorRowStatusClass::Informational,
            address: InspectorRowAddress::None,
            blocked_reason_token: None,
        }
    }
}

/// One inspector section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSection {
    pub section_id: InspectorSectionId,
    pub heading: String,
    pub rows: Vec<InspectorRow>,
}

impl InspectorSection {
    fn new(section_id: InspectorSectionId, rows: Vec<InspectorRow>) -> Self {
        Self {
            section_id,
            heading: section_id.heading().to_owned(),
            rows,
        }
    }
}

/// One inspector action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorActionRow {
    pub action: InspectorAction,
    pub label: String,
}

impl InspectorActionRow {
    fn with_label(action: InspectorAction) -> Self {
        Self {
            action,
            label: action.label().to_owned(),
        }
    }
}

/// Inspector snapshot.
///
/// The snapshot is the canonical record the chrome renders, a support
/// export quotes, and a fixture replays. Every section is always present —
/// even when its rows reduce to the "no items" honesty marker — so a
/// degraded snapshot is never silently smaller than a green snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiContextInspectorSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub composer_draft_id: String,
    pub composer_session_id: String,
    pub request_workspace_id: String,
    pub prototype_label_token: String,
    pub prototype_label_text: String,
    pub sections: Vec<InspectorSection>,
    pub actions: Vec<InspectorActionRow>,
    /// True when at least one block reason other than the always-on
    /// dispatch-disabled honesty marker is recorded. The chrome MUST
    /// render a visible honesty chip when this is true.
    pub has_actionable_blocks: bool,
    /// True when the draft carries an attachment whose trust posture
    /// requires fencing. Surfaces use this to badge the wedge as carrying
    /// tainted context even before the user opens the inspector.
    pub has_tainted_attachments: bool,
}

impl AiContextInspectorSnapshot {
    /// Project a snapshot from a [`ComposerDraft`].
    pub fn project(draft: &ComposerDraft) -> Self {
        let outcome = draft.validate();
        let sections = vec![
            project_prototype_section(draft.prototype_label),
            project_intent_section(&draft.intent_text),
            project_mentions_section(&draft.mentions),
            project_attachments_section(&draft.attachments, &outcome),
            project_slash_commands_section(&draft.slash_command_invocations),
            project_route_section(&draft.route_placeholder),
            project_block_reasons_section(&outcome),
            project_draft_state_section(outcome.state),
        ];

        let has_actionable_blocks = outcome
            .block_reasons
            .iter()
            .any(|reason| !matches!(reason, BlockReason::PolicyBlockedRoute));

        let has_tainted_attachments = outcome.block_reasons.iter().any(|reason| {
            matches!(
                reason,
                BlockReason::TaintedAttachmentOutsideFencedSection { .. }
            )
        });

        Self {
            record_kind: AI_CONTEXT_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: AI_CONTEXT_INSPECTOR_SCHEMA_VERSION,
            composer_draft_id: draft.composer_draft_id.clone(),
            composer_session_id: draft.composer_session_id.clone(),
            request_workspace_id: draft.request_workspace_id.clone(),
            prototype_label_token: draft.prototype_label.as_str().to_owned(),
            prototype_label_text: draft.prototype_label.label().to_owned(),
            sections,
            actions: default_actions(),
            has_actionable_blocks,
            has_tainted_attachments,
        }
    }

    /// Locate one section by id.
    pub fn section(&self, id: InspectorSectionId) -> Option<&InspectorSection> {
        self.sections
            .iter()
            .find(|section| section.section_id == id)
    }

    /// Iterator over the snapshot's stable actions.
    pub fn actions(&self) -> impl Iterator<Item = &InspectorActionRow> {
        self.actions.iter()
    }

    /// Render a deterministic plaintext block downstream consumers can
    /// quote (copy-draft action, support exports, fixture replays). The
    /// block is stable across runs for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("AI composer / context inspector\n");
        out.push_str(&format!(
            "Prototype: {prototype}\nComposer draft: {draft}\nSession: {session}\nWorkspace: {ws}\n\n",
            prototype = self.prototype_label_text,
            draft = self.composer_draft_id,
            session = self.composer_session_id,
            ws = self.request_workspace_id,
        ));
        for section in &self.sections {
            out.push_str(&format!("[{}]\n", section.heading));
            if section.rows.is_empty() {
                out.push_str("  (no rows)\n");
            }
            for row in &section.rows {
                out.push_str(&format!("  {}: {}", row.label, row.value));
                match row.status {
                    InspectorRowStatusClass::Blocked => {
                        if let Some(reason) = &row.blocked_reason_token {
                            out.push_str(&format!("  [blocked: {reason}]"));
                        } else {
                            out.push_str("  [blocked]");
                        }
                    }
                    InspectorRowStatusClass::DispatchDisabled => {
                        out.push_str("  [dispatch_disabled]");
                    }
                    InspectorRowStatusClass::Informational | InspectorRowStatusClass::Live => {}
                }
                out.push('\n');
            }
            out.push('\n');
        }
        out
    }
}

fn default_actions() -> Vec<InspectorActionRow> {
    vec![
        InspectorActionRow::with_label(InspectorAction::CopyDraft),
        InspectorActionRow::with_label(InspectorAction::InspectAttachment),
        InspectorActionRow::with_label(InspectorAction::RemoveAttachment),
        InspectorActionRow::with_label(InspectorAction::ResolveAddressable),
        InspectorActionRow::with_label(InspectorAction::ReturnToComposer),
    ]
}

fn project_prototype_section(label: PrototypeLabel) -> InspectorSection {
    let rows = vec![InspectorRow {
        row_id: "prototype_chip".to_owned(),
        label: "Wedge label".to_owned(),
        value: label.label().to_owned(),
        value_token: Some(label.as_str().to_owned()),
        status: InspectorRowStatusClass::Informational,
        address: InspectorRowAddress::None,
        blocked_reason_token: None,
    }];
    InspectorSection::new(InspectorSectionId::PrototypeLabel, rows)
}

fn project_intent_section(intent_text: &str) -> InspectorSection {
    let value = if intent_text.is_empty() {
        "(empty draft)".to_owned()
    } else {
        intent_text.to_owned()
    };
    let rows = vec![InspectorRow {
        row_id: "intent_text".to_owned(),
        label: "Draft intent".to_owned(),
        value,
        value_token: None,
        status: InspectorRowStatusClass::Informational,
        address: InspectorRowAddress::None,
        blocked_reason_token: None,
    }];
    InspectorSection::new(InspectorSectionId::Intent, rows)
}

fn project_mentions_section(mentions: &[ComposerMention]) -> InspectorSection {
    let rows = if mentions.is_empty() {
        vec![InspectorRow {
            row_id: "no_mentions".to_owned(),
            label: "Mentions".to_owned(),
            value: "(none)".to_owned(),
            value_token: Some("none".to_owned()),
            status: InspectorRowStatusClass::Informational,
            address: InspectorRowAddress::None,
            blocked_reason_token: None,
        }]
    } else {
        mentions
            .iter()
            .map(|mention| {
                let resolved = matches!(
                    mention.resolution_state,
                    aureline_ai::MentionResolutionState::Resolved
                );
                let value = if resolved {
                    mention
                        .target_stable_id
                        .clone()
                        .unwrap_or_else(|| mention.display_label.clone())
                } else {
                    format!(
                        "{label} (unresolved: {state})",
                        label = mention.display_label,
                        state = mention.resolution_state.as_str()
                    )
                };
                InspectorRow {
                    row_id: format!("mention_{}", mention.mention_id),
                    label: format!("{kind}", kind = mention.kind.as_str()),
                    value,
                    value_token: mention.target_stable_id.clone(),
                    status: if resolved {
                        InspectorRowStatusClass::Live
                    } else {
                        InspectorRowStatusClass::Blocked
                    },
                    address: InspectorRowAddress::Mention {
                        mention_id: mention.mention_id.clone(),
                    },
                    blocked_reason_token: if resolved {
                        None
                    } else {
                        Some(mention.resolution_state.as_str().to_owned())
                    },
                }
            })
            .collect()
    };
    InspectorSection::new(InspectorSectionId::Mentions, rows)
}

fn project_attachments_section(
    attachments: &[ComposerAttachment],
    outcome: &ValidationOutcome,
) -> InspectorSection {
    let rows = if attachments.is_empty() {
        vec![InspectorRow {
            row_id: "no_attachments".to_owned(),
            label: "Attachments".to_owned(),
            value: "(none)".to_owned(),
            value_token: Some("none".to_owned()),
            status: InspectorRowStatusClass::Informational,
            address: InspectorRowAddress::None,
            blocked_reason_token: None,
        }]
    } else {
        attachments
            .iter()
            .map(|attachment| attachment_row(attachment, outcome))
            .collect()
    };
    InspectorSection::new(InspectorSectionId::Attachments, rows)
}

fn attachment_row(attachment: &ComposerAttachment, outcome: &ValidationOutcome) -> InspectorRow {
    let block = outcome.block_reasons.iter().find(|reason| match reason {
        BlockReason::StaleAttachment { attachment_id }
        | BlockReason::OverBudgetContext { attachment_id }
        | BlockReason::PolicyBlockedAttachment { attachment_id }
        | BlockReason::OutOfScopeAttachment { attachment_id } => {
            attachment_id == &attachment.attachment_id
        }
        BlockReason::TaintedAttachmentOutsideFencedSection { attachment_id, .. } => {
            attachment_id == &attachment.attachment_id
        }
        _ => false,
    });

    let (status, blocked_reason_token) = if let Some(reason) = block {
        (
            InspectorRowStatusClass::Blocked,
            Some(reason.as_str().to_owned()),
        )
    } else {
        (InspectorRowStatusClass::Live, None)
    };

    let mut value = format!(
        "{label} ({source}/{trust}/{selection}, {bytes} bytes)",
        label = attachment.display_label,
        source = attachment.source_class.as_str(),
        trust = attachment.trust_posture.as_str(),
        selection = attachment.selection_reason.as_str(),
        bytes = attachment.estimated_byte_size,
    );
    if let Some(scope_truth) = attachment.scope_truth.as_ref() {
        value.push_str(&format!(
            "; scope {candidate_scope} via {active_scope}, counts {counts}, freshness {freshness}",
            candidate_scope = scope_truth.candidate_scope_label,
            active_scope = scope_truth.active_scope_label,
            counts = scope_truth.counts.counts_class_token,
            freshness = scope_truth.freshness_token,
        ));
    }

    InspectorRow {
        row_id: format!("attachment_{}", attachment.attachment_id),
        label: attachment.kind.as_str().to_owned(),
        value,
        value_token: Some(attachment.attachment_id.clone()),
        status,
        address: InspectorRowAddress::Attachment {
            attachment_id: attachment.attachment_id.clone(),
        },
        blocked_reason_token,
    }
}

fn project_slash_commands_section(
    invocations: &[ComposerSlashCommandInvocation],
) -> InspectorSection {
    let rows = if invocations.is_empty() {
        vec![InspectorRow {
            row_id: "no_slash_commands".to_owned(),
            label: "Slash commands".to_owned(),
            value: "(none)".to_owned(),
            value_token: Some("none".to_owned()),
            status: InspectorRowStatusClass::Informational,
            address: InspectorRowAddress::None,
            blocked_reason_token: None,
        }]
    } else {
        invocations
            .iter()
            .map(|invocation| {
                let resolved = matches!(
                    invocation.resolution_state,
                    aureline_ai::SlashCommandResolutionState::Resolved
                );
                let value = if resolved {
                    invocation.command_id.clone()
                } else {
                    format!(
                        "{label} (unresolved: {state})",
                        label = invocation.display_label,
                        state = invocation.resolution_state.as_str()
                    )
                };
                InspectorRow {
                    row_id: format!("invocation_{}", invocation.invocation_id),
                    label: invocation.display_label.clone(),
                    value,
                    value_token: if resolved {
                        Some(invocation.command_id.clone())
                    } else {
                        None
                    },
                    status: if resolved {
                        InspectorRowStatusClass::Live
                    } else {
                        InspectorRowStatusClass::Blocked
                    },
                    address: InspectorRowAddress::SlashCommandInvocation {
                        invocation_id: invocation.invocation_id.clone(),
                    },
                    blocked_reason_token: if resolved {
                        None
                    } else {
                        Some(invocation.resolution_state.as_str().to_owned())
                    },
                }
            })
            .collect()
    };
    InspectorSection::new(InspectorSectionId::SlashCommands, rows)
}

fn project_route_section(route: &aureline_ai::RoutePlaceholder) -> InspectorSection {
    let rows = vec![
        InspectorRow {
            row_id: "provider_class".to_owned(),
            label: "Provider".to_owned(),
            value: route.provider_class.label().to_owned(),
            value_token: Some(route.provider_class.as_str().to_owned()),
            status: InspectorRowStatusClass::DispatchDisabled,
            address: InspectorRowAddress::RoutePlaceholder,
            blocked_reason_token: None,
        },
        InspectorRow {
            row_id: "route_path_class".to_owned(),
            label: "Route".to_owned(),
            value: route.route_path_class.label().to_owned(),
            value_token: Some(route.route_path_class.as_str().to_owned()),
            status: InspectorRowStatusClass::DispatchDisabled,
            address: InspectorRowAddress::RoutePlaceholder,
            blocked_reason_token: None,
        },
        InspectorRow {
            row_id: "dispatch_target_class".to_owned(),
            label: "Dispatch target".to_owned(),
            value: route.dispatch_target_class.label().to_owned(),
            value_token: Some(route.dispatch_target_class.as_str().to_owned()),
            status: InspectorRowStatusClass::DispatchDisabled,
            address: InspectorRowAddress::RoutePlaceholder,
            blocked_reason_token: None,
        },
        InspectorRow::descriptive("seed_note", "Seed note", route.seed_note.clone()),
    ];
    InspectorSection::new(InspectorSectionId::RoutePlaceholder, rows)
}

fn project_block_reasons_section(outcome: &ValidationOutcome) -> InspectorSection {
    let rows = if outcome.block_reasons.is_empty() {
        vec![InspectorRow {
            row_id: "no_block_reasons".to_owned(),
            label: "Block reasons".to_owned(),
            value: "None".to_owned(),
            value_token: Some("none".to_owned()),
            status: InspectorRowStatusClass::Informational,
            address: InspectorRowAddress::None,
            blocked_reason_token: None,
        }]
    } else {
        outcome
            .block_reasons
            .iter()
            .enumerate()
            .map(|(idx, reason)| block_reason_row(idx, reason))
            .collect()
    };
    InspectorSection::new(InspectorSectionId::BlockReasons, rows)
}

fn block_reason_row(idx: usize, reason: &BlockReason) -> InspectorRow {
    let (value, address, status) = match reason {
        BlockReason::UnresolvedMention {
            mention_id,
            resolution_state,
        } => (
            format!(
                "{label} ({state}) — mention {mention_id}",
                label = reason.label(),
                state = resolution_state.as_str(),
            ),
            InspectorRowAddress::Mention {
                mention_id: mention_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::StaleAttachment { attachment_id } => (
            format!(
                "{label} — attachment {attachment_id}",
                label = reason.label()
            ),
            InspectorRowAddress::Attachment {
                attachment_id: attachment_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::TaintedAttachmentOutsideFencedSection {
            attachment_id,
            trust_posture,
        } => (
            format!(
                "{label} ({posture}) — attachment {attachment_id}",
                label = reason.label(),
                posture = trust_posture.as_str(),
            ),
            InspectorRowAddress::Attachment {
                attachment_id: attachment_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::OverBudgetContext { attachment_id } => (
            format!(
                "{label} — attachment {attachment_id}",
                label = reason.label()
            ),
            InspectorRowAddress::Attachment {
                attachment_id: attachment_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::PolicyBlockedAttachment { attachment_id } => (
            format!(
                "{label} — attachment {attachment_id}",
                label = reason.label()
            ),
            InspectorRowAddress::Attachment {
                attachment_id: attachment_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::OutOfScopeAttachment { attachment_id } => (
            format!(
                "{label} — attachment {attachment_id}",
                label = reason.label()
            ),
            InspectorRowAddress::Attachment {
                attachment_id: attachment_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::UnresolvedSlashCommand {
            invocation_id,
            resolution_state,
        } => (
            format!(
                "{label} ({state}) — invocation {invocation_id}",
                label = reason.label(),
                state = resolution_state.as_str(),
            ),
            InspectorRowAddress::SlashCommandInvocation {
                invocation_id: invocation_id.clone(),
            },
            InspectorRowStatusClass::Blocked,
        ),
        BlockReason::PolicyBlockedRoute => (
            reason.label().to_owned(),
            InspectorRowAddress::RoutePlaceholder,
            InspectorRowStatusClass::DispatchDisabled,
        ),
    };
    InspectorRow {
        row_id: format!("block_{idx}_{}", reason.as_str()),
        label: reason.as_str().to_owned(),
        value,
        value_token: Some(reason.as_str().to_owned()),
        status,
        address,
        blocked_reason_token: Some(reason.as_str().to_owned()),
    }
}

fn project_draft_state_section(state: ComposerDraftState) -> InspectorSection {
    let row_status = match state {
        ComposerDraftState::Drafting => InspectorRowStatusClass::Live,
        ComposerDraftState::BlockedPendingResolution => InspectorRowStatusClass::Blocked,
        ComposerDraftState::ReadyForReviewOnly | ComposerDraftState::DispatchDisabledInM1Seed => {
            InspectorRowStatusClass::DispatchDisabled
        }
    };
    let rows = vec![InspectorRow {
        row_id: "draft_state".to_owned(),
        label: "State".to_owned(),
        value: state.label().to_owned(),
        value_token: Some(state.as_str().to_owned()),
        status: row_status,
        address: InspectorRowAddress::None,
        blocked_reason_token: None,
    }];
    InspectorSection::new(InspectorSectionId::DraftState, rows)
}

/// Stable record-kind tag for shell projections of the alpha composer context snapshot.
pub const AI_CONTEXT_INSPECTOR_ALPHA_PROJECTION_RECORD_KIND: &str =
    "ai_context_inspector_alpha_projection_record";

/// Shell-side projection of [`ComposerContextAlphaSnapshot`].
///
/// The projection is intentionally shallow: it quotes the AI crate snapshot's
/// tokens and labels into shell rows so the UI can render the alpha context
/// inspector without owning a second context state model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiContextInspectorAlphaProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version from the source snapshot.
    pub schema_version: u32,
    /// Composer context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Composer draft id.
    pub composer_draft_id: String,
    /// Composer session id.
    pub composer_session_id: String,
    /// Request workspace id.
    pub request_workspace_id: String,
    /// Rows rendered by the shell.
    pub rows: Vec<AlphaInspectorRow>,
    /// True when the source snapshot has docs rows with visible citation truth.
    pub has_docs_citation_truth: bool,
    /// True when omitted, blocked, stale, or tainted context is visible.
    pub has_non_included_context_truth: bool,
    /// Evidence handoff row count produced from the same source snapshot.
    pub evidence_handoff_row_count: usize,
}

impl AiContextInspectorAlphaProjection {
    /// Project shell rows from the canonical AI context snapshot.
    pub fn project(snapshot: &ComposerContextAlphaSnapshot) -> Self {
        let mut rows = vec![
            AlphaInspectorRow {
                row_id: "intent_mode".to_owned(),
                label: "Intent mode".to_owned(),
                value: snapshot.intent_mode.as_str().to_owned(),
                value_token: snapshot.intent_mode.as_str().to_owned(),
                status: InspectorRowStatusClass::Live,
                source_ref: snapshot.composer_draft_id.clone(),
            },
            AlphaInspectorRow {
                row_id: "scope".to_owned(),
                label: "Scope".to_owned(),
                value: snapshot.scope_label.clone(),
                value_token: snapshot.scope_label.clone(),
                status: InspectorRowStatusClass::Live,
                source_ref: snapshot.request_workspace_id.clone(),
            },
            AlphaInspectorRow {
                row_id: "budget".to_owned(),
                label: "Budget".to_owned(),
                value: format!(
                    "{} ({} / {} bytes)",
                    snapshot.budget_strip.pressure_class.as_str(),
                    snapshot.budget_strip.aggregate_byte_estimate,
                    snapshot.budget_strip.budget_byte_ceiling
                ),
                value_token: snapshot.budget_strip.pressure_class.as_str().to_owned(),
                status: alpha_state_status(snapshot.budget_strip.pressure_class.as_str()),
                source_ref: snapshot.review_lock.context_snapshot_ref.clone(),
            },
            AlphaInspectorRow {
                row_id: "route".to_owned(),
                label: "Route".to_owned(),
                value: format!(
                    "{} / {} / {}",
                    snapshot.budget_strip.selected_provider_label,
                    snapshot.budget_strip.selected_model_label,
                    snapshot.budget_strip.quota_state_token
                ),
                value_token: snapshot.review_lock.route_snapshot_ref.clone(),
                status: InspectorRowStatusClass::Live,
                source_ref: snapshot.review_lock.route_snapshot_ref.clone(),
            },
        ];

        for mention in &snapshot.mention_previews {
            rows.push(AlphaInspectorRow {
                row_id: format!("mention_{}", mention.mention_id),
                label: mention.kind.as_str().to_owned(),
                value: mention.display_label.clone(),
                value_token: mention.preview_state.as_str().to_owned(),
                status: alpha_state_status(mention.preview_state.as_str()),
                source_ref: mention
                    .target_stable_id
                    .clone()
                    .unwrap_or_else(|| mention.mention_id.clone()),
            });
        }

        for pill in &snapshot.attachment_pills {
            let docs_tail = pill
                .docs_identity
                .as_ref()
                .map(|docs| {
                    format!(
                        " / {} / {}",
                        docs.citation_availability_class.as_str(),
                        docs.source_language_fallback_class.as_str()
                    )
                })
                .unwrap_or_default();
            rows.push(AlphaInspectorRow {
                row_id: format!("attachment_{}", pill.attachment_id),
                label: pill.kind.as_str().to_owned(),
                value: format!(
                    "{} / {} / {}{}",
                    pill.display_label,
                    pill.source_class.as_str(),
                    pill.context_state.as_str(),
                    docs_tail
                ),
                value_token: pill.context_state.as_str().to_owned(),
                status: alpha_state_status(pill.context_state.as_str()),
                source_ref: pill.attachment_id.clone(),
            });
        }

        for item in &snapshot.context_items {
            rows.push(AlphaInspectorRow {
                row_id: format!("context_{}", item.context_item_id),
                label: item.group_class.label().to_owned(),
                value: format!(
                    "{} / {} / {} / {} / {}",
                    item.display_label,
                    item.state_class.as_str(),
                    item.source_class.as_str(),
                    item.freshness_class.as_str(),
                    item.trust_class.as_str()
                ),
                value_token: item.state_class.as_str().to_owned(),
                status: alpha_state_status(item.state_class.as_str()),
                source_ref: item.stable_identity_ref.clone(),
            });
        }

        let handoff = snapshot.evidence_handoff(format!(
            "context-handoff:{}",
            snapshot.review_lock.context_snapshot_ref
        ));
        let has_docs_citation_truth = snapshot.context_items.iter().any(|item| {
            item.docs_identity
                .as_ref()
                .is_some_and(|docs| docs.exact_anchor_ref.is_some() || docs.citation_note.is_some())
        });
        let has_non_included_context_truth = snapshot.context_items.iter().any(|item| {
            matches!(
                item.state_class.as_str(),
                "omitted" | "blocked" | "stale" | "tainted" | "summarized"
            )
        });

        Self {
            record_kind: AI_CONTEXT_INSPECTOR_ALPHA_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: snapshot.schema_version,
            composer_context_snapshot_ref: snapshot.review_lock.context_snapshot_ref.clone(),
            composer_draft_id: snapshot.composer_draft_id.clone(),
            composer_session_id: snapshot.composer_session_id.clone(),
            request_workspace_id: snapshot.request_workspace_id.clone(),
            rows,
            has_docs_citation_truth,
            has_non_included_context_truth,
            evidence_handoff_row_count: handoff.context_rows.len(),
        }
    }
}

/// One shell row projected from the alpha context snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaInspectorRow {
    /// Stable row id.
    pub row_id: String,
    /// User-visible row label.
    pub label: String,
    /// User-visible row value.
    pub value: String,
    /// Stable token backing the value.
    pub value_token: String,
    /// Row status.
    pub status: InspectorRowStatusClass,
    /// Source ref used for open/inspect actions.
    pub source_ref: String,
}

fn alpha_state_status(token: &str) -> InspectorRowStatusClass {
    match token {
        "blocked"
        | "stale"
        | "ambiguous"
        | "unresolved"
        | "budget_review_required"
        | "overflow" => InspectorRowStatusClass::Blocked,
        "omitted" | "tainted" | "summarized" => InspectorRowStatusClass::Informational,
        _ => InspectorRowStatusClass::Live,
    }
}

#[cfg(test)]
mod tests;
