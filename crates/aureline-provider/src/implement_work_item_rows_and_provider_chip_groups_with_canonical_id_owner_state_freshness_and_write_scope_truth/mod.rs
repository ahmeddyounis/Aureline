//! Work-item rows and provider chip groups carrying canonical identity, title,
//! state, owner, priority/severity, linked-change count, keyboard-complete
//! default actions, provider/project-or-space scope, tenant/org cue, and an
//! explicit read-only/comment-link/full-edit/offline-capture/policy-blocked
//! write posture.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_work_item_component_matrix`] — the `work_item_row` and
//! the `provider_chip_group` — into one implemented, export-safe packet with two
//! co-equal control vectors. Together they make list-level work-item identity and
//! provider authority explicit *before* a user opens detail or publish flows.
//!
//! A [`WorkItemRow`] always names its canonical id, title, work-item kind,
//! assignee/owner, priority or severity, and linked-change count, and its state
//! authority is derived from the provider authority and the local-versus-provider
//! state rather than asserted: a local-only draft or a policy-blocked item can
//! never read as provider-authoritative, so a user can tell provider-authoritative
//! state from local-only or blocked capability directly in a list surface without
//! opening a secondary inspector. Its canonical id is always visible and copyable,
//! and its default actions are keyboard-complete.
//!
//! A [`ProviderChipGroup`] always names the provider and the project or space it
//! is scoped to, the tenant/org cue where relevant, and an explicit write posture
//! (read-only, comment-link, full-edit, offline-capture, or policy-blocked). The
//! posture is checked against the provider authority so an offline-capture or
//! policy-blocked chip never presents as a live full-edit connection, and generic
//! ticket/task wording never conceals who owns the object.
//!
//! The work-item kinds ([`M5WorkItemKind`]), provider-authority classes
//! ([`M5WorkItemProviderAuthority`]), local-versus-provider states
//! ([`M5WorkItemLocalState`]), surface families ([`M5WorkItemSurfaceFamily`]),
//! deployment lines ([`M5WorkItemDeploymentLine`]), consumer surfaces
//! ([`M5WorkItemConsumerSurface`]), accessibility routes
//! ([`M5WorkItemAccessibilityRoute`]), and downgrade triggers
//! ([`M5WorkItemDowngradeTrigger`]) are reused directly from the frozen matrix, so
//! this lane never invents a parallel work-item vocabulary. It mints new vocabulary
//! only for what that matrix left implicit about these two controls: the derived
//! state-authority class, the priority/severity signal, the keyboard-complete row
//! actions, and the explicit provider-chip write posture.
//!
//! Raw work-item bodies, pasted paths, credentials, and private endpoints stay
//! outside the support boundary; canonical ids are carried only as opaque,
//! export-safe references.
//!
//! The boundary schema is
//! [`schemas/ui/m5-work-item-row-provider-chip-controls.schema.json`](../../../../schemas/ui/m5-work-item-row-provider-chip-controls.schema.json).
//! The contract doc is
//! [`docs/team-workflows/implement_work_item_rows_and_provider_chip_groups.md`](../../../../docs/team-workflows/implement_work_item_rows_and_provider_chip_groups.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_work_item_row_provider_chip_controls,
    seeded_work_item_row_provider_chip_controls_provider_chip_offline_capture,
    seeded_work_item_row_provider_chip_controls_work_item_row_local_only,
    WORK_ITEM_ROW_PROVIDER_CHIP_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The work-item kind, provider authority, local-versus-provider state, surface
// family, deployment line, consumer surface, accessibility route, and downgrade
// triggers are frozen once, in the work-item component matrix. This lane reuses
// them verbatim so it never invents a parallel work-item vocabulary.
use crate::freeze_the_m5_work_item_component_matrix::{
    M5WorkItemAccessibilityRoute, M5WorkItemComponentFamily, M5WorkItemConsumerSurface,
    M5WorkItemDeploymentLine, M5WorkItemDowngradeTrigger, M5WorkItemKind, M5WorkItemLocalState,
    M5WorkItemProviderAuthority, M5WorkItemSurfaceFamily, M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
    M5_WORK_ITEM_COMPONENT_DOC_REF, M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    M5_WORK_ITEM_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`WorkItemRowProviderChipControlsPacket`].
pub const WORK_ITEM_ROW_PROVIDER_CHIP_RECORD_KIND: &str = "work_item_row_provider_chip_controls";

