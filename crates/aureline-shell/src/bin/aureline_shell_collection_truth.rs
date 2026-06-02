//! Headless inspector for the collection-truth beta packet.
//!
//! The bin emits the same packet consumed by the fixture test and
//! support evidence. It is the mint-from-truth path for
//! `fixtures/ux/m3/collection_truth/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- filter-bar <surface_family>
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- saved-view <surface_family>
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- scope-counter <surface_family>
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- batch-review <surface_family>
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- validate
//! ```

use aureline_shell::collection_truth::{
    seeded_collection_truth_beta_packet, validate_collection_truth_beta_packet,
    CollectionTruthCase, CollectionTruthSurfaceFamily,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_collection_truth_beta_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("filter-bar") => {
            let case = locate_case(&args, &packet.cases)?;
            print_json(&case.filter_bar)?;
        }
        Some("saved-view") => {
            let case = locate_case(&args, &packet.cases)?;
            print_json(&case.saved_view)?;
        }
        Some("scope-counter") => {
            let case = locate_case(&args, &packet.cases)?;
            print_json(&case.scope_counter)?;
        }
        Some("batch-review") => {
            let case = locate_case(&args, &packet.cases)?;
            print_json(&case.batch_review)?;
        }
        Some("validate") => match validate_collection_truth_beta_packet(&packet) {
            Ok(()) => println!("ok"),
            Err(errors) => {
                for err in errors {
                    eprintln!("{err}");
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

fn locate_case<'a>(
    args: &[String],
    cases: &'a [CollectionTruthCase],
) -> Result<&'a CollectionTruthCase, Box<dyn std::error::Error>> {
    let family_token = args
        .get(1)
        .ok_or("surface family argument required")?
        .as_str();
    let family = parse_family(family_token)?;
    cases
        .iter()
        .find(|case| case.surface_family == family)
        .ok_or_else(|| format!("no seeded case for surface family {}", family.as_str()).into())
}

fn parse_family(token: &str) -> Result<CollectionTruthSurfaceFamily, Box<dyn std::error::Error>> {
    for family in CollectionTruthSurfaceFamily::all() {
        if family.as_str() == token {
            return Ok(family);
        }
    }
    Err(format!("unknown surface family: {token}").into())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
