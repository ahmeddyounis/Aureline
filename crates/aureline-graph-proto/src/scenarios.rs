//! Frozen five-scenario table that mirrors the fixtures under
//! `/fixtures/graph/example_workspace_graphs/`.
//!
//! Each scenario is constructed in Rust, validated by the validator,
//! and emitted through the render lane. The scenario label and
//! `doc_section` match the fixture's `__fixture__` block so reviews
//! can cross-reference the hand-authored JSON seed and the
//! machine-constructed graph value one-for-one.

use crate::model::{
    ConfidenceRollup, EdgeBody, EdgeEvidence, ExplainerCitation, FilesystemIdentity,
    FreshnessFrame, GraphEdge, GraphNode, ImpactReason, NodeBody, ProvenanceStamp, SourceAnchor,
    TopologyEdgeSlot, WorkspaceGraph, WorksetScopeRef,
};
use crate::vocab::{
    AnchorKind, CitationClass, ConfidenceLevel, EdgeClass, EdgeEvidenceState, EnvironmentClass,
    Freshness, ImpactReasonClass, InvalidationProducerTag, MissingReason, NodeClass,
    ProvenanceClass, QueryFamilyTag, ShardAffinityTag, SourceClass, StaleReason, SymbolVisibility,
    TopologyKind, TrustState, Visibility, WorksetScopeClass,
};

/// One reviewable scenario mirroring a fixture.
#[derive(Debug, Clone)]
pub struct Scenario {
    pub label: &'static str,
    pub doc_section: &'static str,
    pub graph: WorkspaceGraph,
}

/// All five frozen scenarios.
pub fn all_scenarios() -> Vec<Scenario> {
    vec![
        local_root_workspace(),
        generated_artifact_lineage(),
        provider_resources_and_citations(),
        imported_root_vendor_drop(),
        partial_workset_visibility(),
    ]
}

fn mono(scenario: u8, tick: u16) -> String {
    format!("mono:graph:{scenario:04}:00:00:00.{tick:04}")
}

fn authoritative(scenario: u8, tick: u16) -> FreshnessFrame {
    FreshnessFrame {
        freshness: Freshness::Authoritative,
        recorded_at: mono(scenario, tick),
        stale_reason: None,
        cache_key_ref: None,
        warming_progress_hint: None,
    }
}

fn authoritative_stamp(
    scenario: u8,
    tick: u16,
    source_class: SourceClass,
    producer_ref: &str,
) -> ProvenanceStamp {
    ProvenanceStamp {
        source_class,
        provenance_class: ProvenanceClass::AuthoritativeProducer,
        producer_ref: Some(producer_ref.to_string()),
        producer_version: Some("0.0.0".to_string()),
        recorded_at: mono(scenario, tick),
        imported_bundle_ref: None,
        replay_capture_ref: None,
        support_ref: None,
    }
}

fn current_root_scope() -> WorksetScopeRef {
    WorksetScopeRef {
        scope_class: WorksetScopeClass::CurrentRoot,
        scope_id: "scope:root:0".to_string(),
        visibility: Visibility::FullyVisible,
    }
}

