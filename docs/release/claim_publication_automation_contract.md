# Claim-publication automation, evidence-binding, and stale-claim fail-gate contract

This contract freezes one shared vocabulary for generating public truth
from the claim manifest and current evidence rather than copying it by
hand into each publication channel. It exists so "what claim is the
product publishing today, and what is about to change?" stops being a
manual sweep across docs PRs, Help/About panes, About-pane copy review,
service-health banners, release-notes drafts, CLI/help text, evaluation
artifacts, workflow-bundle and certification badges, known-limits
destinations, and public-proof packets, and becomes one inspectable
binding per claim row plus one bounded publication-diff per run.

The publication-binding record is the projection that the publication
generator, the publication-diff reviewer, the release-evidence packet,
the docs / Help / About / service-health renderers, the support-export
builder, the CLI/help text emitter, the evaluation-artifact builder,
the workflow-bundle and certification-badge renderers, and the
public-proof packet writer all read so they answer the same question
with the same row identities, the same evidence-binding rules, the same
diff vocabulary, and the same fail-gate verdict.

Individual binding rows ride on the boundary schema
[`/schemas/governance/claim_publication_binding.schema.json`](../../schemas/governance/claim_publication_binding.schema.json);
the CI gate policy that consumes them lives at
[`/ci/claim_publication_gate.yaml`](../../ci/claim_publication_gate.yaml);
worked cases live in
[`/fixtures/governance/claim_publication_cases/`](../../fixtures/governance/claim_publication_cases).

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the evidence-binding rules, the
publication-diff vocabulary, the fail-gate vocabulary, the export-parity
floor, and the fixture corpus. It does not implement a documentation
site, a release-note rendering pipeline, or a live publication
generator.

## Companion artifacts

- [`/schemas/governance/claim_publication_binding.schema.json`](../../schemas/governance/claim_publication_binding.schema.json)
  — boundary schema for one `claim_publication_binding_record`.
- [`/ci/claim_publication_gate.yaml`](../../ci/claim_publication_gate.yaml)
  — CI gate policy enumerating the closed blocking-failure vocabulary,
  the fail-closed-by-default rule, and the merge / promotion gates.
- [`/fixtures/governance/claim_publication_cases/`](../../fixtures/governance/claim_publication_cases)
  — worked publication cases covering passed, narrowed-pass,
  blocked-widening, and fail-closed verdicts across docs / Help / About
  / service-health / release-notes / CLI/help / evaluation /
  workflow-bundle and certification badge / known-limits destinations.
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  and
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — canonical claim-row publication contract. Every binding cites a
  `claim_row_ref` that resolves into the claim manifest baseline and
  inherits the row's `effective_claim_posture`, `known_limit_refs`,
  `exclusion_note_refs`, `evidence_links`, and `channel_bindings`
  vocabulary verbatim.
