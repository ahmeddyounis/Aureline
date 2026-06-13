use aureline_shell::artifact_save_truth::{
    artifact_save_truth_report_markdown, seeded_artifact_save_truth_fixtures,
    seeded_artifact_save_truth_packet,
};

fn main() {
    let packet = seeded_artifact_save_truth_packet();
    let fixtures = seeded_artifact_save_truth_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
            "report_markdown": artifact_save_truth_report_markdown(
                &seeded_artifact_save_truth_packet(),
                &seeded_artifact_save_truth_fixtures()
            ),
        }))
        .expect("packet and fixtures serialize")
    );
}