// ------------------------------------------------------------
// §7.1 Local-root workspace.
// ------------------------------------------------------------
fn local_root_workspace() -> Scenario {
    let nodes = vec![
        GraphNode {
            node_id: "node:scope:root:0".into(),
            node_class: NodeClass::WorksetScopeNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::WorksetScope {
                scope_ref: current_root_scope(),
                display_label: Some("Current root".into()),
            },
            display_label: Some("Current root".into()),
            provenance_stamp: authoritative_stamp(1, 1, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(1, 1),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::PublicGraphQuery, QueryFamilyTag::TopologyWalk],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::WorkspaceVfsWriter,
                InvalidationProducerTag::PolicyEpochRoll,
            ],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:dir:root:0".into(),
            node_class: NodeClass::DirectoryNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Directory {
                filesystem_identity: FilesystemIdentity {
                    presentation_path: "/workspace/aureline".into(),
                    logical_workspace_identity: "ws:aureline/root:0".into(),
                    canonical_filesystem_object: "fs:vol:1:dir:1000:gen:1".into(),
                    alias_set: vec!["/workspace/aureline".into()],
                    save_target_token: "sv:ws:aureline:root:0:gen:1".into(),
                },
                role: Some("workspace_root".into()),
            },
            display_label: Some("aureline (root)".into()),
            provenance_stamp: authoritative_stamp(1, 2, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(1, 2),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::LexicalTextSearch,
                QueryFamilyTag::TopologyWalk,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![
                ShardAffinityTag::WorkspaceRootLocal,
                ShardAffinityTag::PerRootIndex,
            ],
            invalidation_producer_tags: vec![InvalidationProducerTag::WorkspaceVfsWriter],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:dir:1000:gen:1".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:file:lib_rs".into(),
            node_class: NodeClass::FileNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::File {
                filesystem_identity: FilesystemIdentity {
                    presentation_path: "/workspace/aureline/src/lib.rs".into(),
                    logical_workspace_identity: "ws:aureline/root:0/src/lib.rs".into(),
                    canonical_filesystem_object: "fs:vol:1:file:1042:gen:3".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:1042:gen:3".into(),
                },
                media_class: Some("text_source".into()),
                language_id: Some("rust".into()),
                large_file_mode: false,
            },
            display_label: Some("src/lib.rs".into()),
            provenance_stamp: authoritative_stamp(1, 3, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(1, 3),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::LexicalTextSearch,
                QueryFamilyTag::SymbolJump,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![
                ShardAffinityTag::WorkspaceRootLocal,
                ShardAffinityTag::PerRootIndex,
            ],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::WorkspaceVfsWriter,
                InvalidationProducerTag::BufferEditorCommit,
            ],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:file:1042:gen:3".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:symbol:greet_fn".into(),
            node_class: NodeClass::SymbolNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Symbol {
                symbol_kind: "function".into(),
                declared_in_file_node_id: "node:file:lib_rs".into(),
                qualified_path: "aureline::greet".into(),
                visibility: Some(SymbolVisibility::PublicApi),
            },
            display_label: Some("aureline::greet".into()),
            provenance_stamp: authoritative_stamp(1, 4, SourceClass::SymbolResolver, "symbol_resolver.rust"),
            freshness_frame: authoritative(1, 4),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::SymbolJump,
                QueryFamilyTag::SemanticCodeSearch,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![
                ShardAffinityTag::SymbolCacheShard,
                ShardAffinityTag::PerRootIndex,
            ],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::SymbolResolverRebuild,
                InvalidationProducerTag::BufferEditorCommit,
            ],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::SymbolDefinitionSite,
                anchor_ref: "node:file:lib_rs".into(),
                line_range: Some("12:28".into()),
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:doc:readme".into(),
            node_class: NodeClass::DocNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Doc {
                doc_kind: "readme".into(),
                doc_ref: "doc:readme:root:0".into(),
                anchor_filesystem_identity: Some(FilesystemIdentity {
                    presentation_path: "/workspace/aureline/README.md".into(),
                    logical_workspace_identity: "ws:aureline/root:0/README.md".into(),
                    canonical_filesystem_object: "fs:vol:1:file:1043:gen:1".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:1043:gen:1".into(),
                }),
            },
            display_label: Some("README.md".into()),
            provenance_stamp: authoritative_stamp(1, 5, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(1, 5),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::DocsSearch, QueryFamilyTag::LexicalTextSearch],
            shard_affinity_tags: vec![
                ShardAffinityTag::DocsPackShard,
                ShardAffinityTag::WorkspaceRootLocal,
            ],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::WorkspaceVfsWriter,
                InvalidationProducerTag::DocsPackRefresh,
            ],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:file:1043:gen:1".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:owner:platform_team".into(),
            node_class: NodeClass::OwnershipNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Ownership {
                ownership_kind: "team".into(),
                ownership_ref: "owner:team:platform".into(),
                display_label: Some("Platform team".into()),
                codeowners_rule_ref: Some("codeowners:rule:platform:src".into()),
            },
            display_label: Some("Platform team".into()),
            provenance_stamp: authoritative_stamp(1, 6, SourceClass::CodeownersResolver, "codeowners_resolver"),
            freshness_frame: authoritative(1, 6),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::OwnershipLookup, QueryFamilyTag::ReviewImpactWalk],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::CodeownersRuleChange],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::CodeownersRule,
                anchor_ref: "codeowners:rule:platform:src".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
    ];

    let edges = vec![
        GraphEdge {
            edge_id: "edge:contains:root_to_lib".into(),
            edge_class: EdgeClass::Contains,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:dir:root:0".into(),
            to_node_id: "node:file:lib_rs".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(1, 7, SourceClass::WorkspaceFilesystem, "workspace_authority"),
                freshness_frame: authoritative(1, 7),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::WorkspaceVfsWriter],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:dir:1000:gen:1".into(),
                line_range: None,
            }],
        },
        GraphEdge {
            edge_id: "edge:defines:lib_to_greet".into(),
            edge_class: EdgeClass::DefinesSymbol,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:file:lib_rs".into(),
            to_node_id: "node:symbol:greet_fn".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(1, 8, SourceClass::SymbolResolver, "symbol_resolver.rust"),
                freshness_frame: authoritative(1, 8),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::SymbolJump, QueryFamilyTag::SemanticCodeSearch],
            shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::SymbolResolverRebuild],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::SymbolDefinitionSite,
                anchor_ref: "node:file:lib_rs".into(),
                line_range: Some("12:28".into()),
            }],
        },
        GraphEdge {
            edge_id: "edge:documented_by:greet_to_readme".into(),
            edge_class: EdgeClass::DocumentedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:symbol:greet_fn".into(),
            to_node_id: "node:doc:readme".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(1, 9, SourceClass::DocsPack, "docs_pack_registry"),
                freshness_frame: authoritative(1, 9),
                confidence_level: ConfidenceLevel::Medium,
                confidence_rollup: None,
            },
            body: EdgeBody {
                topology_edge_slot: None,
                impact_reasons: vec![],
                explainer_citations: vec![ExplainerCitation {
                    citation_class: CitationClass::DocEntry,
                    citation_ref: "node:doc:readme".into(),
                    line_range: Some("1:24".into()),
                    confidence_level: ConfidenceLevel::Medium,
                }],
            },
            query_family_tags: vec![QueryFamilyTag::DocsSearch, QueryFamilyTag::CitedExplainerWalk],
            shard_affinity_tags: vec![ShardAffinityTag::DocsPackShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::DocsPackRefresh],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::DocsPackEntry,
                anchor_ref: "doc:readme:root:0".into(),
                line_range: Some("1:24".into()),
            }],
        },
        GraphEdge {
            edge_id: "edge:owned_by:lib_to_platform".into(),
            edge_class: EdgeClass::OwnedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:file:lib_rs".into(),
            to_node_id: "node:owner:platform_team".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(1, 10, SourceClass::CodeownersResolver, "codeowners_resolver"),
                freshness_frame: authoritative(1, 10),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::OwnershipLookup, QueryFamilyTag::ReviewImpactWalk],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::CodeownersRuleChange],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::CodeownersRule,
                anchor_ref: "codeowners:rule:platform:src".into(),
                line_range: None,
            }],
        },
        GraphEdge {
            edge_id: "edge:scoped_by:lib_to_root_scope".into(),
            edge_class: EdgeClass::ScopedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:file:lib_rs".into(),
            to_node_id: "node:scope:root:0".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(1, 11, SourceClass::PolicyProjection, "scope_resolver"),
                freshness_frame: authoritative(1, 11),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
        },
    ];

    Scenario {
        label: "local_root_workspace",
        doc_section: "§7.1 Local-root workspace",
        graph: WorkspaceGraph {
            workspace_graph_id: "wsg:local_root_workspace:0001".into(),
            workspace_id: "ws:aureline".into(),
            recorded_at: mono(1, 1),
            producer_ref: Some("graph_authority.workspace_seed".into()),
            producer_version: Some("0.0.0".into()),
            scope_refs: vec![current_root_scope()],
            nodes,
            edges,
            notes: vec!["Local-root workspace seed: every node and edge is direct_evidence with authoritative freshness and high confidence.".into()],
        },
    }
}

