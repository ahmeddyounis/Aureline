//! Saved-query, scope-pack, history-retention, and signed deep-link governance
//! for the M5 search/navigation lane.
//!
//! Where [`crate::query_artifacts`] owns the *row-level* durable artifacts
//! ([`SavedQuery`], [`QueryHistoryEntry`], [`ScopePackBinding`], and
//! [`SearchDeepLink`]) and [`crate::session_ledger`] owns the privacy and export
//! vocabulary, this module freezes those objects into one delivery-grade,
//! portable governance packet that makes saved queries, scope packs, query
//! history, and deep links as governed as every other M5 artifact:
//!
//! - [`GovernedSavedQueryRow`] binds one [`SavedQuery`] to its captured
//!   [`ScopePackBinding`] and local [`QueryHistoryEntry`], and adds the
//!   captured-vs-current [`ScopeDriftDisclosure`] that proves a reopen, a
//!   migration, or a scope drift is always *disclosed*, never a silent semantic
//!   break.
//! - [`SignedSearchDeepLink`] wraps a canonical [`SearchDeepLink`] with a
//!   tamper-evident content signature over its disclosed intent, completeness
//!   note, scope, freshness, and return anchor. The signature binds the
//!   disclosure to the link so a recipient can detect tampering; the link
//!   reopens *search intent* under the recipient's own permissions and never
//!   implies live current certainty or widens access.
//! - [`LocalVersusSyncedRetentionRow`] is the machine-readable matrix proving
//!   raw query text stays local-only by default and only widens with an explicit
//!   basis, while hashes, scope metadata, and result refs follow their own
//!   per-data-class sync, retention, and redaction posture.
//!
//! The [`SavedQueryGovernancePacket`] proves the same governed artifacts are
//! reused by the product UI, the sync/portability lane, and the support-export
//! consumers ([`GovernanceConsumerClass`]) without widening authority. Raw query
//! text is confined to local-only artifacts: sync, share, and support export
//! carry redacted metadata only, and [`SavedQueryGovernancePacket::redact_for_export`]
//! materializes the redacted copy a support bundle ships.
//!
//! [`SavedQuery`]: crate::query_artifacts::SavedQuery
//! [`QueryHistoryEntry`]: crate::query_artifacts::QueryHistoryEntry
//! [`ScopePackBinding`]: crate::query_artifacts::ScopePackBinding
//! [`SearchDeepLink`]: crate::query_artifacts::SearchDeepLink

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::lexical::ScopeClass;
use crate::query_artifacts::{
    QueryHistoryEntry, SavedQuery, ScopePackBinding, SearchDeepLink, SearchRedactionProfile,
    SearchResultSemantics, SearchRetentionMode, SearchRetentionWideningBasis,
    SearchScopeHonestyState, SearchSyncClass,
};
use crate::query_session::{stable_query_hash, QueryTextMode, SearchSurface};
use crate::session_ledger::{
    SavedQueryPrivacyClass, SavedQueryRecord, SavedQueryRecordInputs, SavedQuerySharePolicy,
    SavedQuerySourceClass,
};

/// Stable record-kind tag for [`SavedQueryGovernancePacket`].
pub const SAVED_QUERY_GOVERNANCE_PACKET_RECORD_KIND: &str = "saved_query_governance_packet";

/// Stable record-kind tag for [`SavedQueryGovernanceSupportExport`].
pub const SAVED_QUERY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "saved_query_governance_support_export";

/// Integer schema version for the saved-query governance packet.
pub const SAVED_QUERY_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable packet identifier reused by every consumer projection.
pub const SAVED_QUERY_GOVERNANCE_PACKET_ID: &str = "search.m5.saved_query_governance.v1";

/// Repository-relative path of the boundary schema.
pub const SAVED_QUERY_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/search/saved-query-governance.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const SAVED_QUERY_GOVERNANCE_DOC_REF: &str = "docs/search/saved-query-governance.md";

/// Repository-relative path of the checked review artifact.
pub const SAVED_QUERY_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/search/m5/saved-query-governance.md";

/// Repository-relative path of the protected fixture directory.
pub const SAVED_QUERY_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/search/m5/saved-query-retention";

/// Fixed generation timestamp for the seeded corpus.
const SEEDED_GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Privacy classes the governance matrix realizes, in canonical order.
pub const ALL_PRIVACY_CLASSES: [SavedQueryPrivacyClass; 4] = [
    SavedQueryPrivacyClass::LocalOnlyPrivate,
    SavedQueryPrivacyClass::WorkspaceSharedRedacted,
    SavedQueryPrivacyClass::SupportExportRedacted,
    SavedQueryPrivacyClass::PolicyWithheld,
];

/// Sync classes the governance matrix realizes, in canonical order.
pub const ALL_SYNC_CLASSES: [SearchSyncClass; 6] = [
    SearchSyncClass::LocalOnly,
    SearchSyncClass::ExplicitUserSync,
    SearchSyncClass::WorkspaceShared,
    SearchSyncClass::RepoProvided,
    SearchSyncClass::PolicyManaged,
    SearchSyncClass::SupportExportOnly,
];

/// Retention modes the governance matrix realizes, in canonical order.
pub const ALL_RETENTION_MODES: [SearchRetentionMode; 6] = [
    SearchRetentionMode::LocalOnlyDefault,
    SearchRetentionMode::LocalOnlyEphemeral,
    SearchRetentionMode::WorkspaceSharedExplicit,
    SearchRetentionMode::RepoProvidedReadOnly,
    SearchRetentionMode::PolicyOwnedManaged,
    SearchRetentionMode::SupportExportBounded,
];

/// Redaction profiles the governance matrix realizes, in canonical order.
pub const ALL_REDACTION_PROFILES: [SearchRedactionProfile; 5] = [
    SearchRedactionProfile::LiteralLocalOnly,
    SearchRedactionProfile::HashesScopeAndResultRefs,
    SearchRedactionProfile::MetadataOnlyNoQueryMaterial,
    SearchRedactionProfile::PolicyWithheld,
    SearchRedactionProfile::ExplicitLiteralConsent,
];

/// Closed vocabulary of query-material data classes governed by the retention
/// matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryDataClass {
    /// The literal query string a user typed.
    RawQueryText,
    /// A deterministic, non-reversible hash of the query string.
    QueryHash,
    /// The redaction-safe parsed query/filter grammar.
    ParsedQueryAst,
    /// Scope identity, mode, and chip labels captured for replay.
    ScopeMetadata,
    /// Stable references to the results a query produced.
    ResultRefs,
}

impl QueryDataClass {
    /// Every data class, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::RawQueryText,
        Self::QueryHash,
        Self::ParsedQueryAst,
        Self::ScopeMetadata,
        Self::ResultRefs,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawQueryText => "raw_query_text",
            Self::QueryHash => "query_hash",
            Self::ParsedQueryAst => "parsed_query_ast",
            Self::ScopeMetadata => "scope_metadata",
            Self::ResultRefs => "result_refs",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RawQueryText => "Raw query text",
            Self::QueryHash => "Query hash",
            Self::ParsedQueryAst => "Parsed query AST",
            Self::ScopeMetadata => "Scope metadata",
            Self::ResultRefs => "Result refs",
        }
    }

    /// True when this data class can carry literal query material a user typed.
    pub const fn carries_literal_query_material(self) -> bool {
        matches!(self, Self::RawQueryText | Self::ParsedQueryAst)
    }
}

