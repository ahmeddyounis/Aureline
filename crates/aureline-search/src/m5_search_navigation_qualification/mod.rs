//! M5 search-query, result-identity, ranking-reason, and saved-query/privacy
//! qualification.
//!
//! This module freezes the already-landed M5 search and navigation foundations
//! into one qualification index that the product search surfaces, CLI/headless
//! emitter, docs/help, support export, shiproom, and release-manifest consumers
//! can ingest verbatim. It mints no new retrieval behavior; it binds the
//! canonical search contract objects — [`SearchContractObjectClass`] —, the
//! closed result-state vocabulary — [`ResultStateClass`] —, and the privacy /
//! retention / consent posture for query material into one shared decision
//! surface, then proves that every claimed M5 search surface answers off the
//! same query-session and result-identity model.
//!
//! The packet answers, for every claimed M5 search/navigation surface:
//!
//! - which surface is being qualified and which deployment modes (product
//!   surface, CLI/headless) it must cover;
//! - that it references the one shared [`SearchQuerySession`] and result-identity
//!   model rather than a surface-local heuristic;
//! - which canonical contract objects it binds — each citing its *own* lane
//!   schema, fixture corpus, and record kind — and which result-state vocabulary
//!   tokens it can express;
//! - the privacy class governing its query text so raw text stays local-only by
//!   default and saved-query/export material never widens silently; and
//! - the published qualification state plus the stale-proof tokens and
//!   downgrade-rule ids that explain any narrowing.
//!
//! Rows that lose their shared-model anchor, run on a partial or stale index, or
//! cannot prove consent for query material narrow automatically instead of
//! masquerading as whole-workspace certainty, and a surface that persists or
//! exports query material may never publish a row that demotes local-only query
//! text below a sync or export path.
//!
//! [`SearchQuerySession`]: crate::query_session::SearchQuerySession

use serde::{Deserialize, Serialize};

use crate::query_artifacts::{
    SAVED_QUERY_RECORD_KIND, SAVED_QUERY_SCHEMA_REF, SCOPE_PACK_BINDING_RECORD_KIND,
    SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF,
};
use crate::query_session::{SearchQuerySession, SEARCH_QUERY_SESSION_SCHEMA_VERSION};
use crate::ranking_reason::{
    SEARCH_OPERATOR_TRUTH_FIXTURE_DIR, SEARCH_OPERATOR_TRUTH_PACKET_RECORD_KIND,
    SEARCH_OPERATOR_TRUTH_SCHEMA_REF,
};
use crate::result_truth_packet::{
    SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR, SEARCH_RESULT_TRUTH_PACKET_RECORD_KIND,
    SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
};

// Canonical lane refs that have no exported constant of their own but are stable
// checked-in paths. Quoting them here keeps a row's own-proof binding honest
// even where the owning lane never minted a `*_REF` constant.
const QUERY_SESSION_SCHEMA_REF: &str = "schemas/search/query_session.schema.json";
const QUERY_SESSION_FIXTURE_DIR: &str = "fixtures/search/query_session_cases";
const SCOPE_PACK_SCHEMA_REF: &str = "schemas/search/saved_query_and_scope_binding.schema.json";
const SAVED_QUERY_PRIVACY_FIXTURE_DIR: &str = "fixtures/search/m3/saved_query_privacy";
const SEARCH_EXPORT_PACKET_RECORD_KIND: &str = "search_export_packet";

// Checked consumer surfaces that must ingest the qualification index verbatim.
const PRODUCT_SEARCH_CONSUMER_REF: &str = "docs/search/result_identity_and_ranking.md";
const SUPPORT_EXPORT_CONSUMER_REF: &str = "schemas/search/search_export_snapshot.schema.json";
const RELEASE_MANIFEST_CONSUMER_REF: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";

const REQUIRED_PROJECTION_FIELDS: &[&str] = &[
    "qualification_row_id",
    "surface",
    "published_state",
    "deployment_mode_coverage",
    "shared_query_session_ref",
    "shared_result_identity_ref",
    "expressible_states",
    "stale_proof_tokens",
    "downgrade_rule_ids",
];

/// Stable record-kind tag carried by [`M5SearchNavigationQualificationPacket`].
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_RECORD_KIND: &str =
    "m5_search_navigation_qualification_packet";

/// Frozen schema version for the M5 search/navigation qualification packet.
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/search/m5-search-navigation-qualification.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF: &str =
    "docs/search/m5-search-navigation-qualification.md";

/// Repository-relative path of the checked review artifact.
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/search/m5/m5-search-navigation-qualification.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/search/m5/m5-search-navigation-qualification";

/// Stable packet identifier reused by every consumer binding.
pub const M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_ID: &str =
    "search.m5.search_navigation_qualification.v1";

/// One claimed M5 search/navigation surface the qualification certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSurfaceClass {
    /// Quick-open file, recent-place, and symbol jump surface.
    QuickOpen,
    /// Full workspace file and text-search surface.
    FileSearch,
    /// Symbol and structural-navigation search surface.
    SymbolSearch,
    /// Documentation and help search surface.
    DocsSearch,
    /// Graph-backed (definitions, references, topology) search surface.
    GraphBackedSearch,
    /// AI context-retrieval and context-picker surface.
    AiContextRetrieval,
    /// Saved-query reopen and replay surface.
    SavedQueryReopen,
    /// Search-result export and support-handoff surface.
    SearchExport,
}

