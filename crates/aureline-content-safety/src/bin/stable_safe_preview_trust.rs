use aureline_content_safety::stable_safe_preview_trust_packet;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let packet = stable_safe_preview_trust_packet();

    let value = match args.get(1).map(String::as_str) {
        None | Some("validate") => serde_json::to_value(packet.validate()),
        Some("packet") => serde_json::to_value(&packet),
        Some(other) => {
            eprintln!("usage: stable_safe_preview_trust [validate|packet]");
            eprintln!("unknown command: {other}");
            std::process::exit(2);
        }
    };

    match value.and_then(|value| serde_json::to_string_pretty(&value)) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("failed to serialize stable safe-preview trust output: {err}");
            std::process::exit(1);
        }
    }

    if !packet.validate().is_green() {
        std::process::exit(1);
    }
}
