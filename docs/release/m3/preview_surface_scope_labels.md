# Preview Surface Scope Labels

Notebook, voice, browser-companion, and preview-canvas surfaces must not
inherit stable wording from adjacent beta rows. Their release truth lives in:

- `artifacts/milestones/m3/claimed_surface_register.json`
- `artifacts/compat/m3/qualified_preview_rows.json`
- `fixtures/compat/m3/preview_scope_and_handoff/`

## Required Row Shape

Each qualified preview row carries:

- lifecycle label: `Preview` or `Beta`
- support label: `Limited`, `Experimental`, `Retest pending`, or `Unsupported`
- client-scope chip: `Desktop`, `Browser companion`, `Desktop + browser companion`, or `Unsupported`
- evidence freshness, evidence refs, and review window
- handoff target and limitation statement
- downgrade reason tokens and support-export-safe summary

The rows project to Start Center, docs/help, Help/About, service health,
the compatibility report, marketplace/help metadata, and support export.
Consumers must quote the generated packet instead of rewording the row
locally.

## Current Rows

| Surface | Lifecycle | Support | Client Scope | Handoff |
|---|---|---|---|---|
| Notebook workflow parity | Preview | Experimental | Desktop | Desktop notebook/source workflow |
| Voice and dictation | Preview | Experimental | Desktop | Desktop command review |
| Browser companion | Beta | Limited | Browser companion | Desktop native-depth workflow |
| Preview canvas | Preview | Limited | Desktop + browser companion | Desktop preview/source workflow |

## Downgrade Rules

The validator derives effective labels from evidence and gate state:

- missing evidence narrows to `Unsupported`
- stale evidence narrows to `Retest pending`
- incomplete required qualification gates narrow to the row's configured
  `Limited` or `Experimental` state
- stale or missing evidence also forces the lifecycle label to `Preview`
- browser companion and voice rows fail validation if they claim native-depth
  capability without a desktop handoff

## Validation

Refresh the packet and validation capture:

```bash
python3 ci/check_m3_qualified_preview_rows.py --repo-root .
```

CI should use:

```bash
python3 ci/check_m3_qualified_preview_rows.py --repo-root . --check
```

The fixture pack covers browser-companion desktop handoff, voice privileged
action handoff, stale-evidence downgrade, and missing-evidence downgrade.
