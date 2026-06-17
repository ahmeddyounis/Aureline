# Shiproom review packet — Support Center information architecture

This packet is the shiproom- and release-center-facing view of the Support Center layout. It does not
maintain its own summary: the claim scope below is read from the canonical layout packet and narrows
automatically when an entry loses an accessibility invariant or a required shared-inspector facet.

## Canonical inputs

- Layout packet: `artifacts/support/m5/m5-support-center-ui.json`
- Reviewer artifact: `artifacts/support/m5/m5-support-center-ui.md`
- Schema: `schemas/support/m5-support-center-layout.schema.json`
- Companion doc: `docs/help/support/m5-support-center-ui.md`
- Fixtures: `fixtures/support/m5/m5-support-center-ui/`
- Typed model + gate: `aureline-support` crate, `m5_support_center_ui`

- Claim publishable: **yes**
- Presented entries: `3`
- Narrowed entries: `4`
- Withheld entries: `5`

## Claim scope

| Module | Section | Presentation | Recovery |
| --- | --- | --- | --- |
| `doctor` | diagnose | **narrowed** | restore_inspector_facet |
| `safe_mode` | recover | **narrowed** | restore_inspector_facet |
| `bisect` | recover | **presented** | none |
| `performance` | inspect | **narrowed** | restore_inspector_facet |
| `language` | inspect | **narrowed** | restore_inspector_facet |
| `index` | inspect | **withheld** | restore_inspector_facet |
| `ai_usage` | inspect | **withheld** | restore_inspector_facet |
| `crash` | inspect | **presented** | none |
| `network` | inspect | **withheld** | restore_accessibility |
| `artifacts` | inspect | **withheld** | restore_inspector_facet |
| `issue_report_crash_intake` | intake_export | **presented** | none |
| `support_bundle_export_preview` | intake_export | **withheld** | restore_inspector_facet |

## Sign-off gate

Promotion of the Support Center layout holds unless all of the following are true on the current packet
(`M5SupportCenterLayout::validate()` returns no violations):

1. Every Support Center module carries exactly one nav entry, named from the one module registry; no
   module borrows a neighbour's posture and none is missing.
2. The three regions (`left_nav`, `center`, `right_inspector`) are all present, carry unique keyboard
   orders, and satisfy every accessibility invariant; the shared inspector persists across modules and
   declares all four facets.
3. Every entry's `presentation`, `downgrade_reasons`, and `recovery_path` equal the recomputed
   fail-closed gate — a missing accessibility invariant or a degraded/unwired required facet narrows or
   withholds the entry automatically.
4. No withheld entry offers actions, and every narrowed or withheld entry names its recovery path, its
   caveat, and the unmet-or-unwired field driving the narrowing.
5. The four consumer bindings (desktop-shell, cli-headless, docs-help, support-export) are all present
   and reuse this packet's nav labels, shared inspector, and narrowing.

A narrowed or withheld entry is never silent: a degraded policy facet, an unwired residency facet, and a
missing reduced-motion-safe transition each surface as their own downgrade reason and recovery path
rather than shipping as an implied navigable surface. Accessibility is restored before an inspector
facet, because it is the harder invariant.

## Reviewer checklist

- [ ] `cargo test -p aureline-support m5_support_center_ui` passes.
- [ ] The artifact validates against the schema (no schema/example drift).
- [ ] Three entries present cleanly, proving the gate is not a blanket withhold.
- [ ] Each narrowed or withheld entry names its downgrade reason, recovery path, and unmet/unwired
      field.
- [ ] No live authority, secret, or raw private material is embedded in the support export.

## Regenerating this packet

This packet is checked in alongside the layout it reviews. When the Support Center IA changes, update
the packet, schema, reviewer artifact, and fixtures together, then re-run the gate before re-reviewing:

```sh
cargo test -p aureline-support m5_support_center_ui
python3 - <<'PY'
import json
from jsonschema import Draft202012Validator
schema = json.load(open("schemas/support/m5-support-center-layout.schema.json"))
data = json.load(open("artifacts/support/m5/m5-support-center-ui.json"))
errors = list(Draft202012Validator(schema).iter_errors(data))
print("schema OK" if not errors else errors)
PY
```
