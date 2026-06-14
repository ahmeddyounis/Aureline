//! Inspect-to-source component / DOM / widget tree-node mapping with
//! exact / approximate / generated-only / runtime-only labels.
//!
//! Where
//! [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! freezes the *qualification matrix* over claimed preview/runtime surfaces and
//! [`crate::preview_session_descriptors`] materializes the *per-session*
//! descriptor each surface presents, this module materializes the **per-node**
//! truth packet behind every inspectable visual surface: a single shared
//! mapping packet that teaches each inspected component, DOM element, or
//! widget-tree node to say, *before any jump-to-source or mutation affordance
//! appears*, whether its source mapping is exact, approximate, generated-only,
//! or runtime-only.
//!
//! The packet is the one canonical answer to "for the node the user is hovering
//! or selecting in a component / DOM / widget tree, how good is its mapping back
//! to canonical source, where does inspect-to-source land, and what continuity
//! route stays honest when the mapping is weak?" An
//! [`InspectToSourceTreePacket`] binds inspectable component, DOM, and
//! widget-tree surfaces onto the same governed mapping vocabulary —
//! [`NodeMappingQualityClass`], [`ContinuityRoute`], and
//! [`MappingDowngradeTrigger`] — instead of provider-specific extension chrome.
//!
//! Source stays canonical and the mapping packet is derivative — never a second
//! writable truth model. An [`InspectNode`] keeps four honesty rules the spec
//! freezes:
//!
//! - **Label before affordance.** A node resolves and presents its
//!   mapping-quality label before any source-navigation or mutation affordance
//!   is offered; [`InspectNode::label_before_affordance_ok`] enforces it.
//! - **Continuity without silent upgrade.** When the mapping is weak the node
//!   routes to a [`ContinuityRoute`] determined by its quality — an exact jump,
//!   an approximate jump with explicit disclosure, a source-only fallback, or a
//!   runtime-only explanation — and a node that downgraded through a reconnect,
//!   provider loss, or mapping downgrade never silently turns a runtime-only
//!   node into a source-backed one.
//! - **No runtime masquerade.** A runtime-only node is runtime-backed, carries
//!   no canonical source anchor, and never claims to be saved source state.
//! - **No inspect-to-write auto-upgrade.** Only an exact or approximate
//!   (source-backed) node may offer a mutation affordance, and only when it
//!   previews the real source diff before commit; generated-only and
//!   runtime-only nodes stay inspect-only.
//!
//! Raw URLs, hostnames, cookies, raw provider payloads, credentials, raw DOM
//! bodies, and raw runtime handles never cross this boundary; the packet carries
//! only typed class tokens, opaque source/evidence refs, booleans, and redacted
//! labels, so support and diagnostics exports can reconstruct exactly what
//! mapping quality the user saw for each inspected node.
//!
//! The boundary schema is
//! [`schemas/preview/inspect_to_source_tree_mapping.schema.json`](../../../../schemas/preview/inspect_to_source_tree_mapping.schema.json).
//! The contract doc is
//! [`docs/preview/m5/inspect_to_source_tree_mapping.md`](../../../../docs/preview/m5/inspect_to_source_tree_mapping.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/inspect_to_source_tree_mapping/`](../../../../fixtures/preview/m5/inspect_to_source_tree_mapping/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`InspectToSourceTreePacket`].
pub const INSPECT_TO_SOURCE_TREE_RECORD_KIND: &str = "inspect_to_source_tree_mapping";

/// Schema version for the inspect-to-source tree mapping packet.
pub const INSPECT_TO_SOURCE_TREE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INSPECT_TO_SOURCE_TREE_SCHEMA_REF: &str =
    "schemas/preview/inspect_to_source_tree_mapping.schema.json";

/// Repo-relative path of the contract doc.
pub const INSPECT_TO_SOURCE_TREE_DOC_REF: &str =
    "docs/preview/m5/inspect_to_source_tree_mapping.md";

/// Repo-relative path of the protected fixture directory.
pub const INSPECT_TO_SOURCE_TREE_FIXTURE_DIR: &str =
    "fixtures/preview/m5/inspect_to_source_tree_mapping";

/// Repo-relative path of the checked support-export artifact.
pub const INSPECT_TO_SOURCE_TREE_ARTIFACT_REF: &str =
    "artifacts/preview/m5/inspect_to_source_tree_mapping/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const INSPECT_TO_SOURCE_TREE_SUMMARY_REF: &str =
    "artifacts/preview/m5/inspect_to_source_tree_mapping.md";