impl SearchSurfaceClass {
    /// All claimed search/navigation surfaces in canonical order.
    pub const ALL: [Self; 8] = [
        Self::QuickOpen,
        Self::FileSearch,
        Self::SymbolSearch,
        Self::DocsSearch,
        Self::GraphBackedSearch,
        Self::AiContextRetrieval,
        Self::SavedQueryReopen,
        Self::SearchExport,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuickOpen => "quick_open",
            Self::FileSearch => "file_search",
            Self::SymbolSearch => "symbol_search",
            Self::DocsSearch => "docs_search",
            Self::GraphBackedSearch => "graph_backed_search",
            Self::AiContextRetrieval => "ai_context_retrieval",
            Self::SavedQueryReopen => "saved_query_reopen",
            Self::SearchExport => "search_export",
        }
    }

    /// Returns a review-safe label for the surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QuickOpen => "Quick open",
            Self::FileSearch => "File search",
            Self::SymbolSearch => "Symbol search",
            Self::DocsSearch => "Docs search",
            Self::GraphBackedSearch => "Graph-backed search",
            Self::AiContextRetrieval => "AI context retrieval",
            Self::SavedQueryReopen => "Saved-query reopen",
            Self::SearchExport => "Search export",
        }
    }

    /// True when the surface persists or exports query material and must keep
    /// local-only query text first-class beside any sync or export path.
    pub const fn persists_query_material(self) -> bool {
        matches!(self, Self::SavedQueryReopen | Self::SearchExport)
    }

    /// True when the surface answers off the live index and so narrows when the
    /// index is partial or stale; durable-artifact surfaces replay a captured
    /// snapshot and label its freshness instead of masquerading as fresh.
    pub const fn depends_on_live_index(self) -> bool {
        !self.persists_query_material()
    }

    /// Result-state vocabulary tokens the surface can express, in canonical
    /// order.
    fn expressible_states(self) -> Vec<ResultStateClass> {
        use ResultStateClass::*;
        match self {
            Self::QuickOpen => vec![
                Exact,
                ContextPromoted,
                PartialIndex,
                WithheldLatency,
                PolicyHidden,
                Cached,
                Stale,
            ],
            Self::FileSearch
            | Self::SymbolSearch
            | Self::DocsSearch
            | Self::GraphBackedSearch
            | Self::AiContextRetrieval => vec![
                Exact,
                ContextPromoted,
                Semantic,
                PartialIndex,
                WithheldLatency,
                PolicyHidden,
                Cached,
                Stale,
            ],
            Self::SavedQueryReopen | Self::SearchExport => vec![
                Exact,
                ContextPromoted,
                Semantic,
                PartialIndex,
                WithheldLatency,
                PolicyHidden,
                Cached,
                Stale,
                Imported,
            ],
        }
    }

    /// Canonical contract objects the surface binds, in canonical order. Every
    /// surface binds the shared query session and result identity; richer
    /// surfaces add ranking, action, saved-query, scope-pack, and export objects.
    fn bound_objects(self) -> Vec<SearchContractObjectClass> {
        use SearchContractObjectClass::*;
        match self {
            Self::QuickOpen => vec![QuerySession, ResultRef, RankingReason, ActionBinding],
            Self::FileSearch | Self::SymbolSearch | Self::DocsSearch | Self::GraphBackedSearch => {
                vec![
                    QuerySession,
                    ResultRef,
                    RankingReason,
                    ActionBinding,
                    ScopePack,
                ]
            }
            Self::AiContextRetrieval => {
                vec![QuerySession, ResultRef, RankingReason, ScopePack]
            }
            Self::SavedQueryReopen => vec![
                QuerySession,
                ResultRef,
                RankingReason,
                ActionBinding,
                SavedQuery,
                ScopePack,
            ],
            Self::SearchExport => vec![
                QuerySession,
                ResultRef,
                RankingReason,
                ActionBinding,
                SavedQuery,
                ScopePack,
                ExportPacket,
            ],
        }
    }
}

/// One frozen canonical search contract object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchContractObjectClass {
    /// The durable query-session record minted before rerank.
    QuerySession,
    /// The stable result-identity reference that survives virtualization.
    ResultRef,
    /// The structured ranking-reason explanation object.
    RankingReason,
    /// The explicit search action-binding object.
    ActionBinding,
    /// The durable saved-query artifact.
    SavedQuery,
    /// The scope-pack binding artifact.
    ScopePack,
    /// The search-result export packet.
    ExportPacket,
}

impl SearchContractObjectClass {
    /// All canonical contract objects in canonical order.
    pub const ALL: [Self; 7] = [
        Self::QuerySession,
        Self::ResultRef,
        Self::RankingReason,
        Self::ActionBinding,
        Self::SavedQuery,
        Self::ScopePack,
        Self::ExportPacket,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuerySession => "query_session",
            Self::ResultRef => "result_ref",
            Self::RankingReason => "ranking_reason",
            Self::ActionBinding => "action_binding",
            Self::SavedQuery => "saved_query",
            Self::ScopePack => "scope_pack",
            Self::ExportPacket => "export_packet",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QuerySession => "SearchQuerySession",
            Self::ResultRef => "SearchResultRef",
            Self::RankingReason => "RankingReason",
            Self::ActionBinding => "SearchActionBinding",
            Self::SavedQuery => "SavedQuery",
            Self::ScopePack => "ScopePack",
            Self::ExportPacket => "SearchExportPacket",
        }
    }

    /// Stable object identifier reused by surface bindings.
    fn object_id(self) -> String {
        format!("object:{}", self.as_str())
    }

    /// Returns the canonical lane refs that back the object's own proof.
    fn backing_refs(self) -> ObjectBackingRefs {
        match self {
            Self::QuerySession => ObjectBackingRefs {
                schema_ref: QUERY_SESSION_SCHEMA_REF,
                fixture_ref: QUERY_SESSION_FIXTURE_DIR,
                record_kind: SearchQuerySession::RECORD_KIND,
            },
            // SearchResultRef and SearchActionBinding are members of the one
            // result-truth packet, so they cite the same boundary proof.
            Self::ResultRef | Self::ActionBinding => ObjectBackingRefs {
                schema_ref: SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
                fixture_ref: SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR,
                record_kind: SEARCH_RESULT_TRUTH_PACKET_RECORD_KIND,
            },
            Self::RankingReason => ObjectBackingRefs {
                schema_ref: SEARCH_OPERATOR_TRUTH_SCHEMA_REF,
                fixture_ref: SEARCH_OPERATOR_TRUTH_FIXTURE_DIR,
                record_kind: SEARCH_OPERATOR_TRUTH_PACKET_RECORD_KIND,
            },
            Self::SavedQuery => ObjectBackingRefs {
                schema_ref: SAVED_QUERY_SCHEMA_REF,
                fixture_ref: SAVED_QUERY_PRIVACY_FIXTURE_DIR,
                record_kind: SAVED_QUERY_RECORD_KIND,
            },
            Self::ScopePack => ObjectBackingRefs {
                schema_ref: SCOPE_PACK_SCHEMA_REF,
                fixture_ref: SAVED_QUERY_PRIVACY_FIXTURE_DIR,
                record_kind: SCOPE_PACK_BINDING_RECORD_KIND,
            },
            Self::ExportPacket => ObjectBackingRefs {
                schema_ref: SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF,
                fixture_ref: SAVED_QUERY_PRIVACY_FIXTURE_DIR,
                record_kind: SEARCH_EXPORT_PACKET_RECORD_KIND,
            },
        }
    }

    /// The privacy data class that governs the object's most-sensitive field.
    fn privacy_data_class(self) -> PrivacyDataClass {
        match self {
            Self::QuerySession => PrivacyDataClass::RawQueryText,
            Self::ResultRef | Self::RankingReason | Self::ActionBinding => {
                PrivacyDataClass::QueryHash
            }
            Self::SavedQuery | Self::ScopePack => PrivacyDataClass::SavedQuerySync,
            Self::ExportPacket => PrivacyDataClass::SupportExportPacket,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ObjectBackingRefs {
    schema_ref: &'static str,
    fixture_ref: &'static str,
    record_kind: &'static str,
}

/// Deployment mode a search surface must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentMode {
    /// The product (desktop) search surface.
    ProductSurface,
    /// The CLI / headless search emitter.
    CliHeadless,
}

impl DeploymentMode {
    /// All deployment modes in canonical order.
    pub const ALL: [Self; 2] = [Self::ProductSurface, Self::CliHeadless];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductSurface => "product_surface",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// One member of the closed result-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultStateClass {
    /// An exact lexical or path match.
    Exact,
    /// A row promoted by recency, locality, or context signals.
    ContextPromoted,
    /// A semantic / embedding-derived match.
    Semantic,
    /// A row served from a partial (still-warming) index.
    PartialIndex,
    /// A candidate withheld to stay within a latency budget.
    WithheldLatency,
    /// A candidate hidden by policy or scope rules.
    PolicyHidden,
    /// A row served from a cached result set.
    Cached,
    /// A row served from a stale index or snapshot.
    Stale,
    /// A row imported from an external or saved artifact.
    Imported,
}

impl ResultStateClass {
    /// All controlled-vocabulary states in canonical order.
    pub const ALL: [Self; 9] = [
        Self::Exact,
        Self::ContextPromoted,
        Self::Semantic,
        Self::PartialIndex,
        Self::WithheldLatency,
        Self::PolicyHidden,
        Self::Cached,
        Self::Stale,
        Self::Imported,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ContextPromoted => "context_promoted",
            Self::Semantic => "semantic",
            Self::PartialIndex => "partial_index",
            Self::WithheldLatency => "withheld_latency",
            Self::PolicyHidden => "policy_hidden",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Imported => "imported",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::ContextPromoted => "Context-promoted",
            Self::Semantic => "Semantic",
            Self::PartialIndex => "Partial-index",
            Self::WithheldLatency => "Withheld-latency",
            Self::PolicyHidden => "Policy-hidden",
            Self::Cached => "Cached",
            Self::Stale => "Stale",
            Self::Imported => "Imported",
        }
    }

    /// True when the state requires a narrowed (non-whole-workspace) claim — the
    /// row may not imply complete coverage while it holds.
    pub const fn narrows_scope(self) -> bool {
        matches!(
            self,
            Self::PartialIndex
                | Self::WithheldLatency
                | Self::PolicyHidden
                | Self::Cached
                | Self::Stale
                | Self::Imported
        )
    }

    /// True when the state must stay visible to the user and downstream
    /// consumers rather than being silently collapsed into a generic result.
    pub const fn must_stay_visible(self) -> bool {
        // Partial, withheld, blocked, stale, and imported states stay visible;
        // the three positive match classes do not need a disclosure cue.
        self.narrows_scope()
    }
}