// ------------------------------------------------------------
// §7.2 Generated-artifact lineage.
// ------------------------------------------------------------
fn generated_artifact_lineage() -> Scenario {
    let nodes = vec![
        GraphNode {
            node_id: "node:file:main_rs".into(),
            node_class: NodeClass::FileNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::File {
                filesystem_identity: FilesystemIdentity {
                    presentation_path: "/workspace/aureline/src/main.rs".into(),
                    logical_workspace_identity: "ws:aureline/root:0/src/main.rs".into(),
                    canonical_filesystem_object: "fs:vol:1:file:2042:gen:2".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:2042:gen:2".into(),
                },
                media_class: Some("text_source".into()),
                language_id: Some("rust".into()),
                large_file_mode: false,
            },
            display_label: Some("src/main.rs".into()),
            provenance_stamp: authoritative_stamp(2, 1, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(2, 1),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::SymbolJump, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal, ShardAffinityTag::PerRootIndex],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::WorkspaceVfsWriter,
                InvalidationProducerTag::BufferEditorCommit,
            ],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:file:2042:gen:2".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:topology:build_target:aureline_bin".into(),
            node_class: NodeClass::TopologyNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Topology {
                topology_kind: "build_target".into(),
                topology_ref: "topology:build_target:aureline_bin".into(),
                environment_class: Some(EnvironmentClass::LocalHost),
            },
            display_label: Some("build_target: aureline (bin)".into()),
            provenance_stamp: authoritative_stamp(2, 2, SourceClass::BuildToolchain, "build_toolchain.cargo"),
            freshness_frame: authoritative(2, 2),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::TopologyWalk,
                QueryFamilyTag::DependencyWalk,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::BuildToolchainRun],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:generated_artifact:aureline_bin".into(),
            node_class: NodeClass::GeneratedArtifactNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::GeneratedArtifact {
                lineage_record_ref: "lineage:build_output:aureline:bin:0001".into(),
                filesystem_identity: Some(FilesystemIdentity {
                    presentation_path: "/workspace/aureline/target/debug/aureline".into(),
                    logical_workspace_identity: "ws:aureline/root:0/target/debug/aureline".into(),
                    canonical_filesystem_object: "fs:vol:1:file:9001:gen:5".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:9001:gen:5".into(),
                }),
                generation_class: "build_output".into(),
                drift_state: "in_sync".into(),
            },
            display_label: Some("target/debug/aureline".into()),
            provenance_stamp: ProvenanceStamp {
                source_class: SourceClass::BuildToolchain,
                provenance_class: ProvenanceClass::AuthoritativeProducer,
                producer_ref: Some("build_toolchain.cargo".into()),
                producer_version: Some("0.0.0".into()),
                recorded_at: mono(2, 3),
                imported_bundle_ref: None,
                replay_capture_ref: None,
                support_ref: None,
            },
            freshness_frame: FreshnessFrame {
                freshness: Freshness::Cached,
                recorded_at: mono(2, 3),
                stale_reason: Some(StaleReason::CacheServed),
                cache_key_ref: Some("cache:build_output:aureline:bin:0001".into()),
                warming_progress_hint: None,
            },
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::GeneratedArtifactLineageWalk,
                QueryFamilyTag::TopologyWalk,
            ],
            shard_affinity_tags: vec![
                ShardAffinityTag::WorkspaceRootLocal,
                ShardAffinityTag::GraphOverlayShard,
            ],
            invalidation_producer_tags: vec![InvalidationProducerTag::BuildToolchainRun],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::GeneratedArtifactLineage,
                anchor_ref: "lineage:build_output:aureline:bin:0001".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
    ];

    let edges = vec![
        GraphEdge {
            edge_id: "edge:produces_artifact:build_target_to_bin".into(),
            edge_class: EdgeClass::ProducesArtifact,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:topology:build_target:aureline_bin".into(),
            to_node_id: "node:generated_artifact:aureline_bin".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(2, 4, SourceClass::BuildToolchain, "build_toolchain.cargo"),
                freshness_frame: authoritative(2, 4),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody {
                topology_edge_slot: Some(TopologyEdgeSlot {
                    topology_kind: TopologyKind::BuildTargetProducesArtifact,
                    environment_class: Some(EnvironmentClass::LocalHost),
                    deployment_tag: None,
                }),
                impact_reasons: vec![],
                explainer_citations: vec![],
            },
            query_family_tags: vec![
                QueryFamilyTag::TopologyWalk,
                QueryFamilyTag::GeneratedArtifactLineageWalk,
            ],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::BuildToolchainRun],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::GeneratedArtifactLineage,
                anchor_ref: "lineage:build_output:aureline:bin:0001".into(),
                line_range: None,
            }],
        },
        GraphEdge {
            edge_id: "edge:generated_from:bin_from_main".into(),
            edge_class: EdgeClass::GeneratedFrom,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:generated_artifact:aureline_bin".into(),
            to_node_id: "node:file:main_rs".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(2, 5, SourceClass::BuildToolchain, "build_toolchain.cargo"),
                freshness_frame: authoritative(2, 5),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![
                QueryFamilyTag::GeneratedArtifactLineageWalk,
                QueryFamilyTag::ImpactExplorer,
            ],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::BuildToolchainRun],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::GeneratedArtifactLineage,
                anchor_ref: "lineage:build_output:aureline:bin:0001".into(),
                line_range: None,
            }],
        },
    ];

    Scenario {
        label: "generated_artifact_lineage",
        doc_section: "§7.2 Generated-artifact lineage",
        graph: WorkspaceGraph {
            workspace_graph_id: "wsg:generated_artifact_lineage:0001".into(),
            workspace_id: "ws:aureline".into(),
            recorded_at: mono(2, 1),
            producer_ref: Some("graph_authority.workspace_seed".into()),
            producer_version: Some("0.0.0".into()),
            scope_refs: vec![current_root_scope()],
            nodes,
            edges,
            notes: vec![
                "build_target produces_artifact carries the topology_edge_slot so topology maps reuse this edge record."
                    .into(),
                "generated_artifact_node's drift_state and generation_class re-export the generated-artifact-lineage vocabulary."
                    .into(),
            ],
        },
    }
}

