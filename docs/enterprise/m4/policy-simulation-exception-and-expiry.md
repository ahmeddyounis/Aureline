# Policy Simulation, Exception Preview, Approval History, and Expiry Governance

This stable packet turns policy simulation and remembered-approval state into
one reviewable product contract. Desktop trust/admin surfaces, CLI/headless
explain output, support exports, and managed-admin handoff packets all consume
`aureline_policy::policy_simulation_and_expiry` rather than rebuilding their
own waiver or approval-history state.

## Contract

The stable claim holds only when the packet can show all of the following from
typed records:

1. **Policy simulation view** — changed keys or feature areas, previous and
   simulated values, user-visible consequences, affected surfaces, degraded
   modes, and stale/offline notes before enforcement changes land.
2. **Exception or waiver preview sheet** — exact bypass scope, owner or
   approver, reason, mitigation, evidence links, expiry target, and fallback
   behavior when the exception lapses.
3. **Approval history row** — remembered decisions bounded to actor, object,
   action family, environment, and time horizon, with revoke/open-details
   actions and reapproval triggers for target, policy, version, and authority
   drift.
4. **Policy diff and impact summary** — changed feature area, previous versus
   simulated value, affected surfaces, degraded-mode consequences, and
   stale/offline notes.
5. **Expiry banner** — what expires, exact UTC expiry time, relative review
   label, consequence on expiry, and renewal or review action.
6. **Cross-surface review packet** — one packet linking policy diff,
   simulation outcome, approval/waiver owners, expiry banners, chronology, and
   reapproval triggers for desktop, CLI/headless, and admin/support handoff.

## Guardrails

- Source `PolicySimulationBetaPage` defects narrow the packet to review.
- Raw private material in the source policy simulation page withdraws the
  packet immediately.
- High-risk remembered decisions must be expiry-bounded. Connected-provider
  mutations, AI apply, settings writes, records lifecycle actions, remote,
  provider-backed, networked, destructive, or secret-bearing actions are never
  indefinite by default.
- Approval rows must expose target, policy, version, and authority drift
  triggers. Drift does not silently carry remembered authority forward.
- Exceptions and waivers must have visible expiry banners and explicit lapse
  fallback behavior.

## Boundary

The packet is export-safe. It carries opaque refs, timestamps, closed-vocabulary
tokens, user-visible consequence labels, counts, and audit/chronology refs. It
does not export raw policy bundle bodies, raw rule text, raw identities, raw
hostnames, raw file paths, credentials, secret material, or raw exception
justification text.

## Canonical Sources

| Slice | Canonical source |
| --- | --- |
| Runtime module | `crates/aureline-policy/src/policy_simulation_and_expiry/` |
| Schema | `schemas/policy/policy-simulation-exception-and-expiry.schema.json` |
| Fixtures | `fixtures/enterprise/m4/policy-simulation-exception-and-expiry/` |
| Artifact summary | `artifacts/enterprise/m4/policy-simulation-exception-and-expiry.md` |
| Source beta page | `aureline_policy::simulation` |

## Verify

```bash
cargo fmt -p aureline-policy
cargo test -p aureline-policy policy_simulation_and_expiry
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- summary
```
