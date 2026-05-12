//! AI composer and context-inspector seed for the bounded launch AI wedge.
//!
//! This crate is the M1 seed for the AI composer lane. It owns one
//! inspectable [`composer::ComposerDraft`] object plus the typed
//! mention / attachment / slash-command / route-placeholder vocabularies that
//! every consuming surface reads. The composer is a **bounded prototype**:
//! it explicitly carries no mutation authority, dispatches nothing, and
//! reuses the shared canonical command identifiers (from
//! [`aureline_commands`]) and the shared execution-context object
//! (from [`aureline_runtime`]) instead of forking AI-only truth.
//!
//! Surfaces (shell AI-context inspector, support / export flows, evidence
//! consumers) project this draft through typed accessors and never re-derive
//! mention or attachment authority locally.
//!
//! The reviewer-facing landing page is
//! [`/docs/ai/m1_composer_and_context_inspector_seed.md`](../../../docs/ai/m1_composer_and_context_inspector_seed.md).
//! The frozen cross-tool contracts the seed projects against are
//! [`/docs/ai/prompt_composer_contract.md`](../../../docs/ai/prompt_composer_contract.md)
//! and
//! [`/docs/ai/context_assembly_contract.md`](../../../docs/ai/context_assembly_contract.md).
//! The seed deliberately covers a small, honest subset of those
//! vocabularies — enough for one bounded protected row in the live shell —
//! and grows additively without forking truth.

#![doc(html_root_url = "https://docs.rs/aureline-ai/0.0.0")]

pub mod composer;

pub use composer::{
    AttachmentKind, AttachmentStatusClass, BlockReason, ComposerAttachment, ComposerDraft,
    ComposerDraftState, ComposerMention, ComposerSlashCommandInvocation, DispatchTargetClass,
    MentionKind, MentionResolutionState, PrototypeLabel, ProviderClass, RoutePathClass,
    RoutePlaceholder, SelectionReasonClass, SlashCommandResolutionState, SourceClass, TrustPosture,
    ValidationOutcome, COMPOSER_DRAFT_RECORD_KIND, COMPOSER_DRAFT_SCHEMA_VERSION,
};