/// One privacy-bound query-material data class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyDataClass {
    /// Raw query text the user typed.
    RawQueryText,
    /// Deterministic, non-reversible query hashes.
    QueryHash,
    /// Saved-query / scope-pack material eligible for sync.
    SavedQuerySync,
    /// Support / export packets carrying query material.
    SupportExportPacket,
}

impl PrivacyDataClass {
    /// All privacy-bound data classes in canonical order.
    pub const ALL: [Self; 4] = [
        Self::RawQueryText,
        Self::QueryHash,
        Self::SavedQuerySync,
        Self::SupportExportPacket,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawQueryText => "raw_query_text",
            Self::QueryHash => "query_hash",
            Self::SavedQuerySync => "saved_query_sync",
            Self::SupportExportPacket => "support_export_packet",
        }
    }
}

/// Privacy class assigned to a data class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyClass {
    /// Local, sensitive material that never leaves the device by default.
    LocalSensitive,
    /// Local, derived metadata (e.g. a query hash).
    LocalDerived,
    /// User-synced material admitted by explicit opt-in.
    UserSynced,
    /// Export-scoped, redacted metadata.
    ExportMetadata,
}

impl PrivacyClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSensitive => "local_sensitive",
            Self::LocalDerived => "local_derived",
            Self::UserSynced => "user_synced",
            Self::ExportMetadata => "export_metadata",
        }
    }
}

/// Retention mode assigned to a data class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionMode {
    /// Retained locally, never remotely retained by default.
    LocalOnlyDefault,
    /// Only a deterministic hash is retained locally.
    LocalHashOnly,
    /// Retained synced only after explicit opt-in.
    ExplicitSyncOptIn,
    /// Retained only inside a redacted support export.
    SupportExportBounded,
}

impl RetentionMode {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyDefault => "local_only_default",
            Self::LocalHashOnly => "local_hash_only",
            Self::ExplicitSyncOptIn => "explicit_sync_opt_in",
            Self::SupportExportBounded => "support_export_bounded",
        }
    }
}

/// Consent requirement assigned to a data class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentRequirement {
    /// No consent required for local-only retention.
    NoneLocalDefault,
    /// Explicit consent required before the material may be shared.
    ExplicitForShare,
    /// Explicit opt-in required before the material may sync.
    ExplicitSyncOptIn,
    /// Explicit consent required for each export.
    ExplicitPerExport,
}

impl ConsentRequirement {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneLocalDefault => "none_local_default",
            Self::ExplicitForShare => "explicit_for_share",
            Self::ExplicitSyncOptIn => "explicit_sync_opt_in",
            Self::ExplicitPerExport => "explicit_per_export",
        }
    }
}

/// Qualification result published for one search surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationStateClass {
    /// The surface references the shared model, all states are expressible, and
    /// the privacy posture is intact on a fresh index.
    Qualified,
    /// The surface keeps a narrower, scope-limited claim only (partial index,
    /// stale, cached, withheld, or imported) and must not imply whole-workspace
    /// certainty.
    ScopeLimited,
    /// Only the local-only query-text path may be claimed; any sync or export of
    /// query material is unverified pending consent.
    LocalQueryTextOnly,
    /// The broad surface claim is blocked pending fresh proof.
    BlockedUnverified,
}

impl QualificationStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ScopeLimited => "scope_limited",
            Self::LocalQueryTextOnly => "local_query_text_only",
            Self::BlockedUnverified => "blocked_unverified",
        }
    }
}

/// Downgrade trigger automated by the qualification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDowngradeTriggerClass {
    /// A surface stopped anchoring to the shared query-session / result-identity
    /// model, or its result IDs stopped surviving virtualization and churn.
    SharedModelDrift,
    /// The surface answers off a partial or stale index and may not claim
    /// whole-workspace coverage.
    PartialIndexOrStaleScope,
    /// Candidates were withheld for latency or hidden by policy and the row may
    /// not imply complete results.
    WithheldOrPolicyHidden,
    /// Raw query text, saved-query sync, or export of query material lacks
    /// consent.
    QueryTextPrivacyUnconsented,
    /// An imported saved-query or scope-pack provenance is unverified.
    ImportedProvenanceUnverified,
    /// One downstream consumer stopped ingesting the qualification by reference.
    ConsumerBindingMissing,
}

impl QualificationDowngradeTriggerClass {
    /// All downgrade triggers in canonical order.
    pub const ALL: [Self; 6] = [
        Self::SharedModelDrift,
        Self::PartialIndexOrStaleScope,
        Self::WithheldOrPolicyHidden,
        Self::QueryTextPrivacyUnconsented,
        Self::ImportedProvenanceUnverified,
        Self::ConsumerBindingMissing,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedModelDrift => "shared_model_drift",
            Self::PartialIndexOrStaleScope => "partial_index_or_stale_scope",
            Self::WithheldOrPolicyHidden => "withheld_or_policy_hidden",
            Self::QueryTextPrivacyUnconsented => "query_text_privacy_unconsented",
            Self::ImportedProvenanceUnverified => "imported_provenance_unverified",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }
}

