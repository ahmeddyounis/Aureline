# Policy-simulation diff, waiver-expiry, and chronology verification seed

This packet freezes one shared verification story for policy previews,
remembered-decision narrowing, waiver and remembered-decision expiry
drift, and chronology-bar export truth. It exists so later admin,
support, export, and enterprise-facing policy flows reuse one
inspectable object model instead of inventing local exception wording,
silent remembered-decision carry-forward, or ambiguous timeline labels.

If this packet, the
[`simulation_diff_manifest.yaml`](../../fixtures/policy/simulation_diff_manifest.yaml)
corpus, the
[`waiver_expiry_dashboard_contract.yaml`](../../artifacts/policy/waiver_expiry_dashboard_contract.yaml)
contract, and the frozen governance schemas disagree, the machine-
readable artifacts and the base governance schemas win for tooling and
this packet must update in the same change.

Companion artifacts:

- [`/fixtures/policy/simulation_diff_manifest.yaml`](../../fixtures/policy/simulation_diff_manifest.yaml)
  — machine-readable diff-suite roster covering grant-to-deny changes,
  narrower-scope carry-forward, expired remembered decisions, future-
  effective policy changes, legal-hold interaction, and audit-export
  chronology joins.
- [`/artifacts/policy/waiver_expiry_dashboard_contract.yaml`](../../artifacts/policy/waiver_expiry_dashboard_contract.yaml)
  — machine-readable dashboard contract naming required counts, ageing
  fields, expiry windows, row joins, and visibility rules for waivers
  and remembered decisions.
- [`/fixtures/policy/chronology_bar_cases/`](../../fixtures/policy/chronology_bar_cases/)
  — reviewer-facing chronology-bar cases preserving effective time,
  display timezone, actor identity, and ordering truth for support and
  export packets.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — canonical governed-record, chronology, policy-simulation,
  remembered-decision, and waiver-expiry vocabulary this packet reuses.
- [`/schemas/governance/record_state.schema.json`](../../schemas/governance/record_state.schema.json)
  and
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)
  — canonical machine-readable vocabulary for policy simulations,
  chronology packets, waivers, and remembered decisions.
- [`/fixtures/governance/record_state_examples/`](../../fixtures/governance/record_state_examples/)
  — existing schema-conforming governance fixtures this packet cites as
  baseline examples instead of redefining them.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — the canonical requirement-register and evidence-governance posture,
  plus the later waiver-expiry automation epic.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — remembered approvals must mint fresh tickets against the current
  policy epoch; waiver and suppression objects must render identically
  across local and CI/provider surfaces.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — chronology and timeline rows remain append-only and exportable
  rather than reconstructed from display-only labels.
- `.t2/docs/Aureline_Milestones_Document.md`
  — review packets, waivers, and proof artifacts stay inspectable and
  source-anchored during the foundations phase.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.policy_simulation.diff_and_expiry_seed
evidence_id: evidence.verification.policy_simulation.packet
title: Policy-simulation diff, waiver-expiry, and chronology verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
    - GOV-CORPUS-901
    - GOV-DATA-002
  claim_row_refs:
    - packet_row:policy_simulation.input_output_contract
    - packet_row:policy_simulation.diff_severity
    - packet_row:policy_simulation.remembered_decision_narrowing
    - packet_row:policy_simulation.waiver_expiry_dashboard_join
    - packet_row:policy_simulation.chronology_bar_export_truth
    - packet_row:policy_simulation.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: policy_simulation_diff_seed@1
  trigger_revision: policy_simulation_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen governed-record policy vocabulary, the
    new diff-suite corpus, and the dashboard contract for waiver and
    remembered-decision drift. No live policy-engine or admin UI claim
    is made yet.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.policy_simulation.diff_manifest
    - evidence.policy.waiver_expiry.dashboard_contract
    - evidence.verification.policy.chronology_bar_cases
    - evidence.verification.governance.record_state_examples
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/policy/simulation_diff_manifest.yaml
    - fixtures/policy/chronology_bar_cases/
    - fixtures/governance/record_state_examples/deletion_policy_simulation.json
    - fixtures/governance/record_state_examples/remembered_decision_policy_epoch_bound.json
    - fixtures/governance/record_state_examples/waiver_expiry_policy_epoch_bound.json
    - fixtures/governance/record_state_examples/mixed_chronology_admin_timeline.json
  archetype_refs: []
  source_anchor_refs:
    - docs/governance/record_state_and_policy_simulation_models.md
    - schemas/governance/record_state.schema.json
    - schemas/governance/waiver_expiry.schema.json
    - artifacts/governance/requirement_register_seed.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes one policy-diff object model and one
