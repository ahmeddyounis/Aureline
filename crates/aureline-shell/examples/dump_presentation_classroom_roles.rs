//! Emits the seeded presentation classroom-role-and-authority fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- corpus
//! cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- support-export
//! cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- profile-example
//! cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- packet-example
//! ```
//!
//! The `corpus` output is the literal source of
//! `fixtures/presentation/classroom-role-and-authority/classroom-role-and-authority-corpus.json`;
//! the `support-export` output is the source of the sibling
//! `classroom-role-and-authority-support-export.json`. The `profile-example` and
//! `packet-example` outputs are the sources of
//! `artifacts/presentation/classroom-role.example.json` and
//! `artifacts/presentation/exercise-packet.example.json`.

use aureline_shell::presentation::classroom::{
    classroom_role_example, classroom_role_support_export, exercise_packet_example,
    seeded_classroom_role_corpus, validate_classroom_role_corpus,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "corpus".to_owned());
    let corpus = seeded_classroom_role_corpus();
    validate_classroom_role_corpus(&corpus)
        .map_err(|err| format!("seeded corpus failed validation: {err:?}"))?;

    let json = match mode.as_str() {
        "corpus" => serde_json::to_string_pretty(&corpus)?,
        "support-export" => {
            let export = classroom_role_support_export(
                "support-export:presentation-classroom-role:001",
                "2026-06-21T00:00:00Z",
                &corpus,
            );
            let violations = export.validate();
            if !violations.is_empty() {
                return Err(format!("support export failed validation: {violations:?}").into());
            }
            serde_json::to_string_pretty(&export)?
        }
        "profile-example" => serde_json::to_string_pretty(&classroom_role_example())?,
        "packet-example" => serde_json::to_string_pretty(&exercise_packet_example())?,
        other => return Err(format!("unknown mode: {other}").into()),
    };
    println!("{json}");
    Ok(())
}