/// Closed inspectable-tree vocabulary. Names which inspectable visual tree a node
/// belongs to so component, DOM, and widget-tree inspection all normalize onto
/// the same mapping packet instead of bespoke per-framework chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectTreeKind {
    /// A node in a framework component tree (e.g. a rendered component instance).
    Component,
    /// A node in the live DOM element tree.
    DomElement,
    /// A node in a widget tree (e.g. a Flutter-style widget render node).
    WidgetTreeNode,
}

impl InspectTreeKind {
    /// Every inspectable tree kind a claimed M5 surface must cover, in
    /// declaration order.
    pub const ALL: [Self; 3] = [Self::Component, Self::DomElement, Self::WidgetTreeNode];

    /// Stable token recorded in the node.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Component => "component",
            Self::DomElement => "dom_element",
            Self::WidgetTreeNode => "widget_tree_node",
        }
    }
}

/// Closed node mapping-quality vocabulary. The four spec-frozen labels every
/// inspect-to-source node carries before a source-navigation or mutation
/// affordance appears. Adding a new value is additive-minor; repurposing is
/// breaking.
///
/// `exact`          — the node maps unambiguously to a canonical-source span.
/// `approximate`    — the node maps to source heuristically; a jump lands near,
///                    not exactly, on the canonical span.
/// `generated_only` — the node corresponds to generated output (e.g. a compiled
///                    or model-produced fragment) with no hand-authored source
///                    span; inspect-to-source routes to the generator input.
/// `runtime_only`   — the node exists only in the live runtime tree with no
///                    source backing at all; it must never claim saved source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeMappingQualityClass {
    Exact,
    Approximate,
    GeneratedOnly,
    RuntimeOnly,
}

impl NodeMappingQualityClass {
    /// Every mapping-quality class a claimed surface must demonstrate, in
    /// declaration order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Approximate,
        Self::GeneratedOnly,
        Self::RuntimeOnly,
    ];

    /// Stable token recorded in the node.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::GeneratedOnly => "generated_only",
            Self::RuntimeOnly => "runtime_only",
        }
    }

    /// True when the node maps back to a canonical-source span (exact or
    /// approximate) and so must carry a source anchor.
    pub const fn is_source_backed(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate)
    }

    /// True when the node has no source backing at all and must stay runtime-only.
    pub const fn is_runtime_only(self) -> bool {
        matches!(self, Self::RuntimeOnly)
    }

    /// True when a source jump against this mapping can land deterministically on
    /// a canonical-source span.
    pub const fn admits_deterministic_jump(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// The continuity route that stays honest for this mapping quality. The route
    /// is fully determined by the quality so the chrome can never advertise a
    /// stronger jump than the mapping supports.
    pub const fn required_continuity_route(self) -> ContinuityRoute {
        match self {
            Self::Exact => ContinuityRoute::ExactJump,
            Self::Approximate => ContinuityRoute::ApproximateJumpWithDisclosure,
            Self::GeneratedOnly => ContinuityRoute::SourceOnlyFallback,
            Self::RuntimeOnly => ContinuityRoute::RuntimeOnlyExplanation,
        }
    }
}

/// Closed continuity-route vocabulary. Names how inspect-to-source preserves
/// continuity for a node given its mapping quality, so a weak mapping routes to a
/// source-only fallback, a runtime-only explanation, or an approximate jump with
/// explicit disclosure rather than a silent, misleading deterministic jump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityRoute {
    /// Deterministic jump to the canonical-source span.
    ExactJump,
    /// Approximate jump with an explicit "approximate" disclosure shown first.
    ApproximateJumpWithDisclosure,
    /// No node-level span; route the user to the generating source directly.
    SourceOnlyFallback,
    /// No source backing; explain there is no source to jump to.
    RuntimeOnlyExplanation,
}

impl ContinuityRoute {
    /// Stable token recorded in the node.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactJump => "exact_jump",
            Self::ApproximateJumpWithDisclosure => "approximate_jump_with_disclosure",
            Self::SourceOnlyFallback => "source_only_fallback",
            Self::RuntimeOnlyExplanation => "runtime_only_explanation",
        }
    }
}