/// Schema version for work-item-row / provider-chip-group control records.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF: &str =
    "schemas/ui/m5-work-item-row-provider-chip-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_DOC_REF: &str =
    "docs/team-workflows/implement_work_item_rows_and_provider_chip_groups.md";

/// Repo-relative path of the protected fixture directory.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_FIXTURE_DIR: &str =
    "fixtures/ui/m5-work-item-row-provider-chip-controls";

/// Repo-relative path of the checked support-export artifact.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_ARTIFACT_REF: &str =
    "artifacts/release/m5-work-item-row-provider-chip-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_SUMMARY_REF: &str =
    "artifacts/release/m5-work-item-row-provider-chip-proof/summary.md";

// ---- work-item-row vocabulary -------------------------------------------

/// Derived state-authority class a work-item row may present.
///
/// This is the row honesty axis: the class is derived from the provider
/// authority and the local-versus-provider state, never asserted, so a
/// local-only draft or a policy-blocked item can never present as
/// provider-authoritative state in a list surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemStateAuthorityClass {
    /// The provider owns the object and the row is reconciled with it.
    ProviderAuthoritative,
    /// A local-only draft not owned by any provider and not yet published.
    LocalOnlyDraft,
    /// A change queued, deferred, failed, or held for publish; not yet reconciled.
    PublishPending,
    /// A read-only mirror or imported snapshot detached from live provider truth.
    SnapshotOnly,
    /// Capability is blocked by policy; the row cannot be written.
    BlockedCapability,
}

impl WorkItemStateAuthorityClass {
    /// Every state-authority class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAuthoritative,
        Self::LocalOnlyDraft,
        Self::PublishPending,
        Self::SnapshotOnly,
        Self::BlockedCapability,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::LocalOnlyDraft => "local_only_draft",
            Self::PublishPending => "publish_pending",
            Self::SnapshotOnly => "snapshot_only",
            Self::BlockedCapability => "blocked_capability",
        }
    }
}

/// Priority or severity signal a work-item row shows.
///
/// The same rank covers a task's priority and an incident's severity; the row
/// records which scale it uses so a severity is never silently shown as a
/// priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemPriorityClass {
    /// Critical priority / severity.
    Critical,
    /// High priority / severity.
    High,
    /// Medium priority / severity.
    Medium,
    /// Low priority / severity.
    Low,
    /// No priority / severity assigned.
    None,
    /// Priority / severity is unknown.
    Unknown,
}

impl WorkItemPriorityClass {
    /// Every priority class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Critical,
        Self::High,
        Self::Medium,
        Self::Low,
        Self::None,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::None => "none",
            Self::Unknown => "unknown",
        }
    }
}

/// One keyboard-complete default action a work-item row offers, so a row never
/// hides its canonical-id copy, open, transition, or export affordances behind a
/// pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemRowAction {
    /// Copy the canonical id (always available; the id is copyable everywhere).
    CopyCanonicalId,
    /// Open the work-item detail surface.
    OpenDetail,
    /// Open the item in the owning provider.
    OpenInProvider,
    /// Review a status transition before write.
    ReviewTransition,
    /// Reveal the linked engineering context.
    RevealLinkedContext,
    /// Export the row as work-item evidence.
    ExportRow,
}

impl WorkItemRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CopyCanonicalId,
        Self::OpenDetail,
        Self::OpenInProvider,
        Self::ReviewTransition,
        Self::RevealLinkedContext,
        Self::ExportRow,
    ];

    /// The default actions every keyboard-complete row must offer.
    pub const MANDATORY: [Self; 2] = [Self::CopyCanonicalId, Self::OpenDetail];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyCanonicalId => "copy_canonical_id",
            Self::OpenDetail => "open_detail",
            Self::OpenInProvider => "open_in_provider",
            Self::ReviewTransition => "review_transition",
            Self::RevealLinkedContext => "reveal_linked_context",
            Self::ExportRow => "export_row",
        }
    }
}

/// Disclosures a work-item row must carry, derived from authority and local state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkItemRowStateDisclosure {
    /// The derived state-authority class this row may present.
    pub authority_class: WorkItemStateAuthorityClass,
    /// Whether the row's visible state is provider-authoritative.
    pub is_provider_authoritative: bool,
    /// Whether the row is a local-only draft.
    pub is_local_only: bool,
    /// Whether the row's capability is blocked by policy.
    pub is_blocked: bool,
    /// Whether the row must carry an explicit local-versus-provider state note.
    pub needs_local_state_note: bool,
    /// Whether the row must carry an explicit publish-pending note.
    pub needs_publish_pending_note: bool,
    /// Whether the row must carry an explicit blocked-capability note.
    pub needs_blocked_note: bool,
}

