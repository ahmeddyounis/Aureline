# M5 Git certification corpus

Protected fixtures for the M5 Git certification register
(`certify_m5_git_topology_history_recovery_and_provider_parity_rows`). Each file
is a full, schema-valid certification packet that demonstrates the fail-closed
downgrade automation: one row's dimension is degraded and the row's verdict is
re-derived from the evidence.

| Fixture | Degraded dimension | Row verdict |
| --- | --- | --- |
| `stale_topology_retest_pending.json` | topology honesty is stale | `retest_pending` |
| `failed_provider_parity_unsupported.json` | local/provider parity failed | `unsupported` |
| `partial_history_recovery_limited.json` | history recovery honestly partial | `limited` |

The canonical (all-certified) packet lives at
`artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json`.

Regenerate with the `dump_m5_git_certification` example (see the artifact summary
at `artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows.md`).
These files are consumed by `aureline-git` unit tests via `include_str!`; keep
them in sync with the schema and the typed model when the packet shape changes.
