# M5 support-intake / escalation component consumers (M05-905)

The **adoption lane** over the frozen M5 support-intake / escalation component matrix. It
proves the five governed component families are reusable components — not one Project Doctor
result plus a few isolated export objects — by binding every claimed M5 support consumer to
the same canonical component schemas and the same descriptor vocabulary, so **scenario code,
packet id, redaction class, and approved-repair guidance** stay aligned across surfaces even
when the surrounding workflow differs.

This closes the B106 consumer-adoption lane over the frozen support-intake / escalation
component matrix (`schemas/ui/m5-support-intake-escalation-component-matrix.schema.json`),
layered on top of the four `implement_*` primitive lanes (M05-901 … M05-904) that narrowed the
matrix families into working resolvers.

- Module:
  `crates/aureline-support/src/add_shared_doctor_safe_mode_bisect_support_center_docs_help_and_export_consumers_so_support_intake_components_keep_scenario_code_repair_lineage_and_redaction_parity_across_claimed_m5_profiles`
- Emitter bin: `aureline_support_support_intake_escalation_component_consumers`
- Schema: `schemas/ui/m5-support-intake-escalation-component-consumer.schema.json`
- Support export: `artifacts/release/m5-support-intake-escalation-component-consumer-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-support-intake-escalation-component-consumer-proof/matrix.csv`
- Report: `artifacts/release/m5-support-intake-escalation-component-consumer-proof/report.md`
- Fixtures: `fixtures/ui/m5-support-intake-escalation-component-consumers/`

## Consumers

Six claimed M5 support consumers adopt the shared components:

| Consumer | Adopted families (examples) |
| --- | --- |
| **Project Doctor Results** | support-scenario picker row, unsafe-fix blocked note, escalation-packet summary |
| **Safe-Mode Recovery** | support-scenario picker row, handoff-timeline row, unsafe-fix blocked note |
| **Extension Bisect** | support-scenario picker row, issue-report builder step, handoff-timeline row |
| **Support Center** | issue-report builder step, escalation-packet summary, handoff-timeline row, unsafe-fix blocked note |
| **Help / Docs** | support-scenario picker row, issue-report builder step, escalation-packet summary |
| **Support / Export Desk** | escalation-packet summary, handoff-timeline row, issue-report builder step, unsafe-fix blocked note |

Each family is adopted by **at least two** distinct consumers (the acceptance-criterion proof
of reuse). The support / export desk is singled out for a canonical-schema reference so a
support / export lane's prose can never drift from the product truth.

## The shared descriptor vocabulary

Every binding surfaces the four required descriptors — **scenario code**, **packet id**,
**redaction class**, and **approved repair** — read from a single canonical source. A consumer
never re-words these per surface and never invents a second escalation grammar. This is the
acceptance-criterion that these facts stay one truth across in-product and exported support
surfaces.

## Resolver — `resolve_support_intake_binding`

Takes one consumer's adoption of one component family, the descriptor set it surfaces, the
parity-health mode it renders under, and any export caveats. It:

1. Rejects an empty descriptor set, a missing required descriptor, or a note that carries
   forbidden material.
2. Derives the **claim-parity state**: `claims_preserved` at full parity, `claims_auto_narrowed`
   under any weakened parity-health mode.
3. Whenever parity is weakened, emits a **self-contained auto-narrow banner** naming the exact
   reason, the descriptors that stay preserved, the export caveats, and the recovery action —
   never a generic "degraded" note.

### Parity-health modes → reasons → recovery actions

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `full_parity` | — | — | — |
| `scenario_uncertain_narrowed` | `scenario_classification_uncertain` | `classify_scenario_before_escalating` | `scenario_uncertain_local_only` |
| `evidence_incomplete_narrowed` | `evidence_classes_incomplete` | `complete_evidence_selection_first` | `evidence_incomplete_not_full_report` |
| `destination_unavailable_narrowed` | `packet_destination_unavailable` | `choose_available_destination_or_export_locally` | `destination_unavailable_local_bundle_only` |
| `redaction_pending_narrowed` | `redaction_review_required` | `complete_redaction_review_first` | `redaction_pending_not_shareable` |

The narrowed rendering keeps the full descriptor vocabulary; only the claim is narrowed, so a
consumer with missing classification or an unavailable destination **degrades visibly rather
than inheriting full escalation language from healthier profiles**.

## Canonical family → primitive mapping

Each family points at the narrowed primitive that owns it, never a local re-description:

| Family | Canonical schema (owning lane) |
| --- | --- |
| `support_scenario_picker_row` | `schemas/ui/m5-support-scenario-picker-row.schema.json` (M05-901) |
| `issue_report_builder_step` | `schemas/ui/m5-support-issue-report-builder-step.schema.json` (M05-902) |
| `escalation_packet_summary`, `handoff_timeline_row` | `schemas/ui/m5-support-escalation-packet-summary.schema.json` (M05-903) |
| `unsafe_fix_blocked_note` | `schemas/ui/m5-support-unsafe-fix-blocked-note.schema.json` (M05-904) |

## First-consumer compatibility notes

- **Project Doctor results**: full parity on the scenario picker row and unsafe-fix blocked
  note (start diagnosis, review a suggested repair); the escalation-packet summary auto-narrows
  while the scenario classification is uncertain (local-only, cannot escalate).
- **Safe-mode recovery**, **support center**: full parity across their adopted families — the
  reduced-capability recovery lane and the authoritative support center keep the same case
  truth.
- **Extension bisect**: the issue-report builder step auto-narrows because mid-bisect evidence
  classes are incomplete; held at Preview in the narrowed fixture pending complete-evidence
  parity.
- **Help / docs**: the escalation-packet summary auto-narrows because the packet destination is
  unavailable under current policy — documentation degrades to local export; held at Beta in the
  narrowed fixture pending banner parity on every destination-unavailable path.
- **Support / export desk**: the escalation-packet summary auto-narrows under pending redaction
  review (a local bundle, not yet shareable); referencing the canonical schemas so its prose can
  never drift from the product truth.

## Governance

Every consumer adopts the shared primitives, references the canonical schema, keeps the
descriptor vocabulary shared (never re-worded), invents no new escalation grammar, and declares
a non-visual accessibility route. Later M5 rows cannot invent parallel consumer-adoption
vocabulary. Raw crash bodies, raw paths, credentials, and external endpoints never cross the
support boundary; every label is carried only as an opaque, export-safe representation.
