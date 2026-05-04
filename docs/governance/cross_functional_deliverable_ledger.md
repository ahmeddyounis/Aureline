# Cross-functional deliverable ledger and externalization overlay

This document is the human-readable companion to the cross-functional
milestone-deliverable ledger and the externalization overlay. It exists
so that milestone closure depends on more than merged code: every
function (program/product, UX/design, engineering/architecture,
quality/security/accessibility/release, and docs/DevRel/support) names
which artifacts it must hand forward for the next milestone to be real
and externally supportable.

A scorecard row tells reviewers whether a lane is green for the
current milestone. A ledger row tells reviewers what that lane MUST
hand forward — design evidence ids, benchmark corpora, support packs,
docs truth, release packets, public-proof bundles — before the next
milestone may be marked green. The externalization overlay tags the
subset of those deliverables whose visibility is public, control-plane,
or operator-facing so public-proof assets are first-class deliverables
rather than side notes.

Companion artifacts:

- [`/artifacts/governance/milestone_deliverable_ledger.yaml`](../../artifacts/governance/milestone_deliverable_ledger.yaml)
  — machine-readable ledger rows. Tooling reads this file; the
  narrative below describes the same rows.
- [`/artifacts/governance/externalization_overlay.yaml`](../../artifacts/governance/externalization_overlay.yaml)
  — machine-readable externalization overlay. Tags ledger rows whose
  artifacts are public-proof, control-plane, or operator-facing.
- [`/fixtures/governance/hand_forward_examples/`](../../fixtures/governance/hand_forward_examples/)
  — worked foundations, prototype, and alpha hand-forward fixtures.
- [`/artifacts/governance/milestone_scorecard_template.yaml`](../../artifacts/governance/milestone_scorecard_template.yaml)
  — scorecard template that consumes ledger rows verbatim.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  and
  [`./control_artifact_index.md`](./control_artifact_index.md) —
  canonical home of every control asset; ledger rows resolve to
  control-artifact-index rows by canonical location.
- [`/artifacts/governance/packet_class_registry.yaml`](../../artifacts/governance/packet_class_registry.yaml)
  and
  [`./artifact_hierarchy_and_packet_classes.md`](./artifact_hierarchy_and_packet_classes.md)
  — canonical packet classes; every ledger row resolves to one class.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  and
  [`./dri_map.md`](./dri_map.md) —
  ownership matrix and DRI map. Ledger rows reuse `lane_id` values
  from `scorecard_lane_index`.
