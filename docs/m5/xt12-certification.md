# M5 closeout certification: switching, visual-system, attention, and boundary truth

This is the canonical companion for the Milestone 5 closeout certification lane. It
closes the milestone with the same rigor Aureline applies to performance, trust, and
compatibility: one explicit, evidence-backed certification state for **every
marketed M5 surface** across switching truth, visual-system parity, durable
attention, and embedded-boundary integrity.

## What this lane certifies

For every marketed M5 surface the matrix binds one cell per certification dimension
to the canonical depth-lane evidence that landed earlier in the milestone, grouped
into four axes:

| Axis | Dimensions |
| --- | --- |
| Switching truth | onboarding/migration, first-useful-work, command discoverability |
| Visual-system parity | component-state parity, appearance conformance |
| Durable attention | durable-attention routes, notification privacy |
| Embedded-boundary integrity | embedded boundary, desktop conformance, accessibility/i18n |

Each surface ends the milestone in exactly one state — **qualified**, **narrowed**,
or **held back** — and a below-cutline surface always carries the dimension(s) and
reason(s) that hold it there.

## Single source of truth, no widening

The matrix never re-types any surface's lifecycle truth. It ingests the canonical
M5 feature-family register
(`artifacts/release/m5/publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json`),
which already carries each marketed surface's claim ceiling and effective
(published) label after the family-level narrowing rules ran. The matrix carries
that ceiling and effective label straight through, so a surface can never advertise
a wider claim here than its canonical packet permits. The CI gate re-reads the
source register and fails on any drift.

## Automated narrowing

Narrowing is automated, never left to marketing copy or shiproom memory. A surface
narrows to Beta or holds at Preview whenever its certification evidence is **stale,
missing, policy-blocked, or red** on the claimed profile, or while it still depends
on a **forbidden v1-shell pattern**: toast-only long-running truth, theme-only state
meaning, a hidden onboarding/setup gap, or an embedded high-risk approval path. The
publication gate holds final widening while any release-blocking surface is below the
cutline. The full breakdown is published in the narrowing report.

## Where the audiences read it

Release center, Help/About, support exports, and docs/public-truth publication all
ingest the canonical matrix below instead of cloning status text, so every audience
sees the same final M5 posture.

## Canonical artifacts and gate

- Boundary schema: `schemas/governance/m5_xt12_certification_matrix.schema.json`
- Certification matrix (structured): `artifacts/release/m5/xt12-qualification-matrix.json`
- Validation capture: `artifacts/release/m5/captures/xt12-qualification-matrix_validation_capture.json`
- Evidence index (human-readable): `artifacts/release/m5/xt12-evidence-index.md`
- Narrowing report: `artifacts/release/m5/xt12-narrowing-report.md`
- Regenerator: `tools/regenerate_xt12_certification_matrix.py`
- CI gate: `tools/ci/m5/xt12_certification_check.py`
- Negative fixtures: `fixtures/release/m5/xt12-promotion/`

The matrix, capture, evidence index, and narrowing report are generated; do not
hand-edit them. Change the regenerator or its canonical inputs and rerun
`python3 tools/regenerate_xt12_certification_matrix.py`.

## Bound certification evidence

| Dimension | Evidence | Doc |
| --- | --- | --- |
| Onboarding/migration | `artifacts/compat/m5/migration-reports/m5_depth_import_report.md` | `docs/m5/migration-depth-lanes.md` |
| First-useful-work | `artifacts/ux/m5/first-useful-work-packets/m5_entry_routes_packet.md` | `docs/m5/first_useful_work.md` |
| Command discoverability | `artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md` | `docs/ux/m5/command_parity_audit.md` |
| Component-state parity | `artifacts/ux/m5/component-state-audit/m5_component_state_audit.md` | `docs/m5/component-state-parity.md` |
| Appearance conformance | `artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md` | `docs/m5/appearance-and-density-parity.md` |
| Durable attention | `artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md` | `docs/m5/durable-progress-and-reopen.md` |
| Notification privacy | `artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md` | `docs/m5/notification-privacy-and-badges.md` |
| Embedded boundary | `artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md` | `docs/m5/embedded-boundaries-and-auth.md` |
| Desktop conformance | `artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md` | `docs/m5/desktop-and-handoff-parity.md` |
| Accessibility/i18n | `artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md` | `docs/m5/accessibility-and-locale-depth.md` |
