//! Canonical transfer-safety model for clipboard, paste, drag/drop, rich
//! content, large transfers, and named undo-group truth.
//!
//! This module mints the governed [`TransferSafetyPacket`] consumed by editor,
//! terminal, notebook, docs, shell, and support surfaces. The builder enforces
//! the stable interaction-safety invariants before any surface may claim that a
//! transfer path is predictable, reversible, and boundary-aware.

use serde::{Deserialize, Serialize};

/// Schema version for transfer-safety packets.
pub const TRANSFER_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Schema reference consumed by cross-surface transfer-safety fixtures.
pub const TRANSFER_SAFETY_SCHEMA_REF: &str = "schemas/ux/transfer-safety.schema.json";

/// Stable record-kind tag for [`TransferSafetyPacket`].
pub const TRANSFER_SAFETY_PACKET_RECORD_KIND: &str = "transfer_safety_packet";

/// Surface families covered by transfer-safety truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferSurfaceClass {
    /// Source editor and raw diff surfaces.
    Editor,
    /// Integrated terminal and PTY surfaces.
    Terminal,
    /// Notebook cell, output, and result-viewer surfaces.
    Notebook,
    /// Docs, Help, Markdown, and rich preview surfaces.
    Docs,
    /// Support export, incident, and diagnostics handoff surfaces.
    Support,
    /// Shell chrome, tab, pane, and cross-window drag/drop surfaces.
    Shell,
}

impl TransferSurfaceClass {
    /// Returns the stable schema token for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Terminal => "terminal",
            Self::Notebook => "notebook",
            Self::Docs => "docs",
            Self::Support => "support",
            Self::Shell => "shell",
        }
    }
}

/// Representation of the payload or view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferRepresentationClass {
    /// Exact source text or bytes.
    Raw,
    /// Current rendered view.
    Rendered,
    /// Escaped source representation safe for inspection.
    Escaped,
    /// Static snapshot with active content removed.
    Sanitized,
    /// Sandboxed rich rendering; not a transfer output by itself.
    Sandboxed,
    /// Generated content with attribution requirements.
    Generated,
    /// Body withheld, with typed metadata only.
    BlockedMetadataOnly,
}

impl TransferRepresentationClass {
    /// Returns true when this representation can be treated as plain text.
    pub const fn is_plain_text(self) -> bool {
        matches!(self, Self::Raw | Self::Escaped)
    }
}

/// User-visible transfer operation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferActionClass {
    /// Copy bytes or text to a clipboard/register/export target.
    Copy,
    /// Paste bytes or text into a mutating target.
    Paste,
    /// Drag/drop operation across surfaces.
    DragDrop,
    /// Import from an external source into workspace state.
    Import,
    /// Attach output or evidence to another artifact.
    Attach,
    /// Open or reveal a transfer target without mutation.
    Open,
    /// Move an existing object or view.
    Move,
    /// Split an existing view or pane.
    Split,
    /// Apply a broad replacement across files.
    MultiFileReplace,
    /// Apply a model- or assistant-produced change.
    AiApply,
    /// Import settings into effective configuration.
    SettingsImport,
}

impl TransferActionClass {
    /// Returns the stable schema token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Paste => "paste",
            Self::DragDrop => "drag_drop",
            Self::Import => "import",
            Self::Attach => "attach",
            Self::Open => "open",
            Self::Move => "move",
            Self::Split => "split",
            Self::MultiFileReplace => "multi_file_replace",
            Self::AiApply => "ai_apply",
            Self::SettingsImport => "settings_import",
        }
    }
}

/// Resulting drop verb shown before a drop commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DropVerb {
    /// Move the source into the target.
    Move,
    /// Copy the source into the target.
    Copy,
    /// Attach the source to the target artifact.
    Attach,
    /// Open the source at the target.
    Open,
    /// Import the source into workspace state.
    Import,
    /// Split or create a new pane/window.
    Split,
}

impl DropVerb {
    /// Returns the stable schema token for this verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Copy => "copy",
            Self::Attach => "attach",
            Self::Open => "open",
            Self::Import => "import",
            Self::Split => "split",
        }
    }
}

