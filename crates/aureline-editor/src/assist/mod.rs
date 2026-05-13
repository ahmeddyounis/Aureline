//! Editor assistance contracts for completion, signature help, and snippets.
//!
//! This module is the editor-owned consumer of language-router provenance for
//! typing assistance. It keeps completion items, signature-help cards, and
//! snippet sessions source-labeled, export-safe, and inspectable under fallback
//! or degraded provider conditions.

mod records;

pub use records::{
    AssistContractError, AssistSchemaVersion, AssistSessionStore, AssistSourceCounts,
    AssistSourceDescriptor, AssistSourceFamily, AssistSurfaceSnapshot,
    AssistSurfaceSnapshotRequest, AssistSurfaceStateClass, CompletionAcceptanceContract,
    CompletionItemInit, CompletionItemKindClass, CompletionItemRecord, CompletionListRequest,
    CompletionListSnapshot, CompletionSideEffectClass, SignatureHelpInit, SignatureHelpRecord,
    SignaturePlacementClass, SnippetKeyIntentClass, SnippetKeyOutcomeClass,
    SnippetKeyOutcomeRecord, SnippetSessionController, SnippetSessionInit, SnippetSessionRecord,
    SnippetSessionStateClass, SnippetTabBehaviorClass, SnippetUnrelatedKeyPolicyClass,
    ASSIST_SCHEMA_VERSION,
};