/// Stable consumer surface that ingests the qualification result verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationConsumerClass {
    /// The product (desktop) search surface and result pane.
    ProductSearchSurface,
    /// CLI / headless search output.
    CliHeadless,
    /// Docs/help search and discoverability surfaces.
    DocsHelp,
    /// Support-export and handoff surfaces.
    SupportExport,
    /// Shiproom claim and operational-readiness packets.
    Shiproom,
    /// Release manifest and publication control surfaces.
    ReleaseManifest,
}

impl QualificationConsumerClass {
    /// All consumer surfaces in canonical order.
    pub const ALL: [Self; 6] = [
        Self::ProductSearchSurface,
        Self::CliHeadless,
        Self::DocsHelp,
        Self::SupportExport,
        Self::Shiproom,
        Self::ReleaseManifest,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductSearchSurface => "product_search_surface",
            Self::CliHeadless => "cli_headless",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::Shiproom => "shiproom",
            Self::ReleaseManifest => "release_manifest",
        }
    }
}

/// One canonical contract-object catalog row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchContractObjectRow {
    /// Stable object identifier.
    pub object_id: String,
    /// Contract object class.
    pub object_class: SearchContractObjectClass,
    /// Human-readable object label.
    pub label: String,
    /// Lane boundary schema backing the object's own proof.
    pub backing_schema_ref: String,
    /// Lane fixture corpus backing the object's own proof.
    pub backing_fixture_ref: String,
    /// Lane record kind backing the object's own proof.
    pub backing_record_kind: String,
    /// Privacy data class governing the object's most-sensitive field.
    pub privacy_data_class: PrivacyDataClass,
    /// Review-safe summary.
    pub summary: String,
}

/// One controlled-vocabulary result-state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultStateRow {
    /// State class.
    pub state_class: ResultStateClass,
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// True when the state requires a narrowed claim.
    pub narrows_scope: bool,
    /// True when the state must stay visible to the user and consumers.
    pub must_stay_visible: bool,
    /// Review-safe summary.
    pub summary: String,
}

/// One privacy / retention / consent binding for a query-material data class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyBindingRow {
    /// Data class governed by the binding.
    pub data_class: PrivacyDataClass,
    /// Privacy class applied to the data class.
    pub privacy_class: PrivacyClass,
    /// Retention mode applied to the data class.
    pub retention_mode: RetentionMode,
    /// Consent requirement applied to the data class.
    pub consent_requirement: ConsentRequirement,
    /// True when the data class stays local-only unless explicitly admitted.
    pub local_only_by_default: bool,
    /// Review-safe summary.
    pub summary: String,
}

/// One search-surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceQualificationRow {
    /// Stable row identifier.
    pub qualification_row_id: String,
    /// Search surface covered by the row.
    pub surface: SearchSurfaceClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Published qualification state for the row.
    pub published_state: QualificationStateClass,
    /// Deployment modes the surface covers.
    pub deployment_mode_coverage: Vec<DeploymentMode>,
    /// Shared query-session model schema this surface references.
    pub shared_query_session_ref: String,
    /// Shared result-identity model schema this surface references.
    pub shared_result_identity_ref: String,
    /// Canonical contract object ids the surface binds.
    pub bound_object_ids: Vec<String>,
    /// Result-state vocabulary tokens the surface can express.
    pub expressible_states: Vec<ResultStateClass>,
    /// Privacy class governing the surface's query text.
    pub query_text_privacy_class: PrivacyClass,
    /// True when the surface persists or exports query material.
    pub persists_query_material: bool,
    /// True when local-only query text stays first-class beside any sync/export.
    pub local_query_text_first: bool,
    /// Active stale or capability-loss tokens narrowing the row.
    pub stale_proof_tokens: Vec<String>,
    /// Active downgrade-rule identifiers explaining the published state.
    pub downgrade_rule_ids: Vec<String>,
    /// Review-safe summary for downstream surfaces.
    pub summary: String,
}

/// One downgrade rule published by the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationDowngradeRuleRow {
    /// Stable rule identifier.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger_class: QualificationDowngradeTriggerClass,
    /// Source qualification state before the downgrade.
    pub source_state: QualificationStateClass,
    /// Resulting qualification state after the downgrade.
    pub downgraded_state: QualificationStateClass,
    /// User-visible effect of the downgrade.
    pub required_effect: String,
    /// Reviewable rationale for the downgrade.
    pub rationale: String,
    /// Supporting evidence or contract refs used to inspect the rule.
    pub evidence_refs: Vec<String>,
}

/// One consumer-surface binding proving the same qualification result is reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationConsumerBinding {
    /// Consumer surface that ingests the qualification.
    pub consumer: QualificationConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// Number of qualification rows the consumer exposes by reference.
    pub qualification_row_count: usize,
    /// Fields the consumer must preserve verbatim from the packet.
    pub required_verbatim_fields: Vec<String>,
    /// True when the consumer narrows immediately on stale proof or blocked rows.
    pub narrow_on_stale_proof: bool,
    /// True when limited or local-only states stay labeled explicitly.
    pub explicit_limited_state_labels_required: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

