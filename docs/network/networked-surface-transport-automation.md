# Networked-surface transport automation

This layer is the **audit and automation truth source** of the networked-surface
transport-governance lane. The
[transport-decision log](./networked-surface-transport-decision.md) emits one
inspectable decision per network-capable *action*, the
[explainability layer](./networked-surface-transport-explainability.md) projects
that stream into posture, ledger, and explain views, and the
[mirror/offline continuity layer](./networked-surface-mirror-offline-continuity.md)
freezes per-family route handling. This layer answers the question those leave
open for activity history, support exports, and headless automation: **what one
denial vocabulary, one set of activity filters, and one route/origin history do
all M5 networked surfaces share?**

The runtime owner is
`aureline_remote::networked_surface_transport_automation`; the boundary schema is
`schemas/network/networked_surface_transport_automation.schema.json`.

No raw URLs, raw hostnames, raw ports, raw paths, raw query strings, raw
cookies, raw headers, raw bearer or session tokens, raw private certificate
bytes, or raw SSH private material cross the boundary — only closed-vocabulary
tokens, opaque refs, UTC timestamps, counts, and plain-language summary
sentences.

## The canonical denial vocabulary

Every network failure across every claimed M5 surface resolves to exactly one
[`TransportDenialClass`] token. The eight required codes are:

- `proxy_misconfigured` — the resolved proxy configuration is invalid,
- `proxy_auth_required` — the selected proxy demands unsatisfied authentication,
- `ca_untrusted` — the endpoint's certificate chain could not be trusted,
- `ssh_host_key_unknown` — the SSH host key is unknown, changed, or revoked,
- `egress_blocked_policy` — transport policy forbids egress for the action,
- `mirror_unreachable` — the declared signed mirror could not be reached,
- `offline_mode` — the surface is offline and no in-policy route is available,
- `origin_scope_ambiguous` — the origin ownership scope could not be resolved.

`none` is the sentinel for an allowed action. The per-feature denial classes —
the matrix `DenialReasonClass`, the proxy `ProxyResolutionDenialClass`, and the
trust `TrustDenialClass` — all map into this vocabulary via
`TransportDenialClass::from_matrix_denial`, `::from_proxy_denial`, and
`::from_trust_denial`, so a failure is explained with one stable token rather
than a per-feature error string.

## Network-activity filters

Each [`NetworkActivityRecord`] carries the surface, origin scope, endpoint
class, egress class, route choice, allow/deny disposition, and canonical denial
code for one network-capable action. [`ActivityFilter`] selects records by any
of six dimensions (`surface`, `origin_scope`, `endpoint_class`, `route_choice`,
`disposition`, `denial_code`), and the page exposes the available
[`ActivityFilterFacet`] values per dimension, so product, CLI, and support views
render identical filter chips and a saved filter is portable across surfaces.

## Route/origin history joins

[`RouteOriginJoinRow`] aggregates the activity history by `(route_choice,
origin_scope)`: per route and origin it reports how many actions were allowed,
denied, or deferred, which surfaces appeared, and which canonical denial codes
appeared — all without scanning raw logs. The join totals reconstruct the full
activity count exactly.

## Redaction-safe automation packets

Every record, row, join, summary, and support export carries closed-vocabulary
tokens, opaque refs, UTC timestamps, and counts only. The packet preserves
origin scope, route choice, and denial code on every record so headless
automation and support exports retain the audit signal without ever exporting a
secret or a payload.

## Stability conditions

The page qualifies `stable` only when **all** of the following hold:

1. Every required M5 surface has at least one activity record.
2. No raw private material is present on any record.
3. Every record resolved through the shared governance layer (`no_bypass: true`).
4. Every deferred record queues only an idempotent action.
5. Every denied record carries a non-`none` canonical denial code.
6. Every allowed record carries the `none` canonical denial code.
7. The activity history surfaces the complete canonical denial vocabulary, and
   the page exposes it as reusable vocabulary.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false`
  (`raw_private_material_exposed`),
- a record with `no_bypass: false` (`bypassed_shared_governance`),
- a deferred record that queues a non-idempotent action
  (`non_idempotent_replay_queued`).

A missing required surface narrows the packet to `preview`. A denied record
without a canonical code (`denied_without_canonical_code`), a
disposition/denial-code mismatch (`disposition_denial_code_mismatch`), or an
incomplete denial vocabulary (`denial_vocabulary_incomplete`) narrows the
affected row to `beta`, which lets release and support tooling detect and
automatically narrow stale or under-qualified rows before publication.

## CLI / support / product parity

Product, CLI/headless, and support exports all render an activity record through
`render_fields()` over the single [`ACTIVITY_FIELD_NAMES`] catalog, so the
denial codes a user reads in the UI are byte-for-byte the same tokens CLI output
and support packets quote. The
`dump_networked_surface_transport_automation_fixtures` example's `activity-cli`
and `denied-filter` subcommands are the headless rendering of this catalog and
of the activity filter.

## Truth paths

- Doc: `docs/network/networked-surface-transport-automation.md`
- Artifact: `artifacts/network/networked-surface-transport-automation.md`
- Schema:
  `schemas/network/networked_surface_transport_automation.schema.json`
- Fixtures: `fixtures/network/networked_surface_transport_automation/`
- Contract ref: `remote:networked_surface_transport_automation:v1`

[`TransportDenialClass`]: ../../crates/aureline-remote/src/networked_surface_transport_automation/mod.rs
[`NetworkActivityRecord`]: ../../crates/aureline-remote/src/networked_surface_transport_automation/mod.rs
[`ActivityFilter`]: ../../crates/aureline-remote/src/networked_surface_transport_automation/mod.rs
[`ActivityFilterFacet`]: ../../crates/aureline-remote/src/networked_surface_transport_automation/mod.rs
[`RouteOriginJoinRow`]: ../../crates/aureline-remote/src/networked_surface_transport_automation/mod.rs
[`ACTIVITY_FIELD_NAMES`]: ../../crates/aureline-remote/src/networked_surface_transport_automation/mod.rs
