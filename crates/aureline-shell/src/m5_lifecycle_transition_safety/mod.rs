//! Canonical transition-safety certification for every long-lived M5 object family.
//!
//! The [frozen lifecycle matrix][matrix] already binds each long-lived M5 object family — the
//! workspace, the extension, the remote session, the collaboration session, the AI action, the
//! update/rollback, the notebook runtime, the request/API run, the preview session, the pipeline
//! run, the data session, the profiler capture, and the companion session — to an explicit state
//! machine drawn from the controlled lifecycle vocabulary. This lane is the **certification
//! capstone** that keeps each of those state machines *safe to move through*: for every governed
//! object family it certifies that the object **exposes safe retry / cancel / rollback /
//! compensation transition rules**, **attributes every transition to a controlled actor or
//! subsystem**, **cannot skip a required review / checkpoint / rollback state behind an anonymous
//! spinner**, and **keeps local editing as the protected fallback when the managed, collaborative,
//! AI, or remote lane degrades** — and that the same state-truth vocabulary survives a headless or
//! companion-adjacent execution rather than degrading into a surface-specific heuristic.
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`TransitionSafetyRow`]): one row per
//!   [`M5LifecycleObjectFamily`] naming the explicit state machine it certifies (pulled from the
//!   matrix), its safe-transition / attribution / checkpoint-sequencing / local-fallback posture,
//!   whether the same vocabulary survives headless/companion-adjacent execution, the consumer
//!   surfaces it evaluated, any active waiver, and a derived green/yellow/red
//!   [`TransitionSafetyStatus`].
//! - the release **certification packet** ([`TransitionSafetyPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   transition causes ([`TransitionSafetyCause`]), and the blocking findings the lane refuses to
//!   ship with.
//! - the **certification dashboard** ([`TransitionSafetyDashboard`]): a light projection the
//!   product UI / CLI / diagnostics / support / telemetry automation reads to auto-narrow a
//!   governed object family's transition-safety claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment
//! an object exposes a disclosed reduced transition set, discloses a coarse (rather than exact)
//! transition attribution, presents its required checkpoints in a disclosed compacted form, or
//! keeps a disclosed, waivered reduced local-editing fallback; it drops to `red` if an object
//! allows an unsafe or missing transition, stops attributing a transition to an actor or subsystem,
//! skips a required review/checkpoint/rollback state behind an anonymous spinner, loses its
//! protected local-editing fallback, loses the same state-truth vocabulary in a
//! headless/companion-adjacent execution, or fails to certify every consumer surface the matrix
//! declares for that family. That derivation is the auto-narrowing the acceptance criteria require,
//! and the consumer-surface completeness check is the lint that prevents a certification from
//! silently regressing into a partial, single-surface view — the exact regression that would let a
//! protected flow hide a half-ready or maybe-applied state behind one generic spinner on the
//! surfaces it did not certify.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The object family, state, recovery-affordance,
//! consumer-surface, downgrade-trigger, and qualification vocabulary is re-exported by reference
//! from the already frozen [matrix], and every object's explicit state machine and applicable
//! triggers are pulled straight from that matrix's seeded packet, so this lane mints no parallel
//! lifecycle vocabulary and cannot certify a family — or a transition — the matrix does not freeze.
//! Only the transition-safety-specific vocabulary ([`M5LifecycleTransitionDimension`],
//! [`TransitionSafetyStatus`], [`SafeTransitionState`], [`TransitionAttributionState`],
//! [`CheckpointSequencingState`], [`LocalFallbackState`], [`TransitionSafetyWaiver`],
//! [`TransitionSafetyCause`], [`TransitionSafetyFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix as matrix;