/// Closed mapping-downgrade-trigger vocabulary. Names why a node's mapping
/// continuity was preserved through a downgrade event; the chrome quotes the
/// trigger verbatim instead of a generic error, and a downgraded node never
/// silently re-upgrades into a stronger mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingDowngradeTrigger {
    /// The runtime reconnected and the prior source map could not be re-pinned.
    RuntimeReconnect,
    /// The mapping provider was lost (e.g. source-map server went away).
    ProviderLoss,
    /// The mapping quality dropped (e.g. exact became approximate or runtime-only).
    MappingDowngraded,
    /// The source drifted from the runtime tree.
    SourceDrift,
    /// The mapping quality could not be identified.
    UnidentifiedMappingQuality,
    /// Policy narrowed the node below its mapping.
    PolicyNarrowed,
    /// An upstream dependency narrowed and dragged this node down with it.
    UpstreamDependencyNarrowed,
}

impl MappingDowngradeTrigger {
    /// Stable token recorded in the node.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReconnect => "runtime_reconnect",
            Self::ProviderLoss => "provider_loss",
            Self::MappingDowngraded => "mapping_downgraded",
            Self::SourceDrift => "source_drift",
            Self::UnidentifiedMappingQuality => "unidentified_mapping_quality",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// One inspect-to-source tree node: the shared truth packet a single inspectable
/// component / DOM / widget node presents before any jump-to-source or mutation
/// affordance appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectNode {
    /// Stable node id.
    pub node_id: String,
    /// Which inspectable tree this node belongs to.
    pub tree_kind: InspectTreeKind,
    /// Human-readable label summary safe to render on the node row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the node mapping was observed.
    pub observed_at: String,

    /// Mapping-quality label: exact / approximate / generated-only / runtime-only.
    pub mapping_quality: NodeMappingQualityClass,
    /// Continuity route honest for this mapping quality.
    pub continuity_route: ContinuityRoute,

    /// True once the mapping-quality label is resolved and presented. A source
    /// navigation or mutation affordance may only appear after this is true.
    pub mapping_label_resolved: bool,
    /// True when a jump-to-source navigation affordance is offered for this node.
    pub source_navigation_offered: bool,
    /// True when a write / mutation (designer-edit) affordance is offered.
    pub mutation_offered: bool,
    /// True when an offered mutation previews the real source diff before commit.
    pub previews_source_diff_before_commit: bool,

    /// True when a live runtime backs this node.
    pub runtime_backed: bool,
    /// True when the node claims to be saved source state. A runtime-only node
    /// must never set this true.
    pub claims_saved_source: bool,

    /// Opaque ref to the canonical source anchor this node maps to. Required for
    /// source-backed (exact / approximate) quality; absent for a runtime-only node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_anchor_ref: Option<String>,

    /// Trigger that fired a continuity-preserving downgrade; required when a node
    /// carries a degraded label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<MappingDowngradeTrigger>,
    /// Precise degraded label; required when the node carries a downgrade trigger.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,

    /// Evidence packet refs backing this node.
    pub evidence_refs: Vec<String>,
}

impl InspectNode {
    /// Whether the continuity route matches the route required by the mapping
    /// quality, so the chrome never advertises a stronger jump than the mapping
    /// supports.
    pub fn continuity_route_ok(&self) -> bool {
        self.continuity_route == self.mapping_quality.required_continuity_route()
    }

    /// Whether the mapping-quality label is resolved before any source-navigation
    /// or mutation affordance is offered.
    pub fn label_before_affordance_ok(&self) -> bool {
        if self.source_navigation_offered || self.mutation_offered {
            self.mapping_label_resolved
        } else {
            true
        }
    }

    /// Whether the source-anchor presence matches the mapping quality: a
    /// source-backed node references a canonical anchor; a runtime-only node
    /// carries none.
    pub fn source_anchor_presence_ok(&self) -> bool {
        if self.mapping_quality.is_source_backed() {
            self.source_anchor_ref.is_some()
        } else if self.mapping_quality.is_runtime_only() {
            self.source_anchor_ref.is_none()
        } else {
            // Generated-only may carry a generator-input anchor or none.
            true
        }
    }

    /// Whether a runtime-only node is honestly labeled rather than passed off as
    /// saved source state: it is runtime-backed, never claims saved source state,
    /// and carries no canonical source anchor.
    pub fn runtime_masquerade_ok(&self) -> bool {
        if self.mapping_quality.is_runtime_only() {
            self.runtime_backed && !self.claims_saved_source && self.source_anchor_ref.is_none()
        } else {
            true
        }
    }

