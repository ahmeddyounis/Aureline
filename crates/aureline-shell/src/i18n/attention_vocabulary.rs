//! Shell-side projection binding durable-attention surfaces to one governed vocabulary.
//!
//! The activity center, job-row badges, notification summaries, lifecycle-state
//! copy, and quiet-hours/admin suppression messaging each have their own
//! canonical state enum. This projection binds every one of those canonical
//! states to a stable [`aureline_i18n`] attention/lifecycle glossary term, so no
//! surface coins its own translated wording and every surface that means the
//! same thing draws the same governed term.
//!
//! The binding is the cross-surface proof: an activity row's `running` state and
//! a durable job row's `running` state resolve to the same `term_key`, and an
//! activity row held by quiet hours resolves to the same suppression term a
//! quiet-hours fanout decision does. Severity rank and action order come from the
//! glossary, never re-invented per surface, so translation cannot make a state
//! read softer or stronger on one surface than another.
//!
//! The user view carries the localized label the shell paints; the metadata-only
//! support export omits the translated body but keeps the canonical token,
//! governed term key, and severity rank, so support and exported packets can map
//! a localized attention state back to one stable glossary entry.

use serde::{Deserialize, Serialize};

use aureline_i18n::{
    seeded_attention_vocabulary_glossary, AttentionSeverityRank, AttentionTermDomain,
    AttentionVocabularyGlossary, LocalizationRenderState, TextDirection,
    ATTENTION_TERM_TRUNCATION_BUDGET_GRAPHEMES,
};

use crate::activity_center::alpha::{ActivityPartition, ActivityRowStateClass};
use crate::durable_attention_beta::{DurableJobRowStateClass, QuietHoursDecisionClass};
use crate::notifications::actions::BadgeClass;
use crate::notifications::envelope::{QuietHoursMode, SuppressionReason};

/// Record kind for [`AttentionVocabularyView`].
pub const ATTENTION_VOCABULARY_VIEW_RECORD_KIND: &str = "shell_attention_vocabulary_view";

/// Durable-attention surface whose canonical states bind to the governed glossary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSurfaceFamily {
    /// Activity-center row lifecycle state.
    ActivityRowState,
    /// Activity-center partition / grouping header.
    ActivityPartition,
    /// Job-row badge or aggregate badge count.
    JobRowBadge,
    /// Durable job-row lifecycle state.
    DurableLifecycleState,
    /// Quiet-hours / focus / admin mode label.
    QuietHoursMode,
    /// Reason a fanout was suppressed, held, deduped, or muted.
    SuppressionReason,
    /// Quiet-hours suppression decision applied to a fanout.
    QuietHoursDecision,
}

/// Who is reading the attention-vocabulary view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionVocabularyAudience {
    /// User-facing shell surface that paints the localized label.
    User,
    /// Metadata-only support export.
    SupportExport,
}

impl AttentionVocabularyAudience {
    /// Returns true when this audience may carry translated body text.
    const fn carries_translated_body(self) -> bool {
        matches!(self, Self::User)
    }
}

/// One binding of a canonical surface state to a governed glossary term.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionVocabularyRow {
    /// Durable-attention surface family.
    pub surface_family: AttentionSurfaceFamily,
    /// Canonical enum name the state comes from.
    pub canonical_enum: String,
    /// Canonical, locale-neutral state token from the surface enum.
    pub canonical_token: String,
    /// Governed glossary term key the state binds to.
    pub governed_term_key: String,
    /// Governed glossary domain.
    pub domain: AttentionTermDomain,
    /// Locale-neutral severity rank inherited from the glossary.
    pub severity_rank: AttentionSeverityRank,
    /// Stable action-order index inherited from the glossary.
    pub action_order_index: u32,
    /// Whether the locale localized the term or fell back to the source language.
    pub localization_state: LocalizationRenderState,
    /// Writing direction for the requested locale.
    pub text_direction: TextDirection,
    /// Localized length as a percentage of the source length.
    pub expansion_ratio_pct: u32,
    /// Localized label the shell paints (omitted for the support export).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_term: Option<String>,
    /// Label truncated to the view budget (omitted for the support export).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated_term: Option<String>,
    /// Whether the label was shortened to fit the budget.
    pub was_truncated: bool,
    /// Always true; severity and scope cannot be hidden by truncation.
    pub severity_preserved_under_truncation: bool,
}

