//! Shared dense-collection filter, saved-view, count, selection, and batch-review contract.
//!
//! This module is the canonical alpha substrate for collection-shaped
//! surfaces. Search result panes, review queues, package inventories, and
//! later admin grids use these records so filters, counters, saved views,
//! select-all actions, and consequential batch actions keep one portable
//! vocabulary.

use serde::{Deserialize, Serialize};

use crate::{
    SearchQuerySession, SearchScopeCountsClass, SearchScopeCountsRecord,
    SCOPE_TRUTH_COUNTS_SCHEMA_VERSION,
};

/// Schema version for collection filter AST records.
pub const FILTER_AST_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema version for saved collection-view records.
pub const SAVED_VIEW_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema version for collection view, selection, and batch-review records.
pub const COLLECTION_VIEW_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for [`CollectionFilterAst`].
pub const COLLECTION_FILTER_AST_RECORD_KIND: &str = "collection_filter_ast_alpha";

/// Record-kind tag for [`SavedCollectionView`].
pub const SAVED_COLLECTION_VIEW_RECORD_KIND: &str = "saved_collection_view_alpha";

/// Record-kind tag for [`CollectionViewAlphaRecord`].
pub const COLLECTION_VIEW_ALPHA_RECORD_KIND: &str = "collection_view_alpha";

/// Record-kind tag for [`CollectionSelectionState`].
pub const COLLECTION_SELECTION_STATE_RECORD_KIND: &str = "collection_selection_state_alpha";

/// Record-kind tag for [`BatchReviewSheet`].
pub const BATCH_REVIEW_SHEET_RECORD_KIND: &str = "collection_batch_review_sheet_alpha";

/// Surface family that consumes the shared collection contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionSurfaceFamily {
    /// Search results, quick-open result sets, and search export previews.
    SearchCollection,
    /// Review inboxes, review workspaces, and diff-review queues.
    ReviewCollection,
    /// Package, extension, dependency, and inventory grids.
    PackageOrInventoryGrid,
    /// Work-item, issue, and triage collections.
    WorkItemCollection,
    /// Administrative, settings, policy, or audit grids.
    AdminOrSettingsGrid,
}

impl CollectionSurfaceFamily {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchCollection => "search_collection",
            Self::ReviewCollection => "review_collection",
            Self::PackageOrInventoryGrid => "package_or_inventory_grid",
            Self::WorkItemCollection => "work_item_collection",
            Self::AdminOrSettingsGrid => "admin_or_settings_grid",
        }
    }
}

/// Source class for a filter clause or hidden narrowing chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionFilterSourceClass {
    /// Clause was added directly by the user.
    User,
    /// Clause was restored from a saved view.
    SavedView,
    /// Clause was forced by policy or trust posture.
    Policy,
    /// Clause came from the current workset or sparse slice.
    Workset,
    /// Clause came from provider capability or provider-side limitation.
    ProviderLimit,
    /// Clause came from a compact client or local viewport limitation.
    ClientLimit,
    /// Clause discloses partial, stale, or warming data.
    PartialData,
}

impl CollectionFilterSourceClass {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::SavedView => "saved_view",
            Self::Policy => "policy",
            Self::Workset => "workset",
            Self::ProviderLimit => "provider_limit",
            Self::ClientLimit => "client_limit",
            Self::PartialData => "partial_data",
        }
    }

    /// True when the clause represents narrowing that must remain visible.
    pub const fn is_hidden_narrowing_source(self) -> bool {
        matches!(
            self,
            Self::Policy
                | Self::Workset
                | Self::ProviderLimit
                | Self::ClientLimit
                | Self::PartialData
        )
    }
}

/// Stable operator vocabulary for typed collection filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionFilterOperator {
    /// Free text query term interpreted by the collection provider.
    FreeText,
    /// Field equals one value.
    Equals,
    /// Field does not equal one value.
    NotEquals,
    /// Field contains a value or substring.
    Contains,
    /// Field starts with a value.
    Prefix,
    /// Field is one of a finite value set.
    In,
    /// Field is inside an inclusive or provider-defined range.
    Range,
    /// Field exists or is present.
    Exists,
    /// Field is greater than or equal to a value.
    GreaterOrEqual,
    /// Field is less than or equal to a value.
    LessOrEqual,
}

impl CollectionFilterOperator {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreeText => "free_text",
            Self::Equals => "equals",
            Self::NotEquals => "not_equals",
            Self::Contains => "contains",
            Self::Prefix => "prefix",
            Self::In => "in",
            Self::Range => "range",
            Self::Exists => "exists",
            Self::GreaterOrEqual => "greater_or_equal",
            Self::LessOrEqual => "less_or_equal",
        }
    }
}

/// Redaction posture for filter literal material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterLiteralMaterialClass {
    /// Raw literal may remain in local-only state.
    RawLocalOnly,
    /// Literal is represented by redacted reviewable text.
    RedactedText,
    /// Literal is represented by deterministic hash material.
    HashedLiteral,
    /// The clause has only an operator or field presence requirement.
    OperatorOnly,
}

impl FilterLiteralMaterialClass {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawLocalOnly => "raw_local_only",
            Self::RedactedText => "redacted_text",
            Self::HashedLiteral => "hashed_literal",
            Self::OperatorOnly => "operator_only",
        }
    }

    /// True when the value can cross workspace, support, or shared boundaries.
    pub const fn is_portable(self) -> bool {
        !matches!(self, Self::RawLocalOnly)
    }
}

/// Redaction-aware filter literal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionFilterLiteral {
    /// Literal material posture.
    pub material_class: FilterLiteralMaterialClass,
    /// Short display label for chips and review sheets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
    /// Hash material when raw text cannot travel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_hash: Option<String>,
}

impl CollectionFilterLiteral {
    /// Builds a local-only literal.
    pub fn raw_local_only(value: impl Into<String>) -> Self {
        Self {
            material_class: FilterLiteralMaterialClass::RawLocalOnly,
            display_value: Some(value.into()),
            value_hash: None,
        }
    }

    /// Builds a redacted display literal.
    pub fn redacted(display_value: impl Into<String>) -> Self {
        Self {
            material_class: FilterLiteralMaterialClass::RedactedText,
            display_value: Some(display_value.into()),
            value_hash: None,
        }
    }

    /// Builds a hash-only literal.
    pub fn hashed(value_hash: impl Into<String>) -> Self {
        Self {
            material_class: FilterLiteralMaterialClass::HashedLiteral,
            display_value: None,
            value_hash: Some(value_hash.into()),
        }
    }

    /// Builds an operator-only literal.
    pub fn operator_only() -> Self {
        Self {
            material_class: FilterLiteralMaterialClass::OperatorOnly,
            display_value: None,
            value_hash: None,
        }
    }

    /// True when this literal can be stored in a portable saved view.
    pub const fn is_portable(&self) -> bool {
        self.material_class.is_portable()
    }
}

