//! Stable support packet for dashboard freshness, queue order, and follow-up bundles.
//!
//! This module gives support, observability, provider-linked follow-up, copy, and
//! export flows one shared record shape. Dashboard rows carry source freshness
//! and cannot remain green when the source is cached, stale, imported,
//! truncated, or blocked. Queue rows carry order, grouping, and narrowing
//! explanations. Follow-up bundles keep checklist completion local unless a
//! separately reviewed provider command names the exact target, actor, and mode.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a dashboard freshness card.
pub const DASHBOARD_CARD_RECORD_KIND: &str = "dashboard_followup_freshness_card";
/// Stable record-kind tag for a queue-order truth record.
pub const QUEUE_TRUTH_RECORD_KIND: &str = "followup_queue_order_truth";
/// Stable record-kind tag for a provider-linked follow-up bundle.
pub const FOLLOWUP_BUNDLE_RECORD_KIND: &str = "provider_linked_followup_bundle";
/// Stable record-kind tag for the support/export parity packet.
pub const SUPPORT_EXPORT_PACKET_RECORD_KIND: &str = "followup_bundle_support_export_packet";
/// Stable record-kind tag for the whole support truth packet.
pub const TRUTH_PACKET_RECORD_KIND: &str = "dashboard_queue_followup_truth_packet";
/// Schema version for the packet records in this module.
pub const TRUTH_PACKET_SCHEMA_VERSION: u32 = 1;
/// Boundary schema path for follow-up bundles and export packets.
pub const FOLLOWUP_BUNDLE_SCHEMA_REF: &str = "/schemas/support/followup-bundle.schema.json";
/// Support review artifact path for this contract.
pub const FOLLOWUP_BUNDLE_ARTIFACT_REF: &str =
    "/artifacts/support/stabilize-dashboard-queue-and-followup-bundle-truth.md";
/// Help/support documentation path for this contract.
pub const FOLLOWUP_BUNDLE_DOC_REF: &str =
    "/docs/support/stabilize-dashboard-queue-and-followup-bundle-truth.md";
/// Fixture directory consumed by support and release evidence checks.
pub const FOLLOWUP_BUNDLE_FIXTURE_DIR: &str =
    "/fixtures/support/stabilize-dashboard-queue-and-followup-bundle-truth";

/// Source freshness for the data behind a dashboard, queue, or bundle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFreshnessClass {
    /// The source was checked within its review window.
    Fresh,
    /// The source is serving cached local data.
    Cached,
    /// The source is past its freshness window.
    Stale,
    /// The source is an imported snapshot rather than live truth.
    ImportedSnapshot,
    /// The source returned only a truncated subset.
    Truncated,
    /// Policy, provider, or locality blocked the source.
    Blocked,
}

impl SourceFreshnessClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::Truncated => "truncated",
            Self::Blocked => "blocked",
        }
    }

    /// True when the source cannot support a stable green dashboard claim.
    pub const fn downgrades_green(self) -> bool {
        !matches!(self, Self::Fresh)
    }
}

/// Effective dashboard/card state after freshness truth is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveCardState {
    /// The source supports the declared healthy state.
    Healthy,
    /// The declared healthy state was withdrawn because freshness is degraded.
    Downgraded,
    /// The source reports attention is needed.
    Attention,
    /// The source or policy blocks the card.
    Blocked,
}

impl EffectiveCardState {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Downgraded => "downgraded",
            Self::Attention => "attention",
            Self::Blocked => "blocked",
        }
    }
}

/// Why a dashboard card was visibly downgraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardDowngradeReason {
    /// Cached data cannot back a current-health claim.
    CachedData,
    /// Stale data cannot back a current-health claim.
    StaleData,
    /// Imported snapshots are historical evidence, not live truth.
    ImportedSnapshot,
    /// Truncated data means the summarized scope is incomplete.
    TruncatedData,
    /// The source is blocked by policy, provider state, or locality.
    SourceBlocked,
}

impl DashboardDowngradeReason {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CachedData => "cached_data",
            Self::StaleData => "stale_data",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::TruncatedData => "truncated_data",
            Self::SourceBlocked => "source_blocked",
        }
    }
}

impl From<SourceFreshnessClass> for Option<DashboardDowngradeReason> {
    fn from(freshness: SourceFreshnessClass) -> Self {
        match freshness {
            SourceFreshnessClass::Fresh => None,
            SourceFreshnessClass::Cached => Some(DashboardDowngradeReason::CachedData),
            SourceFreshnessClass::Stale => Some(DashboardDowngradeReason::StaleData),
            SourceFreshnessClass::ImportedSnapshot => {
                Some(DashboardDowngradeReason::ImportedSnapshot)
            }
            SourceFreshnessClass::Truncated => Some(DashboardDowngradeReason::TruncatedData),
            SourceFreshnessClass::Blocked => Some(DashboardDowngradeReason::SourceBlocked),
        }
    }
}

