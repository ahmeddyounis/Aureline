# Infrastructure surface qualification

Infrastructure, DevOps/SRE, and incident-adjacent depth is quoted from:

[`artifacts/infra/infrastructure-surface-qualification/support_export.json`](../../artifacts/infra/infrastructure-surface-qualification/support_export.json)

Use each row directly:

- `displayed_posture` is the exact posture docs/help, support, and public-truth
  consumers render.
- `narrow_reasons` explains why a row narrowed instead of inheriting adjacent
  infrastructure depth.
- `packet_refs` shows which checked packet or fixture currently backs the row.

`stable_qualified` applies only when the row's required relationship,
target-context, live-counterpart, plan-viewer, handoff-boundary, and
export-parity proof stays current. Missing relationship proof narrows to
`file_only`; missing target/live/plan proof narrows to `inspect_only`; missing
handoff-boundary proof narrows to `handoff_only`; missing export parity blocks
promotion.
