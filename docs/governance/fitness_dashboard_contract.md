# Fitness-dashboard tile, threshold-state grammar, and evidence-freshness degradation contract

This contract freezes one vocabulary for rendering protected fitness
functions on the dashboard, the milestone scorecard, the release-evidence
shiproom packet, the support bundle, and the public claim manifest. It
exists so protected metrics and governance checks remain human-readable
without treating stale evidence or waived failures as clean passes.

The contract is pre-implementation. It defines the reusable record shape,
allowed vocabulary combinations, projection rules, export parity, and
fixture corpus. It does not implement a dashboard, a benchmark rerun
automation, or a waiver workflow.

## Companion artifacts

- [`/schemas/governance/fitness_tile.schema.json`](../../schemas/governance/fitness_tile.schema.json)
  — boundary schema for one `fitness_tile_record`.
- [`/artifacts/governance/fitness_state_rows.yaml`](../../artifacts/governance/fitness_state_rows.yaml)
  — machine-readable register of the closed state, threshold-mode,
  evidence-freshness, mitigation, corpus / profile, partial-profile,
  and waiver-authority vocabularies.
- [`/fixtures/governance/fitness_tile_cases/`](../../fixtures/governance/fitness_tile_cases/)
  — worked records covering fresh pass, stale-by-time downgrade,
  stale-by-trigger downgrade, waived blocker, expired waiver, partial-
  profile result, and provisional row.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — the protected fitness-function catalog. Every tile resolves
  `fitness_function_row_ref` to a row in this file.
