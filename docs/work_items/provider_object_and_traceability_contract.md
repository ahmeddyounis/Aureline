# Provider-object identity, authority, sync-freshness, and engineering traceability contract

This document freezes the provider-owned work-item identity model and
the cross-tool traceability vocabulary that bind a provider-side issue,
task, story, bug, incident, change request, or RFC back to Aureline's
engineering objects (branches, worktrees, review workspaces, review
packs, evaluation results, review anchors, change objects, patch
stacks, validation evidence, QA runs, check-run snapshots, evidence
packets, incident workspace packets, security incident packets, support
bundles, object-handoff packets, offline-handoff packets,
provider-consequence previews, publish-later queue items,
browser-handoff packets, provider-callback envelopes,
status-transition packets, release artifacts, audit-event streams, and
other work items) so that:

- every provider-linked surface renders one consistent
  provider-authority status, one consistent sync-freshness chip,
  one consistent mapping-provenance posture, and one per-field
  truth row that distinguishes provider-authoritative fields from
  locally-proposed overlays from cached read-only shadow from
  imported-handoff snapshots from mapping-conflict unresolved rows;
- every traceability link from a work item to an engineering
  artifact uses one stable, closed `relation_type_class`, one
  typed `authority_marker_class`, one typed `link_origin_class`,
  one typed `link_freshness_class`, and one typed
  `link_lifecycle_state_class` that survives provider outages,
  browser-handoff-blocked workstations, restricted workspace trust,
  freshness drift, account remaps, and intentional offline capture;
- no provider-owned object can silently flatten into a generic
  local task record;
- users and support / export packets can always tell which parts
  of a work item are provider truth versus local proposal or
  cached shadow.

The machine-readable boundaries are:

- [`/schemas/work_items/provider_object.schema.json`](../../schemas/work_items/provider_object.schema.json)
  — `provider_object_record` and `provider_object_audit_event_record`.
- [`/schemas/work_items/traceability_link.schema.json`](../../schemas/work_items/traceability_link.schema.json)
  — `traceability_link_record` and `traceability_link_audit_event_record`.

Worked fixtures (provider issue linked to branch and review;
stale-cached issue continuing locally past the freshness floor;
policy-blocked provider mutation pinned to a cached read-only shadow;
conflicting local-vs-provider field state with a typed mapping-conflict
block; pending-account-mapping resolution; imported-handoff
evidence-only projection with no live provider path; revoked-by-policy
traceability link; superseded traceability link after account remap)
live under
[`/fixtures/work_items/traceability_cases/`](../../fixtures/work_items/traceability_cases/).

This contract **composes with and does not replace** the upstream
contracts it cites:

