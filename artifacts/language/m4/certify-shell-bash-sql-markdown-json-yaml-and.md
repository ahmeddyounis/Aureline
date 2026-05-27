# Certify shell/bash, SQL, Markdown, JSON/YAML, and Git-oriented language tooling at launch support levels — reviewer artifact

This is the human-readable reviewer artifact for the M4 stable
launch-language tooling truth packet. The machine-readable contract,
checked-in packet, schema, and fixture corpus are:

- Rust contract: `crates/aureline-language/src/launch_language_tooling_truth_packet/`
- Stable packet: `artifacts/language/m4/launch_language_tooling_truth_packet.json`
- Boundary schema: `schemas/language/launch_language_tooling_truth.schema.json`
- Reviewer doc: `docs/languages/m4/certify-shell-bash-sql-markdown-json-yaml-and.md`
- Fixture corpus: `fixtures/language/m4/launch_language_tooling_truth_packet/`

## Lane coverage

The stable packet certifies the following launch-tooling lanes at the
M4 launch-support grade:

- `shell_bash_lane` — shell and bash daily-loop coverage with shfmt
  formatting, ShellCheck linting, and trace-based debug review.
- `sql_lane` — vendor-agnostic SQL daily-loop coverage with formatter,
  validator, and explain-plan review; live query execution scoped out.
- `markdown_lane` — CommonMark and GFM daily-loop coverage with
  prettier/markdownlint formatting and lint review.
- `json_yaml_lane` — JSON and YAML daily-loop coverage with JSON
  Schema-driven completion and schema-bound migration of well-known
  configuration files.
- `git_oriented_lane` — `.gitconfig`, `.gitignore`, `.gitattributes`,
  commit message, and diff/patch daily-loop coverage. On-disk
  Git-oriented language tooling only; repository mutation is scoped
  out.

## What stable certification means

For each lane the packet asserts the full nine-step daily loop is
covered (open/import, navigate, edit, complete, refactor,
run/test/debug, review, migrate, recover) with bound support class,
evidence class, known-limit class, downgrade automation, confidence
class, evidence refs, and (where narrowed) a disclosure ref. Every
row excludes raw source bodies, secrets, and ambient authority. Every
required consumer projection (`editor_language_pack`,
`framework_pack_panel`, `language_settings`, `cli_headless`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) preserves the packet verbatim.

## How to reproduce

Run the unit tests and integration tests:

```
cargo test -p aureline-language launch_language_tooling
```

The integration tests in
`crates/aureline-language/tests/launch_language_tooling_truth_packet.rs`
load every fixture in
`fixtures/language/m4/launch_language_tooling_truth_packet/` and
assert that `LaunchLanguageToolingTruthPacket::materialize` agrees
with the expected promotion state, finding kinds, and closed-vocabulary
token sets. The integration tests also load the checked-in stable
packet and require that it validates cleanly, covers every required
lane, and preserves every required consumer projection.

## Known limits and downgrade automation

The packet does not claim replacement-grade daily-driver depth on any
of these launch-tooling lanes. The `launch_tooling_scope_only` known
limit is declared on rows where launch-tooling capability is certified
but replacement-grade daily-driver behavior (interactive REPL
debugging, full polyglot test orchestration, live query execution
against production datastores, direct repository mutation) is
intentionally excluded. The packet auto-narrows on missing fixtures,
missing archetypes, framework gaps, failed migrations, and low
confidence; it auto-blocks on missing evidence; and a row narrowed
below launch support always surfaces its disclosure ref.
