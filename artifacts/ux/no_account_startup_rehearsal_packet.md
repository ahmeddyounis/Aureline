# No-account local-first startup rehearsal packet

<!--
This packet exists to *prove* — not narrate — that Aureline's
local-first entry path survives first-run, open, import, and
restore without hidden service prerequisites. It is a focused
companion to the broader startup-route rehearsal packet at
[`/artifacts/ux/startup_route_rehearsal_packet.md`](./startup_route_rehearsal_packet.md):

- That packet scores every protected startup route under one
  qualification shape across all deployment profiles, with
  `exact` / `compatible` / `partial` / `failed` outcomes.
- *This* packet narrows the lens to the no-account / local-first
  claim. Each row asks one question: did the user reach
  first-useful-work on this entry verb without account creation,
  sign-in, or managed connectivity?

The packet is the **single shared format** the local-first
rehearsal entry rows are scored under, so reviewers can read
"the local-first claim held on this row" the same way regardless
of which entry verb / target kind / network posture is being
exercised.

Companion artifacts (read-only sources for refs):

- [`/artifacts/ux/first_useful_work_corpus/`](./first_useful_work_corpus/)
  — qualification corpus (one `fuw_row:*` per protected route ×
  case category).
- [`/artifacts/ux/no_account_switching_scoreboard.yaml`](./no_account_switching_scoreboard.yaml)
  — scoreboard the rehearsal results aggregate into.
- [`/artifacts/ux/service_opt_in_boundary_rows.yaml`](./service_opt_in_boundary_rows.yaml)
  — per-surface posture matrix (fully local / degrade gracefully /
  deferred / unavailable until opt-in / unavailable in envelope).
- [`/fixtures/ux/no_account_startup_cases/`](../../fixtures/ux/no_account_startup_cases/)
  — worked rehearsal fixtures keyed by entry verb.
- [`/fixtures/ux/first_useful_work_cases/`](../../fixtures/ux/first_useful_work_cases/)
  — broader first-useful-work fixtures the rehearsal cites by
  reference.
- [`/docs/ux/no_account_local_entry_contract.md`](../../docs/ux/no_account_local_entry_contract.md)
  — frozen contract for entry-surface rows and account-prompt
  records.
- [`/artifacts/governance/deployment_profiles.yaml`](../governance/deployment_profiles.yaml)
  — `deployment_profile_id` register.
- [`/docs/product/boundary_manifest_strawman.md`](../../docs/product/boundary_manifest_strawman.md)
  — `boundary_manifest_row_id` source.

Rules (frozen):

1. The packet does NOT mint new vocabulary. Every typed field
   resolves through a closed set already frozen upstream — see
   the corpus README §1, the no-account contract §3, the
   service-opt-in boundary rows top-matter, or the rehearsal-
   packet template §0.
2. The packet does NOT widen rehearsal qualification classes
   beyond `{exact, compatible, partial, failed}`. A row that
   needs a new class opens a decision row in
   [`/artifacts/governance/decision_index.yaml`](../governance/decision_index.yaml)
   instead of widening this template.
3. The packet does NOT replace the broader startup-route
   rehearsal packet. A reviewer running the broader packet may
   cite this packet's rows by ref instead of re-filling them; a
   reviewer running this packet feeds its findings back through
   the broader packet's §7 findings table when the local-first
   claim is at risk.
4. Milestone or task identifiers MUST NOT appear in any field,
   slug, or notes string. Planning metadata is removed before
   publication.
-->

## 0. Packet header

Every no-account rehearsal packet SHOULD embed this header
verbatim. Fields without a stable ref use the closed
vocabularies named in the corpus README §1 or the no-account
local-entry contract.

- **Packet id:** `<no-account-startup-rehearsal-packet-id>`
- **Packet state:** `draft` | `in_review` | `accepted` |
  `blocked` | `superseded`
- **Reviewer (DRI):** `@<handle>`
- **Co-reviewer:** `@<handle>` or `none`
- **Rehearsal date:** `YYYY-MM-DDTHH:MM:SSZ`
- **Build identity:** `<exact_build_identity_ref>` from
  `schemas/build/exact_build_identity.schema.json`.
