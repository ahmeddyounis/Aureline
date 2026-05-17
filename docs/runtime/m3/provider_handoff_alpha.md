# Alpha Provider Browser-Handoff, Import-Session, and Reconnect Continuity

This document is the reviewer-facing landing page for the alpha provider
browser-handoff record family: every typed
[`BrowserHandoffPacket`](../../../crates/aureline-provider/src/browser_handoff/mod.rs)
names origin lane, destination class, provider-object row id, and intended
follow-up action; every
[`ProviderReconnectFlow`](../../../crates/aureline-provider/src/browser_handoff/mod.rs)
resolves a reopened provider flow back to the authoritative local object or
to a truthful placeholder; every
[`ImportSessionRecord`](../../../crates/aureline-provider/src/browser_handoff/mod.rs)
preserves the same continuity vocabulary the provider-object support export
already uses.

The machine-readable boundary lives at
[`/schemas/providers/provider_browser_handoff_alpha.schema.json`](../../../schemas/providers/provider_browser_handoff_alpha.schema.json).
The Rust implementation lives at
[`/crates/aureline-provider/src/browser_handoff/`](../../../crates/aureline-provider/src/browser_handoff/mod.rs).
The protected fixture lives at
[`/fixtures/providers/m3/browser_handoff/page.json`](../../../fixtures/providers/m3/browser_handoff/page.json).

## The alpha promise

- Every browser-handoff packet identifies **origin** (which workspace lane
  minted it), **destination class** (which provider surface it routes to),
  **object id** (the provider-side id plus the local
  `provider_object_row_ref`), and **intended follow-up action** (the typed
  return contract). A packet without a typed follow-up is refused at mint
  time; a generic "Open in browser" affordance is forbidden.
- Reopened provider flows route back to the authoritative local object row
  whenever the local model is still authoritative. When the authoritative
  row is no longer available, the reconnect resolves to a typed
  [`HandoffPlaceholderClass`](../../../crates/aureline-provider/src/browser_handoff/mod.rs)
  that names *why* the local body could not be restored
  (`local_object_evicted_beyond_retention`,
  `superseded_by_newer_import_session`, `revoked_and_local_body_wiped`,
  `inspect_only_target_without_draft`, `locked_by_trust_posture_change`).
  Silent "session lost" states are forbidden.
- Import sessions and reconnect flows record continuity observations using
  the same closed
  [`ContinuityObservationClass`](../../../crates/aureline-provider/src/object_model/mod.rs),
  [`RetainedCapabilityClass`](../../../crates/aureline-provider/src/object_model/mod.rs),
  and
  [`DegradedActionClass`](../../../crates/aureline-provider/src/object_model/mod.rs)
  vocabulary as the provider-object support export. Support packets read
  one truth across the M3 provider lanes.

## Packet vocabulary

| Origin class | When it fires |
| --- | --- |
| `workspace_start_center` | Minted from the workspace start center / main window |
| `workspace_review_lane` | Minted from the review-pack inspector lane |
| `workspace_runtime_lane` | Minted from the runtime/task-graph lane |
| `workspace_git_lane` | Minted from the git/change-orchestration lane |
| `workspace_provider_lane` | Minted from the provider/registry inspector lane |
| `workspace_support_lane` | Minted from the support/diagnostics export lane |
| `headless_cli_surface` | Minted from the headless CLI |

| Destination class | Frozen route shape |
| --- | --- |
| `code_host_web` | Pull-request, branch view, comment |
| `issue_tracker_web` | Issue / work-item view |
| `ci_provider_web` | Pipeline / check / log / artifact view |
| `identity_provider_web` | Re-auth, step-up |
| `managed_admin_web` | Provider admin / managed admin |
| `docs_or_portal_web` | External docs, runbook, portal acceptance |