- [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  and
  [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
  — proof-class freshness ceilings and named rerun triggers. The tile's
  `evidence_freshness` envelope copies metadata from these registers.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — DRI, backup-owner, decision-forum, and waiver register. The tile's
  `owner` envelope and `waiver` envelope resolve into this matrix.

## Normative sources projected here

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §3.4 quality-
  attribute scenarios and measurable SLOs, §22.9 release governance,
  Appendix BB efficiency state policy.
- `.t2/docs/Aureline_Technical_Design_Document.md` §8.36 release
  evidence, §8.41 supportability evidence, Appendix BU verification
  lanes.
- `.t2/docs/Aureline_PRD.md` verification, release-evidence, and
  compatibility-publication requirements.

If this contract disagrees with those sources, those sources win and this
contract, schema, fixtures, and state-rows artifact update in the same
change.

## Why a fitness-dashboard contract exists

1. The protected fitness-function catalog freezes the metric identities,
   thresholds, owners, and waiver authorities. It does not define how
   the live state of those metrics renders onto a dashboard, a
   scorecard, a release packet, a support bundle, or a claim manifest.
   Without one shared rendering contract, every consuming surface
   invents its own chip vocabulary, every release packet projects its
   own state token, and every waiver looks like a clean pass somewhere.
2. Stale evidence has to stay legible. Once `captured_at + stale_after`
   passes or a named rerun trigger fires, the prior numeric pass result
   is no longer authoritative. The tile's state grammar carries that
   downgrade so consuming surfaces cannot keep rendering a fresh-pass
   chip on aged-out evidence.
3. Waived failures have to stay legible. A release-bearing failure held
   open by an active waiver is a real risk being deferred, not a clean
   pass. The tile's `waived` and `waiver_expired` states carry the
   waiver authority and expiry alongside the state token so the waiver
   posture is visible on every surface.
4. Partial-profile results have to stay legible. A measurement captured
   on one OS, one ring, one design-partner workspace, or one lab is not
   the same as a full-corpus, full-profile capture. The
   `partial_profile_result_class` field carries that scope so a partial
   measurement cannot widen silently into a full-corpus claim.
5. Mitigation notes have to stay actionable. Surfaces need a typed
   description of what is narrower, slower, less portable, or
   temporarily blocked, and what users or operators should do
   meanwhile. The `mitigation_note` block carries that as three bounded
   reviewable sentences against a closed mitigation class so generic
   "see release notes" redirects are non-conforming.

## 1. Tile shape

A `fitness_tile_record` carries:

- `tile_id` — stable, machine-readable id quoted by every consuming
  surface.
- `fitness_function_row_ref` — the row id from the protected fitness-
  function catalog (e.g. `ff.input_to_paint`). The catalog is the source
  of truth; a tile whose row id is absent from the catalog is non-
  conforming.
- `fitness_function_family` — the protected SLO family (e.g.
  `input_response`). Surfaces use the family to group rows without re-
  deriving the mapping.
- `tile_state` — one of the six state grammar tokens (§3).
- `headline_label` — bounded reviewable headline label resolved against
  the tile state plus the row id.
- `threshold_snapshot` — typed threshold mode plus bounded reviewable
  threshold and measured labels (§4).
- `evidence_freshness` — typed freshness class plus packet ref,
  `captured_at`, `stale_after`, computed `expires_at`, and (when
  freshness is `stale_by_trigger`) a named rerun-trigger ref (§5).
- `owner` — primary DRI, owning lane, optional co-owning lane, optional
  backup owner, and (when no backup owner is named) a backup-waiver
  ref.
- `corpus_profile_identity` — pinned reference surface (reference
  hardware, design-partner workspace, air-gapped lab, managed SaaS
  ring, or general corpus with no profile pin), corpus refs, profile
  pin ref, and partial-profile class (§6).
- `waiver` — typed waiver authority, active or previous waiver ref, and
  expiry timestamp (§7).
- `mitigation_note` — typed mitigation class plus three bounded
  reviewable sentences naming what users should do, what operators
  should do, and what is narrower, slower, less portable, or
  temporarily blocked (§8).
- `linked_evidence_refs` and `linked_scorecard_lane_refs` — the
  downstream packets and lanes that consume the tile.
- `export_fields` — which surfaces consume the tile (dashboard,
  scorecard, release packet, support export, claim manifest).
- `contract_doc_ref`, `fitness_function_catalog_ref`, `state_rows_ref`
  — fixed cross-references to this contract, the catalog, and the
  state-rows artifact.

## 2. Stable IDs and human-readable copy

Tiles carry both:

- machine-stable IDs — `tile_id`, `fitness_function_row_ref`,
  `evidence_packet_ref`, `active_waiver_ref`, `previous_waiver_ref`,
  `rerun_trigger_ref`, `profile_pin_ref`, scorecard lane refs; and
- bounded reviewable copy — `headline_label`, `threshold_label`,
  `measured_label`, `freshness_summary`, `identity_summary`,
  `waiver_summary`, `what_users_should_do`, `what_operators_should_do`,
  `mitigation_summary`.

Surfaces render the copy verbatim and quote the IDs as refs.
Tooling consumes the IDs and the typed class fields without parsing the
copy. Raw measurement bytes, raw evidence bodies, raw waiver
justifications, and raw user identifiers MUST NOT appear; the record
carries opaque refs, typed vocabulary, and bounded reviewable summaries
only.

## 3. State grammar

The closed six-class state grammar is frozen in
[`/artifacts/governance/fitness_state_rows.yaml#tile_states`](../../artifacts/governance/fitness_state_rows.yaml).

| State | Meaning | Visual / export floor |
| --- | --- | --- |
| `passing` | Threshold met on fresh (or near-expiry) evidence with no live waiver. | Render the state token, the threshold label, and the measured label. |
| `warning` | Early-signal drift on fresh evidence. The threshold has not yet been breached. | Render the warning token, the drift summary, and the named owner inspecting. |
| `blocked` | Threshold breached on fresh evidence with no active waiver. | Render the blocked token, the threshold label, and the typed mitigation note. |
| `waived` | A failure or near-failure is held open by an active waiver. | Render the waived token alongside the waiver authority, the expiry timestamp, and the held-release condition. |
| `waiver_expired` | The waiver expired without renewal. | Render the expired-waiver token alongside the previous waiver ref and the expired timestamp. Never render as passing. |
| `evidence_stale` | The underlying evidence packet expired by time, by named trigger, or is missing. | Render the stale token, the typed freshness class, and (when stale-by-trigger) the named rerun trigger. Never project the prior numeric pass result as authoritative. |

Acceptance conditions enforced by the schema's `allOf` block:

- A waived failure cannot look like a clean pass. `waived` requires a
  non-null `active_waiver_ref`, a future `waiver_expires_on`, and a
  typed waiver authority. The mitigation class is fixed to
  `waiver_holds_release_until_expiry`.
- An expired waiver cannot look like a clean pass. `waiver_expired`
  requires a non-null `previous_waiver_ref`, a past `waiver_expires_on`,
  and the `temporarily_blocked_pending_owner_action` mitigation class.
  Tiles with `previous_waiver_ref` set and `active_waiver_ref` null
  cannot render as `passing` on a release packet or claim manifest.
- Stale evidence cannot retain fresh-pass semantics. `evidence_stale`
  requires `evidence_freshness_class` in `{stale_by_time,
  stale_by_trigger, missing}` and one of the typed mitigation classes
  that explain the narrower / slower / less-portable / partial-profile
  posture.

## 4. Threshold snapshot

`threshold_snapshot` carries the threshold mode (mirroring the catalog's
`threshold_modes` set), the threshold label, the measured label, and a
bounded reviewable summary of the comparison outcome.

Rules:

- `threshold_label` is resolved against the threshold mode. For
  latency rows the label names the p50 / p95 / release bar; for ratio
  rows it names the ratio floor; for boolean rows it names the contract
  assertion that must hold.
- `measured_label` is resolved against the same mode. For
  `tile_state = evidence_stale` the label MUST be prefixed with the
  typed stale marker (e.g. "last seen p50 = ..."); raw measurement
  bytes do not appear.
- `to_be_set_by_benchmark_council` is admissible on provisional rows;
  the tile renders the placeholder verbatim and the mitigation class is
  fixed to `provisional_no_action_until_seeded`.

## 5. Evidence freshness

`evidence_freshness` carries the typed freshness class, the underlying
evidence packet ref, `captured_at`, `stale_after`, the computed
`expires_at`, and (when freshness is `stale_by_trigger`) a named
`rerun_trigger_ref`.

Freshness rules:

- `fresh` and `near_expiry` are admissible on `passing`, `warning`,
  `blocked`, `waived`, and `waiver_expired` tiles. Surfaces MAY decorate
  `near_expiry` with a refresh-soon hint without changing the state
  token.
- `stale_by_time` triggers on `captured_at + stale_after` in the past.
  The tile MUST move to `evidence_stale`.
- `stale_by_trigger` triggers when a named rerun trigger from
  [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
  fires. The tile MUST move to `evidence_stale` even when the time
  window is open. `rerun_trigger_ref` is required.
- `missing` is the admissible class when required metadata is absent or
  when no evidence has been captured yet for a seeded row. Treated as
  expired for tile, scorecard, signoff, and claim use.
- `not_applicable_provisional` is the admissible class for provisional
  rows. `evidence_packet_ref`, `captured_at`, `stale_after`, and
  `expires_at` MUST be null; the tile renders `passing` with the
  `provisional_no_action_until_seeded` mitigation.

## 6. Corpus and profile identity

`corpus_profile_identity` pins the evidence to a named reference surface
so a tile cannot project a result captured on the reference hardware as
if it were captured on an air-gapped lab, on a design-partner workspace,
or on a managed SaaS ring.

Identity classes:

- `reference_hardware` — pinned to a row in
  [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  or
  [`/artifacts/perf/reference_laptop_matrix.yaml`](../../artifacts/perf/reference_laptop_matrix.yaml).
- `design_partner_workspace` — pinned to a recorded design-partner
  workspace fixture.
- `air_gapped_lab` — pinned to an air-gapped lab capture; the captured
  evidence MUST cite the air-gapped profile so the result is not
  silently widened to the connected profile.
- `managed_saas_ring` — pinned to a managed SaaS ring identity row;
  the captured evidence MUST cite the ring so the result is not widened
  across rings.
- `general_corpus_no_profile_pin` — admissible only when the fitness
  function is host-independent and the corpus manifest does not require
  a profile pin (e.g. command-graph parity).

`partial_profile_result_class` is a separate axis. A tile whose evidence
covers only a subset of the declared corpus / profile set MUST render a
partial-profile class other than `not_partial_full_corpus_run`; in that
case the tile_state MUST be one of `warning`, `blocked`, `waived`,
`waiver_expired`, or `evidence_stale`, and the mitigation class MUST be
`partial_profile_result_pending_full_capture` (or, when an active waiver
covers the gap, `waiver_holds_release_until_expiry`).

## 7. Waiver envelope

`waiver` carries the typed waiver authority, the active or previous
waiver ref, the expiry timestamp, and a bounded reviewable summary of
the waiver posture.

Rules enforced by the schema:

- `waived` tiles require a non-null `active_waiver_ref`, a non-null
  `waiver_expires_on` in the future, and a typed waiver authority other
  than `not_applicable_no_active_waiver`. The mitigation class is
  `waiver_holds_release_until_expiry`.
- `waiver_expired` tiles require a non-null `previous_waiver_ref`, a
  past `waiver_expires_on`, and a null `active_waiver_ref`. The
  mitigation class is `temporarily_blocked_pending_owner_action`.
- All other tile states require
  `waiver_authority_class = not_applicable_no_active_waiver` and null
  active / previous waiver refs.
- A tile with `previous_waiver_ref` non-null MUST NOT render as
  `passing` on a release packet or claim manifest unless
  `active_waiver_ref` is also non-null (i.e. the waiver was renewed
  rather than allowed to lapse).

Raw waiver justification text MUST NOT appear; the `waiver_summary` is a
bounded reviewable sentence naming the authority, the expiry, and the
held release condition.

## 8. Mitigation notes

A mitigation note explains what is narrower, slower, less portable, or
temporarily blocked, and what users or operators should do meanwhile.
The note carries:

- one typed `mitigation_note_class` from the closed nine-class
  vocabulary;
- `what_users_should_do` — a bounded reviewable sentence naming the safe
  in-scope action, the safe inspect-only path, the named workaround, or
  the no-action-required state;
- `what_operators_should_do` — a bounded reviewable sentence naming the
  owner handle, the lane, the decision forum, or the named rerun
  trigger; and
- `mitigation_summary` — a bounded reviewable sentence naming what is
  narrower, slower, less portable, or temporarily blocked, plus the
  meanwhile posture for users.

The closed mitigation classes are:

- `no_mitigation_required_passing` — admissible only on `passing`
  tiles.
- `narrower_scope_until_refresh` — claim scope is narrower than the
  row's full corpus / profile coverage until the evidence is
  recaptured.
- `slower_path_active_until_refresh` — a slower path (degraded cache,
  fallback resolution, mirror-only mode) is active until the evidence
  is recaptured.
- `less_portable_temporarily` — the tile is rendered against a narrower
  portability set (one OS, one host class, one ring) until the evidence
  is recaptured.
- `temporarily_blocked_pending_owner_action` — the tile is blocked
  pending a named owner action.
- `waiver_holds_release_until_expiry` — fixed mitigation for `waived`
  tiles.
- `partial_profile_result_pending_full_capture` — the tile is rendered
  against a partial corpus / profile set pending a full capture.
- `provisional_no_action_until_seeded` — admissible only on `passing`
  tiles whose evidence-freshness class is `not_applicable_provisional`.
- `early_signal_drift_owner_inspecting` — admissible only on `warning`
  tiles.

Surfaces consume the typed class plus the three bounded reviewable
sentences verbatim. Free-text fear copy and free-text "see release
notes" redirects are non-conforming.

## 9. Visual and export parity

Every surface that renders the tile MUST render the same fields. The
parity floor is frozen in
[`/artifacts/governance/fitness_state_rows.yaml#visual_export_parity_floor`](../../artifacts/governance/fitness_state_rows.yaml).

Required on every surface:

- `tile_state`, `headline_label`;
- `threshold_snapshot.threshold_mode_class`,
  `threshold_snapshot.threshold_label`,
  `threshold_snapshot.measured_label`;
- `evidence_freshness.evidence_freshness_class`,
  `evidence_freshness.captured_at`,
  `evidence_freshness.expires_at`;
- `owner.primary_dri`, `owner.owning_lane`;
- `corpus_profile_identity.corpus_profile_identity_class`,
  `corpus_profile_identity.partial_profile_result_class`;
- `mitigation_note.mitigation_note_class`,
  `mitigation_note.mitigation_summary`.

Per-state required extras:

- `waived` — `waiver.waiver_authority_class`, `waiver.active_waiver_ref`,
  `waiver.waiver_expires_on`, `waiver.waiver_summary`.
- `waiver_expired` — `waiver.waiver_authority_class`,
  `waiver.previous_waiver_ref`, `waiver.waiver_expires_on`,
  `waiver.waiver_summary`.
- `evidence_stale` — `evidence_freshness.rerun_trigger_ref` (when the
  freshness class is `stale_by_trigger`),
  `evidence_freshness.freshness_summary`.

Forbidden collapses on release-packet and claim-manifest surfaces:

- rendering a waived tile as passing to unblock publication;
- rendering a waiver-expired tile as passing after renewal lapses;
- rendering an evidence-stale tile with a fresh-pass chip;
- dropping `corpus_profile_identity` when widening a claim across
  profiles;
- omitting the named rerun trigger on a stale-by-trigger tile.

## 10. Authoring rules

When a new fitness function row is seeded:

1. Add the row to
   [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
   first; the catalog is the source of truth for owner, threshold,
   data source, cadence, waiver authority, and review rules.
2. Mint the corresponding `tile_id` and project the row's threshold
   into a `threshold_snapshot` with the matching threshold mode.
3. Wire the evidence pipeline so the tile's `evidence_freshness`
   envelope copies metadata from the underlying packet header.
4. Pin the `corpus_profile_identity` to a named reference surface from
   the manifests; if the function is host-independent and the corpus
   manifest does not require a profile pin, use
   `general_corpus_no_profile_pin` and explain the absence in
   `identity_summary`.
5. Resolve `mitigation_note_class` against the live tile state; the
   class is bounded by the schema's `allOf` block, not by reviewer
   preference.
6. Wire the tile's `linked_scorecard_lane_refs` to the lanes whose
   status the tile influences. A tile with no scorecard lane refs is
   non-conforming.

When evidence ages out:

1. The freshness evaluator detects `captured_at + stale_after` in the
   past or a named rerun-trigger match.
2. The tile state moves to `evidence_stale` automatically; the
   `measured_label` is rewritten with the typed stale marker; the
   mitigation class moves to `narrower_scope_until_refresh`,
   `slower_path_active_until_refresh`, `less_portable_temporarily`, or
   `partial_profile_result_pending_full_capture` depending on which
   posture the lane chose for the meanwhile state.
3. Downstream scorecards, claim rows, release packets, support exports,
   and shiproom packets re-render the tile in `evidence_stale` without
   widening any claim back to a fresh-pass posture.

When a waiver is minted, renewed, or expires:

1. A new `waived` tile is emitted with `active_waiver_ref` resolved into
   `ownership_matrix.waivers` and `waiver_expires_on` copied from the
   waiver record.
2. On expiry the tile moves to `waiver_expired` with the previous
   waiver ref preserved; the tile MUST NOT silently revert to
   `passing`.
3. On renewal a new `waived` tile is emitted; the prior tile is closed
   out of the audit stream rather than mutated in place.

## 11. Out of scope

This contract does not implement:

- a live dashboard surface, a benchmark rerun automation, or a waiver
  workflow;
- the catalog rows themselves (those live in
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml));
- the freshness ceilings or rerun-trigger catalog (those live in
  [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  and
  [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml));
- the waiver records themselves (those live in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)).

This contract is the projection vocabulary that those upstream artifacts
flow through when they reach the dashboard, the scorecard, the release
packet, the support bundle, and the claim manifest.
