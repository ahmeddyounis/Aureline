# Completion row, signature-help, snippet-session, and inline-hint contract

This document freezes how Aureline describes typing-loop assistance
before implementation turns provider accidents into product truth.
Completion rows, signature-help cards, snippet-session controls, code
lenses, inlay hints, inline test actions, and related inline semantic
metadata must stay attributable, keyboard-reviewable, side-effect
honest, and visibly degraded when their semantic basis narrows.

The goal is simple: convenience must not hide imports, generator work,
network lookups, stale signatures, low-confidence hints, or AI-influenced
ranking.

Machine-readable companions:

- [`/schemas/language/completion_row.schema.json`](../../schemas/language/completion_row.schema.json)
  — `completion_row_record`, the canonical row packet every completion
  list, detail pane, accessibility announcement layer, support export,
  and review-safe replay surface reads when it needs one stable answer
  to what this item inserts, where it came from, what side effects it
  carries, how it commits, whether docs exist, and why it is ranked
  where it is.
- [`/schemas/language/signature_help_state.schema.json`](../../schemas/language/signature_help_state.schema.json)
  — `signature_help_state_record`, the state packet for one active
  signature-help session, including source, freshness, overload
  presentation, active parameter visibility, keyboard-review posture,
  and suppression or blocked state.
- [`/schemas/language/inline_hint_state.schema.json`](../../schemas/language/inline_hint_state.schema.json)
  — `inline_hint_state_record`, the packet for one inline semantic
  metadata row or cluster, including hint class, confidence, density
  mode, precedence band, suppression reasons, and side-effect cues for
  action-capable inline affordances.
- [`/fixtures/language/completion_hint_cases/`](../../fixtures/language/completion_hint_cases/)
  — worked YAML fixtures covering the required scenarios.

This contract composes with and does not replace:

- [`/docs/language/provider_graph_and_arbitration_contract.md`](./provider_graph_and_arbitration_contract.md)
  — provider health, scope, freshness, arbitration, and provenance
  rules already frozen for language-capable surfaces.
- [`/docs/language/diagnostics_and_code_action_contract.md`](./diagnostics_and_code_action_contract.md)
  — diagnostic, code-action, suppression, and generated-impact truth
  that outranks decorative inline metadata and broad completion side
  effects.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — shared `Limited`, `Stale`, `Blocked`, and related downgrade
  language that this contract reuses rather than forks.
