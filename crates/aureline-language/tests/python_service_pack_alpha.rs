use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_content_safety::TrustClass;
use aureline_language::{
    default_launch_grammar_registry, PythonServiceLanguagePack,
    PythonServiceLanguagePackEnablementRequest, PythonServiceLanguagePackEnablementStateClass,
    PythonServiceLanguagePackManifest, PYTHON_SERVICE_LANGUAGE_PACK_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    manifest_ref: String,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    case_id: String,
    request: PythonServiceLanguagePackEnablementRequest,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    enablement_state_class: PythonServiceLanguagePackEnablementStateClass,
    enabled_language_count: usize,
    activation_glob_count: usize,
    grammar_entry_count: usize,
    missing_grammar_count: usize,
    provider_route_count: usize,
    diagnostics_profile_count: usize,
    tool_hook_count: usize,
    icon_count: usize,
    docs_pack_count: usize,
    known_gap_count: usize,
    git_surface_count: usize,
    launch_bundle_count: usize,
    archetype_report_count: usize,
    default_trust_class: TrustClass,
    can_enable_without_per_file_assembly: bool,
    markdown_and_git_surfaces_integrated: bool,
    participates_in_launch_bundle_reporting: bool,
    scope_is_bounded_alpha: bool,
    fallback_label_required: bool,
    required_language_ids: Vec<String>,
    required_provider_refs: Vec<String>,
    required_tool_hook_refs: Vec<String>,
    required_known_gap_refs: Vec<String>,
    required_git_surface_refs: Vec<String>,
    required_launch_bundle_refs: Vec<String>,
    required_archetype_report_refs: Vec<String>,
    required_suspicious_content_classes: Vec<String>,
}

#[test]
fn python_service_language_pack_enables_python_markdown_git_and_reporting() {
    let repo_root = repo_root();
    let fixture = load_fixture(&repo_root);
    assert_eq!(
        fixture.record_kind,
        "python_service_language_pack_alpha_cases"
    );
    assert_eq!(
        fixture.schema_version,
        PYTHON_SERVICE_LANGUAGE_PACK_SCHEMA_VERSION
    );

    let manifest = load_manifest(&repo_root, &fixture.manifest_ref);
    assert_eq!(
        manifest.record_kind,
        PythonServiceLanguagePackManifest::RECORD_KIND
    );
    assert_eq!(
        manifest.schema_version,
        PYTHON_SERVICE_LANGUAGE_PACK_SCHEMA_VERSION
    );
    assert_manifest_refs_resolve(&repo_root, &manifest);
    assert_manifest_is_one_pack_contract(&manifest);
    assert_launch_bundle_consumes_pack(&repo_root, &manifest, &fixture.manifest_ref);

    let pack = PythonServiceLanguagePack::new(manifest);
    let registry = default_launch_grammar_registry();

    for case in fixture.cases {
        let snapshot = pack.enablement_snapshot(&registry, case.request);
        assert_eq!(
            snapshot.enablement_state_class, case.expected.enablement_state_class,
            "enablement state mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.enabled_language_ids.len(),
            case.expected.enabled_language_count,
            "language count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.activation_globs.len(),
            case.expected.activation_glob_count,
            "activation glob count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.grammar_entry_refs.len(),
            case.expected.grammar_entry_count,
            "grammar count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.missing_grammar_language_ids.len(),
            case.expected.missing_grammar_count,
            "missing grammar count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.provider_route_refs.len(),
            case.expected.provider_route_count,
            "provider count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.diagnostics_profile_refs.len(),
            case.expected.diagnostics_profile_count,
            "diagnostics profile count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.tool_hook_refs.len(),
            case.expected.tool_hook_count,
            "tool hook count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.icon_refs.len(),
            case.expected.icon_count,
            "icon count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.docs_pack_refs.len(),
            case.expected.docs_pack_count,
            "docs pack count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.known_gap_refs.len(),
            case.expected.known_gap_count,
            "known gap count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.git_surface_refs.len(),
            case.expected.git_surface_count,
            "Git surface count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.launch_bundle_refs.len(),
            case.expected.launch_bundle_count,
            "launch bundle count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.archetype_report_refs.len(),
            case.expected.archetype_report_count,
            "archetype report count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.default_trust_class, case.expected.default_trust_class,
            "trust class mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.can_enable_without_per_file_assembly,
            case.expected.can_enable_without_per_file_assembly,
            "manual assembly mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.markdown_and_git_surfaces_integrated,
            case.expected.markdown_and_git_surfaces_integrated,
            "Markdown/Git integration mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.participates_in_launch_bundle_reporting,
            case.expected.participates_in_launch_bundle_reporting,
            "launch reporting mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.scope_is_bounded_alpha, case.expected.scope_is_bounded_alpha,
            "bounded alpha mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.fallback_label_required, case.expected.fallback_label_required,
            "fallback label mismatch for {}",
            case.case_id
        );

        assert_contains_all(
            &snapshot.enabled_language_ids,
            &case.expected.required_language_ids,
            "language ids",
            &case.case_id,
        );
        assert_contains_all(
            &snapshot.provider_route_refs,
            &case.expected.required_provider_refs,
            "provider refs",
            &case.case_id,
        );
        assert_contains_all(
            &snapshot.tool_hook_refs,
            &case.expected.required_tool_hook_refs,
            "tool hook refs",
            &case.case_id,
        );
        assert_contains_all(
            &snapshot.known_gap_refs,
            &case.expected.required_known_gap_refs,
            "known gap refs",
            &case.case_id,
        );
        assert_contains_all(
            &snapshot.git_surface_refs,
            &case.expected.required_git_surface_refs,
            "Git surface refs",
            &case.case_id,
        );
        assert_contains_all(
            &snapshot.launch_bundle_refs,
            &case.expected.required_launch_bundle_refs,
            "launch bundle refs",
            &case.case_id,
        );
        assert_contains_all(
            &snapshot.archetype_report_refs,
            &case.expected.required_archetype_report_refs,
            "archetype report refs",
            &case.case_id,
        );
        let suspicious_tokens = snapshot
            .suspicious_content_classes
            .iter()
            .map(|class| class.as_str().to_owned())
            .collect::<Vec<_>>();
        assert_contains_all(
            &suspicious_tokens,
            &case.expected.required_suspicious_content_classes,
            "suspicious content classes",
            &case.case_id,
        );
        assert_snapshot_round_trips(&snapshot);
    }
}

