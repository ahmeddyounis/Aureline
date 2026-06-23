//! M5 *attention-action semantics*: the working engine that applies the distinct
//! dismiss, snooze, acknowledge, mute, and resolve actions to a durable attention
//! object and computes exactly what each one means for retention, the badge, exact
//! reopen continuity, cross-client fanout, support export, and audit history.
//!
//! Where [`m5_attention_routing`](crate::m5_attention_routing) *names and freezes
//! the contract* — including the action/retention-semantics object family — and
//! [`m5_envelope_routing`](crate::m5_envelope_routing) *routes a fresh envelope to
//! its surfaces*, this lane *implements what happens after a person acts on it*. The
//! five actions are not one generic "close": they carry distinct retention,
//! distinct badge behavior, and distinct resume and audit meaning, and none of them
//! erases the durable record or reissues the original side effect.
//!
//! [`apply_attention_action`] is a pure function of an [`AttentionItem`] (a durable
//! attention object that already exists in the activity center) and an
//! [`AttentionActionClass`]. It returns an [`AttentionActionOutcome`] that records
//! the resulting lifecycle state, the retention class, the badge effect and exact
//! count delta, the resume condition (for snooze and mute), the per-surface
//! propagation across the activity center, badge, OS notification, companions, and
//! operator dashboard, and a short reviewable support-export sentence — so the same
//! `(item, action)` yields the same outcome byte-for-byte in support export and
//! CLI/headless diagnostics. The honesty rules the track invariant requires are
//! enforced, not just described:
//!
//! - **Distinct semantics, never one generic close.** Each action carries a unique
//!   `(resulting_state, badge_effect, resume, scope)` signature
//!   (`action.semantics_distinct`, `action.badge_effects_distinct`).
//! - **Clearing a badge never erases the record.** Every action keeps the underlying
//!   durable record (`action.keeps_underlying_record`).
//! - **Exact reopen continuity.** Every outcome reopens the same authoritative
//!   target through the same anchor and the same stable action target as its source
//!   item — never a reissued blind side effect
//!   (`action.exact_reopen_continuity`, `action.no_side_effect_replay`).
//! - **One shared action model across surfaces.** The activity center, badge, OS
//!   notification, companions, and operator dashboard reflect the same resulting
//!   state and action target rather than inventing local variants
//!   (`action.surface_parity`).
//! - **Suppression stays separate from audit history.** Snooze and mute record their
//!   deferral in a separate ledger; every action is audit-append-only and never
//!   overwrites history (`action.suppression_separate_from_audit`).
//! - **A security advisory cannot be silenced.** It can only be acknowledged or
//!   resolved, never muted, dismissed, or snoozed (`action.security_not_silenceable`).
//!
//! The canonical [`attention_actions_bundle`] freezes the five action definitions, a
//! representative corpus of durable attention items, and every applied outcome so the
//! freeze gate and checked-in fixture pin the contract byte-for-byte. Every action
//! token, resulting state, retention class, and reopen target binds back to the
//! attention-routing matrix (`action.matrix_bound`), so the action semantics can
//! never drift from the frozen object model.
//!
//! The record carries no message bodies, credentials, raw provider payloads,
//! hostnames, or absolute paths — only opaque object refs, stable tokens, and short
//! reviewable sentences — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{
    all_unique, attention_routing_matrix, is_export_safe_ref, AttentionObjectClass,
    AttentionRoutingMatrix, AttentionScopeClass, AttentionStateClass, FanoutChannelClass,
    NotificationPrivacyClass, ReopenTargetClass, M5_ATTENTION_ROUTING_MATRIX_ID,
};
use crate::m5_envelope_routing::SourceSubsystemClass;

#[cfg(test)]
mod tests;

/// Schema version for the attention-actions bundle.
pub const M5_ATTENTION_ACTIONS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the attention-actions bundle.
pub const M5_ATTENTION_ACTIONS_SCHEMA_REF: &str =
    "schemas/activity/m5-attention-actions.schema.json";

/// Stable record-kind tag for the attention-actions bundle.
pub const M5_ATTENTION_ACTIONS_RECORD_KIND: &str = "m5_attention_actions_bundle";

/// Stable id for the canonical attention-actions bundle.
pub const M5_ATTENTION_ACTIONS_BUNDLE_ID: &str = "m5-attention-actions:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ATTENTION_ACTIONS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The attention-routing matrix fixture this lane binds its vocabulary back to.
pub const M5_ATTENTION_ACTIONS_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

/// The freeze gate that keeps the bundle current. Stable promotion runs this gate;
/// it fails when the in-code bundle drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_ATTENTION_ACTIONS_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_attention_actions.rs";

// ---------------------------------------------------------------------------
// Action vocabulary.
// ---------------------------------------------------------------------------

/// The closed set of distinct attention actions a person can take on a durable
/// attention object.
///
/// These are exactly the `action_semantics` tokens the attention-routing matrix
/// freezes; they are kept distinct rather than collapsed into a single generic
/// "close", because each implies a different retention, badge, resume, and audit
/// meaning. Adding one is a breaking change to the action grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionActionClass {
    /// Clear the item from the active badge while keeping the durable record; the
    /// underlying event is neither read nor resolved.
    Dismiss,
    /// Defer the item with a resume condition; it leaves the badge now and returns
    /// automatically when the condition fires.
    Snooze,
    /// Mark the item read and clear the badge; the underlying work stays open and
    /// durable until it resolves.
    Acknowledge,
    /// Suppress the source from the badge and out-of-window fanout until it is
    /// unmuted; events still accrue durably.
    Mute,
    /// Close the item because its underlying object changed or the user marked it
    /// done; it is retained as resolved history.
    Resolve,
}

