# Companion Scope And Handoff Audit

Audit source:
[`crates/aureline-shell/src/companion_handoff/mod.rs`](../../../crates/aureline-shell/src/companion_handoff/mod.rs)

Fixture source:
[`fixtures/ux/m3/companion_scope/page.json`](../../../fixtures/ux/m3/companion_scope/page.json)

Shared contract ref: `shell:companion_scope_and_handoff_beta:v1`

## Scope Matrix

| Workflow | Client label | Freshness | Companion action | Desktop-owned handoff |
| --- | --- | --- | --- | --- |
| Review triage | `comment_capable` | `live_authoritative_fresh_within_grace` | comment/ack through canonical event id | protected approval, broad mutation |
| Docs and help | `review_only` | `warm_snapshot_within_grace` | read-only docs inspection | optional desktop reopen for workspace context |
| Light remote edit | `light_edit` | `live_authoritative_fresh_within_grace` | bounded edit through desktop command policy | deep local editing, native runtime depth, unmanaged secret entry |
| Remote session join | `follow_capable` | `live_authoritative_fresh_within_grace` | follow and request scoped control | broad control and native-depth workflow |
| CI status | `status_only` | `stale_snapshot_beyond_grace` | inspect stale CI status | rerun, broad mutation, high-risk publish |
| Incident awareness | `light_incident_tooling` | `offline_snapshot_no_refresh_path` | acknowledge/comment via canonical event id | sensitive admin action, high-risk publish, unmanaged secret entry |

## Guardrail Results

| Guardrail | Result |
| --- | --- |
| No companion row claims desktop parity | Pass |
| No companion row allows unmanaged secret entry | Pass |
| No companion row allows deep local project editing | Pass |
| Stale, warm, and offline rows show non-live labels | Pass |
| Every row shows client scope and target identity | Pass |
| Every row shows read-only or desktop-handoff state | Pass |
| Protected approval, admin, publish, and broad mutation stay desktop-owned | Pass |
| Authority-bearing actions reuse desktop step-up or approval semantics | Pass |
| Companion fanout inherits device policy and quiet-hours policy | Pass |
| Support export preserves origin surface, approval owner, handoff destination, and canonical event id | Pass |

## Evidence

```sh
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- validate
cargo test -p aureline-shell --lib companion_handoff
cargo test -p aureline-shell --test companion_scope_beta_fixtures
```

Negative drills:

- `drill_stale_label_missing.json` removes the stale/offline label from the CI
  row and must emit `stale_offline_label_missing`.
- `drill_companion_owns_protected_approval.json` lets the review companion own
  protected approval and must emit `companion_owns_protected_approval`.
