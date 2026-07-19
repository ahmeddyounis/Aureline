//! Relation strips and sync-pending pills carrying linked engineering context,
//! stale/broken relation labeling, and retry-or-export publish-later continuity.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_work_item_component_matrix`] — the `relation_strip` and
//! the `sync_pending_pill` — into one implemented, export-safe packet with two
//! co-equal control vectors. Together they keep code/review/test/incident context
//! and unsent local state compact but truthful in list and side-rail surfaces.
//!
//! A [`RelationStrip`] summarizes the linked branch/worktree, hosted-review,
//! failing-test/incident, and docs/ADR cues attached to a work item, naming *each*
//! linked context by kind and reference rather than collapsing several links into a
//! single vague `Linked` label. Every relation carries a derived
//! [`RelationHealthClass`] — current, stale, broken, or unmapped — so a dangling or
//! out-of-date link is labeled as such instead of reading as live, and every
//! relation offers metadata-safe copy/open actions.
//!
//! A [`SyncPendingPill`] discloses what local change is pending (a comment, a
//! transition, a link, a field edit, or a create), the last sync attempt, and a
//! retry-or-export recovery action, and it is derived to read *visibly differently*
//! from a provider-confirmed state: a local draft not yet published, a failed
//! publish, or an offline-held change can never present as reconciled with the
//! provider, and it stays recoverable when publish fails or the provider is offline.
//! A policy-blocked pill names its policy block explicitly.
//!
//! The relation kinds ([`M5WorkItemRelationKind`]), local-versus-provider states
//! ([`M5WorkItemLocalState`]), surface families ([`M5WorkItemSurfaceFamily`]),
//! deployment lines ([`M5WorkItemDeploymentLine`]), consumer surfaces
//! ([`M5WorkItemConsumerSurface`]), accessibility routes
//! ([`M5WorkItemAccessibilityRoute`]), and downgrade triggers
//! ([`M5WorkItemDowngradeTrigger`]) are reused directly from the frozen matrix, so
//! this lane never invents a parallel work-item vocabulary. It mints new vocabulary
//! only for what that matrix left implicit about these two controls: the derived
//! relation health class, the metadata-safe relation actions, the pending-change
//! type, the derived sync-recovery class, and the sync-pill recovery actions.
//!
//! Raw work-item bodies, pasted paths, credentials, and private endpoints stay
//! outside the support boundary; canonical ids and relation references are carried
//! only as opaque, export-safe strings.
//!
//! The boundary schema is
//! [`schemas/ui/m5-relation-strip-sync-pending-controls.schema.json`](../../../../schemas/ui/m5-relation-strip-sync-pending-controls.schema.json).
//! The contract doc is
//! [`docs/team-workflows/implement_relation_strips_and_sync_pending_pills.md`](../../../../docs/team-workflows/implement_relation_strips_and_sync_pending_pills.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_relation_strip_sync_pending_controls,
    seeded_relation_strip_sync_pending_controls_relation_strip_stale_relation,
    seeded_relation_strip_sync_pending_controls_sync_pending_recoverable_failure,
    RELATION_STRIP_SYNC_PENDING_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The relation kind, local-versus-provider state, surface family, deployment line,
// consumer surface, accessibility route, and downgrade triggers are frozen once, in
// the work-item component matrix. This lane reuses them verbatim so it never invents
// a parallel work-item vocabulary.
use crate::freeze_the_m5_work_item_component_matrix::{
    M5WorkItemAccessibilityRoute, M5WorkItemComponentFamily, M5WorkItemConsumerSurface,
    M5WorkItemDeploymentLine, M5WorkItemDowngradeTrigger, M5WorkItemLocalState,
    M5WorkItemRelationKind, M5WorkItemSurfaceFamily, M5_RELATION_STRIP_SCHEMA_REF,
    M5_SYNC_PENDING_PILL_SCHEMA_REF, M5_WORK_ITEM_COMPONENT_DOC_REF,
    M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`RelationStripSyncPendingControlsPacket`].
pub const RELATION_STRIP_SYNC_PENDING_RECORD_KIND: &str = "relation_strip_sync_pending_controls";

/// Schema version for relation-strip / sync-pending-pill control records.
pub const RELATION_STRIP_SYNC_PENDING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RELATION_STRIP_SYNC_PENDING_SCHEMA_REF: &str =
    "schemas/ui/m5-relation-strip-sync-pending-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const RELATION_STRIP_SYNC_PENDING_DOC_REF: &str =
    "docs/team-workflows/implement_relation_strips_and_sync_pending_pills.md";

