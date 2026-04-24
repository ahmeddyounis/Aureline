# Launch-wedge, P0-persona, and replacement-grade cutline contract

This document is the reviewer-facing contract that keeps v1.0 scope
narrow and deep. It names the personas, workflows, and proof families
that can earn high-confidence product language, and it records the
rules by which a persona or workflow is narrowed, held, or cut when
staffing, benchmark proof, migration proof, or supportability depth
is insufficient.

Tooling reads the companion artifacts; this document sets the reader
expectations that bind them together. If this document and the
artifacts disagree, the artifacts are authoritative and this document
is updated in the same change.

Companion artifacts:

- [`/artifacts/product/p0_persona_rows.yaml`](../../artifacts/product/p0_persona_rows.yaml)
  — canonical P0 persona rows, capability families, daily-driver and
  replacement-grade criteria, held-persona reservations, and the
  persona-inventory invariants.
- [`/artifacts/product/replacement_grade_cutlines.yaml`](../../artifacts/product/replacement_grade_cutlines.yaml)
  — cutline rows binding personas to workflow bundles and certified
  archetypes, required proof families, downgrade triggers, and the
  closed narrowing rules that fire when an insufficiency class
  activates.
- [`/artifacts/product/language_bundle_rows.yaml`](../../artifacts/product/language_bundle_rows.yaml)
  — launch-language bundle and framework-pack inventory that every
  persona and cutline binds to via `launch_bundle_ref` /
  `workflow_bundle_ref`.