/// Trust origin that backs a signed deep link's content signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkSignatureScheme {
    /// Digest signed by a local device key; the link stays on this device.
    LocalContentDigest,
    /// Digest signed by a workspace key for explicit workspace sharing.
    WorkspaceSignedDigest,
    /// Digest signed by a managed policy key.
    PolicySignedDigest,
}

impl DeepLinkSignatureScheme {
    /// Every scheme, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::LocalContentDigest,
        Self::WorkspaceSignedDigest,
        Self::PolicySignedDigest,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalContentDigest => "local_content_digest",
            Self::WorkspaceSignedDigest => "workspace_signed_digest",
            Self::PolicySignedDigest => "policy_signed_digest",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalContentDigest => "Local content digest",
            Self::WorkspaceSignedDigest => "Workspace-signed digest",
            Self::PolicySignedDigest => "Policy-signed digest",
        }
    }
}

/// Consumer that ingests the governed saved-query artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceConsumerClass {
    /// Desktop saved-query, history, and deep-link chrome.
    ProductUi,
    /// Cross-device sync and portability of the artifacts.
    SyncPortability,
    /// Redacted support/export replay.
    SupportExport,
}

impl GovernanceConsumerClass {
    /// Every consumer, in canonical order.
    pub const ALL: [Self; 3] = [Self::ProductUi, Self::SyncPortability, Self::SupportExport];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::SyncPortability => "sync_portability",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProductUi => "Product UI",
            Self::SyncPortability => "Sync / portability",
            Self::SupportExport => "Support export",
        }
    }
}

/// Captured-vs-current scope truth attached to a reopenable saved query.
///
/// This makes a reopen, a migration, or a scope drift an explicit, disclosed
/// state instead of a silent semantic change: a saved query that is reopened
/// against a changed scope rebinds or re-resolves visibly, and never claims the
/// captured rows are still current.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDriftDisclosure {
    /// Stable scope identity captured when the query was saved.
    pub captured_stable_scope_id: String,
    /// Stable scope identity active now.
    pub current_stable_scope_id: String,
    /// Scope honesty state shown to the user on reopen.
    pub scope_honesty_state: SearchScopeHonestyState,
    /// Live-vs-captured semantics; never `current_live_results` without a rerun.
    pub result_semantics: SearchResultSemantics,
    /// True when a live rerun is required before claiming current truth.
    pub rerun_required: bool,
    /// True when a wider current scope was narrowed back to the captured scope.
    pub effective_scope_narrowed_to_captured: bool,
    /// Always `false`: scope drift is disclosed, never a silent semantic break.
    pub silent_semantic_break: bool,
    /// User-visible disclosure rendered on reopen.
    pub disclosure: String,
}

impl ScopeDriftDisclosure {
    /// True when the captured scope still matches the current scope.
    pub fn scope_still_current(&self) -> bool {
        self.captured_stable_scope_id == self.current_stable_scope_id
            && self.scope_honesty_state == SearchScopeHonestyState::CapturedScopeStillCurrent
    }
}

/// One governed saved query bound to its scope pack, local history, and the
/// captured-vs-current scope truth shown on reopen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedSavedQueryRow {
    /// Stable row identity.
    pub row_id: String,
    /// Durable saved query, reused verbatim.
    pub saved_query: SavedQuery,
    /// Captured scope pack the saved query reopens against.
    pub scope_pack: ScopePackBinding,
    /// Local query-history entry linked to the saved query.
    pub history_entry: QueryHistoryEntry,
    /// Captured-vs-current scope drift truth.
    pub scope_drift: ScopeDriftDisclosure,
    /// True when the saved query reopens without losing identity or scope truth.
    pub survives_reopen: bool,
    /// True when the saved query survives schema migration.
    pub survives_migration: bool,
    /// True when scope drift is disclosed rather than silently breaking.
    pub survives_scope_drift: bool,
    /// Reviewable summary of the governed row.
    pub summary: String,
}

impl GovernedSavedQueryRow {
    /// True when the row carries raw query text confined to a local-only
    /// artifact, or carries no raw query text at all.
    pub fn raw_query_text_is_local_only(&self) -> bool {
        self.saved_query.query_text.is_none()
            || (self.saved_query.privacy_class == SavedQueryPrivacyClass::LocalOnlyPrivate
                && self.saved_query.sync_class == SearchSyncClass::LocalOnly
                && self.saved_query.redaction_profile == SearchRedactionProfile::LiteralLocalOnly)
    }
}

/// A canonical [`SearchDeepLink`] wrapped with a tamper-evident content
/// signature over its disclosed intent, completeness, scope, and freshness.
///
/// The signature is a deterministic, verifiable content digest of the
/// disclosure fields, scoped by [`DeepLinkSignatureScheme`]. It is not a
/// cryptographic identity proof; it binds the disclosed scope, freshness, and
/// partiality to the link so a recipient can detect tampering before reopening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedSearchDeepLink {
    /// Stable signed-link identity.
    pub signed_link_id: String,
    /// Canonical deep link reused verbatim.
    pub deep_link: SearchDeepLink,
    /// User-visible intent the link reopens; never frozen result certainty.
    pub intent_summary: String,
    /// Completeness note disclosing partiality and freshness.
    pub completeness_note: String,
    /// Live-vs-captured freshness disclosure shown on the open sheet.
    pub freshness_disclosure: SearchResultSemantics,
    /// Scope honesty state disclosed to the recipient.
    pub scope_disclosure: SearchScopeHonestyState,
    /// True when result partiality is disclosed on the open sheet.
    pub partiality_disclosed: bool,
    /// Supportable return path the recipient can return focus to.
    pub return_anchor_ref: String,
    /// Always `false`: a deep link reopens intent, not live current certainty.
    pub implies_live_current_certainty: bool,
    /// Trust origin backing the content signature.
    pub signature_scheme: DeepLinkSignatureScheme,
    /// Identity of the key that produced the signature.
    pub signing_key_id: String,
    /// Field names the content digest covers.
    pub signed_fields: Vec<String>,
    /// Deterministic content digest over the disclosure fields.
    pub payload_digest: String,
    /// Key-scoped signature over the content digest.
    pub signature: String,
    /// Reviewable summary of the signed link.
    pub summary: String,
}

impl SignedSearchDeepLink {
    /// Field names the deterministic content digest covers.
    pub const SIGNED_FIELDS: [&'static str; 9] = [
        "deep_link_id",
        "scope_binding_id_ref",
        "intent_summary",
        "completeness_note",
        "freshness_disclosure",
        "scope_disclosure",
        "return_anchor_ref",
        "access_widening_allowed",
        "implies_live_current_certainty",
    ];

