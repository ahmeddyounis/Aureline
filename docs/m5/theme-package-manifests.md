# M5 theme-package manifests (companion doc)

This page is the companion to the M5 theme-package manifest audit. It freezes
how every new M5 surface declares its **active theme package** and **supported
appearance modes** through one shared, versioned manifest shape — instead of
inferring theme identity from screenshots or feature-local style code. Theme
packages are versioned artifacts, not loose CSS: supported modes, provenance,
semantic/component/syntax token sets, contrast metadata, and inheritance
expectations stay explicit and portable across desktop, import/export,
diagnostics, and support flows.

The manifests, the per-surface bindings, the per-package coverage, the
provenance index, and the narrowable-surface list all come from one checked-in
truth path — the audit report — so the live shell theme-provenance card, the
About/help and support-export surfaces, the diagnostics inspector, and the
release-center packets never disagree on which package and inheritance posture
each M5 surface actually applies.

Authoritative artifacts:

- [`/artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md`](../../artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md)
  — the rendered audit (`artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md`).
- [`/fixtures/ux/m5/theme-package-modes/report.json`](../../fixtures/ux/m5/theme-package-modes/report.json)
  — the JSON snapshot (`fixtures/ux/m5/theme-package-modes/report.json`) every surface consumes.
- [`/fixtures/ux/m5/theme-package-modes/support_export.json`](../../fixtures/ux/m5/theme-package-modes/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/ux/m5-theme-package-manifest.schema.json`](../../schemas/ux/m5-theme-package-manifest.schema.json)
  — the boundary schema (`schemas/ux/m5-theme-package-manifest.schema.json`) the fixtures conform to.
- [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
  — the canonical theme-package manifest object this audit re-exports by reference (`schemas/ux/theme_package_manifest.schema.json`).
- [`/tools/ci/m5/theme_package_manifest_check.py`](../../tools/ci/m5/theme_package_manifest_check.py)
  — the CI gate (`tools/ci/m5/theme_package_manifest_check.py`) that keeps the audit fresh and honest.

## Versioned theme-package manifests

Each registered manifest is a versioned artifact, not a screenshot. It carries:

- a stable `package_id`, a `package_version_label`, and a `package_revision_ref`;
- a `provenance_class` (built-in, extension-contributed, community-supplied,
  imported-translated, or air-gapped-offline) and a `signature_state`;
- the supported theme modes (`dark_reference`, `light_parity`,
  `high_contrast_dark`, `high_contrast_light`), density classes (`compact`,
  `standard`, `comfortable`), and motion postures (`motion_standard`,
  `motion_reduced`, and the additional postures the product already claims);
- the `semantic`, `component`, and `syntax` token sets it contributes, each as
  an opaque ref plus a token count;
- contrast metadata (AA / AAA normal-text results and a forced-colors-preserved
  flag);
- the supported design-token schema range and the build-`compatibility_state`;
  and
- the `inheritance_expectations` — the appearance axes (`theme`, `contrast`,
  `density`, `focus`, `reduced_motion`) the package expects consuming surfaces
  to honour.

The manifest object mints no parallel theme vocabulary: its theme, density,
motion, provenance, signature, and compatibility values are re-exported from the
canonical
[`schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
without modification. This lane adds the **manifest-audit** layer that binds
those packages to surfaces; it does not add new theme modes.

## The seven surface families

The audit binds every claimed M5 surface family to its active package:

| Surface family | Token |
| -------------- | ----- |
| Notebook | `notebook` |
| Result grid | `result_grid` |
| Profiler timeline | `profiler_timeline` |
| Preview / browser pane | `preview_browser_pane` |
| Docs / help pane | `docs_help_pane` |
| Companion surface | `companion_surface` |
| Extension-backed surface | `extension_backed_surface` |

For each surface the binding records the `active_package_id`, the honoured theme
/ density / motion modes, the `inheritance_posture` and any
`disclosed_inheritance_gaps`, a `provenance_disclosed` flag, and the disclosed
`evidence_state` (`current`, `stale_evidence`, or `disabled_package`). A surface
that paints its own appearance outside the shared appearance-session model is a
blocker.

## One disclosed-downgrade vocabulary

A non-`current` evidence state is honest **only** when it is disclosed in
product, export, and diagnostics:

- `current` — fully supported, fully inherited, fresh evidence.
- `stale_evidence` — captured appearance evidence has aged out. A stale state on
  a marketed surface is a blocker so release tooling can narrow that surface.
- `disabled_package` — the active package is disabled (e.g. its signature failed
  or the author revoked it); the surface must disclose the disabled state and
  fall back, never render a disabled package silently.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `active_package_unknown` — a surface names a package not in the registry.
- `unsupported_mode_claimed` — a surface honours a theme, density, or motion
  mode the package does not support.
- `inheritance_gap_hidden` — a surface neither inherits nor discloses an axis the
  package expects (a hidden appearance downgrade).
- `provenance_not_disclosed`, `disabled_package_rendering_undisclosed`,
  `stale_evidence_on_marketed_surface` — an undisclosed provenance, a disabled
  package still rendering without disclosure, or stale evidence on a marketed
  surface.
- `surface_not_on_appearance_session` — a surface paints its own appearance
  outside the shared appearance-session model.
- `descriptor_missing_appearance_anchor`, `missing_accessibility_note`,
  `inheritance_posture_mismatch` — a descriptor with no appearance anchor or
  accessibility note, or a posture that disagrees with its disclosed gaps.
- `manifest_token_set_incomplete`, `manifest_missing_required_mode`,
  `manifest_signature_failed_still_registered` — a first-party manifest missing a
  semantic / component / syntax token set or a `dark_reference`, `light_parity`,
  or `motion_reduced` mode, or a signature-failed manifest still registered.

## Consuming the audit

About/help, the diagnostics inspector, and the support-export surfaces ingest
the checked-in `report.json` directly when surfacing a surface's active package
and provenance, instead of restating theme behaviour manually. The provenance
index gives those surfaces each package's signature, build-compatibility, and
most-degraded disclosed evidence state, so theme provenance and supported-mode
metadata survive export, diagnostics, and release-evidence generation. The
`narrowable_marketed_surfaces` list names every marketed surface whose
appearance evidence is stale or whose package is disabled, so release tooling can
narrow that surface before publication. In the clean checked-in audit that list
is empty.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- validate
cargo test -p aureline-shell --test m5_theme_package_fixtures
python3 tools/ci/m5/theme_package_manifest_check.py --repo-root .
```