impl AttentionActionClass {
    /// All actions, in grammar order.
    pub const ALL: [Self; 5] = [
        Self::Dismiss,
        Self::Snooze,
        Self::Acknowledge,
        Self::Mute,
        Self::Resolve,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dismiss => "dismiss",
            Self::Snooze => "snooze",
            Self::Acknowledge => "acknowledge",
            Self::Mute => "mute",
            Self::Resolve => "resolve",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dismiss => "Dismiss",
            Self::Snooze => "Snooze",
            Self::Acknowledge => "Acknowledge",
            Self::Mute => "Mute",
            Self::Resolve => "Resolve",
        }
    }

    /// Whether this action silences a source or item rather than reading or resolving
    /// it. A security advisory may never be silenced.
    pub const fn is_silencing(self) -> bool {
        matches!(self, Self::Dismiss | Self::Snooze | Self::Mute)
    }
}

/// How an action changes the badge — distinct per action so the five never collapse
/// into one generic clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeEffectClass {
    /// Drop the item from the count while keeping the durable record (dismiss).
    ClearKeepRecord,
    /// Drop the item from the count by marking it read (acknowledge).
    ClearMarkRead,
    /// Drop the item from the count until its resume condition returns it (snooze).
    ClearUntilResume,
    /// Drop the item and suppress future items from its source until unmuted (mute).
    ClearAndSuppressSource,
    /// Drop the item because the underlying object resolved (resolve).
    ClearOnResolve,
}

impl BadgeEffectClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearKeepRecord => "clear_keep_record",
            Self::ClearMarkRead => "clear_mark_read",
            Self::ClearUntilResume => "clear_until_resume",
            Self::ClearAndSuppressSource => "clear_and_suppress_source",
            Self::ClearOnResolve => "clear_on_resolve",
        }
    }
}

/// Whether an action's effect is scoped to the single item or the whole source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionEffectScopeClass {
    /// The action applies to this one attention item.
    ThisItem,
    /// The action applies to every present and future item from this source.
    ThisSource,
}

impl ActionEffectScopeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThisItem => "this_item",
            Self::ThisSource => "this_source",
        }
    }
}

/// The kind of resume condition an action carries, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeConditionKindClass {
    /// No resume condition — the action does not defer.
    None,
    /// Returns when a snooze timer or its named predicate fires.
    TimerOrPredicate,
    /// Stays suppressed until the source is explicitly unmuted.
    UntilUnmuted,
}

impl ResumeConditionKindClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::TimerOrPredicate => "timer_or_predicate",
            Self::UntilUnmuted => "until_unmuted",
        }
    }
}

/// How an action propagates to one fanout surface — every variant reflects the same
/// resulting state and action target, and none replays a side effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceActionPropagationClass {
    /// The in-app activity center applies the action to the durable record; it is the
    /// authoritative surface and transitions state without reissuing the original
    /// notification.
    ApplyAuthoritative,
    /// The dock/taskbar badge drops this item's contribution from the deduped count.
    ClearCount,
    /// The OS notification is withdrawn without replaying any side effect.
    WithdrawNoReplay,
    /// A companion or operator mirror reflects the new state from the same action
    /// target and never re-executes the action.
    ReflectStateNoReplay,
}

impl SurfaceActionPropagationClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApplyAuthoritative => "apply_authoritative",
            Self::ClearCount => "clear_count",
            Self::WithdrawNoReplay => "withdraw_no_replay",
            Self::ReflectStateNoReplay => "reflect_state_no_replay",
        }
    }

    /// Whether this propagation re-executes the original action's side effect (always
    /// false — propagation reflects state, it never replays).
    pub const fn replays_side_effect(self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Action definition (the static semantics of each action).
// ---------------------------------------------------------------------------

/// The frozen, distinct semantics of one attention action, independent of the item
/// it is applied to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionActionDef {
    /// The action.
    pub action: AttentionActionClass,
    /// Stable, namespaced action id.
    pub action_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the action.
    pub summary: String,
    /// The lifecycle state the durable object enters after the action.
    pub resulting_state: AttentionStateClass,
    /// The retention class token (matrix `retention_classes` vocabulary).
    pub retention_class: String,
    /// The badge effect.
    pub badge_effect: BadgeEffectClass,
    /// Whether the action clears the badge (always true — every action removes the
    /// item from the active badge).
    pub clears_badge: bool,
    /// Whether the underlying durable record is kept (always true — clearing a badge
    /// never erases the record).
    pub keeps_underlying_record: bool,
    /// Whether the action defers with a resume condition (snooze and mute).
    pub requires_resume_condition: bool,
    /// The kind of resume condition.
    pub resume_condition_kind: ResumeConditionKindClass,
    /// The scope of the action's effect.
    pub effect_scope: ActionEffectScopeClass,
    /// Whether the action records a deferral/suppression marker stored separately
    /// from audit history (snooze and mute).
    pub creates_separate_suppression_state: bool,
    /// Whether acting on the object appends an immutable audit event and never
    /// overwrites history (always true).
    pub audit_append_only: bool,
    /// Whether the object stays reopenable after the action (always true).
    pub reopenable_after: bool,
    /// Whether the action is reversible (dismiss/snooze/acknowledge/mute reverse
    /// directly; resolve reopens the resolved object).
    pub reversible: bool,
    /// Whether the action replays the original side effect (always false).
    pub replays_side_effects: bool,
    /// One reviewable sentence safe for support export.
    pub support_export_note: String,
    /// One reviewable sentence stating the action's honesty rule.
    pub boundary_note: String,
}