/// Round-trip state for one filter clause after reopening or provider replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterRoundTripState {
    /// Clause round-trips exactly.
    RoundTrippable,
    /// Provider or extension no longer supports the clause exactly.
    UnsupportedByProvider,
    /// Clause was preserved but degraded because data is stale.
    StaleProviderData,
    /// Clause could not reveal literal material by policy.
    LiteralWithheldByPolicy,
}

impl FilterRoundTripState {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoundTrippable => "round_trippable",
            Self::UnsupportedByProvider => "unsupported_by_provider",
            Self::StaleProviderData => "stale_provider_data",
            Self::LiteralWithheldByPolicy => "literal_withheld_by_policy",
        }
    }

    /// True when the filter must render a stale or degraded label.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::RoundTrippable)
    }
}

/// One typed filter clause inside a collection filter AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionFilterClause {
    /// Stable clause id used by chips and saved-view diagnostics.
    pub clause_id: String,
    /// Stable field id or provider field token.
    pub field_id: String,
    /// Human-readable field label for chips.
    pub label: String,
    /// Operator applied to the field.
    pub operator: CollectionFilterOperator,
    /// Redaction-aware literal material for the clause.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<CollectionFilterLiteral>,
    /// Source class explaining who owns the clause.
    pub source_class: CollectionFilterSourceClass,
    /// True when the user cannot remove the clause directly.
    pub locked: bool,
    /// True when the clause is negated.
    pub negated: bool,
    /// True when the clause discloses narrowing outside ordinary user filters.
    pub hidden_narrowing: bool,
    /// Round-trip or degradation state for saved views and provider replay.
    pub round_trip_state: FilterRoundTripState,
    /// Fallback label shown when the clause cannot be replayed exactly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_label: Option<String>,
}

impl CollectionFilterClause {
    /// Builds a filter clause with the source-derived hidden-narrowing default.
    pub fn new(
        clause_id: impl Into<String>,
        field_id: impl Into<String>,
        label: impl Into<String>,
        operator: CollectionFilterOperator,
        literal: Option<CollectionFilterLiteral>,
        source_class: CollectionFilterSourceClass,
    ) -> Self {
        Self {
            clause_id: clause_id.into(),
            field_id: field_id.into(),
            label: label.into(),
            operator,
            literal,
            source_class,
            locked: source_class.is_hidden_narrowing_source(),
            negated: false,
            hidden_narrowing: source_class.is_hidden_narrowing_source(),
            round_trip_state: FilterRoundTripState::RoundTrippable,
            fallback_label: None,
        }
    }

    /// Marks the clause as explicitly locked or removable.
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    /// Marks the clause as negated.
    pub fn with_negated(mut self, negated: bool) -> Self {
        self.negated = negated;
        self
    }

    /// Marks the clause with a fallback/degradation state.
    pub fn with_round_trip_state(
        mut self,
        state: FilterRoundTripState,
        fallback_label: impl Into<String>,
    ) -> Self {
        self.round_trip_state = state;
        self.fallback_label = Some(fallback_label.into());
        self
    }

    /// True when the clause cannot be persisted outside local-only state.
    pub fn contains_local_only_literal(&self) -> bool {
        self.literal
            .as_ref()
            .is_some_and(|literal| !literal.is_portable())
    }

    /// Projects the clause into a chip record.
    pub fn to_chip(&self) -> CollectionFilterChip {
        CollectionFilterChip {
            clause_id_ref: self.clause_id.clone(),
            label: self.label.clone(),
            value_label: self
                .literal
                .as_ref()
                .and_then(|literal| literal.display_value.clone()),
            source_class: self.source_class,
            locked: self.locked,
            negated: self.negated,
            hidden_narrowing: self.hidden_narrowing,
            degraded: self.round_trip_state.is_degraded(),
            action_token: if self.locked {
                "explain".to_string()
            } else {
                "remove".to_string()
            },
        }
    }
}

/// Serializable filter expression tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "expression_kind", rename_all = "snake_case")]
pub enum CollectionFilterExpression {
    /// Leaf filter clause.
    Clause {
        /// Leaf clause payload.
        clause: CollectionFilterClause,
    },
    /// All child expressions must match.
    And {
        /// Child expressions.
        children: Vec<CollectionFilterExpression>,
    },
    /// At least one child expression must match.
    Or {
        /// Child expressions.
        children: Vec<CollectionFilterExpression>,
    },
    /// Child expression must not match.
    Not {
        /// Negated child expression.
        child: Box<CollectionFilterExpression>,
    },
}

impl CollectionFilterExpression {
    /// Returns all leaf clauses in stable traversal order.
    pub fn clauses(&self) -> Vec<&CollectionFilterClause> {
        let mut clauses = Vec::new();
        self.collect_clauses(&mut clauses);
        clauses
    }

    fn collect_clauses<'a>(&'a self, clauses: &mut Vec<&'a CollectionFilterClause>) {
        match self {
            Self::Clause { clause } => clauses.push(clause),
            Self::And { children } | Self::Or { children } => {
                for child in children {
                    child.collect_clauses(clauses);
                }
            }
            Self::Not { child } => child.collect_clauses(clauses),
        }
    }
}

/// Typed and serializable collection filter AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionFilterAst {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the filter AST.
    pub schema_version: u32,
    /// Stable filter AST identity.
    pub filter_ast_id: String,
    /// Compatibility version used by providers and saved views.
    pub compatibility_version: String,
    /// Root expression of the AST.
    pub root: CollectionFilterExpression,
    /// Scope label shown beside the filter bar.
    pub scope_label: String,
    /// Degraded labels that must remain visible when round-trip is partial.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degradation_labels: Vec<String>,
    /// Actor or provider that minted the AST.
    pub created_by: String,
    /// Creation timestamp or deterministic fixture clock.
    pub created_at: String,
}

impl CollectionFilterAst {
    /// Builds an AST from a conjunction of filter clauses.
    pub fn from_clauses(
        filter_ast_id: impl Into<String>,
        scope_label: impl Into<String>,
        clauses: Vec<CollectionFilterClause>,
        created_by: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let degradation_labels = clauses
            .iter()
            .filter(|clause| clause.round_trip_state.is_degraded())
            .filter_map(|clause| clause.fallback_label.clone())
            .collect();
        Self {
            record_kind: COLLECTION_FILTER_AST_RECORD_KIND.to_string(),
            schema_version: FILTER_AST_ALPHA_SCHEMA_VERSION,
            filter_ast_id: filter_ast_id.into(),
            compatibility_version: format!(
                "collection-filter-ast/v{FILTER_AST_ALPHA_SCHEMA_VERSION}"
            ),
            root: CollectionFilterExpression::And {
                children: clauses
                    .into_iter()
                    .map(|clause| CollectionFilterExpression::Clause { clause })
                    .collect(),
            },
            scope_label: scope_label.into(),
            degradation_labels,
            created_by: created_by.into(),
            created_at: created_at.into(),
        }
    }

    /// Returns all leaf clauses in stable traversal order.
    pub fn clauses(&self) -> Vec<&CollectionFilterClause> {
        self.root.clauses()
    }

