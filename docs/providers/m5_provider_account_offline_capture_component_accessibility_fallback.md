# M5 Provider-Account / Offline-Capture Component Accessibility & Auto-Narrowing (M05-922)

This lane is the **accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone** over the frozen M5 provider-account / offline-capture component matrix
(`schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json`, M05-916). Where
the freeze matrix defines the reusable provider-account row, project/board mapping row,
sync-behavior row, offline-capture row, and privacy/redaction row primitives, and the M05-917
through M05-921 implementation / consumer lanes resolve their per-surface truth, this lane
certifies — per component family — that provider-boundary claims stay **keyboard-complete,
assistive-tech-reachable, CLI/export-safe, and self-narrowing** across desktop, assistive,
headless, and export paths.

- Module: `crates/aureline-provider/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_scope_is_limited_mappings_are_blocked_or_offline_capture_packets_remain_local_only_across_claimed_m5_provider_components`
- Schema: `schemas/ui/m5-provider-account-offline-capture-component-accessibility-fallback.schema.json`
- Support export: `artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-provider-account-offline-capture-component-accessibility-fallback/`

## What it guarantees

1. **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
   screen-reader-reachable, and CLI/headless-reachable path into the same provider identity,
   tenant scope, effective write scope, mapping origin, sync mode, queued-draft state, and
   redaction / export boundary the rich component shows — never a hover-only chip that strands
   assistive-tech or headless users. Hierarchy-heavy families (the offline-capture row's nested
   packet / queued-draft / destination lineage) additionally bind their tree to a flat list /
   textual path.
2. **Export parity.** The support / release / evaluation export reconstructs each component's
   meaning from typed tokens and opaque refs without a screenshot, preserving the same stable
   connection states, tenant / write-scope labels, mapping origins, sync modes, queued-draft
   counts, redaction classes, and narrowing reasons shown in-product.
3. **Honest auto-narrowing.** When provider scope is limited, a session is stale, a mapping is
   policy-blocked, or an offline-capture packet remains local-only, the component's provider
   claim auto-narrows from `provider_committed` / `reviewable_projection` to a
   `limited_scope_projection` / `stale_session_projection` / `policy_blocked_mapping` /
   `local_only_packet` projection, discloses the narrowing with a precise trigger and binding
   dimension, and preserves the canonical account / mapping / queued-draft / redaction lineage.
   A component with every dimension intact must not carry a spurious narrowing, and a **cached or
   offline-captured state never masquerades as provider-committed**.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the account-settings UI,
   mapping-picker, sync-status, offline-queue, privacy-review, status-bar, general product UI,
   headless CLI, and support / release exports so product, docs, and release publication stay
   aligned on provider-boundary downgrade behavior.

## Claim ladder

| Claim | Rank | Meaning |
| --- | --- | --- |
| `provider_committed` | 5 | Reachable, in-scope, session-fresh; Aureline can read and write and commit now. |
| `reviewable_projection` | 4 | Self-sufficient, reviewable read-only projection; not itself a committed write. |
| `limited_scope_projection` | 3 | Effective write scope is limited; read / limited-write only. |
| `stale_session_projection` | 2 | Session is stale; only a cached read; re-authenticate before trusting as live. |
| `policy_blocked_mapping` | 1 | Mapping is policy-blocked; no committed destination resolves. |
| `local_only_packet` | 0 | Offline-capture packet remains local-only; nothing published, publish-later queued. |

## Condition → ceiling → trigger

| Condition state | Permitted ceiling | Frozen trigger |
| --- | --- | --- |
| `in_scope_committed` | `provider_committed` | — (baseline) |
| `scope_limited` | `limited_scope_projection` | `write_scope_unstated` |
| `session_stale` | `stale_session_projection` | `connection_state_unstated` |
| `mapping_policy_blocked` | `policy_blocked_mapping` | `mapping_origin_unstated` |
| `packet_local_only` | `local_only_packet` | `offline_capture_state_unstated` |

`session_stale` and `packet_local_only` are the cached-or-offline states: a row modeling either
must never let its effective claim assert `provider_committed`.

## Coverage

Six rows over five frozen families (the provider-account row is certified twice — scope-limited
and session-stale): **2 green / 4 yellow / 0 red**. Every claim dimension, every condition state,
every claim tier, and all nine consumer surfaces are exercised across the packet.

## Regenerating artifacts

The support export, CSV, report, and fixtures are generated from the single in-code seed and are
byte-checked by the test suite. Regenerate with:

```
GEN_PROVIDER_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-provider --lib generate_artifacts
```
