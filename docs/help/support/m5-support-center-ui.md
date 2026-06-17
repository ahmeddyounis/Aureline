# Support Center information architecture

The **Support Center layout** is the one authoritative contract for the Support Center as a coherent
product home. Where the [Support Center matrix](m5-support-center-matrix.md) governs *what each module
is* — its inspectors, data classes, redaction default, and export modes — this packet governs the
*shell the user navigates*: a three-region layout with a left-nav module rail, center diagnosis /
recovery / inspector / intake-export cards, and one shared right-side build/policy/residency/export
inspector that stays visible across every module.

- Typed model + gate: `aureline-support` crate, `m5_support_center_ui`
- Packet: `artifacts/support/m5/m5-support-center-ui.json`
- Reviewer artifact: `artifacts/support/m5/m5-support-center-ui.md`
- Schema: `schemas/support/m5-support-center-layout.schema.json`
- Fixtures: `fixtures/support/m5/m5-support-center-ui/`
- Shiproom review packet:
  `artifacts/shiproom/m5-support-center-ui-review-packet/support_center_ui_review_packet.md`

## Why this packet exists

Supportability had grown into a scatter of hidden pages and ad hoc entry points: Project Doctor in one
place, Safe mode in another, the performance / language / index / AI-usage / crash / network /
artifacts inspectors each reachable only by their own route, and issue-report / crash-intake and the
export preview somewhere else again. A blocked user who landed on one of them lost the execution
context that explained their finding the moment they navigated away.

This packet makes the Support Center a single product surface:

- It declares the **three layout regions** — the left-nav module rail, the center card stack, and the
  shared right-side inspector — and the accessibility invariants the chrome must hold.
- It reuses the **one module registry** (the same twelve `SupportModule` values the matrix governs) so
  the desktop shell, CLI/headless references, docs/help, and support exports all name the same
  surfaces. Each nav entry defers per-module readiness to its matrix row rather than restating it.
- It pins the **shared inspector** to four always-visible facets — `build`, `policy`, `residency`, and
  `export` — so the truth that explains a finding stays on screen as the user moves between modules.
- It runs a **fail-closed presentation gate** so an inaccessible surface or a dropped inspector facet
  narrows or withholds an entry automatically rather than leaving it navigable by inertia.

## The three regions

| Region | Role |
| --- | --- |
| `left_nav` | The module registry rail listing every Support Center module by its one registry name. |
| `center` | Diagnosis, recovery, inspector, and intake/export cards for the selected module. |
| `right_inspector` | The shared build/policy/residency/export inspector, kept visible across modules. |

Every region must satisfy every accessibility invariant — keyboard-complete navigation, high-contrast
parity, and reduced-motion-safe transitions — or it does not ship.

## The left-nav module registry

The rail groups the twelve modules into four sections, all named from the one registry:

- **Diagnose** — `doctor`.
- **Recover** — `safe_mode`, `bisect`.
- **Inspect** — `performance`, `language`, `index`, `ai_usage`, `crash`, `network`, `artifacts`.
- **Intake & export** — `issue_report_crash_intake`, `support_bundle_export_preview`.

Each entry names the existing source it reuses rather than duplicating — Project Doctor `finding_codes`,
crash-store `crash_ids`, `install_advisory_rows`, and `schema_registry_state` — so the center cards
never mint a second copy of supportability truth.

## The shared inspector

One right-side inspector keeps four facets visible across every module:

- `build` — exact-build identity and release channel.
- `policy` — which config/policy layer won and what it shadowed.
- `residency` — where the module's data is retained and under which residency rule.
- `export` — the redaction manifest and export-consent posture.

The inspector persists across module switches; it is never re-minted per module. Each facet is always a
visible pane, even when it is degraded or unwired, so the user always sees the build, policy, residency,
and export state behind the current finding.

## The fail-closed presentation gate

Each nav entry depends on a subset of the shared inspector's facets and declares the accessibility
guarantees its surface satisfies. The published **presentation** is the weaker of two ceilings:

- **Accessibility ceiling** — an entry that satisfies every invariant is presentable; an entry missing
  any invariant is **withheld**. Accessibility is a hard requirement: the Support Center never presents
  an inaccessible surface.
- **Required-facet ceiling** — a required facet that is `wired` is presentable, a `degraded` facet
  narrows the entry to a flagged, still-actionable surface, and an `unwired` facet withholds it.

The three published decisions are `presented`, `narrowed`, and `withheld`. When the gate narrows or
withholds an entry it records the headline reasons (`accessibility_unmet`, `inspector_facet_degraded`,
`inspector_facet_unwired`) and the recovery path (`restore_accessibility`, `restore_inspector_facet`,
or `none`). Accessibility is restored before an inspector facet, because it is the harder invariant. A
narrowed or withheld entry always names its recovery path, a caveat, and the unmet-or-unwired field
driving the narrowing; a withheld entry offers no actions; and a cleanly presented entry must be whole
— every invariant met, every required facet wired, nothing narrowing it.

The recorded presentation, reasons, and recovery path are recomputed and validated against the gate, so
a narrowing can never be asserted or hidden by hand (`M5SupportCenterLayout::validate()`).

## Downstream surfaces ingest one registry

Four consumer surfaces bind to this one registry: the desktop shell, CLI/headless references,
docs/help, and the support export. Each binding must ingest the registry, preserve its nav labels and
the shared inspector verbatim, and narrow with it, so a module withheld here cannot stay navigable on a
downstream surface, and no surface forks the inspector or the module vocabulary.

This layout is a supportability surface, not a replacement for the main workflow UI.
