# Launch-language bundle and framework-pack rubric

This document is the reviewer-facing rubric for the languages and
frameworks the product is willing to name in top-level claims at the
first stable cut. It exists so product, architecture, and release
owners can keep v1.0 scope narrow and supportable — and so new
candidate rows can be accepted, held, or cut without argument about
which criteria were missing.

Tooling reads the companion artifacts; this document sets the reader
expectations that bind them together. If this document and the
artifacts disagree, the artifacts are authoritative and this document
is updated in the same change.

Companion artifacts:

- [`/artifacts/product/language_bundle_rows.yaml`](../../artifacts/product/language_bundle_rows.yaml)
  — closed vocabularies, selection-criterion scoring, rubric gate
  questions, per-bundle rows, per-pack rows, exclusion reservations,
  and rubric invariants.
- [`/artifacts/product/framework_pack_owners.yaml`](../../artifacts/product/framework_pack_owners.yaml)
  — selection / evidence / publication owner allocation plus
  co-required approvers and review cadence per framework pack.
- [`/artifacts/compat/archetype_rubric.yaml`](../../artifacts/compat/archetype_rubric.yaml)
  — canonical support-class taxonomy (`certified`, `supported`,
  `community`, `experimental`) and the graduation / demotion mechanics
  every bundle and pack resolves through.