- [`/docs/work_items/work_item_contract.md`](work_item_contract.md)
  and the schemas
  [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json),
  [`/schemas/work_items/status_transition_packet.schema.json`](../../schemas/work_items/status_transition_packet.schema.json),
  and
  [`/schemas/work_items/offline_handoff_packet.schema.json`](../../schemas/work_items/offline_handoff_packet.schema.json)
  — `work_item_detail_record`, `status_transition_packet_record`,
  and `offline_handoff_packet_record` are the per-row header,
  previewed-mutation, and captured-under-unavailability records
  this contract cites by reference. The detail header's
  `engineering_artifact_relations` block remains the canonical
  five-axis matrix; the traceability-link record extends that
  matrix with stable per-link records that survive across
  outages, exports, and account remaps.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and
  [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
  — `provider_object_relation_record`, `account_mapping_binding_record`,
  `publish_later_queue_item_record`, `provider_consequence_preview_record`,
  `next_safe_action_class`, `drift_state_class`, `relation_class`.
- [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  — `provider_callback_envelope_record`.
- [`/schemas/providers/connected_account_record.schema.json`](../../schemas/providers/connected_account_record.schema.json)
  — `connected_provider_record_id_ref`.
- [`/docs/vcs/review_workspace_contract.md`](../vcs/review_workspace_contract.md),
  [`/docs/vcs/review_pack_contract.md`](../vcs/review_pack_contract.md),
  [`/docs/vcs/change_stack_contract.md`](../vcs/change_stack_contract.md)
  and the schemas under `/schemas/vcs/` — `branch_row`, `worktree_row`,
  `review_workspace_record`, `review_pack_record`,
  `review_evaluation_result_record`, `review_anchor_record`,
  `change_object_record`, `patch_stack_record`.
- [`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
  and the support / incident / evidence schemas — `object_handoff_packet_record`,
  `support_bundle_record`, `incident_workspace_packet_record`,
  `security_incident_packet_record`, `evidence_packet_record`,
  `qa_run_record`, `check_run_imported_snapshot_record`,
  `release_artifact_record`.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — issue-routing matrix; the imported-evidence link origins compose
  with the route lane vocabulary.
- ADR-0001 / ADR-0007 / ADR-0010 / ADR-0011 / ADR-0018 — workspace
  trust, secret-broker handle / raw-secret-forbidden boundary,
  browser-handoff and approval-ticket envelope, capability lifecycle
  / freshness / client-scope / redaction, and workspace-trust state.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — work-item
  detail, provider-mode, provider-object, and traceability passages.
- `.t2/docs/Aureline_Technical_Design_Document.md` — provider object
  model, mapping-conflict resolution, traceability-link object model.
- `.t2/docs/Aureline_PRD.md` — work-item MUST/SHOULD language for
  honest provider authority and durable engineering traceability.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Without one frozen contract: the issue / planning surface invents a
"provider issue" shape with mixed fields where the user cannot tell
which field came from the provider and which is a local edit; the
review workspace invents a parallel "linked issue" shape that
flattens provider authority into a single boolean; the change-stack
panel invents a third "issue ref" shape; the support-handoff packet
invents a fourth; and a single field rename that collides with a
provider-side rename silently overwrites the user's draft (or
silently overwrites the provider's truth) because no surface
disclosed which side was authoritative.

The five-axis `engineering_artifact_relations` matrix on the
work-item detail header is the right level of granularity for the
header chip, but it is single-valued per axis: a work item that
links to two branches, three review packs, two evidence packets,
and one incident workspace packet cannot be represented by the
header alone. And the header carries no per-field origin truth:
when the user edits the title locally while the provider has
already renamed it remotely, the header's `title_label` cannot
disclose that both sides changed.

This contract closes that gap with **one provider-object projection**
that pins per-field truth (provider-authoritative versus locally
proposed versus cached shadow versus imported evidence versus
mapping conflict), and **one traceability-link record** per
engineering artifact a work item references with a typed
`relation_type_class`, `authority_marker_class`, `link_origin_class`,
`link_freshness_class`, `link_lifecycle_state_class`, and
`directionality_class` that survives outages, exports, and remaps.

## Scope

Frozen at this revision:

- one `provider_object_record` per provider-owned (or local-shadow-
  draft / cached-read-only-shadow / imported-evidence /
  mapping-conflict / mapping-pending) projection — with the
  connected provider, the canonical provider-side identity, the
  typed `provider_authority_status_class`, the typed
  `sync_freshness_class`, the typed `mapping_provenance_class`,
  the per-field `provider_field_rows` array with typed
  `field_origin_class` per row, the typed
  `next_safe_action_class`, the optional typed
  `mapping_conflict_block`, and lineage refs back to the
  bound `provider_object_relation_record`,
  `account_mapping_binding_record`, `provider_callback_envelope_record`s,
  imported-evidence carriers, and the
  `work_item_detail_record`s that read the projection;
- one `traceability_link_record` per work-item-to-artifact link —
  with the source `work_item_detail_record_id_ref`, the optional
  `linked_provider_object_record_id_ref`, the typed
  `relation_type_class`, `authority_marker_class`,
  `link_origin_class`, `link_freshness_class`,
  `link_lifecycle_state_class`, `directionality_class`,
  optional `blocking_relationship_class` for
  work-item-to-other-work-item links, the typed
  `target_artifact_descriptor`, the typed
  `next_safe_action_class`, and lineage refs back to the
  bound `provider_callback_envelope_record`,
  `account_mapping_binding_record`, `browser_handoff_packet_record`,
  imported-evidence carriers, the superseding /
  superseded link records, and the revocation timestamp /
  reviewable summary on revoked links;
- one closed audit-event vocabulary per record family;
- one closed denial-reason vocabulary per record family.

## Out of scope

- Implementing issue-provider adapters (GitHub Issues, Linear, Jira,
  Azure Boards, Asana, Pivotal, Shortcut, on-prem trackers). This
  document freezes the contract those adapters will satisfy.
- Sync-engine implementation (callback ingestion, drift detection,
  conflict-resolution UX). The contract is the vocabulary those
  services read and write.
- Review-UI implementation. The traceability link is read by
  whatever review UI ships.
- Cross-tracker mirroring, dedup heuristics, or cross-provider
  identity reconciliation.
- Final user-facing copy. The schemas freeze the typed vocabulary;
  copy lives in the design system.

## 1. Provider-object projection

Every provider-side object Aureline shows resolves through one
`provider_object_record`. The projection carries:

- a `provider_authority_status_class` from the seven-value frozen
  vocabulary (provider-authoritative-synced,
  provider-authoritative-stale-local-continues,
  local-shadow-draft-no-provider-object,
  cached-read-only-shadow-inspect-only,
  imported-evidence-only-no-live-provider-path,
  mapping-conflict-unresolved, mapping-pending-user-resolution);
- a `target_object_identity` block (provider-side id, opaque
  provider-host handle, opaque tenant / org scope, `object_class`
  re-exported verbatim from the work-item detail vocabulary);
- a `sync_freshness_class` re-exported verbatim from the work-item
  detail vocabulary (live-authoritative-fresh, warm-within-grace,
  degraded-beyond-grace-local-continues,
  unverifiable-provider-unreachable,
  imported-snapshot-no-refresh-path, local-draft-never-published);
- a `mapping_provenance_class` from the ten-value frozen vocabulary
  that names where the local <-> provider mapping came from
  (provider-callback-envelope-validated,
  account-mapping-binding-resolved,
  account-mapping-binding-pending,
  user-authored-local-draft-no-provider-object, four imported_from_*
  lanes, cached-read-only-shadow-no-refresh,
  mapping-conflict-unresolved-local-vs-provider);
- a non-empty `provider_field_rows` array of typed per-field truth
  rows;
- a `next_safe_action_class` (re-exported and extended from the
  publish-later contract) — the only action the surface should
  offer the user;
- an optional `mapping_conflict_block` (required when authority is
  mapping-conflict-unresolved);
- the lineage refs (`linked_provider_object_relation_record_id_ref`,
  `linked_account_mapping_binding_record_id_ref`,
  `linked_provider_callback_envelope_record_id_refs`,
  `linked_offline_handoff_packet_record_id_ref`,
  `linked_object_handoff_packet_record_id_ref`,
  `linked_support_bundle_record_id_ref`,
  `linked_incident_workspace_packet_record_id_ref`);
- the back-references (`connected_work_item_detail_record_id_refs`,
  `supersedes_provider_object_record_id_ref`);
- the `origin_disclosure`, `policy_context`, `redaction_class`,
  and `captured_at` blocks shared with the work-item detail and
  status-transition records.

The projection never carries raw provider URLs, raw provider issue
bodies, raw comment bodies, raw label values that disclose
customer / tenant identity, raw delegated tokens, raw branch /
commit URLs, raw author identity strings, raw absolute paths, or
raw notification payloads. All identity crosses the boundary as
opaque refs and reviewable labels (<= 1024 graphemes).

### 1.1 Per-field truth rows

Every row in `provider_field_rows` names:

- a `field_class` from the eighteen-value frozen vocabulary
  (title, body summary, lifecycle state, assignee, labels,
  milestone or iteration, priority or severity, blocking
  relationship, linked branch or worktree, linked review,
  linked change or patch stack, linked validation evidence,
  publish preview, linked incident or support, watcher or
  subscriber, trust or redaction, freshness, other);
- a `field_origin_class` from the seven-value frozen vocabulary
  (provider-authoritative-field,
  locally-proposed-overlay-pending-publish,
  cached-read-only-shadow-field,
  imported-handoff-field-no-refresh,
  mapping-conflict-unresolved-field,
  derived-from-linked-review-field,
  derived-from-linked-change-field);
- a `field_value_label` (reviewable label) the surface renders;
- an optional `provider_authoritative_value_label` (required on
  mapping-conflict-unresolved-field rows so the user can see the
  provider-side value);
- an optional `local_overlay_value_label` (required on
  locally-proposed-overlay-pending-publish and
  mapping-conflict-unresolved-field rows so the user can see the
  proposed value that has not yet been published);
- optional `source_schema_ref` / `source_field` / `source_ref`
  for derived-from-linked-review-field and
  derived-from-linked-change-field rows so a downstream surface
  can chase the upstream record without guessing.

The two `allOf` gates on the row pin those required pairings:

- `locally_proposed_overlay_pending_publish` requires a non-empty
  `local_overlay_value_label`.
- `mapping_conflict_unresolved_field` requires both a non-empty
  `provider_authoritative_value_label` AND a non-empty
  `local_overlay_value_label`.

The same `field_class` MAY appear at most once per record. A
projection that needs multiple rows for the same field family
(e.g. multiple labels) renders them as one row whose
`field_value_label` carries a reviewable list summary; the design
system owns the rendering.

### 1.2 Mapping-conflict block

When `provider_authority_status_class` is
`mapping_conflict_unresolved`, the projection MUST cite a
`mapping_conflict_block` that names:

- a `conflict_origin_class` from the nine-value frozen vocabulary
  (account-remap, provider-object-split, provider-object-merge,
  three freshness-drift lanes — remote, local, both —
  actor-scope-changed, policy-epoch-rolled,
  imported-handoff-replay);
- a non-empty `conflict_field_classes` set (every entry MUST also
  appear as a `mapping_conflict_unresolved_field` row in
  `provider_field_rows`);
- a `next_safe_action_class` of `resolve_mapping_conflict`,
  `refresh_before_edit`, `escalate_for_admin_review`,
  `reselect_account`, or `open_in_provider_via_browser_handoff`;
- optional refs to the bound callback envelope, account-mapping
  binding, and / or browser-handoff packet that the user can use
  to resolve the conflict.

The projection's top-level `next_safe_action_class` MUST be
`resolve_mapping_conflict` whenever the authority status is
`mapping_conflict_unresolved`; the projection's top-level
`next_safe_action_class` MUST be `no_action_imported_evidence_only`
whenever the authority status is
`imported_evidence_only_no_live_provider_path`.

### 1.3 Authority-status invariants

The schema's `allOf` gates pin the following invariants:

- `provider_authoritative_synced`,
  `provider_authoritative_stale_local_continues`, and
  `cached_read_only_shadow_inspect_only` MUST cite a non-empty
  `linked_provider_object_relation_record_id_ref`.
- `mapping_pending_user_resolution` MUST cite a non-empty
  `linked_account_mapping_binding_record_id_ref` AND resolve
  `mapping_provenance_class` to `account_mapping_binding_pending`.
- `mapping_conflict_unresolved` MUST cite a non-null
  `mapping_conflict_block` AND resolve `mapping_provenance_class`
  to `mapping_conflict_unresolved_local_vs_provider`.
- `imported_evidence_only_no_live_provider_path` MUST resolve
  `sync_freshness_class` to `imported_snapshot_no_refresh_path`
  AND `mapping_provenance_class` to one of the four `imported_from_*`
  lanes AND `next_safe_action_class` to
  `no_action_imported_evidence_only`.
- `local_shadow_draft_no_provider_object` MUST resolve
  `sync_freshness_class` to `local_draft_never_published` or
  `warm_within_grace` AND `mapping_provenance_class` to
  `user_authored_local_draft_no_provider_object`.
- `cached_read_only_shadow_inspect_only` and
  `imported_evidence_only_no_live_provider_path` MUST forbid
  `locally_proposed_overlay_pending_publish` field rows;
  read-only and imported projections MUST NOT carry a live
  local overlay.
- Every `imported_from_*` provenance MUST cite a non-empty
  carrier ref (`linked_offline_handoff_packet_record_id_ref`,
  `linked_object_handoff_packet_record_id_ref`,
  `linked_support_bundle_record_id_ref`, or
  `linked_incident_workspace_packet_record_id_ref`).

The denial reason
`silent_promotion_to_provider_authoritative_synced_forbidden`
exists to forbid a surface from quietly upgrading a stale,
shadow, imported, or pending projection to
`provider_authoritative_synced` without observing a fresh
`provider_callback_envelope_record`. Silent promotion is the
failure mode this contract exists to prevent.

## 2. Traceability-link record

A `traceability_link_record` pins one stable, typed link from a
source work-item detail row to one engineering artifact. The link
carries:

- a `source_work_item_detail_record_id_ref` (required) — the
  bound work item;
- an optional `linked_provider_object_record_id_ref` — the bound
  provider-object projection (when the work item has one);
- a `relation_type_class` from the twenty-three-value frozen
  vocabulary (work-item-to-branch-or-worktree, review-workspace,
  review-pack-evaluation, review-anchor-comment, change-object,
  patch-stack, validation-evidence, qa-run,
  check-run-imported-snapshot, evidence-packet,
  incident-workspace-packet, security-incident-packet,
  support-bundle, object-handoff-packet, offline-handoff-packet,
  provider-consequence-preview, publish-later-queue-item,
  browser-handoff-packet, provider-callback-envelope,
  status-transition-packet, release-artifact, audit-event-stream,
  other-work-item);
- an `authority_marker_class` from the ten-value frozen vocabulary
  (provider-authoritative-link-validated,
  local-authoritative-link-no-provider-overlay,
  provider-overlay-with-local-overlay-synced,
  cached-read-only-shadow-link,
  imported-handoff-evidence-only-link,
  link-pending-provider-validation,
  link-pending-account-mapping-resolution,
  link-revoked-by-user, link-revoked-by-policy,
  link-provider-unreachable-local-continues);
- a `link_origin_class` from the twelve-value frozen vocabulary
  (provider-callback-envelope-minted, local-user-minted,
  four imported_from_* lanes, three derived_from_* lanes,
  account-mapping-binding-minted, policy-admin-minted,
  ai-proposed-link-pending-user-confirmation);
- a `link_freshness_class` from the six-value frozen vocabulary
  (live-authoritative-fresh, warm-within-grace,
  degraded-beyond-grace-local-continues,
  unverifiable-provider-unreachable,
  imported-snapshot-no-refresh-path, local-only-no-provider-path);
- a `link_lifecycle_state_class` from the six-value frozen
  vocabulary (proposed, active, superseded, revoked-by-user,
  revoked-by-policy, archived-after-close);
- a `directionality_class` from the three-value frozen vocabulary
  (outbound-from-work-item, inbound-to-work-item,
  bidirectional-pair);
- an optional `blocking_relationship_class` from the ten-value
  frozen vocabulary (no-blocking, source-blocks-target,
  target-blocks-source, mutual-blocking, source-depends-on-target,
  target-depends-on-source, source-parent-of-target,
  source-child-of-target, source-duplicates-target,
  target-duplicates-source) — required only on
  work-item-to-other-work-item links, forbidden otherwise;
- a typed `target_artifact_descriptor` (artifact-class,
  artifact-record-id-ref, artifact-label,
  artifact-origin-schema-ref);
- a `next_safe_action_class` from the twelve-value frozen
  vocabulary;
- the lineage refs (`linked_provider_callback_envelope_record_id_ref`,
  `linked_account_mapping_binding_record_id_ref`,
  `linked_browser_handoff_packet_ref`,
  `linked_offline_handoff_packet_record_id_ref`,
  `linked_object_handoff_packet_record_id_ref`,
  `linked_support_bundle_record_id_ref`,
  `linked_incident_workspace_packet_record_id_ref`);
- the supersession refs (`superseded_by_traceability_link_record_id_ref`,
  `supersedes_traceability_link_record_id_ref`);
- the revocation block (`revoked_at`, `revocation_reason_summary`)
  required on revoked links;
- the `origin_disclosure`, `policy_context`, `redaction_class`,
  and `summary` blocks.

### 2.1 Relation-type / target-artifact-class invariants

Every `relation_type_class` is paired through `allOf` gates with a
matching `target_artifact_class` so a downstream surface always
knows which schema owns the artifact:

| `relation_type_class`                              | Required `target_artifact_class`                                                  |
|----------------------------------------------------|-----------------------------------------------------------------------------------|
| `work_item_to_branch_or_worktree`                  | `branch_row` or `worktree_row`                                                    |
| `work_item_to_review_workspace`                    | `review_workspace_record`                                                         |
| `work_item_to_review_pack_evaluation`              | `review_pack_record` or `review_evaluation_result_record`                         |
| `work_item_to_review_anchor_comment`               | `review_anchor_record`                                                            |
| `work_item_to_change_object`                       | `change_object_record`                                                            |
| `work_item_to_patch_stack`                         | `patch_stack_record`                                                              |
| `work_item_to_validation_evidence`                 | `review_evaluation_result_record`, `qa_run_record`, `check_run_imported_snapshot_record`, or `evidence_packet_record` |
| `work_item_to_qa_run`                              | `qa_run_record`                                                                   |
| `work_item_to_check_run_imported_snapshot`         | `check_run_imported_snapshot_record`                                              |
| `work_item_to_evidence_packet`                     | `evidence_packet_record`                                                          |
| `work_item_to_incident_workspace_packet`           | `incident_workspace_packet_record`                                                |
| `work_item_to_security_incident_packet`            | `security_incident_packet_record`                                                 |
| `work_item_to_support_bundle`                      | `support_bundle_record`                                                           |
| `work_item_to_object_handoff_packet`               | `object_handoff_packet_record`                                                    |
| `work_item_to_offline_handoff_packet`              | `offline_handoff_packet_record`                                                   |
| `work_item_to_provider_consequence_preview`        | `provider_consequence_preview_record`                                             |
| `work_item_to_publish_later_queue_item`            | `publish_later_queue_item_record`                                                 |
| `work_item_to_browser_handoff_packet`              | `browser_handoff_packet_record`                                                   |
| `work_item_to_provider_callback_envelope`          | `provider_callback_envelope_record`                                               |
| `work_item_to_status_transition_packet`            | `status_transition_packet_record`                                                 |
| `work_item_to_release_artifact`                    | `release_artifact_record`                                                         |
| `work_item_to_audit_event_stream`                  | `audit_event_stream`                                                              |
| `work_item_to_other_work_item`                     | `work_item_detail_record` (and `blocking_relationship_class` is REQUIRED)         |

### 2.2 Authority-marker invariants

The schema's `allOf` gates pin the following invariants:

- `provider_authoritative_link_validated` MUST resolve
  `link_origin_class` to `provider_callback_envelope_minted_link`
  or `account_mapping_binding_minted_link` AND `link_freshness_class`
  to `live_authoritative_fresh` or `warm_within_grace`.
- `imported_handoff_evidence_only_link` MUST resolve
  `link_origin_class` to one of the four `imported_from_*` lanes
  AND `link_freshness_class` to `imported_snapshot_no_refresh_path`.
- `link_pending_provider_validation` MUST resolve
  `link_lifecycle_state_class` to `proposed`.
- `link_pending_account_mapping_resolution` MUST cite a non-empty
  `linked_account_mapping_binding_record_id_ref`.
- `link_revoked_by_user` and `link_revoked_by_policy` MUST cite
  `revoked_at` AND `revocation_reason_summary` AND resolve
  `link_lifecycle_state_class` to the matching `revoked_by_*`
  value.
- `superseded` lifecycle MUST cite a non-empty
  `superseded_by_traceability_link_record_id_ref`.
- `archived_after_close` lifecycle MUST resolve
  `next_safe_action_class` to `no_action_archived_after_close`.
- `local_only_no_provider_path` freshness is admissible only on
  `local_authoritative_link_no_provider_overlay` authority.
- Every `imported_from_*` link origin MUST cite the matching
  carrier ref (offline handoff, object handoff, support bundle,
  or incident workspace).

The denial reason
`silent_promotion_to_provider_authoritative_link_validated_forbidden`
exists to forbid a surface from quietly upgrading a pending,
shadow, imported, or revoked link to
`provider_authoritative_link_validated` without observing a fresh
`provider_callback_envelope_record` or
`account_mapping_binding_record`. Silent promotion is the failure
mode this contract exists to prevent.

### 2.3 Composition with the work-item detail header

The work-item detail header's `engineering_artifact_relations`
block remains the canonical five-axis matrix for the *header
chip*. Every detail row carries the matrix for downstream surfaces
to render mechanically.

The traceability-link record extends the matrix in three ways:

1. It supports **multiple links per axis**. A work item may
   reference two branches, three review packs, and two evidence
   packets; each link is its own `traceability_link_record`.
2. It supports **axes the header does not encode**, including
   incident workspace packets, security incident packets, support
   bundles, object-handoff packets, offline-handoff packets,
   provider-consequence previews, publish-later queue items,
   browser-handoff packets, provider-callback envelopes,
   status-transition packets, release artifacts, audit-event
   streams, and other work items (blocking / dependency / parent /
   duplicate).
3. It supports **per-link authority and freshness**. The header
   chip is single-valued per axis; the link record carries one
   typed `authority_marker_class` and one typed
   `link_freshness_class` per link, so a row whose linked review
   workspace was provider-validated and whose linked evidence
   packet was imported from a support export can render both
   honestly side by side.

A surface that needs both views (the five-axis chip and the
per-link records) reads the header for the chip and the
traceability-link records for the per-link list. No surface mints a
parallel "multi-link" or "extended-axis" shape.

## 3. Truthful escape hatches

Every degraded example in the fixture corpus exposes at least one
truthful escape hatch:

1. **Open externally** — `linked_browser_handoff_packet_ref` on
   the traceability-link record and the mapping-conflict block
   route through a browser-handoff packet (ADR-0010); the user
   can open the artifact in the system browser even when local
   write authority is blocked.
2. **Copy summary** — every record carries reviewable labels
   (<= 1024 graphemes) and a `summary`; surfaces expose
   `clipboard_or_text_export_user_initiated` from the
   offline-handoff `handoff_export_route_classes` set when an
   offline-handoff packet is present.
3. **Export packet** — imported-evidence projections and links
   cite their carriers (`linked_support_bundle_record_id_ref`,
   `linked_incident_workspace_packet_record_id_ref`,
   `linked_object_handoff_packet_record_id_ref`,
   `linked_offline_handoff_packet_record_id_ref`) so the user
   always has a routable carrier rather than a dead-end "no
   provider" copy.
4. **View sync diagnostics** — both record families carry an
   audit-event vocabulary (`provider_object_*`,
   `traceability_link_*`) that lets queue-review, support-export,
   and admin-reconciliation consumers render typed sync
   diagnostics rather than generic "something failed" copy.
5. **Resolve mapping conflict** — `mapping_conflict_block` on
   the provider-object record names a typed
   `next_safe_action_class` (resolve-mapping-conflict,
   refresh-before-edit, escalate-for-admin-review,
   reselect-account, open-in-provider-via-browser-handoff) so
   the surface always offers a typed next action rather than
   silently picking one side.

## 4. Cross-cutting record relations

A provider-owned work item's life across all work-items records is:

- the `work_item_detail_record` is the **per-row header**;
- the `provider_object_record` is the **provider-side projection**
  (per-field truth, authority status, mapping provenance);
- the `traceability_link_record` is the **per-link record**
  (one per work-item-to-artifact link);
- the `status_transition_packet_record` is the **previewed mutation**
  (one per intent);
- the `offline_handoff_packet_record` is the **captured-but-not-
  yet-applied mutation** (one per offline capture).

Apply paths still walk the provider-mode contract (publish-now
ticket, deferred-publish queue item, open-in-provider browser
handoff, local-draft commit, captured-offline pending drain). The
provider-object projection is updated by reference when a
provider-callback envelope arrives; the traceability-link record
is updated by reference when a link is validated, revoked, or
superseded.

## 5. Redaction posture (frozen)

Every record (provider-object projection, traceability link)
declares a `redaction_class` from the ADR-0010 / ADR-0007 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw
provider URLs, raw provider issue bodies, raw comment bodies, raw
label values that disclose customer / tenant identity, raw
delegated tokens, raw branch / commit URLs, raw author identity
strings, raw absolute paths, and raw notification payloads MUST
NOT cross this boundary on any surface regardless of class.
Exports, support bundles, mutation-journal entries, evidence
packets, replay captures, and AI context captures carry opaque
refs and structured fields only.

Narrowing is permitted: admin policy MAY remove
`provider_authoritative_link_validated` from a surface, forbid an
actor class on the bound account-mapping, raise a freshness floor,
or pin the projection / link to `cached_read_only_shadow_inspect_only`
/ `cached_read_only_shadow_link`. Widening beyond the frozen rules
is forbidden.

## 6. Audit-event reuse

Local-only provider-object and traceability-link lifecycle events
fire on the `work_items` audit stream using the closed event ids
per record family:

- `provider_object_record`:
  `provider_object_admitted`,
  `provider_object_authority_status_class_changed`,
  `provider_object_sync_freshness_class_changed`,
  `provider_object_field_origin_class_changed`,
  `provider_object_mapping_conflict_detected`,
  `provider_object_mapping_conflict_resolved`,
  `provider_object_callback_envelope_observed`,
  `provider_object_account_mapping_binding_changed`,
  `provider_object_superseded_by_provider_object_record`,
  `provider_object_audit_denial_emitted`.
- `traceability_link_record`:
  `traceability_link_authored`,
  `traceability_link_authority_marker_changed`,
  `traceability_link_freshness_class_changed`,
  `traceability_link_lifecycle_state_changed`,
  `traceability_link_revoked_by_user`,
  `traceability_link_revoked_by_policy`,
  `traceability_link_superseded`,
  `traceability_link_archived_after_close`,
  `traceability_link_callback_envelope_observed`,
  `traceability_link_audit_denial_emitted`.

Provider-side events (callback validation, queue drain, queue
rejection, handoff revocation, provider-action published /
deferred / denied / rolled back) stay on the ADR-0010
`provider_handoff` audit stream. This contract introduces no new
ids on that stream; the provider-object projection and traceability
links are the *payload* those frozen events reference.

## 7. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                  | Where enforced                                                                                                                                                                                                                       |
|---------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Users and support / export packets can tell which parts of a work item are provider truth versus local proposal or cached shadow.    | Section 1 (`provider_authority_status_class`, `mapping_provenance_class`, per-field rows with typed `field_origin_class`), section 1.3 (authority-status invariants), section 5 (redaction posture).                                |
| Traceability to branch / review / evidence uses stable relation types that survive provider outages and offline work.                | Section 2 (`relation_type_class`, `authority_marker_class`, `link_origin_class`, `link_freshness_class`, `link_lifecycle_state_class`), section 2.1 (relation-type / target-artifact-class invariants), section 2.2 (authority-marker invariants). |
| No provider-owned object can silently flatten into a generic local task record.                                                       | Section 1.3 (denial reason `silent_promotion_to_provider_authoritative_synced_forbidden`), section 2.2 (denial reason `silent_promotion_to_provider_authoritative_link_validated_forbidden`), every authority class is closed and gated on lineage refs. |
| Conflicting local-vs-provider field state is named explicitly with both values surfaced.                                              | Section 1.1 (per-field truth row gates), section 1.2 (`mapping_conflict_block`).                                                                                                                                                     |
| Policy-blocked provider mutation does not silently drop or pretend it succeeded.                                                      | `cached_read_only_shadow_inspect_only` authority on the projection, `cached_read_only_shadow_link` and `link_revoked_by_policy` on the link record, denial reasons across both schemas.                                              |
| Stale cached issues continue to render honestly past the freshness floor with a typed escape hatch.                                   | `provider_authoritative_stale_local_continues` authority paired with `degraded_beyond_grace_local_continues` freshness; section 3 escape hatches.                                                                                    |

## 8. Schema-of-record posture (frozen)

Rust types in the eventual work-items crate are the source of
truth. The JSON Schema exports at
`schemas/work_items/provider_object.schema.json` and
`schemas/work_items/traceability_link.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new record kind, provider-authority-status class,
sync-freshness class, mapping-provenance class, provider-field
class, field-origin class, mapping-conflict-origin class,
relation-type class, authority-marker class, link-origin class,
link-freshness class, link-lifecycle-state class, target-artifact
class, directionality class, blocking-relationship class,
next-safe-action class, denial reason, or audit-event id is
additive-minor and bumps the per-record `*_schema_version`
const. Repurposing an existing value is breaking and requires a
new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture of the upstream contracts the
provider-object and traceability-link records cite by reference.
