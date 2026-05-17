# Reference-Workspace Claim Integration

The beta reference-workspace report is the compatibility proof source for
archetype support classes. Claim rows, archetype scorecards, Help/About,
service health, release evidence, partner packets, and support exports must
consume the same generated state:

- Machine report: `artifacts/compat/m3/reference_workspace_report.json`
- Reviewer report: `artifacts/compat/m3/reference_workspace_report.md`
- Docs copy: `docs/compat/m3/reference_workspace_report.md`
- Badge projection: `artifacts/compat/m3/reference_workspace_badges.json`
- Validation capture: `artifacts/compat/m3/captures/reference_workspace_report_validation_capture.json`

## Integration Rule

`ci/check_m3_reference_workspace_report.py` compares every
`beta_archetype_binding` row in `artifacts/release/m3/claim_manifest.json`
with the matching report row. Publication fails when a claim row's effective
support class is greener than the report. This is the widening block for stale,
missing, failed, blocked, or not-run reference-workspace evidence.

The scorecard validator also reads the report and writes the derived cap into
`artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json`.
`ci/check_m3_claim_manifest.py` then derives the claim manifest from that
register. In the current report, every beta archetype is `retest_pending`
because no current harness row has a `pass` result.

## Surface Mapping

| Surface | Consumes | Required behavior |
|---|---|---|
| Claim manifest | `cohort_archetype_scorecard_register.json` | Effective support cannot exceed the report row. |
| Archetype scorecards | `reference_workspace_report.json` | Downgrade triggers quote `reference_workspace_report:retest_pending`, `:evidence_stale`, or the failing state. |
| Help/About and service health | `claim_manifest.json` plus `reference_workspace_badges.json` | Show the same support and freshness labels as release evidence. |
| Release packet | `reference_workspace_report.json` and `.md` | Carry report refs with compatibility and known-limit packets. |
| Partner/support exports | `reference_workspace_badges.json` | Export badge labels and report row refs without private tooling. |

## Refresh Order

```sh
python3 ci/check_m3_reference_workspace_report.py --repo-root .
python3 ci/check_cohort_archetype_scorecards.py --repo-root .
python3 ci/check_m3_claim_manifest.py --repo-root .
python3 ci/check_m3_reference_workspace_report.py --repo-root . --check
```