- [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  — archetype rows every bundle binds to via `archetype_row_ref`.
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — published claim rows that cite `launch_bundle:` ids.
- [`/docs/release/certified_archetype_report_template.md`](../release/certified_archetype_report_template.md)
  — report shape the certified class demands for any bundle.
- [`/docs/release/compatibility_report_template.md`](../release/compatibility_report_template.md)
  — shared row schema compatibility and archetype reports reuse.

## Why this rubric exists before launch

The PRD already establishes the principle: "supports Python" or
"supports React" is not a precise product claim. The reference-workspace
program turned that principle into a bridge — every claim must resolve
through an `archetype_row_id`. What was missing:

- a mapping from the archetype rows to the **launch-language bundles**
  downstream claim, docs, and release surfaces already quote
  (for example `launch_bundle:typescript_web_app.seed`);
- a mechanical rule for splitting a bundle into **framework packs** so
  the certified-class claim only covers the framework work the
  benchmark corpus and reference workspace actually demonstrate;
- one place where the **eight selection criteria** the program uses to
  accept, hold, or cut a row live.

This rubric closes those gaps without promoting any row. Promotion
still runs through `artifacts/compat/archetype_rubric.yaml`. This
rubric just makes sure every candidate row has answered the same eight
questions before the promotion conversation starts.

## Selection criteria

A candidate row scores one value from `strong`, `moderate`, `weak`, or
`unseeded` against each of the eight criteria.

| Criterion | What a strong score means |
|---|---|
| `user_wedge_fit` | The bundle sits squarely on the product's stated wedge and is not a breadth row trying to look like a wedge row. |
| `p0_persona_fit` | At least one P0 persona carries this bundle in their daily workflow — not a once-a-quarter flow. |
| `reference_workspace_proof` | A reviewable reference workspace already exists (or a named reservation slot under `fixtures/workspaces/reference/`). |
| `benchmark_cost` | The corpus rows the bundle needs are tractable for the benchmark lane — not a multi-corpus uplift. |
| `tooling_depth` | The first-party tooling depth the bundle promises can be sustained by the staffed lane. |
| `docs_burden` | `docs_public_truth` can keep the bundle's pages current at the declared support class without borrowing freshness budget from another row. |
| `migration_importance` | Migration from a specific competing tool is a load-bearing part of the product narrative for this bundle. |
| `supportability` | `support_export` can carry the bundle's failure modes honestly at the declared class. |

A row that is `weak` on three or more criteria is a candidate for
`remove_from_launch_set_if_either_slips` or `cut_first_if_m1_slips`
posture. A row that is `strong` across user_wedge_fit, p0_persona_fit,
reference_workspace_proof, and tooling_depth is a candidate for
`protect_through_m1_m2`.

Scores are reviewer judgement, not measurements. They sit in the row
file so future reviewers can see what the previous review believed.
Changing a score is a reviewable change, not a silent update.

## Rubric gate

A candidate bundle answers every gate question before it is admissible
as a launch-language row. Each question points at the criteria used to
answer it.

1. **Wedge and persona fit** — `user_wedge_fit`, `p0_persona_fit`. Does
   the bundle sit on the stated wedge and carry a P0 persona?
2. **Reference-workspace proof** — `reference_workspace_proof`. Is
   there a reviewable workspace or a named reservation?
3. **Benchmark and tooling cost** — `benchmark_cost`, `tooling_depth`.
   Can the bundle meet its target support class without borrowing
   evidence from another bundle?
4. **Docs and migration load** — `docs_burden`, `migration_importance`.
   Can docs keep the pages current, and is migration load-bearing?
5. **Supportability** — `supportability`. Can support_export carry the
   bundle honestly at the declared class?

A row that cannot answer one gate stays `experimental`. A row that
cannot answer two gates is not admissible and lands in
`excluded_language_bundles` with a reservation note rather than a
silent omission.

## Support-class binding

Launch bundles and framework packs reuse the four-class vocabulary
defined in `artifacts/compat/archetype_rubric.yaml`:

- `certified` — release-blocking reference workspace, current
  certified-archetype report, current claim-manifest row, and the
  benchmark corpus rows carry the claim. Top-level launch wording is
  admissible.
- `supported` — first-party tested and documented, not on every
  permutation. Docs and scoped roadmap wording is admissible with
  explicit caveats. Top-level launch wording is not.
- `community` — discoverable through extensions or community signals.
  Not marketed as first-party replacement-grade.
- `experimental` — valuable direction under active iteration. Preview
  label required.

**A framework pack MAY carry a narrower target class than its parent
bundle; it MAY NOT carry a higher one.** A pack that falls below its
parent's class narrows the pack and stays at that narrower class
independently of the bundle-level posture.

## Replacement-grade claims

A launch bundle cannot carry replacement-grade wording (the wording the
release deck means when it says "switch your TypeScript work to
Aureline") without the full closed proof set:

- `benchmark_corpus_linked` — at least one protected corpus scenario
  row cites the bundle.
- `compatibility_report_linked` — a current compatibility-report row
  names the bundle's archetype `row_id`.
- `migration_evidence_linked` — migration notes or a migration corpus
  row name the bundle.
- `docs_version_match_linked` — docs pages carry an exact-build
  identity match.
- `known_limit_note_linked` — a known-limits note names the bundle's
  scope caveats.

A bundle missing any one item is wording-ineligible for
replacement-grade claims. The missing items demote through the
archetype rubric's demotion rules rather than through a softer
proof bar here.

The per-bundle value of `replacement_grade_evidence_required` in
`language_bundle_rows.yaml` enumerates exactly which items a bundle
must carry to stay eligible at its target class; a bundle may drop
items it does not need (for example `migration_evidence_linked` on
the Rust self-host row where migration is intentionally weak).

## Seed inventory at a glance

| Bundle id | Archetype row | Initial | Target | Inclusion | Cut-first posture |
|---|---|---|---|---|---|
| `launch_bundle:typescript_web_app.seed` | `archetype_row:ts_web_app_or_service` | experimental | certified | first_stable | `protect_through_m1_m2` |
| `launch_bundle:python_service_or_data_app.seed` | `archetype_row:python_service_or_data_app` | experimental | certified | first_stable | `narrow_support_class_before_cutting` |
| `launch_bundle:rust_workspace.seed` | `archetype_row:rust_workspace` | supported | certified | foundations | `protect_through_m1_m2` |
| `launch_bundle:java_or_kotlin_service.seed` | `archetype_row:java_or_kotlin_service` | experimental | certified | first_stable | `cut_first_if_m1_slips` |
| `launch_bundle:go_service_or_monorepo_slice.seed` | `archetype_row:go_service_or_monorepo_slice` | experimental | certified | first_stable | `cut_first_if_m2_slips` |
| `launch_bundle:c_or_cpp_native_project.seed` | `archetype_row:c_or_cpp_native_project` | experimental | certified | first_stable | `preview_label_only_if_either_slips` |
| `launch_bundle:dotnet_service_or_app.seed` | `archetype_row:dotnet_service_or_app` | experimental | supported | first_beta | `narrow_support_class_before_cutting` |
| `launch_bundle:notebook_first_data_workflow.seed` | `archetype_row:notebook_first_data_workflow` | experimental | supported | first_beta | `narrow_support_class_before_cutting` |

The inventory also records explicit exclusion reservations for Ruby /
Rails, PHP, and Swift so the bundle set is complete without implying
support that is not seeded.

## Cut-first posture

If M1 or M2 scope slips, breadth cuts happen in the order the
`cut_first_posture` field names. The vocabulary is:

- `protect_through_m1_m2` — cut other breadth before cutting this row.
  The TypeScript launch wedge and the Rust self-host row carry this
  posture today.
- `narrow_support_class_before_cutting` — fall one class first; do not
  delete. Python, .NET, and the notebook-first row carry this posture.
- `cut_first_if_m1_slips` — primary cut target if M1 scope slips. The
  Java / Kotlin row carries this posture — JDK-selection and
  wrapper-trust cost is higher than the launch-wedge persona can
  absorb at current staffing.
- `cut_first_if_m2_slips` — primary cut target if M2 scope slips. The
  Go row carries this posture.
- `preview_label_only_if_either_slips` — keep the row in the inventory
  but demote the row to preview copy only. The C / C++ row carries
  this posture.
- `remove_from_launch_set_if_either_slips` — withdraw the bundle from
  the public launch set entirely. No row carries this posture today;
  the vocabulary is reserved for rows that cannot be supported
  honestly at any class.

Cut-first posture is an **ordered** policy: bundles carrying
`cut_first_if_m1_slips` are narrowed before bundles carrying
`cut_first_if_m2_slips`, and so on. Two bundles sharing the same
posture are narrowed in the order of their selection scores (weaker
aggregate scores narrow first).

## Framework-pack ownership

Every pack named in `language_bundle_rows.yaml` has an owner row in
`framework_pack_owners.yaml` listing:

- `selection_owner` — accountable for whether the pack belongs in
  scope at all;
- `evidence_owner` — accountable for the corpus, fixture, and
  benchmark evidence;
- `publication_owner` — accountable for what the pack's copy says on
  public surfaces;
- `co_required_approvers` — any additional lanes that must sign off on
  graduation (for example `lane:shiproom_executive_scope_review` for
  certified-class packs);
- `review_cadence` — `each_change`, `per_milestone`, or `each_release`.

The three owner roles are disjoint by intent: one lane can carry
multiple roles on one pack, but the pack row names each role
explicitly. A missing role is non-conforming.

## Top-level claim policy

Top-level product claims cite one `bundle_id` or `pack_id`. The
following wording forms are non-conforming:

- "supports TypeScript" with no bundle cite.
- "full Python support" with no bundle cite.
- "certified for Java and Kotlin" when the Java / Kotlin bundle is
  below certified.
- A framework pack named in launch copy at a class higher than its
  target class.

Conforming wording cites the bundle and, if the framework pack matters
for the claim, the pack too:

- "The TypeScript web-app launch bundle
  (`launch_bundle:typescript_web_app.seed`) is the named launch wedge;
  React (`framework_pack:typescript_web.react`) is the UI pack inside
  that bundle."
- "Rust self-host (`launch_bundle:rust_workspace.seed`) ships at
  supported, with certified wording gated on a certified-archetype
  report that covers the corresponding hardware and toolchain rows."

Replacement-grade wording for the TypeScript/JavaScript launch wedge
should also resolve to the acceptance rows that define “replacement
grade” in concrete terms. The canonical acceptance matrix lives in:

- `docs/compat/typescript_javascript_expert_acceptance_matrix.md`
- `artifacts/compat/ts_js_acceptance_rows.yaml`

## Change discipline

- **Add a new bundle row** when a new archetype enters launch scope.
  The row must bind to an `archetype_row_id`, carry
  `selection_scores` for every criterion, name a `cut_first_posture`,
  and list at least one framework pack or state that the bundle is a
  language-only bundle.
- **Add a new framework pack** when a bundle needs to split a claim.
  The pack must bind to exactly one parent bundle, carry an owner row
  in `framework_pack_owners.yaml`, and carry a `target_support_class_id`
  no higher than its parent's.
- **Promote or demote** through
  `artifacts/compat/archetype_rubric.yaml`. Silent promotion is
  non-conforming; the demotion rules fire automatically when evidence
  goes stale.
- **Exclude a bundle** by adding a row to `excluded_language_bundles`
  with an `exclusion_reason` and an `inclusion_target`. Omitting the
  row is non-conforming; the inventory is meant to be complete.
- **Score a row** by changing the `selection_scores` map. Score
  changes are reviewable; updating a score without a reason is
  non-conforming.

## What this rubric does not do

- It does not certify any bundle. Certification runs through the
  certified-archetype report template and the archetype rubric.
- It does not publish claim copy. The claim-manifest seed and the
  release evidence packet template own that shape.
- It does not expand the launch set. Adding a bundle is a reviewable
  change; the rubric keeps the conversation in one place rather than
  spread across docs, release, and support lanes.
- It does not commit framework packs to the certified target class.
  Packs target the class the evidence and ownership can sustain; the
  parent bundle's class ceiling still binds.
