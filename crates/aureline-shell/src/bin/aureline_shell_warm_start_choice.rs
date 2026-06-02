//! Headless inspector for the beta warm-start choice surface.
//!
//! The same object model backs the Start Center, the workspace switcher, this
//! CLI/headless entry review, docs/help, and the support export. Reviewers and
//! scripts read it here to confirm — before any side effect — that each card
//! distinguishes resume-live, snapshot, clone-fresh, open-minimal, set-up-later,
//! and template paths.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- cards
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- card <card_id>
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- plaintext
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- vocabulary
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- validate
//! ```

use aureline_shell::start_center::warm_start_choice::{
    render_warm_start_choice_plaintext, seeded_warm_start_choice_page,
    seeded_warm_start_choice_support_export, validate_warm_start_choice_page,
    validate_warm_start_choice_support_export, warm_start_choice_vocabulary,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_warm_start_choice_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("cards") => print_json(&page.cards)?,
        Some("card") => {
            let card_id = args.get(1).ok_or("usage: card <card_id>")?.as_str();
            let card = page
                .cards
                .iter()
                .find(|card| card.card_id == card_id)
                .ok_or_else(|| format!("unknown card id: {card_id}"))?;
            print_json(card)?;
        }
        Some("summary") => print_json(&page.summary)?,
        Some("plaintext") => print!("{}", render_warm_start_choice_plaintext(&page)),
        Some("vocabulary") => print_json(&warm_start_choice_vocabulary())?,
        Some("support-export") => {
            let export = seeded_warm_start_choice_support_export();
            print_json(&export)?;
        }
        Some("validate") => {
            let page_result = validate_warm_start_choice_page(&page);
            let export = seeded_warm_start_choice_support_export();
            let export_result = validate_warm_start_choice_support_export(&export);
            match (page_result, export_result) {
                (Ok(()), Ok(())) => println!("ok"),
                (page_result, export_result) => {
                    if let Err(errors) = page_result {
                        for err in errors {
                            eprintln!("page error: {err}");
                        }
                    }
                    if let Err(errors) = export_result {
                        for err in errors {
                            eprintln!("support-export error: {err}");
                        }
                    }
                    std::process::exit(3);
                }
            }
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
