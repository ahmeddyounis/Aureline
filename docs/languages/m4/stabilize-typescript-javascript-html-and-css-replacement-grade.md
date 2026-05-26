# Stabilize TypeScript, JavaScript, HTML, and CSS replacement-grade daily-driver quality — stable contract

Status: Stable lane proof for the TypeScript, JavaScript, HTML, and CSS
replacement-grade daily-driver quality wedges.

This document is the reviewer-facing contract for the stable
daily-driver quality truth packet. The packet is the single source of
truth that the editor language pack, framework pack panel, language
settings/help, CLI/headless inspector, support export, release proof
index, Help/About proof card, and the conformance dashboard all read;
surfaces MUST NOT mint local copies or paraphrase daily-driver posture.

## What the packet asserts

For each governed *language lane × daily-driver row* the packet asserts:

1. The **language lane class** — one of
   `typescript_daily_driver_lane`, `javascript_daily_driver_lane`,
   `html_daily_driver_lane`, `css_daily_driver_lane`. Every certified
   packet MUST carry at least one row for each of the four required
   lanes.
2. The **daily-driver row class** — one of `daily_driver_quality`,
   `daily_loop_step`, `framework_pack`, `migration_evidence`,
   `archetype_repo_evidence`, `unsupported_gap`, `known_limit`, or
   `downgrade_automation`. A `daily_loop_step` row MUST bind a real
   daily-loop step; no other row class is permitted to bind one.
3. The **support class** — one of `replacement_grade`,
   `daily_driver_below_replacement`, `beta_grade_only`, `preview_only`,
   `unsupported`, or `support_unbound`. The validator refuses to
   certify a row that claims `replacement_grade` while any binding is
   unbound (support, known limit, downgrade automation, or evidence).
4. The **daily-loop step class** — one of `open_or_import`, `navigate`,
   `edit`, `complete`, `refactor`, `run_test_debug`, `review`,
   `migrate`, `recover`, or `not_applicable`. A lane that claims
   `replacement_grade` daily-driver quality MUST cover every certified
   daily-loop step.
5. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is `evidence_unbound`
   is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `unsupported_runtime_target`, `beta_capability_sample_only`, or
   `limit_unbound`. A row whose known limit is `limit_unbound` is
   refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`, `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`, `auto_narrow_on_framework_gap`,
   `auto_demote_on_low_confidence`, `auto_block_on_missing_evidence`,
   `manual_only_pending_review`, or `automation_unbound`. A row whose
   automation is `automation_unbound` is refused.
8. The **daily-driver confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `replacement_grade` at `low_confidence` is narrowed below stable
   until evidence grows.
9. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the daily-driver claim.
10. The **disclosure ref** — every row that is not
    `replacement_grade`, that declares a non-`none_declared` known
    limit, or that binds a non-`none` downgrade automation MUST carry a
    repo-relative disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. The packet never admits raw source bodies,
secrets, ambient credentials, or provider payloads.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `replacement_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a lane that claims `replacement_grade` daily-driver quality is
  missing a certified `daily_loop_step` row for any of the nine
  required steps (open/import, navigate, edit, complete, refactor,
  run/test/debug, review, migrate, recover),
- a `daily_loop_step` row drops its daily-loop step binding,
- a non-`daily_loop_step` row binds a daily-loop step it cannot
  certify,
- a row narrowed below `replacement_grade` drops its disclosure ref,
- a row declares a non-`none_declared` known limit and drops its
  disclosure ref,
- a row binds a non-`none` downgrade automation and drops its
  disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies (lane, row class, support
  class, daily-loop step, known limit, downgrade automation, or
  evidence class),
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## Required consumer projections

The packet REQUIRES one preserved projection per surface:
`editor_language_pack`, `framework_pack_panel`, `language_settings`,
`cli_headless`, `support_export`, `release_proof_index`, `help_about`,
and `conformance_dashboard`. Each projection MUST keep the lane class,
row class, support class, daily-loop-step class, known-limit class,
downgrade-automation class, and evidence class verbatim, MUST support
JSON export, and MUST exclude raw private material and ambient
authority.

## How to read the packet

