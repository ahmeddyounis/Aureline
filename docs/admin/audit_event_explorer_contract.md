# Audit-event explorer, actor/scope/outcome taxonomy, and export/filter contract

This contract freezes the user-visible audit-event model Aureline shows
when an admin, support engineer, or enterprise reviewer needs to answer
"who did what, where, when, and with what outcome" for material trust,
policy, auth, provider-routing, collaboration, publication, remote-
mutation, and delete or export lifecycle events. It exists so the audit
event explorer, decision-history rows, support packets, admin handoff
exports, the policy center, and CLI projections all answer the same
questions with the same vocabulary instead of forcing readers into raw
logs or vendor-only browser consoles.

The audit explorer is a reviewable, redaction-aware projection over
already-typed packets and decision rows. It does not author or sign
audit events; it does not retain them; it does not act as a backend or
SIEM. It freezes the boundary that desktop UI, CLI, support exports,
and tenant-scoped admin handoffs can rely on regardless of whether the
deployment is account-free, self-hosted, mirrored, or air-gapped.

## Companion artifacts

- [`/schemas/admin/audit_event_record.schema.json`](../../schemas/admin/audit_event_record.schema.json)
  — boundary schema for one `audit_event_record` row and the
  `audit_event_explorer_view_record` snapshot the explorer renders
  from a filtered set.
- [`/schemas/admin/audit_event_filter.schema.json`](../../schemas/admin/audit_event_filter.schema.json)
  — boundary schema for `audit_event_filter_record` (the saved or
  applied filter), `audit_event_export_record` (the paired
  human-readable summary plus machine-readable packet handoff), and
  `audit_event_history_completeness_record` (the offline, mirrored,
  partial-history posture of the explored view).
- [`/fixtures/admin/audit_event_explorer_cases/`](../../fixtures/admin/audit_event_explorer_cases/)
  — worked YAML cases for a policy change, a trust downgrade, a
  collaboration-control grant, a failed privileged action, and a
  delete-request lifecycle event, plus a partial-history view and a
  tenant-scoped export packet.
- [`/schemas/identity/admin_audit_packet.schema.json`](../../schemas/identity/admin_audit_packet.schema.json)
  — upstream typed audit packet record used by the explorer for
  identity-mode, policy-bundle, entitlement, seat or device, org
  switch, and revocation events.
- [`/schemas/admin/effective_policy_card.schema.json`](../../schemas/admin/effective_policy_card.schema.json)
  and [`/docs/admin/policy_explainability_contract.md`](./policy_explainability_contract.md)
  — sibling vocabulary for effective-policy cards, policy-diff views,
  decision-history rows, and admin handoff exports the explorer deep
  links into.
- [`/schemas/admin/seat_lifecycle_row.schema.json`](../../schemas/admin/seat_lifecycle_row.schema.json),
  [`/schemas/admin/fleet_status_row.schema.json`](../../schemas/admin/fleet_status_row.schema.json),
  and [`/docs/admin/org_admin_seat_and_fleet_contract.md`](./org_admin_seat_and_fleet_contract.md)
  — organization, directory or provider, seat lifecycle, group-to-
  policy targeting, and fleet vocabulary the explorer reuses for
  scope, source, and affected surface family.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — chronology rules (wall-clock, monotonic, UTC export, local
  rendering, skew labelling) and delete-request versus delete-complete
  semantics the explorer preserves for delete or export lifecycle
  rows.
- [`/docs/network/transport_permission_matrix.md`](../network/transport_permission_matrix.md)
  and [`/fixtures/network/audit_event_examples/`](../../fixtures/network/audit_event_examples/)
  — shared network permission-class, mirror or offline, and audit-
  requirement matrix the explorer reuses for provider-routing and
  remote-mutation rows.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  — registry separation between admin-audit, support, telemetry,
  usage, and offboarding payloads. The explorer never collapses these
  payload families into one stream.

Normative product sources for this contract are the enterprise audit
plane requirements, the admin and policy center UX, the support
bundle and admin handoff layouts, the offline-first explainability
guarantees, and the tenant-scoped export rules in `.t2/docs/`. If this
document disagrees with those sources, the `.t2/docs/` source wins and
this contract must be updated in the same change.

