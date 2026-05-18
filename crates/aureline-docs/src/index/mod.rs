//! Docs-pack search index records.
//!
//! This module turns loaded [`DocsPack`](crate::DocsPack) values into a small,
//! searchable index over docs nodes. It preserves the citation identity from
//! [`crate::citations`] and emits canonical docs-anchor refs plus the
//! `docs_anchor` result-kind token consumed by `aureline-search` when it mints
//! surface-specific result IDs.

use serde::{Deserialize, Serialize};

use crate::{
    CitationAnchorAvailability, CitationLocalityClass, DocsExampleValidationClass,
    DocsExternalOpenFallback, DocsFreshnessClass, DocsKnowledgeObjectKind,
    DocsKnowledgeSurfaceKind, DocsKnowledgeSurfaceProjection, DocsKnowledgeSurfaceProjectionInput,
    DocsMirrorOfflinePosture, DocsNodeIdentity, DocsNodeProvenance, DocsNodeProvenanceInput,
    DocsPack, DocsPackNode, VersionMatchState,
};

/// Schema version for docs search index records.
pub const DOCS_SEARCH_INDEX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DocsSearchIndex`] payloads.
pub const DOCS_SEARCH_INDEX_RECORD_KIND: &str = "docs_search_index_record";

/// Stable record-kind tag carried by [`DocsSearchIndexEntry`] payloads.
pub const DOCS_SEARCH_INDEX_ENTRY_RECORD_KIND: &str = "docs_search_index_entry";

/// Stable record-kind tag carried by [`DocsSearchQueryResult`] payloads.
pub const DOCS_SEARCH_QUERY_RESULT_RECORD_KIND: &str = "docs_search_query_result";

/// Search result kind token shared with search surface identity builders.
pub const DOCS_SEARCH_RESULT_KIND_TOKEN: &str = "docs_anchor";

/// Loaded docs-pack search index with partial-index truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchIndex {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable index id.
    pub index_id: String,
    /// Workspace or product authority that owns this index.
    pub workspace_id: String,
    /// Index epoch, pack set digest, or snapshot ref.
    pub index_epoch: String,
    /// Pack ids included in this index.
    pub pack_refs: Vec<String>,
    /// True when the index is known to cover only part of the declared docs scope.
    pub partial_index: bool,
    /// Stable partial-truth causes preserved for search planner projections.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Searchable docs entries.
    pub entries: Vec<DocsSearchIndexEntry>,
}

impl DocsSearchIndex {
    /// Builds an index from one loaded docs pack.
    pub fn from_pack(
        workspace_id: impl Into<String>,
        index_epoch: impl Into<String>,
        pack: DocsPack,
    ) -> Self {
        Self::from_packs(workspace_id, index_epoch, [pack])
    }

    /// Builds an index from loaded docs packs.
    pub fn from_packs<I>(
        workspace_id: impl Into<String>,
        index_epoch: impl Into<String>,
        packs: I,
    ) -> Self
    where
        I: IntoIterator<Item = DocsPack>,
    {
        let workspace_id = workspace_id.into();
        let index_epoch = index_epoch.into();
        let mut pack_refs = Vec::new();
        let mut entries = Vec::new();
        for pack in packs {
            pack_refs.push(pack.pack_id.clone());
            entries.extend(
                pack.nodes
                    .iter()
                    .map(|node| DocsSearchIndexEntry::from_pack_node(&workspace_id, &pack, node)),
            );
        }
        entries.sort_by(|a, b| {
            a.title
                .to_ascii_lowercase()
                .cmp(&b.title.to_ascii_lowercase())
                .then_with(|| a.canonical_ref.cmp(&b.canonical_ref))
        });
        Self {
            record_kind: DOCS_SEARCH_INDEX_RECORD_KIND.to_owned(),
            schema_version: DOCS_SEARCH_INDEX_SCHEMA_VERSION,
            index_id: format!(
                "docs-search-index:{}:{}",
                sanitize_ref(&workspace_id),
                sanitize_ref(&index_epoch)
            ),
            workspace_id,
            index_epoch,
            pack_refs,
            partial_index: false,
            partial_truth_causes: Vec::new(),
            entries,
        }
    }

    /// Attaches partial-index causes without changing entry identity.
    pub fn with_partial_truth<I, S>(mut self, causes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for cause in causes {
            let cause = cause.into();
            if !cause.trim().is_empty() && !self.partial_truth_causes.contains(&cause) {
                self.partial_truth_causes.push(cause);
            }
        }
        self.partial_index = !self.partial_truth_causes.is_empty();
        self
    }

    /// Searches title, summary, body, source refs, docs-node ids, and citation anchors.
    pub fn query(&self, query: impl AsRef<str>) -> DocsSearchQueryResult {
        let query = query.as_ref().to_owned();
        let normalized = normalize_search_text(&query);
        let mut scored = self
            .entries
            .iter()
            .filter_map(|entry| {
                entry
                    .match_score(&normalized)
                    .map(|score| (score, entry.clone()))
            })
            .collect::<Vec<_>>();
        scored.sort_by(|(left_score, left), (right_score, right)| {
            left_score
                .cmp(right_score)
                .then_with(|| {
                    left.title
                        .to_ascii_lowercase()
                        .cmp(&right.title.to_ascii_lowercase())
                })
                .then_with(|| left.canonical_ref.cmp(&right.canonical_ref))
        });
        let entries = scored
            .into_iter()
            .map(|(_, entry)| entry)
            .collect::<Vec<_>>();
        DocsSearchQueryResult {
            record_kind: DOCS_SEARCH_QUERY_RESULT_RECORD_KIND.to_owned(),
            schema_version: DOCS_SEARCH_INDEX_SCHEMA_VERSION,
            index_id_ref: self.index_id.clone(),
            workspace_id: self.workspace_id.clone(),
            index_epoch: self.index_epoch.clone(),
            query,
            total_entry_count: self.entries.len(),
            matched_entry_count: entries.len(),
            partial_index: self.partial_index,
            partial_truth_causes: self.partial_truth_causes.clone(),
            entries,
        }
    }
}

/// One searchable docs node and its citation/opening truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchIndexEntry {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Workspace or product authority that owns this row.
    pub workspace_id: String,
    /// Token consumed by search identity builders.
    pub result_kind_token: String,
    /// Canonical docs target ref used for search result fusion.
    pub canonical_ref: String,
    /// Resolved docs-node identity.
    pub docs_node: DocsNodeIdentity,
    /// User-visible title.
    pub title: String,
    /// Support-safe summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Source ref used to reconstruct the source material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Searchable text retained for local indexing.
    pub search_text: String,
    /// Primary citation anchor opened by an exact docs result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_anchor_ref: Option<String>,
    /// Citation anchor refs backing the row.
    pub citation_anchor_refs: Vec<String>,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Source class token for compact shell/search rows.
    pub source_class_token: String,
    /// Freshness token for compact shell/search rows.
    pub freshness_class_token: String,
    /// Freshness badge label for result chrome.
    pub freshness_badge: String,
    /// Version-match token for compact shell/search rows.
    pub version_match_state_token: String,
    /// Version-match badge label for result chrome.
    pub version_match_badge: String,
    /// Locality token for compact shell/search rows.
    pub locality_class_token: String,
    /// Citation-anchor availability token for compact shell/search rows.
    pub citation_anchor_availability_token: String,
    /// Hidden, omitted, fallback, or missing-anchor disclosure note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_note: Option<String>,
    /// Shared knowledge-surface provenance consumed by docs browser, search, help, AI, and support.
    pub docs_node_provenance: DocsNodeProvenance,
    /// Surface projection preserving source-strip, citation, and keyboard-open truth.
    pub knowledge_surface_projection: DocsKnowledgeSurfaceProjection,
}

impl DocsSearchIndexEntry {
    fn from_pack_node(workspace_id: &str, pack: &DocsPack, node: &DocsPackNode) -> Self {
        let docs_node = node.docs_node.clone();
        let canonical_ref = canonical_ref_for_node(&docs_node);
        let pack_source_ref = pack.index_source_ref();
        let docs_node_provenance =
            docs_node_provenance_for_search(pack, &canonical_ref, &docs_node);
        let knowledge_surface_projection =
            DocsKnowledgeSurfaceProjection::new(DocsKnowledgeSurfaceProjectionInput {
                surface_kind: DocsKnowledgeSurfaceKind::DocsBackedSearch,
                surface_id_ref: canonical_ref.clone(),
                provenance: docs_node_provenance.clone(),
                citation_inspection_action_ref: format!(
                    "action:inspect-citations:{}",
                    sanitize_ref(&canonical_ref)
                ),
                open_supporting_source_action_ref: Some(format!(
                    "action:open-supporting-source:{}",
                    sanitize_ref(&canonical_ref)
                )),
                keyboard_accessible_actions: true,
                export_packet_refs: vec![format!(
                    "docs-evidence-packet:search:{}",
                    sanitize_ref(&canonical_ref)
                )],
            });
        let primary_anchor_ref = docs_node
            .citation_availability
            .is_exact()
            .then(|| docs_node.citation_anchor_refs.first().cloned())
            .flatten();
        let joined_anchors = docs_node.citation_anchor_refs.join(" ");
        let search_text = [
            node.title.as_str(),
            node.summary.as_deref().unwrap_or_default(),
            node.source_ref.as_deref().unwrap_or(&pack_source_ref),
            node.body_markdown.as_str(),
            docs_node.docs_node_id.as_str(),
            docs_node.exact_reopen_ref.as_str(),
            joined_anchors.as_str(),
        ]
        .join("\n");
        Self {
            record_kind: DOCS_SEARCH_INDEX_ENTRY_RECORD_KIND.to_owned(),
            schema_version: DOCS_SEARCH_INDEX_SCHEMA_VERSION,
            workspace_id: workspace_id.to_owned(),
            result_kind_token: DOCS_SEARCH_RESULT_KIND_TOKEN.to_owned(),
            canonical_ref,
            docs_node: docs_node.clone(),
            title: node.title.clone(),
            summary: node.summary.clone(),
            source_ref: node
                .source_ref
                .clone()
                .or_else(|| Some(pack_source_ref.clone())),
            search_text,
            primary_anchor_ref,
            citation_anchor_refs: docs_node.citation_anchor_refs.clone(),
            exact_reopen_ref: docs_node.exact_reopen_ref.clone(),
            source_class_token: docs_node.source_class.as_str().to_owned(),
            freshness_class_token: docs_node.freshness_class.as_str().to_owned(),
            freshness_badge: freshness_badge(docs_node.freshness_class).to_owned(),
            version_match_state_token: docs_node.version_match_state.as_str().to_owned(),
            version_match_badge: version_match_badge(docs_node.version_match_state).to_owned(),
            locality_class_token: docs_node.locality_class.as_str().to_owned(),
            citation_anchor_availability_token: docs_node.citation_availability.as_str().to_owned(),
            hidden_or_omitted_note: docs_node.hidden_or_omitted_note.clone(),
            docs_node_provenance,
            knowledge_surface_projection,
        }
    }

    /// Returns true when the row can open an exact citation anchor.
    pub fn opens_exact_anchor(&self) -> bool {
        self.primary_anchor_ref.is_some()
            && self.docs_node.citation_availability
                == CitationAnchorAvailability::ExactAnchorAvailable
    }

    /// Returns true when freshness, version, locality, locale, or anchor state lowers certainty.
    pub fn degrades_result(&self) -> bool {
        self.docs_node.degrades_certainty()
    }

    /// Returns partial-truth cause tokens implied by this row.
    pub fn partial_truth_causes(&self) -> Vec<String> {
        let mut causes = Vec::new();
        if !self.docs_node.citation_availability.is_exact() {
            push_unique(&mut causes, self.docs_node.citation_availability.as_str());
        }
        if self.docs_node.freshness_class.lowers_certainty() {
            push_unique(&mut causes, self.docs_node.freshness_class.as_str());
        }
        if self.docs_node.version_match_state != VersionMatchState::ExactBuildMatch {
            push_unique(&mut causes, self.docs_node.version_match_state.as_str());
        }
        if self.docs_node.locality_class == CitationLocalityClass::NotInstalled {
            push_unique(&mut causes, self.docs_node.locality_class.as_str());
        }
        if self
            .docs_node
            .locale_overlay_state
            .requires_source_language_fallback()
        {
            push_unique(&mut causes, self.docs_node.locale_overlay_state.as_str());
        }
        causes
    }

    fn match_score(&self, normalized_query: &str) -> Option<u16> {
        if normalized_query.is_empty() {
            return Some(100);
        }
        let title = normalize_search_text(&self.title);
        let summary = normalize_search_text(self.summary.as_deref().unwrap_or_default());
        let text = normalize_search_text(&self.search_text);
        if title == normalized_query {
            Some(0)
        } else if title.starts_with(normalized_query) {
            Some(10)
        } else if title.contains(normalized_query) {
            Some(20)
        } else if summary.contains(normalized_query) {
            Some(40)
        } else if text.contains(normalized_query) {
            Some(60)
        } else {
            None
        }
    }
}

/// Query result over a docs search index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchQueryResult {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Index id that answered the query.
    pub index_id_ref: String,
    /// Workspace or product authority that owns this result.
    pub workspace_id: String,
    /// Index epoch, pack set digest, or snapshot ref.
    pub index_epoch: String,
    /// Query text used for this local projection.
    pub query: String,
    /// Total entries available in the index.
    pub total_entry_count: usize,
    /// Entries returned by this query.
    pub matched_entry_count: usize,
    /// True when the answering index is not full-scope proof.
    pub partial_index: bool,
    /// Stable partial-truth causes preserved for search planner projections.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Matching docs entries in rank order.
    pub entries: Vec<DocsSearchIndexEntry>,
}

fn canonical_ref_for_node(node: &DocsNodeIdentity) -> String {
    if node.citation_availability.is_exact() {
        if let Some(anchor) = node.citation_anchor_refs.first() {
            return anchor.clone();
        }
    }
    node.docs_node_id.clone()
}

fn freshness_badge(freshness: DocsFreshnessClass) -> &'static str {
    match freshness {
        DocsFreshnessClass::AuthoritativeLive => "Live",
        DocsFreshnessClass::WarmCached => "Cached",
        DocsFreshnessClass::DegradedCached => "Degraded cache",
        DocsFreshnessClass::Stale => "Stale",
        DocsFreshnessClass::Unverified => "Unverified",
    }
}

fn version_match_badge(version_match: VersionMatchState) -> &'static str {
    match version_match {
        VersionMatchState::ExactBuildMatch => "Exact build",
        VersionMatchState::CompatibleMinorDrift => "Compatible drift",
        VersionMatchState::IncompatibleDriftDetected => "Version mismatch",
        VersionMatchState::PreReleaseUnverified => "Pre-release",
        VersionMatchState::UnknownTargetBuild => "Unknown build",
    }
}

fn normalize_search_text(value: impl AsRef<str>) -> String {
    value.as_ref().trim().to_ascii_lowercase()
}

fn sanitize_ref(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}

fn push_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|existing| existing == value) {
        target.push(value.to_owned());
    }
}

trait DocsPackIndexSource {
    fn index_source_ref(&self) -> String;
}

impl DocsPackIndexSource for DocsPack {
    fn index_source_ref(&self) -> String {
        format!("{}#{}", self.pack_id, self.pack_revision_ref)
    }
}

fn docs_node_provenance_for_search(
    pack: &DocsPack,
    canonical_ref: &str,
    docs_node: &DocsNodeIdentity,
) -> DocsNodeProvenance {
    let external_open = if matches!(
        docs_node.locality_class,
        CitationLocalityClass::VendorLive | CitationLocalityClass::NotInstalled
    ) {
        DocsExternalOpenFallback::available(
            format!(
                "action:open-external-source:{}",
                sanitize_ref(canonical_ref)
            ),
            "Open supporting source",
            format!("browser-handoff:{}", sanitize_ref(canonical_ref)),
        )
    } else {
        DocsExternalOpenFallback::not_required()
    };
    DocsNodeProvenance::new(DocsNodeProvenanceInput {
        provenance_id: format!("docs-provenance:{}", sanitize_ref(canonical_ref)),
        docs_node: docs_node.clone(),
        knowledge_object_kind: DocsKnowledgeObjectKind::from(docs_node.doc_kind),
        derived_explanation_kind: (docs_node.source_class
            == crate::CitationSourceClass::DerivedExplanation)
            .then_some(crate::DocsDerivedExplanationKind::Generated),
        source_build_at: pack
            .source_truth
            .source_build_at
            .clone()
            .unwrap_or_else(|| pack.pack_revision_ref.clone()),
        running_build_identity_ref: pack.source_truth.running_build_identity_ref.clone(),
        mirror_offline_posture: DocsMirrorOfflinePosture::from_locality(docs_node.locality_class),
        external_open,
        example_validation: DocsExampleValidationClass::NotApplicable,
        citation_drawer_ref: Some(format!(
            "citation-drawer:{}",
            sanitize_ref(&docs_node.docs_node_id)
        )),
        surface_refs: vec!["surface:docs-backed-search".to_owned()],
    })
}
