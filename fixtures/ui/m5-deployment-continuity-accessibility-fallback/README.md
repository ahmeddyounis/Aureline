# M5 deployment/continuity accessibility fallback fixtures

This fixture corpus mirrors the checked release proof for M05-834. It certifies
that deployment/continuity component families expose keyboard-complete,
screen-reader-reachable, CLI/export-safe access and auto-narrow deployment
claims when rollout state, residual dependency, control-plane freshness, mirror
verification, handler ownership, or state-root integrity weakens.

- `support_export.json` is the canonical metadata-only packet.
- `matrix.csv` is the release/support triage projection of the same rows.

Regenerate both from
`seeded_m5_deployment_a11y_fallback_packet()` in
`crates/aureline-install/src/implement_keyboard_screen_reader_cli_export_parity_and_deployment_continuity_auto_narrowing/`.
