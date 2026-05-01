# Requirement-status snapshot, blocker-state grammar, and evidence-link summary contract

This contract freezes one shared vocabulary for the requirement-status
snapshot. It exists so reviewers can reason about milestone close per
requirement row — not from a narrative status update, not from a chat
log, not from a screenshot of a scorecard — and so a row that has been
narrowed, waived, deferred, or excluded from scope stays visible next
to the evidence and claim surfaces it influences instead of quietly
disappearing.

The requirement-status snapshot is one object family. Every row cites
the canonical requirement id, the milestone row it closes against, the
typed accountable and evidence ownership, the typed current state, the
typed blocker class describing why a non-pass row is held, the typed
blocker-state grammar (implementation state x verification state x
release-readiness meaning x integrated-versus-complete-for-close), the
linked evidence packets and decision-right cards, the linked known-
limit notes / claim rows / waiver register entries that narrow truth
without disappearing the row, and the last review chronology. The
snapshot also carries summary rollups by requirement family, launch
wedge, protected workflow, and claim family so the ship-room
dashboard, the architecture pack, the release packet, the support
packet, and the governance review flow consume the same source object.

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the projection rules, the export-
parity floor, and the fixture corpus. It does not implement a live
dashboard, a milestone-close UI, or a project-management integration.
External project-management systems are explicitly out of scope.

## Companion artifacts

- [`/schemas/governance/requirement_status_snapshot.schema.json`](../../schemas/governance/requirement_status_snapshot.schema.json)
  — boundary schema for one `requirement_status_snapshot_record`.
- [`/fixtures/governance/requirement_status_cases/`](../../fixtures/governance/requirement_status_cases/)
  — worked records covering a clean milestone close, a row held by a
  failing protected metric, a row riding on an active waiver, a
  narrowed row pinned to a known-limit note and claim row, and a row
  whose decision route is unresolved.
- [`/schemas/governance/requirement_register.schema.json`](../../schemas/governance/requirement_register.schema.json)
  and
  [`/artifacts/governance/requirement_register_seed.yaml`](../../artifacts/governance/requirement_register_seed.yaml)
  — canonical requirement register. Every `requirement_id` on the
  snapshot resolves through the register; aliases and local labels
  remain non-authoritative.
- [`/schemas/governance/milestone_scorecard.schema.json`](../../schemas/governance/milestone_scorecard.schema.json)
  and
  [`/artifacts/governance/milestone_scorecard_template.yaml`](../../artifacts/governance/milestone_scorecard_template.yaml)
  — milestone scorecard. The snapshot's `milestone_row` cites the
  scorecard lane and the snapshot pairs with the scorecard
  one-to-one for milestone close.
