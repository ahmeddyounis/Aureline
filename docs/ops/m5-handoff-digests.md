# Operator handoff-bundle and shift-digest contract

This document freezes the first real Aureline operator **continuity packets**: the
handoff bundles and daily/shift digests an operator hands the next operator, a
client, or another role so operational meaning survives the handoff instead of
being reconstructed from screenshots and tribal knowledge. A continuity packet is
not a generic "attached info" blob — it preserves, outside the live session, the
same object identity, grouping, freshness, ownership, redaction, unresolved
questions, and live-versus-snapshot truth the operator saw. Every packet binds the
`handoff_bundle` or `shift_digest` family from the
[operator-surface matrix](./m5-operator-surfaces.md) and addresses the same
canonical incident, support, admin, release, service, and review objects the detail
surfaces own.

The whole point of this lane is one guardrail: **live, cached, mirrored, and
snapshot evidence stay distinct — never flattened into one blob.** This contract
pins what every packet must hold:

1. **The storage / freshness / boundary distinction is part of the contract.**
   Every evidence item carries a `storage_class` (`live_link`, `cached`,
   `mirrored`, `snapshot`) and a `freshness`. `is_live` and `can_refresh` are
   *computed* from the storage class — only a `live_link` is live, and a `snapshot`
   can never refresh. The roll-up counts each class separately, so a snapshot can
   never read as a live link.
2. **Reopen-safe continuity.** Every group and packet carries a `reopen_anchor`
   whose `anchor_class` is one of `live_object`, `cached_object_snapshot`,
   `mirrored_offline_view`, or `truthful_placeholder`. `resolves_object` is computed
   and true for everything but a truthful placeholder; a placeholder names what the
   object was. The closed set has no "generic dashboard" variant — reopening always
   lands on the canonical object or a truthful placeholder, never an unscoped home
   screen.
3. **Digests group by object and severity before chronology.** A digest's
   `object_groups` are ordered by `severity` (most severe first); each group keeps
   its `latest_update_at` and major `blocker`, and only *within* a group are
   `events` ordered chronologically, so the next operator resumes from the worst,
   freshest thing first.
4. **Unresolved questions travel with the work.** Each `unresolved_question` names
   the open `question`, its `status` (`open`, `investigating`, `blocked`,
   `needs_decision`), the `owner`, the canonical `linked_object_ref`, and the
   `next_safe_action` — with a `blocker_reason` when blocked.
5. **Explicit scope and boundary truth before share/export.** Every packet names a
   `scope` and a `share_posture` (`private`, `workspace_shared`, `org_shared`) and
   an `export_gate` stating exactly what crosses the boundary on share/export at
   that scope, requiring an acknowledgement above private scope.
6. **Frozen, lossless export.** A packet freezes a `snapshot_only` `export` that
   preserves every truth field — including each evidence item's storage class and
   freshness — plus a computed `roll_up` that answers what changed, what remains
   unresolved, and the next safe action in separate sentences.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/ops/m5-handoff-digests.schema.json`](../../schemas/ops/m5-handoff-digests.schema.json)
  — boundary schema for `m5_handoff_digest_set`.
- [`/fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json`](../../fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json)
  — the published canonical packet set; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/ops/m5-handoff-digests.md`](../../artifacts/ops/m5-handoff-digests.md)
  — the human-readable companion (packet, group, evidence, and invariant tables).
- `crates/aureline-support/src/m5_handoff_digests/` — the builder, the
  storage-class distinction, the reopen-safe-continuity rule, roll-up computation,
  the frozen export, validation, and the human-readable projection.
- `cargo run -p aureline-support --example dump_m5_handoff_digests` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Packets

Each packet binds its surface family (handoff bundle or shift digest) by the
matrix's own surface id, so it renders the shared surface contract rather than a
per-surface clone.