chronology-bar field set so policy previews can explain what changed,
which remembered decision narrowed or expired, which waiver bucket the
change lands in, and which exact time or ordering fields support later
support/export packets.

It does not claim a live policy engine, a finished enterprise admin UI,
or automated schema validation for the new policy-specific corpus yet.
It claims only that the packet, manifest, dashboard contract, and
chronology cases now exist in one reviewable form and reuse the base
governance vocabulary already frozen elsewhere in the repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:policy_simulation.input_output_contract` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.policy_simulation.diff_manifest` | Freezes one machine-readable input/output object for policy diffs. |
| `packet_row:policy_simulation.diff_severity` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.policy_simulation.diff_manifest` | Exact change, narrower result, expired memory, future-effective, and blocked-by-hold states stay typed rather than prose-only. |
| `packet_row:policy_simulation.remembered_decision_narrowing` | `GOV-TRUTH-901`, `GOV-DATA-002` | `seed_only` | `internal` | `evidence.verification.policy_simulation.diff_manifest`, `evidence.verification.governance.record_state_examples` | Carry-forward and reprompt posture now include scope, policy-epoch, and expiry joins. |
| `packet_row:policy_simulation.waiver_expiry_dashboard_join` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.policy.waiver_expiry.dashboard_contract` | Dashboard buckets, counts, and drift rules are reviewable from packet data. |
| `packet_row:policy_simulation.chronology_bar_export_truth` | `GOV-TRUTH-901`, `GOV-DATA-002` | `seed_only` | `internal` | `evidence.verification.policy.chronology_bar_cases` | Effective time, display timezone, actor, and ordering keys stay exportable. |
| `packet_row:policy_simulation.seed_corpus` | `GOV-CORPUS-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.policy_simulation.diff_manifest`, `evidence.verification.policy.chronology_bar_cases` | One stable case-id set now covers the required diff and chronology scenarios. |

## What this seed freezes

- One `simulation_input_object` that names baseline and proposed policy
  refs, governed subject, scope, actor, effective chronology, and
  remembered-decision or waiver bindings.
- One `simulation_output_object` that names the primary result class,
  secondary flags, diff severity, resulting posture, projected effect
  codes, blockers, remembered-decision audit row, waiver-expiry
  projection, chronology-bar case ref, and audit-export field set.
- One closed diff-severity vocabulary so previews stop collapsing
  blocking denies, future-effective changes, expiry-driven reprompts,
  and scope narrowing into the same generic "changed" summary.
- One narrowing-audit rule set so remembered decisions can be narrowed
  or retired explicitly rather than silently replayed on a broader or
  stale scope.
- One chronology-bar field set so timeline chips, support packets, and
  export packets quote the same effective time, timezone, actor, and
  ordering truth.

## Simulation input object

Every case in the machine-readable manifest resolves to one
`simulation_input_object` with these required fields:

- `case_id`
- `baseline_policy_ref`
- `proposed_policy_ref`
- `subject_ref`
- `scope_ref`
- `actor_ref`
- `evaluation_utc_instant`
- `effective_from_utc_instant`
- `remembered_decision_refs`
- `waiver_refs`
- `chronology_bar_case_ref`

Example:

