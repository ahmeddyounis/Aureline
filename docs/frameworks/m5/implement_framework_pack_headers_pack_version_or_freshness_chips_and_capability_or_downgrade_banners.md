# Framework-pack headers, pack version/freshness chips, and capability/downgrade banners

This contract describes the export-safe packet that carries the **framework-pack
presentation** truth for the gallery, pack header, run, diff-review,
diagnostics, and support surfaces: the header provenance, the pinned pack
version and freshness chip, the capability banner, the support class, and the
downgrade banner for each framework pack. The packet is the canonical truth
those surfaces ingest instead of re-describing pack state by hand or presenting
heuristic and bridge behavior as exact first-party truth.

- Boundary schema:
  `schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json`
- Implementation:
  `crates/aureline-templates/src/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/`
- Checked support export:
  `artifacts/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/support_export.json`
- Fixtures:
  `fixtures/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/`

This packet **references** the upstream template-manifest and template-registry
records frozen in `docs/templates/template_registry_and_scaffold_contract.md` —
the `template_manifest_alpha` and `template_registry_entry` contracts — by opaque
ref (`pack_id`, `framework_id`) rather than embedding them, and reuses the prior
support-class vocabulary instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw pack bodies, raw manifests, repository URLs,
hostnames, secrets, and user-authored content never cross this boundary. Rows
carry opaque refs, closed-vocabulary class tokens, short reviewable summaries,
and export-safe chip labels. `validate` rejects any export that leaks obviously
forbidden material.

## Row truth

Each `pack_row` binds one framework pack to:

- **Header** — `pack_label`, `framework_label`, `pack_version_semver`,
  `header_summary`, and `provenance_class` (`first_party`, `partner_certified`,
  `community`, `mirror`, `bridged_from_other_framework`, or
  `provenance_unknown`). The header always shows provenance and the pinned pack
  version.
- **Freshness chip** — `freshness_class` (`fresh`, `update_available`, `aging`,
  `stale`, or `freshness_unknown`), `freshness_chip_label`, and `last_verified`.
  A `stale` or `freshness_unknown` chip must show a downgrade banner.
- **Capability banner** — `capability_class` (`full_capability`,
  `partial_capability`, `bridged_capability`, `heuristic_capability`,
  `capability_degraded`, or `capability_unknown`) and `capability_summary`. Any
  class other than `full_capability` must show a banner.
- **Support honesty** — `support_class` keeps bridge/heuristic behavior labeled.
  A `bridge_behavior` or `heuristic_mapping` pack must disclose a known issue,
  carry the matching `bridge_behavior_disclosed` / `heuristic_mapping_disclosed`
  downgrade trigger, and show a banner, so framework-pack bridge behavior is
  never presented as exact first-party truth.
- **Downgrade banner** — `downgrade_banner_class` (`no_banner`,
  `freshness_banner`, `capability_banner`, `support_class_banner`,
  `policy_block_banner`, or `provenance_unknown_banner`) makes the narrowing cue
  explicit.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_offer`. A blocked pack (stale or unknown freshness, degraded or
  unknown capability, unknown provenance, or a hard-block banner) can never be
  admitted for offer.

## Downgrade automation

`apply_downgrade_automation` narrows rows from observed runtime signals so a
stale or underqualified pack narrows before it is offered, instead of being
hidden or presented as exact first-party truth:

- **Unknown provenance** marks the header, freshness, and capability unknown,
  raises the provenance-unknown banner, and withdraws the offer.
- **A yanked pack version** narrows freshness to `stale`, raises a freshness
  banner, and withdraws the offer.
- **Stale freshness** narrows freshness to `stale`, raises a freshness banner,
  and withdraws the offer.
- **An unverified capability** narrows the capability to `capability_degraded`,
  raises a capability banner, and withdraws the offer.
- **Stale proof or a narrowed upstream** withdraws the offer.

A raised banner is never lowered to a softer cue, and a narrowed row stays a
valid, export-safe packet, so the gallery and support surfaces show a current,
labeled state rather than an optimistic placeholder.

## Consumers

`current_framework_pack_export()` reads and validates the checked support
export. It is the first real consumer: a gallery, pack header, run,
diagnostics, or support-export surface ingests the canonical packet through it.
The two checked fixtures (`provenance_unknown_blocked.json`,
`capability_degraded_withheld.json`) are valid, narrowed packets that exercise
the downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_framework_pack_headers -- canonical
cargo run -p aureline-templates --example dump_framework_pack_headers -- markdown
cargo run -p aureline-templates --example dump_framework_pack_headers -- provenance_unknown
cargo run -p aureline-templates --example dump_framework_pack_headers -- capability_degraded
```