/// Repo-relative path of the protected fixture directory.
pub const RELATION_STRIP_SYNC_PENDING_FIXTURE_DIR: &str =
    "fixtures/ui/m5-relation-strip-sync-pending-controls";

/// Repo-relative path of the checked support-export artifact.
pub const RELATION_STRIP_SYNC_PENDING_ARTIFACT_REF: &str =
    "artifacts/release/m5-relation-strip-sync-pending-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RELATION_STRIP_SYNC_PENDING_SUMMARY_REF: &str =
    "artifacts/release/m5-relation-strip-sync-pending-proof/summary.md";

// ---- relation-strip vocabulary ------------------------------------------

/// Derived health class a single linked relation may present.
///
/// This is the relation honesty axis: the class is derived from whether the linked
/// target is still reachable and whether the reference is current, never asserted,
/// so a dangling or stale link is labeled as such instead of reading as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationHealthClass {
    /// The linked target is reachable and the reference is current.
    Current,
    /// The linked target is reachable but the reference is out of date.
    Stale,
    /// The linked target can no longer be resolved.
    Broken,
    /// The relation is unmapped / dangling and points at no resolved target.
    Unmapped,
}

impl RelationHealthClass {
    /// Every relation health class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Broken, Self::Unmapped];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Broken => "broken",
            Self::Unmapped => "unmapped",
        }
    }

    /// Whether this class is a live, current relation.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// One keyboard-complete, metadata-safe action a relation strip offers per relation,
/// so a linked context never hides its copy or open affordance behind a pointer-only
/// gesture and never exports raw bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationStripAction {
    /// Copy the metadata-safe reference (always available).
    CopyReference,
    /// Open the linked relation in its owning surface (always available).
    OpenRelation,
    /// Reveal the relation's scope / provenance.
    RevealScope,
    /// Export the relation as metadata-safe evidence.
    ExportRelation,
}

impl RelationStripAction {
    /// Every relation action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CopyReference,
        Self::OpenRelation,
        Self::RevealScope,
        Self::ExportRelation,
    ];

    /// The metadata-safe copy/open actions every relation must offer.
    pub const MANDATORY: [Self; 2] = [Self::CopyReference, Self::OpenRelation];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyReference => "copy_reference",
            Self::OpenRelation => "open_relation",
            Self::RevealScope => "reveal_scope",
            Self::ExportRelation => "export_relation",
        }
    }
}

/// Disclosures a single relation must carry, derived from reachability and freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationEntryDisclosure {
    /// The derived health class this relation may present.
    pub health_class: RelationHealthClass,
    /// Whether the relation is a live, current link.
    pub is_current: bool,
    /// Whether the relation must carry an explicit stale/broken/unmapped note.
    pub needs_relation_note: bool,
}

/// Resolves the health truth a single relation may present.
///
/// An unmapped relation kind is unmapped regardless of reachability. Otherwise an
/// unreachable target is broken, a reachable-but-out-of-date reference is stale, and
/// only a reachable, current reference is current.
pub fn resolve_relation_health(
    relation_kind: M5WorkItemRelationKind,
    is_target_reachable: bool,
    is_reference_current: bool,
) -> RelationEntryDisclosure {
    let health_class = if matches!(relation_kind, M5WorkItemRelationKind::UnmappedRelation) {
        RelationHealthClass::Unmapped
    } else if !is_target_reachable {
        RelationHealthClass::Broken
    } else if !is_reference_current {
        RelationHealthClass::Stale
    } else {
        RelationHealthClass::Current
    };

    RelationEntryDisclosure {
        health_class,
        is_current: health_class.is_current(),
        needs_relation_note: !health_class.is_current(),
    }
}

/// One linked relation named by a relation strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationEntry {
    /// Relation kind, reused from the frozen matrix.
    pub relation_kind: M5WorkItemRelationKind,
    /// Metadata-safe reference label; required, non-empty, and distinct per strip.
    pub reference_label: String,
    /// Whether the linked target is still reachable.
    pub is_target_reachable: bool,
    /// Whether the linked reference is current (not out of date).
    pub is_reference_current: bool,
    /// Derived health class (must equal the resolved class).
    pub health_class: RelationHealthClass,
    /// Stale/broken/unmapped note; required when the relation is not current.
    pub relation_note: String,
    /// Metadata-safe actions this relation offers (must include the mandatory copy/open).
    pub actions: Vec<RelationStripAction>,
}