- **Deployment profile under test:** one of
  `individual_local` | `self_hosted` | `enterprise_online` |
  `air_gapped` | `managed_cloud` | `air_gapped_mirror_only` from
  the deployment-profile register.
- **Network posture:** `online` | `offline` | `mirror_only`.
- **Sign-in posture:** `no_sign_in` |
  `sign_in_offered_skipped` | `sign_in_offered_completed` |
  `sign_in_required_completed` | `sign_in_required_skipped`.
- **Service opt-in posture:** `none_offered` |
  `offered_all_declined` | `offered_some_accepted` |
  `offered_all_accepted`.
- **Entry verbs in scope:** subset of the closed set in §1.
- **Rehearsal-fixture refs:** list of fixture paths under
  `/fixtures/ux/no_account_startup_cases/` and / or
  `/fixtures/ux/first_useful_work_cases/`.
- **Corpus rows in scope:** list of `fuw_row:*` ids.
- **Boundary rows in scope:** list of
  `service_opt_in_boundary_row:*` ids from
  [`/artifacts/ux/service_opt_in_boundary_rows.yaml`](./service_opt_in_boundary_rows.yaml).
- **Scoreboard rows in scope:** list of `scoreboard_row:*` ids.
- **Active waiver packet refs:** waiver ids or `none`.
- **Notes:** one-paragraph reviewer comment; never a substitute
  for a typed field.

## 1. Entry verbs the packet exercises

The packet exercises **six** entry verbs the local-first claim
binds to. Each verb resolves to one row in §3 below; any row
the rehearsal does not exercise is recorded as `not_attempted`
(not omitted). The closed set:

1. `fresh_install_first_run` — new device, no prior profile, no
   prior recent work. Renders `startup_state:first_run`.
2. `open_local_folder` — palette / Start Center / OS file
   association open of a local folder. Resolves through
   `er.start_center` or `er.plain_open`.
3. `open_local_workspace` — open of a multi-root local workspace
   manifest (`.code-workspace`, multi-folder file).
4. `restore_previous_local_session` — `er.restore_prompt` after
   crash or warm-start with restore-card present, where the
   user accepts restore on a local-only session.
5. `import_local_pack` — import of a local portable-state
   package, handoff packet, competitor-config root, or template
   / prebuild snapshot via `er.clone_or_import`. The pack is
   already on disk; no remote fetch is performed.
6. `decline_sign_in_or_service_opt_in` — first-run or first-open
   surfaces a managed sign-in card or a non-identity service
   opt-in (telemetry, model gateway, marketplace browse, sync,
   collaboration relay), the user declines, and the local flow
   stays useful.

Rules (frozen):

- A row whose entry verb is `fresh_install_first_run` MUST
  render `startup_state_token = startup_state:first_run` and
  MUST resolve `account_prompt_class = no_prompt` on every
  primary action it exercises.
- A row whose entry verb is `import_local_pack` MUST set
  `boundary_crossing_class` to one of `no_boundary_crossed`,
  `widens_trust` (only if the user explicitly trusts the pack
  source after review), or `attaches_external_runtime` (only if
  the pack triggers a devcontainer attach the user reviewed
  before commit). Pack import that silently reads provider
  state, opens a browser handoff, or emits managed telemetry is
  non-conforming on this packet.
- A row whose entry verb is `decline_sign_in_or_service_opt_in`
  MUST cite at least one declined boundary row and MUST report
  `local_first_claim_held` in §4 below; rows that never present
  an opt-in cannot exercise this verb.

## 2. Executive summary

Two or three sentences naming:

1. how many rows were exercised in this packet,
2. how many scored `exact`, `compatible`, `partial`, or `failed`,
3. whether any row violated its `local_first_claim_class` (the
   row's declared shape from
   [`fuw_row:* local_first_claim_class`](./first_useful_work_corpus/README.md))
   or its boundary row's `surface_posture_class` (from the
   service-opt-in boundary rows top-matter).

This summary feeds the no-account switching scoreboard's
`fuw_no_account_by_route` and `fuw_service_opt_in_decline_outcomes`
families without restating per-row thresholds.