/// Dashboard card input before freshness truth is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardCardInput {
    /// Stable card identifier.
    pub card_id: String,
    /// Short card title.
    pub title: String,
    /// Current scope summarized by the card.
    pub scope: String,
    /// Source reference used to compute the card.
    pub source_ref: String,
    /// Source freshness class.
    pub source_freshness: SourceFreshnessClass,
    /// True when the upstream source declared a healthy/green state.
    pub declared_green: bool,
    /// Canonical evidence object opened by the card.
    pub open_evidence_ref: String,
    /// Reviewable source/freshness explanation.
    pub explanation: String,
}

/// Dashboard card with effective freshness truth applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardFreshnessCard {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable card identifier.
    pub card_id: String,
    /// Short card title.
    pub title: String,
    /// Current scope summarized by the card.
    pub scope: String,
    /// Source reference used to compute the card.
    pub source_ref: String,
    /// Source freshness class.
    pub source_freshness: SourceFreshnessClass,
    /// Stable source freshness token.
    pub source_freshness_token: String,
    /// True when the upstream source declared a healthy/green state.
    pub declared_green: bool,
    /// Effective card state after applying freshness truth.
    pub effective_state: EffectiveCardState,
    /// Stable effective state token.
    pub effective_state_token: String,
    /// True when a green claim was visibly withdrawn.
    pub visibly_downgraded: bool,
    /// Reasons shown next to the downgraded state.
    pub downgrade_reasons: Vec<DashboardDowngradeReason>,
    /// Stable downgrade reason tokens.
    pub downgrade_reason_tokens: Vec<String>,
    /// Canonical evidence object opened by the card.
    pub open_evidence_ref: String,
    /// Reviewable source/freshness explanation.
    pub explanation: String,
}

impl DashboardFreshnessCard {
    /// Builds a dashboard card and applies the no-stale-green rule.
    pub fn from_input(input: DashboardCardInput) -> Self {
        let mut downgrade_reasons = Vec::new();
        if let Some(reason) = Option::<DashboardDowngradeReason>::from(input.source_freshness) {
            downgrade_reasons.push(reason);
        }
        downgrade_reasons.sort();
        let visibly_downgraded = input.declared_green && input.source_freshness.downgrades_green();
        let effective_state = if input.source_freshness == SourceFreshnessClass::Blocked {
            EffectiveCardState::Blocked
        } else if visibly_downgraded {
            EffectiveCardState::Downgraded
        } else if input.declared_green {
            EffectiveCardState::Healthy
        } else {
            EffectiveCardState::Attention
        };
        let downgrade_reason_tokens = downgrade_reasons
            .iter()
            .map(|reason| reason.as_str().to_owned())
            .collect();

        Self {
            record_kind: DASHBOARD_CARD_RECORD_KIND.to_owned(),
            card_id: input.card_id,
            title: input.title,
            scope: input.scope,
            source_ref: input.source_ref,
            source_freshness: input.source_freshness,
            source_freshness_token: input.source_freshness.as_str().to_owned(),
            declared_green: input.declared_green,
            effective_state,
            effective_state_token: effective_state.as_str().to_owned(),
            visibly_downgraded,
            downgrade_reasons,
            downgrade_reason_tokens,
            open_evidence_ref: input.open_evidence_ref,
            explanation: input.explanation,
        }
    }
}

/// Queue row ordering reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueOrderReasonClass {
    /// Highest risk first.
    Severity,
    /// Soonest due item first.
    DueTime,
    /// Provider-owned priority is preserved and labeled.
    ProviderPriority,
    /// User or operator pinned the item.
    ManualPin,
    /// Row is grouped because it blocks other work.
    BlockingDependency,
    /// Default recency fallback.
    DefaultRecency,
}

impl QueueOrderReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Severity => "severity",
            Self::DueTime => "due_time",
            Self::ProviderPriority => "provider_priority",
            Self::ManualPin => "manual_pin",
            Self::BlockingDependency => "blocking_dependency",
            Self::DefaultRecency => "default_recency",
        }
    }
}

/// Queue narrowing reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueNarrowingReasonClass {
    /// Current workspace or workset scope hid rows.
    ScopeFilter,
    /// Policy hid rows.
    PolicyFilter,
    /// Provider account or installation grant hid rows.
    ProviderScope,
    /// Freshness floor hid rows.
    FreshnessFloor,
    /// User filter hid rows.
    SearchFilter,
}

impl QueueNarrowingReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeFilter => "scope_filter",
            Self::PolicyFilter => "policy_filter",
            Self::ProviderScope => "provider_scope",
            Self::FreshnessFloor => "freshness_floor",
            Self::SearchFilter => "search_filter",
        }
    }
}

