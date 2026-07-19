//! Work-item detail headers and status-transition sheets carrying provider-boundary,
//! side-effect, permission-scope, and confirm/export/cancel truth.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_work_item_component_matrix`] — the `work_item_detail_header`
//! and the `status_transition_sheet` — into one implemented, export-safe packet with
//! two co-equal control vectors. Together they make the durable work-item detail
//! surface and every publish-capable transition explicit about identity, boundary,
//! side effects, permission scope, and publish-later continuity.
//!
//! A [`DetailHeader`] states the provider/project-space, canonical id, title, current
//! state, assignee/owner, freshness, and write scope of a work item, and always offers
//! an open-external escape hatch. Its write scope ([`HeaderWriteScope`]) and freshness
//! ([`HeaderFreshnessClass`]) are *derived* from provider authority and the
//! local-versus-provider state rather than asserted, so a local draft never reads as a
//! provider-backed object and a stale snapshot never reads as live.
//!
//! A [`StatusTransitionSheet`] previews, before any publish, what will change (comment,
//! state, assignment, link, or field mutations), the linked branch/review context, the
//! notification side effects, the permission scope that can authorize the change, and
//! its confirm/export/cancel behavior. Its publish class
//! ([`TransitionPublishClass`]) is *derived* from the transition effect, the local
//! state, and a policy-block flag, so a local-only transition never implies external
//! mutation before confirmation, and a metadata-safe export fallback always exists when
//! publish cannot proceed.
//!
//! The work-item kind ([`M5WorkItemKind`]), provider authority
//! ([`M5WorkItemProviderAuthority`]), local-versus-provider state
//! ([`M5WorkItemLocalState`]), transition effect ([`M5WorkItemTransitionEffect`]),
//! surface families ([`M5WorkItemSurfaceFamily`]), deployment lines
//! ([`M5WorkItemDeploymentLine`]), consumer surfaces
//! ([`M5WorkItemConsumerSurface`]), accessibility routes
//! ([`M5WorkItemAccessibilityRoute`]), and downgrade triggers
//! ([`M5WorkItemDowngradeTrigger`]) are reused directly from the frozen matrix, so this
//! lane never invents a parallel work-item vocabulary. It mints new vocabulary only for
//! what that matrix left implicit about these two controls: the derived header write
//! scope, the derived header freshness class, the metadata-safe header actions, the
//! transition mutation kinds, the derived transition publish class, the permission
//! scope class, and the confirm/export/cancel transition actions.
//!
//! Raw work-item bodies, pasted paths, credentials, and private endpoints stay outside
//! the support boundary; canonical ids and references are carried only as opaque,
//! export-safe strings.
//!
//! The boundary schema is
//! [`schemas/ui/m5-work-item-detail-header-status-transition-controls.schema.json`](../../../../schemas/ui/m5-work-item-detail-header-status-transition-controls.schema.json).
//! The contract doc is
//! [`docs/team-workflows/implement_work_item_detail_headers_and_status_transition_sheets.md`](../../../../docs/team-workflows/implement_work_item_detail_headers_and_status_transition_sheets.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_detail_header_transition_controls,
    seeded_detail_header_transition_controls_detail_header_local_draft,
    seeded_detail_header_transition_controls_status_transition_publish_now,
    DETAIL_HEADER_TRANSITION_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The work-item kind, provider authority, local-versus-provider state, transition
// effect, surface family, deployment line, consumer surface, accessibility route, and
// downgrade triggers are frozen once, in the work-item component matrix. This lane
// reuses them verbatim so it never invents a parallel work-item vocabulary.
use crate::freeze_the_m5_work_item_component_matrix::{
    M5WorkItemAccessibilityRoute, M5WorkItemComponentFamily, M5WorkItemConsumerSurface,
    M5WorkItemDeploymentLine, M5WorkItemDowngradeTrigger, M5WorkItemKind, M5WorkItemLocalState,
    M5WorkItemProviderAuthority, M5WorkItemSurfaceFamily, M5WorkItemTransitionEffect,
    M5_STATUS_TRANSITION_SHEET_SCHEMA_REF, M5_WORK_ITEM_COMPONENT_DOC_REF,
    M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF, M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`DetailHeaderTransitionControlsPacket`].
pub const DETAIL_HEADER_TRANSITION_RECORD_KIND: &str = "detail_header_status_transition_controls";

/// Schema version for detail-header / status-transition-sheet control records.
pub const DETAIL_HEADER_TRANSITION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DETAIL_HEADER_TRANSITION_SCHEMA_REF: &str =
    "schemas/ui/m5-work-item-detail-header-status-transition-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const DETAIL_HEADER_TRANSITION_DOC_REF: &str =
    "docs/team-workflows/implement_work_item_detail_headers_and_status_transition_sheets.md";

/// Repo-relative path of the protected fixture directory.
pub const DETAIL_HEADER_TRANSITION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-work-item-detail-header-status-transition-controls";

/// Repo-relative path of the checked support-export artifact.
pub const DETAIL_HEADER_TRANSITION_ARTIFACT_REF: &str =
    "artifacts/release/m5-work-item-detail-header-status-transition-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DETAIL_HEADER_TRANSITION_SUMMARY_REF: &str =
    "artifacts/release/m5-work-item-detail-header-status-transition-proof/summary.md";

// ---- detail-header vocabulary -------------------------------------------

