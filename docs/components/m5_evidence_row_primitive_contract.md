# M5 Evidence / Activity Row Primitive Contract — Stable Verbs, Provenance, and Copy Parity

> Task: M05-759 · Batch B88 · Delivery class: high-trust component contract +
> reusable primitive implementation + support/export parity.

This contract implements Aureline's **one reusable event / history row primitive**
across every M5 lane that explains what happened — AI evidence, task events, policy
changes, provider mutations, remote reconnects, update history, support exports, and
repair flows — so timestamp, actor, action, object / scope, outcome, expandable
detail, and provenance stay consistent and portable instead of drifting into
per-feature prose rows that only a screenshot can preserve. It narrows the event /
history row family named by the frozen
[M5 trust-chronology component matrix](m5_trust_chronology_components_contract.md)
(M05-756) into a working primitive with a resolver and a per-lane parity matrix, and
is the chronology twin of the
[settings-row](m5_settings_row_primitive_contract.md) (M05-757) and
[capability-sheet](m5_capability_sheet_primitive_contract.md) (M05-758) primitives.

- **Boundary schema:** [`schemas/ui/m5-evidence-row.schema.json`](../../schemas/ui/m5-evidence-row.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/implement_the_m5_evidence_and_activity_timeline_row_primitive/`
- **Headless emitter:** `aureline_shell_m5_evidence_row_primitive`
- **Checked support export:** [`artifacts/release/m5-evidence-row-proof/support_export.json`](../../artifacts/release/m5-evidence-row-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-evidence-row-proof/matrix.csv`](../../artifacts/release/m5-evidence-row-proof/matrix.csv)
- **Report:** [`artifacts/components/m5-evidence-row-primitive.md`](../../artifacts/components/m5-evidence-row-primitive.md)
- **Narrowed fixtures:** [`fixtures/ui/m5-evidence-row-primitive/`](../../fixtures/ui/m5-evidence-row-primitive/)

The stable chronology verbs, provenance badges, chronology detail states,
chronology export fields, non-visual accessibility routes, qualification classes,
and downgrade triggers are reused verbatim from the frozen
[M5 trust-chronology component matrix](../../schemas/ui/m5-trust-chronology-components.schema.json);
the shell topology — zones, responsive classes, window classes, and consumer
surfaces — is reused verbatim from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). This lane
mints new vocabulary only for what those matrices left implicit about the row
itself: its **history-lane families**, its **anatomy parts**, its **copy formats**,
and its **focus behaviors**. No M5 surface invents a second row grammar or a second
verb vocabulary.

The primitive also projects from the existing history / event contracts:
[`schemas/events/activity_row.schema.json`](../../schemas/events/activity_row.schema.json),
[`schemas/execution/task_event.schema.json`](../../schemas/execution/task_event.schema.json),
and [`schemas/ops/event_provenance_row.schema.json`](../../schemas/ops/event_provenance_row.schema.json).

## Track invariant

One evidence / chronology model carries stable verbs, provenance badges, and
portable detail / export semantics. The same action never reads under two names; a
human, AI, automation, or remote action is never conflated with another; detail is
always reopenable from durable history; and where a lane already claims portable
evidence, every row copies as text, JSON, and Markdown so the support export never
needs a screenshot to preserve what happened.

## The primitive: two halves

### 1. Resolver — `resolve_evidence_row`

Given one history lane's raw events (each a `timestamp_repr`, an `actor_repr`, a
stable `verb`, an `object_repr`, an `outcome`, a `provenance` badge, and an optional
`detail_ref`), the resolver produces one `M5ResolvedEvidenceLog` carrying:

- one `detail_state` per event — `reopenable_detail` when the event has
  disclosure-ready detail, `collapsed` for a terse no-detail row,
- the three portable copy renderings (`text`, `json`, `markdown`) per event **when
  the lane claims portable evidence**, all carrying the same seven truth columns
  (timestamp, actor, verb, object, outcome, provenance, has-detail), and