    /// Returns the filter chips rendered by a collection filter bar.
    pub fn chips(&self) -> Vec<CollectionFilterChip> {
        self.clauses()
            .into_iter()
            .map(CollectionFilterClause::to_chip)
            .collect()
    }

    /// Returns labels for policy, workset, provider, client, or partial-data narrowing.
    pub fn hidden_narrowing_labels(&self) -> Vec<String> {
        self.clauses()
            .into_iter()
            .filter(|clause| clause.hidden_narrowing)
            .map(|clause| {
                let value = clause
                    .literal
                    .as_ref()
                    .and_then(|literal| literal.display_value.as_deref())
                    .unwrap_or(clause.operator.as_str());
                format!("{}: {value}", clause.label)
            })
            .collect()
    }

    /// True when this filter can be stored in a non-local saved view.
    pub fn is_portable(&self) -> bool {
        self.clauses()
            .into_iter()
            .all(|clause| !clause.contains_local_only_literal())
    }

    /// Returns validation findings for filter portability and visibility.
    pub fn validate(&self) -> Vec<CollectionValidationFinding> {
        let mut findings = Vec::new();
        for clause in self.clauses() {
            if clause.source_class.is_hidden_narrowing_source() && !clause.hidden_narrowing {
                findings.push(CollectionValidationFinding::new(
                    CollectionValidationFindingKind::HiddenNarrowingNotSurfaced,
                    "filter_ast.root",
                    format!("{} must render as hidden narrowing", clause.clause_id),
                ));
            }
            if clause.contains_local_only_literal() {
                findings.push(CollectionValidationFinding::new(
                    CollectionValidationFindingKind::LocalOnlyFilterLiteral,
                    "filter_ast.root",
                    format!("{} carries local-only literal material", clause.clause_id),
                ));
            }
        }
        findings
    }
}

/// Filter chip projected from a typed filter clause.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionFilterChip {
    /// Clause id this chip renders.
    pub clause_id_ref: String,
    /// Human-readable chip label.
    pub label: String,
    /// Optional value label shown on the chip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_label: Option<String>,
    /// Source class for the chip.
    pub source_class: CollectionFilterSourceClass,
    /// True when the chip opens detail instead of direct removal.
    pub locked: bool,
    /// True when the chip represents a negated clause.
    pub negated: bool,
    /// True when the chip discloses hidden narrowing.
    pub hidden_narrowing: bool,
    /// True when the chip is stale or degraded.
    pub degraded: bool,
    /// Stable action token for primary keyboard/pointer behavior.
    pub action_token: String,
}

/// Owner scope for a saved collection view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewOwnerScope {
    /// View belongs to one user profile.
    User,
    /// View belongs to the current workspace.
    Workspace,
    /// View is shared with a team or organization.
    Shared,
    /// View is pinned by policy or administration.
    PolicyPinned,
    /// View belongs to an external provider.
    ProviderOwned,
}

impl SavedViewOwnerScope {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Workspace => "workspace",
            Self::Shared => "shared",
            Self::PolicyPinned => "policy_pinned",
            Self::ProviderOwned => "provider_owned",
        }
    }
}

/// Privacy class for a saved collection view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewPrivacyClass {
    /// View may contain local-only material and must not be shared.
    LocalOnlyPrivate,
    /// View is portable inside the workspace.
    WorkspacePortable,
    /// View is shared only after redaction.
    SharedRedacted,
    /// View is governed by policy and may be pinned.
    PolicyGoverned,
    /// View is provider-owned and may require rebind on import.
    ProviderOwned,
}

impl SavedViewPrivacyClass {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyPrivate => "local_only_private",
            Self::WorkspacePortable => "workspace_portable",
            Self::SharedRedacted => "shared_redacted",
            Self::PolicyGoverned => "policy_governed",
            Self::ProviderOwned => "provider_owned",
        }
    }

    /// True when this privacy class may carry local-only filter literals.
    pub const fn permits_local_only_literals(self) -> bool {
        matches!(self, Self::LocalOnlyPrivate)
    }
}

/// Fallback behavior when a saved view cannot replay exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewFallbackBehavior {
    /// Preserve the view and show degraded labels.
    PreserveAndLabelDegraded,
    /// Load portable clauses and label omitted clauses.
    LoadPortableSubsetWithLabels,
    /// Refuse to load until the user or provider rebinding resolves drift.
    RefuseUntilRebound,
    /// Ask the provider to re-resolve unsupported fields or columns.
    ProviderRebindRequired,
}

impl SavedViewFallbackBehavior {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreserveAndLabelDegraded => "preserve_and_label_degraded",
            Self::LoadPortableSubsetWithLabels => "load_portable_subset_with_labels",
            Self::RefuseUntilRebound => "refuse_until_rebound",
            Self::ProviderRebindRequired => "provider_rebind_required",
        }
    }
}

/// Sort key captured by a saved view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionSortKey {
    /// Stable field id sorted by the view.
    pub field_id: String,
    /// True when the sort direction is descending.
    pub descending: bool,
}

/// Versioned saved-view artifact for a dense collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedCollectionView {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for saved view artifacts.
    pub schema_version: u32,
    /// Stable saved-view identity.
    pub saved_view_id: String,
    /// Human-readable view name.
    pub name: String,
    /// Owner scope for sync, export, and sharing decisions.
    pub owner_scope: SavedViewOwnerScope,
    /// Privacy posture for filter material and provider state.
    pub privacy_class: SavedViewPrivacyClass,
    /// Fallback behavior when providers, columns, or filters cannot replay exactly.
    pub fallback_behavior: SavedViewFallbackBehavior,
    /// Filter AST captured by the saved view.
    pub filter_ast: CollectionFilterAst,
    /// Visible column ids captured by the saved view.
    pub visible_column_ids: Vec<String>,
    /// Pinned column ids that cannot be silently hidden.
    pub pinned_column_ids: Vec<String>,
    /// Sort keys captured by the saved view.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort_keys: Vec<CollectionSortKey>,
    /// Scope label captured with the view.
    pub scope_label: String,
    /// True when transient selection was captured, which is invalid for portable views.
    pub captures_selection: bool,
    /// True when a stale provider cursor was captured, which is invalid for portable views.
    pub captures_provider_cursor: bool,
    /// Labels shown when the view is stale or degraded.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stale_or_degraded_labels: Vec<String>,
    /// Creation timestamp or deterministic fixture clock.
    pub created_at: String,
    /// Update timestamp or deterministic fixture clock.
    pub updated_at: String,
}