/// Active filter state that must survive export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterState {
    /// Human-readable scope boundary.
    pub scope_label: String,
    /// Active filter tokens.
    pub active_filters: Vec<String>,
    /// Hidden result count.
    pub hidden_count: u32,
    /// Narrowing reason tokens.
    pub narrowing_reason_tokens: Vec<String>,
}

/// One hidden-scope bucket for a queue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueNarrowingTruth {
    /// Narrowing reason.
    pub reason: QueueNarrowingReasonClass,
    /// Stable narrowing reason token.
    pub reason_token: String,
    /// Count hidden by this narrowing reason.
    pub hidden_count: u32,
    /// Reviewable explanation of the hidden scope.
    pub explanation: String,
}

/// One visible queue row with explainable ordering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueRowTruth {
    /// Stable row identifier.
    pub row_id: String,
    /// One-based render rank.
    pub order_rank: u32,
    /// Why the row is ordered here.
    pub order_reason: QueueOrderReasonClass,
    /// Stable order reason token.
    pub order_reason_token: String,
    /// Reviewable ordering explanation.
    pub order_explanation: String,
    /// Reviewable grouping explanation.
    pub grouping_reason: String,
    /// Canonical object opened by the row.
    pub open_ref: String,
    /// Provider blocker, if any.
    #[serde(default)]
    pub provider_blocker: Option<String>,
    /// Policy blocker, if any.
    #[serde(default)]
    pub policy_blocker: Option<String>,
}

/// Queue truth record containing row order and hidden-scope explanations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueOrderTruth {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable queue identifier.
    pub queue_id: String,
    /// Human-readable queue label.
    pub queue_label: String,
    /// Active filter state.
    pub filter_state: FilterState,
    /// Visible rows in render order.
    pub rows: Vec<QueueRowTruth>,
    /// Hidden-scope buckets.
    pub hidden_scope: Vec<QueueNarrowingTruth>,
}

impl QueueOrderTruth {
    /// Returns true when every row and hidden bucket has inspectable reasoning.
    pub fn is_explainable(&self) -> bool {
        !self.rows.is_empty()
            && self.rows.iter().enumerate().all(|(index, row)| {
                row.order_rank == (index as u32) + 1
                    && !row.order_explanation.is_empty()
                    && !row.grouping_reason.is_empty()
                    && is_canonical_object_ref(&row.open_ref)
            })
            && self
                .hidden_scope
                .iter()
                .all(|hidden| hidden.hidden_count > 0 && !hidden.explanation.is_empty())
    }

    /// Returns true when provider or policy blockers are visible on the queue.
    pub fn discloses_provider_and_policy_blockers(&self) -> bool {
        self.rows.iter().any(|row| {
            row.provider_blocker
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        }) && self.rows.iter().any(|row| {
            row.policy_blocker
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        })
    }
}

/// Ownership class for linked follow-up objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkedObjectOwnershipClass {
    /// The object is owned locally by Aureline.
    LocalOwned,
    /// The object is owned by a provider.
    ProviderOwned,
    /// Ownership is shared between local state and a provider object.
    Mixed,
}

impl LinkedObjectOwnershipClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOwned => "local_owned",
            Self::ProviderOwned => "provider_owned",
            Self::Mixed => "mixed",
        }
    }
}

/// Local checklist completion semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecklistCompletionSemantics {
    /// Completion closes only a local reminder/checklist item.
    LocalReminderOnly,
    /// Completion records local evidence only.
    LocalEvidenceOnly,
    /// Completion tracks a separately reviewed provider command.
    ProviderActionTrackedSeparately,
}

impl ChecklistCompletionSemantics {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReminderOnly => "local_reminder_only",
            Self::LocalEvidenceOnly => "local_evidence_only",
            Self::ProviderActionTrackedSeparately => "provider_action_tracked_separately",
        }
    }
}

/// Reviewed provider mutation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderMutationMode {
    /// Create or update a local draft only.
    Draft,
    /// Queue the provider mutation for later publication.
    PublishLater,
    /// Publish the provider mutation immediately.
    PublishNow,
    /// Export or hand off only; no provider mutation occurs.
    HandoffOnly,
}

impl ProviderMutationMode {
    /// Stable token required by provider-linked follow-up surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::PublishLater => "publish-later",
            Self::PublishNow => "publish-now",
            Self::HandoffOnly => "handoff-only",
        }
    }
}

/// A separately reviewed command for a provider-owned object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderMutationCommand {
    /// Stable command identifier.
    pub command_id: String,
    /// Exact provider-owned target.
    pub target_ref: String,
    /// Actor that reviewed or will execute the command.
    pub actor_ref: String,
    /// Mutation mode.
    pub mode: ProviderMutationMode,
    /// Stable mutation mode token.
    pub mode_token: String,
    /// True when the command has passed separate review.
    pub reviewed_separately: bool,
}