    /// Signs a canonical deep link, computing a deterministic content digest and
    /// key-scoped signature over the disclosed intent, scope, and freshness.
    #[allow(clippy::too_many_arguments)]
    pub fn sign(
        signed_link_id: impl Into<String>,
        deep_link: SearchDeepLink,
        intent_summary: impl Into<String>,
        completeness_note: impl Into<String>,
        freshness_disclosure: SearchResultSemantics,
        scope_disclosure: SearchScopeHonestyState,
        return_anchor_ref: impl Into<String>,
        signature_scheme: DeepLinkSignatureScheme,
        signing_key_id: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        let mut signed = Self {
            signed_link_id: signed_link_id.into(),
            deep_link,
            intent_summary: intent_summary.into(),
            completeness_note: completeness_note.into(),
            freshness_disclosure,
            scope_disclosure,
            partiality_disclosed: true,
            return_anchor_ref: return_anchor_ref.into(),
            implies_live_current_certainty: false,
            signature_scheme,
            signing_key_id: signing_key_id.into(),
            signed_fields: Self::SIGNED_FIELDS
                .iter()
                .map(|f| (*f).to_string())
                .collect(),
            payload_digest: String::new(),
            signature: String::new(),
            summary: summary.into(),
        };
        signed.payload_digest = signed.expected_payload_digest();
        signed.signature = signed.expected_signature();
        signed
    }

    /// Deterministic canonical serialization of the signed disclosure fields.
    fn canonical_payload(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.deep_link.deep_link_id,
            self.deep_link.scope_binding_id_ref,
            self.intent_summary,
            self.completeness_note,
            self.freshness_disclosure.as_str(),
            self.scope_disclosure.as_str(),
            self.return_anchor_ref,
            self.deep_link.access_widening_allowed,
            self.implies_live_current_certainty,
        )
    }

    /// Content digest the signed link should carry for its current disclosure.
    pub fn expected_payload_digest(&self) -> String {
        stable_query_hash(&self.canonical_payload())
    }

    /// Key-scoped signature the link should carry for its current digest.
    pub fn expected_signature(&self) -> String {
        format!(
            "{}:{}",
            self.signing_key_id,
            stable_query_hash(&format!(
                "{}|{}",
                self.signing_key_id,
                self.expected_payload_digest()
            ))
        )
    }

    /// True when the carried digest and signature match the disclosed fields, so
    /// any tampering with the intent, scope, freshness, or return path is
    /// detectable.
    pub fn signature_verifies(&self) -> bool {
        self.payload_digest == self.expected_payload_digest()
            && self.signature == self.expected_signature()
    }

    /// True when the link discloses freshness as intent, never as live or frozen
    /// current certainty.
    pub fn freshness_is_intent_not_certainty(&self) -> bool {
        matches!(
            self.freshness_disclosure,
            SearchResultSemantics::LiveRerunRequired
                | SearchResultSemantics::ScopeChangedSinceCapture
                | SearchResultSemantics::EmptyBecauseScopeChanged
        )
    }
}

/// One row of the local-versus-synced retention matrix for a query data class.
///
/// This is the machine-readable proof that raw query text stays local-only by
/// default, and that any widening to sync, share, or export carries an explicit
/// basis and redaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalVersusSyncedRetentionRow {
    /// Query-material data class this row governs.
    pub data_class: QueryDataClass,
    /// Retention mode applied to the local copy.
    pub local_retention_mode: SearchRetentionMode,
    /// Sync class of the local copy; always [`SearchSyncClass::LocalOnly`].
    pub local_sync_class: SearchSyncClass,
    /// Redaction profile applied by default.
    pub default_redaction: SearchRedactionProfile,
    /// True only when this data class is synced without explicit opt-in.
    pub synced_by_default: bool,
    /// Retention mode applied to a synced copy, when widening is opted into.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_retention_mode: Option<SearchRetentionMode>,
    /// Sync class of a synced copy, when widening is opted into.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_sync_class: Option<SearchSyncClass>,
    /// Redaction applied before any sync, share, or export.
    pub on_sync_redaction: SearchRedactionProfile,
    /// Basis under which retention or sync may widen past the local default.
    pub widening_basis: SearchRetentionWideningBasis,
    /// True when the data class is retained beyond the live session.
    pub retained_beyond_session: bool,
    /// True when policy redaction must be applied before any export.
    pub policy_redaction_required: bool,
    /// User-visible disclosure of the retention and sync posture.
    pub disclosure: String,
}

impl LocalVersusSyncedRetentionRow {
    /// True when the row keeps raw query material local-only by default.
    pub fn local_only_by_default(&self) -> bool {
        self.local_sync_class == SearchSyncClass::LocalOnly
            && (!self.synced_by_default || !self.data_class.carries_literal_query_material())
    }
}

/// Consumer projection that reuses the governed saved-query artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceConsumerProjection {
    /// Consumer that ingests the packet.
    pub consumer: GovernanceConsumerClass,
    /// Repository-relative pointer to the consumer.
    pub consumer_ref: String,
    /// Packet id the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// True when the consumer preserves the privacy and sync class.
    pub preserves_privacy_and_sync_class: bool,
    /// True when the consumer preserves captured-vs-current scope truth.
    pub preserves_captured_vs_current_scope: bool,
    /// True when the consumer reuses the same artifact objects.
    pub reuses_same_artifacts: bool,
    /// Always `false`: shared intent is never shared authority.
    pub widens_authority: bool,
    /// True when raw query text is excluded from this projection.
    pub raw_query_text_excluded: bool,
    /// True when ambient credentials and authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Reviewable summary of how the consumer reuses the packet.
    pub summary: String,
}

/// One validation finding emitted by [`SavedQueryGovernancePacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryGovernanceValidationFinding {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Delivery-grade governance packet for saved queries, scope packs, query
/// history, retention, and signed deep links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id reused by every consumer projection.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections the packet answers to.
    pub source_spec_refs: Vec<String>,
    /// Existing lane schemas the packet composes.
    pub supporting_contract_refs: Vec<String>,
    /// Privacy classes the packet realizes.
    pub covered_privacy_classes: Vec<SavedQueryPrivacyClass>,
    /// Sync classes the packet realizes.
    pub covered_sync_classes: Vec<SearchSyncClass>,
    /// Retention modes the packet realizes.
    pub covered_retention_modes: Vec<SearchRetentionMode>,
    /// Redaction profiles the packet realizes.
    pub covered_redaction_profiles: Vec<SearchRedactionProfile>,
    /// Query-material data classes the retention matrix governs.
    pub covered_data_classes: Vec<QueryDataClass>,
    /// Governed saved-query rows, one per realized privacy class.
    pub saved_queries: Vec<GovernedSavedQueryRow>,
    /// Signed, intent-preserving deep links.
    pub signed_deep_links: Vec<SignedSearchDeepLink>,
    /// Local-versus-synced retention matrix, one row per data class.
    pub retention_matrix: Vec<LocalVersusSyncedRetentionRow>,
    /// Consumer projections that reuse the governed artifacts.
    pub consumer_projections: Vec<GovernanceConsumerProjection>,
    /// Reviewable summary of the privacy posture.
    pub export_safe_summary: String,
}

impl SavedQueryGovernancePacket {
    /// Returns the governed row for one id, if present.
    pub fn saved_query_row(&self, row_id: &str) -> Option<&GovernedSavedQueryRow> {
        self.saved_queries.iter().find(|row| row.row_id == row_id)
    }

    /// Returns the retention row for one data class, if present.
    pub fn retention_row(
        &self,
        data_class: QueryDataClass,
    ) -> Option<&LocalVersusSyncedRetentionRow> {
        self.retention_matrix
            .iter()
            .find(|row| row.data_class == data_class)
    }

    /// Privacy classes realized across every governed artifact.
    pub fn present_privacy_classes(&self) -> HashSet<SavedQueryPrivacyClass> {
        let mut present = HashSet::new();
        for row in &self.saved_queries {
            present.insert(row.saved_query.privacy_class);
            present.insert(row.scope_pack.privacy_class);
            present.insert(row.history_entry.privacy_class);
        }
        for link in &self.signed_deep_links {
            present.insert(link.deep_link.privacy_class);
        }
        present
    }

