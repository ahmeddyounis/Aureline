# Shiproom review packet — M5 Support Center matrix

This packet is the shiproom- and release-center-facing view of the Support Center matrix. It does not
maintain its own summary: the claim scope below is read from the canonical matrix packet and narrows
automatically when a module goes stale, loses an inspector, or lacks consent.

## Canonical inputs

- Matrix packet: `artifacts/support/m5/m5-support-center-matrix.json`
- Reviewer artifact: `artifacts/support/m5/m5-support-center-matrix.md`
- Schema: `schemas/support/m5-support-center-matrix.schema.json`
- Companion doc: `docs/help/support/m5-support-center-matrix.md`
- Fixtures: `fixtures/support/m5/m5-support-center-matrix/`
- Typed model + gate: `aureline-support` crate, `m5_support_center_matrix`

- Claim publishable: **yes**
- Published modules: `5`
- Narrowed modules: `5`
- Withheld modules: `2`

## Claim scope

| Module | Evidence | Readiness | Claim | Recovery |
| --- | --- | --- | --- | --- |
| `doctor` | current | **operational** | published | none |
| `safe_mode` | aging | **degraded** | narrowed | refresh_evidence |
| `bisect` | current | **degraded** | narrowed | restore_inspector |
| `performance` | current | **inspect_only** | published | none |
| `language` | current | **degraded** | narrowed | resolve_consent |
| `index` | expired | **inspect_only** | narrowed | refresh_evidence |
| `ai_usage` | current | **inspect_only** | narrowed | resolve_consent |
| `crash` | current | **operational** | published | none |
| `network` | current | **unavailable** | withheld | withhold_module |
| `artifacts` | missing | **unavailable** | withheld | withhold_module |
| `issue_report_crash_intake` | current | **operational** | published | none |
| `support_bundle_export_preview` | current | **operational** | published | none |

## Sign-off gate

Promotion of the Support Center claim holds unless all of the following are true on the current matrix
(`M5SupportCenterMatrix::validate()` returns no violations):

1. Every Support Center module carries exactly one row; no module borrows a posture from a neighbour.
2. Every row's `published_readiness`, `module_publication`, `downgrade_reasons`, and `downgrade_path`
   equal the recomputed fail-closed gate — stale evidence, a degraded or unavailable inspector, or an
   ungranted/blocked consent narrows or withholds the module automatically.
3. No withheld module offers actions, and every narrowed or withheld module names its recovery path,
   its caveats, and the stale or missing fields driving the narrowing.
4. Support data classes stay redaction-safe: every module touching `high_risk` defaults to
   `excluded_always`, and every module that shares off-machine reuses the `export_consent` descriptor.
5. Local-save stays a first-class export mode beside team-share and formal-support; the three modes
   share one data-class, redaction, and consent vocabulary.
6. The five consumer bindings (desktop-shell, cli-headless, help-about, shiproom,
   formal-support-handoff) are all present and reuse this packet's published readiness, recovery
   paths, and narrowing.

A narrowed module is never silent: aging capability evidence, a degraded advisory descriptor, an
ungranted formal-support consent, an expired index probe, a blocked AI-transcript export, an
unavailable environment descriptor, and a missing artifact-graph probe each surface as their own
downgrade reason and recovery path rather than shipping as an implied operational module.

## Reviewer checklist

- [ ] `cargo test -p aureline-support m5_support_center_matrix` passes.
- [ ] The artifact validates against the schema (no schema/example drift).
- [ ] Four modules publish a full `operational` claim and one publishes cleanly at `inspect_only`,
      proving the gate is not a blanket downgrade.
- [ ] Each narrowed or withheld module names its downgrade reason, recovery path, and stale/missing
      fields.
- [ ] No live authority, secret, or high-risk material is embedded by default in any export mode.

## Regenerating this packet

This packet is checked in alongside the matrix it reviews. When the Support Center contract changes,
update the matrix, schema, reviewer artifact, and fixtures together, then re-run the gate before
re-reviewing:

```sh
cargo test -p aureline-support m5_support_center_matrix
python3 - <<'PY'
import json
from jsonschema import Draft202012Validator
schema = json.load(open("schemas/support/m5-support-center-matrix.schema.json"))
data = json.load(open("artifacts/support/m5/m5-support-center-matrix.json"))
errors = list(Draft202012Validator(schema).iter_errors(data))
print("schema OK" if not errors else errors)
PY
```