// ------------------------------------------------------------
// §7.3 Provider resources and citations.
// ------------------------------------------------------------
fn provider_resources_and_citations() -> Scenario {
    let nodes = vec![
        GraphNode {
            node_id: "node:topology:code_host".into(),
            node_class: NodeClass::TopologyNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Topology {
                topology_kind: "runtime_host".into(),
                topology_ref: "topology:runtime_host:code_host".into(),
                environment_class: Some(EnvironmentClass::ManagedWorkspace),
            },
            display_label: Some("Code host".into()),
            provenance_stamp: authoritative_stamp(3, 1, SourceClass::ConnectedProvider, "connected_provider.code_host"),
            freshness_frame: authoritative(3, 1),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::ProviderResourceLookup],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ConnectedProviderEvent],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:provider:repo".into(),
            node_class: NodeClass::ProviderResourceNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::ProviderResource {
                provider_kind: "code_host".into(),
                provider_ref: "provider:code_host:main".into(),
                resource_handle: "repo:ahmeddyounis/Aureline".into(),
                reachability_state: Some(crate::vocab::ReachabilityState::Reachable),
            },
            display_label: Some("repo:ahmeddyounis/Aureline".into()),
            provenance_stamp: authoritative_stamp(3, 2, SourceClass::ConnectedProvider, "connected_provider.code_host"),
            freshness_frame: authoritative(3, 2),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::ProviderResourceLookup, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ConnectedProviderEvent],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ProviderResourceHandle,
                anchor_ref: "repo:ahmeddyounis/Aureline".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:provider:issue_42".into(),
            node_class: NodeClass::ProviderResourceNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::ProviderResource {
                provider_kind: "issue_tracker".into(),
                provider_ref: "provider:issue_tracker:main".into(),
                resource_handle: "issue:ahmeddyounis/Aureline#42".into(),
                reachability_state: Some(crate::vocab::ReachabilityState::Reachable),
            },
            display_label: Some("issue #42".into()),
            provenance_stamp: authoritative_stamp(3, 3, SourceClass::ConnectedProvider, "connected_provider.issue_tracker"),
            freshness_frame: authoritative(3, 3),
            confidence_level: ConfidenceLevel::Medium,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::ProviderResourceLookup, QueryFamilyTag::ImpactExplorer],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ConnectedProviderEvent],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ProviderResourceHandle,
                anchor_ref: "issue:ahmeddyounis/Aureline#42".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:file:server_rs".into(),
            node_class: NodeClass::FileNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::File {
                filesystem_identity: FilesystemIdentity {
                    presentation_path: "/workspace/aureline/src/server.rs".into(),
                    logical_workspace_identity: "ws:aureline/root:0/src/server.rs".into(),
                    canonical_filesystem_object: "fs:vol:1:file:3042:gen:1".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:3042:gen:1".into(),
                },
                media_class: Some("text_source".into()),
                language_id: Some("rust".into()),
                large_file_mode: false,
            },
            display_label: Some("src/server.rs".into()),
            provenance_stamp: authoritative_stamp(3, 4, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(3, 4),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::LexicalTextSearch, QueryFamilyTag::SymbolJump],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal, ShardAffinityTag::PerRootIndex],
            invalidation_producer_tags: vec![InvalidationProducerTag::WorkspaceVfsWriter],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:file:3042:gen:1".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:symbol:handle_request".into(),
            node_class: NodeClass::SymbolNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Symbol {
                symbol_kind: "function".into(),
                declared_in_file_node_id: "node:file:server_rs".into(),
                qualified_path: "server::handle_request".into(),
                visibility: Some(SymbolVisibility::PublicApi),
            },
            display_label: Some("server::handle_request".into()),
            provenance_stamp: authoritative_stamp(3, 5, SourceClass::SymbolResolver, "symbol_resolver.rust"),
            freshness_frame: authoritative(3, 5),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::SymbolJump, QueryFamilyTag::SemanticCodeSearch],
            shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::SymbolResolverRebuild],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::SymbolDefinitionSite,
                anchor_ref: "node:file:server_rs".into(),
                line_range: Some("30:64".into()),
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:doc:adr_0007".into(),
            node_class: NodeClass::DocNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Doc {
                doc_kind: "adr".into(),
                doc_ref: "doc:adr:0007".into(),
                anchor_filesystem_identity: None,
            },
            display_label: Some("ADR 0007".into()),
            provenance_stamp: authoritative_stamp(3, 6, SourceClass::DocsPack, "docs_pack_registry"),
            freshness_frame: authoritative(3, 6),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::DocsSearch, QueryFamilyTag::CitedExplainerWalk],
            shard_affinity_tags: vec![ShardAffinityTag::DocsPackShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::DocsPackRefresh],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::DocsPackEntry,
                anchor_ref: "doc:adr:0007".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
    ];

    let edges = vec![
        GraphEdge {
            edge_id: "edge:hosted_by:repo_to_host".into(),
            edge_class: EdgeClass::HostedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:provider:repo".into(),
            to_node_id: "node:topology:code_host".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(3, 7, SourceClass::ConnectedProvider, "connected_provider.code_host"),
                freshness_frame: authoritative(3, 7),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody {
                topology_edge_slot: Some(TopologyEdgeSlot {
                    topology_kind: TopologyKind::ProviderHostsResource,
                    environment_class: Some(EnvironmentClass::ManagedWorkspace),
                    deployment_tag: None,
                }),
                impact_reasons: vec![],
                explainer_citations: vec![],
            },
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::ProviderResourceLookup],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ConnectedProviderEvent],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
        },
        GraphEdge {
            edge_id: "edge:hosted_by:issue_to_host".into(),
            edge_class: EdgeClass::HostedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:provider:issue_42".into(),
            to_node_id: "node:topology:code_host".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(3, 8, SourceClass::ConnectedProvider, "connected_provider.issue_tracker"),
                freshness_frame: authoritative(3, 8),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody {
                topology_edge_slot: Some(TopologyEdgeSlot {
                    topology_kind: TopologyKind::ProviderHostsResource,
                    environment_class: Some(EnvironmentClass::ManagedWorkspace),
                    deployment_tag: None,
                }),
                impact_reasons: vec![],
                explainer_citations: vec![],
            },
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::ProviderResourceLookup],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ConnectedProviderEvent],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
        },
        GraphEdge {
            edge_id: "edge:references:issue_to_symbol".into(),
            edge_class: EdgeClass::ReferencesSymbol,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:provider:issue_42".into(),
            to_node_id: "node:symbol:handle_request".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::InferredRelation,
                provenance_stamp: ProvenanceStamp {
                    source_class: SourceClass::AiInference,
                    provenance_class: ProvenanceClass::InferredByHeuristic,
                    producer_ref: Some("ai_inference.issue_linker".into()),
                    producer_version: Some("0.0.0".into()),
                    recorded_at: mono(3, 9),
                    imported_bundle_ref: None,
                    replay_capture_ref: None,
                    support_ref: None,
                },
                freshness_frame: authoritative(3, 9),
                confidence_level: ConfidenceLevel::Medium,
                confidence_rollup: Some(ConfidenceRollup {
                    rolled_up_level: ConfidenceLevel::Low,
                    source_confidences: vec![ConfidenceLevel::Medium, ConfidenceLevel::Low],
                    rollup_note: Some(
                        "Heuristic text-match on the issue body; text refers to handle_request but ties are not confirmed."
                            .into(),
                    ),
                }),
            },
            body: EdgeBody {
                topology_edge_slot: None,
                impact_reasons: vec![ImpactReason {
                    reason_class: ImpactReasonClass::InferredTransitiveImpact,
                    note: Some("Issue body mentions handle_request by name; impact explorer renders with low rollup.".into()),
                    mutation_journal_ref: None,
                    review_ref: None,
                }],
                explainer_citations: vec![],
            },
            query_family_tags: vec![QueryFamilyTag::ImpactExplorer, QueryFamilyTag::CitedExplainerWalk],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard, ShardAffinityTag::AiContextShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::AiInferenceRefresh],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ProviderResourceHandle,
                anchor_ref: "issue:ahmeddyounis/Aureline#42".into(),
                line_range: None,
            }],
        },
        GraphEdge {
            edge_id: "edge:impacts:issue_to_server_file".into(),
            edge_class: EdgeClass::Impacts,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:provider:issue_42".into(),
            to_node_id: "node:file:server_rs".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::InferredRelation,
                provenance_stamp: ProvenanceStamp {
                    source_class: SourceClass::AiInference,
                    provenance_class: ProvenanceClass::InferredByHeuristic,
                    producer_ref: Some("ai_inference.issue_linker".into()),
                    producer_version: Some("0.0.0".into()),
                    recorded_at: mono(3, 10),
                    imported_bundle_ref: None,
                    replay_capture_ref: None,
                    support_ref: None,
                },
                freshness_frame: authoritative(3, 10),
                confidence_level: ConfidenceLevel::Medium,
                confidence_rollup: None,
            },
            body: EdgeBody {
                topology_edge_slot: None,
                impact_reasons: vec![ImpactReason {
                    reason_class: ImpactReasonClass::InferredTransitiveImpact,
                    note: Some("Transitive via handle_request reference.".into()),
                    mutation_journal_ref: None,
                    review_ref: None,
                }],
                explainer_citations: vec![],
            },
            query_family_tags: vec![QueryFamilyTag::ImpactExplorer],
            shard_affinity_tags: vec![ShardAffinityTag::ProviderOverlayShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::AiInferenceRefresh],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![],
        },
        GraphEdge {
            edge_id: "edge:cites:adr_0007_to_handle_request".into(),
            edge_class: EdgeClass::Cites,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:doc:adr_0007".into(),
            to_node_id: "node:symbol:handle_request".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(3, 11, SourceClass::DocsPack, "docs_pack_registry"),
                freshness_frame: authoritative(3, 11),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody {
                topology_edge_slot: None,
                impact_reasons: vec![],
                explainer_citations: vec![ExplainerCitation {
                    citation_class: CitationClass::SymbolDefinition,
                    citation_ref: "node:symbol:handle_request".into(),
                    line_range: Some("30:64".into()),
                    confidence_level: ConfidenceLevel::High,
                }],
            },
            query_family_tags: vec![QueryFamilyTag::CitedExplainerWalk, QueryFamilyTag::DocsSearch],
            shard_affinity_tags: vec![ShardAffinityTag::DocsPackShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::DocsPackRefresh],
            scope_refs: vec![current_root_scope()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::DocsPackEntry,
                anchor_ref: "doc:adr:0007".into(),
                line_range: None,
            }],
        },
    ];

    Scenario {
        label: "provider_resources_and_citations",
        doc_section: "§7.3 Provider resources and citations",
        graph: WorkspaceGraph {
            workspace_graph_id: "wsg:provider_resources_and_citations:0001".into(),
            workspace_id: "ws:aureline".into(),
            recorded_at: mono(3, 1),
            producer_ref: Some("graph_authority.workspace_seed".into()),
            producer_version: Some("0.0.0".into()),
            scope_refs: vec![current_root_scope()],
            nodes,
            edges,
            notes: vec![
                "Issue → symbol is an inferred_relation with a medium+low rollup pulling the rollup to low; cited-explainer overlay reuses the symbol node id verbatim."
                    .into(),
            ],
        },
    }
}

