# Appearance-object certification

The milestone and technical-design docs turned appearance from broad visual
parity into specific, portable objects: a versioned theme package, a live
appearance session, a round-trip-safe token overlay, an imported-theme report,
and an extension appearance descriptor. Each family already has its own frozen
contract, seeded report, and fail-closed gate. This lane is the final row: it
certifies, for every claimed M5 desktop, extension-backed, and embedded surface,
that all five families stay honest *together*, and it publishes one canonical
appearance-object evidence index that release-center, Help/About, diagnostics,
support-export, and claim-publication surfaces consume instead of restating
appearance behavior by hand.

## Where the truth lives

| Artifact | Path |
| -------- | ---- |
| Typed source of truth | `crates/aureline-shell/src/appearance_object_certification/mod.rs` |
| Headless inspector | `cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- report` |
| Report fixture | `fixtures/ux/m5/appearance-object-certification/report.json` |
| Support-export fixture | `fixtures/ux/m5/appearance-object-certification/support_export.json` |
| Published report | `artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md` |
| Boundary schema | `schemas/ux/m5-appearance-object-certification.schema.json` |
| CI gate | `tools/ci/m5/appearance_object_certification_check.py` |

The records are inspectable, serde-serializable truth packets that carry no raw
token tables, raw screenshots, raw paths, or raw user content — only opaque
refs, closed vocabulary, counts, and short labels.

## The canonical appearance-object index

The report freezes one object-model index, one entry per family. Each entry
names the family's canonical schema, owned vocabulary group, shared contract
ref, and the source report id a consumer pivots to. Support and docs/help
surfaces read this index instead of re-deriving where appearance truth lives.

| Family (`object_family`) | Canonical schema | Source report |
| ------------------------ | ---------------- | ------------- |
| `theme_package` | `schemas/ux/m5-theme-package-manifest.schema.json` | `shell:m5_theme_packages:audit:v1` |
| `appearance_session` | `schemas/ux/appearance-session.schema.json` | `shell:m5_appearance_session:runtime:v1` |
| `token_overlay` | `schemas/ux/token-overlay.schema.json` | `shell:m5_token_overlays:portability:v1` |
| `theme_import_report` | `schemas/ux/m5-theme-import-report.schema.json` | `shell:m5_theme_import_report:v1:default` |
| `extension_appearance_descriptor` | `schemas/ux/extension-appearance-descriptor.schema.json` | `extensions:m5_appearance_descriptor:audit:v1` |

The index source report id is pulled straight from each family module's own
constant, so the index can never drift from the families it certifies.

## What every surface certification proves

Every claimed M5 surface carries one certification with a `family_certification`
per family. Each family certification records:

- the **qualification status** — `qualified` projects a certified family; a
  `not_applicable` family is one the surface never claims (a host-rendered
  surface with no extension); `explicitly_narrowed` / `platform_omitted` /
  `declared_capture_gap` are honest narrowings; `missing_evidence` and
  `unqualified_local_appearance` are blockers;
- the **compatibility state** — `current`, or a disclosed downgrade
  (`stale_evidence`, `unsupported_slot`, `partial_inheritance`,
  `restart_or_reload_required`). A non-current state is honest only when
  `downgrade_disclosed` is `true`;
- the **evidence freshness** and the **source report** the certification is
  backed by; a certified family cited against a report not in the index, or
  carrying stale evidence, is a blocker.

The claimed surfaces this lane certifies are exactly the appearance surface
families the appearance-parity contract already freezes; the lane broadens
certified scope to none beyond them: `notebook_cell_chrome`, `result_grid_row`,
`profiler_panel`, `trace_panel`, `pipeline_card`, `preview_route_badge`,
`docs_browser_pane`, `companion_surface`, `sync_status_surface`, and
`offboarding_surface`.

## How a surface is auto-narrowed

The certified claim scope is **derived**, never asserted. A surface drops from
`certified_full` to `certified_narrowed` the moment any family is honestly
narrowed or carries a disclosed downgrade, and is `blocked` if any family hides a
downgrade, is stale on a certified row, or claims appearance with no backing
evidence. A `not_applicable` family does not narrow a surface — the surface never
claimed it.

That derivation is the auto-narrowing: a claimed surface cannot keep marketing
full appearance stability once its underlying appearance objects go missing or
stale. Release/public-truth tooling reads `certified_claim_scope` and narrows or
blocks the row automatically.

## Consumers

Each surface certification carries a `release_center_ref`, `help_about_ref`,
`diagnostics_ref`, `support_export_ref`, and `claim_publication_ref` so the same
derived claim scope feeds release-center visibility, Help/About truth,
diagnostics inspection, support-export packets, and claim-publication manifests
without restating appearance behavior. The support-export wrapper quotes the
report id, exact-build ref, every index source report and schema, every
certification id, and every family evidence ref as a case id.

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- validate
cargo test -p aureline-shell --test m5_appearance_object_certification_fixtures
python3 tools/ci/m5/appearance_object_certification_check.py --repo-root .
```

Regenerate the report fixture, support-export fixture, compact lines, and
published markdown from the seed with the `report`, `support-export`, `compact`,
and `markdown` subcommands of the inspector.
