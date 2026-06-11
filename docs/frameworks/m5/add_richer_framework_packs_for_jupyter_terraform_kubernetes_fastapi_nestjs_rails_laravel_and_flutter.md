# Richer framework packs: Jupyter adjacency, Terraform/Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter

This contract describes the export-safe packet that carries the **richer
framework-pack lane catalog** for the gallery, pack header, run, diff-review,
diagnostics, and support surfaces. It deepens the framework-pack lineup across
the notebook-adjacency, infrastructure, web-API, and mobile lanes — Jupyter
adjacency, Terraform, Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter —
while keeping each pack's provenance, pinned pack and generator versions,
freshness, capability and support class, authored/generated/runtime-only origin
truth, archetype health state, and downgrade banner inspectable. The packet is
the canonical truth those surfaces ingest instead of re-describing pack state by
hand or presenting heuristic, bridged, or runtime-observed structure as exact
first-party truth.

- Boundary schema:
  `schemas/templates/add-richer-framework-packs-for-jupyter-terraform-kubernetes-fastapi-nestjs-rails-laravel-and-flutter.schema.json`
- Implementation:
  `crates/aureline-templates/src/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/`
- Checked support export:
  `artifacts/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/support_export.json`
- Fixtures:
  `fixtures/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/`

This packet **extends** the framework-pack-header presentation contract and
**references** the upstream template-manifest and template-registry records
frozen in `docs/templates/template_registry_and_scaffold_contract.md` — the
`template_manifest_alpha` and `template_registry_entry` contracts — by opaque ref
(`pack_id`, `framework_id`) rather than embedding them, and reuses the prior
support-class and downgrade vocabulary instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw pack bodies, raw manifests, repository URLs,
hostnames, secrets, and user-authored content never cross this boundary. Rows
carry opaque refs, closed-vocabulary class tokens, short reviewable summaries,
and export-safe chip labels. `validate` rejects any export that leaks obviously
forbidden material.

## Row truth

Each `lane_row` binds one framework pack to:

- **Lane** — `lane_label` and `lane_domain_class` (`notebook_adjacency`,
  `infrastructure_provisioning`, `web_api_service`, `mobile_app`, or
  `domain_unknown`), so the gallery groups the deeper lineup by lane.
- **Header** — `pack_label`, `framework_label`, `pack_version_semver`,
  `generator_version_semver`, `header_summary`, and `provenance_class`
  (`first_party`, `partner_certified`, `community`, `mirror`,
  `bridged_from_other_framework`, or `provenance_unknown`). The header always
  shows provenance and the pinned pack **and generator** versions.
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
  downgrade trigger, and show a banner, so a richer long-tail of lanes never
  presents bridge or heuristic behavior as exact first-party truth.
- **Origin truth** — `origin_truth_class` (`authored_source`,
  `generated_managed`, `runtime_observed`, `bridged_adjacent`, or
  `origin_unknown`) makes the authored/generated/runtime-only truth explicit. An
  `origin_unknown` pack must show a banner and can never be offered.
- **Archetype health** — `archetype_health_class` (`certified_healthy`,
  `healthy_uncertified`, `degraded`, `health_unknown`, or `blocked`) and
  `health_summary`. A `degraded`, `health_unknown`, or `blocked` health state
  must show a banner; `health_unknown` and `blocked` can never be offered.
- **Downgrade banner** — `downgrade_banner_class` (`no_banner`,
  `freshness_banner`, `capability_banner`, `support_class_banner`,
  `origin_truth_banner`, `health_banner`, `policy_block_banner`, or
  `provenance_unknown_banner`) makes the narrowing cue explicit.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_offer`. A blocked pack (stale or unknown freshness, degraded or
  unknown capability, unknown provenance, unknown origin truth, unknown or
  blocked health, or a hard-block banner) can never be admitted for offer.

## Lane lineup

The canonical catalog covers all eight named lanes across the spectrum:

- **Jupyter adjacency** — partner-certified, partial-capability,
  `bridge_behavior` support whose notebook execution is `bridged_adjacent`;
  offered behind a support-class banner because the bridge is disclosed.
- **Terraform** and **FastAPI** — first-party, full-capability,
  `generated_managed`, certified-healthy; offered cleanly.
- **Kubernetes** — first-party, full-capability, certified-healthy, with an
  `update_available` chip; offered cleanly.
- **Nest** — community, partial-capability with a partial worker generator;
  offered behind a capability banner.
- **Rails** — community, full-capability, certified-healthy; offered cleanly.
- **Laravel** — community heuristic mapping with degraded archetype health; held
  from offer behind a support-class banner.
- **Flutter** — served from a mirror whose provenance could not be verified;
  blocked behind a provenance-unknown banner rather than hidden.

## Downgrade automation

`apply_downgrade_automation` narrows rows from observed runtime signals so a
stale or underqualified lane pack narrows before it is offered, instead of being
hidden or presented as exact first-party truth:

- **Unknown provenance** marks the header, freshness, capability, origin truth,
  and health unknown, raises the provenance-unknown banner, and withdraws the
  offer.
- **A yanked pack or generator version** narrows freshness to `stale`, raises a
  freshness banner, and withdraws the offer.
- **Stale freshness** narrows freshness to `stale`, raises a freshness banner,
  and withdraws the offer.
- **An unverified capability** narrows the capability to `capability_degraded`,
  raises a capability banner, and withdraws the offer.
- **A failed archetype health check** narrows health to `degraded`, raises a
  health banner, and withdraws the offer.
- **An unverified origin truth** narrows the origin to `origin_unknown`, raises
  an origin-truth banner, and withdraws the offer.
- **Stale proof or a narrowed upstream** withdraws the offer.

A raised banner is never lowered to a softer cue, and a narrowed row stays a
valid, export-safe packet, so the gallery and support surfaces show a current,
labeled state rather than an optimistic placeholder.

## Consumers

`current_richer_framework_pack_export()` reads and validates the checked support
export. It is the first real consumer: a gallery, pack header, run, diagnostics,
or support-export surface ingests the canonical packet through it. The two
checked fixtures (`health_degraded_withheld.json`,
`generator_version_yanked_blocked.json`) are valid, narrowed packets that
exercise the downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_richer_framework_packs -- canonical
cargo run -p aureline-templates --example dump_richer_framework_packs -- markdown
cargo run -p aureline-templates --example dump_richer_framework_packs -- health_degraded
cargo run -p aureline-templates --example dump_richer_framework_packs -- generator_version_yanked
```
