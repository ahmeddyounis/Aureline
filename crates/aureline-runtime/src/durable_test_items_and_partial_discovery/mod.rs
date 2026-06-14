//! Durable test items, parameterized template / invocation identities,
//! notebook-linked test mappings, and partial-discovery truth for the M5
//! framework, notebook, and test-tree lanes.
//!
//! M5 test intelligence only stays trustworthy if a discovered test tree is a
//! set of **durable, separately identified objects** rather than a list of
//! display rows. Where [`crate::testing_identity`] froze the per-record
//! canonical test-item / session / attempt *ledger* and
//! [`crate::freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix`]
//! froze the surface-level *qualification matrix* over it, this module lands the
//! concrete discovery objects the framework-pack, notebook, and test-tree
//! consumers normalize onto:
//!
//! * a [`DurableTestNode`] with a stable [`node_id`](DurableTestNode::node_id)
//!   carrying a distinct [`DurableTestNodeKind`] for suites, concrete cases,
//!   parameterized templates, concrete invocations, and notebook-linked tests —
//!   so a template never shares a row identity with its invocations and a
//!   notebook-linked test never collapses into an ordinary file-backed case;
//! * a [`DiscoverySnapshot`] that records its [`DiscoveryPartiality`], the
//!   [`OmittedScope`] reasons that keep partial / heuristic / streaming /
//!   imported discovery visible, and a [`MappingSupportClass`] so the snapshot's
//!   support class survives reopen, support export, and release evidence;
//! * an identity chain so an identity remap or a source move degrades a node to
//!   [`TestItemIdentityClass::RemapReviewRequired`] **without losing the prior
//!   durable identity chain**.
//!
//! The taxonomy reuses the frozen
//! [`TestItemIdentityClass`](crate::testing_identity::TestItemIdentityClass)
//! vocabulary rather than minting synonyms, and bridges the existing
//! [`CanonicalTestItem`](crate::testing_identity::CanonicalTestItem) kinds onto
//! durable nodes through
//! [`DurableTestNode::from_canonical_item`] so the first real framework-pack /
//! test-tree consumer normalizes onto these objects instead of brittle display
//! rows.
//!
//! [`DurableTestDiscoveryPacket::validate`] refuses a packet that lets a display
//! label stand in for durable identity, collapses a parameterized template into
//! a concrete invocation, hides partial discovery behind a complete-looking
//! enumeration, lets imported / provider-backed discovery read as live local
//! truth, or drops a node's prior identity chain on remap.
//!
//! Raw test source, raw provider payloads, raw log bytes, provider cursors,
//! credentials, and raw artifact bodies never cross this boundary; the packet
//! carries only typed class tokens, booleans, opaque ids, and redaction-aware
//! reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/durable-test-items-and-partial-discovery.schema.json`](../../../../schemas/testing/durable-test-items-and-partial-discovery.schema.json).
//! The contract doc is
//! [`docs/testing/m5/durable-test-items-and-partial-discovery.md`](../../../../docs/testing/m5/durable-test-items-and-partial-discovery.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/durable-test-items-and-partial-discovery/`](../../../../fixtures/testing/m5/durable-test-items-and-partial-discovery/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::testing_identity::{CanonicalTestItem, CanonicalTestItemKind, TestItemIdentityClass};

/// Stable record-kind tag carried by [`DurableTestDiscoveryPacket`].
pub const DURABLE_TEST_DISCOVERY_RECORD_KIND: &str = "durable_test_item_discovery_packet";

/// Schema version for the durable test-item discovery packet.
pub const DURABLE_TEST_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DURABLE_TEST_DISCOVERY_SCHEMA_REF: &str =
    "schemas/testing/durable-test-items-and-partial-discovery.schema.json";

/// Repo-relative path of the contract doc.
pub const DURABLE_TEST_DISCOVERY_DOC_REF: &str =
    "docs/testing/m5/durable-test-items-and-partial-discovery.md";

/// Repo-relative path of the checked support-export artifact.
pub const DURABLE_TEST_DISCOVERY_ARTIFACT_REF: &str =
    "artifacts/testing/m5/durable-test-items-and-partial-discovery/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DURABLE_TEST_DISCOVERY_SUMMARY_REF: &str =
    "artifacts/testing/m5/durable-test-items-and-partial-discovery.md";

/// Repo-relative path of the protected fixture directory.
pub const DURABLE_TEST_DISCOVERY_FIXTURE_DIR: &str =
    "fixtures/testing/m5/durable-test-items-and-partial-discovery";

