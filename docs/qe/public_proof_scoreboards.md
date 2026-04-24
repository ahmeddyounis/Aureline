# Public-proof scoreboards

This document is the normative narrative for the seven scoreboard
families the launch-wedge contract reads before any replacement-grade,
daily-driver, or certified wording is even tentatively used.

If the narrative and the machine-readable register disagree,
`artifacts/qe/workflow_bundle_ids.yaml` is authoritative for tooling
and this document is updated in the same change. Packets conform to
`schemas/qe/public_proof_packet.schema.json`.

## Why this exists

Benchmark claims, migration proof, compatibility reports, docs freshness,
and support-export coverage have each previously been asserted in their
own ad hoc artifact. The launch-wedge contract
(`docs/product/launch_wedge_contract.md`) and the replacement-grade
cutlines (`artifacts/product/replacement_grade_cutlines.yaml`) already
name which proof families a cutline extends; the missing piece was a
single scoreboard family a cutline row could point to, and a single
packet shape that proof could be emitted as, so that:

- each cutline points to **one scoreboard family and one packet shape**
  per proof family rather than minting a bespoke bundle;
- benchmark, migration, and compatibility evidence share **one set of
  workflow-bundle and archetype IDs** without renaming or
  reinterpretation across lanes or milestones; and
- public-proof packets are detailed enough — fixture owner, environment
  / build identity, task script or success criterion, result diff
  versus prior claim state, known limits or exclusions, docs/help
  version match, and freshness metadata — to support narrowing,
  downgrade, and repeatability decisions from the packet alone.

## Upstream sources this narrative composes with

This document deliberately does not restate the rules of its upstream
sources. It composes with:

- launch-wedge contract — `docs/product/launch_wedge_contract.md`
- P0 persona rows — `artifacts/product/p0_persona_rows.yaml`
- replacement-grade cutlines —
  `artifacts/product/replacement_grade_cutlines.yaml`
- language bundle rows — `artifacts/product/language_bundle_rows.yaml`
- archetype rubric — `artifacts/compat/archetype_rubric.yaml`
- reference-workspace rows —
  `artifacts/compat/reference_workspace_rows.yaml`
- compatibility-row schema — `schemas/release/compatibility_row.schema.json`
- compatibility-report template —
  `docs/release/compatibility_report_template.md`
- certified-archetype report template —
  `docs/release/certified_archetype_report_template.md`
- benchmark-publication pack template —
  `docs/benchmarks/benchmark_publication_pack_template.md`
- benchmark run-result schema — `schemas/benchmarks/run_result.schema.json`
- protected corpus manifest — `fixtures/benchmarks/corpus_manifest.yaml`
- fitness-function catalog — `artifacts/bench/fitness_function_catalog.yaml`
- evidence freshness SLOs —
  `artifacts/governance/evidence_freshness_slos.yaml`
- evidence rerun triggers —
  `artifacts/governance/evidence_rerun_triggers.yaml`
- assurance-claim matrix —
  `artifacts/release/assurance_claim_rows.yaml` and
  `docs/release/assurance_claim_matrix.md`
- claim-manifest seed —
  `artifacts/governance/claim_manifest_seed.yaml`

All closed vocabularies below are quoted verbatim from these sources.

## The seven scoreboard families

Each family qualifies one slice of the launch-wedge contract's required
proof set. Every family has exactly one packet shape, named so the
packet discriminator is obvious from context. Readers reach the
admissible proof surfaces, proof classes, downgrade triggers, and
ownership lanes from the register file; this table only fixes the
single-line identity.

| Scoreboard family | Packet shape | Qualifies |
|---|---|---|
| `bootstrap_and_entry_parity_scoreboard` | `bootstrap_entry_parity_packet` | First-run → first-useful-edit paths under the persona's required support class. |
| `migration_fidelity_scoreboard` | `migration_fidelity_packet` | Dry-run import / recorded diff / migration-note reproduction on the reference workspace. |
| `task_run_test_debug_parity_scoreboard` | `task_run_test_debug_parity_packet` | Task / run / test / debug / REPL workflow-floor parity. |
| `extension_or_package_bridge_parity_scoreboard` | `extension_or_package_bridge_parity_packet` | Extension-host and package-manager bridge parity at the bundle revision. |
| `workflow_bundle_or_archetype_proof_scoreboard` | `workflow_bundle_or_archetype_proof_packet` | Certified-archetype report / reference-workspace proof at the support class. |
| `benchmark_and_public_proof_packet_scoreboard` | `benchmark_public_proof_packet` | Benchmark-lab run results, publishable benchmark packs, public head-to-head comparisons. |
| `docs_known_limits_support_copy_alignment_scoreboard` | `docs_known_limits_support_copy_alignment_packet` | Docs / help / release-notes / known-limits / support-export copy alignment with the cutline's claim. |

