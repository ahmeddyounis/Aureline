# Beta AI graduation packets

This directory is the checked-in graduation state for claimed beta AI
surfaces. `graduation_state.json` is the canonical state ref consumed by docs,
release truth, CLI/headless inspection, and support export. The standalone
packet files hold the current packet for each claimed AI surface in the beta
provider/model registry.

The generated support projection is `support_export_projection.json`; it is
validated by `cargo test -p aureline-ai graduation --no-fail-fast`.
