//! Time-bounded, actor-scoped policy exceptions and expiry for M5 records.
//!
//! This module projects policy exceptions, waivers, and remembered decisions
//! onto the durable M5 artifact families. Every row is bound to an exact
//! actor/object/target/policy-epoch/environment scope, carries an explicit
//! expiry, and lists the reapproval triggers that automatically revalidate it on
//! drift. A remembered decision never widens authority across any of those
//! dimensions; on drift it must be re-approved rather than silently reused.
//!
//! The packet is the policy-side companion to the records-side hold/retention
//! truth source (`aureline-records::m5_records_policy`). The records rows
//! reference these exception rows by id so support/export surfaces can prove a
//! managed/support claim is gated by a live, bounded exception instead of an
//! indefinite waiver.

use serde::{Deserialize, Serialize};

use crate::policy_simulation_and_expiry::{ExpirySubjectClass, ReapprovalTriggerClass};

#[cfg(test)]
mod tests;

/// Schema version for the M5 exception/expiry packet.
pub const M5_EXCEPTION_EXPIRY_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the top-level packet.
pub const M5_EXCEPTION_EXPIRY_RECORD_KIND: &str = "m5_exception_expiry_packet";

/// Shared contract reference shared with the records-side hold/retention lane.
pub const M5_EXCEPTION_EXPIRY_SHARED_CONTRACT_REF: &str = "policy:m5_exception_expiry_truth:v1";

/// Repo-relative doc reference for the exception/expiry contract.
pub const M5_EXCEPTION_EXPIRY_DOC_REF: &str = "docs/governance/m5_exception_expiry.md";

/// Repo-relative artifact summary for the exception/expiry contract.
pub const M5_EXCEPTION_EXPIRY_ARTIFACT_REF: &str = "artifacts/governance/m5_exception_expiry.md";

/// Reference to the records-side hold/retention contract this packet gates.
pub const M5_EXCEPTION_EXPIRY_RECORDS_CONTRACT_REF: &str = "records:m5_hold_retention_truth:v1";

/// Stable record kind for [`ExceptionRequestSheet`] projections.
pub const M5_EXCEPTION_REQUEST_SHEET_RECORD_KIND: &str = "m5_exception_request_sheet";

/// Stable record kind for [`ApprovalHistoryRow`] projections.
pub const M5_APPROVAL_HISTORY_ROW_RECORD_KIND: &str = "m5_exception_approval_history_row";

/// Stable record kind for [`ExpiryBanner`] projections.
pub const M5_EXPIRY_BANNER_RECORD_KIND: &str = "m5_exception_expiry_banner";

/// Stable record kind for [`RememberedDecisionRevalidation`] projections.
pub const M5_REMEMBERED_DECISION_REVALIDATION_RECORD_KIND: &str =
    "m5_remembered_decision_revalidation";

/// The five authority dimensions a remembered decision must never widen across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityDimension {
    /// The acting principal.
    Actor,
    /// The object the decision applies to.
    Object,
    /// The action target.
    Target,
    /// The policy epoch in force.
    PolicyEpoch,
    /// The execution environment.
    Environment,
}

impl AuthorityDimension {
    /// All authority dimensions an exception must pin.
    pub const ALL: [Self; 5] = [
        Self::Actor,
        Self::Object,
        Self::Target,
        Self::PolicyEpoch,
        Self::Environment,
    ];

    /// Returns the stable snake_case token for the dimension.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Actor => "actor",
            Self::Object => "object",
            Self::Target => "target",
            Self::PolicyEpoch => "policy_epoch",
            Self::Environment => "environment",
        }
    }
}

/// The exact scope an exception is pinned to across all authority dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExceptionScopeBinding {
    /// Pinned actor.
    pub actor_ref: String,
    /// Pinned object.
    pub object_ref: String,
    /// Pinned target.
    pub target_ref: String,
    /// Pinned policy epoch.
    pub policy_epoch: String,
    /// Pinned environment.
    pub environment_ref: String,
}

impl ExceptionScopeBinding {
    /// Returns the dimensions whose pin is empty (and therefore unbounded).
    pub fn unbound_dimensions(&self) -> Vec<AuthorityDimension> {
        let mut missing = Vec::new();
        if self.actor_ref.trim().is_empty() {
            missing.push(AuthorityDimension::Actor);
        }
        if self.object_ref.trim().is_empty() {
            missing.push(AuthorityDimension::Object);
        }
        if self.target_ref.trim().is_empty() {
            missing.push(AuthorityDimension::Target);
        }
        if self.policy_epoch.trim().is_empty() {
            missing.push(AuthorityDimension::PolicyEpoch);
        }
        if self.environment_ref.trim().is_empty() {
            missing.push(AuthorityDimension::Environment);
        }
        missing
    }

