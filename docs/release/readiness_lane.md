# Program readiness lane: contract readiness vs operational readiness

This page tracks the beta exit conditions that **cannot be closed by the
contracts layer alone**. The repository today is a contracts + validator +
test-corpus codebase: typed record models, projections, frozen fixtures, and
CI gates. Several beta exit-gate conditions are *operational* — they require a
program or a running client doing real work, not just a record model that
describes it.

Those conditions are easy to mistake for closed, because their **contract
models already exist and pass their gates**. A green validator on a rollback
plan, a policy preview record, a support-bundle manifest, or a cohort
scorecard proves the *shape* of the truth is governed. It does **not** prove
that an installer rolled back a real machine, that a client enforced a proxy
policy, that a support bundle was captured from live state, or that partner
orgs ran a daily build.

> **Contract presence is not operational readiness.** A row in this lane is
> *not* closed when its backing contract model lands and its gate is green. It
> is closed only when the named runtime or program evidence below is produced
> and recorded.

## How to read a row

Each row names:

- **Operational exit condition** — the user-visible condition, described as a
  behavior (not as a planning code).
- **Backing contract (crate / path)** — the typed model, schema, doc, or
  artifact that already exists and that a reviewer might mistake for the
  finished condition.
- **Missing runtime / program evidence** — what still has to happen *at
  runtime* or *in the program* before the condition is honestly done. This is
  the part the contracts layer cannot supply.
- **Owner / placeholder** — who carries the runtime/program lane. Where the
  runtime lane is not yet staffed, the cell says so explicitly rather than
  implying coverage.

A backing contract being present is necessary but not sufficient: it bounds
and makes the future behavior inspectable, which is exactly why it must not be
read as the behavior itself.

## Readiness lane

| Operational exit condition | Backing contract (crate / path) | Missing runtime / program evidence | Owner / placeholder |
|---|---|---|---|
| Debug, test, and task sessions actually run and emit real run evidence | `crates/aureline-runtime/src/debug/`, `crates/aureline-runtime/src/tasks/`, `crates/aureline-runtime/src/tests/`, `crates/aureline-runtime/src/execution_context/` (execution-context object model and resolver seed) | A running client that launches debug/test/task sessions and produces a real `TaskEventStream` from a live process. The crate mints contexts from a resolver seed; nothing executes a session. | Runtime/execution lane — **owner placeholder, not yet staffed** |
| A debugger remote-attaches and reattaches across live host lanes | `crates/aureline-remote/src/route_governance/` (`RouteObject`, `ExposureReview`), `crates/aureline-runtime/src/managed_alpha/` (suspend/resume/reattach inspection), `docs/support/host_lane_and_reattach_beta.md` (`reattach_review_sheet_record`) | A remote agent plus debugger that attaches to a live process and reattaches after a host change. The records describe the review and exposure of an attach; nothing performs it. | Remote/runtime lane — **owner placeholder, not yet staffed** |
| An installer/updater performs a rollback on a real install | `crates/aureline-install/src/rollback/`, `docs/release/update_and_rollback_contract.md`, `docs/release/m3/update_rollback_beta.md`, `artifacts/release/m3/update_rollback/rollback_plan.json` | An installer/updater that executes the admitted rollback on a real machine and restores the retained artifact set. `aureline-install` states it "does not implement an installer, updater, package manager, or fleet-control service." | Release/install lane — runtime owner **placeholder, not yet staffed**; rollback drill: `@ahmeddyounis` |
| Enterprise policy and proxy/transport are enforced by a running client | `crates/aureline-policy/src/authority/`, `crates/aureline-policy/src/simulation/`, `crates/aureline-policy/src/runtime_authority_issuers/` | A client that enforces policy and routes through an enterprise proxy/transport. `aureline-policy` "does not evaluate a full policy language; it provides typed preview records," and the enterprise proxy lab is not yet a standing runtime. | Policy/transport lane — **owner placeholder, not yet staffed** |
| A support export is captured from live state at runtime | `crates/aureline-support/src/bundle/`, `crates/aureline-support/src/export_review/`, `crates/aureline-support/src/runtime_evidence/`, `docs/support/support_bundle_contract.md` | A running client that captures and emits a real support bundle from live state. The preview builder projects fixtures; runtime replay packs are "minted by `aureline-runtime`," which has no running app. | Support lane — runtime owner **placeholder, not yet staffed**; redaction review: `@ahmeddyounis` |
| An importer ingests a real foreign project and migration runs end-to-end | `docs/migration/first_run_import_diff_and_rollback_contract.md`, `docs/migration/migration_equivalence_and_parity_scorecard.md`, `docs/migration/m3/migration_wizard_beta.md` | An importer that ingests a real incumbent project and a migration that runs to completion, producing measured parity rather than a declared scorecard. | Migration lane — **owner placeholder, not yet staffed** |
| The extension runtime loads, isolates, and publishes real add-ons | `crates/aureline-extensions/src/manifest_baseline/`, `crates/aureline-extensions/src/install_review/`, `crates/aureline-extensions/src/conformance_reports/`, `docs/extensions/m3/runtime_v1_beta.md`, `docs/extensions/m3/host_isolation_beta.md`, `docs/extensions/m3/publication_pipeline_beta.md` | A host process that loads and isolates an extension at runtime, plus a publication/registry pipeline that actually admits one. The crate models manifests, install review, and conformance reports; it does not host or run an extension. | Extensions lane — **owner placeholder, not yet staffed** |
| Design-partner repositories run on daily beta builds | `artifacts/milestones/m3/cohorts/design_partner_scorecard.md`, `docs/partners/m3/design_partner_beta_pack.md`, `docs/program/design_partner_and_public_proof_packet.md` | Real partner organizations pulling and running daily beta builds on their own hardware. The scorecard's current `evidence_refs` are inherited alpha intake packets and templates, not live partner-run results. | Program/partners lane — scorecard owner `@ahmeddyounis`; live daily-build cohort **not yet running** |
| Named partner and certified-archetype cohorts produce live scorecards | `artifacts/milestones/m3/cohorts/*.md`, `artifacts/compat/m3/archetype_scorecards/*.md`, `ci/check_cohort_archetype_scorecards.py`, `docs/milestones/m3/beta_admission_matrix.md` (cohort/archetype tables) | Scorecard inputs fed from real cohort runs. The scorecards are hand-authored YAML with `as_of` dates; the validator recomputes effective support class from those dates and waivers, not from telemetry of cohorts actually running. | Program lane — scorecard owner `@ahmeddyounis`; live cohort feed **not yet running** |