/// Inspectable attention-vocabulary view shared by durable-attention surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionVocabularyView {
    /// Boundary record kind.
    pub record_kind: String,
    /// Who is reading the view.
    pub audience: AttentionVocabularyAudience,
    /// Source glossary id.
    pub glossary_id_ref: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Source-language locale.
    pub source_language_locale: String,
    /// Writing direction for the requested locale.
    pub text_direction: TextDirection,
    /// Visible-grapheme budget the truncated terms were cut to.
    pub truncation_budget_graphemes: usize,
    /// Total bound states.
    pub total_rows: usize,
    /// Distinct governed term keys bound across surfaces.
    pub distinct_term_keys: usize,
    /// Per-state binding rows.
    pub rows: Vec<AttentionVocabularyRow>,
    /// True for the support export: no translated body crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

impl AttentionVocabularyView {
    /// Returns the binding rows for a governed term key.
    ///
    /// More than one row means several surfaces share one governed term, which is
    /// exactly the cross-surface consistency the vocabulary governance enforces.
    pub fn rows_for_term(&self, term_key: &str) -> Vec<&AttentionVocabularyRow> {
        self.rows
            .iter()
            .filter(|row| row.governed_term_key == term_key)
            .collect()
    }

    /// Returns the rows for one surface family.
    pub fn rows_for_surface(
        &self,
        surface_family: AttentionSurfaceFamily,
    ) -> Vec<&AttentionVocabularyRow> {
        self.rows
            .iter()
            .filter(|row| row.surface_family == surface_family)
            .collect()
    }

    /// Returns true when severity and scope survive truncation on every row.
    pub fn all_severities_preserved(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.severity_preserved_under_truncation)
    }
}

/// Projects the attention-vocabulary view for an audience and requested locale.
pub fn project_attention_vocabulary(
    audience: AttentionVocabularyAudience,
    requested_locale: &str,
) -> AttentionVocabularyView {
    let glossary = seeded_attention_vocabulary_glossary();
    build_view(&glossary, audience, requested_locale)
}

/// Projects the user-facing attention-vocabulary view.
pub fn project_user_attention_vocabulary(requested_locale: &str) -> AttentionVocabularyView {
    project_attention_vocabulary(AttentionVocabularyAudience::User, requested_locale)
}

/// Projects the metadata-only support-export attention-vocabulary view.
pub fn project_support_attention_vocabulary(requested_locale: &str) -> AttentionVocabularyView {
    project_attention_vocabulary(AttentionVocabularyAudience::SupportExport, requested_locale)
}

/// One canonical surface state and the governed term key it binds to.
struct Binding {
    surface_family: AttentionSurfaceFamily,
    canonical_enum: &'static str,
    canonical_token: String,
    term_key: &'static str,
}

impl Binding {
    fn new(
        surface_family: AttentionSurfaceFamily,
        canonical_enum: &'static str,
        canonical_token: &str,
        term_key: &'static str,
    ) -> Self {
        Self {
            surface_family,
            canonical_enum,
            canonical_token: canonical_token.to_owned(),
            term_key,
        }
    }
}

fn build_view(
    glossary: &AttentionVocabularyGlossary,
    audience: AttentionVocabularyAudience,
    requested_locale: &str,
) -> AttentionVocabularyView {
    let bindings = governed_bindings();
    let carries_body = audience.carries_translated_body();
    let mut rows = Vec::with_capacity(bindings.len());
    let mut distinct = std::collections::BTreeSet::new();

    for binding in &bindings {
        distinct.insert(binding.term_key.to_owned());
        let term = glossary
            .term(binding.term_key)
            .expect("binding references a governed term key");
        let translation = term.translation(requested_locale);

        let (
            localization_state,
            text_direction,
            expansion_ratio_pct,
            display,
            truncated,
            was_truncated,
        ) = match translation {
            Some(tr) => (
                tr.localization_state,
                tr.text_direction,
                tr.expansion_ratio_pct,
                tr.localized_term.clone(),
                tr.truncated_term.clone(),
                tr.was_truncated,
            ),
            None => (
                LocalizationRenderState::SourceLanguageFallback,
                TextDirection::LeftToRight,
                100,
                term.source_term.clone(),
                term.source_term.clone(),
                false,
            ),
        };

        rows.push(AttentionVocabularyRow {
            surface_family: binding.surface_family,
            canonical_enum: binding.canonical_enum.to_owned(),
            canonical_token: binding.canonical_token.clone(),
            governed_term_key: binding.term_key.to_owned(),
            domain: term.domain,
            severity_rank: term.severity_rank,
            action_order_index: term.action_order_index,
            localization_state,
            text_direction,
            expansion_ratio_pct,
            display_term: carries_body.then_some(display),
            truncated_term: carries_body.then_some(truncated),
            was_truncated,
            severity_preserved_under_truncation: true,
        });
    }

    AttentionVocabularyView {
        record_kind: ATTENTION_VOCABULARY_VIEW_RECORD_KIND.to_owned(),
        audience,
        glossary_id_ref: glossary.glossary_id.clone(),
        requested_locale: requested_locale.to_owned(),
        source_language_locale: glossary.source_language_locale.clone(),
        text_direction: TextDirection::for_locale(requested_locale),
        truncation_budget_graphemes: ATTENTION_TERM_TRUNCATION_BUDGET_GRAPHEMES,
        total_rows: rows.len(),
        distinct_term_keys: distinct.len(),
        rows,
        raw_translated_body_omitted: !carries_body,
    }
}

