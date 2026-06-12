# M5 workflow-bundle and project-entry governance report — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-entry-and-bundle-governance.json`. The full contract and gate
semantics live in `docs/workspace/m5/m5-entry-and-bundle-governance.md`; the typed model lives
in the `aureline-workspace` crate (`m5_entry_and_bundle_governance`).

This artifact freezes the canonical M5 switching/entry matrix by aggregating the stable-line
entry and bundle lanes and publishing, for each lane, **only the assurance label its evidence
actually backs**. Unverified, probable, root-missing, partially-restored, stale, or
unsupported lanes are automatically narrowed to a bounded or retest-pending label, or refused,
before publication.

## Governance roll-up (as of 2026-06-11)

| Lane | Verb | Declared | Published label | Admission | Recovery |
| --- | --- | --- | --- | --- | --- |
| `workflow_bundle` | install | verified | **retest_pending** | admit_retest | refresh_bundle_scorecard |
| `source_acquisition` | clone | verified | **bounded** | admit_bounded | verify_source |
| `project_open` | open | verified | **verified** | admit_full | none |
| `project_import` | import | verified | **retest_pending** | admit_retest | confirm_archetype |
| `session_resume` | resume | verified | **retest_pending** | admit_retest | repair_restore |
| `recent_work` | open | verified | **verified** | admit_full | none |
| `workspace_admission` | open | retest_pending | **withheld** | refuse | withhold_claim |

Two lanes admit at full trust (`project_open`, `recent_work`), proving the gate is not a
blanket downgrade; one narrows to bounded, three to retest-pending, and one is refused. The
published label of every lane equals the gate's recomputed ceiling and never widens trust past
the weakest observed state.

## What the gate proves

- **Distinct verbs.** Clone, open, import, and resume stay distinct verbs pinned to their
  lanes, and bundle install is its own verb.
- **Trust never silently widens.** The probable, trusted-remote `source_acquisition` clone is
  bounded to its slice; the mixed `project_import` migration is held at retest-pending with all
  six downgrade reasons; the untrusted, root-missing `workspace_admission` target is refused.
- **First-useful-work stays explicit.** `source_acquisition` defers 3 setup steps
  (`setup_later`), `project_import` blocks on 5 (`blocked_on_setup`), and
  `workspace_admission` reports 2 missing roots (`missing_root`) — each distinct from a clean
  ready entry.
- **Bundle install always shows its diff and a rollback checkpoint** while a stale scorecard or
  experimental topology holds the label at retest-pending.

## Consumer surfaces

Release evidence, help/start-center, docs badges, and support export each bind to this one
packet, ingest it, preserve its labels and recovery paths, and narrow with it, so a row
narrowed here cannot stay stable downstream. The export projection carries typed states,
counts, and opaque refs only — no credential bodies, raw provider payloads, or workspace
contents.