impl RelationEntry {
    /// Health disclosures this relation must carry, derived from reachability and freshness.
    pub fn health_disclosure(&self) -> RelationEntryDisclosure {
        resolve_relation_health(
            self.relation_kind,
            self.is_target_reachable,
            self.is_reference_current,
        )
    }

    /// Whether the relation offers every mandatory metadata-safe action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RelationStripAction> = self.actions.iter().copied().collect();
        RelationStripAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

/// A relation strip summarizing the linked engineering context of a work item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationStrip {
    /// Frozen component this control implements; must be `relation_strip`.
    pub component: M5WorkItemComponentFamily,
    /// Stable strip id.
    pub strip_id: String,
    /// Canonical id of the work item this strip belongs to; always non-empty.
    pub canonical_id: String,
    /// The linked relations, each named by kind and reference (never collapsed).
    pub relations: Vec<RelationEntry>,
    /// Hard invariant: multiple linked contexts are never collapsed into a single
    /// vague `Linked` label. MUST be `false`.
    pub collapses_into_generic_linked_label: bool,
    /// Claimed M5 work-item surface families that render this strip.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this strip keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this strip offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this strip's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this strip.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals linked context.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl RelationStrip {
    /// Whether the strip labels every relation distinctly (no duplicate collapsed labels).
    fn labels_each_relation_distinctly(&self) -> bool {
        let distinct: BTreeSet<&str> = self
            .relations
            .iter()
            .map(|relation| relation.reference_label.trim())
            .collect();
        distinct.len() == self.relations.len()
    }
}

// ---- sync-pending-pill vocabulary ---------------------------------------

/// The kind of local change a sync-pending pill discloses as not yet published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingChangeType {
    /// A pending comment.
    PendingComment,
    /// A pending status transition.
    PendingTransition,
    /// A pending link / relation change.
    PendingLink,
    /// A pending field edit.
    PendingFieldEdit,
    /// A pending create of a new work item.
    PendingCreate,
}

impl PendingChangeType {
    /// Every pending change type, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PendingComment,
        Self::PendingTransition,
        Self::PendingLink,
        Self::PendingFieldEdit,
        Self::PendingCreate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingComment => "pending_comment",
            Self::PendingTransition => "pending_transition",
            Self::PendingLink => "pending_link",
            Self::PendingFieldEdit => "pending_field_edit",
            Self::PendingCreate => "pending_create",
        }
    }
}

/// Derived sync-recovery class a sync-pending pill may present.
///
/// This is the sync honesty axis: the class is derived from the local-versus-provider
/// state, the policy-block flag, and provider reachability, never asserted, so a
/// pending, failed, or offline-held change can never read as provider-confirmed and
/// always names a recovery path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncRecoveryClass {
    /// The change is reconciled and confirmed by the provider.
    ProviderConfirmed,
    /// A local change is pending publish to the provider.
    PendingPublish,
    /// A prior publish attempt failed and is recoverable via retry or export.
    RecoverableFailure,
    /// The provider is offline; the change is held locally and stays recoverable.
    OfflineHeld,
    /// The change is blocked by policy and cannot be published.
    PolicyBlocked,
}

impl SyncRecoveryClass {
    /// Every sync-recovery class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderConfirmed,
        Self::PendingPublish,
        Self::RecoverableFailure,
        Self::OfflineHeld,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderConfirmed => "provider_confirmed",
            Self::PendingPublish => "pending_publish",
            Self::RecoverableFailure => "recoverable_failure",
            Self::OfflineHeld => "offline_held",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One recovery action a sync-pending pill offers, so a failed or offline publish
/// always stays recoverable via retry or export instead of stranding the change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncPillAction {
    /// Retry the publish to the provider.
    RetryPublish,
    /// Export the pending change as a metadata-safe packet.
    ExportPacket,
    /// Open the item in the owning provider.
    OpenInProvider,
    /// View the held conflict.
    ViewConflict,
    /// Discard the local draft.
    DiscardDraft,
}

impl SyncPillAction {
    /// Every sync-pill action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RetryPublish,
        Self::ExportPacket,
        Self::OpenInProvider,
        Self::ViewConflict,
        Self::DiscardDraft,
    ];

    /// The recovery actions that keep a pending change recoverable; a recoverable
    /// pill must offer at least one of these.
    pub const RECOVERY: [Self; 2] = [Self::RetryPublish, Self::ExportPacket];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryPublish => "retry_publish",
            Self::ExportPacket => "export_packet",
            Self::OpenInProvider => "open_in_provider",
            Self::ViewConflict => "view_conflict",
            Self::DiscardDraft => "discard_draft",
        }
    }
}

