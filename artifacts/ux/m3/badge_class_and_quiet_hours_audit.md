# Badge Class And Quiet-Hours Audit

This audit summarizes the durable-attention badge and suppression rows
generated under
[`fixtures/ux/m3/activity_center_corpus/`](../../../fixtures/ux/m3/activity_center_corpus/).
The source of truth is the generated packet, not screenshots or copied
toast text.

## Badge-Class Results

| Rule | Result |
| --- | --- |
| Badge counts use one class at a time | PASS |
| Mixed-class totals are denied | PASS |
| Badge expansion maps to the authoritative object | PASS |
| Repeated failures coalesce before count contribution | PASS |
| Held/suppressed counts remain visible | PASS |
| Non-badge rows contribute zero counts | PASS |

Badge source classes covered:

- `derived_from_envelope_state`
- `derived_from_canonical_object`
- `aggregated_grouped_burst`
- `not_a_badge_source`

Badge classes covered:

- `durable_running_count`
- `completion_unread`
- `needs_review`
- `held_or_suppressed_count`
- `failed_runs`
- `session_requests`

## Quiet-Hours And Suppression Results

| Rule | Result |
| --- | --- |
| Held quiet-hours rows preserve durable history | PASS |
| Admin suppression preserves audit and support-export lineage | PASS |
| Critical or blocking trust bypass uses typed escalation | PASS |
| Cross-client fanout dedupes by canonical event id | PASS |
| Suppressed external payloads keep privacy-safe summaries | PASS |
| Shortcut actions cannot bypass preview, approval, or trust logic | PASS |

Decision classes covered:

- `not_suppressed`
- `held_quiet_hours`
- `admin_suppressed`
- `critical_bypass`
- `cross_client_deduped`

## Exact Reopen Results

| Reopen class | Count | Result |
| --- | ---: | --- |
| `exact_durable_object` | 11 | PASS |
| `truthful_placeholder` | 1 | PASS |
| `denied_requires_revalidation` | 1 | PASS |

Every proof sets `generic_home_fallback_denied=true` and
`preserves_preview_approval_trust_logic=true`.

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- badge-audit
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- quiet-hours-audit
cargo test -q -p aureline-shell --test durable_attention_beta_fixtures
```
