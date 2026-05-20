# Stale-example and broken-link audit

This audit reports the stale-example and broken-link findings the M3
docs-maintenance lane gates, the suppression attribution that keeps each
finding accountable, and the branch / channel / audience matrix that bounds
README / changelog / onboarding maintenance. It is generated from the governed
docs-maintenance records in `aureline-docs::maintenance` and the drill corpus
at [`fixtures/docs/m3/docs_maintenance_corpus/`](../../../fixtures/docs/m3/docs_maintenance_corpus/).
Every row below is replayed by
`cargo test -p aureline-qe --test docs_maintenance_conformance`.

## Finding ledger

Each finding pins its class, detection posture, validation mode, and the
evidence the verdict rested on. A rendered preview is never proof: findings cite
a concrete validation run or are honestly labeled as suspected / unverified.

| Finding id | Class | Detection | Validation mode | Last checked | Suppression |
| --- | --- | --- | --- | --- | --- |
| `docs-finding:readme:broken-link` | broken_link | proven_broken | rendered | 2026-05-20 | active |
| `docs-finding:help-article:stale-example-suppressed` | stale_example | suspected_stale | stale | 2026-05-12 | suppressed_until_reviewed |
| `docs-finding:help-article:version-mismatch` | version_mismatch | unchanged_unverified | skipped | — | active |
| `docs-finding:module-doc:command-output-drift` | command_output_drift | proven_broken | executed_locally | 2026-05-20 | active |
| `docs-finding:reference:api-mismatch` | api_mismatch | proven_broken | executed_remotely | 2026-05-20 | active |
| `docs-finding:benchmark-copy:unverifiable` | unverifiable_benchmark_copy | unchanged_unverified | unsupported | — | active |
| `docs-finding:settings:renamed-setting` | renamed_setting | unchanged_unverified | skipped | — | suppressed_until_reviewed |
| `docs-finding:reference:renamed-symbol` | renamed_symbol | suspected_stale | stale | 2026-05-12 | acknowledged |
| `docs-finding:migration-notes:missing-note` | missing_migration_note | unchanged_unverified | not_validated | — | active |

The first three rows are the dedicated corpus drills; the remainder are carried
by the full seeded contract (`contract.seeded_full`). Every validation mode
(rendered, syntax-checked, executed locally, executed remotely, unsupported,
skipped, stale, not-validated) is exercised so the distinction between proven
failure and suspected / unverified drift stays visible.

## Suppression attribution

A suppress-until-reviewed finding is never anonymous. The lane requires actor,
reason, expiry, and evidence on every suppressed finding; a finding that drops
this attribution fails validation with `finding_row.suppression_attribution`
(`negative.finding_stale_example_missing_attribution`).

| Finding id | Actor | Reason | Expiry | Evidence |
| --- | --- | --- | --- | --- |
| `docs-finding:help-article:stale-example-suppressed` | `actor:maintainer:help-owner-02` | Snippet rewrite tracked behind the example-runner update | 2026-06-30 | `evidence:stale-example-scan:help.snippet-01`, `evidence:tracking-item:example-runner-update` |
| `docs-finding:settings:renamed-setting` | `actor:maintainer:docs-owner-01` | Renamed setting confirmed; doc fix scheduled with the settings owner | 2026-06-15 | `evidence:settings-registry:renamed-setting` |

## Branch / channel / audience matrix

README / changelog / onboarding maintenance preserves its branch, release, and
channel scope and its audience so beta notes can never masquerade as current
stable docs. An update that crosses a review or publish boundary without a scope
fails with `maintenance_row.publish_scope`; an export that flips the channel is
caught as `review_packet.row_drift`.

| Maintenance row | Audience | Boundary | Branch | Release | Channel |
| --- | --- | --- | --- | --- | --- |
| `docs-maintenance-row:readme:local-only` | public_reader | local_only | — | — | — |
| `docs-maintenance-row:onboarding:review-handoff` | end_user | review_handoff_scoped | branch:docs/onboarding-beta | — | beta |
| `docs-maintenance-row:changelog:scoped-publish` | release_manager | publish_handoff_scoped | branch:release/beta-2 | release:beta-2 | beta |

## Gate

These rows can gate beta docs-authoring claims: the audit is regenerated from
the same records the product renders, and the conformance suite fails the build
if any finding loses its class, attribution, validation honesty, or scope.

```sh
cargo test -p aureline-qe --test docs_maintenance_conformance
```
