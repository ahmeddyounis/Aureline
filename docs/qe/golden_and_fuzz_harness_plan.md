# Rendering/layout golden, accessibility-tree, and protocol-fuzz
# harness plan

This document converts the quality-engineering (QE) strategy seed in
[`docs/qe/qe_strategy_seed.md`](qe_strategy_seed.md) into a concrete
first-harness plan for the three highest-risk correctness surfaces:

- deterministic rendering and layout goldens,
- accessibility-tree regression capture on core shell and editor
  surfaces,
- structure-aware fuzzing for the protocol, parser, index, settings,
  importer, and extension-manifest boundaries.

The narrative lives here. The machine-readable contracts live in:

- [`fixtures/qe/render_golden_manifest.yaml`](../../fixtures/qe/render_golden_manifest.yaml)
  — golden-fixture categories with owner, runner location, artifact
  format, drift posture, and refresh policy.
- [`fixtures/qe/fuzz_target_inventory.yaml`](../../fixtures/qe/fuzz_target_inventory.yaml)
  — fuzz-target rows with target surface, corpus layout, raw-corpus
  retention rule, runner location, and blocking posture.
- [`artifacts/qe/harness_owner_map.yaml`](../../artifacts/qe/harness_owner_map.yaml)
  — harness-family rows binding each category to an owner, a lane in
  [`artifacts/qe/test_lane_registry.yaml`](../../artifacts/qe/test_lane_registry.yaml),
  the artifact sink, and the drift-review forum.

If the narrative here disagrees with any of those files, the files are
authoritative for tooling and this document is updated in the same
change.

This plan composes against the QE lanes — it does not invent new ones.
Every harness family below MUST resolve to at least one lane row in
`artifacts/qe/test_lane_registry.yaml`:

- Rendering and layout goldens → `rendering_layout_goldens`.
- Accessibility-tree regression → `accessibility_regression`.
- Protocol and parser fuzzing → `protocol_parser_fuzzing`.

Scenario coverage MUST cite scenario ids from
[`artifacts/qe/quality_scenario_hooks.yaml`](../../artifacts/qe/quality_scenario_hooks.yaml)
verbatim. A harness row that cannot be named by a scenario id is a
validation failure.

## Operating rule

A harness family is accepted into the plan only when every row can
answer all four of the following without a waiver:

1. Which QE lane does this family run under, and what is the lane's
   declared blocking posture for this row's severity class?
2. Where does a run land its evidence (repo path, archive layout,
   retention policy)?
3. Is the row deterministic, or does it depend on environment posture
   the benchmark-lab quarantine rules already govern — and which
   applies?
4. How is drift reviewed: who signs off on a golden refresh or a
   corpus rotation, under which forum, on which cadence?

A family that cannot answer all four is recorded with
`status: planned_not_yet_seeded` and a freeze-exception or risk-row
reference. It does not become release-gating until the gap closes.

## Golden versus environment-sensitive vocabulary

Harness rows declare one of the following postures. The posture binds
to how drift is reviewed and whether a run can clear under the
benchmark-lab quarantine rules in
[`artifacts/benchmarks/quarantine_rules.yaml`](../../artifacts/benchmarks/quarantine_rules.yaml).

| Posture | Meaning |
|---|---|
| `deterministic_golden` | Output is a byte-exact or structurally-exact match under pinned inputs, display posture, DPI, locale, theme, and toolchain. A mismatch is a review signal, not an environment signal. |
| `structurally_deterministic_with_declared_tolerance` | Output has a declared tolerance window (e.g. rasterization SSIM band, ordering-independent role-group bag equivalence) named on the row. The tolerance is part of the golden; exceeding it is a review signal. |
| `environment_sensitive_observation` | Output depends on host GPU, compositor, AT vendor version, or kernel scheduling, and rides the benchmark-lab quarantine ladder. Surfaces on the QE dashboard only; never release-gating without cross-run clearance. |
| `flake_quarantined_with_repro_required` | Row has produced a non-reproducing failure. The finding is quarantined pending a reduced repro input landing in the raw-corpus directory. |