## Scope

Frozen at this revision:

- one shared vocabulary for stable event id and class, actor class,
  target scope, decision outcome, source system class, mirror or
  offline posture, redaction class, related-object reference,
  affected surface family, completeness state, and export consumer
  class;
- three strict JSON Schema files that together cover:
  - `audit_event_record` — one durable row;
  - `audit_event_explorer_view_record` — the explorer's reviewable
    snapshot of a filter applied to a tenant scope at a moment in
    time;
  - `audit_event_filter_record` — the filter that produced the view;
  - `audit_event_export_record` — the paired human-readable summary
    plus machine-readable handoff;
  - `audit_event_history_completeness_record` — the offline,
    mirrored, partial-history posture of the explored view;
- worked YAML fixtures for at least the five required event families
  plus partial-history and export packets; and
- export and filter rules that hold for desktop, CLI, support, and
  tenant-scoped admin handoff surfaces.

Out of scope:

- implementing an audit backend, SIEM forwarder, or retention store;
- authoring or signing audit packets; the explorer reads them;
- a browser-only admin console;
- live OIDC, SCIM, billing, or provider integrations; and
- any flow that requires a vendor-only hosted control plane to be
  reachable for the explorer to render.

## Core principles

1. Every row answers who, what, where, when, and outcome. The five
   axes are mandatory on every record. A row that cannot answer one
   of them MUST mark that axis as a typed unknown with a reason class
   instead of silently dropping it.
2. The explorer is a projection, not a sink. Records are minted by
   upstream packet schemas (admin-audit packet, network audit event,
   policy decision-history row, record-state lifecycle event) and the
   explorer carries the typed event class, an opaque source packet
   ref, and a redaction-aware reviewable summary.
3. Standards-based, file-based, mirrored, and air-gapped sources are
   peers. The same actor, target, outcome, and source vocabulary
   covers an OIDC- or SCIM-driven event, a signed bundle import, a
   manual file import, an offline transfer, and a runtime preload.
4. Stale and offline are first-class state. Partial, mirrored, or
   stale views are labelled rather than silently presenting an
   implied full history. The explorer never claims completeness it
   cannot prove.
5. Delete request is distinct from delete completion. Delete or export
   lifecycle events preserve the record-state model from
   `governance/record_state_and_policy_simulation_models.md`; a
   delete-requested row does not imply a delete-complete row.
6. No vendor-only control plane is assumed. Every event family must
   be readable from a self-hosted, mirrored, file-imported, or air-
   gapped install when the upstream packet is locally available, with
   typed gap notes when it is not.
7. Exports stay clean. The machine-readable packet carries refs,
   schema refs, typed vocabulary, completeness posture, and the
   reviewable summary. Raw policy rule bodies, raw bundle bytes, raw
   signing material, raw issuer URLs, raw SCIM endpoints, raw mirror
   hostnames, raw IP addresses, raw device fingerprints, raw user
   identifiers, raw email or display names, raw subject claims, raw
   provider payloads, raw paths, raw tokens, raw command lines, and
   raw secret material never cross this boundary.

## Shared terms

Every audit-explorer surface MUST preserve these field families by
name when it renders, logs, exports, or emits CLI output.

