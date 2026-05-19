use std::collections::HashSet;
use std::path::{Path, PathBuf};

use aureline_framework::{
    CertaintyLabelClass, ConventionCertaintyClass, ConventionDiagnosticClass, FrameworkObjectCertainty,
    FrameworkObjectKind, FrameworkSupportStrip, GeneratorKindClass, HealthClass, PackSourceClass,
    RollbackClass, SupportClass, FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND,
    FRAMEWORK_SUPPORT_STRIP_RECORD_KIND,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/framework/m3/status_strip_and_convention_truth")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_support_classes: Vec<String>,
    expected_certainty_label_classes: Vec<String>,
    expected_framework_object_kinds: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StripExpectations {
    record_kind: String,
    support_class: SupportClass,
    pack_source_class: PackSourceClass,
    health_class: HealthClass,
    findings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StripFixtureMeta {
    name: String,
    #[serde(default)]
    scenario: Option<String>,
    expected: StripExpectations,
}

#[derive(Debug, Deserialize)]
struct StripFixture {
    #[serde(rename = "__fixture__")]
    fixture_meta: StripFixtureMeta,
    #[serde(flatten)]
    strip: FrameworkSupportStrip,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CertaintyExpectations {
    record_kind: String,
    framework_object_kind: FrameworkObjectKind,
    #[serde(default)]
    certainty_label_class: Option<CertaintyLabelClass>,
    #[serde(default)]
    support_class: Option<SupportClass>,
    #[serde(default)]
    pack_source_class: Option<PackSourceClass>,
    #[serde(default)]
    convention_diagnostic_class: Option<ConventionDiagnosticClass>,
    #[serde(default)]
    convention_certainty_class: Option<ConventionCertaintyClass>,
    #[serde(default)]
    generator_kind_class: Option<GeneratorKindClass>,
    #[serde(default)]
    rollback_class: Option<RollbackClass>,
    findings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CertaintyFixtureMeta {
    name: String,
    #[serde(default)]
    scenario: Option<String>,
    expected: CertaintyExpectations,
}

#[derive(Debug, Deserialize)]
struct CertaintyFixture {
    #[serde(rename = "__fixture__")]
    fixture_meta: CertaintyFixtureMeta,
    #[serde(flatten)]
    record: FrameworkObjectCertainty,
}

fn read_fixture(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path:?}: {err}"))
}

#[test]
fn status_strip_and_convention_truth_fixtures_validate_and_match_manifest() {
    let manifest_path = fixture_root().join("manifest.yaml");
    let manifest_payload = read_fixture(&manifest_path);
    let manifest: Manifest =
        serde_yaml::from_str(&manifest_payload).expect("manifest must parse");

    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.case_refs.is_empty());

    let expected_support: HashSet<String> =
        manifest.expected_support_classes.iter().cloned().collect();
    let expected_certainty: HashSet<String> = manifest
        .expected_certainty_label_classes
        .iter()
        .cloned()
        .collect();
    let expected_kinds: HashSet<String> = manifest
        .expected_framework_object_kinds
        .iter()
        .cloned()
        .collect();

    let mut observed_support: HashSet<String> = HashSet::new();
    let mut observed_certainty: HashSet<String> = HashSet::new();
    let mut observed_kinds: HashSet<String> = HashSet::new();

    for case_rel in &manifest.case_refs {
        let case_path = repo_root().join(case_rel);
        let payload = read_fixture(&case_path);

        if case_rel.contains("/strip_") {
            let case: StripFixture = serde_yaml::from_str(&payload)
                .unwrap_or_else(|err| panic!("parse {case_path:?}: {err}"));
            let scope = format!("{case_rel} ({})", case.fixture_meta.name);

            assert_eq!(
                case.strip.record_kind, FRAMEWORK_SUPPORT_STRIP_RECORD_KIND,
                "{scope}: record kind"
            );
            assert_eq!(
                case.fixture_meta.expected.record_kind, FRAMEWORK_SUPPORT_STRIP_RECORD_KIND,
                "{scope}: fixture meta record kind"
            );

            let findings = case.strip.validate();
            assert!(
                findings.is_empty(),
                "{scope}: strip must validate clean, found {findings:?}"
            );
            assert_eq!(
                case.fixture_meta.expected.findings,
                Vec::<String>::new(),
                "{scope}: fixture must declare zero expected findings"
            );

            assert_eq!(
                case.strip.support_class, case.fixture_meta.expected.support_class,
                "{scope}: support class"
            );
            assert_eq!(
                case.strip.pack_or_bridge_source_block.pack_source_class,
                case.fixture_meta.expected.pack_source_class,
                "{scope}: pack source class"
            );
            assert_eq!(
                case.strip.health_block.health_class, case.fixture_meta.expected.health_class,
                "{scope}: health class"
            );

            observed_support.insert(case.strip.support_class.as_str().to_owned());
        } else {
            let case: CertaintyFixture = serde_yaml::from_str(&payload)
                .unwrap_or_else(|err| panic!("parse {case_path:?}: {err}"));
            let scope = format!("{case_rel} ({})", case.fixture_meta.name);

            assert_eq!(
                case.record.record_kind, FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND,
                "{scope}: record kind"
            );
            assert_eq!(
                case.fixture_meta.expected.record_kind, FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND,
                "{scope}: fixture meta record kind"
            );

            let findings = case.record.validate();
            assert!(
                findings.is_empty(),
                "{scope}: record must validate clean, found {findings:?}"
            );
            assert_eq!(
                case.fixture_meta.expected.findings,
                Vec::<String>::new(),
                "{scope}: fixture must declare zero expected findings"
            );

            assert_eq!(
                case.record.framework_object_kind,
                case.fixture_meta.expected.framework_object_kind,
                "{scope}: framework object kind"
            );

            if let Some(expected) = case.fixture_meta.expected.support_class {
                assert_eq!(case.record.support_class, expected, "{scope}: support class");
            }
            if let Some(expected) = case.fixture_meta.expected.certainty_label_class {
                assert_eq!(
                    case.record.certainty_label_class, expected,
                    "{scope}: certainty label class"
                );
            }
            if let Some(expected) = case.fixture_meta.expected.pack_source_class {
                assert_eq!(
                    case.record.pack_or_bridge_source_block.pack_source_class, expected,
                    "{scope}: pack source class"
                );
            }
            if let Some(expected) = case.fixture_meta.expected.convention_diagnostic_class {
                let diag = case
                    .record
                    .convention_diagnostic_block
                    .as_ref()
                    .unwrap_or_else(|| panic!("{scope}: diagnostic block required"));
                assert_eq!(
                    diag.convention_diagnostic_class, expected,
                    "{scope}: convention diagnostic class"
                );
            }
            if let Some(expected) = case.fixture_meta.expected.convention_certainty_class {
                let diag = case
                    .record
                    .convention_diagnostic_block
                    .as_ref()
                    .unwrap_or_else(|| panic!("{scope}: diagnostic block required"));
                assert_eq!(
                    diag.convention_certainty_class, expected,
                    "{scope}: convention certainty class"
                );
            }
            if let Some(expected) = case.fixture_meta.expected.generator_kind_class {
                let gen = case
                    .record
                    .generator_preview_block
                    .as_ref()
                    .unwrap_or_else(|| panic!("{scope}: generator block required"));
                assert_eq!(
                    gen.generator_kind_class, expected,
                    "{scope}: generator kind class"
                );
            }
            if let Some(expected) = case.fixture_meta.expected.rollback_class {
                let gen = case
                    .record
                    .generator_preview_block
                    .as_ref()
                    .unwrap_or_else(|| panic!("{scope}: generator block required"));
                assert_eq!(gen.rollback_class, expected, "{scope}: rollback class");
            }

            observed_support.insert(case.record.support_class.as_str().to_owned());
            observed_certainty.insert(case.record.certainty_label_class.as_str().to_owned());
            observed_kinds.insert(case.record.framework_object_kind.as_str().to_owned());
        }
    }

    for label in &expected_support {
        assert!(
            observed_support.contains(label),
            "expected support class '{label}' to appear in the corpus; observed {observed_support:?}"
        );
    }
    for label in &expected_certainty {
        assert!(
            observed_certainty.contains(label),
            "expected certainty label class '{label}' to appear in the corpus; observed {observed_certainty:?}"
        );
    }
    for label in &expected_kinds {
        assert!(
            observed_kinds.contains(label),
            "expected framework object kind '{label}' to appear in the corpus; observed {observed_kinds:?}"
        );
    }
}