- [`/docs/governance/evidence_freshness_policy.md`](./evidence_freshness_policy.md)
  and
  [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  — freshness ceilings; ledger rows reuse the same vocabulary
  (`each_change`, `weekly`, `per_milestone`).

**One row per (milestone, owner lane, deliverable).** The ledger does
not restate ADR prose, schema fields, fitness thresholds, or claim
posture. It only freezes which function owes which artifact to which
consumer milestone, who owns the evidence, and what freshness rule
applies. Tooling joins the ledger to the scorecard on
`(producing_milestone, lane_id)` and refuses milestone close when a
required hand-forward row is in `in_progress`,
`drafted_pending_review`, or `missing_blocks_close`.

## Why this exists

Pre-implementation milestones are routinely declared green on the
strength of merged code while a function (UX evidence, support
packets, release notice copy, benchmark publication, compatibility
report) silently defers the artifact that makes the work supportable
or explainable. The deferred artifact does not "block" anything — until
the next milestone needs to cite it, at which point the project either
re-does the work under freeze pressure or ships an undocumented
narrowing.

The cross-functional deliverable ledger turns those silent slips into
mechanical questions:

1. **What does this milestone owe the next one?** Every owner-lane
   category names its rows; the ledger is the single source of truth.
2. **Is each row delivered, drafted, deferred, or missing?** A closed
   `hand_forward_status` vocabulary makes the answer unambiguous.
3. **Does a missing artifact block milestone close?** A small set of
   `hand_forward_rules` answers yes-or-no without prose interpretation.

The externalization overlay does the same job for the subset of
artifacts whose visibility is public or control-plane. It exists so
"we shipped code, we just haven't published the boundary manifest yet"
fails the milestone-close gate the same way "we shipped code, we just
haven't reviewed the ADR yet" does.

## Owner-lane categories

The ledger resolves every row to one of five canonical categories.
The categories mirror the spec's cross-functional split; the scorecard
lanes that feed each category are listed in
`milestone_deliverable_ledger.yaml` under `lane_categories`.

| Category | Scope | Scorecard lanes that feed it |
|---|---|---|
| **Program / product** | Milestone scope, cutline discipline, claim posture, change budget, decision routing, scorecards, requirement register, claim manifest, governance packets. | `governance_packets` |
| **UX / design** | UX research, design-system seeds, interaction packets, design tokens, accessibility-input review evidence, design-evidence indices. | `design_system_seeds`, `accessibility_input_review` |
| **Engineering / architecture** | Protected-path code, ADR set, surface contracts, schema/wire shapes, package topology, runtime/transport, language and editor cores, scheduler/worker contracts. | `aureline-render`, `aureline-buffer`, `aureline-vfs`, `aureline-text`, `aureline-rpc`, `aureline-telemetry`, `shell_command_system`, `aureline-shell-spike`, `aureline-bench` |
| **Quality / security / accessibility / release** | Verification corpora, benchmark plans and publication packs, compatibility reports, security review and threat-model packets, accessibility verification, fitness functions, release-evidence, shiproom packets. | `benchmark_lab`, `release_evidence`, `accessibility_input_review` |
| **Docs / DevRel / support** | Public-truth docs, help/About surfaces, migration packs, support packets, runbook content, claim publication targets, public-proof bundles, DevRel-facing publication artifacts. | `docs_public_truth`, `support_export` |

Every row in the ledger names exactly one category and one
`lane_id` from the ownership matrix. The same scorecard lane may feed
more than one category when its work spans functions (for example
`accessibility_input_review` feeds both UX/design and QSAR).

## Hand-forward status vocabulary

`hand_forward_status` is a closed vocabulary. Tooling refuses to close
a milestone green when any row carries a status in the
"blocks milestone close" column.

| Status | Meaning | Blocks close? |
|---|---|---|
| `not_yet_started` | Producing milestone has not begun work. Acceptable only when the producing milestone is itself not the current open milestone. | no |
| `in_progress` | Work has started; deliverable is not yet handed forward in a shape the consumer milestone can cite. | yes |
| `drafted_pending_review` | A draft exists at the canonical home and is in review; review must close before the milestone closes green. | yes |
| `delivered_to_consumer` | Artifact lives at canonical home, is reviewed and accepted, carries an evidence id consumers cite verbatim, and meets the freshness rule. | no |
| `intentionally_deferred_with_packet` | Deliverable is explicitly deferred to a later milestone with a named exception or descoping packet, an updated cutline row, and a recorded compensating-evidence plan. | no |
| `missing_blocks_close` | Required for the consumer milestone, not handed forward, and not covered by a deferral packet. | yes |

A `delivered_to_consumer` row also carries an evidence id that
downstream milestones cite verbatim. A `intentionally_deferred_with_packet`
row carries a `deferral_packet_ref` resolving to a descoping or
narrowing packet plus an updated cutline entry; a row may not flip
into this status without the packet on file.

## Hand-forward rules

Six rules govern milestone close. Each rule names an enforcement
pointer so reviewers can find the tool or scorecard hook that fires
when the rule is violated.

1. **No silent function skip.** A milestone may not be marked green
   when one function shipped code but another deferred the artifact
   that makes the work supportable, explainable, or publishable.
   Scorecard tooling joins the ledger on
   `(producing_milestone, lane_id)` and refuses a green call when any
   row carries `in_progress`, `drafted_pending_review`, or
   `missing_blocks_close`.
2. **Deferral requires a packet.** `intentionally_deferred_with_packet`
   is only valid when the row cites a `deferral_packet_ref` resolving
   to a descoping or narrowing packet under
   `artifacts/governance/` plus an updated cutline entry.
3. **Consumer blocks on missing evidence.** A consumer milestone may
   not begin claim-bearing work on a row whose `hand_forward_status`
   is `missing_blocks_close` or `not_yet_started`. The consumer's
   claim manifest, public-proof bundle, or release packet MUST cite
   the producing-milestone evidence id by reference.
4. **Freshness floor carries forward.** A handed-forward artifact MUST
   stay within the freshness ceiling named on its row through the
   lifetime of every consumer milestone that cites it. Stale evidence
   silently downgrades every consumer claim to `evidence_stale` until
   the rerun lands.
5. **Public-proof overlay is first class.** Externalization-overlay
   rows (boundary manifest, help/About truth prototype, migration
   parity scoreboard, benchmark publication, compatibility
   publication, support-window statements, public-proof bundles,
   release notice packs) are first-class deliverables. A milestone may
   not close green when its overlay rows are missing, even when every
   internal ledger row is delivered.
6. **Scorecard and handoff consume verbatim.** Ledger rows are
   consumed verbatim by milestone scorecards and handoff packets
   without reformatting. Scorecard rows MUST cite `ledger_row_id`;
   handoff packets MUST cite the same `ledger_row_id` in their
   `linked_ledger_rows` field. Restating ledger content in scorecard
   or handoff prose is forbidden.

## Externalization overlay

The overlay tags the public-proof, control-plane, or operator-facing
subset of ledger rows. Closed vocabulary:

- **Boundary manifest** — public manifest naming every trust boundary,
  deployment profile, managed-vs-local dependency, and external-effect
  surface that the product asserts at the milestone.
- **Help / About truth surface** — help and About surfaces wired to
  the canonical claim manifest, build identity, and known-limit
  notice. Replaces paraphrased product copy with citations.
- **Migration parity scoreboard** — public scoreboard naming which
  migration paths carry what parity guarantee, plus the test corpus
  and known-limit notice they cite. First-class at first_beta and
  stable_v1.
- **Benchmark publication artifact** — published benchmark pack
  (hardware row, lab-image revision, protected-metric set,
  comparability disclosure, reproduction bundle).
- **Compatibility / certified-archetype publication** — public
  certified-archetype report for launch personas, deployment profiles,
  language packs, and framework matrices.
- **Support-window and supportability statement** — public statement
  of the support window, expected coverage, escalation path,
  exact-build identity, rollback contract, and known-limit notice
  surfaced in product.
- **Public-proof publication bundle** — reproducibility bundle
  published with claim, benchmark, or compatibility releases:
  provenance attestation, exact-build identity, dependency manifest,
  rerun harness.
- **Release notice / notice-pack** — public release notice naming
  version identity, claim diff, narrowing diff, known-limit diff,
  support-window posture, and rollback contract.

Adding a class is additive-minor and bumps `schema_version`;
repurposing a class is breaking and opens a decision row.

## Sample milestone walks

The fixtures under
[`/fixtures/governance/hand_forward_examples/`](../../fixtures/governance/hand_forward_examples/)
walk the foundations, prototype, and alpha hand-forwards end-to-end.
Each fixture cites ledger row ids and overlay row ids verbatim; none
of them mint parallel identifiers.

### Foundations

[`foundations_close_hand_forward.yaml`](../../fixtures/governance/hand_forward_examples/foundations_close_hand_forward.yaml)
shows the foundations milestone closing green with thirteen ledger
rows and three overlay rows handed forward to prototype:

- **Program / product:** architecture pack, requirement register seed,
  phase change-budget table.
- **UX / design:** design-evidence index and accessibility / input
  baseline.
- **Engineering / architecture:** ADR set, stable-surface inventory
  and surface contracts, package topology.
- **QSAR:** benchmark-lab seed (hardware row, lab-image revision,
  protected metrics), fitness-function seed, security baseline.
- **Docs / DevRel / support:** public-truth seed and claim-publication
  target list, support-packet index and shiproom dashboard seed.
- **Externalization overlay:** boundary manifest, benchmark publication
  seed, compatibility seed.

The fixture demonstrates that prototype's editor-truth packet, dogfood
verification corpus, dogfood runbook, and help/About truth prototype
all cite foundations evidence ids verbatim — no parallel naming
appears at prototype.

### Prototype

[`prototype_close_hand_forward.yaml`](../../fixtures/governance/hand_forward_examples/prototype_close_hand_forward.yaml)
shows the prototype milestone failing to close green because the
dogfood verification corpus is in `drafted_pending_review`. The
fixture documents:

- Five rows in `delivered_to_consumer` (scorecard, dogfood design
  packets, editor dogfood truth, dogfood runbook, help/About truth
  prototype).
- One row in `drafted_pending_review` (dogfood verification corpus)
  that blocks milestone close under
  `hand_forward_rule:no_silent_function_skip`.
- Three overlay rows delivered (help/About, support-window dogfood,
  boundary manifest dogfood).
- Alpha-producing rows kept in `not_yet_started` to make the rule that
  alpha rows are not in scope at prototype close explicit.
- A milestone-close decision of `open` with `blocked_by:
  prototype.qsar.dogfood_corpus`.

Alpha cannot open under a defensible launch-wedge cutline until the
blocked corpus row reaches `delivered_to_consumer`.

### Alpha

[`alpha_close_hand_forward.yaml`](../../fixtures/governance/hand_forward_examples/alpha_close_hand_forward.yaml)
shows the alpha milestone closing yellow rather than green to make a
deferred support-window statement visible:

- Six rows in `delivered_to_consumer` (claim manifest, launch-wedge
  design pack, launch-wedge contracts, launch-wedge benchmark
  publication, certified-archetype alpha, alpha release packet).
- One row in `intentionally_deferred_with_packet` (alpha support-window
  statement) paired with a descoping packet ref and an updated
  cutline row.
- Five overlay rows in `delivered_to_consumer` (help/About launch
  wedge, benchmark publication launch wedge, compatibility publication
  launch wedge, release notice alpha, public-proof bundle alpha).
- One overlay row in `intentionally_deferred_with_packet` (support-window
  statement alpha) mirroring the deferred ledger row.
- A milestone-close decision of `closed_yellow` with explicit
  rationale: closing green would hide the intentional deferral, while
  closing yellow keeps it on the board until first_beta restores the
  full statement.

The fixture demonstrates that all five categories continue to flow
together at alpha: design evidence ids feed UX/design and QSAR,
benchmark corpora feed QSAR and the help/About row, support packs
feed docs/DevRel/support and the support-window overlay row, docs
truth feeds the release packet, and the release packet cites every
upstream id.

## How tooling consumes the ledger

Tooling joins the ledger to the scorecard on
`(producing_milestone, lane_id)` and to the externalization overlay on
`linked_ledger_row_id`. The minimum join contract:

```
scorecard_row.lane_id          == ledger_row.owner_lane.lane_id
scorecard_row.milestone_id     == ledger_row.producing_milestone
overlay_row.linked_ledger_row_id == ledger_row.ledger_row_id
```

Scorecard rows in green status whose corresponding ledger rows are
not in `delivered_to_consumer` or `intentionally_deferred_with_packet`
fail the milestone-close gate. Overlay rows whose linked ledger rows
are missing, deferred without a packet, or stale fail the same gate.

Handoff packets (the artifacts that pass between milestones) MUST
list every ledger row they consume in `linked_ledger_rows` rather
than restating ledger content. The fixtures under
`fixtures/governance/hand_forward_examples/` show the canonical shape.

## Adding rows

Adding a ledger row is additive-minor and bumps `schema_version` on
`milestone_deliverable_ledger.yaml`. Adding a row requires:

- A `ledger_row_id` namespaced as
  `<producing_milestone>.<owner_lane_category>.<short_slug>`.
- A `producing_milestone` and at least one `consumer_milestone` from
  `governed_milestone_slugs`.
- An `owner_lane.category` from the closed `lane_categories` list and
  an `owner_lane.lane_id` from `ownership_matrix.scorecard_lane_index`.
- An `evidence_owner` (may equal the lane DRI).
- A `freshness_or_maintenance_rule` from the freshness vocabulary.
- A `hand_forward_status` from the closed status vocabulary.
- A `deliverable_class_ref` resolving to the packet-class registry
  when the deliverable is an instance of one of the closed classes.

Adding an externalization-overlay row additionally requires:

- An `overlay_class` from the closed
  `overlay_class_vocabulary` in `externalization_overlay.yaml`.
- A `linked_ledger_row_id` resolving to a ledger row whose
  `visibility_class` is public.
- A `must_exist_at_milestone_close` flag (true unless paired with an
  explicit deferral packet).

Repurposing a `hand_forward_status`, `overlay_class`, or
`hand_forward_rule` is breaking. Open a row in
`artifacts/governance/decision_index.yaml` and bump
`schema_version` accordingly.