/// Builds every canonical durable-attention state binding.
///
/// The exhaustive `match` in each `term_key_for_*` keeps this complete: adding a
/// variant to a canonical surface enum fails to compile until it is bound to a
/// governed glossary term.
fn governed_bindings() -> Vec<Binding> {
    let mut bindings = Vec::new();

    for state in ACTIVITY_ROW_STATES {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::ActivityRowState,
            "ActivityRowStateClass",
            &token(&state),
            term_key_for_activity_row_state(state),
        ));
    }
    for partition in ACTIVITY_PARTITIONS {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::ActivityPartition,
            "ActivityPartition",
            &token(&partition),
            term_key_for_activity_partition(partition),
        ));
    }
    for badge in BADGE_CLASSES {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::JobRowBadge,
            "BadgeClass",
            &token(&badge),
            term_key_for_badge(badge),
        ));
    }
    for state in DURABLE_JOB_STATES {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::DurableLifecycleState,
            "DurableJobRowStateClass",
            &token(&state),
            term_key_for_durable_state(state),
        ));
    }
    for mode in QUIET_HOURS_MODES {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::QuietHoursMode,
            "QuietHoursMode",
            &token(&mode),
            term_key_for_quiet_hours_mode(mode),
        ));
    }
    for reason in SUPPRESSION_REASONS {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::SuppressionReason,
            "SuppressionReason",
            &token(&reason),
            term_key_for_suppression_reason(reason),
        ));
    }
    for decision in QUIET_HOURS_DECISIONS {
        bindings.push(Binding::new(
            AttentionSurfaceFamily::QuietHoursDecision,
            "QuietHoursDecisionClass",
            &token(&decision),
            term_key_for_quiet_hours_decision(decision),
        ));
    }

    bindings
}

/// Returns the canonical, locale-neutral serde token for a surface-state enum.
///
/// Using the enum's own serde contract keeps the recorded token identical to the
/// token every other durable-attention record uses for the same state.
fn token<T: Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(serde_json::Value::String(s)) => s,
        _ => String::new(),
    }
}

const ACTIVITY_ROW_STATES: [ActivityRowStateClass; 11] = [
    ActivityRowStateClass::QueuedWaiting,
    ActivityRowStateClass::Preparing,
    ActivityRowStateClass::Running,
    ActivityRowStateClass::NeedsApproval,
    ActivityRowStateClass::Completed,
    ActivityRowStateClass::Failed,
    ActivityRowStateClass::PartiallyCompleted,
    ActivityRowStateClass::Cancelled,
    ActivityRowStateClass::Superseded,
    ActivityRowStateClass::QuietHoursHeld,
    ActivityRowStateClass::PolicySuppressed,
];

const ACTIVITY_PARTITIONS: [ActivityPartition; 4] = [
    ActivityPartition::CurrentWork,
    ActivityPartition::NeedsAttention,
    ActivityPartition::Completed,
    ActivityPartition::SuppressedHeld,
];

const BADGE_CLASSES: [BadgeClass; 9] = [
    BadgeClass::NeedsReview,
    BadgeClass::FailedRuns,
    BadgeClass::Mentions,
    BadgeClass::SecurityNotices,
    BadgeClass::SessionRequests,
    BadgeClass::OfflinePublishPending,
    BadgeClass::DurableRunningCount,
    BadgeClass::HeldOrSuppressedCount,
    BadgeClass::CompletionUnread,
];