fn assert_manifest_is_one_pack_contract(manifest: &PythonServiceLanguagePackManifest) {
    let language_ids = manifest
        .language_rows
        .iter()
        .map(|row| row.language_id.as_str())
        .collect::<BTreeSet<_>>();
    let provider_refs = manifest
        .provider_routes
        .iter()
        .map(|route| route.provider_ref.as_str())
        .collect::<BTreeSet<_>>();
    let tool_hook_refs = manifest
        .tool_hooks
        .iter()
        .map(|hook| hook.tool_hook_ref.as_str())
        .collect::<BTreeSet<_>>();
    let icon_refs = manifest
        .icon_rows
        .iter()
        .map(|icon| icon.icon_ref.as_str())
        .collect::<BTreeSet<_>>();
    let docs_pack_refs = manifest
        .docs_pack_refs
        .iter()
        .map(|doc| doc.pack_ref.as_str())
        .collect::<BTreeSet<_>>();
    let known_gap_refs = manifest
        .known_gap_rows
        .iter()
        .map(|gap| gap.gap_ref.as_str())
        .collect::<BTreeSet<_>>();
    let git_surface_refs = manifest
        .git_surface_rows
        .iter()
        .map(|surface| surface.git_surface_ref.as_str())
        .collect::<BTreeSet<_>>();

    for row in &manifest.language_rows {
        assert!(
            provider_refs.contains(row.default_provider_ref.as_str()),
            "language {} references missing provider {}",
            row.language_id,
            row.default_provider_ref
        );
        if let Some(formatter_hook_ref) = row.formatter_hook_ref.as_deref() {
            assert!(
                tool_hook_refs.contains(formatter_hook_ref),
                "language {} references missing formatter hook {}",
                row.language_id,
                formatter_hook_ref
            );
        }
        assert!(
            icon_refs.contains(row.icon_ref.as_str()),
            "language {} references missing icon {}",
            row.language_id,
            row.icon_ref
        );
        for doc_ref in &row.docs_pack_refs {
            assert!(
                docs_pack_refs.contains(doc_ref.as_str()),
                "language {} references missing docs pack {}",
                row.language_id,
                doc_ref
            );
        }
        for gap_ref in &row.known_gap_refs {
            assert!(
                known_gap_refs.contains(gap_ref.as_str()),
                "language {} references missing known gap {}",
                row.language_id,
                gap_ref
            );
        }
    }

    for route in &manifest.provider_routes {
        for language_id in &route.language_ids {
            assert!(
                language_ids.contains(language_id.as_str()),
                "provider {} references missing language {}",
                route.provider_ref,
                language_id
            );
        }
        assert!(
            !route.capability_classes.is_empty(),
            "provider {} must name capabilities",
            route.provider_ref
        );
        assert!(
            !route.surface_classes.is_empty(),
            "provider {} must name consuming surfaces",
            route.provider_ref
        );
    }

    for hook in &manifest.tool_hooks {
        assert!(
            provider_refs.contains(hook.provider_ref.as_str()),
            "hook {} references missing provider {}",
            hook.tool_hook_ref,
            hook.provider_ref
        );
        for language_id in &hook.language_ids {
            assert!(
                language_ids.contains(language_id.as_str()),
                "hook {} references missing language {}",
                hook.tool_hook_ref,
                language_id
            );
        }
    }

    for surface in &manifest.git_surface_rows {
        assert!(
            surface.local_git_truth_required && surface.markdown_representation_required,
            "Git surface {} must preserve local Git and Markdown representation truth",
            surface.git_surface_ref
        );
        for language_id in &surface.linked_language_ids {
            assert!(
                language_ids.contains(language_id.as_str()),
                "Git surface {} references missing language {}",
                surface.git_surface_ref,
                language_id
            );
        }
        for provider_ref in &surface.required_provider_refs {
            assert!(
                provider_refs.contains(provider_ref.as_str()),
                "Git surface {} references missing provider {}",
                surface.git_surface_ref,
                provider_ref
            );
        }
        for doc_ref in &surface.required_docs_pack_refs {
            assert!(
                docs_pack_refs.contains(doc_ref.as_str()),
                "Git surface {} references missing docs pack {}",
                surface.git_surface_ref,
                doc_ref
            );
        }
    }

    for flow in &manifest.enablement_flows {
        assert!(
            !flow.manual_per_file_assembly_required,
            "flow {} must enable from the pack",
            flow.flow_ref
        );
        for provider_ref in &flow.required_provider_refs {
            assert!(
                provider_refs.contains(provider_ref.as_str()),
                "flow {} references missing provider {}",
                flow.flow_ref,
                provider_ref
            );
        }
        for hook_ref in &flow.required_tool_hook_refs {
            assert!(
                tool_hook_refs.contains(hook_ref.as_str()),
                "flow {} references missing hook {}",
                flow.flow_ref,
                hook_ref
            );
        }
        for doc_ref in &flow.required_docs_pack_refs {
            assert!(
                docs_pack_refs.contains(doc_ref.as_str()),
                "flow {} references missing docs pack {}",
                flow.flow_ref,
                doc_ref
            );
        }
        for surface_ref in &flow.required_git_surface_refs {
            assert!(
                git_surface_refs.contains(surface_ref.as_str()),
                "flow {} references missing Git surface {}",
                flow.flow_ref,
                surface_ref
            );
        }
    }
}

