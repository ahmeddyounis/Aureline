# M5 theme-package and appearance objects (companion doc)

This page is the companion to the M5 theme-import-parity qualification audit. It
freezes the canonical appearance **object model** so every new M5 surface
inherits one governed representation of theme packages, live appearance
sessions, token overlays, imported-theme reports, and extension appearance
descriptors — instead of inventing feature-local theme semantics. Appearance
stays semantic-first: a theme package may change look, but not meaning; an
appearance change may downgrade, but only when the downgrade is disclosed; and
no claimed M5 surface may hide trust, severity, or lifecycle meaning behind a
theme.

The audit data, the per-surface bindings, the per-row coverage numbers, the
object-model index, and the narrowable-marketed-rows list all come from one
checked-in truth path — the interop report — so the live shell appearance
inspector, the docs/help and support-export surfaces, the cross-surface
hardening matrix, the release-center packets, and the CI gate never disagree on
what each M5 surface certifies for imported themes, token overlays, and
extension inheritance.

Authoritative artifacts:

- [`/artifacts/ux/m5/theme-import-parity/m5_theme_import_parity_audit.md`](../../artifacts/ux/m5/theme-import-parity/m5_theme_import_parity_audit.md)
  — the rendered audit (`artifacts/ux/m5/theme-import-parity/m5_theme_import_parity_audit.md`).
- [`/fixtures/ux/m5/theme-package-interop/report.json`](../../fixtures/ux/m5/theme-package-interop/report.json)
  — the JSON snapshot (`fixtures/ux/m5/theme-package-interop/report.json`) every surface consumes.