/// Closed kind vocabulary for a durable test node. Each kind carries its own
/// stable [`DurableTestNode::node_id`] so suites, concrete cases, parameterized
/// templates, concrete invocations, and notebook-linked tests are never
/// flattened into one row identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableTestNodeKind {
    /// A grouping suite / module / class that contains other nodes.
    Suite,
    /// A single concrete, file-backed runnable case.
    ConcreteCase,
    /// A parameterized template (family root) whose invocations are enumerated
    /// separately and which is never itself run as a single concrete case.
    ParameterizedTemplate,
    /// One concrete invocation of a [`DurableTestNodeKind::ParameterizedTemplate`].
    ConcreteInvocation,
    /// A test bound to a notebook cell rather than an ordinary source file.
    NotebookLinkedTest,
}

impl DurableTestNodeKind {
    /// Every node kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Suite,
        Self::ConcreteCase,
        Self::ParameterizedTemplate,
        Self::ConcreteInvocation,
        Self::NotebookLinkedTest,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Suite => "suite",
            Self::ConcreteCase => "concrete_case",
            Self::ParameterizedTemplate => "parameterized_template",
            Self::ConcreteInvocation => "concrete_invocation",
            Self::NotebookLinkedTest => "notebook_linked_test",
        }
    }

    /// True when the kind denotes an individually runnable leaf (not a grouping
    /// suite and not a non-runnable template family root).
    pub const fn is_runnable_leaf(self) -> bool {
        matches!(
            self,
            Self::ConcreteCase | Self::ConcreteInvocation | Self::NotebookLinkedTest
        )
    }
}

/// Closed vocabulary for the first real consumers that normalize raw discovery
/// onto durable nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryConsumerKind {
    /// A language framework pack's test explorer (e.g. a pytest-compatible pack).
    FrameworkPack,
    /// A notebook whose cells contribute notebook-linked tests.
    Notebook,
    /// The cross-pack test tree that aggregates discovered nodes.
    TestTree,
    /// An imported CI evidence overlay whose nodes are read-only locally.
    ImportedCi,
}

impl DiscoveryConsumerKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPack => "framework_pack",
            Self::Notebook => "notebook",
            Self::TestTree => "test_tree",
            Self::ImportedCi => "imported_ci",
        }
    }

    /// True when the consumer's discovery is provider / CI owned and read-only.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedCi)
    }
}

/// Closed partiality vocabulary for a discovery snapshot. Names how completely
/// the snapshot enumerated its claimed scope so a partial or imported discovery
/// never reads as a complete local enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryPartiality {
    /// The local discovery enumerated the full claimed scope; nothing omitted.
    Complete,
    /// Discovery is partial; the uncovered scope is recorded and stays visible.
    PartialVisible,
    /// Discovery used a heuristic / best-effort parse; uncovered scope recorded.
    Heuristic,
    /// Discovery is still streaming; the snapshot is incomplete by construction.
    Streaming,
    /// The set is imported from CI or a provider and is read-only locally.
    ProviderImported,
}

impl DiscoveryPartiality {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::PartialVisible => "partial_visible",
            Self::Heuristic => "heuristic",
            Self::Streaming => "streaming",
            Self::ProviderImported => "provider_imported",
        }
    }

    /// True when the snapshot is not a complete local enumeration and so must
    /// keep its uncovered or imported scope visible via recorded omitted scopes.
    pub const fn requires_omitted_scopes(self) -> bool {
        !matches!(self, Self::Complete)
    }

    /// True when the snapshot is imported / provider-owned.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ProviderImported)
    }
}

/// Closed reason vocabulary for an omitted discovery scope. The chrome and
/// support export quote the reason verbatim instead of collapsing partial
/// discovery into an empty or overconfident tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmittedScopeReason {
    /// Discovery exceeded its time budget for this scope.
    DiscoveryTimedOut,
    /// The framework adapter for this scope was unavailable.
    AdapterUnavailable,
    /// This scope requires an explicit opt-in before discovery runs.
    RequiresOptIn,
    /// This scope has not yet been streamed in.
    NotYetStreamed,
    /// This scope is owned and completed provider-side, not locally.
    ProviderOwnedScope,
    /// A parse error was isolated to this scope; the rest of the tree survived.
    ParseErrorIsolated,
    /// This scope was excluded by the active query / filter, not lost.
    FilteredByQuery,
}

impl OmittedScopeReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscoveryTimedOut => "discovery_timed_out",
            Self::AdapterUnavailable => "adapter_unavailable",
            Self::RequiresOptIn => "requires_opt_in",
            Self::NotYetStreamed => "not_yet_streamed",
            Self::ProviderOwnedScope => "provider_owned_scope",
            Self::ParseErrorIsolated => "parse_error_isolated",
            Self::FilteredByQuery => "filtered_by_query",
        }
    }
}