const DURABLE_JOB_STATES: [DurableJobRowStateClass; 7] = [
    DurableJobRowStateClass::Running,
    DurableJobRowStateClass::QueuedWaiting,
    DurableJobRowStateClass::NeedsApproval,
    DurableJobRowStateClass::Completed,
    DurableJobRowStateClass::Failed,
    DurableJobRowStateClass::Cancelled,
    DurableJobRowStateClass::HistoryOnly,
];

const QUIET_HOURS_MODES: [QuietHoursMode; 10] = [
    QuietHoursMode::ModeNone,
    QuietHoursMode::ModeQuietHoursUser,
    QuietHoursMode::ModeDoNotDisturbUser,
    QuietHoursMode::ModeFocusModeUser,
    QuietHoursMode::ModePresentation,
    QuietHoursMode::ModeScreenShare,
    QuietHoursMode::ModePrivacyMode,
    QuietHoursMode::ModeReducedAttentionPolicy,
    QuietHoursMode::ModePowerSaverRuntime,
    QuietHoursMode::ModeAdminSuppression,
];

const SUPPRESSION_REASONS: [SuppressionReason; 14] = [
    SuppressionReason::QuietHoursUserPolicy,
    SuppressionReason::DoNotDisturbUserPolicy,
    SuppressionReason::FocusModeUserPolicy,
    SuppressionReason::PresentationModeActive,
    SuppressionReason::ScreenShareActive,
    SuppressionReason::PrivacyModeActive,
    SuppressionReason::AdminPolicySuppression,
    SuppressionReason::ReducedAttentionPosture,
    SuppressionReason::PowerSaverBackgroundPause,
    SuppressionReason::DedupeSameCanonicalEvent,
    SuppressionReason::DedupeSameGroupedBurst,
    SuppressionReason::ClassMutedByUser,
    SuppressionReason::ClassSnoozedByUser,
    SuppressionReason::ReleasePendingNextUnsuppressedSurface,
];

const QUIET_HOURS_DECISIONS: [QuietHoursDecisionClass; 5] = [
    QuietHoursDecisionClass::NotSuppressed,
    QuietHoursDecisionClass::HeldQuietHours,
    QuietHoursDecisionClass::AdminSuppressed,
    QuietHoursDecisionClass::CriticalBypass,
    QuietHoursDecisionClass::CrossClientDeduped,
];

fn term_key_for_activity_row_state(state: ActivityRowStateClass) -> &'static str {
    match state {
        ActivityRowStateClass::QueuedWaiting => "attention.lifecycle.queued",
        ActivityRowStateClass::Preparing => "attention.lifecycle.preparing",
        ActivityRowStateClass::Running => "attention.lifecycle.running",
        ActivityRowStateClass::NeedsApproval => "attention.lifecycle.needs_approval",
        ActivityRowStateClass::Completed => "attention.lifecycle.completed",
        ActivityRowStateClass::Failed => "attention.lifecycle.failed",
        ActivityRowStateClass::PartiallyCompleted => "attention.lifecycle.partially_completed",
        ActivityRowStateClass::Cancelled => "attention.lifecycle.cancelled",
        ActivityRowStateClass::Superseded => "attention.lifecycle.superseded",
        ActivityRowStateClass::QuietHoursHeld => "attention.suppression.held_quiet_hours",
        ActivityRowStateClass::PolicySuppressed => "attention.suppression.suppressed_by_policy",
    }
}

fn term_key_for_activity_partition(partition: ActivityPartition) -> &'static str {
    match partition {
        ActivityPartition::CurrentWork => "attention.partition.current_work",
        ActivityPartition::NeedsAttention => "attention.partition.needs_attention",
        ActivityPartition::Completed => "attention.partition.completed",
        ActivityPartition::SuppressedHeld => "attention.partition.suppressed_held",
    }
}

