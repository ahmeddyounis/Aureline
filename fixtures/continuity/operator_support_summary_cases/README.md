# Operator/support continuity summary cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures`.

Every file validates against
`schemas/continuity/operator_support_continuity_summary.schema.json`
(`python3 tools/validate_m5_operator_support_continuity_summary_fixtures.py`).

The same page packet is checked in as the canonical artifact
`artifacts/m5/continuity/operator_support_continuity_summary.json` (a copy of
`page.json`).

## Files

- `page.json` — seeded clean page; every continuity summary names its exact row,
  discloses locality/tenant/key posture, labels its outage taxonomy, and is
  current, so the page qualifies `stable` and nothing narrows
- `summary.json` — seeded page summary record
- `support_export.json` — support-export wrapper for the seeded page
- `case_generic_wording_withdrawn.json` — a degraded relay summary uses generic
  "service degraded" wording when the exact row and fallback are known; the
  summary is withheld (`withdrawn`)
- `case_locality_undisclosed_beta.json` — a managed summary stops disclosing
  storage locality; the summary narrows to `beta`
- `case_evidence_stale_beta.json` — a self-hosted summary's backing continuity
  evidence is stale; the summary narrows to `beta`
- `case_evidence_missing_preview.json` — a sovereign summary's backing evidence
  is missing; the summary narrows to `preview`
- `case_admin_leak_withdrawn.json` — a summary carries admin-only routing and is
  not export-safe; the summary is withheld (`withdrawn`)
- `case_local_core_stays_green.json` — a managed summary loses its backing
  evidence (and narrows), but the local-core summary stays `stable` and never
  narrows or is withheld

## Regeneration

```sh
DIR=fixtures/continuity/operator_support_summary_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX case-generic-wording-withdrawn > $DIR/case_generic_wording_withdrawn.json
$EX case-locality-undisclosed-beta > $DIR/case_locality_undisclosed_beta.json
$EX case-evidence-stale-beta > $DIR/case_evidence_stale_beta.json
$EX case-evidence-missing-preview > $DIR/case_evidence_missing_preview.json
$EX case-admin-leak-withdrawn > $DIR/case_admin_leak_withdrawn.json
$EX case-local-core-stays-green > $DIR/case_local_core_stays_green.json
cp $DIR/page.json artifacts/m5/continuity/operator_support_continuity_summary.json
```