pub use matrix::{
    M5LifecycleConsumerSurface, M5LifecycleDowngradeTrigger, M5LifecycleObjectFamily,
    M5LifecycleQualificationClass, M5LifecycleState, M5RecoveryAffordance,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_lifecycle_transition_safety_packet,
    seeded_m5_lifecycle_transition_safety_packet_ai_action_unsafe_transition_blocked,
    seeded_m5_lifecycle_transition_safety_packet_data_local_fallback_lost_blocked,
    seeded_m5_lifecycle_transition_safety_packet_extension_headless_parity_lost_blocked,
    seeded_m5_lifecycle_transition_safety_packet_request_attribution_missing_blocked,
    seeded_m5_lifecycle_transition_safety_packet_update_checkpoint_skipped_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_SHARED_CONTRACT_REF: &str =
    "lifecycle:m5_lifecycle_transition_safety:v1";

/// Stable record kind for [`TransitionSafetyPacket`] payloads.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_transition_safety_packet_record";

/// Stable record kind for [`TransitionSafetyDashboard`] payloads.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_transition_safety_dashboard_record";

/// Stable record kind for [`TransitionSafetySupportExport`] payloads.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_transition_safety_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PACKET_ID: &str =
    "m5-lifecycle-transition-safety:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_DASHBOARD_ID: &str =
    "m5-lifecycle-transition-safety-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-lifecycle-transition-safety:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-lifecycle-transition-safety.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-lifecycle-transition-safety.md";

/// Published certification-packet artifact ref.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-lifecycle-transition-safety-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-lifecycle-transition-safety-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-lifecycle-transition-safety-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-lifecycle-transition-safety-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_lifecycle_transition_safety_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_MATRIX_DOC_REF: &str = matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for the explicit-state-machine binding.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the local-fallback binding.
pub const M5_LIFECYCLE_TRANSITION_SAFETY_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every governed long-lived object family the certification must cover, in canonical order.
/// These are exactly the families the frozen lifecycle matrix freezes; a certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_OBJECT_FAMILIES: [M5LifecycleObjectFamily; 13] = M5LifecycleObjectFamily::ALL;

/// Every transition-safety dimension each object row certifies, in canonical order.
pub const REQUIRED_TRANSITION_DIMENSIONS: [M5LifecycleTransitionDimension; 4] =
    M5LifecycleTransitionDimension::ALL;

/// One of the four transition-safety dimensions each object row certifies.
///
/// These are exactly the four ways the acceptance criteria require a long-lived M5 object's
/// explicit state machine to stay safe to move through: it exposes safe retry / cancel / rollback /
/// compensation transition rules, it attributes every transition to a controlled actor or
/// subsystem, it cannot skip a required review / checkpoint / rollback state behind an anonymous
/// spinner, and it keeps local editing as the protected fallback when a managed lane degrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleTransitionDimension {
    /// The object exposes safe retry / cancel / rollback / compensation transition rules.
    SafeTransition,
    /// Every transition is attributed to a controlled actor or subsystem.
    TransitionAttribution,
    /// Required review / checkpoint / rollback states cannot be skipped behind an anonymous spinner.
    CheckpointSequencing,
    /// Local editing stays the protected fallback when a managed lane degrades.
    LocalFallback,
}

impl M5LifecycleTransitionDimension {
    /// Every transition-safety dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SafeTransition,
        Self::TransitionAttribution,
        Self::CheckpointSequencing,
        Self::LocalFallback,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeTransition => "safe_transition",
            Self::TransitionAttribution => "transition_attribution",
            Self::CheckpointSequencing => "checkpoint_sequencing",
            Self::LocalFallback => "local_fallback",
        }
    }
}

/// The derived transition-safety light an object family carries.
///
/// `green` means the object exposes safe retry / cancel / rollback / compensation transition rules,
/// attributes every transition to a controlled actor or subsystem, keeps its required checkpoints
/// named rather than anonymous, and keeps local editing as the protected fallback — and the same
/// state-truth vocabulary survives a headless/companion-adjacent execution across every declared
/// consumer surface. `yellow` is a disclosed narrowing (a disclosed reduced transition set, a
/// disclosed coarse attribution, disclosed compacted checkpoints, or a waivered reduced local
/// fallback). `red` is blocked: an unsafe or missing transition, a missing transition attribution,
/// a skipped required checkpoint, a lost local-editing fallback, a headless/companion-adjacent
/// vocabulary loss, or a row that did not certify every declared consumer surface — and it may not
/// keep a transition-safety claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionSafetyStatus {
    /// Full standing: all four transition dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl TransitionSafetyStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the object exposes safe retry / cancel / rollback / compensation transition rules.
///
/// `safe_retry_cancel_rollback_rules` means every transition the object's explicit state machine
/// allows is restartable or compensatable: a retry cannot double-apply, a cancel cannot strand a
/// half-applied change, and a rollback or compensation path is always reachable.
/// `disclosed_reduced_transition_set` means the object exposes a disclosed reduced set of safe
/// transitions on a subset of surfaces — for example deferring cancel until a reconnect resolves —
/// while retry, rollback, and compensation stay safe (a yellow narrowing).
/// `unsafe_or_missing_transition_rules` means the object allowed an unsafe or missing transition
/// that could double-apply, strand, or skip its rollback/compensation — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeTransitionState {
    /// Every transition is a safe retry / cancel / rollback / compensation rule.
    SafeRetryCancelRollbackRules,
    /// The object exposes a disclosed reduced set of safe transitions.
    DisclosedReducedTransitionSet,
    /// The object allowed an unsafe or missing transition — a blocker.
    UnsafeOrMissingTransitionRules,
}