- [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  — archetype rows every persona and cutline binds to via
  `archetype_row_ref`.
- [`/artifacts/compat/archetype_rubric.yaml`](../../artifacts/compat/archetype_rubric.yaml)
  — canonical support-class taxonomy (`certified`, `supported`,
  `community`, `experimental`) and graduation / demotion mechanics.
- [`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml)
  — assurance-claim rows downstream of a cutline (which specific
  public surfaces are allowed to render what wording).
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — published claim rows that cite `launch_bundle:` ids and extend a
  cutline row.
- [`/docs/product/launch_language_bundle_rubric.md`](./launch_language_bundle_rubric.md)
  — companion rubric that converts archetypes into launch-language
  bundles and framework packs.
- [`/docs/governance/descoping_policy.md`](../governance/descoping_policy.md)
  — descoping authority, cut-order rules, and rebaseline mechanics.

## Why this contract exists

The launch-language rubric already ensured that every candidate
bundle answered the same eight selection questions before the
promotion conversation started. What was missing:

- a canonical list of **which personas** the product will market as
  daily-driver or replacement-grade targets at the first stable cut;
- a **mapping** from each persona to the workflow bundles and
  certified archetypes that serve them;
- a **closed proof set** that any replacement-grade wording must
  carry before it is admissible, recorded as named cutline rows so
  the release, docs, and architecture lanes can point to one row
  rather than an ad-hoc list;
- a **mechanical narrowing rule** that fires when staffing,
  benchmark proof, migration proof, or supportability depth is
  insufficient, so a slipping row demotes on a rule instead of
  tribal memory.

This contract closes those gaps without promoting any row. Promotion
still runs through `artifacts/compat/archetype_rubric.yaml` and the
assurance-claim matrix; this document is the bridge between the
persona view and the proof view so reviewers can point at one
artifact when deciding whether a workflow may be marketed or must
remain preview or limited.

## Three active P0 personas

`p0_persona_rows.yaml` seeds three active P0 personas and seven held-
persona reservations:

| Persona id | Kind | Bound bundle | Daily-driver floor | Replacement-grade floor |
|---|---|---|---|---|
| `persona:p0.typescript_web_app_developer` | `external_daily_driver` | `launch_bundle:typescript_web_app.seed` | `supported` | `certified` |
| `persona:p0.python_service_or_data_app_developer` | `external_daily_driver` | `launch_bundle:python_service_or_data_app.seed` | `supported` | `certified` |
| `persona:p0.rust_self_host_developer` | `internal_self_host` | `launch_bundle:rust_workspace.seed` | `supported` | `certified` (self-host class; `migration_note_current` waived) |

Held-persona reservations cover Java / Kotlin, Go, C / C++, .NET, the
notebook-first data scientist, Ruby / Rails, PHP, and Swift. Each row
records a closed set of `held_reasons` and an
`inclusion_target_if_admitted` so the reservation is never a silent
omission. Promotion of a held persona to P0 is a reviewable change —
it opens a decision row and requires an update to the cutlines file.

## Capability families and target workflows

Every active P0 persona row names:

- **`target_workflows`** — the workflows drawn from the shared
  vocabulary the persona runs through (for example `open`, `search`,
  `rename`, `run_tests`, `debug`, `git_review`, `interpreter_select`,
  `notebook_handoff`, `startup`). These are the same workflow ids
  the reference-workspace rows enumerate in `core_workflows`.
- **`required_capability_families`** — the capability families the
  persona depends on every day (`edit_and_navigate`,
  `language_intelligence`, `run_tests`, `run_or_debug`,
  `git_daily_loop`, `package_and_dependency`, `terminal_and_repl`,
  `build_and_tasks`, `notebook_execution`, `remote_and_container`,
  `recovery_and_restore`, `support_export`).
- **`daily_driver_criteria`** — the closed workflow floor the
  persona MUST run every day, the `support_class_floor` every bound
  bundle MUST hold, the `remote_mode_floor` that names the minimum
  non-local proof (local-only, local-plus-one-remote-mode, local-
  plus-devcontainer-or-container), and the demoting failure modes
  that drop the persona off the daily-driver set.
- **`replacement_grade_criteria`** — the closed proof set from
  `replacement_grade_evidence_vocabulary` the persona's bundles MUST
  carry before any replacement-grade wording is admissible, the
  `support_class_floor` (always `certified` for active P0 rows),
  and a `replacement_target_note` that names the competing tool the
  migration corpus row cites.

A persona that is intentionally ineligible for one proof item (the
Rust self-host persona and `migration_evidence_linked`, for example)
records the omission in `replacement_target_note` and in the
corresponding cutline's `waived_proof_families` block. Silent
omissions are non-conforming.

## Cutline classes

`replacement_grade_cutlines.yaml` admits exactly three cutline
classes:

- **`replacement_grade_cutline`** — admits "switch your X work to
  Aureline" wording. Requires every entry in
  `proof_family_vocabulary` (compatibility-report, certified-
  archetype report, benchmark corpus, migration note, docs-version
  match, support class, known-limit note, claim-manifest row,
  support-export coverage, persona workflow floor). `certified`
  support-class floor. No waiver is admissible for this class.
- **`daily_driver_cutline`** — admits "daily driver for X" wording,
  narrower than replacement-grade. Same support-class ceiling but
  `migration_note_current` may be partial provided the known-limit
  note names the gap explicitly.
- **`self_host_cutline`** — admits "self-host daily driver" wording
  for the internal Rust persona. `migration_note_current` is
  explicitly waived via `waived_proof_families` with a stated
  `waiver_reason`; the waiver is auditable and does not extend to
  any other cutline.

### Required proof families

Every cutline cites the proof families it extends. The closed
vocabulary is:

| Proof family | What it means |
|---|---|
| `compatibility_report_current` | A current compatibility-report row names the archetype. |
| `certified_archetype_report_current` | A current certified-archetype report names the archetype. |
| `benchmark_corpus_current` | Protected corpus scenario rows cite the bundle. |
| `migration_note_current` | Migration notes or a migration corpus row name the competing tool. |
| `docs_version_match_current` | Docs pages carry an exact-build identity match. |
| `support_class_current` | Every bound bundle holds the cutline's `support_class_floor`. |
| `known_limit_note_current` | A known-limits note enumerates the row's scope caveats. |
| `claim_manifest_row_current` | A published claim-manifest row cites the bundle. |
| `support_export_covers_persona` | `support_export` can redact the persona's workspace honestly. |
| `persona_workflow_floor_current` | Every persona target workflow has a current corpus row or fixture. |

A replacement-grade cutline missing any family is wording-ineligible.
Missing proof demotes the row through the cutlines' demotion path
rather than through a softer proof bar locally.

### Downgrade triggers

Each cutline lists the triggers that fire before its
`narrowing_posture` activates. Triggers and proof families are
one-to-one by design: a cutline that requires a proof family but does
not list its corresponding trigger is non-conforming because a stale
proof family would then fail to demote the cutline.

## Narrowing rules

Five closed narrowing rules own the "what gets cut first" decision.
A cutline's `narrowing_posture` extends exactly one rule; inventing a
narrowing path inline is non-conforming. The rules read top-down: the
first rule whose `insufficiency_class` matches the live condition
selects the cutline action.

| Rule id | Insufficiency class | First cut |
|---|---|---|
| `rule:staffing_insufficient.breadth_before_wedge` | `staffing_insufficient` | Cut held personas, then breadth-row bundles, then narrow the Python cutline, then narrow the TypeScript wedge, then narrow Rust self-host, then rebaseline. |
| `rule:benchmark_proof_insufficient.corpus_before_wording` | `benchmark_proof_insufficient` | Withdraw replacement-grade wording on every affected cutline first, then hold the bundle at current support class until corpus refreshes. |
| `rule:migration_proof_insufficient.cut_migration_claim_first` | `migration_proof_insufficient` | Drop replacement-grade wording that names the competing tool; leave the self-host cutline alone (its waiver covers this trigger). |
| `rule:supportability_depth_insufficient.narrow_support_class` | `supportability_depth_insufficient` | Narrow the bundle's support class one step before cutting; block the cutline entirely only when `support_export` cannot redact the persona's workspace at any class. |
| `rule:docs_freshness_insufficient.version_match_before_wording` | `docs_freshness_insufficient` | Withdraw certified / replacement-grade wording when `docs_version_match_current` breaks; route to stale-docs copy on every channel before the wording is re-asserted. |

The rules compose with the language-bundle rubric's `cut_first_posture`
vocabulary: bundles carrying `cut_first_if_m1_slips` are narrowed
before bundles carrying `cut_first_if_m2_slips`, and persona cut order
follows the bundle order when two cutlines share the same insufficiency
class.

## Top-level claim policy

Top-level product claims that name a daily-driver or replacement-
grade persona cite exactly:

- one `persona_id` from `p0_persona_rows.yaml#p0_personas`;
- one `cutline_id` from `replacement_grade_cutlines.yaml#cutlines`.

The following wording forms are non-conforming:

- "daily driver for TypeScript developers" with no persona or cutline
  cite.
- "replacement for your current Python IDE" when
  `cutline:replacement_grade.python_service_or_data_app_developer`
  has any downgrade trigger active.
- "built for Rust developers, replaces X" — the Rust self-host
  cutline intentionally waives migration-target wording; a reviewer
  MUST instead use "self-host daily driver" wording gated on the
  remaining proof families.
- Any persona-naming wording on a held-persona row.

Conforming wording cites both the persona and the cutline:

- "Aureline is a daily-driver environment for the TypeScript web-app
  developer
  (`persona:p0.typescript_web_app_developer`;
  `cutline:replacement_grade.typescript_web_app_developer`) once the
  launch wedge's certified-archetype report lands."
- "The Rust self-host persona
  (`persona:p0.rust_self_host_developer`;
  `cutline:self_host.rust_self_host_developer`) drives the project's
  own daily loop; migration-target wording is intentionally waived."

## Change discipline

- **Add a new active P0 persona** by moving a held-persona row into
  `p0_personas` with a full
  `daily_driver_criteria` block, a full
  `replacement_grade_criteria` block, every required capability
  family, and a cut-first posture. The move opens a decision row in
  `artifacts/governance/decision_index.yaml` and a matching cutline
  row. Silent promotion is non-conforming.
- **Add a new cutline** by binding it to one persona, at least one
  workflow bundle, and exactly one archetype row; every entry in
  `required_proof_families` MUST have a matching trigger in
  `downgrade_triggers`. Replacement-grade cutlines require the full
  proof set with no waivers.
- **Widen a persona or cutline** (adding a bundle, adding an
  archetype, lowering `support_class_floor`, relaxing a required
  proof family) by opening a decision row and updating the persona
  row, the cutline row, and — when the widening touches a published
  claim — a claim-manifest row in the same change.
- **Narrow or withdraw a cutline** by following the demotion path
  already recorded on the row; the demotion MUST be visible in the
  row's history, the affected claim-manifest rows, and the docs
  channel bindings.
- **Hold or cut a persona** by moving the active row back to
  `held_personas` with a named `held_reason` from
  `held_reason_vocabulary` and an
  `inclusion_target_if_admitted`. The corresponding cutline row is
  archived (status only; never deleted) so future reviewers can see
  why the row left the launch set.

## What this contract does not do

- It does not certify any bundle. Certification runs through the
  certified-archetype report template and the archetype rubric.
- It does not publish claim copy. The claim-manifest seed and the
  assurance-claim row schema own the shape of the copy each channel
  renders.
- It does not guarantee a full stable launch bundle at M0. The
  foundations milestone seats the contract; delivering every
  certified cutline in full is explicitly out of scope here and
  tracked by the release-evidence lane.
- It does not expand the launch set. Adding a persona, a cutline, or
  a workflow bundle is always a reviewable change, and the rubric
  keeps the conversation in one place rather than spread across
  docs, release, and support lanes.