## 3. Per-entry-verb rehearsal table

Reviewers fill **one row per entry verb in scope**. Every column
resolves to a closed vocabulary. Free-text goes only in the
`notes` column at the end of each row.

| Field | Vocabulary / source |
|---|---|
| `entry_verb_class` | §1 closed set |
| `corpus_row_ref` | `fuw_row:*` from `artifacts/ux/first_useful_work_corpus/`; or `not_attempted` |
| `case_category` | corpus README §1.3 |
| `entry_route_id` | onboarding measurement plan §4 |
| `deployment_profile_id` | deployment-profile register |
| `startup_state_token` | startup-state copy-review `startup_state_token_vocabulary` |
| `network_posture` | `{online, offline, mirror_only}` |
| `account_prompt_class` | no-account contract §3.2 |
| `account_prompt_timing_class` | no-account contract §3.3 |
| `boundary_crossing_class` | no-account contract §3.4 |
| `expected_blocker_class` | corpus README §1.1 |
| `observed_blocker_class` | corpus README §1.1 (`no_blocker` if none) |
| `expected_qualification_class` | `{exact, compatible, partial, failed}` |
| `observed_qualification_class` | `{exact, compatible, partial, failed}` or `not_attempted` |
| `time_to_first_useful_work_class` | `{within_envelope, within_envelope_after_safe_exit, outside_envelope, not_reached, not_attempted}` |
| `local_first_claim_held` | `{yes, narrowed_advertised, account_required_continue_local_offered, no_violation_recorded, not_attempted}` |
| `safe_exit_action_invoked` | `next_step_decision_hook` value or `none_required` or `not_attempted` |
| `declined_opt_ins` | list of `boundary_manifest_row_id` (empty when no opt-in fired) |
| `service_opt_in_boundary_row_refs` | list of `service_opt_in_boundary_row:*` ids the verb exercised |
| `notes` | free-text rationale; must not mint state names |

A row whose `observed_qualification_class` differs from the
corpus row's declared `rehearsal_qualification_class` is non-
conforming unless the difference resolves to a typed
`observed_blocker_class` the row was not predicted to encounter
— in which case the row is filed as a packet-level finding (§6).
A row whose `local_first_claim_held` is
`no_violation_recorded` MUST raise at least one packet-level
finding in §6 regardless of the per-row qualification score.

### 3.1 Empty row template

Copy and fill one block per entry verb in scope.

```yaml
- entry_verb_class: <fresh_install_first_run|open_local_folder|open_local_workspace|restore_previous_local_session|import_local_pack|decline_sign_in_or_service_opt_in>
  corpus_row_ref: fuw_row:<case_category>.<slug>
  case_category: <case_category>
  entry_route_id: <er.*>
  deployment_profile_id: <profile>
  startup_state_token: <startup_state:*>
  network_posture: <online|offline|mirror_only>
  account_prompt_class: <no_prompt|optional_prompt|deferrable_prompt|required_prompt|policy_forced_prompt|unavailable_prompt|not_applicable>
  account_prompt_timing_class: <closed-set value from no-account contract §3.3>
  boundary_crossing_class: <closed-set value from no-account contract §3.4>
  expected_blocker_class: <blocker_class>
  observed_blocker_class: <blocker_class>
  expected_qualification_class: <exact|compatible|partial|failed>
  observed_qualification_class: <exact|compatible|partial|failed|not_attempted>
  time_to_first_useful_work_class: <within_envelope|within_envelope_after_safe_exit|outside_envelope|not_reached|not_attempted>
  local_first_claim_held: <yes|narrowed_advertised|account_required_continue_local_offered|no_violation_recorded|not_attempted>
  safe_exit_action_invoked: <next_step_decision_hook | none_required | not_attempted>
  declined_opt_ins: []
  service_opt_in_boundary_row_refs: []
  notes: >
    <one-paragraph rationale; closed-vocab fields above are the
    actual record of truth>
```

## 4. Local-first claim qualification

Reviewers state whether Aureline preserved a credible local-
first path on every exercised row when sign-in or managed
services were absent or declined.

