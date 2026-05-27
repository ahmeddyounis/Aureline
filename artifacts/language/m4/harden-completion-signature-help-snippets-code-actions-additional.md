# Harden completion, signature help, snippets, code actions, additional edits, source labeling, and AI-ghost-text boundaries across launch languages — reviewer artifact

This is the human-readable reviewer artifact for the M4 stable
assistant-surface hardening truth packet. The machine-readable
contract, checked-in packet, schema, and fixture corpus are:

- Rust contract: `crates/aureline-language/src/assistant_surface_hardening_truth_packet/`
- Stable packet: `artifacts/language/m4/assistant_surface_hardening_truth_packet.json`
- Boundary schema: `schemas/language/assistant_surface_hardening_truth.schema.json`
- Reviewer doc: `docs/languages/m4/harden-completion-signature-help-snippets-code-actions-additional.md`
- Fixture corpus: `fixtures/language/m4/assistant_surface_hardening_truth_packet/`

## Lane coverage

The stable packet certifies the following assistant-surface lanes at
the M4 launch-hardened grade across launch languages:

- `completion_lane` — deterministic completion plus cached/local-word
  fallback proposals, with provider/source class bound on every
  entry and side-effect class exposed when additional edits accompany
  the proposal.
- `signature_help_lane` — signature help rendered under IME,
  multi-cursor, large-file, restricted-mode, and degraded-provider
  cross-cut conditions.
- `snippet_session_lane` — snippet session entry/exit, placeholder
  navigation, and Tab semantics; surfaces snippet/source label,
  placeholder index/count, exit route, and multi-cursor compatibility.
- `code_action_lane` — code-action entries that preserve provider/
  source, side-effect class, partial-support reason, and preview
  requirement; preview required for multi-file, generated-file,
  dependency-changing, or protected-surface mutations.
- `additional_edit_lane` — admission of side-effect-bearing apply
  paths (additional edits, generated files, dependency/config
  changes, protected-surface writes) with bound preview requirement.
- `source_labeling_lane` — provider/source attribution distinguishing
  deterministic completion, cached/local-word fallback, snippet-only
  suggestion, and AI ghost text by stable provider/source rather than
  by theme accident.
- `ai_ghost_text_lane` — AI ghost-text inline proposals always
  rendered as `ai_ghost_text` provider/source; never collapsed into
  deterministic-completion or snippet-only paths.

## What stable certification means

For each lane the packet asserts the five required cross-cut
conditions (IME, multi-cursor, large-file, restricted-mode,
degraded-provider) are covered with bound support class, evidence
class, known-limit class, downgrade automation, confidence class,
evidence refs, and (where narrowed) a disclosure ref. The completion,
source labeling, and AI ghost-text lanes carry a provider_source_
binding row per required provider/source class. The snippet session
lane carries a snippet_session_truth row per required field. The code
action lane carries a code_action_truth row per required field. The
additional edit lane admits its side-effect-bearing classes with bound
preview requirements. Every row excludes raw source bodies, secrets,
and ambient authority. Every required consumer projection
(`editor_language_pack`, `framework_pack_panel`, `language_settings`,
`cli_headless`, `support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) preserves the packet verbatim.

## How to reproduce

Run the unit tests and integration tests:

```
cargo test -p aureline-language assistant_surface_hardening
```

The integration tests in
`crates/aureline-language/tests/assistant_surface_hardening_truth_packet.rs`
load every fixture in
`fixtures/language/m4/assistant_surface_hardening_truth_packet/` and
assert that `AssistantSurfaceHardeningTruthPacket::materialize` agrees
with the expected promotion state, finding kinds, and closed-vocabulary
token sets. The integration tests also load the checked-in stable
packet and require that it validates cleanly, covers every required
lane, preserves every required consumer projection, covers all
cross-cut conditions, binds every required provider/source class on
the completion/source-labeling/AI ghost-text lanes, surfaces every
required snippet-session field, and preserves every required code-
action field.

## Known limits and downgrade automation

The packet does not over-claim assistant-surface depth across every
launch language. The `side_effect_scope_only` known limit is declared
on rows where additional-edit or protected-surface admission is
certified for a documented scope only. The packet auto-narrows on
missing fixtures, missing archetypes, provider gaps, condition
failures, and ghost-text label drift; auto-demotes on low confidence;
auto-blocks on missing evidence; and a row narrowed below
launch-hardened always surfaces its disclosure ref.