    /// Whether only an exact or approximate (source-backed) node claims saved
    /// source state. Generated-only and runtime-only nodes are derivative and may
    /// never claim to be saved source.
    pub fn saved_source_claim_ok(&self) -> bool {
        if self.claims_saved_source {
            self.mapping_quality.is_source_backed()
        } else {
            true
        }
    }

    /// Whether the mutation affordance stays honest: only a source-backed node
    /// may offer a mutation, and only when it references a canonical anchor and
    /// previews the real source diff before commit. Generated-only and
    /// runtime-only nodes stay inspect-only and never auto-upgrade into a
    /// write-capable designer flow.
    pub fn mutation_affordance_ok(&self) -> bool {
        if self.mutation_offered {
            self.mapping_quality.is_source_backed()
                && self.source_anchor_ref.is_some()
                && self.previews_source_diff_before_commit
        } else {
            true
        }
    }

    /// Whether this node demonstrates a continuity-preserving downgrade: it
    /// carries a recorded trigger plus a precise, non-generic degraded label.
    pub fn has_downgrade(&self) -> bool {
        self.downgrade_trigger.is_some()
    }

    /// Whether the downgrade evidence is consistent: a node carrying a trigger
    /// also carries a precise, non-generic degraded label, and a node with no
    /// trigger carries no degraded label.
    pub fn downgrade_consistent(&self) -> bool {
        if self.downgrade_trigger.is_some() {
            self.degraded_label
                .as_ref()
                .is_some_and(|label| !label_is_generic(label))
        } else {
            self.degraded_label.is_none()
        }
    }

    /// Deterministic governed chip line for this node: the closed-vocabulary chips
    /// downstream consumers render verbatim instead of bespoke copy.
    pub fn chip_tokens(&self) -> String {
        format!(
            "tree={tree} mapping={mapping} continuity={continuity} runtime_backed={runtime}",
            tree = self.tree_kind.as_str(),
            mapping = self.mapping_quality.as_str(),
            continuity = self.continuity_route.as_str(),
            runtime = self.runtime_backed,
        )
    }

    /// Whether every dimension required to record this node is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.continuity_route_ok()
            && self.label_before_affordance_ok()
            && self.source_anchor_presence_ok()
            && self.runtime_masquerade_ok()
            && self.saved_source_claim_ok()
            && self.mutation_affordance_ok()
            && self.downgrade_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the inspect-to-source tree packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeGuardrails {
    /// Source remains canonical; the mapping packet is derivative, never a second
    /// writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// Runtime state never hides source-mapping uncertainty behind a node label.
    pub runtime_state_never_hides_source_mapping_uncertainty: bool,
    /// Inspect-only nodes are never auto-upgraded into write-capable designer flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview/browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// The mapping-quality label is shown before any navigation or mutation.
    pub mapping_label_shown_before_navigation_or_mutation: bool,
    /// Continuity is preserved without silently upgrading a runtime-only node into
    /// a source-backed one.
    pub continuity_preserved_without_silent_source_upgrade: bool,
}

impl TreeGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.runtime_state_never_hides_source_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.mapping_label_shown_before_navigation_or_mutation
            && self.continuity_preserved_without_silent_source_upgrade
    }
}

/// Consumer-projection block for the inspect-to-source tree packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeConsumerProjection {
    /// Product surfaces ingest these node packets instead of cloning chip text.
    pub product_ingests_nodes: bool,
    /// Docs/help ingests the same node packets.
    pub docs_help_ingests_nodes: bool,
    /// Diagnostics ingests the same node packets.
    pub diagnostics_ingests_nodes: bool,
    /// Support export ingests the same node packets.
    pub support_export_ingests_nodes: bool,
    /// Release-control surfaces ingest the same node packets.
    pub release_control_ingests_nodes: bool,
    /// Support / diagnostics exports can reconstruct the mapping quality the user
    /// saw for each inspected node.
    pub support_export_reconstructs_mapping_quality: bool,
}

impl TreeConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_nodes
            && self.docs_help_ingests_nodes
            && self.diagnostics_ingests_nodes
            && self.support_export_ingests_nodes
            && self.release_control_ingests_nodes
            && self.support_export_reconstructs_mapping_quality
    }
}