/// Derived write scope a work-item detail header may present.
///
/// This is one of the header honesty axes: the scope is derived from the provider
/// authority and the local-versus-provider state, never asserted, so a header never
/// implies Aureline may write to a provider object when it may not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderWriteScope {
    /// The provider owns the object and Aureline may write to it.
    ProviderWritable,
    /// A local draft not yet owned by any provider; writes stay local.
    LocalDraftOnly,
    /// A read-only mirror or imported snapshot; Aureline may not write.
    ReadOnlyMirror,
    /// Writes are blocked by policy.
    PolicyBlockedWrite,
}

impl HeaderWriteScope {
    /// Every write scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderWritable,
        Self::LocalDraftOnly,
        Self::ReadOnlyMirror,
        Self::PolicyBlockedWrite,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderWritable => "provider_writable",
            Self::LocalDraftOnly => "local_draft_only",
            Self::ReadOnlyMirror => "read_only_mirror",
            Self::PolicyBlockedWrite => "policy_blocked_write",
        }
    }

    /// Whether this scope permits writing to the provider.
    pub const fn is_provider_writable(self) -> bool {
        matches!(self, Self::ProviderWritable)
    }
}

/// Derived freshness class a work-item detail header may present.
///
/// This is the other header honesty axis: the class is derived from the provider
/// authority, whether the reference is current, and whether freshness is known, never
/// asserted, so a stale snapshot or a local-only draft never reads as a live,
/// reconciled object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderFreshnessClass {
    /// The header reflects live, reconciled provider truth.
    LiveSynced,
    /// The header reflects a snapshot that is out of date or detached from live truth.
    StaleSnapshot,
    /// The header reflects a local-only draft with no provider truth yet.
    LocalOnly,
    /// The header's freshness cannot currently be determined.
    UnknownFreshness,
}

impl HeaderFreshnessClass {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveSynced,
        Self::StaleSnapshot,
        Self::LocalOnly,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSynced => "live_synced",
            Self::StaleSnapshot => "stale_snapshot",
            Self::LocalOnly => "local_only",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }

    /// Whether this class is live, reconciled provider truth.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveSynced)
    }
}

/// One keyboard-complete, metadata-safe action a detail header offers, so the header
/// never hides its copy or open-external affordance behind a pointer-only gesture and
/// never exports raw bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailHeaderAction {
    /// Copy the canonical id (always available).
    CopyCanonicalId,
    /// Open the item in its owning provider — the escape hatch (always available).
    OpenExternal,
    /// Reveal the item's provider scope / provenance.
    RevealScope,
    /// Export the header as metadata-safe evidence.
    ExportHeader,
}

impl DetailHeaderAction {
    /// Every header action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CopyCanonicalId,
        Self::OpenExternal,
        Self::RevealScope,
        Self::ExportHeader,
    ];

    /// The copy-id / open-external actions every header must offer.
    pub const MANDATORY: [Self; 2] = [Self::CopyCanonicalId, Self::OpenExternal];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyCanonicalId => "copy_canonical_id",
            Self::OpenExternal => "open_external",
            Self::RevealScope => "reveal_scope",
            Self::ExportHeader => "export_header",
        }
    }
}

/// Disclosures a detail header must carry, derived from authority, state, and freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderDisclosure {
    /// The derived write scope this header may present.
    pub write_scope: HeaderWriteScope,
    /// The derived freshness class this header may present.
    pub freshness_class: HeaderFreshnessClass,
    /// Whether the item is backed by a real provider object (never a bare local draft).
    pub is_provider_backed: bool,
    /// Whether the item is a local-only draft with no provider object yet.
    pub is_local_draft: bool,
    /// Whether the provider owns the object and Aureline may write to it.
    pub is_provider_writable: bool,
    /// Whether the header must carry an explicit write-scope note (any non-writable scope).
    pub needs_write_scope_note: bool,
    /// Whether the header must carry an explicit freshness note (any non-live class).
    pub needs_freshness_note: bool,
    /// Whether the header must carry an explicit policy-block note.
    pub needs_policy_note: bool,
}

/// Resolves the boundary truth a detail header may present.
///
/// A policy-pinned item is policy-blocked for writes. A mirror or imported snapshot is
/// read-only. A local draft or unlinked local item is local-only. Otherwise the
/// provider owns the object and it is writable. Freshness is live only for a
/// provider-backed object whose reference is current and whose freshness is known.
pub fn resolve_detail_header(
    provider_authority: M5WorkItemProviderAuthority,
    local_state: M5WorkItemLocalState,
    is_reference_current: bool,
    is_freshness_known: bool,
) -> HeaderDisclosure {
    use M5WorkItemProviderAuthority as Authority;

    let is_local_draft = matches!(
        provider_authority,
        Authority::LocalDraft | Authority::UnlinkedLocal
    ) || (matches!(provider_authority, Authority::ProviderOwned)
        && matches!(local_state, M5WorkItemLocalState::LocalOnlyDraft));
    let is_provider_backed = !matches!(
        provider_authority,
        Authority::LocalDraft | Authority::UnlinkedLocal
    );

    let write_scope = if matches!(provider_authority, Authority::PolicyPinned) {
        HeaderWriteScope::PolicyBlockedWrite
    } else if matches!(
        provider_authority,
        Authority::MirroredReadOnly | Authority::ImportedSnapshot
    ) {
        HeaderWriteScope::ReadOnlyMirror
    } else if matches!(
        provider_authority,
        Authority::LocalDraft | Authority::UnlinkedLocal
    ) {
        HeaderWriteScope::LocalDraftOnly
    } else {
        HeaderWriteScope::ProviderWritable
    };

    let freshness_class = if !is_freshness_known {
        HeaderFreshnessClass::UnknownFreshness
    } else if matches!(
        provider_authority,
        Authority::LocalDraft | Authority::UnlinkedLocal
    ) {
        HeaderFreshnessClass::LocalOnly
    } else if matches!(provider_authority, Authority::ImportedSnapshot) {
        HeaderFreshnessClass::StaleSnapshot
    } else if is_reference_current {
        HeaderFreshnessClass::LiveSynced
    } else {
        HeaderFreshnessClass::StaleSnapshot
    };

    HeaderDisclosure {
        write_scope,
        freshness_class,
        is_provider_backed,
        is_local_draft,
        is_provider_writable: write_scope.is_provider_writable(),
        needs_write_scope_note: !write_scope.is_provider_writable(),
        needs_freshness_note: !freshness_class.is_live(),
        needs_policy_note: matches!(write_scope, HeaderWriteScope::PolicyBlockedWrite),
    }
}