    /// Returns an export-safe single-line summary of every pinned dimension.
    pub fn scope_summary(&self) -> String {
        format!(
            "actor={} object={} target={} policy_epoch={} environment={}",
            self.actor_ref,
            self.object_ref,
            self.target_ref,
            self.policy_epoch,
            self.environment_ref
        )
    }
}

/// Lifecycle class of one approval-history event on an exception row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalEventClass {
    /// The exception or waiver was requested.
    Requested,
    /// The request was approved and the exception became live.
    Approved,
    /// The exception was renewed before lapse with a fresh bound.
    Renewed,
    /// The exception was revalidated against current drift and held.
    Revalidated,
    /// The exception was narrowed in scope after a partial-drift review.
    Narrowed,
    /// The exception was revoked before its expiry.
    Revoked,
    /// The exception lapsed at its expiry and fell back automatically.
    Expired,
}

impl ApprovalEventClass {
    /// Returns the stable snake_case token for the event class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Approved => "approved",
            Self::Renewed => "renewed",
            Self::Revalidated => "revalidated",
            Self::Narrowed => "narrowed",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
        }
    }
}

/// One immutable approval-lineage event recorded against an exception row.
///
/// The ordered list of events on a row is the audit trail a support, CLI, or
/// admin surface replays to show exactly how an exception reached its current
/// state. Each event names the acting principal, the absolute UTC instant, and
/// an export-safe reason; it carries no credential bodies or raw payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalEvent {
    /// Lifecycle class of the event.
    pub event_class: ApprovalEventClass,
    /// Absolute UTC timestamp the event occurred.
    pub at: String,
    /// Principal that performed or triggered the event.
    pub actor_ref: String,
    /// Export-safe reason or note for the event.
    pub note: String,
}

/// One time-bounded, actor-scoped exception gating an M5 records claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExceptionExpiryRow {
    /// Stable exception id referenced by the records packet.
    pub exception_id: String,
    /// Subject class (exception, waiver, or remembered decision).
    pub subject_class: ExpirySubjectClass,
    /// Governed artifact family token this exception applies to.
    pub artifact_family_token: String,
    /// Record class token this exception applies to.
    pub record_class_token: String,
    /// Exact bypass scope this exception grants.
    pub exact_bypass_scope: String,
    /// Owner or approver accountable for the exception.
    pub owner_or_approver_ref: String,
    /// Reason the exception was granted.
    pub reason: String,
    /// Mitigation applied while the exception is live.
    pub mitigation: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Exact expiry timestamp.
    pub expires_at: String,
    /// Review-target timestamp.
    pub review_target_at: String,
    /// Pinned authority scope across all five dimensions.
    pub scope_binding: ExceptionScopeBinding,
    /// Reapproval triggers that automatically revalidate the exception on drift.
    pub reapproval_triggers: Vec<ReapprovalTriggerClass>,
    /// Ordered approval-lineage events from request to current state.
    pub approval_history: Vec<ApprovalEvent>,
    /// Current lifecycle state derived from the latest approval event.
    pub current_state: ApprovalEventClass,
    /// Behavior when the exception lapses.
    pub fallback_behavior_on_lapse: String,
    /// Whether the exception is bounded by an expiry (must be true).
    pub bounded_by_expiry: bool,
    /// Whether the exception widens authority (must be false).
    pub widens_authority: bool,
}

impl M5ExceptionExpiryRow {
    /// Projects this row into an exception/waiver request sheet that shows the
    /// exact variance, scope, reason, approver, and expiry instead of generic
    /// bypass language.
    pub fn request_sheet(&self) -> ExceptionRequestSheet {
        ExceptionRequestSheet {
            record_kind: M5_EXCEPTION_REQUEST_SHEET_RECORD_KIND.to_owned(),
            sheet_id: format!("m5-exception-request:{}", self.exception_id),
            exception_id: self.exception_id.clone(),
            subject_class: self.subject_class,
            subject_class_token: self.subject_class.as_str().to_owned(),
            artifact_family_token: self.artifact_family_token.clone(),
            record_class_token: self.record_class_token.clone(),
            exact_variance: self.exact_bypass_scope.clone(),
            scope_summary: self.scope_binding.scope_summary(),
            reason: self.reason.clone(),
            mitigation: self.mitigation.clone(),
            owner_or_approver_ref: self.owner_or_approver_ref.clone(),
            created_at: self.created_at.clone(),
            expires_at: self.expires_at.clone(),
            review_target_at: self.review_target_at.clone(),
            fallback_behavior_on_lapse: self.fallback_behavior_on_lapse.clone(),
            reapproval_trigger_tokens: self
                .reapproval_triggers
                .iter()
                .map(|trigger| trigger.as_str().to_owned())
                .collect(),
            bounded_by_expiry: self.bounded_by_expiry,
            widens_authority: self.widens_authority,
        }
    }

