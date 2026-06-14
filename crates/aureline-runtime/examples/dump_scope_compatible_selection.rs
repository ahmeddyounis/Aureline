//! Conformance dump for the M5 scope-compatible selection packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::scope_compatible_selection_objects_and_widened_selection_review::*;
use aureline_runtime::testing_identity::TestItemIdentityClass;

const PACKET_ID: &str = "portable-selection:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn target(
    target_id: &str,
    node_kind: DurableTestNodeKind,
    identity_class: TestItemIdentityClass,
) -> SelectionTarget {
    SelectionTarget {
        target_id: target_id.to_owned(),
        node_kind,
        target_fingerprint_token: format!("fingerprint:{target_id}"),
        identity_class,
    }
}

fn local_target(target_id: &str, node_kind: DurableTestNodeKind) -> SelectionTarget {
    target(target_id, node_kind, TestItemIdentityClass::Stable)
}

fn snapshot(snapshot_id: &str, digest: &str, consumer: &str) -> SnapshotFingerprint {
    SnapshotFingerprint {
        snapshot_id: snapshot_id.to_owned(),
        snapshot_digest: digest.to_owned(),
        consumer_token: consumer.to_owned(),
    }
}

/// UI test-tree rerun-all selection over a complete framework snapshot, with a
/// parameterized template kept distinct from its two concrete invocations.
fn ui_rerun_all() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:ui:checkout-rerun".to_owned(),
        label: "Rerun the checkout suite selected from the test tree".to_owned(),
        origin_channel: SelectorChannel::Ui,
        intent: SelectionIntentKind::RerunAll,
        expansion_policy: ExpansionPolicy::ReresolveWithinSnapshot,
        snapshot_fingerprint: snapshot(
            "snapshot:framework-pack:checkout",
            "digest:checkout:v1",
            "framework_pack",
        ),
        query: SelectionQuery {
            include_query_tokens: refs(&["suite:checkout"]),
            exclude_query_tokens: Vec::new(),
            changed_since_ref: None,
        },
        pinned_targets: vec![
            local_target("framework:case:add-item", DurableTestNodeKind::ConcreteCase),
            local_target(
                "framework:template:totals",
                DurableTestNodeKind::ParameterizedTemplate,
            ),
            local_target(
                "framework:invocation:totals:usd",
                DurableTestNodeKind::ConcreteInvocation,
            ),
            local_target(
                "framework:invocation:totals:eur",
                DurableTestNodeKind::ConcreteInvocation,
            ),
        ],
        evidence_refs: refs(&["evidence:selection:ui:checkout-rerun"]),
    }
}

/// CLI rerun-failed selection pinned to the exact failing invocations.
fn cli_rerun_failed() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:cli:rerun-failed".to_owned(),
        label: "CLI rerun of only the failed invocations from the last session".to_owned(),
        origin_channel: SelectorChannel::Cli,
        intent: SelectionIntentKind::RerunFailed,
        expansion_policy: ExpansionPolicy::PinnedExact,
        snapshot_fingerprint: snapshot(
            "snapshot:framework-pack:checkout",
            "digest:checkout:v1",
            "framework_pack",
        ),
        query: SelectionQuery::default(),
        pinned_targets: vec![local_target(
            "framework:invocation:totals:eur",
            DurableTestNodeKind::ConcreteInvocation,
        )],
        evidence_refs: refs(&["evidence:selection:cli:rerun-failed"]),
    }
}

/// AI test-plan changed-since selection scoped by a changed-since ref.
fn ai_changed_since() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:ai:changed-since".to_owned(),
        label: "AI test plan proposing the tests changed since the base ref".to_owned(),
        origin_channel: SelectorChannel::Ai,
        intent: SelectionIntentKind::ChangedSince,
        expansion_policy: ExpansionPolicy::AllowWidenWithReview,
        snapshot_fingerprint: snapshot(
            "snapshot:test-tree:aggregate",
            "digest:aggregate:v3",
            "test_tree",
        ),
        query: SelectionQuery {
            include_query_tokens: refs(&["path:src/checkout"]),
            exclude_query_tokens: refs(&["tag:slow"]),
            changed_since_ref: Some("ref:base-main".to_owned()),
        },
        pinned_targets: vec![
            local_target("tree:case:login", DurableTestNodeKind::ConcreteCase),
            local_target(
                "tree:notebook:analysis",
                DurableTestNodeKind::NotebookLinkedTest,
            ),
        ],
        evidence_refs: refs(&["evidence:selection:ai:changed-since"]),
    }
}