/// A durable work-item detail header stating identity, boundary, and freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailHeader {
    /// Frozen component this control implements; must be `work_item_detail_header`.
    pub component: M5WorkItemComponentFamily,
    /// Stable header id.
    pub header_id: String,
    /// Canonical id of the work item; always non-empty.
    pub canonical_id: String,
    /// Provider / project-space label; always non-empty.
    pub provider_space_label: String,
    /// Work-item title; always non-empty.
    pub title: String,
    /// Work-item kind, reused from the frozen matrix.
    pub work_item_kind: M5WorkItemKind,
    /// Current typed state label; always non-empty.
    pub state_label: String,
    /// Assignee / owner label; always non-empty.
    pub owner_label: String,
    /// Provider authority, reused from the frozen matrix.
    pub provider_authority: M5WorkItemProviderAuthority,
    /// Local-versus-provider state, reused from the frozen matrix.
    pub local_state: M5WorkItemLocalState,
    /// Whether the linked reference is current (not out of date).
    pub is_reference_current: bool,
    /// Whether the header's freshness is currently known.
    pub is_freshness_known: bool,
    /// Derived write scope (must equal the resolved scope).
    pub write_scope: HeaderWriteScope,
    /// Derived freshness class (must equal the resolved class).
    pub freshness_class: HeaderFreshnessClass,
    /// Whether the header claims a provider-backed object (must equal the derived truth).
    pub claims_provider_backed: bool,
    /// Write-scope note; required when the item is not provider-writable.
    pub write_scope_note: String,
    /// Freshness note; required when the header is not live-synced.
    pub freshness_note: String,
    /// Policy-block note; required when writes are policy-blocked.
    pub policy_block_note: String,
    /// Metadata-safe actions this header offers (must include copy-id and open-external).
    pub actions: Vec<DetailHeaderAction>,
    /// Claimed M5 work-item surface families that render this header.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this header keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this header offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this header's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this header.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals authority or boundary.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl DetailHeader {
    /// Boundary disclosures this header must carry, derived from authority, state, freshness.
    pub fn boundary_disclosure(&self) -> HeaderDisclosure {
        resolve_detail_header(
            self.provider_authority,
            self.local_state,
            self.is_reference_current,
            self.is_freshness_known,
        )
    }

    /// Whether the header offers every mandatory metadata-safe action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DetailHeaderAction> = self.actions.iter().copied().collect();
        DetailHeaderAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- status-transition-sheet vocabulary ---------------------------------

/// The kind of mutation a status-transition sheet previews before write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionMutationKind {
    /// A comment will be posted.
    CommentMutation,
    /// The state / status will change.
    StateMutation,
    /// The assignee / owner will change.
    AssignmentMutation,
    /// A link / relation will change.
    LinkMutation,
    /// A field / property will change.
    FieldMutation,
}

impl TransitionMutationKind {
    /// Every mutation kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CommentMutation,
        Self::StateMutation,
        Self::AssignmentMutation,
        Self::LinkMutation,
        Self::FieldMutation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommentMutation => "comment_mutation",
            Self::StateMutation => "state_mutation",
            Self::AssignmentMutation => "assignment_mutation",
            Self::LinkMutation => "link_mutation",
            Self::FieldMutation => "field_mutation",
        }
    }
}

/// Derived publish class a status-transition sheet may present.
///
/// This is the transition honesty axis: the class is derived from the transition
/// effect, the local-versus-provider state, and a policy-block flag, never asserted, so
/// a local-only transition never implies external mutation before confirmation, and a
/// blocked or policy-held transition always names its block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionPublishClass {
    /// A local-only transition; nothing publishes to the provider.
    LocalDraftOnly,
    /// The transition publishes to the provider on confirm.
    PublishesToProvider,
    /// The transition opens the item in the provider instead of writing locally.
    OpensInProvider,
    /// The transition is blocked and needs additional permission to proceed.
    BlockedNeedsPermission,
    /// The transition is blocked by policy and cannot be published.
    PolicyBlockedTransition,
}

impl TransitionPublishClass {
    /// Every publish class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalDraftOnly,
        Self::PublishesToProvider,
        Self::OpensInProvider,
        Self::BlockedNeedsPermission,
        Self::PolicyBlockedTransition,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraftOnly => "local_draft_only",
            Self::PublishesToProvider => "publishes_to_provider",
            Self::OpensInProvider => "opens_in_provider",
            Self::BlockedNeedsPermission => "blocked_needs_permission",
            Self::PolicyBlockedTransition => "policy_blocked_transition",
        }
    }

    /// Whether this class writes to, or opens, an external provider.
    pub const fn publishes_externally(self) -> bool {
        matches!(self, Self::PublishesToProvider | Self::OpensInProvider)
    }
}

