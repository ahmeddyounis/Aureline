# Drift report: `DR-YYYYMMDD-<short-slug>`

<!--
Copy this file into a new drift report when a canonical source document
under `.t2/docs/` changes in a way that could invalidate a row in
`artifacts/governance/source_anchor_map.yaml`.

Companion:
  - docs/governance/canonical_reference_rules.md — when and how to open
    a drift report.
  - artifacts/governance/source_anchor_map.yaml — authoritative map.
  - tools/check_source_anchors.py — linter that detects missing anchors,
    orphan rows, duplicate aliases, and broken source-document refs.
-->

- **Report id:** `DR-YYYYMMDD-<short-slug>`
- **Opened on:** `YYYY-MM-DD`
- **Opened by:** `@handle`
- **Status:** `open` | `rebaselined` | `waived` | `closed`

## Source-document drift

- **Source document:** `.t2/docs/<filename>`
- **Source `doc_id`:** one of the `doc_id` values declared in
  `canonical_source_documents` inside the source-anchor map.
- **Anchor(s) that changed:**
  - `"<previous anchor / section title>"` →
    `"<new anchor / section title>"` or `removed`.
- **Nature of the change:**
  - `section_rename` | `section_removed` | `section_renumbered` |
    `obligation_narrowed` | `obligation_widened` |
    `requirement_id_renamed` | `prose_clarified_no_obligation_change`

## Linter evidence

- **Last clean linter run:** commit id or date.
- **First linter run that flagged the drift:** commit id or date, or
  `manual_detection` if the drift was noticed before the linter ran.
- **Findings cited:** list the `check_id` and `row_ref` values emitted
  by `tools/check_source_anchors.py --report <path>`.

## Affected rows by class

Group every affected `anchor_id` by the `artifact_class` of the owning
row in the source-anchor map. Use `none` for classes that are not
affected.

- **`requirement_row`:** `req.<ID>`, ...
- **`control_artifact`:** `control.<slug>`, ...
- **`packet_family`:** `packet.<family>`, ...
- **`shiproom_artifact`:** `shiproom.<slug>`, ...
- **`evidence_label`:** `evidence.<slug>`, ...

## Downstream tasks and packets

Name the tasks, scorecards, waivers, exception packets, or public
claims that quote the affected rows. Include the ids so the re-baseline
action is traceable.

## Proposed action

Pick one:

- `rebaseline_in_place` — update the affected anchors and notes in the
  source-anchor map to the new source-document section. Requires:
  linked change-set that updates the map in the same change as the
  rebaseline description here.
- `narrow_requirement_or_claim` — narrow the canonical row in the
  requirement register or the claim manifest to match the new
  source-document posture. Requires: link to the narrowing change.
- `open_waiver` — open a time-boxed waiver in
  `artifacts/governance/ownership_matrix.yaml` while a rebaseline is
  prepared. Requires: waiver id, expiry, and escalation path.
- `route_to_freeze_exception` — open a freeze-exception packet under
  `docs/governance/templates/exception_packet_template.md`. Requires:
  packet id and approving forum.

## Owner

- **Primary owner:** `@handle`
- **Backup owner or waiver:** `@handle` or active waiver id
- **Approving forum(s):** one or more forum ids from
  `artifacts/governance/ownership_matrix.yaml`.

## Decision

Fill in once the drift report closes.

- **Decided on:** `YYYY-MM-DD`
- **Outcome:** `rebaselined` | `narrowed` | `waived` | `rejected`
- **Closing change:** commit id, PR link, or packet id that landed the
  rebaseline / narrowing / waiver / rejection.
- **Linter status after close:** PASS or link to the linter report
  that shows zero findings for the affected rows.
