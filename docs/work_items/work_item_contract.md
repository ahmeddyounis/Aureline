# Work-item detail, status-transition, and offline-handoff contract

This document freezes the cross-tool work-item object model that
every Aureline surface reads when it presents a provider-owned issue,
work item, story, task, bug, incident, change request, or RFC. The
goal is to keep linked engineering context — the issue's branch /
worktree, its review workspace or review pack, its change object or
patch stack, its validation evidence, and its publish preview —
attached to the same record family, and to keep the row truthful
across provider outages, browser-blocked launches, restricted
workspace trust, freshness drift, account-mapping switches,
policy-epoch rolls, and intentional offline capture.

The machine-readable boundaries are:

- [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json)
  — `work_item_detail_record` and `work_item_detail_audit_event_record`.
- [`/schemas/work_items/status_transition_packet.schema.json`](../../schemas/work_items/status_transition_packet.schema.json)
  — `status_transition_packet_record` and `status_transition_packet_audit_event_record`.
- [`/schemas/work_items/offline_handoff_packet.schema.json`](../../schemas/work_items/offline_handoff_packet.schema.json)
  — `offline_handoff_packet_record` and `offline_handoff_packet_audit_event_record`.

Worked fixtures (provider-authoritative-synced detail row fully linked
into branch / review / change / evidence / preview; provider-overlay-
stale row degrading honestly; local-draft row never published; queued-
publish row pinned to a publish-later queue item; linked-review-only
row with no provider overlay; imported-handoff evidence-only row;
publish-now status-transition packet; deferred-publish status-transition
packet; open-in-provider status-transition packet routed through a
browser-handoff packet; local-draft-only status-transition packet;
captured-offline status-transition packet pinned to an offline-handoff
packet; offline-handoff packet captured under provider unreachable;
offline-handoff packet captured under browser blocked; offline-handoff
packet imported from a support export with no live provider path;
offline-handoff packet drained then accepted; offline-handoff packet
drained then rejected with a typed reason) live under
[`/fixtures/work_items/provider_cases/`](../../fixtures/work_items/provider_cases/).

This contract **composes with and does not replace** the upstream
contracts it cites:

- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and the schemas
  [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  and
  [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
  — mutation modes (`local_draft`, `publish_now`, `open_in_provider`,
  `deferred_publish`, `inspect_only`), surface classes,
  provider-actor classes, redaction classes, replay postures,
  predicted-side-effect classes, irreversibility classes,
  conflict-policy classes, mode-admission reasons, queue-state
  vocabulary, and the `publish_later_queue_item_record` /
  `provider_consequence_preview_record` /
  `account_mapping_binding_record` /
  `provider_object_relation_record` /
  `provider_callback_envelope_record` shapes work-item rows cite by
  reference.
- [`/docs/vcs/review_workspace_contract.md`](../vcs/review_workspace_contract.md)
  and the schemas
  [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  and
  [`/schemas/vcs/review_anchor.schema.json`](../../schemas/vcs/review_anchor.schema.json)
  — `review_workspace_record`, `review_anchor_record`, and
  `merge_queue_action_record` the work-item engineering-relation
  block cites for issue-to-branch and linked-review.
- [`/docs/vcs/review_pack_contract.md`](../vcs/review_pack_contract.md)
  and
  [`/schemas/vcs/review_pack.schema.json`](../../schemas/vcs/review_pack.schema.json)
  and
  [`/schemas/vcs/review_evaluation_result.schema.json`](../../schemas/vcs/review_evaluation_result.schema.json)
  — `review_pack_record` and `review_evaluation_result_record` the
  work-item engineering-relation block cites for linked-review and
  validation-evidence.
- [`/docs/vcs/change_stack_contract.md`](../vcs/change_stack_contract.md)
  and
  [`/schemas/vcs/change_object.schema.json`](../../schemas/vcs/change_object.schema.json)
  and
  [`/schemas/vcs/patch_stack.schema.json`](../../schemas/vcs/patch_stack.schema.json)
  — `change_object_record` and `patch_stack_record` the work-item
  engineering-relation block cites for change-intent.
- [`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
  and
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — `object_handoff_packet_record` the offline-handoff packet may
  attach by reference when the user routes the offline capture
  through the support-handoff lane.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — issue-routing matrix the offline-handoff packet's
  `external_handoff_export_to_managed_admin_only` and
  `support_bundle_attachment_by_reference` export routes compose
  with.
- ADR-0001 / ADR-0007 / ADR-0010 / ADR-0011 / ADR-0018 — workspace
  trust, secret-broker handle / raw-secret-forbidden boundary,
  browser-handoff and approval-ticket envelope, capability lifecycle
  / freshness / client-scope / redaction, and workspace-trust state.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — work-item
  detail, provider-mode, and offline-handoff passages.
- `.t2/docs/Aureline_Technical_Design_Document.md` — provider object
  model, status-transition preview, offline-handoff packet, and
  publish-later passages.
- `.t2/docs/Aureline_PRD.md` — work-item detail MUST/SHOULD language
  for honest provider freshness, durable engineering linkage, and
  truthful offline / queued / browser-handoff disclosure.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Without one frozen contract: the issue / planning surface invents a
local-draft work-item shape, the review workspace invents a parallel
"linked issue" shape, the change-stack panel invents a third "issue
ref" shape, the support-handoff packet invents a fourth, the support
export reader sees four incompatible work-item shapes, and a single
"Save / Comment / Close / Reopen" button on one surface means
something different from the same button on another. Worse, when the
provider goes down, every surface reinvents its own "the provider is
unavailable" copy, and offline captures silently relabel themselves
as "saved" without disclosing that the provider has not yet accepted.

This contract closes that gap with **one work-item detail record**,
**one status-transition packet** for the previewed-but-not-yet-applied
mutation, and **one offline-handoff packet** for the captured-under-
unavailability case. Every provider-linked work-item surface reads
these three records.

## Scope

Frozen at this revision:

- one `work_item_detail_record` per provider-owned (or local-draft /
  imported-handoff / linked-review-only / cached-read-only-shadow)
  work item — with provider, canonical id, title, current state rows,
  owner / assignee rows, freshness chip, write-authority chip, and
  the engineering-artifact relation block (issue-to-branch,
  linked-review, change-intent, validation-evidence, publish-preview);
- one `status_transition_packet_record` per previewed transition
  over a work-item detail row — with typed transition entries naming
  the transition kind, the transition action class (mutate-provider /
  save-local-draft / queue-for-publish-later / route-through-browser-
  handoff / inspect-only / captured-offline-pending-drain), the
  projected notification side effects, the projected permission
  scope, and the typed admissibility class;
- one `offline_handoff_packet_record` per captured-offline update —
  with the typed admission reason, the typed provider-acceptance
  class (defaulting to *not_submitted_local_capture_only*), the
  typed export-route set, the typed retry-route set, the typed
  drain-state, snapshotted state rows / owner rows / engineering
  relations, and optional bound publish-later queue item /
  status-transition packet / browser-handoff packet / account-mapping
  / support-bundle / incident-workspace / object-handoff refs;
- one closed audit-event vocabulary per record family;
- one closed denial-reason vocabulary per record family.

## Out of scope

- Implementing issue-provider adapters (GitHub Issues, Linear, Jira,
  Azure Boards, Asana, Pivotal, Shortcut, on-prem trackers). This
  document freezes the contract those adapters will satisfy.
- Live provider HTTP / OAuth / webhook / signature-verification
  protocols. Auth callbacks stay on the auth-callback packet;
  provider-object callbacks land on the provider-mode contract's
  `provider_callback_envelope_record`.
- The publish-later drain service, the offline-handoff queue
  drainer, or the admin reconciliation console. The contract is the
  vocabulary those services read and write.
- Final user-facing copy. The schemas freeze the typed vocabulary;
  copy lives in the design system.
- Cross-tracker mirroring, dedup heuristics, or cross-provider
  identity reconciliation.

## 1. Work-item detail header

Every work-item row Aureline shows resolves through one
`work_item_detail_record`. The header carries:

- a `work_item_authority_class` from the seven-value frozen vocabulary
  (provider-authoritative-synced, provider-authoritative-stale-local-
  continues, local-draft-no-provider-object, queued-publish-local-
  authored, linked-review-only-no-provider-overlay, imported-handoff-
  evidence-only, cached-read-only-shadow-inspect-only);
- a `target_object_identity` block (provider-side id, provider-host
  handle, tenant / org scope, `object_class` from the frozen nine-
  value vocabulary issue_or_work_item / epic_or_initiative /
  user_story / task_or_subtask / bug_report / incident_report /
  change_request / rfc_or_proposal / other);
- a reviewable `title_label`;
- an ordered `current_state_rows` array of typed state rows;
- a `freshness_class` from the six-value frozen vocabulary
  (live-authoritative-fresh, warm-within-grace, degraded-beyond-grace-
  local-continues, unverifiable-provider-unreachable, imported-
  snapshot-no-refresh-path, local-draft-never-published);
- a `write_authority_class` from the eleven-value frozen vocabulary;
- the `engineering_artifact_relations` block (section 2);
- an `origin_disclosure` and `policy_context` block;
- a `redaction_class` from the ADR-0010 / ADR-0007 set.

The header never carries raw provider URLs, raw provider issue
bodies, raw comment bodies, raw delegated tokens, raw branch /
commit URLs, raw author identity strings, raw absolute paths, or
raw notification payloads. All identity crosses the boundary as
opaque refs and reviewable labels (<= 1024 graphemes).

### 1.1 State rows

Each row in `current_state_rows` names a `state_family_class` (one of
eleven frozen axes: lifecycle / assignee / label / milestone /
priority / blocking / review-or-merge / validation-evidence /
publish-preview / freshness / trust-or-redaction), the verbatim
upstream `state_value` token, a `state_value_origin_class` from the
four-value frozen vocabulary (provider-authoritative-state-token,
local-draft-pending-publish, imported-handoff-state-token-no-refresh,
derived-from-linked-review-state), and a reviewable `summary`.

This preserves the provider's exact upstream vocabulary without
collapsing into a packet-local enum, and it lets a downstream surface
render the row mechanically from `state_family_class` even when the
`state_value` token is opaque.

### 1.2 Owner / assignee rows

Each row in `owner_or_assignee_rows` names an `owner_role_class`
(primary-owner, secondary-owner, assignee, reviewer, reporter,
watcher-or-subscriber, managed-admin-owner), an opaque
`actor_subject_ref`, the typed `actor_class` from the ADR-0010
provider-actor vocabulary, and an optional reviewable `actor_label`.
Raw author identity strings never cross this boundary.

### 1.3 Freshness chip

The `freshness_class` chip MUST be rendered explicitly on every
surface that shows the row. The repair-aware lanes (degraded-beyond-
grace-local-continues, unverifiable-provider-unreachable, imported-
snapshot-no-refresh-path) MUST NOT be relabelled as live-authoritative-
fresh. local-draft-never-published is admissible only on local-draft-
no-provider-object and queued-publish-local-authored authority.

### 1.4 Write-authority chip

The `write_authority_class` chip pre-resolves whether the surface may
mint a write-class control. The four `write_admissible_*` lanes name
the only path that admits a write (publish-now, browser-handoff, queue,
local-draft); the seven `write_blocked_*` lanes deny a write with a
typed reason rather than a generic "unavailable" label. Every
`status_transition_packet_record` authored against the row inherits
the write-authority chip's narrowing.

## 2. Engineering-artifact relations

The `engineering_artifact_relations` block surfaces a frozen 5-axis
matrix that binds a provider-owned work item back into Aureline's
engineering objects. **Every axis is required on every detail row**
so a downstream surface never has to guess whether a relation is
"missing" or "not modeled".

| Axis                      | Class enum                                                              | Typed ref(s)                                                                                        |
|---------------------------|-------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------|
| issue-to-branch link      | `issue_to_branch_link_class` (six values)                              | `linked_branch_local_locator_ref`, `linked_branch_review_workspace_record_id_ref`, `linked_branch_browser_handoff_packet_ref` |
| linked review             | `linked_review_class` (seven values)                                   | `linked_review_workspace_record_id_ref`, `linked_review_pack_record_id_ref`, `linked_review_evaluation_result_record_id_ref`, `linked_review_anchor_record_id_refs` |
| change intent             | `change_intent_class` (seven values)                                   | `linked_change_object_record_id_ref`, `linked_patch_stack_record_id_ref`                            |
| validation evidence       | `validation_evidence_class` (eight values)                             | `linked_validation_evidence_record_id_refs`                                                         |
| publish preview           | `publish_preview_class` (five values)                                  | `linked_provider_consequence_preview_record_id_ref`, `linked_publish_later_queue_item_record_id_ref`, `linked_provider_object_relation_record_id_ref`, `linked_object_handoff_packet_record_id_ref`, `linked_offline_handoff_packet_record_id_ref` |

Each axis is paired through `allOf` gates with the matching ref:
`linked_review_workspace_local_truth_only` /
`linked_review_workspace_with_provider_overlay` MUST cite a
`linked_review_workspace_record_id_ref`;
`change_object_local_draft` / `change_object_provider_authoritative`
MUST cite a `linked_change_object_record_id_ref`; and so on. The
authority class also gates: `imported_handoff_evidence_only` MUST
cite a `linked_offline_handoff_packet_record_id_ref`;
`queued_publish_local_authored` MUST cite a
`linked_publish_later_queue_item_record_id_ref`;
`linked_review_only_no_provider_overlay` MUST cite a review-workspace
or review-pack ref.

The relation block is the single seam every downstream surface reads
to render a work-item row's engineering context. No surface mints a
parallel "linked branch" or "linked review" or "change ref" or
"evidence ref" or "publish-preview ref" shape.

## 3. Status-transition packet

A `status_transition_packet_record` is the structured preview the
user confirms against before any state change is applied. It binds
back to one `work_item_detail_record_id_ref`, names a top-level
`mutation_mode` from the ADR-0010 set, and carries an ordered list of
`transition_entries`. Each entry names:

- a `transition_kind_class` from the seventeen-value frozen vocabulary
  (add-or-update-comment, delete-comment, assign-or-unassign-owner,
  change-lifecycle-state-token, change-priority-or-severity-token,
  change-milestone-or-iteration-token, add-or-remove-label,
  link-branch-or-worktree, unlink-branch-or-worktree,
  link-review-or-change-object, unlink-review-or-change-object,
  attach-validation-evidence, detach-validation-evidence,
  update-publish-preview, add-or-remove-subscriber-or-watcher,
  mark-blocked-or-blocking-relationship, rename-title-label,
  merge-or-close-with-resolution-token);
- a `transition_action_class` from the six-value frozen vocabulary
  (mutate-provider-state-publish-now, save-local-draft-only-no-
  provider-path, queue-for-publish-later-deferred, route-through-
  browser-handoff-open-in-provider, inspect-only-no-mutation,
  captured-offline-pending-drain);
- a `notification_side_effect_class` from the nine-value frozen
  vocabulary (including the safe default
  `notification_unknown_until_publish_admitted_pending_review` for
  queue / browser-handoff routes and
  `notification_blocked_pending_workspace_trust` for restricted-trust
  authority);
- a `permission_scope_class` from the ten-value frozen vocabulary;
- a `transition_admissibility_class` from the twelve-value frozen
  vocabulary;
- the matching opaque refs (`linked_browser_handoff_packet_ref` for
  open-in-provider, `linked_publish_later_queue_item_record_id_ref`
  for deferred or captured-offline, `linked_provider_consequence_
  preview_record_id_ref` for publish-now and deferred,
  `linked_account_mapping_binding_record_id_ref` for unknown-actor-
  resolution, `linked_offline_handoff_packet_record_id_ref` for
  captured-offline);
- a typed `linked_artifact_change_class` (with bound /
  unbound ref) for link / unlink / attach / detach / publish-preview
  kinds.

### 3.1 Mutation-mode disclosure rules

The packet's `mutation_mode` MUST be disclosed at the point of intent
on every surface that authors the packet — control label, surface
header, notification line, and export row — not only on the apply
button. The packet's `transition_action_class` per entry MUST agree
with `mutation_mode`:

- `mutation_mode = local_draft` → every entry is
  `save_local_draft_only_no_provider_path` or
  `inspect_only_no_mutation`; provider-bound apply refs (queue item,
  browser-handoff packet, consequence preview) MUST be empty.
- `mutation_mode = publish_now` → every entry is
  `mutate_provider_state_publish_now`; the packet MUST cite a
  `linked_provider_consequence_preview_record_id_ref`.
- `mutation_mode = deferred_publish` → every entry is
  `queue_for_publish_later_deferred` or
  `captured_offline_pending_drain`; the packet MUST cite a
  `linked_publish_later_queue_item_record_id_ref`.
- `mutation_mode = open_in_provider` → every entry is
  `route_through_browser_handoff_open_in_provider`; the packet MUST
  cite a `linked_browser_handoff_packet_ref`.
- `mutation_mode = inspect_only` → every entry is
  `inspect_only_no_mutation` and the packet is a what-if preview
  with no apply path.

The denial reason
`silent_provider_mutation_under_local_draft_label_forbidden` exists
to forbid a surface from claiming `save_local_draft_only_no_provider_
path` while citing any provider-bound apply ref. Silent provider
mutation under a local-draft label is the failure mode this contract
exists to prevent.

### 3.2 Notification truth

Notification fan-out depends on the provider's actual state at apply
time. For queue / browser-handoff routes, the projected
`notification_side_effect_class` is
`notification_unknown_until_publish_admitted_pending_review` —
the surface discloses the unknown lane rather than label it as
"no notification". Local-draft and inspect-only entries MUST resolve
to `no_notification_locally_only`; this is gated through `allOf`.

### 3.3 Permission truth

`permission_unknown_pending_actor_resolution` is admissible only when
the bound `account_mapping_binding_record` is in
`pending_user_selection`, `pending_account_link`, or
`pending_policy_review`. Every other admissibility lane resolves a
typed permission-scope class against the work item's
`write_authority_class`.

### 3.4 Apply path

The apply path is a separate event recorded through the provider-mode
contract:

- `publish_now` apply mints an approval ticket and emits
  `status_transition_packet_applied_publish_now` plus
  `provider_action_published` on the `provider_handoff` audit stream.
- `deferred_publish` apply mints (or updates) a
  `publish_later_queue_item_record` and emits
  `status_transition_packet_admitted_to_publish_later_queue` plus
  `provider_action_deferred`.
- `open_in_provider` apply hands off to the system browser through
  a `browser_handoff_packet_record` and emits
  `status_transition_packet_routed_to_browser_handoff` plus the
  matching `browser_handoff_callback_validated` /
  `browser_handoff_callback_rejected` on return.
- `local_draft` apply writes a local draft and emits
  `status_transition_packet_applied_local_draft_only`. No provider
  call.
- `captured_offline_pending_drain` apply writes the bound
  `offline_handoff_packet_record` and emits
  `status_transition_packet_captured_offline_pending_drain` plus
  `offline_handoff_packet_captured`.

Revoke before apply emits `status_transition_packet_revoked_before_
apply`. Denial events emit `status_transition_packet_audit_denial_
emitted` with a typed reason from the closed denial vocabulary.

## 4. Offline-handoff packet

An `offline_handoff_packet_record` preserves a work-item update the
user authored when the provider was unreachable, the system browser
was blocked, the workspace was restricted, the freshness floor was
unsatisfied, the policy bundle had rolled, the account mapping was
pending, the step-up authenticator was pending, or the user
explicitly deferred the action into a review packet (a deliberate
"file this for later" choice). The packet binds back to one
`work_item_detail_record_id_ref` and (typically) one
`linked_status_transition_packet_record_id_ref`.

### 4.1 Admission and acceptance truth

The packet names exactly one `handoff_admission_reason_class` from
the twelve-value frozen vocabulary. The two `imported_from_*`
admission reasons mark the packet as **evidence-only with no live
provider path** (e.g. a packet replayed from a support export or an
incident workspace bundle); every other admission reason pins a
typed prerequisite the apply path watches.

The packet names exactly one `handoff_provider_acceptance_class`
from the five-value frozen vocabulary:

| Class                                                 | Meaning                                                                          |
|-------------------------------------------------------|----------------------------------------------------------------------------------|
| `not_submitted_local_capture_only`                    | **Default.** The packet has not been submitted to the provider yet.              |
| `submitted_pending_provider_accept_unverified`        | The packet has been admitted into the publish-later queue but provider acceptance has not been confirmed. |
| `provider_accept_confirmed_publish_later_drained`     | The publish-later drain ran AND a provider-callback envelope confirmed acceptance. |
| `provider_accept_rejected_with_typed_reason`          | The drain ran and the provider rejected with a typed reason.                     |
| `imported_handoff_evidence_only_no_provider_path`     | The packet was reconstructed from an evidence bundle; no live provider path.    |

**The packet MUST NOT silently advance to
`provider_accept_confirmed_publish_later_drained`.** The drain
runs through a fresh approval ticket and a fresh
`provider_callback_envelope_record`, and the offline-handoff packet
flips to confirmed only when:

1. a non-empty `linked_publish_later_queue_item_record_id_ref` is
   present, AND
2. a non-empty `linked_provider_callback_envelope_record_id_refs` is
   present.

The denial reason
`captured_handoff_must_not_advance_to_accepted_without_callback_envelope`
forbids any drain path that flips the packet without a callback envelope.

### 4.2 Export and retry truth

The packet names a non-empty `handoff_export_route_classes` set from
the eight-value frozen vocabulary (local-only, support-bundle, incident-
workspace, object-handoff, companion-browser-handoff, clipboard /
text, CLI export, external-handoff-to-managed-admin-only). The
packet may name multiple export routes; each route binds the matching
ref through an `allOf` gate.

The packet names a non-empty `handoff_retry_route_classes` set from
the ten-value frozen vocabulary. `no_retry_imported_evidence_only` is
admissible only on the two `imported_from_*` admission reasons.

### 4.3 Snapshot truth

The packet snapshots the bound work-item detail row's state rows,
owner / assignee rows, and engineering-artifact relations at capture
time. The drain path revalidates against the current detail row; if
the upstream state has drifted past the snapshot (a remote close, a
remote assignee change, a remote label add), the drain denies with
`captured_state_drift_must_deny_silent_drain` and the packet flips
to `drained_publish_later_rejected_with_typed_reason` or
`revoked_by_user_before_drain` depending on the user's choice.

The packet's `snapshot_engineering_relations` MUST carry one row per
of the five axes (issue-to-branch, linked-review, change-intent,
validation-evidence, publish-preview) so a downstream support /
hosted-review / queue-review / admin-reconciliation surface can
render the relation chip mechanically without re-fetching the
detail row.

### 4.4 Drain-state lifecycle

| Drain state                                          | Meaning                                                                |
|------------------------------------------------------|------------------------------------------------------------------------|
| `captured_pending_drain`                             | Captured. Auto-retry will reopen on prerequisite resolution.           |
| `captured_pending_export_user_initiated`             | Captured. The user must run the export route before drain can proceed. |
| `exported_pending_external_apply`                    | The packet has been exported (e.g. into a support bundle) and the apply is owned externally. |
| `drain_admitted_to_publish_later_queue`              | Admitted into the publish-later queue; awaiting provider call.          |
| `drained_publish_later_completed`                    | Drain completed. Pair with `provider_accept_confirmed_*` or `provider_accept_rejected_*`. |
| `drained_publish_later_rejected_with_typed_reason`   | Drain ran; provider rejected. Pair with `provider_accept_rejected_*`. |
| `revoked_by_user_before_drain`                       | User revoked.                                                           |
| `superseded_by_handoff_packet`                       | A new offline-handoff packet superseded this one.                      |

## 5. Truthful escape hatches

Every degraded example in the fixture corpus exposes at least one
truthful escape hatch. The contract names the four canonical hatches
through the typed vocabularies:

1. **Open externally** — `linked_browser_handoff_packet_ref` on the
   transition packet or the offline-handoff packet routes through a
   browser-handoff packet (ADR-0010); the user can open the work
   item in the system browser even when local write authority is
   blocked.
2. **Copy summary** — every record carries reviewable labels
   (<= 1024 graphemes) and a `summary`; surfaces expose
   `clipboard_or_text_export_user_initiated` from the offline-handoff
   `handoff_export_route_classes` set.
3. **Export packet** — the offline-handoff packet's export-route
   set names support-bundle, incident-workspace, object-handoff,
   companion-browser-handoff, clipboard / text, CLI export, and
   managed-admin external-handoff lanes. Every degraded fixture
   resolves at least one export route or names
   `local_only_no_export_path` honestly.
4. **View sync diagnostics** — the work-item detail audit stream
   (`work_item_detail_freshness_chip_changed`,
   `work_item_detail_authority_class_changed`) plus the publish-later
   `provider_handoff` audit stream let queue-review, support-export,
   and admin-reconciliation consumers render typed sync diagnostics
   instead of generic "something failed" copy.

## 6. Cross-cutting record relations

A work item's life across these three records is:

- the `work_item_detail_record` is the **header**;
- the `status_transition_packet_record` is the **previewed mutation**;
- the `offline_handoff_packet_record` is the **captured-but-not-yet-
  applied mutation under unavailability**.

Apply paths walk the provider-mode contract:

- a `publish_now` packet apply produces a
  `provider_object_relation_record` of class
  `provider_authoritative_object` and updates the detail row's
  authority to `provider_authoritative_synced`.
- a `deferred_publish` packet apply produces a
  `publish_later_queue_item_record` and a
  `provider_object_relation_record` of class `queued_publish`. On
  drain, the queue item's `provider_callback_envelope_record`
  refresh promotes the detail row's authority to
  `provider_authoritative_synced` and the relation to
  `provider_authoritative_object`.
- an `open_in_provider` packet apply produces a
  `browser_handoff_packet_record` and a
  `provider_object_relation_record` of class `browser_handoff`.
- a `local_draft` packet apply leaves authority at
  `local_draft_no_provider_object` with relation `local_draft`.
- a `captured_offline_pending_drain` packet apply produces an
  `offline_handoff_packet_record`. The detail row's authority either
  stays at its prior value (the row is still readable) or flips to
  `imported_handoff_evidence_only` if the row itself was reconstructed
  from a support export.

The frozen relation set (`local_draft`, `queued_publish`,
`browser_handoff`, `provider_authoritative_object`,
`cached_read_only_shadow`) is the publish-later contract's relation
class vocabulary; this contract reuses it through
`linked_provider_object_relation_record_id_ref` rather than minting
a parallel "work item ↔ provider object" relation set.

## 7. Redaction posture (frozen)

Every record (detail, transition, offline-handoff) declares a
`redaction_class` from the ADR-0010 / ADR-0007 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw provider
URLs, raw provider issue bodies, raw comment bodies, raw label values
that disclose customer / tenant identity, raw delegated tokens, raw
branch / commit URLs, raw author identity strings, raw absolute paths,
and raw notification payloads MUST NOT cross this boundary on any
surface regardless of class. Exports, support bundles,
mutation-journal entries, evidence packets, replay captures, and AI
context captures carry opaque refs and structured fields only.

Narrowing is permitted: admin policy MAY remove `publish_now` from a
work-item surface, forbid an actor class on the bound
account-mapping, raise a freshness floor, or pin the row to
`cached_read_only_shadow_inspect_only`. Widening beyond the frozen
rules is forbidden.

## 8. Audit-event reuse

Local-only work-item lifecycle events fire on the `work_items` audit
stream using the closed event ids per record family:

- `work_item_detail_record`:
  `work_item_detail_admitted`,
  `work_item_detail_freshness_chip_changed`,
  `work_item_detail_authority_class_changed`,
  `work_item_detail_engineering_relation_bound`,
  `work_item_detail_engineering_relation_unbound`,
  `work_item_detail_audit_denial_emitted`.
- `status_transition_packet_record`:
  `status_transition_packet_authored`,
  `status_transition_packet_applied_publish_now`,
  `status_transition_packet_applied_local_draft_only`,
  `status_transition_packet_admitted_to_publish_later_queue`,
  `status_transition_packet_routed_to_browser_handoff`,
  `status_transition_packet_captured_offline_pending_drain`,
  `status_transition_packet_revoked_before_apply`,
  `status_transition_packet_audit_denial_emitted`.
- `offline_handoff_packet_record`:
  `offline_handoff_packet_captured`,
  `offline_handoff_packet_exported`,
  `offline_handoff_packet_admitted_to_publish_later_queue`,
  `offline_handoff_packet_drained_publish_later_completed`,
  `offline_handoff_packet_drained_publish_later_rejected`,
  `offline_handoff_packet_revoked_by_user_before_drain`,
  `offline_handoff_packet_superseded_by_handoff_packet`,
  `offline_handoff_packet_audit_denial_emitted`.

Provider-side events (callback validation, queue drain, queue
rejection, handoff revocation, provider-action published / deferred /
denied / rolled back) stay on the ADR-0010 `provider_handoff` audit
stream. This contract introduces no new ids on that stream; the
work-item records are the *payload* those frozen events reference.

## 9. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                                  | Where enforced                                                                                                                                                                                                       |
|-------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Provider outages or blocked write scope still allow truthful local draft or handoff artifacts rather than a dead-end error state.                     | Sections 1.3 / 1.4 (freshness chip, write-authority chip), section 4 (offline-handoff packet), `freshness_class` repair-aware lanes, `write_authority_class` blocked lanes pair with typed `transition_admissibility_class` lanes. |
| Transition fixtures disclose whether the action will mutate provider state or only save a local draft.                                                | Section 3 (transition packet), `mutation_mode` × `transition_action_class` matrix, denial reason `silent_provider_mutation_under_local_draft_label_forbidden`.                                                       |
| The contract exposes at least one truthful escape hatch in every degraded example: open externally, copy summary, export packet, or view sync diagnostics. | Section 5 (escape hatches), offline-handoff `handoff_export_route_classes`, browser-handoff packet refs on the transition packet, audit-event vocabulary on every record family.                                     |
| Users and support packets can tell whether a work-item update is only local, queued, provider-authoritative, or linked to a review/change object.     | Section 1 (`work_item_authority_class`), section 2 (`engineering_artifact_relations`), section 4.1 (`handoff_provider_acceptance_class`), reuse of provider-mode `provider_object_relation_record` relation classes. |
| Status-transition truth survives cross-linking to branches, reviews, or evidence packets without inventing separate traceability identifiers.         | Section 2 (engineering-artifact relation block), reuse of `review_workspace_record_id_ref`, `review_pack_record_id_ref`, `review_evaluation_result_record_id_ref`, `change_object_record_id_ref`, `patch_stack_record_id_ref` by reference. |

## 10. Schema-of-record posture (frozen)

Rust types in the eventual work-items crate are the source of truth.
The JSON Schema exports at
`schemas/work_items/work_item_detail.schema.json`,
`schemas/work_items/status_transition_packet.schema.json`, and
`schemas/work_items/offline_handoff_packet.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new record kind, authority class, freshness class,
state-value-origin class, write-authority class, issue-to-branch
class, linked-review class, change-intent class, validation-evidence
class, publish-preview class, transition-kind class, transition-action
class, notification-side-effect class, permission-scope class,
transition-admissibility class, linked-artifact-change class,
handoff-admission-reason class, handoff-provider-acceptance class,
handoff-export-route class, handoff-retry-route class,
handoff-drain-state class, denial reason, or audit-event id is
additive-minor and bumps the per-record `*_schema_version` const.
Repurposing an existing value is breaking and requires a new
decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture of the upstream contracts the
work-items records cite by reference.
