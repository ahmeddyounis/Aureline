# M5 serialization qualification — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-serialization-qualification.json`. The full contract and gate semantics
live in `docs/workspace/m5/m5-serialization-qualification.md`; the typed model lives in the
`aureline-workspace` crate (`m5_serialization_qualification`).

This artifact qualifies the M5 serialization and restore-fidelity families by ingesting the
[serialization-and-restore matrix](m5-serialization-and-restore-matrix.md) and reusing its restore
fidelity vocabulary (`exact_restore`, `compatible_restore`, `layout_only`, `manual_review`). It
turns serialization and restore fidelity into **named qualification rows** — per family, per
profile, per deployment mode — instead of broad "restore supported" language, and **automatically
narrows** the claim wherever the matrix narrowed the surface, the evidence is stale or missing, or a
drill narrowed or failed.

## Qualification roll-up (as of 2026-06-16)

| Family | Profile | Mode | Matrix | Evidence | Published fidelity | Claim | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `remembered_state` | `desktop.stable` | `desktop` | exact_restore | current | **exact_restore** | published | none |
| `restore_fidelity` | `desktop.stable` | `desktop` | exact_restore | current | **exact_restore** | published | none |
| `restore_fidelity` | `managed.fleet` | `managed_fleet` | compatible_restore | current | **compatible_restore** | narrowed | adopt_matrix_narrowing |
| `portable_state_review` | `desktop.stable` | `desktop` | exact_restore | current | **exact_restore** | published | none |
| `portable_state_review` | `companion.browser` | `companion_browser` | exact_restore | aging | **compatible_restore** | narrowed | refresh_evidence |
| `migration_remap` | `desktop.beta` | `desktop` | compatible_restore | current | **compatible_restore** | narrowed | rerun_drills |
| `migration_remap` | `managed.fleet` | `managed_fleet` | layout_only | expired | **manual_review** | withheld | withhold_claim |
| `missing_surface_continuity` | `desktop.stable` | `desktop` | layout_only | current | **layout_only** | narrowed | rerun_drills |
| `missing_surface_continuity` | `companion.browser` | `companion_browser` | manual_review | missing | **manual_review** | withheld | withhold_claim |

Three rows publish a full claim (exact restore on desktop-stable remembered-state, restore-fidelity,
and portable-state-review), proving the qualifier is not a blanket downgrade; four rows are
automatically narrowed and two are withheld. The published fidelity of every row equals the gate's
recomputed ceiling and never exceeds the matrix claim.

## How each row narrows

- `restore_fidelity` / `managed.fleet` — every drill passes and evidence is current, but the
  serialization matrix already publishes `compatible_restore` for the remapped display geometry, so
  the qualification **adopts that narrowing** rather than re-asserting exact restore.
- `portable_state_review` / `companion.browser` — the companion/browser re-entry evidence is aging,
  so the claim is held at `compatible_restore` until the import-comparison evidence is refreshed.
- `migration_remap` / `desktop.beta` — the schema-jump drill narrowed on a forward-migrated import,
  capping the row at `compatible_restore` and pointing the owner at a drill rerun.
- `migration_remap` / `managed.fleet` — the schema-jump drill failed on an unmigratable schema, the
  matrix claim is `layout_only`, and the evidence is expired: the claim is **withheld** while the
  slot-preserving placeholder still holds. No exact restore is claimed.
- `missing_surface_continuity` / `desktop.stable` — a missing extension downgrades the affected
  panes to slot-preserving placeholder cards; layout is preserved (`layout_only`) and the
  missing-extension drill is queued for a rerun. Layout is never silently deleted.
- `missing_surface_continuity` / `companion.browser` — the missing-extension drill failed, the
  downgrade drill did not run, and provenance evidence is missing: the claim is **withheld** while
  the placeholder still holds.

## Invariants the gate enforces

- **No inheritance.** Each `(family, profile, deployment_mode)` row carries its own proof; a profile
  is never green because a nearby profile passed a superficially similar restore flow. The published
  fidelity can never exceed the matrix claim it narrows from.
- **Fail-closed.** A matrix-narrowed surface, stale or missing evidence, or an unproven, narrowed,
  or failed drill narrows or withholds the row automatically. Every `published_fidelity`,
  `claim_publication`, `downgrade_reasons`, and `downgrade_path` equals the recomputed gate.
- **No silent loss.** Missing-surface rows narrow to `layout_only` or `manual_review` with
  slot-preserving placeholders; the matrix's `silent_delete` is never reachable from a qualified
  row.
- **No over-portability.** Portable-state-review rows reuse the matrix redaction vocabulary; the
  qualification never claims full portability where a package depends on machine-local state or an
  unsupported feature pack.
- **One source of truth.** Docs/help, support export, companion/browser handoff, release center, and
  shiproom each bind to this one packet and narrow with it, so a row narrowed here cannot stay green
  downstream by inertia.
