# Learning Aureline's depth features

Aureline's depth features — notebooks, request and database workspaces,
profiler and trace flows, docs/browser depth, preview surfaces, template and
scaffold planners, the companion/incident surface, and sync/offboarding — each
ship in-product learning assets so you can learn and revisit a flow without
leaving Aureline and without mandatory setup videos.

This page is the human-readable companion to the canonical learning manifest
checked in at
[`fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json`](../../../fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json).
Downstream Help/About, Start Center, and support-export surfaces ingest that
manifest rather than cloning the status text below.

## What each family gives you

For every claimed feature family, the learning bundle includes:

- **A glossary pack** — the terms a flow uses, each citing an authoritative
  command or docs anchor.
- **A guided tour** — a short, command-backed walkthrough. Every step runs the
  same command, opens the same preview, and uses the same approval prompt as
  ordinary work. Tours never take a tutorial-only shortcut.
- **Contextual help cards** — in-place explanations that link back to the
  authoritative command and docs.
- **An exercise rail** — an optional hands-on path whose Apply steps are
  reversible and go through the standard preview/approval fence.
- **A progress snapshot** — your learning progress, dismissals, and resume point.

## Offline and mirrored profiles

Learning assets stay available on local-only, air-gapped, and mirrored profiles.
Instead of silently dead-linking when content is cached or the origin is
unreachable, each bundle shows an explicit freshness label:

| Label | Meaning |
|---|---|
| `live_authoritative` | Served from the installed, current pack revision. |
| `mirror_synced_disclosed` | Served from a mirror, disclosed as such. |
| `cached_disclosed` | Served from a cached revision, freshness disclosed. |
| `local_only_disclosed` | Available locally only; not yet mirror-synced. |
| `stale_disclosed` | Known stale; disclosed rather than hidden. |

## Your progress is yours

Learning progress is stored locally by default. It is not visible to the
repository, is never read at telemetry grade, survives a restart, and is safe to
include in a support bundle. You can pause, skip, reset, and resume at any time.

## Current status

The notebook, request, database, profiler/trace, docs/browser, template/scaffold,
and sync/offboarding families qualify Stable. The preview family is in Beta while
its learning pack finishes mirror sync, and the companion family is in Beta while
its tour anchors move from a cached revision to the live one. In both cases the
flow is fully usable; the Beta label reflects the missing parity proof, not a
broken experience.

## See also

- Release evidence packet: [`artifacts/ux/m5/learning-packets/implement-m5-feature-family-guided-tours-and-learning-rails.md`](../../../artifacts/ux/m5/learning-packets/implement-m5-feature-family-guided-tours-and-learning-rails.md)
- Schema: [`schemas/learning/m5-feature-family-learning-rails.schema.json`](../../../schemas/learning/m5-feature-family-learning-rails.schema.json)
- Learning-mode qualification: [`docs/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md`](../../m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md)
