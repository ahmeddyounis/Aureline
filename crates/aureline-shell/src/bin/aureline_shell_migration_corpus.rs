//! Headless inspector for the beta migration corpus and top-incumbent
//! flow scoreboard.
//!
//! The bin emits the same scoreboard records consumed by the live
//! migration center, the docs scoreboard, and the support-export
//! wrapper, and is the only mint-from-truth path for the JSON
//! checked in under `fixtures/migration/m3/incumbent_flows/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- vscode
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- jetbrains
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- vim
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- emacs
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard-md
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- validate
//! ```

use aureline_shell::migration_corpus::{
    seeded_migration_scoreboard, validate_migration_scoreboard, IncumbentEcosystem,
    MigrationCorpusSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let scoreboard = seeded_migration_scoreboard();

    match args.first().map(String::as_str) {
        Some("scoreboard") | None => {
            print_json(&scoreboard)?;
        }
        Some(name @ ("vscode" | "jetbrains" | "vim" | "emacs")) => {
            let ecosystem = match name {
                "vscode" => IncumbentEcosystem::VsCodeCodeOss,
                "jetbrains" => IncumbentEcosystem::JetBrainsFamily,
                "vim" => IncumbentEcosystem::VimNeovim,
                "emacs" => IncumbentEcosystem::Emacs,
                _ => unreachable!(),
            };
            let section = scoreboard
                .sections
                .iter()
                .find(|section| section.ecosystem == ecosystem)
                .ok_or_else(|| format!("ecosystem {name} not in scoreboard"))?;
            print_json(section)?;
        }
        Some("support-export") => {
            let export = MigrationCorpusSupportExport::from_scoreboard(
                "support-export:migration-corpus:001",
                scoreboard,
            );
            print_json(&export)?;
        }
        Some("scoreboard-md") => {
            print!("{}", scoreboard.render_scoreboard_markdown());
        }
        Some("compact") => {
            for line in scoreboard.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match validate_migration_scoreboard(&scoreboard) {
            Ok(()) => {
                println!("ok");
            }
            Err(errors) => {
                for err in &errors {
                    eprintln!(
                        "error: {}",
                        serde_json::to_string(err).unwrap_or_else(|_| format!("{err:?}"))
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
