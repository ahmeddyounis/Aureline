//! Protected fixture checks for the beta migration corpus scoreboard.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/migration/m3/incumbent_flows/` through the Rust types
//! and asserts the contract invariants. The scoreboard fixture is
//! also asserted bit-for-bit equal to the scoreboard minted by
//! `seeded_migration_scoreboard`, and the markdown artifact under
//! `artifacts/migration/m3/migration_scoreboard.md` is asserted
//! bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::migration_corpus::{
    seeded_migration_scoreboard, validate_migration_scoreboard, EcosystemScoreboardSection,
    IncumbentEcosystem, MigrationCorpusSupportExport, MigrationScoreboard,
    MIGRATION_CORPUS_SHARED_CONTRACT_REF, MIGRATION_SCOREBOARD_RECORD_KIND,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/migration/m3/incumbent_flows")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/migration/m3")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_scoreboard_is_bit_for_bit_equal_to_seed() {
    let on_disk: MigrationScoreboard = load_json("scoreboard.json");
    let seeded = seeded_migration_scoreboard();
    assert_eq!(
        on_disk, seeded,
        "fixture scoreboard diverged from seeded scoreboard"
    );
    assert_eq!(seeded.record_kind, MIGRATION_SCOREBOARD_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        MIGRATION_CORPUS_SHARED_CONTRACT_REF
    );
}

#[test]
fn fixture_scoreboard_passes_validation() {
    let scoreboard: MigrationScoreboard = load_json("scoreboard.json");
    validate_migration_scoreboard(&scoreboard).expect("fixture scoreboard must validate");
}

#[test]
fn fixture_scoreboard_covers_every_required_ecosystem() {
    let scoreboard: MigrationScoreboard = load_json("scoreboard.json");
    assert!(scoreboard.covers_every_required_ecosystem());
    for ecosystem in IncumbentEcosystem::required_ecosystems() {
        let section = scoreboard
            .sections
            .iter()
            .find(|section| section.ecosystem == ecosystem)
            .unwrap_or_else(|| panic!("section {} must be present", ecosystem.as_str()));
        assert!(!section.rows.is_empty());
    }
}

#[test]
fn fixture_scoreboard_covers_every_required_classification() {
    let scoreboard: MigrationScoreboard = load_json("scoreboard.json");
    assert!(scoreboard.covers_every_required_classification());
    assert!(scoreboard.overall_summary.exact >= 1);
    assert!(scoreboard.overall_summary.translated >= 1);
    assert!(scoreboard.overall_summary.partial >= 1);
    assert!(scoreboard.overall_summary.shimmed >= 1);
    assert!(scoreboard.overall_summary.unsupported >= 1);
}

#[test]
fn fixture_per_ecosystem_sections_match_scoreboard() {
    let scoreboard: MigrationScoreboard = load_json("scoreboard.json");
    let vscode: EcosystemScoreboardSection = load_json("vs_code_code_oss.json");
    let jetbrains: EcosystemScoreboardSection = load_json("jetbrains_family.json");
    let vim: EcosystemScoreboardSection = load_json("vim_neovim.json");
    let emacs: EcosystemScoreboardSection = load_json("emacs.json");

    let by = |ecosystem: IncumbentEcosystem| {
        scoreboard
            .sections
            .iter()
            .find(|section| section.ecosystem == ecosystem)
            .expect("section must be present")
            .clone()
    };

    assert_eq!(by(IncumbentEcosystem::VsCodeCodeOss), vscode);
    assert_eq!(by(IncumbentEcosystem::JetBrainsFamily), jetbrains);
    assert_eq!(by(IncumbentEcosystem::VimNeovim), vim);
    assert_eq!(by(IncumbentEcosystem::Emacs), emacs);
}

#[test]
fn fixture_support_export_quotes_every_flow_id() {
    let scoreboard: MigrationScoreboard = load_json("scoreboard.json");
    let export: MigrationCorpusSupportExport = load_json("support_export.json");
    let expected = MigrationCorpusSupportExport::from_scoreboard(
        export.support_export_id.clone(),
        scoreboard.clone(),
    );
    assert_eq!(export, expected);
    for section in &scoreboard.sections {
        for row in &section.rows {
            assert!(
                export.case_ids.contains(&row.flow_id),
                "support export must quote flow {}",
                row.flow_id
            );
        }
    }
}

#[test]
fn fixture_rows_quote_wizard_mapping_report() {
    let scoreboard: MigrationScoreboard = load_json("scoreboard.json");
    let report_ref = &scoreboard.wizard_mapping_report_ref;
    for section in &scoreboard.sections {
        for row in &section.rows {
            assert_eq!(&row.wizard_mapping_report_ref, report_ref);
            assert_eq!(
                &row.rollback_checkpoint_ref,
                &scoreboard.rollback_checkpoint_ref
            );
        }
    }
}

#[test]
fn published_scoreboard_md_matches_seeded_rendering() {
    let scoreboard = seeded_migration_scoreboard();
    let rendered = scoreboard.render_scoreboard_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("migration_scoreboard.md"))
        .expect("published scoreboard.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published scoreboard.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard-md`",
    );
}

#[test]
fn published_flow_matrix_links_every_required_ecosystem() {
    let matrix_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/migration/m3/incumbent_flow_matrix.md");
    let body = std::fs::read_to_string(&matrix_path)
        .expect("published incumbent flow matrix doc must exist");
    for ecosystem in IncumbentEcosystem::required_ecosystems() {
        let row_ref = ecosystem.source_ecosystem_row_ref();
        assert!(
            body.contains(row_ref),
            "incumbent_flow_matrix.md must quote source row {}",
            row_ref
        );
    }
    assert!(body.contains("/artifacts/migration/m3/migration_scoreboard.md"));
    assert!(body.contains("/fixtures/migration/m3/incumbent_flows/scoreboard.json"));
}