| Follow-up action class | Meaning |
| --- | --- |
| `return_to_local_draft_authoring` | Return to the local-draft authoring view for the same object row |
| `return_to_publish_later_queue_item` | Return to the publish-later queue entry (requires `publish_later_queue_item_ref`) |
| `return_to_inspect_only_view` | Return to the inspect-only view of the same object row |
| `return_to_review_anchor` | Return to the review-pack anchor that minted the packet |
| `return_to_run_or_task_anchor` | Return to the run/task-graph anchor that minted the packet |
| `return_to_change_lineage_view` | Return to the change-lineage / publish-readiness view |
| `return_to_authority_repair` | Return to the reauth / rescope prompt (must route to `identity_provider_web` or `managed_admin_web`) |
| `return_to_truthful_placeholder` | Return to a truthful placeholder when no authoritative local row remains |

## Packet states

| State | Meaning |
| --- | --- |
| `minted_awaiting_confirmation` | Packet minted and waiting for the user to confirm departure |
| `user_confirmed_pending_launch` | User confirmed departure; packet pending launch |
| `launched_awaiting_return` | Browser was launched and Aureline is awaiting return |
| `returned_authoritative_local_object` | Returned; follow-up resolved to the authoritative local row |
| `returned_truthful_placeholder` | Returned; follow-up resolved to a typed placeholder (requires `placeholder_kind`) |
| `returned_provider_observed_authoritative` | Returned; provider observation supersedes the local model |
| `returned_user_cancelled` | User cancelled before launch or before return |
| `returned_callback_invalid` | Callback failed validation (origin mismatch, replay outside window, …) |
| `returned_authority_revoked` | Provider revoked authority before return |
| `expired_unused` | Packet expired before the user returned |

Returned states (any of the `returned_*` states) MUST cite a
`return_summary`. Silent transition between states is forbidden.

## Import-session states

| Session state | Required freshness class |
| --- | --- |
| `pending_first_observation` | any |
| `observed_fresh` | `fresh` |
| `stale_within_window` | `stale_within_window` |
| `expired_beyond_window` | `expired_beyond_window` |
| `revoked_or_disconnected` | `revoked_or_disconnected` |
| `replaced_by_newer_session` | any (requires `superseded_session_ref`) |
| `abandoned_by_user` | any |

Degraded freshness classes (`stale_within_window`,
`expired_beyond_window`, `never_observed`, `revoked_or_disconnected`) MUST
cite a `degraded_reason`.

## Reconnect outcomes

| Outcome class | Required refs |
| --- | --- |
| `restored_authoritative_local_object` | `restored_object_row_ref` (and no `placeholder_kind`) |
| `restored_truthful_placeholder` | `placeholder_kind` (and no `restored_object_row_ref`) |
| `deferred_to_publish_later_queue` | `publish_later_queue_item_ref` |
| `browser_handoff_offered_again` | `follow_up_packet_ref` (referencing a packet in the page) |
| `denied_unknown_actor` | — (closes mutation authority) |
| `denied_policy_or_trust_mismatch` | — (closes mutation authority) |
| `pending_reauth` | — (closes mutation authority) |
| `pending_rescope` | — (closes mutation authority) |
| `pending_reapproval` | — (closes mutation authority) |

Every `originating_packet_ref` and `follow_up_packet_ref` MUST reference a
packet that exists on the page; every `import_session_ref` MUST reference
an import session on the page.

## Continuity-observation invariants

Continuity observations reuse the provider-object alpha vocabulary verbatim:

| Observation class | When it fires |
| --- | --- |
| `provider_offline` | Provider unreachable |
| `provider_stale_within_window` | Truth older than the freshness floor but inside the review window |
| `provider_expired_beyond_window` | Truth past the freshness window; refresh required |
| `provider_revoked_or_disconnected` | Grant revoked or disconnected |
| `provider_disagrees_with_local` | Provider truth disagrees with the local model |
| `provider_never_observed` | Provider has never been observed |

Each observation names a typed
[`RetainedCapabilityClass`](../../../crates/aureline-provider/src/object_model/mod.rs)
and a typed
[`DegradedActionClass`](../../../crates/aureline-provider/src/object_model/mod.rs).
`no_capability_retained` cannot pair with `local_editing_preserved=true` or
`continue_local_authoring`. Every observation MUST bind to exactly one of
a packet, an import session, or a reconnect flow.

