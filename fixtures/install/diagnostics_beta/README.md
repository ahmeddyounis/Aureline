# Install Diagnostics Beta Fixtures

These fixtures exercise the exact-build install diagnostics contract consumed by
`crates/aureline-install`.

The canonical packet is checked in at
`artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json`; the
support-export projection is checked in beside it. The fixture manifest here
keeps the test input discoverable from the install fixture tree without copying
the same packet into a second location.

Run:

```bash
cargo test -p aureline-install --test install_diagnostics_beta
```