/// One validation error returned by
/// [`M5SearchNavigationQualificationPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SearchNavigationQualificationViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 search/navigation qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SearchNavigationQualificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing lane schemas this qualification composes.
    pub supporting_contract_refs: Vec<String>,
    /// Claimed M5 search surfaces covered by the packet.
    pub claimed_surfaces: Vec<SearchSurfaceClass>,
    /// Deployment modes every surface must cover.
    pub deployment_modes: Vec<DeploymentMode>,
    /// Canonical contract object catalog.
    pub contract_objects: Vec<SearchContractObjectRow>,
    /// Closed result-state vocabulary.
    pub result_state_vocabulary: Vec<ResultStateRow>,
    /// Privacy / retention / consent bindings for query material.
    pub privacy_bindings: Vec<PrivacyBindingRow>,
    /// Canonical search-surface qualification rows.
    pub qualification_rows: Vec<SurfaceQualificationRow>,
    /// Automatic downgrade rules used by the packet.
    pub downgrade_rules: Vec<QualificationDowngradeRuleRow>,
    /// Consumer-surface bindings that prove one qualification index is reused.
    pub consumer_bindings: Vec<QualificationConsumerBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5SearchNavigationQualificationPacket {
    /// Validates surface coverage, shared-model anchoring, object catalog,
    /// vocabulary completeness, privacy bindings, downgrade automation, and
    /// shared-consumer bindings.
    pub fn validate(&self) -> Vec<M5SearchNavigationQualificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_RECORD_KIND {
            push(&mut violations, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_VERSION {
            push(
                &mut violations,
                "schema_version",
                "unexpected schema_version",
            );
        }
        if self.packet_id != M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_ID {
            push(&mut violations, "packet_id", "unexpected packet_id");
        }
        if self.doc_ref != M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF {
            push(
                &mut violations,
                "doc_ref",
                "packet must quote the canonical reviewer doc",
            );
        }
        if self.schema_ref != M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF {
            push(
                &mut violations,
                "schema_ref",
                "packet must quote the canonical schema ref",
            );
        }
        if self.artifact_ref != M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF {
            push(
                &mut violations,
                "artifact_ref",
                "packet must quote the checked review artifact ref",
            );
        }
        if self.source_spec_refs.is_empty() {
            push(
                &mut violations,
                "source_spec_refs",
                "packet must quote at least one authoritative spec ref",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut violations,
                "supporting_contract_refs",
                "packet must cite the composed lane contracts",
            );
        }

        for required in SearchSurfaceClass::ALL {
            if !self.claimed_surfaces.contains(&required) {
                push(
                    &mut violations,
                    "claimed_surfaces",
                    &format!("missing claimed surface {}", required.as_str()),
                );
            }
        }
        for required in DeploymentMode::ALL {
            if !self.deployment_modes.contains(&required) {
                push(
                    &mut violations,
                    "deployment_modes",
                    &format!("missing deployment mode {}", required.as_str()),
                );
            }
        }

        self.validate_contract_objects(&mut violations);
        self.validate_vocabulary(&mut violations);
        self.validate_privacy_bindings(&mut violations);

        for surface in SearchSurfaceClass::ALL {
            if !self
                .qualification_rows
                .iter()
                .any(|row| row.surface == surface)
            {
                push(
                    &mut violations,
                    "qualification_rows",
                    &format!("missing qualification row for surface {}", surface.as_str()),
                );
            }
        }

        let rule_ids: Vec<&str> = self
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        let object_ids: Vec<&str> = self
            .contract_objects
            .iter()
            .map(|object| object.object_id.as_str())
            .collect();

        for row in &self.qualification_rows {
            self.validate_row(&mut violations, row, &rule_ids, &object_ids);
        }

        for required in QualificationDowngradeTriggerClass::ALL {
            if !self
                .downgrade_rules
                .iter()
                .any(|rule| rule.trigger_class == required)
            {
                push(
                    &mut violations,
                    "downgrade_rules",
                    &format!("missing downgrade trigger {}", required.as_str()),
                );
            }
        }
        for rule in &self.downgrade_rules {
            if rule.evidence_refs.is_empty() {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "downgrade rule must cite at least one evidence ref",
                );
            }
        }

        self.validate_consumer_bindings(&mut violations);

        violations
    }

    fn validate_contract_objects(
        &self,
        violations: &mut Vec<M5SearchNavigationQualificationViolation>,
    ) {
        for required in SearchContractObjectClass::ALL {
            let Some(object) = self
                .contract_objects
                .iter()
                .find(|object| object.object_class == required)
            else {
                push(
                    violations,
                    "contract_objects",
                    &format!("missing contract object {}", required.as_str()),
                );
                continue;
            };
            let base = format!("contract_objects.{}", object.object_id);
            // Own-proof guard: an object may not borrow an adjacent lane's proof.
            let refs = required.backing_refs();
            if object.object_id != required.object_id() {
                push(violations, &base, "object_id must match the object class");
            }
            if object.label != required.label() {
                push(violations, &base, "label must match the canonical label");
            }
            if object.backing_schema_ref != refs.schema_ref {
                push(
                    violations,
                    &format!("{base}.backing_schema_ref"),
                    "object must cite its own lane boundary schema",
                );
            }
            if object.backing_fixture_ref != refs.fixture_ref {
                push(
                    violations,
                    &format!("{base}.backing_fixture_ref"),
                    "object must cite its own lane fixture corpus",
                );
            }
            if object.backing_record_kind != refs.record_kind {
                push(
                    violations,
                    &format!("{base}.backing_record_kind"),
                    "object must cite its own lane record kind",
                );
            }
            if object.privacy_data_class != required.privacy_data_class() {
                push(
                    violations,
                    &format!("{base}.privacy_data_class"),
                    "object must cite its canonical privacy data class",
                );
            }
        }
    }

    fn validate_vocabulary(&self, violations: &mut Vec<M5SearchNavigationQualificationViolation>) {
        for required in ResultStateClass::ALL {
            let Some(row) = self
                .result_state_vocabulary
                .iter()
                .find(|row| row.state_class == required)
            else {
                push(
                    violations,
                    "result_state_vocabulary",
                    &format!("missing result state {}", required.as_str()),
                );
                continue;
            };
            let base = format!("result_state_vocabulary.{}", row.token);
            if row.token != required.as_str() {
                push(violations, &base, "token must match the state class");
            }
            if row.label != required.label() {
                push(violations, &base, "label must match the canonical label");
            }
            if row.narrows_scope != required.narrows_scope() {
                push(
                    violations,
                    &base,
                    "narrows_scope must match the canonical state",
                );
            }
            if row.must_stay_visible != required.must_stay_visible() {
                push(
                    violations,
                    &base,
                    "must_stay_visible must match the canonical state",
                );
            }
        }
        // Every state must be expressible by at least one claimed surface, so the
        // vocabulary cannot drift away from the surfaces that surface it.
        for state in ResultStateClass::ALL {
            if !SearchSurfaceClass::ALL
                .into_iter()
                .any(|surface| surface.expressible_states().contains(&state))
            {
                push(
                    violations,
                    "result_state_vocabulary",
                    &format!("no surface expresses result state {}", state.as_str()),
                );
            }
        }
    }

    fn validate_privacy_bindings(
        &self,
        violations: &mut Vec<M5SearchNavigationQualificationViolation>,
    ) {
        for required in PrivacyDataClass::ALL {
            let Some(binding) = self
                .privacy_bindings
                .iter()
                .find(|binding| binding.data_class == required)
            else {
                push(
                    violations,
                    "privacy_bindings",
                    &format!("missing privacy binding {}", required.as_str()),
                );
                continue;
            };
            let base = format!("privacy_bindings.{}", binding.data_class.as_str());
            // Raw query text must stay local-only by default and never carry a
            // weaker-than-explicit consent requirement for sharing.
            if binding.data_class == PrivacyDataClass::RawQueryText
                && !binding.local_only_by_default
            {
                push(
                    violations,
                    &base,
                    "raw query text must remain local-only by default",
                );
            }
            if binding.summary.trim().is_empty() {
                push(
                    violations,
                    &base,
                    "privacy binding summary may not be empty",
                );
            }
        }
    }

    fn validate_row(
        &self,
        violations: &mut Vec<M5SearchNavigationQualificationViolation>,
        row: &SurfaceQualificationRow,
        rule_ids: &[&str],
        object_ids: &[&str],
    ) {
        let base = format!("qualification_rows.{}", row.qualification_row_id);
        if row.surface_label != row.surface.label() {
            push(
                violations,
                &format!("{base}.surface_label"),
                "surface_label must match the canonical surface label",
            );
        }
        if row.summary.trim().is_empty() {
            push(
                violations,
                &format!("{base}.summary"),
                "summary may not be empty",
            );
        }

        // Shared-model invariant: every claimed surface answers off the one query
        // session and result-identity model, not a surface-local heuristic.
        if row.shared_query_session_ref != QUERY_SESSION_SCHEMA_REF {
            push(
                violations,
                &format!("{base}.shared_query_session_ref"),
                "row must reference the one shared query-session model",
            );
        }
        if row.shared_result_identity_ref != SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF {
            push(
                violations,
                &format!("{base}.shared_result_identity_ref"),
                "row must reference the one shared result-identity model",
            );
        }

        // Bound objects must exist in the catalog and match the surface's own
        // binding set, and must always include the shared session + result ref.
        let expected_objects = row.surface.bound_objects();
        for object in [
            SearchContractObjectClass::QuerySession,
            SearchContractObjectClass::ResultRef,
        ] {
            if !row.bound_object_ids.contains(&object.object_id()) {
                push(
                    violations,
                    &format!("{base}.bound_object_ids"),
                    &format!("row must bind the shared {} object", object.as_str()),
                );
            }
        }
        for object_id in &row.bound_object_ids {
            if !object_ids.contains(&object_id.as_str()) {
                push(
                    violations,
                    &format!("{base}.bound_object_ids"),
                    &format!("row binds unknown object {object_id}"),
                );
                continue;
            }
            if !expected_objects
                .iter()
                .any(|object| &object.object_id() == object_id)
            {
                push(
                    violations,
                    &format!("{base}.bound_object_ids"),
                    &format!("row binds object {object_id} that is not part of its surface"),
                );
            }
        }

        // Expressible states must be a non-empty subset of the surface's own set.
        if row.expressible_states.is_empty() {
            push(
                violations,
                &format!("{base}.expressible_states"),
                "row must express at least one result state",
            );
        }
        let allowed_states = row.surface.expressible_states();
        for state in &row.expressible_states {
            if !allowed_states.contains(state) {
                push(
                    violations,
                    &format!("{base}.expressible_states"),
                    &format!("row expresses unsupported state {}", state.as_str()),
                );
            }
        }

        // Deployment-mode coverage must be a non-empty subset of declared modes.
        if row.deployment_mode_coverage.is_empty() {
            push(
                violations,
                &format!("{base}.deployment_mode_coverage"),
                "row must cover at least one deployment mode",
            );
        }
        for mode in &row.deployment_mode_coverage {
            if !self.deployment_modes.contains(mode) {
                push(
                    violations,
                    &format!("{base}.deployment_mode_coverage"),
                    &format!("row covers undeclared deployment mode {}", mode.as_str()),
                );
            }
        }

        // Local-only query-text invariant: a surface that persists or exports
        // query material may never demote local-only query text below a path.
        if row.persists_query_material != row.surface.persists_query_material() {
            push(
                violations,
                &format!("{base}.persists_query_material"),
                "persists_query_material must match the surface's capability",
            );
        }
        if row.persists_query_material && !row.local_query_text_first {
            push(
                violations,
                &format!("{base}.local_query_text_first"),
                "a query-material surface must keep local-only query text first-class",
            );
        }

        if row.published_state == QualificationStateClass::Qualified
            && !row.stale_proof_tokens.is_empty()
        {
            push(
                violations,
                &format!("{base}.stale_proof_tokens"),
                "qualified rows may not carry stale proof tokens",
            );
        }
        if row.published_state != QualificationStateClass::Qualified
            && row.downgrade_rule_ids.is_empty()
        {
            push(
                violations,
                &format!("{base}.downgrade_rule_ids"),
                "non-qualified rows must cite downgrade rules",
            );
        }
        for rule_id in &row.downgrade_rule_ids {
            if !rule_ids.contains(&rule_id.as_str()) {
                push(
                    violations,
                    &format!("{base}.downgrade_rule_ids"),
                    &format!("row cites unknown downgrade rule {rule_id}"),
                );
            }
        }
    }

    fn validate_consumer_bindings(
        &self,
        violations: &mut Vec<M5SearchNavigationQualificationViolation>,
    ) {
        for required in QualificationConsumerClass::ALL {
            let Some(binding) = self
                .consumer_bindings
                .iter()
                .find(|binding| binding.consumer == required)
            else {
                push(
                    violations,
                    "consumer_bindings",
                    &format!("missing consumer binding {}", required.as_str()),
                );
                continue;
            };
            let base = format!("consumer_bindings.{}", binding.consumer.as_str());
            if binding.ingested_packet_id != self.packet_id {
                push(
                    violations,
                    &base,
                    "consumer binding must ingest the canonical packet id",
                );
            }
            if binding.qualification_row_count != self.qualification_rows.len() {
                push(
                    violations,
                    &base,
                    "consumer binding row count must match qualification rows",
                );
            }
            if !binding.narrow_on_stale_proof {
                push(
                    violations,
                    &base,
                    "consumer binding must narrow on stale proof",
                );
            }
            for field in REQUIRED_PROJECTION_FIELDS {
                if !binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field)
                {
                    push(
                        violations,
                        &base,
                        &format!("consumer binding must preserve {field}"),
                    );
                }
            }
        }
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .consumer_bindings
                .iter()
                .all(|binding| binding.narrow_on_stale_proof)
    }

    /// Returns the number of rows in each published state, for claim packets.
    pub fn state_counts(&self) -> QualificationStateCounts {
        let mut counts = QualificationStateCounts::default();
        for row in &self.qualification_rows {
            match row.published_state {
                QualificationStateClass::Qualified => counts.qualified += 1,
                QualificationStateClass::ScopeLimited => counts.scope_limited += 1,
                QualificationStateClass::LocalQueryTextOnly => counts.local_query_text_only += 1,
                QualificationStateClass::BlockedUnverified => counts.blocked_unverified += 1,
            }
        }
        counts
    }
}

