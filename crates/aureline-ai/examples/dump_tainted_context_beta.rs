use std::error::Error;

use aureline_ai::current_beta_tainted_context_support_export;

fn main() -> Result<(), Box<dyn Error>> {
    let packet = current_beta_tainted_context_support_export()?;
    println!("{}", packet.export_safe_json());
    Ok(())
}
