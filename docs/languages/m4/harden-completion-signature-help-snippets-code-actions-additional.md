# Harden completion, signature help, snippets, code actions, additional edits, source labeling, and AI-ghost-text boundaries across launch languages — stable contract

Status: Stable lane proof for completion, signature help, snippet
session, code action, additional edit, source labeling, and AI
ghost-text assistant-surface lanes at the M4 launch-hardened grade.

This document is the reviewer-facing contract for the stable
assistant-surface hardening truth packet. The packet is the single
source of truth that the editor language pack, framework pack panel,
language settings/help, CLI/headless inspector, support export,
release proof index, Help/About proof card, and the conformance
dashboard all read; surfaces MUST NOT mint local copies or paraphrase
assistant-surface posture.

The assistant-surface lanes here are orthogonal to the launch-language
daily-driver and launch-tooling packets pinned by
`stabilize-typescript-javascript-html-and-css-replacement-grade.md`,
`stabilize-python-daily-driver-quality-with-interpreter-venv.md`, and
`certify-shell-bash-sql-markdown-json-yaml-and.md`. Those packets
certify the per-language daily loop; this packet certifies that the
assistant surfaces those lanes share — completion, signature help,
snippet sessions, code actions, additional edits, source labeling, and
AI ghost text — are hardened across launch languages with provider/
source honesty, side-effect admission, snippet-session truth, code-
action truth, and cross-cut condition coverage.

## What the packet asserts

For each governed *assistant-surface lane × row* the packet asserts:

1. The **assistant-surface lane class** — one of `completion_lane`,
   `signature_help_lane`, `snippet_session_lane`, `code_action_lane`,
   `additional_edit_lane`, `source_labeling_lane`, or
   `ai_ghost_text_lane`. Every certified packet MUST carry at least
   one row for each of the seven required lanes.
2. The **assistant-surface row class** — one of
   `assistant_surface_quality`, `provider_source_binding`,
   `side_effect_admission`, `snippet_session_truth`,
   `code_action_truth`, `cross_cut_condition`,
   `launch_language_coverage`, `unsupported_gap`, `known_limit`, or
   `downgrade_automation`. Each binding row class MUST bind exactly
   the corresponding field; no other row class is permitted to bind
   one.
3. The **support class** — one of `launch_hardened`,
   `launch_hardened_below`, `beta_grade_only`, `preview_only`,
   `unsupported`, or `support_unbound`. The validator refuses to
   certify a row that claims `launch_hardened` while any binding is
   unbound (support, known limit, downgrade automation, evidence,
   side-effect, or preview-requirement).
4. The **provider/source class** — one of
   `deterministic_completion`, `cached_local_word_fallback`,
   `snippet_only_suggestion`, `ai_ghost_text`, or `not_applicable`.
   The `completion_lane`, `source_labeling_lane`, and
   `ai_ghost_text_lane` MUST bind every required provider/source class
   when claiming `launch_hardened` so deterministic completion versus
   AI ghost text is distinguished by stable provider/source rather
   than by theme accident.
5. The **side-effect class** — one of `no_side_effect`,
   `additional_edits`, `generated_files`,
   `dependency_or_config_change`, `protected_surface_write`,
   `not_applicable`, or `side_effect_unbound`. Accepting any proposal
   with additional edits, generated files, dependency/config changes,
   or protected-surface writes MUST expose its side-effect class
   before apply.
6. The **preview-requirement class** — one of
   `preview_not_required`, `preview_required_for_multi_file`,
   `preview_required_for_generated_file`,
   `preview_required_for_dependency_change`,
   `preview_required_for_protected_surface`, `not_applicable`, or
   `preview_unbound`. Multi-file, generated-file, dependency-changing,
   or protected-surface code actions MUST require preview before
   apply.
7. The **snippet-session field class** — one of
   `snippet_or_source_label`, `placeholder_index_count`, `exit_route`,
   `multi_cursor_compatibility`, or `not_applicable`. The
   `snippet_session_lane` MUST cover every required snippet-session
   field when claiming `launch_hardened` so snippet mode cannot
   silently hijack Tab semantics outside an active session or strand
   keyboard users without an exit path.
