# Shiproom review packet — precedence inspection

This packet is the shiproom- and release-center-facing view of the precedence-inspector registry. It
does not maintain its own summary: the claim scope below is read from the canonical packet and narrows
automatically when an inspector's resolution is a fallback, override, drift, conflict, policy block, or
redaction boundary.

## Canonical inputs

- Packet: `artifacts/support/m5/m5-precedence-inspector.json`
- Reviewer artifact: `artifacts/support/m5/m5-precedence-inspector.md`
- Schema: `schemas/support/m5-precedence-inspector.schema.json`
- Companion doc: `docs/help/support/m5-precedence-inspection.md`
- Fixtures: `fixtures/support/m5/m5-precedence-inspector/`
- Typed model + gate: `aureline-support` crate, `m5_precedence_inspector`

- Claim publishable: **yes**
- Transparent inspectors: `1`
- Narrowed inspectors: `5`
- Blocked inspectors: `1`
- Families covered: `5`

## Claim scope

| Inspector | Family | Resolution | Presentation | Resolution path |
| --- | --- | --- | --- | --- |
| `toolchain-resolved` | toolchain | resolved | **transparent** | none |
| `toolchain-fallback` | toolchain | fallback | **narrowed** | restore_preferred_source |
| `setting-workspace-over-user` | setting | override | **narrowed** | review_override |
| `policy-lock-blocked` | policy | blocked | **blocked** | request_policy_change |
| `credential-class-change` | credential | drift | **narrowed** | reauthenticate |
| `route-target-drift` | route | drift | **narrowed** | reconnect_source |
| `route-conflict` | route | conflict | **narrowed** | reconcile_conflict |

## Sign-off gate

Promotion of the precedence-inspector registry holds unless all of the following are true on the
current packet (`M5PrecedenceInspectors::validate()` returns no violations):

1. Every resolver family carries at least one inspector; inspector ids are unique.
2. Every inspector shows the winning value, at least one overshadowed candidate, at least one affected
   surface, its one-step explain entry, its CLI / headless equivalent, and its source-of-truth lineage.
3. Every inspector's `presentation`, `downgrade_reasons`, `resolution_path`, and `blocked_before_use`
   flag equal the recomputed fail-closed gate.
4. The winner out-precedes its overshadowed candidates, a fallback is forced by an unavailable higher
   candidate, and a conflict declares no winner — no lower-precedence value wins silently.
5. No raw secret material or hidden policy payload is carried; identity-bearing values are shown by
   class / health / provenance only.
6. The five consumer bindings are present and reuse this packet's precedence vocabulary and object ids.

This packet projects from the canonical precedence-inspector truth source; it does not restate the
precedence vocabulary in its own words.