/// Returns the frozen semantics for an action.
pub fn action_definition(action: AttentionActionClass) -> AttentionActionDef {
    use AttentionActionClass::*;
    let (
        resulting_state,
        retention_class,
        badge_effect,
        requires_resume_condition,
        resume_condition_kind,
        effect_scope,
        creates_separate_suppression_state,
    ) = match action {
        Dismiss => (
            AttentionStateClass::Dismissed,
            "durable_until_archived",
            BadgeEffectClass::ClearKeepRecord,
            false,
            ResumeConditionKindClass::None,
            ActionEffectScopeClass::ThisItem,
            false,
        ),
        Snooze => (
            AttentionStateClass::Snoozed,
            "suppression_state_separate",
            BadgeEffectClass::ClearUntilResume,
            true,
            ResumeConditionKindClass::TimerOrPredicate,
            ActionEffectScopeClass::ThisItem,
            true,
        ),
        Acknowledge => (
            AttentionStateClass::Acknowledged,
            "durable_until_resolved",
            BadgeEffectClass::ClearMarkRead,
            false,
            ResumeConditionKindClass::None,
            ActionEffectScopeClass::ThisItem,
            false,
        ),
        Mute => (
            AttentionStateClass::Suppressed,
            "suppression_state_separate",
            BadgeEffectClass::ClearAndSuppressSource,
            true,
            ResumeConditionKindClass::UntilUnmuted,
            ActionEffectScopeClass::ThisSource,
            true,
        ),
        Resolve => (
            AttentionStateClass::Resolved,
            "durable_until_archived",
            BadgeEffectClass::ClearOnResolve,
            false,
            ResumeConditionKindClass::None,
            ActionEffectScopeClass::ThisItem,
            false,
        ),
    };

    let summary = match action {
        Dismiss => "Clear the item from the active badge while keeping the durable record; the \
                    underlying event is neither read-acknowledged nor resolved, and the object stays \
                    reopenable.",
        Snooze => "Defer the item with a resume condition; it leaves the active badge now and \
                   returns automatically when the condition fires, with the durable record untouched.",
        Acknowledge => "Mark the item read and clear the badge; the underlying work stays open and \
                        durable until it resolves, and the object stays reopenable.",
        Mute => "Suppress this source from the badge and out-of-window fanout until it is unmuted; \
                 existing and future events stay durable but do not raise the badge.",
        Resolve => "Close the item because its underlying object changed or the user marked it done; \
                    it leaves the badge and is retained as resolved history, still reopenable.",
    };

    let support_export_note = match action {
        Dismiss => {
            "Dismissed from the active badge; the durable record and its reopen route are \
                    unchanged, and no side effect was replayed."
        }
        Snooze => {
            "Snoozed until its resume condition; the deferral is recorded separately from \
                   audit history, the durable record and reopen route are unchanged, and no side \
                   effect was replayed."
        }
        Acknowledge => {
            "Acknowledged (marked read); the underlying item remains open and durable, \
                        the reopen route is unchanged, and no side effect was replayed."
        }
        Mute => {
            "Muted at source until unmuted; the suppression is recorded separately from audit \
                 history, durable records still accrue and stay reopenable, and no side effect was \
                 replayed."
        }
        Resolve => {
            "Resolved on the underlying change; the resolved record is retained in history \
                    and stays reopenable, and no side effect was replayed."
        }
    };

    let boundary_note = match action {
        Dismiss => {
            "Dismiss clears the badge only; it never erases the durable record or reissues \
                    the original action."
        }
        Snooze => {
            "Snooze always names a resume condition and stores the deferral separately from \
                   audit history; it never drops the durable record."
        }
        Acknowledge => {
            "Acknowledge marks read without resolving; the underlying work stays open and \
                        durable."
        }
        Mute => {
            "Mute suppresses the source separately from audit history; muted events still accrue \
                 durably and a security advisory can never be muted."
        }
        Resolve => {
            "Resolve closes on the underlying change and retains the resolved record; it is \
                    reopenable and never replays a side effect."
        }
    };

    AttentionActionDef {
        action,
        action_id: format!("attention_action.{}", action.as_str()),
        label: action.label().to_owned(),
        summary: summary.to_owned(),
        resulting_state,
        retention_class: retention_class.to_owned(),
        badge_effect,
        clears_badge: true,
        keeps_underlying_record: true,
        requires_resume_condition,
        resume_condition_kind,
        effect_scope,
        creates_separate_suppression_state,
        audit_append_only: true,
        reopenable_after: true,
        reversible: true,
        replays_side_effects: false,
        support_export_note: support_export_note.to_owned(),
        boundary_note: boundary_note.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Attention item (the durable object an action is applied to).
// ---------------------------------------------------------------------------

/// A durable attention object already present in the activity center, against which
/// an action is applied.
///
/// This is the working record behind the durable items the matrix names; it carries
/// the stable identity, reopen route, and badge contribution an action needs to
/// compute its outcome, but never raw payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionItem {
    /// Stable, namespaced item id.
    pub item_id: String,
    /// Human-readable label.
    pub label: String,
    /// The attention object family this item belongs to.
    pub object_class: AttentionObjectClass,
    /// The source subsystem that produced the item.
    pub source_subsystem: SourceSubsystemClass,
    /// The scope namespace the item applies to.
    pub scope: AttentionScopeClass,
    /// The privacy class governing what may be shown, mirrored, or exported.
    pub privacy_class: NotificationPrivacyClass,
    /// The lifecycle state the item is in before the action.
    pub prior_state: AttentionStateClass,
    /// The authoritative object the item reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref (never a URL, host, or path).
    pub reopen_anchor_ref: String,
    /// The stable action-target id every surface uses to act on the item.
    pub action_target_id: String,
    /// The active badge count that includes this item, before the action.
    pub badge_count_before: u32,
    /// This item's contribution to the deduped badge count.
    pub badge_contribution: u32,
    /// The surfaces this item fans out to (always includes the activity center and
    /// the badge).
    pub fanout_surfaces: Vec<FanoutChannelClass>,
    /// The actions this item supports. A security advisory supports only acknowledge
    /// and resolve.
    pub supported_actions: Vec<AttentionActionClass>,
    /// Evaluation stamp.
    pub created_at: String,
}

impl AttentionItem {
    /// Whether the item supports an action.
    pub fn supports(&self, action: AttentionActionClass) -> bool {
        self.supported_actions.contains(&action)
    }
}

// ---------------------------------------------------------------------------
// Per-surface propagation and applied outcome.
// ---------------------------------------------------------------------------

/// How an action propagates to one fanout surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceActionPropagation {
    /// The fanout surface.
    pub surface: FanoutChannelClass,
    /// Stable, namespaced surface id.
    pub surface_id: String,
    /// How the action propagates here.
    pub propagation: SurfaceActionPropagationClass,
    /// The stable action target id reflected here (identical on every surface).
    pub reflects_action_target_id: String,
    /// Whether this surface replays a side effect (always false).
    pub replays_side_effect: bool,
    /// One reviewable sentence describing the propagation.
    pub note: String,
}

