//! Unit tests for the target graph state card.
//!
//! These tests cover the protected walk (a workspace target with a fully
//! authoritative producer renders an `exact` / `live_workspace_authoritative`
//! card) plus the named failure drill (a partial / stale / unavailable
//! basis MUST NOT advertise authority).

use aureline_graph_proto::{
    ConfidenceLevel, NodeClass, ProvenanceClass, QueryFamilyTag, ShardAffinityTag, SourceClass,
    Visibility, WorksetScopeClass,
};
use aureline_reactive_state::{
    open_workspace_readiness, republish_workspace_readiness, LiveReactiveStore, ReadinessLabel,
    WatcherHealthPhase, WorkspaceLifecyclePhase, WorkspaceReadinessSnapshot,
};

use super::*;

fn snapshot(
    phase: WorkspaceLifecyclePhase,
    watcher: Option<WatcherHealthPhase>,
    hot_index: bool,
    command_graph: bool,
) -> WorkspaceReadinessSnapshot {
    WorkspaceReadinessSnapshot {
        workspace_id: "ws-protected".to_owned(),
        lifecycle_phase: phase,
        watcher_health: watcher,
        hot_index_ready: hot_index,
        command_graph_ready: command_graph,
        observed_at: "mono:1".to_owned(),
    }
}

fn nominal_subject() -> GraphStateCardSubject {
    GraphStateCardSubject {
        target_id: "topology_walk:workset:hot_path".to_owned(),
        node_class: NodeClass::WorksetScopeNode,
        query_family: QueryFamilyTag::TopologyWalk,
        shard_affinity: ShardAffinityTag::GraphOverlayShard,
        source_class: SourceClass::SymbolResolver,
        provenance_class: ProvenanceClass::AuthoritativeProducer,
        scope_class: WorksetScopeClass::NamedWorkset,
        scope_visibility: Visibility::FullyVisible,
        rolled_up_confidence: Some(ConfidenceLevel::High),
        partial_note: None,
    }
}

#[test]
fn protected_walk_renders_live_authoritative_card_when_workspace_ready() {
    // Workspace is fully ready, watcher healthy, gates true, provenance is
    // authoritative, scope fully visible. The wedge MUST report
    // `exact` + `live_workspace_authoritative` and `is_authoritative = true`.
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::Ready,
            Some(WatcherHealthPhase::Healthy),
            true,
            true,
        ),
        nominal_subject(),
    )
    .unwrap();
    let card = mount.latest_record().expect("initial card");
    assert_eq!(card.readiness_label_token, "exact");
    assert_eq!(card.basis_class_token, "live_workspace_authoritative");
    assert!(card.is_authoritative);
    assert_eq!(card.degraded_token, None);
    assert_eq!(
        card.prototype_label_token,
        "m1_prototype_graph_readiness_card"
    );
    assert_eq!(card.workspace_id, "ws-protected");
    assert_eq!(card.node_class_token, "workset_scope_node");
    assert_eq!(card.query_family_token, "topology_walk");
    assert_eq!(card.provenance_class_token, "authoritative_producer");
    assert_eq!(card.scope_visibility_token, "fully_visible");
    assert!(
        card.claim_limits
            .iter()
            .any(|l| l.token == "workspace_local_only"),
        "claim limits must always quote the workspace-local row verbatim",
    );
    let plain = card.render_plaintext();
    assert!(plain.contains("m1_prototype_graph_readiness_card"));
    assert!(plain.contains("authoritative=true"));
    assert!(plain.contains("workspace_local_only"));
}

