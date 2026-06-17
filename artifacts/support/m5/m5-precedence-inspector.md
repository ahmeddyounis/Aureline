# Precedence inspection — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-precedence-inspector.json`. The full contract and gate semantics live in
`docs/help/support/m5-precedence-inspection.md`; the typed model lives in the `aureline-support` crate
(`m5_precedence_inspector`).

This registry gives every major M5 resolver family a **precedence inspector** that shows the winning
value, the overshadowed candidates and why they lost, the affected surfaces, and the source-of-truth
lineage. A fail-closed precedence gate narrows or blocks any inspector whose win is a silent fallback,
a hidden override, a drift, a conflict, a policy block, or a redaction boundary rather than letting it
present a clean "this value won" chip.

## Inspector roll-up (as of 2026-06-16)

| Inspector | Family | Resolution | Presentation | Resolution path | Posture |
| --- | --- | --- | --- | --- | --- |
| `toolchain-resolved` | toolchain | resolved | **transparent** | none | none |
| `toolchain-fallback` | toolchain | fallback | **narrowed** | restore_preferred_source | restart_required |
| `setting-workspace-over-user` | setting | override | **narrowed** | review_override | none |
| `policy-lock-blocked` | policy | blocked | **blocked** | request_policy_change | none |
| `credential-class-change` | credential | drift | **narrowed** | reauthenticate | reauth_required |
| `route-target-drift` | route | drift | **narrowed** | reconnect_source | reconnect_required |
| `route-conflict` | route | conflict | **narrowed** | reconcile_conflict | none |

One inspector resolves cleanly and transparently (`toolchain-resolved`), proving the gate is not a
blanket flag; five narrow on a fallback, override, drift, conflict, or redaction boundary; and one
(`policy-lock-blocked`) blocks and warns before the value is used. All five resolver families are
covered.

## The cases this corpus proves

### Hidden fallback elimination — `toolchain-fallback`

The preferred, project-pinned interpreter is shown as **unavailable** (its `.venv` is missing); the
system interpreter is named as the **fallback winner** rather than silently substituted. The gate
proves the fallback is *forced* — the unavailable candidate out-precedes the winner — so nothing falls
back without cause. Restoring the preferred source is offered.

### Workspace-over-user — `setting-workspace-over-user`

The workspace value wins and the suppressed **user value is still shown** beside it, so the override is
visible, not hidden. Reviewing the override is offered.

### Policy-over-user — `policy-lock-blocked`

The policy value wins, is **locked**, and the user value is shown as **blocked** rather than silently
dropped. The inspector blocks before use and offers requesting a policy change. The hidden policy
payload is never dumped — only the effective values are shown.

### Credential-class change — `credential-class-change`

The active credential class changed (the personal refresh token gave way to a managed enterprise
session). Both candidates are shown by **class, health, and provenance only** (`metadata_only`), never
as raw values, and the `redaction_boundary` reason flags the narrowing. Re-authentication is the
posture.

### Route / target drift — `route-target-drift`

The active endpoint **drifted** from the configured target; both are shown so the request never
silently hits a different target. Reconnecting to re-resolve the route is offered before sending.

### Unreconciled conflict — `route-conflict`

Two routes claim the request at the **same precedence**; the inspector declares **no winner** and shows
both as conflicting rather than picking one silently. Reconciling the conflict is offered.

## Sign-off gate

Promotion of the precedence-inspector registry holds unless all of the following are true on the
current packet (`M5PrecedenceInspectors::validate()` returns no violations):

1. Every resolver family (toolchain, setting, policy, credential, route) carries at least one
   inspector, and inspector ids are unique.
2. Every inspector shows at least two candidates including the overshadowed ones, names at least one
   affected surface, and carries its one-step explain entry, its CLI / headless equivalent object, and
   its source-of-truth lineage refs.
3. Every inspector's `presentation`, `downgrade_reasons`, `resolution_path`, and `blocked_before_use`
   flag equal the recomputed fail-closed gate — a fallback, override, drift, conflict, policy block, or
   redaction boundary narrows or blocks the inspector automatically.
4. The winner genuinely out-precedes its overshadowed candidates (or the fallback is forced by an
   unavailable higher candidate, or a conflict declares no winner); no lower-precedence value wins
   silently.
5. No raw secret material or hidden policy payload is carried (`raw_material_excluded`); identity-
   bearing values are shown by class / health / provenance only.
6. The five consumer bindings (active-surface, support-center, support-export, issue-report-packet,
   cli-headless) are all present and reuse this packet's precedence vocabulary and object ids.

## Regenerating this packet

This packet is checked in alongside the registry it reviews. When the precedence-inspector registry
changes, update the packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_precedence_inspector
cargo run -p aureline-support --example dump_m5_precedence_inspector
```
