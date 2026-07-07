# M5 provider-account / offline-capture component consumers

Status: Stable · Schema `schemas/ui/m5-provider-account-offline-capture-component-consumer.schema.json` · Record kind `add_shared_work_item_status_transition_help_support_and_export_consumers_so_provider_account_and_offline_capture_components_keep_account_mapping_sync_and_redaction_language_aligned_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 provider-account / offline-capture
component matrix
(`docs/providers/m5_provider_account_offline_capture_component_matrix.md`). The matrix
freezes five governed component families and three sibling implement lanes narrow them into
working primitives:

| Component family | Narrowed primitive | Canonical schema |
| --- | --- | --- |
| `provider_account_row` | provider-account row | `schemas/ui/m5-provider-account-row.schema.json` |
| `project_or_board_mapping_row` | mapping / sync-behavior row | `schemas/ui/m5-provider-mapping-sync-behavior-row.schema.json` |
| `sync_behavior_row` | mapping / sync-behavior row | `schemas/ui/m5-provider-mapping-sync-behavior-row.schema.json` |
| `offline_capture_row` | offline-capture / privacy-redaction row | `schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json` |
| `privacy_redaction_row` | offline-capture / privacy-redaction row | `schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json` |

This lane proves those five families are **reusable components** — not one
provider-settings page plus a few isolated export objects — by binding every claimed M5
provider consumer to the same canonical component schemas and the same descriptor
vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| Work-Item Detail | `work_item_detail` | reads account / mapping / sync truth while viewing a linked work item |
| Status-Transition Review | `status_transition_review` | reviews a transition before it publishes to a provider |
| Issue Intake | `issue_intake` | captures a new issue, possibly offline |
| Help / Docs | `docs_help` | documents the same account / redaction / sync truth the product renders |
| Support / Export Desk | `support_export` | the authoritative rendering; references the canonical schemas so its prose can never drift |
| Browser Handoff | `browser_handoff` | reconnects / imports mid-handoff, where a session may be stale |

Every family is adopted by **at least two** distinct consumers, and the support / export
desk references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **account state, destination mapping, queued-draft
state, and redaction posture** across every provider surface. Those four descriptors
(`account_state`, `destination_mapping`, `queued_draft_state`, `redaction_posture`) are
required on every binding.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always
discloses a self-contained banner naming the exact reason and the recovery action — never a
generic "degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `scope_limited_narrowed` | `provider_scope_limited` | `reauthorize_for_full_scope` | `scope_limited_read_only` |
| `session_stale_narrowed` | `session_stale` | `refresh_stale_session` | `session_stale_cached_read` |
| `mapping_policy_locked_narrowed` | `mapping_policy_locked` | `request_mapping_policy_change_or_use_local` | `mapping_policy_locked_no_publish` |
| `packet_local_only_narrowed` | `packet_local_only` | `publish_queued_packet_when_online` | `packet_local_only_not_committed` |

### Cached / offline state is never committed state

`session_stale` and `packet_local_only` reflect **cached or offline-captured** state. The
resolver marks such a binding `reflects_cached_or_offline_state = true`, always narrows it,
and always resolves `asserts_provider_committed = false`. Only a full-parity binding may
reflect a provider-committed publish. This is the acceptance criterion that cached or
offline-captured state no longer masquerades as provider-committed state on any claimed M5
provider consumer.

## Resolver

`resolve_provider_component_binding` takes one consumer's adoption of one component family,
the descriptor set it surfaces, the parity-health mode, and any export caveats, and produces
one `M5ProviderComponentResolvedBinding`. It rejects an empty or incomplete descriptor set
and any forbidden binding material, preserves the descriptor vocabulary at full parity,
auto-narrows under any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-provider-account-offline-capture-component-consumer-proof/`, and the
two narrowed fixtures (browser-handoff → Beta, issue-intake → Preview) live under
`fixtures/ui/m5-provider-account-offline-capture-component-consumers/`. All are minted only
by the `aureline_provider_component_consumers` headless emitter so the in-code matrix, the
artifact, the worked bindings, and the fixtures never drift. Raw credentials, endpoints,
tokens, and raw provider bodies never cross this boundary.