/// Boundary class that may alter transfer authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// Local desktop or workspace boundary.
    Local,
    /// Remote SSH host.
    SshRemote,
    /// Container or devcontainer boundary.
    Container,
    /// Managed cloud workspace boundary.
    ManagedWorkspace,
    /// Browser companion or embedded web boundary.
    BrowserCompanion,
    /// Support or incident export boundary.
    SupportExport,
}

/// Rich-content trust posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RichTrustClass {
    /// Plain source bytes are shown.
    RawText,
    /// Rich content is sanitized into an inert view.
    SanitizedRich,
    /// Trusted local active content is allowed.
    TrustedLocalActive,
    /// Remote active content is isolated.
    IsolatedRemoteActive,
}

/// Named undo/recovery class for a transfer mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryClass {
    /// Local exact undo is available.
    ExactUndo,
    /// A durable checkpoint restores the prior state.
    RestoreFromCheckpoint,
    /// A compensating rollback is available.
    CompensatingRollback,
    /// Evidence-only path; no command rerun is allowed.
    EvidenceOnlyNoRerun,
}

/// Plain/rich representation truth for one transfer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationTruth {
    /// Default transfer preserves the most useful plain-text representation.
    pub default_plain_text_preserved: bool,
    /// Raw/source copy remains reachable when source exists.
    pub raw_copy_available: bool,
    /// Rendered copy is available as an explicit additive action.
    pub rendered_copy_available: bool,
    /// Escaped copy is available for suspicious or control-heavy content.
    pub escaped_copy_available: bool,
    /// Representation class used by the default transfer.
    pub default_representation: TransferRepresentationClass,
    /// User-visible label naming the default representation.
    pub representation_label: String,
}

/// Sensitive or boundary-crossing review state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SensitiveReview {
    /// Sensitive content classes present in the transfer.
    pub content_classes: Vec<String>,
    /// Visible label shown before copy/paste/write completes.
    pub visible_label: String,
    /// True when review is shown before the risky action completes.
    pub preview_before_commit: bool,
    /// True when the action crosses local/remote/support boundaries.
    pub boundary_crossing: bool,
    /// True when policy can allow, deny, or narrow the transfer.
    pub policy_gate_present: bool,
}

/// Active boundary context shown with risky transfers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryContext {
    /// Boundary class that applies to the target.
    pub boundary_class: BoundaryClass,
    /// Human-facing boundary label.
    pub visible_boundary_label: String,
    /// True when the boundary label is shown before commit.
    pub shown_before_commit: bool,
}

/// Paste guardrails for high-risk or multiline paste.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasteGuardrail {
    /// Number of lines in the paste payload.
    pub line_count: u32,
    /// True when bracketed paste is available end-to-end.
    pub bracketed_paste_available: bool,
    /// True when automatic submit is disabled for this paste.
    pub automatic_submit_disabled: bool,
    /// True when user confirmation is required before sending.
    pub confirmation_required: bool,
    /// Summary text shown in the paste review surface.
    pub review_summary: String,
}

/// Drag/drop preview truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DropPreview {
    /// Resulting verb shown before the drop commits.
    pub verb: DropVerb,
    /// True when an insertion or target indicator is visible.
    pub insertion_indicator_visible: bool,
    /// True when modifier-key meaning is shown near the pointer or target.
    pub modifier_cues_visible: bool,
    /// True when an equivalent keyboard/command route exists.
    pub keyboard_route_available: bool,
    /// Stable command fallback id.
    pub command_fallback_id: String,
}

/// Named undo group and history/recovery truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoGroupTruth {
    /// True when the mutation registers a named undo group.
    pub named: bool,
    /// User-facing undo-group label.
    pub group_label: String,
    /// Source attribution for the mutation.
    pub source_attribution: String,
    /// Stable mutation journal entry reference.
    pub mutation_journal_entry_ref: String,
    /// Recovery class exposed to history/reopen surfaces.
    pub recovery_class: RecoveryClass,
    /// Surfaces that can explain or reopen the mutation context.
    pub history_surfaces: Vec<String>,
}

/// Progress, cancellation, and completion summary for large transfers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeTransferFeedback {
    /// True when progress is visible during the transfer.
    pub progress_visible: bool,
    /// True when cancellation is available.
    pub cancellation_available: bool,
    /// True when a post-action summary is shown.
    pub post_action_summary_present: bool,
    /// Summary shown after completion or cancellation.
    pub result_summary: String,
}

