use std::fs;
use std::path::PathBuf;

use aureline_editor::{
    open_document, ClassificationPolicy, DocumentOpenDisposition, DocumentOpenOutcome,
    LargeFileViewerConfig,
};
use aureline_vfs::{LocalFilesystemRoot, VfsUri};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Meta {
    name: String,
    scenario: String,
}

#[derive(Debug, Deserialize)]
struct DocumentFixture {
    /// Path relative to the repo root.
    path: String,
}

#[derive(Debug, Default, Deserialize)]
struct PolicyOverrides {
    #[serde(default)]
    large_file_size_threshold: Option<u64>,
    #[serde(default)]
    soft_rss_budget: Option<u64>,
    #[serde(default)]
    sniff_bytes: Option<u64>,
    #[serde(default)]
    null_byte_marks_binary: Option<bool>,
    #[serde(default)]
    non_printable_per_mille_marks_binary: Option<u16>,
    #[serde(default)]
    minified_line_length: Option<u64>,
    #[serde(default)]
    large_file_pack_suffixes: Option<Vec<String>>,
}

impl PolicyOverrides {
    fn apply(&self, mut policy: ClassificationPolicy) -> ClassificationPolicy {
        if let Some(v) = self.large_file_size_threshold {
            policy.large_file_size_threshold = v;
        }
        if let Some(v) = self.soft_rss_budget {
            policy.soft_rss_budget = v;
        }
        if let Some(v) = self.sniff_bytes {
            policy.sniff_bytes = v;
        }
        if let Some(v) = self.null_byte_marks_binary {
            policy.null_byte_marks_binary = v;
        }
        if let Some(v) = self.non_printable_per_mille_marks_binary {
            policy.non_printable_per_mille_marks_binary = v;
        }
        if let Some(v) = self.minified_line_length {
            policy.minified_line_length = v;
        }
        if let Some(v) = self.large_file_pack_suffixes.clone() {
            policy.large_file_pack_suffixes = v;
        }
        policy
    }
}

#[derive(Debug, Default, Deserialize)]
struct ViewerOverrides {
    #[serde(default)]
    page_size: Option<usize>,
    #[serde(default)]
    max_resident_pages: Option<usize>,
}

impl ViewerOverrides {
    fn apply(&self, mut cfg: LargeFileViewerConfig) -> LargeFileViewerConfig {
        if let Some(v) = self.page_size {
            cfg.page_size = v;
        }
        if let Some(v) = self.max_resident_pages {
            cfg.max_resident_pages = v;
        }
        cfg
    }
}

#[derive(Debug, Deserialize)]
struct FindFirstExpected {
    needle: String,
    expect_found: bool,
}

#[derive(Debug, Deserialize)]
struct OverrideExpected {
    expected_trigger: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    mode: String,
    #[serde(default)]
    trigger: Option<String>,
    #[serde(default)]
    find_first: Option<FindFirstExpected>,
    #[serde(default)]
    override_anyway: Option<OverrideExpected>,
}

#[derive(Debug, Deserialize)]
struct LargeFileCaseFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    #[serde(default)]
    policy_overrides: PolicyOverrides,
    #[serde(default)]
    viewer_overrides: ViewerOverrides,
    expected: ExpectedFixture,
}