impl SavedCollectionView {
    /// Builds a saved view from a filter AST and column state.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        saved_view_id: impl Into<String>,
        name: impl Into<String>,
        owner_scope: SavedViewOwnerScope,
        privacy_class: SavedViewPrivacyClass,
        fallback_behavior: SavedViewFallbackBehavior,
        filter_ast: CollectionFilterAst,
        visible_column_ids: Vec<String>,
        pinned_column_ids: Vec<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let created_at = created_at.into();
        let stale_or_degraded_labels = filter_ast.degradation_labels.clone();
        Self {
            record_kind: SAVED_COLLECTION_VIEW_RECORD_KIND.to_string(),
            schema_version: SAVED_VIEW_ALPHA_SCHEMA_VERSION,
            saved_view_id: saved_view_id.into(),
            name: name.into(),
            owner_scope,
            privacy_class,
            fallback_behavior,
            scope_label: filter_ast.scope_label.clone(),
            filter_ast,
            visible_column_ids,
            pinned_column_ids,
            sort_keys: Vec::new(),
            captures_selection: false,
            captures_provider_cursor: false,
            stale_or_degraded_labels,
            created_at: created_at.clone(),
            updated_at: created_at,
        }
    }

    /// Returns a copy with sort keys attached.
    pub fn with_sort_keys(mut self, sort_keys: Vec<CollectionSortKey>) -> Self {
        self.sort_keys = sort_keys;
        self
    }

    /// Returns validation findings for saved-view portability.
    pub fn validate_portability(&self) -> Vec<CollectionValidationFinding> {
        let mut findings = Vec::new();
        if self.captures_selection {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::SavedViewCapturedTransientSelection,
                "captures_selection",
                "saved views cannot persist transient row selection",
            ));
        }
        if self.captures_provider_cursor {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::SavedViewCapturedProviderCursor,
                "captures_provider_cursor",
                "saved views cannot persist provider cursors as portable state",
            ));
        }
        if !self.privacy_class.permits_local_only_literals() && !self.filter_ast.is_portable() {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::LocalOnlyFilterLiteral,
                "filter_ast",
                "non-local saved views cannot carry local-only literals",
            ));
        }
        if self.visible_column_ids.is_empty() {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::SavedViewMissingColumns,
                "visible_column_ids",
                "saved views need explicit visible columns",
            ));
        }
        for pinned in &self.pinned_column_ids {
            if !self.visible_column_ids.contains(pinned) {
                findings.push(CollectionValidationFinding::new(
                    CollectionValidationFindingKind::SavedViewMissingPinnedColumn,
                    "pinned_column_ids",
                    format!("pinned column {pinned} must stay visible"),
                ));
            }
        }
        findings
    }

    /// True when stale or degraded labels must be rendered next to the view.
    pub fn is_degraded(&self) -> bool {
        !self.stale_or_degraded_labels.is_empty()
            || self
                .filter_ast
                .clauses()
                .iter()
                .any(|clause| clause.round_trip_state.is_degraded())
    }
}

/// Count term shown by collection counters and batch sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionCountTerm {
    /// Rows currently rendered or visible.
    Visible,
    /// Rows fetched into the client.
    Loaded,
    /// Rows matching the active filter/query across the authoritative source.
    AllMatching,
    /// Rows explicitly selected by stable identity or query scope.
    Selected,
    /// Rows included in a batch preview.
    Included,
    /// Rows excluded from a batch preview.
    Excluded,
    /// Rows blocked by policy, ownership, permission, or capability.
    Blocked,
    /// Rows hidden by any cause.
    Hidden,
    /// Rows hidden by policy.
    HiddenByPolicy,
    /// Rows hidden by the current filter or offscreen selected state.
    HiddenByFilter,
    /// Rows stale relative to the current query basis.
    Stale,
}

impl CollectionCountTerm {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Loaded => "loaded",
            Self::AllMatching => "all_matching",
            Self::Selected => "selected",
            Self::Included => "included",
            Self::Excluded => "excluded",
            Self::Blocked => "blocked",
            Self::Hidden => "hidden",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::HiddenByFilter => "hidden_by_filter",
            Self::Stale => "stale",
        }
    }
}

/// Exactness and limitation status for one collection count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionCountStatus {
    /// Count is exact for the stated term and scope.
    Exact,
    /// Count is approximate.
    Approximate,
    /// Count is narrowed or capped by provider limits.
    ProviderLimited,
    /// Count is narrowed or capped by client limits.
    ClientLimited,
    /// Count came from stale source data.
    Stale,
    /// Count came from cached source data.
    Cached,
    /// Count is known to be partial.
    Partial,
    /// Count is not known.
    Unknown,
}

impl CollectionCountStatus {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::ProviderLimited => "provider_limited",
            Self::ClientLimited => "client_limited",
            Self::Stale => "stale",
            Self::Cached => "cached",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
        }
    }
}

/// One count value with a scope term and exactness status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionCountValue {
    /// Count term.
    pub term: CollectionCountTerm,
    /// Exactness or limitation status.
    pub status: CollectionCountStatus,
    /// Count value when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<u64>,
    /// Display label for the count.
    pub label: String,
}

impl CollectionCountValue {
    /// Builds a known count value.
    pub fn known(term: CollectionCountTerm, status: CollectionCountStatus, value: u64) -> Self {
        Self {
            term,
            status,
            value: Some(value),
            label: format_count_label(term, status, Some(value)),
        }
    }

    /// Builds an unknown count value.
    pub fn unknown(term: CollectionCountTerm) -> Self {
        Self {
            term,
            status: CollectionCountStatus::Unknown,
            value: None,
            label: format_count_label(term, CollectionCountStatus::Unknown, None),
        }
    }
}

/// Counter family carried by a collection view and batch-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionScopeCounters {
    /// Count of visible rows.
    pub visible: CollectionCountValue,
    /// Count of loaded rows.
    pub loaded: CollectionCountValue,
    /// Count of all matching rows.
    pub all_matching: CollectionCountValue,
    /// Count of selected rows.
    pub selected: CollectionCountValue,
    /// Count of blocked rows.
    pub blocked: CollectionCountValue,
    /// Count of hidden rows.
    pub hidden: CollectionCountValue,
    /// Count of rows hidden by policy.
    pub hidden_by_policy: CollectionCountValue,
    /// Count of selected or matching rows hidden by filter.
    pub hidden_by_filter: CollectionCountValue,
}

impl CollectionScopeCounters {
    /// Projects existing search scope counts into the collection vocabulary.
    pub fn from_search_counts(
        counts: &SearchScopeCountsRecord,
        selected_count: u64,
        blocked_count: u64,
        hidden_selected_count: u64,
    ) -> Self {
        let status = search_count_status(counts);
        let hidden_count = counts.hidden_by_current_scope_rows
            + counts.hidden_by_policy_rows
            + counts.hidden_by_remote_cache_rows
            + hidden_selected_count;
        Self {
            visible: CollectionCountValue::known(
                CollectionCountTerm::Visible,
                CollectionCountStatus::Exact,
                counts.visible_rows,
            ),
            loaded: known_or_unknown(CollectionCountTerm::Loaded, counts.loaded_rows, status),
            all_matching: known_or_unknown(
                CollectionCountTerm::AllMatching,
                counts.all_matching_rows,
                status,
            ),
            selected: CollectionCountValue::known(
                CollectionCountTerm::Selected,
                CollectionCountStatus::Exact,
                selected_count,
            ),
            blocked: CollectionCountValue::known(
                CollectionCountTerm::Blocked,
                CollectionCountStatus::Exact,
                blocked_count,
            ),
            hidden: CollectionCountValue::known(CollectionCountTerm::Hidden, status, hidden_count),
            hidden_by_policy: CollectionCountValue::known(
                CollectionCountTerm::HiddenByPolicy,
                status,
                counts.hidden_by_policy_rows,
            ),
            hidden_by_filter: CollectionCountValue::known(
                CollectionCountTerm::HiddenByFilter,
                CollectionCountStatus::Exact,
                hidden_selected_count,
            ),
        }
    }