/// Disclosures a sync-pending pill must carry, derived from state, policy, and reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncPillDisclosure {
    /// The derived sync-recovery class this pill may present.
    pub recovery_class: SyncRecoveryClass,
    /// Whether the pill's visible state is provider-confirmed.
    pub is_provider_confirmed: bool,
    /// Whether the pill is a pending (not confirmed, not blocked) state.
    pub is_pending: bool,
    /// Whether the pill must read visibly differently from a provider-confirmed state.
    pub needs_distinct_style: bool,
    /// Whether the pill must name its last sync attempt.
    pub needs_last_sync_attempt: bool,
    /// Whether the pill must offer a retry-or-export recovery action.
    pub needs_recovery_action: bool,
    /// Whether the pill must carry an explicit policy-block note.
    pub needs_policy_block_note: bool,
}

/// Resolves the sync-recovery truth a sync-pending pill may present.
///
/// A policy-blocked pill is policy-blocked regardless of state. Otherwise a synced
/// pill is provider-confirmed, a failed publish is a recoverable failure, an offline
/// provider yields an offline-held change, and any other unsynced state is pending
/// publish. Every non-confirmed, non-blocked state stays recoverable.
pub fn resolve_sync_recovery(
    local_state: M5WorkItemLocalState,
    is_policy_blocked: bool,
    is_provider_offline: bool,
) -> SyncPillDisclosure {
    use M5WorkItemLocalState as Local;
    use SyncRecoveryClass as Class;

    let recovery_class = if is_policy_blocked {
        Class::PolicyBlocked
    } else if matches!(local_state, Local::SyncedWithProvider) {
        Class::ProviderConfirmed
    } else if matches!(local_state, Local::PublishFailed) {
        Class::RecoverableFailure
    } else if is_provider_offline {
        Class::OfflineHeld
    } else {
        Class::PendingPublish
    };

    let is_provider_confirmed = matches!(recovery_class, Class::ProviderConfirmed);
    let is_pending = matches!(
        recovery_class,
        Class::PendingPublish | Class::RecoverableFailure | Class::OfflineHeld
    );

    SyncPillDisclosure {
        recovery_class,
        is_provider_confirmed,
        is_pending,
        needs_distinct_style: !is_provider_confirmed,
        needs_last_sync_attempt: is_pending,
        needs_recovery_action: is_pending,
        needs_policy_block_note: matches!(recovery_class, Class::PolicyBlocked),
    }
}

/// A sync-pending pill disclosing pending change, last attempt, and recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncPendingPill {
    /// Frozen component this control implements; must be `sync_pending_pill`.
    pub component: M5WorkItemComponentFamily,
    /// Stable pill id.
    pub pill_id: String,
    /// Canonical id of the work item this pill belongs to; always non-empty.
    pub canonical_id: String,
    /// The kind of pending local change.
    pub pending_change_type: PendingChangeType,
    /// Pending-change label; required and non-empty (never a bare "pending").
    pub pending_change_label: String,
    /// Local-versus-provider state, reused from the frozen matrix.
    pub local_state: M5WorkItemLocalState,
    /// Whether the change is blocked by policy.
    pub is_policy_blocked: bool,
    /// Whether the provider is currently offline / unreachable.
    pub is_provider_offline: bool,
    /// Derived sync-recovery class (must equal the resolved class).
    pub recovery_class: SyncRecoveryClass,
    /// Whether the pill claims provider-confirmed state (must equal the derived truth).
    pub claims_provider_confirmed: bool,
    /// Whether the pill reads visibly differently from a confirmed state (required
    /// when the pill is not provider-confirmed).
    pub distinct_from_confirmed_style: bool,
    /// Last sync attempt label; required when the pill is pending.
    pub last_sync_attempt_label: String,
    /// Retry-or-export recovery actions (must include a recovery action when pending).
    pub recovery_actions: Vec<SyncPillAction>,
    /// Policy-block note; required when the pill is policy-blocked.
    pub policy_block_note: String,
    /// Claimed M5 work-item surface families that render this pill.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this pill keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this pill offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this pill's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this pill.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals queued state.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl SyncPendingPill {
    /// Sync-recovery disclosures this pill must carry, derived from state, policy, and reach.
    pub fn recovery_disclosure(&self) -> SyncPillDisclosure {
        resolve_sync_recovery(
            self.local_state,
            self.is_policy_blocked,
            self.is_provider_offline,
        )
    }

    /// Whether the pill offers at least one retry-or-export recovery action.
    fn declares_recovery_action(&self) -> bool {
        let present: BTreeSet<SyncPillAction> = self.recovery_actions.iter().copied().collect();
        SyncPillAction::RECOVERY
            .iter()
            .any(|action| present.contains(action))
    }
}

