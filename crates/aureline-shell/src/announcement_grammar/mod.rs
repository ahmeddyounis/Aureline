//! Live-announcement grammar for the claimed M5 dynamic events.
//!
//! Where the frozen dynamic-surface matrix
//! ([`crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix`])
//! governs *that* a live-announcement class exists and *which* controlled
//! vocabularies it carries, this module materializes the concrete grammar: one
//! [`M5AnnouncementGrammarClass`] per governed dynamic-event class — mode/state
//! changes, blockers, progress milestones, selection/context changes,
//! success-with-recovery, and degraded/stale truth — that binds a stable message
//! id and placeholder-driven template to its live-region channel
//! (polite/assertive/silent), its required runtime fields, its coalescing budget
//! and suppression rules, and the durable fallback surface a screen-reader user can
//! reopen.
//!
//! The grammar is the single source M5 dynamic events narrate through, so
//! shell, editor, terminal, notebook, data, review, notifications, and help
//! surfaces announce concise meaning with one governed grammar rather than
//! per-surface improvised prose. Narrated state changes use a stable
//! [`M5AnnouncementMessageTemplate`] with `{placeholder}` insertion rather than
//! concatenated fragments; repeated polls, streaming updates, and background
//! refreshes are bounded by a [`M5CoalescingBudget`] and explicit
//! [`M5AnnouncementSuppressionRule`]s so the live region never floods; and every
//! high-value announcement points back to a durable
//! [`M5DurableFallbackRef`] (activity row, run header, patch-review header, banner
//! detail, selection summary, or notification entry) the user can revisit instead
//! of relying on ephemeral narration alone. When a class's bridge or proof state
//! goes stale the claimed grammar auto-narrows rather than implying silent
//! screen-reader completeness.
//!
//! The controlled state vocabularies — announcement politeness, coalescing
//! strategy, and fallback durability — are reused verbatim from the frozen matrix
//! rather than minting parallel tokens. Only the grammar-shaped vocabularies this
//! lane adds (dynamic-event class, durable fallback surface, placeholder value
//! kind, and suppression rule) are minted here and frozen in a self-describing
//! [`M5AnnouncementGrammarVocabularySet`]. Raw provider payloads, credentials,
//! secret material, screenshots, and untranslated free-text prose stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/a11y/m5-announcement-grammar.schema.json`](../../../../schemas/a11y/m5-announcement-grammar.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-announcement-grammar.md`](../../../../docs/a11y/m5-announcement-grammar.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-announcements/`](../../../../fixtures/a11y/m5-announcements/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_announcement_grammar_catalog,
    seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed,
    seeded_m5_announcement_grammar_catalog_proof_stale_narrowed,
    M5_ANNOUNCEMENT_GRAMMAR_CATALOG_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The frozen matrix owns the canonical announcement-channel, coalescing, and
