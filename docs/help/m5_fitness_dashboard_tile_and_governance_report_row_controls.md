# M5 fitness dashboard tile and governance report row controls

This contract implements two governed governance-dashboard component families —
the **fitness dashboard tile** and the **governance report row** — frozen in the
[M5 governance-dashboard component matrix](m5_governance_dashboard_components_contract.md)
as one reusable controls packet, so the assurance center, the operator board, the
shiproom, the CLI, and the support/export packet all read the same fitness and
governance truth.

- Boundary schema:
  [`schemas/ui/m5-fitness-governance-report-controls.schema.json`](../../schemas/ui/m5-fitness-governance-report-controls.schema.json)
- Per-component contracts:
  [`schemas/ui/m5-fitness-dashboard-tile.schema.json`](../../schemas/ui/m5-fitness-dashboard-tile.schema.json),
  [`schemas/ui/m5-governance-report-row.schema.json`](../../schemas/ui/m5-governance-report-row.schema.json)
- Proof artifacts:
  [`artifacts/release/m5-fitness-governance-report-controls-proof/`](../../artifacts/release/m5-fitness-governance-report-controls-proof/)
- Protected fixtures:
  [`fixtures/ui/m5-fitness-governance-report-controls/`](../../fixtures/ui/m5-fitness-governance-report-controls/)

The Rust validator in `crates/aureline-release` is the authoritative gate; this doc
describes the intent.

## Fitness dashboard tile

`resolve_fitness_tile` takes one protected metric's identity, declared reading,
threshold state, corpus/profile provenance, evidence freshness, profile-match state,
owner alias, and linked evidence, and derives one readiness state drawn from the
frozen `M5GovernanceReadinessState` vocabulary. The derivation is degrade-first:

| Condition | Readiness |
| --- | --- |
| metric not run, or any unknown reading | `not_evaluated` |
| no resolved owner | `owner_unresolved` |
| evidence missing | `blocked` |
| evidence stale | `evidence_stale` |
| metric failed or threshold breached | `blocked` |
| failure held under a disclosed waiver | `waived` |
| wrong or unpinned profile, or unknown provenance | `warning` |
| metric at warning, at-threshold, or aging evidence | `warning` |
| passing within threshold, fresh, profile-matched, owned | `passing` |

**A green metric with stale or wrong-profile evidence never resolves to `passing`.**
It degrades visibly to `evidence_stale` or `warning` with a self-describing degrade
reason and a next action, so it never looks equivalent to a fresh pass.

## Governance report row

`resolve_governance_report` takes one report's identity, report type, corpus/profile
scope, provenance class, timestamp, declared outcome, evidence freshness, and
support-class boundedness, and derives a readiness state, a **provenance disclosure**,
and the compare/open-report actions. The provenance disclosure names what kind of
corpus or profile produced the result and whether it may be trusted outside its
support class:

| Provenance | Disclosure | Trustable outside support class |
| --- | --- | --- |
| canonical corpus, in support class | `canonical_within_support_class` | yes |
| canonical corpus, out of support class | `profile_pinned_disclose_scope` | no |
| pinned profile | `profile_pinned_disclose_scope` | no |
| sampled corpus | `sampled_disclose_caveat` | no |
| synthetic corpus | `synthetic_disclose_caveat` | no |
| unknown provenance | `provenance_undisclosed` | no |

**Only a canonical corpus consumed within its support class may be trusted outside
that support class without a stated caveat.** Every other provenance carries a
disclosure so a user can tell what kind of corpus or profile produced a governance
result before trusting it further; an undisclosed provenance, or a result read
outside its support class, degrades to `warning`.

## Parity matrix and guardrails

`M5FitnessGovernanceControlsPacket` binds one row per claimed assurance consumer to
the shared anatomy, the same readiness/provenance/evidence vocabulary, the report
actions (always including compare and open-report), the export fields, and the
non-visual accessibility routes, with worked resolution cases that must reproduce the
resolver output exactly. Hard invariants (all `false`) forbid rendering stale or
wrong-profile evidence as a clean pass, hiding the corpus/profile provenance, hiding
the owner or evidence freshness, and inventing a dashboard-local status word. Owner
aliases are role aliases, never personal contact details, and raw URLs, tokens,
credentials, and user text bodies never cross the export boundary.