Consumers materialize the packet through
`DailyDriverQualityTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only and
suitable for inclusion in any support export or release proof bundle.

## Closed vocabulary

**Language lane classes** — `typescript_daily_driver_lane`,
`javascript_daily_driver_lane`, `html_daily_driver_lane`,
`css_daily_driver_lane`.

**Daily-driver row classes** — `daily_driver_quality`,
`daily_loop_step`, `framework_pack`, `migration_evidence`,
`archetype_repo_evidence`, `unsupported_gap`, `known_limit`,
`downgrade_automation`.

**Support classes** — `replacement_grade`,
`daily_driver_below_replacement`, `beta_grade_only`, `preview_only`,
`unsupported`, `support_unbound`.

**Daily-loop step classes** — `open_or_import`, `navigate`, `edit`,
`complete`, `refactor`, `run_test_debug`, `review`, `migrate`,
`recover`, `not_applicable`.

**Evidence classes** — `archetype_repo_evidence`,
`framework_migration_evidence`, `design_partner_evidence`,
`fixture_repo_evidence`, `conformance_suite_evidence`,
`benchmark_evidence`, `docs_disclosure_evidence`, `evidence_unbound`.

**Known-limit classes** — `none_declared`, `framework_subset_only`,
`language_subset_only`, `archetype_subset_only`,
`migration_subset_only`, `unsupported_runtime_target`,
`beta_capability_sample_only`, `limit_unbound`.

**Downgrade-automation classes** — `none`,
`auto_narrow_on_missing_fixture`, `auto_narrow_on_missing_archetype`,
`auto_narrow_on_failed_migration`, `auto_narrow_on_framework_gap`,
`auto_demote_on_low_confidence`, `auto_block_on_missing_evidence`,
`manual_only_pending_review`, `automation_unbound`.

**Consumer surfaces** — `editor_language_pack`, `framework_pack_panel`,
`language_settings`, `cli_headless`, `support_export`,
`release_proof_index`, `help_about`, `conformance_dashboard`.

**Finding kinds** — see
`schemas/language/daily_driver_quality_truth.schema.json` for the
closed list. Notable invariants:

- `raw_source_material_present`, `secrets_present`,
  `ambient_authority_present` — boundary safety.
- `missing_support_class`, `missing_known_limit`,
  `missing_downgrade_automation`, `missing_evidence_class`,
  `replacement_grade_with_unbound_binding` — the row claims a
  replacement-grade daily-driver wedge without binding the support,
  limit, automation, or evidence needed to defend it.
- `missing_daily_loop_step_coverage`, `daily_loop_step_not_applicable`,
  `daily_loop_step_not_permitted_on_row_class` — daily-loop coverage
  drift.
- `narrowed_row_missing_disclosure_ref`,
  `known_limit_missing_disclosure_ref`,
  `downgrade_automation_missing_disclosure_ref` — required disclosure
  refs were dropped from the row.
- `missing_consumer_projection`, `consumer_projection_drift`,
  `lane_vocabulary_collapsed`, `row_class_vocabulary_collapsed`,
  `support_class_vocabulary_collapsed`,
  `daily_loop_step_vocabulary_collapsed`,
  `known_limit_vocabulary_collapsed`,
  `downgrade_automation_vocabulary_collapsed`,
  `evidence_class_vocabulary_collapsed` — surface drift.

## Companion artifacts

- Schema: `schemas/language/daily_driver_quality_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/daily_driver_quality_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/daily_driver_quality_truth_packet/`
- Reviewer artifact:
  `artifacts/language/m4/stabilize-typescript-javascript-html-and-css-replacement-grade.md`
- Rust contract:
  `crates/aureline-language/src/daily_driver_quality_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/daily_driver_quality_truth_packet.rs`

## Anchored normative sources

- `.t2/docs/Aureline_PRD.md` Appendix P LANG-WEB-001 and LANG-CORE-001
  — TypeScript, JavaScript, HTML, and CSS launch-language and
  framework support-class requirements that the daily-driver packet
  binds.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — launch
  language platform and framework support architecture.
- `.t2/docs/Aureline_Technical_Design_Document.md` — language and
  framework component contracts and milestone scorecard sections.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — editor, language, and
  conformance surface rules for displaying support class, known
  limits, and downgrade state.
- `.t2/docs/Aureline_Milestones_Document.md` §6.3–§6.4, §6.6,
  §7.5–§7.6, §9.5 — milestone fit, dependency intake, and exit-gate
  anchors for the daily-driver quality wedge.

If any of those sources disagree with this document, the source wins
and this document plus the schema and fixtures MUST be updated in the
same change.

#auto_block_on_missing_evidence

A `daily_driver_quality` row pairs `auto_block_on_missing_evidence`
with the daily-driver claim itself: if any required evidence ref drops
from the row, the lane blocks publication rather than inherit an
adjacent replacement-grade row. This is how the packet refuses to let
a beta-grade capability sample masquerade as a replacement-grade daily
driver.

#auto_narrow_on_missing_fixture

A `daily_loop_step` row pairs `auto_narrow_on_missing_fixture` with
fixture-repo evidence: when a certified fixture is missing or stale,
the lane auto-narrows the published step claim and surfaces the
disclosure ref shown in this document.

#auto_narrow_on_failed_migration

The `migrate` step row pairs `auto_narrow_on_failed_migration` with
framework-migration evidence: when a migration probe fails on any
certified archetype repo, the lane auto-narrows below replacement
grade until the regression is repaired.

#framework_subset_only

The `framework_subset_only` known-limit class is paired with the
`auto_narrow_on_framework_gap` downgrade automation: when a framework
pack drops below the certified depth (e.g., a recently certified
framework version regresses), the lane auto-narrows the published
support class so a replacement-grade label never inherits a beta-grade
framework gap.

#language_subset_only

The `language_subset_only` known-limit class is paired with the
`auto_narrow_on_framework_gap` downgrade automation: when a language
subset (for example, a specific CSS feature mode) is not yet covered
by the certified daily-driver rows, the lane narrows below replacement
grade with the precise unsupported gap disclosed in the row.

#migration_subset_only

The `migration_subset_only` known-limit class is paired with the
`auto_narrow_on_failed_migration` downgrade automation: when only a
subset of certified migrations is currently passing, the lane
auto-narrows below replacement grade and discloses which migrations
remain certified.

#archetype_subset_only

The `archetype_subset_only` known-limit class is paired with the
`auto_narrow_on_missing_archetype` downgrade automation: when a
certified archetype repo is missing or skipped, the lane auto-narrows
below replacement grade and discloses which archetypes remain
certified.
