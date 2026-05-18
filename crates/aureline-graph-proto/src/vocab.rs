//! Frozen semantic-workspace-graph vocabulary.
//!
//! Mirrors the enums and field sets in `docs/graph/workspace_graph_seed.md`
//! and the boundary schema at `schemas/graph/workspace_graph_seed.schema.json`.
//! Every `as_str()` here matches the exact token emitted in JSON and
//! enumerated in the schema.
//!
//! Adding a value to any of these enums is additive-minor and bumps
//! `WORKSPACE_GRAPH_SCHEMA_VERSION`. Repurposing a value is breaking.

/// Integer schema version for this workspace-graph vocabulary export.
pub const WORKSPACE_GRAPH_SCHEMA_VERSION: u32 = 1;

macro_rules! string_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Name:ident {
            $( $(#[$var_meta:meta])* $Variant:ident => $token:literal ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $Name {
            $( $(#[$var_meta])* $Variant ),+
        }

        impl $Name {
            /// Exact token emitted in JSON and enumerated in the
            /// boundary schema.
            pub fn as_str(self) -> &'static str {
                match self {
                    $( Self::$Variant => $token ),+
                }
            }

            /// All variants in declaration order. Deterministic; used
            /// by tests and the render layer to enforce byte-stability.
            pub fn all() -> &'static [Self] {
                const VALUES: &[$Name] = &[ $( $Name::$Variant ),+ ];
                VALUES
            }
        }
    };
}

string_enum! {
    /// Twelve-member node-class vocabulary.
    pub enum NodeClass {
        FileNode => "file_node",
        DirectoryNode => "directory_node",
        SymbolNode => "symbol_node",
        DocNode => "doc_node",
        OwnershipNode => "ownership_node",
        TopologyNode => "topology_node",
        ProviderResourceNode => "provider_resource_node",
        GeneratedArtifactNode => "generated_artifact_node",
        ImportedRootNode => "imported_root_node",
        WorksetScopeNode => "workset_scope_node",
        PolicyViewNode => "policy_view_node",
        MissingAnchorNode => "missing_anchor_node",
    }
}

string_enum! {
    /// Twenty-member edge-class vocabulary.
    pub enum EdgeClass {
        Contains => "contains",
        DefinesSymbol => "defines_symbol",
        ReferencesSymbol => "references_symbol",
        ImportsModule => "imports_module",
        DependsOn => "depends_on",
        OwnedBy => "owned_by",
        DocumentedBy => "documented_by",
        Cites => "cites",
        GeneratedFrom => "generated_from",
        MirrorsUpstream => "mirrors_upstream",
        DeployedTo => "deployed_to",
        RunsIn => "runs_in",
        HostedBy => "hosted_by",
        ProducesArtifact => "produces_artifact",
        ConsumesArtifact => "consumes_artifact",
        Impacts => "impacts",
        Explains => "explains",
        ScopedBy => "scoped_by",
        Aliases => "aliases",
        MissingAnchorFor => "missing_anchor_for",
    }
}

string_enum! {
    /// Five-member evidence-state vocabulary.
    pub enum EdgeEvidenceState {
        DirectEvidence => "direct_evidence",
        ImportedEvidence => "imported_evidence",
        InferredRelation => "inferred_relation",
        StaleRelation => "stale_relation",
        MissingAnchor => "missing_anchor",
    }
}

string_enum! {
    /// Seventeen-member source-class vocabulary.
    pub enum SourceClass {
        WorkspaceFilesystem => "workspace_filesystem",
        BufferEditor => "buffer_editor",
        SymbolResolver => "symbol_resolver",
        DocsPack => "docs_pack",
        CodeownersResolver => "codeowners_resolver",
        BuildToolchain => "build_toolchain",
        CodegenTool => "codegen_tool",
        PackageResolver => "package_resolver",
        NotebookKernel => "notebook_kernel",
        PreviewRuntime => "preview_runtime",
        ConnectedProvider => "connected_provider",
        RemoteAgent => "remote_agent",
        AiInference => "ai_inference",
        ImportedBundle => "imported_bundle",
        ReplayCapture => "replay_capture",
        PolicyProjection => "policy_projection",
        ManualAnnotation => "manual_annotation",
    }
}

string_enum! {
    /// Seven-member provenance-class vocabulary.
    pub enum ProvenanceClass {
        AuthoritativeProducer => "authoritative_producer",
        ProjectedFromProducer => "projected_from_producer",
        ImportedExternal => "imported_external",
        ReplayedCapture => "replayed_capture",
        InferredByHeuristic => "inferred_by_heuristic",
        PolicyProjected => "policy_projected",
        ManuallyAnnotated => "manually_annotated",
    }
}