    /// Builds counters from explicit values for non-search collection consumers.
    #[allow(clippy::too_many_arguments)]
    pub fn from_known_values(
        visible: u64,
        loaded: u64,
        all_matching: u64,
        selected: u64,
        blocked: u64,
        hidden: u64,
        hidden_by_policy: u64,
        hidden_by_filter: u64,
        status: CollectionCountStatus,
    ) -> Self {
        Self {
            visible: CollectionCountValue::known(
                CollectionCountTerm::Visible,
                CollectionCountStatus::Exact,
                visible,
            ),
            loaded: CollectionCountValue::known(CollectionCountTerm::Loaded, status, loaded),
            all_matching: CollectionCountValue::known(
                CollectionCountTerm::AllMatching,
                status,
                all_matching,
            ),
            selected: CollectionCountValue::known(
                CollectionCountTerm::Selected,
                CollectionCountStatus::Exact,
                selected,
            ),
            blocked: CollectionCountValue::known(
                CollectionCountTerm::Blocked,
                CollectionCountStatus::Exact,
                blocked,
            ),
            hidden: CollectionCountValue::known(CollectionCountTerm::Hidden, status, hidden),
            hidden_by_policy: CollectionCountValue::known(
                CollectionCountTerm::HiddenByPolicy,
                status,
                hidden_by_policy,
            ),
            hidden_by_filter: CollectionCountValue::known(
                CollectionCountTerm::HiddenByFilter,
                CollectionCountStatus::Exact,
                hidden_by_filter,
            ),
        }
    }

    /// Returns count values in the order they should appear in support exports.
    pub fn values(&self) -> [&CollectionCountValue; 8] {
        [
            &self.visible,
            &self.loaded,
            &self.all_matching,
            &self.selected,
            &self.blocked,
            &self.hidden,
            &self.hidden_by_policy,
            &self.hidden_by_filter,
        ]
    }

    /// True when any displayed count is approximate, limited, stale, cached, partial, or unknown.
    pub fn has_non_exact_truth(&self) -> bool {
        self.values()
            .iter()
            .any(|value| value.status != CollectionCountStatus::Exact)
    }
}

/// Stable identity for one collection item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableCollectionItemRef {
    /// Stable item identity used for selection across sorting and virtualization.
    pub stable_item_id: String,
    /// Surface family that owns the item.
    pub surface_family: CollectionSurfaceFamily,
    /// Source record or provider ref for the item.
    pub source_ref: String,
    /// Redaction-aware display label for review surfaces.
    pub display_label: String,
    /// True when the item is blocked from the proposed action.
    pub blocked: bool,
    /// True when the item is hidden outside the current filter or viewport.
    pub hidden: bool,
    /// True when the item is stale relative to the current query basis.
    pub stale: bool,
}

impl StableCollectionItemRef {
    /// Builds a stable item ref with no blocked, hidden, or stale caveats.
    pub fn new(
        stable_item_id: impl Into<String>,
        surface_family: CollectionSurfaceFamily,
        source_ref: impl Into<String>,
        display_label: impl Into<String>,
    ) -> Self {
        Self {
            stable_item_id: stable_item_id.into(),
            surface_family,
            source_ref: source_ref.into(),
            display_label: display_label.into(),
            blocked: false,
            hidden: false,
            stale: false,
        }
    }

    /// Returns a copy marked as blocked.
    pub fn with_blocked(mut self, blocked: bool) -> Self {
        self.blocked = blocked;
        self
    }

    /// Returns a copy marked as hidden.
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Returns a copy marked as stale.
    pub fn with_stale(mut self, stale: bool) -> Self {
        self.stale = stale;
        self
    }
}

/// Scope class for a selection or batch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionScopeClass {
    /// Action applies only to the focused/current item.
    CurrentItemOnly,
    /// Action applies to currently rendered rows.
    VisibleRange,
    /// Action applies to rows fetched into the client.
    LoadedSet,
    /// Action applies to the authoritative query result set.
    AllMatchingQuery,
    /// Action applies to an explicit stable identity set.
    ExplicitCustomSet,
    /// Action applies to a provider-side query or selection object.
    ProviderSideQuery,
}

impl SelectionScopeClass {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentItemOnly => "current_item_only",
            Self::VisibleRange => "visible_range",
            Self::LoadedSet => "loaded_set",
            Self::AllMatchingQuery => "all_matching_query",
            Self::ExplicitCustomSet => "explicit_custom_set",
            Self::ProviderSideQuery => "provider_side_query",
        }
    }
}

/// Selection state that survives sorting, filtering, pagination, and virtualization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionSelectionState {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for selection state.
    pub schema_version: u32,
    /// Stable selection identity.
    pub selection_id: String,
    /// Collection view this selection belongs to.
    pub collection_view_id_ref: String,
    /// Selection scope class.
    pub scope_class: SelectionScopeClass,
    /// Stable selected item ids.
    pub selected_item_id_refs: Vec<String>,
    /// Optional visible range anchor item id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_item_id_ref: Option<String>,
    /// Selected item count.
    pub selected_count: u64,
    /// Selected items hidden outside the current filter or viewport.
    pub hidden_selected_count: u64,
    /// Selected items blocked for the next action.
    pub blocked_selected_count: u64,
    /// Selected items stale relative to the current query basis.
    pub stale_selected_count: u64,
    /// Query snapshot used by all-matching or provider-side selections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_snapshot_id_ref: Option<String>,
    /// True when the selection basis needs review before mutation.
    pub basis_is_stale: bool,
    /// Screen-reader and keyboard announcement text.
    pub accessibility_summary: String,
}

impl CollectionSelectionState {
    /// Builds explicit identity selection state.
    #[allow(clippy::too_many_arguments)]
    pub fn explicit_identity_set(
        selection_id: impl Into<String>,
        collection_view_id_ref: impl Into<String>,
        selected_item_id_refs: Vec<String>,
        anchor_item_id_ref: Option<String>,
        hidden_selected_count: u64,
        blocked_selected_count: u64,
        stale_selected_count: u64,
    ) -> Self {
        let selected_count = selected_item_id_refs.len() as u64;
        Self::new(
            selection_id,
            collection_view_id_ref,
            SelectionScopeClass::ExplicitCustomSet,
            selected_item_id_refs,
            anchor_item_id_ref,
            selected_count,
            hidden_selected_count,
            blocked_selected_count,
            stale_selected_count,
            None,
            stale_selected_count > 0,
        )
    }

