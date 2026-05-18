//! Protected fixture checks for the command-reference catalog.
//!
//! The integration test replays the JSON fixture under
//! `fixtures/ux/m3/command_reference_and_discoverability/` through
//! the Rust types and asserts contract invariants. The catalog
//! fixture is asserted bit-for-bit equal to the catalog minted by
//! `seeded_command_reference_catalog`, the markdown artifact at
//! `artifacts/ux/m3/command_reference_parity_report.md` is asserted
//! bit-for-bit equal to the rendering, and the companion beta-
//! contract doc is checked for the required artifact and surface
//! anchors so the docs/help / parity / fixture paths stay in
//! agreement.

use std::path::{Path, PathBuf};

use aureline_shell::command_reference::{
    render_catalog_markdown, search_entries, seeded_command_reference_catalog,
    validate_command_reference_catalog, CommandReferenceCatalog, ReferenceSurfaceFamily,
    SearchTokenClass, COMMAND_REFERENCE_CATALOG_RECORD_KIND, COMMAND_REFERENCE_PUBLISHED_DOC_REF,
    COMMAND_REFERENCE_PUBLISHED_REPORT_REF, COMMAND_REFERENCE_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ux/m3/command_reference_and_discoverability")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m3")
}

fn docs_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/ux/m3")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_catalog_is_bit_for_bit_equal_to_seed() {
    let on_disk: CommandReferenceCatalog = load_json("catalog.json");
    let seeded = seeded_command_reference_catalog();
    assert_eq!(on_disk, seeded, "fixture catalog diverged from seeded catalog");
    assert_eq!(seeded.record_kind, COMMAND_REFERENCE_CATALOG_RECORD_KIND);
    assert_eq!(seeded.shared_contract_ref, COMMAND_REFERENCE_SHARED_CONTRACT_REF);
}

#[test]
fn fixture_catalog_passes_validation() {
    let catalog: CommandReferenceCatalog = load_json("catalog.json");
    validate_command_reference_catalog(&catalog).expect("fixture catalog must validate");
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let catalog = seeded_command_reference_catalog();
    let rendered = render_catalog_markdown(&catalog);
    let path = artifacts_root().join("command_reference_parity_report.md");
    let on_disk = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "published {} must exist: {err}",
            path.display()
        )
    });
    assert_eq!(
        on_disk, rendered,
        "published command_reference_parity_report.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_command_reference -- report-md`",
    );
}

#[test]
fn published_doc_links_every_required_artifact() {
    let body = std::fs::read_to_string(docs_root().join("command_reference_beta_contract.md"))
        .expect("published command_reference_beta_contract doc must exist");
    assert!(body.contains("/artifacts/ux/m3/command_reference_parity_report.md"));
    assert!(body.contains(
        "/fixtures/ux/m3/command_reference_and_discoverability/catalog.json",
    ));
    assert!(body.contains("/schemas/commands/command_reference_entry.schema.json"));
    assert!(body.contains("aureline_shell_command_reference"));
}

#[test]
fn published_paths_match_constants() {
    let catalog = seeded_command_reference_catalog();
    assert_eq!(catalog.shared_contract_ref, COMMAND_REFERENCE_SHARED_CONTRACT_REF);
    let report_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(COMMAND_REFERENCE_PUBLISHED_REPORT_REF);
    assert!(report_path.exists(), "report must exist at {}", report_path.display());
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(COMMAND_REFERENCE_PUBLISHED_DOC_REF);
    assert!(doc_path.exists(), "doc must exist at {}", doc_path.display());
}

#[test]
fn search_index_resolves_label_id_alias_and_chord() {
    let catalog = seeded_command_reference_catalog();

    let by_label = search_entries(&catalog, "Open Folder");
    assert!(!by_label.is_empty());
    assert_eq!(by_label[0].entry.command_id, "cmd:workspace.open_folder");
    assert_eq!(by_label[0].token_class, SearchTokenClass::HumanLabel);

    let by_id = search_entries(&catalog, "cmd:workspace.import_profile");
    assert!(!by_id.is_empty());
    assert_eq!(by_id[0].entry.command_id, "cmd:workspace.import_profile");
    assert_eq!(by_id[0].token_class, SearchTokenClass::CommandId);

    let by_alias = search_entries(
        &catalog,
        "alias:workspace.open_folder:legacy_file_open_folder",
    );
    assert!(!by_alias.is_empty());
    assert_eq!(by_alias[0].entry.command_id, "cmd:workspace.open_folder");
    assert_eq!(by_alias[0].token_class, SearchTokenClass::AliasId);

    let by_chord = search_entries(&catalog, "chord:cmd+shift+p");
    assert!(!by_chord.is_empty());
    assert_eq!(by_chord[0].entry.command_id, "cmd:command_palette.open");
    assert_eq!(by_chord[0].token_class, SearchTokenClass::KeySequence);
}

#[test]
fn every_entry_lists_at_least_one_surface_and_canonical_anchor() {
    let catalog = seeded_command_reference_catalog();
    for entry in &catalog.entries {
        assert!(
            !entry.availability.supported_surfaces.is_empty(),
            "entry {} must list supported surfaces",
            entry.command_id
        );
        assert!(
            entry
                .availability
                .supported_surfaces
                .contains(&ReferenceSurfaceFamily::DocsHelp),
            "entry {} must be reachable from docs/help",
            entry.command_id
        );
        assert!(
            !entry.docs_help_anchor_ref.trim().is_empty(),
            "entry {} must quote a canonical docs/help anchor",
            entry.command_id
        );
    }
}

#[test]
fn high_risk_entries_require_preview() {
    let catalog = seeded_command_reference_catalog();
    for entry in &catalog.entries {
        if entry.risk_class.is_high_risk() {
            assert!(
                entry.preview_class.requires_preview(),
                "high-risk entry {} must require preview",
                entry.command_id
            );
        }
    }
}

#[test]
fn deprecated_aliases_carry_replacement_and_impact() {
    let catalog = seeded_command_reference_catalog();
    let mut saw_deprecated_alias = false;
    for entry in &catalog.entries {
        for alias in &entry.aliases {
            if alias.lifecycle_state
                != aureline_shell::command_reference::AliasLifecycleState::Active
            {
                saw_deprecated_alias = true;
                assert!(
                    alias.replacement_command_id.is_some(),
                    "deprecated alias {} on {} must name a replacement",
                    alias.alias_id,
                    entry.command_id
                );
                assert!(
                    alias.retirement_version.is_some(),
                    "deprecated alias {} on {} must name retirement version",
                    alias.alias_id,
                    entry.command_id
                );
                assert!(
                    alias.import_impact_class.is_some(),
                    "deprecated alias {} on {} must name import impact",
                    alias.alias_id,
                    entry.command_id
                );
            }
        }
    }
    assert!(
        saw_deprecated_alias,
        "catalog must seed at least one deprecated alias so deprecation truth is exercised",
    );
}

#[test]
fn schema_file_is_present_and_quotes_record_kinds() {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("schemas/commands/command_reference_entry.schema.json");
    let body = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("schema must exist at {}: {err}", schema_path.display()));
    assert!(body.contains("command_reference_entry_record"));
    assert!(body.contains("command_reference_catalog_record"));
    assert!(body.contains("search_index_token"));
    assert!(body.contains("keybinding_fact"));
    assert!(body.contains("availability_section"));
}
