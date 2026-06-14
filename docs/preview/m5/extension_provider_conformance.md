# M5 extension-provider conformance

This document is the contract for the M5 extension/provider conformance packet. It
makes the canonical M5 preview/browser-runtime vocabularies **enforceable across
first-party packs and contributed extension providers**: every provider that backs
a claimed M5 preview/browser-runtime row must declare what it can do before it can
back the row, and a provider that becomes unavailable or weaker degrades to bounded
truth with repair guidance instead of silently swapping in weaker semantics.

Where the
[browser-runtime inspectors](browser_runtime_inspectors.md) packet materializes the
*per-inspector* truth a single DOM/CSS/console/network/storage inspector presents,
and the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed preview/runtime surface, this packet
materializes the *per-provider conformance* truth behind every claimed row.

Source remains canonical; the conformance packet is derivative — never a second
writable truth model. Each row names which provider backs it, what that provider
declared, whether the declaration satisfies the claimed row, and — when the
provider is stale, weaker, or unavailable — what bounded operating profile and
repair guidance apply.

## Source of truth

- Packet type: `ProviderConformancePacket`
  (`crates/aureline-preview/src/extension_provider_conformance/`).
- Boundary schema:
  `schemas/preview/extension_provider_conformance.schema.json`.
- Checked support export:
  `artifacts/preview/m5/extension_provider_conformance/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/extension_provider_conformance.md`.
- Protected fixtures:
  `fixtures/preview/m5/extension_provider_conformance/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_extension_provider_conformance [support|summary]`.

## Declare before you back

A provider — first-party or contributed — must carry an explicit
`ProviderDeclaration` before it can back a claimed M5 row. The declaration names:

| Field | Meaning |
| --- | --- |
| `supported_target_kinds` | Runtime target kinds the provider can back (`embedded_preview`, `external_browser`, `simulator_or_emulator`, `device_browser`, `remote_preview_session`, `captured_snapshot`) |
| `supported_mapping_qualities` | Source-mapping quality classes it can produce (`exact` / `approximate` / `generated_only` / `runtime_only`) |
| `max_attach_depth` | Deepest attach it declares (`no_attach` → `dom_only` → `dom_and_styles` → `dom_styles_network` → `dom_styles_network_storage`) |
| `hot_reload` | Hot-reload posture it declares (`supported` / `restart_only` / `unsupported` / `not_applicable`) |
| `client_scope_limit_token` | Opaque token naming its client-scope limit (e.g. single-client, shared-session-capped) |

Each claimed row carries a `ClaimedRowRequirement` (required target kind, minimum
mapping quality, minimum attach depth, whether hot reload is required). A provider
declaration **satisfies** a requirement when it supports the required target kind,
can produce a mapping quality at least as strong as required, attaches at least as
deep as required, and declares hot reload when the row requires it. A `live`
operating profile may only be backed by a satisfying, conformant provider — a row
never advertises capability its provider did not declare.

Both first-party and contributed providers must carry a complete declaration; a
contributed provider can never quietly inherit first-party trust.

## Provider status and operating profile

The `status` names the provider's conformance state right now, and the
`operating_profile` names the bounded posture the row currently presents:

| Status | Meaning |
| --- | --- |
| `conformant` | Declaration satisfies the requirement and the provider is available; the only status that may back `live` |
| `stale_declaration` | Declaration went stale and must be re-verified; the row is unresolved |
| `weaker_replacement` | A weaker provider would replace a stronger one; the swap is refused |
| `unavailable` | The provider became unavailable; its last declaration is preserved |

| Operating profile | Meaning |
| --- | --- |
| `live` | Full live runtime backing from a conformant provider |
| `mirror_offline` | A mirror/offline snapshot — bounded truth, no live runtime |
| `inspect_only` | No write-capable designer flow is offered |
| `policy_limited` | Policy narrows what the row may show or do |

A non-conformant provider never stays on a `live` profile. A `live` profile carries
no downgrade disclosure.

## No silent weaker swap; preserved history

`stale_declaration`, `weaker_replacement`, and `unavailable` all preserve the prior
provider declaration (`prior_declaration`) as history so a degraded provider never
erases what came before. A `conformant` row carries no prior declaration.

A `weaker_replacement` row must prove the swap is actually a regression: the current
declaration must be **strictly weaker** than the preserved prior on at least one
declared dimension (it drops a supported target kind, drops its best mapping
quality, reduces its max attach depth, or downgrades its hot-reload posture). A
weaker provider is never allowed to silently take over stronger-looking semantics.

## Bounded profiles stay explicit and exportable

Every non-conformant row, and every row on a bounded operating profile
(`mirror_offline`, `inspect_only`, `policy_limited`), must disclose why:

- a `downgrade_trigger` (e.g. `provider_unavailable`, `provider_declaration_stale`,
  `weaker_provider_proposed`, `offline_mirror_only`, `policy_narrowed`) names the
  cause verbatim instead of a generic error;
- a precise, non-generic `degraded_label` names the bounded truth the user is
  seeing; and
- `repair` guidance (a `RepairActionClass` plus a precise summary) tells the
  operator how to recover, so a degraded provider is never a dead end.

A clean `live`, `conformant` row carries no trigger, no degraded label, and no
repair guidance. Generic labels (`unavailable`, `error`, `stale`, `try again`, …)
are rejected.

## No inspect-to-write auto-upgrade

`offers_write_capable_flow` may be `true` only on a `live`, `conformant` row. An
`inspect_only`, `mirror_offline`, or `policy_limited` row is never auto-upgraded
into a write-capable designer flow.

## Guardrails

The packet carries a guardrail block that must hold in full:

- source remains canonical; the conformance packet is never a second writable truth
  model;
- runtime state and extension-private wording never hide source-mapping
  uncertainty;
- inspect-only rows are never auto-upgraded into write-capable designer flows;
- embedded preview/browser boundaries are not blurred into product authority;
- a weaker provider never silently swaps in stronger-looking semantics;
- a stale or unavailable provider preserves prior history and limitation notes;
- mirror/offline, inspect-only, and policy-limited profiles stay explicit and
  exportable on every claimed row.

## Boundary safety

Raw provider payloads, credentials, raw URLs, hostnames, raw runtime handles, and
extension-private wording never cross this boundary. The packet carries only typed
class tokens, opaque provider/evidence refs, booleans, and precise operator-facing
labels, so support and diagnostics exports can reconstruct exactly which provider
backed each claimed row and what operating profile the user saw.

## Consumers

Product, docs/help, diagnostics, support-export, and release-control surfaces
ingest these conformance rows instead of narrating provider behavior by hand, and
support/diagnostics exports can reconstruct the operating profile the user saw for
each claimed row.
