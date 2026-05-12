# Alpha Palette Query Fixtures

These fixtures exercise the launch-wedge discoverability projection in
`crates/aureline-shell/src/palette/discoverability.rs`.

They prove that files, symbols, commands, and recent actions share one row
contract, and that disabled or preview-required command entries keep routing
through the canonical descriptor, enablement, diagnostics, and invocation
preview paths.

| Fixture | Purpose |
|---|---|
| `wedge_query_open.json` | Query returns recent action, command, symbol, and file rows with stable category/source/shortcut/side-effect/footer fields. |
| `blocked_clone_discoverable.json` | Blocked command remains discoverable with a structured diagnostics-sheet route. |
| `import_preview_deeplink.json` | Preview-required command deep link routes to the same invocation-preview sheet before apply. |