/// The outcome of applying one action to one attention item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionActionOutcome {
    /// Stable, namespaced outcome id.
    pub outcome_id: String,
    /// The item the action was applied to.
    pub item_id: String,
    /// The action applied.
    pub action: AttentionActionClass,
    /// The lifecycle state before the action.
    pub prior_state: AttentionStateClass,
    /// The lifecycle state after the action.
    pub resulting_state: AttentionStateClass,
    /// The retention class token applied.
    pub retention_class: String,
    /// The badge effect.
    pub badge_effect: BadgeEffectClass,
    /// The active badge count before the action.
    pub badge_count_before: u32,
    /// The active badge count after the action.
    pub badge_count_after: u32,
    /// The change to the badge count (always non-positive — actions clear, never add).
    pub badge_delta: i64,
    /// Whether the underlying durable record is kept (always true).
    pub keeps_underlying_record: bool,
    /// Whether the action records a deferral/suppression marker stored separately
    /// from audit history.
    pub creates_separate_suppression_state: bool,
    /// Whether acting appends an immutable audit event and never overwrites history.
    pub audit_append_only: bool,
    /// The resume condition, present iff the action defers (snooze and mute).
    pub resume_condition: Option<String>,
    /// The authoritative object the outcome reopens (identical to the item's).
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref (identical to the item's — exact reopen
    /// continuity).
    pub reopen_anchor_ref: String,
    /// The stable action target id (identical to the item's — never reissued).
    pub action_target_id: String,
    /// Whether the outcome reopens the same authoritative object through the same
    /// anchor and action target as its source item.
    pub reopen_continuity_preserved: bool,
    /// Whether the action replays the original side effect (always false).
    pub replays_side_effects: bool,
    /// The per-surface propagation across the item's fanout surfaces.
    pub surface_propagation: Vec<SurfaceActionPropagation>,
    /// One reviewable sentence safe for support export.
    pub support_export_note: String,
    /// One reviewable sentence explaining the outcome.
    pub reason: String,
}

impl AttentionActionOutcome {
    /// The propagation for a surface, if handled.
    pub fn propagation(&self, surface: FanoutChannelClass) -> Option<&SurfaceActionPropagation> {
        self.surface_propagation
            .iter()
            .find(|p| p.surface == surface)
    }
}

/// The stable propagation kind for a surface. Every non-authoritative surface
/// reflects state without replaying a side effect.
fn surface_propagation_kind(surface: FanoutChannelClass) -> SurfaceActionPropagationClass {
    use FanoutChannelClass::*;
    use SurfaceActionPropagationClass::*;
    match surface {
        InAppActivityCenter => ApplyAuthoritative,
        DockTaskbarBadge => ClearCount,
        OsNativeNotification => WithdrawNoReplay,
        BrowserCompanion | MobileCompanion | OperatorDashboard => ReflectStateNoReplay,
    }
}

fn surface_propagation_note(kind: SurfaceActionPropagationClass) -> String {
    use SurfaceActionPropagationClass::*;
    match kind {
        ApplyAuthoritative => {
            "The in-app activity center applies the action to the durable record \
                               and transitions state without reissuing the original notification."
        }
        ClearCount => {
            "The dock/taskbar badge drops this item's contribution from the deduped \
                       count."
        }
        WithdrawNoReplay => {
            "The OS notification is withdrawn without replaying any side effect; \
                             reopening lands on the in-product record."
        }
        ReflectStateNoReplay => {
            "The companion or operator mirror reflects the new state from the \
                                 same action target and never re-executes the action."
        }
    }
    .to_owned()
}

// ---------------------------------------------------------------------------
// The action engine.
// ---------------------------------------------------------------------------

/// Applies one action to one attention item, deterministically.
///
/// Pure: the same `(item, action)` yields the same [`AttentionActionOutcome`] every
/// call, so an applied action is reproducible in support export and CLI/headless
/// diagnostics. The outcome clears the badge but keeps the durable record, preserves
/// exact reopen continuity (the same authoritative target, anchor, and action
/// target), defers with a resume condition only for snooze and mute, records
/// suppression separately from audit history, and never replays a side effect.
pub fn apply_attention_action(
    item: &AttentionItem,
    action: AttentionActionClass,
) -> AttentionActionOutcome {
    let def = action_definition(action);
    let badge_count_after = item
        .badge_count_before
        .saturating_sub(item.badge_contribution);
    let badge_delta = i64::from(badge_count_after) - i64::from(item.badge_count_before);

    let resume_condition = match def.resume_condition_kind {
        ResumeConditionKindClass::None => None,
        ResumeConditionKindClass::TimerOrPredicate => Some(
            "Returns to the active badge automatically when the snooze timer expires or its named \
             predicate fires."
                .to_owned(),
        ),
        ResumeConditionKindClass::UntilUnmuted => {
            Some("Stays suppressed until the source is explicitly unmuted.".to_owned())
        }
    };

    let surface_propagation = item
        .fanout_surfaces
        .iter()
        .map(|surface| {
            let kind = surface_propagation_kind(*surface);
            SurfaceActionPropagation {
                surface: *surface,
                surface_id: surface.channel_id(),
                propagation: kind,
                reflects_action_target_id: item.action_target_id.clone(),
                replays_side_effect: kind.replays_side_effect(),
                note: surface_propagation_note(kind),
            }
        })
        .collect();

    // Exact reopen continuity: the outcome reopens the same authoritative object,
    // through the same anchor and the same stable action target, as its source item.
    let reopen_target = item.reopen_target;
    let reopen_anchor_ref = item.reopen_anchor_ref.clone();
    let action_target_id = item.action_target_id.clone();
    let reopen_continuity_preserved = reopen_target == item.reopen_target
        && reopen_anchor_ref == item.reopen_anchor_ref
        && action_target_id == item.action_target_id;

    let reason = format!(
        "{} on '{}' transitions it {} \u{2192} {} under {} retention; the badge clears ({} \u{2192} \
         {}), the underlying record is kept, and the reopen route to the {} is preserved without \
         replaying a side effect.",
        def.label,
        item.label,
        item.prior_state.as_str(),
        def.resulting_state.as_str(),
        def.retention_class,
        item.badge_count_before,
        badge_count_after,
        item.reopen_target.as_str(),
    );

    AttentionActionOutcome {
        outcome_id: format!(
            "m5-attention-actions:outcome:{}:{}",
            item.item_id,
            action.as_str()
        ),
        item_id: item.item_id.clone(),
        action,
        prior_state: item.prior_state,
        resulting_state: def.resulting_state,
        retention_class: def.retention_class.clone(),
        badge_effect: def.badge_effect,
        badge_count_before: item.badge_count_before,
        badge_count_after,
        badge_delta,
        keeps_underlying_record: def.keeps_underlying_record,
        creates_separate_suppression_state: def.creates_separate_suppression_state,
        audit_append_only: def.audit_append_only,
        resume_condition,
        reopen_target,
        reopen_anchor_ref,
        action_target_id,
        reopen_continuity_preserved,
        replays_side_effects: def.replays_side_effects,
        surface_propagation,
        support_export_note: def.support_export_note.clone(),
        reason,
    }
}