/// Row counts by published qualification state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QualificationStateCounts {
    /// Rows that remain fully qualified.
    pub qualified: usize,
    /// Rows narrowed to a scope-limited claim.
    pub scope_limited: usize,
    /// Rows narrowed to a local-only query-text claim.
    pub local_query_text_only: usize,
    /// Rows blocked pending fresh proof.
    pub blocked_unverified: usize,
}

/// Returns the canonical seeded M5 search/navigation qualification packet.
pub fn seeded_m5_search_navigation_qualification_packet() -> M5SearchNavigationQualificationPacket {
    build_packet(QualificationVariant::Canonical)
}

/// Returns a seeded packet where the index is partial/stale, so every
/// live-retrieval surface narrows to a scope-limited claim while the
/// durable-artifact surfaces (which label their own captured freshness) stay
/// qualified.
pub fn seeded_partial_index_stale_m5_search_navigation_qualification_packet(
) -> M5SearchNavigationQualificationPacket {
    build_packet(QualificationVariant::PartialIndexStale)
}

/// Returns a seeded packet where query-material consent is missing, so every
/// surface that persists or exports query material narrows to a local-only
/// query-text claim while local-only query text stays first-class.
pub fn seeded_unconsented_query_text_m5_search_navigation_qualification_packet(
) -> M5SearchNavigationQualificationPacket {
    build_packet(QualificationVariant::UnconsentedQueryText)
}

#[derive(Debug, Clone, Copy)]
enum QualificationVariant {
    Canonical,
    PartialIndexStale,
    UnconsentedQueryText,
}

