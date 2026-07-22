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

Shell consumers load this generated packet through a bounded artifact reader.
The packet must resolve as a regular file with no untrusted ancestor or
final-component redirect observed by the loader. The only ancestor-redirect
exceptions are the exact macOS platform pairs `/var` -> `/private/var` and
`/tmp` -> `/private/tmp`; every other Unix redirect fails closed. A parent
metadata stability token, the open descriptor, and the resolved path are
captured and rechecked around the read. On Windows that parent token comprises
the stable Rust 1.75 attributes and creation time; it is not a unique object
identifier. Mutable directory last-write metadata is excluded because staging
a child changes it. The packet must fit within 4
MiB and contain no more than 4,096 product rows or 4,096 support-export rows.
Exceeding any bound fails the consumer closed as
unavailable/downgraded truth; diagnostics name the artifact class and limit
without echoing a private local path. These portable path-based checks reject
visible parent replacement but do not claim race-free dirfd semantics against
a swap and restoration wholly between checks.

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