/// Permission scope that can authorize a status transition, so a user always sees who
/// can authorize a change before attempting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScopeClass {
    /// The current user is authorized to make this change.
    CurrentUserAuthorized,
    /// The change needs an elevated project / repository role.
    NeedsElevatedRole,
    /// The change needs a re-authenticated or higher-scope provider grant.
    NeedsProviderAuth,
    /// The change is restricted by policy regardless of role.
    PolicyRestricted,
}

impl PermissionScopeClass {
    /// Every permission scope class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentUserAuthorized,
        Self::NeedsElevatedRole,
        Self::NeedsProviderAuth,
        Self::PolicyRestricted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentUserAuthorized => "current_user_authorized",
            Self::NeedsElevatedRole => "needs_elevated_role",
            Self::NeedsProviderAuth => "needs_provider_auth",
            Self::PolicyRestricted => "policy_restricted",
        }
    }
}

/// One action a status-transition sheet offers, so a publish always previews its
/// confirm/export/cancel behavior and a metadata-safe export fallback is never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionSheetAction {
    /// Confirm the transition (publishes only after this).
    Confirm,
    /// Export the pending transition as a metadata-safe packet — the fallback.
    ExportPacket,
    /// Cancel the transition without mutating anything.
    Cancel,
    /// Open the item in the owning provider instead of writing locally.
    OpenInProvider,
    /// Save the transition as a local draft for publish-later.
    SaveDraft,
}

impl TransitionSheetAction {
    /// Every transition-sheet action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Confirm,
        Self::ExportPacket,
        Self::Cancel,
        Self::OpenInProvider,
        Self::SaveDraft,
    ];

    /// The confirm/export/cancel affordances every sheet must offer before publish.
    pub const MANDATORY: [Self; 3] = [Self::Confirm, Self::ExportPacket, Self::Cancel];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confirm => "confirm",
            Self::ExportPacket => "export_packet",
            Self::Cancel => "cancel",
            Self::OpenInProvider => "open_in_provider",
            Self::SaveDraft => "save_draft",
        }
    }
}

/// Disclosures a transition sheet must carry, derived from effect, state, and policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionDisclosure {
    /// The derived publish class this sheet may present.
    pub publish_class: TransitionPublishClass,
    /// Whether the transition is local-only (nothing publishes).
    pub is_local_only: bool,
    /// Whether the transition writes to, or opens, an external provider.
    pub publishes_externally: bool,
    /// Whether the transition is blocked (needs permission or is policy-held).
    pub is_blocked: bool,
    /// Whether the sheet must carry a notification-side-effect note (external publish).
    pub needs_notification_note: bool,
    /// Whether the sheet must carry an explicit policy-block note.
    pub needs_policy_note: bool,
}

/// Resolves the publish truth a status-transition sheet may present.
///
/// A policy-blocked transition is policy-held regardless of effect. Otherwise a blocked
/// transition needs permission, a local-only transition stays local, an open-in-provider
/// effect opens externally, and a publish-now / comment / status effect publishes to the
/// provider.
pub fn resolve_transition_publish(
    transition_effect: M5WorkItemTransitionEffect,
    is_policy_blocked: bool,
) -> TransitionDisclosure {
    use M5WorkItemTransitionEffect as Effect;
    use TransitionPublishClass as Class;

    let publish_class = if is_policy_blocked {
        Class::PolicyBlockedTransition
    } else if matches!(transition_effect, Effect::BlockedTransition) {
        Class::BlockedNeedsPermission
    } else if matches!(transition_effect, Effect::LocalOnlyTransition) {
        Class::LocalDraftOnly
    } else if matches!(transition_effect, Effect::OpenInProvider) {
        Class::OpensInProvider
    } else {
        Class::PublishesToProvider
    };

    let publishes_externally = publish_class.publishes_externally();

    TransitionDisclosure {
        publish_class,
        is_local_only: matches!(publish_class, Class::LocalDraftOnly),
        publishes_externally,
        is_blocked: matches!(
            publish_class,
            Class::BlockedNeedsPermission | Class::PolicyBlockedTransition
        ),
        needs_notification_note: publishes_externally,
        needs_policy_note: matches!(publish_class, Class::PolicyBlockedTransition),
    }
}