// ---------------------------------------------------------------------------
// Bundle record.
// ---------------------------------------------------------------------------

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionActionsInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen attention-actions bundle: the action definitions, the attention-item
/// corpus, and every applied outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionActionsBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_attention_actions_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The attention-routing matrix this bundle binds its vocabulary back to.
    pub matrix_ref: String,
    /// The matrix id the bundle binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps the bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the bundle.
    pub summary: String,
    /// The five distinct action definitions.
    pub action_definitions: Vec<AttentionActionDef>,
    /// The representative attention-item corpus.
    pub items: Vec<AttentionItem>,
    /// Every applied outcome (each item against each action it supports).
    pub outcomes: Vec<AttentionActionOutcome>,
    /// The computed invariants.
    pub invariants: Vec<AttentionActionsInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttentionActionsValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for AttentionActionsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attention-actions bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for AttentionActionsValidationError {}

impl AttentionActionsBundle {
    /// The item with a given id, if present.
    pub fn item(&self, item_id: &str) -> Option<&AttentionItem> {
        self.items.iter().find(|i| i.item_id == item_id)
    }

    /// The action definition for an action, if present.
    pub fn definition(&self, action: AttentionActionClass) -> Option<&AttentionActionDef> {
        self.action_definitions.iter().find(|d| d.action == action)
    }