/// Support-reconstructed snapshot-scoped selection over the test tree.
fn support_snapshot_scoped() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:support:snapshot-scoped".to_owned(),
        label: "Support reconstruction of a snapshot-scoped triage selection".to_owned(),
        origin_channel: SelectorChannel::Support,
        intent: SelectionIntentKind::SnapshotScoped,
        expansion_policy: ExpansionPolicy::ReresolveWithinSnapshot,
        snapshot_fingerprint: snapshot(
            "snapshot:test-tree:aggregate",
            "digest:aggregate:v3",
            "test_tree",
        ),
        query: SelectionQuery {
            include_query_tokens: refs(&["scope:test-tree:aggregate"]),
            exclude_query_tokens: Vec::new(),
            changed_since_ref: None,
        },
        pinned_targets: vec![local_target(
            "tree:case:login",
            DurableTestNodeKind::ConcreteCase,
        )],
        evidence_refs: refs(&["evidence:selection:support:snapshot-scoped"]),
    }
}

/// Imported CI overlay selection that must never re-dispatch as a local rerun.
fn imported_overlay() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:support:imported-ci".to_owned(),
        label: "Imported CI overlay selection retained read-only for triage".to_owned(),
        origin_channel: SelectorChannel::Support,
        intent: SelectionIntentKind::ExplicitItems,
        expansion_policy: ExpansionPolicy::FrozenImportedReadOnly,
        snapshot_fingerprint: snapshot(
            "snapshot:imported-ci:smoke",
            "digest:imported:v1",
            "imported_ci",
        ),
        query: SelectionQuery::default(),
        pinned_targets: vec![target(
            "ci:case:smoke",
            DurableTestNodeKind::ConcreteCase,
            TestItemIdentityClass::ImportedReadOnly,
        )],
        evidence_refs: refs(&["evidence:selection:support:imported-ci"]),
    }
}

fn guardrails() -> SelectionGuardrails {
    SelectionGuardrails {
        templates_distinct_from_invocations: true,
        imported_never_rerun_as_local: true,
        widening_opens_review: true,
        snapshot_drift_opens_review: true,
        origin_preserved_or_reviewed: true,
        selection_reconstructable_from_export: true,
    }
}

fn channel_projection() -> SelectionChannelProjection {
    SelectionChannelProjection {
        ui_consumes_selection: true,
        cli_consumes_selection: true,
        ai_plan_consumes_selection: true,
        support_export_reconstructs_selection: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        PORTABLE_SELECTION_SCHEMA_REF,
        PORTABLE_SELECTION_DOC_REF,
        PORTABLE_SELECTION_ARTIFACT_REF,
        "schemas/testing/durable-test-items-and-partial-discovery.schema.json",
    ])
}

fn packet() -> PortableSelectionPacket {
    let ui = ui_rerun_all();
    let cli = cli_rerun_failed();
    let ai = ai_changed_since();
    let support = support_snapshot_scoped();
    let imported = imported_overlay();

    // Compatible: re-resolved against the identical snapshot with the same
    // targets, so the originating selection is preserved without review.
    let compatible = cli.assess_against(
        "assessment:cli:compatible".to_owned(),
        cli.snapshot_fingerprint.clone(),
        &cli.pinned_targets,
        refs(&["evidence:assessment:cli:compatible"]),
    );

    // Widened: re-resolving the AI plan surfaces a newly added invocation, so a
    // widened-selection review opens before any rerun can dispatch.
    let widened = ai.assess_against(
        "assessment:ai:widened".to_owned(),
        ai.snapshot_fingerprint.clone(),
        &[
            local_target("tree:case:login", DurableTestNodeKind::ConcreteCase),
            local_target(
                "tree:notebook:analysis",
                DurableTestNodeKind::NotebookLinkedTest,
            ),
            local_target("tree:case:logout", DurableTestNodeKind::ConcreteCase),
        ],
        refs(&["evidence:assessment:ai:widened"]),
    );

    // Snapshot drift: the UI selection is re-resolved against a newer snapshot
    // digest, forcing review even though the target ids would still match.
    let drifted = ui.assess_against(
        "assessment:ui:drifted".to_owned(),
        snapshot(
            "snapshot:framework-pack:checkout",
            "digest:checkout:v2",
            "framework_pack",
        ),
        &ui.pinned_targets,
        refs(&["evidence:assessment:ui:drifted"]),
    );

    // Imported overlays are blocked from re-dispatching as a local rerun.
    let blocked = imported.assess_against(
        "assessment:imported:blocked".to_owned(),
        imported.snapshot_fingerprint.clone(),
        &imported.pinned_targets,
        refs(&["evidence:assessment:imported:blocked"]),
    );

    PortableSelectionPacket::new(PortableSelectionPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Scope-Compatible Selection Objects".to_owned(),
        selections: vec![ui, cli, ai, support, imported],
        assessments: vec![compatible, widened, drifted, blocked],
        guardrails: guardrails(),
        channel_projection: channel_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