/// A status-transition sheet previewing side effects, permission, and confirm/export/cancel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusTransitionSheet {
    /// Frozen component this control implements; must be `status_transition_sheet`.
    pub component: M5WorkItemComponentFamily,
    /// Stable sheet id.
    pub sheet_id: String,
    /// Canonical id of the work item this sheet mutates; always non-empty.
    pub canonical_id: String,
    /// The status being transitioned from; always non-empty.
    pub from_status: String,
    /// The status being transitioned to; always non-empty.
    pub to_status: String,
    /// The mutations this transition previews (min one).
    pub mutation_kinds: Vec<TransitionMutationKind>,
    /// The transition effects, reused from the frozen matrix (min one).
    pub transition_effects: Vec<M5WorkItemTransitionEffect>,
    /// The transition effect that drives the derived publish class.
    pub primary_transition_effect: M5WorkItemTransitionEffect,
    /// Local-versus-provider state, reused from the frozen matrix.
    pub local_state: M5WorkItemLocalState,
    /// Whether the transition is blocked by policy.
    pub is_policy_blocked: bool,
    /// Derived publish class (must equal the resolved class).
    pub publish_class: TransitionPublishClass,
    /// Whether the sheet implies external mutation (must equal the derived truth).
    pub implies_external_mutation: bool,
    /// Side-effect preview label — what will change; always non-empty.
    pub side_effect_preview_label: String,
    /// Linked branch/review context note; always non-empty.
    pub linked_context_note: String,
    /// Notification-side-effect note; required when the transition publishes externally.
    pub notification_side_effect_note: String,
    /// Permission scope that can authorize this transition.
    pub permission_scope: PermissionScopeClass,
    /// Permission-scope note — who can authorize; always non-empty.
    pub permission_scope_note: String,
    /// Confirm/export/cancel actions (must include the mandatory confirm/export/cancel).
    pub actions: Vec<TransitionSheetAction>,
    /// Export-fallback note — what fallback exists when publish cannot proceed; non-empty.
    pub export_fallback_note: String,
    /// Policy-block note; required when the transition is policy-blocked.
    pub policy_block_note: String,
    /// Claimed M5 work-item surface families that render this sheet.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this sheet keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Non-visual accessibility routes this sheet offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this sheet's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this sheet.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: generic ticket/task wording never conceals side effects or scope.
    /// MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl StatusTransitionSheet {
    /// Publish disclosures this sheet must carry, derived from effect and policy.
    pub fn publish_disclosure(&self) -> TransitionDisclosure {
        resolve_transition_publish(self.primary_transition_effect, self.is_policy_blocked)
    }

    /// Whether the sheet offers every mandatory confirm/export/cancel action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<TransitionSheetAction> = self.actions.iter().copied().collect();
        TransitionSheetAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- review blocks ------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailHeaderTransitionTrustReview {
    /// The detail header states provider space, canonical id, title, state, and owner.
    pub header_states_identity_and_owner: bool,
    /// The header's write scope is derived from authority, never asserted.
    pub header_write_scope_derived: bool,
    /// The header's freshness is derived and a stale snapshot never reads as live.
    pub header_freshness_derived: bool,
    /// A local draft never reads as a provider-backed object.
    pub local_draft_never_reads_provider_backed: bool,
    /// The header always offers an open-external escape hatch.
    pub header_offers_open_external_escape_hatch: bool,
    /// The transition sheet previews what will change before publish.
    pub transition_previews_mutations_before_publish: bool,
    /// A local-only transition never implies external mutation before confirmation.
    pub local_transition_never_implies_external_mutation: bool,
    /// The transition sheet discloses notification side effects on external publish.
    pub transition_discloses_notification_side_effects: bool,
    /// The transition sheet names who can authorize the change.
    pub transition_names_permission_scope: bool,
    /// The transition sheet offers confirm/export/cancel behavior before publish.
    pub transition_offers_confirm_export_cancel: bool,
    /// A metadata-safe export fallback stays available when publish cannot proceed.
    pub export_fallback_always_available: bool,
    /// No generic ticket/task wording conceals authority, side effects, or scope.
    pub no_generic_ticket_wording_conceals_truth: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl DetailHeaderTransitionTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.header_states_identity_and_owner
            && self.header_write_scope_derived
            && self.header_freshness_derived
            && self.local_draft_never_reads_provider_backed
            && self.header_offers_open_external_escape_hatch
            && self.transition_previews_mutations_before_publish
            && self.local_transition_never_implies_external_mutation
            && self.transition_discloses_notification_side_effects
            && self.transition_names_permission_scope
            && self.transition_offers_confirm_export_cancel
            && self.export_fallback_always_available
            && self.no_generic_ticket_wording_conceals_truth
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailHeaderTransitionConsumerProjection {
    /// The detail surface renders the header with derived boundary and freshness.
    pub detail_surface_renders_header_boundary: bool,
    /// The transition-sheet surface previews side effects and permission before publish.
    pub transition_surface_previews_before_publish: bool,
    /// The confirm/export/cancel and open-external paths are reachable headless.
    pub confirm_export_cancel_reachable_headless: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl DetailHeaderTransitionConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.detail_surface_renders_header_boundary
            && self.transition_surface_previews_before_publish
            && self.confirm_export_cancel_reachable_headless
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailHeaderTransitionProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`DetailHeaderTransitionControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetailHeaderTransitionControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Detail headers.
    pub detail_headers: Vec<DetailHeader>,
    /// Status-transition sheets.
    pub status_transition_sheets: Vec<StatusTransitionSheet>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: DetailHeaderTransitionTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DetailHeaderTransitionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DetailHeaderTransitionProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe detail-header / status-transition-sheet controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailHeaderTransitionControlsPacket {
    /// Record kind; must equal [`DETAIL_HEADER_TRANSITION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DETAIL_HEADER_TRANSITION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Detail headers.
    pub detail_headers: Vec<DetailHeader>,
    /// Status-transition sheets.
    pub status_transition_sheets: Vec<StatusTransitionSheet>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Trust review block.
    pub trust_review: DetailHeaderTransitionTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DetailHeaderTransitionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DetailHeaderTransitionProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DetailHeaderTransitionControlsPacket {
    /// Builds a detail-header / status-transition-sheet controls packet from stable-lane input.
    pub fn new(input: DetailHeaderTransitionControlsPacketInput) -> Self {
        Self {
            record_kind: DETAIL_HEADER_TRANSITION_RECORD_KIND.to_owned(),
            schema_version: DETAIL_HEADER_TRANSITION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            detail_headers: input.detail_headers,
            status_transition_sheets: input.status_transition_sheets,
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

    /// Validates the detail-header / status-transition-sheet control invariants.
    pub fn validate(&self) -> Vec<DetailHeaderTransitionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DETAIL_HEADER_TRANSITION_RECORD_KIND {
            violations.push(DetailHeaderTransitionViolation::WrongRecordKind);
        }
        if self.schema_version != DETAIL_HEADER_TRANSITION_SCHEMA_VERSION {
            violations.push(DetailHeaderTransitionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DetailHeaderTransitionViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(DetailHeaderTransitionViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(DetailHeaderTransitionViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_detail_headers(self, &mut violations);
        validate_transition_sheets(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(DetailHeaderTransitionViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(DetailHeaderTransitionViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(DetailHeaderTransitionViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("detail header transition packet serializes"),
        ) {
            violations.push(DetailHeaderTransitionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("detail header transition packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("control,id,kind_or_from,state_or_to,derived,external_or_writable\n");
        for header in &self.detail_headers {
            let disclosure = header.boundary_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{}|{},{}\n",
                "detail_header",
                csv_field(&header.header_id),
                header.work_item_kind.as_str(),
                header.local_state.as_str(),
                disclosure.write_scope.as_str(),
                disclosure.freshness_class.as_str(),
                disclosure.is_provider_writable,
            ));
        }
        for sheet in &self.status_transition_sheets {
            let disclosure = sheet.publish_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                "status_transition_sheet",
                csv_field(&sheet.sheet_id),
                csv_field(&sheet.from_status),
                csv_field(&sheet.to_status),
                disclosure.publish_class.as_str(),
                disclosure.publishes_externally,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let local_drafts = self
            .detail_headers
            .iter()
            .filter(|header| header.boundary_disclosure().is_local_draft)
            .count();
        let external_transitions = self
            .status_transition_sheets
            .iter()
            .filter(|sheet| sheet.publish_disclosure().publishes_externally)
            .count();

        let mut out = String::new();
        out.push_str("# Work-item detail headers and status-transition sheets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Detail headers: {} ({} local drafts)\n",
            self.detail_headers.len(),
            local_drafts
        ));
        out.push_str(&format!(
            "- Status-transition sheets: {} ({} publish externally)\n",
            self.status_transition_sheets.len(),
            external_transitions
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Detail headers\n\n");
        for header in &self.detail_headers {
            let disclosure = header.boundary_disclosure();
            out.push_str(&format!(
                "- **{}** ({}) [{} / {}] → `{}`\n",
                header.header_id,
                header.canonical_id,
                disclosure.write_scope.as_str(),
                disclosure.freshness_class.as_str(),
                header.state_label,
            ));
        }

        out.push_str("\n## Status-transition sheets\n\n");
        for sheet in &self.status_transition_sheets {
            out.push_str(&format!(
                "- **{}** `{}` → `{}` [{}] auth: {}\n",
                sheet.sheet_id,
                sheet.from_status,
                sheet.to_status,
                sheet.publish_disclosure().publish_class.as_str(),
                sheet.permission_scope.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in detail-header / transition export.
#[derive(Debug)]
pub enum DetailHeaderTransitionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DetailHeaderTransitionViolation>),
}

impl fmt::Display for DetailHeaderTransitionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "detail header transition export parse failed: {error}"
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
                    "detail header transition export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DetailHeaderTransitionArtifactError {}

/// Validation failures emitted by [`DetailHeaderTransitionControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetailHeaderTransitionViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No detail headers are present.
    DetailHeadersMissing,
    /// A detail header is incomplete.
    DetailHeaderIncomplete,
    /// A detail header carries the wrong frozen component class.
    DetailHeaderWrongComponentClass,
    /// A detail header misrepresents its derived write scope.
    HeaderWriteScopeMisrepresented,
    /// A detail header misrepresents its derived freshness class.
    HeaderFreshnessMisrepresented,
    /// A local draft reads as, or claims to be, a provider-backed object.
    LocalDraftMisrepresentedAsProviderBacked,
    /// A non-writable header does not name its write scope.
    WriteScopeNoteMissing,
    /// A non-live header does not name its freshness.
    FreshnessNoteMissing,
    /// A policy-blocked header does not name its policy block.
    HeaderPolicyBlockNoteMissing,
    /// A header omits the mandatory copy-id / open-external escape hatch.
    HeaderOpenExternalOrCopyMissing,
    /// The detail headers do not cover every derived write scope.
    HeaderWriteScopeCoverageMissing,
    /// The detail headers do not cover every derived freshness class.
    HeaderFreshnessCoverageMissing,
    /// No status-transition sheets are present.
    StatusTransitionSheetsMissing,
    /// A status-transition sheet is incomplete.
    StatusTransitionSheetIncomplete,
    /// A status-transition sheet carries the wrong frozen component class.
    StatusTransitionSheetWrongComponentClass,
    /// A status-transition sheet does not name its from/to status.
    TransitionStatusLabelsMissing,
    /// A status-transition sheet previews no mutations.
    TransitionMutationKindsMissing,
    /// A status-transition sheet lists no transition effects.
    TransitionEffectsMissing,
    /// A status-transition sheet misrepresents its derived publish class.
    TransitionPublishClassMisrepresented,
    /// A local-only transition implies, or a publishing transition denies, external mutation.
    ExternalMutationMisrepresented,
    /// A status-transition sheet does not preview what will change before publish.
    SideEffectPreviewMissing,
    /// A status-transition sheet does not name its linked branch/review context.
    LinkedContextNoteMissing,
    /// An externally-publishing transition does not name its notification side effects.
    NotificationSideEffectNoteMissing,
    /// A status-transition sheet does not name who can authorize the change.
    PermissionScopeNoteMissing,
    /// A status-transition sheet omits a mandatory confirm/export/cancel action.
    ConfirmExportCancelIncomplete,
    /// A status-transition sheet does not name its export fallback.
    ExportFallbackNoteMissing,
    /// A policy-blocked transition does not name its policy block.
    TransitionPolicyBlockNoteMissing,
    /// The transition sheets do not cover every derived publish class.
    TransitionPublishClassCoverageMissing,
    /// The transition sheets do not cover every mutation kind.
    TransitionMutationKindCoverageMissing,
    /// The transition sheets do not cover every permission scope class.
    PermissionScopeCoverageMissing,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control lets generic ticket/task wording conceal authority, side effects, or scope.
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

impl DetailHeaderTransitionViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DetailHeadersMissing => "detail_headers_missing",
            Self::DetailHeaderIncomplete => "detail_header_incomplete",
            Self::DetailHeaderWrongComponentClass => "detail_header_wrong_component_class",
            Self::HeaderWriteScopeMisrepresented => "header_write_scope_misrepresented",
            Self::HeaderFreshnessMisrepresented => "header_freshness_misrepresented",
            Self::LocalDraftMisrepresentedAsProviderBacked => {
                "local_draft_misrepresented_as_provider_backed"
            }
            Self::WriteScopeNoteMissing => "write_scope_note_missing",
            Self::FreshnessNoteMissing => "freshness_note_missing",
            Self::HeaderPolicyBlockNoteMissing => "header_policy_block_note_missing",
            Self::HeaderOpenExternalOrCopyMissing => "header_open_external_or_copy_missing",
            Self::HeaderWriteScopeCoverageMissing => "header_write_scope_coverage_missing",
            Self::HeaderFreshnessCoverageMissing => "header_freshness_coverage_missing",
            Self::StatusTransitionSheetsMissing => "status_transition_sheets_missing",
            Self::StatusTransitionSheetIncomplete => "status_transition_sheet_incomplete",
            Self::StatusTransitionSheetWrongComponentClass => {
                "status_transition_sheet_wrong_component_class"
            }
            Self::TransitionStatusLabelsMissing => "transition_status_labels_missing",
            Self::TransitionMutationKindsMissing => "transition_mutation_kinds_missing",
            Self::TransitionEffectsMissing => "transition_effects_missing",
            Self::TransitionPublishClassMisrepresented => "transition_publish_class_misrepresented",
            Self::ExternalMutationMisrepresented => "external_mutation_misrepresented",
            Self::SideEffectPreviewMissing => "side_effect_preview_missing",
            Self::LinkedContextNoteMissing => "linked_context_note_missing",
            Self::NotificationSideEffectNoteMissing => "notification_side_effect_note_missing",
            Self::PermissionScopeNoteMissing => "permission_scope_note_missing",
            Self::ConfirmExportCancelIncomplete => "confirm_export_cancel_incomplete",
            Self::ExportFallbackNoteMissing => "export_fallback_note_missing",
            Self::TransitionPolicyBlockNoteMissing => "transition_policy_block_note_missing",
            Self::TransitionPublishClassCoverageMissing => {
                "transition_publish_class_coverage_missing"
            }
            Self::TransitionMutationKindCoverageMissing => {
                "transition_mutation_kind_coverage_missing"
            }
            Self::PermissionScopeCoverageMissing => "permission_scope_coverage_missing",
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

/// Reads and validates the checked-in stable detail-header / transition export.
pub fn current_detail_header_transition_export(
) -> Result<DetailHeaderTransitionControlsPacket, DetailHeaderTransitionArtifactError> {
    let packet: DetailHeaderTransitionControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-work-item-detail-header-status-transition-proof/support_export.json"
    )))
    .map_err(DetailHeaderTransitionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DetailHeaderTransitionArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &DetailHeaderTransitionControlsPacket,
    violations: &mut Vec<DetailHeaderTransitionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DETAIL_HEADER_TRANSITION_SCHEMA_REF,
        DETAIL_HEADER_TRANSITION_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
        M5_STATUS_TRANSITION_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DetailHeaderTransitionViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_detail_headers(
    packet: &DetailHeaderTransitionControlsPacket,
    violations: &mut Vec<DetailHeaderTransitionViolation>,
) {
    if packet.detail_headers.is_empty() {
        violations.push(DetailHeaderTransitionViolation::DetailHeadersMissing);
        return;
    }

    let mut write_scopes: BTreeSet<HeaderWriteScope> = BTreeSet::new();
    let mut freshness_classes: BTreeSet<HeaderFreshnessClass> = BTreeSet::new();

    for header in &packet.detail_headers {
        let disclosure = header.boundary_disclosure();
        write_scopes.insert(disclosure.write_scope);
        freshness_classes.insert(disclosure.freshness_class);

        if header.header_id.trim().is_empty()
            || header.canonical_id.trim().is_empty()
            || header.provider_space_label.trim().is_empty()
            || header.title.trim().is_empty()
            || header.state_label.trim().is_empty()
            || header.owner_label.trim().is_empty()
            || header.fields_shown.is_empty()
            || header.surface_families.is_empty()
            || header.deployment_lines.is_empty()
            || header.consumer_surfaces.is_empty()
            || header.source_contract_refs.is_empty()
        {
            violations.push(DetailHeaderTransitionViolation::DetailHeaderIncomplete);
        }
        if header.component != M5WorkItemComponentFamily::WorkItemDetailHeader {
            violations.push(DetailHeaderTransitionViolation::DetailHeaderWrongComponentClass);
        }
        if header.write_scope != disclosure.write_scope {
            violations.push(DetailHeaderTransitionViolation::HeaderWriteScopeMisrepresented);
        }
        if header.freshness_class != disclosure.freshness_class {
            violations.push(DetailHeaderTransitionViolation::HeaderFreshnessMisrepresented);
        }
        // AC1: a local draft never reads as a provider-backed object.
        if header.claims_provider_backed != disclosure.is_provider_backed {
            violations
                .push(DetailHeaderTransitionViolation::LocalDraftMisrepresentedAsProviderBacked);
        }
        if disclosure.needs_write_scope_note && header.write_scope_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::WriteScopeNoteMissing);
        }
        if disclosure.needs_freshness_note && header.freshness_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::FreshnessNoteMissing);
        }
        if disclosure.needs_policy_note && header.policy_block_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::HeaderPolicyBlockNoteMissing);
        }
        if !header.declares_mandatory_actions() {
            violations.push(DetailHeaderTransitionViolation::HeaderOpenExternalOrCopyMissing);
        }
        if header.accessibility_routes.is_empty()
            || !header
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(DetailHeaderTransitionViolation::AccessibilityRouteMissing);
        }
        if header.uses_generic_ticket_wording {
            violations.push(DetailHeaderTransitionViolation::GenericTicketWordingUsed);
        }
    }

    for required in HeaderWriteScope::ALL {
        if !write_scopes.contains(&required) {
            violations.push(DetailHeaderTransitionViolation::HeaderWriteScopeCoverageMissing);
            break;
        }
    }
    for required in HeaderFreshnessClass::ALL {
        if !freshness_classes.contains(&required) {
            violations.push(DetailHeaderTransitionViolation::HeaderFreshnessCoverageMissing);
            break;
        }
    }
}

fn validate_transition_sheets(
    packet: &DetailHeaderTransitionControlsPacket,
    violations: &mut Vec<DetailHeaderTransitionViolation>,
) {
    if packet.status_transition_sheets.is_empty() {
        violations.push(DetailHeaderTransitionViolation::StatusTransitionSheetsMissing);
        return;
    }

    let mut publish_classes: BTreeSet<TransitionPublishClass> = BTreeSet::new();
    let mut mutation_kinds: BTreeSet<TransitionMutationKind> = BTreeSet::new();
    let mut permission_scopes: BTreeSet<PermissionScopeClass> = BTreeSet::new();

    for sheet in &packet.status_transition_sheets {
        let disclosure = sheet.publish_disclosure();
        publish_classes.insert(disclosure.publish_class);
        permission_scopes.insert(sheet.permission_scope);
        for kind in &sheet.mutation_kinds {
            mutation_kinds.insert(*kind);
        }

        if sheet.sheet_id.trim().is_empty()
            || sheet.canonical_id.trim().is_empty()
            || sheet.fields_shown.is_empty()
            || sheet.surface_families.is_empty()
            || sheet.deployment_lines.is_empty()
            || sheet.consumer_surfaces.is_empty()
            || sheet.source_contract_refs.is_empty()
        {
            violations.push(DetailHeaderTransitionViolation::StatusTransitionSheetIncomplete);
        }
        if sheet.component != M5WorkItemComponentFamily::StatusTransitionSheet {
            violations
                .push(DetailHeaderTransitionViolation::StatusTransitionSheetWrongComponentClass);
        }
        if sheet.from_status.trim().is_empty() || sheet.to_status.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::TransitionStatusLabelsMissing);
        }
        if sheet.mutation_kinds.is_empty() {
            violations.push(DetailHeaderTransitionViolation::TransitionMutationKindsMissing);
        }
        if sheet.transition_effects.is_empty() {
            violations.push(DetailHeaderTransitionViolation::TransitionEffectsMissing);
        }
        if sheet.publish_class != disclosure.publish_class {
            violations.push(DetailHeaderTransitionViolation::TransitionPublishClassMisrepresented);
        }
        // AC1: a local-only transition never implies external mutation before confirmation.
        if sheet.implies_external_mutation != disclosure.publishes_externally {
            violations.push(DetailHeaderTransitionViolation::ExternalMutationMisrepresented);
        }
        // AC2: users can see what will change...
        if sheet.side_effect_preview_label.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::SideEffectPreviewMissing);
        }
        if sheet.linked_context_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::LinkedContextNoteMissing);
        }
        if disclosure.needs_notification_note
            && sheet.notification_side_effect_note.trim().is_empty()
        {
            violations.push(DetailHeaderTransitionViolation::NotificationSideEffectNoteMissing);
        }
        // AC2: ...who can authorize it...
        if sheet.permission_scope_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::PermissionScopeNoteMissing);
        }
        if !sheet.declares_mandatory_actions() {
            violations.push(DetailHeaderTransitionViolation::ConfirmExportCancelIncomplete);
        }
        // AC2: ...and what fallback exists when publish cannot proceed.
        if sheet.export_fallback_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::ExportFallbackNoteMissing);
        }
        if disclosure.needs_policy_note && sheet.policy_block_note.trim().is_empty() {
            violations.push(DetailHeaderTransitionViolation::TransitionPolicyBlockNoteMissing);
        }
        if sheet.accessibility_routes.is_empty()
            || !sheet
                .accessibility_routes
                .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(DetailHeaderTransitionViolation::AccessibilityRouteMissing);
        }
        if sheet.uses_generic_ticket_wording {
            violations.push(DetailHeaderTransitionViolation::GenericTicketWordingUsed);
        }
    }

    for required in TransitionPublishClass::ALL {
        if !publish_classes.contains(&required) {
            violations.push(DetailHeaderTransitionViolation::TransitionPublishClassCoverageMissing);
            break;
        }
    }
    for required in TransitionMutationKind::ALL {
        if !mutation_kinds.contains(&required) {
            violations.push(DetailHeaderTransitionViolation::TransitionMutationKindCoverageMissing);
            break;
        }
    }
    for required in PermissionScopeClass::ALL {
        if !permission_scopes.contains(&required) {
            violations.push(DetailHeaderTransitionViolation::PermissionScopeCoverageMissing);
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
