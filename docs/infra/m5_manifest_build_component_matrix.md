# M5 manifest / build component matrix contract

This packet freezes one canonical family of reusable manifest and
build-confidence components so Milestone 5 stops depending on
feature-local infra/build chrome. It names, once, every governed
primitive later M5 rows reference by name instead of restating infra /
build confidence truth in feature-local prose: manifest editor headers,
schema / validator rows, target-context chip groups, resource-link and
resource-explorer rows, adapter source badges, target-graph rows,
capability matrices, raw-event drawers, and fallback-confidence drawers.

The matrix is minted and validated in
[`crates/aureline-infra`](../../crates/aureline-infra/src/freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix/mod.rs)
(`record_kind = m5_manifest_build_component_matrix`, `schema_version = 1`).
The Rust builder `seeded_manifest_build_component_matrix()` is the source
of truth; the checked-in artifacts below are byte-for-byte emissions of
it (`current_m5_manifest_build_component_matrix_export()` re-reads the
support export via `include_str!`).

If this doc, the machine-readable schema, and the checked-in artifacts
disagree, the schema plus the Rust builder win and all companion
artifacts update in the same change.

Companion artifacts:

- [`/schemas/ui/m5-manifest-build-component-matrix.schema.json`](../../schemas/ui/m5-manifest-build-component-matrix.schema.json)
  — boundary schema for the matrix packet, its components, per-family
  descriptors, guardrails, and consumer projection.
- [`/artifacts/infra/m5-manifest-build-component-matrix/support_export.json`](../../artifacts/infra/m5-manifest-build-component-matrix/support_export.json)
  — the `include_str!` canonical support export (release / support proof
  anchor).
- [`/artifacts/infra/m5-manifest-build-component-matrix/matrix.csv`](../../artifacts/infra/m5-manifest-build-component-matrix/matrix.csv)
  — flat CSV projection, one row per component.
- [`/artifacts/design/m5-manifest-build-component-matrix.md`](../../artifacts/design/m5-manifest-build-component-matrix.md)
  — human-readable Markdown summary.
- [`/fixtures/ui/m5-manifest-build-components/`](../../fixtures/ui/m5-manifest-build-components/)
  — checked-in fixture corpus (canonical packet JSON + CSV).

## Component families

Ten governed families, each defined exactly once. Every component row
carries the shared `truth_mode` truth class, a visible `target_context_ref`,
an explicit `adapter_source`, the six required labels, `export_safe` and
`assistive_ready` parity flags, an evidence trail, and — where a
degraded path exists — a specific (never generic) degraded label bound to
a downgrade trigger.

| Family | Governed primitive | Family-specific descriptor |
| --- | --- | --- |
| `manifest_editor_header` | Manifest editor header | truth mode, schema freshness, edit posture, target-context visibility |
| `schema_validator_row` | Schema / validator row | validation state, schema freshness, blocks-apply-on-error |
| `target_context_chip_group` | Target-context chip group | truth mode, target identity, context completeness, stays-visible-on-scroll |
| `resource_link_row` | Resource-link row | link class, from/to truth, confidence, never-overwrites-higher-confidence |
| `resource_explorer_row` | Resource-explorer row | truth mode, freshness, confidence, target-context visibility |
| `adapter_source_badge` | Adapter source badge | adapter source, confidence, source-kind-explicit |
| `target_graph_row` | Target-graph row | node kind, truth mode, edge confidence, target identity |
| `capability_matrix` | Capability matrix cell | capability state, adapter source, discloses-source-and-confidence, confidence |
| `raw_event_drawer` | Raw-event drawer | event channel, redaction-applied, preserves-event-identity |
| `fallback_confidence_drawer` | Fallback-confidence drawer | confidence state, fallback reason, recovery route, never-overwrites-structured-silently |

## Shared state vocabularies

All components bind the same authored / rendered / planned / live /
provider-overlay truth classes and degraded-state language used elsewhere
in Aureline. Closed vocabularies:

- **Truth mode** (`truth_mode`): `desired`, `rendered`, `plan`, `live`,
  `provider_overlay`.
- **Adapter source** (`adapter_source`): `native_build_server`,
  `native_build_event`, `heuristic_parse`, `imported_snapshot`,
  `provider_overlay`, `unknown`.
- **Discovery confidence**: `high`, `medium`, `low`, `unknown`.
- **Schema freshness**: `fresh`, `stale`, `unversioned`, `unavailable`.
- **Required labels** (all six mandatory per row): `identity`,
  `target_context`, `truth_class`, `freshness_or_confidence`,
  `adapter_source`, `keyboard_route`.
- **Downgrade triggers**: `schema_stale`, `adapter_unavailable`,
  `connector_loss`, `policy_block`, `drift_from_source`,
  `low_confidence_discovery`, `structured_channel_lost`,
  `target_context_unresolved`.

## Guardrails (validated invariants)

`validate()` rejects any packet that violates these; each guardrail flag
must be `true`:

1. Authored, rendered, planned, live, cached, and provider-overlay truth
   never blur.
2. Target context stays visible on every read- or mutate-capable surface.
3. Schema freshness and adapter source kind are always explicit.
4. Lower-confidence discovery / results never overwrite higher-confidence
   truth silently.
5. Drift, connector loss, and policy blocks narrow actions before
   execution.
6. Exported evidence preserves the same target IDs, adapter kinds, and
   freshness / confidence states shown in-product.
7. Components stay bound to the shared vocabulary (no second naming
   system).
8. This lane hardens shared UI contracts only — no new build adapters,
   live-resource connectors, or infra mutation engines.

The matrix must contain at least one component per family and at least
one degraded case, and the checked-in support export must match the
builder byte-for-byte.

## Consumer projection

The same component records are ingested by product surfaces, docs / help,
diagnostics, support export, and release control — so later M5 rows
reference one canonical component family instead of restating infra /
build confidence truth. Export safety is enforced: raw boundary material
(secrets, credentials, bearer tokens, URLs) is rejected from the export.
