# Freeze the M5 package-management component matrix

This document describes the canonical **M5 package-management component matrix**
— the single frozen record that pins the reusable component vocabulary and state
model for package browsing and dependency mutation, so more M5 package surfaces
cannot diverge into private scope, auth, or risk UI.

It is the user-facing companion to:

- the typed model in the `aureline-deps` crate
  (`freeze_the_m5_package_management_component_matrix`),
- the boundary schema at
  `schemas/ui/m5-package-management-component-matrix.schema.json`,
- the checked release evidence at
  `artifacts/release/m5-package-management-proof/`, and
- the protected narrowed fixtures at
  `fixtures/ui/m5-package-management-components/`.

Where the
[package-state matrix](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
freezes the *data* vocabulary (package-state labels, manifest scope, registry
source, auth mode, lockfile authority, rollback class), this matrix freezes the
*component* vocabulary — the reusable rows, switchers, sheets, notices, cards,
planners, and strips users actually rely on while browsing, planning, reviewing,
or rolling back package changes. Both are the source of truth the whole package
lane references by id rather than re-inventing.

## The eight components

Each component is a `M5PackageComponent` and each row in the matrix binds it to a
maturity class, disclosure obligations, a registry-degradation narrowing
vocabulary, evidence requirements, downgrade triggers, a rollback posture, source
contracts, and consumer surfaces.

| Component | Maturity | Primary truth it carries |
| --- | --- | --- |
| `package_explorer_row` | stable | Owning manifest + direct/transitive/workspace-local relation while browsing |
| `manifest_scope_switcher` | stable | Exact target manifest + explicit confirmation before a whole-workspace mutation |
| `install_review_sheet` | stable | Scope, script/native-build risk, resolver identity, and lockfile churn before commit |
| `registry_or_mirror_row` | stable | Public/private/mirror/cache/offline source + auth posture, never a token body |
| `script_risk_notice` | beta | Whether a mutation may run install scripts or a native build, before install |
| `lockfile_impact_card` | stable | Quantified lockfile diff / blast radius without understating churn |
| `grouped_update_planner` | preview | Grouped-update reason + constraint/conflict cards before a batch applies |
| `rollback_checkpoint_strip` | stable | Durable checkpoint identity + revert/open-diff/export-patch recovery |

Six components hold a Stable claim; `script_risk_notice` is narrowed to Beta and
`grouped_update_planner` to Preview. Each component's
`canonical_source_contract_ref()` maps to a real frozen upstream schema, and
every row must list that ref among its `source_contract_refs` — a component can
never be re-homed onto generic manage-package chrome that hides its canonical
source of truth.

## Per-row disclosure obligations

Every row carries, in distinct fields, the exact disclosure it must preserve
wherever it is projected:

- **`manifest_scope_disclosure`** — the target manifest and direct/transitive/
  workspace-local relation, never a flat unscoped list.
- **`registry_source_disclosure`** — the registry, mirror, cache, or offline
  snapshot the answer came from, never collapsed into a generic "connected".
- **`auth_posture_disclosure`** — the credential mode and whether auth was
  satisfied, never a token body or private URL.
- **`script_native_build_disclosure`** — whether install scripts or a native
  build may run, never downgrading unknown risk to none.
- **`lockfile_churn_disclosure`** — the lockfile blast radius, never understating
  a broad regeneration as a single-line pin change.
- **`rollback_checkpoint_disclosure`** — the durable checkpoint identity and
  reachability, never a generic undo.

Each has its own validation violation, so a row that drops any one disclosure
fails the matrix rather than silently shipping a thinner component.

## Registry-degradation narrowing vocabulary

Every row preserves a `degradation_narrowing_vocab` drawn from
`M5PackageComponentDegradationState` — `resolved_exact`, `manifest_range_only`,
`mirror_backed`, `offline_snapshot_only`, `auth_required_unsatisfied`, and
`unknown_or_stale`. This keeps a mirror-backed, offline-snapshot, auth-required,
or stale answer from being flattened into a generic "installed" or "not found"
message, and a range-only resolution from being presented as an exact pin.

## Trust, projection, and freshness

The `trust_review` block records twelve recomputed invariants (manifest scope,
direct/transitive state, registry source, auth posture, script/native-build
risk, lockfile churn, grouped-update reason, rollback/checkpoint identity, and
mirror/offline continuity all stay explicit; one-click language never conceals
scope or risk; downgrade narrows instead of hides; stale rows block promotion).
The `consumer_projection` block asserts each component and the CLI/support-export
surfaces show component truth. The `proof_freshness` block records the SLO and
last refresh so a stale proof auto-narrows the affected component.

## What stays outside the boundary

Raw manifest bodies, raw lockfile bodies, registry credentials, private registry
URLs, and live registry responses never cross this boundary. The export is
metadata-only and is checked by a heuristic that rejects obvious credential
material.

## Regenerating the evidence

The checked support export, Markdown summary, and narrowed fixtures are
regenerated from the seed builder in the crate's tests:

```
GEN_PACKAGE_MANAGEMENT_COMPONENT_ARTIFACTS=1 \
  cargo test -p aureline-deps --lib \
  freeze_the_m5_package_management_component_matrix::tests::generate_artifacts
```

`checked_support_export_matches_seed` then asserts the checked JSON equals the
seed packet, so the evidence can never drift from the typed model.