| Field family | Meaning |
|---|---|
| `event_identity` | Stable opaque event id, monotonic event sequence, source packet ref, source schema ref, and event class. |
| `actor` | Actor class plus opaque actor ref and export-safe label. Raw subject claims, raw email addresses, and raw display names never appear. |
| `target_scope` | Tenant or org ref, deployment profile, scope kind (tenant, workspace, install profile, group, seat, device, capability scope, account boundary, session or command), affected surface family, and inheritance summary. |
| `outcome` | Decision class plus outcome class (`succeeded`, `denied`, `narrowed`, `paused`, `deferred`, `failed_with_error`, `superseded`, `requested_pending_completion`, `completed`, `rolled_back`, `unknown_offline`). |
| `chronology` | Event-time UTC timestamp, monotonic sequence, original timezone label, render-time hint, and skew or partial-order class. |
| `source_system` | Source class (admin-audit packet, network audit event, policy decision-history row, record-state lifecycle event, retention or hold event, collaboration-control grant, remote mutation receipt, publication state event, support hold event), plus mirror or offline posture. |
| `redaction_summary` | Closed redaction class plus the included and omitted data classes, mirroring the policy and seat contracts. |
| `related_objects` | Typed refs to related effective-policy cards, policy diffs, decision-history rows, retention or deletion matrix rows, seat lifecycle rows, fleet ring dashboards, network audit event examples, support packets, admin handoff exports, and docs anchors. |
| `actions` | Open-related-object actions (`open_audit_event_detail`, `open_source_packet`, `open_decision_history_row`, `open_policy_diff`, `open_retention_deletion_matrix`, `open_seat_lifecycle_row`, `open_fleet_ring_dashboard`, `open_collab_control_grant`, `open_remote_mutation_receipt`, `open_publication_state_record`, `open_record_state_event`, `copy_human_summary`, `export_machine_packet`, `open_admin_handoff`, `open_support_packet`, `open_cli_json`, `open_docs`). |
| `completeness` | The history-completeness class for the view (`live_authoritative`, `mirrored_current`, `mirrored_stale_within_grace`, `mirrored_stale_past_grace`, `offline_last_known_good`, `offline_unverified`, `partial_history_window`, `partial_history_filtered`, `partial_history_redacted`, `unavailable_offline`). A view that cannot prove completeness MUST mark itself accordingly. |
| `export_pair` | Paired human-readable summary ref plus machine-readable packet ref with format set, schema ref, redaction summary, completeness posture, and compatible consumer classes. |

## Event families

The explorer freezes a minimum set of event families. Every family
maps to one or more typed source packet schemas; the explorer never
invents a parallel event vocabulary.

| `event_family_class` | Description | Typical source packet |
|---|---|---|
| `trust_change` | Workspace trust state, signer continuity, trust-root rotation, or verification posture change. | admin-audit packet, policy decision-history row. |
| `policy_change` | Policy-bundle import, narrowing, widening, rollback, or emergency disable. | admin-audit packet (`policy_bundle_change`), policy decision-history row. |
| `permission_or_entitlement_change` | Entitlement lifecycle transition, plan change, quota grant or revoke, capability scope change, or seat-bound capability change. | admin-audit packet (`entitlement_lifecycle_transition`, `seat_action`), entitlement snapshot, decision-history row. |
| `provider_routing_change` | AI provider, hosted-provider, mirror, broker, or runtime egress route narrowing, switch, or denial. | network audit event, policy decision-history row. |
| `collaboration_control_grant` | Real-time collaboration capability grant, revocation, or constraint (review owner, approval gate, sharing surface). | collaboration-control grant record, decision-history row. |
| `publication_state_change` | Marketplace publish, unpublish, deprecation, or quarantine of an extension, model pack, doc pack, or workspace artifact. | publication-state record, decision-history row. |
| `remote_mutation` | Remote write, port-forward, tunnel start or stop, remote command, or signed remote action receipt. | remote mutation receipt, network audit event. |
| `delete_or_export_lifecycle_event` | Delete request, delete completion, hold applied, hold released, retention waiver, export ready, export consumed, offboarding export. | record-state lifecycle event, retention or hold event, support hold event. |
| `failed_privileged_action` | A privileged action denied or failed (admin attempt without entitlement, signature failure, missing credential, unverified mirror, blocked by hold). Mirrors the outcome `denied` or `failed_with_error` across other families and is rendered as its own family for triage. | any of the above. |

A surface that lists event families MUST list every family it knows
about, including `failed_privileged_action`, instead of silently
filtering failures out of "successful events". Failures are first-
class.

## Audit-event record

`audit_event_record` is the durable row the explorer renders, stores
in a view snapshot, or exports. Every record MUST preserve:

