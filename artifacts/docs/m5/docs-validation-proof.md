# Docs Validation Report (example/link validation rows)

- Packet: `packet:m5:docs_validation_report:retry_backoff_release`
- Report: docs validation report: the retry/backoff release docs sweep
- Promotion: `stable` (0 findings)
- Rows: 8 | Degradations: 1

## Rows

- [code_example] `row:readme:config_example_executed_local` (the retry_with_backoff configuration example) — subject `README → Configuration → max_elapsed example#configuration`
  - Mode/outcome: `executed_local` / `executed_pass` — last checked 2026-06-11T22:14:00Z
  - Scope: rustc 1.84.0 on x86_64-unknown-linux-gnu | toolchain `toolchain:rust-1.84.0:x86_64-unknown-linux-gnu` | target `source:crates/aureline-net/src/retry.rs@workspace-rev` | version `exact_build_match`
  - Produced by: `local_example_harness` (exec-context:local-harness:rust-1.84.0) | chips authoritative_live / exact_build_match / local
  - Actions: snippet `open-snippet:docs/guides/retry_with_backoff/README.md#configuration` | failing-source `open-source:repo:crates/aureline-net/src/retry.rs#with_backoff` | compare `compare:readme-config-example:current-source` | suppress true | rerun true
  - Provenance: `first_party_verified` | trace `open-source:repo:crates/aureline-net/src/retry.rs#with_backoff` | suppression `active` | cited true
- [code_example] `row:tutorial:overview_rendered_preview` (the overview pseudo-code block) — subject `Tutorial → Resilient networking → Overview#overview-diagram`
  - Mode/outcome: `rendered` / `rendered_preview_only` — last checked 2026-06-11T22:15:00Z
  - Scope: rendered-preview engine (no execution) | toolchain `render-engine:commonmark-safe` | target `doc:docs/tutorials/resilient-networking.md@workspace-rev` | version `exact_build_match`
  - Produced by: `rendered_preview_engine` (exec-context:render-preview:commonmark-safe) | chips authoritative_live / exact_build_match / local
  - Actions: snippet `open-snippet:docs/tutorials/resilient-networking.md#overview-diagram` | failing-source `open-source:doc:docs/tutorials/resilient-networking.md#overview-diagram` | compare `compare:tutorial-overview:rendered-only` | suppress true | rerun true
  - Provenance: `first_party_verified` | trace `open-snippet:docs/tutorials/resilient-networking.md#overview-diagram` | suppression `active` | cited true
- [code_example] `row:help:builder_syntax_checked` (the builder-API example) — subject `Help → Retry and backoff → Builder API#builder-api`
  - Mode/outcome: `syntax_checked` / `syntax_valid` — last checked 2026-06-11T22:16:00Z
  - Scope: rustc 1.84.0 parse-only (no execution) | toolchain `toolchain:rust-1.84.0:parse-only` | target `source:crates/aureline-net/src/retry.rs@workspace-rev` | version `exact_build_match`
  - Produced by: `syntax_checker` (exec-context:syntax-checker:rust-1.84.0) | chips authoritative_live / exact_build_match / local
  - Actions: snippet `open-snippet:docs/help/retry-and-backoff.md#builder-api` | failing-source `open-source:repo:crates/aureline-net/src/retry.rs#builder` | compare `compare:help-builder:current-source` | suppress true | rerun true
  - Provenance: `first_party_verified` | trace `open-source:repo:crates/aureline-net/src/retry.rs#builder` | suppression `active` | cited true
- [shell_example] `row:guide:cli_executed_remote` (the operations smoke-test command) — subject `Guide → Operations → Smoke test#smoke-test`
  - Mode/outcome: `executed_remote` / `passed_with_warnings` — last checked 2026-06-11T22:18:00Z
  - Scope: managed runner: ubuntu-24.04, aarch64 | toolchain `toolchain:managed-runner:ubuntu-24.04-aarch64` | target `release:next-channel@retry_backoff` | version `compatible_minor_drift`
  - Produced by: `remote_example_runner` (exec-context:remote-runner:ubuntu-24.04-aarch64) | chips warm_cached / compatible_minor_drift / managed
  - Actions: snippet `open-snippet:docs/guides/retry_with_backoff/operations.md#smoke-test` | failing-source `open-source:repo:crates/aureline-net/examples/smoke.rs` | compare `compare:guide-smoke:current-source` | suppress true | rerun true
  - Provenance: `first_party_verified` | trace `open-source:repo:crates/aureline-net/examples/smoke.rs` | suppression `active` | cited true
