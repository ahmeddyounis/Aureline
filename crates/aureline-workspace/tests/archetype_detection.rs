use aureline_workspace::{
    default_archetype_seed_catalog, detect_workspace_archetype,
    detect_workspace_archetype_with_catalog, load_archetype_seed_catalog,
    propose_workspace_archetype, ArchetypeDetectionOutcome, DetectionConfidenceClass,
    DetectionOutcome, LaunchArchetypeFamily, SignalFreshnessClass, SupportClaimClass,
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
fn rust_workspace_fixture_proposes_supported_scorecard_row() {
    let temp = TempWorkspace::new("rust_workspace");
    std::fs::create_dir_all(temp.path().join("src")).expect("src dir");
    std::fs::write(
        temp.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    )
    .expect("cargo manifest");
    std::fs::write(temp.path().join("Cargo.lock"), "# lock\n").expect("cargo lock");
    std::fs::write(temp.path().join("src/lib.rs"), "pub fn ready() {}\n").expect("rust source");

    let report = detect_workspace_archetype(temp.path()).expect("detect archetype");
    let proposal = report.proposal.as_ref().expect("rust proposal");
    let truth = report.to_archetype_truth();
    let catalog = default_archetype_seed_catalog().expect("seed catalog");
    let seed_row = catalog
        .row_for_family(LaunchArchetypeFamily::RustWorkspace)
        .expect("rust seed row");

    assert_eq!(report.outcome, ArchetypeDetectionOutcome::Proposed);
    assert_eq!(proposal.family, LaunchArchetypeFamily::RustWorkspace);
    assert_eq!(proposal.archetype_seed_row_id, seed_row.row_id);
    assert_eq!(truth.outcome, DetectionOutcome::ProbableArchetype);
    assert_eq!(
        truth.support_claim_class,
        SupportClaimClass::SupportedScoped
    );
    assert_eq!(
        truth.evidence_freshness[0].freshness_class,
        SignalFreshnessClass::CachedCurrentEnough
    );
    assert_eq!(
        truth.evidence_freshness[0].reviewed_on.as_deref(),
        Some("2026-05-15")
    );
}

#[test]
fn certified_catalog_row_projects_certified_truth() {
    let temp = TempWorkspace::new("certified_ts_web_app");
    std::fs::write(
        temp.path().join("package.json"),
        r#"{
          "dependencies": { "react": "latest", "vite": "latest" },
          "devDependencies": { "typescript": "latest", "vitest": "latest" }
        }"#,
    )
    .expect("package manifest");
    std::fs::write(temp.path().join("tsconfig.json"), "{}\n").expect("tsconfig");
    let catalog = load_archetype_seed_catalog(
        r#"
archetype_seed_rows:
  - row_id: archetype_detection:test_certified_ts
    archetype_row_ref: archetype_row:ts_web_app_or_service
    public_label: TypeScript certified web app
    representative_stack: TypeScript React Vite web app
    bundle_ref: launch_bundle:typescript_web_app.certified
    bundle_manifest_ref: artifacts/bundles/tsjs_launch_bundle_alpha.yaml
    current_support_class: certified
    certification_state: certified
    evidence_packet_ref: artifacts/compat/m3/archetype_scorecards/ts_web_app_or_service.md
    freshness:
      state: current
      reviewed_on: "2026-05-15"
      stale_after: P21D
"#,
    )
    .expect("catalog parses");

    let report =
        detect_workspace_archetype_with_catalog(temp.path(), &catalog).expect("detect archetype");
    let truth = report.to_archetype_truth();

    assert_eq!(truth.outcome, DetectionOutcome::CertifiedArchetypeMatch);
    assert_eq!(
        truth.confidence_class,
        DetectionConfidenceClass::CertifiedExact
    );
    assert_eq!(
        truth.support_claim_class,
        SupportClaimClass::CertifiedCurrent
    );
    assert_eq!(
        truth.evidence_freshness[0].freshness_class,
        SignalFreshnessClass::FreshCurrent
    );
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