#[test]
fn failure_drill_partial_basis_refuses_authoritative_claim() {
    // The chosen target graph is in a partial / warming state. The card
    // MUST surface `partial` + `partial_subscope`, must NOT advertise
    // authority, and MUST badge the surface as `Partial` rather than
    // collapsing to "loading".
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::PartiallyReady,
            Some(WatcherHealthPhase::Warming),
            false,
            true,
        ),
        GraphStateCardSubject {
            partial_note: Some("Backend folders are still warming.".to_owned()),
            ..nominal_subject()
        },
    )
    .unwrap();
    let card = mount.latest_record().expect("initial card");
    assert_eq!(card.readiness_label_token, "partial");
    assert_eq!(card.basis_class_token, "partial_subscope");
    assert!(!card.is_authoritative);
    assert_eq!(card.degraded_token.as_deref(), Some("Partial"));
    assert_eq!(
        card.partial_note.as_deref(),
        Some("Backend folders are still warming.")
    );
}

#[test]
fn stale_after_invalidation_surfaces_reason_and_not_authoritative() {
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::Degraded,
            Some(WatcherHealthPhase::FallbackPolling),
            true,
            true,
        ),
        nominal_subject(),
    )
    .unwrap();
    let card = mount.latest_record().expect("initial card");
    assert_eq!(card.readiness_label_token, "stale");
    assert_eq!(card.basis_class_token, "stale_after_invalidation");
    assert!(!card.is_authoritative);
    assert_eq!(card.degraded_token.as_deref(), Some("Stale"));
    assert_eq!(card.not_ready_reason.as_deref(), Some("watcher_dropped"));
}

#[test]
fn closed_workspace_surfaces_unavailable_no_basis() {
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::Closed,
            Some(WatcherHealthPhase::Unavailable),
            false,
            false,
        ),
        nominal_subject(),
    )
    .unwrap();
    let card = mount.latest_record().expect("initial card");
    assert_eq!(card.readiness_label_token, "unavailable");
    assert_eq!(card.basis_class_token, "unavailable_no_basis");
    assert!(!card.is_authoritative);
    assert_eq!(card.degraded_token.as_deref(), Some("Offline"));
}

#[test]
fn out_of_scope_subject_surfaces_out_of_scope_basis() {
    // Workspace is fully ready, but the subject is outside the active
    // scope. The card MUST report `out_of_scope_for_current_workspace`
    // even though the readiness projection alone would say `exact`.
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::Ready,
            Some(WatcherHealthPhase::Healthy),
            true,
            true,
        ),
        GraphStateCardSubject {
            scope_visibility: Visibility::MissingInScope,
            scope_class: WorksetScopeClass::PolicyLimitedView,
            ..nominal_subject()
        },
    )
    .unwrap();
    let card = mount.latest_record().expect("initial card");
    assert_eq!(card.basis_class_token, "out_of_scope_for_current_workspace");
    assert!(!card.is_authoritative);
    assert_eq!(card.degraded_token.as_deref(), Some("Limited"));
}

#[test]
fn imported_basis_never_advertised_as_authoritative_even_when_provenance_says_authoritative() {
    // Even if a buggy producer claimed `AuthoritativeProducer` for an
    // imported snapshot, the readiness label drives the basis class:
    // `imported` always maps to `imported_bundle`, not authoritative.
    let projection = aureline_reactive_state::ReadinessProjection {
        query_family: "workspace.readiness".to_owned(),
        scope_ref: aureline_reactive_state::ScopeRef {
            class: aureline_reactive_state::ScopeClass::Workspace,
            id: "ws-protected".to_owned(),
        },
        subscription_id: 1,
        snapshot_epoch: 1,
        delta_seq: 0,
        frame_class: aureline_reactive_state::FrameClass::Snapshot,
        freshness: aureline_reactive_state::Freshness::Imported,
        completeness: aureline_reactive_state::Completeness::Full,
        readiness_label: ReadinessLabel::Imported,
        not_ready_reason: None,
        producer_id: "aureline.workspace.readiness".to_owned(),
        producer_version: None,
        observed_at: "mono:1".to_owned(),
    };
    let card = materialize_graph_state_card(&projection, &nominal_subject());
    assert_eq!(card.basis_class_token, "imported_bundle");
    assert!(!card.is_authoritative);
    assert_eq!(card.degraded_token.as_deref(), Some("Cached"));
}