    /// Sync classes realized across every governed artifact and matrix row.
    pub fn present_sync_classes(&self) -> HashSet<SearchSyncClass> {
        let mut present = HashSet::new();
        for row in &self.saved_queries {
            present.insert(row.saved_query.sync_class);
            present.insert(row.scope_pack.sync_class);
            present.insert(row.history_entry.sync_class);
        }
        for link in &self.signed_deep_links {
            present.insert(link.deep_link.sync_class);
        }
        for row in &self.retention_matrix {
            present.insert(row.local_sync_class);
            if let Some(sync) = row.synced_sync_class {
                present.insert(sync);
            }
        }
        present
    }

    /// Retention modes realized across every governed artifact and matrix row.
    pub fn present_retention_modes(&self) -> HashSet<SearchRetentionMode> {
        let mut present = HashSet::new();
        for row in &self.saved_queries {
            present.insert(row.saved_query.retention_mode);
            present.insert(row.scope_pack.retention_mode);
            present.insert(row.history_entry.retention_mode);
        }
        for link in &self.signed_deep_links {
            present.insert(link.deep_link.retention_mode);
        }
        for row in &self.retention_matrix {
            present.insert(row.local_retention_mode);
            if let Some(mode) = row.synced_retention_mode {
                present.insert(mode);
            }
        }
        present
    }

    /// Redaction profiles realized across every governed artifact and matrix row.
    pub fn present_redaction_profiles(&self) -> HashSet<SearchRedactionProfile> {
        let mut present = HashSet::new();
        for row in &self.saved_queries {
            present.insert(row.saved_query.redaction_profile);
            present.insert(row.scope_pack.redaction_profile);
            present.insert(row.history_entry.redaction_profile);
        }
        for link in &self.signed_deep_links {
            present.insert(link.deep_link.redaction_profile);
        }
        for row in &self.retention_matrix {
            present.insert(row.default_redaction);
            present.insert(row.on_sync_redaction);
        }
        present
    }

    /// Data classes realized by the retention matrix.
    pub fn present_data_classes(&self) -> HashSet<QueryDataClass> {
        self.retention_matrix
            .iter()
            .map(|row| row.data_class)
            .collect()
    }

    /// True when every governed artifact confines raw query text to a local-only
    /// posture; sync, share, and export never carry the literal by default.
    pub fn raw_query_text_is_local_only(&self) -> bool {
        self.saved_queries
            .iter()
            .all(GovernedSavedQueryRow::raw_query_text_is_local_only)
    }

    /// True when the canonical packet carries no raw query text at all (the
    /// posture of the redacted export copy).
    pub fn contains_no_raw_query_text(&self) -> bool {
        self.saved_queries
            .iter()
            .all(|row| row.saved_query.query_text.is_none())
    }

    /// True when the packet is safe to project to sync and support consumers:
    /// it validates, confines raw text to local-only, and no consumer widens
    /// authority or carries raw text.
    pub fn is_export_safe(&self) -> bool {
        self.validate().is_empty()
            && self.raw_query_text_is_local_only()
            && self.consumer_projections.iter().all(|projection| {
                projection.raw_query_text_excluded
                    && projection.ambient_authority_excluded
                    && !projection.widens_authority
            })
    }

    /// Returns a redacted copy with all raw query text removed, as carried by a
    /// sync or support export. Hashes, scope metadata, and result refs are kept.
    pub fn redact_for_export(&self) -> Self {
        let mut redacted = self.clone();
        for row in &mut redacted.saved_queries {
            if row.saved_query.query_text.is_some() {
                row.saved_query.query_text = None;
                if row.saved_query.query_text_mode == QueryTextMode::LocalText {
                    row.saved_query.query_text_mode = QueryTextMode::HashOnly;
                }
                if row.history_entry.stored_text_mode == QueryTextMode::LocalText {
                    row.history_entry.stored_text_mode = QueryTextMode::HashOnly;
                }
            }
        }
        redacted
    }