8. The **code-action field class** — one of `provider_or_source`,
   `side_effect_class`, `partial_support_reason`,
   `preview_requirement`, or `not_applicable`. The `code_action_lane`
   MUST cover every required code-action field when claiming
   `launch_hardened` so code-action entries preserve provider/source,
   side-effect class, partial-support reason, and preview requirement.
9. The **cross-cut condition class** — one of `ime`, `multi_cursor`,
   `large_file`, `restricted_mode`, `degraded_provider`, or
   `not_applicable`. A lane that claims `launch_hardened` MUST cover
   every cross-cut condition required by the launch addendum.
10. The **evidence class** — one of `archetype_repo_evidence`,
    `framework_migration_evidence`, `design_partner_evidence`,
    `fixture_repo_evidence`, `conformance_suite_evidence`,
    `benchmark_evidence`, `docs_disclosure_evidence`, or
    `evidence_unbound`. A row whose evidence class is
    `evidence_unbound` is refused.
11. The **known-limit class** — one of `none_declared`,
    `provider_subset_only`, `language_subset_only`,
    `archetype_subset_only`, `condition_subset_only`,
    `field_subset_only`, `side_effect_scope_only`,
    `preview_subset_only`, `unsupported_runtime_target`,
    `beta_capability_sample_only`, or `limit_unbound`. A row whose
    known limit is `limit_unbound` is refused.
12. The **downgrade-automation class** — one of `none`,
    `auto_narrow_on_missing_fixture`,
    `auto_narrow_on_missing_archetype`, `auto_narrow_on_provider_gap`,
    `auto_narrow_on_condition_failure`,
    `auto_narrow_on_ghost_text_label_drift`,
    `auto_demote_on_low_confidence`, `auto_block_on_missing_evidence`,
    `manual_only_pending_review`, or `automation_unbound`. A row whose
    automation is `automation_unbound` is refused.
13. The **assistant-surface confidence class** — `high_confidence`,
    `medium_confidence`, or `low_confidence`. A row that claims
    `launch_hardened` at `low_confidence` is narrowed below stable
    until evidence grows.
14. The **evidence refs** — every row preserves at least one
    repo-relative evidence ref proving the assistant-surface claim.
15. The **disclosure ref** — every row that is not `launch_hardened`,
    that declares a non-`none_declared` known limit, or that binds a
    non-`none` downgrade automation MUST carry a repo-relative
    disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. The packet never admits raw completion
proposals, raw snippet bodies, raw ghost-text output, raw partial-
support reasons, raw code-action commands, secrets, ambient editor
credentials, or any other private material past the boundary.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `launch_hardened` while its support, known-limit,
  downgrade-automation, evidence, side-effect, or preview-requirement
  class is unbound,
- a lane that claims `launch_hardened` is missing a certified
  `cross_cut_condition` row for any of the five required conditions
  (IME, multi-cursor, large-file, restricted-mode, degraded-provider),
- the completion, source labeling, or AI ghost-text lane claims
  `launch_hardened` without a `provider_source_binding` row for each
  of `deterministic_completion`, `cached_local_word_fallback`,
  `snippet_only_suggestion`, and `ai_ghost_text`,
- the snippet-session lane claims `launch_hardened` without a
  `snippet_session_truth` row for each of `snippet_or_source_label`,
  `placeholder_index_count`, `exit_route`, and
  `multi_cursor_compatibility`,
- the code-action lane claims `launch_hardened` without a
  `code_action_truth` row for each of `provider_or_source`,
  `side_effect_class`, `partial_support_reason`, and
  `preview_requirement`,
- a binding-typed row drops its field binding or a non-binding row
  binds a field it cannot certify,
- a row narrowed below `launch_hardened` drops its disclosure ref,
- a row declares a non-`none_declared` known limit and drops its
  disclosure ref,
