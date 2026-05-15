use std::path::{Path, PathBuf};

use aureline_docs::DocsPack;
use aureline_shell::docs_browser::{docs_browser_row_cards_from_pack, DocsBrowserContentContext};

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

#[test]
fn tsjs_launch_docs_pack_materializes_browser_rows() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("TS/JS launch docs pack fixture loads");
    let context = DocsBrowserContentContext::local_pack(&pack);
    let rows = docs_browser_row_cards_from_pack(&pack, &context);

    assert_eq!(rows.len(), 2);
    let setup = rows
        .iter()
        .find(|row| row.surface_id_ref == "docs-node:launch-bundle:typescript-web:start")
        .expect("setup docs row is materialized");

    assert_eq!(setup.source_row.class_token, "curated_knowledge_pack");
    assert_eq!(setup.source_row.label, "Curated knowledge pack");
    assert_eq!(
        setup.source_row.snapshot_age_label.as_deref(),
        Some("Pack captured 2026-05-12")
    );
    assert_eq!(setup.version_row.state_token, "exact_build_match");
    assert_eq!(setup.version_row.label, "Exact build match");
    assert_eq!(
        setup.version_row.running_build_identity_ref,
        "id:build:aureline:running"
    );
    assert_eq!(setup.freshness_row.class_token, "warm_cached");
    assert!(!setup.freshness_row.degraded);
    assert_eq!(
        setup.client_scope_row.identity_mode_token,
        "account_free_local"
    );
    assert_eq!(setup.client_scope_row.trust_state_token, "trusted");
    assert!(setup.browser_handoff_row.available);
    assert_eq!(
        setup
            .browser_handoff_row
            .browser_handoff_packet_ref
            .as_deref(),
        Some("id:browser-handoff:docs-pack:typescript-web-alpha")
    );
    assert_eq!(
        setup.browser_handoff_row.posture_class_token,
        "system_browser_first"
    );
    assert_eq!(
        setup.browser_handoff_row.fallback_target_class_token,
        "system_browser_handoff_packet"
    );

    let lines = setup.render_lines();
    for expected in [
        "Source: Curated knowledge pack",
        "Version: Exact build match",
        "Freshness: Warm cached",
        "Client scope: Local docs pack within the current workspace boundary.",
    ] {
        assert!(
            lines.iter().any(|line| line.contains(expected)),
            "expected rendered line containing {expected:?}: {lines:#?}"
        );
    }
}

#[test]
fn missing_handoff_packet_keeps_handoff_row_explicit_but_unavailable() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("TS/JS launch docs pack fixture loads");
    let mut context = DocsBrowserContentContext::local_pack(&pack);
    context.browser_handoff_packet_ref = None;

    let rows = docs_browser_row_cards_from_pack(&pack, &context);
    let row = rows.first().expect("at least one row is materialized");

    assert!(!row.browser_handoff_row.available);
    assert_eq!(row.browser_handoff_row.action_label, "Open in browser");
    assert!(row
        .render_lines()
        .iter()
        .any(|line| line.contains("packet: missing")));
}
