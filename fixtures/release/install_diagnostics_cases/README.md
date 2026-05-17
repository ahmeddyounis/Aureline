# Install Diagnostics Cases

This directory anchors release-side install diagnostics fixtures. The canonical
packet lives under `artifacts/release/m3/install_diagnostics/` so release,
support, and Rust tests all consume one source instead of copied JSON.

See `fixtures/install/diagnostics_beta/manifest.json` for the fixture manifest
used by `cargo test -p aureline-install --test install_diagnostics_beta`.