- [`/schemas/governance/evidence_packet_header.schema.json`](../../schemas/governance/evidence_packet_header.schema.json)
  and
  [`/artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
  — evidence packet header and stable evidence-id conventions. Every
  `evidence_packet_ref` on the snapshot resolves through the shared
  header.
- [`/schemas/governance/decision_right_card.schema.json`](../../schemas/governance/decision_right_card.schema.json)
  and
  [`/schemas/governance/release_truth_summary.schema.json`](../../schemas/governance/release_truth_summary.schema.json)
  — decision-right card and release-truth summary. The snapshot
  reuses the `degraded_state_label_class`, `decision_route_status_class`,
  and ownership vocabularies so a row's blocker, the gating decision,
  and the live ship-or-no-ship posture render with the same tokens.
- [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — claim manifest. Narrowed rows cite `claim_row` ids verbatim so
  the snapshot, the claim manifest, and the public-truth surface
  agree on the narrowing.
- [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)
  and
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  — waiver register and waiver-expiry queue. Waived rows cite the
  waiver record and inherit the expiry-proximity grammar.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` — requirement-id, signoff, evidence,
  waiver, and milestone-close requirements (RFC 2119 MUST / SHOULD
  language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — milestone
  governance, scorecard cadence, and verification-packet shape.
- `.t2/docs/Aureline_Technical_Design_Document.md` — release-evidence
  and supportability-evidence record shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — decision-visibility,
  release-status, and known-limit / About / Help disclosure rules.

If this contract disagrees with those sources, those sources win and
this contract, the schema, and the fixtures update in the same change.

## 1. Why this contract exists

1. **Narrative status fragments milestone close.** Without one shared
   per-requirement snapshot, the milestone scorecard says "lane:
   yellow", the architecture pack says "renderer viability:
   acceptable", the release packet says "go for foundations close",
   and the support packet quotes nothing at all. The snapshot exists
   so every consuming surface renders the **same** typed state, the
   **same** typed blocker grammar, the **same** linked evidence and
   decision-right cards, and the **same** narrowing posture per
   requirement row.
2. **Narrowed and waived rows must remain visible.** A row that is
   narrowed against a known-limit note, a claim manifest row, a
   waiver register entry, a deferral decision, or an out-of-scope
   decision MUST still appear on the snapshot. Surfaces MUST NOT
   silently drop a row to keep the milestone-close posture clean.
3. **Integrated is not the same as complete for close.** A row whose
   implementation has merged but is gated, flagged, or otherwise
   off the milestone-close path is `integrated_only_not_complete_for_close`,
   not `pass`. The snapshot enforces this: an integrated-only row
   cannot collapse into pass on any consuming surface.
4. **Machine-readable linkage back to evidence and decision IDs.** Every
   row's blocker entries cite a `decision_right_card_ref` and (where
   applicable) a `linked_evidence_ref`. Every row's narrowing entries
   cite the narrowing object (known-limit note, claim row, waiver
   register entry, deferral decision, out-of-scope decision). Every
   row's evidence-link summary cites stable evidence ids resolved
   through `schemas/governance/evidence_packet_header.schema.json`.
5. **Rollups consume the same object.** The dashboard, the
   architecture pack, the release packet, the support packet, and the
   governance review flow read the same `rollups[]` array projected
   over the same rows. A surface that reconstructs a different
   per-family or per-wedge count from raw notes is non-conforming.

## 2. Snapshot shape

A `requirement_status_snapshot_record` carries:

- `snapshot_id` — stable, machine-readable id quoted by every
  consuming surface.
- `evaluated_at` — RFC 3339 UTC timestamp at which the snapshot was
  projected. Distinct from underlying evidence `captured_at` and
  decision-right card `evaluated_at`; the snapshot can be reprojected
  when a row closes, a waiver expires, or evidence ages out.
- `snapshot_target` — typed `target_milestone` (closed
  milestone-slug vocabulary) plus a bounded reviewable
  `snapshot_scope_summary`.
- `milestone_close_posture_class` — typed milestone-close posture
  (§9). A snapshot whose milestone is closed cleanly cannot also
  carry a row in `fail` or `partial`.
- `rows[]` — per-requirement status rows in scope (§3).
- `rollups[]` — typed summary rollups by requirement family, launch
  wedge, protected workflow, and claim family (§8).
- `linked_milestone_scorecard_ref`,
  `linked_architecture_pack_ref`, `linked_release_truth_summary_refs`,
  `linked_support_export_packet_refs`,
  `linked_governance_packet_refs` — opaque refs into the surfaces
  that consume the snapshot.
- `export_fields` — typed booleans for dashboard, milestone
  scorecard, architecture pack, release packet, support export,
  governance packet, claim manifest, and enterprise review (§10).

## 3. Requirement-status row shape

A `requirement_status_row` carries:

- `row_id` — stable, machine-readable row id.
- `requirement_id` — canonical requirement id from
  [`requirement_register_seed.yaml`](../../artifacts/governance/requirement_register_seed.yaml).
- `requirement_class` — closed requirement-class vocabulary mirroring
  the register (FR / LANG / PERF / REL / SEC / A11Y / ECOS / AI / OPS /
  GOV / ARCH / TOOL / CERT / ENT / REPO / COMP).
- `milestone_row` — typed `target_milestone`, `scorecard_lane_ref`,
  `milestone_scorecard_ref`, and `milestone_close_posture_class`.
- `ownership` — typed `primary_dri`, `evidence_owner`, `owning_lane`,
  `decision_forum_ref`, and `decision_route_status_class` (§6).
- `requirement_state_class` — typed current state (§4).
- `blocker_state` — typed `implementation_state_class`,
  `verification_state_class`, `release_readiness_class`, and
  `integrated_versus_complete_class` plus a bounded reviewable
  `blocker_state_summary` (§5).
- `blockers[]` — typed blocker entries (§5). Each entry carries the
  `blocker_class`, the `decision_right_card_ref`, the
  `linked_evidence_ref`, and the bounded `blocker_summary`.
- `narrowing_links[]` — typed narrowing-link entries (§7) into known-
  limit notes, claim rows, waiver register entries, deferral
  decisions, and out-of-scope decisions.
- `evidence_link_summary_entries[]` — typed evidence-link summaries
  (§8) with `evidence_packet_ref`, `result_status`, `freshness_class`,
  `captured_at`, `stale_after`, `expires_at`, and a bounded
  `evidence_link_summary`.
- `linked_decision_right_card_refs`, `linked_release_truth_summary_refs`,
  `linked_known_limit_refs`, `linked_claim_row_refs`,
  `linked_waiver_register_refs` — opaque refs into the gating
  decisions, the release-truth summaries, the known-limit notes, the
  claim rows, and the waiver register entries.
- `last_review` — `last_reviewed_at`, `last_reviewer_role_ref`, and a
  bounded reviewable `review_summary`. Personal handles MUST NOT
  appear; the role registry resolves the reviewer.
- `headline_label` and `row_summary` — bounded reviewable label and
  one-sentence summary.

## 4. Requirement-state vocabulary

Closed seven-class vocabulary:

| Class | Plain-language meaning |
| --- | --- |
| `pass` | The requirement is fully met against the target milestone with fresh, non-stale evidence and no open blockers. |
| `fail` | The requirement is not met against the target milestone; at least one blocker entry is required. |
| `partial` | The requirement is partly met; the blocker-state grammar names `implementation_state` and `verification_state` separately so 'integrated' is not confused with 'complete for close'. |
| `waived` | The requirement is held under an active waiver inside its expiry window; the waiver register entry MUST be linked. |
| `deferred` | The requirement is acknowledged but pushed to a later milestone; the deferral decision and target milestone MUST be named. |
| `narrowed` | The requirement is narrowed against a known-limit note or claim row; the narrowing link MUST be cited. |
| `out_of_scope` | The requirement is explicitly out of scope for the milestone (or for the program); the rationale MUST be cited. |

A row whose state cannot be typed denies with
`requirement_state_class_unresolved` rather than defaulting.

Schema-enforced pairings (snapshot `allOf` block):

- `pass` requires no blocker entries and
  `release_readiness_class = ready_for_close`.
- `fail` requires at least one blocker entry and a non-empty
  `linked_decision_right_card_refs` array.
- `waived` requires at least one `waiver_register_entry` narrowing
  link, `release_readiness_class = ready_under_active_waiver`, and a
  non-empty `linked_waiver_register_refs` array.
- `narrowed` requires at least one `known_limit_note` or `claim_row`
  narrowing link and `release_readiness_class = ready_with_named_narrowing`.
- `deferred` requires a `deferral_decision` narrowing link with a
  non-null `deferral_target_milestone` and
  `release_readiness_class = deferred_to_later_milestone`.
- `out_of_scope` requires an `out_of_scope_decision` narrowing link
  and `release_readiness_class = out_of_scope_by_decision`.

## 5. Blocker-state grammar

Every row carries a `blocker_state` block with four typed axes plus a
`blockers[]` array describing the open or held blockers.

### 5.1 Implementation state

Closed six-class vocabulary:

| Class | Meaning |
| --- | --- |
| `not_started` | No implementation work has landed. |
| `in_progress` | Work is landing but the row is not integrated. |
| `integrated_only` | Code or contract has merged but is gated, flagged, or otherwise not on the milestone-close path. |
| `complete_for_close` | Implementation is on the milestone-close path; surfaces MUST distinguish this from `integrated_only`. |
| `rolled_back` | A previously integrated change was rolled back. |
| `not_required` | The row is contract-only, narrative-only, or otherwise carries no implementation surface (the verification state still applies). |

### 5.2 Verification state

Closed eight-class vocabulary:

| Class | Meaning |
| --- | --- |
| `no_evidence` | No packet has been linked. |
| `evidence_partial` | At least one packet is linked but coverage is partial against the requirement's verification classes. |
| `evidence_passing_fresh` | Passing evidence within the freshness window. |
| `evidence_passing_stale` | Passing evidence aged out against `evidence_packet_header.schema.json#freshness`. |
| `evidence_failing` | At least one linked packet is failing. |
| `evidence_waived_within_expiry` | A waiver inside its expiry window stands in for the evidence. |
| `evidence_waiver_expired` | The waiver lapsed; surfaces MUST NOT render the row as held. The schema forces the row state into `fail`. |
| `evidence_not_required` | Closed-vocabulary administrative classes that do not require packets. |

### 5.3 Release-readiness meaning

Closed eight-class vocabulary:

| Class | Meaning |
| --- | --- |
| `ready_for_close` | The row is on the milestone-close path with fresh passing evidence. |
| `ready_with_named_narrowing` | The row is on the close path under a named narrowing (claim row or known-limit note). |
| `ready_under_active_waiver` | The row is on the close path under an active waiver inside its expiry window. |
| `not_ready_blockers_open` | At least one open blocker is held. |
| `not_ready_waiver_expired` | A waiver lapsed; ship-room cannot collapse this into 'held'. |
| `not_ready_decision_route_unresolved` | No forum is named for the next decision; surfaces MUST render `degraded_decision_route_missing`. |
| `deferred_to_later_milestone` | The row is acknowledged but moved out. |
| `out_of_scope_by_decision` | The row is explicitly excluded. |

### 5.4 Integrated versus complete for close

Closed five-class vocabulary:

| Class | Meaning |
| --- | --- |
| `complete_for_milestone_close` | Implementation is `complete_for_close` and verification is `evidence_passing_fresh` or `evidence_not_required`. |
| `integrated_only_not_complete_for_close` | Implementation is `integrated_only` or `in_progress`. The schema forbids `requirement_state_class = pass` on these rows. |
| `complete_with_named_narrowing` | Pairs with at least one linked claim row or known-limit note. |
| `complete_under_active_waiver` | Pairs with an active waiver register entry. |
| `not_applicable_no_implementation_surface` | Pairs with `implementation_state_class = not_required`. |

### 5.5 Blocker-class vocabulary

Closed sixteen-class blocker vocabulary on `blockers[]`:

| Class | Meaning |
| --- | --- |
| `no_blocker_pass` | Reserved for non-blocker rows; pairs with `requirement_state_class = pass` and an empty `blockers[]` array. |
| `evidence_missing` | No evidence has been linked for a row that requires it. |
| `evidence_stale` | Linked evidence aged out against the freshness window. |
| `evidence_failing` | At least one linked packet is failing. |
| `evidence_partial_coverage` | Coverage is partial against the requirement's verification classes. |
| `implementation_missing` | No implementation exists for a row that requires it. |
| `implementation_partial` | Implementation is partial; the row is `integrated_only` or `in_progress`. |
| `verification_class_unresolved` | The row's verification-class assignment cannot be typed. |
| `decision_route_unresolved` | No forum is named for the next decision. Pairs with `decision_route_status_class = no_forum_named_degraded`. |
| `owner_unresolved` | The accountable owner does not resolve through the ownership matrix. |
| `waiver_active_within_expiry` | An active waiver is holding the row inside its expiry window. Pairs with `requirement_state_class = waived`. |
| `waiver_expired_no_renewal` | The waiver lapsed and was not renewed. The schema forces the row state into `fail`. |
| `narrowed_against_known_limit` | The row is narrowed against a known-limit note. Pairs with `requirement_state_class = narrowed`. |
| `narrowed_against_claim_row` | The row is narrowed against a claim manifest row. Pairs with `requirement_state_class = narrowed`. |
| `deferred_to_later_milestone` | The row is deferred. Pairs with `requirement_state_class = deferred`. |
| `out_of_scope_by_decision` | The row is excluded. Pairs with `requirement_state_class = out_of_scope`. |

A blocker whose class cannot be typed denies with
`blocker_class_unresolved` rather than defaulting.

## 6. Ownership and decision-route status

Closed four-class `decision_route_status_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `named_forum_owner` | The row's gating decision-right card has a named forum and a named accountable owner. |
| `named_forum_owner_unresolved` | A forum is named but the accountable owner does not resolve; surfaces MUST render `degraded_owner_unresolved`. |
| `no_forum_named_degraded` | No forum can be named; surfaces MUST render `degraded_decision_route_missing`. The schema forces `decision_forum_ref = null` and at least one blocker entry of class `decision_route_unresolved`. |
| `not_required_for_class` | The requirement does not require a decision-right card (closed-vocabulary administrative classes). |

`primary_dri`, `evidence_owner`, and `owning_lane` MUST resolve into
[`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
Personal handles, raw email addresses, raw phone numbers, raw chat-
room URLs, raw on-call rotation entries, and raw calendar URLs MUST
NOT appear; the snapshot carries opaque role / lane / forum refs only.

## 7. Narrowing-link grammar

Closed five-class `narrowing_link_kind_class` vocabulary:

| Class | Plain-language meaning |
| --- | --- |
| `known_limit_note` | Link into a known-limit note ref carried on the evidence-packet header (`known_limit_refs`). |
| `claim_row` | Link into a `claim_row` id from `schemas/governance/claim_manifest.schema.json`. |
| `waiver_register_entry` | Link into a waiver record on `artifacts/governance/ownership_matrix.yaml` or `schemas/governance/waiver_expiry.schema.json`. |
| `deferral_decision` | Link into the `decision_register` row that pushed the requirement to a later milestone. The `deferral_target_milestone` MUST be non-null. |
| `out_of_scope_decision` | Link into the `decision_register` row that explicitly excluded the requirement. |

Every narrowing link carries:

- `linked_ref` — opaque ref into the narrowing object.
- `linked_claim_surface_refs` — optional refs into the claim surfaces
  (docs rows, About / Help destinations, public claim manifest rows)
  this narrowing influences. Carried so a narrowed row remains
  visible next to the surfaces it affects.
- `expires_at` — for `waiver_register_entry` and `deferral_decision`
  narrowings.
- `deferral_target_milestone` — for `deferral_decision` narrowings.
- `narrowing_summary` — bounded reviewable summary; raw policy bodies
  and raw waiver justifications do not appear.

A narrowed or waived row remains visible on every consuming surface.
A surface that drops a narrowed row to keep the milestone-close
posture clean is non-conforming.

## 8. Evidence-link summary and rollups

### 8.1 Evidence-link summary

Every row carries an `evidence_link_summary_entries[]` array. Each
entry carries:

- `evidence_packet_ref` — stable evidence packet id resolved through
  `schemas/governance/evidence_packet_header.schema.json`.
- `result_status` — mirrors the evidence-packet header verbatim.
- `freshness_class` — mirrors `schemas/governance/capability_lifecycle.schema.json#freshness_class`
  verbatim.
- `captured_at`, `stale_after`, `expires_at` — copied from the
  evidence-packet header.
- `evidence_link_summary` — bounded reviewable summary; raw
  measurement bytes do not appear.

The array MAY be empty only when `verification_state_class` is in
`{ no_evidence, evidence_not_required }`. A `pass` row with a non-
empty evidence array MUST render at least one entry with
`result_status = pass` and `freshness_class` in `{ live, warm_cached }`.

### 8.2 Rollups

Closed four-axis `rollup_axis_class` vocabulary:

| Axis | Bucket |
| --- | --- |
| `requirement_family` | Grouped by `requirement_class` (FR / LANG / PERF / REL / SEC / A11Y / ECOS / AI / OPS / GOV / ARCH / TOOL / CERT / ENT / REPO / COMP). |
| `launch_wedge` | Grouped by launch-wedge id from `artifacts/milestones/<milestone>_design_evidence_index.yaml`. |
| `protected_workflow` | Grouped by protected-workflow id from `artifacts/governance/protected_change_budget.yaml`. |
| `claim_family` | Grouped by `claim_family` from `schemas/governance/claim_manifest.schema.json`. |

Each rollup entry carries `rollup_axis_class`, `rollup_key`,
per-state counts (`row_count`, `pass_count`, `fail_count`,
`partial_count`, `waived_count`, `deferred_count`, `narrowed_count`,
`out_of_scope_count`), `milestone_close_posture_class`,
`linked_requirement_ids`, and a bounded `rollup_summary`. The
ship-room dashboard, the architecture pack, the release packet, the
support packet, and the governance review flow MUST consume the same
rollup array. A surface that reconstructs a different per-family or
per-wedge count from raw notes is non-conforming.

## 9. Milestone-close posture

Closed eight-class `milestone_close_posture_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `milestone_close_clean` | Every required row is `pass` with fresh evidence. The schema forbids any row in `fail` or `partial`. |
| `milestone_close_with_named_narrowing` | At least one row is narrowed but every narrowing is cited and visible. |
| `milestone_close_with_active_waiver` | At least one row rides on an active waiver inside its expiry window. |
| `milestone_close_blocked` | At least one required row is `fail` or `waiver_expired_no_renewal`. The schema requires at least one row in `fail`. |
| `milestone_close_blocked_decision_route_unresolved` | At least one row carries `decision_route_unresolved`. |
| `milestone_close_pending_review_window_open` | The milestone close has not been called yet but the review window is open. |
| `milestone_already_closed` | The milestone scorecard is closed. |
| `milestone_rebaselined` | The milestone has been rebaselined. |

A milestone-close posture that says `milestone_close_clean` cannot
coexist with a row in `fail` or `partial`; the schema enforces this.

## 10. Export parity

Every consuming surface that renders the snapshot MUST render the
same typed fields. The parity floor is enforced by the schema's
`export_fields` block.

Required on every consuming surface:

- `snapshot_id`, `snapshot_target.target_milestone`,
  `milestone_close_posture_class`;
- For every `requirement_status_row`:
  - `row_id`, `requirement_id`, `requirement_class`;
  - `milestone_row.target_milestone`, `milestone_row.scorecard_lane_ref`,
    `milestone_row.milestone_close_posture_class`;
  - `ownership.decision_route_status_class`;
  - `requirement_state_class`;
  - `blocker_state.implementation_state_class`,
    `blocker_state.verification_state_class`,
    `blocker_state.release_readiness_class`,
    `blocker_state.integrated_versus_complete_class`;
  - `blockers[]` (each entry's `blocker_class`,
    `decision_right_card_ref`, `blocker_summary`);
  - `narrowing_links[]` (each entry's `narrowing_link_kind_class`,
    `linked_ref`, `narrowing_summary`);
  - `evidence_link_summary_entries[]` (each entry's
    `evidence_packet_ref`, `result_status`, `freshness_class`,
    `evidence_link_summary`);
  - `linked_decision_right_card_refs`,
    `linked_release_truth_summary_refs`,
    `linked_known_limit_refs`, `linked_claim_row_refs`,
    `linked_waiver_register_refs`;
  - `last_review.last_reviewed_at`,
    `last_review.last_reviewer_role_ref`,
    `last_review.review_summary`;
  - `headline_label`, `row_summary`.
- `rollups[]` (each entry's `rollup_axis_class`, `rollup_key`,
  per-state counts, `milestone_close_posture_class`,
  `rollup_summary`).

Forbidden collapses on dashboard, architecture-pack, release-packet,
support-export, governance-packet, claim-manifest, and enterprise-
review surfaces:

- Rendering a row in `fail` as a clean `pass` chip.
- Rendering a row in `waived` without surfacing the waiver register
  entry id and expiry proximity.
- Rendering a row in `narrowed` without surfacing the narrowing
  object id and the affected claim surfaces.
- Rendering a row in `deferred` without surfacing the deferral
  target milestone.
- Rendering a row in `out_of_scope` without surfacing the
  out-of-scope decision id.
- Rendering an `integrated_only_not_complete_for_close` row as
  `pass`.
- Rendering an `evidence_waiver_expired` row as held.
- Reconstructing a per-family, per-wedge, per-workflow, or per-
  claim-family count that disagrees with the snapshot's rollups.

## 11. Projection rules

The snapshot is a projection. The source rows resolve through:

1. **Requirement register**
   ([`requirement_register_seed.yaml`](../../artifacts/governance/requirement_register_seed.yaml))
   — the canonical requirement id, requirement class, and target
   milestones.
2. **Milestone scorecard**
   ([`milestone_scorecard_template.yaml`](../../artifacts/governance/milestone_scorecard_template.yaml))
   — the lane id, primary DRI, evidence owner, backup-owner posture,
   and current scorecard status.
3. **Evidence packets**
   ([`evidence_packet_header.schema.json`](../../schemas/governance/evidence_packet_header.schema.json))
   — `result_status`, `freshness_class`, `captured_at`, and
   `stale_after` copied verbatim.
4. **Decision-right cards**
   ([`decision_right_card.schema.json`](../../schemas/governance/decision_right_card.schema.json))
   — gating decision id and degraded-state label.
5. **Release-truth summaries**
   ([`release_truth_summary.schema.json`](../../schemas/governance/release_truth_summary.schema.json))
   — for rows whose requirement class is in
   `{ PERF, REL, SEC, A11Y, ECOS, AI, CERT, ENT, COMP }`.
6. **Claim manifest, known-limit notes, waiver register**
   — narrowing links, claim surfaces, and waiver expiry.
7. **Decision register**
   ([`decision_register.yaml`](../../artifacts/governance/decision_register.yaml))
   — deferral and out-of-scope decisions.

A reprojection is required when:

- A linked evidence packet's `freshness_class` flips out of
  `{ live, warm_cached }`.
- A linked decision-right card flips state.
- A linked waiver register entry's `expiry_proximity_class` flips
  into `expired_past_due`.
- A linked claim manifest row narrows or republishes.
- The milestone scorecard transitions through
  `{ open, closed_green, closed_yellow, closed_red, rebaselined }`.

The reprojection MUST update `evaluated_at` and the affected
`last_review` entries; rollup counts MUST be recomputed in the same
write.

## 12. Out of scope

This contract does not:

- Implement a milestone-close UI, dashboard, or live editor.
- Integrate with external project-management systems (GitHub
  Projects, Jira, Linear, Asana, Monday). The acceptance criteria in
  [`/.plans/M00-517.md`](../../.plans/M00-517.md) explicitly mark
  external project-management integration as out of scope.
- Replace the milestone scorecard, the architecture pack, the
  release-truth summary, or the support-export packet. The snapshot
  is a projection consumed by those packets, not a replacement.
- Define the persistence model, the storage backend, or the
  reprojection scheduler.

Future implementation tasks pick up the live UI, the persistence
layer, and the reprojection scheduler. They MUST consume this
contract verbatim; a downstream surface that invents a parallel
status grammar is non-conforming.