#[test]
fn large_file_cases_fixture_set_stays_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/large_file_cases");

    let mut fixture_paths: Vec<PathBuf> = fs::read_dir(&fixtures_dir)
        .expect("fixture directory must exist")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixture_paths.sort();

    assert!(
        !fixture_paths.is_empty(),
        "expected at least one fixture under {fixtures_dir:?}"
    );

    let root = LocalFilesystemRoot::new("ws-test", "repo-root", repo_root.clone())
        .expect("local filesystem root must construct");

    for path in fixture_paths {
        let raw = fs::read_to_string(&path).expect("fixture should be readable");
        let fixture: LargeFileCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let file_path = repo_root.join(&fixture.document.path);
        let uri = VfsUri::file_url_for_path(&file_path)
            .unwrap_or_else(|| panic!("fixture path must map to file uri: {file_path:?}"));

        let policy = fixture
            .policy_overrides
            .apply(ClassificationPolicy::default());
        let viewer = fixture
            .viewer_overrides
            .apply(LargeFileViewerConfig::default());

        let outcome = open_document(&root, &uri, &policy, viewer, DocumentOpenDisposition::Auto)
            .unwrap_or_else(|err| {
                panic!(
                    "open failed for {:?} ({}): {err}",
                    path, fixture.meta.scenario
                )
            });

        match (fixture.expected.mode.as_str(), outcome) {
            ("large_file", DocumentOpenOutcome::LargeFile(mut doc)) => {
                let trigger = doc.viewer.decision().trigger.map(|t| t.as_str().to_owned());
                assert_eq!(
                    trigger, fixture.expected.trigger,
                    "trigger mismatch for {:?} ({})",
                    path, fixture.meta.scenario
                );

                let notice = doc.notice();
                assert_eq!(
                    notice.title, "Large-file mode",
                    "notice title mismatch for {:?}",
                    path
                );
                assert_eq!(
                    notice.trigger, fixture.expected.trigger,
                    "notice trigger mismatch for {:?}",
                    path
                );
                assert!(
                    !notice.reason.trim().is_empty(),
                    "expected notice reason to be non-empty for {:?}",
                    path
                );
                assert!(
                    notice.reduced_capabilities.len() >= 3,
                    "expected reduced capability list for {:?}",
                    path
                );
                assert_eq!(
                    notice.escalation_label, "Open anyway",
                    "escalation label mismatch for {:?}",
                    path
                );

                if let Some(find) = fixture.expected.find_first {
                    let found = doc
                        .viewer
                        .find_first(&find.needle)
                        .expect("find_first should not error")
                        .is_some();
                    assert_eq!(
                        found, find.expect_found,
                        "find_first mismatch for {:?} ({})",
                        path, fixture.meta.scenario
                    );
                }

                if let Some(override_expected) = fixture.expected.override_anyway {
                    let normal = doc
                        .open_anyway(&root)
                        .unwrap_or_else(|err| panic!("override failed for {:?}: {err}", path));
                    let override_info = normal
                        .large_file_override
                        .expect("expected override record to be present");
                    let trigger = override_info
                        .decision
                        .trigger
                        .expect("override decision should be large-file");
                    assert_eq!(
                        trigger.as_str(),
                        override_expected.expected_trigger,
                        "override trigger mismatch for {:?} ({})",
                        path,
                        fixture.meta.scenario
                    );
                }
            }
            ("normal", DocumentOpenOutcome::Normal(doc)) => {
                assert!(
                    fixture.expected.trigger.is_none(),
                    "normal fixtures must not define trigger: {:?}",
                    path
                );
                assert!(
                    doc.large_file_override.is_none(),
                    "normal open should not carry large-file override record: {:?}",
                    path
                );
            }
            ("large_file", other) => {
                panic!(
                    "expected large-file open outcome for {:?} ({}), got {:?}",
                    path,
                    fixture.meta.scenario,
                    other_kind(&other)
                );
            }
            ("normal", other) => {
                panic!(
                    "expected normal open outcome for {:?} ({}), got {:?}",
                    path,
                    fixture.meta.scenario,
                    other_kind(&other)
                );
            }
            (mode, _) => panic!("unknown expected mode {mode:?} for {:?}", path),
        }

        let _ = fixture.meta.name;
    }
}

fn other_kind(outcome: &DocumentOpenOutcome) -> &'static str {
    match outcome {
        DocumentOpenOutcome::Normal(_) => "normal",
        DocumentOpenOutcome::LargeFile(_) => "large_file",
    }
}