    /// Builds a redacted support export that wraps the redacted packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SavedQueryGovernanceSupportExport {
        let redacted_packet = self.redact_for_export();
        SavedQueryGovernanceSupportExport {
            record_kind: SAVED_QUERY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SAVED_QUERY_GOVERNANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: redacted_packet.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_query_text_excluded: true,
            ambient_authority_excluded: true,
            redacted_packet,
        }
    }

    /// Validates the packet against the saved-query governance guardrails.
    ///
    /// An empty result means the matrix is fully covered, every governed
    /// artifact is internally consistent, raw query text stays local-only, and
    /// every signed deep link is verifiable, intent-only, and authority-safe.
    pub fn validate(&self) -> Vec<SavedQueryGovernanceValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != SAVED_QUERY_GOVERNANCE_PACKET_RECORD_KIND {
            push(&mut findings, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != SAVED_QUERY_GOVERNANCE_SCHEMA_VERSION {
            push(&mut findings, "schema_version", "unexpected schema_version");
        }
        if self.packet_id != SAVED_QUERY_GOVERNANCE_PACKET_ID {
            push(&mut findings, "packet_id", "unexpected packet_id");
        }
        if self.generated_at.trim().is_empty() {
            push(&mut findings, "generated_at", "generated_at is required");
        }
        if self.doc_ref != SAVED_QUERY_GOVERNANCE_DOC_REF {
            push(&mut findings, "doc_ref", "unexpected doc_ref");
        }
        if self.schema_ref != SAVED_QUERY_GOVERNANCE_SCHEMA_REF {
            push(&mut findings, "schema_ref", "unexpected schema_ref");
        }
        if self.artifact_ref != SAVED_QUERY_GOVERNANCE_ARTIFACT_REF {
            push(&mut findings, "artifact_ref", "unexpected artifact_ref");
        }
        if self.source_spec_refs.is_empty() {
            push(
                &mut findings,
                "source_spec_refs",
                "source_spec_refs is required",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut findings,
                "supporting_contract_refs",
                "supporting_contract_refs is required",
            );
        }

        self.validate_coverage(&mut findings);
        self.validate_saved_queries(&mut findings);
        self.validate_signed_deep_links(&mut findings);
        self.validate_retention_matrix(&mut findings);
        self.validate_consumers(&mut findings);

        findings
    }

    fn validate_coverage(&self, findings: &mut Vec<SavedQueryGovernanceValidationFinding>) {
        if self.covered_privacy_classes != ALL_PRIVACY_CLASSES.to_vec() {
            push(
                findings,
                "covered_privacy_classes",
                "covered_privacy_classes must list every privacy class in canonical order",
            );
        }
        if self.covered_sync_classes != ALL_SYNC_CLASSES.to_vec() {
            push(
                findings,
                "covered_sync_classes",
                "covered_sync_classes must list every sync class in canonical order",
            );
        }
        if self.covered_retention_modes != ALL_RETENTION_MODES.to_vec() {
            push(
                findings,
                "covered_retention_modes",
                "covered_retention_modes must list every retention mode in canonical order",
            );
        }
        if self.covered_redaction_profiles != ALL_REDACTION_PROFILES.to_vec() {
            push(
                findings,
                "covered_redaction_profiles",
                "covered_redaction_profiles must list every redaction profile in canonical order",
            );
        }
        if self.covered_data_classes != QueryDataClass::ALL.to_vec() {
            push(
                findings,
                "covered_data_classes",
                "covered_data_classes must list every data class in canonical order",
            );
        }

        let privacy = self.present_privacy_classes();
        for required in ALL_PRIVACY_CLASSES {
            if !privacy.contains(&required) {
                push(
                    findings,
                    "saved_queries",
                    &format!("no artifact realizes privacy class {}", required.as_str()),
                );
            }
        }
        let sync = self.present_sync_classes();
        for required in ALL_SYNC_CLASSES {
            if !sync.contains(&required) {
                push(
                    findings,
                    "retention_matrix",
                    &format!("no artifact realizes sync class {}", required.as_str()),
                );
            }
        }
        let retention = self.present_retention_modes();
        for required in ALL_RETENTION_MODES {
            if !retention.contains(&required) {
                push(
                    findings,
                    "retention_matrix",
                    &format!("no artifact realizes retention mode {}", required.as_str()),
                );
            }
        }
        let redaction = self.present_redaction_profiles();
        for required in ALL_REDACTION_PROFILES {
            if !redaction.contains(&required) {
                push(
                    findings,
                    "retention_matrix",
                    &format!(
                        "no artifact realizes redaction profile {}",
                        required.as_str()
                    ),
                );
            }
        }
    }

    fn validate_saved_queries(&self, findings: &mut Vec<SavedQueryGovernanceValidationFinding>) {
        if self.saved_queries.is_empty() {
            push(
                findings,
                "saved_queries",
                "at least one governed saved query is required",
            );
        }
        for row in &self.saved_queries {
            let base = format!("saved_queries.{}", row.row_id);

            for finding in row.saved_query.validate() {
                push(findings, &format!("{base}.saved_query"), &finding.summary);
            }
            for finding in row.scope_pack.validate() {
                push(findings, &format!("{base}.scope_pack"), &finding.summary);
            }
            for finding in row.history_entry.validate() {
                push(findings, &format!("{base}.history_entry"), &finding.summary);
            }

            // Cross-references keep the bound artifacts attributable.
            if row.saved_query.scope_binding_id_ref != row.scope_pack.scope_binding_id {
                push(
                    findings,
                    &format!("{base}.scope_pack"),
                    "saved query and scope pack must share one scope binding id",
                );
            }
            if row.history_entry.saved_query_id_ref.as_deref()
                != Some(row.saved_query.saved_query_id.as_str())
            {
                push(
                    findings,
                    &format!("{base}.history_entry"),
                    "history entry must reference its saved query",
                );
            }

            // Reopen, migration, and scope drift must be disclosed, not silent.
            if row.scope_drift.silent_semantic_break {
                push(
                    findings,
                    &format!("{base}.scope_drift"),
                    "scope drift must be disclosed, never a silent semantic break",
                );
            }
            if !row.survives_scope_drift {
                push(
                    findings,
                    &format!("{base}.survives_scope_drift"),
                    "a governed saved query must survive scope drift through disclosure",
                );
            }
            if matches!(
                row.scope_drift.result_semantics,
                SearchResultSemantics::CurrentLiveResults
            ) {
                push(
                    findings,
                    &format!("{base}.scope_drift"),
                    "scope drift must rerun before claiming current live results",
                );
            }
            if !row.scope_drift.scope_still_current() && !row.scope_drift.rerun_required {
                push(
                    findings,
                    &format!("{base}.scope_drift"),
                    "a drifted scope must require a rerun before presenting truth",
                );
            }

            // Raw query text never leaves the local-only posture.
            if !row.raw_query_text_is_local_only() {
                push(
                    findings,
                    &format!("{base}.saved_query"),
                    "raw query text must stay confined to a local-only artifact",
                );
            }
        }
    }

    fn validate_signed_deep_links(
        &self,
        findings: &mut Vec<SavedQueryGovernanceValidationFinding>,
    ) {
        if self.signed_deep_links.is_empty() {
            push(
                findings,
                "signed_deep_links",
                "at least one signed deep link is required",
            );
        }
        for link in &self.signed_deep_links {
            let base = format!("signed_deep_links.{}", link.signed_link_id);

            for finding in link.deep_link.validate() {
                push(findings, &format!("{base}.deep_link"), &finding.summary);
            }
            if !link.signature_verifies() {
                push(
                    findings,
                    &format!("{base}.signature"),
                    "signed deep link signature must verify the disclosed intent, scope, and freshness",
                );
            }
            if link.implies_live_current_certainty {
                push(
                    findings,
                    &format!("{base}.implies_live_current_certainty"),
                    "a deep link must not imply live current certainty",
                );
            }
            if !link.freshness_is_intent_not_certainty() {
                push(
                    findings,
                    &format!("{base}.freshness_disclosure"),
                    "freshness disclosure must reopen intent, not frozen or live certainty",
                );
            }
            if link.return_anchor_ref.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.return_anchor_ref"),
                    "signed deep link must preserve a supportable return path",
                );
            }
            if link.intent_summary.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.intent_summary"),
                    "signed deep link must disclose its intent",
                );
            }
            if link.completeness_note.trim().is_empty() || !link.partiality_disclosed {
                push(
                    findings,
                    &format!("{base}.completeness_note"),
                    "signed deep link must disclose completeness and partiality",
                );
            }
            if link.deep_link.access_widening_allowed {
                push(
                    findings,
                    &format!("{base}.deep_link"),
                    "a signed deep link must not widen access — shared intent is not shared authority",
                );
            }
        }
    }

    fn validate_retention_matrix(&self, findings: &mut Vec<SavedQueryGovernanceValidationFinding>) {
        for required in QueryDataClass::ALL {
            let count = self
                .retention_matrix
                .iter()
                .filter(|row| row.data_class == required)
                .count();
            if count != 1 {
                push(
                    findings,
                    "retention_matrix",
                    &format!("data class {} must appear exactly once", required.as_str()),
                );
            }
        }

        for row in &self.retention_matrix {
            let base = format!("retention_matrix.{}", row.data_class.as_str());

            if row.local_sync_class != SearchSyncClass::LocalOnly {
                push(
                    findings,
                    &base,
                    "the local copy of every data class must stay local-only",
                );
            }
            if !row.local_only_by_default() {
                push(
                    findings,
                    &base,
                    "literal query material must stay local-only by default",
                );
            }
            // Any widening to sync must carry an explicit, non-default basis.
            let widens = row
                .synced_retention_mode
                .is_some_and(SearchRetentionMode::widens_local_default)
                || row
                    .synced_sync_class
                    .is_some_and(SearchSyncClass::widens_local_default);
            if widens
                && matches!(
                    row.widening_basis,
                    SearchRetentionWideningBasis::NotWidenedLocalDefault
                )
            {
                push(
                    findings,
                    &base,
                    "a synced data class must carry an explicit widening basis",
                );
            }
        }

        // Raw query text is the strict invariant: local-only, never synced by
        // default, literal kept only in local storage.
        if let Some(raw) = self.retention_row(QueryDataClass::RawQueryText) {
            if raw.synced_by_default {
                push(
                    findings,
                    "retention_matrix.raw_query_text",
                    "raw query text must never sync by default",
                );
            }
            if raw.local_retention_mode != SearchRetentionMode::LocalOnlyDefault {
                push(
                    findings,
                    "retention_matrix.raw_query_text",
                    "raw query text must default to local-only retention",
                );
            }
            if raw.default_redaction != SearchRedactionProfile::LiteralLocalOnly {
                push(
                    findings,
                    "retention_matrix.raw_query_text",
                    "raw query text must default to the local-only literal profile",
                );
            }
        } else {
            push(
                findings,
                "retention_matrix",
                "the retention matrix must govern raw query text",
            );
        }
    }

    fn validate_consumers(&self, findings: &mut Vec<SavedQueryGovernanceValidationFinding>) {
        for required in GovernanceConsumerClass::ALL {
            if !self
                .consumer_projections
                .iter()
                .any(|projection| projection.consumer == required)
            {
                push(
                    findings,
                    "consumer_projections",
                    &format!("missing consumer {}", required.as_str()),
                );
            }
        }
        for projection in &self.consumer_projections {
            let base = format!("consumer_projections.{}", projection.consumer.as_str());
            if projection.ingested_packet_id != self.packet_id {
                push(
                    findings,
                    &base,
                    "consumer must ingest the packet id verbatim",
                );
            }
            if !projection.preserves_privacy_and_sync_class {
                push(
                    findings,
                    &base,
                    "consumer must preserve the privacy and sync class",
                );
            }
            if !projection.preserves_captured_vs_current_scope {
                push(
                    findings,
                    &base,
                    "consumer must preserve captured-vs-current scope truth",
                );
            }
            if !projection.reuses_same_artifacts {
                push(
                    findings,
                    &base,
                    "consumer must reuse the same artifact objects",
                );
            }
            if projection.widens_authority {
                push(findings, &base, "consumer must not widen authority");
            }
            if !projection.raw_query_text_excluded {
                push(findings, &base, "consumer must exclude raw query text");
            }
            if !projection.ambient_authority_excluded {
                push(findings, &base, "consumer must exclude ambient authority");
            }
        }
    }
}