```yaml
simulation_input_object:
  case_id: policy.diff.grant_to_deny.connected_provider
  baseline_policy_ref: policy.connected_provider.workspace.allow.v1
  proposed_policy_ref: policy.connected_provider.workspace.deny.v2
  subject_ref:
    subject_kind: connected_provider_record
    subject_id: cpr-provider-alpha-0019
  scope_ref:
    scope_kind: workspace
    scope_id: ws-aureline
  actor_ref:
    stable_id: admin-policy-review
    display_name: policy admin
    role: admin
  evaluation_utc_instant: 2026-04-23T17:15:00Z
  effective_from_utc_instant: 2026-04-23T17:30:00Z
  remembered_decision_refs:
    - rd-connected-provider-grant-0019
  waiver_refs:
    - we-remembered-decision-pe-bound-0019
  chronology_bar_case_ref: chronology_bar.policy_change_immediate_denial
```

Rule: `effective_from_utc_instant` is always explicit. A future-
effective change never renders as an immediate deny or allow merely
because a relative label said "tomorrow."

## Simulation output object

Every case resolves to one `simulation_output_object` with these
required fields:

- `primary_result_class`
- `secondary_result_flags`
- `decision_delta_class`
- `diff_severity`
- `resulting_posture`
- `projected_effect_codes`
- `blocked_by_refs`
- `remembered_decision_audit`
- `waiver_expiry_projection`
- `chronology_bar_case_ref`
- `audit_export_required_fields`

Example:

```yaml
simulation_output_object:
  primary_result_class: exact_change
  secondary_result_flags:
    - exact_change_detected
  decision_delta_class: grant_to_deny
  diff_severity: blocking
  resulting_posture: deny
  projected_effect_codes:
    - would_be_refused_by_policy
    - would_supersede_remembered_decision
  blocked_by_refs: []
  remembered_decision_audit:
    remembered_decision_ref: rd-connected-provider-grant-0019
    baseline_scope: workspace
    proposed_scope: workspace
    carry_forward_scope: none
    carry_forward_reason_class: policy_changed
    memory_state_before: active
    memory_state_after: superseded
  waiver_expiry_projection:
    waiver_ref: we-remembered-decision-pe-bound-0019
    waiver_state_before: active
    waiver_state_after: force_retired_by_policy
    dashboard_bucket: force_retired_by_policy
    drift_reason_codes: []
  chronology_bar_case_ref: chronology_bar.policy_change_immediate_denial
  audit_export_required_fields:
    - effective_utc_instant
    - display_local_time
    - display_timezone_id
    - actor_ref
    - event_kind
    - event_order_key
```

Rule: a simulation output may not say only "changed." It must at least
name one primary result class, one diff severity, and the exact
remembered-decision or blocker story that explains the change.

## Diff severity

The diff suite freezes four severity labels:

| `diff_severity` | Meaning | Required rendering behavior |
|---|---|---|
| `informational` | Exact non-blocking change or export-only chronology evidence | Show delta and chronology, but no admin intervention badge is implied. |
| `review_required` | Narrower scope or future-effective change that requires explicit disclosure | Show scope or effective-time narrowing before any apply action. |
| `warning` | Expired or expiring remembered decision / waiver that requires reprompt, cleanup, or renewal review | Show expiry posture and next actor; do not collapse into generic stale copy. |
| `blocking` | Policy deny, legal-hold interception, or other blocker that prevents the previewed outcome from proceeding now | Show blocker ids and exact blocked state; do not render a soft warning chip. |

The manifest also freezes these result classes:

- `exact_change`
- `narrower_result`
- `expired_memory`
- `future_effective_pending`
- `blocked_by_hold`

And these secondary flags:

- `exact_change_detected`
- `narrower_result_detected`
- `expired_memory_detected`
- `future_effective_detected`
- `blocked_by_hold_detected`

Rule: `primary_result_class = blocked_by_hold` requires
`diff_severity = blocking` and a non-empty `blocked_by_refs` list.

## Remembered-decision narrowing audit

The remembered-decision audit row names:

- `remembered_decision_ref`
- `baseline_scope`
- `proposed_scope`
- `carry_forward_scope`
- `carry_forward_reason_class`
- `memory_state_before`
- `memory_state_after`

The carry-forward reason vocabulary is:

- `unchanged_subject_and_scope`
- `narrowed_scope_with_same_policy_epoch`
- `policy_changed`
- `policy_epoch_changed`
- `expiry_elapsed`
- `hold_intercepted`

The memory-state vocabulary is:

