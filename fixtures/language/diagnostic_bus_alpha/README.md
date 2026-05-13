# Diagnostic bus alpha fixtures

These JSON fixtures exercise the diagnostic bus envelope at
`schemas/diagnostics/diagnostic_bus.schema.json` and the Rust
implementation in `crates/aureline-language/src/diagnostics/`.

The corpus keeps only opaque diagnostic, provider, rule, target, epoch,
anchor, and evidence refs plus typed vocabulary and export-safe
summaries. It does not include raw source text, raw provider payloads,
raw logs, raw paths, raw command lines, raw URLs, or secret material.

## Cases

| Fixture | Scenario |
|---|---|
| `bus_cases.json` | Current compiler and language-server diagnostics coexist with cached linter output, imported scan evidence, a partial-scope framework analyzer, a quarantined language-server provider row from the router, and an unavailable linter row. |