- `event_id_ref`, an opaque stable id;
- `event_class` from the closed event-class vocabulary
  (`policy_bundle_change`, `entitlement_lifecycle_transition`,
  `seat_action`, `device_action`, `revocation_event`, `org_switch`,
  `identity_mode_transition`, `trust_state_change`,
  `signer_rotation`, `provider_route_change`, `network_egress_event`,
  `collaboration_control_grant`, `collaboration_control_revoke`,
  `publication_state_change`, `remote_mutation_receipt`,
  `record_delete_requested`, `record_delete_completed`,
  `record_hold_applied`, `record_hold_released`,
  `record_export_ready`, `record_export_consumed`,
  `support_hold_event`, `failed_privileged_action`,
  `admin_console_explainability_assertion`);
- `event_family` from the closed family vocabulary above;
- `actor`, with closed `actor_class`, opaque `actor_ref`, export-safe
  label, and reviewable note;
- `target_scope`, with `scope_kind` (tenant_or_org, deployment_profile,
  install_profile, workspace, folder_or_module, group, seat, device,
  capability_scope, account_boundary, session_or_command),
  `tenant_or_org_ref`, deployment-profile scope, affected surface
  family (one of `editor`, `terminal`, `notebook`, `git`, `vcs`,
  `extension`, `marketplace_publish`, `ai`, `provider_routing`,
  `network`, `remote_session`, `collaboration`, `policy_center`,
  `settings`, `support_export`, `retention_or_deletion`,
  `auth_or_session`, `admin_handoff`, `not_applicable`), and
  reviewable inheritance summary;
- `outcome`, with `decision_class` (allow, deny, narrow, force_disable,
  quota_limit, defer_pending_refresh, defer_pending_admin, escalate,
  export_only, local_only_continue, mutation_recorded,
  request_recorded, rollback_recorded, unknown_offline) and
  `outcome_class` (succeeded, denied, narrowed, paused, deferred,
  failed_with_error, superseded, requested_pending_completion,
  completed, rolled_back, unknown_offline);
- `source_system`, with `source_class` (admin_audit_packet,
  policy_decision_history_row, network_audit_event,
  record_state_lifecycle_event, retention_or_hold_event,
  collaboration_control_grant, publication_state_event,
  remote_mutation_receipt, support_hold_event, signer_continuity_event,
  effective_policy_resolution, locally_derived_explanation),
  `source_packet_ref` (opaque), `source_schema_ref`, mirror or offline
  posture from the shared distribution-freshness vocabulary, and
  reviewable note;
- `chronology`, with `event_at_utc` (RFC 3339 UTC, monotonic or
  bounded-skew), `monotonic_sequence` (integer, scoped to the source
  packet stream), `original_timezone_label` (export-safe label, e.g.
  `America/New_York`), `render_timezone_hint_class` (`utc_only`,
  `viewer_local`, `tenant_default`, `unspecified_partial_order`), and
  `skew_class` (`monotonic_clean`, `bounded_skew`,
  `partial_order_imported`, `partial_order_offline`,
  `unsynchronized_skew`);
- `policy_epoch_ref` and `entitlement_epoch_ref`, each nullable for
  events that legitimately have no bound epoch (for example a
  `record_export_ready` for an account-free local install);
- `redaction_summary`, with the closed redaction class plus the
  included and omitted data classes;
- `related_objects`, an array of typed refs into effective policy
  cards, policy diffs, decision history rows, retention or deletion
  matrix rows, seat lifecycle rows, fleet ring dashboards, network
  audit event examples, support packets, and docs anchors. Empty
  arrays are admissible; null entries are not;
- `actions`, an array of typed action refs from the closed action
  vocabulary, each with enabled state and reviewable reason; and
- `reviewable_summary`, a single reviewable sentence answering who,
  what, where, when, and outcome without naming raw secrets, raw
  policy bodies, raw user identities, or raw provider payloads.

A row MUST NOT collapse `failed_privileged_action` into a generic
`policy_change` or `provider_routing_change`. The family carries the
triage signal even when the underlying packet is also one of those
classes.

A row MUST NOT collapse `record_delete_requested` and
`record_delete_completed` into a single `delete` row. Delete request
is the moment the user (or admin, or policy) asked for deletion;
delete completion is the moment the record-state lifecycle event
transitioned to delete-complete. Holds, retention windows, and
operator review can sit between them.

