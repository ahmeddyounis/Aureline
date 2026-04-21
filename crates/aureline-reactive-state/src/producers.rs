//! Sample producer factories.
//!
//! Four families the spec for the prototype asks for: shell
//! health, workspace readiness, file-identity, and at least one
//! derived / materialized-view state. Each factory builds a
//! [`Producer`] the store can register; the producer's authority
//! class, derivation class, view class, and initial
//! `producer_refs` match what the ADR would expect for its
//! lane. Scenarios in the harness script the lifecycle on top.

use crate::envelope::{
    AuthorityClass, BackpressureMode, DerivationClass, InputDigest, ProducerRef, ScopeClass,
    ScopeRef, ViewClass,
};
use crate::store::Producer;

/// Produce a stable producer instance tag that does not depend
/// on the host. The ADR requires the instance to carry enough
/// identity for consumers to distinguish restarts; the
/// prototype names a synthetic host + pid + boot-epoch triple
/// so the committed artifacts stay byte-stable across hosts.
pub fn synthetic_instance(producer_id: &str, boot_epoch: u32, pid: u32) -> String {
    format!("synthetic-host/pid-{pid}/boot-{boot_epoch:010}/{producer_id}")
}

/// Build a workspace scope ref.
pub fn workspace(id: &str) -> ScopeRef {
    ScopeRef {
        class: ScopeClass::Workspace,
        id: id.to_owned(),
    }
}

/// Build a window scope ref.
pub fn window(id: &str) -> ScopeRef {
    ScopeRef {
        class: ScopeClass::Window,
        id: id.to_owned(),
    }
}

/// Shell health subscription. Authority class = `execution`,
/// derivation class = `authoritative`. Models the shell
/// process-health frames the shell / command lane publishes
/// (e.g. `execution_shell_health` family). View class is
/// `ephemeral_projection` — the status bar reads it live and
/// never persists.
pub fn shell_health(scope: ScopeRef) -> Producer {
    let producer_id = "aureline.execution.shell";
    Producer {
        query_family: "execution.shell_health".to_owned(),
        scope_ref: scope,
        authority_class: AuthorityClass::Execution,
        derivation_class: DerivationClass::Authoritative,
        view_class: ViewClass::EphemeralProjection,
        backpressure_mode: BackpressureMode::Realtime,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: synthetic_instance(producer_id, 2026041900, 4800),
            producer_version: Some("shell-0.0.1-pre".to_owned()),
            input_digests: vec![],
            derivation_epoch: None,
            source: None,
        }],
    }
}

/// Workspace readiness subscription. Authority class =
/// `workspace_vfs`, derivation class = `authoritative`. Models
/// the warming → full transition a workspace goes through when
/// the VFS first attaches. View class is
/// `durable_local_materialization` so consumers may cache
/// rebuildable state.
pub fn workspace_readiness(scope: ScopeRef) -> Producer {
    let producer_id = "aureline.vfs.readiness";
    Producer {
        query_family: "vfs.workspace_readiness".to_owned(),
        scope_ref: scope,
        authority_class: AuthorityClass::WorkspaceVfs,
        derivation_class: DerivationClass::Authoritative,
        view_class: ViewClass::DurableLocalMaterialization,
        backpressure_mode: BackpressureMode::Coalesced,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: synthetic_instance(producer_id, 2026041900, 4812),
            producer_version: Some("vfs-0.1.0-pre".to_owned()),
            input_digests: vec![],
            derivation_epoch: None,
            source: None,
        }],
    }
}

/// File-identity subscription. Authority class =
/// `workspace_vfs`, derivation class = `authoritative`. Models
/// file-identity change notifications (canonical path, alias
/// set, strongest generation token moves). The emitted digests
/// are what derived lanes cite in their `upstream_input_stale`
/// resyncs.
pub fn file_identity(scope: ScopeRef) -> Producer {
    let producer_id = "aureline.vfs.file_identity";
    Producer {
        query_family: "vfs.file_identity".to_owned(),
        scope_ref: scope,
        authority_class: AuthorityClass::WorkspaceVfs,
        derivation_class: DerivationClass::Authoritative,
        view_class: ViewClass::DurableLocalMaterialization,
        backpressure_mode: BackpressureMode::Realtime,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: synthetic_instance(producer_id, 2026041900, 4813),
            producer_version: Some("vfs-0.1.0-pre".to_owned()),
            input_digests: vec![],
            derivation_epoch: None,
            source: None,
        }],
    }
}

