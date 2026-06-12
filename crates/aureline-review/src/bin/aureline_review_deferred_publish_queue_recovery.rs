use std::io::{self, Write};

use aureline_review::canonical_deferred_publish_queue_recovery_packet;

fn main() -> io::Result<()> {
    let packet = canonical_deferred_publish_queue_recovery_packet();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer_pretty(&mut stdout, &packet)?;
    stdout.write_all(b"\n")?;
    Ok(())
}
