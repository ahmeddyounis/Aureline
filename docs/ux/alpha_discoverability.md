# Alpha Palette Discoverability

This note freezes the alpha command-palette discoverability proof path. The
runtime projection lives in `crates/aureline-shell/src/palette/discoverability.rs`
and is exercised by fixtures in `fixtures/commands/alpha_palette_queries/`.

## Contract

- Palette rows for recent actions, commands, symbols, and files use one row
  shape: origin/source badge, category or path, winning keybinding state,
  dominant side-effect cue, availability class, preview detail, and action
  footer.
- Command rows are projected from the canonical command registry and evaluated
  through the command enablement/preflight engine.
- Disabled commands remain discoverable when they teach the user what is
  missing. The row carries the structured disabled reason and exposes the
  command diagnostics sheet.
- Preview-required and approval-required commands route to the invocation
  preview sheet before apply, including command scope, side-effect class,
  target refs, and rollback or checkpoint posture.
- Command deep links reuse the same descriptor, enablement, diagnostics, and
  invocation-preview path as local palette invocation. Deep links do not widen
  authority or bypass preview.
- Support export uses a redacted projection over the palette snapshot. It keeps
  row kinds and command ids, but omits raw query text.

## Protected Fixtures

| Fixture | Proof |
|---|---|
| `wedge_query_open.json` | Recent action, command, symbol, and file rows all surface stable row contract fields and useful ordering. |
| `blocked_clone_discoverable.json` | A command blocked by missing execution context remains visible with a diagnostics-sheet route. |
| `import_preview_deeplink.json` | A high-risk import command deep link routes to invocation preview with side-effect and rollback evidence before apply. |

## Verification

Run:

```bash
cargo test -p aureline-shell --test alpha_palette_discoverability_tests
```