## Audit-event explorer view

`audit_event_explorer_view_record` is the snapshot the explorer
renders for one tenant scope at one moment in time. It MUST
preserve:

- `view_id_ref`, an opaque stable id;
- the `audit_event_filter_record` ref that produced the view;
- the `audit_event_history_completeness_record` ref describing the
  view's completeness posture;
- `tenant_scope` and `policy_context`, mirrored from the records the
  view summarizes;
- a bounded `event_rows` array of `audit_event_record` instances,
  page-respecting and ordered by `event_at_utc` desc within
  `monotonic_sequence` desc;
- a `family_count_summary` mapping each `event_family_class` to a
  non-negative count over the visible window. Counts are exact for
  the visible window only; they are not implied to cover the full
  history. The summary MUST flag any family the completeness record
  marks as truncated;
- `outcome_count_summary`, the same shape over the closed
  `outcome_class` vocabulary;
- `affected_surface_summary`, the same shape over affected surface
  families;
- `export_pair` for the human-readable summary plus machine-readable
  packet handoff;
- `deep_links` to the filter record, related effective-policy cards,
  decision-history rows, retention or deletion matrix rows, seat
  lifecycle rows, fleet ring dashboards, network audit event
  examples, support packets, and docs anchors; and
- a `redaction_summary` for the view as a whole.

The view MAY be rendered as a desktop dashboard, a CLI table, or a
support-bundle section. A compact projection MAY hide individual rows
behind disclosure controls but MUST NOT drop the completeness record,
the family count summary, or the redaction summary.

## Filter contract

`audit_event_filter_record` is the saved or applied filter the view
materializes. Filters are reviewable, redaction-aware, and stable
across desktop, CLI, support, and admin handoff surfaces. Every
filter MUST cover:

- `tenant_or_org_ref` (nullable for account-free local) and
  `deployment_profile_scope` (one or more deployment profile
  classes);
- `scope_filters`: zero or more closed `scope_kind` selectors, each
  with an opaque scope ref and reviewable label. An empty array means
  "every scope visible to the caller", not "every scope in the
  tenant";
- `actor_filters`: zero or more `actor_class` selectors plus
  optional opaque actor refs;
- `outcome_filters`: zero or more `outcome_class` selectors;
- `event_family_filters`: zero or more `event_family_class`
  selectors;
- `event_class_filters`: zero or more `event_class` selectors. A
  filter that names an event class MUST also name (or accept) the
  matching family;
- `source_class_filters`: zero or more `source_class` selectors;
- `affected_surface_filters`: zero or more affected surface family
  selectors;
- `time_range`: an event-time UTC window with `from_at`, `to_at`,
  inclusive flags, plus a `time_basis_class` (`event_time_utc`,
  `monotonic_sequence`, `tenant_local_time`,
  `imported_partial_order`). When `imported_partial_order` is in
  effect, the explorer MUST flag the view as partial-order in the
  completeness record;
- `epoch_pin`: a nullable pair of `policy_epoch_ref` and
  `entitlement_epoch_ref`. When set, the explorer MUST exclude rows
  outside that epoch from the visible window;
- `redaction_floor`: a redaction class. The view MUST refuse to
  expand below this floor. A user without operator-only restricted
  access cannot widen a filter to peek at operator-only rows;
- `result_limit` and `result_offset`: bounded paging controls; and
- a `filter_label` (export-safe) plus a reviewable filter summary.

A filter that requests a scope, source, family, class, surface, or
outcome value the caller is not authorized to see MUST surface a
typed `filter_clause_blocked_class` value (`scope_not_authorized`,
`source_not_authorized`, `redaction_floor_locked`,
`epoch_outside_visible_window`, `time_range_outside_visible_history`)
on the view rather than silently producing zero rows.

## Offline, mirrored, and partial-history behavior

`audit_event_history_completeness_record` is the typed posture of the
view. It MUST cover:

- `completeness_class` from the closed completeness vocabulary in
  Shared terms;
- `mirror_posture_class` reused from the policy explainability
  vocabulary;