- [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
  — channel-by-channel claim-row projection rules. The publication
  binding reads the parity matrix to determine which channels are
  required to project each claim row and which fields they must carry.
- [`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml)
  — propagation rules and worst-supporting-truth-wins composition. The
  publication binding inherits the composition rule and the
  forbidden-generic-copy gate verbatim.
- [`/docs/public_truth/help_about_service_health_audit_packet.md`](../public_truth/help_about_service_health_audit_packet.md)
  and
  [`/schemas/public_truth/public_drift_item.schema.json`](../../schemas/public_truth/public_drift_item.schema.json)
  — public-truth audit packet and public-drift ledger. The publication
  binding cites any open drift items that touch its claim row through
  `linked_artifact_families.public_drift_item_refs[]` so a drift case
  open against the row blocks widening at this gate.
- [`/docs/release/release_notes_whats_new_service_health_contract.md`](./release_notes_whats_new_service_health_contract.md)
  and
  [`/schemas/release/whats_new_card.schema.json`](../../schemas/release/whats_new_card.schema.json)
  — release-notes / what's-new / service-health communication
  contract. The publication binding cites what's-new card row refs for
  release-notes destinations.
- [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  and
  [`/schemas/docs/help_pane_state.schema.json`](../../schemas/docs/help_pane_state.schema.json)
  — destination-descriptor and help-pane-state schemas. The publication
  binding cites destination descriptor row refs and help-pane state
  refs for docs / Help / About / service-health destinations.
- [`/docs/governance/waiver_register_contract.md`](../governance/waiver_register_contract.md)
  and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)
  — waiver register. A binding held open under a release-blocking
  fail-closed verdict cites the waiver register entry that holds the
  release.

If this contract disagrees with those companion sources, the schemas
and the claim-manifest contract win and this contract, the schema, the
CI gate, and the fixtures update in the same change.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` — public-truth, claim-manifest, docs /
  Help / About / service-health, release-notes, known-limits,
  workflow-bundle / certification, and public-proof requirements (RFC
  2119 MUST / SHOULD language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — claim-row,
  publication-binding, destination-descriptor, and public-proof record
  shapes.
- `.t2/docs/Aureline_Technical_Design_Document.md` — release-evidence,
  service-health, supportability-evidence, and certified-archetype
  record shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — Help/About / service-
  health / release-notes / known-limits / badge disclosure rules.

## 1. Why this contract exists

1. **Channel-local copy drifts.** Without one publication automation
   contract, "what does the product claim about this surface today?"
   lives as a paragraph in a docs PR, a hand-typed line in a release-
   notes draft, a pasted string in a Help/About pane, a separate copy
   block in CLI `--help`, and a third paragraph in a public-proof
   packet appendix. The reviewer cannot tell whether all five rows
   came from the same claim row, whether one channel widened past the
   evidence, or whether one channel forgot a required caveat. The
   binding exists so every consuming surface generates from the same
   typed binding row.
2. **Evidence binding must be typed.** A claim row carries evidence
   links, freshness expectations, scope expectations, support-class
   posture, and known-limit refs. A publication automation that
   ignores any of those fields produces silently optimistic copy when
   the underlying evidence is stale, partial, or scoped narrower than
   the claim text. The binding pins one typed evidence-binding rule
   envelope plus one typed evidence-resolution row per evidence input
   so the publication-diff reader can see exactly which evidence drove
   the verdict.
3. **The fail gate must be closed and inspectable.** Orphaned claim
   text, missing evidence rows, broader wording than current proof,
   stale certified or managed-approved badge states, and destinations
   missing required caveats or support-window language are not
   warnings — they are fail-closed states. The binding pins a typed
   gate verdict with a closed blocking-failure vocabulary so a
   reviewer can read the gate's verdict mechanically.
4. **Publication diffs must be reproducible.** The publication-diff
   exists so reviewers can compare old and new published claim sets
   from one generated diff instead of reading every destination
   manually. The binding pins the previous and current publication
   snapshot refs and emits one typed per-channel change row per
   destination so the diff is reproducible from the binding alone.
5. **Automatic narrowing must be explicit.** When evidence is stale,
   partial, or scoped narrower than the claim, the publication
   automation MUST narrow the published copy rather than ship the
   declared claim text. The binding pins one typed
   `narrowing_action_class` so the narrowing is visible to reviewers
   and matches the claim manifest's stale-behavior vocabulary.
6. **Audience and redaction are explicit.** Engineering, support,
   enterprise audit, release readiness, and public-proof reviewers
   all consume the same binding body. The schema pins the typed
   audience and redaction profile so a public-proof export does not
   include support-only payload refs, and a support handoff does not
   include enterprise-only contract churn.
7. **One binding drives every consuming surface.** The publication
   generator, the publication-diff reviewer, the release-evidence
   packet, the docs / Help / About / service-health renderers, the
   support-export builder, the CLI/help text emitter, the
   evaluation-artifact builder, the workflow-bundle / certification
   badge renderer, and the public-proof packet writer read the same
   `claim_publication_binding_record`. A surface that re-authors the
   binding, takes a screenshot of the publication generator's output,
   or maintains a parallel narrative copy block is non-conforming.

## 2. Binding shape

A `claim_publication_binding_record` carries:

- one stable `binding_id`;
- one stable `publication_run_id` shared by every binding written
  against the same run;
- one `claim_row_ref` resolving into the claim manifest baseline;
- one `claim_manifest_baseline_ref` (the claim-manifest packet id whose
  rows the binding was resolved against);
- one `evaluated_at` RFC 3339 UTC timestamp;
- the typed `declared_claim_posture` and `effective_claim_posture`
  (mirroring the claim row);
- the typed `evidence_binding_rules` envelope (§4);
- the typed `evidence_resolution_rows[]` set (§5);
- the typed `destination_publication_rows[]` set (§6);
- the typed `publication_diff` envelope (§7);
- the typed `gate_verdict` envelope (§8);
- the typed `audience_class` plus matching `redaction_profile` (§11);
- the typed `linked_artifact_families` floor (§12);
- the typed `consuming_surface_parity` floor (§9);
- bounded reviewable `headline_label` and `publication_binding_summary`.

Raw page-body bytes, raw release-note prose, raw screenshots, raw user
identifiers, and raw support-case bodies MUST NOT appear; the record
carries opaque refs, typed vocabulary, and bounded reviewable summaries
only.

The binding is reprojected when:

- the claim manifest packet baseline advances;
- a claim row's `effective_claim_posture`,
  `active_downgrade_reasons[]`, `evidence_links[]`,
  `known_limit_refs[]`, or `channel_bindings` entry moves;
- an evidence packet referenced by an evidence-resolution row moves
  through a freshness or scope class;
- a destination descriptor, what's-new card, help-pane state,
  workflow-bundle, or certification badge referenced by a
  destination-publication row moves;
- a public-drift item open against the claim row transitions through
  any ledger state class; or
- the publication run is rerun against the same baseline (e.g. a
  rebuild with rotated keys).

The reprojection MUST advance `evaluated_at` and recompute the gate
verdict.

## 3. Channel vocabulary

Closed fourteen-class `channel_id`:

| Class | Meaning |
| --- | --- |
| `docs_site` | Canonical docs site / docs pane row. |
| `migration_notes` | Migration notes / downgrade-route row. |
| `help_about` | Help and About pane row (combined here per the parity matrix; the destination descriptor row keeps the surface identity). |
| `service_health` | Service-health pane row. |
| `support_export` | Support-export packet card row. |
| `release_packet` | Release-evidence packet row. |
| `release_notes` | Release-notes / what's-new card row. |
| `cli_help` | CLI / `--explain` help text row. |
| `evaluation_artifact` | Evaluation kit / certified-archetype packet row. |
| `marketplace_discovery` | Marketplace-style discovery card row. |
| `public_proof_packet` | Public-proof / benchmark publication packet row. |
| `known_limits_destination` | Known-limits / exclusion-notes destination quoted by docs, release notes, support exports, and the claim manifest projection. |
| `workflow_bundle_badge` | Workflow-bundle compatibility / coverage badge destination. |
| `certification_badge` | Certified-archetype badge destination. |

`channel_id` mirrors the claim manifest's eleven-class `channel_id`
verbatim and extends it with `known_limits_destination`,
`workflow_bundle_badge`, and `certification_badge` so caveat
destinations and badge destinations can be modeled with the same
publication-binding shape rather than as out-of-band footnotes.

A destination row whose `channel_id` cannot be typed is non-conforming;
the schema rejects unknown channel ids.

## 4. Evidence-binding rules

The binding pins one typed `evidence_binding_rules` envelope describing
how the publication automation MUST handle freshness, support-class
alignment, known-limit coverage, badge downgrade, and automatic
narrowing for this claim row.

### 4.1 Freshness floor

Closed four-class `freshness_floor_class` (mirrors the claim manifest's
`freshness_expectation` verbatim):

| Class | Meaning |
| --- | --- |
| `must_be_current` | Required-evidence rows MUST be `current`; any `warm_cached`, `stale`, or `missing` row triggers a fail-closed or narrowed-pass verdict. |
| `may_be_warm_cached` | Required-evidence rows MAY be `warm_cached`; `stale` and `missing` rows trigger fail-closed or narrowed-pass. |
| `seed_only_allowed` | Required-evidence rows MAY be `seed_only`; the publication binding renders the `seed_only` posture and routes to the seed-only copy field. |
| `not_applicable` | The claim row carries no freshness floor (e.g. a withdrawn row). |

### 4.2 Support-class alignment

Closed six-class `support_class_alignment_class`:

| Class | Meaning |
| --- | --- |
| `aligned` | Declared posture matches the lifecycle / support-class state on the claim row. |
| `narrowed_to_match_support_class` | Declared `claim_bearing` posture narrowed to `experimental` or `limited` because the support-class state is below the declared posture. |
| `narrowed_for_replacement_grade` | Declared posture narrowed to `replacement_grade`; the binding cites the `replacement_claim_row_ref`. |
| `blocked_widening_above_support_class` | Declared posture would widen above the support-class state; the binding gate state is `blocked_widening`. |
| `support_window_expired_routes_to_replacement` | Support window is `expired`; the binding routes to the replacement claim row. |
| `not_applicable` | Claim row has no support window posture (e.g. seed-only). |

### 4.3 Known-limit coverage

Closed six-class `known_limit_coverage_class`:

| Class | Meaning |
| --- | --- |
| `complete` | Every required known-limit ref is present on every destination publication row. |
| `missing_required_caveat` | At least one destination publication row is missing a required known-limit ref; gate fail-closed. |
| `missing_support_window_language` | At least one destination publication row is missing required support-window language; gate fail-closed. |
| `redundant_caveat` | A destination publication row carries a known-limit ref that the claim row does not require; informational only. |
| `caveat_links_unresolved` | At least one known-limit ref does not resolve in the claim manifest baseline; gate fail-closed. |
| `not_applicable` | Claim row has no caveat requirements (e.g. withdrawn). |

### 4.4 Badge downgrade

Closed seven-class `badge_downgrade_class` for certified, managed-
approved, and workflow-bundle badge destinations:

| Class | Meaning |
| --- | --- |
| `no_downgrade` | Badge state remains current; no badge downgrade applied. |
| `downgrade_to_provisional` | Badge state moves to `*_provisional`; required evidence is partial or warm-cached. |
| `downgrade_to_pending` | Badge state moves to `badge_pending`; required evidence is missing or in-progress. |
| `downgrade_to_withheld` | Badge state moves to `badge_withheld_pending_evidence`; the binding withholds the badge entirely. |
| `downgrade_to_stale` | Badge state moves to `badge_stale`; required evidence is past freshness floor. |
| `badge_revoked` | Badge state moves to `badge_revoked`; the path no longer qualifies. |
| `not_applicable` | Claim row has no badge destination. |

### 4.5 Automatic narrowing

Closed seven-class `narrowing_action_class`:

| Class | Meaning |
| --- | --- |
| `no_narrowing_required` | The published copy matches the declared posture; no narrowing applied. |
| `narrowed_to_limited` | Published copy narrowed to `limited`; binding inherits the claim manifest's `stale_behavior = downgrade_to_limited`. |
| `narrowed_to_experimental` | Published copy narrowed to `experimental`; binding inherits the claim manifest's `stale_behavior = downgrade_to_experimental`. |
| `narrowed_to_seed_only` | Published copy narrowed to `seed_only`; binding renders the `seed_only` copy field. |
| `narrowed_to_replacement_grade` | Published copy narrowed to `replacement_grade` and routed to the replacement claim row. |
| `hidden_and_routed_to_repair` | Published copy hidden; binding routes to the repair hook (mirrors the claim manifest's `hide_claim_and_route_to_repair`). |
| `withdrawn_from_public_lane` | Claim row withdrawn from public lane; every destination publication row is suppressed. |

The binding's `narrowing_action_class` MUST agree with the claim row's
`stale_behavior` vocabulary on the affected channel binding. A binding
whose narrowing action contradicts the claim manifest is non-conforming.

## 5. Evidence resolution

The binding carries one `evidence_resolution_rows[]` entry per
`claim_evidence_id` on the claim row. Each row pins:

- the `claim_evidence_id` (mirroring the claim manifest);
- the typed `evidence_kind` (mirroring the claim manifest verbatim);
- the `packet_ref` and `evidence_id_refs[]`;
- the typed `freshness_floor_class` (mirroring the claim row's
  `freshness_expectation`);
- the typed `evidence_freshness_state` (the publication automation's
  observed freshness for this evidence input);
- the typed `evidence_scope_state` (whether the evidence's scope
  matches, narrows, or widens past the claim text);
- the typed `minimum_result_status` (mirroring the claim row);
- the typed `observed_result_status` (the current evidence result);
- the `link_requirement` (`required` or `advisory`);
- the optional `known_limit_ref`;
- a bounded `evidence_resolution_summary`.

A required evidence row whose `evidence_freshness_state` is `missing`
emits a `missing_evidence_row` blocking failure. A row whose
`evidence_scope_state` is `scope_narrower_than_claim` emits an
`evidence_narrower_than_claim` blocking failure unless the binding's
`narrowing_action_class` is in
`{narrowed_to_limited, narrowed_to_experimental, narrowed_to_seed_only,
narrowed_to_replacement_grade, hidden_and_routed_to_repair}`. A row
whose `evidence_scope_state` is `scope_broader_than_claim` is
informational only (the publication binding does not widen the claim
beyond its declared scope).

## 6. Destination publication rows

The binding carries one `destination_publication_rows[]` entry per
publication channel that projects the claim row. Each row pins:

- the `destination_row_id`;
- the typed `channel_id`;
- the `surface_ref` (e.g. destination descriptor row id, what's-new
  card id, help-pane state id, badge id);
- the typed `projection_kind` (mirrors the claim manifest's
  `projection_kind` and extends with `badge_only` and
  `not_published_this_run`);
- the typed `projected_copy_state_class` (one of eleven closed states
  the publication automation pins per row);
- the typed `badge_state_class` (eleven closed states for badge-
  bearing destinations; `not_applicable` for non-badge channels);
- the typed `support_window_state` (mirroring the claim row);
- the boolean `known_limit_present`;
- the boolean `support_window_language_present`;
- the boolean `narrowed_from_declared`;
- the typed `narrowing_action_class` (per-row);
- a bounded `destination_summary`.

The schema enforces:

- `channel_id` in `{certification_badge, workflow_bundle_badge}`
  requires `projection_kind = badge_only` and a non-`not_applicable`
  `badge_state_class`;
- `projection_kind = not_published_this_run` pairs with one of the
  `suppressed_*` `projected_copy_state_class` values;
- `narrowed_from_declared = true` requires
  `narrowing_action_class != no_narrowing_required` and conversely;
- `effective_claim_posture = withdrawn` forces every destination row
  to a `suppressed_*` state;
- `effective_claim_posture = policy_disabled` forces every destination
  row to one of `{suppressed_policy_disabled, published_status_only,
  published_repair_route_only, suppressed_audience_redacted}`.

## 7. Publication diff

The binding pins one `publication_diff` envelope so reviewers can
compare old and new published claim sets without reading every
destination manually.

Closed ten-class `diff_kind_class`:

| Class | Meaning |
| --- | --- |
| `new_claim_row_published` | The claim row is publishing for the first time; `previous_publication_snapshot_ref` is `null`. |
| `claim_row_narrowed_in_run` | The published posture narrowed compared to the previous snapshot. |
| `claim_row_widened_in_run` | The published posture widened compared to the previous snapshot. Allowed only when the gate verdict is `passed` or `narrowed_pass`. |
| `claim_row_widening_blocked` | A widening attempt was blocked at the gate; the diff carries the blocked widening intent for review. |
| `claim_row_retired_in_run` | The claim row exited the public lane; every destination row is suppressed. |
| `badge_downgraded_in_run` | At least one badge destination's `badge_state_class` downgraded compared to the previous snapshot. |
| `badge_widened_blocked` | A badge widening attempt was blocked at the gate. |
| `caveat_added_in_run` | At least one destination row added a known-limit ref. |
| `caveat_removed_blocked` | A caveat-removal attempt was blocked at the gate; the caveat remains. |
| `no_change` | The published row is identical to the previous snapshot. |

Each `per_channel_diff_rows[]` entry pins one typed
`per_channel_change_class` plus the previous and current
`projected_copy_state_class` and `badge_state_class` so the diff is
reproducible from the binding alone.

Closed ten-class `per_channel_change_class`:

| Class | Meaning |
| --- | --- |
| `added` | Destination row added in this run. |
| `removed_blocked` | A removal attempt was blocked; the row remains published. |
| `removed_via_narrowing` | Destination row removed because the binding narrowed the row out of the channel. |
| `removed_via_retirement` | Destination row removed because the claim row was retired. |
| `copy_narrowed` | Published copy narrowed compared to the previous snapshot. |
| `copy_widened_blocked` | A copy-widening attempt was blocked. |
| `badge_state_changed` | Badge state changed (downgrade or revocation). |
| `caveat_added` | Known-limit ref added on this destination. |
| `caveat_removed_blocked` | A caveat-removal attempt was blocked. |
| `no_change` | No change on this destination. |

A binding whose diff `diff_kind_class` is `claim_row_widened_in_run`
but whose gate verdict is `fail_closed` or `blocked_widening` is
non-conforming.

## 8. Gate verdict

The binding pins one `gate_verdict` envelope. The four-class
`gate_state_class`:

| Class | Meaning |
| --- | --- |
| `passed` | No blocking failures; no automatic narrowing applied; publication admits. |
| `narrowed_pass` | Automatic narrowing applied; the binding's `narrowed_to_posture` is non-null; publication admits at the narrowed posture. |
| `blocked_widening` | A widening attempt was blocked; the binding does not advance the previously-published row but does not retire it either. |
| `fail_closed` | Publication is denied; the previously-published row remains as-is and the run does not ship. |

Each `blocking_failure_rows[]` entry carries one typed
`blocking_failure_class`. The thirteen closed classes:

| Class | Meaning |
| --- | --- |
| `orphan_claim_text` | Published copy exists on a destination row that does not resolve to a `claim_row_ref` in the manifest baseline. |
| `missing_evidence_row` | A required evidence row is absent from the binding. |
| `broader_than_current_proof` | Published copy is broader than the current evidence's scope or freshness state. |
| `stale_certified_badge` | A `certification_badge` destination's evidence is past the freshness floor. |
| `stale_managed_approved_badge` | A managed-approved badge destination's evidence is past the freshness floor. |
| `stale_workflow_bundle_badge` | A `workflow_bundle_badge` destination's evidence is past the freshness floor. |
| `destination_missing_required_caveat` | A destination row is missing a required known-limit ref. |
| `destination_missing_support_window_language` | A destination row is missing required support-window language. |
| `evidence_narrower_than_claim` | A required evidence row's scope is narrower than the claim text. |
| `support_class_widening_above_evidence` | A destination row attempts to widen above the support-class state on the claim row. |
| `badge_revoked_for_path` | A badge destination is `badge_revoked`; the row may not publish a current badge. |
| `evidence_freshness_below_floor` | A required evidence row's freshness state is below the binding's freshness floor. |
| `claim_row_not_in_baseline` | The binding's `claim_row_ref` does not resolve in the claim manifest baseline. |

A blocking failure row's `permissible_under_narrowed_pass = true` is
set when the failure is automatically resolved by the binding's
`narrowing_action_class` (e.g. `evidence_narrower_than_claim` resolved
by `narrowed_to_limited`). When every blocking failure is permissible
under narrowed pass, `gate_state_class` MUST be `narrowed_pass` and
`automatic_narrowing_applied` MUST be `true`. When any blocking
failure is not permissible, `gate_state_class` MUST be `fail_closed`
or `blocked_widening`.

## 9. Consuming-surface parity

Every consuming surface that renders a binding MUST render the same
typed fields. The parity floor is enforced by the schema's
`consuming_surface_parity` block.

Required on every consuming surface:

- `binding_id`, `publication_run_id`, `claim_row_ref`,
  `claim_manifest_baseline_ref`, `evaluated_at`;
- `declared_claim_posture`, `effective_claim_posture`;
- `evidence_binding_rules` (all five class fields);
- `evidence_resolution_rows` (all eleven required fields per row);
- `destination_publication_rows` (all twelve required fields per row);
- `publication_diff.diff_kind_class` and `per_channel_diff_rows`;
- `gate_verdict.gate_state_class` and `blocking_failure_rows`;
- `audience_class`, `redaction_profile_class`;
- `headline_label`, `publication_binding_summary`.

Forbidden collapses on every consuming surface:

- Rendering a `fail_closed` verdict as "needs review" without the
  typed `gate_state_class` and `blocking_failure_class` set.
- Dropping `evidence_resolution_rows` to keep the diff readable.
- Re-authoring `destination_publication_rows.surface_ref` strings to
  shorten them; refs are opaque and stable.
- Substituting "should be quick" or "close to ready" for the typed
  `narrowing_action_class` and `narrowed_to_posture`.
- Filtering out a `narrowed_pass` row under a "blocked" affordance
  when widening is admitted at the narrowed posture.
- Reformatting the publication diff into a screenshot or a parallel
  narrative block.

## 10. Reuse rules

Docs, release, support, and governance reviews consume the same
publication binding by reading the typed binding records. The reuse
rules:

1. **No surface re-runs the publication automation by hand.** A docs
   review, a release-notes review, a support-export review, an
   evaluation-artifact review, or a public-proof-packet review that
   wants the current "what claim is the product publishing today?"
   answer reads the binding records and the typed `publication_diff`
   instead of re-typing the row from a screenshot.
2. **No surface invents a parallel evidence-binding scale.** Freshness
   floors, support-class alignment classes, known-limit coverage
   classes, badge-downgrade classes, and narrowing-action classes are
   typed; a surface that introduces a "warning" or "yellow" chip on
   top of these is non-conforming.
3. **No surface drops blocking failures selectively.** A consuming
   surface that filters blocking failures by audience MUST go through
   the typed `redaction_profile_class` projection; ad-hoc filtering by
   failure age or by channel is non-conforming.
4. **Publication runs MUST consult the audit packet.** A publication
   run MUST resolve the public-truth audit packet's
   `widening_frozen_count` before admitting `claim_row_widened_in_run`
   or `badge_state_changed = widened`; if the count is non-zero and
   the binding's `claim_row_ref` participates in any open drift item,
   the gate verdict MUST be `blocked_widening` unless the gating drift
   item is held under a waiver register entry cited on the binding.
5. **Claim narrowings MUST cite the originating binding.** A claim-
   manifest review that narrows `effective_claim_posture` because of
   a publication-gate verdict cites the originating binding id; the
   binding retains the closed verdict for audit traceability.
6. **Late-copy narrowings MUST cite the originating binding.** A
   late-copy change packet that narrows protected publication copy
   after string freeze cites the originating binding id and the
   publication run id; the binding's
   `linked_artifact_families.late_copy_change_packet_refs[]` carries
   the late-copy packet id.
7. **Public-proof packets MUST cite the originating binding.** A
   public-proof / benchmark publication packet that publishes a claim
   row cites the binding id and the publication run id so the public-
   proof index can render the same gate state.
8. **Workflow-bundle and certification badges MUST cite the
   originating binding.** A workflow-bundle compatibility / coverage
   badge or a certified-archetype badge cites the binding id and the
   `badge_state_class` so the badge renderer can show the same
   downgrade or revocation state without inventing a parallel badge
   scale.

## 11. Audience and redaction vocabulary

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

The schema enforces that `redaction_profile_class` matches
`audience_class` per the table above. A binding whose redaction
profile widens scope past the named audience denies with
`redaction_profile_widens_scope`.

The honesty invariants on the redaction profile:

- `raw_page_body_bytes_excluded` — constant `true`.
- `raw_screenshots_excluded` — constant `true`.
- `raw_user_identifiers_excluded` — constant `true`.
- `raw_support_case_bodies_excluded` — constant `true`.
- `raw_release_note_prose_excluded` — constant `true`.

## 12. Linked artifact families

Refs cite stable artifact-family ids resolved through the named
schemas / docs. Empty arrays are admissible except where noted.

- `claim_manifest_row_refs[]` — claim manifest row ids the binding
  composes over. MUST equal the union of `{claim_row_ref}` plus any
  successor / replacement row refs.
- `evidence_packet_refs[]` — evidence packet ids the binding composes
  over.
- `destination_descriptor_row_refs[]` — destination descriptor row
  ids. Required non-empty when any destination publication row uses
  `channel_id` in `{help_about, service_health, docs_site}`.
- `whats_new_card_row_refs[]` — what's-new / release-notes card row
  ids. Required non-empty when any destination publication row uses
  `channel_id = release_notes`.
- `help_pane_state_refs[]` — docs/help-pane state row ids. Required
  non-empty when any destination publication row uses
  `channel_id = help_about`.
- `public_proof_row_refs[]` — public-proof / claim-manifest
  publication row ids. Required non-empty when any destination
  publication row uses `channel_id = public_proof_packet`.
- `support_export_packet_refs[]` — support export packet ids.
  Required non-empty when any destination publication row uses
  `channel_id = support_export`.
- `release_packet_refs[]` — release-evidence packet ids. Required
  non-empty when any destination publication row uses
  `channel_id = release_packet`.
- `certification_badge_refs[]` — certified-archetype badge ids.
  Required non-empty when any destination publication row uses
  `channel_id = certification_badge`.
- `workflow_bundle_refs[]` — workflow-bundle ids. Required non-empty
  when any destination publication row uses
  `channel_id = workflow_bundle_badge`.
- `known_limit_destination_refs[]` — known-limits / exclusion-notes
  destination ids. Required non-empty when any destination publication
  row uses `channel_id = known_limits_destination`.
- `late_copy_change_packet_refs[]` — late-copy change packet ids that
  carried any narrowing post-string-freeze.
- `public_drift_item_refs[]` — public-drift item ids open against the
  claim row. Empty when no open drift item touches the row.

The schema enforces the surface-pairing requirements above through
the destination-publication row's `channel_id` and the linked-artifact-
family floor.

## 13. Authoring rules

When a publication run resolves one claim row's published copy:

1. Mint one `claim_publication_binding_record` per claim row.
2. Resolve the claim row through the claim manifest baseline pinned on
   the run; deny `claim_row_not_in_baseline` if the row does not
   resolve.
3. Resolve every `claim_evidence_id` on the row into an
   `evidence_resolution_row` with the typed freshness, scope, and
   result_status states.
4. Compute the `evidence_binding_rules` envelope (freshness floor,
   support-class alignment, known-limit coverage, badge downgrade,
   automatic narrowing) from the claim row and the resolved evidence.
5. Project one `destination_publication_row` per publication channel
   the claim row's `channel_bindings` requires; pin the typed
   `projection_kind`, `projected_copy_state_class`, `badge_state_class`,
   `support_window_state`, `known_limit_present`,
   `support_window_language_present`, `narrowed_from_declared`, and
   `narrowing_action_class`.
6. Compute the `publication_diff` against the previous publication
   snapshot; pin the typed `diff_kind_class` and emit one
   `per_channel_diff_row` per destination.
7. Compute the `gate_verdict`; emit one `blocking_failure_row` per
   failing condition; pin the typed `gate_state_class`,
   `automatic_narrowing_applied`, and `narrowed_to_posture`.
8. Pin the `audience_class` and matching `redaction_profile`.
9. Compute `consuming_surface_parity` so the publication generator,
   the publication-diff reviewer, the release-evidence packet, the
   docs / Help / About / service-health renderers, the support-export
   builder, the CLI/help text emitter, the evaluation-artifact
   builder, the workflow-bundle / certification badge renderer, and
   the public-proof packet writer all render the same row.

A reprojection is required when:

- the claim manifest baseline advances;
- a claim row's `effective_claim_posture`,
  `active_downgrade_reasons[]`, `evidence_links[]`, or
  `known_limit_refs[]` moves;
- an evidence packet referenced by an evidence-resolution row moves
  through a freshness or scope class;
- a destination descriptor, what's-new card, help-pane state, badge,
  or workflow-bundle referenced by a destination-publication row
  moves; or
- a public-drift item open against the claim row transitions through
  any ledger state class.

The reprojection MUST advance `evaluated_at` and recompute the gate
verdict.

## 14. Out of scope

This contract does not implement:

- A documentation site, a release-notes rendering pipeline, a Help/
  About renderer, a service-health renderer, a CLI/help text emitter,
  an evaluation-artifact builder, a workflow-bundle or certification
  badge renderer, or a public-proof publishing service.
- The claim manifest itself (that lives on
  [`./../governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  and
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)).
- The public-truth parity matrix (that lives on
  [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)).
- The public-truth audit packet or the public-drift ledger (those live
  on
  [`./../public_truth/help_about_service_health_audit_packet.md`](../public_truth/help_about_service_health_audit_packet.md)
  and
  [`/schemas/public_truth/public_drift_item.schema.json`](../../schemas/public_truth/public_drift_item.schema.json)).
- The waiver register or the renewal-or-close decision objects (those
  live on
  [`./../governance/waiver_register_contract.md`](../governance/waiver_register_contract.md)
  and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)).
- The destination-descriptor, what's-new-card, help-pane-state, or
  late-copy change-packet schemas (those live on the named contracts).

This contract is the projection vocabulary that publication automation
flows through when it generates docs, Help/About, service-health,
release notes, CLI/help, evaluation artifacts, workflow-bundle and
certification badges, known-limits destinations, and public-proof
packets — so a reviewer can answer "what is the product about to
publish, and does it match the current evidence?" mechanically rather
than by manual sweep.
