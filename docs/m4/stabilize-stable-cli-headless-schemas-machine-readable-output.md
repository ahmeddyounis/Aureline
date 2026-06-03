# Stabilize stable CLI/headless schemas, machine-readable output, and support/export compatibility promises

## Goal

Turn the M4 stable line’s CLI command surfaces, headless output schemas, machine-readable output formats, and support/export compatibility promises into release-grade proof so public claims, docs/help, and support exports all stay aligned with the exact M4 build.

## Machine-readable source

The canonical register is [`/artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json`](../../artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json).

## What the register covers

- **CLI command surfaces** — command grammar, flags, exit codes, and schema versioning.
- **Headless output schemas** — JSON, XML, and other structured output schemas used in headless mode.
- **Machine-readable output formats** — tab-separated, JSON-lines, and other machine-readable formats.
- **Support/export compatibility promises** — backward-compatibility guarantees for support bundles, diagnostics exports, and general export formats.

## Downgrade behavior

Any row whose proof packet ages out, whose evidence becomes incomplete, whose schema breaks, whose waiver expires, or whose owner sign-off is missing is automatically narrowed below the stable cutline. Docs, Help/About, and support exports ingest the narrowed label directly from the register.

## Re-verification

```
cargo test -p aureline-cli
```

This runs the typed contract tests that bind the model to the checked-in register.
