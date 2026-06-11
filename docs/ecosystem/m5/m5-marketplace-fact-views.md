# M5 marketplace fact-views

This document describes the canonical packet that freezes the **M5 marketplace
fact-views** — source-aware result rows, per-listing detail fact grids, and
side-by-side compare views — so framework packs, docs packs, local-model packs,
recipe packs, templates, and bridge-backed or side-loaded packages are never
presented as undifferentiated catalog rows. It is the user-facing companion to the
governed artifact at `artifacts/ecosystem/m5/m5-marketplace-fact-views.json` and the
typed model in the `aureline-ecosystem` crate (`m5_marketplace_fact_views`).

Where the
[`M5 ecosystem install-governance matrix`](m5-ecosystem-install-governance-matrix.md)
freezes one governance row per marketed artifact family, this packet freezes how
those families are **presented** in the marketplace and mirror/private-registry
discovery surfaces. It reuses the governance vocabulary — package kind, support
class, runtime origin, lifecycle state, and evidence freshness — instead of inventing
separate view logic per pack family, and adds the presentation-only vocabulary the
storefront needs.

## What this packet covers

The packet carries three view shapes that all project the same fact set:

- a **result row** is the at-a-glance catalog row;
- a **detail fact grid** is the per-listing detail page; and
- a **compare view** sets two or more listings side by side.

Every row, grid, and compare entry answers, for its listing:

- **What is it?** A `package_kind` — `first_party_framework_pack`, `docs_pack`,
  `local_model_pack`, `signed_recipe_pack`, `template_artifact`,
  `bridge_backed_package`, `side_loaded_package`, or `mirrored_registry_variant`.
- **Who stands behind it?** A `source_class` of `first_party`, `verified_partner`,
  `community`, or `unverified`.
- **Where is it in its lifecycle?** A `lifecycle_state` of `available`, `installed`,
  `update_available`, `disabled`, `quarantined`, `rolled_back`, or `retired`.
- **How well is it supported?** A `support_class` of `fully_supported`,
  `best_effort_supported`, `community_supported`, or `unsupported`.
- **How fresh is the evidence?** An `evidence_freshness` of `current`, `stale`,
  `expired`, or `unknown`.
- **How does it run?** A `runtime_origin` (signing/provenance) and a
  `bridge_native_state` of `native`, `bridge_backed`, or `local_model_hosted`.
- **Where is it served from?** A `mirror_posture` of `direct_first_party`,
  `enterprise_mirrored`, `private_registry`, or `manually_imported`.
- **Which flow surfaced it?** A `discovery_channel` of `public_registry`,
  `enterprise_mirror`, `private_registry`, or `manual_import`.
- **How much disclosure does it warrant?** A `disclosure_level` of `standard`,
  `caution`, or `heightened`, plus the `disclosure_reasons` that explain it.

A detail grid additionally pins the backing `provenance_ref`,
`permission_manifest_ref`, `compatibility_ref`, `activation_budget_ref`,
`rollback_ref`, and `support_export_ref`, and every result row carries a
`governance_family_ref` that resolves to its row in the install-governance matrix.

## Disclosure is recomputed, not stored by hand

The `disclosure_level` a row publishes is **not** hand-entered. It is recomputed from
the row's facts, and the stored `disclosure_level` and `disclosure_reasons` must equal
that recomputation or validation fails. Each reason forces a minimum level and the row
takes the widest:

- **`reduced_provenance`** (forces `heightened`) — the source class is `community` or
  `unverified`, the runtime origin is `unsigned_side_loaded`, or the listing was
  manually imported.
- **`mirrored_or_private_distribution`** (forces `caution`) — the posture is
  `enterprise_mirrored` or `private_registry`.
- **`evidence_not_current`** (forces `caution`) — freshness is `stale`, `expired`, or
  `unknown`.
- **`support_narrowed`** (forces `caution`) — the support class is below
  `fully_supported`.
- **`non_native_runtime`** (forces `caution`) — the bridge/native state is
  `bridge_backed` or `local_model_hosted`.

## Reduced provenance widens warnings, it never collapses fields

This is the lane guardrail. A mirrored, private-registry, or manually-imported
listing must keep **every** fact and backing ref its first-party sibling shows; the
only thing that changes is that its `disclosure_level` widens. The packet enforces
this several ways:

- every detail grid must reproduce its row's facts exactly (`GridDriftsFromRow`
  otherwise) and carry its own non-empty provenance, permission, compatibility,
  activation, rollback, and support-export refs;
- every compare-view entry must reproduce its row's facts exactly
  (`CompareEntryDriftsFromRow` otherwise); and
- the recomputed disclosure can only ever raise a row's level above `standard`, so a
  reduced-provenance listing can never present a narrower warning than its facts
  warrant.

## The same information architecture holds across every channel

Because the result row, detail grid, and compare entry share one fact set, the same
information architecture renders identically across the public registry, an enterprise
mirror, a private registry, and manual import — even when some fields (mirror posture,
private-registry channel) are derived from local policy or mirror metadata rather than
a public storefront.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each listing's package
kind, source class, lifecycle, support, freshness, runtime origin, bridge/native
state, mirror posture, discovery channel, disclosure level, and reason tokens, plus
`widened_disclosure_count`, `mirrored_or_private_count`, and `manual_import_count`.
Support bundles and docs/help should ingest this projection directly rather than
writing custom explanations for each artifact class, so the product, support exports,
and release evidence all cite the same rows.

## Validation

`M5MarketplaceFactViews::validate()` reports every violation, including an unsupported
schema version or record kind, non-canonical closed vocabularies, empty required
fields, duplicate row/grid/compare ids, duplicate disclosure reasons, a disclosure
level or reason set that disagrees with the recomputation, a grid or compare entry
that drifts from or dangles off its row, a compare view with fewer than two rows or
mismatched entries, and a summary block that disagrees with the views.
