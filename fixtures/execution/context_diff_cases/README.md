# Execution-context diff cases

These fixtures are short, reviewable scenarios that anchor the shared
execution-context snapshot/diff vocabulary frozen in
[`/schemas/execution/context_snapshot.schema.json`](../../../schemas/execution/context_snapshot.schema.json)
and described by
[`/docs/execution/context_inspector_packet.md`](../../../docs/execution/context_inspector_packet.md).

Each fixture is one `context_snapshot_diff_record` rendered as a worked
scenario. The set exists so reviewers can diff a preserved layer, a
changed layer, a degraded layer, and a redaction-limited layer without
reverse-engineering per-surface prose. Snapshots A and B in each case
are snapshots emitted by `task_launch`, `terminal_session_seed`, or
`debug_prep_seed`; the same schema applies regardless of which surface
emitted either side.

## Scope rules

- Fixtures validate against the shared snapshot schema. They carry
  `context_snapshot_schema_version: 1`.
- Fixtures MUST NOT encode raw env bodies, raw command lines, raw paths,
  raw URLs, or raw secret values. Only class labels, frozen tokens,
  opaque ids, hashes, and counts are admissible.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- `value_summary` strings are short reviewer-facing sentences. They
  name tokens and counts; they do not paste configuration bodies.

## Index

| Fixture | Result | Key coverage |
|---|---|---|
| [`exact_match.json`](./exact_match.json) | `preserved` across every layer | two runs of the same task launch produce identical snapshots |
| [`environment_drift.json`](./environment_drift.json) | `changed` on `environment_capsule_ref` and `cache_disposition` | capsule hash changes; cache rejected_drift on side B |
| [`wrong_target.json`](./wrong_target.json) | `changed` on `target_identity` | requested target alias resolves to a different canonical target; route class promotes to tunneled |
| [`policy_limited_context.json`](./policy_limited_context.json) | `changed` on `workset_scope` | side B promotes to `policy_limited_view` with a non-zero hidden_member_count |
| [`degraded_unknown_fields.json`](./degraded_unknown_fields.json) | `degraded_on_b` + `unknown_on_b` + `redaction_limited` | toolchain fallback, trust-blocked activator, and env-delta explanation gated by redaction |

## Coverage contract

The fixture set MUST keep:

- at least one case where every layer is `preserved` (exact match);
- at least one case that changes the environment capsule without
  changing the target;
- at least one case that changes the target and marks the route-
  dependency change;
- at least one case that narrows the scope via
  `policy_limited_view` with a hidden-member count;
- at least one case that records `degraded_on_<side>`,
  `unknown_on_<side>`, and `redaction_limited` statuses without
  over-claiming.

Removing a layer of coverage is a breaking change.