- `none`
- `active`
- `narrowed`
- `expiring_soon`
- `expired`
- `superseded`
- `force_retired_by_policy`

Rules:

- Carry-forward is allowed only when the resulting scope is equal to or
  narrower than the remembered scope and the policy epoch still matches
  the remembered-decision binding.
- If the policy epoch changed or the remembered decision expired,
  `carry_forward_scope` must be `none` and the output must render either
  `expired_memory` or a supersession state explicitly.
- A legal hold may block the previewed effect, but it may not silently
  widen a remembered allow. The output must keep the remembered-decision
  audit row and the hold blocker visible at the same time.

## Waiver and remembered-decision expiry states

The packet and dashboard contract use one review vocabulary for expiry
projection buckets:

- `healthy`
- `expiring_soon`
- `future_effective_pending`
- `narrowed_carry_forward`
- `expired_reprompt_due`
- `blocked_by_hold`
- `force_retired_by_policy`
- `superseded_pending_cleanup`
- `drift_detected`

The packet does not replace the underlying waiver schema's
`status` field. It adds one dashboard-facing projection so packet data
can roll into counts and stale-review surfaces without free-text
interpretation.

The waiver-state projection used in the diff suite is:

- `none`
- `active`
- `expired_silent`
- `expired_with_reprompt`
- `renewed`
- `superseded`
- `retired`
- `force_retired_by_policy`

Drift reason codes are:

- `remembered_decision_expired`
- `waiver_expiry_elapsed`
- `carry_forward_scope_broader_than_result`
- `missing_linked_governed_record`
- `policy_epoch_mismatch`

Rule: a row whose expiry instant or policy-epoch boundary has already
passed while the packet still projects it as reusable must set
`dashboard_bucket = drift_detected` and include at least one drift
reason code.

## Chronology-bar requirements

Every chronology-bar case carries these required fields:

- `case_id`
- `actor_ref`
- `event_ref`
- `effective_time.utc_instant`
- `effective_time.local_iso_with_offset`
- `effective_time.timezone_id`
- `effective_time.offset_at_instant`
- `source_clock_class`
- `skew_flag`
- `ordering.ordering_relation`
- `export_representation_rule`
- `rendering_representation`
- `support_export_fields`

The bar may also carry either `ordering_anchor_ref` or
`canonical_sequence_uid` depending on the ordering relation. Display
strings are never the ordering key.

Example:

```yaml
chronology_bar_record:
  case_id: chronology_bar.audit_export_mixed_timezones
  actor_ref:
    stable_id: support-export-service
    display_name: support export service
    role: system
  event_ref:
    event_kind: audit_export_assembled
    event_id: evt-audit-export-assembled-01
    event_label: Audit export assembled
  effective_time:
    utc_instant: 2026-04-24T07:40:00Z
    local_iso_with_offset: 2026-04-24T09:40:00+02:00
    timezone_id: Europe/Berlin
    offset_at_instant: "+02:00"
  source_clock_class: remote_host_wall_clock
  skew_flag: skew_bounded
  ordering:
    ordering_relation: total_order_from_canonical_uid
    canonical_sequence_uid: audit-export-00042
  export_representation_rule: both_utc_and_local
  rendering_representation: local_with_tz_shadow_utc
  support_export_fields:
    - effective_utc_instant
    - display_local_time
    - display_timezone_id
    - actor_ref
    - event_kind
    - event_order_key
```

Rules:

- A chronology bar must preserve both `utc_instant` and local display
  time whenever `rendering_representation` uses a timezone-aware local
  label.
- The bar must preserve `actor_ref` and `event_kind`; export packets
  must not reduce the row to time-only history.
- `ordering_relation = total_order_from_canonical_uid` requires the
  canonical sequence uid to travel with the export. Sorting by rendered
  local text is non-conforming.
- A future-effective row must preserve the future effective instant and
  its display timezone; it may not be rewritten as "active now" merely
  because the viewer sits in another timezone.

## Seed corpus

The machine-readable manifest seeds these case ids:

