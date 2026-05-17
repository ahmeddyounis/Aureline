use aureline_ai::current_beta_cost_routing_packet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let packet = current_beta_cost_routing_packet()?;
    println!("{}", packet.export_safe_json());
    Ok(())
}