### Pairing with the replacement-grade cutlines

Each seeded cutline row points to exactly one scoreboard family and
one packet shape per admissible family. The pairings live in
`artifacts/qe/workflow_bundle_ids.yaml#cutline_scoreboard_pairings` and
are enforced by the `invariant:every_active_cutline_has_full_pairing`
invariant. The Rust self-host cutline intentionally waives the
migration-fidelity scoreboard; the waiver is recorded in
`waived_pairings` with a `waiver_reason` so the absence is never a
silent weakening.

## Versioned workflow-bundle and archetype IDs

The contract deliberately does not mint new bundle or archetype
identifiers. It re-exports the bundle IDs from
`artifacts/product/language_bundle_rows.yaml` and the archetype row
IDs from `artifacts/compat/reference_workspace_rows.yaml`, and adds
`bundle_revision` and `archetype_revision` integer fields so benchmark,
compatibility, migration, docs, and support rows can cite the same
identity across milestones without renaming.

Rules:

- **Append-only revisions.** Bumping a `bundle_revision` or
  `archetype_revision` is additive-minor. Renaming a `bundle_id` or
  `archetype_row_id` is breaking and opens a decision row in
  `artifacts/governance/decision_index.yaml`.
- **One identity across lanes.** A benchmark-lab run, a compatibility
  report, a migration note, a docs-pack anchor, and a support-export
  row that cite the same `(bundle_id, bundle_revision)` tuple MUST be
  talking about the same bundle.
- **Waiver parity.** A cutline's `waived_proof_families` entry MUST be
  mirrored by a `waived_scoreboard_families` entry on the bound bundle
  row and by a `waived_pairings` entry on the cutline's
  scoreboard-pairing row.

## Packet contract

A public-proof packet is one record conforming to
`schemas/qe/public_proof_packet.schema.json`. The schema pins:

- **Identity.** `packet_id`, `packet_state`, `scoreboard_family_id`,
  `packet_shape` (the family-shape pairing is enforced by `allOf`),
  `cutline_ref`, `persona_ref`.
- **Versioned bundle / archetype.** `workflow_bundle_ref` (bundle id +
  revision) and `archetype_row_ref` (row id + revision).
- **Environment / build identity.** `exact_build_identity_ref`,
  `release_channel_class`, `workspace_version`, optional
  `hardware_definition_ref` and `environment_definition_ref` (required
  on `benchmark_public_proof_packet`), and the deployment-profile ids
  the packet applies to.
- **Task script or success criterion.** `task_script.task_script_kind`
  from a closed set, plus an opaque `task_summary_ref`, optional
  `success_criterion_ref`, and `corpus_refs` / `reference_workspace_refs`
  / `protected_path_refs`.
- **Result.** `result_class` from the closed set, plus
  `active_downgrade_reasons` drawn from the assurance-claim downgrade
  vocabulary; `pass_full_proof` forbids active reasons;
  `narrow_claim_before_publish`, `retest_pending`, `fail_claim_blocked`,
  and `quarantined` require at least one.
- **Result diff vs prior claim state.** `prior_claim_diff` names one
  closed `prior_claim_state_class`, a non-null `prior_claim_state_ref`
  on every class except `first_emission_no_prior_claim`, a typed
  `diff_summary_ref`, and typed deltas for `result_class` and
  `support_class`.
- **Known limits and exclusions.** `known_limits_and_exclusions.declared_class`
  from the closed set; `no_limits_declared` MUST carry an empty notes
  list, every other class MUST carry at least one note.
- **Docs/help version match.** `docs_help_version_match.state` from the
  closed set; `not_applicable` forbids a pack revision pin; every
  other state requires one.
- **Freshness.** `captured_at`, optional `stale_after`, `cadence_class`,
  `proof_class`, and at least one `rerun_trigger_ref` into
  `artifacts/governance/evidence_rerun_triggers.yaml`.
- **Publication envelope.** `publication_posture` from the closed set;
  only `benchmark_public_proof_packet` MAY carry
  `public_head_to_head_comparison`, and that posture MUST carry a
  `competitor_settings` block (every other posture MUST NOT).
- **Ownership.** `fixture_owner_ref`, `evidence_owner_ref`, and
  `owner_ref` resolving through
  `artifacts/governance/ownership_matrix.yaml`.

Raw marketing copy, raw trace bodies, raw log bodies, and raw
credential material never cross this boundary.

