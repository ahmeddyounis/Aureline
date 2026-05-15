use aureline_workspace::{
    default_archetype_seed_catalog, detect_workspace_archetype, propose_workspace_archetype,
    ArchetypeDetectionOutcome, LaunchArchetypeFamily,
};

#[test]
fn ts_web_app_fixture_proposes_seed_row_with_high_confidence() {
    let temp = TempWorkspace::new("ts_web_app");
    std::fs::create_dir_all(temp.path().join("src")).expect("src dir");
    std::fs::write(
        temp.path().join("package.json"),
        r#"{
          "scripts": { "dev": "vite", "build": "vite build", "test": "vitest" },
          "dependencies": { "react": "latest", "vite": "latest" },
          "devDependencies": { "typescript": "latest", "vitest": "latest" }
        }"#,
    )
    .expect("package manifest");
    std::fs::write(temp.path().join("tsconfig.json"), "{}\n").expect("tsconfig");
    std::fs::write(temp.path().join("vite.config.ts"), "export default {}\n").expect("vite config");
    std::fs::write(
        temp.path().join("src/App.tsx"),
        "export const App = () => null;\n",
    )
    .expect("tsx source");

    let report = detect_workspace_archetype(temp.path()).expect("detect archetype");
    let proposal = report.proposal.expect("ts proposal");
    let catalog = default_archetype_seed_catalog().expect("seed catalog");
    let seed_row = catalog
        .row_for_family(LaunchArchetypeFamily::TypeScriptJavaScriptWeb)
        .expect("ts seed row");

    assert_eq!(report.outcome, ArchetypeDetectionOutcome::Proposed);
    assert_eq!(proposal.archetype_seed_row_id, seed_row.row_id);
    assert_eq!(proposal.archetype_row_ref, seed_row.archetype_row_ref);
    assert!(proposal.confidence_score >= 85);
    assert!(proposal.public_label.contains("TypeScript"));
}

#[test]
fn python_data_app_fixture_proposes_python_seed_row() {
    let temp = TempWorkspace::new("python_data_app");
    std::fs::create_dir_all(temp.path().join("src")).expect("src dir");
    std::fs::write(
        temp.path().join("pyproject.toml"),
        r#"[project]
name = "analysis-app"
dependencies = ["pandas", "pytest", "fastapi"]

[tool.pytest.ini_options]
testpaths = ["tests"]
"#,
    )
    .expect("pyproject");
    std::fs::write(temp.path().join("uv.lock"), "# lock\n").expect("uv lock");
    std::fs::write(temp.path().join("src/main.py"), "print('ready')\n").expect("python source");

    let report = detect_workspace_archetype(temp.path()).expect("detect archetype");
    let proposal = report.proposal.expect("python proposal");
    let catalog = default_archetype_seed_catalog().expect("seed catalog");
    let seed_row = catalog
        .row_for_family(LaunchArchetypeFamily::PythonServiceOrDataApp)
        .expect("python seed row");

    assert_eq!(report.outcome, ArchetypeDetectionOutcome::Proposed);
    assert_eq!(proposal.archetype_seed_row_id, seed_row.row_id);
    assert_eq!(proposal.archetype_row_ref, seed_row.archetype_row_ref);
    assert!(proposal.confidence_score >= 80);
    assert!(proposal.public_label.contains("Python"));
}

#[test]
fn ambiguous_repo_returns_no_archetype() {
    let temp = TempWorkspace::new("ambiguous");
    std::fs::write(temp.path().join("README.md"), "notes only\n").expect("readme");

    let proposal = propose_workspace_archetype(temp.path()).expect("detect archetype");
    let report = detect_workspace_archetype(temp.path()).expect("detect archetype report");

    assert!(proposal.is_none());
    assert_eq!(
        report.outcome,
        ArchetypeDetectionOutcome::NoRecognizedArchetype
    );
}

#[test]
fn conflicting_markers_return_no_single_archetype() {
    let temp = TempWorkspace::new("conflicting");
    std::fs::create_dir_all(temp.path().join("src")).expect("src dir");
    std::fs::write(
        temp.path().join("package.json"),
        r#"{
          "scripts": { "dev": "vite", "test": "vitest" },
          "dependencies": { "react": "latest", "vite": "latest" },
          "devDependencies": { "typescript": "latest" }
        }"#,
    )
    .expect("package manifest");
    std::fs::write(temp.path().join("tsconfig.json"), "{}\n").expect("tsconfig");
    std::fs::write(
        temp.path().join("pyproject.toml"),
        r#"[project]
dependencies = ["pandas", "pytest", "fastapi"]
"#,
    )
    .expect("pyproject");
    std::fs::write(temp.path().join("src/main.py"), "print('ready')\n").expect("python source");

    let report = detect_workspace_archetype(temp.path()).expect("detect archetype");

    assert!(report.proposal.is_none());
    assert_eq!(
        report.outcome,
        ArchetypeDetectionOutcome::ConflictingMarkers
    );
    assert!(report.competing_archetype_refs.len() >= 2);
}

struct TempWorkspace {
    path: std::path::PathBuf,
}

impl TempWorkspace {
    fn new(label: &str) -> Self {
        let unique = format!(
            "aureline-archetype-detection-{label}-{}-{:x}",
            std::process::id(),
            now_nanos()
        );
        let path = std::env::temp_dir().join(unique);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("temp workspace");
        Self { path }
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn now_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_nanos()
}