/// Derived diagnostics view. Authority class =
/// `derived_knowledge`, derivation class = `derived`. Consumes
/// [`file_identity`] as its upstream input digest. When the
/// upstream digest moves, the derived producer MUST emit a
/// `resync_required` with stale_reason = `upstream_input_stale`;
/// this factory names the input digest the scenario script
/// later drifts.
pub fn derived_diagnostics(scope: ScopeRef, upstream_digest: &str) -> Producer {
    let producer_id = "aureline.derived.diagnostics";
    Producer {
        query_family: "language.diagnostics".to_owned(),
        scope_ref: scope,
        authority_class: AuthorityClass::DerivedKnowledge,
        derivation_class: DerivationClass::Derived,
        view_class: ViewClass::DurableLocalMaterialization,
        backpressure_mode: BackpressureMode::Realtime,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: synthetic_instance(producer_id, 2026041900, 5120),
            producer_version: Some("language-0.0.1-pre".to_owned()),
            input_digests: vec![InputDigest {
                name: "vfs.file_identity@upstream".to_owned(),
                digest: upstream_digest.to_owned(),
            }],
            derivation_epoch: Some(1),
            source: None,
        }],
    }
}

/// Graph-backed / materialized derived view. Authority class
/// = `derived_knowledge`, derivation class = `derived`. Models
/// a workspace-wide graph neighbourhood index that sits behind
/// an `exportable_snapshot` view class so support bundles can
/// capture it.
pub fn graph_neighborhood(scope: ScopeRef, upstream_digest: &str) -> Producer {
    let producer_id = "aureline.derived.graph";
    Producer {
        query_family: "graph.neighborhood".to_owned(),
        scope_ref: scope,
        authority_class: AuthorityClass::DerivedKnowledge,
        derivation_class: DerivationClass::Derived,
        view_class: ViewClass::ExportableSnapshot,
        backpressure_mode: BackpressureMode::Coalesced,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: synthetic_instance(producer_id, 2026041900, 5401),
            producer_version: Some("graph-0.0.1-pre".to_owned()),
            input_digests: vec![InputDigest {
                name: "vfs.file_identity@upstream".to_owned(),
                digest: upstream_digest.to_owned(),
            }],
            derivation_epoch: Some(1),
            source: None,
        }],
    }
}

/// Provider-overlay subscription. Authority class =
/// `provider_overlay`, derivation class = `derived`. Used by
/// the terminal-unavailable scenario to exercise the
/// `watcher_dropped` / terminal-reason = `unavailable` path.
pub fn provider_overlay(scope: ScopeRef) -> Producer {
    let producer_id = "aureline.provider.github";
    Producer {
        query_family: "provider.ci_checks".to_owned(),
        scope_ref: scope,
        authority_class: AuthorityClass::ProviderOverlay,
        derivation_class: DerivationClass::Derived,
        view_class: ViewClass::ManagedReplicatedView,
        backpressure_mode: BackpressureMode::Coalesced,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: synthetic_instance(producer_id, 2026041900, 5301),
            producer_version: Some("provider-github-0.0.1-pre".to_owned()),
            input_digests: vec![],
            derivation_epoch: Some(1),
            source: None,
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factories_set_expected_vocabulary() {
        let s = shell_health(window("win-1"));
        assert_eq!(s.authority_class.as_str(), "execution");
        assert_eq!(s.derivation_class.as_str(), "authoritative");

        let w = workspace_readiness(workspace("ws"));
        assert_eq!(w.authority_class.as_str(), "workspace_vfs");

        let fi = file_identity(workspace("ws"));
        assert_eq!(fi.query_family, "vfs.file_identity");

        let d = derived_diagnostics(workspace("ws"), "sha256:aa");
        assert_eq!(d.derivation_class.as_str(), "derived");
        assert_eq!(d.producer_refs[0].input_digests.len(), 1);

        let p = provider_overlay(workspace("ws"));
        assert_eq!(p.view_class.as_str(), "managed_replicated_view");
    }
}