// ------------------------------------------------------------
// §7.4 Imported root — vendor drop.
// ------------------------------------------------------------
fn imported_root_vendor_drop() -> Scenario {
    let vendor_scope = WorksetScopeRef {
        scope_class: WorksetScopeClass::NamedWorkset,
        scope_id: "scope:vendor:acme".into(),
        visibility: Visibility::FullyVisible,
    };

    let imported_stamp = ProvenanceStamp {
        source_class: SourceClass::ImportedBundle,
        provenance_class: ProvenanceClass::ImportedExternal,
        producer_ref: Some("bundle.vendor.acme".into()),
        producer_version: Some("1.2.0".into()),
        recorded_at: mono(4, 3),
        imported_bundle_ref: Some("bundle:vendor:acme:1.2.0".into()),
        replay_capture_ref: None,
        support_ref: Some("support:import:vendor:acme:1.2.0".into()),
    };

    let imported_frame = FreshnessFrame {
        freshness: Freshness::Imported,
        recorded_at: mono(4, 3),
        stale_reason: Some(StaleReason::ImportedFromExternal),
        cache_key_ref: None,
        warming_progress_hint: None,
    };

    let nodes = vec![
        GraphNode {
            node_id: "node:scope:vendor:acme".into(),
            node_class: NodeClass::WorksetScopeNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::WorksetScope {
                scope_ref: vendor_scope.clone(),
                display_label: Some("Vendor: acme".into()),
            },
            display_label: Some("Vendor: acme".into()),
            provenance_stamp: authoritative_stamp(4, 2, SourceClass::PolicyProjection, "scope_resolver"),
            freshness_frame: authoritative(4, 2),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::PublicGraphQuery, QueryFamilyTag::TopologyWalk],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
            scope_refs: vec![vendor_scope.clone()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:imported_root:acme:1_2_0".into(),
            node_class: NodeClass::ImportedRootNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::ImportedRoot {
                import_kind: "vendor_drop".into(),
                import_ref: "import:vendor:acme:1.2.0".into(),
                filesystem_identity: Some(FilesystemIdentity {
                    presentation_path: "/workspace/aureline/vendor/acme".into(),
                    logical_workspace_identity: "ws:aureline/root:0/vendor/acme".into(),
                    canonical_filesystem_object: "fs:vol:1:dir:4000:gen:1".into(),
                    alias_set: vec!["/workspace/aureline/vendor/acme".into()],
                    save_target_token: "sv:ws:aureline:root:0:dir:4000:gen:1".into(),
                }),
                trust_state: Some(TrustState::Restricted),
            },
            display_label: Some("vendor/acme@1.2.0".into()),
            provenance_stamp: imported_stamp.clone(),
            freshness_frame: imported_frame.clone(),
            confidence_level: ConfidenceLevel::Medium,
            confidence_rollup: Some(ConfidenceRollup {
                rolled_up_level: ConfidenceLevel::Medium,
                source_confidences: vec![ConfidenceLevel::Medium],
                rollup_note: Some(
                    "Imported bundle carries medium confidence: signed upstream, no workspace-local cross-validation."
                        .into(),
                ),
            }),
            query_family_tags: vec![QueryFamilyTag::PublicGraphQuery, QueryFamilyTag::TopologyWalk],
            shard_affinity_tags: vec![ShardAffinityTag::ImportedBundleShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ImportedBundleUpdate],
            scope_refs: vec![current_root_scope(), vendor_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ImportedBundleEntry,
                anchor_ref: "bundle:vendor:acme:1.2.0".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:file:vendor_acme_lib_rs".into(),
            node_class: NodeClass::FileNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::File {
                filesystem_identity: FilesystemIdentity {
                    presentation_path: "/workspace/aureline/vendor/acme/src/lib.rs".into(),
                    logical_workspace_identity: "ws:aureline/root:0/vendor/acme/src/lib.rs".into(),
                    canonical_filesystem_object: "fs:vol:1:file:4042:gen:1".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:4042:gen:1".into(),
                },
                media_class: Some("vendor_imported".into()),
                language_id: Some("rust".into()),
                large_file_mode: false,
            },
            display_label: Some("vendor/acme/src/lib.rs".into()),
            provenance_stamp: ProvenanceStamp {
                recorded_at: mono(4, 4),
                support_ref: None,
                ..imported_stamp.clone()
            },
            freshness_frame: FreshnessFrame {
                recorded_at: mono(4, 4),
                ..imported_frame.clone()
            },
            confidence_level: ConfidenceLevel::Medium,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::LexicalTextSearch, QueryFamilyTag::DependencyWalk],
            shard_affinity_tags: vec![ShardAffinityTag::ImportedBundleShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ImportedBundleUpdate],
            scope_refs: vec![vendor_scope.clone()],
            source_anchors: vec![
                SourceAnchor {
                    anchor_kind: AnchorKind::ImportedBundleEntry,
                    anchor_ref: "bundle:vendor:acme:1.2.0".into(),
                    line_range: None,
                },
                SourceAnchor {
                    anchor_kind: AnchorKind::FilesystemIdentity,
                    anchor_ref: "fs:vol:1:file:4042:gen:1".into(),
                    line_range: None,
                },
            ],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:owner:acme_maintainers".into(),
            node_class: NodeClass::OwnershipNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Ownership {
                ownership_kind: "team".into(),
                ownership_ref: "owner:team:acme_maintainers".into(),
                display_label: Some("acme maintainers".into()),
                codeowners_rule_ref: None,
            },
            display_label: Some("acme maintainers".into()),
            provenance_stamp: ProvenanceStamp {
                recorded_at: mono(4, 5),
                support_ref: None,
                ..imported_stamp.clone()
            },
            freshness_frame: FreshnessFrame {
                recorded_at: mono(4, 5),
                ..imported_frame.clone()
            },
            confidence_level: ConfidenceLevel::Medium,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::OwnershipLookup],
            shard_affinity_tags: vec![ShardAffinityTag::ImportedBundleShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ImportedBundleUpdate],
            scope_refs: vec![vendor_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ImportedBundleEntry,
                anchor_ref: "bundle:vendor:acme:1.2.0".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
    ];

    let edges = vec![
        GraphEdge {
            edge_id: "edge:contains:imported_root_to_lib_rs".into(),
            edge_class: EdgeClass::Contains,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:imported_root:acme:1_2_0".into(),
            to_node_id: "node:file:vendor_acme_lib_rs".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::ImportedEvidence,
                provenance_stamp: ProvenanceStamp {
                    recorded_at: mono(4, 6),
                    support_ref: None,
                    ..imported_stamp.clone()
                },
                freshness_frame: FreshnessFrame {
                    recorded_at: mono(4, 6),
                    ..imported_frame.clone()
                },
                confidence_level: ConfidenceLevel::Medium,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::ImportedBundleShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ImportedBundleUpdate],
            scope_refs: vec![vendor_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ImportedBundleEntry,
                anchor_ref: "bundle:vendor:acme:1.2.0".into(),
                line_range: None,
            }],
        },
        GraphEdge {
            edge_id: "edge:owned_by:vendor_lib_to_acme".into(),
            edge_class: EdgeClass::OwnedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:file:vendor_acme_lib_rs".into(),
            to_node_id: "node:owner:acme_maintainers".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::ImportedEvidence,
                provenance_stamp: ProvenanceStamp {
                    recorded_at: mono(4, 7),
                    support_ref: None,
                    ..imported_stamp.clone()
                },
                freshness_frame: FreshnessFrame {
                    recorded_at: mono(4, 7),
                    ..imported_frame.clone()
                },
                confidence_level: ConfidenceLevel::Medium,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::OwnershipLookup, QueryFamilyTag::ReviewImpactWalk],
            shard_affinity_tags: vec![ShardAffinityTag::ImportedBundleShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::ImportedBundleUpdate],
            scope_refs: vec![vendor_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::ImportedBundleEntry,
                anchor_ref: "bundle:vendor:acme:1.2.0".into(),
                line_range: None,
            }],
        },
        GraphEdge {
            edge_id: "edge:scoped_by:imported_root_to_vendor_scope".into(),
            edge_class: EdgeClass::ScopedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:imported_root:acme:1_2_0".into(),
            to_node_id: "node:scope:vendor:acme".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(4, 8, SourceClass::PolicyProjection, "scope_resolver"),
                freshness_frame: authoritative(4, 8),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
            scope_refs: vec![vendor_scope.clone()],
            source_anchors: vec![],
        },
    ];

    Scenario {
        label: "imported_root_vendor_drop",
        doc_section: "§7.4 Imported root — vendor drop",
        graph: WorkspaceGraph {
            workspace_graph_id: "wsg:imported_root_vendor_drop:0001".into(),
            workspace_id: "ws:aureline".into(),
            recorded_at: mono(4, 1),
            producer_ref: Some("graph_authority.workspace_seed".into()),
            producer_version: Some("0.0.0".into()),
            scope_refs: vec![current_root_scope(), vendor_scope],
            nodes,
            edges,
            notes: vec![
                "Imported rows project through imported_external provenance and imported freshness; trust_state on the imported root is restricted so surfaces render a read-only / review-before-edit badge."
                    .into(),
            ],
        },
    }
}

