# M5 Work-Item Component Accessibility & Auto-Narrowing (M05-986)

This lane is the **accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone** over the frozen M5 work-item component matrix
(`schemas/ui/m5-work-item-component-matrix.schema.json`, M05-980). Where the freeze matrix
defines the reusable work-item row, provider-chip group, relation strip, sync-pending pill,
work-item detail header, status-transition sheet, related-evidence card, and
offline-handoff-packet card primitives, and the M05-981 through M05-985 implementation /
consumer lanes resolve their per-surface truth, this lane certifies — per component family —
that work-item claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
self-narrowing** across desktop, assistive, headless, and export paths.

- Module: `crates/aureline-provider/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_write_scope_sync_state_or_packet_publishability_is_stale_blocked_or_local_only_across_claimed_m5_work_item_components`
- Schema: `schemas/ui/m5-work-item-component-accessibility-parity.schema.json`
- Support export: `artifacts/release/m5-work-item-component-accessibility-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-work-item-component-accessibility-proof/matrix.csv`
- Report: `artifacts/release/m5-work-item-component-accessibility-proof.md`
- Fixtures: `fixtures/ui/m5-work-item-component-accessibility-parity/`

## What it guarantees

1. **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
   screen-reader-reachable, and CLI/headless-reachable path into the same canonical work-item
   identity, provider authority, local-versus-provider state, linked engineering context,
   side-effect preview, and publish-later continuity the rich component shows — never a
   hover-only chip that strands assistive-tech or headless users. Hierarchy-heavy families (the
   offline-handoff-packet card's nested packet / evidence / queued-draft lineage) additionally
   bind their tree to a flat list / textual path.
2. **Export parity.** The support / release / evaluation export reconstructs each component's
   meaning from typed tokens and opaque refs without a screenshot, preserving the same canonical
   IDs, provider authorities, local-versus-provider labels, linked-context refs, side-effect
   previews, queued-draft counts, redaction classes, and narrowing reasons shown in-product.
3. **Honest auto-narrowing.** When provider freshness is stale, write scope is read-only or
   policy-blocked, sync state is local-only, or an offline-handoff packet cannot publish safely,
   the component's provider claim auto-narrows from `provider_committed` / `reviewable_projection`
   to a `stale_freshness_projection` / `read_only_projection` / `local_only_projection` /
   `unpublishable_packet_projection` projection, discloses the narrowing with a precise trigger
   and binding dimension, and preserves the canonical identity / authority / linked-context /
   queued-draft lineage. A component with every dimension intact must not carry a spurious
   narrowing, and a **cached or offline-captured state never masquerades as provider-committed**.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the inbox, detail,
   relation-panel, sync-status, transition-sheet, evidence-panel, status-bar, general product UI,
   headless CLI, and support / release exports so product, docs, and release publication stay
   aligned on provider-boundary downgrade behavior — no generic ticket / task wording is allowed
   to conceal provider ownership, queued local state, offline capture, or linked context.

## Claim ladder

| Claim | Rank | Meaning |
| --- | --- | --- |
| `provider_committed` | 5 | Fresh, in-scope, provider-synced; Aureline can read and write and commit now. |
| `reviewable_projection` | 4 | Self-sufficient, reviewable read-only projection; not itself a committed write. |
| `stale_freshness_projection` | 3 | Provider projection is stale; only a cached read; refresh before trusting as live. |
| `read_only_projection` | 2 | Effective write scope is read-only or policy-blocked; no committed write. |
| `local_only_projection` | 1 | Work item is local-only and unsynced; nothing published, publish-later queued. |
| `unpublishable_packet_projection` | 0 | Offline-handoff packet cannot publish safely; nothing handed off, retry-or-export held. |

## Condition → ceiling → trigger

| Condition state | Permitted ceiling | Frozen trigger |
| --- | --- | --- |
| `fresh_committed` | `provider_committed` | — (baseline) |
| `freshness_stale` | `stale_freshness_projection` | `local_versus_provider_state_hidden` |
| `write_scope_blocked` | `read_only_projection` | `provider_authority_unstated` |
| `sync_local_only` | `local_only_projection` | `sync_pending_state_hidden` |
| `packet_unpublishable` | `unpublishable_packet_projection` | `publish_later_continuity_hidden` |

`freshness_stale`, `sync_local_only`, and `packet_unpublishable` are the cached-or-offline
states: a row modeling any of them must never let its effective claim assert `provider_committed`.

## Coverage

Eight rows over eight frozen families (one per family): **4 green / 4 yellow / 0 red**. Every
claim dimension, every condition state, every claim tier, and all nine consumer surfaces are
exercised across the packet.

## Regenerating artifacts

The support export, CSV, report, and fixtures are generated from the single in-code seed and are
byte-checked by the test suite. Regenerate with:

```
GEN_WORK_ITEM_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-provider --lib generate_artifacts
```