// ---- review blocks ------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationStripSyncPendingTrustReview {
    /// The relation strip names each linked context rather than a vague `Linked` label.
    pub relation_strip_names_each_linked_context: bool,
    /// Stale and broken relations are labeled as such.
    pub stale_and_broken_relations_labeled: bool,
    /// Relation actions are metadata-safe copy/open.
    pub relation_actions_metadata_safe_copy_open: bool,
    /// The sync-pending state reads visibly differently from a confirmed state.
    pub sync_pending_visibly_distinct_from_confirmed: bool,
    /// The sync-pending pill discloses its pending change type.
    pub sync_pending_discloses_change_type: bool,
    /// The last sync attempt is shown when pending.
    pub last_sync_attempt_shown_when_pending: bool,
    /// A retry-or-export recovery path stays available when publish fails or is offline.
    pub retry_or_export_recovery_always_available: bool,
    /// A policy-blocked state stays explicit.
    pub policy_blocked_state_always_explicit: bool,
    /// No generic ticket/task wording conceals linked context or queued state.
    pub no_generic_ticket_wording_conceals_context: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl RelationStripSyncPendingTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.relation_strip_names_each_linked_context
            && self.stale_and_broken_relations_labeled
            && self.relation_actions_metadata_safe_copy_open
            && self.sync_pending_visibly_distinct_from_confirmed
            && self.sync_pending_discloses_change_type
            && self.last_sync_attempt_shown_when_pending
            && self.retry_or_export_recovery_always_available
            && self.policy_blocked_state_always_explicit
            && self.no_generic_ticket_wording_conceals_context
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationStripSyncPendingConsumerProjection {
    /// Side-rail relation strips name each linked context inline.
    pub side_rail_relation_strips_name_each_context: bool,
    /// List and side-rail surfaces distinguish pending from provider-confirmed state.
    pub list_and_rail_distinguish_pending_from_confirmed: bool,
    /// The retry and export recovery paths are reachable headless.
    pub retry_and_export_reachable_headless: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl RelationStripSyncPendingConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.side_rail_relation_strips_name_each_context
            && self.list_and_rail_distinguish_pending_from_confirmed
            && self.retry_and_export_reachable_headless
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationStripSyncPendingProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RelationStripSyncPendingControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationStripSyncPendingControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Relation strips.
    pub relation_strips: Vec<RelationStrip>,
    /// Sync-pending pills.
    pub sync_pending_pills: Vec<SyncPendingPill>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: RelationStripSyncPendingTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RelationStripSyncPendingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RelationStripSyncPendingProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe relation-strip / sync-pending-pill controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationStripSyncPendingControlsPacket {
    /// Record kind; must equal [`RELATION_STRIP_SYNC_PENDING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RELATION_STRIP_SYNC_PENDING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Relation strips.
    pub relation_strips: Vec<RelationStrip>,
    /// Sync-pending pills.
    pub sync_pending_pills: Vec<SyncPendingPill>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: RelationStripSyncPendingTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RelationStripSyncPendingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RelationStripSyncPendingProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RelationStripSyncPendingControlsPacket {
    /// Builds a relation-strip / sync-pending-pill controls packet from stable-lane input.
    pub fn new(input: RelationStripSyncPendingControlsPacketInput) -> Self {
        Self {
            record_kind: RELATION_STRIP_SYNC_PENDING_RECORD_KIND.to_owned(),
            schema_version: RELATION_STRIP_SYNC_PENDING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            relation_strips: input.relation_strips,
            sync_pending_pills: input.sync_pending_pills,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the relation-strip / sync-pending-pill control invariants.
    pub fn validate(&self) -> Vec<RelationStripSyncPendingViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RELATION_STRIP_SYNC_PENDING_RECORD_KIND {
            violations.push(RelationStripSyncPendingViolation::WrongRecordKind);
        }
        if self.schema_version != RELATION_STRIP_SYNC_PENDING_SCHEMA_VERSION {
            violations.push(RelationStripSyncPendingViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RelationStripSyncPendingViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(RelationStripSyncPendingViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RelationStripSyncPendingViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_relation_strips(self, &mut violations);
        validate_sync_pending_pills(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(RelationStripSyncPendingViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(RelationStripSyncPendingViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(RelationStripSyncPendingViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("relation strip sync pending packet serializes"),
        ) {
            violations.push(RelationStripSyncPendingViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("relation strip sync pending packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("control,id,kind_or_type,relations_or_state,derived,recoverable_or_current\n");
        for strip in &self.relation_strips {
            let non_current = strip
                .relations
                .iter()
                .filter(|relation| !relation.health_disclosure().is_current)
                .count();
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                "relation_strip",
                csv_field(&strip.strip_id),
                csv_field(&strip.canonical_id),
                strip.relations.len(),
                non_current,
                strip.relations.len() - non_current,
            ));
        }
        for pill in &self.sync_pending_pills {
            let disclosure = pill.recovery_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                "sync_pending_pill",
                csv_field(&pill.pill_id),
                pill.pending_change_type.as_str(),
                pill.local_state.as_str(),
                disclosure.recovery_class.as_str(),
                !disclosure.is_provider_confirmed,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let non_current_relations = self
            .relation_strips
            .iter()
            .flat_map(|strip| strip.relations.iter())
            .filter(|relation| !relation.health_disclosure().is_current)
            .count();
        let pending_pills = self
            .sync_pending_pills
            .iter()
            .filter(|pill| !pill.recovery_disclosure().is_provider_confirmed)
            .count();

        let mut out = String::new();
        out.push_str("# Relation strips and sync-pending pills\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Relation strips: {} ({} non-current relations)\n",
            self.relation_strips.len(),
            non_current_relations
        ));
        out.push_str(&format!(
            "- Sync-pending pills: {} ({} not provider-confirmed)\n",
            self.sync_pending_pills.len(),
            pending_pills
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Relation strips\n\n");
        for strip in &self.relation_strips {
            out.push_str(&format!(
                "- **{}** ({}):\n",
                strip.strip_id, strip.canonical_id
            ));
            for relation in &strip.relations {
                out.push_str(&format!(
                    "  - {} → `{}` [{}]\n",
                    relation.relation_kind.as_str(),
                    relation.reference_label,
                    relation.health_disclosure().health_class.as_str(),
                ));
            }
        }

        out.push_str("\n## Sync-pending pills\n\n");
        for pill in &self.sync_pending_pills {
            out.push_str(&format!(
                "- **{}** {} [{}] → `{}`\n",
                pill.pill_id,
                pill.pending_change_type.as_str(),
                pill.local_state.as_str(),
                pill.recovery_disclosure().recovery_class.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in relation-strip / sync-pending export.
#[derive(Debug)]
pub enum RelationStripSyncPendingArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RelationStripSyncPendingViolation>),
}

impl fmt::Display for RelationStripSyncPendingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "relation strip sync pending export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "relation strip sync pending export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RelationStripSyncPendingArtifactError {}

/// Validation failures emitted by [`RelationStripSyncPendingControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationStripSyncPendingViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No relation strips are present.
    RelationStripsMissing,
    /// A relation strip is incomplete.
    RelationStripIncomplete,
    /// A relation strip carries the wrong frozen component class.
    RelationStripWrongComponentClass,
    /// A relation strip names no relations.
    RelationEntriesMissing,
    /// A relation strip does not name each linked context distinctly (vague `Linked`).
    RelationsCollapsedIntoVagueLabel,
    /// A relation has no metadata-safe reference label.
    RelationLabelMissing,
    /// A relation misrepresents its derived health class.
    RelationHealthMisrepresented,
    /// A stale, broken, or unmapped relation does not name its relation state.
    StaleOrBrokenRelationNoteMissing,
    /// A relation omits a mandatory metadata-safe copy/open action.
    RelationCopyOpenActionsIncomplete,
    /// The relation strips do not cover every derived relation health class.
    RelationHealthCoverageMissing,
    /// No sync-pending pills are present.
    SyncPendingPillsMissing,
    /// A sync-pending pill is incomplete.
    SyncPendingPillIncomplete,
    /// A sync-pending pill carries the wrong frozen component class.
    SyncPendingPillWrongComponentClass,
    /// A sync-pending pill does not name its pending change type.
    PendingChangeTypeLabelMissing,
    /// A sync-pending pill misrepresents its derived sync-recovery class.
    SyncRecoveryMisrepresented,
    /// A pending sync state reads as, or claims to be, provider-confirmed.
    SyncStateMisrepresented,
    /// A pending sync state is not visibly distinct from a provider-confirmed state.
    NotVisiblyDistinctFromConfirmed,
    /// A pending sync-pending pill does not name its last sync attempt.
    LastSyncAttemptMissing,
    /// A recoverable sync-pending pill offers no retry-or-export recovery action.
    RecoveryActionMissing,
    /// A policy-blocked sync-pending pill does not name its policy block.
    PolicyBlockNoteMissing,
    /// The sync-pending pills do not cover every derived sync-recovery class.
    SyncRecoveryCoverageMissing,
    /// The sync-pending pills do not cover every pending change type.
    PendingChangeTypeCoverageMissing,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control lets generic ticket/task wording conceal context or queued state.
    GenericTicketWordingUsed,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RelationStripSyncPendingViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RelationStripsMissing => "relation_strips_missing",
            Self::RelationStripIncomplete => "relation_strip_incomplete",
            Self::RelationStripWrongComponentClass => "relation_strip_wrong_component_class",
            Self::RelationEntriesMissing => "relation_entries_missing",
            Self::RelationsCollapsedIntoVagueLabel => "relations_collapsed_into_vague_label",
            Self::RelationLabelMissing => "relation_label_missing",
            Self::RelationHealthMisrepresented => "relation_health_misrepresented",
            Self::StaleOrBrokenRelationNoteMissing => "stale_or_broken_relation_note_missing",
            Self::RelationCopyOpenActionsIncomplete => "relation_copy_open_actions_incomplete",
            Self::RelationHealthCoverageMissing => "relation_health_coverage_missing",
            Self::SyncPendingPillsMissing => "sync_pending_pills_missing",
            Self::SyncPendingPillIncomplete => "sync_pending_pill_incomplete",
            Self::SyncPendingPillWrongComponentClass => "sync_pending_pill_wrong_component_class",
            Self::PendingChangeTypeLabelMissing => "pending_change_type_label_missing",
            Self::SyncRecoveryMisrepresented => "sync_recovery_misrepresented",
            Self::SyncStateMisrepresented => "sync_state_misrepresented",
            Self::NotVisiblyDistinctFromConfirmed => "not_visibly_distinct_from_confirmed",
            Self::LastSyncAttemptMissing => "last_sync_attempt_missing",
            Self::RecoveryActionMissing => "recovery_action_missing",
            Self::PolicyBlockNoteMissing => "policy_block_note_missing",
            Self::SyncRecoveryCoverageMissing => "sync_recovery_coverage_missing",
            Self::PendingChangeTypeCoverageMissing => "pending_change_type_coverage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::GenericTicketWordingUsed => "generic_ticket_wording_used",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable relation-strip / sync-pending export.
pub fn current_relation_strip_sync_pending_export(
) -> Result<RelationStripSyncPendingControlsPacket, RelationStripSyncPendingArtifactError> {
    let packet: RelationStripSyncPendingControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-relation-strip-sync-pending-proof/support_export.json"
        )))
        .map_err(RelationStripSyncPendingArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RelationStripSyncPendingArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &RelationStripSyncPendingControlsPacket,
    violations: &mut Vec<RelationStripSyncPendingViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RELATION_STRIP_SYNC_PENDING_SCHEMA_REF,
        RELATION_STRIP_SYNC_PENDING_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_RELATION_STRIP_SCHEMA_REF,
        M5_SYNC_PENDING_PILL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RelationStripSyncPendingViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_relation_strips(
    packet: &RelationStripSyncPendingControlsPacket,
    violations: &mut Vec<RelationStripSyncPendingViolation>,
) {
    if packet.relation_strips.is_empty() {
        violations.push(RelationStripSyncPendingViolation::RelationStripsMissing);
        return;
    }

    let mut health_classes: BTreeSet<RelationHealthClass> = BTreeSet::new();

    for strip in &packet.relation_strips {
        if strip.strip_id.trim().is_empty()
            || strip.canonical_id.trim().is_empty()
            || strip.fields_shown.is_empty()
            || strip.surface_families.is_empty()
            || strip.deployment_lines.is_empty()
            || strip.consumer_surfaces.is_empty()
            || strip.source_contract_refs.is_empty()
        {
            violations.push(RelationStripSyncPendingViolation::RelationStripIncomplete);
        }
        if strip.component != M5WorkItemComponentFamily::RelationStrip {
            violations.push(RelationStripSyncPendingViolation::RelationStripWrongComponentClass);
        }
        if strip.relations.is_empty() {
            violations.push(RelationStripSyncPendingViolation::RelationEntriesMissing);
        }
        // AC1: multiple linked contexts are never collapsed into a vague `Linked` label.
        if strip.collapses_into_generic_linked_label || !strip.labels_each_relation_distinctly() {
            violations.push(RelationStripSyncPendingViolation::RelationsCollapsedIntoVagueLabel);
        }
        if strip.accessibility_routes.is_empty()
            || !strip
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(RelationStripSyncPendingViolation::AccessibilityRouteMissing);
        }
        if strip.uses_generic_ticket_wording {
            violations.push(RelationStripSyncPendingViolation::GenericTicketWordingUsed);
        }

        for relation in &strip.relations {
            let disclosure = relation.health_disclosure();
            health_classes.insert(disclosure.health_class);

            if relation.reference_label.trim().is_empty() {
                violations.push(RelationStripSyncPendingViolation::RelationLabelMissing);
            }
            if relation.health_class != disclosure.health_class {
                violations.push(RelationStripSyncPendingViolation::RelationHealthMisrepresented);
            }
            if disclosure.needs_relation_note && relation.relation_note.trim().is_empty() {
                violations
                    .push(RelationStripSyncPendingViolation::StaleOrBrokenRelationNoteMissing);
            }
            if !relation.declares_mandatory_actions() {
                violations
                    .push(RelationStripSyncPendingViolation::RelationCopyOpenActionsIncomplete);
            }
        }
    }

    for required in RelationHealthClass::ALL {
        if !health_classes.contains(&required) {
            violations.push(RelationStripSyncPendingViolation::RelationHealthCoverageMissing);
            break;
        }
    }
}

fn validate_sync_pending_pills(
    packet: &RelationStripSyncPendingControlsPacket,
    violations: &mut Vec<RelationStripSyncPendingViolation>,
) {
    if packet.sync_pending_pills.is_empty() {
        violations.push(RelationStripSyncPendingViolation::SyncPendingPillsMissing);
        return;
    }

    let mut recovery_classes: BTreeSet<SyncRecoveryClass> = BTreeSet::new();
    let mut change_types: BTreeSet<PendingChangeType> = BTreeSet::new();

    for pill in &packet.sync_pending_pills {
        recovery_classes.insert(pill.recovery_disclosure().recovery_class);
        change_types.insert(pill.pending_change_type);
        let disclosure = pill.recovery_disclosure();

        if pill.pill_id.trim().is_empty()
            || pill.canonical_id.trim().is_empty()
            || pill.fields_shown.is_empty()
            || pill.surface_families.is_empty()
            || pill.deployment_lines.is_empty()
            || pill.consumer_surfaces.is_empty()
            || pill.source_contract_refs.is_empty()
        {
            violations.push(RelationStripSyncPendingViolation::SyncPendingPillIncomplete);
        }
        if pill.component != M5WorkItemComponentFamily::SyncPendingPill {
            violations.push(RelationStripSyncPendingViolation::SyncPendingPillWrongComponentClass);
        }
        if pill.pending_change_label.trim().is_empty() {
            violations.push(RelationStripSyncPendingViolation::PendingChangeTypeLabelMissing);
        }
        if pill.recovery_class != disclosure.recovery_class {
            violations.push(RelationStripSyncPendingViolation::SyncRecoveryMisrepresented);
        }
        if pill.claims_provider_confirmed != disclosure.is_provider_confirmed {
            violations.push(RelationStripSyncPendingViolation::SyncStateMisrepresented);
        }
        if disclosure.needs_distinct_style && !pill.distinct_from_confirmed_style {
            violations.push(RelationStripSyncPendingViolation::NotVisiblyDistinctFromConfirmed);
        }
        if disclosure.needs_last_sync_attempt && pill.last_sync_attempt_label.trim().is_empty() {
            violations.push(RelationStripSyncPendingViolation::LastSyncAttemptMissing);
        }
        if disclosure.needs_recovery_action && !pill.declares_recovery_action() {
            violations.push(RelationStripSyncPendingViolation::RecoveryActionMissing);
        }
        if disclosure.needs_policy_block_note && pill.policy_block_note.trim().is_empty() {
            violations.push(RelationStripSyncPendingViolation::PolicyBlockNoteMissing);
        }
        if pill.accessibility_routes.is_empty()
            || !pill
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(RelationStripSyncPendingViolation::AccessibilityRouteMissing);
        }
        if pill.uses_generic_ticket_wording {
            violations.push(RelationStripSyncPendingViolation::GenericTicketWordingUsed);
        }
    }

    for required in SyncRecoveryClass::ALL {
        if !recovery_classes.contains(&required) {
            violations.push(RelationStripSyncPendingViolation::SyncRecoveryCoverageMissing);
            break;
        }
    }
    for required in PendingChangeType::ALL {
        if !change_types.contains(&required) {
            violations.push(RelationStripSyncPendingViolation::PendingChangeTypeCoverageMissing);
            break;
        }
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
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
