# LSP Router Alpha Fixtures

These fixtures exercise the baseline LSP router and workspace-scoped language
host supervisor. They prove that launch-language hosts are routed through one
decision path, restart/reconnect/quarantine states remain visible, and support
packets carry traceable host identity without raw source, raw process args, raw
provider logs, hostnames, or secrets.

## Cases

| Case | Scenario |
|---|---|
| `ready_ts_completion_uses_lsp` | A ready TypeScript language host wins completion and the syntax lane remains a lower-authority fallback. |
| `reconnecting_python_hover_falls_back` | A Python host in reconnecting/warming state forces hover to a labeled syntax fallback. |
| `unavailable_ts_definition_falls_back` | An unavailable TypeScript host cannot silently disappear; definition uses syntax fallback with degraded state. |
| `quarantined_ts_diagnostics_falls_back` | A crash-loop-quarantined TypeScript host stays inspectable and diagnostics fall back with a quarantine label. |

The Rust tests in `crates/aureline-language/tests/lsp_router_alpha.rs` and
`crates/aureline-runtime/tests/language_hosts_alpha.rs` load this fixture.