/// Closed support-class vocabulary recording how a snapshot maps onto durable
/// identity. The class is export-safe so the snapshot's support disposition
/// survives reopen, support export, and release evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingSupportClass {
    /// Every node mapped to a durable, locally resolved identity.
    FullyMappedLocal,
    /// Some scope is omitted but the mapped nodes stay durable and visible.
    PartiallyMappedVisible,
    /// The snapshot is imported / provider-owned and read-only locally.
    ImportedReadOnlyMapped,
    /// At least one node needs remap review but keeps its prior identity chain.
    NeedsRemapPreserved,
    /// At least one node resolves to display text only and must fail closed.
    DisplayOnlyDenied,
}

impl MappingSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyMappedLocal => "fully_mapped_local",
            Self::PartiallyMappedVisible => "partially_mapped_visible",
            Self::ImportedReadOnlyMapped => "imported_read_only_mapped",
            Self::NeedsRemapPreserved => "needs_remap_preserved",
            Self::DisplayOnlyDenied => "display_only_denied",
        }
    }
}

/// Closed source-state vocabulary for a durable node. Distinguishes a locally
/// resolved node from one whose source moved (needs remap), one imported
/// read-only, and one that resolves to display text only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeSourceState {
    /// The node resolves to current local source.
    LocalResolved,
    /// The node's source moved; it needs remap review before rerun or debug.
    SourceMovedNeedsRemap,
    /// The node is imported from CI / a provider and is read-only locally.
    ImportedReadOnly,
    /// The node resolves to display text only and must fail closed.
    DisplayTextOnly,
}

impl NodeSourceState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalResolved => "local_resolved",
            Self::SourceMovedNeedsRemap => "source_moved_needs_remap",
            Self::ImportedReadOnly => "imported_read_only",
            Self::DisplayTextOnly => "display_text_only",
        }
    }
}

/// Stable mapping of a notebook-linked test onto its notebook cell. Carries only
/// opaque ids — never raw cell source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookLinkage {
    /// Stable notebook id.
    pub notebook_id: String,
    /// Stable cell id within the notebook.
    pub cell_id: String,
    /// Zero-based ordinal of the cell within the notebook at discovery time.
    pub cell_ordinal: u32,
}

impl NotebookLinkage {
    /// Whether the linkage carries the opaque ids needed to bind a durable
    /// notebook-linked node back to its cell.
    pub fn is_valid(&self) -> bool {
        !self.notebook_id.trim().is_empty() && !self.cell_id.trim().is_empty()
    }
}

/// One durable test node with a stable identity, independent of any display
/// label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableTestNode {
    /// Stable, durable node id.
    pub node_id: String,
    /// Node kind — distinguishes suites, cases, templates, invocations, and
    /// notebook-linked tests.
    pub node_kind: DurableTestNodeKind,
    /// Parent node id, when this node is nested under a suite / template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_node_id: Option<String>,
    /// Human-readable display label. Never the identity basis.
    pub display_label: String,
    /// Non-display stable identity basis token (e.g. a node path hash). Must
    /// differ from [`display_label`](DurableTestNode::display_label) so a label
    /// never stands in for durable identity.
    pub identity_basis_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
    /// Source-resolution state.
    pub source_state: NodeSourceState,
    /// Template node id this node invokes, required on a concrete invocation and
    /// forbidden elsewhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_node_id: Option<String>,
    /// Concrete parameter key for an invocation, required on a concrete
    /// invocation and forbidden elsewhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation_key: Option<String>,
    /// Notebook linkage, required on a notebook-linked test and forbidden
    /// elsewhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notebook_linkage: Option<NotebookLinkage>,
    /// Prior durable identity basis tokens, newest last. A remap or source move
    /// appends the superseded basis here so the durable identity chain is never
    /// lost.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prior_identity_chain: Vec<String>,
    /// Evidence packet refs backing this node.
    pub evidence_refs: Vec<String>,
}

impl DurableTestNode {
    /// Whether this node is a parameterized template (family root).
    pub fn is_template(&self) -> bool {
        self.node_kind == DurableTestNodeKind::ParameterizedTemplate
    }

    /// Whether this node is a concrete invocation of a template.
    pub fn is_invocation(&self) -> bool {
        self.node_kind == DurableTestNodeKind::ConcreteInvocation
    }

    /// Whether this node is bound to a notebook cell.
    pub fn is_notebook_linked(&self) -> bool {
        self.node_kind == DurableTestNodeKind::NotebookLinkedTest
    }

    /// Whether the node's durable identity is established independently of its
    /// display label.
    pub fn identity_independent_of_display_name(&self) -> bool {
        let basis = self.identity_basis_token.trim();
        !basis.is_empty() && basis != self.display_label.trim()
    }

