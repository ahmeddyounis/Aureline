# M5 live-resource navigation primitive contract

This packet narrows two families of the frozen
[M5 manifest / build component matrix](./m5_manifest_build_component_matrix.md)
— the resource-link row and the resource-explorer row — plus the rendered /
live compare card and the drift / unavailable banner they imply into one
working primitive with a real **resolver**. A single live-resource context
resolves onto four surfaces that share one resource identity and one disclosed
truth class, so source-to-live navigation and live-resource browsing stay honest
*before* users act under drift, partiality, and permission limits.

The primitive is minted and validated in
[`crates/aureline-infra`](../../crates/aureline-infra/src/implement_the_m5_resource_link_compare_card_explorer_and_drift_banner_primitive/mod.rs)
(`record_kind = m5_live_resource_navigation_primitive`, `schema_version = 1`).
The Rust resolver `resolve_live_resource_navigation()` and the builder
`seeded_m5_live_resource_packet()` are the source of truth; the checked-in
artifacts below are byte-for-byte emissions of the builder
(`current_stable_m5_live_resource_export()` re-reads the support export via
`include_str!`).

If this doc, the machine-readable schema, and the checked-in artifacts disagree,
the schema plus the Rust builder win and all companion artifacts update in the
same change.

## The resolver

`resolve_live_resource_navigation(&M5LiveResourceInput)` projects one
live-resource context onto:

- **`M5ResolvedResourceLinkRow`** — the source-to-live link class, its two
  distinct truth-class sides, discovery confidence, permission posture, the
  never-silent-overwrite guarantee, and whether each side is navigable.
- **`M5ResolvedCompareCard`** — the disclosed truth class, the rendered / live
  compare verdict, both compared sides, exactly what diverged when it drifted,
  whether the comparison reflects current live truth, and that inspection stays
  safe.
- **`M5ResolvedResourceExplorerRow`** — the typed resource identity (kind,
  stable id, namespace / project), truth class, freshness, confidence, health,
  permission posture, permission / connection note, the logs / events /
  open-detail actions, and whether the row may present as fully current.
- **`M5ResolvedDriftBanner`** — whether a banner is present, whether the resource
  drifted or went unavailable, what diverged, whether the data is stale, that
  inspection stays safe, and — when narrowed — a precise, reconstructable
  reason.

All four carry the same `resource_id`; the compare card, explorer row, and drift
banner disclose the same truth class.

## Acceptance criteria

- **AC1 — source config and live / cached resources never blur.** The link row
  keeps its two truth-class sides distinct (the resolver refuses a link that
  collapses them), and the compare card, explorer row, and drift banner all
  disclose one shared truth class, so a user can move between source and live
  truth without the two collapsing into one.
- **AC2 — drift and unavailability are visible before users act.** The drift
  banner is present whenever a resource has drifted, gone unavailable
  (connector lost, offline, or comparison unavailable), gone stale, lost
  permission, or dropped confidence, and it names exactly what diverged and what
  remains safe to inspect before any logs / events / open-detail action. Drift
  must name a divergence detail; degraded blocks must carry a precise, non-generic
  label.
- **AC3 — partial or permission-limited data is never shown as fully current.** A
  resource reads as current only when it is live-fresh, reachable (full / read
  access), undrifted, and high / medium confidence; any cached, imported,
  drifted, permission-limited, or low-confidence resource is disclosed as such.
  Limited access must always carry a precise permission / connection note, and
  live-fresh data may not claim a non-live truth class.

## Reused and minted vocabulary

The primitive **reuses** the frozen matrix vocabulary rather than restating it:
`TruthMode`, `M5ResourceLinkClass`, `M5ResourceFreshness`,
`M5DiscoveryConfidence`, `M5ManifestBuildDowngradeTrigger`, and `DegradedState`.

It **mints** only the navigation-specific vocabulary:
`M5LiveResourceSurfaceFamily` (6 parity surfaces), `M5ResourceKind`,
`M5ResourceHealth`, `M5PermissionPosture`, `M5CompareVerdict`,
`M5ResourceActionKind`, and `M5LiveResourceExportField`.

## Companion artifacts

- [`/schemas/ui/m5-live-resource-navigation-primitive.schema.json`](../../schemas/ui/m5-live-resource-navigation-primitive.schema.json)
  — boundary schema for the primitive packet, its surface rows, worked
  navigation cases (input + resolved), vocabulary set, governance review,
  consumer projection, and release posture.
- [`/artifacts/release/m5-live-resource-navigation-primitive-proof/support_export.json`](../../artifacts/release/m5-live-resource-navigation-primitive-proof/support_export.json)
  — the `include_str!` canonical support export (release / support proof).
- [`/artifacts/release/m5-live-resource-navigation-primitive-proof/matrix.csv`](../../artifacts/release/m5-live-resource-navigation-primitive-proof/matrix.csv)
  — machine-readable per-surface CSV.
- [`/artifacts/release/m5-live-resource-navigation-primitive-proof/report.md`](../../artifacts/release/m5-live-resource-navigation-primitive-proof/report.md)
  — human-readable Markdown report.
- [`/fixtures/ui/m5-live-resource-navigation-primitive/`](../../fixtures/ui/m5-live-resource-navigation-primitive/)
  — protected fixtures (byte-identical copies of the support export and CSV).

The fixture-emitting bin is
`crates/aureline-infra/src/bin/emit_live_resource_navigation_primitive_fixture.rs`
(`support` | `csv` | `summary`); its `support` output is the byte-for-byte
`include_str!` canonical.

## Privacy boundary

Raw resource bodies, live payloads, credentials, connector tokens, and endpoint
data never cross this boundary. The packet carries only opaque refs, typed class
tokens, booleans, and redacted labels, so support and diagnostics exports
reconstruct exactly what a surface would have shown without leaking source or
live payloads.
