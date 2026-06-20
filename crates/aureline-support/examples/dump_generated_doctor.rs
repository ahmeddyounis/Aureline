//! Dumps or regenerates the generated-artifact Project Doctor packet and
//! fixture corpus from the seeded projection.
//!
//! With no argument it prints `{ "packet": ..., "fixtures": [...] }` for human
//! inspection. Pass `packet` to print only the proof packet (the form written
//! to `artifacts/generated/generated-doctor-packet.json`), `fixtures` to print
//! only the fixture corpus, or `write` to (re)write the checked-in packet and
//! fixture files on disk.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::generated_doctor::{
    seeded_generated_doctor_findings_packet, seeded_generated_doctor_fixtures,
    GeneratedDoctorFixture, GENERATED_DOCTOR_FIXTURE_DIR, GENERATED_DOCTOR_PACKET_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn fixture_file_stem(fixture: &GeneratedDoctorFixture) -> String {
    fixture
        .finding
        .finding_id
        .trim_start_matches("doctor.")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn write_files() {
    let root = repo_root();

    let packet = seeded_generated_doctor_findings_packet();
    let packet_path = root.join(GENERATED_DOCTOR_PACKET_REF);
    let packet_json =
        serde_json::to_string_pretty(&packet).expect("doctor packet serializes") + "\n";
    fs::write(&packet_path, packet_json)
        .unwrap_or_else(|err| panic!("write {}: {err}", packet_path.display()));
    println!("wrote {}", packet_path.display());

    let dir = root.join(GENERATED_DOCTOR_FIXTURE_DIR);
    fs::create_dir_all(&dir).expect("fixture dir");
    for fixture in seeded_generated_doctor_fixtures() {
        let path = dir.join(format!("{}.json", fixture_file_stem(&fixture)));
        let json = serde_json::to_string_pretty(&fixture).expect("fixture serializes") + "\n";
        fs::write(&path, json).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
        println!("wrote {}", path.display());
    }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    match mode.as_str() {
        "write" => {
            write_files();
        }
        "packet" => {
            let value = serde_json::to_value(seeded_generated_doctor_findings_packet())
                .expect("packet serializes");
            println!(
                "{}",
                serde_json::to_string_pretty(&value).expect("pretty JSON")
            );
        }
        "fixtures" => {
            let value = serde_json::to_value(seeded_generated_doctor_fixtures())
                .expect("fixtures serialize");
            println!(
                "{}",
                serde_json::to_string_pretty(&value).expect("pretty JSON")
            );
        }
        _ => {
            let value = serde_json::to_value(serde_json::json!({
                "packet": seeded_generated_doctor_findings_packet(),
                "fixtures": seeded_generated_doctor_fixtures(),
            }))
            .expect("packet and fixtures serialize");
            println!(
                "{}",
                serde_json::to_string_pretty(&value).expect("pretty JSON")
            );
        }
    }
}
