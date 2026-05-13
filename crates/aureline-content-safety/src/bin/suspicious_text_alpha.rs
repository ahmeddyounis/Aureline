use std::io::{self, Read};

use aureline_content_safety::{
    project_suspicious_text_core_surfaces, SuspiciousTextProjectionSeed,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage: suspicious_text_alpha <case-id> < input.txt");
        std::process::exit(2);
    }

    let mut content = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut content) {
        eprintln!("failed to read stdin: {err}");
        std::process::exit(1);
    }

    let case_id = &args[1];
    let seed = SuspiciousTextProjectionSeed {
        case_id,
        content: &content,
        editor_subject_ref: "cli:editor:stdin",
        diff_hunk_ref: "cli:diff:stdin:hunk:1",
        search_row_ref: "cli:search:stdin:row:1",
        review_anchor_ref: "cli:review:stdin:anchor:1",
    };
    let packet = project_suspicious_text_core_surfaces(&seed);
    match serde_json::to_string_pretty(&packet) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("failed to serialize packet: {err}");
            std::process::exit(1);
        }
    }
}