    /// Projects this row into an approval-history row exposing the full lineage
    /// and current lifecycle state.
    pub fn approval_history_row(&self) -> ApprovalHistoryRow {
        ApprovalHistoryRow {
            record_kind: M5_APPROVAL_HISTORY_ROW_RECORD_KIND.to_owned(),
            approval_history_row_id: format!("m5-exception-history:{}", self.exception_id),
            exception_id: self.exception_id.clone(),
            subject_class: self.subject_class,
            subject_class_token: self.subject_class.as_str().to_owned(),
            owner_or_approver_ref: self.owner_or_approver_ref.clone(),
            current_state: self.current_state,
            current_state_token: self.current_state.as_str().to_owned(),
            events: self.approval_history.clone(),
            expires_at: self.expires_at.clone(),
            review_target_at: self.review_target_at.clone(),
            bounded_by_expiry: self.bounded_by_expiry,
            reapproval_trigger_tokens: self
                .reapproval_triggers
                .iter()
                .map(|trigger| trigger.as_str().to_owned())
                .collect(),
        }
    }

    /// Projects this row into an expiry banner whose state is computed by
    /// comparing the exact expiry and review-target instants against `as_of`.
    ///
    /// ISO-8601 UTC instants compare lexicographically in chronological order,
    /// so the banner stays deterministic without a wall clock.
    pub fn expiry_banner(&self, as_of: &str) -> ExpiryBanner {
        let state = if !self.expires_at.is_empty() && as_of >= self.expires_at.as_str() {
            ExpiryState::Expired
        } else if !self.review_target_at.is_empty() && as_of >= self.review_target_at.as_str() {
            ExpiryState::ExpiringSoon
        } else {
            ExpiryState::Active
        };
        ExpiryBanner {
            record_kind: M5_EXPIRY_BANNER_RECORD_KIND.to_owned(),
            banner_id: format!("m5-exception-banner:{}", self.exception_id),
            exception_id: self.exception_id.clone(),
            subject_class: self.subject_class,
            subject_class_token: self.subject_class.as_str().to_owned(),
            state,
            state_token: state.as_str().to_owned(),
            what_expires: format!(
                "{} for {}",
                self.exact_bypass_scope.trim_end_matches('.'),
                self.scope_binding.object_ref
            ),
            as_of: as_of.to_owned(),
            exact_expiry_at: self.expires_at.clone(),
            review_target_at: self.review_target_at.clone(),
            consequence_on_expiry: self.fallback_behavior_on_lapse.clone(),
            renew_or_review_action_ref: format!("policy.m5_exception.review:{}", self.exception_id),
        }
    }

    /// Returns the observed context that exactly matches this row's pinned scope
    /// at `as_of` — the no-drift baseline a revalidation compares against.
    pub fn observed_at(&self, as_of: &str) -> ObservedContext {
        ObservedContext {
            actor_ref: self.scope_binding.actor_ref.clone(),
            object_ref: self.scope_binding.object_ref.clone(),
            target_ref: self.scope_binding.target_ref.clone(),
            policy_epoch: self.scope_binding.policy_epoch.clone(),
            environment_ref: self.scope_binding.environment_ref.clone(),
            as_of: as_of.to_owned(),
        }
    }

