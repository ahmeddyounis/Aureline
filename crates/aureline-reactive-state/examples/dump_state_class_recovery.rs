use aureline_reactive_state::{
    seeded_state_class_recovery_fixtures, seeded_state_class_recovery_packet,
};

fn main() {
    let packet = seeded_state_class_recovery_packet();
    let fixtures = seeded_state_class_recovery_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