    /// The outcome for an `(item, action)` pair, if present.
    pub fn outcome(
        &self,
        item_id: &str,
        action: AttentionActionClass,
    ) -> Option<&AttentionActionOutcome> {
        self.outcomes
            .iter()
            .find(|o| o.item_id == item_id && o.action == action)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let fixed = [
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
            self.schema_ref.as_str(),
        ]
        .into_iter();
        let from_items = self.items.iter().map(|i| i.reopen_anchor_ref.as_str());
        let from_outcomes = self.outcomes.iter().map(|o| o.reopen_anchor_ref.as_str());
        fixed.chain(from_items).chain(from_outcomes)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), AttentionActionsValidationError> {
        let fail = |reason: String| Err(AttentionActionsValidationError { reason });

        if self.record_kind != M5_ATTENTION_ACTIONS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ATTENTION_ACTIONS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.items.is_empty() || self.outcomes.is_empty() {
            return fail("items and outcomes must be non-empty".to_owned());
        }

        // Every action is defined exactly once.
        for action in AttentionActionClass::ALL {
            if self
                .action_definitions
                .iter()
                .filter(|d| d.action == action)
                .count()
                != 1
            {
                return fail(format!(
                    "action {} not defined exactly once",
                    action.as_str()
                ));
            }
        }

        // Stable ids are unique.
        if !all_unique(self.items.iter().map(|i| i.item_id.as_str())) {
            return fail("item ids are not unique".to_owned());
        }
        if !all_unique(self.outcomes.iter().map(|o| o.outcome_id.as_str())) {
            return fail("outcome ids are not unique".to_owned());
        }
        if !all_unique(self.action_definitions.iter().map(|d| d.action_id.as_str())) {
            return fail("action ids are not unique".to_owned());
        }

        // Every item carries a durable, reopenable identity and fans out to at least
        // the activity center and the badge.
        for item in &self.items {
            if item.badge_contribution == 0 || item.badge_contribution > item.badge_count_before {
                return fail(format!(
                    "item {} has an inconsistent badge contribution",
                    item.item_id
                ));
            }
            if item.supported_actions.is_empty() {
                return fail(format!("item {} supports no action", item.item_id));
            }
            if !item
                .fanout_surfaces
                .contains(&FanoutChannelClass::InAppActivityCenter)
                || !item
                    .fanout_surfaces
                    .contains(&FanoutChannelClass::DockTaskbarBadge)
            {
                return fail(format!(
                    "item {} must fan out to the activity center and the badge",
                    item.item_id
                ));
            }
            if item.reopen_anchor_ref.is_empty() || item.action_target_id.is_empty() {
                return fail(format!(
                    "item {} is missing its reopen anchor or action target",
                    item.item_id
                ));
            }
        }

        // Every outcome references a known item and a supported action and recomputes
        // identically (reproducible application).
        for outcome in &self.outcomes {
            let Some(item) = self.item(&outcome.item_id) else {
                return fail(format!(
                    "outcome {} references unknown item {}",
                    outcome.outcome_id, outcome.item_id
                ));
            };
            if !item.supports(outcome.action) {
                return fail(format!(
                    "outcome {} applies unsupported action {}",
                    outcome.outcome_id,
                    outcome.action.as_str()
                ));
            }
            if &apply_attention_action(item, outcome.action) != outcome {
                return fail(format!(
                    "outcome {} is not reproducible from its item and action",
                    outcome.outcome_id
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical attention-actions bundle.
///
/// Deterministic: the same bytes every call. The action definitions and the item
/// corpus are fixed, every outcome is computed by [`apply_attention_action`], and
/// each invariant's `holds` flag is computed from the built data, so an inconsistent
/// edit flips an invariant rather than silently passing.
pub fn attention_actions_bundle() -> AttentionActionsBundle {
    let action_definitions: Vec<AttentionActionDef> = AttentionActionClass::ALL
        .iter()
        .map(|a| action_definition(*a))
        .collect();
    let items = build_items();
    let outcomes = build_outcomes(&items);
    let invariants = compute_invariants(&action_definitions, &items, &outcomes);

    AttentionActionsBundle {
        record_kind: M5_ATTENTION_ACTIONS_RECORD_KIND.to_owned(),
        m5_attention_actions_schema_version: M5_ATTENTION_ACTIONS_SCHEMA_VERSION,
        schema_ref: M5_ATTENTION_ACTIONS_SCHEMA_REF.to_owned(),
        bundle_id: M5_ATTENTION_ACTIONS_BUNDLE_ID.to_owned(),
        as_of: M5_ATTENTION_ACTIONS_AS_OF.to_owned(),
        matrix_ref: M5_ATTENTION_ACTIONS_MATRIX_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ATTENTION_ACTIONS_FREEZE_GATE_REF.to_owned(),
        summary: "Distinct dismiss, snooze, acknowledge, mute, and resolve semantics for Aureline's \
                  durable attention objects: each action carries its own retention, badge, resume, \
                  and audit meaning rather than collapsing into one generic close. Every outcome \
                  keeps the underlying record, preserves exact reopen continuity through the same \
                  authoritative target and stable action target, defers with a resume condition only \
                  for snooze and mute, records suppression separately from audit history, propagates \
                  the same resulting state across the activity center, badge, OS notification, \
                  companions, and operator dashboard, and never replays a side effect. A security \
                  advisory can only be acknowledged or resolved, never silenced."
            .to_owned(),
        action_definitions,
        items,
        outcomes,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_outcomes(items: &[AttentionItem]) -> Vec<AttentionActionOutcome> {
    let mut out = Vec::new();
    for item in items {
        for action in &item.supported_actions {
            out.push(apply_attention_action(item, *action));
        }
    }
    out
}

fn surfaces(extra: &[FanoutChannelClass]) -> Vec<FanoutChannelClass> {
    let mut out = vec![
        FanoutChannelClass::InAppActivityCenter,
        FanoutChannelClass::DockTaskbarBadge,
    ];
    for s in extra {
        if !out.contains(s) {
            out.push(*s);
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn item(
    slug: &str,
    label: &str,
    object_class: AttentionObjectClass,
    source_subsystem: SourceSubsystemClass,
    scope: AttentionScopeClass,
    privacy_class: NotificationPrivacyClass,
    prior_state: AttentionStateClass,
    reopen_target: ReopenTargetClass,
    badge_count_before: u32,
    badge_contribution: u32,
    fanout_surfaces: Vec<FanoutChannelClass>,
    supported_actions: Vec<AttentionActionClass>,
) -> AttentionItem {
    AttentionItem {
        item_id: format!("attention_item:{slug}:0001"),
        label: label.to_owned(),
        object_class,
        source_subsystem,
        scope,
        privacy_class,
        prior_state,
        reopen_target,
        reopen_anchor_ref: format!("aureline://object/{slug}/0001"),
        action_target_id: format!("action_target:{slug}"),
        badge_count_before,
        badge_contribution,
        fanout_surfaces,
        supported_actions,
        created_at: M5_ATTENTION_ACTIONS_AS_OF.to_owned(),
    }
}

fn build_items() -> Vec<AttentionItem> {
    use AttentionActionClass::*;
    use AttentionObjectClass as O;
    use AttentionScopeClass as Sc;
    use AttentionStateClass as St;
    use FanoutChannelClass::*;
    use NotificationPrivacyClass as P;
    use ReopenTargetClass as R;
    use SourceSubsystemClass as S;

    vec![
        item(
            "task.run_failed",
            "Task run failed",
            O::ActivityObject,
            S::TaskRunner,
            Sc::Session,
            P::WorkspaceSensitive,
            St::Failed,
            R::ActivityJobRow,
            4,
            1,
            surfaces(&[OsNativeNotification, MobileCompanion]),
            vec![Dismiss, Snooze, Acknowledge, Resolve],
        ),
        item(
            "ai.awaiting_approval",
            "AI change awaiting approval",
            O::NotificationEnvelope,
            S::Ai,
            Sc::Session,
            P::WorkspaceSensitive,
            St::Shown,
            R::ReviewRequest,
            5,
            1,
            surfaces(&[OsNativeNotification, BrowserCompanion, MobileCompanion]),
            vec![Snooze, Acknowledge, Mute, Resolve],
        ),
        item(
            "collab.review_requested",
            "Collaboration review requested",
            O::NotificationEnvelope,
            S::Collaboration,
            Sc::Collaboration,
            P::WorkspaceSensitive,
            St::Shown,
            R::ReviewRequest,
            6,
            1,
            surfaces(&[OsNativeNotification, BrowserCompanion, MobileCompanion]),
            // The full grammar: a routine review supports every action.
            vec![Dismiss, Snooze, Acknowledge, Mute, Resolve],
        ),
        item(
            "incident.thread_opened",
            "Incident thread opened",
            O::NotificationEnvelope,
            S::Incident,
            Sc::Workspace,
            P::SecurityCritical,
            St::Shown,
            R::IncidentThread,
            3,
            1,
            surfaces(&[OsNativeNotification, MobileCompanion, OperatorDashboard]),
            vec![Snooze, Acknowledge, Resolve],
        ),
        item(
            "managed.policy_changed",
            "Managed policy changed",
            O::RoutingContext,
            S::ManagedPolicy,
            Sc::TenantOrg,
            P::ManagedSensitive,
            St::Shown,
            R::PolicyDiff,
            2,
            1,
            surfaces(&[OperatorDashboard]),
            vec![Acknowledge, Resolve],
        ),
        item(
            "security.credential_revoked",
            "Security credential revoked",
            O::NotificationEnvelope,
            S::Security,
            Sc::AppGlobal,
            P::SecurityCritical,
            St::Shown,
            R::AuditEvent,
            4,
            1,
            surfaces(&[OsNativeNotification, MobileCompanion, OperatorDashboard]),
            // A security advisory can only be acknowledged or resolved, never silenced.
            vec![Acknowledge, Resolve],
        ),
        item(
            "shell.command_result",
            "Shell command result",
            O::NotificationEnvelope,
            S::Shell,
            Sc::Window,
            P::SummarySafe,
            St::Shown,
            R::RouteObject,
            1,
            1,
            surfaces(&[]),
            vec![Dismiss, Acknowledge],
        ),
        item(
            "support.export_ready",
            "Support export ready",
            O::NotificationEnvelope,
            S::Support,
            Sc::AppGlobal,
            P::SummarySafe,
            St::Completed,
            R::EvidencePacket,
            3,
            1,
            surfaces(&[OsNativeNotification]),
            vec![Dismiss, Snooze, Acknowledge, Resolve],
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> AttentionActionsInvariant {
    AttentionActionsInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    defs: &[AttentionActionDef],
    items: &[AttentionItem],
    outcomes: &[AttentionActionOutcome],
) -> Vec<AttentionActionsInvariant> {
    let matrix = attention_routing_matrix();
    let mut out = Vec::new();

    // All five actions are defined and distinct, never collapsed into one close.
    out.push(invariant(
        "action.five_distinct_actions",
        "All five actions — dismiss, snooze, acknowledge, mute, and resolve — are defined, each with \
         a distinct resulting lifecycle state, so they are never collapsed into one generic close.",
        AttentionActionClass::ALL
            .iter()
            .all(|a| defs.iter().filter(|d| d.action == *a).count() == 1)
            && all_unique(defs.iter().map(|d| d.resulting_state.as_str())),
    ));

    // The badge effect is distinct per action.
    out.push(invariant(
        "action.badge_effects_distinct",
        "Each action carries a distinct badge effect — clear-keep-record, clear-mark-read, \
         clear-until-resume, clear-and-suppress-source, and clear-on-resolve — so dismiss, snooze, \
         acknowledge, mute, and resolve never share one badge behavior.",
        all_unique(defs.iter().map(|d| d.badge_effect.as_str())),
    ));

    // The full action signature is distinct per action.
    let signatures: Vec<String> = defs
        .iter()
        .map(|d| {
            format!(
                "{}|{}|{}|{}",
                d.resulting_state.as_str(),
                d.badge_effect.as_str(),
                d.resume_condition_kind.as_str(),
                d.effect_scope.as_str(),
            )
        })
        .collect();
    out.push(invariant(
        "action.semantics_distinct",
        "The (resulting state, badge effect, resume kind, effect scope) signature is unique per \
         action, so each action's retention and badge meaning is distinct.",
        all_unique(signatures.iter().map(String::as_str)),
    ));

    // Clearing a badge never erases the durable record.
    out.push(invariant(
        "action.keeps_underlying_record",
        "Every action clears the badge but keeps the underlying durable record, so no action erases \
         the authoritative object.",
        defs.iter().all(|d| d.clears_badge && d.keeps_underlying_record)
            && outcomes.iter().all(|o| o.keeps_underlying_record),
    ));

    // Exact reopen continuity survives every action.
    out.push(invariant(
        "action.exact_reopen_continuity",
        "Every outcome reopens the same authoritative target through the same anchor and the same \
         stable action target as its source item, so exact reopen continuity survives every action.",
        outcomes.iter().all(|o| {
            o.reopen_continuity_preserved
                && item_by_id(items, &o.item_id).is_some_and(|it| {
                    it.reopen_target == o.reopen_target
                        && it.reopen_anchor_ref == o.reopen_anchor_ref
                        && it.action_target_id == o.action_target_id
                })
        }),
    ));

    // No action or surface propagation replays a side effect.
    out.push(invariant(
        "action.no_side_effect_replay",
        "No action and no surface propagation replays the original side effect; acting reopens or \
         reflects the authoritative object rather than reissuing it.",
        defs.iter().all(|d| !d.replays_side_effects)
            && outcomes.iter().all(|o| {
                !o.replays_side_effects
                    && o.surface_propagation.iter().all(|p| !p.replays_side_effect)
            }),
    ));

    // Every surface reflects the same resulting state and action target.
    out.push(invariant(
        "action.surface_parity",
        "Every outcome propagates to the in-app activity center and the badge, the activity center \
         applies the action authoritatively, and every surface reflects the same stable action \
         target, so OS notification, in-app rows, companions, and operator surfaces share one action \
         model rather than inventing local variants.",
        outcomes.iter().all(|o| {
            o.propagation(FanoutChannelClass::InAppActivityCenter)
                .is_some_and(|p| p.propagation == SurfaceActionPropagationClass::ApplyAuthoritative)
                && o.propagation(FanoutChannelClass::DockTaskbarBadge).is_some()
                && o.surface_propagation
                    .iter()
                    .all(|p| p.reflects_action_target_id == o.action_target_id)
        }),
    ));

    // Suppression stays separate from audit history; acting is append-only.
    out.push(invariant(
        "action.suppression_separate_from_audit",
        "Snooze and mute record their deferral separately from audit history, no other action does, \
         and every action is audit-append-only and never overwrites history.",
        defs.iter().all(|d| {
            d.audit_append_only
                && d.creates_separate_suppression_state
                    == matches!(d.action, AttentionActionClass::Snooze | AttentionActionClass::Mute)
        }) && outcomes.iter().all(|o| o.audit_append_only),
    ));

    // A resume condition is present iff the action defers.
    out.push(invariant(
        "action.resume_condition_present_iff_required",
        "Snooze and mute carry a resume condition; dismiss, acknowledge, and resolve do not — a \
         resume condition is present exactly when the action defers.",
        outcomes.iter().all(|o| {
            let requires = matches!(
                o.action,
                AttentionActionClass::Snooze | AttentionActionClass::Mute
            );
            o.resume_condition.is_some() == requires
        }) && defs.iter().all(|d| {
            d.requires_resume_condition
                == (d.resume_condition_kind != ResumeConditionKindClass::None)
        }),
    ));

    // The badge clears and never goes negative or increases.
    out.push(invariant(
        "action.badge_clears_never_negative",
        "Every outcome's badge count after the action equals the count before minus the item's \
         contribution, never below zero and never above the count before, so an action clears the \
         badge and never inflates it.",
        outcomes.iter().all(|o| {
            o.badge_count_after <= o.badge_count_before
                && o.badge_delta == i64::from(o.badge_count_after) - i64::from(o.badge_count_before)
                && o.badge_delta <= 0
        }),
    ));

    // Support exports explain what happened without replaying side effects.
    out.push(invariant(
        "action.support_export_explains_without_replay",
        "Every outcome carries a non-empty support-export note and replays no side effect, so a \
         support export can explain what happened without reissuing it.",
        outcomes.iter().all(|o| {
            !o.support_export_note.is_empty() && !o.replays_side_effects && !o.reason.is_empty()
        }),
    ));

    // A security advisory cannot be silenced.
    out.push(invariant(
        "action.security_not_silenceable",
        "A security-advisory item is never dismissed, snoozed, or muted; it can only be acknowledged \
         or resolved.",
        outcomes.iter().all(|o| {
            match item_by_id(items, &o.item_id) {
                Some(it) if it.source_subsystem == SourceSubsystemClass::Security => {
                    !o.action.is_silencing()
                }
                _ => true,
            }
        }) && items
            .iter()
            .filter(|it| it.source_subsystem == SourceSubsystemClass::Security)
            .all(|it| it.supported_actions.iter().all(|a| !a.is_silencing())),
    ));

    // Every action and object family is exercised.
    out.push(invariant(
        "action.all_actions_exercised",
        "Every action appears in at least one outcome, and the corpus exercises more than one \
         attention object family.",
        AttentionActionClass::ALL
            .iter()
            .all(|a| outcomes.iter().any(|o| o.action == *a))
            && items
                .iter()
                .map(|i| i.object_class)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                > 1,
    ));

    // Every outcome is reproducible from its item and action.
    out.push(invariant(
        "action.outcomes_reproducible",
        "Re-applying every outcome's action to its item yields an identical outcome, so an applied \
         action is reproducible in support export and diagnostics.",
        outcomes.iter().all(|o| match item_by_id(items, &o.item_id) {
            Some(it) => &apply_attention_action(it, o.action) == o,
            None => false,
        }),
    ));

    // Every token binds back to the attention-routing matrix.
    out.push(invariant(
        "action.matrix_bound",
        "Every action token, resulting state, retention class, and item reopen target is one the \
         attention-routing matrix defines, and the action/retention-semantics object can show every \
         resulting state, so the action semantics never drift from the frozen object model.",
        matrix_bound_holds(defs, items, &matrix),
    ));

    // Every reference is support-export safe.
    out.push(invariant(
        "action.support_export_safe",
        "Every item reopen anchor and every outcome reopen anchor is a repo-relative object ref or \
         opaque aureline:// handle, never a URL, host, credential, or absolute path.",
        items.iter().all(|i| is_export_safe_ref(&i.reopen_anchor_ref))
            && outcomes.iter().all(|o| is_export_safe_ref(&o.reopen_anchor_ref)),
    ));

    out
}

fn item_by_id<'a>(items: &'a [AttentionItem], item_id: &str) -> Option<&'a AttentionItem> {
    items.iter().find(|i| i.item_id == item_id)
}

fn matrix_bound_holds(
    defs: &[AttentionActionDef],
    items: &[AttentionItem],
    matrix: &AttentionRoutingMatrix,
) -> bool {
    let action_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .action_semantics
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let retention_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .retention_classes
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let reopen_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .reopen_targets
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let ars = matrix.object(AttentionObjectClass::ActionRetentionSemantics);

    let defs_bound = defs.iter().all(|d| {
        action_tokens.contains(&d.action.as_str())
            && retention_tokens.contains(&d.retention_class.as_str())
            && ars.is_some_and(|o| o.can_show(d.resulting_state))
    });
    let items_bound = items
        .iter()
        .all(|i| reopen_tokens.contains(&i.reopen_target.as_str()));

    defs_bound && items_bound
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn attention_actions_lines(bundle: &AttentionActionsBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Attention-actions bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Actions: {}  Items: {}  Outcomes: {}  Invariants: {}",
        bundle.action_definitions.len(),
        bundle.items.len(),
        bundle.outcomes.len(),
        bundle.invariants.len(),
    ));

    lines.push("Actions:".to_owned());
    for d in &bundle.action_definitions {
        lines.push(format!(
            "  - {} [{}] state={} retention={} badge={} resume={} scope={} suppression_separate={}",
            d.action.as_str(),
            d.action_id,
            d.resulting_state.as_str(),
            d.retention_class,
            d.badge_effect.as_str(),
            d.resume_condition_kind.as_str(),
            d.effect_scope.as_str(),
            d.creates_separate_suppression_state,
        ));
        lines.push(format!("      {}", d.summary));
    }

    lines.push("Items:".to_owned());
    for i in &bundle.items {
        let actions: Vec<&str> = i.supported_actions.iter().map(|a| a.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] object={} subsystem={} reopen={} badge={} actions={}",
            i.label,
            i.item_id,
            i.object_class.as_str(),
            i.source_subsystem.as_str(),
            i.reopen_target.as_str(),
            i.badge_count_before,
            actions.join(", "),
        ));
    }

    lines.push("Outcomes:".to_owned());
    for o in &bundle.outcomes {
        lines.push(format!(
            "  - {} + {} -> {} retention={} badge={}->{} reopen={} resume={}",
            o.item_id,
            o.action.as_str(),
            o.resulting_state.as_str(),
            o.retention_class,
            o.badge_count_before,
            o.badge_count_after,
            o.reopen_target.as_str(),
            o.resume_condition.is_some(),
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