    /// Builds selection state for a select-all scope.
    #[allow(clippy::too_many_arguments)]
    pub fn select_all_scope(
        selection_id: impl Into<String>,
        collection_view_id_ref: impl Into<String>,
        scope_class: SelectionScopeClass,
        selected_count: u64,
        hidden_selected_count: u64,
        blocked_selected_count: u64,
        stale_selected_count: u64,
        query_snapshot_id_ref: Option<String>,
    ) -> Self {
        Self::new(
            selection_id,
            collection_view_id_ref,
            scope_class,
            Vec::new(),
            None,
            selected_count,
            hidden_selected_count,
            blocked_selected_count,
            stale_selected_count,
            query_snapshot_id_ref,
            stale_selected_count > 0,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        selection_id: impl Into<String>,
        collection_view_id_ref: impl Into<String>,
        scope_class: SelectionScopeClass,
        selected_item_id_refs: Vec<String>,
        anchor_item_id_ref: Option<String>,
        selected_count: u64,
        hidden_selected_count: u64,
        blocked_selected_count: u64,
        stale_selected_count: u64,
        query_snapshot_id_ref: Option<String>,
        basis_is_stale: bool,
    ) -> Self {
        let accessibility_summary = accessibility_summary(
            selected_count,
            scope_class,
            hidden_selected_count,
            blocked_selected_count,
            basis_is_stale,
        );
        Self {
            record_kind: COLLECTION_SELECTION_STATE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_VIEW_ALPHA_SCHEMA_VERSION,
            selection_id: selection_id.into(),
            collection_view_id_ref: collection_view_id_ref.into(),
            scope_class,
            selected_item_id_refs,
            anchor_item_id_ref,
            selected_count,
            hidden_selected_count,
            blocked_selected_count,
            stale_selected_count,
            query_snapshot_id_ref,
            basis_is_stale,
            accessibility_summary,
        }
    }
}

/// Inputs for projecting a search result set as a collection view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchCollectionViewInputs {
    /// Stable collection view identity.
    pub collection_view_id: String,
    /// Search query session backing the collection.
    pub query_session: SearchQuerySession,
    /// Filter AST shown by the filter bar.
    pub filter_ast: CollectionFilterAst,
    /// Saved view applied to this collection, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_view_id_ref: Option<String>,
    /// Existing search count truth.
    pub search_counts: SearchScopeCountsRecord,
    /// Stable result identity refs currently loaded or visible.
    pub result_identity_refs: Vec<String>,
    /// Selected stable result ids.
    #[serde(default)]
    pub selected_result_ids: Vec<String>,
    /// Blocked selected result ids.
    #[serde(default)]
    pub blocked_result_ids: Vec<String>,
    /// Selected results hidden outside the current filter.
    pub hidden_selected_count: u64,
}

/// Collection view record consumed by UI, CLI, export, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionViewAlphaRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for collection view records.
    pub schema_version: u32,
    /// Stable collection view identity.
    pub collection_view_id: String,
    /// Surface family consuming the collection contract.
    pub surface_family: CollectionSurfaceFamily,
    /// Scope label rendered by the filter bar.
    pub scope_label: String,
    /// Filter AST rendered by the filter bar.
    pub filter_ast: CollectionFilterAst,
    /// Saved view applied to the collection, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_view_id_ref: Option<String>,
    /// Result and selection counters.
    pub counters: CollectionScopeCounters,
    /// Filter chips rendered by the filter bar.
    pub filter_chips: Vec<CollectionFilterChip>,
    /// Hidden narrowing labels rendered near the filter bar.
    pub hidden_narrowing_labels: Vec<String>,
    /// Selection state for the collection.
    pub selection_state: CollectionSelectionState,
    /// Stable item ids loaded or visible in the current view.
    pub item_id_refs: Vec<String>,
    /// Stable reset action id.
    pub reset_action_id: String,
    /// Stable export action id.
    pub export_action_id: String,
}

impl CollectionViewAlphaRecord {
    /// Projects a search result set into the shared collection-view contract.
    pub fn from_search_results(inputs: SearchCollectionViewInputs) -> Self {
        let selected_count = inputs.selected_result_ids.len() as u64;
        let blocked_count = inputs.blocked_result_ids.len() as u64;
        let counters = CollectionScopeCounters::from_search_counts(
            &inputs.search_counts,
            selected_count,
            blocked_count,
            inputs.hidden_selected_count,
        );
        let selection_state = CollectionSelectionState::explicit_identity_set(
            format!("selection:{}", inputs.collection_view_id),
            inputs.collection_view_id.clone(),
            inputs.selected_result_ids,
            None,
            inputs.hidden_selected_count,
            blocked_count,
            0,
        );
        let hidden_narrowing_labels = inputs.filter_ast.hidden_narrowing_labels();
        let filter_chips = inputs.filter_ast.chips();
        Self {
            record_kind: COLLECTION_VIEW_ALPHA_RECORD_KIND.to_string(),
            schema_version: COLLECTION_VIEW_ALPHA_SCHEMA_VERSION,
            collection_view_id: inputs.collection_view_id.clone(),
            surface_family: CollectionSurfaceFamily::SearchCollection,
            scope_label: inputs.query_session.scope_label,
            filter_ast: inputs.filter_ast,
            saved_view_id_ref: inputs.saved_view_id_ref,
            counters,
            filter_chips,
            hidden_narrowing_labels,
            selection_state,
            item_id_refs: inputs.result_identity_refs,
            reset_action_id: format!("collection.reset:{}", inputs.collection_view_id),
            export_action_id: format!("collection.export:{}", inputs.collection_view_id),
        }
    }

    /// Builds a collection view for non-search consumers using explicit counters.
    #[allow(clippy::too_many_arguments)]
    pub fn from_explicit_parts(
        collection_view_id: impl Into<String>,
        surface_family: CollectionSurfaceFamily,
        scope_label: impl Into<String>,
        filter_ast: CollectionFilterAst,
        saved_view_id_ref: Option<String>,
        counters: CollectionScopeCounters,
        selection_state: CollectionSelectionState,
        item_id_refs: Vec<String>,
    ) -> Self {
        let collection_view_id = collection_view_id.into();
        let hidden_narrowing_labels = filter_ast.hidden_narrowing_labels();
        let filter_chips = filter_ast.chips();
        Self {
            record_kind: COLLECTION_VIEW_ALPHA_RECORD_KIND.to_string(),
            schema_version: COLLECTION_VIEW_ALPHA_SCHEMA_VERSION,
            collection_view_id: collection_view_id.clone(),
            surface_family,
            scope_label: scope_label.into(),
            filter_ast,
            saved_view_id_ref,
            counters,
            filter_chips,
            hidden_narrowing_labels,
            selection_state,
            item_id_refs,
            reset_action_id: format!("collection.reset:{collection_view_id}"),
            export_action_id: format!("collection.export:{collection_view_id}"),
        }
    }

