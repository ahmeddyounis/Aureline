# Source drift reports

One directory for drift reports raised against the source-anchor map at
[`/artifacts/governance/source_anchor_map.yaml`](../source_anchor_map.yaml).

A drift report is opened when a canonical source document under
`.t2/docs/` changes in a way that could invalidate a row in the
source-anchor map. The canonical-reference rules at
[`/docs/governance/canonical_reference_rules.md`](../../../docs/governance/canonical_reference_rules.md)
describe when a drift report is required and how re-baseline or waiver
decisions are routed.

## File naming

- Use `DR-YYYYMMDD-<short-slug>.md` (for example
  `DR-20260423-prd_appendix_p_renumber.md`).
- Keep one report per drift event. A drift event may affect several
  rows across several classes; the report groups affected rows by
  `artifact_class` inside the file instead of creating one file per
  class.

## Template

Copy [`template.md`](./template.md) when opening a new report.

## Linter integration

`tools/check_source_anchors.py` does not parse drift-report bodies. It
only checks that the directory exists and that the template file is
present. The drift-report narrative is reviewed by the owner named in
the report, not by the linter.
