# Canonical-reference rules

This document explains how Aureline keeps the growing control-artifact
set anchored to its source documents, and how the linter enforces that
discipline mechanically.

Companion artifacts:

- [`/artifacts/governance/source_anchor_map.yaml`](../../artifacts/governance/source_anchor_map.yaml)
  — canonical source-anchor map. Authoritative for tooling.
- [`/schemas/governance/source_anchor_map.schema.json`](../../schemas/governance/source_anchor_map.schema.json)
  — machine-readable contract.
- [`/tools/check_source_anchors.py`](../../tools/check_source_anchors.py)
  — linter entry point.
- [`/artifacts/governance/source_drift_reports/`](../../artifacts/governance/source_drift_reports/)
  — drift-report directory.
- [`/artifacts/governance/source_drift_reports/template.md`](../../artifacts/governance/source_drift_reports/template.md)
  — drift-report template.

If this narrative and the YAML disagree, the YAML wins for the linter
and the narrative must be updated in the same change.

## Why this exists

Aureline accumulates requirement ids, governance artifacts, and
packet families faster than any reviewer can track by hand. Each one
must remain anchored to a specific section of a source document under
`.t2/docs/`. Without that discipline:

- aliases drift into parallel requirement ids,
- packet-local labels turn into shadow obligations,
- copied prose becomes the real spec,
- and it becomes impossible to tell which tasks or packets need review
  when a source section changes.

The source-anchor map gives every governed row one binding source
anchor. The linter keeps that binding mechanical.

## Concepts

### Artifact classes

Every row in the map picks exactly one class from the closed vocabulary:

| `class_id` | Authority posture | What it covers |
|---|---|---|
| `requirement_row` | `claims_source_authority` | A canonical requirement id from the requirement register. |
| `control_artifact` | `claims_source_authority` | A row in the control-artifact index. |
| `packet_family` | `claims_source_authority` | A governance packet family (verification, benchmark_report, compatibility_report, claim_manifest, shiproom, waiver_register). |
| `shiproom_artifact` | `claims_source_authority` | A release, shiproom, or ring-progression artifact that participates in go/no-go. |
| `evidence_label` | `no_source_authority` | A non-authoritative label (scorecard call, fitness row, contract-family label, milestone-exit label) that resolves to one or more canonical rows. |

The class drives two things at once:

1. **Scoping a drift event.** When a source section changes, the drift
   report lists every affected row grouped by class, so re-baseline or
   waiver owners see "every affected requirement row, every affected
   control artifact, every affected packet family" rather than only the
   rows that happened to name the source doc in free text.
2. **Authority enforcement.** Rows whose class declares
   `authority_posture=claims_source_authority` MUST carry at least one
   canonical anchor. Rows in the `evidence_label` class MUST have
   `claims_source_authority=false` and no canonical anchors.

### Anchors

A canonical anchor is `{ doc_id, anchor, note }`.

- `doc_id` is a handle into `canonical_source_documents`.
- `anchor` is the section title or heading within that source doc.
- `note` is one sentence on why the section authorizes the row.

The linter does not try to interpret the section prose. It enforces
that the anchor exists in the declared source document and that every
`doc_id` resolves to a source document row whose `doc_ref` exists on
disk.

### Aliases

Aliases are lookup handles. They cover spec-local labels, legacy ids,
packet-local labels, scorecard calls, and fitness rows. They never
claim source authority on their own.

- `alias_id` MUST be unique across the whole map. Two rows cannot both
  own the alias `benchmark_governance`.
- `alias_id` MUST NOT collide with any `anchor_id` from another row.
- `alias_kind` picks from the closed vocabulary declared in the schema.
- The narrative behind an alias belongs in the `note` field, not in a
  free-text companion doc.

## The linter

Run the linter from the repository root:

```
python3 tools/check_source_anchors.py --repo-root .
```

Optional arguments:

- `--report <path>` writes a machine-readable JSON report with every
  finding, every class coverage count, and every unreachable source
  document.
- `--scenario <path>` replaces the on-disk map with a test scenario so
  CI can assert a specific failure shape without editing the real map.

Exit code:

- `0` when every check passes.
- `1` when any finding has severity `error`.

### Checks performed

| Check id | Severity | Description |
|---|---|---|
| `source_authority_requires_anchor` | error | Rows whose class declares `claims_source_authority` MUST carry at least one canonical anchor. |
| `source_authority_posture_matches_class` | error | A row may not declare `claims_source_authority=true` unless its class allows it, and a row with `claims_source_authority=false` must be in the `evidence_label` class. |
| `unique_anchor_ids` | error | `anchor_id` values are unique across the map. |
| `unique_alias_ids` | error | `alias_id` values are unique across the map and do not collide with any `anchor_id`. |
| `unknown_source_document` | error | Every `doc_id` used in a canonical anchor resolves to a row in `canonical_source_documents`. |
| `missing_source_document_file` | error | Every `canonical_source_documents[].doc_ref` exists on disk. |
| `requirement_id_coverage` | error | Every canonical requirement id in `artifacts/governance/requirement_register_seed.yaml` has a matching `requirement_row` in the map. |
| `orphaned_requirement_row` | error | Every `requirement_row` in the map corresponds to a canonical requirement id that exists in the register. |
| `class_coverage_gap` | error | At least one row exists in each `claims_source_authority` class. |

### Scenario format

Scenarios are JSON objects with the same top-level shape as the map
(`rows`, `canonical_source_documents`, etc.). A scenario only needs to
override the fields it wants to test; missing fields fall back to the
on-disk map. A scenario with
`{ "override_row": { "anchor_id": "req.ARCH-PACK-901",
"canonical_anchors": [] } }` drops the anchors from one row so the
linter emits the matching `missing_canonical_anchor` finding
deterministically.

## Drift-report workflow

When a source document changes materially (new obligation, renumbered
appendix, renamed section, removed anchor), the owner opens a drift
report under
[`/artifacts/governance/source_drift_reports/`](../../artifacts/governance/source_drift_reports/).
The template at
[`/artifacts/governance/source_drift_reports/template.md`](../../artifacts/governance/source_drift_reports/template.md)
captures:

- which source doc and which anchor changed;
- the run of the linter that detected (or would have detected) the
  drift;
- the affected rows, grouped by class (requirement rows, control
  artifacts, packet families, shiproom artifacts, evidence labels);
- which tasks, waivers, or exception packets inherit from those rows;
- the proposed re-baseline action or waiver, and the owner accountable
  for closing the drift.

Freeze-exception and re-baseline workflows cite one drift report id
instead of saying "docs changed". That way the exception packet carries
concrete source-drift evidence, not prose.

Routing rules:

- Changes that affect only `evidence_label` rows can be handled
  through the alias crosswalk without a separate drift report.
- Changes that affect any `claims_source_authority` class require a
  drift report before the map is re-baselined.
- Changes that affect more than one class require one drift report,
  not one per class — the class grouping is visible inside the report.

## Extending the map

- Add a new canonical requirement row to the register first; then add
  a matching `requirement_row` to the map.
- Add a new control artifact to the control-artifact index first; then
  add a matching `control_artifact` row if it claims source authority
  for a section that does not already have a requirement anchor.
- Add a new packet family to the governance packet template first;
  then add a matching `packet_family` row.
- Add an `evidence_label` row only when a scorecard call, fitness row,
  or contract-family label needs to stay visible but must not claim
  source authority.
- Update the map, this narrative, the drift-report directory, and any
  affected packet or scorecard in the same change.