| Packet token | Kind | Target / audience | Scope / share posture | Default redaction |
| --- | --- | --- | --- | --- |
| `outgoing_shift_handoff` | handoff_bundle | incoming_on_call / next_operator_shift | shared_team / workspace_shared | operator_only_restricted |
| `client_status_handoff` | handoff_bundle | customer_success_lead / client_facing | shared_team / workspace_shared | metadata_safe_default |
| `daily_operations_digest` | shift_digest | operations_lead / team_wide | managed_org / org_shared | internal_support_restricted |
| `night_shift_digest` | shift_digest | night_on_call / next_operator_shift | local_private / private | private_triage_only |

Each packet carries: a stable `packet_id` (`continuity_packet.<token>`), its
`coverage_window`, an `owning_role` and `decision_right` and the `target_role` it is
handed to, the consumers that render it, its packet-level `reopen_anchor`, its
`export_gate`, its actions, its severity-ordered `object_groups`, its
`unresolved_questions`, a computed `roll_up`, and a frozen `snapshot_only` `export`.

## Object groups, evidence, and the storage distinction

An object group is one canonical object in the packet. Required fields: the stable
`group_id`; the canonical `object_ref` and `object_kind`; the `object_label`; the
`severity` (the most severe of the group's events); the `blocker` and
`blocker_reason` (the reason required when blocked or waived); the `freshness`; the
`latest_update_at` (the newest event time); the `what_changed` sentence; the
`reopen_anchor`; the pinned `evidence`; and the chronological `events`.

Each evidence item keeps its **storage class** and **freshness** distinct:

- `storage_class` is `live_link` (reopening resolves current truth), `cached`
  (a copy captured at handoff time, refreshable when reconnected), `mirrored` (a
  last-synced offline mirror), or `snapshot` (a frozen, immutable copy);
- `is_live` is computed and true **only** for a `live_link`;
- `can_refresh` is computed and true for everything but a `snapshot`.

The roll-up's `live_link_count`, `cached_count`, `mirrored_count`, and
`snapshot_count` are reported separately — the contract never collapses them into
one bucket.

## The reopen-safe-continuity rule

`resolves_object` is computed by `compute_resolves_object`:

- it is **true** for `live_object`, `cached_object_snapshot`, and
  `mirrored_offline_view` (the anchor names a canonical `target_ref`);
- it is **false** only for `truthful_placeholder` (the anchor carries no
  `target_ref` and a `placeholder_label` naming what the object was).

There is no "generic dashboard" anchor class. The fixture deliberately includes an
archived release gate (`client_status_handoff`) whose object no longer resolves, so
its anchor is a `truthful_placeholder` that names the archived gate rather than
dropping the next operator on an unscoped home screen.

## Digests group by object and severity before chronology

For a digest (`daily_operations_digest`, `night_shift_digest`), `object_groups` are
ordered by `severity` (Sev1 first), each group keeps its `latest_update_at` and
major `blocker`, and `events` are ordered chronologically only **within** a group.
The same ordering holds for the handoff bundles. The daily digest leads with the
open Sev1 auth incident, then the held Sev2 access review, then the recovered Sev3
service — so the next operator reads the worst, freshest thing first.

## Unresolved questions

Each question names the open `question`, its `status`, the `owner`, the canonical
`linked_object_ref`, and the `next_safe_action`. A `blocked` question carries a
`blocker_reason`. The questions answer "what remains unresolved" and "what is the
next safe action" so the handoff is actionable, not just informational.

## Scope, share posture, and the export gate

A packet can be private, workspace-shared, or org-shared. The `share_posture` maps
one-to-one onto the governance `scope` (`private` ↔ `local_private`,
`workspace_shared` ↔ `shared_team`, `org_shared` ↔ `managed_org`). The `export_gate`
states the explicit boundary truth **before** save/share/export: its `scope`,
`share_posture`, and `redaction_class` agree with the packet; `requires_boundary_ack`
is true for every posture above `private`; and `crosses_on_share` names exactly what
leaves the local boundary (object identity, grouping, severities, latest updates,
blockers, evidence refs with their storage class and freshness, unresolved
questions, and ownership — never raw payloads, credentials, or endpoint URLs). The
private night-shift digest stays on the host until the operator changes its scope.

## Actions

A packet exposes `open_object`, `open_evidence`, `reopen_at_anchor`,
`capture_answer`, `export_snapshot`, and `share_packet`. Each action carries a
computed `local_safe` flag and a `routes_to_canonical_object` flag: every action but
`share_packet` is local-safe, and `open_object` / `reopen_at_anchor` route to the
canonical detail object.

## Roll-up and export

`compute_roll_up` reports per-severity object counts, the four storage-class counts
separately, the per-status unresolved counts, the `latest_update_at`, and three
sentences — `what_changed`, `what_unresolved`, and `next_safe_action` — plus a
`headline` that keeps the storage classes distinct and ends with "never flattened
into one blob." `export_packet` freezes the packet as a `snapshot_only`
`continuity_handoff_export` carrying the exact object groups (storage classes
preserved), unresolved questions, reopen anchor, and roll-up, so the truth survives
outside the live UI and a lossy export fails CI.

## Invariants

The builder computes each invariant's `holds` flag from the built packets, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `continuity.surface_binding` — every packet binds its matrix surface by the
  matrix's own surface id.
- `continuity.both_surfaces_present` — the set proves both the handoff-bundle and
  the shift-digest surfaces.
- `continuity.canonical_object_linkage` — every object, evidence ref, question
  link, and resolvable reopen target is a canonical `aureline://` handle.
- `continuity.storage_class_not_flattened` — all four storage classes are proven and
  the roll-up counts them separately, never flattening them.
- `continuity.evidence_freshness_preserved` — every evidence item carries an origin,
  a captured-at, and live/refresh flags computed from its storage class.
- `continuity.digests_group_by_severity_before_chronology` — every digest orders
  groups by severity and events chronologically within a group.
- `continuity.all_packets_grouped_and_chronological` — handoff bundles too keep
  groups severity-ordered and within-group events chronological.
- `continuity.latest_update_and_blockers_preserved` — every group preserves its
  latest update and blocker reason; the roll-up's latest update is the newest
  group's.
- `continuity.reopen_lands_on_object_or_placeholder` — every reopen anchor resolves
  to a canonical object or a truthful placeholder, never a generic dashboard.
- `continuity.reopen_anchor_classes_distinct` — all four reopen-anchor classes are
  proven.
- `continuity.unresolved_questions_answerable` — every packet carries unresolved
  questions, each naming an owner, a canonical object, and a next safe action.
- `continuity.scope_boundary_truth` — every packet declares a scope and a matching
  export gate naming what crosses and requiring acknowledgement above private scope.
- `continuity.share_postures_distinct` — the set proves a private, a
  workspace-shared, and an org-shared packet.
- `continuity.ownership_present` — every packet names an owning role, a decision
  right, and the target role.
- `continuity.export_parity` — each packet's frozen export equals re-exporting it
  and is `snapshot_only`.
- `continuity.export_preserves_storage_distinction` — each export preserves the
  exact groups (with storage class and freshness), questions, anchor, and roll-up.
- `continuity.roll_up_answers_three_questions` — each roll-up answers what changed,
  what remains unresolved, and the next safe action.
- `continuity.first_real_packets_present` — all four packets are present.
- `continuity.object_kinds_distinct` — all six canonical object kinds are proven.
- `continuity.severities_distinct` — all four severities are proven.
- `continuity.stable_ids_unique` — packet, group, evidence, and question ids are
  unique.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, and short reviewable sentences. `is_support_export_safe()` enforces
that `raw_payload_excluded` is true and every ref is a repo-relative object ref or
`aureline://` handle, so the set is safe to embed in a support export verbatim.

## Composes with

This contract builds on (and does not replace) the
[operator-surface matrix](./m5-operator-surfaces.md), which freezes the surface
families and the shared scope/redaction/ownership/freshness vocabulary, the
[action plans](./m5-action-plans.md), which build the first ordered checklists, and
the [triage inboxes](./m5-triage-inbox.md), which turn many canonical objects into
reason-bearing rows. It reuses the matrix's object-kind, freshness, blocker/waiver,
and share-posture vocabularies rather than restating them, and stays inside
operator handoff/digest continuity for already-claimed surfaces — it does not
broaden into a full organization knowledge-management suite.
