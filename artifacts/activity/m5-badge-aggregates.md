# Badge-aggregates bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-badge-aggregates/canonical_bundle.json`](../../fixtures/activity/m5-badge-aggregates/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-badge-aggregates.schema.json`](../../schemas/activity/m5-badge-aggregates.schema.json).
It gives reviewers the frozen surface, aggregate, coalescing, and telemetry tables without
reading the JSON. The contract narrative lives in
[`/docs/activity/m5-badge-aggregates.md`](../../docs/activity/m5-badge-aggregates.md).

- Bundle id: `m5-badge-aggregates:bundle:0001`
- Record kind: `m5_badge_aggregates_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Surfaces: 5 · Items: 25 · Aggregates: 5 · Surface badges: 25 · Coalesced failures: 1 · Invariants: 14

## Governed badge surfaces

The five badge-bearing surfaces whose counts must match the same deduped durable truth. The
in-app activity center is the authoritative durable record; the others project the same
aggregate. The OS native notification is governed by the fanout-receipts lane, not here.

| Surface | Base redaction | Durable authoritative |
| --- | --- | --- |
| `in_app_activity_center` | `summary_only` | yes |
| `dock_taskbar_badge` | `count_only` | no |
| `browser_companion` | `summary_only` | no |
| `mobile_companion` | `summary_only` | no |
| `operator_dashboard` | `summary_only` | no |

## Per-scope aggregates

Each aggregate is the deduped count of distinct durable objects pending attention in a scope,
with the count class, freshness, privacy floor, and the reasons that explain why eligible
items are excluded.

| Scope | Count | Class | Raw events | Privacy floor | Reopen |
| --- | --- | --- | --- | --- | --- |
| `app_global` | 0 | `none` | 0 | `summary_only` | scope pending list |
| `workspace` | 10 | `many` | 11 | `redacted_payload` | scope pending list |
| `session` | 3 | `few` | 5 | `redacted_payload` | scope pending list |
| `collaboration` | 2 | `few` | 2 | `redacted_payload` | scope pending list |
| `tenant_org` | 1 | `single` | 1 | `count_only` | exact object |

The `app_global` badge counts zero yet names a `muted_by_focus_mode` mute reason and a
`rate_limited` suppression reason; the `workspace` and `session` badges show the count is
**deduped, not a raw tally** (10 from 11, 3 from 5); the `tenant_org` badge counts one
managed alert while naming a `policy_suppressed` reason and a `follow_admin_policy`
quiet-hours mode; and the `collaboration` badge counts a `security_advisory` the user muted
and quiet hours deferred — **a security advisory is never silenced**.

## Cross-surface parity

For every scope, all five governed surfaces show the **same count and count class**; only the
applied redaction differs. The dock/taskbar badge is always `count_only`; no surface widens
privacy below the aggregate floor. A worked example — the `workspace` badge (count 10):

| Surface | Count | Count class | Applied redaction |
| --- | --- | --- | --- |
| `in_app_activity_center` | 10 | `many` | `redacted_payload` |
| `dock_taskbar_badge` | 10 | `many` | `count_only` |
| `browser_companion` | 10 | `many` | `redacted_payload` |
| `mobile_companion` | 10 | `many` | `redacted_payload` |
| `operator_dashboard` | 10 | `many` | `redacted_payload` |

## Repeated-failure coalescing

Repeated failures sharing one root cause collapse into one durable object — counted once in
the badge — with the correct authoritative reopen path, instead of spamming the OS
notification, badge, companions, and operator dashboard.

| Root cause | Scope | Occurrences | Counted in badge | Spam prevented | Reopens |
| --- | --- | --- | --- | --- | --- |
| `session:save_conflict:doc7` | `session` | 3 | once | yes | representative object |

## Telemetry — stable enums and counts, no message text

The telemetry packet records the notification class, route, and outcome by stable token plus
a count only. `captures_message_text` is `false`.

| Total | Value |
| --- | --- |
| items | 25 |
| counted (deduped) | 16 |
| raw counted | 19 |
| deduped repeats | 3 |
| muted | 2 |
| suppressed | 2 |
| deferred | 1 |
| settled | 1 |
| coalesced failures | 1 |
| failure occurrences | 3 |

Every route (badge-bearing surface) reports the same total badge count (16) — parity at the
telemetry level.

## Computed invariants (all hold)

| Invariant |
| --- |
| `badge.counts_deduped_durable_items` |
| `badge.count_class_matches_count` |
| `badge.excluded_reasons_named` |
| `badge.muted_suppressed_not_counted` |
| `badge.cross_surface_parity` |
| `badge.surface_never_widens_privacy` |
| `badge.route_to_authoritative` |
| `badge.repeated_failures_coalesce` |
| `badge.coalesced_failure_reopen_authoritative` |
| `badge.security_never_silenced` |
| `badge.telemetry_stable_enums_no_text` |
| `badge.matrix_bound` |
| `badge.deterministic_reproducible` |
| `badge.support_export_safe` |

The freeze gate `crates/aureline-activity/tests/m5_badge_aggregates.rs` rebuilds the bundle
in code and asserts it equals this fixture byte-for-byte; an inconsistent edit flips an
invariant or fails the round-trip.