/// Redacted support export that wraps the redacted governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryGovernanceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Product packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw query text is excluded.
    pub raw_query_text_excluded: bool,
    /// True when ambient credentials and authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Redacted packet preserved by the export.
    pub redacted_packet: SavedQueryGovernancePacket,
}

impl SavedQueryGovernanceSupportExport {
    /// True when the export preserves the packet safely with no raw query text.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SAVED_QUERY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SAVED_QUERY_GOVERNANCE_SCHEMA_VERSION
            && self.packet_id_ref == self.redacted_packet.packet_id
            && self.raw_query_text_excluded
            && self.ambient_authority_excluded
            && self.redacted_packet.validate().is_empty()
            && self.redacted_packet.contains_no_raw_query_text()
    }
}

/// Errors returned when reading the checked-in governance packet.
#[derive(Debug)]
pub enum SavedQueryGovernanceArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SavedQueryGovernanceValidationFinding>),
}

impl fmt::Display for SavedQueryGovernanceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "saved-query governance packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.path.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "saved-query governance packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SavedQueryGovernanceArtifactError {}

/// Returns the checked-in canonical governance packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_saved_query_governance_packet(
) -> Result<SavedQueryGovernancePacket, SavedQueryGovernanceArtifactError> {
    let packet: SavedQueryGovernancePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/saved-query-retention/packet.json"
    )))
    .map_err(SavedQueryGovernanceArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SavedQueryGovernanceArtifactError::Validation(findings))
    }
}

/// Returns the canonical seeded governance packet.
pub fn seeded_saved_query_governance_packet() -> SavedQueryGovernancePacket {
    let saved_queries = vec![
        build_governed_row(GovernedRowSeed {
            row_id: "saved-query:local-private",
            privacy_class: SavedQueryPrivacyClass::LocalOnlyPrivate,
            source_class: SavedQuerySourceClass::UserAuthored,
            share_policy: SavedQuerySharePolicy::LocalOnlyNoShare,
            surface: SearchSurface::QuickOpen,
            query_text: "retry budget",
            scope_class: ScopeClass::CurrentRepo,
            scope_label: "Current repo",
            display_name: "Retry budget triage",
            drift: ScopeDriftSeed::StillCurrent,
            migrated: false,
            summary: "A local-only saved query keeps its literal query text on device and reopens against the still-current captured scope.",
        }),
        build_governed_row(GovernedRowSeed {
            row_id: "saved-query:workspace-shared",
            privacy_class: SavedQueryPrivacyClass::WorkspaceSharedRedacted,
            source_class: SavedQuerySourceClass::TeamShared,
            share_policy: SavedQuerySharePolicy::WorkspaceShareExplicit,
            surface: SearchSurface::FileSearch,
            query_text: "kind:file flaky",
            scope_class: ScopeClass::FullWorkspace,
            scope_label: "Full workspace",
            display_name: "Flaky file sweep",
            drift: ScopeDriftSeed::StillCurrent,
            migrated: false,
            summary: "A workspace-shared saved query carries hashes and scope metadata only and reopens for teammates under their own permissions.",
        }),
        build_governed_row(GovernedRowSeed {
            row_id: "saved-query:support-redacted",
            privacy_class: SavedQueryPrivacyClass::SupportExportRedacted,
            source_class: SavedQuerySourceClass::SupportCaptured,
            share_policy: SavedQuerySharePolicy::SupportExportRedactedOnly,
            surface: SearchSurface::SymbolSearch,
            query_text: "SearchPlanner",
            scope_class: ScopeClass::SelectedWorkset,
            scope_label: "Triage workset",
            display_name: "Planner symbol triage",
            drift: ScopeDriftSeed::Drifted,
            migrated: false,
            summary: "A support-captured saved query travels only inside a redacted export and discloses that its captured scope drifted and must rebind before reopening.",
        }),
        build_governed_row(GovernedRowSeed {
            row_id: "saved-query:policy-withheld",
            privacy_class: SavedQueryPrivacyClass::PolicyWithheld,
            source_class: SavedQuerySourceClass::PolicyProvided,
            share_policy: SavedQuerySharePolicy::ShareDisabledByPolicy,
            surface: SearchSurface::DocsSearch,
            query_text: "auth policy",
            scope_class: ScopeClass::PolicyLimitedView,
            scope_label: "Policy-limited view",
            display_name: "Auth policy lookup",
            drift: ScopeDriftSeed::StillCurrent,
            migrated: true,
            summary: "A policy-withheld saved query carries neither literal nor hash query material, survives a schema migration, and is owned and distributed by policy.",
        }),
    ];

    let signed_deep_links = vec![
        SignedSearchDeepLink::sign(
            "signed-link:local-private",
            SearchDeepLink::for_saved_query(
                "deep-link:local-private",
                &saved_queries[0].saved_query,
                None,
                SEEDED_GENERATED_AT,
            ),
            "Reopen the local retry-budget triage query",
            "Captured while the hot set was ready; cold paths may still be warming, so results reopen as intent, not frozen truth.",
            SearchResultSemantics::LiveRerunRequired,
            SearchScopeHonestyState::RecipientMustReResolve,
            "return:quick_open:caret:0",
            DeepLinkSignatureScheme::LocalContentDigest,
            "local:device:search-link",
            "A local content-digest signed deep link reopens the saved query's intent on this device and never claims live certainty.",
        ),
        SignedSearchDeepLink::sign(
            "signed-link:workspace-shared",
            SearchDeepLink::for_saved_query(
                "deep-link:workspace-shared",
                &saved_queries[1].saved_query,
                Some("2026-09-17T00:00:00Z".to_string()),
                SEEDED_GENERATED_AT,
            ),
            "Reopen the workspace flaky-file sweep for a teammate",
            "The workspace scope changed since capture; the recipient must re-resolve under current permissions and some matches may be hidden.",
            SearchResultSemantics::ScopeChangedSinceCapture,
            SearchScopeHonestyState::RecipientMustReResolve,
            "return:file_search:list:0",
            DeepLinkSignatureScheme::WorkspaceSignedDigest,
            "workspace:aureline:search-link",
            "A workspace-signed deep link reopens shared intent under the recipient's own permissions and discloses that the scope drifted.",
        ),
        SignedSearchDeepLink::sign(
            "signed-link:policy-withheld",
            SearchDeepLink::for_saved_query(
                "deep-link:policy-withheld",
                &saved_queries[3].saved_query,
                Some("2026-12-17T00:00:00Z".to_string()),
                SEEDED_GENERATED_AT,
            ),
            "Reopen the policy-owned auth-policy lookup intent",
            "Policy withholds the literal query; the link reopens scoped intent only and may return no rows under the current policy view.",
            SearchResultSemantics::EmptyBecauseScopeChanged,
            SearchScopeHonestyState::CurrentScopeNarrowerDisclosed,
            "return:docs_search:caret:0",
            DeepLinkSignatureScheme::PolicySignedDigest,
            "policy:aureline:search-link",
            "A policy-signed deep link reopens intent inside the policy-limited view and discloses that it may return no rows.",
        ),
    ];

    let retention_matrix = seeded_retention_matrix();
    let consumer_projections = seeded_consumer_projections();

    SavedQueryGovernancePacket {
        record_kind: SAVED_QUERY_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: SAVED_QUERY_GOVERNANCE_SCHEMA_VERSION,
        packet_id: SAVED_QUERY_GOVERNANCE_PACKET_ID.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        doc_ref: SAVED_QUERY_GOVERNANCE_DOC_REF.to_owned(),
        schema_ref: SAVED_QUERY_GOVERNANCE_SCHEMA_REF.to_owned(),
        artifact_ref: SAVED_QUERY_GOVERNANCE_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/search/saved_query.schema.json".to_owned(),
            "schemas/search/query_history.schema.json".to_owned(),
            "schemas/search/saved_query_and_scope_binding.schema.json".to_owned(),
            "schemas/search/search_export_snapshot.schema.json".to_owned(),
            "schemas/search/query_session.schema.json".to_owned(),
        ],
        covered_privacy_classes: ALL_PRIVACY_CLASSES.to_vec(),
        covered_sync_classes: ALL_SYNC_CLASSES.to_vec(),
        covered_retention_modes: ALL_RETENTION_MODES.to_vec(),
        covered_redaction_profiles: ALL_REDACTION_PROFILES.to_vec(),
        covered_data_classes: QueryDataClass::ALL.to_vec(),
        saved_queries,
        signed_deep_links,
        retention_matrix,
        consumer_projections,
        export_safe_summary:
            "Raw query text stays local-only by default; saved queries, scope packs, history, and signed deep links carry privacy, sync, and captured-vs-current scope truth, and sync, share, and support export carry redacted metadata only."
                .to_owned(),
    }
}

