# Support Center information architecture — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-support-center-ui.json`. The full contract and gate semantics live in
`docs/help/support/m5-support-center-ui.md`; the typed model lives in the `aureline-support` crate
(`m5_support_center_ui`).

This layout makes the Support Center a **single product home** instead of a scatter of hidden pages. It
declares a three-region shell — a left-nav module rail, center diagnosis / recovery / inspector /
intake-export cards, and one shared right-side `build`/`policy`/`residency`/`export` inspector that
stays visible across modules — reusing the one Support Center module registry. A fail-closed
presentation gate withholds any entry that is not keyboard-complete, high-contrast, and
reduced-motion-safe, narrows any entry that depends on a degraded shared-inspector facet, and withholds
any entry that depends on an unwired one.

## Regions (as of 2026-06-16)

| Region | Keyboard order | Accessible |
| --- | --- | --- |
| `left_nav` | 1 | yes |
| `center` | 2 | yes |
| `right_inspector` | 3 | yes |

## Shared inspector facets

| Facet | Availability | Truth source |
| --- | --- | --- |
| `build` | wired | exact-build identity and release channel |
| `policy` | degraded | which config/policy layer won and what it shadowed |
| `residency` | unwired | where the module's data is retained and under which rule |
| `export` | wired | redaction manifest and export-consent posture |

## Module roll-up (as of 2026-06-16)

| Module | Section | Surface | Presentation | Recovery | Required facets |
| --- | --- | --- | --- | --- | --- |
| `doctor` | diagnose | diagnosis_cards | **narrowed** | restore_inspector_facet | build, policy, export |
| `safe_mode` | recover | recovery_actions | **narrowed** | restore_inspector_facet | build, policy, export |
| `bisect` | recover | recovery_actions | **presented** | none | build, export |
| `performance` | inspect | inspector_readout | **narrowed** | restore_inspector_facet | build, policy, export |
| `language` | inspect | inspector_readout | **narrowed** | restore_inspector_facet | build, policy, export |
| `index` | inspect | inspector_readout | **withheld** | restore_inspector_facet | build, residency, export |
| `ai_usage` | inspect | inspector_readout | **withheld** | restore_inspector_facet | policy, residency, export |
| `crash` | inspect | inspector_readout | **presented** | none | build, export |
| `network` | inspect | inspector_readout | **withheld** | restore_accessibility | build, policy, export |
| `artifacts` | inspect | inspector_readout | **withheld** | restore_inspector_facet | build, residency, export |
| `issue_report_crash_intake` | intake_export | intake_and_export | **presented** | none | build, export |
| `support_bundle_export_preview` | intake_export | intake_and_export | **withheld** | restore_inspector_facet | build, policy, residency, export |

Three entries present cleanly (`bisect`, `crash`, `issue_report_crash_intake`), proving the gate is not
a blanket withhold; four narrow on the degraded `policy` facet; and five are withheld — four on the
unwired `residency` facet and `network` on its missing reduced-motion-safe transitions.

## Per-module notes

### doctor

Opens the diagnose section; reuses `finding_codes` and `install_advisory_rows`. Narrows on the degraded
shared `policy` facet while keeping finding cards and guided repair offered.

### safe_mode

Recover section; reuses `install_advisory_rows`. Narrows on the degraded `policy` facet; entry and
retained-capability review stay offered.

### bisect

Recover section; reuses `install_advisory_rows`. Presents cleanly — keyboard-complete, high-contrast,
reduced-motion-safe, and dependent only on wired `build`/`export` facets.

### performance

Inspect section; reuses `schema_registry_state`. Narrows on the degraded `policy` facet; timeline and
hot-spot views stay offered.

### language

Inspect section; reuses `schema_registry_state`. Narrows on the degraded `policy` facet; server state
and restart stay offered.

### index

Inspect section; reuses `schema_registry_state`. Withheld because it depends on the unwired `residency`
facet; returns once residency is wired.

### ai_usage

Inspect section; reuses `schema_registry_state`. Withheld on the unwired `residency` facet; the degraded
`policy` facet is also flagged for restore.

### crash

Inspect section; reuses `crash_ids` and `install_advisory_rows`. Presents cleanly on wired
`build`/`export` facets.

### network

Inspect section; reuses `schema_registry_state`. Withheld because its route-origin transitions are not
reduced-motion-safe; accessibility is restored first, then the degraded `policy` facet.

### artifacts

Inspect section; reuses `schema_registry_state`. Withheld on the unwired `residency` facet; provenance
review returns once residency is wired.

### issue_report_crash_intake

Intake/export section; reuses `crash_ids` and `finding_codes`. Presents cleanly on wired `build`/`export`
facets.

### support_bundle_export_preview

Intake/export section; reuses `schema_registry_state` and `finding_codes`. Withheld on the unwired
`residency` facet; the degraded `policy` facet is also flagged for restore.

## Sign-off gate

Promotion of the Support Center layout holds unless all of the following are true on the current packet
(`M5SupportCenterLayout::validate()` returns no violations):

1. Every Support Center module carries exactly one nav entry, named from the one registry; the three
   regions are all present and accessible; and the shared inspector persists and declares all four
   facets.
2. Every entry's `presentation`, `downgrade_reasons`, and `recovery_path` equal the recomputed
   fail-closed gate — a missing accessibility invariant or a degraded/unwired required facet narrows or
   withholds the entry automatically.
3. No withheld entry offers actions, and every narrowed or withheld entry names its recovery path, its
   caveat, and the unmet-or-unwired field driving the narrowing.
4. The four consumer bindings (desktop-shell, cli-headless, docs-help, support-export) are all present
   and reuse this packet's nav labels, shared inspector, and narrowing.

## Regenerating this packet

This packet is checked in alongside the layout it reviews. When the Support Center IA changes, update
the packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_support_center_ui
cargo run -p aureline-support --example dump_m5_support_center_ui
```
