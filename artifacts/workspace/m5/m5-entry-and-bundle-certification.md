# M5 entry-and-bundle certification report — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-entry-and-bundle-certification.json`. The full contract and gate
semantics live in `docs/workspace/m5/m5-entry-and-bundle-certification.md`; the typed model
lives in the `aureline-workspace` crate (`m5_entry_and_bundle_certification`).

This artifact certifies the M5 switching and project-entry depth lanes by ingesting the
[entry-and-bundle governance matrix](../../../docs/workspace/m5/m5-entry-and-bundle-governance.md)
and graduating each lane **only where its evidence is current and provable**. Stale, unproven,
or governance-narrowed lanes are automatically downgraded to a narrower label before publication.

## Certification roll-up (as of 2026-06-11)

| Lane | Governance claim | Evidence | Published label | Decision | Recovery |
| --- | --- | --- | --- | --- | --- |
| `workflow_bundle` | retest_pending | current | **retest_pending** | admit_retest | adopt_governance_narrowing |
| `source_acquisition` | bounded | current | **bounded** | admit_bounded | rerun_drills |
| `project_open` | verified | current | **verified** | admit_full | none |
| `project_import` | retest_pending | expired | **retest_pending** | admit_retest | refresh_evidence |
| `session_resume` | retest_pending | aging | **retest_pending** | admit_retest | rerun_drills |
| `recent_work` | verified | current | **verified** | admit_full | none |
| `workspace_admission` | withheld | missing | **withheld** | refuse | withhold_row |

Two lanes certify verified (`project_open`, `recent_work`), proving the certifier is not a
blanket downgrade; five lanes are automatically narrowed or withheld. The published label of
every lane equals the gate's recomputed ceiling and never exceeds the governance claim.

## How each lane narrows

- `workflow_bundle` — every drill passes and evidence is current, but the governance matrix
  already published retest_pending for the bundle-install lane, so the certification **adopts
  that narrowing** rather than re-asserting a broader claim.
- `source_acquisition` — the clone/open/import/resume drill narrowed on the unverified-remote
  slice, capping the certification at bounded and pointing the owner at a drill rerun.
- `project_open` / `recent_work` — verified governance, current evidence, all drills passed:
  certified whole.
- `project_import` — the import-fidelity evidence is expired, so the row is held at
  retest_pending until the evidence is refreshed.
- `session_resume` — evidence is aging and the source-acquisition drill narrowed on partial
  restore, so the row is held at retest_pending pending a drill rerun.
- `workspace_admission` — governance withheld, evidence missing, and the admission drill failed
  (with the downgrade drill never run), so the row is withheld and offers no supported profile.

## Consumer surfaces

`start_center`, `migration_center`, `help_about`, `release_center`, `docs_help`, and
`support_export` each bind to this packet and narrow with it. A lane narrowed here cannot stay
green on a start-center tile, a migration banner, a help/About line, a release-evidence row, a
docs badge, or a support export.

## Guardrail

No blanket "best-in-class onboarding" or "one-click project entry" copy is published without
row-level qualification, freshness, and a downgrade path. The packet is metadata-only and carries
no credential bodies, raw provider payloads, or workspace contents.
