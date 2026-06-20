use aureline_reactive_state::{
    seeded_reactive_command_parity_fixtures, seeded_reactive_command_parity_packet,
};

fn main() {
    let packet = seeded_reactive_command_parity_packet();
    let fixtures = seeded_reactive_command_parity_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