impl SafeTransitionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeRetryCancelRollbackRules => "safe_retry_cancel_rollback_rules",
            Self::DisclosedReducedTransitionSet => "disclosed_reduced_transition_set",
            Self::UnsafeOrMissingTransitionRules => "unsafe_or_missing_transition_rules",
        }
    }

    /// `true` when the object exposes safe transition rules at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::SafeRetryCancelRollbackRules)
    }

    /// `true` when the object took a disclosed reduced-transition-set narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedTransitionSet)
    }
}

/// How the object attributes every transition to a controlled actor or subsystem.
///
/// `actor_subsystem_attributed` means every transition names the controlled actor (a user, an
/// automation, a policy) or subsystem that drove it, so a retried or cancelled transition can
/// always be traced. `disclosed_coarse_attribution` means the object attributes a transition to a
/// disclosed coarse subsystem group rather than the exact actor until the specific attribution
/// resolves, while still naming a controlled subsystem (a yellow narrowing).
/// `attribution_missing_on_transition` means the object stopped attributing a transition to any
/// actor or subsystem — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionAttributionState {
    /// Every transition names its controlled actor or subsystem.
    ActorSubsystemAttributed,
    /// The object attributes a transition to a disclosed coarse subsystem group.
    DisclosedCoarseAttribution,
    /// The object stopped attributing a transition to any actor or subsystem — a blocker.
    AttributionMissingOnTransition,
}

impl TransitionAttributionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorSubsystemAttributed => "actor_subsystem_attributed",
            Self::DisclosedCoarseAttribution => "disclosed_coarse_attribution",
            Self::AttributionMissingOnTransition => "attribution_missing_on_transition",
        }
    }

    /// `true` when every transition is attributed at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ActorSubsystemAttributed)
    }

    /// `true` when the object took a disclosed coarse-attribution narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCoarseAttribution)
    }
}

/// How the object keeps its required review / checkpoint / rollback states from being skipped.
///
/// `required_checkpoints_enforced` means a protected journey always shows its required named
/// review, checkpoint, or rollback states in order and cannot skip one behind an anonymous spinner
/// or success banner. `disclosed_compacted_checkpoints` means the object presents its required
/// checkpoints in a disclosed compacted form on a compact surface while still naming each milestone
/// individually (a yellow narrowing). `required_checkpoint_skipped` means a protected journey
/// skipped a required review/checkpoint/rollback state or fell back to an anonymous spinner —
/// always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointSequencingState {
    /// Required review / checkpoint / rollback states are enforced and named.
    RequiredCheckpointsEnforced,
    /// The object presents its required checkpoints in a disclosed compacted form.
    DisclosedCompactedCheckpoints,
    /// The object skipped a required checkpoint or fell back to an anonymous spinner — a blocker.
    RequiredCheckpointSkipped,
}

impl CheckpointSequencingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredCheckpointsEnforced => "required_checkpoints_enforced",
            Self::DisclosedCompactedCheckpoints => "disclosed_compacted_checkpoints",
            Self::RequiredCheckpointSkipped => "required_checkpoint_skipped",
        }
    }

    /// `true` when required checkpoints are enforced at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::RequiredCheckpointsEnforced)
    }

    /// `true` when the object took a disclosed compacted-checkpoints narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCompactedCheckpoints)
    }
}

/// How the object keeps local editing as the protected fallback when a managed lane degrades.
///
/// `local_editing_protected_fallback` means that when the managed, collaborative, AI, or remote
/// lane degrades, local editing stays available as the protected fallback the user can always fall
/// back to. `disclosed_reduced_fallback` means the object keeps a disclosed, waivered reduced
/// local-editing fallback — for example continuing local edits read-only until the lane rejoins —
/// while still keeping a safe local path (a yellow narrowing). `local_fallback_lost` means the
/// object lost its protected local-editing fallback, leaving the user with no safe local path while
/// the lane is unavailable — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalFallbackState {
    /// Local editing stays the protected fallback when the managed lane degrades.
    LocalEditingProtectedFallback,
    /// The object keeps a disclosed, waivered reduced local-editing fallback.
    DisclosedReducedFallback,
    /// The object lost its protected local-editing fallback — a blocker.
    LocalFallbackLost,
}

impl LocalFallbackState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditingProtectedFallback => "local_editing_protected_fallback",
            Self::DisclosedReducedFallback => "disclosed_reduced_fallback",
            Self::LocalFallbackLost => "local_fallback_lost",
        }
    }

    /// `true` when local editing stays the protected fallback at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::LocalEditingProtectedFallback)
    }

    /// `true` when the object took a disclosed reduced-fallback narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedFallback)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow)