### Detail bar for narrowing, downgrade, and repeatability

The packet schema is deliberately detailed enough that three decisions
can be made from the packet alone:

- **Narrowing.** A reviewer reads `result_class`,
  `active_downgrade_reasons`, and `known_limits_and_exclusions` and
  can tell which claim class the packet still supports without reading
  marketing copy.
- **Downgrade.** A reviewer reads `prior_claim_diff.result_class_delta`
  and `prior_claim_diff.support_class_delta` and can tell whether the
  packet narrows, widens, supersedes, or withdraws the prior claim
  state; widening on a packet whose state is narrowed / retest_pending
  / quarantined / withdrawn is non-conforming.
- **Repeatability.** A reviewer reads `environment_ref` plus
  `task_script` plus `publication_envelope.competitor_settings` and
  can tell whether the packet is reproducible; benchmark packets with
  a head-to-head comparison carry the competitor block, everything
  else does not.

## How the scoreboards narrow a cutline

The cutline's `narrowing_posture` (from
`artifacts/product/replacement_grade_cutlines.yaml`) names which
ordered cuts fire when a proof family slips. The scoreboard family's
`downgrade_triggers` list names the triggers a packet may flag. When
a packet flags a trigger, the cutline follows the trigger's matching
narrowing rule:

- `benchmark_corpus_missing_or_stale` on the benchmark / public-proof
  scoreboard → `rule:benchmark_proof_insufficient.corpus_before_wording`.
- `migration_note_missing_or_stale` on the migration-fidelity
  scoreboard → `rule:migration_proof_insufficient.cut_migration_claim_first`.
- `support_export_cannot_redact_persona_workspace` on any scoreboard →
  `rule:supportability_depth_insufficient.narrow_support_class`.
- `docs_version_match_unmet` or `docs_freshness_floor_unmet` on the
  docs / known-limits / support-copy scoreboard →
  `rule:docs_freshness_insufficient.version_match_before_wording`.
- Any `required_evidence_*` trigger on any scoreboard → the cutline's
  demotion_path drops one class (replacement_grade → daily_driver →
  supported_with_caveats → withdrawn) as the cutline's
  `narrowing_posture` requires.

The trigger-to-rule pairing is enforced by the existing cutline
`invariant:every_cutline_lists_downgrade_triggers_matching_proof`; the
scoreboard narrative here just names the round-trip so reviewers can
read it in one place.

## Acceptance mapping

The contract satisfies the acceptance criteria in the launch-wedge
spec as follows:

- **Each seeded launch-wedge row points to one scoreboard family and
  one packet shape.** Enforced by
  `invariant:every_active_cutline_has_full_pairing` in
  `artifacts/qe/workflow_bundle_ids.yaml`.
- **Benchmark, migration, and compatibility evidence share bundle IDs
  without renaming.** Enforced by
  `invariant:every_workflow_bundle_pins_revision_and_archetype` plus
  the rule that bundle / archetype ids are quoted verbatim from their
  upstream registers.
- **Public-proof packets are detailed enough for narrowing, downgrade,
  and repeatability decisions.** Enforced by the `allOf` gates in
  `schemas/qe/public_proof_packet.schema.json` (family-shape pairing,
  head-to-head posture requires competitor block, pass_full_proof
  forbids active downgrade reasons, known-limits class ↔ notes
  consistency, docs version-match state ↔ pack revision consistency,
  prior-claim-diff class ↔ prior-state ref consistency, benchmark
  shape requires hardware + environment).

## Change discipline

- Adding a scoreboard family, a packet shape, a workflow bundle, an
  archetype revision, a result class, a publication posture, a known-
  limit class, or a docs version-match state is **additive-minor**;
  the value lands in the schema, the register, and this narrative in
  the same change.
- Renaming or repurposing any existing value in the above vocabularies
  is **breaking** and requires a decision row in
  `artifacts/governance/decision_index.yaml`.
- Widening a cutline's scoreboard pairing (promoting a waived family,
  adding a bundle, lowering a bundle's support_class_floor) requires a
  matching decision row plus a claim-manifest row update.
- Retiring a scoreboard family is not a delete; set the family row's
  `lifecycle_state` to retired and record the supersede note so the
  audit trail "this family once applied" survives.

## Out of scope

This milestone deliberately does not stand up:

- a public scoreboard website,
- continuous scoreboard emission from the runtime,
- automated packet generation by CI, or
- any marketing-copy rendering pipeline.

Those are later-milestone concerns. The contract here is the frozen
structure the runtime and CI lanes will emit against.
