//! Headless emitter for the migration-wizard import-fidelity packet.
//!
//! Reads a JSON envelope carrying the migration-wizard import-fidelity inputs
//! and prints the governed, export-safe fidelity packet. This is the CLI /
//! replay surface that proves the same projection the migration center and
//! entry surfaces consume.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "migration_session_ref": "migration-session-example",
//!   "source_editor": "vs_code_code_oss",
//!   "selected_target_families": ["settings", "keybindings"],
//!   "detected_source_profile_refs": ["profile://vscode/settings.json"],
//!   "require_rollback_checkpoint": true,
//!   "consumer_surfaces": ["migration_center", "cli_inspector"]
//! }
//! ```

use std::io::{self, Read};

use aureline_workspace::{
    project_migration_wizard_import_fidelity_packet, MigrationWizardImportFidelityInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FidelityEmitterEnvelope {
    migration_session_ref: String,
    source_editor: String,
    #[serde(default)]
    selected_target_families: Vec<String>,
    #[serde(default)]
    detected_source_profile_refs: Vec<String>,
    #[serde(default)]
    require_rollback_checkpoint: bool,
    #[serde(default)]
    consumer_surfaces: Vec<String>,
}

fn main() {
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_migration_wizard_import_fidelity [envelope.json]\n\
                     reads the fidelity envelope from stdin when no path is given."
                );
                return;
            }
            other => path_arg = Some(other.to_owned()),
        }
    }

    let raw = match path_arg {
        Some(path) => std::fs::read_to_string(&path).unwrap_or_else(|err| {
            eprintln!("failed to read {path}: {err}");
            std::process::exit(2);
        }),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("stdin must be readable");
            buf
        }
    };

    let envelope: FidelityEmitterEnvelope = serde_json::from_str(&raw).unwrap_or_else(|err| {
        eprintln!("failed to parse fidelity envelope: {err}");
        std::process::exit(2);
    });

    let input = MigrationWizardImportFidelityInput {
        migration_session_ref: envelope.migration_session_ref,
        source_editor: envelope.source_editor,
        selected_target_families: envelope.selected_target_families,
        detected_source_profile_refs: envelope.detected_source_profile_refs,
        require_rollback_checkpoint: envelope.require_rollback_checkpoint,
        consumer_surfaces: envelope.consumer_surfaces,
    };

    let projection =
        project_migration_wizard_import_fidelity_packet(&input).unwrap_or_else(|err| {
            eprintln!("failed to project fidelity packet: {err}");
            std::process::exit(2);
        });

    println!(
        "{}",
        serde_json::to_string_pretty(&projection.packet).expect("fidelity packet must serialize")
    );
}