impl ProviderMutationCommand {
    /// Returns true when the command names the exact target, actor, and mode.
    pub fn is_reviewable(&self) -> bool {
        !self.command_id.is_empty()
            && is_canonical_object_ref(&self.target_ref)
            && is_canonical_object_ref(&self.actor_ref)
            && self.mode_token == self.mode.as_str()
            && self.reviewed_separately
    }
}

/// Linked object included in a follow-up bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedObjectRef {
    /// Canonical object reference.
    pub object_ref: String,
    /// Ownership class.
    pub ownership: LinkedObjectOwnershipClass,
    /// Stable ownership token.
    pub ownership_token: String,
    /// Source freshness class.
    pub freshness: SourceFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
}

/// One checklist item in a follow-up bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecklistItem {
    /// Stable checklist item identifier.
    pub item_id: String,
    /// Short task title.
    pub title: String,
    /// Linked object reference.
    pub linked_object_ref: String,
    /// Local completion semantics.
    pub completion_semantics: ChecklistCompletionSemantics,
    /// Stable completion semantics token.
    pub completion_semantics_token: String,
    /// True would mean local completion mutates provider state and is invalid.
    pub local_completion_mutates_provider: bool,
    /// Separately reviewed command, when provider-owned mutation is intended.
    #[serde(default)]
    pub provider_mutation_command: Option<ProviderMutationCommand>,
}

impl ChecklistItem {
    /// Returns true when local completion cannot mutate provider state.
    pub fn local_completion_is_non_mutating(&self) -> bool {
        !self.local_completion_mutates_provider
    }
}

/// Provider-linked follow-up bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowupBundle {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable bundle identifier.
    pub bundle_id: String,
    /// Bundle owner.
    pub owner_ref: String,
    /// Scope carried by the bundle.
    pub scope: String,
    /// Filter state captured when the bundle was created.
    pub filter_state: FilterState,
    /// Bundle freshness.
    pub freshness: SourceFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Linked objects included by reference.
    pub linked_objects: Vec<LinkedObjectRef>,
    /// Checklist items and completion semantics.
    pub checklist_items: Vec<ChecklistItem>,
}

impl FollowupBundle {
    /// Validates ownership, checklist, and provider-command semantics.
    pub fn validate(&self) -> Result<(), TruthValidationError> {
        let mut errors = Vec::new();
        if self.record_kind != FOLLOWUP_BUNDLE_RECORD_KIND {
            errors.push("follow-up bundle record_kind is not canonical".to_owned());
        }
        if !is_canonical_object_ref(&self.owner_ref) {
            errors.push(format!("owner_ref is not canonical: {}", self.owner_ref));
        }
        for object in &self.linked_objects {
            if !is_canonical_object_ref(&object.object_ref) {
                errors.push(format!(
                    "linked object ref is not canonical: {}",
                    object.object_ref
                ));
            }
            if object.ownership_token != object.ownership.as_str() {
                errors.push(format!("ownership token drifted for {}", object.object_ref));
            }
            if object.freshness_token != object.freshness.as_str() {
                errors.push(format!("freshness token drifted for {}", object.object_ref));
            }
        }
        for item in &self.checklist_items {
            if item.completion_semantics_token != item.completion_semantics.as_str() {
                errors.push(format!(
                    "completion semantics token drifted for {}",
                    item.item_id
                ));
            }
            if !item.local_completion_is_non_mutating() {
                errors.push(format!(
                    "checklist item {} would mutate provider state on local completion",
                    item.item_id
                ));
            }
            let linked = self
                .linked_objects
                .iter()
                .find(|object| object.object_ref == item.linked_object_ref);
            match linked {
                Some(object)
                    if matches!(
                        object.ownership,
                        LinkedObjectOwnershipClass::ProviderOwned
                            | LinkedObjectOwnershipClass::Mixed
                    ) =>
                {
                    if item.completion_semantics
                        == ChecklistCompletionSemantics::ProviderActionTrackedSeparately
                    {
                        match &item.provider_mutation_command {
                            Some(command) if command.is_reviewable() => {}
                            _ => errors.push(format!(
                                "provider-owned checklist item {} lacks a reviewed command",
                                item.item_id
                            )),
                        }
                    }
                }
                Some(_) => {}
                None => errors.push(format!(
                    "checklist item {} links to an object outside the bundle",
                    item.item_id
                )),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(TruthValidationError { errors })
        }
    }
}

/// Support/export packet preserving bundle meaning outside the live shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowupSupportExportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable export identifier.
    pub export_id: String,
    /// Source bundle identifier.
    pub source_bundle_id: String,
    /// Exported scope.
    pub exported_scope: String,
    /// Exported owner.
    pub exported_owner_ref: String,
    /// Exported filter state.
    pub exported_filter_state: FilterState,
    /// Exported freshness token.
    pub exported_freshness_token: String,
    /// Exported linked object references.
    pub linked_object_refs: Vec<String>,
    /// Exported ownership tokens.
    pub ownership_tokens: Vec<String>,
    /// Exported checklist semantics tokens.
    pub checklist_semantics_tokens: Vec<String>,
    /// Exported provider commands.
    pub provider_mutation_commands: Vec<ProviderMutationCommand>,
    /// True when export/copy/support packet meaning matches the source bundle.
    pub meaning_preserved: bool,
}

