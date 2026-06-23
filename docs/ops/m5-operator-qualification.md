# Operator-surface qualification packet

One certification packet that binds the M5 operator-surface truth sources into a
per-family claim verdict and **auto-narrows** a claimed operator family when its
ownership, freshness, or continuity proof is stale or failing.

The product claims a set of operator surfaces — operational overview boards,
triage inboxes, action plans, evidence handoff bundles, shift digests,
service-ownership / on-call strips, runbook-step cards, maintenance / read-only /
drain notices, failover / migration notices, and embedded provider/auth boundary
states. Each one is governed by its own frozen truth lane. This packet does
**not** re-prove those contracts; it consumes them as proof sources and projects
one verdict that About/help, service-health, compatibility, release automation,
and support export all render instead of restating operator-surface quality
claims by hand.

- Schema: [`schemas/ops/m5-operator-qualification.schema.json`](../../schemas/ops/m5-operator-qualification.schema.json)
- Canonical fixture: [`fixtures/ops/m5-operator-qualification/canonical_packet.json`](../../fixtures/ops/m5-operator-qualification/canonical_packet.json)
- Rust truth source: `crates/aureline-support/src/m5_operator_qualification`
- Headless emitter: `cargo run -p aureline-support --example dump_m5_operator_qualification`
- Freeze gate: `cargo test -p aureline-support --test m5_operator_qualification`

This lane is the certification layer over the operator-surface matrix
([operator-surface matrix](m5-operator-surfaces.md)), [overview boards](m5-operator-boards.md),
[triage inbox](m5-triage-inbox.md), [action plans](m5-action-plans.md),
[handoff bundles / shift digests](m5-handoff-digests.md),
[response panes](m5-response-panes.md), [maintenance / failover windows](m5-maintenance-windows.md),
and [embedded dashboards](m5-embedded-dashboards.md). It reuses their schema
refs and freshness stamps rather than forking new ones.

## Proof dimensions

The closed set of operator-surface claims a family is certified on. Each
dimension cites the upstream lane that proves it and carries a freshness budget
(default 30 days).

| Dimension | Critical | Primary proof source | Governs |
| --- | --- | --- | --- |
| `canonical_matrix_binding` | yes | operator-surface matrix | every family (dashboards/queues resolve through the same frozen matrix) |
| `overview_truth` | no | overview boards | the operational overview board |
| `triage_truth` | no | triage inbox | the triage inbox |
| `action_plan_continuity` | no | action plans | the action plan |
| `handoff_bundle_fidelity` | no | handoff digests | handoff bundle + shift digest |
| `service_ownership` | no | response panes | the service-ownership / on-call strip |
| `runbook_step_authority` | yes | response panes | the runbook-step card |
| `maintenance_failover_communication` | no | maintenance windows | maintenance + failover notices |
| `embedded_boundary_honesty` | yes | embedded dashboards | the embedded provider/auth boundary |

Every family claims the `canonical_matrix_binding` plus the one dimension that
directly governs it. A maintenance notice is never penalized when triage proof
ages out — its claim never promised it.

### Proof state

Each dimension resolves to exactly one state, derived from the upstream lane's
pass state and its capture stamp measured against the packet's evaluation stamp:

- `fresh` — present, passing, and within its freshness budget.
- `stale` — present and was passing, but captured outside its budget (silently
  aged out).
- `failing` — present, but the upstream contract did not hold.
- `missing` — no proof supplied for the dimension.

Only `fresh` keeps a claim fully supported. The freshness derivation lives in
code (`derive_proof_state`), so release automation feeds raw capture stamps and
pass flags and the packet decides the verdict — it never pre-decides it.

## Per-family claim support

For each claimed family, the packet evaluates only the dimensions that family
claims and folds them into one support level:

- `fully_supported` — every claimed dimension is `fresh`.
- `narrowed` — at least one claimed dimension is `stale` or `failing`, but no
  critical safety dimension failed; the claim is degraded and names the
  responsible dimension(s).
- `blocked` — a **critical** dimension (`canonical_matrix_binding`,
  `runbook_step_authority`, or `embedded_boundary_honesty`) failed or is missing;
  the family's claim is withdrawn.

Three dimensions are critical safety properties the operator surface cannot ship
around: a divergent matrix binding (dashboards point at the wrong objects), a
runbook mutating step running without preview/approval, and a webview/auth
surface impersonating a native approval. Every other dimension degrades honestly —
it narrows the claim and discloses the limit rather than blocking the family.

## Frozen invariants

The packet evaluates these over its own data; a structural regression flips an
invariant to `holds = false` rather than silently shipping.

1. `dimension_set_complete` — every proof dimension resolves to exactly one
   global proof.
2. `every_claimed_family_present` — every claimed operator family has a row.
3. `every_family_anchored_to_canonical_matrix` — every family claims the
   canonical-matrix binding, so dashboards and queues resolve through the same
   frozen matrix.
4. `no_fully_supported_family_with_nonfresh_proof` — a family stays fully
   supported only when every dimension it claims is fresh. This is the guardrail
   against silent aging.
5. `every_downgrade_is_named` — every narrowed or blocked family names the
   responsible dimension(s).
6. `critical_failure_blocks_claim` — a failing or missing critical dimension
   blocks every family that claims it.
7. `release_evidence_dimensions_present` — service-ownership, runbook-step
   authority, handoff-bundle fidelity, maintenance/failover communication, and
   embedded-boundary honesty rows are all present.

## Release automation

`project_operator_qualification(evaluated_as_of, proofs)` is the
release-automation entry point. Release automation supplies one `ProofInput` per
dimension carrying the upstream lane's capture stamp, pass flag, and freshness
budget; the projection derives each state and folds the per-family verdicts.
Because staleness is derived from `evaluated_as_of`, re-running the same proof
inputs at a later date automatically downgrades families whose operator-surface
evidence has aged past its budget, even when core incident or support evidence
stays fresh.

`operator_qualification_packet()` is the canonical binding: it reads the real
in-code proof sources (each lane's `all_invariants_hold` and `AS_OF` stamp) and
feeds them to the projection, so the checked-in fixture and the freeze gate pin
the certified state byte-for-byte.

## About / help / service-health / compatibility

These surfaces consume the packet directly. They read `families[].support` for
the per-family badge and `rollup` for the cross-family summary, and render
`operator_qualification_lines` for the export-safe text projection. None of them
restates operator-surface quality claims by hand.

## Verification

```sh
cargo run -p aureline-support --example dump_m5_operator_qualification            # JSON
cargo run -p aureline-support --example dump_m5_operator_qualification -- --lines # human-readable
cargo test -p aureline-support --test m5_operator_qualification
cargo test -p aureline-support m5_operator_qualification
```

## Risks and follow-ups

- **The freshness budget is uniform (30 days) in the canonical binding.** Per
  dimension budgets are already a field on `ProofInput`; tightening them for the
  fastest-moving surfaces (maintenance/failover notices) is a release-automation
  tuning follow-up.
- **The canonical packet binds in-code lane invariants, not live CI evidence
  ages.** The release-automation entry point accepts real capture stamps; wiring
  the live evidence pipeline to feed those stamps (so a missed CI refresh ages a
  dimension out) is incremental follow-up as the evidence store matures.
- **Per-family granularity stops at the dimension.** The packet certifies a
  family at the dimension level; surfacing which exact object inside a dimension
  aged out is left to the underlying operator-surface lane the dimension cites.
