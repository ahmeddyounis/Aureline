# Help / About / service-health truth-audit packet, public-drift ledger, and closure-SLA contract

This contract freezes one shared vocabulary for auditing public and
in-product truth across Help, About, service-health, release notes,
known-limits language, and the public-proof surfaces that quote them
against the current claim manifest and evidence bindings. It exists so
"is the public story still true?" stops being a manual spot check
threaded through screenshots, chat history, and review-meeting memory
and becomes one inspectable record per drift case plus one bounded
audit packet per review window.

The truth-audit packet is the projection that the milestone scorecard,
the ship-room dashboard, the release-evidence packet, the claim
manifest review, the support handoff bundle, the public-proof index,
and the weekly governance review all read so they answer the same
audit question with the same row identities, the same severity
vocabulary, the same closure SLA, and the same escalation rules.
Individual drift cases ride on the boundary schema
[`/schemas/public_truth/public_drift_item.schema.json`](../../schemas/public_truth/public_drift_item.schema.json);
worked cases live in
[`/fixtures/public_truth/public_truth_audit_cases/`](../../fixtures/public_truth/public_truth_audit_cases).

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the projection rules, the export-
parity floor, the closure SLA, the escalation rules, and the fixture
corpus. It does not implement documentation publishing, service-status
tooling, or live drift-detection backends.

## Companion artifacts

- [`/schemas/public_truth/public_drift_item.schema.json`](../../schemas/public_truth/public_drift_item.schema.json)
  — boundary schema for one `public_drift_item_record`.
- [`/fixtures/public_truth/public_truth_audit_cases/`](../../fixtures/public_truth/public_truth_audit_cases)
  — worked drift cases covering open, blocking, and closed postures
  across help/About/service-health/release-notes/known-limits/public-
  proof surfaces.
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  and
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — canonical claim-row publication contract. Every drift item cites
  the affected `claim_row_id` set and inherits the row's
  `effective_claim_posture`, `known_limit_refs`, and
  `exclusion_note_refs` vocabulary.