    /// Whether a node whose source moved or whose identity needs remap still
    /// carries its prior durable identity chain rather than dropping it.
    pub fn remap_preserves_chain(&self) -> bool {
        let needs_remap = self.identity_class == TestItemIdentityClass::RemapReviewRequired
            || self.source_state == NodeSourceState::SourceMovedNeedsRemap;
        if needs_remap {
            !self.prior_identity_chain.is_empty()
                && self
                    .prior_identity_chain
                    .iter()
                    .all(|t| !t.trim().is_empty())
        } else {
            true
        }
    }

    /// Degrades this node to needs-remap review after a source move while
    /// preserving its prior durable identity chain. The superseded identity
    /// basis is appended to [`prior_identity_chain`](DurableTestNode::prior_identity_chain)
    /// and a fresh basis token is recorded for the relocated source.
    pub fn degrade_to_needs_remap(mut self, relocated_identity_basis_token: String) -> Self {
        self.prior_identity_chain.push(std::mem::replace(
            &mut self.identity_basis_token,
            relocated_identity_basis_token,
        ));
        self.identity_class = TestItemIdentityClass::RemapReviewRequired;
        self.source_state = NodeSourceState::SourceMovedNeedsRemap;
        self
    }

    /// Whether this node's kind-specific linkage invariants hold: a concrete
    /// invocation references a template and an invocation key with a distinct
    /// node id; a notebook-linked test carries a valid linkage; and no other
    /// kind carries template / invocation / notebook linkage.
    pub fn linkage_consistent(&self) -> bool {
        match self.node_kind {
            DurableTestNodeKind::ConcreteInvocation => {
                self.notebook_linkage.is_none()
                    && self.template_node_id.as_ref().is_some_and(|template_id| {
                        !template_id.trim().is_empty() && template_id != &self.node_id
                    })
                    && self
                        .invocation_key
                        .as_ref()
                        .is_some_and(|key| !key.trim().is_empty())
            }
            DurableTestNodeKind::NotebookLinkedTest => {
                self.template_node_id.is_none()
                    && self.invocation_key.is_none()
                    && self
                        .notebook_linkage
                        .as_ref()
                        .is_some_and(NotebookLinkage::is_valid)
            }
            DurableTestNodeKind::ParameterizedTemplate
            | DurableTestNodeKind::Suite
            | DurableTestNodeKind::ConcreteCase => {
                self.template_node_id.is_none()
                    && self.invocation_key.is_none()
                    && self.notebook_linkage.is_none()
            }
        }
    }

    /// Whether every field required to record this node is present and the
    /// node's invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.display_label.trim().is_empty()
            && self.identity_independent_of_display_name()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
            && self.source_state != NodeSourceState::DisplayTextOnly
            && self.linkage_consistent()
            && self.remap_preserves_chain()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Bridges an existing [`CanonicalTestItem`] onto a durable node so the first
    /// real framework-pack / test-tree consumer normalizes onto this taxonomy.
    /// The canonical kinds map as: [`CanonicalTestItemKind::Case`] →
    /// [`DurableTestNodeKind::ConcreteCase`], [`CanonicalTestItemKind::ParameterizedFamily`]
    /// → [`DurableTestNodeKind::ParameterizedTemplate`],
    /// [`CanonicalTestItemKind::ParameterizedInstance`] →
    /// [`DurableTestNodeKind::ConcreteInvocation`], and
    /// [`CanonicalTestItemKind::ImportedProviderInstance`] →
    /// [`DurableTestNodeKind::ConcreteCase`] with imported read-only state.
    ///
    /// The canonical item carries only a display-label *digest* — display text is
    /// not identity — so the reviewable `display_label` is supplied by the caller
    /// and the node's identity basis is taken from the item's
    /// [`logical_item_key`](CanonicalTestItem::logical_item_key).
    pub fn from_canonical_item(
        item: &CanonicalTestItem,
        display_label: String,
        evidence_refs: Vec<String>,
    ) -> Self {
        let node_kind = match item.item_kind {
            CanonicalTestItemKind::Case => DurableTestNodeKind::ConcreteCase,
            CanonicalTestItemKind::ParameterizedFamily => {
                DurableTestNodeKind::ParameterizedTemplate
            }
            CanonicalTestItemKind::ParameterizedInstance => DurableTestNodeKind::ConcreteInvocation,
            CanonicalTestItemKind::ImportedProviderInstance => DurableTestNodeKind::ConcreteCase,
        };
        let source_state = match item.identity_class {
            TestItemIdentityClass::ImportedReadOnly => NodeSourceState::ImportedReadOnly,
            TestItemIdentityClass::RemapReviewRequired => NodeSourceState::SourceMovedNeedsRemap,
            TestItemIdentityClass::DisplayTextOnlyDenied => NodeSourceState::DisplayTextOnly,
            TestItemIdentityClass::Stable | TestItemIdentityClass::UnknownRequiresReview => {
                NodeSourceState::LocalResolved
            }
        };
        let (template_node_id, invocation_key) =
            if node_kind == DurableTestNodeKind::ConcreteInvocation {
                (
                    item.parameterized_family_ref.clone(),
                    item.parameterized_instance_key.clone(),
                )
            } else {
                (None, None)
            };
        Self {
            node_id: item.canonical_test_item_id.clone(),
            node_kind,
            parent_node_id: None,
            display_label,
            identity_basis_token: item.logical_item_key.clone(),
            identity_class: item.identity_class,
            source_state,
            template_node_id,
            invocation_key,
            notebook_linkage: None,
            prior_identity_chain: Vec::new(),
            evidence_refs,
        }
    }
}