fn assert_manifest_refs_resolve(repo_root: &Path, manifest: &PythonServiceLanguagePackManifest) {
    for source_ref in &manifest.source_refs {
        assert_ref_exists(repo_root, source_ref);
    }
    for doc in &manifest.docs_pack_refs {
        assert_ref_exists(repo_root, &doc.source_ref);
    }
    for gap in &manifest.known_gap_rows {
        assert_ref_exists(repo_root, &gap.docs_ref);
    }
    for report in &manifest.launch_bundle_report_refs {
        assert_ref_exists(repo_root, &report.bundle_manifest_ref);
        assert_ref_exists(repo_root, &report.proof_packet_ref);
    }
}

fn assert_launch_bundle_consumes_pack(
    repo_root: &Path,
    manifest: &PythonServiceLanguagePackManifest,
    manifest_ref: &str,
) {
    let path = repo_root.join("artifacts/bundles/python_launch_bundle_alpha.yaml");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let bundle: serde_yaml::Value =
        serde_yaml::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    let language_pack_refs = bundle
        .get("language_pack_refs")
        .and_then(serde_yaml::Value::as_sequence)
        .expect("python launch bundle must expose language_pack_refs");

    let found = language_pack_refs.iter().any(|raw_row| {
        let Some(row) = raw_row.as_mapping() else {
            return false;
        };
        yaml_str(row, "pack_ref") == Some(manifest.pack_id.as_str())
            && yaml_str(row, "manifest_ref") == Some(manifest_ref)
            && yaml_str(row, "runtime_consumer_ref")
                == Some("crates/aureline-language/src/packs/python_service.rs")
    });
    assert!(
        found,
        "python launch bundle must reference the canonical Python service language pack"
    );
}

fn yaml_str<'a>(row: &'a serde_yaml::Mapping, key: &str) -> Option<&'a str> {
    row.get(serde_yaml::Value::String(key.to_owned()))
        .and_then(serde_yaml::Value::as_str)
}

fn assert_ref_exists(repo_root: &Path, artifact_ref: &str) {
    let clean_ref = artifact_ref.split('#').next().unwrap_or(artifact_ref);
    assert!(
        repo_root.join(clean_ref).exists(),
        "artifact ref does not resolve: {artifact_ref}"
    );
}

fn assert_contains_all(actual: &[String], expected: &[String], label: &str, case_id: &str) {
    let actual_set = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
    for value in expected {
        assert!(
            actual_set.contains(value.as_str()),
            "{label} missing {value} for {case_id}"
        );
    }
}

fn assert_snapshot_round_trips(
    snapshot: &aureline_language::PythonServiceLanguagePackEnablementSnapshot,
) {
    let serialized = serde_json::to_string(snapshot).expect("pack snapshot serializes");
    let round_trip: aureline_language::PythonServiceLanguagePackEnablementSnapshot =
        serde_json::from_str(&serialized).expect("pack snapshot deserializes");
    assert_eq!(round_trip, *snapshot);
}

fn load_fixture(repo_root: &Path) -> Fixture {
    let path =
        repo_root.join("fixtures/language/packs/python_service_alpha/pack_enablement_cases.yaml");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_manifest(repo_root: &Path, manifest_ref: &str) -> PythonServiceLanguagePackManifest {
    let path = repo_root.join(manifest_ref);
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