/// Returns the seeded redacted export copy of the canonical packet.
pub fn seeded_redacted_export_packet() -> SavedQueryGovernancePacket {
    seeded_saved_query_governance_packet().redact_for_export()
}

/// Seed for one governed saved-query row.
struct GovernedRowSeed {
    row_id: &'static str,
    privacy_class: SavedQueryPrivacyClass,
    source_class: SavedQuerySourceClass,
    share_policy: SavedQuerySharePolicy,
    surface: SearchSurface,
    query_text: &'static str,
    scope_class: ScopeClass,
    scope_label: &'static str,
    display_name: &'static str,
    drift: ScopeDriftSeed,
    migrated: bool,
    summary: &'static str,
}

/// Whether a seeded row's captured scope still matches the current scope.
#[derive(Debug, Clone, Copy)]
enum ScopeDriftSeed {
    StillCurrent,
    Drifted,
}

fn build_governed_row(seed: GovernedRowSeed) -> GovernedSavedQueryRow {
    let session = crate::query_session::SearchQuerySession::for_local_text(
        format!("search:session:{}", seed.row_id),
        seed.surface,
        seed.query_text,
        seed.scope_class,
        seed.scope_label,
        "search-planner-alpha",
        "hot_set_ready",
        SEEDED_GENERATED_AT,
    );

    let retention_mode = SearchRetentionMode::default_for(seed.source_class, seed.privacy_class);
    let sync_class = SearchSyncClass::default_for(seed.source_class, seed.privacy_class);
    let redaction_profile = SearchRedactionProfile::default_for(seed.privacy_class);
    let widening_basis =
        SearchRetentionWideningBasis::default_for(seed.source_class, retention_mode, sync_class);

    let scope_binding_id = format!("scope-pack:{}", seed.row_id);
    let mut scope_pack = ScopePackBinding::from_query_session(
        scope_binding_id.clone(),
        &session,
        seed.source_class,
        seed.privacy_class,
        retention_mode,
        sync_class,
        redaction_profile,
        widening_basis,
        SEEDED_GENERATED_AT,
    );

    let record = SavedQueryRecord::from_session(SavedQueryRecordInputs {
        saved_query_id: format!("saved-query-record:{}", seed.row_id),
        source_class: seed.source_class,
        privacy_class: seed.privacy_class,
        share_policy: seed.share_policy,
        query_session: session,
        policy_epoch: None,
        created_at: SEEDED_GENERATED_AT.to_string(),
    });

    let mut saved_query = SavedQuery::from_saved_query_record(
        &record,
        seed.display_name,
        scope_binding_id.clone(),
        retention_mode,
        sync_class,
        redaction_profile,
        widening_basis,
    );
    if seed.migrated {
        saved_query.migration_state =
            crate::query_artifacts::SearchArtifactMigrationState::MigratedFromPreviousVersion;
    }

    let captured_scope_id = saved_query.stable_scope_id.clone();
    let scope_drift = match seed.drift {
        ScopeDriftSeed::StillCurrent => ScopeDriftDisclosure {
            captured_stable_scope_id: captured_scope_id.clone(),
            current_stable_scope_id: captured_scope_id,
            scope_honesty_state: SearchScopeHonestyState::CapturedScopeStillCurrent,
            result_semantics: SearchResultSemantics::LiveRerunRequired,
            rerun_required: true,
            effective_scope_narrowed_to_captured: false,
            silent_semantic_break: false,
            disclosure:
                "The captured scope still matches the current scope; reopening reruns to confirm current truth."
                    .to_string(),
        },
        ScopeDriftSeed::Drifted => {
            let current_scope_id = format!("{captured_scope_id}:changed");
            saved_query.result_semantics = SearchResultSemantics::ScopeChangedSinceCapture;
            saved_query.scope_honesty_state =
                SearchScopeHonestyState::CurrentScopeChangedRebindRequired;
            scope_pack.scope_honesty_state =
                SearchScopeHonestyState::CurrentScopeChangedRebindRequired;
            ScopeDriftDisclosure {
                captured_stable_scope_id: captured_scope_id,
                current_stable_scope_id: current_scope_id,
                scope_honesty_state: SearchScopeHonestyState::CurrentScopeChangedRebindRequired,
                result_semantics: SearchResultSemantics::ScopeChangedSinceCapture,
                rerun_required: true,
                effective_scope_narrowed_to_captured: false,
                silent_semantic_break: false,
                disclosure:
                    "The captured scope changed since the query was saved; reopening rebinds to the current scope and reruns before claiming truth."
                        .to_string(),
            }
        }
    };

    let expires_at =
        matches!(seed.drift, ScopeDriftSeed::Drifted).then(|| "2026-12-17T00:00:00Z".to_string());
    let history_entry = QueryHistoryEntry::from_saved_query(
        format!("history:{}", seed.row_id),
        &saved_query,
        SEEDED_GENERATED_AT,
        expires_at,
    );

    GovernedSavedQueryRow {
        row_id: seed.row_id.to_string(),
        saved_query,
        scope_pack,
        history_entry,
        survives_reopen: true,
        survives_migration: true,
        survives_scope_drift: true,
        scope_drift,
        summary: seed.summary.to_string(),
    }
}