- [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
  — channel-by-channel claim-row projection rules. The audit packet
  reads the parity matrix to determine which channels were required to
  project each affected row.
- [`/docs/governance/public_surface_truth_map.md`](../governance/public_surface_truth_map.md)
  and
  [`/artifacts/governance/source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml)
  — canonical owner-routing map. Every drift item resolves a canonical
  owner artifact through the source-of-truth map before assigning a
  mismatch category.
- [`/docs/governance/drift_blocking_rules.md`](../governance/drift_blocking_rules.md)
  — severity classes, named mismatch categories, same-change-set
  rules, and conservative resolution rule. The audit packet reuses
  this vocabulary verbatim and never invents a parallel severity scale.
- [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  and
  [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  — destination-descriptor contract Help/About/service-health surfaces
  project. Drift on a destination descriptor field (display source
  version, version match state, support class, freshness class,
  availability state, offline behavior) routes through this audit
  packet.
- [`/docs/release/release_notes_whats_new_service_health_contract.md`](../release/release_notes_whats_new_service_health_contract.md)
  and
  [`/schemas/release/whats_new_card.schema.json`](../../schemas/release/whats_new_card.schema.json)
  — release-notes / what's-new / service-health communication
  contract. Drift on a release-note caveat, change-class, or
  service-health banner routes through this audit packet.
- [`/docs/docs/docs_help_pane_contract.md`](../docs/docs_help_pane_contract.md)
  and
  [`/schemas/docs/help_pane_state.schema.json`](../../schemas/docs/help_pane_state.schema.json)
  — docs/help-pane source-version-freshness, mirror/offline, and
  external-open contract. Drift on a docs/help-pane state routes
  through this audit packet.
- [`/docs/governance/waiver_register_contract.md`](../governance/waiver_register_contract.md)
  and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)
  — waiver register. A drift item closed under an active waiver cites
  the waiver register entry id; an SLA breach that crosses the
  release-blocking threshold without a waiver register entry denies.
- [`/docs/docs/reviewed_pack_and_late_copy_policy.md`](../docs/reviewed_pack_and_late_copy_policy.md)
  and
  [`/schemas/docs/late_copy_change_packet.schema.json`](../../schemas/docs/late_copy_change_packet.schema.json)
  — reviewed-pack binding and late-copy packet family. A drift item
  closed by narrowing post-string-freeze copy cites the late-copy
  packet that carried the narrowing.

If this contract disagrees with those companion sources, the schemas
win and this contract, the schema, and the fixtures update in the same
change.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` — public-truth, claim-manifest, docs /
  Help / About / service-health, release-notes, known-limits, and
  public-proof requirements (RFC 2119 MUST / SHOULD language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — claim-row,
  destination-descriptor, and public-proof record shapes.
- `.t2/docs/Aureline_Technical_Design_Document.md` — release-evidence,
  service-health, and supportability-evidence record shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — Help/About/service-
  health/release-notes/known-limits disclosure rules.

## 1. Why this contract exists

1. **Manual spot checks fail at scale.** Without one shared audit
   packet, "is Help/About/service-health/release-notes/known-limits
   still aligned with the claim manifest?" lives as a paragraph in a
   release-readiness review, a screenshot pasted into a support thread,
   a comment on a docs PR, and a sentence in the public-proof packet
   appendix. The reviewer cannot tell whether a row is drifted,
   whether the drift is blocking widening, whether closure is owed
   today or in a release-train window, or whether the drift was already
   closed by a narrowing. The audit packet exists so every consuming
   surface renders the same drift cases with the same row identities.
2. **Drift severity, mismatch category, and closure path must be
   typed.** A drift item that does not name its mismatch category
   (using the
   [`drift_blocking_rules.md`](../governance/drift_blocking_rules.md)
   vocabulary) and its severity class collapses release-blocking
   overclaim, same-change-set blockers, time-boxed defects, and
   internal seed gaps into one chip. Surfaces that render every drift
   as "needs attention" or every drift as "blocking release" hide the
   difference. The schema rejects items whose category and severity
   pair is not allowed by the drift-blocking rules table.
3. **Stale public truth must visibly block channel widening.** A drift
   item older than its closure SLA forfeits the right to widen support
   posture, narrow internal-only seed gaps, or carry a claim-bearing
   release note. The audit packet pins one
   `escalation_state_class` per item so a release-train widening that
   would otherwise close cannot ride past an unresolved drift older
   than its SLA.
4. **Channel widening and claim narrowing are separate paths.** A
   drift item may require **owner-row correction in the same change
   set** (the canonical owner artifact moves to match the public copy),
   **public-copy narrowing** (the public copy moves to match the
   canonical owner row), **claim-row narrowing** (the
   `effective_claim_posture` of the claim row narrows), or
   **public-claim retirement** (the claim row exits the public lane
   entirely). Surfaces that conflate "fix the docs" with "narrow the
   claim" hide which side moved.
5. **Audience and redaction are explicit.** Engineering, support,
   enterprise audit, release readiness, and public-proof reviewers all
   consume the same audit-packet body. The packet pins the typed
   audience and redaction profile so a public-proof export does not
   include support-only payload refs, and a support handoff does not
   include enterprise-only contract churn. The schema rejects items
   whose redaction profile would widen scope past the named audience.
6. **One audit packet drives every consuming surface.** The ship-room
   dashboard, the milestone scorecard, the release-evidence packet,
   the claim-manifest review, the support handoff bundle, the public-
   proof index, and the weekly governance review read the same
   `public_drift_item_record` set. A surface that reformats the audit
   packet, takes a screenshot of the dashboard, or reconstructs a
   parallel narrative status note is non-conforming.

## 2. Audit-packet shape

A truth-audit packet for a given review window carries:

- one stable `audit_packet_id` quoted by every consuming surface;
- one typed `window_kind_class` and `window_ref` (§3);
- the typed `audited_surface_coverage` block (§4) — which
  Help/About/service-health/release-notes/known-limits/public-proof
  surfaces were audited and against which canonical owner artifacts;
- the typed `claim_manifest_baseline_ref` (§5) — the claim-manifest
  packet baseline whose rows the audit was run against;
- the open `public_drift_item_record` set, each carrying the typed
  fields described in §6 onward;
- the closed `public_drift_item_record` set for the window (closed
  during this window, retained on the packet for audit traceability);
- the typed `closure_sla_summary` (§9) — counts per
  `closure_sla_class` and per `escalation_state_class`;
- the typed `change_significance_summary` (§10) — informational,
  release-bearing, claim-narrowing, claim-widening-blocked;
- the typed `consuming_surface_parity` floor (§11);
- bounded reviewable `headline_label` and `audit_packet_summary`.

The audit packet is reprojected when:

- the claim manifest packet baseline advances;
- a canonical owner artifact named in the source-of-truth map moves;
- a destination descriptor row moves on a Help/About/service-health
  surface;
- a release-notes / what's-new / service-health card row moves;
- a known-limit or exclusion note ref moves on a claim row;
- a public-proof packet rotates; or
- a drift item transitions through any `ledger_state_class`.

The audit packet itself is one bounded review object; it is **not** a
schema-bound boundary in this revision. Drift items are the schema-
bound boundary. The audit packet is the bounded composition the
consuming surfaces render. A surface that needs a stable audit-packet
id quotes `audit_packet_ref` from any drift item; every drift item in
one audit window MUST cite the same `audit_packet_ref`.

## 3. Window vocabulary

Closed four-class `window_kind_class`:

| Class | Meaning |
| --- | --- |
| `milestone_close_window` | Audit projects a milestone-close window; `window_ref` cites the milestone slug. The packet pairs with the milestone scorecard and the requirement-status snapshot. |
| `release_train_window` | Audit projects a release train window (stable, LTS, preview); `window_ref` cites the release train id. The packet pairs with the release-truth summary and the public-proof index. |
| `weekly_governance_review_window` | Audit projects the weekly governance review cadence; `window_ref` cites the review row. |
| `ad_hoc_review_window` | Audit projects a one-off review (incident close, audit deep-dive, claim narrowing); `window_ref` cites the ad-hoc review id. |

`baseline_evaluated_at` is the prior audit packet's evaluation
timestamp (or the claim-manifest seed timestamp on the first
projection). Every drift case is computed against that baseline.

## 4. Audited-surface coverage

The audit packet carries one `audited_surface_coverage` block that
names which surfaces were audited this window and against which
canonical owner artifacts. The audited-surface vocabulary mirrors the
parity matrix and the destination descriptor / what's-new / docs-help-
pane / public-proof contracts.

Closed eight-class `audited_surface_class`:

| Class | Meaning |
| --- | --- |
| `help_pane` | Embedded Help/about-this-feature pane state, including source class, version match, freshness class, cache state, offline/mirror posture, locale availability, external-open path, policy-limited behavior. |
| `about_pane` | About / provenance pane state, including exact-build identity, channel, version match, install topology row, support class. |
| `service_health_pane` | Service-health pane state, including local/remote/control-plane/vendor degradations, stale-status banner, policy-blocked banner. |
| `release_notes_card` | Release-notes / what's-new / update-detail card, including change-class, breaking-change notice, migration notice, admin note. |
| `known_limits_section` | Known-limits / exclusion-notes language quoted by docs, release notes, support exports, claim manifest projections. |
| `public_proof_packet` | Public-proof / benchmark publication packet text, including methodology-only or quarantined wording. |
| `cli_help_text` | CLI/headless help text and `--explain` output that quotes claim-bearing wording. |
| `support_export_card` | Support export / handoff packet card that quotes claim-bearing or known-limit wording. |

A drift item whose source surface cannot be typed denies with
`audited_surface_class_unresolved` rather than collapsing to
`help_pane`.

## 5. Claim-manifest baseline binding

Every audit packet pins one `claim_manifest_baseline_ref` (the claim-
manifest packet id whose rows the drift items were resolved against).
A drift item that cites a `claim_row_id` not present in the baseline
denies with `claim_row_not_in_baseline`. A drift item whose claim row
moved between the baseline and the audit window MUST be reprojected
against the new baseline before the audit packet closes.

## 6. Drift-item shape

A `public_drift_item_record` carries:

- `public_drift_item_schema_version` — integer. Current `1`.
- `record_kind` — constant `public_drift_item_record`.
- `drift_item_id` — stable, machine-readable id quoted by every
  consuming surface.
- `audit_packet_ref` — stable opaque ref into the audit packet this
  drift item participates in.
- `detected_at` — RFC 3339 UTC timestamp at which the drift was first
  observed against the baseline. The drift age is `now -
  detected_at`.
- `last_observed_at` — RFC 3339 UTC timestamp at which the drift was
  last reprojected against the live baseline (used so a closed item
  retains the last live observation; an open item carries a fresh
  observation each reprojection).
- `closed_at` — RFC 3339 UTC timestamp or `null`. Required non-null
  when `ledger_state_class` is one of the closed classes.
- `audited_surface_class` — typed (§4).
- `source_surface_ref` — opaque ref to the surface row that drifted
  (e.g. destination descriptor row id, what's-new card id, claim
  manifest projection row id).
- `canonical_owner_artifact_ref` — opaque ref into the canonical owner
  artifact resolved through the
  [`source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml).
- `affected_claim_row_refs[]` — non-empty refs into claim manifest
  row ids whose published meaning is affected by the drift.
- `affected_known_limit_refs[]` — refs into known-limit / exclusion
  note ids the drift drops, contradicts, or makes stale. Empty when
  the drift does not touch caveat refs.
- `affected_evidence_refs[]` — refs into evidence packets whose
  freshness, scope, or downgrade reasons disagree with the drifted
  surface wording. Empty when the drift is purely a copy mismatch.
- `mismatch_category_class` — typed (§7.1).
- `severity_class` — typed (§7.2).
- `narrowing_path_class` — typed (§7.3).
- `channel_widening_blocked` — boolean. `true` when the drift blocks
  the next channel widening (e.g. preview → stable, internal seed →
  public claim) until closed.
- `claim_narrowing_required` — boolean. `true` when the drift forces
  a claim-row narrowing rather than only a public-copy fix.
- `closure_sla_class` — typed (§8.1).
- `sla_due_at` — RFC 3339 UTC timestamp or `null`. Required non-null
  when `closure_sla_class` is one of the timed classes.
- `escalation_state_class` — typed (§8.2).
- `escalation_owner_role_class` — typed (§8.3).
- `ledger_state_class` — typed (§9.1).
- `closure_action_class` — typed (§9.2) or `null`. Required non-null
  when `ledger_state_class` is one of the closed classes; required
  `null` when `ledger_state_class` is one of the open classes (the
  closure path is decided at closure time).
- `closure_notes` — bounded reviewable sentence or `null`. Required
  non-null when `ledger_state_class` is one of the closed classes.
- `linked_change_set_refs[]` — opaque refs into the change set(s)
  that landed the closure (PR ids, decision register row ids, late-
  copy packet ids).
- `linked_late_copy_packet_refs[]` — refs into late-copy change
  packets that carried the closure when it crossed string-freeze.
- `linked_waiver_register_refs[]` — refs into waiver register entry
  ids that hold the drift open under an active waiver.
- `audience_class` — typed (§12). Mirrors the records-governance
  packet vocabulary.
- `redaction_profile_class` — typed (§12). Schema-paired with
  `audience_class`.
- `linked_artifact_families` — typed refs into the artifact families
  the drift item composes over (§13).
- `consuming_surface_parity` — typed booleans (§11).
- `headline_label` and `drift_item_summary` — bounded reviewable
  label and one-sentence summary.
- `contract_doc_ref` — constant
  `docs/public_truth/help_about_service_health_audit_packet.md`.
- `claim_manifest_contract_ref` — constant
  `docs/governance/claim_manifest_contract.md`.
- `public_truth_parity_matrix_ref` — constant
  `artifacts/governance/public_truth_parity_matrix.yaml`.
- `drift_blocking_rules_ref` — constant
  `docs/governance/drift_blocking_rules.md`.
- `notes` — optional bounded reviewable sentence.

Raw screenshots, raw page bodies, raw release-notes prose, raw user
identifiers, raw support-case bodies, and raw waiver justifications
MUST NOT appear; the record carries opaque refs, typed vocabulary, and
bounded reviewable summaries only.

## 7. Mismatch category, severity, and narrowing-path vocabulary

### 7.1 Mismatch category

Closed seven-class `mismatch_category_class` mirroring
[`drift_blocking_rules.md`](../governance/drift_blocking_rules.md)
verbatim:

| Class | Severity pairing |
| --- | --- |
| `owner_row_missing` | `same_change_blocker` |
| `projection_broader_than_owner` | `release_blocking_overclaim` |
| `policy_disabled_hidden` | `release_blocking_overclaim` |
| `support_window_mismatch` | `same_change_blocker` |
| `known_limit_missing` | `same_change_blocker` |
| `compatibility_or_skew_alias_drift` | `release_blocking_overclaim` |
| `proof_packet_out_of_sync` | `time_boxed_truth_defect` or `release_blocking_overclaim` |

A drift whose category cannot be typed denies with
`mismatch_category_class_unresolved` rather than defaulting.

### 7.2 Severity

Closed four-class `severity_class` mirroring
[`drift_blocking_rules.md`](../governance/drift_blocking_rules.md)
verbatim:

| Class | Promotion impact |
| --- | --- |
| `release_blocking_overclaim` | Blocks merge on protected public-truth changes and blocks release or proof promotion. |
| `same_change_blocker` | Blocks merge until every required companion artifact is updated together. |
| `time_boxed_truth_defect` | Blocks widening and blocks the next channel move while open. |
| `seed_gap_review_required` | Does not block internal review, but blocks public or support-language widening. |

The schema enforces the category × severity pairing per §7.1: a drift
whose category and severity disagree denies with
`mismatch_category_severity_pair_invalid`.

### 7.3 Narrowing path

Closed five-class `narrowing_path_class`:

| Class | Meaning |
| --- | --- |
| `same_change_set_owner_correction` | The canonical owner artifact moves to match the public copy in the same change set. The downstream surface stays as written. |
| `public_copy_narrowing` | The public copy moves to match the canonical owner row in the same change set or via a late-copy packet. The owner row stays as written. |
| `claim_row_narrowing` | The `effective_claim_posture` of the claim row narrows; downstream channels reproject from the narrowed row. |
| `public_claim_retirement` | The claim row exits the public lane entirely; the row is retained internally with a successor or replacement-grade reference. |
| `seed_gap_internal_only` | The drift is held internal-only; no public claim is widened until the seed gap closes. |

A drift item whose `claim_narrowing_required` flag is `true` MUST set
`narrowing_path_class` to `claim_row_narrowing` or
`public_claim_retirement`; the schema enforces this pairing.

## 8. Closure SLA and escalation rules

### 8.1 Closure SLA vocabulary

Closed five-class `closure_sla_class`:

| Class | Closure window | Pairs with severity | `sla_due_at` |
| --- | --- | --- | --- |
| `same_change_set_required` | The closure must land in the same change set as the trigger. | `same_change_blocker`, `release_blocking_overclaim` | non-null; pinned to the gating change set's merge target |
| `time_boxed_24h` | Closure within 24 hours of `detected_at` or before the next channel move, whichever is sooner. | `time_boxed_truth_defect` | non-null; `detected_at + 24h` or the next channel-move timestamp |
| `time_boxed_next_channel_move` | Closure before the next channel move (e.g. preview → stable, stable → LTS). | `time_boxed_truth_defect`, `release_blocking_overclaim` | non-null; pinned to the gating channel-move timestamp |
| `internal_only_no_sla` | Drift is held internal-only; no public widening until closure. | `seed_gap_review_required` | `null` (no public clock) |
| `claim_narrowed_no_sla` | Drift was already closed by a claim-row narrowing or claim-row retirement; the public clock no longer applies. | any | `null` |

A drift whose closure SLA cannot be typed denies with
`closure_sla_class_unresolved` rather than defaulting.

### 8.2 Escalation state

Closed five-class `escalation_state_class`:

| Class | Meaning |
| --- | --- |
| `within_sla` | `now < sla_due_at` and the drift is open under review. |
| `sla_warning` | `now` is within 25% of `sla_due_at` (or within 6 hours for a 24-hour SLA); review owner is paged but the drift is not yet blocking. |
| `sla_breached` | `now >= sla_due_at`; the drift is open and the closure window expired. Channel widening is frozen and the next release-train window cannot close until the drift closes or a waiver register entry holds it. |
| `sla_breached_release_blocking` | `sla_breached` AND severity is `release_blocking_overclaim` or `same_change_blocker`. The release-evidence packet MUST cite this drift as a release blocker. |
| `sla_breached_widening_frozen` | `sla_breached` AND `channel_widening_blocked = true`. Stable widening (preview → stable, support-class widening, public-claim widening) is frozen until closure. |

The schema enforces:

- `escalation_state_class = sla_breached_release_blocking` requires
  severity in `{release_blocking_overclaim, same_change_blocker}` and
  a non-empty `linked_waiver_register_refs[]` if the drift is being
  held open past the SLA;
- `escalation_state_class = sla_breached_widening_frozen` requires
  `channel_widening_blocked = true`.

### 8.3 Escalation owner role

Closed six-class `escalation_owner_role_class`:

| Class | Meaning |
| --- | --- |
| `docs_public_truth_owner` | Default closure owner for help/About/service-health/release-notes/known-limits drift. |
| `claim_manifest_owner` | Required closure owner when the drift forces a claim-row narrowing or retirement. |
| `release_council` | Required closure owner when the drift is `release_blocking_overclaim` and the next release train is gated. |
| `shiproom_executive` | Required closure owner when the drift sits at `sla_breached_release_blocking` and crosses release lines. |
| `support_lane_owner` | Required closure owner when the drift originates from `support_export_card` or carries a `support_window_mismatch` category. |
| `governance_council` | Required closure owner when the drift forces a waiver register entry, a decision register row, or a public-claim retirement. |

A drift item whose owner role cannot be typed denies with
`escalation_owner_role_class_unresolved` rather than defaulting.

## 9. Ledger state and closure action

### 9.1 Ledger state

Closed seven-class `ledger_state_class`:

| Class | Meaning |
| --- | --- |
| `open_under_review` | Drift is open and within SLA; review owner is named; no widening is blocked yet. |
| `open_blocking_widening` | Drift is open, within or past SLA, and `channel_widening_blocked = true`. |
| `open_blocking_release` | Drift is open and `severity_class` is `release_blocking_overclaim` or `same_change_blocker`; the next release packet MUST cite this drift. |
| `closed_via_owner_correction` | Drift closed by `same_change_set_owner_correction`; the canonical owner artifact moved to match the public copy. |
| `closed_via_public_copy_narrowing` | Drift closed by `public_copy_narrowing`; the public copy moved to match the canonical owner row. |
| `closed_via_claim_narrowing` | Drift closed by `claim_row_narrowing` or `public_claim_retirement`; the claim row narrowed or retired and downstream channels reprojected. |
| `closed_no_change_required` | Drift was misclassified or resolved by an unrelated change; the audit retains the row for traceability. |

### 9.2 Closure action

Closed eight-class `closure_action_class`:

| Class | Meaning |
| --- | --- |
| `owner_artifact_updated` | Canonical owner artifact moved in the same change set. |
| `public_copy_narrowed_in_release` | Public copy narrowed in the same release; pairs with `release_notes_card`, `help_pane`, `about_pane`, `service_health_pane`. |
| `public_copy_narrowed_via_late_copy_packet` | Public copy narrowed post-string-freeze via a `late_copy_change_packet`. |
| `claim_row_effective_posture_narrowed` | `effective_claim_posture` narrowed on the claim row; downstream channels reprojected. |
| `claim_row_retired_from_public_lane` | Claim row exited the public lane; row retained internally with a successor or replacement-grade reference. |
| `held_under_active_waiver` | Drift held open under an active waiver register entry; closure owed by the waiver renewal review. |
| `held_internal_only_seed_gap` | Drift kept internal-only; no public widening is allowed until the seed gap closes. |
| `closed_no_change_required` | Drift retired without a closing change. |

The schema enforces:

- `closure_action_class` is non-null when `ledger_state_class` is one
  of the closed classes and `null` when `ledger_state_class` is one
  of the open classes;
- `closure_action_class = public_copy_narrowed_via_late_copy_packet`
  requires `linked_late_copy_packet_refs[]` non-empty;
- `closure_action_class = held_under_active_waiver` requires
  `linked_waiver_register_refs[]` non-empty;
- closed `ledger_state_class` values require non-null `closed_at`,
  non-null `closure_notes`, and non-null `closure_action_class`.

### 9.3 Closure SLA summary block

The audit packet's `closure_sla_summary` is a typed counts block:

- per-`closure_sla_class` count of open items;
- per-`escalation_state_class` count of open items;
- per-`severity_class` count of open items;
- aggregate `widening_frozen_count` and `release_blocking_count`.

A consuming surface that drops one of these counters or substitutes a
narrative paragraph for the typed counts is non-conforming.

## 10. Change-significance rules

Closed four-class `overall_significance_class` mirroring the records-
governance packet vocabulary:

| Class | Meaning |
| --- | --- |
| `informational` | Audit packet records drift activity that does not move a release floor, does not narrow a marketed claim, and does not block an active widening lane. |
| `release_bearing` | At least one open or closed-this-window drift item gates a release train, milestone, or compatibility window. The release-truth summary MUST cite this audit packet. |
| `claim_narrowing` | At least one drift item carries `narrowing_path_class` in `{claim_row_narrowing, public_claim_retirement}` and at least one claim row narrowed this window. |
| `claim_widening_blocked` | A previously narrowed claim is awaiting a widening decision; the widening is blocked pending decision. The audit packet MUST cite the gating waiver register entry and the gating decision register row. |

Schema-enforced pairings (per drift item):

- `narrowing_path_class = claim_row_narrowing` requires
  `claim_narrowing_required = true`;
- `narrowing_path_class = public_claim_retirement` requires
  `claim_narrowing_required = true`;
- `closure_action_class = claim_row_effective_posture_narrowed`
  requires `narrowing_path_class = claim_row_narrowing`;
- `closure_action_class = claim_row_retired_from_public_lane`
  requires `narrowing_path_class = public_claim_retirement`.

## 11. Consuming-surface parity

Every consuming surface that renders a drift item MUST render the same
typed fields. The parity floor is enforced by the schema's
`consuming_surface_parity` block.

Required on every consuming surface:

- `drift_item_id`, `audit_packet_ref`, `detected_at`,
  `last_observed_at`, `closed_at`;
- `audited_surface_class`, `source_surface_ref`,
  `canonical_owner_artifact_ref`;
- `affected_claim_row_refs`, `affected_known_limit_refs`;
- `mismatch_category_class`, `severity_class`,
  `narrowing_path_class`;
- `channel_widening_blocked`, `claim_narrowing_required`;
- `closure_sla_class`, `sla_due_at`, `escalation_state_class`,
  `escalation_owner_role_class`;
- `ledger_state_class`, `closure_action_class`, `closure_notes`;
- `audience_class`, `redaction_profile_class`;
- `headline_label`, `drift_item_summary`.

Forbidden collapses on every consuming surface:

- Rendering a `sla_breached_release_blocking` drift as
  "review needed" without the typed escalation state.
- Dropping `affected_known_limit_refs[]` to keep the dashboard quiet.
- Reformatting the drift item into a screenshot or a parallel
  narrative status note.
- Substituting "should be quick" or "close to closure" for a typed
  `sla_due_at` and `escalation_state_class` value.
- Rendering a `claim_widening_blocked` drift as `informational` to
  keep the public claim manifest clean.
- Filtering out a `held_under_active_waiver` drift under a "review
  needed" affordance.

## 12. Audience and redaction vocabulary

Closed five-class `audience_class` and matching five-class
`redaction_profile_class` (mirrors the records-governance packet
contract verbatim):

| Audience | Required redaction profile |
| --- | --- |
| `engineering_internal` | `engineering_internal_only` |
| `support_handoff` | `support_handoff_redacted` |
| `enterprise_audit` | `enterprise_audit_redacted` |
| `release_readiness` | `release_readiness_summary` |
| `public_proof_safe` | `public_proof_safe_zero_payload` |

The schema enforces that `redaction_profile_class` matches the
`audience_class` per the table above. A drift item whose redaction
profile widens scope past the named audience denies with
`redaction_profile_widens_scope`.

The honesty invariants on the redaction profile:

- `raw_screenshots_excluded` — constant `true`.
- `raw_page_body_bytes_excluded` — constant `true`.
- `raw_user_identifiers_excluded` — constant `true`.
- `raw_waiver_justifications_excluded` — constant `true`.

## 13. Linked artifact families

Refs cite stable artifact-family ids resolved through the named
schemas / docs. Empty arrays are admissible.

- `claim_manifest_row_refs[]` — claim manifest row ids the drift item
  composes over. MUST equal the union of `affected_claim_row_refs[]`.
- `destination_descriptor_row_refs[]` — destination descriptor row
  ids the drift item composes over. Required non-empty when
  `audited_surface_class` is in
  `{help_pane, about_pane, service_health_pane}`.
- `whats_new_card_row_refs[]` — what's-new / release-notes card row
  ids. Required non-empty when `audited_surface_class` is
  `release_notes_card`.
- `help_pane_state_refs[]` — docs/help-pane state row ids. Required
  non-empty when `audited_surface_class` is `help_pane`.
- `public_proof_row_refs[]` — public-proof / claim-manifest
  publication row ids. Required non-empty when
  `audited_surface_class` is `public_proof_packet`.
- `support_export_packet_refs[]` — support export packet ids. Required
  non-empty when `audited_surface_class` is `support_export_card`.
- `evidence_packet_refs[]` — evidence packet ids whose freshness or
  scope disagrees with the drifted surface wording.
- `late_copy_change_packet_refs[]` — late-copy change packet ids that
  carried the closure post-string-freeze.

The schema enforces the surface-pairing requirements above.

## 14. Reuse rules

Docs, release, support, and governance reviews consume the same audit
packet by reading the typed drift-item record set. The reuse rules:

1. **No surface re-runs the audit by hand.** A docs review, a release
   readiness review, a support handoff packet, or a weekly governance
   review that wants the current "is the public story aligned?" answer
   reads the audit packet's drift items and the typed
   `closure_sla_summary` instead of re-screenshotting Help/About/
   service-health surfaces.
2. **No surface invents a parallel severity scale.** Severity classes,
   mismatch categories, closure SLA classes, escalation state classes,
   and escalation owner roles are typed; a surface that introduces a
   "soft warning" or "yellow" chip on top of these is non-conforming.
3. **No surface drops drift items selectively.** A consuming surface
   that filters drift items by audience MUST go through the typed
   `redaction_profile_class` projection; ad-hoc filtering by drift age
   or by claim row is non-conforming.
4. **Channel widening MUST consult the audit packet.** A release
   train widening, a preview → stable widening, a support-class
   widening, or a public-claim widening MUST resolve the audit
   packet's `widening_frozen_count` and refuse to widen while it is
   non-zero unless the gating drift items are held under waiver
   register entries.
5. **Claim narrowings MUST cite the audit packet.** A claim manifest
   review that narrows `effective_claim_posture` because of a drift
   cites the originating drift item id; the audit packet retains the
   closed item with `closure_action_class =
   claim_row_effective_posture_narrowed`.
6. **Public-proof packets MUST cite the audit packet.** A public-proof
   / benchmark publication packet that crosses an audit window cites
   the audit packet id and the closed-this-window drift items so the
   public-proof index can render the same review state.
7. **Late-copy narrowings MUST cite the audit packet.** A late-copy
   change packet that narrows protected publication copy after string
   freeze cites the originating drift item id and the audit packet id;
   the drift item carries `closure_action_class =
   public_copy_narrowed_via_late_copy_packet` and a non-empty
   `linked_late_copy_packet_refs[]`.

## 15. Authoring rules

When a Help, About, service-health, release-notes, known-limits,
public-proof, CLI/help, or support-export surface drifts away from the
canonical owner artifact or the claim manifest:

1. Mint a `public_drift_item_record` projecting the drift against the
   audit packet's window.
2. Resolve the canonical owner artifact through
   [`source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml).
3. Resolve every `claim_row_id` against the claim manifest baseline
   pinned on the audit packet.
4. Resolve every `known_limit_ref` and `exclusion_note_ref` against
   the affected claim rows.
5. Assign `mismatch_category_class` and `severity_class` from
   [`drift_blocking_rules.md`](../governance/drift_blocking_rules.md);
   the schema enforces the allowed pair.
6. Assign `narrowing_path_class` from §7.3 and set
   `claim_narrowing_required` accordingly.
7. Assign `closure_sla_class`, `sla_due_at`, and the initial
   `escalation_state_class` per §8.
8. Assign `ledger_state_class` per §9.1.
9. When the drift closes, set `closure_action_class`, `closed_at`,
   `closure_notes`, and the linked change-set / late-copy / waiver
   refs per §9.2.
10. Recompute `consuming_surface_parity` so the dashboard, milestone
    scorecard, release packet, support export, governance packet,
    claim manifest, public-proof index, and weekly governance review
    render the same record.

A reprojection is required when:

- the claim-manifest baseline advances;
- a canonical owner artifact moves;
- the drift's `escalation_state_class` should advance based on the
  current clock against `sla_due_at`;
- a waiver register entry holding the drift transitions through
  `register_active_within_expiry`,
  `register_active_pending_renewal`,
  `register_renewed_under_new_decision`,
  `register_closed_correction_landed`,
  `register_narrowed_claim_published`,
  `register_escalated_pending_resolution`,
  `register_rejected_no_protection`, or
  `register_expired_no_decision`.

The reprojection MUST advance `last_observed_at` and recompute the
escalation state.

## 16. Out of scope

This contract does not implement:

- Documentation publishing, service-status tooling, live drift-
  detection backends, or any automated surface scraping.
- The claim manifest itself (that lives on
  [`./../governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  and
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)).
- The destination descriptor, what's-new card, docs/help-pane state,
  or public-proof packet schemas (those live on the named
  contracts).
- The drift-blocking rules table itself (that lives on
  [`./../governance/drift_blocking_rules.md`](../governance/drift_blocking_rules.md)).
  This contract reuses the vocabulary and projects it through one
  inspectable record per drift case.
- The waiver register or the renewal-or-close decision objects (those
  live on
  [`./../governance/waiver_register_contract.md`](../governance/waiver_register_contract.md)
  and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)).

This contract is the projection vocabulary that public-truth drift
flows through when it reaches the milestone scorecard, the release
packet, the support handoff bundle, the claim manifest, the public-
proof index, and the weekly governance review — so a reviewer can
answer "is the public story aligned?" mechanically rather than by
manual memory.
