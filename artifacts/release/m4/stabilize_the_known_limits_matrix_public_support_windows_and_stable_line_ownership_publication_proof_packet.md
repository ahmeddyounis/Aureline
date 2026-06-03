# Proof packet: M04-182 — Stabilize the known-limits matrix, public support windows, and stable-line ownership publication

**Register**: `stabilize:m4:known_limits_support_windows_ownership`  
**As of**: 2026-06-03  
**Decision**: HOLD

## Summary

| Metric | Value |
|--------|-------|
| Total entries | 12 |
| Entries published stable | 7 |
| Entries narrowed below cutline | 5 |
| Entries on active waiver | 2 |
| Release-blocking total | 7 |
| Release-blocking narrowed | 0 |
| Known-limit entries | 4 |
| Support-window entries | 4 |
| Ownership entries | 4 |
| Packets current | 8 |
| Packets breached | 1 |
| Packets missing | 3 |
| Rules firing | 3 |

## Rows

- **kl:extension_api** — Extension API known limits (`known_limit`) → `stable` 🟢
  - Claim ref: manifest_entry:extension_platform (`stable`)
  - Owner: platform_team (signed: True)

- **kl:ai_assist** — AI Assist known limits (`known_limit`) → `stable` 🟢
  - Claim ref: manifest_entry:ai_intelligence (`stable`)
  - Owner: ai_team (signed: True)

- **kl:remote_dev** — Remote development known limits (`known_limit`) → `beta` 🔴
  - Gap reasons: claim_label_narrowed
  - Claim ref: manifest_entry:remote_workspace (`beta`)
  - Owner: infra_team (signed: False)

- **sw:core_lts** — Core LTS support window (`public_support_window`) → `lts` 🟢
  - Claim ref: manifest_entry:core_editor (`lts`)
  - Owner: release_team (signed: True)

- **sw:stable_monthly** — Stable monthly support window (`public_support_window`) → `stable` 🟢
  - Claim ref: manifest_entry:stable_monthly (`stable`)
  - Owner: release_team (signed: True)

- **sw:preview_weekly** — Preview weekly support window (`public_support_window`) → `preview` 🔴
  - Gap reasons: support_window_expired, proof_packet_freshness_breached
  - Claim ref: manifest_entry:preview_weekly (`preview`)
  - Owner: release_team (signed: False)

- **so:core_editor_owner** — Core editor stable-line ownership (`stable_line_ownership`) → `lts` 🟢
  - Claim ref: manifest_entry:core_editor (`lts`)
  - Owner: platform_team (signed: True)

- **so:extension_platform_owner** — Extension platform stable-line ownership (`stable_line_ownership`) → `stable` 🟢
  - Claim ref: manifest_entry:extension_platform (`stable`)
  - Owner: platform_team (signed: True)

- **so:ai_intelligence_owner** — AI intelligence stable-line ownership (`stable_line_ownership`) → `beta` 🔴
  - Gap reasons: ownership_unpublished, proof_packet_missing
  - Claim ref: manifest_entry:ai_intelligence (`stable`)
  - Owner: ai_team (signed: False)

- **kl:browser_runtime** — Browser runtime known limits (`known_limit`) → `preview` 🔴
  - Gap reasons: evidence_incomplete, proof_packet_missing
  - Claim ref: manifest_entry:browser_runtime (`stable`)
  - Owner: security_team (signed: False)

- **sw:security_only** — Security-only support window (`public_support_window`) → `stable` 🟢
  - Claim ref: manifest_entry:security_maintenance (`stable`)
  - Owner: security_team (signed: True)

- **so:remote_workspace_owner** — Remote workspace stable-line ownership (`stable_line_ownership`) → `beta` 🔴
  - Gap reasons: claim_label_narrowed
  - Claim ref: manifest_entry:remote_workspace (`beta`)
  - Owner: infra_team (signed: False)

## Publication verdict

- **Gate**: `m4_stable_known_limits_support_windows_ownership`
- **Decision**: HOLD
- **Blocking rules**: rule:evidence_incomplete, rule:packet_missing, rule:ownership_missing
- **Blocking entries**: kl:browser_runtime, so:ai_intelligence_owner
- **Rationale**: Publication proceeds with some narrowed rows; no blocking rules are firing.