No other postures are allowed. A row that reports a value outside this
list is non-conforming.

Goldens are deterministic by default. An
`environment_sensitive_observation` row MUST cite its quarantine rule
reference; without one, the row is treated as `flake_quarantined_with_
repro_required`.

## Golden refresh policy

Goldens are refreshed only through one of the following declared
paths. A refresh PR MUST cite one:

- `intentional_change_owner_signoff` — the design, token, or layout
  intent changed, the owner and the accessibility review (when the
  change is AT-visible) sign off, the changed goldens are regenerated
  on the pinned lab image, and the refresh commit cites the source
  PR.
- `token_or_locale_pack_revision` — the token catalog, locale pack,
  or theme/icon/motion package advanced a declared revision; the
  goldens lane regenerates the rows bound to that revision and the
  UX + localization review sign off.
- `toolchain_pinned_revision_bump` — the pinned renderer toolchain,
  font set, or shaper revision advanced; the refresh lands with the
  new pin and a regression-review note describing inspected diffs.
- `incident_learning_row_closure` — a post-incident review recorded
  an `add_golden_row` rule; the fixture lands alongside the fix.

A refresh outside these paths is non-conforming. The goldens lane
MUST NOT accept "regenerate because it failed" as a refresh path.

## Accessibility-tree regression categories

