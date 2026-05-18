use std::error::Error;

use aureline_ai::current_beta_prompt_composer_conformance_export;

fn main() -> Result<(), Box<dyn Error>> {
    let packet = current_beta_prompt_composer_conformance_export()?;
    println!("{}", packet.export_safe_json());
    Ok(())
}
