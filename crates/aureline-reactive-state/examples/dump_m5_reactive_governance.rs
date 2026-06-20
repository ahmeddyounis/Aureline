use aureline_reactive_state::{
    seeded_m5_reactive_governance_fixtures, seeded_m5_reactive_governance_packet,
};

fn main() {
    let packet = seeded_m5_reactive_governance_packet();
    let fixtures = seeded_m5_reactive_governance_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