- [`/fixtures/ux/m5/theme-package-interop/support_export.json`](../../fixtures/ux/m5/theme-package-interop/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/ux/m5-theme-import-parity.schema.json`](../../schemas/ux/m5-theme-import-parity.schema.json)
  — the boundary schema (`schemas/ux/m5-theme-import-parity.schema.json`) the fixtures conform to.
- [`/tools/ci/m5/theme_import_parity_check.py`](../../tools/ci/m5/theme_import_parity_check.py)
  — the CI gate (`tools/ci/m5/theme_import_parity_check.py`) that keeps the audit fresh and honest.

## The frozen object-model index

The matrix mints no parallel appearance object. It freezes a canonical
**object-model index**: each appearance-object family names exactly one
already-frozen schema and the vocabulary group it owns. Every new M5 surface
inherits these objects by reference rather than re-declaring theme semantics.

| Object family | Canonical schema | Record kind |
| ------------- | ---------------- | ----------- |
| `theme_package` | `schemas/ux/theme_package_manifest.schema.json` | `theme_package_manifest_record` |
| `appearance_session` | `schemas/ux/appearance_checkpoint.schema.json` | `appearance_session_record` |
| `token_overlay` | `schemas/design/token_overlay.schema.json` | `token_overlay_record` |
| `theme_import_report` | `schemas/ux/theme_import_report.schema.json` | `theme_import_report_record` |
| `extension_appearance_descriptor` | `schemas/design/extension_ui_appearance_descriptor.schema.json` | `extension_ui_appearance_descriptor_record` |

The CI gate fails if the index omits a family or points at a schema that was
renamed or removed, so the freeze can never drift away from the objects it
governs.

## The five parity rows

The audit certifies exactly five parity rows — one per object family — that
every registered M5 surface must report:

| Row | Dimension | Meaning |
| --- | --------- | ------- |
| `theme_package_compatibility` | theme_package | The surface honours the active theme package's supported modes and version range. |
| `appearance_session_integrity` | appearance_session | The surface reflects the effective appearance session, and any restart-or-reload-required change is disclosed. |
| `token_overlay_validation` | token_overlay | Token overlays validate; unsupported or inert tokens are disclosed, never silently dropped. |
| `imported_theme_parity` | import_report | Imported-theme mapping is honest: unsupported and unresolved slots are disclosed with a rollback path. |
| `extension_surface_inheritance` | extension_descriptor | Extension and embedded surfaces disclose theme, contrast, density, focus, and reduced-motion inheritance gaps. |

For every registered M5 surface, each row carries a qualification binding. The
qualification status is one of:

- `qualified` — the row is qualified with disclosed downgrade truth and captured
  evidence (a compatibility state, an object ref, and fresh evidence).
- `explicitly_narrowed` — the surface narrows this row but names a
  `narrowing_reason`.
- `not_applicable` — the row does not apply to this surface (e.g. a first-party
  surface that hosts no extension content); a reason is named.
- `platform_omitted` — the row is not surfaced on this client/platform; a reason
  is named.
- `declared_capture_gap` — a surface declares a known inheritance or capture gap
  honestly, with a reason, instead of silently shipping an un-qualified row.
- `hidden_downgrade` — the surface hides an appearance-object downgrade. **Always
  blocking.**
- `missing_evidence` — a marketed row is claimed with no captured evidence.
  **Always blocking.**

A surface is "high-salience" when its descriptor pins a semantic salience of
`lifecycle_bearing`, `trust_bearing`, or `severity_bearing` — i.e. it conveys
lifecycle, trust, or severity meaning. A high-salience surface must keep that
meaning legible across every theme package, density, and contrast mode.

## One compatibility and downgrade vocabulary

The matrix publishes one closed compatibility and downgrade vocabulary that
every surface and every consumer reuses, so stale evidence, unsupported slots,
partial inheritance, and restart-or-reload-required changes always read the
same way:

- `current` — fully supported, fully inherited, fresh evidence.
- `stale_evidence` — captured evidence has aged out.
- `unsupported_slot` — one or more slots or modes are unsupported.
- `partial_inheritance` — an extension or embedded surface inherits only part of
  the appearance posture.
- `restart_or_reload_required` — the appearance change applies only after a
  surface reload or an app restart.

A non-`current` state is honest **only** when the downgrade is disclosed in
product, export, and diagnostics. A disclosed downgrade still qualifies; an
undisclosed one is a blocker.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `hidden_downgrade`, `missing_evidence` — a hidden appearance-object downgrade,
  or a marketed row with no evidence.
- `token_silently_dropped`, `unresolved_mapping_hidden`, `inheritance_gap_hidden`
  — a silently dropped overlay token, a hidden unresolved import mapping, or a
  hidden extension inheritance gap.
- `rollback_path_missing` — an applied or rolled-back import, or a rolled-back
  overlay, with no rollback path.
- `restart_reload_undisclosed` — a restart-or-reload-required change that is not
  disclosed.
- `parity_claim_without_report` — an imported-theme parity claim with no report.
- `stale_evidence_on_marketed_row` — stale evidence on a marketed row.
- `object_model_index_drift` — the frozen object-model index omits a family or
  points at a missing schema.
- `dimension_drift`, `missing_narrowing_reason`, `missing_projection` — a binding
  whose dimension or family disagrees with its row, a narrowed row with no
  reason, or a qualified row missing required disclosed evidence.
- `descriptor_missing_appearance_anchor`, `missing_accessibility_note`,
  `surface_not_on_appearance_session` — a descriptor with no appearance anchor or
  accessibility note, or a surface that paints its own appearance outside the
  shared appearance-session model.

## Consuming the audit and narrowing marketed rows

The cross-surface hardening matrix, the docs/help and support-export surfaces,
and the release-center packets ingest the checked-in `report.json` directly when
qualifying or narrowing a marketed M5 row instead of cloning status text. The
report's `narrowable_marketed_rows` list names every marketed surface row whose
appearance evidence is stale or whose downgrade is hidden, so release tooling can
narrow that marketed M5 row before publication. In the clean checked-in audit
that list is empty. The same report is the canonical source the sync/import,
extension-inspection, and support-export surfaces consume — they read these
objects rather than restating appearance behaviour manually.

## Verification

```sh
python3 tools/ci/m5/theme_import_parity_check.py --repo-root .
```