string_enum! {
    /// Six-member freshness vocabulary re-exporting ADR-0005.
    pub enum Freshness {
        Authoritative => "authoritative",
        Warming => "warming",
        Cached => "cached",
        Stale => "stale",
        Replayed => "replayed",
        Imported => "imported",
    }
}

string_enum! {
    /// Fifteen-member stale-reason vocabulary. A non-authoritative
    /// frame MUST carry a stale_reason; authoritative frames MUST
    /// carry `None`.
    pub enum StaleReason {
        ProducerRestart => "producer_restart",
        AuthorityEpochRolled => "authority_epoch_rolled",
        PolicyEpochChanged => "policy_epoch_changed",
        WatcherDropped => "watcher_dropped",
        QueueSaturation => "queue_saturation",
        UpstreamInputStale => "upstream_input_stale",
        ExplicitRefreshRequested => "explicit_refresh_requested",
        CacheServed => "cache_served",
        ReplayedFromBundle => "replayed_from_bundle",
        ImportedFromExternal => "imported_from_external",
        ScopeRemoved => "scope_removed",
        CausalityLost => "causality_lost",
        ReindexFailed => "reindex_failed",
        GeneratorChanged => "generator_changed",
        ProviderUnreachable => "provider_unreachable",
    }
}

string_enum! {
    /// Four-member confidence vocabulary. The rollup floor rule:
    /// any low contributor pulls the rollup to at least low; any
    /// unknown contributor pulls to at least low.
    pub enum ConfidenceLevel {
        High => "high",
        Medium => "medium",
        Low => "low",
        Unknown => "unknown",
    }
}

impl ConfidenceLevel {
    /// Rank used by the rollup floor rule. Lower number = stronger.
    /// `unknown` counts as low for rollup purposes per the doc.
    fn rank(self) -> u8 {
        match self {
            Self::High => 0,
            Self::Medium => 1,
            Self::Low => 2,
            Self::Unknown => 2,
        }
    }

    /// Roll up a list of contributor confidences per the doc: any
    /// low/unknown pulls the rollup to at least low; otherwise the
    /// weakest contributor wins. Returns `None` for an empty list.
    pub fn roll_up(contributors: &[ConfidenceLevel]) -> Option<ConfidenceLevel> {
        if contributors.is_empty() {
            return None;
        }
        let has_unknown = contributors.iter().any(|c| matches!(c, Self::Unknown));
        let weakest_rank = contributors.iter().map(|c| c.rank()).max().unwrap_or(0);
        let base = match weakest_rank {
            0 => Self::High,
            1 => Self::Medium,
            _ => Self::Low,
        };
        Some(if has_unknown && !matches!(base, Self::Low) {
            Self::Low
        } else {
            base
        })
    }
}

string_enum! {
    /// Fifteen-member provisional query-family tag.
    pub enum QueryFamilyTag {
        LexicalTextSearch => "lexical_text_search",
        SymbolJump => "symbol_jump",
        SemanticCodeSearch => "semantic_code_search",
        DocsSearch => "docs_search",
        OwnershipLookup => "ownership_lookup",
        TopologyWalk => "topology_walk",
        ImpactExplorer => "impact_explorer",
        DependencyWalk => "dependency_walk",
        GeneratedArtifactLineageWalk => "generated_artifact_lineage_walk",
        ProviderResourceLookup => "provider_resource_lookup",
        CitedExplainerWalk => "cited_explainer_walk",
        AiContextAssembly => "ai_context_assembly",
        PublicGraphQuery => "public_graph_query",
        ReviewImpactWalk => "review_impact_walk",
        SupportExportWalk => "support_export_walk",
    }
}

string_enum! {
    /// Ten-member provisional shard-affinity tag.
    pub enum ShardAffinityTag {
        WorkspaceRootLocal => "workspace_root_local",
        PerRootIndex => "per_root_index",
        SymbolCacheShard => "symbol_cache_shard",
        DocsPackShard => "docs_pack_shard",
        GraphOverlayShard => "graph_overlay_shard",
        ProviderOverlayShard => "provider_overlay_shard",
        AiContextShard => "ai_context_shard",
        PolicyProjectedShard => "policy_projected_shard",
        EphemeralSessionShard => "ephemeral_session_shard",
        ImportedBundleShard => "imported_bundle_shard",
    }
}

string_enum! {
    /// Sixteen-member provisional invalidation-producer tag.
    pub enum InvalidationProducerTag {
        WorkspaceVfsWriter => "workspace_vfs_writer",
        BufferEditorCommit => "buffer_editor_commit",
        SymbolResolverRebuild => "symbol_resolver_rebuild",
        DocsPackRefresh => "docs_pack_refresh",
        CodeownersRuleChange => "codeowners_rule_change",
        BuildToolchainRun => "build_toolchain_run",
        CodegenRun => "codegen_run",
        PackageResolverRefresh => "package_resolver_refresh",
        NotebookKernelExecute => "notebook_kernel_execute",
        PreviewRuntimeRefresh => "preview_runtime_refresh",
        ConnectedProviderEvent => "connected_provider_event",
        RemoteAgentEvent => "remote_agent_event",
        AiInferenceRefresh => "ai_inference_refresh",
        ImportedBundleUpdate => "imported_bundle_update",
        ReplayCaptureReload => "replay_capture_reload",
        PolicyEpochRoll => "policy_epoch_roll",
    }
}