/// One omitted discovery scope kept visible so partial discovery never collapses
/// into an empty or overconfident tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmittedScope {
    /// Stable omitted-scope id.
    pub scope_id: String,
    /// Reason this scope was omitted.
    pub reason: OmittedScopeReason,
    /// Precise reviewable label for the omitted scope.
    pub label: String,
    /// True when re-running discovery (or opting in / streaming) can recover it.
    pub recoverable: bool,
}

impl OmittedScope {
    /// Whether the omitted-scope record carries the identity and a precise label
    /// support and release evidence need.
    pub fn is_valid(&self) -> bool {
        !self.scope_id.trim().is_empty() && !self.label.trim().is_empty()
    }
}

/// One discovery snapshot for a single consumer (framework pack, notebook, test
/// tree, or imported CI overlay).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverySnapshot {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Consumer that produced this snapshot.
    pub consumer: DiscoveryConsumerKind,
    /// Human-readable snapshot label.
    pub label: String,
    /// Partiality of this snapshot's enumeration.
    pub partiality: DiscoveryPartiality,
    /// Export-safe mapping support class.
    pub mapping_support_class: MappingSupportClass,
    /// Durable nodes discovered in this snapshot.
    pub nodes: Vec<DurableTestNode>,
    /// Omitted scopes kept visible.
    pub omitted_scopes: Vec<OmittedScope>,
    /// True when imported / provider-backed nodes are never rendered as live
    /// local truth.
    pub imported_not_shown_as_local: bool,
    /// Evidence packet refs backing this snapshot.
    pub evidence_refs: Vec<String>,
}

impl DiscoverySnapshot {
    /// Node ids present in this snapshot.
    pub fn node_ids(&self) -> BTreeSet<&str> {
        self.nodes.iter().map(|n| n.node_id.as_str()).collect()
    }

    /// Whether every node id in this snapshot is unique.
    pub fn node_ids_unique(&self) -> bool {
        self.node_ids().len() == self.nodes.len()
    }

    /// Whether every concrete invocation references a template node that is
    /// itself present in this snapshot and kind-tagged as a template — so a
    /// template and its invocations never collapse into one row identity.
    pub fn template_invocation_integrity(&self) -> bool {
        for node in &self.nodes {
            if node.is_invocation() {
                let Some(template_id) = node.template_node_id.as_deref() else {
                    return false;
                };
                let resolves = self
                    .nodes
                    .iter()
                    .any(|candidate| candidate.node_id == template_id && candidate.is_template());
                if !resolves {
                    return false;
                }
            }
        }
        true
    }

    /// Whether partial / heuristic / streaming / imported discovery keeps its
    /// uncovered scope visible: a non-complete snapshot records at least one
    /// omitted scope, and a complete snapshot records none.
    pub fn partial_visibility_ok(&self) -> bool {
        if self.partiality.requires_omitted_scopes() {
            !self.omitted_scopes.is_empty()
        } else {
            self.omitted_scopes.is_empty()
        }
    }

    /// Whether imported / provider-backed discovery is kept separate from live
    /// local truth.
    pub fn imported_separation_ok(&self) -> bool {
        let imported = self.consumer.is_imported()
            || self.partiality.is_imported()
            || self
                .nodes
                .iter()
                .any(|n| n.source_state == NodeSourceState::ImportedReadOnly);
        if imported {
            self.imported_not_shown_as_local
        } else {
            true
        }
    }

