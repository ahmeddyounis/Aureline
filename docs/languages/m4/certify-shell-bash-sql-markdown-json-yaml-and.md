# Certify shell/bash, SQL, Markdown, JSON/YAML, and Git-oriented language tooling at launch support levels — stable contract

Status: Stable lane proof for the shell/bash, SQL, Markdown,
JSON/YAML, and Git-oriented launch-tooling lanes at the M4
launch-support grade.

This document is the reviewer-facing contract for the stable
launch-language tooling truth packet. The packet is the single source
of truth that the editor language pack, framework pack panel, language
settings/help, CLI/headless inspector, support export, release proof
index, Help/About proof card, and the conformance dashboard all read;
surfaces MUST NOT mint local copies or paraphrase launch-tooling
posture.

The launch-tooling lanes are intentionally narrower than the
replacement-grade daily-driver lanes pinned by
`stabilize-typescript-javascript-html-and-css-replacement-grade.md`,
`stabilize-python-daily-driver-quality-with-interpreter-venv.md`,
`stabilize-go-daily-driver-quality-with-modules-workspaces.md`,
`stabilize-rust-daily-driver-quality-with-cargo-workspaces.md`,
`stabilize-java-and-kotlin-daily-driver-quality-with.md`, and
`stabilize-c-and-cpp-daily-driver-quality-with.md`. The launch-support
grade asserts daily-loop coverage on certified archetype repos at the
launch wedge — not full replacement-grade depth across every framework
in the ecosystem.

## What the packet asserts

For each governed *launch-tooling lane × row* the packet asserts:

1. The **tooling lane class** — one of
   `shell_bash_lane`, `sql_lane`, `markdown_lane`, `json_yaml_lane`,
   or `git_oriented_lane`. Every certified packet MUST carry at least
   one row for each of the five required lanes.
2. The **launch-tooling row class** — one of `launch_tooling_quality`,
   `daily_loop_step`, `framework_pack`, `migration_evidence`,
   `archetype_repo_evidence`, `unsupported_gap`, `known_limit`, or
   `downgrade_automation`. A `daily_loop_step` row MUST bind a real
   daily-loop step; no other row class is permitted to bind one.
3. The **support class** — one of `launch_support`,
   `launch_support_below`, `beta_grade_only`, `preview_only`,
   `unsupported`, or `support_unbound`. The validator refuses to
   certify a row that claims `launch_support` while any binding is
   unbound (support, known limit, downgrade automation, or evidence).
4. The **daily-loop step class** — one of `open_or_import`, `navigate`,
   `edit`, `complete`, `refactor`, `run_test_debug`, `review`,
   `migrate`, `recover`, or `not_applicable`. A lane that claims
   `launch_support` MUST cover every certified daily-loop step.
5. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is `evidence_unbound`
   is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `launch_tooling_scope_only`, `unsupported_runtime_target`,
   `beta_capability_sample_only`, or `limit_unbound`. The
   `launch_tooling_scope_only` token is reserved for rows that
   intentionally exclude replacement-grade daily-driver behavior
   because the lane is certified at the launch-tooling wedge only. A
   row whose known limit is `limit_unbound` is refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`, `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`, `auto_narrow_on_framework_gap`,
   `auto_demote_on_low_confidence`, `auto_block_on_missing_evidence`,
   `manual_only_pending_review`, or `automation_unbound`. A row whose
   automation is `automation_unbound` is refused.
8. The **launch-tooling confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `launch_support` at `low_confidence` is narrowed below stable until
   evidence grows.
9. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the launch-tooling claim.
10. The **disclosure ref** — every row that is not `launch_support`,
    that declares a non-`none_declared` known limit, or that binds a
    non-`none` downgrade automation MUST carry a repo-relative
    disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. The packet never admits raw shell scripts,
raw SQL query text, raw Markdown bodies, raw JSON/YAML payloads, Git
commit message bodies, Git credentials, `.gitconfig` secrets,
`~/.netrc` tokens, database connection strings, ambient
`SHELL`/`PATH`/`HISTFILE` values, or any other private material past
the boundary.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `launch_support` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a lane that claims `launch_support` is missing a certified
  `daily_loop_step` row for any of the nine required steps
  (open/import, navigate, edit, complete, refactor, run/test/debug,
  review, migrate, recover),
- a `daily_loop_step` row drops its daily-loop step binding,
- a non-`daily_loop_step` row binds a daily-loop step it cannot
  certify,
- a row narrowed below `launch_support` drops its disclosure ref,
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

## Lane scope

Each launch-tooling lane is certified at the launch wedge with the
following understood scope:

- **`shell_bash_lane`** — POSIX shell and bash editing, navigation
  across sourced files, command and option completion, formatting via
  `shfmt`, linting via ShellCheck, script execution and trace-based
  debugging (`set -x`, `bash -x`), review on certified shell-script
  archetype repos.
- **`sql_lane`** — vendor-agnostic SQL editing for the dialect subset
  enumerated under `language_subset_only`, navigation across schema
  files, keyword and identifier completion, formatter integration,
  query validation and explain-plan-driven review on certified SQL
  archetype repos. Live query execution against production datastores
  is intentionally scoped out under `launch_tooling_scope_only`.
- **`markdown_lane`** — CommonMark and GitHub-Flavored Markdown
  editing, heading/link navigation, completion of reference-style
  links and code-block info strings, format/lint via prettier or
  markdownlint, preview-based review and table-of-contents migration
  on certified Markdown archetype repos.
- **`json_yaml_lane`** — JSON and YAML editing, JSON Schema-driven
  completion and validation, formatter integration, schema-aware
  refactor of identifiers, schema-bound migration of well-known
  configuration files (npm, GitHub Actions, Kubernetes manifest
  subset) on certified archetype repos. Full polyglot YAML anchor
  flow-control debugging is scoped out under
  `launch_tooling_scope_only`.
- **`git_oriented_lane`** — `.gitconfig`, `.gitignore`,
  `.gitattributes`, commit message, and diff/patch authoring. Commit
  message linting (e.g., Conventional Commits), gitignore-pattern
  validation, gitattributes review, and rebase/merge migration on
  certified Git archetype repos. Direct repository mutation is scoped
  out; only on-disk Git-oriented language tooling is certified.

## Consumer projections

The packet declares eight required consumer projections — one per
surface in `ConsumerSurface::REQUIRED`. Each projection preserves the
packet id, the closed vocabularies (lane, row class, support class,
daily-loop step, known limit, downgrade automation, evidence class),
supports JSON export, and excludes raw private material and ambient
authority. The validator emits
`missing_consumer_projection`,
`consumer_projection_drift`,
`lane_vocabulary_collapsed`, `row_class_vocabulary_collapsed`,
`support_class_vocabulary_collapsed`,
`daily_loop_step_vocabulary_collapsed`,
`known_limit_vocabulary_collapsed`,
`downgrade_automation_vocabulary_collapsed`, or
`evidence_class_vocabulary_collapsed` as blockers if a surface drops
or remints the packet.

## Reviewer checklist

- The checked-in stable packet at
  `artifacts/language/m4/launch_language_tooling_truth_packet.json`
  parses, validates, and covers all five required lanes plus all
  eight required consumer projections.
- The Rust contract at
  `crates/aureline-language/src/launch_language_tooling_truth_packet/`
  materializes a stable baseline, narrows the five recorded
  non-baseline postures, and refuses raw source material, secrets, or
  ambient authority past the boundary.
- The fixture corpus at
  `fixtures/language/m4/launch_language_tooling_truth_packet/` pins
  the baseline and the five narrowed-below-stable postures and is
  exercised by the integration tests at
  `crates/aureline-language/tests/launch_language_tooling_truth_packet.rs`.
