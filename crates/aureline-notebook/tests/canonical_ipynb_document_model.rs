//! End-to-end coverage for the canonical .ipynb document model corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    NotebookAttachment, NotebookAttachmentPreviewClass, NotebookCanonicalPreservationClass,
    NotebookCell, NotebookCellIdStabilityClass, NotebookCellType, NotebookDocument,
    NotebookLocalStateOverlay, NotebookMetadataSurvivalClass, NotebookNoKernelEditabilityClass,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_cell_types: Vec<String>,
    expected_canonical_preservation_classes: Vec<String>,
    expected_cell_id_stability_classes: Vec<String>,
    expected_metadata_survival_classes: Vec<String>,
    expected_no_kernel_editability_classes: Vec<String>,
    expected_attachment_preview_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    notebook_document: NotebookDocument,
    notebook_cell: NotebookCell,
    notebook_attachment: NotebookAttachment,
    notebook_local_state_overlay: NotebookLocalStateOverlay,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    cell_type: NotebookCellType,
    canonical_preservation_class: NotebookCanonicalPreservationClass,
    cell_id_stability_class: NotebookCellIdStabilityClass,
    metadata_survival_class: NotebookMetadataSurvivalClass,
    no_kernel_editability_class: NotebookNoKernelEditabilityClass,
    attachment_preview_class: NotebookAttachmentPreviewClass,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_document: Vec<String>,
    #[serde(default)]
    notebook_cell: Vec<String>,
    #[serde(default)]
    notebook_attachment: Vec<String>,
    #[serde(default)]
    notebook_local_state_overlay: Vec<String>,
}

fn read_manifest() -> Manifest {
    let path = fixture_root().join("manifest.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read manifest {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse manifest {}: {err}", path.display()))
}

fn read_case(case_path: &str) -> FixtureCase {
    let path = repo_root().join(case_path);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn assert_findings_match(
    check_ids: &[String],
    findings: &[aureline_notebook::DocumentModelFinding],
) {
    let actual: Vec<String> = findings.iter().map(|f| f.check_id.clone()).collect();
    assert_eq!(
        actual, *check_ids,
        "expected findings {check_ids:?}, got {actual:?}"
    );
}

#[test]
fn manifest_lists_all_case_files() {
    let manifest = read_manifest();
    assert_eq!(manifest.schema_version, 1);

    for case in &manifest.case_refs {
        let path = repo_root().join(case);
        assert!(path.exists(), "manifest references missing file: {case}");
    }

    let dir = fixture_root();
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".yaml"))
        .filter(|name| name != "manifest.yaml")
        .collect();
    on_disk.sort();

    let mut referenced: Vec<String> = manifest
        .case_refs
        .iter()
        .map(|case| {
            Path::new(case)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    referenced.sort();

    assert_eq!(
        on_disk, referenced,
        "manifest case_refs must match yaml files on disk"
    );
}

#[test]
fn every_case_validates_and_matches_expectations() {
    let manifest = read_manifest();
    let mut observed_cell_types = BTreeMap::new();
    let mut observed_canonical_preservation = BTreeMap::new();
    let mut observed_cell_id_stability = BTreeMap::new();
    let mut observed_metadata_survival = BTreeMap::new();
    let mut observed_no_kernel_editability = BTreeMap::new();
    let mut observed_attachment_preview = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let doc_findings = case.notebook_document.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_document,
            &doc_findings,
        );

        let cell_findings = case.notebook_cell.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_cell,
            &cell_findings,
        );

        let attachment_findings = case.notebook_attachment.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_attachment,
            &attachment_findings,
        );

        let overlay_findings = case.notebook_local_state_overlay.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_local_state_overlay,
            &overlay_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.notebook_cell.cell_type, case.fixture.expected.cell_type,
            "fixture {name} cell_type mismatch"
        );
        assert_eq!(
            case.notebook_document.canonical_preservation_class,
            case.fixture.expected.canonical_preservation_class,
            "fixture {name} canonical_preservation_class mismatch"
        );
        assert_eq!(
            case.notebook_document.cell_id_stability_class,
            case.fixture.expected.cell_id_stability_class,
            "fixture {name} cell_id_stability_class mismatch"
        );
        assert_eq!(
            case.notebook_document.metadata_survival_class,
            case.fixture.expected.metadata_survival_class,
            "fixture {name} metadata_survival_class mismatch"
        );
        assert_eq!(
            case.notebook_document.no_kernel_editability_class,
            case.fixture.expected.no_kernel_editability_class,
            "fixture {name} no_kernel_editability_class mismatch"
        );
        assert_eq!(
            case.notebook_attachment.preview_class, case.fixture.expected.attachment_preview_class,
            "fixture {name} attachment_preview_class mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case.notebook_document.cells.is_empty(),
            "fixture {name}: notebook must contain at least one cell"
        );
        assert!(
            case.notebook_document.local_state_overlay.document_id_ref
                == case.notebook_document.document_id,
            "fixture {name}: overlay document_id_ref must match document_id"
        );
        for cell in &case.notebook_document.cells {
            assert_eq!(
                cell.document_id_ref, case.notebook_document.document_id,
                "fixture {name}: cell document_id_ref must match document_id"
            );
        }

        observed_cell_types.insert(case.notebook_cell.cell_type.as_str(), ());
        observed_canonical_preservation.insert(
            case.notebook_document.canonical_preservation_class.as_str(),
            (),
        );
        observed_cell_id_stability
            .insert(case.notebook_document.cell_id_stability_class.as_str(), ());
        observed_metadata_survival
            .insert(case.notebook_document.metadata_survival_class.as_str(), ());
        observed_no_kernel_editability.insert(
            case.notebook_document.no_kernel_editability_class.as_str(),
            (),
        );
        observed_attachment_preview.insert(case.notebook_attachment.preview_class.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_cell_types {
        assert!(
            observed_cell_types.contains_key(expected.as_str()),
            "no fixture exercises cell type '{expected}'"
        );
    }
    for expected in &manifest.expected_canonical_preservation_classes {
        assert!(
            observed_canonical_preservation.contains_key(expected.as_str()),
            "no fixture exercises canonical preservation class '{expected}'"
        );
    }
    for expected in &manifest.expected_cell_id_stability_classes {
        assert!(
            observed_cell_id_stability.contains_key(expected.as_str()),
            "no fixture exercises cell-id stability class '{expected}'"
        );
    }
    for expected in &manifest.expected_metadata_survival_classes {
        assert!(
            observed_metadata_survival.contains_key(expected.as_str()),
            "no fixture exercises metadata survival class '{expected}'"
        );
    }
    for expected in &manifest.expected_no_kernel_editability_classes {
        assert!(
            observed_no_kernel_editability.contains_key(expected.as_str()),
            "no fixture exercises no-kernel editability class '{expected}'"
        );
    }
    for expected in &manifest.expected_attachment_preview_classes {
        assert!(
            observed_attachment_preview.contains_key(expected.as_str()),
            "no fixture exercises attachment preview class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_document_model_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
