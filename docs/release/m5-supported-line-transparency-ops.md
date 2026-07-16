# M5 Supported-Line Public-Proof, Transparency-Report, Migration-Scoreboard, ORR-History, and Correction-Train-Archive Contract

Status: frozen (B147 opening matrix)

This contract freezes Aureline's durable post-launch external-proof object model — its supported-line
public-proof ledgers, upstream/compatibility transparency reports, versioned migration scoreboards,
supported-line ORR-history events, and correction-train archive packets, plus their public-safe versus
internal-only visibility posture — into one export-safe matrix. It is the canonical source of M5+ external
post-launch truth: release, help, docs, support, public-proof, and partner/procurement surfaces consume it
directly rather than reconstructing public-proof freshness, migration pain, ORR history, or correction history
from prose-only docs or scattered internal notes.

- Matrix schema: `schemas/program/m5-supported-line-transparency-matrix.schema.json`
- Public-proof-freshness-ledger domain schema (public-proof ledger / transparency report): `schemas/program/m5-public-proof-freshness-ledger.schema.json`
- Migration-scoreboard domain schema (migration scoreboard): `schemas/program/m5-migration-scoreboard.schema.json`
- Supported-line-ORR-history domain schema (ORR-history event): `schemas/program/m5-supported-line-orr-history.schema.json`
- Correction-train-archive domain schema (correction-train archive): `schemas/program/m5-correction-train-archive.schema.json`
- Support export: `artifacts/release/m5-supported-line-transparency/support_export.json`
- Matrix CSV: `artifacts/release/m5-supported-line-transparency/matrix.csv`
- Design report: `artifacts/program/m5-supported-line-transparency-matrix.md`
- Public-proof dashboard: `dashboards/m5-supported-line-public-proof.json`
- Narrowed fixtures: `fixtures/release/m5-supported-line-transparency/`
- Authoritative validator: `crates/aureline-ui` (`m5_supported_line_transparency_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix`

## Governed proof objects

The matrix freezes **five** supported-line proof objects, each qualified independently and each pointing at one
canonical domain schema:

| Object | External-proof concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `public_proof_ledger` | Current public-claim / compatibility-report / support-window proof kept fresh within its freshness window | Public-proof ledger owner | public-proof-freshness-ledger |
| `transparency_report` | Export-safe upstream-health / compatibility-health / maintainer-durability report with no internal-only leakage | Transparency-report owner | public-proof-freshness-ledger |
| `migration_scoreboard` | Versioned migration path scored with tracked blockers and recorded migration-pain deltas | Migration-scoreboard owner | migration-scoreboard |
| `orr_history_event` | Retained ORR / go-no-go / support-window decision history archived per supported line | ORR-history archive owner | supported-line-orr-history |
| `correction_train_archive` | Correction-train / hotfix-backport / advisory packets archived and bound to exact build identity | Correction-train archive owner | correction-train-archive |

## Shared transparency-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`freshness_window`, `transparency_disclosure`, `migration_scoreboard_currency`, `orr_history_retention`,
`correction_archive_retention`, `public_proof_freshness`, `correction_history_join`.

The freshness-window / transparency-disclosure / orr-history-retention / correction-archive-retention roles
(`freshness_window`, `transparency_disclosure`, `orr_history_retention`, `correction_archive_retention`) must
preserve the evidence snapshot and confirm ownership before widening — a claim may never widen because a report
once existed without current freshness, a public feed may never leak internal-only detail, and ORR or correction
history may never be dropped. The descriptive structure roles (`migration_scoreboard_currency`,
`public_proof_freshness`, `correction_history_join`) are inspectable descriptors.

## Visibility classes

Every domain object carries an export class of `public_safe` or `internal_only`. Transparency reports,
public-proof ledgers, and any partner/procurement feed stay export-safe; internal-only incident and security
detail never crosses into a public-safe view.

## Widening stages

Each object gates the widening stages it must clear before a supported line may claim the next channel,
answering which external-proof gate is required before **alpha**, **beta**, **release_candidate**, **stable**,
and **long_term_support** widening once rather than leaving it to shiproom folklore.

## Hard invariants (release blockers)

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block asserts the
corresponding fleet-level guarantees:

1. A claim never widens because a report once existed without current freshness.
2. No supported line stays green on stale external proof or opaque upstream health.
3. Internal-only incident or security detail never leaks into public-safe or partner/procurement feeds.
4. Public-proof, migration, and history stay joined to exact build and release-line identity.
5. Migration pain and ORR / correction history are never left unretained.

The frozen downgrade triggers also enumerate the remaining release blockers: a stale public-proof field, an
unretained ORR history, a leaked internal detail, an unscored migration, an object leaving its freshness-window
/ export-class / line-association state unstated, a missing registry reference, and a stale proof packet.

## Automatic narrowing

Claim publication and support/export narrow supported-line claims automatically when the B147 matrix row is
missing, stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping every
object visible:

- `orr_history_event_beta_narrowed.json` — the ORR-history event held at **Beta** pending its ORR / go-no-go
  decision history being fully archived.
- `correction_train_archive_preview_narrowed.json` — the correction-train archive narrowed to **Preview**
  pending correction-train / advisory packets archived and bound to exact build identity.

## Bound source contracts

The matrix binds back to already-landed proof so post-launch external truth is never split across scattered
notes: the stable-proof-index schema (`schemas/release/stable_proof_index.schema.json`) and the migration-task-row
schema (`schemas/release/m5-migration-task-row.schema.json`).