- a row binds a non-`none` downgrade automation and drops its
  disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies (lane, row class, support
  class, provider/source class, side-effect class, preview-
  requirement, snippet-session field, code-action field, cross-cut
  condition, known limit, downgrade automation, or evidence class),
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## Lane scope

Each assistant-surface lane is certified across launch languages with
the following understood scope:

- **`completion_lane`** — deterministic completion from a language
  server, parser, or schema, plus the cached/local-word fallback
  proposal class. Provider/source class is bound on every entry; the
  apply path exposes side-effect class when additional edits accompany
  the proposal.
- **`signature_help_lane`** — signature help that surfaces parameter
  index/count and the active parameter; bounded to the lane's
  cross-cut condition coverage so signature help renders under IME,
  multi-cursor, large-file, restricted-mode, and degraded-provider
  conditions.
- **`snippet_session_lane`** — snippet session entry/exit, placeholder
  navigation, and Tab semantics. Each session surfaces snippet/source
  label, placeholder index/count, exit route, and multi-cursor
  compatibility so snippet mode cannot silently hijack Tab outside an
  active session or strand keyboard users.
- **`code_action_lane`** — code-action entry catalog. Each entry
  preserves provider/source, side-effect class, partial-support
  reason, and preview requirement. Multi-file, generated-file,
  dependency-changing, or protected-surface mutations require preview
  before apply.
- **`additional_edit_lane`** — admission of side-effect-bearing apply
  paths. Any proposal carrying additional edits, generated files,
  dependency/config changes, or protected-surface writes is admitted
  through this lane with a bound side-effect class and a bound
  preview-requirement class.
- **`source_labeling_lane`** — provider/source attribution rendered on
  every proposal. Deterministic completion, cached/local-word
  fallback, snippet-only suggestion, and AI ghost text are
  distinguished by stable provider/source rather than by theme
  accident.
- **`ai_ghost_text_lane`** — AI ghost-text inline proposals. Always
  rendered as `ai_ghost_text` provider/source; never collapsed into
  deterministic-completion or snippet-only paths.

## Consumer projections

The packet declares eight required consumer projections — one per
surface in `ConsumerSurface::REQUIRED`. Each projection preserves the
packet id, the closed vocabularies (lane, row class, support class,
provider/source class, side-effect class, preview-requirement,
snippet-session field, code-action field, cross-cut condition, known
limit, downgrade automation, evidence class), supports JSON export,
and excludes raw private material and ambient authority. The validator
emits `missing_consumer_projection`, `consumer_projection_drift`,
`lane_vocabulary_collapsed`, `row_class_vocabulary_collapsed`,
`support_class_vocabulary_collapsed`,
`provider_source_class_vocabulary_collapsed`,
`side_effect_class_vocabulary_collapsed`,
`preview_requirement_vocabulary_collapsed`,
`snippet_session_field_vocabulary_collapsed`,
`code_action_field_vocabulary_collapsed`,
`cross_cut_condition_vocabulary_collapsed`,
`known_limit_vocabulary_collapsed`,
`downgrade_automation_vocabulary_collapsed`, or
`evidence_class_vocabulary_collapsed` as blockers if a surface drops
or remints the packet.

## Reviewer checklist

- The checked-in stable packet at
  `artifacts/language/m4/assistant_surface_hardening_truth_packet.json`
  parses, validates, and covers all seven required lanes plus all
  eight required consumer projections.
- The Rust contract at
  `crates/aureline-language/src/assistant_surface_hardening_truth_packet/`
  materializes a stable baseline, narrows the recorded
  non-baseline postures, and refuses raw source material, secrets, or
  ambient authority past the boundary.
- The fixture corpus at
  `fixtures/language/m4/assistant_surface_hardening_truth_packet/`
  pins the baseline and the five narrowed-below-stable postures and
  is exercised by the integration tests at
  `crates/aureline-language/tests/assistant_surface_hardening_truth_packet.rs`.