/// Resolves the state-authority truth a work-item row may present.
///
/// A policy-pinned item is a blocked capability. A queued / deferred / failed /
/// conflict-held item is publish-pending. A local draft, an unlinked-local item,
/// or a local-only-draft state is a local-only draft. A mirrored or imported item
/// is a snapshot. Only a provider-owned, provider-synced item is
/// provider-authoritative, so a local or blocked row can never claim otherwise.
pub fn resolve_work_item_state_authority(
    provider_authority: M5WorkItemProviderAuthority,
    local_state: M5WorkItemLocalState,
) -> WorkItemRowStateDisclosure {
    use M5WorkItemLocalState as Local;
    use M5WorkItemProviderAuthority as Authority;
    use WorkItemStateAuthorityClass as Class;

    let authority_class = if matches!(provider_authority, Authority::PolicyPinned) {
        Class::BlockedCapability
    } else if matches!(
        local_state,
        Local::QueuedForPublish
            | Local::PublishDeferred
            | Local::PublishFailed
            | Local::ConflictHeld
    ) {
        Class::PublishPending
    } else if matches!(
        provider_authority,
        Authority::LocalDraft | Authority::UnlinkedLocal
    ) || matches!(local_state, Local::LocalOnlyDraft)
    {
        Class::LocalOnlyDraft
    } else if matches!(
        provider_authority,
        Authority::MirroredReadOnly | Authority::ImportedSnapshot
    ) {
        Class::SnapshotOnly
    } else {
        Class::ProviderAuthoritative
    };

    WorkItemRowStateDisclosure {
        authority_class,
        is_provider_authoritative: matches!(authority_class, Class::ProviderAuthoritative),
        is_local_only: matches!(authority_class, Class::LocalOnlyDraft),
        is_blocked: matches!(authority_class, Class::BlockedCapability),
        needs_local_state_note: !matches!(local_state, Local::SyncedWithProvider),
        needs_publish_pending_note: matches!(authority_class, Class::PublishPending),
        needs_blocked_note: matches!(authority_class, Class::BlockedCapability),
    }
}

/// A work-item row naming canonical id, title, state, owner, priority, and links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRow {
    /// Frozen component this control implements; must be `work_item_row`.
    pub component: M5WorkItemComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Canonical id; always visible and non-empty (no guessing).
    pub canonical_id: String,
    /// Whether the canonical id is copyable; always required to be `true`.
    pub canonical_id_copyable: bool,
    /// Human-readable title; required and non-empty.
    pub title: String,
    /// Work-item kind, reused from the frozen matrix.
    pub work_item_kind: M5WorkItemKind,
    /// Provider authority, reused from the frozen matrix.
    pub provider_authority: M5WorkItemProviderAuthority,
    /// Local-versus-provider state, reused from the frozen matrix.
    pub local_state: M5WorkItemLocalState,
    /// Assignee / owner label; required and non-empty.
    pub owner_label: String,
    /// Priority or severity class.
    pub priority_class: WorkItemPriorityClass,
    /// Whether the priority signal uses a severity scale (e.g. an incident).
    pub uses_severity_scale: bool,
    /// Priority / severity label; required and non-empty.
    pub priority_label: String,
    /// Linked-change count.
    pub linked_change_count: u32,
    /// Linked-change label; required when the count is non-zero.
    pub linked_change_label: String,
    /// Derived state-authority class (must equal the resolved class).
    pub state_authority_class: WorkItemStateAuthorityClass,
    /// Whether the row claims provider-authoritative state (must equal the derived truth).
    pub claims_provider_authoritative: bool,
    /// Local-versus-provider state note; required when the row is not synced.
    pub local_state_note: String,
    /// Publish-pending note; required when the row is publish-pending.
    pub publish_pending_note: String,
    /// Blocked-capability note; required when the row is blocked by policy.
    pub blocked_capability_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub default_actions: Vec<WorkItemRowAction>,
    /// Claimed M5 work-item surface families that render this row.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals ownership or
    /// queued state. MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl WorkItemRow {
    /// State-authority disclosures this row must carry, derived from authority and state.
    pub fn state_authority_disclosure(&self) -> WorkItemRowStateDisclosure {
        resolve_work_item_state_authority(self.provider_authority, self.local_state)
    }

    /// Whether the row offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<WorkItemRowAction> = self.default_actions.iter().copied().collect();
        WorkItemRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- provider-chip-group vocabulary -------------------------------------

