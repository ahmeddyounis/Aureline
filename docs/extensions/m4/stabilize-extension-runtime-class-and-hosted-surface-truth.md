# Stable Extension Runtime-Class And Hosted-Surface Truth

This document freezes the stable extension runtime-class and hosted-surface truth packet consumed by marketplace rows, install review, active contribution inspectors, extension-authored hosted surfaces, diagnostics, local development / sideload / publish-preview flows, and support export.

The canonical implementation is:

- `crates/aureline-extensions/src/stabilize_extension_runtime_class_and_hosted_surface_truth/`
- `schemas/extensions/runtime-class.schema.json`
- `fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/`

## Runtime-Class Vocabulary

Every contributed package or surface uses one closed runtime class:

| Token | User-facing label | Meaning |
|---|---|---|
| `passive_package` | Passive package | Themes, snippets, icon packs, docs packs, or other non-executing assets. |
| `wasm_capability_sandbox` | Wasm capability sandbox | Capability-bounded Wasm extensions with explicit grants and budgets. |
| `declarative_host_rendered_view` | Declarative/host-rendered view | Host-owned panels, settings, status items, previews, or providers. |
| `external_host` | External host | Supervised helper processes, language servers, debug adapters, or service helpers. |
| `compatibility_bridge` | Compatibility bridge | Bridged or shimmed contributions with possible feature loss. |
| `remote_side_component` | Remote-side component | Extension components executing on SSH, container, managed, or other remote targets. |

The packet derives the effective stability tier. A claimed Stable row narrows when runtime truth is catalog-only, unverified, absent from required consumers, hidden behind generic wording, missing active inspectors, missing inspector actions, missing downgrade banners, hiding hosted-surface boundary chrome, missing safe external handoff, drifting between public and local authoring vocabulary, or missing support export.

## Required Consumers

A stable claim must include at least:

- `marketplace_result_row`
- `install_review`
- `active_contribution_inspector`
- `diagnostics`
- `support_export`

Additional surfaces such as detail pages, active UI surfaces, hosted-surface chrome, local-dev strips, sideload review, publish preview, and help/docs surfaces should consume the same packet when present.

## Active Inspectors

Active contribution inspectors carry package ID/version, publisher or signature state, runtime class, execution locus, trust tier, permissions used in the current session, current host health, host identity or last-known-good host ref, recent events, and actions. The required action set is `pause`, `restart`, and `quarantine`; additional actions such as logs, disable, permission review, revert, or migrate can be present.

## Downgraded Hosts

When a contribution falls back from native or host-rendered behavior to a compatibility bridge, external host, remote-side component, or browser-like content, the packet must include a downgraded-host banner. The banner names the previous runtime class, current runtime class, reason, feature loss, and recovery choices. A disclosed downgrade narrows below Stable while preserving the supportable truth needed for diagnostics and recovery.

## Hosted Surfaces

Extension-authored webviews, dashboards, account panes, documentation panes, and browser-runtime bridges must include owner/origin chrome, boundary and egress summary, storage/cookie posture, accessibility note, theming note, and open-in-browser fallback posture. If the surface is safer outside the product boundary, `available` or `recommended_safer` handoff must be present.

## Authoring Flows

Local development, sideload, and publish-preview rows use the same runtime-class, permission, rollback, and registry-binding vocabulary as public packages. Local or unsigned packages do not inherit verified-publisher or enterprise-approved trust merely because they run on the author machine.

## Support Export

The support export includes runtime class tokens, active inspector refs, downgrade banner refs, hosted surface refs, downgrade reasons, and whether the packet blocks a stable runtime-truth claim. It carries only opaque refs and reviewable summaries; raw credentials, package binaries, and hosted content do not cross this boundary.