    /// Whether the mapping support class is consistent with the snapshot's nodes
    /// and partiality — so the export-safe support class never overstates the
    /// truth a reopen or release-evidence reader will see.
    pub fn mapping_support_consistent(&self) -> bool {
        let any_needs_remap = self
            .nodes
            .iter()
            .any(|n| n.source_state == NodeSourceState::SourceMovedNeedsRemap);
        let imported = self.consumer.is_imported() || self.partiality.is_imported();
        match self.mapping_support_class {
            MappingSupportClass::FullyMappedLocal => {
                self.partiality == DiscoveryPartiality::Complete && !any_needs_remap && !imported
            }
            MappingSupportClass::PartiallyMappedVisible => {
                self.partiality.requires_omitted_scopes() && !imported
            }
            MappingSupportClass::ImportedReadOnlyMapped => imported,
            MappingSupportClass::NeedsRemapPreserved => any_needs_remap,
            // DisplayOnlyDenied snapshots are rejected at the node level; the
            // class is recorded for fail-closed support exports only.
            MappingSupportClass::DisplayOnlyDenied => false,
        }
    }

    /// Whether every dimension required to record this snapshot is present and
    /// internally consistent.
    pub fn is_valid(&self) -> bool {
        !self.snapshot_id.trim().is_empty()
            && !self.label.trim().is_empty()
            && !self.nodes.is_empty()
            && self.nodes.iter().all(DurableTestNode::is_valid)
            && self.node_ids_unique()
            && self.template_invocation_integrity()
            && self.partial_visibility_ok()
            && self.omitted_scopes.iter().all(OmittedScope::is_valid)
            && self.imported_separation_ok()
            && self.mapping_support_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Count of durable runnable leaf nodes (concrete cases, invocations, and
    /// notebook-linked tests).
    pub fn runnable_leaf_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.node_kind.is_runnable_leaf())
            .count()
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableDiscoveryGuardrails {
    /// Display labels never stand in for durable test identity.
    pub display_labels_never_substitute_identity: bool,
    /// Parameterized templates stay distinct from their concrete invocations.
    pub templates_distinct_from_invocations: bool,
    /// Notebook-linked tests stay distinct from ordinary file-backed tests.
    pub notebook_tests_distinct_from_file_tests: bool,
    /// Partial / heuristic / streaming discovery stays visible.
    pub partial_discovery_stays_visible: bool,
    /// Imported / provider-backed discovery never masquerades as live local truth.
    pub imported_never_masquerades_as_local: bool,
    /// Identity remap and source moves preserve the prior durable identity chain.
    pub remap_preserves_identity_chain: bool,
}

impl DurableDiscoveryGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.display_labels_never_substitute_identity
            && self.templates_distinct_from_invocations
            && self.notebook_tests_distinct_from_file_tests
            && self.partial_discovery_stays_visible
            && self.imported_never_masquerades_as_local
            && self.remap_preserves_identity_chain
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableDiscoveryConsumerProjection {
    /// Framework-pack test explorers ingest these durable nodes.
    pub framework_pack_ingests_nodes: bool,
    /// Notebook test surfaces ingest these durable nodes.
    pub notebook_ingests_nodes: bool,
    /// The test tree ingests these durable nodes.
    pub test_tree_ingests_nodes: bool,
    /// Support export and release evidence ingest the omitted-scope and mapping
    /// support classes.
    pub support_export_ingests_partiality: bool,
}

impl DurableDiscoveryConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.framework_pack_ingests_nodes
            && self.notebook_ingests_nodes
            && self.test_tree_ingests_nodes
            && self.support_export_ingests_partiality
    }
}