The accessibility-tree harness captures semantic roles, named
regions, focus order, live-region announcements, and assistive-tech
identity on core shell and editor surfaces. Categories below route
through the `accessibility_regression` lane and compose against
[`artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml)
and the platform-input matrix.

- `semantic_roles` — required role-group coverage per surface
  (application/window root, dialog/sheet, searchbox, list/tree,
  toolbar, status/alert region, terminal/log region). A missing
  visible host-owned control is a tree-coverage failure, not an
  allowed optimization.
- `focus_order` — forward and reverse Tab order, focus ring
  continuity across shell zones, focus-return targets after overlay
  or sheet dismiss. Focus-return targets are captured as part of the
  tree snapshot, not inferred.
- `announcements` — live-region announcement shape and cardinality
  (once vs streaming), degraded-state and blocked-state announcement
  wording, shortcut narration on keyboard activation.
- `screen_reader_relevant_structure` — assistive-tech identity lines
  per platform profile (VoiceOver, NVDA, Orca), heading hierarchy,
  landmark regions, and table/grid row-column identity on the
  editor, terminal, diagnostics, and palette surfaces.

Every category row MUST cite at least one checklist id from
`artifacts/accessibility/shell_conformance_checklist.yaml`, at least
one tree-coverage row id from
`artifacts/accessibility/accessibility_tree_coverage_rows.yaml`, and
at least one assistive-tech row id from
`artifacts/accessibility/assistive_tech_matrix.yaml`.

A tree snapshot MUST be comparable across AT vendor revisions by
role-group bag and focus-order list; byte-exact vendor strings are
treated as `environment_sensitive_observation` and ride the
benchmark-lab quarantine ladder rather than gating merge.

## Fuzz-target inventory and raw-corpus retention

Fuzz targets route through `protocol_parser_fuzzing`. The five
initial families are:

- `watcher_stream_events` — filesystem/VFS watcher event streams,
  coalesced and uncoalesced, under rename storms, symlink cycles,
  and partial writes. The target is the watcher decoder and the
  reconciler that produces VFS canonical-path identity.
- `malformed_ipc_or_protocol_payloads` — typed-payload envelopes on
  the RPC boundary, remote-agent hello/heartbeat frames, subscription
  envelopes, and mixed-version negotiation envelopes. The target is
  the envelope decoder and the typed-payload deserializer.
- `corrupted_indexes` — persisted index and cache formats
  (search index, symbol index, VFS metadata cache, settings-resolver
  cache). The target is the loader and the repair path.
- `settings_and_importers` — settings-resolver input, migration
  importers, and legacy-format bridges. The target is the parser, the
  resolver merge, and the importer's downgrade/denial path.
- `extension_manifests` — extension manifest parsers, permission-
  scope deserializers, and compatibility-bridge profile decoders.
  The target is the manifest decoder, the permission-scope evaluator,
  and the host-negotiation admission path.

Every fuzz-target row MUST declare:

- `target_surface` — the function or module under test.
- `input_shape` — the structured grammar or byte-level shape the
  fuzzer generates (schema-derived, grammar-derived, or byte-level).
- `seed_corpus_location` — repo path for curated seed inputs.
- `raw_corpus_retention_rule` — one of:
  - `retain_all_crashing_inputs_forever` (default for crashes,
    panics, and schema-nonconformance under a protected family).
  - `retain_minimised_only_after_triage` (corpus trims to the
    minimised reproducer once the finding is closed).
  - `rotate_on_revision_change` (corpus is regenerated when the
    grammar revision or schema revision advances).
- `runner_location` — where the fuzzer runs (nightly lane harness,
  on-incident lane harness, or pre-release packet generation).
- `artifact_format` — the output shape for a finding (minimised
  input, seed hash, crash class, schema-invariant row reference,
  and any redactions applied under the support-export policy).
- `incident_learning_rule` — drawn from the closed vocabulary in
  `artifacts/qe/test_lane_registry.yaml#incident_learning_rules`
  (default: `add_fuzz_corpus_row`).

A fuzz finding without a minimised reproducer and an invariant-row
reference is a validation failure; the lane MUST NOT accept a
"flake; moving on" close.

## Deterministic vs flaky review path

Harness rows split into two drift-review paths:

- **Deterministic paths** — `deterministic_golden` and
  `structurally_deterministic_with_declared_tolerance` rows. Drift
  is reviewed by the row's owner under the row's declared forum
  (architecture council co-required with accessibility review on
  AT-visible rows, UX review on token/theme rows, localization
  review on locale-pack rows). A refresh follows the refresh-policy
  vocabulary above.
- **Environment-sensitive paths** — `environment_sensitive_
  observation` and `flake_quarantined_with_repro_required` rows.
  Drift rides the benchmark-lab regression-review rubric in
  [`artifacts/benchmarks/quarantine_rules.yaml`](../../artifacts/benchmarks/quarantine_rules.yaml):
  rerun the same `run_context` on the same hardware and image row;
  only escalate to a code regression if the suspect drift does not
  reproduce. A row that cannot reproduce under the rubric becomes
  `flake_quarantined_with_repro_required` until a minimised repro
  lands.

The QE dashboard surfaces both paths but only deterministic paths
produce merge-blocking or release-blocking verdicts under the rules
in [`artifacts/qe/release_blocking_rules.yaml`](../../artifacts/qe/release_blocking_rules.yaml).

## Review forums

- Rendering/layout goldens — architecture council, co-required
  with accessibility review on AT-visible rows, UX review on
  token/theme rows, and localization review on locale-pack rows.
- Accessibility-tree regression — accessibility review,
  co-required with architecture council.
- Protocol and parser fuzzing — security-trust review,
  co-required with architecture council.

A harness family with no declared forum is non-conforming.

## Out of scope

This plan seeds the first corpus inventory and names the contracts;
it does not populate every fixture, stand up full fuzzing depth, or
wire CI enforcement. The first corpus inventory is concrete enough
that CI work can start without re-deciding scope: the golden
categories, the accessibility-tree categories, and the fuzz-target
families are fixed, and each carries an owner, runner location,
artifact format, and refresh policy.

Milestone and task planning metadata MUST NOT appear in any fixture
id, manifest id, runner label, or harness note.
