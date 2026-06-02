# Hot-path performance budget stabilization — M4 milestone note

This is the milestone-level note for the hot-path performance budget stabilization lane that binds startup, restore, quick open, typing, scrolling, search, and Git status to published p50/p95 budgets for the M4 stable line.

The authoritative typed consumer is `aureline_release::stabilize_hot_path_performance_against_published_budgets_for`.
The canonical checked-in artifact is `artifacts/release/stabilize_hot_path_performance_against_published_budgets_for.json`.
The proof packet lives at `artifacts/release/m4/stabilize_hot_path_performance_against_published_budgets_for_proof_packet.md`.
The fixture corpus lives under `fixtures/release/stabilize_hot_path_performance_against_published_budgets_for/`.
The validation capture lives at `artifacts/release/captures/stabilize_hot_path_performance_against_published_budgets_for_validation_capture.json`.

## Scope

This lane governs the seven hot paths that users experience most directly in the editor:

| Hot path | Metric | Published p50 | Published p95 |
|----------|--------|--------------|---------------|
| Startup | `metric:startup_time_ms` | 800 ms | 2000 ms |
| Restore | `metric:restore_time_ms` | 1200 ms | 3000 ms |
| Quick open | `metric:quick_open_latency_ms` | 100 ms | 250 ms |
| Typing | `metric:typing_latency_ms` | 8 ms | 20 ms |
| Scrolling | `metric:scroll_frame_time_ms` | 16 ms | 33 ms |
| Search | `metric:search_results_latency_ms` | 200 ms | 600 ms |
| Git status | `metric:git_status_latency_ms` | 100 ms | 300 ms |

Each budget is grounded in a benchmark-lab trace with attached corpus metadata. The register protects published p50/p95 ceilings: a measured regression beyond the published budget automatically narrows the path below the stable cutline. A budget may be held on an active waiver only when the threshold is intentionally tightened and the measured numbers have not yet caught up.

## Downgrade behavior

Any row that loses freshness, certification, or proof narrows automatically instead of lingering as an unearned stable promise:

- **Budget regression** (`budget_regressed`): the measured p50 or p95 exceeds the published ceiling.
- **Stale proof packet** (`proof_packet_freshness_breached`): the benchmark-lab trace is older than the freshness SLO.
- **Missing proof packet** (`proof_packet_missing`): no trace has been captured.
- **Missing corpus or trace** (`corpus_metadata_missing`): the published numbers cannot be traced to a run.
- **Expired waiver** (`waiver_expired`): the tightened-budget waiver passed its expiry date.
- **Missing owner sign-off** (`owner_signoff_missing`): the performance council has not signed.
- **Narrowed backing claim** (`claim_label_narrowed`): the stable claim manifest entry this path backs is itself below the cutline.

## Verification

```
cargo test -p aureline-release
```

Run this from the repository root to validate the typed model against the checked-in artifact and fixtures.
