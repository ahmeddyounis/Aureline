# M5 Support-Intake / Escalation Component Surface Certification (M05-907)

This is the **closing capstone** of the B106 support-intake / escalation component lane. Where
the freeze matrix (`m5_support_intake_escalation_component_matrix.md`, M05-900) defines the five
reusable components, the M05-901..904 primitive lanes narrow each one, the M05-905 consumer lane
proves they are reusable across the claimed Doctor / safe-mode / bisect / support-center / docs-help
/ export consumers, and the M05-906 accessibility / auto-narrowing capstone certifies keyboard /
screen-reader / CLI / export parity per family, this capstone **certifies** that the shared
support-intake / escalation component truth holds on every claimed M5 supportability surface — and
auto-narrows any surface that cannot sustain it.

- Boundary schema: `schemas/ui/m5-support-intake-escalation-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-support-intake-escalation-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-support-intake-escalation-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-support-intake-escalation-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-support-intake-escalation-component-certification/`
- Implementation: `crates/aureline-support/src/certify_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_truth_on_every_claimed_m5_support_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user starts diagnosis from, reviews a suggested
repair on, builds a report on, or escalates a case from — not on component family or primitive lane.
The eight certified surfaces are:

`doctor_results`, `safe_mode`, `extension_bisect`, `support_center`, `docs_help`,
`support_bundle_preview`, `cli_headless`, and `support_export`.

Each surface is scored on six truth axes:

1. `visual` — scenario family, incident scope, selected / omitted evidence classes, Doctor finding
   lineage, approved repair class, packet destination, and next human step are shown on the primary
   surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same support identity.
5. `degraded_state` — an uncertain scenario, an omitted evidence class, a local-only destination, or
   a policy-blocked repair honestly downgrades a `ready_to_escalate` / `reviewable_case` claim.
6. `support_intake_and_escalation_provenance` — scenario / scope / evidence / finding lineage /
   approved repair / destination / next step stay explicit before any diagnosis start, repair
   review, report build, or escalation, never inheriting a healthier lane's truth, and **escalation
   never drops scenario / finding / packet lineage** between local diagnosis and human handoff.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`ready_to_escalate` / `reviewable_case` claim while a truth axis is not current — the scenario
classification is uncertain, an evidence class was omitted, the packet destination is local-only,
the approved-repair guidance is policy-blocked, or the next-human-step continuity is unstated — is
over-claiming and is blocked (`red`). A surface that discloses the reduction by narrowing its
support claim (with a bound reason and a frozen downgrade trigger) is honestly `yellow`. The
always-on `cli_export` axis must always stay certified. **Escalation never drops lineage**: a
narrowed case preserves its scenario / finding / packet lineage continuity rather than dropping it
between local diagnosis and human handoff (`lineage_preserved` / `preserves_lineage_continuity`).

The support-claim ladder (strongest first) is reused from the M05-906 accessibility capstone:
`ready_to_escalate` (5) > `reviewable_case` (4) > `local_only_diagnosis` (3) >
`evidence_incomplete_case` (2) > `unclassified_scenario` (1) > `policy_blocked_repair` (0).
Certification may only narrow a claim, never strengthen it.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, lineage
preservation, export parity, and claim narrowing. A row is `red` when it is malformed, drops
CLI/export parity, drops lineage, hides an undisclosed drift, retains a degraded axis behind a full
claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed and
bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the five frozen component families on at least one surface, every axis on every
row, and lineage preservation on every surface. Every row cites the one canonical proof bundle
(`artifacts/release/m5-support-intake-escalation-proof/support_export.json`) plus the M05-905
consumer and M05-906 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's four auto-narrowing conditions: an uncertain scenario
(`extension_bisect` → `unclassified_scenario`), omitted evidence (`docs_help` →
`evidence_incomplete_case`), a local-only destination (`support_bundle_preview` →
`local_only_diagnosis`), and a policy-blocked repair (`cli_headless` → `policy_blocked_repair`).

## Regenerating the artifacts

The seed builder (`seeded_m5_support_intake_escalation_component_certification_packet`) is the one
source of truth for both the tests and the on-disk export. To regenerate:

```
GEN_SUPPORT_CERT_ARTIFACTS=1 cargo test -p aureline-support --lib \
  certify_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_truth_on_every_claimed_m5_support_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: raw logs, report bodies, redacted evidence contents, and
credentials never cross this boundary.
