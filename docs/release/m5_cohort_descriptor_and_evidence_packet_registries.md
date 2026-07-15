# M5 cohort-descriptor and cohort-evidence-packet registries

This lane is the first implement lane over the frozen
[M5 launch-control matrix](./m5_launch_control_contract.md). It turns the *cohort-descriptor* grammar (how a
widening cohort declares the exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
known limits, rollback target, and diagnostics posture it is auditable by) and the *cohort-evidence-packet*
grammar (how a launch-bearing lane proves which cohort evidence — dogfood-ring telemetry, current rehearsal
cadence, or an explicit go/no-go signoff — backs it, keeping partner / public support language and known-limits
packets bound to that proof rather than to hand-edited prose) into registry resolvers that produce export-safe,
honest projections, so the shiproom, release-center, executive-steering, program-governance, diagnostics, docs,
CLI, support, and public-proof surfaces resolve one canonical cohort truth instead of a per-cohort, hand-copied
mailing list. The cohort descriptor and the cohort-evidence packet are separated in runtime and serialized
state: the cohort archetype, exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
known limits, rollback target, and diagnostics posture live on the descriptor, while the resolved cohort
identity, known-limits ledger, rollback-target reference, rehearsal-currency state, readiness-signoff state,
cohort-bound support-language reference, and last widening revision live on the cohort-evidence packet, and a
cohort's rollback and diagnostics posture stays preserved so a cohort never widens without it.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_cohort_descriptor_and_evidence_packet_registries` (the
  authoritative validator).
- **Combined schema:** `schemas/program/m5-cohort-descriptor-and-evidence-packet-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-cohort-descriptor.schema.json`](../../schemas/program/m5-cohort-descriptor.schema.json)
  and
  [`schemas/program/m5-cohort-evidence-packet.schema.json`](../../schemas/program/m5-cohort-evidence-packet.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-cohort-descriptor-and-evidence-packet-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-cohort-descriptor-and-evidence-packet-registries/`
  (`cohort_descriptor_beta_narrowed.json`, `cohort_evidence_preview_narrowed.json`).

## Two registries

1. **Cohort descriptor** (`resolve_cohort_descriptor_entry`) — publishes one typed cohort-descriptor object per
   cohort: the cohort archetype and canonical archetype mode, the exact repo / archetype rows, the bundle IDs,
   the install topology, the toolchain envelope, the known limits, the rollback target, and the diagnostics
   posture. A clean entry names a canonical registry token, a classified cohort archetype, and a launch-control
   role, covers the canonical / accessible / audit resolution forms, publishes a complete object, preserves its
   rollback and diagnostics posture before widening, and keeps a public-facing cohort's support language matched
   to cohort proof. Otherwise it degrades honestly — a cohort widening without a preserved rollback target and
   diagnostics posture, or a public-facing cohort running its support language ahead of proof, degrades to
   `descriptor_lets_cohort_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker reason
   a widen-without-rollback attempt must surface.
2. **Cohort-evidence packet** (`resolve_cohort_evidence_packet_entry`) — keeps the cohort evidence honest. A
   clean entry names a classified evidence scope and provides the complete cohort-identity / known-limits-ledger
   / rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
   evidence object; an evidence packet that would run partner / public support language ahead of cohort proof,
   hide the cohort evidence, or let a known-limits gap masquerade as covered degrades to
   `cohort_evidence_runs_support_ahead_of_proof_or_drops_cohort_evidence`.

## Per-entry cohort reference

The cohort archetype carries its canonical mode, and the resolver publishes the full descriptor object, so the
registry — never a hand-copied per-cohort mailing list — is the single source of truth.
`cohort_descriptor_object_is_complete` rejects an object missing any field,
`cohort_preserves_rollback_and_diagnostics_before_widening` rejects an unpreserved rollback / diagnostics
posture or partner / public support language running ahead of proof, and `cohort_evidence_stays_honest` rejects
an evidence packet that has run support language ahead of cohort proof.

| cohort archetype | archetype mode | exact repo / archetype rows | rollback target | diagnostics posture |
| --- | --- | --- | --- | --- |
| dogfood core-team canary | dogfood_core_team_canary_archetype | `repo.rows.core-team-canary-archetypes` | `rollback.target.canary-previous-stable` | `diagnostics.posture.full-telemetry` |
| migration alpha | migration_alpha_archetype | `repo.rows.migration-alpha-archetypes` | `rollback.target.migration-previous-toolchain` | `diagnostics.posture.migration-telemetry` |
| certified archetype | certified_archetype_archetype | `repo.rows.certified-archetype-archetypes` | `rollback.target.certified-previous-stable` | `diagnostics.posture.certified-telemetry` |

A widen-without-rollback attempt degrades to
`descriptor_lets_cohort_widen_without_rollback_or_runs_support_ahead_of_proof`, an incomplete object degrades to
`cohort_descriptor_object_incomplete`, and support language run ahead of proof degrades to
`cohort_evidence_runs_support_ahead_of_proof_or_drops_cohort_evidence`, so a widen-without-rollback attempt, an
incomplete object, or support language running ahead of proof can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.** Clean
  descriptor entries cover the canonical dogfood core-team canary / migration alpha / extension-author /
  design-partner preview / public preview / certified-archetype archetypes and the first release-center /
  shiproom / executive-steering / program-governance / support surfaces, an object-incomplete example degrades,
  and no clean descriptor entry published an incomplete object.
- **Cohort packets preserve rollback and diagnostics posture before widening.** A widen-without-rollback example
  and an unbound example degrade, a clean bounded descriptor entry is present, and no clean entry is unbounded or
  unbound.
- **Claim publication can prove which cohort evidence backs each launch-bearing lane.** Clean cohort-evidence
  entries cover the dogfood-ring / rehearsal-currency / go-no-go-signoff evidence scopes with full
  resolution-form coverage while providing the complete evidence object — the resolved cohort identity and the
  cohort-bound support-language reference — and an evidence packet that would run support language ahead of
  cohort proof or drop cohort evidence degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_cohort_descriptor_and_evidence_packet_registries -- support-export
cargo run -p aureline-ui --example dump_m5_cohort_descriptor_and_evidence_packet_registries -- csv
cargo run -p aureline-ui --example dump_m5_cohort_descriptor_and_evidence_packet_registries -- report
cargo run -p aureline-ui --example dump_m5_cohort_descriptor_and_evidence_packet_registries -- cohort-descriptor-table
cargo run -p aureline-ui --example dump_m5_cohort_descriptor_and_evidence_packet_registries -- fixture-cohort-descriptor-beta-narrowed
cargo run -p aureline-ui --example dump_m5_cohort_descriptor_and_evidence_packet_registries -- fixture-cohort-evidence-preview-narrowed
```