| Case id | Primary result class | Severity | Chronology bar case | Notes |
|---|---|---|---|---|
| `policy.diff.grant_to_deny.connected_provider` | `exact_change` | `blocking` | `chronology_bar.policy_change_immediate_denial` | Immediate deny replaces a remembered allow on the same subject and scope. |
| `policy.diff.narrower_scope_carry_forward.connected_provider_root` | `narrower_result` | `review_required` | `chronology_bar.narrower_scope_carry_forward` | Global remembered allow carries forward only as a narrower root-scoped allow. |
| `policy.diff.expired_remembered_decision.policy_epoch_rollover` | `expired_memory` | `warning` | `chronology_bar.expired_remembered_decision_reprompt` | Policy epoch rollover retires remembered memory and requires reprompt. |
| `policy.diff.future_effective_policy.connected_provider` | `future_effective_pending` | `review_required` | `chronology_bar.future_effective_policy_activation` | Preview shows a deny that becomes active later in Europe/Berlin civil time. |
| `policy.diff.legal_hold_interaction.retention_delete` | `blocked_by_hold` | `blocking` | `chronology_bar.legal_hold_delete_block` | Retention delete stays explicitly blocked by a legal hold. |
| `policy.diff.audit_export_chronology.support_bundle` | `exact_change` | `informational` | `chronology_bar.audit_export_mixed_timezones` | Export availability change keeps mixed-timezone ordering and actor truth exportable. |

## Dashboard join contract

The packet and the dashboard contract join on:

- `waiver_id`
- `remembered_decision_id`
- `subject_ref.subject_kind`
- `subject_ref.subject_id`
- `linked_governed_record_refs`
- `policy_epoch`
- `applicable_scope`
- `chronology_bar_case_ref`
- `dashboard_bucket`

These joins let a dashboard detect:

- remembered decisions whose expiry already elapsed;
- carry-forward scopes broader than the actual resulting scope;
- waivers with missing governed-record linkage; and
- blocked-by-hold rows whose blocker id is present in the packet but not
  surfaced in dashboard counts.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.policy_simulation.diff_manifest` | `verification_corpus` | Defines the machine-readable input/output objects and case roster this packet freezes. | current | `fixtures/policy/simulation_diff_manifest.yaml` |
| `evidence.policy.waiver_expiry.dashboard_contract` | `dashboard_contract` | Defines required counts, ageing fields, windows, and drift rules for waivers and remembered decisions. | current | `artifacts/policy/waiver_expiry_dashboard_contract.yaml` |
| `evidence.verification.policy.chronology_bar_cases` | `verification_corpus` | Supplies reviewer-facing chronology-bar rows the packet and manifest cite. | current | `fixtures/policy/chronology_bar_cases/` |
| `evidence.verification.governance.record_state_examples` | `verification_corpus` | Provides the existing schema-conforming baseline examples this packet composes over. | current | `fixtures/governance/record_state_examples/` |

## Verification method

- **Verification classes used:** design review, fixture review, schema-
  alignment review
- **Procedure summary:** verified that the new packet vocabulary reuses
  the frozen governed-record and waiver-expiry schemas, that every
  required scenario has one stable case id, and that chronology-bar
  cases preserve effective time, display timezone, actor, and ordering
  fields needed by future support/export packets.
- **Automation refs:** `not_yet_seeded` for a dedicated policy-corpus
  validator; structural parsing is currently the available automation.

## Known gaps and waivers

- **Waiver refs:** `none`
- **Known-limit refs:** `none`
- **Migration-packet refs:** `none`
- **Explicit gaps:** no live policy-engine execution or admin UI is
  wired to this packet yet.
- **Explicit gaps:** no dedicated JSON Schema exists yet for the
  policy-specific diff manifest or chronology-bar case family.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:** `packet_row:policy_simulation.input_output_contract`, `packet_row:policy_simulation.diff_severity`, `packet_row:policy_simulation.remembered_decision_narrowing`, `packet_row:policy_simulation.waiver_expiry_dashboard_join`, `packet_row:policy_simulation.chronology_bar_export_truth`, `packet_row:policy_simulation.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_fixture_revision_changed`
- **Expected freshness window:** `P30D`
- **Next packet family to update with the same evidence ids:** support
  packet or release packet that starts quoting policy-preview or
  waiver-expiry posture