/// Explicit write posture a provider chip group presents.
///
/// These are the five postures a user must be able to tell apart directly: a
/// read-only mirror, a comment-link (limited) connection, a full-edit connection,
/// a local offline-capture, and a policy-blocked binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderChipWritePosture {
    /// A read-only binding; nothing can be written back.
    ReadOnly,
    /// A comment-link binding; only comments / limited fields can be written.
    CommentLink,
    /// A full-edit binding; the provider object can be written.
    FullEdit,
    /// A local offline-capture binding; changes are captured locally, not published.
    OfflineCapture,
    /// A policy-blocked binding; capability is blocked by policy.
    PolicyBlocked,
}

impl ProviderChipWritePosture {
    /// Every write posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReadOnly,
        Self::CommentLink,
        Self::FullEdit,
        Self::OfflineCapture,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::CommentLink => "comment_link",
            Self::FullEdit => "full_edit",
            Self::OfflineCapture => "offline_capture",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether this posture allows any write back to the provider.
    pub const fn is_writable(self) -> bool {
        matches!(self, Self::CommentLink | Self::FullEdit)
    }

    /// Whether this posture is consistent with the given provider authority.
    ///
    /// A policy-blocked chip requires a policy-pinned authority, an offline-capture
    /// chip requires a local authority, a comment-link or full-edit chip requires a
    /// provider-owned authority, and a read-only chip requires a mirrored, imported,
    /// or provider-owned authority.
    pub const fn is_consistent_with_authority(
        self,
        authority: M5WorkItemProviderAuthority,
    ) -> bool {
        use M5WorkItemProviderAuthority as Authority;
        match self {
            Self::PolicyBlocked => matches!(authority, Authority::PolicyPinned),
            Self::OfflineCapture => {
                matches!(authority, Authority::LocalDraft | Authority::UnlinkedLocal)
            }
            Self::CommentLink | Self::FullEdit => matches!(authority, Authority::ProviderOwned),
            Self::ReadOnly => matches!(
                authority,
                Authority::MirroredReadOnly
                    | Authority::ImportedSnapshot
                    | Authority::ProviderOwned
            ),
        }
    }
}

/// Disclosures a provider chip group must carry, derived from authority and posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderChipDisclosure {
    /// Whether the chip presents a writable connection.
    pub is_writable: bool,
    /// Whether the chip must carry an explicit offline-capture note.
    pub needs_offline_capture_note: bool,
    /// Whether the chip must carry an explicit policy-block note.
    pub needs_policy_block_note: bool,
    /// Whether the chip must carry an explicit read-only note.
    pub needs_read_only_note: bool,
    /// Whether the posture is consistent with the provider authority.
    pub posture_matches_authority: bool,
}

/// Resolves the write-posture truth a provider chip group may present.
pub fn resolve_provider_chip_group_disclosure(
    provider_authority: M5WorkItemProviderAuthority,
    write_posture: ProviderChipWritePosture,
) -> ProviderChipDisclosure {
    ProviderChipDisclosure {
        is_writable: write_posture.is_writable(),
        needs_offline_capture_note: matches!(
            write_posture,
            ProviderChipWritePosture::OfflineCapture
        ),
        needs_policy_block_note: matches!(write_posture, ProviderChipWritePosture::PolicyBlocked),
        needs_read_only_note: matches!(write_posture, ProviderChipWritePosture::ReadOnly),
        posture_matches_authority: write_posture.is_consistent_with_authority(provider_authority),
    }
}

/// A provider chip group naming provider, project/space scope, tenant cue, and posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderChipGroup {
    /// Frozen component this control implements; must be `provider_chip_group`.
    pub component: M5WorkItemComponentFamily,
    /// Stable group id.
    pub group_id: String,
    /// Provider label; required and non-empty.
    pub provider_label: String,
    /// Project or space scope label; required and non-empty.
    pub project_or_space_label: String,
    /// Provider authority, reused from the frozen matrix.
    pub provider_authority: M5WorkItemProviderAuthority,
    /// Whether a tenant/org scope cue is relevant for this chip group.
    pub has_tenant_scope: bool,
    /// Tenant / org cue; required when the tenant scope is relevant.
    pub tenant_scope_note: String,
    /// Explicit write posture.
    pub write_posture: ProviderChipWritePosture,
    /// Whether the chip presents a writable connection (must equal the derived truth).
    pub is_writable: bool,
    /// Read-only note; required when the posture is read-only.
    pub read_only_note: String,
    /// Offline-capture note; required when the posture is offline-capture.
    pub offline_capture_note: String,
    /// Policy-block note; required when the posture is policy-blocked.
    pub policy_block_note: String,
    /// Claimed M5 work-item surface families that render this chip group.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this chip group keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this chip group offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this chip group's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this chip group.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals provider ownership.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl ProviderChipGroup {
    /// Write-posture disclosures this chip group must carry, derived from authority and posture.
    pub fn chip_disclosure(&self) -> ProviderChipDisclosure {
        resolve_provider_chip_group_disclosure(self.provider_authority, self.write_posture)
    }
}