| Field | Vocabulary / source |
|---|---|
| `account_present_during_rehearsal` | `{none, offered_skipped, offered_completed, required_completed, required_skipped}` |
| `managed_service_reachable` | `{none_required, all_reachable, some_unreachable, none_reachable}` |
| `network_posture_observed` | `{online, offline, mirror_only}` |
| `local_first_floor_held` | `yes` / `narrowed_advertised` / `no_with_typed_reason` |
| `narrowing_advertised_before_commit` | `yes` / `no` / `not_applicable` |
| `decline_path_offered_same_weight` | `yes` / `no` / `not_applicable` |
| `retroactive_degradation_observed` | `yes` / `no` |
| `account_or_marketplace_prompt_above_primary_work_resume` | `yes` / `no` |
| `summary` | one paragraph naming every (`fuw_row:*`, `local_first_claim_class`) pair the rehearsal exercised and how it resolved |

Rules (frozen):

1. A rehearsal that records
   `retroactive_degradation_observed: yes` MUST raise at least
   one packet-level finding in §6.
2. A rehearsal that records
   `account_or_marketplace_prompt_above_primary_work_resume: yes`
   MUST raise at least one packet-level finding in §6 with
   `finding_class = account_prompt_above_primary_work_resume`.
3. A rehearsal that records `local_first_floor_held:
   no_with_typed_reason` MUST cite the closed-set typed reason
   and the corpus row that owns the violated claim.

## 5. Surface-posture roll-up

For each entry verb the rehearsal exercised, fill one block. The
block aggregates the §3 table into the format the service-opt-in
boundary rows file consumes by ref.

```yaml
- entry_verb_class: <verb>
  exercised_surfaces:
    - service_opt_in_boundary_row_ref: service_opt_in_boundary_row:<...>
      observed_surface_posture_class: <fully_local|degrade_gracefully|deferred|unavailable_until_opt_in|unavailable_in_envelope>
      declared_surface_posture_class: <same set; resolved from boundary rows file>
      consistent: <yes|no>
      notes: >
        <one-paragraph reviewer rationale>
  rolled_up_local_first_claim_held: <yes|narrowed_advertised|account_required_continue_local_offered|no_violation_recorded>
  rolled_up_qualification_class: <exact|compatible|partial|failed>
```

A roll-up block whose `consistent: no` for any row MUST raise
at least one packet-level finding in §6 with
`finding_class = surface_posture_drift`.

## 6. Packet-level findings

Every finding resolves to one of three outcomes: `pass`,
`narrowed_pass`, or `fail`. A `narrowed_pass` is permitted only
when narrowing was advertised before commit per the cited
corpus row or boundary row.

```yaml
- finding_id: <packet-id>.finding.<short-slug>
  finding_class: <forced_sign_in_before_useful_local_work
                  | network_required_for_local_entry
                  | resulting_mode_silently_downgraded
                  | restore_level_promised_higher_than_delivered
                  | rollback_checkpoint_missing
                  | retroactive_degradation_after_decline
                  | hidden_setup_post_apply
                  | startup_route_drift_unadvertised
                  | account_prompt_above_primary_work_resume
                  | marketplace_prompt_above_primary_work_resume
                  | surface_posture_drift
                  | service_dependence_overstated_in_docs
                  | other>
  outcome: <pass|narrowed_pass|fail>
  related_corpus_row_refs:
    - fuw_row:<case_category>.<slug>
  related_boundary_row_refs:
    - service_opt_in_boundary_row:<...>
  related_scoreboard_row_refs:
    - scoreboard_row:<...>
  remediation_owner_lane: <ownership-matrix lane id>
  remediation_target: <next packet | next decision row | next milestone review>
  notes: >
    <reviewer rationale; closed-vocab fields above are the
    record of truth>
```

A packet that lists any `fail` finding cannot be marked
`accepted` and is routed to the lane named in
`remediation_owner_lane`. A packet that lists a finding under
`service_dependence_overstated_in_docs` MUST also reopen the
docs / help / support row the overstated copy lives on, so
docs and support packets cannot accidentally overstate service
dependence on a claimed local row.

## 7. Account-free proof

