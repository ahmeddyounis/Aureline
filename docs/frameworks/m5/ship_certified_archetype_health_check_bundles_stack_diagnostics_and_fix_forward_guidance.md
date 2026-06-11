# Certified-archetype health-check bundles, stack diagnostics, and fix-forward guidance

This contract describes the export-safe packet that carries the **certified-archetype
health-check bundle** truth for the archetype gallery, health-check panel,
stack-diagnostics, fix-forward guidance, run, diagnostics, and support surfaces: the
archetype certification class each bundle carries, the pinned health-check bundle
version, the overall health state it reports, the worst stack-diagnostic severity it
observed, whether and how fix-forward guidance is available, how scan-fresh it is, its
support class, and its downgrade banner. The packet is the canonical truth those
surfaces ingest instead of presenting an uncertified, heuristic, or bridged check as
exact first-party certified truth.

- Boundary schema:
  `schemas/templates/ship-certified-archetype-health-check-bundles-stack-diagnostics-and-fix-forward-guidance.schema.json`
- Implementation:
  `crates/aureline-templates/src/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/`
- Checked support export:
  `artifacts/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/support_export.json`
- Fixtures:
  `fixtures/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/`

This packet **references** the upstream template-manifest, framework-pack, and
framework-generator records — the `template_manifest_alpha` contract, the
framework-pack header packet, and the framework generator-run packet frozen in
`docs/templates/template_registry_and_scaffold_contract.md` and the framework-pack
lane — by opaque ref (`app_id`, `framework_pack_ref`, `archetype_id`, `bundle_id`,
`diagnostic_refs`, `fix_forward_refs`) rather than embedding them, and reuses the
prior support-class and downgrade vocabulary instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw source bodies, raw diagnostic logs, generated file
contents, repository URLs, hostnames, secrets, and user-authored content never cross
this boundary. Rows carry opaque refs, closed-vocabulary class tokens, short
reviewable summaries, structural locators (`archetype_locator`, `diagnostic_refs`,
`fix_forward_refs`), and export-safe chip labels (`diagnostic_stat_label`,
`freshness_chip_label`). `validate` rejects any export that leaks obviously forbidden
material.

## Row truth

Each `archetype_health_row` binds one health-check bundle run to:

- **Kind, certification, and provenance** — `archetype_kind` (`service_archetype`,
  `web_app_archetype`, `full_stack_archetype`, `cli_archetype`, or
  `library_archetype`), `certification_class` (`certified_archetype`,
  `provisional_archetype`, `community_archetype`, `uncertified_archetype`, or
  `certification_unknown`), `archetype_version`, and `bundle_version` (both pinned and
  always disclosed). A non-certified archetype must show a banner and carry the
  `uncertified_archetype_disclosed` trigger — it is never presented as certified.
- **Health** — `health_check_class` (`healthy`, `healthy_with_advisories`, `degraded`,
  `failing`, or `health_unknown`) and `health_summary`. A `health_unknown` bundle must
  raise the `health_unknown_banner` and is blocked from a confident verdict — a verdict
  is never invented.
- **Stack diagnostics** — `stack_diagnostic_class` (`no_diagnostics`, `advisory`,
  `warning`, `error`, `blocking`, or `diagnostics_unavailable`), `diagnostic_summary`,
  `diagnostic_stat_label`, and `diagnostic_refs`. A `warning`, `error`, or `blocking`
  severity carries at least one diagnostic ref. A `diagnostics_unavailable` state must
  show a downgrade banner and is blocked from a confident verdict.
- **Fix-forward guidance** — `fix_forward_class` (`no_fix_needed`, `fix_automatic`,
  `fix_guided`, `fix_advisory_only`, or `fix_unavailable`), `fix_forward_summary`, and
  `fix_forward_refs`. A state carrying guidance carries at least one fix-forward ref. A
  `fix_unavailable` state must show a banner. A missing fix-forward path is labeled,
  never silently hidden, and does not by itself block the health verdict — the bundle
  stays offered.
- **Freshness chip** — `freshness_class` (`fresh`, `rescan_available`, `aging`,
  `stale`, or `freshness_unknown`), `freshness_chip_label`, and `last_checked`. A
  `stale` or `freshness_unknown` chip must show a downgrade banner.
- **Support honesty** — `support_class` keeps bridge/heuristic behavior labeled. A
  `bridge_behavior` or `heuristic_mapping` bundle must disclose a known issue, carry
  the matching `bridge_behavior_disclosed` / `heuristic_mapping_disclosed` downgrade
  trigger, and show a banner, so an inferred or bridged check is never presented as
  exact first-party truth.
- **Downgrade banner** — `downgrade_banner_class` (`no_banner`, `freshness_banner`,
  `health_unknown_banner`, `diagnostics_unavailable_banner`, `fix_unavailable_banner`,
  `certification_banner`, `support_class_banner`, or `policy_block_banner`) makes the
  narrowing cue explicit.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_display`. A blocked bundle (stale or unknown freshness, unknown health,
  unavailable diagnostics, or a hard-block banner) can never be admitted for a confident
  verdict.

## Downgrade automation

`apply_downgrade_automation` narrows bundles from observed runtime signals so a stale or
underqualified archetype health bundle narrows before it is published, instead of being
hidden or presented as exact certified truth:

- **An undeterminable health verdict** marks the health unknown, the diagnostics
  unavailable, and the fix-forward guidance unavailable, raises the health-unknown
  banner, and withdraws display.
- **Unavailable diagnostics** mark the diagnostics unavailable, raise the
  diagnostics-unavailable banner, and withdraw display.
- **An unverified certification** narrows the certification to unknown, raises the
  certification banner, and withdraws display.
- **A lost fix-forward path** narrows the fix state to unavailable, raises a fix banner,
  and keeps the bundle offered — a missing fix is honest, not a block on the verdict.
- **A stale scan record** narrows freshness to `stale`, raises a freshness banner, and
  withdraws display.
- **Stale proof or a narrowed upstream** withdraws display.

A raised banner is never lowered to a softer cue, and a narrowed bundle stays a valid,
export-safe packet, so the health and support surfaces show a current, labeled state
rather than an optimistic placeholder.

## Consumers

`current_archetype_health_export()` reads and validates the checked support export. It
is the first real consumer: an archetype gallery, health-check panel, stack-diagnostics,
fix-forward guidance, run, diagnostics, or support-export surface ingests the canonical
packet through it. The two checked fixtures (`health_unknown_blocked.json`,
`fix_forward_unavailable_labeled.json`) are valid, narrowed packets that exercise the
downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical builder:

```text
cargo run -p aureline-templates --example dump_archetype_health_bundles -- canonical
cargo run -p aureline-templates --example dump_archetype_health_bundles -- markdown
cargo run -p aureline-templates --example dump_archetype_health_bundles -- health_unknown
cargo run -p aureline-templates --example dump_archetype_health_bundles -- fix_forward_unavailable
```