/// Rich-content trust and raw/source inspectability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichContentTrust {
    /// Trust class used to render or transfer the content.
    pub trust_class: RichTrustClass,
    /// True when active content is blocked or isolated.
    pub active_content_blocked_or_isolated: bool,
    /// True when raw source inspection remains reachable.
    pub raw_source_available: bool,
    /// True when plain text copy remains available.
    pub copy_plain_text_available: bool,
}

/// Per-surface projection that proves shared packet consumption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProjection {
    /// Surface consuming the shared packet.
    pub surface: TransferSurfaceClass,
    /// True when the surface reads the shared record instead of local semantics.
    pub reads_shared_record: bool,
    /// Summary line suitable for diagnostics/support export.
    pub summary_line: String,
}

/// Input used to build a governed transfer-safety packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferSafetyInput {
    /// Stable packet id.
    pub packet_id: String,
    /// User-facing title.
    pub title: String,
    /// Export-safe summary.
    pub summary: String,
    /// Primary surface for this packet.
    pub surface: TransferSurfaceClass,
    /// Transfer action class.
    pub action: TransferActionClass,
    /// Representation truth.
    pub representation: RepresentationTruth,
    /// Sensitive review state, when applicable.
    pub sensitive_review: Option<SensitiveReview>,
    /// Boundary context, when applicable.
    pub boundary_context: Option<BoundaryContext>,
    /// Paste guardrail state, when applicable.
    pub paste_guardrail: Option<PasteGuardrail>,
    /// Drop preview state, when applicable.
    pub drop_preview: Option<DropPreview>,
    /// Undo-group truth, when this action mutates user-visible state.
    pub undo_group: Option<UndoGroupTruth>,
    /// Large transfer feedback, when applicable.
    pub large_transfer: Option<LargeTransferFeedback>,
    /// Rich-content trust state.
    pub rich_content: RichContentTrust,
    /// Surface projections that consume this packet.
    pub surface_projections: Vec<SurfaceProjection>,
    /// Support-safe refs exported with the packet.
    pub support_export_refs: Vec<String>,
}

/// Top-level governed transfer-safety record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSafetyPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// User-facing title.
    pub title: String,
    /// Export-safe summary.
    pub summary: String,
    /// Primary surface for this packet.
    pub surface: TransferSurfaceClass,
    /// Transfer action class.
    pub action: TransferActionClass,
    /// Representation truth.
    pub representation: RepresentationTruth,
    /// Sensitive review state, when applicable.
    pub sensitive_review: Option<SensitiveReview>,
    /// Boundary context, when applicable.
    pub boundary_context: Option<BoundaryContext>,
    /// Paste guardrail state, when applicable.
    pub paste_guardrail: Option<PasteGuardrail>,
    /// Drop preview state, when applicable.
    pub drop_preview: Option<DropPreview>,
    /// Undo-group truth, when this action mutates user-visible state.
    pub undo_group: Option<UndoGroupTruth>,
    /// Large transfer feedback, when applicable.
    pub large_transfer: Option<LargeTransferFeedback>,
    /// Rich-content trust state.
    pub rich_content: RichContentTrust,
    /// Cross-surface projections that consume this packet.
    pub surface_projections: Vec<SurfaceProjection>,
    /// Support-safe refs exported with the packet.
    pub support_export_refs: Vec<String>,
}

