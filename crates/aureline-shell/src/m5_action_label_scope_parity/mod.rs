//! Verb-first action-label and count/scope-language parity for approval and
//! mutation surfaces.
//!
//! This module materializes the canonical, export-safe inventory of the action
//! labels and count/scope phrases the shell renders when an action approves,
//! reruns, exports, installs, applies, deletes, or publishes objects. It is the
//! concrete catalog that consumes the count/scope/freshness microcopy grammar and
//! the safety-critical string catalog: where those lock *which* wording is governed
//! and the controlled scope terms, this catalog locks the *actual* action labels and
//! disclosures so a primary action can never hide its scope, side effect, or
//! selection class behind a vague verb such as `Continue`, `Accept`, or `Submit`.
//!
//! Every [`ActionLabel`] is verb-first: a stable, locale-neutral label id, a
//! controlled [`ActionVerb`], the [`ActionObject`] class it narrows, the
//! [`ScopeClass`] it targets, the [`MutationClass`] risk family, the
//! [`ReviewState`] it carries, and a reference template built from `{verb}`,
//! `{count:<name>}`, `{scope:<scope_id>}`, `{object_one}`, and `{object_many}`
//! placeholders. The label resolves the verb, scope phrase, and object noun from the
//! registries — it never inlines a scope word as a literal — so the same controlled
//! scope phrase set resolves identically across batch action bars, review sheets,
//! toast/activity rows, CLI/help summaries, export/report headings, and confirmation
//! dialogs.
//!
//! Count/scope honesty is a first-class object too. A [`ScopeDisclosure`] states how
//! many objects each scope holds — how many are `selected`, `visible`, `loaded`, or
//! `all matching`, and how many are excluded because they are `hidden by policy` or
//! `outside the current workset` — using the same controlled phrase set the action
//! labels use, so a batch bar, a CLI summary, and an export heading cannot compress
//! partial truth into an ambiguous word.
//!
//! Machine-facing identity stays locale-neutral while human prose localizes safely
//! around it. Label ids, verb ids, scope ids, object ids, count-variable names, and
//! the `{...}` placeholders are lowercase ascii (`[a-z0-9_.]`); only the canonical
//! verb labels, scope phrases, object nouns, and reference templates carry human
//! prose. A localized overlay rewrites the prose but never the ids or placeholders,
//! so a translation can never fork the meaning of a scope or hide the object class an
//! action mutates.
//!
//! The boundary schema is
//! [`schemas/content/m5-action-label-scope.schema.json`](../../../../schemas/content/m5-action-label-scope.schema.json).
//! The contract doc is
//! [`docs/content/m5/m5_action_label_scope_parity.md`](../../../../docs/content/m5/m5_action_label_scope_parity.md).
//! The protected fixture directory is
//! [`fixtures/content/m5-action-label-scope/`](../../../../fixtures/content/m5-action-label-scope/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_action_label_scope_catalog, seeded_action_label_scope_catalog_localized,
    seeded_action_label_scope_catalog_offline_mirror, ACTION_LABEL_SCOPE_CATALOG_ID,
};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ActionLabelScopeCatalog`].
pub const ACTION_LABEL_SCOPE_CATALOG_RECORD_KIND: &str = "m5_action_label_scope_catalog";

/// Schema version for action-label/scope catalog records.
pub const ACTION_LABEL_SCOPE_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct surfaces a shared scope phrase must appear on.
pub const SHARED_SCOPE_MIN_REUSE_SURFACES: usize = 3;

/// Minimum number of labels that must carry product-UI, docs, and support-export
/// parity together so runtime, docs, and exports cannot drift.
pub const DOCS_EXPORT_PARITY_MIN_LABELS: usize = 3;

/// Repo-relative path of the boundary schema.
pub const ACTION_LABEL_SCOPE_CATALOG_SCHEMA_REF: &str =
    "schemas/content/m5-action-label-scope.schema.json";

/// Repo-relative path of the catalog contract doc.
pub const ACTION_LABEL_SCOPE_CATALOG_DOC_REF: &str =
    "docs/content/m5/m5_action_label_scope_parity.md";

/// Repo-relative path of the frozen UI copy contract (action labels, error copy).
pub const CATALOG_UI_COPY_CONTRACT_REF: &str = "docs/copy/ui_copy_contract.md";

/// Repo-relative path of the frozen naming and state-label contract.
pub const CATALOG_NAMING_LABEL_CONTRACT_REF: &str = "docs/copy/naming_and_state_label_contract.md";

/// Repo-relative path of the frozen count/scope/freshness grammar contract.
pub const CATALOG_COUNT_SCOPE_GRAMMAR_REF: &str = "docs/copy/count_scope_freshness_grammar.md";

/// Repo-relative path of the controlled count/scope term set this catalog consumes.
pub const CATALOG_COUNT_SCOPE_TERM_SET_REF: &str = "artifacts/copy/count_scope_term_set.yaml";

/// Repo-relative path of the upstream safety-critical string catalog schema.
pub const CATALOG_SAFETY_CRITICAL_SCHEMA_REF: &str =
    "schemas/content/m5-safety-critical-strings.schema.json";

/// Repo-relative path of the upstream safety-critical string catalog doc.
pub const CATALOG_SAFETY_CRITICAL_DOC_REF: &str =
    "docs/content/m5/m5_safety_critical_string_catalog.md";

/// Repo-relative path of the protected fixture directory.
pub const ACTION_LABEL_SCOPE_CATALOG_FIXTURE_DIR: &str = "fixtures/content/m5-action-label-scope";

/// Repo-relative path of the checked support-export artifact.
pub const ACTION_LABEL_SCOPE_CATALOG_ARTIFACT_REF: &str =
    "artifacts/content/m5-action-label-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ACTION_LABEL_SCOPE_CATALOG_SUMMARY_REF: &str =
    "artifacts/content/m5-action-label-proof/m5_action_label_scope_parity.md";

/// The controlled scope vocabulary. Each variant maps to a controlled phrase from
/// the count/scope/freshness microcopy grammar, plus a single-object scope for
/// single-target actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    /// Items explicitly admitted to the current selection.
    Selected,
    /// Items currently rendered in the active page or viewport.
    Visible,
    /// Items materialized in the client or pinned by the provider cursor.
    Loaded,
    /// The full population matching the active query, filter, or basis.
    AllMatching,
    /// Items withheld by trust, admin policy, redaction, or authorization.
    HiddenByPolicy,
    /// Items outside the active workset or slice.
    OutsideCurrentWorkset,
    /// Exactly one named object.
    SingleObject,
}

impl ScopeClass {
    /// Every scope class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Selected,
        Self::Visible,
        Self::Loaded,
        Self::AllMatching,
        Self::HiddenByPolicy,
        Self::OutsideCurrentWorkset,
        Self::SingleObject,
    ];

    /// Locale-neutral token recorded in the catalog. Matches the controlled
    /// count/scope term ids the grammar contract owns.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::Visible => "visible",
            Self::Loaded => "loaded",
            Self::AllMatching => "all_matching",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::OutsideCurrentWorkset => "outside_current_workset",
            Self::SingleObject => "single_object",
        }
    }

    /// Whether an action may target this scope directly. Exclusion scopes are
    /// disclosed, never acted on as a primary target.
    pub const fn actionable(self) -> bool {
        !matches!(self, Self::HiddenByPolicy | Self::OutsideCurrentWorkset)
    }

    /// Whether this scope is an exclusion (disclosed as withheld or out-of-scope).
    pub const fn is_exclusion(self) -> bool {
        matches!(self, Self::HiddenByPolicy | Self::OutsideCurrentWorkset)
    }

    /// Whether a count must accompany this scope phrase. The full-population and
    /// single-object scopes carry their honesty in the phrase, so a count is not
    /// mandatory; every counted multi-object scope requires one.
    pub const fn requires_count(self) -> bool {
        !matches!(self, Self::AllMatching | Self::SingleObject)
    }

    /// Whether an action label must name this scope explicitly (vs. relying on the
    /// surrounding sheet). Single-object scope is conveyed by the object noun.
    pub const fn requires_explicit_scope_word(self) -> bool {
        self.actionable() && !matches!(self, Self::SingleObject)
    }
}

/// The surface an action label or scope disclosure renders on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionSurface {
    /// The batch action bar over a multi-select collection.
    BatchActionBar,
    /// A review sheet that lists the objects an action affects.
    ReviewSheet,
    /// A toast or activity-feed row describing a completed action.
    ToastActivityRow,
    /// A CLI / help summary line.
    CliHelpSummary,
    /// An export or report heading.
    ExportReportHeading,
    /// A destructive/approval confirmation dialog.
    ConfirmationDialog,
}

impl ActionSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BatchActionBar,
        Self::ReviewSheet,
        Self::ToastActivityRow,
        Self::CliHelpSummary,
        Self::ExportReportHeading,
        Self::ConfirmationDialog,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BatchActionBar => "batch_action_bar",
            Self::ReviewSheet => "review_sheet",
            Self::ToastActivityRow => "toast_activity_row",
            Self::CliHelpSummary => "cli_help_summary",
            Self::ExportReportHeading => "export_report_heading",
            Self::ConfirmationDialog => "confirmation_dialog",
        }
    }
}

/// The risk family an action belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationClass {
    /// Approving or rejecting a proposal or review.
    Approval,
    /// Deleting, discarding, or resetting state.
    Destructive,
    /// Applying or rerunning across many objects.
    BatchMutation,
    /// Publishing or sharing outward.
    Publish,
    /// Installing or enabling a capability.
    Install,
    /// Exporting or producing an outward report.
    Export,
}

impl MutationClass {
    /// Every mutation class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Approval,
        Self::Destructive,
        Self::BatchMutation,
        Self::Publish,
        Self::Install,
        Self::Export,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approval => "approval",
            Self::Destructive => "destructive",
            Self::BatchMutation => "batch_mutation",
            Self::Publish => "publish",
            Self::Install => "install",
            Self::Export => "export",
        }
    }

    /// Whether actions in this class carry a per-object review obligation, so a
    /// label must declare its [`ReviewState`] as something other than
    /// [`ReviewState::NoReviewNeeded`].
    pub const fn carries_review_obligation(self) -> bool {
        matches!(self, Self::Approval | Self::BatchMutation)
    }

    /// Whether actions in this class must disclose their side effect explicitly.
    pub const fn must_disclose_side_effect(self) -> bool {
        matches!(self, Self::Destructive | Self::Publish | Self::Install)
    }
}

/// The review state an action carries about the objects it affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    /// Each affected object has been individually reviewed.
    Reviewed,
    /// A batch over objects that were not individually reviewed.
    UnreviewedBatch,
    /// Some affected objects are reviewed and some are not.
    PartiallyReviewed,
    /// The action must be explicitly reviewed before it runs.
    ReviewRequired,
    /// The action carries no per-object review obligation.
    NoReviewNeeded,
}

impl ReviewState {
    /// Every review state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Reviewed,
        Self::UnreviewedBatch,
        Self::PartiallyReviewed,
        Self::ReviewRequired,
        Self::NoReviewNeeded,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reviewed => "reviewed",
            Self::UnreviewedBatch => "unreviewed_batch",
            Self::PartiallyReviewed => "partially_reviewed",
            Self::ReviewRequired => "review_required",
            Self::NoReviewNeeded => "no_review_needed",
        }
    }
}

/// How reversible a verb's effect is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversibilityClass {
    /// The effect can be reversed cleanly.
    Reversible,
    /// The effect can be undone only within a bounded window.
    UndoableWindow,
    /// The effect cannot be undone.
    Irreversible,
}

impl ReversibilityClass {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "reversible",
            Self::UndoableWindow => "undoable_window",
            Self::Irreversible => "irreversible",
        }
    }
}

/// The status of a counted scope in a disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountStatus {
    /// Proven for the declared scope and freshness.
    Exact,
    /// Approximate; a provider cap, sampling, or estimate.
    Approximate,
    /// Some requested data is missing or unloaded.
    Partial,
    /// A previous known-good value without fresh confirmation.
    Cached,
    /// A previous value after its freshness floor was lost.
    Stale,
    /// Rows or counts can still change as a stream continues.
    Streaming,
    /// Background preparation is still in progress.
    Warming,
    /// The count is not known.
    Unknown,
}

impl CountStatus {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Partial => "partial",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Streaming => "streaming",
            Self::Warming => "warming",
            Self::Unknown => "unknown",
        }
    }

    /// Default-locale phrase rendered for this status.
    pub const fn canonical_phrase(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approx.",
            Self::Partial => "partial",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Streaming => "streaming",
            Self::Warming => "warming",
            Self::Unknown => "unknown",
        }
    }
}

/// A consumer surface that must reuse a catalog label or disclosure verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Product UI (buttons, bars, sheets, dialogs).
    ProductUi,
    /// CLI / help text.
    CliHelp,
    /// Documentation.
    Docs,
    /// Support / export packet.
    SupportExport,
    /// Screen-reader / narrated surface.
    ScreenReader,
    /// Activity feed / history rows.
    ActivityFeed,
}

impl ConsumerSurface {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::CliHelp => "cli_help",
            Self::Docs => "docs",
            Self::SupportExport => "support_export",
            Self::ScreenReader => "screen_reader",
            Self::ActivityFeed => "activity_feed",
        }
    }
}

/// A controlled scope phrase: one [`ScopeClass`], one canonical phrase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDefinition {
    /// Stable, locale-neutral scope id (equal to the scope-class token).
    pub scope_id: String,
    /// The scope class this definition realizes.
    pub scope_class: ScopeClass,
    /// Canonical (default-locale) phrase, e.g. `selected`, `all matching`.
    pub canonical_phrase: String,
    /// Whether an action may target this scope directly.
    pub actionable: bool,
    /// Whether this scope is an exclusion (disclosed, never acted on).
    pub is_exclusion: bool,
    /// Whether a count must accompany this scope phrase.
    pub requires_count: bool,
}

/// A controlled action verb.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionVerb {
    /// Stable, locale-neutral verb id, e.g. `approve`.
    pub verb_id: String,
    /// Canonical (default-locale) imperative label, e.g. `Approve`.
    pub canonical_label: String,
    /// How reversible the verb's effect is.
    pub reversibility: ReversibilityClass,
    /// The mutation class this verb most often realizes.
    pub default_mutation_class: MutationClass,
}

/// A controlled object noun an action narrows to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionObject {
    /// Stable, locale-neutral object id, e.g. `change`.
    pub object_id: String,
    /// Canonical (default-locale) singular noun, e.g. `change`.
    pub singular_label: String,
    /// Canonical (default-locale) plural noun, e.g. `changes`.
    pub plural_label: String,
}

/// One verb-first action label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionLabel {
    /// Stable, locale-neutral label id, e.g. `action.review.approve_selected_changes`.
    pub label_id: String,
    /// The controlled verb id this label uses.
    pub verb_ref: String,
    /// The controlled object id this label narrows.
    pub object_ref: String,
    /// The scope id this label targets.
    pub scope_ref: String,
    /// The risk family of the action.
    pub mutation_class: MutationClass,
    /// The review state the action carries.
    pub review_state: ReviewState,
    /// The surface the label renders on.
    pub surface: ActionSurface,
    /// The count-variable name, when the label is a counted batch.
    pub count_var: Option<String>,
    /// Verb-first reference template built from `{verb}`, `{count:<name>}`,
    /// `{scope:<scope_id>}`, `{object_one}`, and `{object_many}`.
    pub reference_label: String,
    /// True when the surrounding sheet already makes the scope unambiguous, so the
    /// visible label may omit the scope word. The screen-reader label still names it.
    pub scope_unambiguous_in_sheet: bool,
    /// True when the label discloses the action's side effect.
    pub discloses_side_effect: bool,
    /// Narrated label template; always names verb, scope, and object and is never
    /// truncated.
    pub screen_reader_label: String,
    /// Consumer surfaces that must reuse this label verbatim.
    pub consumer_surfaces: Vec<ConsumerSurface>,
}

/// One count/scope disclosure phrase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDisclosure {
    /// Stable, locale-neutral disclosure id.
    pub disclosure_id: String,
    /// The surface the disclosure renders on.
    pub surface: ActionSurface,
    /// The controlled object id this disclosure counts.
    pub object_ref: String,
    /// Optional verb id, when the disclosure restates a completed action.
    pub verb_ref: Option<String>,
    /// The primary scope the count is about.
    pub primary_scope_ref: String,
    /// Other scopes disclosed (typically exclusions).
    pub disclosed_scope_refs: Vec<String>,
    /// The status of the primary count.
    pub count_status: CountStatus,
    /// The named count variables this disclosure uses.
    pub count_vars: Vec<String>,
    /// Reference template built from `{count:<name>}`, `{scope:<scope_id>}`,
    /// `{object_one}`, `{object_many}`, `{count_status}`, and optional `{verb}`.
    pub reference_phrase: String,
    /// Consumer surfaces that must reuse this disclosure verbatim.
    pub consumer_surfaces: Vec<ConsumerSurface>,
}

/// Catalog-level wording-parity review block. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityReview {
    /// Every action label is verb-first.
    pub labels_are_verb_first: bool,
    /// No primary label hides scope behind a vague verb.
    pub no_ambiguous_primary_labels: bool,
    /// Every batch label declares its scope or relies on an unambiguous sheet.
    pub scope_declared_or_unambiguous_in_sheet: bool,
    /// Counted batch actions declare their count.
    pub batch_actions_declare_count: bool,
    /// Approval and batch mutations declare a review state.
    pub review_state_declared_on_approval_and_batch: bool,
    /// Every label narrows the object class.
    pub object_class_narrowed: bool,
    /// One controlled scope phrase set is reused across surfaces.
    pub one_controlled_scope_phrase_set: bool,
    /// Destructive, publish, and install actions disclose their side effect.
    pub side_effects_disclosed: bool,
    /// Screen-reader labels always carry the full verb-plus-scope phrase.
    pub screen_reader_labels_complete: bool,
    /// Docs and support exports reuse the runtime labels.
    pub docs_and_export_reuse_runtime_labels: bool,
}

/// Consumer projection block. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    /// Product UI resolves labels through the catalog.
    pub product_ui_resolves_through_catalog: bool,
    /// CLI / help text uses the catalog action labels.
    pub cli_help_uses_action_labels: bool,
    /// Docs render the catalog action labels.
    pub docs_render_action_labels: bool,
    /// Support export uses the catalog action labels.
    pub support_export_uses_action_labels: bool,
    /// Screen-reader announcements reuse the catalog labels.
    pub screen_reader_reuses_labels: bool,
    /// Activity feed rows reuse the catalog labels.
    pub activity_feed_reuses_labels: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the catalog claim.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every label.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every label.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`ActionLabelScopeCatalog::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionLabelScopeCatalogInput {
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default templates (e.g. `en`).
    pub reference_locale: String,
    /// Banned ambiguous default tokens (e.g. `continue`, `accept`, `submit`).
    pub banned_ambiguous_tokens: Vec<String>,
    /// Controlled scope phrases.
    pub scopes: Vec<ScopeDefinition>,
    /// Controlled verbs.
    pub verbs: Vec<ActionVerb>,
    /// Controlled object nouns.
    pub objects: Vec<ActionObject>,
    /// Verb-first action labels.
    pub labels: Vec<ActionLabel>,
    /// Count/scope disclosures.
    pub disclosures: Vec<ScopeDisclosure>,
    /// Shared scope phrase ids that must span multiple surfaces.
    pub shared_scope_phrase_ids: Vec<String>,
    /// Parity review block.
    pub parity_review: ParityReview,
    /// Consumer projection block.
    pub consumer_projection: ConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CatalogProofFreshness,
    /// Release posture.
    pub release_posture: CatalogReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe action-label and count/scope-language parity catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionLabelScopeCatalog {
    /// Record kind; must equal [`ACTION_LABEL_SCOPE_CATALOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ACTION_LABEL_SCOPE_CATALOG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default templates.
    pub reference_locale: String,
    /// Closed scope-class inventory (locale-neutral tokens).
    pub scope_inventory: Vec<String>,
    /// Closed surface inventory (locale-neutral tokens).
    pub surface_inventory: Vec<String>,
    /// Closed mutation-class inventory (locale-neutral tokens).
    pub mutation_class_inventory: Vec<String>,
    /// Closed review-state inventory (locale-neutral tokens).
    pub review_state_inventory: Vec<String>,
    /// Banned ambiguous default tokens.
    pub banned_ambiguous_tokens: Vec<String>,
    /// Controlled scope phrases.
    pub scopes: Vec<ScopeDefinition>,
    /// Controlled verbs.
    pub verbs: Vec<ActionVerb>,
    /// Controlled object nouns.
    pub objects: Vec<ActionObject>,
    /// Verb-first action labels.
    pub labels: Vec<ActionLabel>,
    /// Count/scope disclosures.
    pub disclosures: Vec<ScopeDisclosure>,
    /// Shared scope phrase ids that must span multiple surfaces.
    pub shared_scope_phrase_ids: Vec<String>,
    /// Parity review block.
    pub parity_review: ParityReview,
    /// Consumer projection block.
    pub consumer_projection: ConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CatalogProofFreshness,
    /// Release posture.
    pub release_posture: CatalogReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ActionLabelScopeCatalog {
    /// Builds a catalog packet from lane input, filling the closed inventories from
    /// the canonical enum token lists.
    pub fn new(input: ActionLabelScopeCatalogInput) -> Self {
        Self {
            record_kind: ACTION_LABEL_SCOPE_CATALOG_RECORD_KIND.to_owned(),
            schema_version: ACTION_LABEL_SCOPE_CATALOG_SCHEMA_VERSION,
            catalog_id: input.catalog_id,
            catalog_label: input.catalog_label,
            reference_locale: input.reference_locale,
            scope_inventory: token_list(&ScopeClass::ALL, ScopeClass::as_str),
            surface_inventory: token_list(&ActionSurface::ALL, ActionSurface::as_str),
            mutation_class_inventory: token_list(&MutationClass::ALL, MutationClass::as_str),
            review_state_inventory: token_list(&ReviewState::ALL, ReviewState::as_str),
            banned_ambiguous_tokens: input.banned_ambiguous_tokens,
            scopes: input.scopes,
            verbs: input.verbs,
            objects: input.objects,
            labels: input.labels,
            disclosures: input.disclosures,
            shared_scope_phrase_ids: input.shared_scope_phrase_ids,
            parity_review: input.parity_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// The canonical set of vague tokens a primary action must never use.
    pub const fn required_banned_tokens() -> [&'static str; 11] {
        [
            "continue", "accept", "submit", "ok", "confirm", "proceed", "done", "go", "yes",
            "next", "finish",
        ]
    }

    /// Resolves a scope definition by id.
    pub fn scope(&self, scope_id: &str) -> Option<&ScopeDefinition> {
        self.scopes.iter().find(|s| s.scope_id == scope_id)
    }

    /// Resolves a verb by id.
    pub fn verb(&self, verb_id: &str) -> Option<&ActionVerb> {
        self.verbs.iter().find(|v| v.verb_id == verb_id)
    }

    /// Resolves an object by id.
    pub fn object(&self, object_id: &str) -> Option<&ActionObject> {
        self.objects.iter().find(|o| o.object_id == object_id)
    }

    /// Resolves an action label by id.
    pub fn label(&self, label_id: &str) -> Option<&ActionLabel> {
        self.labels.iter().find(|l| l.label_id == label_id)
    }

    /// Renders the default-locale reference text for an action label, resolving the
    /// verb, scope phrase, and object noun and keeping each `{count:<name>}` as a
    /// named slot. Returns `None` if the label id is unknown.
    ///
    /// This is the catalog's consumer entry point: a surface never inlines a scope
    /// word, it asks the catalog to resolve the controlled phrase.
    pub fn render_label(&self, label_id: &str) -> Option<String> {
        let label = self.label(label_id)?;
        Some(self.render_template(
            &label.reference_label,
            Some(label.verb_ref.as_str()),
            label.object_ref.as_str(),
            None,
        ))
    }

    /// Renders the default-locale reference text for a disclosure.
    pub fn render_disclosure(&self, disclosure_id: &str) -> Option<String> {
        let disclosure = self
            .disclosures
            .iter()
            .find(|d| d.disclosure_id == disclosure_id)?;
        Some(self.render_template(
            &disclosure.reference_phrase,
            disclosure.verb_ref.as_deref(),
            disclosure.object_ref.as_str(),
            Some(disclosure.count_status),
        ))
    }

    fn render_template(
        &self,
        template: &str,
        verb_ref: Option<&str>,
        object_ref: &str,
        count_status: Option<CountStatus>,
    ) -> String {
        let mut out = String::new();
        for segment in parse_template(template) {
            match segment {
                TemplateSegment::Text(text) => out.push_str(&text),
                TemplateSegment::Verb => match verb_ref.and_then(|id| self.verb(id)) {
                    Some(verb) => out.push_str(&verb.canonical_label),
                    None => out.push_str("{verb}"),
                },
                TemplateSegment::Count(name) => out.push_str(&format!("{{{name}}}")),
                TemplateSegment::Scope(scope_id) => match self.scope(&scope_id) {
                    Some(scope) => out.push_str(&scope.canonical_phrase),
                    None => out.push_str(&format!("{{scope:{scope_id}}}")),
                },
                TemplateSegment::ObjectOne => match self.object(object_ref) {
                    Some(object) => out.push_str(&object.singular_label),
                    None => out.push_str("{object_one}"),
                },
                TemplateSegment::ObjectMany => match self.object(object_ref) {
                    Some(object) => out.push_str(&object.plural_label),
                    None => out.push_str("{object_many}"),
                },
                TemplateSegment::CountStatus => match count_status {
                    Some(status) => out.push_str(status.canonical_phrase()),
                    None => out.push_str("{count_status}"),
                },
                TemplateSegment::Unknown(raw) => out.push_str(&raw),
            }
        }
        out
    }

    /// Maps each scope id to the distinct surfaces that use it across labels and
    /// disclosures. This is the reuse proof for the single controlled phrase set.
    pub fn cross_surface_reuse(&self) -> BTreeMap<String, BTreeSet<&'static str>> {
        let mut reuse: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
        for label in &self.labels {
            reuse
                .entry(label.scope_ref.clone())
                .or_default()
                .insert(label.surface.as_str());
        }
        for disclosure in &self.disclosures {
            for scope_id in std::iter::once(&disclosure.primary_scope_ref)
                .chain(disclosure.disclosed_scope_refs.iter())
            {
                reuse
                    .entry(scope_id.clone())
                    .or_default()
                    .insert(disclosure.surface.as_str());
            }
        }
        reuse
    }

    /// Validates every catalog invariant.
    pub fn validate(&self) -> Vec<ActionLabelScopeCatalogViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ACTION_LABEL_SCOPE_CATALOG_RECORD_KIND {
            violations.push(ActionLabelScopeCatalogViolation::WrongRecordKind);
        }
        if self.schema_version != ACTION_LABEL_SCOPE_CATALOG_SCHEMA_VERSION {
            violations.push(ActionLabelScopeCatalogViolation::WrongSchemaVersion);
        }
        if self.catalog_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.reference_locale.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ActionLabelScopeCatalogViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_inventories(self, &mut violations);
        validate_banned_tokens(self, &mut violations);
        validate_scopes(self, &mut violations);
        validate_verbs(self, &mut violations);
        validate_objects(self, &mut violations);
        validate_labels(self, &mut violations);
        validate_disclosures(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_shared_reuse(self, &mut violations);
        validate_docs_export_parity(self, &mut violations);
        validate_parity_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("action-label/scope catalog serializes"),
        ) {
            violations.push(ActionLabelScopeCatalogViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("action-label/scope catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Action-Label and Count/Scope-Language Parity Catalog\n\n");
        out.push_str(&format!("- Catalog: `{}`\n", self.catalog_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Reference locale: `{}`\n",
            self.reference_locale
        ));
        out.push_str(&format!(
            "- Verbs: {} | Scopes: {} | Objects: {} | Labels: {} | Disclosures: {}\n",
            self.verbs.len(),
            self.scopes.len(),
            self.objects.len(),
            self.labels.len(),
            self.disclosures.len()
        ));
        out.push_str(&format!(
            "- Banned ambiguous tokens: {}\n",
            self.banned_ambiguous_tokens.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Action labels\n\n");
        for label in &self.labels {
            out.push_str(&format!(
                "- `{}` [{} / {} / {}] on `{}`\n",
                label.label_id,
                label.mutation_class.as_str(),
                self.scope(&label.scope_ref)
                    .map(|s| s.scope_class.as_str())
                    .unwrap_or("?"),
                label.review_state.as_str(),
                label.surface.as_str()
            ));
            if let Some(rendered) = self.render_label(&label.label_id) {
                out.push_str(&format!("  - Label: {rendered}\n"));
            }
        }

        out.push_str("\n## Count/scope disclosures\n\n");
        for disclosure in &self.disclosures {
            out.push_str(&format!(
                "- `{}` on `{}`\n",
                disclosure.disclosure_id,
                disclosure.surface.as_str()
            ));
            if let Some(rendered) = self.render_disclosure(&disclosure.disclosure_id) {
                out.push_str(&format!("  - Phrase: {rendered}\n"));
            }
        }

        out.push_str("\n## Cross-surface scope reuse\n\n");
        for (scope_id, surfaces) in self.cross_surface_reuse() {
            out.push_str(&format!(
                "- `{}`: {}\n",
                scope_id,
                surfaces.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in catalog export.
#[derive(Debug)]
pub enum ActionLabelScopeCatalogArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ActionLabelScopeCatalogViolation>),
}

impl fmt::Display for ActionLabelScopeCatalogArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "action-label/scope catalog export parse failed: {error}"
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
                    "action-label/scope catalog export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ActionLabelScopeCatalogArtifactError {}

/// Validation failures emitted by [`ActionLabelScopeCatalog::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionLabelScopeCatalogViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A closed inventory drifted from the canonical token list.
    InventoryDrift,
    /// The banned-token set omits a required vague token.
    BannedTokenSetMissing,
    /// A scope definition is incomplete or drifts from its scope class.
    ScopeIncomplete,
    /// A scope id or phrase is malformed.
    ScopeTokenMalformed,
    /// A scope is duplicated.
    DuplicateScope,
    /// The scope inventory is not fully realized by scope definitions.
    ScopeDefinitionsIncomplete,
    /// A verb is incomplete.
    VerbIncomplete,
    /// A verb id is not locale-neutral.
    VerbTokenNotLocaleNeutral,
    /// A verb is duplicated.
    DuplicateVerb,
    /// A controlled verb's label is itself an ambiguous default token.
    VerbLabelAmbiguous,
    /// An object is incomplete.
    ObjectIncomplete,
    /// An object id is not locale-neutral.
    ObjectTokenNotLocaleNeutral,
    /// An object is duplicated.
    DuplicateObject,
    /// A label is incomplete.
    LabelIncomplete,
    /// A label id is not locale-neutral.
    LabelIdNotLocaleNeutral,
    /// A label is duplicated.
    DuplicateLabel,
    /// A label's verb ref does not resolve.
    VerbRefUnresolved,
    /// A label's or disclosure's scope ref does not resolve.
    ScopeRefUnresolved,
    /// A label's or disclosure's object ref does not resolve.
    ObjectRefUnresolved,
    /// A label is not verb-first.
    LabelNotVerbFirst,
    /// A primary label hides scope behind a vague verb.
    AmbiguousPrimaryLabel,
    /// A label targets a non-actionable (exclusion) scope.
    ActionScopeNotActionable,
    /// A batch label does not declare its scope and no sheet disambiguates it.
    ScopeNotDeclared,
    /// A label does not narrow the object class.
    ObjectClassNotNarrowed,
    /// A counted batch label omits its count.
    BatchCountMissing,
    /// A label's count variable and template `{count:...}` slot disagree.
    CountVarMismatch,
    /// An approval or batch label does not declare a review obligation.
    ReviewStateNotDeclared,
    /// A destructive, publish, or install label does not disclose its side effect.
    SideEffectNotDisclosed,
    /// A screen-reader label omits the verb or scope, or uses a vague token.
    ScreenReaderLabelIncomplete,
    /// A template placeholder does not resolve to a declared token.
    TemplatePlaceholderUnresolved,
    /// A declared scope ref or count variable is not used by the template.
    DeclaredTokenUnused,
    /// A label has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A disclosure is incomplete.
    DisclosureIncomplete,
    /// A disclosure is duplicated.
    DuplicateDisclosure,
    /// A scope class, surface, mutation class, or review state is never used.
    CoverageGap,
    /// A shared scope phrase does not span enough surfaces.
    SharedScopeReuseInsufficient,
    /// Too few labels carry product-UI, docs, and support-export parity together.
    DocsExportParityMissing,
    /// Parity review does not satisfy required invariants.
    ParityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ActionLabelScopeCatalogViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::InventoryDrift => "inventory_drift",
            Self::BannedTokenSetMissing => "banned_token_set_missing",
            Self::ScopeIncomplete => "scope_incomplete",
            Self::ScopeTokenMalformed => "scope_token_malformed",
            Self::DuplicateScope => "duplicate_scope",
            Self::ScopeDefinitionsIncomplete => "scope_definitions_incomplete",
            Self::VerbIncomplete => "verb_incomplete",
            Self::VerbTokenNotLocaleNeutral => "verb_token_not_locale_neutral",
            Self::DuplicateVerb => "duplicate_verb",
            Self::VerbLabelAmbiguous => "verb_label_ambiguous",
            Self::ObjectIncomplete => "object_incomplete",
            Self::ObjectTokenNotLocaleNeutral => "object_token_not_locale_neutral",
            Self::DuplicateObject => "duplicate_object",
            Self::LabelIncomplete => "label_incomplete",
            Self::LabelIdNotLocaleNeutral => "label_id_not_locale_neutral",
            Self::DuplicateLabel => "duplicate_label",
            Self::VerbRefUnresolved => "verb_ref_unresolved",
            Self::ScopeRefUnresolved => "scope_ref_unresolved",
            Self::ObjectRefUnresolved => "object_ref_unresolved",
            Self::LabelNotVerbFirst => "label_not_verb_first",
            Self::AmbiguousPrimaryLabel => "ambiguous_primary_label",
            Self::ActionScopeNotActionable => "action_scope_not_actionable",
            Self::ScopeNotDeclared => "scope_not_declared",
            Self::ObjectClassNotNarrowed => "object_class_not_narrowed",
            Self::BatchCountMissing => "batch_count_missing",
            Self::CountVarMismatch => "count_var_mismatch",
            Self::ReviewStateNotDeclared => "review_state_not_declared",
            Self::SideEffectNotDisclosed => "side_effect_not_disclosed",
            Self::ScreenReaderLabelIncomplete => "screen_reader_label_incomplete",
            Self::TemplatePlaceholderUnresolved => "template_placeholder_unresolved",
            Self::DeclaredTokenUnused => "declared_token_unused",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::DuplicateDisclosure => "duplicate_disclosure",
            Self::CoverageGap => "coverage_gap",
            Self::SharedScopeReuseInsufficient => "shared_scope_reuse_insufficient",
            Self::DocsExportParityMissing => "docs_export_parity_missing",
            Self::ParityReviewIncomplete => "parity_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in catalog support export.
pub fn current_action_label_scope_catalog_export(
) -> Result<ActionLabelScopeCatalog, ActionLabelScopeCatalogArtifactError> {
    let packet: ActionLabelScopeCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5-action-label-proof/support_export.json"
    )))
    .map_err(ActionLabelScopeCatalogArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ActionLabelScopeCatalogArtifactError::Validation(violations))
    }
}

/// A parsed segment of a reference template.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TemplateSegment {
    /// Literal text run.
    Text(String),
    /// A `{verb}` placeholder.
    Verb,
    /// A `{count:<name>}` placeholder; carries the count-variable name.
    Count(String),
    /// A `{scope:<scope_id>}` placeholder; carries the scope id.
    Scope(String),
    /// An `{object_one}` placeholder.
    ObjectOne,
    /// An `{object_many}` placeholder.
    ObjectMany,
    /// A `{count_status}` placeholder.
    CountStatus,
    /// An unrecognized `{...}` placeholder; carries the raw `{...}` text.
    Unknown(String),
}

/// Parses a reference template into ordered text/placeholder segments.
fn parse_template(template: &str) -> Vec<TemplateSegment> {
    let mut segments = Vec::new();
    let mut text = String::new();
    let mut chars = template.char_indices().peekable();
    while let Some((_, ch)) = chars.next() {
        if ch == '{' {
            if !text.is_empty() {
                segments.push(TemplateSegment::Text(std::mem::take(&mut text)));
            }
            let mut inner = String::new();
            let mut closed = false;
            for (_, inner_ch) in chars.by_ref() {
                if inner_ch == '}' {
                    closed = true;
                    break;
                }
                inner.push(inner_ch);
            }
            if !closed {
                segments.push(TemplateSegment::Unknown(format!("{{{inner}")));
            } else if inner == "verb" {
                segments.push(TemplateSegment::Verb);
            } else if inner == "object_one" {
                segments.push(TemplateSegment::ObjectOne);
            } else if inner == "object_many" {
                segments.push(TemplateSegment::ObjectMany);
            } else if inner == "count_status" {
                segments.push(TemplateSegment::CountStatus);
            } else if let Some(name) = inner.strip_prefix("count:") {
                segments.push(TemplateSegment::Count(name.to_owned()));
            } else if let Some(id) = inner.strip_prefix("scope:") {
                segments.push(TemplateSegment::Scope(id.to_owned()));
            } else {
                segments.push(TemplateSegment::Unknown(format!("{{{inner}}}")));
            }
        } else {
            text.push(ch);
        }
    }
    if !text.is_empty() {
        segments.push(TemplateSegment::Text(text));
    }
    segments
}

/// True when `token` is a locale-neutral machine identifier.
fn is_locale_neutral(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
}

/// Normalizes a rendered word to its ascii-alphanumeric lowercase core, so locale
/// markers and punctuation around a verb do not hide an ambiguous default token.
fn ambiguity_core(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn token_list<T: Copy>(all: &[T], as_str: fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| as_str(*t).to_owned()).collect()
}

fn validate_source_contracts(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ACTION_LABEL_SCOPE_CATALOG_SCHEMA_REF,
        ACTION_LABEL_SCOPE_CATALOG_DOC_REF,
        CATALOG_UI_COPY_CONTRACT_REF,
        CATALOG_NAMING_LABEL_CONTRACT_REF,
        CATALOG_COUNT_SCOPE_GRAMMAR_REF,
        CATALOG_COUNT_SCOPE_TERM_SET_REF,
        CATALOG_SAFETY_CRITICAL_SCHEMA_REF,
        CATALOG_SAFETY_CRITICAL_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ActionLabelScopeCatalogViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_inventories(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    if packet.scope_inventory != token_list(&ScopeClass::ALL, ScopeClass::as_str)
        || packet.surface_inventory != token_list(&ActionSurface::ALL, ActionSurface::as_str)
        || packet.mutation_class_inventory != token_list(&MutationClass::ALL, MutationClass::as_str)
        || packet.review_state_inventory != token_list(&ReviewState::ALL, ReviewState::as_str)
    {
        violations.push(ActionLabelScopeCatalogViolation::InventoryDrift);
    }
}

fn validate_banned_tokens(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let present: BTreeSet<&str> = packet
        .banned_ambiguous_tokens
        .iter()
        .map(String::as_str)
        .collect();
    for required in ActionLabelScopeCatalog::required_banned_tokens() {
        if !present.contains(required) {
            violations.push(ActionLabelScopeCatalogViolation::BannedTokenSetMissing);
            return;
        }
    }
}

fn validate_scopes(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_classes: BTreeSet<ScopeClass> = BTreeSet::new();
    for scope in &packet.scopes {
        if scope.canonical_phrase.trim().is_empty() {
            violations.push(ActionLabelScopeCatalogViolation::ScopeIncomplete);
        }
        if !is_locale_neutral(&scope.scope_id) {
            violations.push(ActionLabelScopeCatalogViolation::ScopeTokenMalformed);
        }
        // The scope definition must agree with its declared class so a phrase can
        // never carry a posture (actionable, exclusion, count) the class forbids.
        if scope.scope_id != scope.scope_class.as_str()
            || scope.actionable != scope.scope_class.actionable()
            || scope.is_exclusion != scope.scope_class.is_exclusion()
            || scope.requires_count != scope.scope_class.requires_count()
        {
            violations.push(ActionLabelScopeCatalogViolation::ScopeIncomplete);
        }
        if !seen_ids.insert(scope.scope_id.as_str()) {
            violations.push(ActionLabelScopeCatalogViolation::DuplicateScope);
        }
        seen_classes.insert(scope.scope_class);
    }
    if !ScopeClass::ALL.iter().all(|c| seen_classes.contains(c)) {
        violations.push(ActionLabelScopeCatalogViolation::ScopeDefinitionsIncomplete);
    }
}

fn validate_verbs(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let banned: BTreeSet<&str> = packet
        .banned_ambiguous_tokens
        .iter()
        .map(String::as_str)
        .collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for verb in &packet.verbs {
        if verb.canonical_label.trim().is_empty() {
            violations.push(ActionLabelScopeCatalogViolation::VerbIncomplete);
        }
        if !is_locale_neutral(&verb.verb_id) {
            violations.push(ActionLabelScopeCatalogViolation::VerbTokenNotLocaleNeutral);
        }
        if !seen.insert(verb.verb_id.as_str()) {
            violations.push(ActionLabelScopeCatalogViolation::DuplicateVerb);
        }
        if banned.contains(verb.verb_id.as_str())
            || banned.contains(ambiguity_core(&verb.canonical_label).as_str())
        {
            violations.push(ActionLabelScopeCatalogViolation::VerbLabelAmbiguous);
        }
    }
}

fn validate_objects(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for object in &packet.objects {
        if object.singular_label.trim().is_empty() || object.plural_label.trim().is_empty() {
            violations.push(ActionLabelScopeCatalogViolation::ObjectIncomplete);
        }
        if !is_locale_neutral(&object.object_id) {
            violations.push(ActionLabelScopeCatalogViolation::ObjectTokenNotLocaleNeutral);
        }
        if !seen.insert(object.object_id.as_str()) {
            violations.push(ActionLabelScopeCatalogViolation::DuplicateObject);
        }
    }
}

fn validate_labels(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let banned: BTreeSet<&str> = packet
        .banned_ambiguous_tokens
        .iter()
        .map(String::as_str)
        .collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();

    for label in &packet.labels {
        if label.reference_label.trim().is_empty()
            || label.screen_reader_label.trim().is_empty()
            || label.consumer_surfaces.is_empty()
        {
            violations.push(ActionLabelScopeCatalogViolation::LabelIncomplete);
        }
        if label.consumer_surfaces.is_empty() {
            violations.push(ActionLabelScopeCatalogViolation::ConsumerSurfacesMissing);
        }
        if !is_locale_neutral(&label.label_id) {
            violations.push(ActionLabelScopeCatalogViolation::LabelIdNotLocaleNeutral);
        }
        if !seen.insert(label.label_id.as_str()) {
            violations.push(ActionLabelScopeCatalogViolation::DuplicateLabel);
        }

        let verb = packet.verb(&label.verb_ref);
        if verb.is_none() {
            violations.push(ActionLabelScopeCatalogViolation::VerbRefUnresolved);
        }
        if packet.object(&label.object_ref).is_none() {
            violations.push(ActionLabelScopeCatalogViolation::ObjectRefUnresolved);
        }
        let scope = packet.scope(&label.scope_ref);
        match scope {
            None => violations.push(ActionLabelScopeCatalogViolation::ScopeRefUnresolved),
            Some(scope) => {
                if !scope.actionable {
                    violations.push(ActionLabelScopeCatalogViolation::ActionScopeNotActionable);
                }
            }
        }

        validate_label_verb_first(label, violations);
        validate_label_ambiguity(packet, label, verb, &banned, violations);
        validate_label_template(packet, label, scope, violations);
        validate_label_review_and_side_effect(label, violations);
        validate_label_screen_reader(packet, label, scope, &banned, violations);
    }
}

fn validate_label_verb_first(
    label: &ActionLabel,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let first = parse_template(&label.reference_label)
        .into_iter()
        .find(|segment| !matches!(segment, TemplateSegment::Text(t) if t.trim().is_empty()));
    if !matches!(first, Some(TemplateSegment::Verb)) {
        violations.push(ActionLabelScopeCatalogViolation::LabelNotVerbFirst);
    }
}

fn validate_label_ambiguity(
    packet: &ActionLabelScopeCatalog,
    label: &ActionLabel,
    verb: Option<&ActionVerb>,
    banned: &BTreeSet<&str>,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let banned_verb = verb
        .map(|verb| {
            banned.contains(verb.verb_id.as_str())
                || banned.contains(ambiguity_core(&verb.canonical_label).as_str())
        })
        .unwrap_or(false);
    let banned_first_word = packet
        .render_label(&label.label_id)
        .and_then(|rendered| {
            rendered
                .split_whitespace()
                .map(ambiguity_core)
                .find(|word| !word.is_empty())
        })
        .map(|word| banned.contains(word.as_str()))
        .unwrap_or(false);
    if banned_verb || banned_first_word {
        violations.push(ActionLabelScopeCatalogViolation::AmbiguousPrimaryLabel);
    }
}

fn validate_label_template(
    packet: &ActionLabelScopeCatalog,
    label: &ActionLabel,
    scope: Option<&ScopeDefinition>,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let mut has_object = false;
    let mut used_scope = false;
    let mut used_count: Option<String> = None;
    let mut count_in_template = false;

    for segment in parse_template(&label.reference_label) {
        match segment {
            TemplateSegment::Scope(scope_id) => {
                if packet.scope(&scope_id).is_none() {
                    violations
                        .push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
                }
                if scope_id == label.scope_ref {
                    used_scope = true;
                }
            }
            TemplateSegment::Count(name) => {
                count_in_template = true;
                used_count = Some(name);
            }
            TemplateSegment::ObjectOne | TemplateSegment::ObjectMany => has_object = true,
            TemplateSegment::CountStatus => {
                // Count status belongs to disclosures, not action labels.
                violations.push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
            }
            TemplateSegment::Unknown(_) => {
                violations.push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
            }
            TemplateSegment::Verb | TemplateSegment::Text(_) => {}
        }
    }

    if !has_object {
        violations.push(ActionLabelScopeCatalogViolation::ObjectClassNotNarrowed);
    }

    if let Some(scope) = scope {
        if scope.scope_class.requires_explicit_scope_word()
            && !used_scope
            && !label.scope_unambiguous_in_sheet
        {
            violations.push(ActionLabelScopeCatalogViolation::ScopeNotDeclared);
        }
        if scope.requires_count && label.count_var.is_none() {
            violations.push(ActionLabelScopeCatalogViolation::BatchCountMissing);
        }
    }

    // The declared count variable and the template `{count:...}` slot must agree.
    match (&label.count_var, count_in_template) {
        (Some(name), true) if used_count.as_deref() == Some(name.as_str()) => {}
        (None, false) => {}
        _ => violations.push(ActionLabelScopeCatalogViolation::CountVarMismatch),
    }
}

fn validate_label_review_and_side_effect(
    label: &ActionLabel,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    if label.mutation_class.carries_review_obligation()
        && label.review_state == ReviewState::NoReviewNeeded
    {
        violations.push(ActionLabelScopeCatalogViolation::ReviewStateNotDeclared);
    }
    if label.mutation_class.must_disclose_side_effect() && !label.discloses_side_effect {
        violations.push(ActionLabelScopeCatalogViolation::SideEffectNotDisclosed);
    }
}

fn validate_label_screen_reader(
    packet: &ActionLabelScopeCatalog,
    label: &ActionLabel,
    scope: Option<&ScopeDefinition>,
    banned: &BTreeSet<&str>,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let segments = parse_template(&label.screen_reader_label);
    let mut has_verb = false;
    let mut names_scope = false;
    let mut has_object = false;
    let mut ok = true;
    for segment in &segments {
        match segment {
            TemplateSegment::Verb => has_verb = true,
            TemplateSegment::Scope(scope_id) => {
                if packet.scope(scope_id).is_none() {
                    ok = false;
                }
                if scope_id == &label.scope_ref {
                    names_scope = true;
                }
            }
            TemplateSegment::Count(_) => {}
            TemplateSegment::ObjectOne | TemplateSegment::ObjectMany => has_object = true,
            TemplateSegment::CountStatus | TemplateSegment::Unknown(_) => ok = false,
            TemplateSegment::Text(_) => {}
        }
    }
    // The narrated label must always carry the verb, the object class, and — for an
    // actionable multi-object scope — the scope phrase, even when the visible button
    // relies on the surrounding sheet to disambiguate.
    let scope_required = scope
        .map(|scope| scope.scope_class.requires_explicit_scope_word())
        .unwrap_or(false);
    let rendered_clean = packet
        .render_template(
            &label.screen_reader_label,
            Some(label.verb_ref.as_str()),
            label.object_ref.as_str(),
            None,
        )
        .split_whitespace()
        .map(ambiguity_core)
        .all(|word| !banned.contains(word.as_str()));
    if !has_verb || !has_object || (scope_required && !names_scope) || !ok || !rendered_clean {
        violations.push(ActionLabelScopeCatalogViolation::ScreenReaderLabelIncomplete);
    }
}

fn validate_disclosures(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for disclosure in &packet.disclosures {
        if disclosure.reference_phrase.trim().is_empty()
            || disclosure.consumer_surfaces.is_empty()
            || disclosure.count_vars.is_empty()
        {
            violations.push(ActionLabelScopeCatalogViolation::DisclosureIncomplete);
        }
        if !is_locale_neutral(&disclosure.disclosure_id) {
            violations.push(ActionLabelScopeCatalogViolation::LabelIdNotLocaleNeutral);
        }
        if !seen.insert(disclosure.disclosure_id.as_str()) {
            violations.push(ActionLabelScopeCatalogViolation::DuplicateDisclosure);
        }
        if packet.object(&disclosure.object_ref).is_none() {
            violations.push(ActionLabelScopeCatalogViolation::ObjectRefUnresolved);
        }
        if let Some(verb_ref) = &disclosure.verb_ref {
            if packet.verb(verb_ref).is_none() {
                violations.push(ActionLabelScopeCatalogViolation::VerbRefUnresolved);
            }
        }
        for scope_id in std::iter::once(&disclosure.primary_scope_ref)
            .chain(disclosure.disclosed_scope_refs.iter())
        {
            if packet.scope(scope_id).is_none() {
                violations.push(ActionLabelScopeCatalogViolation::ScopeRefUnresolved);
            }
        }
        validate_disclosure_template(packet, disclosure, violations);
    }
}

fn validate_disclosure_template(
    packet: &ActionLabelScopeCatalog,
    disclosure: &ScopeDisclosure,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let declared_counts: BTreeSet<&str> =
        disclosure.count_vars.iter().map(String::as_str).collect();
    let declared_scopes: BTreeSet<&str> = std::iter::once(disclosure.primary_scope_ref.as_str())
        .chain(disclosure.disclosed_scope_refs.iter().map(String::as_str))
        .collect();
    let mut used_counts: BTreeSet<String> = BTreeSet::new();
    let mut used_scopes: BTreeSet<String> = BTreeSet::new();
    let mut has_object = false;

    for segment in parse_template(&disclosure.reference_phrase) {
        match segment {
            TemplateSegment::Verb => {
                if disclosure.verb_ref.is_none() {
                    violations
                        .push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
                }
            }
            TemplateSegment::Count(name) => {
                if !declared_counts.contains(name.as_str()) {
                    violations
                        .push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
                }
                used_counts.insert(name);
            }
            TemplateSegment::Scope(scope_id) => {
                if !declared_scopes.contains(scope_id.as_str()) || packet.scope(&scope_id).is_none()
                {
                    violations
                        .push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
                }
                used_scopes.insert(scope_id);
            }
            TemplateSegment::ObjectOne | TemplateSegment::ObjectMany => has_object = true,
            TemplateSegment::CountStatus => {}
            TemplateSegment::Unknown(_) => {
                violations.push(ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved);
            }
            TemplateSegment::Text(_) => {}
        }
    }

    if !has_object {
        violations.push(ActionLabelScopeCatalogViolation::ObjectClassNotNarrowed);
    }
    // Every declared count variable and scope ref must be used, or the disclosure
    // claims a population it never names.
    let unused_counts = declared_counts.iter().any(|c| !used_counts.contains(*c));
    let unused_scopes = declared_scopes.iter().any(|s| !used_scopes.contains(*s));
    if unused_counts || unused_scopes {
        violations.push(ActionLabelScopeCatalogViolation::DeclaredTokenUnused);
    }
}

fn validate_coverage(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let mut scopes: BTreeSet<&str> = BTreeSet::new();
    let mut surfaces: BTreeSet<ActionSurface> = BTreeSet::new();
    let mutations: BTreeSet<MutationClass> =
        packet.labels.iter().map(|l| l.mutation_class).collect();
    let reviews: BTreeSet<ReviewState> = packet.labels.iter().map(|l| l.review_state).collect();

    for label in &packet.labels {
        scopes.insert(label.scope_ref.as_str());
        surfaces.insert(label.surface);
    }
    for disclosure in &packet.disclosures {
        surfaces.insert(disclosure.surface);
        scopes.insert(disclosure.primary_scope_ref.as_str());
        for scope_id in &disclosure.disclosed_scope_refs {
            scopes.insert(scope_id.as_str());
        }
    }

    let scopes_covered = ScopeClass::ALL.iter().all(|c| scopes.contains(c.as_str()));
    let surfaces_covered = ActionSurface::ALL.iter().all(|s| surfaces.contains(s));
    let mutations_covered = MutationClass::ALL.iter().all(|m| mutations.contains(m));
    let reviews_covered = ReviewState::ALL.iter().all(|r| reviews.contains(r));
    if !scopes_covered || !surfaces_covered || !mutations_covered || !reviews_covered {
        violations.push(ActionLabelScopeCatalogViolation::CoverageGap);
    }
}

fn validate_shared_reuse(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    if packet.shared_scope_phrase_ids.is_empty() {
        violations.push(ActionLabelScopeCatalogViolation::SharedScopeReuseInsufficient);
        return;
    }
    let reuse = packet.cross_surface_reuse();
    for scope_id in &packet.shared_scope_phrase_ids {
        let spans = reuse.get(scope_id).map(BTreeSet::len).unwrap_or(0);
        if spans < SHARED_SCOPE_MIN_REUSE_SURFACES {
            violations.push(ActionLabelScopeCatalogViolation::SharedScopeReuseInsufficient);
        }
    }
}

fn validate_docs_export_parity(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let parity_labels = packet
        .labels
        .iter()
        .filter(|label| {
            let surfaces: BTreeSet<ConsumerSurface> =
                label.consumer_surfaces.iter().copied().collect();
            surfaces.contains(&ConsumerSurface::ProductUi)
                && surfaces.contains(&ConsumerSurface::Docs)
                && surfaces.contains(&ConsumerSurface::SupportExport)
        })
        .count();
    if parity_labels < DOCS_EXPORT_PARITY_MIN_LABELS {
        violations.push(ActionLabelScopeCatalogViolation::DocsExportParityMissing);
    }
}

fn validate_parity_review(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let review = &packet.parity_review;
    for ok in [
        review.labels_are_verb_first,
        review.no_ambiguous_primary_labels,
        review.scope_declared_or_unambiguous_in_sheet,
        review.batch_actions_declare_count,
        review.review_state_declared_on_approval_and_batch,
        review.object_class_narrowed,
        review.one_controlled_scope_phrase_set,
        review.side_effects_disclosed,
        review.screen_reader_labels_complete,
        review.docs_and_export_reuse_runtime_labels,
    ] {
        if !ok {
            violations.push(ActionLabelScopeCatalogViolation::ParityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.product_ui_resolves_through_catalog,
        projection.cli_help_uses_action_labels,
        projection.docs_render_action_labels,
        projection.support_export_uses_action_labels,
        projection.screen_reader_reuses_labels,
        projection.activity_feed_reuses_labels,
    ] {
        if !ok {
            violations.push(ActionLabelScopeCatalogViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ActionLabelScopeCatalogViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &ActionLabelScopeCatalog,
    violations: &mut Vec<ActionLabelScopeCatalogViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(ActionLabelScopeCatalogViolation::ReleasePostureIncomplete);
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Rewrites a reference template into a pseudo-localized form by wrapping each human
/// text run in locale markers while leaving every `{...}` placeholder — the
/// locale-neutral machine identity — untouched.
pub fn pseudo_localize_template(template: &str) -> String {
    let mut out = String::new();
    for segment in parse_template(template) {
        match segment {
            TemplateSegment::Text(text) => {
                let trimmed = text.trim();
                if trimmed.is_empty() {
                    out.push_str(&text);
                } else {
                    let leading = &text[..text.len() - text.trim_start().len()];
                    let trailing = &text[text.trim_end().len()..];
                    out.push_str(leading);
                    out.push('\u{27e6}');
                    out.push_str(trimmed);
                    out.push('\u{27e7}');
                    out.push_str(trailing);
                }
            }
            TemplateSegment::Verb => out.push_str("{verb}"),
            TemplateSegment::Count(name) => out.push_str(&format!("{{count:{name}}}")),
            TemplateSegment::Scope(id) => out.push_str(&format!("{{scope:{id}}}")),
            TemplateSegment::ObjectOne => out.push_str("{object_one}"),
            TemplateSegment::ObjectMany => out.push_str("{object_many}"),
            TemplateSegment::CountStatus => out.push_str("{count_status}"),
            TemplateSegment::Unknown(raw) => out.push_str(&raw),
        }
    }
    out
}

/// Pseudo-localizes a plain default-locale word (verb label, scope phrase, object
/// noun) while keeping it a single non-empty token run.
pub fn pseudo_localize_phrase(phrase: &str) -> String {
    if phrase.trim().is_empty() {
        phrase.to_owned()
    } else {
        format!("\u{27e6}{phrase}\u{27e7}")
    }
}