/// rather than blocked — never lets an unsafe transition, a missing attribution, a skipped
/// checkpoint, or a lost local fallback hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetyWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The object family the waiver applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl TransitionSafetyWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked an object family's transition-safety certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetyCause {
    /// The object family the cause applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause
    /// is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl TransitionSafetyCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed long-lived object family, certified across its safe-transition, attribution,
/// checkpoint-sequencing, and local-fallback dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetyRow {
    /// The object family being certified.
    pub object_family: M5LifecycleObjectFamily,
    /// Short reviewer-facing object label.
    pub object_label: String,
    /// Qualification class the matrix earned for this object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this object governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short scope summary. Pulled from the matrix.
    pub scope_summary: String,
    /// The controlled states the object's explicit state machine admits — the machine whose
    /// transitions this row certifies. Pulled from the matrix.
    pub admitted_states: Vec<M5LifecycleState>,
    /// The one named recovery affordance the local fallback anchors on. Pulled from the matrix.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Consumer surfaces the matrix declares this object must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Safe-transition posture.
    pub safe_transition: SafeTransitionState,
    /// Transition-attribution posture.
    pub transition_attribution: TransitionAttributionState,
    /// Checkpoint-sequencing posture.
    pub checkpoint_sequencing: CheckpointSequencingState,
    /// Local-fallback posture.
    pub local_fallback: LocalFallbackState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to this object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced local fallback is in force.
    pub active_waiver: Option<TransitionSafetyWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: TransitionSafetyStatus,
    /// The exact transition causes that narrowed or blocked this row.
    pub transition_causes: Vec<TransitionSafetyCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl TransitionSafetyRow {
    /// `true` when the row certified every consumer surface the matrix declares for this object —
    /// no declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.safe_transition,
            SafeTransitionState::UnsafeOrMissingTransitionRules
        ) {
            return true;
        }
        if matches!(
            self.transition_attribution,
            TransitionAttributionState::AttributionMissingOnTransition
        ) {
            return true;
        }
        if matches!(
            self.checkpoint_sequencing,
            CheckpointSequencingState::RequiredCheckpointSkipped
        ) {
            return true;
        }
        if matches!(self.local_fallback, LocalFallbackState::LocalFallbackLost) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.safe_transition.is_disclosed_narrowing()
            || self.transition_attribution.is_disclosed_narrowing()
            || self.checkpoint_sequencing.is_disclosed_narrowing()
            || self.local_fallback.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the object posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> TransitionSafetyStatus {
        if self.has_hard_blocker() {
            TransitionSafetyStatus::Red
        } else if self.has_narrowing() {
            TransitionSafetyStatus::Yellow
        } else {
            TransitionSafetyStatus::Green
        }
    }

    /// Recomputes the exact transition causes for the row, in deterministic order (safe-transition,
    /// attribution, checkpoint-sequencing, local-fallback, then headless parity).
    pub fn recompute_causes(&self) -> Vec<TransitionSafetyCause> {
        let mut causes = Vec::new();
        match self.safe_transition {
            SafeTransitionState::SafeRetryCancelRollbackRules => {}
            SafeTransitionState::DisclosedReducedTransitionSet => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object exposes a disclosed reduced set of safe transitions on a \
                             subset of surfaces — for example deferring cancel until a reconnect or \
                             checkpoint resolves — while retry, rollback, and compensation stay \
                             safe, so the transition set is narrowed and disclosed rather than \
                             unsafe."
                        .to_owned(),
                });
            }
            SafeTransitionState::UnsafeOrMissingTransitionRules => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "The object's explicit state machine allowed an unsafe or missing \
                             transition — a retry or cancel that could double-apply, strand, or skip \
                             its rollback or compensation — so a half-ready or maybe-applied state \
                             can no longer be safely restarted or compensated."
                        .to_owned(),
                });
            }
        }
        match self.transition_attribution {
            TransitionAttributionState::ActorSubsystemAttributed => {}
            TransitionAttributionState::DisclosedCoarseAttribution => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object attributes a transition to a disclosed coarse subsystem \
                             group rather than the exact actor until the specific attribution \
                             resolves, while still naming a controlled subsystem, so attribution is \
                             narrowed and disclosed rather than missing."
                        .to_owned(),
                });
            }
            TransitionAttributionState::AttributionMissingOnTransition => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::LastFailureReasonMissing,
                    disclosed: false,
                    detail: "The object stopped attributing its transitions to a controlled actor \
                             or subsystem, so a failed, retried, or cancelled transition can no \
                             longer be traced to who or what drove it."
                        .to_owned(),
                });
            }
        }
        match self.checkpoint_sequencing {
            CheckpointSequencingState::RequiredCheckpointsEnforced => {}
            CheckpointSequencingState::DisclosedCompactedCheckpoints => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail:
                        "The object presents its required checkpoints in a disclosed compacted \
                             form on a compact surface while still naming each milestone \
                             individually, so the checkpoint sequence is narrowed and disclosed \
                             rather than collapsing into an anonymous spinner."
                            .to_owned(),
                });
            }
            CheckpointSequencingState::RequiredCheckpointSkipped => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::AnonymousCheckpoint,
                    disclosed: false,
                    detail: "A protected journey skipped a required review, checkpoint, or rollback \
                             state or fell back to an anonymous spinner, so a half-ready or \
                             maybe-applied state hides behind one generic progress indicator instead \
                             of a named checkpoint."
                        .to_owned(),
                });
            }
        }
        match self.local_fallback {
            LocalFallbackState::LocalEditingProtectedFallback => {}
            LocalFallbackState::DisclosedReducedFallback => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "When the managed, collaborative, AI, or remote lane degrades, the \
                             object keeps a disclosed, waivered reduced local-editing fallback — for \
                             example continuing local edits read-only until the lane rejoins — while \
                             still keeping a safe local path, so the fallback is narrowed and \
                             disclosed rather than lost."
                        .to_owned(),
                });
            }
            LocalFallbackState::LocalFallbackLost => {
                causes.push(TransitionSafetyCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing,
                    disclosed: false,
                    detail: "When the managed, collaborative, AI, or remote lane degraded, the \
                             object lost its protected local-editing fallback, leaving the user with \
                             no safe local path forward while the lane is unavailable."
                        .to_owned(),
                });
            }
        }
        if !self.headless_parity_preserved {
            causes.push(TransitionSafetyCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail:
                    "A headless or companion-adjacent execution of this object lost the shared \
                         state-truth vocabulary for its transitions, so the same object reports a \
                         different transition and state language depending on how it runs."
                        .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced local fallback may only stay yellow (rather than red) when a waiver
    /// discloses it — reducing the protected local-editing fallback is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.local_fallback,
            LocalFallbackState::DisclosedReducedFallback
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<TransitionSafetyFinding> {
        let mut findings = Vec::new();
        let object = self.object_family.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(TransitionSafetyFinding::ConsumerSurfacesIncomplete {
                object: object.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(TransitionSafetyFinding::HeadlessParityLost {
                object: object.clone(),
            });
        }
        if matches!(
            self.safe_transition,
            SafeTransitionState::UnsafeOrMissingTransitionRules
        ) {
            findings.push(TransitionSafetyFinding::UnsafeOrMissingTransitionRules {
                object: object.clone(),
            });
        }
        if matches!(
            self.transition_attribution,
            TransitionAttributionState::AttributionMissingOnTransition
        ) {
            findings.push(TransitionSafetyFinding::AttributionMissingOnTransition {
                object: object.clone(),
            });
        }
        if matches!(
            self.checkpoint_sequencing,
            CheckpointSequencingState::RequiredCheckpointSkipped
        ) {
            findings.push(TransitionSafetyFinding::RequiredCheckpointSkipped {
                object: object.clone(),
            });
        }
        if matches!(self.local_fallback, LocalFallbackState::LocalFallbackLost) {
            findings.push(TransitionSafetyFinding::LocalFallbackLost {
                object: object.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, TransitionSafetyStatus::Green) && !self.has_reason() {
            findings.push(TransitionSafetyFinding::NarrowedRowWithoutReason {
                object: object.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(TransitionSafetyFinding::NarrowedRowWithoutWaiver {
                object: object.clone(),
            });
        }
        // An attached waiver must still be active and must point at this object family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_family != self.object_family {
                findings.push(TransitionSafetyFinding::WaiverObjectMismatch {
                    object: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(TransitionSafetyFinding::WaiverExpired {
                    object: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(TransitionSafetyFinding::RowStatusStale {
                object: object.clone(),
            });
        }
        if self.transition_causes != self.recompute_causes() {
            findings.push(TransitionSafetyFinding::RowCausesStale { object });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} transitions={} attribution={} checkpoints={} fallback={} headless={} surfaces={} waiver={}",
            self.object_family.as_str(),
            self.derived_status.as_str(),
            self.safe_transition.as_str(),
            self.transition_attribution.as_str(),
            self.checkpoint_sequencing.as_str(),
            self.local_fallback.as_str(),
            self.headless_parity_preserved,
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the transition-safety certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum TransitionSafetyFinding {
    /// A governed object family has no certification row.
    ObjectMissing {
        /// The missing object token.
        object: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The object token.
        object: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The object token.
        object: String,
    },
    /// The object allowed an unsafe or missing transition.
    UnsafeOrMissingTransitionRules {
        /// The object token.
        object: String,
    },
    /// The object stopped attributing a transition to any actor or subsystem.
    AttributionMissingOnTransition {
        /// The object token.
        object: String,
    },
    /// A protected journey skipped a required checkpoint or fell back to an anonymous spinner.
    RequiredCheckpointSkipped {
        /// The object token.
        object: String,
    },
    /// The object lost its protected local-editing fallback.
    LocalFallbackLost {
        /// The object token.
        object: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The object token.
        object: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The object token.
        object: String,
    },
    /// An attached waiver does not point at the row's object family.
    WaiverObjectMismatch {
        /// The object token.
        object: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The object token.
        object: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The object token.
        object: String,
    },
    /// The declared transition causes do not match the recomputed causes.
    RowCausesStale {
        /// The object token.
        object: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered object families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl TransitionSafetyFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectMissing { .. } => "object_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::UnsafeOrMissingTransitionRules { .. } => "unsafe_or_missing_transition_rules",
            Self::AttributionMissingOnTransition { .. } => "attribution_missing_on_transition",
            Self::RequiredCheckpointSkipped { .. } => "required_checkpoint_skipped",
            Self::LocalFallbackLost { .. } => "local_fallback_lost",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverObjectMismatch { .. } => "waiver_object_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ObjectMissing { object }
            | Self::ConsumerSurfacesIncomplete { object }
            | Self::HeadlessParityLost { object }
            | Self::UnsafeOrMissingTransitionRules { object }
            | Self::AttributionMissingOnTransition { object }
            | Self::RequiredCheckpointSkipped { object }
            | Self::LocalFallbackLost { object }
            | Self::NarrowedRowWithoutReason { object }
            | Self::NarrowedRowWithoutWaiver { object }
            | Self::WaiverObjectMismatch { object, .. }
            | Self::WaiverExpired { object, .. }
            | Self::RowStatusStale { object }
            | Self::RowCausesStale { object } => object,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release transition-safety certification packet shared by the product UI / CLI / diagnostics /
/// support / telemetry automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetyPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen lifecycle matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen lifecycle object-state schema.
    pub object_state_schema_ref: String,
    /// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
    pub journey_checkpoint_schema_ref: String,
    /// Frozen lifecycle-matrix contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// State-object inventory this proof mirrors for the explicit-state-machine binding.
    pub state_object_inventory_ref: String,
    /// State-class recovery reference this proof mirrors for the local-fallback binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four transition-safety dimensions every object row certifies.
    pub required_transition_dimensions: Vec<String>,
    /// The thirteen governed object families the certification must cover.
    pub required_object_families: Vec<String>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<TransitionSafetyRow>,
    /// Object families certified, in canonical (sorted) order.
    pub covered_object_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-safety) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<TransitionSafetyWaiver>,
    /// Every exact transition cause, in row then cause order.
    pub transition_causes: Vec<TransitionSafetyCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<TransitionSafetyFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Lifecycle / release automation refs that consume this packet to auto-narrow governed
    /// objects.
    pub lifecycle_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TransitionSafetyPacket {
    /// Returns the certification row for `object_family`, if present.
    pub fn row(&self, object_family: M5LifecycleObjectFamily) -> Option<&TransitionSafetyRow> {
        self.rows
            .iter()
            .find(|row| row.object_family == object_family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.object_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.transition_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.object_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the lifecycle automation consumes.
    pub fn dashboard(&self) -> TransitionSafetyDashboard {
        TransitionSafetyDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 lifecycle-transition-safety packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per object family naming its
    /// status, the four transition postures, headless parity, the evaluated-surface count, and the
    /// waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_family,status,safe_transition,transition_attribution,checkpoint_sequencing,local_fallback,headless_parity,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.object_family.as_str(),
                row.derived_status.as_str(),
                row.safe_transition.as_str(),
                row.transition_attribution.as_str(),
                row.checkpoint_sequencing.as_str(),
                row.local_fallback.as_str(),
                row.headless_parity_preserved,
                row.evaluated_consumer_surfaces.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 lifecycle transition safety: safe retry/cancel/rollback rules, transition attribution, checkpoint sequencing, and the protected local-editing fallback on every long-lived M5 object\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_lifecycle_transition_safety`](../../crates/aureline-shell/src/m5_lifecycle_transition_safety/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety -- markdown > \\\n  artifacts/lifecycle/m5-lifecycle-transition-safety.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required transition dimensions: {}\n",
            self.required_transition_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Object families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full safety): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Object family | Status | Safe transition | Attribution | Checkpoints | Local fallback | Headless | Waiver |\n\
             | ------------- | ------ | --------------- | ----------- | ----------- | -------------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.object_label,
                row.derived_status.as_str(),
                row.safe_transition.as_str(),
                row.transition_attribution.as_str(),
                row.checkpoint_sequencing.as_str(),
                row.local_fallback.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&TransitionSafetyRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, TransitionSafetyStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every long-lived M5 object exposes safe retry/cancel/rollback/compensation transition rules, attributes every transition to a controlled actor or subsystem, keeps its required checkpoints named, and keeps local editing as the protected fallback across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.object_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact transition causes\n\n");
        if self.transition_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.transition_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.object_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.object_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_lifecycle_transition_safety_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetyDashboardRow {
    /// The object family.
    pub object_family: M5LifecycleObjectFamily,
    /// Short object label.
    pub object_label: String,
    /// Derived green/yellow/red status.
    pub status: TransitionSafetyStatus,
    /// Number of declared consumer surfaces certified for this object.
    pub evaluated_surface_count: usize,
    /// Safe-transition posture.
    pub safe_transition: SafeTransitionState,
    /// Transition-attribution posture.
    pub transition_attribution: TransitionAttributionState,
    /// Checkpoint-sequencing posture.
    pub checkpoint_sequencing: CheckpointSequencingState,
    /// Local-fallback posture.
    pub local_fallback: LocalFallbackState,
    /// `true` when headless/companion-adjacent parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the product UI / CLI / diagnostics / support / telemetry
/// automation reads to auto-narrow a governed object family's transition-safety claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetyDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<TransitionSafetyDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Lifecycle / release automation refs that consume the dashboard.
    pub lifecycle_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl TransitionSafetyDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &TransitionSafetyPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| TransitionSafetyDashboardRow {
                object_family: row.object_family,
                object_label: row.object_label.clone(),
                status: row.derived_status,
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                safe_transition: row.safe_transition,
                transition_attribution: row.transition_attribution,
                checkpoint_sequencing: row.checkpoint_sequencing,
                local_fallback: row.local_fallback,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .transition_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_LIFECYCLE_TRANSITION_SAFETY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_TRANSITION_SAFETY_SCHEMA_VERSION,
            dashboard_id: M5_LIFECYCLE_TRANSITION_SAFETY_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            lifecycle_automation_refs: packet.lifecycle_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 lifecycle-transition-safety dashboard serializes")
    }
}

/// Support-export wrapper for the transition-safety certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSafetySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: TransitionSafetyPacket,
    /// Dashboard quoted in full.
    pub dashboard: TransitionSafetyDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl TransitionSafetySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the lifecycle automation
    /// — can name the same object and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: TransitionSafetyPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.object_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_LIFECYCLE_TRANSITION_SAFETY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_TRANSITION_SAFETY_SCHEMA_VERSION,
            shared_contract_ref: M5_LIFECYCLE_TRANSITION_SAFETY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_lifecycle_transition_safety_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionSafetyInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<TransitionSafetyRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`TransitionSafetyPacket`] from the exact build identity, the frozen matrix ref, and
/// the per-family certification rows.
///
/// Each row's derived status and transition causes, the aggregate counts, the active waivers, and
/// the blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_lifecycle_transition_safety_packet(
    input: TransitionSafetyInput,
) -> TransitionSafetyPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<TransitionSafetyRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.transition_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<TransitionSafetyFinding> = Vec::new();

    // Every governed object family must carry a certification row.
    let present: BTreeSet<M5LifecycleObjectFamily> =
        rows.iter().map(|row| row.object_family).collect();
    for object_family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&object_family) {
            blocking_findings.push(TransitionSafetyFinding::ObjectMissing {
                object: object_family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_object_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|object_family| object_family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TransitionSafetyStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TransitionSafetyStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TransitionSafetyStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(TransitionSafetyFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<TransitionSafetyWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let transition_causes: Vec<TransitionSafetyCause> = rows
        .iter()
        .flat_map(|row| row.transition_causes.clone())
        .collect();

    let required_transition_dimensions: Vec<String> = REQUIRED_TRANSITION_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|object_family| object_family.as_str().to_owned())
        .collect();

    let mut packet = TransitionSafetyPacket {
        record_kind: M5_LIFECYCLE_TRANSITION_SAFETY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_LIFECYCLE_TRANSITION_SAFETY_SCHEMA_VERSION,
        shared_contract_ref: M5_LIFECYCLE_TRANSITION_SAFETY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_LIFECYCLE_TRANSITION_SAFETY_PACKET_ID.to_owned(),
        source_schema_ref: M5_LIFECYCLE_TRANSITION_SAFETY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Explicit state machines and safe retry/cancel semantics on every long-lived M5 \
                   object: the workspace, extension, remote session, collaboration session, AI \
                   action, update/rollback, notebook runtime, request/API run, preview session, \
                   pipeline run, data session, profiler capture, and companion session each \
                   certified so the object exposes safe retry/cancel/rollback/compensation \
                   transition rules, attributes every transition to a controlled actor or \
                   subsystem, cannot skip a required review/checkpoint/rollback state behind an \
                   anonymous spinner, and keeps local editing as the protected fallback when a \
                   managed lane degrades — across every declared consumer surface, with the same \
                   state-truth vocabulary preserved in headless and companion-adjacent execution — \
                   and each object's green/yellow/red claim auto-narrowed from its four transition \
                   postures."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_LIFECYCLE_TRANSITION_SAFETY_OBJECT_STATE_SCHEMA_REF.to_owned(),
        journey_checkpoint_schema_ref: M5_LIFECYCLE_TRANSITION_SAFETY_JOURNEY_CHECKPOINT_SCHEMA_REF
            .to_owned(),
        matrix_doc_ref: M5_LIFECYCLE_TRANSITION_SAFETY_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_LIFECYCLE_TRANSITION_SAFETY_STATE_OBJECT_INVENTORY_REF
            .to_owned(),
        state_class_recovery_ref: M5_LIFECYCLE_TRANSITION_SAFETY_STATE_CLASS_RECOVERY_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_transition_dimensions,
        required_object_families,
        rows,
        covered_object_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        transition_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.transition_safety_registry".to_owned(),
            "release_automation.auto_narrow.lifecycle_transition_safety_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.lifecycle_transition_safety".to_owned(),
            M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-lifecycle-transition-safety".to_owned()],
        published_report_ref: M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_LIFECYCLE_TRANSITION_SAFETY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(TransitionSafetyFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_lifecycle_transition_safety_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum TransitionSafetyValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required transition dimensions do not match the lane constants.
    RequiredTransitionDimensionsStale,
    /// The declared required object families do not match the lane constants.
    RequiredObjectFamiliesStale,
    /// The rows do not cover all thirteen governed object families.
    CoverageIncomplete,
    /// The declared covered object families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared transition causes do not match the recomputed causes.
    TransitionCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the transition-safety certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed object family
/// carries a current certification row; each row's status is the derived auto-narrowed value, never
/// asserted; a green row cannot keep a claim while a transition is unsafe or missing, a transition
/// attribution goes missing, a required review/checkpoint/rollback state is skipped, the protected
/// local-editing fallback is lost, headless/companion-adjacent parity is lost, or the row fails to
/// certify every declared consumer surface; and a disclosed narrowing is backed by a reason and,
/// where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_lifecycle_transition_safety_packet(
    packet: &TransitionSafetyPacket,
) -> Result<(), Vec<TransitionSafetyValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(TransitionSafetyValidationError::NoRows);
    }
    if packet.record_kind != M5_LIFECYCLE_TRANSITION_SAFETY_PACKET_RECORD_KIND {
        errors.push(TransitionSafetyValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_LIFECYCLE_TRANSITION_SAFETY_SCHEMA_VERSION {
        errors.push(TransitionSafetyValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(TransitionSafetyValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(TransitionSafetyValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_TRANSITION_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_transition_dimensions != expected_dimensions {
        errors.push(TransitionSafetyValidationError::RequiredTransitionDimensionsStale);
    }
    let expected_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|object_family| object_family.as_str().to_owned())
        .collect();
    if packet.required_object_families != expected_object_families {
        errors.push(TransitionSafetyValidationError::RequiredObjectFamiliesStale);
    }

    let present: BTreeSet<M5LifecycleObjectFamily> =
        packet.rows.iter().map(|row| row.object_family).collect();
    let coverage_complete = REQUIRED_OBJECT_FAMILIES
        .iter()
        .all(|object_family| present.contains(object_family));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_FAMILIES.len() {
        errors.push(TransitionSafetyValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|object_family| object_family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_object_families {
        errors.push(TransitionSafetyValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TransitionSafetyStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TransitionSafetyStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TransitionSafetyStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(TransitionSafetyValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<TransitionSafetyWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(TransitionSafetyValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<TransitionSafetyCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.transition_causes {
        errors.push(TransitionSafetyValidationError::TransitionCausesStale);
    }

    let mut recomputed: Vec<TransitionSafetyFinding> = Vec::new();
    for object_family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&object_family) {
            recomputed.push(TransitionSafetyFinding::ObjectMissing {
                object: object_family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(TransitionSafetyFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(TransitionSafetyFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(TransitionSafetyValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(TransitionSafetyValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(TransitionSafetyValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(TransitionSafetyValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(TransitionSafetyValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(TransitionSafetyValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