fn build_packet(variant: QualificationVariant) -> M5SearchNavigationQualificationPacket {
    let qualification_rows: Vec<SurfaceQualificationRow> = SearchSurfaceClass::ALL
        .into_iter()
        .map(|surface| seed_row(surface, variant))
        .collect();
    let row_count = qualification_rows.len();

    M5SearchNavigationQualificationPacket {
        record_kind: M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_VERSION,
        packet_id: M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        doc_ref: M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF.to_owned(),
        schema_ref: M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF.to_owned(),
        artifact_ref: M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            QUERY_SESSION_SCHEMA_REF.to_owned(),
            SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
            SEARCH_OPERATOR_TRUTH_SCHEMA_REF.to_owned(),
            SAVED_QUERY_SCHEMA_REF.to_owned(),
            SCOPE_PACK_SCHEMA_REF.to_owned(),
            SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF.to_owned(),
        ],
        claimed_surfaces: SearchSurfaceClass::ALL.to_vec(),
        deployment_modes: DeploymentMode::ALL.to_vec(),
        contract_objects: seeded_contract_objects(),
        result_state_vocabulary: seeded_result_state_vocabulary(),
        privacy_bindings: seeded_privacy_bindings(),
        qualification_rows,
        downgrade_rules: seeded_downgrade_rules(),
        consumer_bindings: seeded_consumer_bindings(row_count),
        export_safe_summary:
            "This metadata-safe qualification index freezes the M5 search-query, result-identity, ranking-reason, and saved-query/privacy contract: every claimed search surface answers off the one shared query-session and result-identity model, binds each canonical contract object to its own lane schema, fixture, and record kind, and expresses the closed Exact / Context-promoted / Semantic / Partial-index / Withheld-latency / Policy-hidden / Cached / Stale / Imported vocabulary; raw query text stays local-only by default, a partial or stale index narrows the claim instead of masquerading as whole-workspace certainty, and no raw query text, source bodies, provider payloads, or secrets cross the boundary."
                .to_owned(),
    }
}

fn seed_row(surface: SearchSurfaceClass, variant: QualificationVariant) -> SurfaceQualificationRow {
    let persists = surface.persists_query_material();
    let bound_object_ids: Vec<String> = surface
        .bound_objects()
        .iter()
        .map(|object| object.object_id())
        .collect();
    let query_text_privacy_class = if persists {
        PrivacyClass::UserSynced
    } else {
        PrivacyClass::LocalSensitive
    };
    let mut row = SurfaceQualificationRow {
        qualification_row_id: format!("m5_search_navigation:{}", surface.as_str()),
        surface,
        surface_label: surface.label().to_owned(),
        published_state: QualificationStateClass::Qualified,
        deployment_mode_coverage: DeploymentMode::ALL.to_vec(),
        shared_query_session_ref: QUERY_SESSION_SCHEMA_REF.to_owned(),
        shared_result_identity_ref: SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
        bound_object_ids,
        expressible_states: surface.expressible_states(),
        query_text_privacy_class,
        persists_query_material: persists,
        local_query_text_first: true,
        stale_proof_tokens: Vec::new(),
        downgrade_rule_ids: Vec::new(),
        summary: format!(
            "{} answers off the one shared query-session and result-identity model across the product surface and CLI/headless, expresses the closed result-state vocabulary, and keeps local-only query text first-class{}.",
            surface.label(),
            if persists {
                " beside any sync or export of query material"
            } else {
                ""
            }
        ),
    };

    match variant {
        QualificationVariant::Canonical => {}
        QualificationVariant::PartialIndexStale => {
            if surface.depends_on_live_index() {
                apply_downgrade(
                    &mut row,
                    QualificationStateClass::ScopeLimited,
                    "partial_index_stale_epoch",
                    "partial_index_or_stale_scope_narrows_claim",
                    &format!(
                        "{} narrows to a scope-limited claim because the index epoch is partial/stale; the surface keeps the partial-index and stale states visible and may not imply whole-workspace certainty until the index warms.",
                        surface.label()
                    ),
                );
            }
        }
        QualificationVariant::UnconsentedQueryText => {
            if surface.persists_query_material() {
                apply_downgrade(
                    &mut row,
                    QualificationStateClass::LocalQueryTextOnly,
                    "query_material_consent_missing",
                    "query_text_privacy_unconsented_narrows_to_local",
                    &format!(
                        "{} narrows to a local-only query-text claim because consent for syncing or exporting query material is missing; raw query text stays local-only and first-class while the sync and export paths are held to hash-only or omitted.",
                        surface.label()
                    ),
                );
            }
        }
    }

    row
}

fn apply_downgrade(
    row: &mut SurfaceQualificationRow,
    state: QualificationStateClass,
    token: &str,
    rule_id: &str,
    summary: &str,
) {
    row.published_state = state;
    row.stale_proof_tokens.push(token.to_owned());
    row.downgrade_rule_ids.push(rule_id.to_owned());
    row.summary = summary.to_owned();
}

fn seeded_contract_objects() -> Vec<SearchContractObjectRow> {
    SearchContractObjectClass::ALL
        .into_iter()
        .map(|object| {
            let refs = object.backing_refs();
            SearchContractObjectRow {
                object_id: object.object_id(),
                object_class: object,
                label: object.label().to_owned(),
                backing_schema_ref: refs.schema_ref.to_owned(),
                backing_fixture_ref: refs.fixture_ref.to_owned(),
                backing_record_kind: refs.record_kind.to_owned(),
                privacy_data_class: object.privacy_data_class(),
                summary: format!(
                    "{} is frozen against its own lane proof ({}) and governed by the {} privacy data class.",
                    object.label(),
                    refs.schema_ref,
                    object.privacy_data_class().as_str()
                ),
            }
        })
        .collect()
}

fn seeded_result_state_vocabulary() -> Vec<ResultStateRow> {
    ResultStateClass::ALL
        .into_iter()
        .map(|state| ResultStateRow {
            state_class: state,
            token: state.as_str().to_owned(),
            label: state.label().to_owned(),
            narrows_scope: state.narrows_scope(),
            must_stay_visible: state.must_stay_visible(),
            summary: format!(
                "The {} state {} a narrowed claim and {} stay visible to the user and downstream consumers.",
                state.label().to_lowercase(),
                if state.narrows_scope() {
                    "requires"
                } else {
                    "does not require"
                },
                if state.must_stay_visible() {
                    "must"
                } else {
                    "need not"
                }
            ),
        })
        .collect()
}

fn seeded_privacy_bindings() -> Vec<PrivacyBindingRow> {
    vec![
        PrivacyBindingRow {
            data_class: PrivacyDataClass::RawQueryText,
            privacy_class: PrivacyClass::LocalSensitive,
            retention_mode: RetentionMode::LocalOnlyDefault,
            consent_requirement: ConsentRequirement::ExplicitForShare,
            local_only_by_default: true,
            summary: "Raw query text stays in the local session/profile by default; it may leave the device only with explicit share consent, and is redacted to a hash at any workspace, sync, or support boundary.".to_owned(),
        },
        PrivacyBindingRow {
            data_class: PrivacyDataClass::QueryHash,
            privacy_class: PrivacyClass::LocalDerived,
            retention_mode: RetentionMode::LocalHashOnly,
            consent_requirement: ConsentRequirement::NoneLocalDefault,
            local_only_by_default: true,
            summary: "Deterministic, non-reversible query hashes are retained locally and may stand in for raw text at redacted boundaries without additional consent.".to_owned(),
        },
        PrivacyBindingRow {
            data_class: PrivacyDataClass::SavedQuerySync,
            privacy_class: PrivacyClass::UserSynced,
            retention_mode: RetentionMode::ExplicitSyncOptIn,
            consent_requirement: ConsentRequirement::ExplicitSyncOptIn,
            local_only_by_default: true,
            summary: "Saved queries and scope packs stay local-only until the user explicitly opts a saved query into sync; sync never widens silently and keeps the local copy first-class.".to_owned(),
        },
        PrivacyBindingRow {
            data_class: PrivacyDataClass::SupportExportPacket,
            privacy_class: PrivacyClass::ExportMetadata,
            retention_mode: RetentionMode::SupportExportBounded,
            consent_requirement: ConsentRequirement::ExplicitPerExport,
            local_only_by_default: true,
            summary: "Search-export packets carry only redacted metadata, require explicit per-export consent, and never embed raw query text, source bodies, or secrets.".to_owned(),
        },
    ]
}

