# Example: browser-runtime shell proposal (rejected)

## Proposal summary

Implement the flagship desktop shell on an Electron/Chromium runtime to accelerate UI iteration and reuse web tooling.

## Rejection anchor

- Rejected pattern: `rp.flagship_shell_browser_runtime`
- Ledger: `artifacts/architecture/rejected_pattern_rows.yaml`

## Why this is rejected (short)

The browser-runtime shell posture conflicts with the native shell/renderer performance and accessibility-control thesis and reintroduces the explicitly rejected browser-first flagship pattern.

## Governing refs (starting points)

- `docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`
- `.t2/docs/Aureline_Technical_Architecture_Document.md` (decision summary + rejected patterns)

## What would be required to reopen

Name and satisfy a revisit trigger:

- Trigger: `rt.protected_performance_ceiling_forces_stack_swap`
  - Required artifacts: `benchmark_report` + `adr`
  - Forums: `performance_council` + `architecture_council`

Concrete minimum packet expectations:

- Benchmark report demonstrates protected-path budgets on the agreed corpus/hardware under the browser-runtime shell with the same measurement hooks used for the native posture.
- ADR explains why the native shell posture cannot meet requirements, and how accessibility parity, input latency, and failure isolation are preserved under the new stack.

