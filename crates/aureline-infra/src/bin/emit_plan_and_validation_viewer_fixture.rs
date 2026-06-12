fn main() {
    let packet = aureline_infra::seeded_plan_and_validation_viewer_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("fixture serializes")
    );
}
