# M5 embedded-boundary component consumer fixtures

Mirror of `artifacts/release/m5-embedded-boundary-component-consumer-proof/`. Proves the
docs/help pane, marketplace/account content, extension-owned embedded webview, browser /
device-code auth handoff, remote/service dashboard, and support/export + release-packet
consumers all reuse the same frozen embedded-boundary component families and vocabulary.
Regenerate with `GEN_EMBEDDED_BOUNDARY_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.