- [`/docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md)
  — large-file mode limits that inline hints and code lenses must obey.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 11.4 and 11.7,
  plus `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` sections
  16.58, 18.15, 18.5, 30.24, and 31.91.

If this document disagrees with the PRD, TAD, TDD, UI/UX spec, or the
linked contracts above, those upstream documents win and this contract
plus the companion schemas update in the same change.

## Why freeze this now

Typing-loop assistance is one of the easiest places to ship accidental
product truth:

- a local-word or stale cached suggestion can render like full semantic
  completion;
- a completion row can quietly add imports, rewrite surrounding text,
  or start a snippet session while the surface still looks like plain
  insert text;
- generator-backed snippets can look like harmless templates even when
  they derive structure from manifests, schemas, or remote metadata;
- signature help can stay visible after its provider fell back to cached
  state;
- code lenses and inlay hints can crowd out diagnostics, review state,
  or large-file safety constraints; and
- AI ranking overlays can quietly reshuffle deterministic results unless
  the boost stays attributable.

The design docs already require source labels, side-effect honesty,
keyboard-complete interaction, degraded-state disclosure, large-file
limits, and density-aware inline metadata. This contract turns those
requirements into one frozen vocabulary and export-safe packet family.

## Scope

Frozen at this revision:

- completion-row source taxonomy for local word, local index, language
  service, framework provider, snippet pack, history-based, limited
  fallback, and AI-assist lanes;
- completion-row fields for label, kind, source, insert posture,
  commit posture, docs availability, availability or deprecation state,
  confidence or degraded label, side-effect cues, and ranking
  attribution;
- signature-help state vocabulary for source, freshness, overload
  presentation, active-parameter visibility, documentation posture,
  keyboard-review stability, and suppression or blocked state;
- snippet-session disclosure rules, including placeholder count,
  transform or generator structure, multi-cursor compatibility, and
  exit semantics;
- inline-hint, code-lens, and related inline semantic metadata
  vocabularies for class, density mode, precedence band, suppression
  reason, confidence label, actionability, and side-effect cues;
- the precedence rules that force diagnostics, debug, merge, diff,
  review, test, and coverage state to outrank decorative hints; and
- the downgrade rules that make it structurally impossible for cached,
  partial, fallback, or blocked providers to masquerade as exact
  semantic truth.

Out of scope:

- the final ranking algorithm or ML feature weights;
- live LSP, framework, graph, snippet-engine, or inline-hint
  implementation work;
- visual styling, motion, iconography, or layout metrics beyond the
  vocabulary frozen here; and
- provider-specific engines for completion, signature help, snippets,
  or code lenses.

## 1. Completion rows

Completion is not one opaque suggestion list. Every row must state what
it is, where it came from, how it commits, what it changes, and why it
is being shown near the top.

### 1.1 Source kinds

These are the only completion source kinds protected surfaces may name.

| `source_kind_class` | Plain-language label | Canonical examples | Never imply |
|---|---|---|---|
| `local_word` | Local word | words from the active file or current unsaved buffer state | project-wide semantic proof |
| `local_index` | Local index | repo-local lexical index or warm non-semantic token corpus | exact current semantic truth |
| `language_service` | Language service | LSP completion item, typed symbol completion, import-aware semantic completion | framework certainty or broad write safety beyond the row posture |
| `framework_provider` | Framework provider | route/component/config-aware completion driven by framework metadata | that ordinary language-service or imported evidence proved the same claim |
| `snippet_pack` | Snippet pack | built-in, workspace, user, or extension snippet entry | plain-text insertion when snippet mode or transforms will start |
| `history_based` | History-based | prior accept history or local recency ranking | semantic authority by itself |
| `fallback_text` | Limited fallback | syntax or text fallback shown while semantic providers are missing or narrowed | equal confidence with a live semantic provider |
| `ai_assist` | AI assist | advisory model-generated completion row | authoritative semantic certainty |

Rules:

1. A completion row MUST expose `source_kind_class` even when ranking
   blends more than one signal.
2. `local_word`, `local_index`, and `fallback_text` rows MUST never
   carry a confidence label that implies current exact semantics.
3. `ai_assist` rows remain advisory even when they are accepted through
   the same keyboard route as other rows.

### 1.2 Required row fields

Every completion row must answer these questions mechanically:

| Question | Required field family |
|---|---|
| What string or symbol is this? | label, item kind |
| Where did it come from? | source kind, source display label, source ref when available |
| What happens on accept? | insert posture, commit posture |
| Does accept change anything beyond visible text? | side-effect cue set |
| Are docs available? | documentation availability and optional docs summary |
| Is it active, deprecated, unavailable, or blocked? | availability state plus optional reason summary |
| How trustworthy is the current row? | confidence or degraded label |
| Why is it ranked here? | ranking-attribution rows |
| Will it start snippet mode? | snippet-session preview when applicable |

### 1.3 Insert and commit posture

Insert posture describes *what accepting the row does*. Commit posture
describes *how the row may be accepted*.

Allowed insert postures:

| `insert_posture_class` | Meaning |
|---|---|
| `plain_insert` | Insert or replace visible text only. |
| `replace_range` | Replace the admitted completion range only. |
| `insert_with_import_edits` | Accept inserts text and adds import edits or equivalent same-file semantic edits. |
| `insert_with_additional_edits` | Accept inserts text and applies reviewable same-file or bounded additional edits. |
| `expand_snippet_session` | Accept inserts text and starts a snippet session with placeholder navigation. |
| `preview_required_multi_file` | Accept requires preview because created files, dependency/config changes, or broader writes are involved. |
| `blocked` | The row is visible for explanation or parity only and cannot currently be applied. |

Allowed commit postures:

| `commit_posture_class` | Meaning |
|---|---|
| `explicit_accept_only` | Accept only through an explicit completion action. |
| `standard_accept_keys` | Accept through the standard configured accept keys. |
| `commit_characters_allowed` | Punctuation or other commit characters may accept the row. |
| `manual_preview_required` | Acceptance routes through an explicit preview or review surface first. |
| `blocked` | No commit route is currently admissible. |

Rules:

1. `preview_required_multi_file` MUST pair with
   `manual_preview_required`. It may not masquerade as plain insert.
2. `blocked` rows MUST name why they are blocked.
3. If a row starts snippet mode, import edits, additional edits, or
   preview-required work, that fact MUST be visible before accept.

### 1.4 Docs, availability, and confidence labels

Completion rows use one availability state plus one confidence or
degraded label.

Allowed availability states:

| `availability_state_class` | Meaning |
|---|---|
| `available` | Row may be accepted under the stated posture. |
| `deprecated` | Row remains selectable but is visibly deprecated. |
| `unavailable` | Row is inspectable but not currently valid here. |
| `blocked_by_policy` | Policy or trust posture prevents use. |
| `blocked_by_mode` | Current mode such as large-file, read-only, restricted, or generated limits prevents use. |

Allowed confidence or degraded labels:

| `confidence_label_class` | Meaning |
|---|---|
| `semantic_current` | Backed by current semantic truth for the admitted scope. |
| `semantic_partial` | Semantics exist but scope, provider set, or certainty is narrowed. |
| `cached_recent` | Cached and still reviewable, but not current exact proof. |
| `stale` | Past the freshness floor. |
| `limited_fallback` | Narrow fallback without semantic proof. |
| `advisory_ai` | AI-generated or AI-ranked advisory posture. |
| `blocked` | A known row exists, but apply is blocked. |
| `unsupported` | The current language, mode, or environment cannot support the row honestly. |

Rules:

1. `semantic_current` is reserved for rows with current semantic basis
   for the admitted scope.
2. `cached_recent`, `stale`, `limited_fallback`, `blocked`, and
   `unsupported` MUST stay visibly weaker than `semantic_current`.
3. Rows sourced from cached, fallback, local-word, or AI-only lanes may
   not re-label themselves as `semantic_current`.

### 1.5 Ranking attribution

Ranking must stay explainable. The row source and the ranking signals
that moved the row are different fields.

Allowed ranking signals:

| `ranking_signal_class` | Meaning |
|---|---|
| `provider_base_rank` | Provider-supplied or default host ordering. |
| `prefix_or_fuzzy_match` | Prefix, fuzzy, or lexical match strength. |
| `history_recency` | User-history recency or prior accept frequency. |
| `framework_context` | Framework-aware contextual boost. |
| `diagnostic_fix_context` | Nearby diagnostic or quick-fix relevance. |
| `ai_boost` | Explicit AI ranking overlay. |
| `freshness_penalty` | Degradation because the row is cached, stale, or narrowed. |
| `policy_or_mode_penalty` | Demotion caused by policy, trust, large-file, or mode limits. |

Rules:

1. A ranking signal may boost or penalize a row, but it may not rewrite
   the row's `source_kind_class`.
2. If `ai_boost` contributes to ordering, the row MUST expose that
   attribution explicitly while keeping the underlying source kind
   intact.
3. Ranking updates may adapt as the user types, but the active row MUST
   remain stable enough for arrow-key review, screen-reader
   announcement, and rapid accept or undo cycles.

## 2. Signature-help sessions

Signature help keeps callable shape and the active argument visible
without forcing the user into a different mode.

### 2.1 Source kinds and presentation

Allowed signature-help source kinds:

| `source_kind_class` | Meaning |
|---|---|
| `language_service` | Live language-service signature help. |
| `framework_provider` | Framework-aware signature or callable contract metadata. |
| `snippet_metadata` | Snippet-authored placeholder or callable guidance. |
| `cached_snapshot` | Cached signature state shown with freshness notice. |
| `fallback_text` | Narrow text or syntax fallback without trusted semantic parameter truth. |

Allowed overload presentation classes:

| `overload_presentation_class` | Meaning |
|---|---|
| `single_signature` | One callable shape is active. |
| `multi_signature_with_active_index` | Several overloads exist and the active overload remains explicit. |
| `multi_signature_collapsed` | Several overloads exist, but only a compact active-overload summary is shown. |
| `signature_count_only` | Only the overload count is trustworthy enough to show. |

Allowed active-parameter visibility classes:

| `parameter_visibility_class` | Meaning |
|---|---|
| `active_parameter_highlighted` | The active parameter is exact and highlighted. |
| `active_parameter_with_truncation` | The active parameter remains visible, but the presentation is compacted. |
| `parameter_names_only` | Parameter list is visible, but exact active highlighting is unavailable. |
| `parameter_unavailable` | A signature row exists, but active-parameter truth cannot be claimed. |

Rules:

1. A visible signature-help state MUST expose the current overload count
   and the current active signature when known.
2. `cached_snapshot`, `snippet_metadata`, and `fallback_text` states
   MUST never claim `semantic_current`.
3. Signature help must remain non-blocking. It may pin or suppress
   itself, but it may not steal accept or dismiss semantics from the
   typing loop.

### 2.2 Keyboard-review posture

Allowed keyboard-review classes:

| `keyboard_review_class` | Meaning |
|---|---|
| `stable_during_parameter_entry` | The card stays stable while the user moves through current-argument typing. |
| `stable_until_context_change` | Stable until the callable or active argument materially changes. |
| `suppressed_during_ime` | Suppressed rather than risking IME or composition confusion. |
| `pinned_secondary_review` | Moved or pinned for deliberate review without blocking typing. |

Rules:

1. IME composition and dead-key confirmation must never be mistaken for
   signature-help acceptance or dismissal.
2. Cached or stale signature help MUST say so with the same degraded
   vocabulary used elsewhere.
3. The product may compact the card at high zoom or narrow width, but
   it may not hide the only active-parameter cue without saying that
   the parameter state is limited.

## 3. Snippet-session disclosure

Snippet sessions are part of editing, not hidden provider magic.

At this revision, snippet-session machine-readable disclosure lives
inside `completion_row_record` as `snippet_session_preview` rather than
as a standalone schema because the protected admission decision happens
at completion accept time.

Every row that starts snippet mode must disclose:

- placeholder count;
- snippet structure class (`plain_placeholders`,
  `mirrored_placeholders`, `transforms_present`, or
  `generator_backed`);
- multi-cursor compatibility (`full_sync`, `primary_caret_only`,
  `insert_all_without_placeholder_sync`, or `unsupported`);
- structural-effect summary; and
- exit semantics summary.

Rules:

1. A snippet-backed row MUST say whether `Tab` advances placeholders or
   whether ordinary indentation semantics continue after accept.
2. Generator-backed snippets MUST name that generator/scaffold posture
   before accept. If the generator creates files, changes dependencies,
   or mutates config, the row must escalate to
   `preview_required_multi_file` rather than pretending it is an
   ordinary snippet session.
3. Multi-cursor limitations MUST be visible before apply.
4. Snippet transforms, placeholder mirrors, and generator-backed
   structure must stay export-safe and summary-only. Raw generated text
   does not cross these boundary packets.

## 4. Inline hints, code lenses, and related inline metadata

Inline semantic metadata must help interpretation without stealing
typing budget or obscuring higher-priority truth.

### 4.1 Precedence bands

These are the only precedence bands protected inline metadata may use.

| `precedence_band_class` | Meaning |
|---|---|
| `diagnostic_or_debug` | Diagnostics, current-frame state, debug stop state, or equivalent high-priority execution truth. |
| `merge_diff_review` | Merge, conflict, diff, or review state. |
| `test_coverage_execution` | Test, run, benchmark, or coverage cues. |
| `code_lens_or_inlay` | Advisory code lenses, inlay hints, and similar inline semantic metadata. |
| `decorative_qualifier` | Non-actionable freshness or generated qualifiers. |

Rules:

1. `diagnostic_or_debug` outranks every other inline band.
2. `merge_diff_review` outranks convenience metadata.
3. `test_coverage_execution` outranks advisory code lenses and inlay
   hints.
4. `code_lens_or_inlay` and `decorative_qualifier` may never obscure
   caret, selection, or readable code content.

### 4.2 Density and suppression

Allowed density modes:

| `density_mode_class` | Meaning |
|---|---|
| `off` | Suppress non-essential inline metadata. |
| `compact` | Keep only critical hints and compact action cues. |
| `rich` | Show expanded hints, lenses, and detail affordances. |
| `auto` | Adapt by file size, zoom, confidence, and runtime posture. |

Allowed suppression reasons:

| `suppression_reason_class` | Meaning |
|---|---|
| `large_file_mode` | Large-file mode narrowed or removed inline metadata. |
| `high_zoom` | High zoom or constrained width reduced density. |
| `reduced_motion` | Reduced-motion posture suppressed non-essential animated or attention-grabbing hints. |
| `low_certainty` | Provider confidence fell below the display floor. |
| `read_only_or_generated` | Generated, read-only, protected, or narrowed-file posture hid unsafe affordances. |
| `higher_priority_decoration` | A higher-priority band consumed the available inline slot. |
| `policy_blocked` | Policy or trust posture prevented the hint or action. |
| `typing_budget_protection` | Host suppressed the hint to protect typing stability. |

Allowed visibility states:

| `visibility_state_class` | Meaning |
|---|---|
| `visible_primary` | Hint is visible without downgrade. |
| `visible_downgraded` | Hint is visible, but confidence or freshness is visibly downgraded. |
| `suppressed` | Hint is intentionally not shown. |
| `hidden_by_precedence` | Hint yielded to a higher-priority inline band. |

Rules:

1. Large-file, high-zoom, reduced-motion, low-certainty, generated, or
   read-only states may reduce hint density automatically, but the
   record must preserve why.
2. `semantic_partial`, `cached_recent`, `stale`, `limited`, and
   `blocked` inline hints may not render as `visible_primary`.
3. Cached or fallback inline metadata cannot claim semantic certainty
   when the provider graph is stale, partial, or blocked.

### 4.3 Actionability and side effects

Inline metadata sometimes launches real work. A code lens that opens a
review, runs a test, calls a generator, or requires remote lookup must
say so.

Allowed actionability classes:

| `actionability_class` | Meaning |
|---|---|
| `informational_only` | Purely informational. |
| `jump_or_peek` | Opens source, target, or related detail. |
| `run_or_test` | Launches runtime, test, or benchmark work. |
| `review_or_diff` | Opens review, diff, or mutation detail. |
| `preview_required_action` | Routes through explicit preview or governed review. |
| `blocked` | Visible for explanation only. |

Rules:

1. Action-capable inline metadata MUST expose side-effect cues for
   runtime launches, review surfaces, network lookups, generator work,
   config changes, or preview-first mutations.
2. Generated or protected files may narrow which inline actions stay
   visible so unsafe affordances are not suggested where they cannot be
   applied honestly.
3. Keyboard and screen-reader routes must remain available even when
   density reduces or inline actions suppress themselves.

## 5. Cross-surface downgrade rules

These rules apply across completion rows, signature-help states, and
inline metadata.

1. A surface MAY reuse cached, stale, fallback, or advisory answers,
   but it MUST label them as such before the user confuses them with
   live semantic truth.
2. No completion row, signature-help state, or inline hint may claim
   semantic exactness when the underlying provider graph is partial,
   stale, crash-looped, policy-blocked, or scope-narrowed.
3. AI influence must stay attributable. AI may provide a row or boost a
   row, but it may not silently repaint a deterministic source as if AI
   supplied the underlying semantic proof.
4. Hidden imports, hidden additional edits, hidden snippet sessions,
   hidden network calls, hidden generator work, and hidden dependency or
   config changes are forbidden.
5. When the only honest answer is suppression, blocked state, or
   preview-required escalation, the contract prefers that posture over a
   convenience-first lie.

## 6. Fixture coverage

The worked corpus freezes these required scenarios:

- auto-import completion with explicit import cue;
- generator-backed snippet with placeholder and structural disclosure;
- cached signature help with active overload and parameter visibility;
- suppressed code lenses in large-file mode; and
- AI-boosted ranking with explicit attribution while the row source
  remains deterministic.