    /// Revalidates a remembered decision against the real-world context observed
    /// at reuse time.
    ///
    /// The original exception is only honored when every pinned authority
    /// dimension still matches and the decision has not expired. Any drift
    /// across actor/object/target/policy-epoch/environment, or a lapsed expiry,
    /// forces a re-review rather than silent reuse. The outcome never widens the
    /// original authority: at most it confirms the unchanged, pinned scope.
    pub fn revalidate(&self, observed: &ObservedContext) -> RememberedDecisionRevalidation {
        let mut drifted = Vec::new();
        if observed.actor_ref != self.scope_binding.actor_ref {
            drifted.push(AuthorityDimension::Actor);
        }
        if observed.object_ref != self.scope_binding.object_ref {
            drifted.push(AuthorityDimension::Object);
        }
        if observed.target_ref != self.scope_binding.target_ref {
            drifted.push(AuthorityDimension::Target);
        }
        if observed.policy_epoch != self.scope_binding.policy_epoch {
            drifted.push(AuthorityDimension::PolicyEpoch);
        }
        if observed.environment_ref != self.scope_binding.environment_ref {
            drifted.push(AuthorityDimension::Environment);
        }

        let expired = !self.expires_at.is_empty()
            && !observed.as_of.is_empty()
            && observed.as_of.as_str() >= self.expires_at.as_str();

        let outcome = if drifted.is_empty() && !expired {
            RevalidationOutcome::StillValid
        } else {
            RevalidationOutcome::MustReReview
        };

        let reason = if !drifted.is_empty() {
            let tokens: Vec<&str> = drifted.iter().map(|dimension| dimension.as_str()).collect();
            format!(
                "remembered decision invalidated: drift across {}; fresh approval required",
                tokens.join(", ")
            )
        } else if expired {
            "remembered decision expired; fresh approval required before reuse".to_owned()
        } else {
            "remembered decision still valid within its pinned, unexpired scope".to_owned()
        };

        RememberedDecisionRevalidation {
            record_kind: M5_REMEMBERED_DECISION_REVALIDATION_RECORD_KIND.to_owned(),
            exception_id: self.exception_id.clone(),
            subject_class: self.subject_class,
            subject_class_token: self.subject_class.as_str().to_owned(),
            outcome,
            outcome_token: outcome.as_str().to_owned(),
            drifted_dimensions: drifted,
            expired,
            must_reauthorize: outcome == RevalidationOutcome::MustReReview,
            // A revalidation never broadens the original grant.
            widens_authority: false,
            reason,
        }
    }
}

/// Request sheet showing the exact variance, scope, reason, approver, and
/// expiry for one exception or waiver instead of generic bypass language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExceptionRequestSheet {
    /// Stable record kind.
    pub record_kind: String,
    /// Durable sheet id.
    pub sheet_id: String,
    /// Source exception id.
    pub exception_id: String,
    /// Subject class (exception, waiver, or remembered decision).
    pub subject_class: ExpirySubjectClass,
    /// Stable token for [`Self::subject_class`].
    pub subject_class_token: String,
    /// Governed artifact family token.
    pub artifact_family_token: String,
    /// Record class token.
    pub record_class_token: String,
    /// Exact variance from policy this exception grants.
    pub exact_variance: String,
    /// Single-line summary of the pinned authority scope.
    pub scope_summary: String,
    /// Reason the exception was requested.
    pub reason: String,
    /// Mitigation applied while the exception is live.
    pub mitigation: String,
    /// Owner or approver accountable for the exception.
    pub owner_or_approver_ref: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Exact expiry timestamp.
    pub expires_at: String,
    /// Review-target timestamp.
    pub review_target_at: String,
    /// Behavior when the exception lapses.
    pub fallback_behavior_on_lapse: String,
    /// Stable tokens for the reapproval triggers that revalidate on drift.
    pub reapproval_trigger_tokens: Vec<String>,
    /// Whether the exception is bounded by an expiry (must be true).
    pub bounded_by_expiry: bool,
    /// Whether the exception widens authority (must be false).
    pub widens_authority: bool,
}

/// Approval-history row exposing the full lineage and current state of an
/// exception across product, CLI/headless, and support-export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalHistoryRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Durable row id.
    pub approval_history_row_id: String,
    /// Source exception id.
    pub exception_id: String,
    /// Subject class.
    pub subject_class: ExpirySubjectClass,
    /// Stable token for [`Self::subject_class`].
    pub subject_class_token: String,
    /// Owner or approver accountable for the exception.
    pub owner_or_approver_ref: String,
    /// Current lifecycle state.
    pub current_state: ApprovalEventClass,
    /// Stable token for [`Self::current_state`].
    pub current_state_token: String,
    /// Ordered approval-lineage events from request to current state.
    pub events: Vec<ApprovalEvent>,
    /// Exact expiry timestamp.
    pub expires_at: String,
    /// Review-target timestamp.
    pub review_target_at: String,
    /// Whether the exception is bounded by an expiry.
    pub bounded_by_expiry: bool,
    /// Stable tokens for the reapproval triggers that revalidate on drift.
    pub reapproval_trigger_tokens: Vec<String>,
}

/// Lifecycle state of an exception relative to its expiry and review target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryState {
    /// The exception is live and before its review target.
    Active,
    /// The exception has passed its review target but not yet expired.
    ExpiringSoon,
    /// The exception has reached or passed its exact expiry.
    Expired,
}

impl ExpiryState {
    /// Returns the stable snake_case token for the state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
        }
    }
}