// ---- review blocks ------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRowProviderChipTrustReview {
    /// The canonical id is always visible and copyable.
    pub canonical_id_always_visible_and_copyable: bool,
    /// The work-item state names the provider authority behind it.
    pub work_item_state_shows_provider_authority: bool,
    /// A local-only or blocked row never reads as provider-authoritative.
    pub local_or_blocked_never_reads_as_provider_authoritative: bool,
    /// Blocked capability stays explicit.
    pub blocked_capability_always_explicit: bool,
    /// The linked-change count is always shown.
    pub linked_change_count_always_shown: bool,
    /// The default row actions are keyboard-complete.
    pub keyboard_complete_default_actions: bool,
    /// The provider chip group names its project or space scope.
    pub provider_chip_shows_project_or_space_scope: bool,
    /// The provider chip group names its write posture.
    pub provider_chip_shows_write_posture: bool,
    /// The tenant/org cue is shown when relevant.
    pub tenant_org_cue_shown_when_relevant: bool,
    /// Offline-capture and policy-block postures stay explicit.
    pub offline_capture_and_policy_block_explicit: bool,
    /// No generic ticket/task wording conceals provider ownership or queued state.
    pub no_generic_ticket_wording_conceals_ownership: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl WorkItemRowProviderChipTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.canonical_id_always_visible_and_copyable
            && self.work_item_state_shows_provider_authority
            && self.local_or_blocked_never_reads_as_provider_authoritative
            && self.blocked_capability_always_explicit
            && self.linked_change_count_always_shown
            && self.keyboard_complete_default_actions
            && self.provider_chip_shows_project_or_space_scope
            && self.provider_chip_shows_write_posture
            && self.tenant_org_cue_shown_when_relevant
            && self.offline_capture_and_policy_block_explicit
            && self.no_generic_ticket_wording_conceals_ownership
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRowProviderChipConsumerProjection {
    /// List surfaces distinguish provider-authoritative from local-only or blocked
    /// state without opening a secondary inspector.
    pub list_rows_distinguish_authority_without_inspector: bool,
    /// The canonical id is copyable anywhere a row can influence code, review,
    /// incident, or AI context.
    pub canonical_id_copyable_everywhere: bool,
    /// The chip group shows scope and posture inline.
    pub chip_group_shows_scope_and_posture_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl WorkItemRowProviderChipConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.list_rows_distinguish_authority_without_inspector
            && self.canonical_id_copyable_everywhere
            && self.chip_group_shows_scope_and_posture_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRowProviderChipProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`WorkItemRowProviderChipControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemRowProviderChipControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Work-item rows.
    pub work_item_rows: Vec<WorkItemRow>,
    /// Provider chip groups.
    pub provider_chip_groups: Vec<ProviderChipGroup>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: WorkItemRowProviderChipTrustReview,
    /// Consumer projection block.
    pub consumer_projection: WorkItemRowProviderChipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: WorkItemRowProviderChipProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe work-item-row / provider-chip-group controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRowProviderChipControlsPacket {
    /// Record kind; must equal [`WORK_ITEM_ROW_PROVIDER_CHIP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Work-item rows.
    pub work_item_rows: Vec<WorkItemRow>,
    /// Provider chip groups.
    pub provider_chip_groups: Vec<ProviderChipGroup>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: WorkItemRowProviderChipTrustReview,
    /// Consumer projection block.
    pub consumer_projection: WorkItemRowProviderChipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: WorkItemRowProviderChipProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl WorkItemRowProviderChipControlsPacket {
    /// Builds a work-item-row / provider-chip-group controls packet from stable-lane input.
    pub fn new(input: WorkItemRowProviderChipControlsPacketInput) -> Self {
        Self {
            record_kind: WORK_ITEM_ROW_PROVIDER_CHIP_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            work_item_rows: input.work_item_rows,
            provider_chip_groups: input.provider_chip_groups,
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

    /// Validates the work-item-row / provider-chip-group control invariants.
    pub fn validate(&self) -> Vec<WorkItemRowProviderChipViolation> {
        let mut violations = Vec::new();

        if self.record_kind != WORK_ITEM_ROW_PROVIDER_CHIP_RECORD_KIND {
            violations.push(WorkItemRowProviderChipViolation::WrongRecordKind);
        }
        if self.schema_version != WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_VERSION {
            violations.push(WorkItemRowProviderChipViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(WorkItemRowProviderChipViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(WorkItemRowProviderChipViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(WorkItemRowProviderChipViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_work_item_rows(self, &mut violations);
        validate_provider_chip_groups(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(WorkItemRowProviderChipViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(WorkItemRowProviderChipViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(WorkItemRowProviderChipViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("work item row provider chip packet serializes"),
        ) {
            violations.push(WorkItemRowProviderChipViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("work item row provider chip packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("control,id,kind_or_scope,authority,state_or_posture,derived,writable\n");
        for row in &self.work_item_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "work_item_row",
                csv_field(&row.canonical_id),
                row.work_item_kind.as_str(),
                row.provider_authority.as_str(),
                row.local_state.as_str(),
                row.state_authority_disclosure().authority_class.as_str(),
                row.state_authority_disclosure().is_provider_authoritative,
            ));
        }
        for group in &self.provider_chip_groups {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "provider_chip_group",
                csv_field(&group.group_id),
                csv_field(&group.project_or_space_label),
                group.provider_authority.as_str(),
                group.write_posture.as_str(),
                group.chip_disclosure().posture_matches_authority,
                group.chip_disclosure().is_writable,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let local_or_blocked = self
            .work_item_rows
            .iter()
            .filter(|row| !row.state_authority_disclosure().is_provider_authoritative)
            .count();
        let non_writable = self
            .provider_chip_groups
            .iter()
            .filter(|group| !group.chip_disclosure().is_writable)
            .count();

        let mut out = String::new();
        out.push_str("# Work-item rows and provider chip groups\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Work-item rows: {} ({} not provider-authoritative)\n",
            self.work_item_rows.len(),
            local_or_blocked
        ));
        out.push_str(&format!(
            "- Provider chip groups: {} ({} not writable)\n",
            self.provider_chip_groups.len(),
            non_writable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Work-item rows\n\n");
        for row in &self.work_item_rows {
            out.push_str(&format!(
                "- **{}** ({}) — {} [{}] → `{}`\n",
                row.canonical_id,
                row.work_item_kind.as_str(),
                row.provider_authority.as_str(),
                row.local_state.as_str(),
                row.state_authority_disclosure().authority_class.as_str(),
            ));
        }

        out.push_str("\n## Provider chip groups\n\n");
        for group in &self.provider_chip_groups {
            out.push_str(&format!(
                "- **{}** / {} [{}] posture `{}`\n",
                group.provider_label,
                group.project_or_space_label,
                group.provider_authority.as_str(),
                group.write_posture.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in work-item-row / provider-chip export.
#[derive(Debug)]
pub enum WorkItemRowProviderChipArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<WorkItemRowProviderChipViolation>),
}

impl fmt::Display for WorkItemRowProviderChipArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "work item row provider chip export parse failed: {error}"
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
                    "work item row provider chip export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for WorkItemRowProviderChipArtifactError {}

/// Validation failures emitted by [`WorkItemRowProviderChipControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkItemRowProviderChipViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No work-item rows are present.
    WorkItemRowsMissing,
    /// A work-item row is incomplete.
    WorkItemRowIncomplete,
    /// A work-item row carries the wrong frozen component class.
    WorkItemRowWrongComponentClass,
    /// A work-item row does not keep its canonical id visible and copyable.
    CanonicalIdNotCopyable,
    /// A work-item row omits a mandatory keyboard-complete default action.
    DefaultActionsIncomplete,
    /// A work-item row misrepresents its derived state authority.
    StateAuthorityMisrepresented,
    /// A non-synced work-item row does not name its local-versus-provider state.
    LocalStateNoteMissing,
    /// A publish-pending work-item row does not name its publish-pending state.
    PublishPendingNoteMissing,
    /// A blocked work-item row does not name its blocked capability.
    BlockedCapabilityNoteMissing,
    /// A work-item row with linked changes does not name its linked-change context.
    LinkedChangeLabelMissing,
    /// The work-item rows do not cover every derived state-authority class.
    StateAuthorityCoverageMissing,
    /// No provider chip groups are present.
    ProviderChipGroupsMissing,
    /// A provider chip group is incomplete.
    ProviderChipGroupIncomplete,
    /// A provider chip group carries the wrong frozen component class.
    ProviderChipGroupWrongComponentClass,
    /// A provider chip group does not name its project or space scope.
    ProjectOrSpaceScopeMissing,
    /// A provider chip group misrepresents its writability.
    ChipWritabilityMisrepresented,
    /// A provider chip group's posture is inconsistent with its provider authority.
    ChipPostureMisrepresented,
    /// A read-only chip group does not name its read-only posture.
    ReadOnlyNoteMissing,
    /// An offline-capture chip group does not name its offline-capture posture.
    OfflineCaptureNoteMissing,
    /// A policy-blocked chip group does not name its policy-block posture.
    PolicyBlockNoteMissing,
    /// A tenant-scoped chip group does not name its tenant/org cue.
    TenantScopeNoteMissing,
    /// The provider chip groups do not cover the five required write postures.
    WritePostureCoverageMissing,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control lets generic ticket/task wording conceal ownership or queued state.
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

impl WorkItemRowProviderChipViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::WorkItemRowsMissing => "work_item_rows_missing",
            Self::WorkItemRowIncomplete => "work_item_row_incomplete",
            Self::WorkItemRowWrongComponentClass => "work_item_row_wrong_component_class",
            Self::CanonicalIdNotCopyable => "canonical_id_not_copyable",
            Self::DefaultActionsIncomplete => "default_actions_incomplete",
            Self::StateAuthorityMisrepresented => "state_authority_misrepresented",
            Self::LocalStateNoteMissing => "local_state_note_missing",
            Self::PublishPendingNoteMissing => "publish_pending_note_missing",
            Self::BlockedCapabilityNoteMissing => "blocked_capability_note_missing",
            Self::LinkedChangeLabelMissing => "linked_change_label_missing",
            Self::StateAuthorityCoverageMissing => "state_authority_coverage_missing",
            Self::ProviderChipGroupsMissing => "provider_chip_groups_missing",
            Self::ProviderChipGroupIncomplete => "provider_chip_group_incomplete",
            Self::ProviderChipGroupWrongComponentClass => {
                "provider_chip_group_wrong_component_class"
            }
            Self::ProjectOrSpaceScopeMissing => "project_or_space_scope_missing",
            Self::ChipWritabilityMisrepresented => "chip_writability_misrepresented",
            Self::ChipPostureMisrepresented => "chip_posture_misrepresented",
            Self::ReadOnlyNoteMissing => "read_only_note_missing",
            Self::OfflineCaptureNoteMissing => "offline_capture_note_missing",
            Self::PolicyBlockNoteMissing => "policy_block_note_missing",
            Self::TenantScopeNoteMissing => "tenant_scope_note_missing",
            Self::WritePostureCoverageMissing => "write_posture_coverage_missing",
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

/// Reads and validates the checked-in stable work-item-row / provider-chip export.
pub fn current_work_item_row_provider_chip_export(
) -> Result<WorkItemRowProviderChipControlsPacket, WorkItemRowProviderChipArtifactError> {
    let packet: WorkItemRowProviderChipControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-work-item-row-provider-chip-proof/support_export.json"
        )))
        .map_err(WorkItemRowProviderChipArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(WorkItemRowProviderChipArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &WorkItemRowProviderChipControlsPacket,
    violations: &mut Vec<WorkItemRowProviderChipViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF,
        WORK_ITEM_ROW_PROVIDER_CHIP_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_WORK_ITEM_ROW_SCHEMA_REF,
        M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(WorkItemRowProviderChipViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_work_item_rows(
    packet: &WorkItemRowProviderChipControlsPacket,
    violations: &mut Vec<WorkItemRowProviderChipViolation>,
) {
    if packet.work_item_rows.is_empty() {
        violations.push(WorkItemRowProviderChipViolation::WorkItemRowsMissing);
        return;
    }

    let mut authority_classes: BTreeSet<WorkItemStateAuthorityClass> = BTreeSet::new();

    for row in &packet.work_item_rows {
        let disclosure = row.state_authority_disclosure();
        authority_classes.insert(disclosure.authority_class);

        if row.row_id.trim().is_empty()
            || row.title.trim().is_empty()
            || row.owner_label.trim().is_empty()
            || row.priority_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(WorkItemRowProviderChipViolation::WorkItemRowIncomplete);
        }
        if row.component != M5WorkItemComponentFamily::WorkItemRow {
            violations.push(WorkItemRowProviderChipViolation::WorkItemRowWrongComponentClass);
        }
        if row.canonical_id.trim().is_empty() || !row.canonical_id_copyable {
            violations.push(WorkItemRowProviderChipViolation::CanonicalIdNotCopyable);
        }
        if !row.declares_mandatory_actions() {
            violations.push(WorkItemRowProviderChipViolation::DefaultActionsIncomplete);
        }
        if row.state_authority_class != disclosure.authority_class
            || row.claims_provider_authoritative != disclosure.is_provider_authoritative
        {
            violations.push(WorkItemRowProviderChipViolation::StateAuthorityMisrepresented);
        }
        if disclosure.needs_local_state_note && row.local_state_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::LocalStateNoteMissing);
        }
        if disclosure.needs_publish_pending_note && row.publish_pending_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::PublishPendingNoteMissing);
        }
        if disclosure.needs_blocked_note && row.blocked_capability_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::BlockedCapabilityNoteMissing);
        }
        if row.linked_change_count > 0 && row.linked_change_label.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::LinkedChangeLabelMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(WorkItemRowProviderChipViolation::AccessibilityRouteMissing);
        }
        if row.uses_generic_ticket_wording {
            violations.push(WorkItemRowProviderChipViolation::GenericTicketWordingUsed);
        }
    }

    for required in WorkItemStateAuthorityClass::ALL {
        if !authority_classes.contains(&required) {
            violations.push(WorkItemRowProviderChipViolation::StateAuthorityCoverageMissing);
            break;
        }
    }
}

fn validate_provider_chip_groups(
    packet: &WorkItemRowProviderChipControlsPacket,
    violations: &mut Vec<WorkItemRowProviderChipViolation>,
) {
    if packet.provider_chip_groups.is_empty() {
        violations.push(WorkItemRowProviderChipViolation::ProviderChipGroupsMissing);
        return;
    }

    let mut postures: BTreeSet<ProviderChipWritePosture> = BTreeSet::new();

    for group in &packet.provider_chip_groups {
        postures.insert(group.write_posture);
        let disclosure = group.chip_disclosure();

        if group.group_id.trim().is_empty()
            || group.provider_label.trim().is_empty()
            || group.fields_shown.is_empty()
            || group.surface_families.is_empty()
            || group.deployment_lines.is_empty()
            || group.consumer_surfaces.is_empty()
            || group.source_contract_refs.is_empty()
        {
            violations.push(WorkItemRowProviderChipViolation::ProviderChipGroupIncomplete);
        }
        if group.component != M5WorkItemComponentFamily::ProviderChipGroup {
            violations.push(WorkItemRowProviderChipViolation::ProviderChipGroupWrongComponentClass);
        }
        if group.project_or_space_label.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::ProjectOrSpaceScopeMissing);
        }
        if group.is_writable != disclosure.is_writable {
            violations.push(WorkItemRowProviderChipViolation::ChipWritabilityMisrepresented);
        }
        if !disclosure.posture_matches_authority {
            violations.push(WorkItemRowProviderChipViolation::ChipPostureMisrepresented);
        }
        if disclosure.needs_read_only_note && group.read_only_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::ReadOnlyNoteMissing);
        }
        if disclosure.needs_offline_capture_note && group.offline_capture_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::OfflineCaptureNoteMissing);
        }
        if disclosure.needs_policy_block_note && group.policy_block_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::PolicyBlockNoteMissing);
        }
        if group.has_tenant_scope && group.tenant_scope_note.trim().is_empty() {
            violations.push(WorkItemRowProviderChipViolation::TenantScopeNoteMissing);
        }
        if group.accessibility_routes.is_empty()
            || !group
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(WorkItemRowProviderChipViolation::AccessibilityRouteMissing);
        }
        if group.uses_generic_ticket_wording {
            violations.push(WorkItemRowProviderChipViolation::GenericTicketWordingUsed);
        }
    }

    for required in ProviderChipWritePosture::ALL {
        if !postures.contains(&required) {
            violations.push(WorkItemRowProviderChipViolation::WritePostureCoverageMissing);
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