/// Reasons a [`TransferSafetyPacket`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// Plain-text copy is not preserved as the default or as a reachable action.
    PlainTextRepresentationMissing,
    /// Rendered/rich copy hides raw or escaped alternatives.
    RichCopyHidesSource,
    /// Sensitive or boundary-crossing content lacks a pre-commit review.
    SensitiveReviewMissing,
    /// Remote or support boundary is not visible before commit.
    BoundaryLabelMissing,
    /// Multiline or high-risk paste lacks confirmation, bracketed paste, or submit guardrails.
    PasteGuardrailMissing,
    /// Drag/drop does not expose the resulting verb, indicator, modifiers, or keyboard route.
    DropPreviewAmbiguous,
    /// Mutating transfer lacks named undo group or recovery lineage.
    UndoGroupMissing,
    /// Large transfer lacks progress, cancellation, or completion summary.
    LargeTransferFeedbackMissing,
    /// Rich content lacks raw/plain alternatives or active-content guardrails.
    RichContentTrustMissing,
    /// Packet is not projected to a consuming surface.
    SurfaceProjectionMissing,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PlainTextRepresentationMissing => write!(
                f,
                "default copy must preserve useful plain text or expose a reachable raw/escaped alternative"
            ),
            Self::RichCopyHidesSource => write!(
                f,
                "rich or rendered copy must not hide source, escaped, or plain-text alternatives"
            ),
            Self::SensitiveReviewMissing => write!(
                f,
                "sensitive or boundary-crossing transfer must show review before commit"
            ),
            Self::BoundaryLabelMissing => {
                write!(f, "boundary-crossing transfers must show boundary before commit")
            }
            Self::PasteGuardrailMissing => write!(
                f,
                "multiline or high-risk paste must require confirmation, bracketed paste, and disabled auto-submit"
            ),
            Self::DropPreviewAmbiguous => write!(
                f,
                "drop targets must show verb, insertion indicator, modifier cues, and keyboard fallback"
            ),
            Self::UndoGroupMissing => write!(
                f,
                "user-visible transfer mutations must register named undo groups with recovery lineage"
            ),
            Self::LargeTransferFeedbackMissing => write!(
                f,
                "large transfers must show progress, cancellation, and a post-action summary"
            ),
            Self::RichContentTrustMissing => write!(
                f,
                "rich content must expose trust posture, raw inspection, and plain-text copy"
            ),
            Self::SurfaceProjectionMissing => write!(
                f,
                "transfer-safety packet must have at least one shared-record surface projection"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl TransferSafetyPacket {
    /// Builds a governed transfer-safety packet from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a packet that hides
    /// representation truth, local/remote boundary state, drag/drop outcome,
    /// paste risk, undo lineage, or large-transfer progress.
    pub fn build(input: TransferSafetyInput) -> Result<Self, BuildError> {
        let packet = Self {
            record_kind: TRANSFER_SAFETY_PACKET_RECORD_KIND.to_string(),
            schema_version: TRANSFER_SAFETY_SCHEMA_VERSION,
            schema_ref: TRANSFER_SAFETY_SCHEMA_REF.to_string(),
            packet_id: input.packet_id,
            title: input.title,
            summary: input.summary,
            surface: input.surface,
            action: input.action,
            representation: input.representation,
            sensitive_review: input.sensitive_review,
            boundary_context: input.boundary_context,
            paste_guardrail: input.paste_guardrail,
            drop_preview: input.drop_preview,
            undo_group: input.undo_group,
            large_transfer: input.large_transfer,
            rich_content: input.rich_content,
            surface_projections: input.surface_projections,
            support_export_refs: input.support_export_refs,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Returns true when this packet satisfies all transfer-safety invariants.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }

    /// Returns every invariant violation found in this packet.
    pub fn contract_findings(&self) -> Vec<BuildError> {
        let mut findings = Vec::new();
        if !self.plain_text_preserved() {
            findings.push(BuildError::PlainTextRepresentationMissing);
        }
        if self.representation.rendered_copy_available
            && !(self.representation.raw_copy_available
                || self.representation.escaped_copy_available
                || self.representation.default_representation.is_plain_text())
        {
            findings.push(BuildError::RichCopyHidesSource);
        }
        if self.requires_sensitive_review() && !self.has_sensitive_review() {
            findings.push(BuildError::SensitiveReviewMissing);
        }
        if self.requires_boundary_label() && !self.has_boundary_label() {
            findings.push(BuildError::BoundaryLabelMissing);
        }
        if matches!(self.action, TransferActionClass::Paste) && !self.has_paste_guardrail() {
            findings.push(BuildError::PasteGuardrailMissing);
        }
        if matches!(
            self.action,
            TransferActionClass::DragDrop
                | TransferActionClass::Move
                | TransferActionClass::Split
                | TransferActionClass::Attach
                | TransferActionClass::Import
        ) && !self.has_drop_preview()
        {
            findings.push(BuildError::DropPreviewAmbiguous);
        }
        if self.is_mutating_action() && !self.has_named_undo_group() {
            findings.push(BuildError::UndoGroupMissing);
        }
        if self.requires_large_transfer_feedback() && !self.has_large_transfer_feedback() {
            findings.push(BuildError::LargeTransferFeedbackMissing);
        }
        if !self.has_rich_content_trust() {
            findings.push(BuildError::RichContentTrustMissing);
        }
        if !self.has_surface_projection() {
            findings.push(BuildError::SurfaceProjectionMissing);
        }
        findings
    }

    fn validate(&self) -> Result<(), BuildError> {
        if let Some(finding) = self.contract_findings().into_iter().next() {
            Err(finding)
        } else {
            Ok(())
        }
    }

    fn plain_text_preserved(&self) -> bool {
        self.representation.default_plain_text_preserved
            && (self.representation.default_representation.is_plain_text()
                || self.representation.raw_copy_available
                || self.representation.escaped_copy_available)
    }

    fn requires_sensitive_review(&self) -> bool {
        self.sensitive_review
            .as_ref()
            .is_some_and(|review| review.boundary_crossing || !review.content_classes.is_empty())
    }

    fn has_sensitive_review(&self) -> bool {
        self.sensitive_review.as_ref().is_some_and(|review| {
            review.preview_before_commit
                && review.policy_gate_present
                && !review.visible_label.trim().is_empty()
        })
    }

    fn requires_boundary_label(&self) -> bool {
        self.boundary_context.is_some()
            || self
                .sensitive_review
                .as_ref()
                .is_some_and(|review| review.boundary_crossing)
    }

    fn has_boundary_label(&self) -> bool {
        self.boundary_context.as_ref().is_some_and(|boundary| {
            boundary.shown_before_commit && !boundary.visible_boundary_label.trim().is_empty()
        })
    }

    fn has_paste_guardrail(&self) -> bool {
        self.paste_guardrail.as_ref().is_some_and(|guardrail| {
            guardrail.line_count > 1
                && guardrail.bracketed_paste_available
                && guardrail.automatic_submit_disabled
                && guardrail.confirmation_required
                && !guardrail.review_summary.trim().is_empty()
        })
    }

    fn has_drop_preview(&self) -> bool {
        self.drop_preview.as_ref().is_some_and(|preview| {
            preview.insertion_indicator_visible
                && preview.modifier_cues_visible
                && preview.keyboard_route_available
                && !preview.command_fallback_id.trim().is_empty()
        })
    }

    fn is_mutating_action(&self) -> bool {
        matches!(
            self.action,
            TransferActionClass::Paste
                | TransferActionClass::DragDrop
                | TransferActionClass::Import
                | TransferActionClass::Attach
                | TransferActionClass::Move
                | TransferActionClass::Split
                | TransferActionClass::MultiFileReplace
                | TransferActionClass::AiApply
                | TransferActionClass::SettingsImport
        )
    }

    fn has_named_undo_group(&self) -> bool {
        self.undo_group.as_ref().is_some_and(|undo| {
            undo.named
                && !undo.group_label.trim().is_empty()
                && !undo.source_attribution.trim().is_empty()
                && !undo.mutation_journal_entry_ref.trim().is_empty()
                && !undo.history_surfaces.is_empty()
        })
    }

    fn requires_large_transfer_feedback(&self) -> bool {
        self.large_transfer.is_some()
    }

    fn has_large_transfer_feedback(&self) -> bool {
        self.large_transfer.as_ref().is_some_and(|feedback| {
            feedback.progress_visible
                && feedback.cancellation_available
                && feedback.post_action_summary_present
                && !feedback.result_summary.trim().is_empty()
        })
    }

    fn has_rich_content_trust(&self) -> bool {
        match self.rich_content.trust_class {
            RichTrustClass::RawText => self.rich_content.copy_plain_text_available,
            RichTrustClass::SanitizedRich
            | RichTrustClass::TrustedLocalActive
            | RichTrustClass::IsolatedRemoteActive => {
                self.rich_content.active_content_blocked_or_isolated
                    && self.rich_content.raw_source_available
                    && self.rich_content.copy_plain_text_available
            }
        }
    }

    fn has_surface_projection(&self) -> bool {
        !self.surface_projections.is_empty()
            && self
                .surface_projections
                .iter()
                .all(|projection| projection.reads_shared_record)
    }
}