## What closes a row

A row moves from *contract present* to *operationally ready* only when the
runtime/program evidence in its third column is produced and recorded against
the current exact build — for example:

- a rollback drill that ran on real hardware and restored the retained
  artifact set, recorded through the rollback contract, not just a green
  `rollback_plan.json`;
- a support bundle captured from a live session and passed through redaction
  review, not just a green manifest fixture;
- a cohort scorecard whose `evidence_refs` point at real partner-run results
  for the current build, not at inherited templates.

Until then, the contract row stays green and this lane row stays open. The two
states are tracked separately on purpose.

## Owners and placeholders

Where a cell names `@ahmeddyounis`, that owner already carries the *contract*
or *review* facet (for example the rollback drill or the support-export
redaction review). The *runtime/program* facet of each row is a distinct
responsibility; where it is not yet staffed, the row says **owner placeholder,
not yet staffed** rather than implying the contract owner also owns the
runtime lane. Release-council escalation and final sign-off route through
`docs/governance/decision_rights_and_signoff_matrix.md`.

## Cross-references

- Beta admission claim surface and cohort routing:
  `docs/milestones/m3/beta_admission_matrix.md`
- Release-control packet for the current candidate:
  `docs/release/m3/release_center_beta.md`
- Update and rollback contract: `docs/release/update_and_rollback_contract.md`
- Support bundle / export contract: `docs/support/support_bundle_contract.md`
- Design-partner and public-proof program:
  `docs/program/design_partner_and_public_proof_packet.md`

## How to verify

This is a documentation lane; there is no running app to exercise. To confirm
the lane is honest:

1. Every operational beta exit condition appears as a row above with a backing
   contract path and the missing runtime/program evidence.
2. Each backing-contract path resolves to a real crate module, doc, artifact,
   or validator in this repository.
3. The page states plainly that contract presence is not operational
   readiness, and the cross-linked release and support docs point back here so
   reviewers see the two readiness states as distinct.