/// Expiry banner keeping an exception or remembered decision time-bounded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiryBanner {
    /// Stable record kind.
    pub record_kind: String,
    /// Durable banner id.
    pub banner_id: String,
    /// Source exception id.
    pub exception_id: String,
    /// Subject class.
    pub subject_class: ExpirySubjectClass,
    /// Stable token for [`Self::subject_class`].
    pub subject_class_token: String,
    /// Lifecycle state computed against [`Self::as_of`].
    pub state: ExpiryState,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// Export-safe label of what expires.
    pub what_expires: String,
    /// UTC instant the banner state was computed against.
    pub as_of: String,
    /// Exact UTC expiry timestamp.
    pub exact_expiry_at: String,
    /// Review-target timestamp.
    pub review_target_at: String,
    /// Consequence once the exception lapses.
    pub consequence_on_expiry: String,
    /// Renewal or review action ref exposed by product and CLI.
    pub renew_or_review_action_ref: String,
}

/// The real-world context observed when a remembered decision is reused.
///
/// Each field is the *current* value of an authority dimension; the
/// revalidation compares it against the exception's pinned scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedContext {
    /// Currently acting principal.
    pub actor_ref: String,
    /// Object the action now applies to.
    pub object_ref: String,
    /// Current action target.
    pub target_ref: String,
    /// Policy epoch now in force.
    pub policy_epoch: String,
    /// Current execution environment.
    pub environment_ref: String,
    /// UTC instant of reuse, compared against the exception's expiry.
    pub as_of: String,
}

/// Outcome of revalidating a remembered decision against observed drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationOutcome {
    /// Every pinned dimension still matches and the decision is unexpired.
    StillValid,
    /// Drift or expiry forces a fresh review before reuse.
    MustReReview,
}

impl RevalidationOutcome {
    /// Returns the stable snake_case token for the outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StillValid => "still_valid",
            Self::MustReReview => "must_re_review",
        }
    }
}

/// Result of revalidating a remembered decision at reuse time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedDecisionRevalidation {
    /// Stable record kind.
    pub record_kind: String,
    /// Source exception id.
    pub exception_id: String,
    /// Subject class.
    pub subject_class: ExpirySubjectClass,
    /// Stable token for [`Self::subject_class`].
    pub subject_class_token: String,
    /// Revalidation outcome.
    pub outcome: RevalidationOutcome,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Authority dimensions whose observed value drifted from the pinned scope.
    pub drifted_dimensions: Vec<AuthorityDimension>,
    /// Whether the decision has passed its exact expiry.
    pub expired: bool,
    /// Whether a fresh authorization is required before reuse.
    pub must_reauthorize: bool,
    /// Whether the revalidation widened the original authority (always false).
    pub widens_authority: bool,
    /// Export-safe explanation of the outcome.
    pub reason: String,
}

/// Top-level canonical M5 exception/expiry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExceptionExpiryPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// UTC packet timestamp.
    pub as_of: String,
    /// Overview doc ref.
    pub overview_doc_ref: String,
    /// Artifact summary ref.
    pub artifact_summary_ref: String,
    /// Reference to the records-side hold/retention contract this packet gates.
    pub records_contract_ref: String,
    /// Exception/expiry rows.
    pub rows: Vec<M5ExceptionExpiryRow>,
    /// Review-safe summary.
    pub summary: String,
}

/// Validation issue emitted by the M5 exception/expiry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum M5ExceptionExpiryViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch { found: u32 },
    /// Record kind mismatch.
    RecordKindMismatch { found: String },
    /// Exception is not bounded by an expiry.
    ExceptionNotBounded { exception_id: String },
    /// Exception expiry timestamp is empty.
    ExceptionExpiryMissing { exception_id: String },
    /// Exception widens authority instead of pinning it.
    ExceptionWidensAuthority { exception_id: String },
    /// Exception omits any reapproval trigger to revalidate on drift.
    ReapprovalTriggerMissing { exception_id: String },
    /// Exception leaves an authority dimension unpinned.
    ScopeDimensionUnbound {
        exception_id: String,
        dimension: AuthorityDimension,
    },
    /// Exception omits a fallback behavior on lapse.
    FallbackBehaviorMissing { exception_id: String },
    /// Exception carries no approval-history lineage.
    ApprovalHistoryMissing { exception_id: String },
    /// Exception's current state does not match its latest approval event.
    CurrentStateInconsistent {
        exception_id: String,
        latest_event: ApprovalEventClass,
        current_state: ApprovalEventClass,
    },
}

