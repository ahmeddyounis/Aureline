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
pub const M5_EXCEPTION_EXPIRY_DOC_REF: &str = "docs/governance/m5_records_policy_sim.md";

/// Repo-relative artifact summary for the exception/expiry contract.
pub const M5_EXCEPTION_EXPIRY_ARTIFACT_REF: &str = "artifacts/governance/m5_records_policy_sim.md";

/// Reference to the records-side hold/retention contract this packet gates.
pub const M5_EXCEPTION_EXPIRY_RECORDS_CONTRACT_REF: &str = "records:m5_hold_retention_truth:v1";

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
    /// Behavior when the exception lapses.
    pub fallback_behavior_on_lapse: String,
    /// Whether the exception is bounded by an expiry (must be true).
    pub bounded_by_expiry: bool,
    /// Whether the exception widens authority (must be false).
    pub widens_authority: bool,
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
        }

        violations
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