    /// True when the view surfaces at least one hidden narrowing label.
    pub fn surfaces_hidden_narrowing(&self) -> bool {
        !self.hidden_narrowing_labels.is_empty()
            && self.filter_chips.iter().any(|chip| chip.hidden_narrowing)
    }
}

/// Class of a batch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchActionClass {
    /// Non-mutating action with routine scope.
    RoutineNonMutating,
    /// Local reversible mutation.
    LocalReversible,
    /// Remote mutation through a connector or provider.
    RemoteMutation,
    /// Destructive local mutation.
    DestructiveMutation,
    /// Export or share action.
    ExportOrShare,
    /// Provider-owned mutation.
    ProviderOwnedMutation,
}

impl BatchActionClass {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutineNonMutating => "routine_non_mutating",
            Self::LocalReversible => "local_reversible",
            Self::RemoteMutation => "remote_mutation",
            Self::DestructiveMutation => "destructive_mutation",
            Self::ExportOrShare => "export_or_share",
            Self::ProviderOwnedMutation => "provider_owned_mutation",
        }
    }

    /// True when the action requires a review sheet before it continues.
    pub const fn requires_review_sheet(self) -> bool {
        matches!(
            self,
            Self::RemoteMutation
                | Self::DestructiveMutation
                | Self::ExportOrShare
                | Self::ProviderOwnedMutation
        )
    }
}

/// Execution origin for a batch action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchExecutionOriginClass {
    /// Execution happens locally in the client.
    ClientLocalExecution,
    /// Execution happens against provider-authoritative state.
    ProviderAuthoritativeExecution,
    /// Execution starts in the client and completes through a provider.
    MixedClientThenProvider,
}

impl BatchExecutionOriginClass {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClientLocalExecution => "client_local_execution",
            Self::ProviderAuthoritativeExecution => "provider_authoritative_execution",
            Self::MixedClientThenProvider => "mixed_client_then_provider",
        }
    }
}

/// Disposition of one item inside a batch-review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchMemberDisposition {
    /// Item will be acted on if the review continues.
    Included,
    /// Item is excluded by the selection, filter, or review choice.
    Excluded,
    /// Item is blocked and cannot be acted on.
    Blocked,
    /// Item is hidden outside the current filter or loaded window.
    Hidden,
    /// Item is stale relative to the current query basis.
    Stale,
}

impl BatchMemberDisposition {
    /// Stable token used in records, fixtures, and schemas.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::Excluded => "excluded",
            Self::Blocked => "blocked",
            Self::Hidden => "hidden",
            Self::Stale => "stale",
        }
    }
}

/// One member row shown by a batch-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewMember {
    /// Stable collection item.
    pub item: StableCollectionItemRef,
    /// Disposition in the review sheet.
    pub disposition: BatchMemberDisposition,
    /// Redaction-aware reason label.
    pub reason_label: String,
}

/// Aftermath summary preserving mixed batch outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchAftermathSummary {
    /// Count of successful item outcomes.
    pub succeeded_count: u64,
    /// Count of failed item outcomes.
    pub failed_count: u64,
    /// Count of skipped item outcomes.
    pub skipped_count: u64,
    /// Count of blocked item outcomes.
    pub blocked_count: u64,
    /// Reviewable mixed-result summary label.
    pub summary_label: String,
}

/// Review sheet shown before consequential collection batch actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchReviewSheet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for batch-review sheets.
    pub schema_version: u32,
    /// Stable batch-review identity.
    pub batch_review_id: String,
    /// Collection view this review sheet previews.
    pub collection_view_id_ref: String,
    /// Stable action id.
    pub action_id: String,
    /// Human-readable action label.
    pub action_label: String,
    /// Batch action class.
    pub action_class: BatchActionClass,
    /// Selection scope the action targets.
    pub selection_scope_class: SelectionScopeClass,
    /// Execution origin class.
    pub execution_origin_class: BatchExecutionOriginClass,
    /// Counters shown in the review sheet.
    pub counters: CollectionScopeCounters,
    /// Included item ids.
    pub included_item_id_refs: Vec<String>,
    /// Excluded item ids.
    pub excluded_item_id_refs: Vec<String>,
    /// Blocked item ids.
    pub blocked_item_id_refs: Vec<String>,
    /// Hidden item ids.
    pub hidden_item_id_refs: Vec<String>,
    /// Stale item ids.
    pub stale_item_id_refs: Vec<String>,
    /// Member rows rendered by the review sheet.
    pub members: Vec<BatchReviewMember>,
    /// Recovery or rollback guidance label.
    pub recovery_guidance: String,
    /// True when a review sheet is mandatory before continuing.
    pub review_required: bool,
    /// Stable continue action id.
    pub continue_action_id: String,
    /// Stable cancel action id.
    pub cancel_action_id: String,
    /// Optional aftermath summary after execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aftermath_summary: Option<BatchAftermathSummary>,
}

impl BatchReviewSheet {
    /// Builds a batch-review sheet from member dispositions.
    #[allow(clippy::too_many_arguments)]
    pub fn from_members(
        batch_review_id: impl Into<String>,
        collection_view_id_ref: impl Into<String>,
        action_id: impl Into<String>,
        action_label: impl Into<String>,
        action_class: BatchActionClass,
        selection_scope_class: SelectionScopeClass,
        execution_origin_class: BatchExecutionOriginClass,
        base_counters: CollectionScopeCounters,
        members: Vec<BatchReviewMember>,
        recovery_guidance: impl Into<String>,
    ) -> Self {
        let batch_review_id = batch_review_id.into();
        let collection_view_id_ref = collection_view_id_ref.into();
        let action_id = action_id.into();
        let included_item_id_refs = member_ids(&members, BatchMemberDisposition::Included);
        let excluded_item_id_refs = member_ids(&members, BatchMemberDisposition::Excluded);
        let blocked_item_id_refs = member_ids(&members, BatchMemberDisposition::Blocked);
        let hidden_item_id_refs = member_ids(&members, BatchMemberDisposition::Hidden);
        let stale_item_id_refs = member_ids(&members, BatchMemberDisposition::Stale);
        let counters = CollectionScopeCounters {
            selected: base_counters.selected.clone(),
            visible: base_counters.visible.clone(),
            loaded: base_counters.loaded.clone(),
            all_matching: base_counters.all_matching.clone(),
            hidden_by_policy: base_counters.hidden_by_policy.clone(),
            hidden_by_filter: base_counters.hidden_by_filter.clone(),
            blocked: CollectionCountValue::known(
                CollectionCountTerm::Blocked,
                CollectionCountStatus::Exact,
                base_counters
                    .blocked
                    .value
                    .unwrap_or(0)
                    .max(blocked_item_id_refs.len() as u64),
            ),
            hidden: CollectionCountValue::known(
                CollectionCountTerm::Hidden,
                if base_counters.hidden.status == CollectionCountStatus::Exact {
                    CollectionCountStatus::Exact
                } else {
                    base_counters.hidden.status
                },
                base_counters
                    .hidden
                    .value
                    .unwrap_or(0)
                    .max(hidden_item_id_refs.len() as u64),
            ),
        };
        Self {
            record_kind: BATCH_REVIEW_SHEET_RECORD_KIND.to_string(),
            schema_version: COLLECTION_VIEW_ALPHA_SCHEMA_VERSION,
            batch_review_id: batch_review_id.clone(),
            collection_view_id_ref,
            action_id: action_id.clone(),
            action_label: action_label.into(),
            action_class,
            selection_scope_class,
            execution_origin_class,
            counters,
            included_item_id_refs,
            excluded_item_id_refs,
            blocked_item_id_refs,
            hidden_item_id_refs,
            stale_item_id_refs,
            members,
            recovery_guidance: recovery_guidance.into(),
            review_required: action_class.requires_review_sheet(),
            continue_action_id: format!("batch_review.continue:{batch_review_id}:{action_id}"),
            cancel_action_id: format!("batch_review.cancel:{batch_review_id}:{action_id}"),
            aftermath_summary: None,
        }
    }