- [link] `row:tutorial:runbook_broken_link` (the operations runbook link) — subject `Tutorial → Resilient networking → Operations runbook link#operations-runbook`
  - Mode/outcome: `broken_link` / `link_broken` — last checked 2026-06-11T22:20:00Z
  - Scope: link checker against the imported ops pack mirror | toolchain `link-checker:imported-ops-pack-mirror` | target `pack:ops/runbooks@imported-rev` | version `compatible_minor_drift`
  - Produced by: `link_checker` (exec-context:link-checker:imported-ops-pack) | chips warm_cached / compatible_minor_drift / imported_pack
  - Actions: snippet `open-snippet:docs/tutorials/resilient-networking.md#operations-runbook` | failing-source `open-failing-source:pack:ops/runbooks/retry_backoff_runbook.md` | compare `compare:tutorial-runbook-link:current-target` | suppress true | rerun true
  - Provenance: `imported` | trace `open-failing-source:pack:ops/runbooks/retry_backoff_runbook.md` | suppression `active` | cited true
- [code_example] `row:readme:jitter_stale_example` (the with_jitter example) — subject `README → Jitter → with_jitter example#jitter`
  - Mode/outcome: `stale` / `not_run` — last checked 2026-05-30T09:00:00Z
  - Scope: rustc 1.84.0 on x86_64-unknown-linux-gnu (prior run) | toolchain `toolchain:rust-1.84.0:x86_64-unknown-linux-gnu` | target `source:crates/aureline-net/src/retry.rs@prior-rev` | version `incompatible_drift_detected`
  - Produced by: `local_example_harness` (exec-context:local-harness:prior-rev) | chips stale / incompatible_drift_detected / local
  - Actions: snippet `open-snippet:docs/guides/retry_with_backoff/README.md#jitter` | failing-source `open-failing-source:repo:crates/aureline-net/src/retry.rs#with_full_jitter` | compare `compare:readme-jitter:prior-vs-current-source` | suppress true | rerun true
  - Provenance: `stale` | trace `open-failing-source:repo:crates/aureline-net/src/retry.rs#with_full_jitter` | suppression `active` | cited true
- [shell_example] `row:help:network_skipped` (the live-endpoint smoke command) — subject `Help → Retry and backoff → Live endpoint#live-endpoint`
  - Mode/outcome: `skipped` / `not_run` — last checked 2026-06-11T22:22:00Z
  - Scope: local harness without network access | toolchain `toolchain:rust-1.84.0:offline` | target `doc:docs/help/retry-and-backoff.md@workspace-rev` | version `exact_build_match`
  - Produced by: `local_example_harness` (exec-context:local-harness:offline) | chips warm_cached / exact_build_match / local
  - Actions: snippet `open-snippet:docs/help/retry-and-backoff.md#live-endpoint` | failing-source `open-source:doc:docs/help/retry-and-backoff.md#live-endpoint` | compare `compare:help-live-endpoint:current-source` | suppress true | rerun true
  - Provenance: `first_party_verified` | trace `open-snippet:docs/help/retry-and-backoff.md#live-endpoint` | suppression `suppressed` | cited true
- [shell_example] `row:guide:windows_unsupported` (the Windows service registration command) — subject `Guide → Operations → Windows service#windows-service`
  - Mode/outcome: `unsupported` / `not_run` — last checked 2026-06-11T22:24:00Z
  - Scope: mirrored runner pool (linux only) | toolchain `toolchain:mirrored-runner-pool:linux` | target `pack:ops/runbooks@mirrored-rev` | version `unknown_target_build`
  - Produced by: `remote_example_runner` (exec-context:mirrored-runner-pool:linux) | chips warm_cached / unknown_target_build / mirrored_pack
  - Actions: snippet `open-snippet:docs/guides/retry_with_backoff/operations.md#windows-service` | failing-source `open-failing-source:pack:ops/runbooks/windows_service.md` | compare `compare:guide-windows-service:current-source` | suppress true | rerun true
  - Provenance: `mirrored` | trace `open-failing-source:pack:ops/runbooks/windows_service.md` | suppression `active` | cited true

## Degradations

- [link_checker_offline/advisory]: the live link checker was offline for one external host; the broken-link row is served from the last snapshot