impl FollowupSupportExportPacket {
    /// Builds an export packet from a source bundle.
    pub fn from_bundle(export_id: impl Into<String>, bundle: &FollowupBundle) -> Self {
        Self {
            record_kind: SUPPORT_EXPORT_PACKET_RECORD_KIND.to_owned(),
            export_id: export_id.into(),
            source_bundle_id: bundle.bundle_id.clone(),
            exported_scope: bundle.scope.clone(),
            exported_owner_ref: bundle.owner_ref.clone(),
            exported_filter_state: bundle.filter_state.clone(),
            exported_freshness_token: bundle.freshness.as_str().to_owned(),
            linked_object_refs: bundle
                .linked_objects
                .iter()
                .map(|object| object.object_ref.clone())
                .collect(),
            ownership_tokens: bundle
                .linked_objects
                .iter()
                .map(|object| object.ownership.as_str().to_owned())
                .collect(),
            checklist_semantics_tokens: bundle
                .checklist_items
                .iter()
                .map(|item| item.completion_semantics.as_str().to_owned())
                .collect(),
            provider_mutation_commands: bundle
                .checklist_items
                .iter()
                .filter_map(|item| item.provider_mutation_command.clone())
                .collect(),
            meaning_preserved: true,
        }
    }

    /// Returns true when scope, freshness, ownership, filters, and commands survived export.
    pub fn preserves_bundle_meaning(&self, bundle: &FollowupBundle) -> bool {
        self.record_kind == SUPPORT_EXPORT_PACKET_RECORD_KIND
            && self.source_bundle_id == bundle.bundle_id
            && self.exported_scope == bundle.scope
            && self.exported_owner_ref == bundle.owner_ref
            && self.exported_filter_state == bundle.filter_state
            && self.exported_freshness_token == bundle.freshness.as_str()
            && self.linked_object_refs
                == bundle
                    .linked_objects
                    .iter()
                    .map(|object| object.object_ref.clone())
                    .collect::<Vec<_>>()
            && self.ownership_tokens
                == bundle
                    .linked_objects
                    .iter()
                    .map(|object| object.ownership.as_str().to_owned())
                    .collect::<Vec<_>>()
            && self.meaning_preserved
            && self
                .provider_mutation_commands
                .iter()
                .all(ProviderMutationCommand::is_reviewable)
    }
}

/// Whole stable truth packet consumed by support/export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardQueueFollowupTruthPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Boundary schema reference.
    pub schema_ref: String,
    /// Support artifact reference.
    pub artifact_ref: String,
    /// Help/support documentation reference.
    pub doc_ref: String,
    /// Dashboard cards included in the packet.
    pub dashboard_cards: Vec<DashboardFreshnessCard>,
    /// Queue ordering and narrowing truth.
    pub queue_order: QueueOrderTruth,
    /// Provider-linked follow-up bundle.
    pub followup_bundle: FollowupBundle,
    /// Support/export parity packet.
    pub support_export: FollowupSupportExportPacket,
}