// fallback vocabularies; reuse its tokens rather than minting parallel synonyms.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use matrix::{
    A11yAnnouncementPoliteness, A11yCoalescingStrategy, A11yFallbackDurability,
    M5DynamicSurfaceA11yConsumerSurface, M5DynamicSurfaceA11yDowngradeTrigger,
    M5DynamicSurfaceA11yProofFreshness, M5DynamicSurfaceA11yQualificationClass,
    M5DynamicSurfaceA11yReleasePosture, M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5AnnouncementGrammarCatalogPacket`].
pub const M5_ANNOUNCEMENT_GRAMMAR_RECORD_KIND: &str = "m5_live_announcement_grammar_catalog";

/// Schema version for M5 live-announcement grammar catalogs.
pub const M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_REF: &str =
    "schemas/a11y/m5-announcement-grammar.schema.json";

/// Repo-relative path of the M5 announcement-grammar contract doc.
pub const M5_ANNOUNCEMENT_GRAMMAR_DOC_REF: &str = "docs/a11y/m5-announcement-grammar.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that
/// governs this lane's controlled vocabularies and qualification classes.
pub const M5_ANNOUNCEMENT_GRAMMAR_MATRIX_REF: &str =
    "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the frozen screen-reader announcement / live-region
/// contract.
pub const M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF: &str =
    "docs/accessibility/screen_reader_and_live_region_contract.md";

/// Repo-relative path of the frozen dense-collection announcement contract.
pub const M5_ANNOUNCEMENT_GRAMMAR_COLLECTION_CONTRACT_REF: &str =
    "docs/accessibility/collection_announcement_contract.md";

/// Repo-relative path of the frozen locale-fallback / message-id copy contract
/// that anchors stable message ids and placeholder insertion.
pub const M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF: &str =
    "docs/accessibility/locale_fallback_and_copy_representation_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ANNOUNCEMENT_GRAMMAR_FIXTURE_DIR: &str = "fixtures/a11y/m5-announcements";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ANNOUNCEMENT_GRAMMAR_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-live-announcement-proof/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_ANNOUNCEMENT_GRAMMAR_SUMMARY_REF: &str =
    "artifacts/a11y/m5-live-announcement-proof/live-announcement-proof.md";

/// Stable prefix every governed announcement message id carries.
pub const M5_ANNOUNCEMENT_MESSAGE_ID_PREFIX: &str = "announcement.";

/// One governed class of M5 dynamic event the live-announcement grammar covers.
///
/// These are exactly the dynamic-event classes the grammar must narrate: a
/// mode/state change, a blocker, a progress milestone, a selection/context change,
/// a success-with-recovery, and a degraded/stale-truth disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AnnouncementEventClass {
    /// A mode or major state change on a surface.
    ModeOrStateChange,
    /// A blocker that prevents an action, requiring assertive interruption.
    BlockerRaised,
    /// A progress milestone in a long-running or streaming operation.
    ProgressMilestone,
    /// A selection or context change on a dense or interactive surface.
    SelectionOrContextChange,
    /// A success that completed with a recovery or compensating action.
    SuccessWithRecovery,
    /// A degraded or stale-truth disclosure that auto-narrows a claim.
    DegradedOrStaleTruth,
}

impl M5AnnouncementEventClass {
    /// Every governed event class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ModeOrStateChange,
        Self::BlockerRaised,
        Self::ProgressMilestone,
        Self::SelectionOrContextChange,
        Self::SuccessWithRecovery,
        Self::DegradedOrStaleTruth,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeOrStateChange => "mode_or_state_change",
            Self::BlockerRaised => "blocker_raised",
            Self::ProgressMilestone => "progress_milestone",
            Self::SelectionOrContextChange => "selection_or_context_change",
            Self::SuccessWithRecovery => "success_with_recovery",
            Self::DegradedOrStaleTruth => "degraded_or_stale_truth",
        }
    }

    /// The live-region channel this event class is required to speak on.
    ///
    /// Only a blocker may interrupt with an assertive live region; every other
    /// class stays polite (or silent) so the live region is never spammed.
    pub const fn required_channel_is_assertive(self) -> bool {
        matches!(self, Self::BlockerRaised)
    }
}

/// Durable fallback surface a narrated announcement points back to.
///
/// Every high-value announcement must have a durable UI counterpart the user can
/// reopen rather than relying on ephemeral live-region narration alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableFallbackSurface {
    /// An activity-center row.
    ActivityRow,
    /// A run header / run-status surface.
    RunHeader,
    /// A patch / diff review header.
    PatchReviewHeader,
    /// A banner detail surface.
    BannerDetail,
    /// A selection-summary surface.
    SelectionSummary,
    /// A durable notification-center entry.
    NotificationCenterEntry,
    /// A status-bar / mode-strip detail surface.
    StatusDetail,
}

impl M5DurableFallbackSurface {
    /// Every durable fallback surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ActivityRow,
        Self::RunHeader,
        Self::PatchReviewHeader,
        Self::BannerDetail,
        Self::SelectionSummary,
        Self::NotificationCenterEntry,
        Self::StatusDetail,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityRow => "activity_row",
            Self::RunHeader => "run_header",
            Self::PatchReviewHeader => "patch_review_header",
            Self::BannerDetail => "banner_detail",
            Self::SelectionSummary => "selection_summary",
            Self::NotificationCenterEntry => "notification_center_entry",
            Self::StatusDetail => "status_detail",
        }
    }
}

/// Controlled value kind a message-template placeholder inserts.
///
/// A narrated state change inserts typed values into a stable template rather than
/// concatenating raw prose fragments, so the grammar — not the call site — owns the
/// sentence shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AnnouncementValueKind {
    /// A mode name (e.g. the entered mode).
    ModeName,
    /// A state name (e.g. the new state).
    StateName,
    /// A count or "n of m" progress value.
    Count,
    /// A duration label.
    DurationLabel,
    /// A severity label.
    SeverityLabel,
    /// A surface name.
    SurfaceName,
    /// An item / row identity.
    ItemIdentity,
    /// A recovery / compensating-action label.
    RecoveryLabel,
    /// A freshness / staleness label.
    FreshnessLabel,
}

impl M5AnnouncementValueKind {
    /// Every value kind, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ModeName,
        Self::StateName,
        Self::Count,
        Self::DurationLabel,
        Self::SeverityLabel,
        Self::SurfaceName,
        Self::ItemIdentity,
        Self::RecoveryLabel,
        Self::FreshnessLabel,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeName => "mode_name",
            Self::StateName => "state_name",
            Self::Count => "count",
            Self::DurationLabel => "duration_label",
            Self::SeverityLabel => "severity_label",
            Self::SurfaceName => "surface_name",
            Self::ItemIdentity => "item_identity",
            Self::RecoveryLabel => "recovery_label",
            Self::FreshnessLabel => "freshness_label",
        }
    }
}

/// Controlled suppression rule that keeps a live region from flooding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AnnouncementSuppressionRule {
    /// Drop an announcement whose meaning is unchanged from the last.
    SuppressUnchangedMeaning,
    /// Drop repaint-only ticks that carry no new meaning.
    SuppressRepaintOnlyTicks,
    /// Drop duplicates within the coalescing window.
    SuppressDuplicateWithinWindow,
    /// Drop background-refresh narration while the surface is unfocused.
    SuppressBackgroundRefreshWhenUnfocused,
    /// Drop low-value intermediate progress midpoints.
    SuppressLowValueProgressMidpoints,
    /// Drop the live announcement while its durable surface is already visible.
    SuppressWhenDurableSurfaceVisible,
}

impl M5AnnouncementSuppressionRule {
    /// Every suppression rule, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SuppressUnchangedMeaning,
        Self::SuppressRepaintOnlyTicks,
        Self::SuppressDuplicateWithinWindow,
        Self::SuppressBackgroundRefreshWhenUnfocused,
        Self::SuppressLowValueProgressMidpoints,
        Self::SuppressWhenDurableSurfaceVisible,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppressUnchangedMeaning => "suppress_unchanged_meaning",
            Self::SuppressRepaintOnlyTicks => "suppress_repaint_only_ticks",
            Self::SuppressDuplicateWithinWindow => "suppress_duplicate_within_window",
            Self::SuppressBackgroundRefreshWhenUnfocused => {
                "suppress_background_refresh_when_unfocused"
            }
            Self::SuppressLowValueProgressMidpoints => "suppress_low_value_progress_midpoints",
            Self::SuppressWhenDurableSurfaceVisible => "suppress_when_durable_surface_visible",
        }
    }
}

/// One declared placeholder within a message template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementPlaceholder {
    /// Placeholder name, matching the `{name}` token in the template.
    pub name: String,
    /// Controlled value kind inserted at this placeholder.
    pub value_kind: M5AnnouncementValueKind,
    /// True when a runtime value for this placeholder is required.
    pub required: bool,
}

/// Stable message id and placeholder-driven template for one announcement class.
///
/// The single `template` string with `{placeholder}` tokens is the only narration
/// source — there is no fragment-concatenation path. The validator enforces that
/// every `{...}` token resolves to a declared placeholder and vice versa.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementMessageTemplate {
    /// Stable message id; carries the [`M5_ANNOUNCEMENT_MESSAGE_ID_PREFIX`].
    pub message_id: String,
    /// Template string with `{placeholder}` insertion points.
    pub template: String,
    /// Declared placeholders, one per distinct `{name}` token.
    pub placeholders: Vec<M5AnnouncementPlaceholder>,
}

impl M5AnnouncementMessageTemplate {
    /// Returns the declared placeholder names.
    fn declared_names(&self) -> BTreeSet<&str> {
        self.placeholders.iter().map(|p| p.name.as_str()).collect()
    }

    /// Returns the names of declared placeholders that require a runtime value.
    fn required_names(&self) -> BTreeSet<&str> {
        self.placeholders
            .iter()
            .filter(|p| p.required)
            .map(|p| p.name.as_str())
            .collect()
    }
}

/// Numeric coalescing budget that bounds how often a live region speaks.
///
/// The budget pairs the matrix-owned [`A11yCoalescingStrategy`] with hard caps so
/// repeated polls, streaming updates, and background refreshes cannot flood the
/// live region with duplicate or low-value narration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoalescingBudget {
    /// Coalescing strategy (matrix-owned); never `none` for a governed class.
    pub strategy: A11yCoalescingStrategy,
    /// Maximum announcements admitted within the window.
    pub max_announcements_per_window: u32,
    /// Coalescing window in seconds.
    pub window_seconds: u32,
    /// Minimum spacing between announcements, in milliseconds.
    pub min_interval_ms: u32,
    /// True when announcements with unchanged meaning are suppressed.
    pub suppress_unchanged_meaning: bool,
}

/// Reference to the durable surface a narrated announcement points back to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DurableFallbackRef {
    /// Durable fallback surface kind.
    pub surface: M5DurableFallbackSurface,
    /// Stable id of the durable surface the user can reopen.
    pub surface_ref: String,
    /// True when the user can reopen the durable surface for this announcement.
    pub reopenable: bool,
}

/// One governed live-announcement grammar class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementGrammarClass {
    /// Stable class id, unique within the catalog.
    pub class_id: String,
    /// Governed dynamic-event class.
    pub event_class: M5AnnouncementEventClass,
    /// Human-readable class label.
    pub label: String,
    /// Owner role accountable for keeping this class's grammar current.
    pub owner_role: String,
    /// Qualification class earned by this grammar class.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Live-region channel (politeness) the class announces on.
    pub channel: A11yAnnouncementPoliteness,
    /// Stable message id and placeholder-driven template.
    pub message_template: M5AnnouncementMessageTemplate,
    /// Required runtime fields supplied at narration time.
    pub required_fields: Vec<String>,
    /// Coalescing budget that bounds repeated narration.
    pub coalescing_budget: M5CoalescingBudget,
    /// Suppression rules that keep the live region from flooding.
    pub suppression_rules: Vec<M5AnnouncementSuppressionRule>,
    /// Delivery / fallback durability (matrix-owned).
    pub fallback_durability: A11yFallbackDurability,
    /// Durable fallback surface the announcement points back to.
    pub durable_fallback: M5DurableFallbackRef,
    /// Downgrade triggers that can narrow this class below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this class current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this class.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that narrate through this class.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

/// Self-describing controlled-vocabulary set for the grammar-shaped tokens this
/// lane mints (the shared channel/coalescing/fallback tokens live in the matrix).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementGrammarVocabularySet {
    /// Dynamic-event-class tokens.
    pub event_classes: Vec<String>,
    /// Durable-fallback-surface tokens.
    pub durable_fallback_surfaces: Vec<String>,
    /// Placeholder value-kind tokens.
    pub value_kinds: Vec<String>,
    /// Suppression-rule tokens.
    pub suppression_rules: Vec<String>,
}

impl M5AnnouncementGrammarVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            event_classes: M5AnnouncementEventClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            durable_fallback_surfaces: M5DurableFallbackSurface::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            value_kinds: M5AnnouncementValueKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            suppression_rules: M5AnnouncementSuppressionRule::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block for the announcement lane.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementGrammarConformanceReview {
    /// Dynamic events announce through one governed grammar, not per-surface prose.
    pub one_governed_grammar_not_per_surface_prose: bool,
    /// Narrated state changes use stable message ids with placeholder insertion.
    pub stable_message_ids_with_placeholders_not_concatenated_fragments: bool,
    /// Polite / assertive channel rules are enforced per event class.
    pub polite_assertive_channel_rules_enforced: bool,
    /// Coalescing budgets bound repeated narration.
    pub coalescing_budgets_bound_repeated_narration: bool,
    /// Repeated polls and refreshes do not flood the live region.
    pub repeated_polls_and_refreshes_do_not_flood_live_region: bool,
    /// Every high-value announcement has a durable fallback surface.
    pub every_high_value_announcement_has_durable_fallback: bool,
    /// Narrated state can be reopened on its durable surface.
    pub narrated_state_points_back_to_durable_surface: bool,
    /// Announcements convey meaning, not repaint noise.
    pub announcements_convey_meaning_not_repaint_noise: bool,
    /// Claimed classes auto-narrow when bridge or proof state goes stale.
    pub claimed_classes_auto_narrow_when_bridge_or_proof_stale: bool,
    /// Downgrade narrows the claim rather than hiding the class.
    pub downgrade_narrows_instead_of_hides: bool,
}

/// Consumer projection block: who narrates through the grammar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementGrammarConsumerProjection {
    /// Shell narrates mode/state and blocker events through the grammar.
    pub shell_consumes_grammar: bool,
    /// Editor narrates through the grammar.
    pub editor_consumes_grammar: bool,
    /// Terminal narrates streaming progress through the grammar.
    pub terminal_consumes_grammar: bool,
    /// Notebook narrates cell run state through the grammar.
    pub notebook_consumes_grammar: bool,
    /// Data grid narrates selection/context changes through the grammar.
    pub data_grid_consumes_grammar: bool,
    /// Review narrates patch-review state through the grammar.
    pub review_consumes_grammar: bool,
    /// Notifications route durable announcements through the grammar.
    pub notifications_consume_grammar: bool,
    /// Help documents the announcement grammar.
    pub help_documents_grammar: bool,
    /// Support export reuses the grammar.
    pub support_export_reuses_grammar: bool,
    /// Assistive-tech conformance packets reuse the grammar.
    pub at_conformance_packets_reuse_grammar: bool,
}

/// Constructor input for [`M5AnnouncementGrammarCatalogPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AnnouncementGrammarCatalogPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Grammar classes.
    pub classes: Vec<M5AnnouncementGrammarClass>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Grammar-shaped controlled-vocabulary set.
    pub grammar_vocabulary_set: M5AnnouncementGrammarVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5AnnouncementGrammarConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AnnouncementGrammarConsumerProjection,
    /// Proof freshness block (reused from the matrix lane).
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture (reused from the matrix lane).
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 live-announcement grammar catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementGrammarCatalogPacket {
    /// Record kind; must equal [`M5_ANNOUNCEMENT_GRAMMAR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Grammar classes.
    pub classes: Vec<M5AnnouncementGrammarClass>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Grammar-shaped controlled-vocabulary set.
    pub grammar_vocabulary_set: M5AnnouncementGrammarVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5AnnouncementGrammarConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AnnouncementGrammarConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AnnouncementGrammarCatalogPacket {
    /// Builds an announcement-grammar catalog packet from seed input.
    pub fn new(input: M5AnnouncementGrammarCatalogPacketInput) -> Self {
        Self {
            record_kind: M5_ANNOUNCEMENT_GRAMMAR_RECORD_KIND.to_owned(),
            schema_version: M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalog_label: input.catalog_label,
            classes: input.classes,
            shared_vocabulary_set: input.shared_vocabulary_set,
            grammar_vocabulary_set: input.grammar_vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the announcement-grammar catalog invariants.
    pub fn validate(&self) -> Vec<M5AnnouncementGrammarViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ANNOUNCEMENT_GRAMMAR_RECORD_KIND {
            violations.push(M5AnnouncementGrammarViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_VERSION {
            violations.push(M5AnnouncementGrammarViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AnnouncementGrammarViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_classes(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 announcement grammar catalog serializes"),
        ) {
            violations.push(M5AnnouncementGrammarViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 announcement grammar catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable = self
            .classes
            .iter()
            .filter(|c| c.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Live-Announcement Grammar\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Classes: {} ({} stable)\n",
            self.classes.len(),
            stable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Announcement classes\n\n");
        for class in &self.classes {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}`\n",
                class.class_id,
                class.event_class.as_str(),
                class.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", class.owner_role));
            out.push_str(&format!("  - Channel: {}\n", class.channel.as_str()));
            out.push_str(&format!(
                "  - Message id: `{}`\n",
                class.message_template.message_id
            ));
            out.push_str(&format!(
                "  - Template: `{}`\n",
                class.message_template.template
            ));
            out.push_str(&format!(
                "  - Coalescing: {} (max {} / {}s, min interval {}ms)\n",
                class.coalescing_budget.strategy.as_str(),
                class.coalescing_budget.max_announcements_per_window,
                class.coalescing_budget.window_seconds,
                class.coalescing_budget.min_interval_ms
            ));
            out.push_str(&format!(
                "  - Durable fallback: {} (`{}`)\n",
                class.durable_fallback.surface.as_str(),
                class.durable_fallback.surface_ref
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in grammar-catalog export.
#[derive(Debug)]
pub enum M5AnnouncementGrammarArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AnnouncementGrammarViolation>),
}

impl fmt::Display for M5AnnouncementGrammarArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 announcement grammar export parse failed: {error}"
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
                    "m5 announcement grammar export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AnnouncementGrammarArtifactError {}

/// Validation failures emitted by [`M5AnnouncementGrammarCatalogPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AnnouncementGrammarViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A governed dynamic-event class has no grammar class.
    RequiredEventClassMissing,
    /// Two classes share a class id.
    DuplicateClassId,
    /// Two classes cover the same dynamic-event class.
    DuplicateEventClass,
    /// A class row is incomplete.
    ClassIncomplete,
    /// The message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// The template's `{...}` tokens and declared placeholders do not agree.
    MessageTemplatePlaceholderMismatch,
    /// The required fields do not match the required placeholders.
    RequiredFieldPlaceholderMismatch,
    /// The class's live-region channel violates the per-event-class rule.
    ChannelRuleViolated,
    /// The coalescing strategy is `none`, so the live region would not coalesce.
    CoalescingStrategyMissing,
    /// The coalescing budget caps are not positive.
    CoalescingBudgetInvalid,
    /// The class declares no suppression rule.
    SuppressionRulesMissing,
    /// The class has no reopenable durable fallback surface.
    DurableFallbackMissing,
    /// A class claiming Stable is missing required proof packet refs.
    StableClassMissingProof,
    /// A class has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5AnnouncementGrammarViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredEventClassMissing => "required_event_class_missing",
            Self::DuplicateClassId => "duplicate_class_id",
            Self::DuplicateEventClass => "duplicate_event_class",
            Self::ClassIncomplete => "class_incomplete",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::MessageTemplatePlaceholderMismatch => "message_template_placeholder_mismatch",
            Self::RequiredFieldPlaceholderMismatch => "required_field_placeholder_mismatch",
            Self::ChannelRuleViolated => "channel_rule_violated",
            Self::CoalescingStrategyMissing => "coalescing_strategy_missing",
            Self::CoalescingBudgetInvalid => "coalescing_budget_invalid",
            Self::SuppressionRulesMissing => "suppression_rules_missing",
            Self::DurableFallbackMissing => "durable_fallback_missing",
            Self::StableClassMissingProof => "stable_class_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable grammar-catalog export.
pub fn current_stable_m5_announcement_grammar_export(
) -> Result<M5AnnouncementGrammarCatalogPacket, M5AnnouncementGrammarArtifactError> {
    let packet: M5AnnouncementGrammarCatalogPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-live-announcement-proof/support_export.json"
    )))
    .map_err(M5AnnouncementGrammarArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AnnouncementGrammarArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_REF,
        M5_ANNOUNCEMENT_GRAMMAR_DOC_REF,
        M5_ANNOUNCEMENT_GRAMMAR_MATRIX_REF,
        M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
        M5_ANNOUNCEMENT_GRAMMAR_COLLECTION_CONTRACT_REF,
        M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AnnouncementGrammarViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    if !packet.shared_vocabulary_set.matches_canonical()
        || !packet.grammar_vocabulary_set.matches_canonical()
    {
        violations.push(M5AnnouncementGrammarViolation::VocabularySetDrift);
    }
}

fn validate_classes(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let present: BTreeSet<M5AnnouncementEventClass> =
        packet.classes.iter().map(|c| c.event_class).collect();
    for required in M5AnnouncementEventClass::ALL {
        if !present.contains(&required) {
            violations.push(M5AnnouncementGrammarViolation::RequiredEventClassMissing);
            break;
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_event_classes: BTreeSet<M5AnnouncementEventClass> = BTreeSet::new();
    for class in &packet.classes {
        if !seen_ids.insert(class.class_id.as_str()) {
            violations.push(M5AnnouncementGrammarViolation::DuplicateClassId);
        }
        if !seen_event_classes.insert(class.event_class) {
            violations.push(M5AnnouncementGrammarViolation::DuplicateEventClass);
        }

        if class.class_id.trim().is_empty()
            || class.label.trim().is_empty()
            || class.owner_role.trim().is_empty()
            || class.message_template.message_id.trim().is_empty()
            || class.message_template.template.trim().is_empty()
            || class.source_contract_refs.is_empty()
        {
            violations.push(M5AnnouncementGrammarViolation::ClassIncomplete);
        }

        validate_class_message_template(class, violations);
        validate_class_channel(class, violations);
        validate_class_coalescing(class, violations);
        validate_class_durable_fallback(class, violations);

        if class.suppression_rules.is_empty() {
            violations.push(M5AnnouncementGrammarViolation::SuppressionRulesMissing);
        }
        if class.qualification.is_stable() && class.required_proof_packet_refs.is_empty() {
            violations.push(M5AnnouncementGrammarViolation::StableClassMissingProof);
        }
        if class.downgrade_triggers.is_empty() {
            violations.push(M5AnnouncementGrammarViolation::DowngradeTriggersMissing);
        }
        if class.consumer_surfaces.is_empty() {
            violations.push(M5AnnouncementGrammarViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_class_message_template(
    class: &M5AnnouncementGrammarClass,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let template = &class.message_template;

    if !template
        .message_id
        .starts_with(M5_ANNOUNCEMENT_MESSAGE_ID_PREFIX)
    {
        violations.push(M5AnnouncementGrammarViolation::MessageIdPrefixMissing);
    }

    // The single template string with `{placeholder}` tokens is the only narration
    // path; the parsed tokens must agree exactly with the declared placeholders so
    // there is no orphan declaration and no undeclared (concatenated) insertion.
    match parse_template_placeholders(&template.template) {
        Ok(parsed) => {
            let declared = template.declared_names();
            let declared_owned: BTreeSet<String> =
                declared.iter().map(|s| (*s).to_owned()).collect();
            if parsed != declared_owned {
                violations.push(M5AnnouncementGrammarViolation::MessageTemplatePlaceholderMismatch);
            }
            // Placeholder names must be unique across the declaration.
            if declared.len() != template.placeholders.len() {
                violations.push(M5AnnouncementGrammarViolation::MessageTemplatePlaceholderMismatch);
            }
        }
        Err(()) => {
            violations.push(M5AnnouncementGrammarViolation::MessageTemplatePlaceholderMismatch);
        }
    }

    // The declared required fields must be exactly the required placeholders, so
    // the runtime contract and the template stay in lockstep.
    let required_fields: BTreeSet<&str> =
        class.required_fields.iter().map(String::as_str).collect();
    if required_fields != template.required_names() {
        violations.push(M5AnnouncementGrammarViolation::RequiredFieldPlaceholderMismatch);
    }
}

fn validate_class_channel(
    class: &M5AnnouncementGrammarClass,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    // Only a blocker may interrupt with an assertive live region; every other class
    // stays polite or silent so the live region is never spammed by urgency.
    let channel_ok = if class.event_class.required_channel_is_assertive() {
        class.channel == A11yAnnouncementPoliteness::Assertive
    } else {
        class.channel != A11yAnnouncementPoliteness::Assertive
    };
    if !channel_ok {
        violations.push(M5AnnouncementGrammarViolation::ChannelRuleViolated);
    }
}

fn validate_class_coalescing(
    class: &M5AnnouncementGrammarClass,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let budget = &class.coalescing_budget;
    // A governed class must coalesce; `none` would let a poll loop spam.
    if budget.strategy == A11yCoalescingStrategy::None {
        violations.push(M5AnnouncementGrammarViolation::CoalescingStrategyMissing);
    }
    // The numeric budget must cap how often the live region speaks.
    if budget.max_announcements_per_window == 0 || budget.window_seconds == 0 {
        violations.push(M5AnnouncementGrammarViolation::CoalescingBudgetInvalid);
    }
}

fn validate_class_durable_fallback(
    class: &M5AnnouncementGrammarClass,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    // Every announcement must have a durable UI counterpart the user can reopen,
    // never narration alone.
    let fallback = &class.durable_fallback;
    if fallback.surface_ref.trim().is_empty() || !fallback.reopenable {
        violations.push(M5AnnouncementGrammarViolation::DurableFallbackMissing);
    }
}

fn validate_conformance_review(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.one_governed_grammar_not_per_surface_prose,
        review.stable_message_ids_with_placeholders_not_concatenated_fragments,
        review.polite_assertive_channel_rules_enforced,
        review.coalescing_budgets_bound_repeated_narration,
        review.repeated_polls_and_refreshes_do_not_flood_live_region,
        review.every_high_value_announcement_has_durable_fallback,
        review.narrated_state_points_back_to_durable_surface,
        review.announcements_convey_meaning_not_repaint_noise,
        review.claimed_classes_auto_narrow_when_bridge_or_proof_stale,
        review.downgrade_narrows_instead_of_hides,
    ] {
        if !ok {
            violations.push(M5AnnouncementGrammarViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_grammar,
        projection.editor_consumes_grammar,
        projection.terminal_consumes_grammar,
        projection.notebook_consumes_grammar,
        projection.data_grid_consumes_grammar,
        projection.review_consumes_grammar,
        projection.notifications_consume_grammar,
        projection.help_documents_grammar,
        projection.support_export_reuses_grammar,
        projection.at_conformance_packets_reuse_grammar,
    ] {
        if !ok {
            violations.push(M5AnnouncementGrammarViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AnnouncementGrammarViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AnnouncementGrammarCatalogPacket,
    violations: &mut Vec<M5AnnouncementGrammarViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5AnnouncementGrammarViolation::ReleasePostureIncomplete);
    }
}

/// Parses the distinct `{placeholder}` token names from a template string.
///
/// Returns `Err(())` when a brace is unbalanced or a token name is empty, so a
/// malformed template is rejected rather than silently dropping a placeholder.
fn parse_template_placeholders(template: &str) -> Result<BTreeSet<String>, ()> {
    let mut names = BTreeSet::new();
    let mut chars = template.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                let mut name = String::new();
                let mut closed = false;
                for inner in chars.by_ref() {
                    if inner == '}' {
                        closed = true;
                        break;
                    }
                    if inner == '{' {
                        // A nested open brace is malformed.
                        return Err(());
                    }
                    name.push(inner);
                }
                if !closed || name.trim().is_empty() {
                    return Err(());
                }
                names.insert(name);
            }
            '}' => return Err(()),
            _ => {}
        }
    }
    Ok(names)
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
