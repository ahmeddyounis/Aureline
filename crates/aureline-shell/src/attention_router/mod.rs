//! Governed attention routing across durable attention surfaces.
//!
//! This module is the single governed routing system the shell uses to turn a
//! typed [`crate::notifications::envelope::NotificationEnvelope`] into one
//! [`NotificationRouteOutcome`] that reasons about the same alert consistently
//! across in-app toasts, banners, status overflow, the activity center, native
//! OS notifications, and companion fanout — instead of letting each surface
//! mint its own alert behavior.
//!
//! It is a thin governance layer over the existing notification primitives in
//! [`crate::notifications`]:
//!
//! - [`context`] carries the shell's *live* channel facts ([`ChannelContext`])
//!   — active window state, screen-reader posture, companion availability, and
//!   presentation/follow state — that the envelope cannot know at mint time.
//! - [`outcome`] holds the [`AttentionRouter`] engine and the single
//!   [`NotificationRouteOutcome`] object. The engine folds the live context
//!   into an effective quiet posture, routes through the dedupe + suppression
//!   core ([`crate::notifications::router::NotificationRouter`]), then narrows
//!   each surface against the live channel state without ever widening
//!   authority or upgrading a held / suppressed / deduped surface.
//! - [`lifecycle`] describes the six governed user verbs — dismiss, snooze,
//!   acknowledge, mute, clear, and resolve — with their badge, retention, and
//!   export semantics.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and the headless inspector share.
//!
//! The canonical boundary object is frozen at
//! `schemas/ux/notification_route_outcome.schema.json` and documented in
//! `docs/ux/m3/notification_envelope_beta_contract.md`.

pub mod context;
pub mod corpus;
pub mod lifecycle;
pub mod outcome;

pub use context::{
    ActiveWindowState, ChannelContext, ChannelContextSnapshot, CompanionAvailability,
    PresentationFollowState, ScreenReaderPosture,
};
pub use corpus::{
    seeded_attention_routing_corpus, validate_attention_routing_corpus,
    AttentionRouteSupportExport, AttentionRouteSupportExportRow, AttentionRoutingCase,
    AttentionRoutingCorpus, AttentionRoutingCorpusSummary, SupportSurfaceResolution,
    ATTENTION_ROUTER_BETA_SCHEMA_VERSION, ATTENTION_ROUTER_BETA_SHARED_CONTRACT_REF,
    ATTENTION_ROUTE_SUPPORT_EXPORT_RECORD_KIND, ATTENTION_ROUTE_SUPPORT_EXPORT_ROW_RECORD_KIND,
    ATTENTION_ROUTING_CASE_RECORD_KIND, ATTENTION_ROUTING_CORPUS_RECORD_KIND,
};
pub use lifecycle::{
    governed_user_actions, AvailableLifecycleAction, LifecycleBadgeEffect, LifecycleExportEffect,
    LifecycleRetentionEffect, GOVERNED_USER_ACTIONS,
};
pub use outcome::{
    AttentionRouter, ChannelResolutionClass, CompanionHandoffClass, CompanionHandoffPosture,
    NotificationRouteOutcome, ResolvedSurfaceRoute, NOTIFICATION_ROUTE_OUTCOME_RECORD_KIND,
    NOTIFICATION_ROUTE_OUTCOME_SCHEMA_VERSION,
};

#[cfg(test)]
mod tests;
