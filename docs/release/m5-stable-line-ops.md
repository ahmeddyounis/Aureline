# M5 Stable-Line Protection, Evidence-Refresh, Correction-Line, and LTS-Readiness Contract

Status: frozen (B146 opening matrix)

This contract freezes Aureline's concrete post-stable operating model — its stable-line taxonomy, support
windows, correction-line owners, backport-decision SLAs, evidence-refresh cadences, bundle-refresh obligations,
and LTS-eligibility state — into one export-safe matrix. It is the canonical source of M5 post-stable truth:
release, help, support, public-proof, shiproom, and program-governance surfaces consume it directly rather than
reconstructing support-window, refresh, correction-ownership, or LTS state from prose-only docs or shiproom
notes.

- Matrix schema: `schemas/program/m5-stable-line-protection-matrix.schema.json`
- Stable-line-refresh-policy domain schema (fresh stable line / evidence-refresh line): `schemas/program/m5-stable-line-refresh-policy.schema.json`
- Supported-line-defect-ledger domain schema (correction/backport line / launch-bundle-currentness line): `schemas/program/m5-supported-line-defect-ledger.schema.json`
- LTS-readiness-decision domain schema (LTS-candidate line): `schemas/program/m5-lts-readiness-decision.schema.json`
- Support export: `artifacts/release/m5-stable-line-correction-reports/support_export.json`
- Matrix CSV: `artifacts/release/m5-stable-line-correction-reports/matrix.csv`
- Design report: `artifacts/program/m5-stable-line-protection-matrix.md`
- Stable-line health dashboard: `dashboards/m5-stable-line-health.json`
- Narrowed fixtures: `fixtures/release/m5-stable-line-protection/`
- Authoritative validator: `crates/aureline-ui` (`m5_stable_line_protection_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix`

## Governed lines

The matrix freezes **five** active stable / stable-candidate lines, each qualified independently and each
pointing at one canonical domain schema:

| Line | Post-stable concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `fresh_stable_line` | Crash / rollback / support-export / migration flows protected through the first 30 days after stable | Stable-line release owner | stable-line-refresh-policy |
| `evidence_refresh_line` | Certified-archetype / compatibility / known-limits evidence refreshed on an ordinary release-ops cadence | Evidence-refresh cadence owner | stable-line-refresh-policy |
| `correction_backport_line` | First correction / backport path exercised; backport decisions recorded within SLA; correction reports published | Correction-line owner | supported-line-defect-ledger |
| `bundle_currentness_line` | Launch-bundle freshness re-checked and bundle-refresh obligation met on the shipping line | Bundle-currentness owner | supported-line-defect-ledger |
| `lts_candidate_line` | Backport / rollback / support discipline demonstrated and an LTS decision packet recorded | LTS-readiness decision owner | lts-readiness-decision |

## Shared stable-line-protection-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`support_window`, `correction_ownership`, `evidence_refresh`, `backport_decision`, `lts_eligibility`,
`bundle_currentness`, `defect_ledger`.

The support-window / correction-ownership / lts-eligibility / backport-decision roles (`support_window`,
`correction_ownership`, `lts_eligibility`, `backport_decision`) must preserve the evidence snapshot and confirm
ownership before widening — support language may never widen without current refresh and correction evidence,
LTS may never be claimed without current rollback and support evidence, a supported-line defect may never be
left unowned, and a decision may never rest on a stale evidence snapshot. The descriptive structure roles
(`evidence_refresh`, `bundle_currentness`, `defect_ledger`) are inspectable descriptors.

## Widening stages

Each line gates the widening stages it must clear before claiming the next channel, answering which
line-protection gate is required before **alpha**, **beta**, **release_candidate**, **stable**, and
**long_term_support** widening once rather than leaving it to shiproom folklore.

## Hard invariants (release blockers)

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block asserts
the corresponding fleet-level guarantees:

1. Support language never widens without current refresh and correction evidence.
2. A shipping line never drifts on stale evidence or frozen launch bundles.
3. A backport never rests on tribal memory instead of a documented correction packet.
4. LTS eligibility is never claimed without current rollback and support evidence.
5. A supported-line defect is never left unowned or unresolved past its SLA.

The frozen downgrade triggers also enumerate the remaining release blockers: a stale refresh field, an
undocumented backport, a missing registry reference, a line leaving its support-window / refresh / LTS-posture
state unstated, and a stale proof packet.

## Automatic narrowing

Claim publication and support/export narrow line claims automatically when the B146 registry is missing, stale,
or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping every line visible:

- `bundle_currentness_beta_narrowed.json` — the launch-bundle-currentness line held at **Beta** pending a
  current bundle-refresh audit on the shipping line.
- `lts_candidate_preview_narrowed.json` — the LTS-candidate line narrowed to **Preview** pending an LTS decision
  packet backed by current rollback and support evidence.

## Bound source contracts

The matrix binds back to already-landed launch-time proof so post-stable truth is never split across scattered
shiproom notes: the stable claim-manifest schema (`schemas/release/stable_claim_manifest.schema.json`) and the
release-center schema (`schemas/release/release_center.schema.json`).
