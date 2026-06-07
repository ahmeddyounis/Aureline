//! Headless emitter for the transfer-safety packet corpus.
//!
//! Writes every scenario in the claimed-stable matrix to disk under
//! `fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails/`.
//!
//! Usage:
//! ```sh
//! cargo run -p aureline-editor --bin aureline_transfer_safety
//! ```

use std::path::PathBuf;

use aureline_editor::{transfer_safety_corpus, TRANSFER_SAFETY_CORPUS_AS_OF};

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("repo root must resolve");
    let fixture_dir = repo_root
        .join("fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails");
    std::fs::create_dir_all(&fixture_dir).unwrap_or_else(|err| {
        eprintln!("failed to create fixture dir {fixture_dir:?}: {err}");
        std::process::exit(2);
    });

    let corpus = transfer_safety_corpus();
    let mut written = 0;
    for scenario in &corpus {
        let path = fixture_dir.join(scenario.fixture_filename);
        let payload = serde_json::to_string_pretty(&scenario.packet())
            .unwrap_or_else(|err| panic!("serialize {}: {err}", scenario.scenario_id));
        std::fs::write(&path, payload).unwrap_or_else(|err| {
            eprintln!("failed to write {path:?}: {err}");
            std::process::exit(2);
        });
        println!("wrote {} -> {}", scenario.scenario_id, path.display());
        written += 1;
    }

    let manifest = serde_json::json!({
        "record_kind": "transfer_safety_corpus_manifest",
        "schema_version": 1,
        "as_of": TRANSFER_SAFETY_CORPUS_AS_OF,
        "scenario_count": corpus.len(),
        "scenarios": corpus.iter().map(|s| {
            serde_json::json!({
                "scenario_id": s.scenario_id,
                "fixture_filename": s.fixture_filename,
                "expected_surface": s.expected_surface.as_str(),
                "expected_action": s.expected_action.as_str(),
                "expects_sensitive_review": s.expects_sensitive_review,
                "expects_drop_preview": s.expects_drop_preview,
                "expects_paste_guardrail": s.expects_paste_guardrail,
                "expects_large_transfer": s.expects_large_transfer,
                "expects_named_undo_group": s.expects_named_undo_group,
            })
        }).collect::<Vec<_>>(),
    });
    let manifest_path = fixture_dir.join("manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).expect("manifest must serialize"),
    )
    .unwrap_or_else(|err| {
        eprintln!("failed to write manifest {manifest_path:?}: {err}");
        std::process::exit(2);
    });
    println!("wrote manifest -> {}", manifest_path.display());
    written += 1;

    println!(
        "done: {written} files written under {}",
        fixture_dir.display()
    );
}