#[test]
fn republish_refreshes_card_via_live_store_without_drift() {
    // Tie the card to live state: drive a workspace through partial -> ready
    // and confirm the mount refreshes the record from the same live store,
    // not from a private cache.
    let store = LiveReactiveStore::new();
    let (mount, sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::PartiallyReady,
            Some(WatcherHealthPhase::Warming),
            false,
            true,
        ),
        nominal_subject(),
    )
    .unwrap();
    let first = mount.latest_record().expect("initial card");
    assert_eq!(first.basis_class_token, "partial_subscope");
    assert!(!first.is_authoritative);

    republish_workspace_readiness(
        &store,
        sid,
        &snapshot(
            WorkspaceLifecyclePhase::Ready,
            Some(WatcherHealthPhase::Healthy),
            true,
            true,
        ),
    )
    .unwrap();
    let second = mount.latest_record().expect("updated card");
    assert_eq!(second.basis_class_token, "live_workspace_authoritative");
    assert!(second.is_authoritative);
    // The card mount and a workspace-readiness chip mount that attach to
    // the same subscription cannot drift; the live store fans out one
    // projection to both observers.
    assert!(store.observer_count(sid) >= 1);
}

#[test]
fn unmount_stops_card_refresh() {
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::Ready,
            Some(WatcherHealthPhase::Healthy),
            true,
            true,
        ),
        nominal_subject(),
    )
    .unwrap();
    let token = mount.token();
    mount.unmount(&store).unwrap();
    assert_eq!(store.observer_count(token.subscription_id), 0);
}

#[test]
fn render_plaintext_lists_every_field_a_reviewer_needs() {
    let store = LiveReactiveStore::new();
    let (mount, _sid) = GraphStateCardMount::open_and_mount(
        &store,
        &snapshot(
            WorkspaceLifecyclePhase::Degraded,
            Some(WatcherHealthPhase::FallbackPolling),
            true,
            true,
        ),
        nominal_subject(),
    )
    .unwrap();
    let plain = mount.latest_record().unwrap().render_plaintext();
    for token in [
        "m1_prototype_graph_readiness_card",
        "topology_walk:workset:hot_path",
        "topology_walk",
        "stale",
        "stale_after_invalidation",
        "authoritative=false",
        "watcher_dropped",
        "workspace_local_only",
        "no_refactor_scope_expansion",
    ] {
        assert!(
            plain.contains(token),
            "plaintext missing token {token}: {plain}"
        );
    }
}