impl M5ExceptionExpiryPacket {
    /// Validates the packet against the exception/expiry honesty contract.
    pub fn validate(&self) -> Vec<M5ExceptionExpiryViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_EXCEPTION_EXPIRY_SCHEMA_VERSION {
            violations.push(M5ExceptionExpiryViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_EXCEPTION_EXPIRY_RECORD_KIND {
            violations.push(M5ExceptionExpiryViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        for row in &self.rows {
            if !row.bounded_by_expiry {
                violations.push(M5ExceptionExpiryViolation::ExceptionNotBounded {
                    exception_id: row.exception_id.clone(),
                });
            }
            if row.expires_at.trim().is_empty() {
                violations.push(M5ExceptionExpiryViolation::ExceptionExpiryMissing {
                    exception_id: row.exception_id.clone(),
                });
            }
            if row.widens_authority {
                violations.push(M5ExceptionExpiryViolation::ExceptionWidensAuthority {
                    exception_id: row.exception_id.clone(),
                });
            }
            if row.reapproval_triggers.is_empty() {
                violations.push(M5ExceptionExpiryViolation::ReapprovalTriggerMissing {
                    exception_id: row.exception_id.clone(),
                });
            }
            for dimension in row.scope_binding.unbound_dimensions() {
                violations.push(M5ExceptionExpiryViolation::ScopeDimensionUnbound {
                    exception_id: row.exception_id.clone(),
                    dimension,
                });
            }
            if row.fallback_behavior_on_lapse.trim().is_empty() {
                violations.push(M5ExceptionExpiryViolation::FallbackBehaviorMissing {
                    exception_id: row.exception_id.clone(),
                });
            }
            match row.approval_history.last() {
                None => violations.push(M5ExceptionExpiryViolation::ApprovalHistoryMissing {
                    exception_id: row.exception_id.clone(),
                }),
                Some(latest) if latest.event_class != row.current_state => {
                    violations.push(M5ExceptionExpiryViolation::CurrentStateInconsistent {
                        exception_id: row.exception_id.clone(),
                        latest_event: latest.event_class,
                        current_state: row.current_state,
                    });
                }
                Some(_) => {}
            }
        }

        violations
    }

    /// Projects every row into an exception/waiver request sheet.
    pub fn request_sheets(&self) -> Vec<ExceptionRequestSheet> {
        self.rows
            .iter()
            .map(M5ExceptionExpiryRow::request_sheet)
            .collect()
    }

    /// Projects every row into an approval-history row.
    pub fn approval_history(&self) -> Vec<ApprovalHistoryRow> {
        self.rows
            .iter()
            .map(M5ExceptionExpiryRow::approval_history_row)
            .collect()
    }

    /// Projects every row into an expiry banner computed against [`Self::as_of`].
    pub fn expiry_banners(&self) -> Vec<ExpiryBanner> {
        self.rows
            .iter()
            .map(|row| row.expiry_banner(&self.as_of))
            .collect()
    }

    /// Revalidates every row against its own pinned scope at [`Self::as_of`].
    ///
    /// With no observed drift and the packet's own timestamp, every row is
    /// expected to revalidate as still valid; this anchors the contract that the
    /// seeded packet's exceptions are live and self-consistent.
    pub fn self_revalidation(&self) -> Vec<RememberedDecisionRevalidation> {
        self.rows
            .iter()
            .map(|row| row.revalidate(&row.observed_at(&self.as_of)))
            .collect()
    }

    /// Returns the exception ids the packet defines.
    pub fn exception_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rows
            .iter()
            .map(|row| row.exception_id.clone())
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }
}

/// Returns the canonical seeded M5 exception/expiry packet.
pub fn seeded_m5_exception_expiry_packet() -> M5ExceptionExpiryPacket {
    let rows = vec![
        M5ExceptionExpiryRow {
            exception_id: "m5-exception:ai-evidence-retention-waiver".to_owned(),
            subject_class: ExpirySubjectClass::PolicyWaiver,
            artifact_family_token: "ai_evidence_packet".to_owned(),
            record_class_token: "ai_retained_evidence_packet".to_owned(),
            exact_bypass_scope: "Shorten the managed evidence retention floor for this case."
                .to_owned(),
            owner_or_approver_ref: "approver:records-governance".to_owned(),
            reason: "Time-limited investigation requires earlier purge eligibility.".to_owned(),
            mitigation: "Managed copies remain audited until the waiver expires.".to_owned(),
            created_at: "2026-06-13T16:00:00Z".to_owned(),
            expires_at: "2026-09-13T16:00:00Z".to_owned(),
            review_target_at: "2026-08-13T16:00:00Z".to_owned(),
            scope_binding: ExceptionScopeBinding {
                actor_ref: "actor:org-admin".to_owned(),
                object_ref: "object:ai-evidence-case-0001".to_owned(),
                target_ref: "target:managed-evidence-archive".to_owned(),
                policy_epoch: "policy:m5-records:v1".to_owned(),
                environment_ref: "env:managed-control-plane".to_owned(),
            },
            reapproval_triggers: vec![
                ReapprovalTriggerClass::PolicyDrift,
                ReapprovalTriggerClass::VersionDrift,
                ReapprovalTriggerClass::AuthorityDrift,
            ],
            approval_history: vec![
                ApprovalEvent {
                    event_class: ApprovalEventClass::Requested,
                    at: "2026-06-13T15:40:00Z".to_owned(),
                    actor_ref: "actor:org-admin".to_owned(),
                    note: "Investigation requested an earlier purge eligibility window.".to_owned(),
                },
                ApprovalEvent {
                    event_class: ApprovalEventClass::Approved,
                    at: "2026-06-13T16:00:00Z".to_owned(),
                    actor_ref: "approver:records-governance".to_owned(),
                    note: "Approved as a bounded waiver; managed copies stay audited until expiry."
                        .to_owned(),
                },
            ],
            current_state: ApprovalEventClass::Approved,
            fallback_behavior_on_lapse:
                "On lapse, the managed retention floor is reinstated automatically.".to_owned(),
            bounded_by_expiry: true,
            widens_authority: false,
        },
        M5ExceptionExpiryRow {
            exception_id: "m5-exception:companion-hold-review".to_owned(),
            subject_class: ExpirySubjectClass::PolicyException,
            artifact_family_token: "companion_continuity_packet".to_owned(),
            record_class_token: "companion_continuity_packet".to_owned(),
            exact_bypass_scope: "Permit read-only export while the hold status is confirmed."
                .to_owned(),
            owner_or_approver_ref: "approver:org-admin".to_owned(),
            reason: "Support needs read-only access while the hold evaluation resolves.".to_owned(),
            mitigation: "Destruction stays blocked; only reads are permitted.".to_owned(),
            created_at: "2026-06-13T16:00:00Z".to_owned(),
            expires_at: "2026-07-13T16:00:00Z".to_owned(),
            review_target_at: "2026-06-27T16:00:00Z".to_owned(),
            scope_binding: ExceptionScopeBinding {
                actor_ref: "actor:support-agent".to_owned(),
                object_ref: "object:companion-packet-0001".to_owned(),
                target_ref: "target:managed-companion-archive".to_owned(),
                policy_epoch: "policy:m5-records:v1".to_owned(),
                environment_ref: "env:managed-control-plane".to_owned(),
            },
            reapproval_triggers: vec![
                ReapprovalTriggerClass::TargetDrift,
                ReapprovalTriggerClass::AuthorityDrift,
            ],
            approval_history: vec![
                ApprovalEvent {
                    event_class: ApprovalEventClass::Requested,
                    at: "2026-06-13T15:45:00Z".to_owned(),
                    actor_ref: "actor:support-agent".to_owned(),
                    note: "Support requested read-only export while the hold resolves.".to_owned(),
                },
                ApprovalEvent {
                    event_class: ApprovalEventClass::Approved,
                    at: "2026-06-13T16:00:00Z".to_owned(),
                    actor_ref: "approver:org-admin".to_owned(),
                    note: "Approved read-only; destruction stays blocked.".to_owned(),
                },
            ],
            current_state: ApprovalEventClass::Approved,
            fallback_behavior_on_lapse: "On lapse, read access is withdrawn automatically."
                .to_owned(),
            bounded_by_expiry: true,
            widens_authority: false,
        },
        M5ExceptionExpiryRow {
            exception_id: "m5-exception:sync-mirror-hold".to_owned(),
            subject_class: ExpirySubjectClass::PolicyException,
            artifact_family_token: "sync_mirror_ledger".to_owned(),
            record_class_token: "sync_mirror_ledger".to_owned(),
            exact_bypass_scope: "Acknowledge the active litigation hold on the managed mirror."
                .to_owned(),
            owner_or_approver_ref: "approver:legal".to_owned(),
            reason: "Litigation hold requires the managed mirror be preserved.".to_owned(),
            mitigation: "Deletion stays blocked; local snapshots are unaffected.".to_owned(),
            created_at: "2026-06-13T16:00:00Z".to_owned(),
            expires_at: "2026-12-13T16:00:00Z".to_owned(),
            review_target_at: "2026-10-13T16:00:00Z".to_owned(),
            scope_binding: ExceptionScopeBinding {
                actor_ref: "actor:legal".to_owned(),
                object_ref: "object:sync-mirror-ledger-0001".to_owned(),
                target_ref: "target:managed-sync-mirror".to_owned(),
                policy_epoch: "policy:m5-records:v1".to_owned(),
                environment_ref: "env:managed-control-plane".to_owned(),
            },
            reapproval_triggers: vec![
                ReapprovalTriggerClass::PolicyDrift,
                ReapprovalTriggerClass::TargetDrift,
            ],
            approval_history: vec![
                ApprovalEvent {
                    event_class: ApprovalEventClass::Requested,
                    at: "2026-06-13T15:30:00Z".to_owned(),
                    actor_ref: "actor:legal".to_owned(),
                    note: "Legal opened a litigation hold on the managed mirror.".to_owned(),
                },
                ApprovalEvent {
                    event_class: ApprovalEventClass::Approved,
                    at: "2026-06-13T16:00:00Z".to_owned(),
                    actor_ref: "approver:legal".to_owned(),
                    note: "Hold acknowledged; deletion blocked, local snapshots unaffected."
                        .to_owned(),
                },
            ],
            current_state: ApprovalEventClass::Approved,
            fallback_behavior_on_lapse:
                "On lapse, the hold acknowledgement must be renewed before any deletion.".to_owned(),
            bounded_by_expiry: true,
            widens_authority: false,
        },
        M5ExceptionExpiryRow {
            exception_id: "m5-exception:offboarding-retention-floor".to_owned(),
            subject_class: ExpirySubjectClass::RememberedDecision,
            artifact_family_token: "offboarding_record".to_owned(),
            record_class_token: "offboarding_exit_packet".to_owned(),
            exact_bypass_scope: "Remember the admin's acknowledgement of the retention floor."
                .to_owned(),
            owner_or_approver_ref: "approver:org-admin".to_owned(),
            reason: "Admin acknowledged that offboarding deletes wait for the retention floor."
                .to_owned(),
            mitigation:
                "Decision is re-prompted on any drift across actor/object/target/epoch/env."
                    .to_owned(),
            created_at: "2026-06-13T16:00:00Z".to_owned(),
            expires_at: "2026-07-13T16:00:00Z".to_owned(),
            review_target_at: "2026-06-27T16:00:00Z".to_owned(),
            scope_binding: ExceptionScopeBinding {
                actor_ref: "actor:org-admin".to_owned(),
                object_ref: "object:offboarding-packet-0001".to_owned(),
                target_ref: "target:managed-offboarding-archive".to_owned(),
                policy_epoch: "policy:m5-records:v1".to_owned(),
                environment_ref: "env:managed-control-plane".to_owned(),
            },
            reapproval_triggers: ReapprovalTriggerClass::ALL.to_vec(),
            approval_history: vec![
                ApprovalEvent {
                    event_class: ApprovalEventClass::Requested,
                    at: "2026-06-13T15:50:00Z".to_owned(),
                    actor_ref: "actor:org-admin".to_owned(),
                    note: "Admin acknowledged the offboarding retention floor.".to_owned(),
                },
                ApprovalEvent {
                    event_class: ApprovalEventClass::Approved,
                    at: "2026-06-13T16:00:00Z".to_owned(),
                    actor_ref: "approver:org-admin".to_owned(),
                    note: "Remembered; re-prompted on any actor/object/target/epoch/env drift."
                        .to_owned(),
                },
            ],
            current_state: ApprovalEventClass::Approved,
            fallback_behavior_on_lapse:
                "On lapse, the admin must re-acknowledge before any offboarding delete.".to_owned(),
            bounded_by_expiry: true,
            widens_authority: false,
        },
    ];

    M5ExceptionExpiryPacket {
        schema_version: M5_EXCEPTION_EXPIRY_SCHEMA_VERSION,
        record_kind: M5_EXCEPTION_EXPIRY_RECORD_KIND.to_owned(),
        packet_id: "m5-exception-expiry:0001".to_owned(),
        shared_contract_ref: M5_EXCEPTION_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
        as_of: "2026-06-13T16:00:00Z".to_owned(),
        overview_doc_ref: M5_EXCEPTION_EXPIRY_DOC_REF.to_owned(),
        artifact_summary_ref: M5_EXCEPTION_EXPIRY_ARTIFACT_REF.to_owned(),
        records_contract_ref: M5_EXCEPTION_EXPIRY_RECORDS_CONTRACT_REF.to_owned(),
        rows,
        summary: "Time-bounded, actor-scoped exceptions gating the M5 hold/retention claims; \
                  each is pinned across actor/object/target/policy-epoch/environment and \
                  revalidated on drift."
            .to_owned(),
    }
}