string_enum! {
    /// Seven-member workset / scope class re-exported from the
    /// execution-context schema.
    pub enum WorksetScopeClass {
        CurrentRoot => "current_root",
        NamedWorkset => "named_workset",
        SparseSlice => "sparse_slice",
        FullWorkspace => "full_workspace",
        PolicyLimitedView => "policy_limited_view",
        ReviewWorkspace => "review_workspace",
        CompanionSurface => "companion_surface",
    }
}

string_enum! {
    /// Four-member scope visibility vocabulary.
    pub enum Visibility {
        FullyVisible => "fully_visible",
        PartialVisible => "partial_visible",
        PolicyHidden => "policy_hidden",
        MissingInScope => "missing_in_scope",
    }
}

string_enum! {
    /// Eighteen-member impact-reason class.
    pub enum ImpactReasonClass {
        DirectEdit => "direct_edit",
        SymbolRename => "symbol_rename",
        SignatureChange => "signature_change",
        DependencyBump => "dependency_bump",
        GeneratedArtifactRegeneration => "generated_artifact_regeneration",
        PolicyChange => "policy_change",
        OwnershipChange => "ownership_change",
        ProviderResourceUpdate => "provider_resource_update",
        ImportedBundleRollover => "imported_bundle_rollover",
        WorksetScopeNarrowed => "workset_scope_narrowed",
        WorksetScopeWidened => "workset_scope_widened",
        InferredTransitiveImpact => "inferred_transitive_impact",
        ExactEdge => "exact_edge",
        SharedTarget => "shared_target",
        OwnershipRule => "ownership_rule",
        GeneratedLinkage => "generated_linkage",
        HeuristicSimilarity => "heuristic_similarity",
        PolicyCoupling => "policy_coupling",
    }
}

string_enum! {
    /// Ten-member explainer-citation class.
    pub enum CitationClass {
        SymbolDefinition => "symbol_definition",
        FileRange => "file_range",
        DocEntry => "doc_entry",
        GeneratedArtifactLineage => "generated_artifact_lineage",
        CodeownersRule => "codeowners_rule",
        TopologyEdge => "topology_edge",
        ProviderResource => "provider_resource",
        MutationJournalEntry => "mutation_journal_entry",
        ImportedBundleEntry => "imported_bundle_entry",
        ReplayCaptureEntry => "replay_capture_entry",
    }
}

string_enum! {
    /// Nine-member topology-edge kind vocabulary.
    pub enum TopologyKind {
        PackageDependsOnPackage => "package_depends_on_package",
        ServiceCallsService => "service_calls_service",
        BuildTargetProducesArtifact => "build_target_produces_artifact",
        RuntimeHostsPackage => "runtime_hosts_package",
        DeployTargetRunsService => "deploy_target_runs_service",
        ProviderHostsResource => "provider_hosts_resource",
        PackMirrorsUpstream => "pack_mirrors_upstream",
        NotebookKernelExecutesCell => "notebook_kernel_executes_cell",
        PreviewRuntimeRendersSnapshot => "preview_runtime_renders_snapshot",
    }
}

string_enum! {
    /// Ten-member environment-class hint re-exported from
    /// execution-context.
    pub enum EnvironmentClass {
        LocalHost => "local_host",
        SshRemote => "ssh_remote",
        ContainerLocal => "container_local",
        Devcontainer => "devcontainer",
        RemoteWorkspaceVm => "remote_workspace_vm",
        PrebuildRuntime => "prebuild_runtime",
        ManagedWorkspace => "managed_workspace",
        NotebookKernelLocal => "notebook_kernel_local",
        NotebookKernelRemote => "notebook_kernel_remote",
        AiSandbox => "ai_sandbox",
    }
}

string_enum! {
    /// Ten-member anchor-kind vocabulary for source anchors.
    pub enum AnchorKind {
        FilesystemIdentity => "filesystem_identity",
        SymbolDefinitionSite => "symbol_definition_site",
        DocsPackEntry => "docs_pack_entry",
        MutationJournalEntry => "mutation_journal_entry",
        GeneratedArtifactLineage => "generated_artifact_lineage",
        ProviderResourceHandle => "provider_resource_handle",
        ImportedBundleEntry => "imported_bundle_entry",
        ReplayCaptureEntry => "replay_capture_entry",
        CodeownersRule => "codeowners_rule",
        AnnotationNote => "annotation_note",
    }
}