## Validator invariants

[`ProviderBrowserHandoffAlphaPage::validate`](../../../crates/aureline-provider/src/browser_handoff/mod.rs)
enforces:

- `record_kind`, `schema_version`, and `shared_contract_ref` match the
  alpha constants on every page, packet, import session, reconnect flow,
  and continuity observation;
- packet ids, import-session ids, reconnect-flow ids, and observation ids
  are unique within a page;
- packets cite origin lane, host/workspace/execution-context refs,
  destination class, canonical-host and tenant refs, the provider-object
  row ref, and a non-empty provider-side object id;
- returned packets cite a `return_summary`;
- `returned_truthful_placeholder` packets cite a `placeholder_kind`;
- `return_to_publish_later_queue_item` follow-ups cite a
  `publish_later_queue_item_ref`;
- `return_to_authority_repair` follow-ups route to
  `identity_provider_web` or `managed_admin_web`;
- import sessions cite a freshness-floor ref, hold the admissible
  freshness class for their state, and (when degraded) cite a
  `degraded_reason`;
- `replaced_by_newer_session` import sessions cite a
  `superseded_session_ref`;
- reconnect flows reference an import session present on the page;
- restored-authoritative outcomes cite `restored_object_row_ref` and not a
  `placeholder_kind`; placeholder outcomes cite `placeholder_kind` and not
  a `restored_object_row_ref`;
- `originating_packet_ref` and `follow_up_packet_ref` reference packets on
  the page;
- continuity observations bind to exactly one of packet / import session /
  reconnect flow, reference an entry on the page, name a non-empty
  rationale, never silently widen mutation authority, and respect the
  `no_capability_retained` guardrails;
- the page covers review, runtime, and provider workspace-lane origins;
  code-host, issue-tracker, and CI-provider destinations; and the
  local-draft, publish-later, and truthful-placeholder follow-ups;
- the page covers both the `restored_authoritative_local_object` and
  `restored_truthful_placeholder` reconnect outcomes;
- guardrails are pinned closed: `raw_url_present`,
  `raw_token_material_present`, `raw_provider_payload_present`, and
  `silent_authority_widening_taken` are all `false` on packets and
  reconnect flows; `local_editing_preserved` is `true` on import sessions
  and reconnect flows.

The
[`ProviderBrowserHandoffAlphaSupportExport`](../../../crates/aureline-provider/src/browser_handoff/mod.rs)
projection drops action refs (`integration_packet_ref`,
`publish_later_queue_item_ref`, `approval_ticket_ref`,
`follow_up_packet_ref`) and any raw provider payloads so support bundles
inherit the redaction posture without per-surface review.

## Reviewer fixture

The protected fixture
[`/fixtures/providers/m3/browser_handoff/page.json`](../../../fixtures/providers/m3/browser_handoff/page.json)
covers:

- a **review-lane** code-host PR handoff in flight
  (`launched_awaiting_return`) with `return_to_local_draft_authoring`;
- a **runtime-lane** issue-tracker publish-later handoff that returned
  to the authoritative local object via a typed reconnect;
- a **provider-lane** CI annotation handoff that returned to a
  `revoked_and_local_body_wiped` truthful placeholder;
- three import sessions in `observed_fresh`, `stale_within_window`, and
  `revoked_or_disconnected` states;
- two reconnect flows exercising
  `restored_authoritative_local_object` and
  `restored_truthful_placeholder`;
- three continuity observations bound to packet, import session, and
  reconnect flow respectively.

Run the fixture validator:

```bash
cargo test -p aureline-provider --test browser_handoff_alpha
cargo run -p aureline-provider --bin aureline_provider_alpha -- \
    --browser-handoff-alpha --validate-only
```

## Out of scope

This alpha lane lands the typed record family and the first consumer (the
provider crate plus its alpha bin entry, fixture, schema, integration test,
and reviewer doc). Wiring the record into the workspace start center,
runtime task graph, review-pack inspector, and git change-lineage surfaces
is later work; this lane keeps the contract honest before those consumers
expand.