- `emits_portable_copy` (true when every resolved row carries the copy triple).

The resolver rejects malformed input: no events, an empty timestamp, actor, or
object, an event that claims expandable detail but names no anchor
(`missing_detail_ref`), an event that names an anchor without claiming detail
(`unexpected_detail_ref`), and any representation carrying URLs, credentials, or
other forbidden material.

### 2. Parity matrix — one row per history lane

Each of the eight history lanes carries the same shared anatomy, the same stable
verbs and provenance badges, the same chronology detail states, the same copy
formats (all three when portable, none otherwise), and the same export fields, plus
worked resolution cases proving the resolver on that lane. Every lane renders in the
`bottom_panel` zone — the execution, output, problems, terminal, and timeline zone.

| History lane | Portable | Worked resolution highlight |
| --- | --- | --- |
| `ai_evidence` | yes | `ran` (AI-initiated) with reopenable detail |
| `task_events` | yes | `created` then `updated` (human-initiated) |
| `policy_changes` | yes | `approved` + `rejected`/`denied` (system-initiated) |
| `provider_mutations` | yes | `updated` (remote-actor / provider-owned) |
| `remote_reconnects` | no | `recovered` (remote-actor), no copy renderings |
| `update_history` | yes | `failed` then `recovered` (automation-initiated) |
| `support_exports` | yes | `exported` (human-initiated) with reopenable detail |
| `repair_flows` | yes | `recovered` replayed-from-history, reverted a change |

The non-portable `remote_reconnects` lane proves the primitive renders the same row
grammar with no copy renderings where a lane does not yet claim portable evidence.

## Anatomy (shared row)

`timestamp`, `actor`, `action`, `object_or_scope`, `outcome`, and
`provenance_badge` are mandatory on every row. `detail_link` is the conditional part
shown whenever an event has disclosure-ready expandable detail.

## Stable verbs and provenance

The verb vocabulary is closed and reused verbatim: `created`, `updated`, `ran`,
`approved`, `rejected`, `failed`, `recovered`, `exported`. No lane invents local
prose verbs. The provenance badges — `human_initiated`, `ai_initiated`,
`automation_initiated`, `remote_actor`, `system_initiated`, `replayed_from_history`
— attribute every event so a user, AI, automation, remote host, provider-owned, or
replayed action is never conflated. (The outcome vocabulary — `succeeded`, `failed`,
`pending`, `denied`, `reverted` — is a resolver-side concept kept orthogonal to the
verb: the verb is *what happened*, the outcome is *how it ended*.)

## Copy / export parity

The copy formats `text`, `json`, and `markdown` are offered by every lane that
claims portable evidence, and are absent from lanes that do not — enforced by the
`copy_format_parity_mismatch` check. The export fields `event_verb`, `provenance`,
`timestamp`, `object_ref`, `actor_role`, and `outcome_code` are mandatory;
`redaction_class` completes the record.

## Support / export reconstruction

Each lane carries its worked resolution cases in the export, and the validator
re-runs the resolver on every stored input and asserts it equals the stored output.
Packet-level lints require that the worked cases collectively (a) exercise every
stable verb, (b) attribute every provenance badge, and (c) prove at least one row
copyable as text, JSON, and Markdown — so the support export reconstructs what
happened from the same shared row model with the same vocabulary, no screenshot
required.

## Hard invariants (per surface row, all MUST be false)

- `drifts_from_verb_vocabulary`
- `drops_provenance_badge`
- `detail_not_reopenable`
- `drops_export_or_audit_truth`

The Rust validator and resolver in `crates/aureline-shell` are the authoritative
gate; the schema and this doc document the shape. Regenerate the checked export,
CSV, report, and fixtures with the headless emitter subcommands (`support-export`,
`csv`, `report`, `fixture-update-history-beta-narrowed`,
`fixture-repair-flows-preview-narrowed`).