- `last_successful_sync_at` (nullable) for the source streams the
  view depends on;
- `partial_history_reason_class`, one of `none`,
  `mirror_unreachable`, `mirror_stale_past_grace`,
  `offline_unverified`, `time_window_pruned`,
  `epoch_window_pruned`, `redaction_floor_truncated`,
  `result_limit_truncated`, `imported_partial_order`,
  `vendor_console_only_acknowledged_gap`;
- `acknowledged_gap_waiver_refs`, an array of opaque waiver refs that
  acknowledge a vendor-console-only gap when applicable;
- `next_safe_action_class` (`refresh_mirror`, `import_offline_bundle`,
  `widen_time_range`, `widen_redaction_floor`, `escalate_to_admin`,
  `continue_local_only`, `none`); and
- a reviewable `completeness_summary`.

A view that cannot prove `live_authoritative` posture MUST NOT claim
it. The view MUST render with the visible window plus an explicit
typed gap note. Counts MUST be qualified ("counts within the visible
window"). New managed privilege MUST NOT be displayed as
authoritatively granted from a `mirrored_stale_past_grace` or
`offline_unverified` source; the row MUST still render but with the
typed posture and a reviewable note.

## Tenant-scoped admin-audit export

`audit_event_export_record` freezes the tenant-scoped admin-audit
export packet. Every export MUST carry:

- `export_id_ref`, an opaque stable id;
- `assembled_at_utc`;
- `tenant_scope` and `policy_context`;
- `policy_epoch_at_export` and `entitlement_epoch_at_export`,
  required because a downstream reviewer must reproduce the policy
  and entitlement context the explorer used;
- `vendor_console_independence_class` from
  (`fully_independent_local_packet`,
  `independent_with_signed_evidence`,
  `vendor_console_required_for_full_detail`,
  `vendor_console_only_acknowledged_gap`). The first two values mean
  an enterprise reviewer can reconstruct the event from the packet
  alone; the second two require declared waivers or evidence;
- `included_event_rows`, a bounded list of `audit_event_record`
  rows, each carrying the minimum field set defined below;
- `included_completeness_record`, the
  `audit_event_history_completeness_record` for the export window;
- `human_readable_summary` (a reviewable sentence) and
  `machine_readable_packet` (an `export_pair` ref) that together
  pair plain-language and machine-readable handoff;
- `redaction_summary` for the export as a whole; and
- `compatible_consumer_classes` from `desktop_policy_center`,
  `cli_json_reader`, `support_bundle_exporter`,
  `admin_audit_reader`, `project_doctor`,
  `automated_policy_diff_tool`, `enterprise_audit_reviewer`.

### Minimal export field set

The minimum field set every exported `audit_event_record` MUST carry
when it crosses the tenant-scoped export boundary is:

- `event_id_ref`;
- `event_class` and `event_family`;
- `actor.actor_class`, `actor.actor_ref`, `actor.actor_label`;
- `target_scope.tenant_or_org_ref`, `target_scope.scope_kind`,
  `target_scope.affected_surface_family`,
  `target_scope.scope_label`;
- `outcome.decision_class` and `outcome.outcome_class`;
- `chronology.event_at_utc`,
  `chronology.original_timezone_label`,
  `chronology.skew_class`;
- `source_system.source_class`,
  `source_system.source_packet_ref`,
  `source_system.source_schema_ref`,
  `source_system.mirror_or_offline_posture`;
- `policy_epoch_ref` and `entitlement_epoch_ref` when bound;
- `redaction_summary.redaction_class` plus the included and omitted
  data classes;
- `related_objects` refs needed to reconstruct the event (related
  effective-policy card, policy diff, decision history row,
  retention or deletion matrix row, seat lifecycle row, fleet ring
  dashboard, network audit event example, support packet); and
- `reviewable_summary`.

The minimum field set is intentionally narrower than the in-explorer
record. Raw policy rule bodies, raw bundle bytes, raw signing
material, raw issuer URLs, raw SCIM endpoints, raw mirror hostnames,
raw IP addresses, raw device fingerprints, raw user identifiers, raw
email or display names, raw subject claims, raw provider payloads,
raw paths, raw tokens, raw command lines, and raw secret material
never cross the export boundary; the packet carries opaque refs,
typed vocabulary, and reviewable summaries only.

### Vendor-console independence

A `fully_independent_local_packet` export MUST be reconstructable by
an enterprise reviewer with no access to a vendor admin console.
That means every row's source packet ref resolves to a locally
available record (admin-audit packet, network audit event example,
policy decision-history row, record-state lifecycle event), every
related-object ref resolves locally, and the completeness record
states `live_authoritative`, `mirrored_current`,
`offline_last_known_good`, or
`partial_history_window` with a typed reason.

A `vendor_console_only_acknowledged_gap` export MUST cite at least
one waiver ref in the completeness record. The export must still
carry the minimum field set; the waiver acknowledges that some
related-object refs cannot be resolved locally.

## Deep-link and reuse rules

1. Every `audit_event_record` MUST link to its source packet ref,
   schema ref, and the related-object refs needed to answer who,
   what, where, when, and outcome from inside the product.
2. A `policy_change` row MUST link to the matching effective-policy
   card, policy diff view, and decision-history row when those
   records exist.
3. A `permission_or_entitlement_change` row MUST link to the matching
   seat lifecycle row, entitlement snapshot ref (via the source
   packet), and decision-history row.
4. A `provider_routing_change` row MUST link to the matching network
   audit event example or its source schema.
5. A `collaboration_control_grant` or `collaboration_control_revoke`
   row MUST link to the matching collaboration-control grant record
   and decision-history row.
6. A `publication_state_change` row MUST link to the matching
   publication-state record and, when applicable, the marketplace
   review or moderation record.
7. A `remote_mutation` row MUST link to the matching remote mutation
   receipt and network audit event example.
8. A `record_delete_requested`, `record_delete_completed`,
   `record_hold_applied`, `record_hold_released`,
   `record_export_ready`, or `record_export_consumed` row MUST link
   to the matching record-state lifecycle event and retention or
   deletion matrix row.
9. A `failed_privileged_action` row MUST link to the matching
   decision-history row and the next safe action.
10. Deep links are stable route refs, not browser-only console URLs.
    They MUST work in desktop, CLI projection, self-hosted, mirrored,
    air-gapped, or partially managed contexts whenever the referenced
    record is locally available.
11. If a referenced record is unavailable offline, the link still
    renders with an unavailable reason class and the next safe
    action.

## Cross-surface vocabulary parity

The audit explorer reuses, rather than re-defines, the vocabulary
established by sibling contracts:

| Concept | Source contract |
|---|---|
| `actor_class` | `schemas/identity/admin_audit_packet.schema.json` plus `schemas/admin/effective_policy_card.schema.json`. The explorer extends the closed set only with values already present in those schemas. |
| `decision_class` | `schemas/admin/effective_policy_card.schema.json`. |
| `outcome_class` | The explorer freezes the outcome vocabulary listed above, which is a strict superset of the policy-explainability `decision_outcome_class` plus typed mutation, request, rollback, and unknown-offline outcomes. |
| `mirror_or_offline_posture` and `distribution_freshness_class` | `schemas/admin/effective_policy_card.schema.json` and `schemas/identity/admin_audit_packet.schema.json`. |
| Affected surface family | The explorer freezes the closed list above, which mirrors product feature areas already named in the seat lifecycle, fleet, and network permission contracts. |
| Redaction class and redaction summary | Mirrored from the policy explainability and seat lifecycle contracts. |
| Record-state lifecycle vocabulary | `docs/governance/record_state_and_policy_simulation_models.md` and the linked record-state schema. |

Surfaces that need a new actor, target, outcome, or source value MUST
extend the upstream schema first, then surface it in the explorer.
The explorer never mints near-synonyms.

## Action vocabulary

The closed `action_class` vocabulary is:

- `open_audit_event_detail`;
- `open_source_packet`;
- `open_decision_history_row`;
- `open_policy_diff`;
- `open_retention_deletion_matrix`;
- `open_seat_lifecycle_row`;
- `open_fleet_ring_dashboard`;
- `open_collab_control_grant`;
- `open_remote_mutation_receipt`;
- `open_publication_state_record`;
- `open_record_state_event`;
- `copy_human_summary`;
- `export_machine_packet`;
- `open_admin_handoff`;
- `open_support_packet`;
- `open_cli_json`;
- `open_docs`.

Every action ref MUST carry an `enabled` boolean and a reviewable
`reason`. A disabled action MUST cite why ("source packet is offline",
"redaction floor blocks export", "completeness record marks the row
truncated") rather than render as a silently dead control.

## Filtering and export rules

1. The explorer MUST always show the active filter and the active
   completeness record. A view that hides either is a defect.
2. A view that returns zero rows MUST distinguish "filter matched
   nothing" from "filter clause blocked" and from "completeness
   record truncated the visible window".
3. The same filter applied to a desktop view and a CLI projection
   MUST produce the same row set within a single completeness window.
   Differences only arise when the completeness record changes.
4. Machine-readable export output MUST stay clean. CLI or headless
   JSON cannot be polluted by progress text, instructional copy,
   screenshots, or rendered Markdown. Human-readable context belongs
   in the paired summary.
5. A tenant-scoped export MUST carry the `policy_epoch_at_export`
   and `entitlement_epoch_at_export`. An export without bound epochs
   MUST be marked `not_applicable_account_free_local` rather than
   omitted.
6. Exports MUST NOT include whole policy bundles, raw policy rule
   bodies, raw signatures, raw tenant directory payloads, raw
   provider payloads, raw issuer URLs, raw SCIM endpoint URLs, raw
   mirror hostnames, raw user identifiers, raw email addresses, raw
   display names, raw group display names, raw device hostnames, raw
   IP addresses, raw serial numbers, raw command lines, raw paths,
   raw tokens, or raw secret material. They MAY include opaque refs,
   fingerprints, source labels, schema refs, redaction notes, and
   reviewable summaries.

## Fixture coverage

The seeded audit-event explorer cases cover at least:

- a `policy_change` row driven by an admin-audit `policy_bundle_change`
  packet, with deep links into the effective policy card, policy diff,
  and decision-history row;
- a `trust_state_change` row that downgrades workspace trust, with
  deep links into the matching policy-decision row and the next safe
  action;
- a `collaboration_control_grant` row that constrains a real-time
  collaboration capability for a workspace and links to the matching
  collaboration-control grant record;
- a `failed_privileged_action` row where a privileged action is
  denied by a stale-past-grace policy bundle, with the typed source
  posture preserved on the row;
- a `record_delete_requested` row that is held by an active support
  hold, distinct from any `record_delete_completed` row that may
  appear later;
- a partial-history view whose completeness record cites
  `mirror_stale_past_grace` and a `next_safe_action_class` of
  `refresh_mirror`; and
- a tenant-scoped admin-audit export carrying the minimum field set,
  the bound policy and entitlement epochs, and a vendor-console-
  independence class of `fully_independent_local_packet`.

Each fixture carries actor, target scope, outcome, chronology, source
system posture, redaction summary, related-object refs, and the
reviewable summary. Fixtures are examples of the product contract;
they do not describe an audit backend, retention store, or browser-
only admin console.

## Change management

- Adding a new `event_class`, `event_family_class`, `outcome_class`,
  `source_class`, `affected_surface_family`, `action_class`,
  `completeness_class`, `partial_history_reason_class`,
  `vendor_console_independence_class`, or
  `filter_clause_blocked_class` value is additive-minor and bumps
  `audit_event_explorer_schema_version` in each affected schema.
- Renaming or repurposing an existing value is breaking and requires
  a governance decision because it would change the meaning of
  existing support packets, CLI output, decision-history rows,
  fixture cases, and admin handoff exports.
- Any change that weakens five-axis answerability (who, what, where,
  when, outcome), collapses delete-requested with delete-completed,
  hides `failed_privileged_action` as a first-class family, lets
  partial or stale views imply full history, or lets vendor-console-
  only state cross the export boundary without a typed waiver MUST
  update this contract, both schemas, and the fixtures together.
