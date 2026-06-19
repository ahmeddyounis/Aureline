# Glossary packs and guided tours that travel with you

Aureline's depth features ship their learning content as **versioned packages**:
a glossary pack and a guided tour for each family. Because the content is
versioned and points at stable product objects rather than screen positions,
a package can be localized, mirrored to an offline profile, exported, and
reopened later without losing its meaning or its citations.

This page is the human-readable companion to the canonical package manifest
checked in at
[`fixtures/help/m5/tour-and-glossary-packages/m5_tour_and_glossary_packages.json`](../../../fixtures/help/m5/tour-and-glossary-packages/m5_tour_and_glossary_packages.json).
Help/About, docs/migration, and support-export surfaces ingest that manifest
rather than cloning the status text below.

## What a package gives you

- **A glossary pack** — the terms a flow uses. Each term points at the stable
  command, file, symbol, docs node, graph node, or surface it refers to, and
  cites an authoritative command or docs anchor.
- **A guided tour** — a short, command-backed walkthrough. Each step points at a
  stable object — never a pixel coordinate — so the step still works after a
  layout, theme, or window change. Steps run the same command and use the same
  approval prompt as ordinary work.

Every package carries a version and a content revision, so a copy you exported
last month can be matched back to exactly the content it shipped.

## Stable targets, not coordinates

A tour step references stable objects:

| Target kind | Example |
|---|---|
| `command_id` | the command the step runs |
| `file_object_id` / `symbol_object_id` | a file or symbol the step points at |
| `docs_node_id` / `graph_node_id` | a docs or graph node |
| `surface_object_id` | a panel, view, or region |

When a step widens the working scope — for example, applying a scaffold writes
every planned file in a folder, not just the one you previewed — the tour
**names the widening**: it states the scope before, the scope after, and why.
You are never silently handed a wider blast radius than the previous step
implied.

## Localized without losing meaning

Each package can carry locale overlays. An overlay localizes the display labels
for the same entry and step ids; it never touches the stable targets or the
citations. So the French and Japanese versions of a tour point at the same
commands and cite the same docs as the original — translation changes the words,
not the targets.

## Offline and mirrored copies stay honest

A package records its freshness state, and a cached or mirrored copy is always
visibly distinct from current live help:

| Freshness | Meaning |
|---|---|
| `live_authoritative` | The installed, current authoritative revision. |
| `mirror_synced_disclosed` | Served from a mirror, disclosed as such. |
| `cached_disclosed` | A cached revision, freshness disclosed. |
| `local_only_disclosed` | Available locally only; not yet mirror-synced. |
| `stale_disclosed` | Known stale; disclosed rather than hidden. |

A non-live package is never presented as current live knowledge — its freshness
is disclosed, and it is labeled Beta until it is live or mirror-synced.

## Current status

The notebook, request, database, profiler/trace, docs/browser, template/scaffold,
and sync/offboarding families ship Stable, live (or mirror-synced) packages. The
preview family's packages are local-only while mirror sync finishes, and the
companion family's packages are served from a cached revision; both are in Beta
and clearly disclosed. In every case the content is fully usable — the Beta label
reflects the missing freshness/parity proof, not a broken experience.

## See also

- Release evidence packet: [`artifacts/ux/m5/tour-package-proof/implement-versioned-glossary-pack-and-tour-package-manifests.md`](../../../artifacts/ux/m5/tour-package-proof/implement-versioned-glossary-pack-and-tour-package-manifests.md)
- Schema: [`schemas/help/m5-tour-and-glossary-packages.schema.json`](../../../schemas/help/m5-tour-and-glossary-packages.schema.json)
- Single-package tour contract: [`schemas/help/tour_package.schema.json`](../../../schemas/help/tour_package.schema.json)
- Feature-family learning rails: [`docs/help/m5/m5-feature-family-learning-rails.md`](./m5-feature-family-learning-rails.md)