fn term_key_for_badge(badge: BadgeClass) -> &'static str {
    match badge {
        BadgeClass::NeedsReview => "attention.badge.needs_review",
        BadgeClass::FailedRuns => "attention.badge.failed_runs",
        BadgeClass::Mentions => "attention.badge.mentions",
        BadgeClass::SecurityNotices => "attention.badge.security_notices",
        BadgeClass::SessionRequests => "attention.badge.session_requests",
        BadgeClass::OfflinePublishPending => "attention.badge.offline_publish_pending",
        BadgeClass::DurableRunningCount => "attention.badge.durable_running_count",
        BadgeClass::HeldOrSuppressedCount => "attention.badge.held_or_suppressed_count",
        BadgeClass::CompletionUnread => "attention.badge.completion_unread",
    }
}

fn term_key_for_durable_state(state: DurableJobRowStateClass) -> &'static str {
    match state {
        DurableJobRowStateClass::Running => "attention.lifecycle.running",
        DurableJobRowStateClass::QueuedWaiting => "attention.lifecycle.queued",
        DurableJobRowStateClass::NeedsApproval => "attention.lifecycle.needs_approval",
        DurableJobRowStateClass::Completed => "attention.lifecycle.completed",
        DurableJobRowStateClass::Failed => "attention.lifecycle.failed",
        DurableJobRowStateClass::Cancelled => "attention.lifecycle.cancelled",
        DurableJobRowStateClass::HistoryOnly => "attention.lifecycle.history_only",
    }
}

fn term_key_for_quiet_hours_mode(mode: QuietHoursMode) -> &'static str {
    match mode {
        QuietHoursMode::ModeNone => "attention.quiet_hours.none",
        QuietHoursMode::ModeQuietHoursUser => "attention.quiet_hours.quiet_hours",
        QuietHoursMode::ModeDoNotDisturbUser => "attention.quiet_hours.do_not_disturb",
        QuietHoursMode::ModeFocusModeUser => "attention.quiet_hours.focus_mode",
        QuietHoursMode::ModePresentation => "attention.quiet_hours.presentation",
        QuietHoursMode::ModeScreenShare => "attention.quiet_hours.screen_share",
        QuietHoursMode::ModePrivacyMode => "attention.quiet_hours.privacy_mode",
        QuietHoursMode::ModeReducedAttentionPolicy => {
            "attention.quiet_hours.reduced_attention_policy"
        }
        QuietHoursMode::ModePowerSaverRuntime => "attention.quiet_hours.power_saver",
        QuietHoursMode::ModeAdminSuppression => "attention.quiet_hours.admin_suppression",
    }
}

fn term_key_for_suppression_reason(reason: SuppressionReason) -> &'static str {
    match reason {
        SuppressionReason::QuietHoursUserPolicy => "attention.suppression.held_quiet_hours",
        SuppressionReason::DoNotDisturbUserPolicy => "attention.quiet_hours.do_not_disturb",
        SuppressionReason::FocusModeUserPolicy => "attention.quiet_hours.focus_mode",
        SuppressionReason::PresentationModeActive => "attention.quiet_hours.presentation",
        SuppressionReason::ScreenShareActive => "attention.quiet_hours.screen_share",
        SuppressionReason::PrivacyModeActive => "attention.quiet_hours.privacy_mode",
        SuppressionReason::AdminPolicySuppression => "attention.suppression.admin_suppression",
        SuppressionReason::ReducedAttentionPosture => "attention.suppression.reduced_attention",
        SuppressionReason::PowerSaverBackgroundPause => "attention.suppression.power_saver_paused",
        SuppressionReason::DedupeSameCanonicalEvent => "attention.suppression.deduped",
        SuppressionReason::DedupeSameGroupedBurst => "attention.suppression.deduped",
        SuppressionReason::ClassMutedByUser => "attention.suppression.muted_by_user",
        SuppressionReason::ClassSnoozedByUser => "attention.suppression.snoozed_by_user",
        SuppressionReason::ReleasePendingNextUnsuppressedSurface => {
            "attention.suppression.released_from_hold"
        }
    }
}

fn term_key_for_quiet_hours_decision(decision: QuietHoursDecisionClass) -> &'static str {
    match decision {
        QuietHoursDecisionClass::NotSuppressed => "attention.suppression.not_suppressed",
        QuietHoursDecisionClass::HeldQuietHours => "attention.suppression.held_quiet_hours",
        QuietHoursDecisionClass::AdminSuppressed => "attention.suppression.admin_suppression",
        QuietHoursDecisionClass::CriticalBypass => "attention.suppression.critical_bypass",
        QuietHoursDecisionClass::CrossClientDeduped => "attention.suppression.deduped",
    }
}