#[test]
fn fixture_cases_match_expected_card_classification() {
    use serde::Deserialize;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct CaseFixture {
        record_kind: String,
        schema_version: u32,
        #[allow(dead_code)]
        case_id: String,
        #[allow(dead_code)]
        title: String,
        input: CaseInput,
        expect: CaseExpect,
    }

    #[derive(Debug, Deserialize)]
    struct CaseInput {
        workspace_id: String,
        lifecycle_phase: String,
        watcher_health: Option<String>,
        hot_index_ready: bool,
        command_graph_ready: bool,
        observed_at: String,
        subject: CaseSubject,
    }

    #[derive(Debug, Deserialize)]
    struct CaseSubject {
        target_id: String,
        node_class: String,
        query_family: String,
        shard_affinity: String,
        source_class: String,
        provenance_class: String,
        scope_class: String,
        scope_visibility: String,
        rolled_up_confidence: Option<String>,
        partial_note: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct CaseExpect {
        readiness_label: String,
        basis_class: String,
        is_authoritative: bool,
        degraded_token: Option<String>,
        not_ready_reason: Option<String>,
    }

    fn parse_node_class(s: &str) -> NodeClass {
        for v in NodeClass::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown node_class token: {s}");
    }
    fn parse_query_family(s: &str) -> QueryFamilyTag {
        for v in QueryFamilyTag::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown query_family token: {s}");
    }
    fn parse_shard(s: &str) -> ShardAffinityTag {
        for v in ShardAffinityTag::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown shard_affinity token: {s}");
    }
    fn parse_source(s: &str) -> SourceClass {
        for v in SourceClass::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown source_class token: {s}");
    }
    fn parse_provenance(s: &str) -> ProvenanceClass {
        for v in ProvenanceClass::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown provenance_class token: {s}");
    }
    fn parse_scope(s: &str) -> WorksetScopeClass {
        for v in WorksetScopeClass::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown scope_class token: {s}");
    }
    fn parse_visibility(s: &str) -> Visibility {
        for v in Visibility::all() {
            if v.as_str() == s {
                return *v;
            }
        }
        panic!("unknown scope_visibility token: {s}");
    }
    fn parse_confidence(s: &str) -> ConfidenceLevel {
        for v in [
            ConfidenceLevel::High,
            ConfidenceLevel::Medium,
            ConfidenceLevel::Low,
            ConfidenceLevel::Unknown,
        ] {
            if v.as_str() == s {
                return v;
            }
        }
        panic!("unknown confidence token: {s}");
    }

    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/graph/m1_readiness_cases");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .expect("m1_readiness_cases directory must exist")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(
        fixtures.len() >= 5,
        "expected at least 5 m1_readiness_cases fixtures (nominal + degraded variants), found {}",
        fixtures.len()
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let fixture: CaseFixture = serde_json::from_str(&payload).expect("fixture must parse");
        assert_eq!(
            fixture.record_kind, "graph_state_card_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let lifecycle_phase = WorkspaceLifecyclePhase::from_token(&fixture.input.lifecycle_phase)
            .unwrap_or_else(|| panic!("unknown lifecycle_phase in {path:?}"));
        let watcher = fixture.input.watcher_health.as_deref().map(|t| {
            WatcherHealthPhase::from_token(t)
                .unwrap_or_else(|| panic!("unknown watcher_health in {path:?}"))
        });
        let snapshot_inputs = WorkspaceReadinessSnapshot {
            workspace_id: fixture.input.workspace_id,
            lifecycle_phase,
            watcher_health: watcher,
            hot_index_ready: fixture.input.hot_index_ready,
            command_graph_ready: fixture.input.command_graph_ready,
            observed_at: fixture.input.observed_at,
        };

        let subject = GraphStateCardSubject {
            target_id: fixture.input.subject.target_id,
            node_class: parse_node_class(&fixture.input.subject.node_class),
            query_family: parse_query_family(&fixture.input.subject.query_family),
            shard_affinity: parse_shard(&fixture.input.subject.shard_affinity),
            source_class: parse_source(&fixture.input.subject.source_class),
            provenance_class: parse_provenance(&fixture.input.subject.provenance_class),
            scope_class: parse_scope(&fixture.input.subject.scope_class),
            scope_visibility: parse_visibility(&fixture.input.subject.scope_visibility),
            rolled_up_confidence: fixture
                .input
                .subject
                .rolled_up_confidence
                .as_deref()
                .map(parse_confidence),
            partial_note: fixture.input.subject.partial_note,
        };

        let store = LiveReactiveStore::new();
        let (_sid, projection) = open_workspace_readiness(&store, &snapshot_inputs).unwrap();
        let card = materialize_graph_state_card(&projection, &subject);
        assert_eq!(
            card.readiness_label_token, fixture.expect.readiness_label,
            "readiness_label mismatch in {path:?}"
        );
        assert_eq!(
            card.basis_class_token, fixture.expect.basis_class,
            "basis_class mismatch in {path:?}"
        );
        assert_eq!(
            card.is_authoritative, fixture.expect.is_authoritative,
            "is_authoritative mismatch in {path:?}"
        );
        assert_eq!(
            card.degraded_token, fixture.expect.degraded_token,
            "degraded_token mismatch in {path:?}"
        );
        assert_eq!(
            card.not_ready_reason, fixture.expect.not_ready_reason,
            "not_ready_reason mismatch in {path:?}"
        );
    }
}