The packet's account-free claim is held only when **every** §3
row that exercised an entry verb on a deployment profile that
declares a no-account floor records:

1. `account_prompt_class` in `{no_prompt, optional_prompt,
   deferrable_prompt, unavailable_prompt}`. (`required_prompt`
   and `policy_forced_prompt` are permitted only when the row's
   `local_first_claim_class = account_required_route` and the
   row's `decline_path_class` is one of
   `continue_local_same_weight`,
   `continue_in_restricted_mode_same_weight`, or
   `roll_back_import_available`.)
2. `boundary_crossing_class = no_boundary_crossed` when the
   row's `account_prompt_class = no_prompt`. A row that claims
   `no_prompt` and reports any other crossing class MUST raise
   a packet-level finding under
   `finding_class = forced_sign_in_before_useful_local_work` (or
   the more specific `service_dependence_overstated_in_docs`
   when the disclaimer drift is documentation-only).
3. `decline_path_offered_same_weight = yes` for every row that
   exercised the `decline_sign_in_or_service_opt_in` verb. The
   marketplace, telemetry, model-gateway, sync, identity, and
   collaboration-relay opt-ins MUST stay secondary to local
   work on the rehearsed surfaces.
4. `account_or_marketplace_prompt_above_primary_work_resume =
   no`. The Start Center contract's zone rule (`primary_work_resume`
   sits above `secondary_entry` and `account_opt_in_card`)
   holds for every rehearsed surface.

Reviewers state the account-free claim as a single block at
the end of the packet:

```yaml
account_free_proof:
  rows_exercised: <integer>
  rows_passing: <integer>
  rows_with_narrowed_pass: <integer>
  rows_failing: <integer>
  rows_not_attempted: <integer>
  account_free_claim_held: <yes | narrowed_advertised | no_with_typed_reason>
  marketplace_or_account_pages_secondary_to_local_work: <yes | no>
  managed_service_offers_secondary_to_local_work: <yes | no>
  summary: >
    <one paragraph naming every entry verb the rehearsal
    exercised and whether the account-free claim held on each>
```

A packet whose `account_free_claim_held: no_with_typed_reason`
MUST raise at least one `fail` finding in §6 and cannot be
marked `accepted`.

## 8. Worked example (informational)

The following sketch shows a minimal packet covering one row
per entry verb on `individual_local`. It is illustrative, not
exhaustive; a real packet exercises every verb in scope on at
least the profile the lane owns.