fn seeded_retention_matrix() -> Vec<LocalVersusSyncedRetentionRow> {
    vec![
        LocalVersusSyncedRetentionRow {
            data_class: QueryDataClass::RawQueryText,
            local_retention_mode: SearchRetentionMode::LocalOnlyDefault,
            local_sync_class: SearchSyncClass::LocalOnly,
            default_redaction: SearchRedactionProfile::LiteralLocalOnly,
            synced_by_default: false,
            synced_retention_mode: Some(SearchRetentionMode::WorkspaceSharedExplicit),
            synced_sync_class: Some(SearchSyncClass::ExplicitUserSync),
            on_sync_redaction: SearchRedactionProfile::ExplicitLiteralConsent,
            widening_basis: SearchRetentionWideningBasis::ExplicitUserOptIn,
            retained_beyond_session: true,
            policy_redaction_required: true,
            disclosure:
                "Raw query text stays on device by default; it leaves only under an explicit user opt-in with literal consent."
                    .to_string(),
        },
        LocalVersusSyncedRetentionRow {
            data_class: QueryDataClass::QueryHash,
            local_retention_mode: SearchRetentionMode::LocalOnlyDefault,
            local_sync_class: SearchSyncClass::LocalOnly,
            default_redaction: SearchRedactionProfile::HashesScopeAndResultRefs,
            synced_by_default: false,
            synced_retention_mode: Some(SearchRetentionMode::WorkspaceSharedExplicit),
            synced_sync_class: Some(SearchSyncClass::WorkspaceShared),
            on_sync_redaction: SearchRedactionProfile::HashesScopeAndResultRefs,
            widening_basis: SearchRetentionWideningBasis::TeamSharedArtifact,
            retained_beyond_session: true,
            policy_redaction_required: false,
            disclosure:
                "Deterministic query hashes stay local-only by default and sync only with an explicit workspace-share basis."
                    .to_string(),
        },
        LocalVersusSyncedRetentionRow {
            data_class: QueryDataClass::ParsedQueryAst,
            local_retention_mode: SearchRetentionMode::LocalOnlyDefault,
            local_sync_class: SearchSyncClass::LocalOnly,
            default_redaction: SearchRedactionProfile::MetadataOnlyNoQueryMaterial,
            synced_by_default: false,
            synced_retention_mode: Some(SearchRetentionMode::WorkspaceSharedExplicit),
            synced_sync_class: Some(SearchSyncClass::WorkspaceShared),
            on_sync_redaction: SearchRedactionProfile::MetadataOnlyNoQueryMaterial,
            widening_basis: SearchRetentionWideningBasis::TeamSharedArtifact,
            retained_beyond_session: true,
            policy_redaction_required: true,
            disclosure:
                "The parsed query grammar is redaction-safe; literal fragments stay local and only the metadata-only form syncs."
                    .to_string(),
        },
        LocalVersusSyncedRetentionRow {
            data_class: QueryDataClass::ScopeMetadata,
            local_retention_mode: SearchRetentionMode::LocalOnlyDefault,
            local_sync_class: SearchSyncClass::LocalOnly,
            default_redaction: SearchRedactionProfile::MetadataOnlyNoQueryMaterial,
            synced_by_default: true,
            synced_retention_mode: Some(SearchRetentionMode::RepoProvidedReadOnly),
            synced_sync_class: Some(SearchSyncClass::RepoProvided),
            on_sync_redaction: SearchRedactionProfile::MetadataOnlyNoQueryMaterial,
            widening_basis: SearchRetentionWideningBasis::RepoProvidedArtifact,
            retained_beyond_session: true,
            policy_redaction_required: false,
            disclosure:
                "Scope identity, mode, and chip labels are metadata-only and may travel with repo-provided read-only scope packs."
                    .to_string(),
        },
        LocalVersusSyncedRetentionRow {
            data_class: QueryDataClass::ResultRefs,
            local_retention_mode: SearchRetentionMode::LocalOnlyEphemeral,
            local_sync_class: SearchSyncClass::LocalOnly,
            default_redaction: SearchRedactionProfile::HashesScopeAndResultRefs,
            synced_by_default: false,
            synced_retention_mode: Some(SearchRetentionMode::SupportExportBounded),
            synced_sync_class: Some(SearchSyncClass::SupportExportOnly),
            on_sync_redaction: SearchRedactionProfile::HashesScopeAndResultRefs,
            widening_basis: SearchRetentionWideningBasis::SupportCaseExport,
            retained_beyond_session: false,
            policy_redaction_required: false,
            disclosure:
                "Result references are ephemeral local refs; they leave only inside a bounded, redacted support export."
                    .to_string(),
        },
    ]
}

fn seeded_consumer_projections() -> Vec<GovernanceConsumerProjection> {
    let make = |consumer: GovernanceConsumerClass,
                consumer_ref: &str,
                raw_query_text_excluded: bool,
                summary: &str| GovernanceConsumerProjection {
        consumer,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: SAVED_QUERY_GOVERNANCE_PACKET_ID.to_owned(),
        preserves_privacy_and_sync_class: true,
        preserves_captured_vs_current_scope: true,
        reuses_same_artifacts: true,
        widens_authority: false,
        raw_query_text_excluded,
        ambient_authority_excluded: true,
        summary: summary.to_owned(),
    };

    vec![
        make(
            GovernanceConsumerClass::ProductUi,
            "crates/aureline-shell/src/saved_query_governance/mod.rs",
            true,
            "The desktop saved-query, history, and deep-link chrome reuses the governed artifacts and renders captured-vs-current scope truth without reading raw query text.",
        ),
        make(
            GovernanceConsumerClass::SyncPortability,
            "docs/search/saved-query-governance.md",
            true,
            "The sync and portability lane carries the artifacts with their privacy and sync class intact and never syncs raw query text by default.",
        ),
        make(
            GovernanceConsumerClass::SupportExport,
            "artifacts/search/m5/saved-query-governance.md",
            true,
            "Support export wraps the redacted packet so a bundle inspects the governed artifacts with no raw query material.",
        ),
    ]
}

fn push(findings: &mut Vec<SavedQueryGovernanceValidationFinding>, path: &str, message: &str) {
    findings.push(SavedQueryGovernanceValidationFinding {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests;