impl DashboardQueueFollowupTruthPacket {
    /// Validates the stable-line dashboard, queue, follow-up, and export invariants.
    pub fn validate(&self) -> Result<(), TruthValidationError> {
        let mut errors = Vec::new();
        if self.record_kind != TRUTH_PACKET_RECORD_KIND {
            errors.push("packet record_kind is not canonical".to_owned());
        }
        if self.schema_version != TRUTH_PACKET_SCHEMA_VERSION {
            errors.push("packet schema_version is not supported".to_owned());
        }
        if self.schema_ref != FOLLOWUP_BUNDLE_SCHEMA_REF {
            errors.push("packet schema_ref is not canonical".to_owned());
        }
        if self.artifact_ref != FOLLOWUP_BUNDLE_ARTIFACT_REF {
            errors.push("packet artifact_ref is not canonical".to_owned());
        }
        if self.doc_ref != FOLLOWUP_BUNDLE_DOC_REF {
            errors.push("packet doc_ref is not canonical".to_owned());
        }
        for card in &self.dashboard_cards {
            if card.record_kind != DASHBOARD_CARD_RECORD_KIND {
                errors.push(format!(
                    "card {} has non-canonical record_kind",
                    card.card_id
                ));
            }
            if card.source_freshness_token != card.source_freshness.as_str() {
                errors.push(format!("card {} freshness token drifted", card.card_id));
            }
            if card.effective_state_token != card.effective_state.as_str() {
                errors.push(format!("card {} effective token drifted", card.card_id));
            }
            if card.declared_green
                && card.source_freshness.downgrades_green()
                && !card.visibly_downgraded
            {
                errors.push(format!("card {} kept a stale green claim", card.card_id));
            }
            if card.declared_green
                && card.source_freshness.downgrades_green()
                && card.effective_state == EffectiveCardState::Healthy
            {
                errors.push(format!(
                    "card {} stayed healthy after downgrade",
                    card.card_id
                ));
            }
            if !is_canonical_object_ref(&card.open_evidence_ref) {
                errors.push(format!(
                    "card {} evidence ref is not canonical",
                    card.card_id
                ));
            }
        }
        if !self.queue_order.is_explainable() {
            errors.push("queue order is not fully explainable".to_owned());
        }
        if !self.queue_order.discloses_provider_and_policy_blockers() {
            errors.push("queue does not disclose provider and policy blockers".to_owned());
        }
        if let Err(err) = self.followup_bundle.validate() {
            errors.extend(err.errors);
        }
        if !self
            .support_export
            .preserves_bundle_meaning(&self.followup_bundle)
        {
            errors.push("support export does not preserve follow-up bundle meaning".to_owned());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(TruthValidationError { errors })
        }
    }
}

/// Validation error for stable dashboard/queue/follow-up truth packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthValidationError {
    /// Validation failures.
    pub errors: Vec<String>,
}

impl fmt::Display for TruthValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "dashboard/queue/follow-up truth validation failed: {}",
            self.errors.join("; ")
        )
    }
}

impl Error for TruthValidationError {}

/// Returns true when a reference points to a canonical Aureline object.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let Some(rest) = reference.strip_prefix("aureline://") else {
        return false;
    };
    let Some((class, id)) = rest.split_once('/') else {
        return false;
    };
    !class.is_empty()
        && !id.is_empty()
        && !matches!(
            class,
            "home" | "dashboard" | "overview" | "index" | "start" | "root"
        )
}

fn dashboard_card(
    card_id: &str,
    title: &str,
    scope: &str,
    source_ref: &str,
    source_freshness: SourceFreshnessClass,
    declared_green: bool,
    open_evidence_ref: &str,
    explanation: &str,
) -> DashboardFreshnessCard {
    DashboardFreshnessCard::from_input(DashboardCardInput {
        card_id: card_id.to_owned(),
        title: title.to_owned(),
        scope: scope.to_owned(),
        source_ref: source_ref.to_owned(),
        source_freshness,
        declared_green,
        open_evidence_ref: open_evidence_ref.to_owned(),
        explanation: explanation.to_owned(),
    })
}

fn queue_row(
    row_id: &str,
    order_rank: u32,
    order_reason: QueueOrderReasonClass,
    order_explanation: &str,
    grouping_reason: &str,
    open_ref: &str,
    provider_blocker: Option<&str>,
    policy_blocker: Option<&str>,
) -> QueueRowTruth {
    QueueRowTruth {
        row_id: row_id.to_owned(),
        order_rank,
        order_reason,
        order_reason_token: order_reason.as_str().to_owned(),
        order_explanation: order_explanation.to_owned(),
        grouping_reason: grouping_reason.to_owned(),
        open_ref: open_ref.to_owned(),
        provider_blocker: provider_blocker.map(str::to_owned),
        policy_blocker: policy_blocker.map(str::to_owned),
    }
}

fn narrowing(
    reason: QueueNarrowingReasonClass,
    hidden_count: u32,
    explanation: &str,
) -> QueueNarrowingTruth {
    QueueNarrowingTruth {
        reason,
        reason_token: reason.as_str().to_owned(),
        hidden_count,
        explanation: explanation.to_owned(),
    }
}

fn linked_object(
    object_ref: &str,
    ownership: LinkedObjectOwnershipClass,
    freshness: SourceFreshnessClass,
) -> LinkedObjectRef {
    LinkedObjectRef {
        object_ref: object_ref.to_owned(),
        ownership,
        ownership_token: ownership.as_str().to_owned(),
        freshness,
        freshness_token: freshness.as_str().to_owned(),
    }
}

fn provider_command(
    command_id: &str,
    target_ref: &str,
    actor_ref: &str,
    mode: ProviderMutationMode,
) -> ProviderMutationCommand {
    ProviderMutationCommand {
        command_id: command_id.to_owned(),
        target_ref: target_ref.to_owned(),
        actor_ref: actor_ref.to_owned(),
        mode,
        mode_token: mode.as_str().to_owned(),
        reviewed_separately: true,
    }
}

