# Proof packet: status-bar seed for target, profile, trust, encoding, and background state

Purpose: anchor proof captures for the live status-bar projection that
exposes durable state items for target, profile, trust, encoding, and
background work. The bar reads upstream truth (execution context, profile
identity, workspace trust, source-fidelity record, activity center) and
renders one row per kind, ordered by the priority ladder, with degraded
chips that surface when an upstream truth source flips to a degraded
posture.

Reviewer landing page: [`docs/ux/status_bar_seed.md`](../../../docs/ux/status_bar_seed.md).

Canonical sources:

- Crate: `crates/aureline-shell/src/status_bar/mod.rs`
  - `StatusBarSnapshot::project` — pure projection of upstream truth.
  - `StatusBarItemKind` — frozen item kinds (target, profile, trust,
    encoding, background_state) with their stable slot keys, default
    classes, primary command ids, and inspector targets.
  - `StatusItemClass` — priority ladder bands (recovery_critical,
    active_context_truth, ongoing_work, ambient_metadata).
- Frozen contract: `docs/ux/status_bar_contract.md` and
  `schemas/ux/status_item.schema.json`.
- Seed fixtures: `fixtures/ux/status_bar_cases/`.
- Integration test: `crates/aureline-shell/tests/status_bar_state_cases.rs`.

Protected walk: open a local workspace and a terminal; project a snapshot
with `StatusBarSnapshot::project` and confirm one row per item kind, all
synchronized with the upstream truth. No row carries a degraded chip and
no row is promoted to recovery-critical. Evidence:
`fixtures/ux/status_bar_cases/protected_walk_local_workspace.json` and
`crates/aureline-shell/src/status_bar/mod.rs::tests::protected_walk_renders_truthful_active_row_per_kind`.

Failure drill: flip workspace trust to `Restricted`. The trust row must
promote to recovery-critical, surface the `PolicyBlocked` chip, and lead
the bar order, while every other row continues to mirror its upstream
truth. Evidence:
`fixtures/ux/status_bar_cases/failure_drill_restricted_trust.json` and
`crates/aureline-shell/src/status_bar/mod.rs::tests::failure_drill_restricted_trust_promotes_recovery_critical_label`.

Multi-degraded coverage: a remote SSH target unreachable, sync transport
offline, and the active buffer has mixed line endings. The status bar
surfaces three degraded chips (one recovery-critical) without dropping any
ambient row or inventing additional recovery-critical promotions.
Evidence: `fixtures/ux/status_bar_cases/multi_degraded_remote_offline.json`.

Validation commands:

```
cargo test -p aureline-shell --lib status_bar
cargo test -p aureline-shell --test status_bar_state_cases
```

Evidence storage:

- Crate sources: `crates/aureline-shell/src/status_bar/mod.rs`
- Reviewer doc: `docs/ux/status_bar_seed.md`
- Frozen contract: `docs/ux/status_bar_contract.md`,
  `schemas/ux/status_item.schema.json`
- Seed fixtures: `fixtures/ux/status_bar_cases/`
- Integration test: `crates/aureline-shell/tests/status_bar_state_cases.rs`
