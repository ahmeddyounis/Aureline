# Precedence inspection — why did this value win?

Every major M5 resolver — the toolchain / execution context, the effective setting, the policy, the
credential, and the route / target — picks a **winning value** out of several candidates. A
**precedence inspector** shows that decision in full: the value that won, the candidates it
overshadowed (and why they lost), where the winner came from, which surfaces the decision affects,
and what it would take to change it. The precedence-inspector registry is the one authoritative
contract for that supportability UX, so a silent fallback or a hidden override is impossible to miss
and route, credential, setting, policy, and toolchain precedence all share one grammar.

- Typed model + gate: `aureline-support` crate, `m5_precedence_inspector`
- Packet: `artifacts/support/m5/m5-precedence-inspector.json`
- Reviewer artifact: `artifacts/support/m5/m5-precedence-inspector.md`
- Schema: `schemas/support/m5-precedence-inspector.schema.json`
- Fixtures: `fixtures/support/m5/m5-precedence-inspector/`
- Shiproom review packet:
  `artifacts/shiproom/m5-precedence-inspector-review-packet/precedence_inspector_review_packet.md`

## Why this packet exists

M5 adds many more resolver families, and each one resolves silently: a value wins, the others
disappear, and a blocked user is left guessing whether a fallback fired, an override applied, a
target drifted, or a policy lock blocked their change. This packet projects each resolver's truth — by
reference, never re-deriving it — into an inspector that makes the *whole* decision legible: not only
what won, but what lost and why.

It is a **projection layer**, not a new truth model. Every candidate carries a `descriptor_ref` and
every inspector a `source_of_truth_ref` that point at the existing effective-setting,
execution-context, policy, credential, and route-origin objects.

## The inspector

Each inspector explains one resolver decision and shows:

- the **winning value** and its **source class** — one unified precedence vocabulary
  (`policy_scoped`, `project_scoped`, `user_scoped`, `system_scoped`, `fallback_scoped`) so a policy
  lock, a workspace override, a user default, a system-detected value, and a last-resort fallback rank
  the same way across families;
- the **candidates** the winner overshadowed, each with a disposition (`winner`, `overshadowed`,
  `unavailable`, `blocked`, `conflicting`), a reason, and its own lineage ref;
- the **affected surfaces** the decision influences, and their count;
- the **policy-lock state** and the **restart-or-reauth posture** needed to change the value;
- a one-step `explain_entrypoint` that opens the inspectable "Why did this value win?" answer, and the
  equivalent **CLI / headless object** id, so the same answer is reachable without the desktop UI.

## The fail-closed precedence gate

An inspector must never present a clean "this value won" chip that hides what lost, why it lost, or
that the win is a silent fallback, a hidden override, a drift, a conflict, or a policy block. Its
published **presentation** is therefore the weaker of two ceilings:

- **Resolution ceiling** — a `resolved` win presents transparently; a `fallback`, `override`, `drift`,
  or `conflict` **narrows** the inspector; a policy-lock `blocked` resolution caps it at **blocked**.
- **Disclosure ceiling** — `plain_values` present transparently; `metadata_only` values (secret- or
  identity-bearing, shown by class / health / provenance) **narrow** the inspector.

The three published decisions are `transparent`, `narrowed`, and `blocked`. When the gate narrows or
blocks an inspector it records the headline reasons (`silent_fallback_eliminated`, `hidden_override`,
`source_drift`, `unreconciled_conflict`, `policy_lock_blocked`, `redaction_boundary`) and the
resolution path (`restore_preferred_source`, `review_override`, `request_policy_change`,
`reauthenticate`, `reconnect_source`, `reconcile_conflict`, or `none`). A resolution that needs the
user to act always names a real path, a caveat, and the source that drove the downgrade; a blocked
inspector warns before the value is used; and a transparent inspector must be whole — a clean
resolution, plain values, nothing flagging it — while still showing the lower-precedence value it
overshadowed.

The gate also enforces real precedence semantics: a clean win or an override must genuinely
out-precede every candidate it overshadowed (a lower-precedence value that wins without a fallback,
drift, or conflict explanation is exactly the silent fallback this packet catches), and a fallback's
winner must be out-precedence-ed by an *unavailable* higher candidate, so nothing falls back without
cause. The recorded presentation, reasons, path, and blocked-before-use flag are all recomputed and
validated against the gate (`M5PrecedenceInspectors::validate()`), so a clean win can never be
asserted by hand over a degraded or redacted resolution.

## Redaction and safety

Precedence is made transparent **without** dumping raw secrets or hidden policy payloads. A
secret- or identity-bearing family (credentials) is shown `metadata_only`: each candidate is a class /
health / provenance label, never a raw value, and the `redaction_boundary` reason flags the narrowing.
Every inspector attests `raw_material_excluded`, and the policy family shows the effective values it
resolves while the hidden policy payload is never included.

## One precedence truth across surfaces

Five consumer surfaces bind to this one registry: the active run-capable surface, the Support Center,
the support export, the issue-report packet, and the CLI / headless inspect path. Each binding must
ingest the registry, preserve its precedence vocabulary and object ids verbatim, and narrow with it —
so the same winning-value, overshadowed-candidate, and affected-surface truth appears across desktop,
Support Center, support packets, and CLI, and an inspector narrowed or blocked here cannot read as a
clean "this value won" chip on a downstream surface.

This registry is a supportability surface for explaining a resolution; it does not change which value
a resolver picks.
