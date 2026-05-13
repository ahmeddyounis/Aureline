use std::io::{self, Read};

use aureline_language::{ParseRequest, TreeSitterParserSupervisor};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(language_id) = args.next() else {
        eprintln!("usage: aureline_language_tree_sitter <language-id> [source-text]");
        std::process::exit(2);
    };

    let source_text = match args.next() {
        Some(source_text) => source_text,
        None => {
            let mut source_text = String::new();
            io::stdin()
                .read_to_string(&mut source_text)
                .expect("stdin must be readable");
            source_text
        }
    };

    let supervisor = TreeSitterParserSupervisor::with_default_registry();
    let request = ParseRequest::foreground_file(
        "parse-session:cli:tree-sitter",
        "doc:cli:stdin",
        "buffer:cli:stdin",
        1,
        language_id,
        "2026-05-13T00:00:00Z",
    );
    let output = supervisor.parse_text(request, &source_text);
    println!(
        "{}",
        serde_json::to_string_pretty(&output.record).expect("parse-session record must serialize")
    );
}