```yaml
packet_id: no-account-local-first-rehearsal-bootstrap-0001
packet_state: draft
reviewer_dri: "@ahmedyounis"
co_reviewer: none
rehearsal_date: "2026-05-04T00:00:00Z"
build_identity: build-identity-seed-no-account-rehearsal-example
deployment_profile_under_test: individual_local
network_posture: online
sign_in_posture: no_sign_in
service_opt_in_posture: offered_all_declined

executive_summary: >
  Six entry verbs exercised on individual_local. Five rows
  scored exact or compatible; one row (decline_sign_in_or_service_opt_in
  on telemetry) scored compatible because narrowing was
  advertised before the decline. No row violated its
  local_first_claim_class; no row presented an account or
  marketplace prompt above primary_work_resume.

per_entry_verb_rehearsal_table:
  - entry_verb_class: fresh_install_first_run
    corpus_row_ref: fuw_row:local_open.first_run_start_center_local_folder
    case_category: local_open
    entry_route_id: er.start_center
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    network_posture: online
    account_prompt_class: no_prompt
    account_prompt_timing_class: never_shown
    boundary_crossing_class: no_boundary_crossed
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: exact
    observed_qualification_class: exact
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: none_required
    declined_opt_ins: []
    service_opt_in_boundary_row_refs: []
    notes: >
      Start Center first-run reached first-useful-edit on
      individual_local with no opt-in card, no managed sign-in
      prompt, and no network call beyond OS DNS / time sync.

  - entry_verb_class: open_local_folder
    corpus_row_ref: fuw_row:local_open.plain_open_unknown_archetype
    case_category: local_open
    entry_route_id: er.plain_open
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    network_posture: online
    account_prompt_class: no_prompt
    account_prompt_timing_class: never_shown
    boundary_crossing_class: no_boundary_crossed
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: exact
    observed_qualification_class: exact
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: none_required
    declined_opt_ins: []
    service_opt_in_boundary_row_refs: []
    notes: >
      File > Open / palette open of a previously-unrecognised
      folder. Archetype detection unavailable; the row reaches
      first-useful-work on the workspace tree.

  - entry_verb_class: open_local_workspace
    corpus_row_ref: not_attempted
    case_category: local_open
    entry_route_id: er.start_center
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    network_posture: online
    account_prompt_class: no_prompt
    account_prompt_timing_class: never_shown
    boundary_crossing_class: no_boundary_crossed
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: exact
    observed_qualification_class: not_attempted
    time_to_first_useful_work_class: not_attempted
    local_first_claim_held: not_attempted
    safe_exit_action_invoked: not_attempted
    declined_opt_ins: []
    service_opt_in_boundary_row_refs: []
    notes: >
      Multi-root local workspace open is reserved on this
      deployment profile; row reserved until the multi-root
      fixture seeds. No corpus row is filed; this row does not
      gate the account-free proof.

  - entry_verb_class: restore_previous_local_session
    corpus_row_ref: fuw_row:restore.compatible_restore_after_crash
    case_category: restore
    entry_route_id: er.restore_prompt
    deployment_profile_id: individual_local
    startup_state_token: startup_state:reopen_with_pending_restore
    network_posture: online
    account_prompt_class: no_prompt
    account_prompt_timing_class: never_shown
    boundary_crossing_class: no_boundary_crossed
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: compatible
    observed_qualification_class: compatible
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: narrowed_advertised
    safe_exit_action_invoked: open_without_restore
    declined_opt_ins: []
    service_opt_in_boundary_row_refs: []
    notes: >
      Restore prompt advertised compatible_restore before
      commit; user picked Restore. No mutating commands re-ran
      silently. Open without restore was offered same-weight.

  - entry_verb_class: import_local_pack
    corpus_row_ref: fuw_row:import.handoff_packet_inspect_only_no_apply
    case_category: import
    entry_route_id: er.clone_or_import
    deployment_profile_id: individual_local
    startup_state_token: startup_state:open_without_restore
    network_posture: offline
    account_prompt_class: no_prompt
    account_prompt_timing_class: never_shown
    boundary_crossing_class: no_boundary_crossed
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: exact
    observed_qualification_class: exact
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: none_required
    declined_opt_ins: []
    service_opt_in_boundary_row_refs: []
    notes: >
      Inspect-only handoff-packet import on a device with the
      network disabled. No remote fetch was attempted; the
      review sheet rendered the artifact_class and
      inspect_only path before any durable write.

  - entry_verb_class: decline_sign_in_or_service_opt_in
    corpus_row_ref: fuw_row:service_opt_in_declined.telemetry_optional_at_first_run
    case_category: service_opt_in_declined
    entry_route_id: er.start_center
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    network_posture: online
    account_prompt_class: optional_prompt
    account_prompt_timing_class: shown_at_first_run_declinable
    boundary_crossing_class: no_boundary_crossed
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: compatible
    observed_qualification_class: compatible
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: set_up_later
    declined_opt_ins:
      - telemetry_support_pipeline
    service_opt_in_boundary_row_refs:
      - service_opt_in_boundary_row:telemetry_first_run_individual_local
    notes: >
      Telemetry opt-in offered same-weight with Continue local;
      user declined. Local bundles remain the canonical
      supportability artefact; no retroactive degradation
      observed.

local_first_claim_qualification:
  account_present_during_rehearsal: offered_skipped
  managed_service_reachable: none_required
  network_posture_observed: online
  local_first_floor_held: yes
  narrowing_advertised_before_commit: yes
  decline_path_offered_same_weight: yes
  retroactive_degradation_observed: no
  account_or_marketplace_prompt_above_primary_work_resume: no
  summary: >
    On individual_local, the no-account floor held across all
    five exercised rows. Continue local was offered same-weight
    with every opt-in card; declines preserved first-useful-edit;
    restore narrowing was advertised before commit.

surface_posture_rollup:
  - entry_verb_class: decline_sign_in_or_service_opt_in
    exercised_surfaces:
      - service_opt_in_boundary_row_ref: service_opt_in_boundary_row:telemetry_first_run_individual_local
        observed_surface_posture_class: degrade_gracefully
        declared_surface_posture_class: degrade_gracefully
        consistent: yes
        notes: >
          Declining telemetry left local bundles canonical;
          posture matched the declared row.
    rolled_up_local_first_claim_held: yes
    rolled_up_qualification_class: compatible

findings: []

account_free_proof:
  rows_exercised: 5
  rows_passing: 5
  rows_with_narrowed_pass: 1
  rows_failing: 0
  rows_not_attempted: 1
  account_free_claim_held: yes
  marketplace_or_account_pages_secondary_to_local_work: yes
  managed_service_offers_secondary_to_local_work: yes
  summary: >
    Account-free first-useful-work was reached on every
    exercised verb. Marketplace, telemetry, sync, model-gateway,
    and managed sign-in cards stayed below primary_work_resume
    on every rendered surface; declining any of them did not
    degrade prior local capability.

packet_state_at_close: accepted
```