string_enum! {
    /// Fourteen-member audit-event id vocabulary.
    pub enum AuditEventId {
        WorkspaceGraphSnapshotEmitted => "workspace_graph_snapshot_emitted",
        GraphNodeAdded => "graph_node_added",
        GraphNodeUpdated => "graph_node_updated",
        GraphNodeRetired => "graph_node_retired",
        GraphEdgeAdded => "graph_edge_added",
        GraphEdgeUpdated => "graph_edge_updated",
        GraphEdgeRetired => "graph_edge_retired",
        GraphFreshnessDowngraded => "graph_freshness_downgraded",
        GraphConfidenceDowngraded => "graph_confidence_downgraded",
        GraphMissingAnchorRecorded => "graph_missing_anchor_recorded",
        GraphWorksetScopeNarrowed => "graph_workset_scope_narrowed",
        GraphWorksetScopeWidened => "graph_workset_scope_widened",
        GraphPolicyViewProjected => "graph_policy_view_projected",
        WorkspaceGraphSchemaVersionBumped => "workspace_graph_schema_version_bumped",
    }
}

string_enum! {
    /// Optional warming-progress hint when freshness == warming.
    pub enum WarmingProgressHint {
        FirstPass => "first_pass",
        HotSetReady => "hot_set_ready",
        PartialIndex => "partial_index",
        FullIndex => "full_index",
    }
}

string_enum! {
    /// Missing-anchor reason vocabulary for `missing_anchor_node`.
    pub enum MissingReason {
        DeletedFilesystemObject => "deleted_filesystem_object",
        RenamedWithoutForwarding => "renamed_without_forwarding",
        UnresolvedSymbol => "unresolved_symbol",
        ProviderResourceUnreachable => "provider_resource_unreachable",
        ImportedBundleMissingEntry => "imported_bundle_missing_entry",
        ReplayCaptureMissingEntry => "replay_capture_missing_entry",
        PolicyHidden => "policy_hidden",
        ScopeRemoved => "scope_removed",
    }
}

string_enum! {
    /// Symbol visibility on `symbol_node`.
    pub enum SymbolVisibility {
        PublicApi => "public_api",
        WorkspaceInternal => "workspace_internal",
        CrateLocal => "crate_local",
        ModulePrivate => "module_private",
    }
}

string_enum! {
    /// Reachability-state hint re-exported from execution-context.
    pub enum ReachabilityState {
        Reachable => "reachable",
        Warming => "warming",
        Degraded => "degraded",
        Unreachable => "unreachable",
        PolicyBlocked => "policy_blocked",
    }
}

string_enum! {
    /// Trust-state hint re-exported from ADR-0001.
    pub enum TrustState {
        Restricted => "restricted",
        Trusted => "trusted",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_classes_cover_schema() {
        assert_eq!(NodeClass::all().len(), 12);
    }

    #[test]
    fn edge_classes_cover_schema() {
        assert_eq!(EdgeClass::all().len(), 20);
    }

    #[test]
    fn evidence_states_cover_schema() {
        assert_eq!(EdgeEvidenceState::all().len(), 5);
    }

    #[test]
    fn freshness_covers_schema() {
        assert_eq!(Freshness::all().len(), 6);
    }

    #[test]
    fn stale_reasons_cover_schema() {
        assert_eq!(StaleReason::all().len(), 15);
    }

    #[test]
    fn provenance_classes_cover_schema() {
        assert_eq!(ProvenanceClass::all().len(), 7);
    }

    #[test]
    fn confidence_rollup_any_low_pulls_down() {
        let got = ConfidenceLevel::roll_up(&[ConfidenceLevel::High, ConfidenceLevel::Low]);
        assert_eq!(got, Some(ConfidenceLevel::Low));
    }

    #[test]
    fn confidence_rollup_any_unknown_pulls_down() {
        let got = ConfidenceLevel::roll_up(&[ConfidenceLevel::High, ConfidenceLevel::Unknown]);
        assert_eq!(got, Some(ConfidenceLevel::Low));
    }

    #[test]
    fn confidence_rollup_all_high() {
        let got = ConfidenceLevel::roll_up(&[ConfidenceLevel::High, ConfidenceLevel::High]);
        assert_eq!(got, Some(ConfidenceLevel::High));
    }

    #[test]
    fn confidence_rollup_medium_floor() {
        let got = ConfidenceLevel::roll_up(&[ConfidenceLevel::High, ConfidenceLevel::Medium]);
        assert_eq!(got, Some(ConfidenceLevel::Medium));
    }

    #[test]
    fn confidence_rollup_empty_is_none() {
        assert_eq!(ConfidenceLevel::roll_up(&[]), None);
    }
}
