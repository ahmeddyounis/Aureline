use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use aureline_provider::{
    seeded_infrastructure_intelligence_alpha_page, InfrastructureIntelligenceAlphaPage,
};

fn main() {
    let options = match parse_options(env::args().skip(1).collect::<Vec<_>>().as_slice()) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            eprintln!(
                "usage: aureline_provider_infrastructure_intelligence_alpha [page|validate|search-projection|review-projection|support-export] [--fixture PATH]"
            );
            process::exit(2);
        }
    };

    let page = match options.fixture_path {
        Some(path) => load_fixture(path),
        None => seeded_infrastructure_intelligence_alpha_page(),
    };

    match options.mode {
        Mode::Page => print_json(&page),
        Mode::Validate => {
            let report = page.validate();
            print_json(&report);
            if !report.passed {
                process::exit(1);
            }
        }
        Mode::SearchProjection => print_json(&page.search_projection()),
        Mode::ReviewProjection => print_json(&page.review_projection()),
        Mode::SupportExport => print_json(&page.support_export_projection()),
    }
}

fn load_fixture(path: PathBuf) -> InfrastructureIntelligenceAlphaPage {
    let payload = match fs::read_to_string(&path) {
        Ok(payload) => payload,
        Err(error) => {
            eprintln!("failed to read {}: {error}", path.display());
            process::exit(2);
        }
    };
    match serde_json::from_str(&payload) {
        Ok(page) => page,
        Err(error) => {
            eprintln!("failed to parse {}: {error}", path.display());
            process::exit(2);
        }
    }
}

fn print_json<T>(value: &T)
where
    T: serde::Serialize,
{
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("payload serializes")
    );
}

fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut mode = Mode::Page;
    let mut fixture_path = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "page" => mode = Mode::Page,
            "validate" => mode = Mode::Validate,
            "search-projection" => mode = Mode::SearchProjection,
            "review-projection" => mode = Mode::ReviewProjection,
            "support-export" => mode = Mode::SupportExport,
            "--fixture" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("--fixture requires a path".to_string());
                };
                fixture_path = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        index += 1;
    }
    Ok(Options { mode, fixture_path })
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Page,
    Validate,
    SearchProjection,
    ReviewProjection,
    SupportExport,
}

#[derive(Debug)]
struct Options {
    mode: Mode,
    fixture_path: Option<PathBuf>,
}