/// Constructor input for [`InspectToSourceTreePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectToSourceTreePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-node mapping descriptors.
    pub nodes: Vec<InspectNode>,
    /// Guardrail invariants block.
    pub guardrails: TreeGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TreeConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe inspect-to-source tree mapping packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectToSourceTreePacket {
    /// Record kind; must equal [`INSPECT_TO_SOURCE_TREE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INSPECT_TO_SOURCE_TREE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-node mapping descriptors.
    pub nodes: Vec<InspectNode>,
    /// Guardrail invariants block.
    pub guardrails: TreeGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TreeConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl InspectToSourceTreePacket {
    /// Builds an inspect-to-source tree mapping packet.
    pub fn new(input: InspectToSourceTreePacketInput) -> Self {
        Self {
            record_kind: INSPECT_TO_SOURCE_TREE_RECORD_KIND.to_owned(),
            schema_version: INSPECT_TO_SOURCE_TREE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            nodes: input.nodes,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Inspectable tree kinds represented by some node in this packet.
    pub fn represented_tree_kinds(&self) -> BTreeSet<InspectTreeKind> {
        self.nodes.iter().map(|n| n.tree_kind).collect()
    }

    /// Mapping-quality classes represented by some node in this packet.
    pub fn represented_mapping_qualities(&self) -> BTreeSet<NodeMappingQualityClass> {
        self.nodes.iter().map(|n| n.mapping_quality).collect()
    }

    /// Count of nodes that demonstrate a continuity-preserving downgrade.
    pub fn downgraded_node_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.has_downgrade()).count()
    }

    /// Validates the inspect-to-source tree mapping packet invariants.
    pub fn validate(&self) -> Vec<InspectToSourceTreeViolation> {
        let mut violations = Vec::new();

        if self.record_kind != INSPECT_TO_SOURCE_TREE_RECORD_KIND {
            violations.push(InspectToSourceTreeViolation::WrongRecordKind);
        }
        if self.schema_version != INSPECT_TO_SOURCE_TREE_SCHEMA_VERSION {
            violations.push(InspectToSourceTreeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(InspectToSourceTreeViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_nodes(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("inspect-to-source tree packet serializes"),
        ) {
            violations.push(InspectToSourceTreeViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("inspect-to-source tree packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Inspect-to-Source Tree Mapping\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Nodes: {} ({} downgraded)\n",
            self.nodes.len(),
            self.downgraded_node_count()
        ));
        out.push_str(&format!(
            "- Tree kinds: {} / {}\n",
            self.represented_tree_kinds().len(),
            InspectTreeKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Mapping qualities: {} / {}\n",
            self.represented_mapping_qualities().len(),
            NodeMappingQualityClass::ALL.len()
        ));
        out.push_str("\n## Nodes\n\n");
        for node in &self.nodes {
            out.push_str(&format!(
                "- **{}** ({})\n",
                node.node_id,
                node.tree_kind.as_str()
            ));
            out.push_str(&format!("  - {}\n", node.label_summary));
            out.push_str(&format!("  - {}\n", node.chip_tokens()));
            out.push_str(&format!(
                "  - source_anchor=`{}` source_nav={} mutation={}\n",
                node.source_anchor_ref.as_deref().unwrap_or("none"),
                node.source_navigation_offered,
                node.mutation_offered,
            ));
            if let Some(label) = &node.degraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in inspect-to-source tree export.
#[derive(Debug)]
pub enum InspectToSourceTreeArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<InspectToSourceTreeViolation>),
}

impl fmt::Display for InspectToSourceTreeArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "inspect-to-source tree export parse failed: {error}"
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
                    "inspect-to-source tree export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for InspectToSourceTreeArtifactError {}

/// Validation failures emitted by [`InspectToSourceTreePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectToSourceTreeViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required inspectable tree kind is represented by no node.
    RequiredTreeKindMissing,
    /// A required mapping-quality class is represented by no node.
    RequiredMappingQualityMissing,
    /// The packet demonstrates no continuity-preserving downgrade node.
    DowngradedNodeCaseMissing,
    /// A node is incomplete.
    NodeIncomplete,
    /// A node's continuity route disagrees with its mapping quality.
    ContinuityRouteMismatch,
    /// A node offers navigation or mutation before its mapping label is resolved.
    AffordanceBeforeLabel,
    /// A node's source-anchor presence is inconsistent with its mapping quality.
    SourceAnchorPresenceInconsistent,
    /// A runtime-only node masquerades as saved source state.
    RuntimeOnlyMasqueradesAsSource,
    /// A non-source-backed node claims saved source state.
    NonSourceBackedClaimsSavedSource,
    /// A node offers a mutation it cannot honestly back.
    MutationAffordanceUnbacked,
    /// A node carries a downgrade trigger without a precise label, or vice versa.
    DowngradeInconsistent,
    /// A node lacks evidence refs.
    NodeEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl InspectToSourceTreeViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredTreeKindMissing => "required_tree_kind_missing",
            Self::RequiredMappingQualityMissing => "required_mapping_quality_missing",
            Self::DowngradedNodeCaseMissing => "downgraded_node_case_missing",
            Self::NodeIncomplete => "node_incomplete",
            Self::ContinuityRouteMismatch => "continuity_route_mismatch",
            Self::AffordanceBeforeLabel => "affordance_before_label",
            Self::SourceAnchorPresenceInconsistent => "source_anchor_presence_inconsistent",
            Self::RuntimeOnlyMasqueradesAsSource => "runtime_only_masquerades_as_source",
            Self::NonSourceBackedClaimsSavedSource => "non_source_backed_claims_saved_source",
            Self::MutationAffordanceUnbacked => "mutation_affordance_unbacked",
            Self::DowngradeInconsistent => "downgrade_inconsistent",
            Self::NodeEvidenceMissing => "node_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in inspect-to-source tree export.
pub fn current_m5_inspect_to_source_tree_export(
) -> Result<InspectToSourceTreePacket, InspectToSourceTreeArtifactError> {
    let packet: InspectToSourceTreePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/inspect_to_source_tree_mapping/support_export.json"
    )))
    .map_err(InspectToSourceTreeArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(InspectToSourceTreeArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &InspectToSourceTreePacket,
    violations: &mut Vec<InspectToSourceTreeViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        INSPECT_TO_SOURCE_TREE_SCHEMA_REF,
        INSPECT_TO_SOURCE_TREE_DOC_REF,
        INSPECT_TO_SOURCE_TREE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(InspectToSourceTreeViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &InspectToSourceTreePacket,
    violations: &mut Vec<InspectToSourceTreeViolation>,
) {
    let tree_kinds = packet.represented_tree_kinds();
    for required in InspectTreeKind::ALL {
        if !tree_kinds.contains(&required) {
            violations.push(InspectToSourceTreeViolation::RequiredTreeKindMissing);
            break;
        }
    }

    let qualities = packet.represented_mapping_qualities();
    for required in NodeMappingQualityClass::ALL {
        if !qualities.contains(&required) {
            violations.push(InspectToSourceTreeViolation::RequiredMappingQualityMissing);
            break;
        }
    }

    if !packet
        .nodes
        .iter()
        .any(|n| n.has_downgrade() && n.downgrade_consistent())
    {
        violations.push(InspectToSourceTreeViolation::DowngradedNodeCaseMissing);
    }
}

fn validate_nodes(
    packet: &InspectToSourceTreePacket,
    violations: &mut Vec<InspectToSourceTreeViolation>,
) {
    for node in &packet.nodes {
        if !node.is_complete() {
            violations.push(InspectToSourceTreeViolation::NodeIncomplete);
        }
        if !node.continuity_route_ok() {
            violations.push(InspectToSourceTreeViolation::ContinuityRouteMismatch);
        }
        if !node.label_before_affordance_ok() {
            violations.push(InspectToSourceTreeViolation::AffordanceBeforeLabel);
        }
        if !node.source_anchor_presence_ok() {
            violations.push(InspectToSourceTreeViolation::SourceAnchorPresenceInconsistent);
        }
        if !node.runtime_masquerade_ok() {
            violations.push(InspectToSourceTreeViolation::RuntimeOnlyMasqueradesAsSource);
        }
        if !node.saved_source_claim_ok() {
            violations.push(InspectToSourceTreeViolation::NonSourceBackedClaimsSavedSource);
        }
        if !node.mutation_affordance_ok() {
            violations.push(InspectToSourceTreeViolation::MutationAffordanceUnbacked);
        }
        if !node.downgrade_consistent() {
            violations.push(InspectToSourceTreeViolation::DowngradeInconsistent);
        }
        if node.evidence_refs.is_empty() || node.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(InspectToSourceTreeViolation::NodeEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &InspectToSourceTreePacket,
    violations: &mut Vec<InspectToSourceTreeViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(InspectToSourceTreeViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &InspectToSourceTreePacket,
    violations: &mut Vec<InspectToSourceTreeViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(InspectToSourceTreeViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "stale"
            | "downgraded"
            | "runtime only"
            | "no source"
    )
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