## 9. How this packet feeds downstream artifacts

When the packet reaches `accepted` state, the assembling lane:

1. Updates each cited
   [`scoreboard_row:fuw_*`](./no_account_switching_scoreboard.yaml)
   row's supporting fixture refs and
   `latest_observed_qualification_class` (when this field is
   present).
2. Files the per-entry-verb table from §3 into the lane's UX
   evidence packet for the milestone review.
3. Cross-references any §4 narrowing that crossed the
   `local_first_claim_class` boundary into the
   [no-account local-entry contract](../../docs/ux/no_account_local_entry_contract.md)
   review queue.
4. Cross-references any §6 finding under
   `service_dependence_overstated_in_docs` into the docs / help
   / support packet review queue, so docs and support cannot
   accidentally overstate service dependence on a claimed
   local row.
5. Files §6 findings against the
   [decision index](../governance/decision_index.yaml)
   when the finding is `fail` and reopens the cited corpus
   row under
   [`/artifacts/ux/first_useful_work_corpus/`](./first_useful_work_corpus/)
   when the row's declared `rehearsal_qualification_class`
   needs to narrow.

## 10. Schema references

- Closed vocabularies for every typed field in this packet:
  [`/artifacts/ux/first_useful_work_corpus/README.md`](./first_useful_work_corpus/README.md).
- Service-opt-in boundary posture rows (per-surface matrix):
  [`/artifacts/ux/service_opt_in_boundary_rows.yaml`](./service_opt_in_boundary_rows.yaml).
- Account-prompt / boundary-crossing / portability vocabulary:
  [`/docs/ux/no_account_local_entry_contract.md`](../../docs/ux/no_account_local_entry_contract.md).
- Scoreboard the packet feeds:
  [`/artifacts/ux/no_account_switching_scoreboard.yaml`](./no_account_switching_scoreboard.yaml).
- Audited startup-state copy-review:
  [`/artifacts/ux/startup_state_copy_review.yaml`](./startup_state_copy_review.yaml).
- Entry / restore object model:
  [`/docs/workspace/entry_restore_object_model.md`](../../docs/workspace/entry_restore_object_model.md).
- Onboarding measurement plan:
  [`/docs/product/onboarding_measurement_plan.md`](../../docs/product/onboarding_measurement_plan.md).
- Boundary manifest:
  [`/docs/product/boundary_manifest_strawman.md`](../../docs/product/boundary_manifest_strawman.md).
- Deployment-profile register:
  [`/artifacts/governance/deployment_profiles.yaml`](../governance/deployment_profiles.yaml).
- Rehearsal fixtures:
  [`/fixtures/ux/no_account_startup_cases/`](../../fixtures/ux/no_account_startup_cases/).
- Broader startup-route rehearsal packet:
  [`/artifacts/ux/startup_route_rehearsal_packet.md`](./startup_route_rehearsal_packet.md).