fn seeded_downgrade_rules() -> Vec<QualificationDowngradeRuleRow> {
    vec![
        QualificationDowngradeRuleRow {
            rule_id: "shared_model_drift_blocks_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::SharedModelDrift,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::BlockedUnverified,
            required_effect: "When a surface stops minting a durable query session before rerank, stops referencing the shared result-identity model, or its result IDs stop surviving virtualization and preview churn, its broad claim blocks until the shared-model anchor is restored.".to_owned(),
            rationale: "The whole point of the index is one shared query-session and result-identity model; a drifting surface may not keep a green claim off a private heuristic.".to_owned(),
            evidence_refs: vec![
                M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF.to_owned(),
                QUERY_SESSION_SCHEMA_REF.to_owned(),
                SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "partial_index_or_stale_scope_narrows_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::PartialIndexOrStaleScope,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::ScopeLimited,
            required_effect: "When a surface answers off a partial (still-warming), cached, or stale index, its row narrows to a scope-limited claim, keeps the partial-index/cached/stale state visible, and may not imply whole-workspace certainty.".to_owned(),
            rationale: "Search and navigation artifacts must never masquerade as complete coverage when scope or freshness is limited.".to_owned(),
            evidence_refs: vec![
                M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF.to_owned(),
                SEARCH_OPERATOR_TRUTH_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "withheld_or_policy_hidden_narrows_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::WithheldOrPolicyHidden,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::ScopeLimited,
            required_effect: "When candidates are withheld to hold a latency budget or hidden by policy or scope, the row narrows to a scope-limited claim and keeps the withheld-latency/policy-hidden state and counts visible.".to_owned(),
            rationale: "Withheld and policy-hidden results must stay visible so a surface never implies it returned every match.".to_owned(),
            evidence_refs: vec![
                M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF.to_owned(),
                SEARCH_OPERATOR_TRUTH_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "query_text_privacy_unconsented_narrows_to_local".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::QueryTextPrivacyUnconsented,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::LocalQueryTextOnly,
            required_effect: "When raw query text, saved-query sync, or export of query material lacks consent, the surface narrows to a local-only query-text claim; raw query text stays local-only and first-class while sync and export are held to hash-only or omitted.".to_owned(),
            rationale: "Raw query text is local-only by default; a sync or export claim is only safe while consent for that data class is current.".to_owned(),
            evidence_refs: vec![
                SAVED_QUERY_SCHEMA_REF.to_owned(),
                SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "imported_provenance_unverified_narrows_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::ImportedProvenanceUnverified,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::ScopeLimited,
            required_effect: "When an imported saved-query or scope-pack provenance is unverified, the row narrows to a scope-limited claim and keeps the imported state visible rather than presenting imported material as local truth.".to_owned(),
            rationale: "Imported artifacts carry external provenance; the surface must label them imported, not silently merge them into the local result identity.".to_owned(),
            evidence_refs: vec![
                SAVED_QUERY_SCHEMA_REF.to_owned(),
                SCOPE_PACK_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "consumer_binding_missing_blocks_shared_truth".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::ConsumerBindingMissing,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::BlockedUnverified,
            required_effect: "If the product search surface, CLI/headless, docs/help, support export, shiproom, or release manifest stops ingesting this packet by reference, the broad search claim blocks until parity is restored.".to_owned(),
            rationale: "The task requires one shared search/navigation qualification index; a broken consumer binding invalidates that promise.".to_owned(),
            evidence_refs: vec![
                M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF.to_owned(),
                M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF.to_owned(),
                RELEASE_MANIFEST_CONSUMER_REF.to_owned(),
            ],
        },
    ]
}

fn seeded_consumer_bindings(row_count: usize) -> Vec<QualificationConsumerBinding> {
    let verbatim_fields: Vec<String> = REQUIRED_PROJECTION_FIELDS
        .iter()
        .map(|field| (*field).to_owned())
        .collect();
    let binding = |consumer: QualificationConsumerClass, consumer_ref: &str, summary: &str| {
        QualificationConsumerBinding {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: M5_SEARCH_NAVIGATION_QUALIFICATION_PACKET_ID.to_owned(),
            qualification_row_count: row_count,
            required_verbatim_fields: verbatim_fields.clone(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: summary.to_owned(),
        }
    };
    vec![
        binding(
            QualificationConsumerClass::ProductSearchSurface,
            PRODUCT_SEARCH_CONSUMER_REF,
            "The product search surface and result pane reuse the qualification row ids, surface tokens, published state, expressible states, and stale-proof tokens verbatim instead of inventing a surface-local badge.",
        ),
        binding(
            QualificationConsumerClass::CliHeadless,
            M5_SEARCH_NAVIGATION_QUALIFICATION_ARTIFACT_REF,
            "CLI/headless search output projects the same rows so ranking, withheld, and privacy states read identically off the product surface.",
        ),
        binding(
            QualificationConsumerClass::DocsHelp,
            M5_SEARCH_NAVIGATION_QUALIFICATION_DOC_REF,
            "Docs/help search and discoverability surfaces describe the same vocabulary and states by reference rather than paraphrasing search maturity.",
        ),
        binding(
            QualificationConsumerClass::SupportExport,
            SUPPORT_EXPORT_CONSUMER_REF,
            "Support-export packets attach the same row ids, expressible states, and downgrade tokens instead of minting a parallel search badge, and stay metadata-only.",
        ),
        binding(
            QualificationConsumerClass::Shiproom,
            M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_REF,
            "The shiproom derives its publishable / scope-limited / local-only / blocked scope from this index and narrows automatically when a surface row goes stale or red.",
        ),
        binding(
            QualificationConsumerClass::ReleaseManifest,
            RELEASE_MANIFEST_CONSUMER_REF,
            "Release manifests consume the same qualification index so a partial index, withheld results, or missing query-material consent cannot keep a broader release claim green.",
        ),
    ]
}

fn push(violations: &mut Vec<M5SearchNavigationQualificationViolation>, path: &str, message: &str) {
    violations.push(M5SearchNavigationQualificationViolation {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

// The frozen schema version must track the upstream query-session schema this
// index composes; a bump there should be a deliberate, reviewed change here too.
const _: () = assert!(
    M5_SEARCH_NAVIGATION_QUALIFICATION_SCHEMA_VERSION == SEARCH_QUERY_SESSION_SCHEMA_VERSION
);

#[cfg(test)]
mod tests;