// ------------------------------------------------------------
// §7.5 Partial workset visibility with missing anchors.
// ------------------------------------------------------------
fn partial_workset_visibility() -> Scenario {
    let workset_scope = WorksetScopeRef {
        scope_class: WorksetScopeClass::NamedWorkset,
        scope_id: "scope:workset:editor_core".into(),
        visibility: Visibility::FullyVisible,
    };
    let policy_view_scope = WorksetScopeRef {
        scope_class: WorksetScopeClass::PolicyLimitedView,
        scope_id: "scope:policy_view:restricted_module".into(),
        visibility: Visibility::PartialVisible,
    };
    let missing_scope = WorksetScopeRef {
        scope_class: WorksetScopeClass::NamedWorkset,
        scope_id: "scope:workset:editor_core".into(),
        visibility: Visibility::MissingInScope,
    };

    let stale_frame = FreshnessFrame {
        freshness: Freshness::Stale,
        recorded_at: mono(5, 9),
        stale_reason: Some(StaleReason::UpstreamInputStale),
        cache_key_ref: None,
        warming_progress_hint: None,
    };

    let nodes = vec![
        GraphNode {
            node_id: "node:scope:workset:editor_core".into(),
            node_class: NodeClass::WorksetScopeNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::WorksetScope {
                scope_ref: workset_scope.clone(),
                display_label: Some("Workset: editor_core".into()),
            },
            display_label: Some("Workset: editor_core".into()),
            provenance_stamp: authoritative_stamp(5, 2, SourceClass::PolicyProjection, "scope_resolver"),
            freshness_frame: authoritative(5, 2),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::PublicGraphQuery, QueryFamilyTag::TopologyWalk],
            shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
            invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
            scope_refs: vec![workset_scope.clone()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:policy_view:restricted_module".into(),
            node_class: NodeClass::PolicyViewNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::PolicyView {
                underlying_scope_id: "scope:workset:restricted_module".into(),
                policy_ref: "policy:redaction:restricted_module".into(),
                hidden_member_count: 4,
            },
            display_label: Some("Policy view: restricted_module".into()),
            provenance_stamp: ProvenanceStamp {
                source_class: SourceClass::PolicyProjection,
                provenance_class: ProvenanceClass::PolicyProjected,
                producer_ref: Some("policy_projector".into()),
                producer_version: Some("0.0.0".into()),
                recorded_at: mono(5, 3),
                imported_bundle_ref: None,
                replay_capture_ref: None,
                support_ref: None,
            },
            freshness_frame: authoritative(5, 3),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::PublicGraphQuery, QueryFamilyTag::TopologyWalk],
            shard_affinity_tags: vec![ShardAffinityTag::PolicyProjectedShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
            scope_refs: vec![policy_view_scope.clone()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:file:editor_core_rs".into(),
            node_class: NodeClass::FileNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::File {
                filesystem_identity: FilesystemIdentity {
                    presentation_path: "/workspace/aureline/crates/editor_core/src/lib.rs".into(),
                    logical_workspace_identity: "ws:aureline/root:0/crates/editor_core/src/lib.rs".into(),
                    canonical_filesystem_object: "fs:vol:1:file:5010:gen:2".into(),
                    alias_set: vec![],
                    save_target_token: "sv:ws:aureline:root:0:file:5010:gen:2".into(),
                },
                media_class: Some("text_source".into()),
                language_id: Some("rust".into()),
                large_file_mode: false,
            },
            display_label: Some("crates/editor_core/src/lib.rs".into()),
            provenance_stamp: authoritative_stamp(5, 4, SourceClass::WorkspaceFilesystem, "workspace_authority"),
            freshness_frame: authoritative(5, 4),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::LexicalTextSearch,
                QueryFamilyTag::SymbolJump,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![
                ShardAffinityTag::WorkspaceRootLocal,
                ShardAffinityTag::PerRootIndex,
            ],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::WorkspaceVfsWriter,
                InvalidationProducerTag::BufferEditorCommit,
            ],
            scope_refs: vec![workset_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::FilesystemIdentity,
                anchor_ref: "fs:vol:1:file:5010:gen:2".into(),
                line_range: None,
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:symbol:editor_apply_edit".into(),
            node_class: NodeClass::SymbolNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::Symbol {
                symbol_kind: "function".into(),
                declared_in_file_node_id: "node:file:editor_core_rs".into(),
                qualified_path: "editor_core::apply_edit".into(),
                visibility: Some(SymbolVisibility::PublicApi),
            },
            display_label: Some("editor_core::apply_edit".into()),
            provenance_stamp: authoritative_stamp(5, 5, SourceClass::SymbolResolver, "symbol_resolver.rust"),
            freshness_frame: authoritative(5, 5),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
            query_family_tags: vec![
                QueryFamilyTag::SymbolJump,
                QueryFamilyTag::SemanticCodeSearch,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![
                ShardAffinityTag::SymbolCacheShard,
                ShardAffinityTag::PerRootIndex,
            ],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::SymbolResolverRebuild,
                InvalidationProducerTag::BufferEditorCommit,
            ],
            scope_refs: vec![workset_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::SymbolDefinitionSite,
                anchor_ref: "node:file:editor_core_rs".into(),
                line_range: Some("44:78".into()),
            }],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
        GraphNode {
            node_id: "node:missing_anchor:restricted_helper".into(),
            node_class: NodeClass::MissingAnchorNode,
            workspace_id: "ws:aureline".into(),
            node_body: NodeBody::MissingAnchor {
                expected_node_class: NodeClass::SymbolNode,
                missing_reason: MissingReason::UnresolvedSymbol,
                last_known_ref: Some("symbol:restricted_module::legacy_helper".into()),
            },
            display_label: Some("restricted_module::legacy_helper (missing)".into()),
            provenance_stamp: authoritative_stamp(5, 6, SourceClass::SymbolResolver, "symbol_resolver.rust"),
            freshness_frame: FreshnessFrame {
                freshness: Freshness::Stale,
                recorded_at: mono(5, 6),
                stale_reason: Some(StaleReason::UpstreamInputStale),
                cache_key_ref: None,
                warming_progress_hint: None,
            },
            confidence_level: ConfidenceLevel::Low,
            confidence_rollup: None,
            query_family_tags: vec![QueryFamilyTag::SymbolJump, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::SymbolResolverRebuild],
            scope_refs: vec![missing_scope.clone()],
            source_anchors: vec![],
            impact_reasons: vec![],
            explainer_citations: vec![],
        },
    ];

    let edges = vec![
        GraphEdge {
            edge_id: "edge:defines:editor_core_to_apply_edit".into(),
            edge_class: EdgeClass::DefinesSymbol,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:file:editor_core_rs".into(),
            to_node_id: "node:symbol:editor_apply_edit".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: authoritative_stamp(5, 7, SourceClass::SymbolResolver, "symbol_resolver.rust"),
                freshness_frame: authoritative(5, 7),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::SymbolJump, QueryFamilyTag::SemanticCodeSearch],
            shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::SymbolResolverRebuild],
            scope_refs: vec![workset_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::SymbolDefinitionSite,
                anchor_ref: "node:file:editor_core_rs".into(),
                line_range: Some("44:78".into()),
            }],
        },
        GraphEdge {
            edge_id: "edge:scoped_by:editor_core_to_policy_view".into(),
            edge_class: EdgeClass::ScopedBy,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:file:editor_core_rs".into(),
            to_node_id: "node:policy_view:restricted_module".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::DirectEvidence,
                provenance_stamp: ProvenanceStamp {
                    source_class: SourceClass::PolicyProjection,
                    provenance_class: ProvenanceClass::PolicyProjected,
                    producer_ref: Some("policy_projector".into()),
                    producer_version: Some("0.0.0".into()),
                    recorded_at: mono(5, 8),
                    imported_bundle_ref: None,
                    replay_capture_ref: None,
                    support_ref: None,
                },
                freshness_frame: authoritative(5, 8),
                confidence_level: ConfidenceLevel::High,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![QueryFamilyTag::TopologyWalk, QueryFamilyTag::PublicGraphQuery],
            shard_affinity_tags: vec![ShardAffinityTag::PolicyProjectedShard],
            invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
            scope_refs: vec![policy_view_scope.clone()],
            source_anchors: vec![],
        },
        GraphEdge {
            edge_id: "edge:references:apply_edit_to_restricted_helper".into(),
            edge_class: EdgeClass::ReferencesSymbol,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:symbol:editor_apply_edit".into(),
            to_node_id: "node:missing_anchor:restricted_helper".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::StaleRelation,
                provenance_stamp: authoritative_stamp(5, 9, SourceClass::SymbolResolver, "symbol_resolver.rust"),
                freshness_frame: stale_frame.clone(),
                confidence_level: ConfidenceLevel::Low,
                confidence_rollup: Some(ConfidenceRollup {
                    rolled_up_level: ConfidenceLevel::Low,
                    source_confidences: vec![ConfidenceLevel::Medium, ConfidenceLevel::Low],
                    rollup_note: Some(
                        "Reference observed in last index pass but the target symbol is not present in the current policy-limited view; surface renders a stale badge, not a broken-link."
                            .into(),
                    ),
                }),
            },
            body: EdgeBody {
                topology_edge_slot: None,
                impact_reasons: vec![ImpactReason {
                    reason_class: ImpactReasonClass::PolicyChange,
                    note: Some(
                        "Target symbol now sits inside a policy-limited view; the reference survives as a stale anchor."
                            .into(),
                    ),
                    mutation_journal_ref: None,
                    review_ref: None,
                }],
                explainer_citations: vec![],
            },
            query_family_tags: vec![QueryFamilyTag::SymbolJump, QueryFamilyTag::ImpactExplorer],
            shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::SymbolResolverRebuild,
                InvalidationProducerTag::PolicyEpochRoll,
            ],
            scope_refs: vec![workset_scope.clone(), policy_view_scope.clone()],
            source_anchors: vec![SourceAnchor {
                anchor_kind: AnchorKind::SymbolDefinitionSite,
                anchor_ref: "node:file:editor_core_rs".into(),
                line_range: Some("44:78".into()),
            }],
        },
        GraphEdge {
            edge_id: "edge:missing_anchor_for:restricted_helper_to_apply_edit".into(),
            edge_class: EdgeClass::MissingAnchorFor,
            workspace_id: "ws:aureline".into(),
            from_node_id: "node:missing_anchor:restricted_helper".into(),
            to_node_id: "node:symbol:editor_apply_edit".into(),
            evidence: EdgeEvidence {
                evidence_state: EdgeEvidenceState::MissingAnchor,
                provenance_stamp: authoritative_stamp(5, 10, SourceClass::SymbolResolver, "symbol_resolver.rust"),
                freshness_frame: FreshnessFrame {
                    freshness: Freshness::Stale,
                    recorded_at: mono(5, 10),
                    stale_reason: Some(StaleReason::UpstreamInputStale),
                    cache_key_ref: None,
                    warming_progress_hint: None,
                },
                confidence_level: ConfidenceLevel::Low,
                confidence_rollup: None,
            },
            body: EdgeBody::default(),
            query_family_tags: vec![
                QueryFamilyTag::SymbolJump,
                QueryFamilyTag::ImpactExplorer,
                QueryFamilyTag::PublicGraphQuery,
            ],
            shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
            invalidation_producer_tags: vec![
                InvalidationProducerTag::SymbolResolverRebuild,
                InvalidationProducerTag::PolicyEpochRoll,
            ],
            scope_refs: vec![missing_scope],
            source_anchors: vec![],
        },
    ];

    Scenario {
        label: "partial_workset_visibility",
        doc_section: "§7.5 Partial workset visibility with missing anchors",
        graph: WorkspaceGraph {
            workspace_graph_id: "wsg:partial_workset_visibility:0001".into(),
            workspace_id: "ws:aureline".into(),
            recorded_at: mono(5, 1),
            producer_ref: Some("graph_authority.workspace_seed".into()),
            producer_version: Some("0.0.0".into()),
            scope_refs: vec![workset_scope, policy_view_scope],
            nodes,
            edges,
            notes: vec![
                "Workset narrows visibility to editor_core; a reference crosses into a policy-limited view and the target is no longer resolvable in the current projection, so the graph carries a missing_anchor_node plus a stale_relation edge rather than silently dropping the reference."
                    .into(),
            ],
        },
    }
}
