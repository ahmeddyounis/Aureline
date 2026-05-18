//! Editor assistance contracts for completion, signature help, snippets, and quick-fix previews.
//!
//! This module is the editor-owned consumer of language-router provenance for
//! typing assistance. It keeps completion items, signature-help cards, and
//! snippet sessions source-labeled, export-safe, and inspectable under fallback
//! or degraded provider conditions.

mod code_action_preview;
mod records;

pub use code_action_preview::{
    CodeActionPreviewDecisionClass, CodeActionPreviewRecord, CodeActionPreviewRequest,
    CodeActionPreviewSchemaVersion, QuickFixEvidenceTrustClass, CODE_ACTION_PREVIEW_SCHEMA_VERSION,
};
pub use records::{
    AssistContractError, AssistSchemaVersion, AssistSessionStore, AssistSourceCounts,
    AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass,
    AssistSourceLabelProjection, AssistSurfaceSnapshot, AssistSurfaceSnapshotRequest,
    AssistSurfaceStateClass, CompletionAcceptanceContract, CompletionItemInit,
    CompletionItemKindClass, CompletionItemRecord, CompletionListRequest, CompletionListSnapshot,
    CompletionSideEffectClass, SignatureHelpInit, SignatureHelpRecord, SignaturePlacementClass,
    SnippetCursorPostureClass, SnippetImePostureClass, SnippetKeyIntentClass,
    SnippetKeyOutcomeClass, SnippetKeyOutcomeRecord, SnippetSessionController, SnippetSessionInit,
    SnippetSessionRecord, SnippetSessionStateClass, SnippetTabBehaviorClass,
    SnippetUnrelatedKeyPolicyClass, ASSIST_SCHEMA_VERSION,
};