fn checklist_item(
    item_id: &str,
    title: &str,
    linked_object_ref: &str,
    completion_semantics: ChecklistCompletionSemantics,
    provider_mutation_command: Option<ProviderMutationCommand>,
) -> ChecklistItem {
    ChecklistItem {
        item_id: item_id.to_owned(),
        title: title.to_owned(),
        linked_object_ref: linked_object_ref.to_owned(),
        completion_semantics,
        completion_semantics_token: completion_semantics.as_str().to_owned(),
        local_completion_mutates_provider: false,
        provider_mutation_command,
    }
}

/// Builds the canonical dashboard/queue/follow-up truth packet fixture.
pub fn canonical_dashboard_queue_followup_truth_packet() -> DashboardQueueFollowupTruthPacket {
    let filter_state = FilterState {
        scope_label: "workspace: payments-api / current review workset".to_owned(),
        active_filters: vec![
            "state:open".to_owned(),
            "owner:on-call".to_owned(),
            "freshness:include-degraded".to_owned(),
        ],
        hidden_count: 7,
        narrowing_reason_tokens: vec![
            QueueNarrowingReasonClass::PolicyFilter.as_str().to_owned(),
            QueueNarrowingReasonClass::ProviderScope.as_str().to_owned(),
        ],
    };

    let dashboard_cards = vec![
        dashboard_card(
            "card:review_health",
            "Review health",
            "workspace: payments-api / current review workset",
            "provider:code-host:pull-requests",
            SourceFreshnessClass::Fresh,
            true,
            "aureline://dashboard_card/review_health",
            "Code-host review data refreshed inside the stable review window.",
        ),
        dashboard_card(
            "card:ci_summary",
            "CI summary",
            "workspace: payments-api / current review workset",
            "provider:ci:imported-run-884",
            SourceFreshnessClass::ImportedSnapshot,
            true,
            "aureline://dashboard_card/ci_summary",
            "CI data is imported from a prior run and cannot back a current green claim.",
        ),
        dashboard_card(
            "card:support_context",
            "Support context",
            "workspace: payments-api / current review workset",
            "support:cached:last-good",
            SourceFreshnessClass::Cached,
            true,
            "aureline://dashboard_card/support_context",
            "Support context is a cached last-good snapshot and is visibly downgraded.",
        ),
        dashboard_card(
            "card:provider_scope",
            "Provider scope",
            "workspace: payments-api / current review workset",
            "provider:issue-tracker:limited-grant",
            SourceFreshnessClass::Blocked,
            true,
            "aureline://dashboard_card/provider_scope",
            "Provider grant does not include transitions, so the card is blocked.",
        ),
    ];

    let queue_order = QueueOrderTruth {
        record_kind: QUEUE_TRUTH_RECORD_KIND.to_owned(),
        queue_id: "queue:stable_followup:provider_linked".to_owned(),
        queue_label: "Provider-linked follow-up queue".to_owned(),
        filter_state: filter_state.clone(),
        rows: vec![
            queue_row(
                "followup:publish_review_comments",
                1,
                QueueOrderReasonClass::ProviderPriority,
                "Provider marked this review response as blocking merge.",
                "Grouped with provider-owned review objects awaiting publication.",
                "aureline://followup_item/publish_review_comments",
                Some("provider grant allows comments but not status transitions"),
                None,
            ),
            queue_row(
                "followup:verify_imported_ci",
                2,
                QueueOrderReasonClass::Severity,
                "Imported CI summary is stale and affects a release-health card.",
                "Grouped under dashboard freshness downgrades.",
                "aureline://followup_item/verify_imported_ci",
                None,
                Some("policy hides two prod-only check runs from this account"),
            ),
            queue_row(
                "followup:handoff_support_packet",
                3,
                QueueOrderReasonClass::DueTime,
                "Support handoff is due before the on-call rotation changes.",
                "Grouped with export-ready handoff bundles.",
                "aureline://followup_item/handoff_support_packet",
                None,
                None,
            ),
        ],
        hidden_scope: vec![
            narrowing(
                QueueNarrowingReasonClass::PolicyFilter,
                2,
                "Two prod-only check runs are hidden by policy scope.",
            ),
            narrowing(
                QueueNarrowingReasonClass::ProviderScope,
                5,
                "Five issue-tracker rows are outside the connected installation grant.",
            ),
        ],
    };

    let linked_review = "aureline://provider_object/code_host_pr_1842";
    let linked_ci = "aureline://provider_object/ci_run_884";
    let linked_support = "aureline://support_packet/followup_handoff_1842";
    let followup_bundle = FollowupBundle {
        record_kind: FOLLOWUP_BUNDLE_RECORD_KIND.to_owned(),
        bundle_id: "followup_bundle:provider_linked_review_1842".to_owned(),
        owner_ref: "aureline://actor/on_call_driver".to_owned(),
        scope: "workspace: payments-api / PR-1842 / current review workset".to_owned(),
        filter_state: filter_state.clone(),
        freshness: SourceFreshnessClass::ImportedSnapshot,
        freshness_token: SourceFreshnessClass::ImportedSnapshot.as_str().to_owned(),
        linked_objects: vec![
            linked_object(
                linked_review,
                LinkedObjectOwnershipClass::ProviderOwned,
                SourceFreshnessClass::Fresh,
            ),
            linked_object(
                linked_ci,
                LinkedObjectOwnershipClass::ProviderOwned,
                SourceFreshnessClass::ImportedSnapshot,
            ),
            linked_object(
                linked_support,
                LinkedObjectOwnershipClass::LocalOwned,
                SourceFreshnessClass::Cached,
            ),
        ],
        checklist_items: vec![
            checklist_item(
                "checklist:review_comment_draft",
                "Review draft comments",
                linked_review,
                ChecklistCompletionSemantics::ProviderActionTrackedSeparately,
                Some(provider_command(
                    "provider_command:publish_review_comments",
                    linked_review,
                    "aureline://actor/on_call_driver",
                    ProviderMutationMode::PublishLater,
                )),
            ),
            checklist_item(
                "checklist:verify_ci_snapshot",
                "Verify imported CI snapshot",
                linked_ci,
                ChecklistCompletionSemantics::LocalEvidenceOnly,
                None,
            ),
            checklist_item(
                "checklist:send_support_handoff",
                "Send support handoff",
                linked_support,
                ChecklistCompletionSemantics::LocalReminderOnly,
                Some(provider_command(
                    "provider_command:support_handoff_only",
                    linked_support,
                    "aureline://actor/on_call_driver",
                    ProviderMutationMode::HandoffOnly,
                )),
            ),
        ],
    };
    let support_export = FollowupSupportExportPacket::from_bundle(
        "support_export:followup_bundle:provider_linked_review_1842",
        &followup_bundle,
    );

    DashboardQueueFollowupTruthPacket {
        record_kind: TRUTH_PACKET_RECORD_KIND.to_owned(),
        schema_version: TRUTH_PACKET_SCHEMA_VERSION,
        packet_id: "truth_packet:dashboard_queue_followup:provider_linked_review_1842".to_owned(),
        schema_ref: FOLLOWUP_BUNDLE_SCHEMA_REF.to_owned(),
        artifact_ref: FOLLOWUP_BUNDLE_ARTIFACT_REF.to_owned(),
        doc_ref: FOLLOWUP_BUNDLE_DOC_REF.to_owned(),
        dashboard_cards,
        queue_order,
        followup_bundle,
        support_export,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_imported_cached_and_blocked_green_cards_downgrade() {
        let packet = canonical_dashboard_queue_followup_truth_packet();
        let downgraded: Vec<_> = packet
            .dashboard_cards
            .iter()
            .filter(|card| card.declared_green && card.source_freshness.downgrades_green())
            .collect();
        assert_eq!(downgraded.len(), 3);
        assert!(downgraded.iter().all(|card| card.visibly_downgraded));
        assert!(downgraded
            .iter()
            .all(|card| card.effective_state != EffectiveCardState::Healthy));
    }

    #[test]
    fn queue_rows_explain_order_grouping_scope_and_blockers() {
        let packet = canonical_dashboard_queue_followup_truth_packet();
        assert!(packet.queue_order.is_explainable());
        assert!(packet.queue_order.discloses_provider_and_policy_blockers());
        assert_eq!(packet.queue_order.filter_state.hidden_count, 7);
    }

    #[test]
    fn checklist_completion_never_mutates_provider_objects() {
        let packet = canonical_dashboard_queue_followup_truth_packet();
        packet.followup_bundle.validate().expect("bundle validates");
        assert!(packet
            .followup_bundle
            .checklist_items
            .iter()
            .all(ChecklistItem::local_completion_is_non_mutating));
    }

    #[test]
    fn export_preserves_bundle_scope_freshness_ownership_filters_and_commands() {
        let packet = canonical_dashboard_queue_followup_truth_packet();
        assert!(packet
            .support_export
            .preserves_bundle_meaning(&packet.followup_bundle));
        assert!(packet
            .support_export
            .provider_mutation_commands
            .iter()
            .all(ProviderMutationCommand::is_reviewable));
    }

    #[test]
    fn full_packet_validates() {
        canonical_dashboard_queue_followup_truth_packet()
            .validate()
            .expect("canonical truth packet validates");
    }
}