    /// Returns a copy with an aftermath summary attached.
    pub fn with_aftermath_summary(mut self, aftermath_summary: BatchAftermathSummary) -> Self {
        self.aftermath_summary = Some(aftermath_summary);
        self
    }

    /// Returns validation findings for protected batch-review semantics.
    pub fn validate(&self) -> Vec<CollectionValidationFinding> {
        let mut findings = Vec::new();
        if self.action_class.requires_review_sheet() && !self.review_required {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::BatchReviewRequiredButMissing,
                "review_required",
                "protected batch actions require review sheets",
            ));
        }
        if self.selection_scope_class == SelectionScopeClass::AllMatchingQuery
            && self.counters.all_matching.status == CollectionCountStatus::Unknown
        {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::AllMatchingScopeLacksCountTruth,
                "counters.all_matching",
                "all-matching actions need exact, approximate, or limited count truth",
            ));
        }
        if self.action_class.requires_review_sheet()
            && self.included_item_id_refs.is_empty()
            && self.blocked_item_id_refs.is_empty()
            && self.hidden_item_id_refs.is_empty()
            && self.excluded_item_id_refs.is_empty()
        {
            findings.push(CollectionValidationFinding::new(
                CollectionValidationFindingKind::BatchReviewHasNoMembers,
                "members",
                "batch review sheets need included, excluded, blocked, hidden, or stale members",
            ));
        }
        findings
    }
}

/// Validation finding kind for collection artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionValidationFindingKind {
    /// Hidden narrowing exists but is not represented as visible state.
    HiddenNarrowingNotSurfaced,
    /// Filter literal is local-only where portable state is required.
    LocalOnlyFilterLiteral,
    /// Saved view captured transient selection.
    SavedViewCapturedTransientSelection,
    /// Saved view captured a provider cursor.
    SavedViewCapturedProviderCursor,
    /// Saved view lacks explicit visible columns.
    SavedViewMissingColumns,
    /// Saved view pinned a column that is no longer visible.
    SavedViewMissingPinnedColumn,
    /// Protected batch action lacks required review state.
    BatchReviewRequiredButMissing,
    /// All-matching batch scope lacks count truth.
    AllMatchingScopeLacksCountTruth,
    /// Batch review sheet has no member facts.
    BatchReviewHasNoMembers,
}

impl CollectionValidationFindingKind {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HiddenNarrowingNotSurfaced => "hidden_narrowing_not_surfaced",
            Self::LocalOnlyFilterLiteral => "local_only_filter_literal",
            Self::SavedViewCapturedTransientSelection => "saved_view_captured_transient_selection",
            Self::SavedViewCapturedProviderCursor => "saved_view_captured_provider_cursor",
            Self::SavedViewMissingColumns => "saved_view_missing_columns",
            Self::SavedViewMissingPinnedColumn => "saved_view_missing_pinned_column",
            Self::BatchReviewRequiredButMissing => "batch_review_required_but_missing",
            Self::AllMatchingScopeLacksCountTruth => "all_matching_scope_lacks_count_truth",
            Self::BatchReviewHasNoMembers => "batch_review_has_no_members",
        }
    }
}

/// Structured validation finding for collection artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionValidationFinding {
    /// Stable finding kind.
    pub finding_kind: CollectionValidationFindingKind,
    /// Field or record section that failed validation.
    pub field: String,
    /// Short support-safe explanation.
    pub summary: String,
}

impl CollectionValidationFinding {
    fn new(
        finding_kind: CollectionValidationFindingKind,
        field: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            field: field.into(),
            summary: summary.into(),
        }
    }
}

fn search_count_status(counts: &SearchScopeCountsRecord) -> CollectionCountStatus {
    if !counts.readiness_is_ready {
        return CollectionCountStatus::Partial;
    }
    if counts.hidden_by_remote_cache_rows > 0 {
        return CollectionCountStatus::ProviderLimited;
    }
    if counts.counts_class_token == SearchScopeCountsClass::GloballyAuthoritative.as_str() {
        CollectionCountStatus::Exact
    } else {
        CollectionCountStatus::Partial
    }
}

fn known_or_unknown(
    term: CollectionCountTerm,
    value: Option<u64>,
    status: CollectionCountStatus,
) -> CollectionCountValue {
    value.map_or_else(
        || CollectionCountValue::unknown(term),
        |value| CollectionCountValue::known(term, status, value),
    )
}

fn format_count_label(
    term: CollectionCountTerm,
    status: CollectionCountStatus,
    value: Option<u64>,
) -> String {
    let term_label = term.as_str().replace('_', " ");
    match (status, value) {
        (CollectionCountStatus::Unknown, None) => format!("{term_label} unknown"),
        (CollectionCountStatus::Approximate, Some(value)) => format!("~{value} {term_label}"),
        (_, Some(value)) => format!("{value} {term_label}"),
        (_, None) => format!("{term_label} unknown"),
    }
}

fn accessibility_summary(
    selected_count: u64,
    scope_class: SelectionScopeClass,
    hidden_selected_count: u64,
    blocked_selected_count: u64,
    basis_is_stale: bool,
) -> String {
    let mut summary = format!(
        "{selected_count} selected; scope {}; {hidden_selected_count} hidden selected; {blocked_selected_count} blocked",
        scope_class.as_str()
    );
    if basis_is_stale {
        summary.push_str("; selection basis stale");
    }
    summary
}

fn member_ids(members: &[BatchReviewMember], disposition: BatchMemberDisposition) -> Vec<String> {
    members
        .iter()
        .filter(|member| member.disposition == disposition)
        .map(|member| member.item.stable_item_id.clone())
        .collect()
}

/// Returns the schema version of the scope-count source consumed by collection counters.
pub const fn consumed_scope_counts_schema_version() -> u32 {
    SCOPE_TRUTH_COUNTS_SCHEMA_VERSION
}
