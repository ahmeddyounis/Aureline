//! Headless emitter for the modal-editing safety packet corpus.
//!
//! Writes every scenario in the claimed-stable matrix to disk under
//! `fixtures/editor/m4/stabilize-modal-editing-leader-register-safety/`.
//!
//! Usage:
//! ```sh
//! cargo run --bin aureline_modal_editing_safety
//! ```

use std::path::PathBuf;

use aureline_editor::{modal_editing_safety_corpus, MODAL_EDITING_SAFETY_CORPUS_AS_OF};

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("repo root must resolve");
    let fixture_dir =
        repo_root.join("fixtures/editor/m4/stabilize-modal-editing-leader-register-safety");
    std::fs::create_dir_all(&fixture_dir).unwrap_or_else(|err| {
        eprintln!("failed to create fixture dir {fixture_dir:?}: {err}");
        std::process::exit(2);
    });

    let corpus = modal_editing_safety_corpus();
    let mut written = 0;
    for scenario in &corpus {
        let path = fixture_dir.join(&scenario.fixture_filename);
        let payload = serde_json::to_string_pretty(&scenario.packet())
            .unwrap_or_else(|err| panic!("serialize {}: {err}", scenario.scenario_id));
        std::fs::write(&path, payload).unwrap_or_else(|err| {
            eprintln!("failed to write {path:?}: {err}");
            std::process::exit(2);
        });
        println!("wrote {} -> {}", scenario.scenario_id, path.display());
        written += 1;
    }

    // Write a manifest that lists every scenario.
    let manifest = serde_json::json!({
        "record_kind": "modal_editing_safety_corpus_manifest",
        "schema_version": 1,
        "as_of": MODAL_EDITING_SAFETY_CORPUS_AS_OF,
        "scenario_count": corpus.len(),
        "scenarios": corpus.iter().map(|s| {
            serde_json::json!({
                "scenario_id": s.scenario_id,
                "fixture_filename": s.fixture_filename,
                "expected_mode": s.expected_mode.as_str(),
                "expected_downgrade_count": s.expected_downgrade_count,
                "expected_regression_count": s.expected_regression_count,
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