/// Constructor input for [`DurableTestDiscoveryPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableTestDiscoveryPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-consumer discovery snapshots.
    pub snapshots: Vec<DiscoverySnapshot>,
    /// Guardrail invariants block.
    pub guardrails: DurableDiscoveryGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DurableDiscoveryConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe durable test-item discovery packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableTestDiscoveryPacket {
    /// Record kind; must equal [`DURABLE_TEST_DISCOVERY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DURABLE_TEST_DISCOVERY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-consumer discovery snapshots.
    pub snapshots: Vec<DiscoverySnapshot>,
    /// Guardrail invariants block.
    pub guardrails: DurableDiscoveryGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DurableDiscoveryConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DurableTestDiscoveryPacket {
    /// Builds a durable test-item discovery packet.
    pub fn new(input: DurableTestDiscoveryPacketInput) -> Self {
        Self {
            record_kind: DURABLE_TEST_DISCOVERY_RECORD_KIND.to_owned(),
            schema_version: DURABLE_TEST_DISCOVERY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            snapshots: input.snapshots,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Consumers represented by some snapshot in this packet.
    pub fn represented_consumers(&self) -> BTreeSet<DiscoveryConsumerKind> {
        self.snapshots.iter().map(|s| s.consumer).collect()
    }

    /// Node kinds represented across every snapshot.
    pub fn represented_node_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.snapshots
            .iter()
            .flat_map(|s| s.nodes.iter().map(|n| n.node_kind))
            .collect()
    }

    /// Count of snapshots whose discovery is partial (not a complete local
    /// enumeration).
    pub fn partial_snapshot_count(&self) -> usize {
        self.snapshots
            .iter()
            .filter(|s| s.partiality.requires_omitted_scopes())
            .count()
    }

    /// Validates the durable test-item discovery invariants.
    pub fn validate(&self) -> Vec<DurableTestDiscoveryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DURABLE_TEST_DISCOVERY_RECORD_KIND {
            violations.push(DurableTestDiscoveryViolation::WrongRecordKind);
        }
        if self.schema_version != DURABLE_TEST_DISCOVERY_SCHEMA_VERSION {
            violations.push(DurableTestDiscoveryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DurableTestDiscoveryViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_snapshots(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(DurableTestDiscoveryViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(DurableTestDiscoveryViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("durable test discovery packet serializes"),
        ) {
            violations.push(DurableTestDiscoveryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("durable test discovery packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Durable Test-Item Discovery\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Snapshots: {} ({} partial)\n",
            self.snapshots.len(),
            self.partial_snapshot_count()
        ));
        out.push_str(&format!(
            "- Consumers: {} / 4\n",
            self.represented_consumers().len()
        ));
        out.push_str(&format!(
            "- Node kinds present: {} / {}\n",
            self.represented_node_kinds().len(),
            DurableTestNodeKind::ALL.len()
        ));
        out.push_str("\n## Snapshots\n\n");
        for snapshot in &self.snapshots {
            out.push_str(&format!(
                "- **{}** ({}): partiality `{}`, support `{}`\n",
                snapshot.snapshot_id,
                snapshot.consumer.as_str(),
                snapshot.partiality.as_str(),
                snapshot.mapping_support_class.as_str()
            ));
            out.push_str(&format!("  - {}\n", snapshot.label));
            out.push_str(&format!(
                "  - nodes: {} ({} runnable leaves)\n",
                snapshot.nodes.len(),
                snapshot.runnable_leaf_count()
            ));
            for node in &snapshot.nodes {
                out.push_str(&format!(
                    "    - `{}` [{}] identity=`{}` source=`{}`",
                    node.node_id,
                    node.node_kind.as_str(),
                    node.identity_class.as_str(),
                    node.source_state.as_str()
                ));
                if let Some(template) = &node.template_node_id {
                    out.push_str(&format!(" template=`{template}`"));
                }
                if !node.prior_identity_chain.is_empty() {
                    out.push_str(&format!(" prior_chain={}", node.prior_identity_chain.len()));
                }
                out.push('\n');
            }
            for omitted in &snapshot.omitted_scopes {
                out.push_str(&format!(
                    "    - omitted `{}` ({}): {}\n",
                    omitted.scope_id,
                    omitted.reason.as_str(),
                    omitted.label
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in discovery export.
#[derive(Debug)]
pub enum DurableTestDiscoveryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DurableTestDiscoveryViolation>),
}

impl fmt::Display for DurableTestDiscoveryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "durable test discovery export parse failed: {error}"
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
                    "durable test discovery export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DurableTestDiscoveryArtifactError {}

/// Validation failures emitted by [`DurableTestDiscoveryPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurableTestDiscoveryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required consumer is represented by no snapshot.
    RequiredConsumerMissing,
    /// No snapshot demonstrates partial-but-visible discovery.
    PartialDiscoveryCaseMissing,
    /// No node demonstrates a needs-remap chain preserved across a source move.
    RemapChainCaseMissing,
    /// A snapshot is incomplete.
    SnapshotInvalid,
    /// Snapshot node ids are not unique.
    NodeIdsNotUnique,
    /// A display label stands in for durable test identity.
    DisplayLabelSubstitutesIdentity,
    /// A parameterized template was collapsed into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// A notebook-linked test was collapsed into a file-backed test.
    NotebookTestCollapsedWithFileTest,
    /// Partial / heuristic / streaming / imported discovery was hidden.
    PartialDiscoveryHidden,
    /// Imported / provider-backed discovery was shown as live local truth.
    ImportedShownAsLocal,
    /// A remap / source move dropped the prior durable identity chain.
    RemapChainLost,
    /// A snapshot's mapping support class overstates its truth.
    MappingSupportInconsistent,
    /// A node or snapshot lacks evidence refs.
    EvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl DurableTestDiscoveryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::PartialDiscoveryCaseMissing => "partial_discovery_case_missing",
            Self::RemapChainCaseMissing => "remap_chain_case_missing",
            Self::SnapshotInvalid => "snapshot_invalid",
            Self::NodeIdsNotUnique => "node_ids_not_unique",
            Self::DisplayLabelSubstitutesIdentity => "display_label_substitutes_identity",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::NotebookTestCollapsedWithFileTest => "notebook_test_collapsed_with_file_test",
            Self::PartialDiscoveryHidden => "partial_discovery_hidden",
            Self::ImportedShownAsLocal => "imported_shown_as_local",
            Self::RemapChainLost => "remap_chain_lost",
            Self::MappingSupportInconsistent => "mapping_support_inconsistent",
            Self::EvidenceMissing => "evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable discovery export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_durable_test_discovery_export(
) -> Result<DurableTestDiscoveryPacket, DurableTestDiscoveryArtifactError> {
    let packet: DurableTestDiscoveryPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/durable-test-items-and-partial-discovery/support_export.json"
    )))
    .map_err(DurableTestDiscoveryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DurableTestDiscoveryArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &DurableTestDiscoveryPacket,
    violations: &mut Vec<DurableTestDiscoveryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DURABLE_TEST_DISCOVERY_SCHEMA_REF,
        DURABLE_TEST_DISCOVERY_DOC_REF,
        DURABLE_TEST_DISCOVERY_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DurableTestDiscoveryViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &DurableTestDiscoveryPacket,
    violations: &mut Vec<DurableTestDiscoveryViolation>,
) {
    let consumers = packet.represented_consumers();
    for required in [
        DiscoveryConsumerKind::FrameworkPack,
        DiscoveryConsumerKind::Notebook,
        DiscoveryConsumerKind::TestTree,
    ] {
        if !consumers.contains(&required) {
            violations.push(DurableTestDiscoveryViolation::RequiredConsumerMissing);
            break;
        }
    }

    let node_kinds = packet.represented_node_kinds();
    let distinguishes_template = node_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && node_kinds.contains(&DurableTestNodeKind::ConcreteInvocation);
    let distinguishes_notebook = node_kinds.contains(&DurableTestNodeKind::NotebookLinkedTest)
        && node_kinds.contains(&DurableTestNodeKind::ConcreteCase);
    if !distinguishes_template {
        violations.push(DurableTestDiscoveryViolation::TemplateCollapsedWithInvocation);
    }
    if !distinguishes_notebook {
        violations.push(DurableTestDiscoveryViolation::NotebookTestCollapsedWithFileTest);
    }

    if !packet
        .snapshots
        .iter()
        .any(|s| s.partiality.requires_omitted_scopes() && s.partial_visibility_ok())
    {
        violations.push(DurableTestDiscoveryViolation::PartialDiscoveryCaseMissing);
    }

    let has_remap_chain = packet.snapshots.iter().any(|s| {
        s.nodes.iter().any(|n| {
            n.source_state == NodeSourceState::SourceMovedNeedsRemap
                && !n.prior_identity_chain.is_empty()
        })
    });
    if !has_remap_chain {
        violations.push(DurableTestDiscoveryViolation::RemapChainCaseMissing);
    }
}

fn validate_snapshots(
    packet: &DurableTestDiscoveryPacket,
    violations: &mut Vec<DurableTestDiscoveryViolation>,
) {
    for snapshot in &packet.snapshots {
        if !snapshot.is_valid() {
            violations.push(DurableTestDiscoveryViolation::SnapshotInvalid);
        }
        if !snapshot.node_ids_unique() {
            violations.push(DurableTestDiscoveryViolation::NodeIdsNotUnique);
        }
        if !snapshot.template_invocation_integrity() {
            violations.push(DurableTestDiscoveryViolation::TemplateCollapsedWithInvocation);
        }
        if !snapshot.partial_visibility_ok() {
            violations.push(DurableTestDiscoveryViolation::PartialDiscoveryHidden);
        }
        if !snapshot.imported_separation_ok() {
            violations.push(DurableTestDiscoveryViolation::ImportedShownAsLocal);
        }
        if !snapshot.mapping_support_consistent() {
            violations.push(DurableTestDiscoveryViolation::MappingSupportInconsistent);
        }
        if snapshot.evidence_refs.is_empty()
            || snapshot.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(DurableTestDiscoveryViolation::EvidenceMissing);
        }
        for node in &snapshot.nodes {
            if !node.identity_independent_of_display_name() {
                violations.push(DurableTestDiscoveryViolation::DisplayLabelSubstitutesIdentity);
            }
            if !node.remap_preserves_chain() {
                violations.push(DurableTestDiscoveryViolation::RemapChainLost);
            }
            if node.evidence_refs.is_empty()
                || node.evidence_refs.iter().any(|r| r.trim().is_empty())
            {
                violations.push(DurableTestDiscoveryViolation::EvidenceMissing);
            }
        }
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
