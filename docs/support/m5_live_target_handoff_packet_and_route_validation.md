# M5 Live-Target Handoff Packet and Route Validation

The live-target-handoff lane (row **M05-1251**, batch **B149**) makes "open current live object" a
**reviewable, validated handoff** rather than a hidden jump from non-live evidence into live mutable state. It
is the implement lane over the five non-live-evidence object classes frozen in the
[historical-reference matrix](../../artifacts/program/m5-historical-reference-matrix.md): retirement snapshot,
captured support / export evidence, archived runbook packet, imported / offline route evidence, and
review / incident snapshot.

Where the archive-viewer lane proves how a preserved snapshot is *shown* as non-live and the compare-flow lane
proves how it is *compared* against its live target, this lane proves how a preserved snapshot is *handed off*
to its current live object: every pivot carries a typed, versioned handoff packet and either completes only
after every precondition clears, or reports the exact blocker and falls back to a satisfy-prerequisite or
metadata-only exit — never a dead end, never a silent authority widen.

## Canonical source

- Boundary schema: `schemas/program/m5-live-target-handoff-packet-and-route-validation.schema.json`
- Reused domain schema: `schemas/program/m5-live-target-handoff.schema.json` (matrix-minted)
- Support export: `artifacts/support/m5-live-target-handoff/support_export.json`
- Matrix CSV: `artifacts/support/m5-live-target-handoff/matrix.csv`
- Markdown summary: `artifacts/support/m5-live-target-handoff/summary.md`
- Narrowed fixtures: `fixtures/recovery/m5-live-target-handoff/`

Everything is minted from the seed builder in
`crates/aureline-ui/src/m5_live_target_handoff_packet_and_route_validation/` through the example
`dump_m5_live_target_handoff_packet_and_route_validation`; the checked-in artifacts are never hand-edited.

## The typed handoff packet

Each binding carries a `LiveTargetHandoffRequest`:

- `source_snapshot_id` — the historical snapshot the handoff pivots from.
- `target_identity` — the id, label, and kind of the current live object.
- `required_route_class` — `in_process_workspace`, `remote_managed_service`, `companion_browser_surface`, or
  `cli_reopen_path`.
- `required_trust_posture` — `trusted_current_session`, `needs_trust_revalidation`, or `untrusted`.
- `required_auth_prerequisites` — named prerequisites (session authenticated, fresh credential within TTL,
  approval on record, authority scope confirmed) — **controlled tokens, never embedded secrets or ambient
  credentials.**
- `requested_authority_class` and `direct_open_authority_class` — the authority the handoff would open at and
  the authority a direct open would grant; the requested authority may never exceed the direct one.
- `precondition_check` — the five preconditions.
- `fallback_behavior` — what happens when the target cannot be reopened live.

## The five preconditions

A handoff completes only when all five clear:

1. `target_exists`
2. `target_in_current_scope` (scope / workset visibility)
3. `route_available` (remote / managed route availability)
4. `trust_posture_satisfied`
5. `auth_and_approval_satisfied`

## Outcomes

- `handoff_cleared` — every precondition cleared; the pivot completes and offers `open_current_live_object` at
  the validated authority.
- `blocked_needs_prerequisite` — a route, trust, or auth / approval prerequisite is unmet; the handoff blocks
  and offers a satisfy-prerequisite-then-retry fallback (it never bypasses the prerequisite).
- `blocked_target_unavailable` — no live target exists or it is outside the current scope; falls back to a
  metadata-only exit.
- `blocked_by_policy` — a policy or lifecycle rule blocks the reopen; falls back to a metadata-only exit.

Each blocked binding names the exact `blocker_reason`, and the reason must be supported by a genuinely failed
precondition.

## Acceptance criteria mapping

- **A seeded snapshot produces a typed packet that either completes safely or names the exact blocker** — the
  `handoff_cleared` outcome completes; the three blocked outcomes each carry an explicit `blocker_note`.
- **A handoff never widens authority** — `requested_authority_class <= direct_open_authority_class` is enforced
  and the `widens_authority_beyond_direct_open` guardrail is always false.
- **Packets are export-safe and auditable without leaking secrets** — auth / approval prerequisites are named
  tokens, no secrets / credentials / private endpoints appear, and any actual elevation is delegated to a
  separate, reviewed `ReviewedAuthorityHandoff` path. This lane defines the typed handoff and its validation
  checks; it never bypasses approval, trust, or auth refresh.

## Guardrails (row invariants, one per binding)

- `historical_side_mutation_blocked` — MUST be true.
- `reopens_live_target_without_validating_identity_trust_route_and_authority` — MUST be false.
- `widens_authority_beyond_direct_open` — MUST be false.
- `dead_ends_when_target_unavailable` — MUST be false.
- `leaks_secret_or_ambient_credential` — MUST be false.
- `presents_snapshot_as_current_live_object` — MUST be false.

## Regenerating

```text
cargo run -p aureline-ui --example dump_m5_live_target_handoff_packet_and_route_validation -- support-export
cargo run -p aureline-ui --example dump_m5_live_target_handoff_packet_and_route_validation -- csv
cargo run -p aureline-ui --example dump_m5_live_target_handoff_packet_and_route_validation -- report
cargo run -p aureline-ui --example dump_m5_live_target_handoff_packet_and_route_validation -- fixture-blocked-target-narrowed
cargo run -p aureline-ui --example dump_m5_live_target_handoff_packet_and_route_validation -- fixture-needs-prerequisite-narrowed
cargo run -p aureline-ui --example dump_m5_live_target_handoff_packet_and_route_validation -- validate
```
