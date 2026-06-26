# Safety-Critical String Catalog and Controlled Terms

This document is the contract for the stable safety-critical string catalog. The
catalog is the single source of truth for the wording Aureline renders on its
safety-critical surfaces — trust prompts, degraded-state banners, Project Doctor
findings, AI review flows, execution-context sheets, support/export headings,
recovery action blocks, and runtime status. UI, CLI/help, docs, support exports, AI
surfaces, onboarding, Help/About, and narrated/durable surfaces resolve copy
through this catalog rather than inlining literal strings.

Where the [content-wording governance matrix](./freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md)
freezes *which* wording objects are governed, this catalog freezes the *actual*
objects: the concrete controlled terms and the concrete safety-critical messages.

- Record kind: `m5_safety_critical_string_catalog`
- Schema: [`schemas/content/m5-safety-critical-strings.schema.json`](../../../schemas/content/m5-safety-critical-strings.schema.json)
- Canonical support export: [`artifacts/content/m5-terminology-proof/support_export.json`](../../../artifacts/content/m5-terminology-proof/support_export.json)
- Summary artifact: [`artifacts/content/m5-terminology-proof/m5_safety_critical_string_catalog.md`](../../../artifacts/content/m5-terminology-proof/m5_safety_critical_string_catalog.md)
- Fixtures: [`fixtures/content/m5-safety-critical-strings/`](../../../fixtures/content/m5-safety-critical-strings/)
- Producer: `aureline_shell::m5_safety_critical_string_catalog::current_stable_safety_critical_string_catalog_export`
- Headless emitter: `aureline_shell_m5_safety_critical_strings`

## Controlled terms

A `ControlledTerm` is a first-class object: one reserved meaning, one locale-neutral
machine token, one canonical (default-locale) label, an alias posture, and the
surface families it may appear on. Every term is `never_softened` — its reserved
meaning is never weakened for tone. The term classes mirror the canonical state
vocabularies owned by the controlled glossary and the product truth vocabulary, so
the catalog reuses those tokens rather than minting parallel synonyms:

- **Trust** — `unverified_source`, `official_source`.
- **Policy** — `trust_required`, `policy_blocked`, `restricted`, `requires_review`.
  `Trust required`, `Policy blocked`, and `Restricted` are never softened into
  generic unavailability.
- **Compatibility** — `incompatible`, `minor_skew_compatible`.
- **Freshness** — `proven_current`, `cached`, `stale`, `warming`. A cached or stale
  value never implies proven-current authority.
- **Client scope** — `local_only`, `browser_companion`. A browser companion never
  implies full desktop parity.
- **Lifecycle** — `preview`, `beta`, `disabled_by_policy`.

## Safety-critical messages

A `SafetyCriticalMessage` carries a stable, locale-neutral `message_id`, a
`message_class`, an `audience`, a `severity`, the `surface_family` it renders on, the
controlled-term ids it embeds, its declared variables, a reference template, and
truncation guidance.

- **Audiences**: `end_user`, `operator`, `developer`, `support`, `screen_reader`.
- **Severities** (low → high): `info`, `notice`, `caution`, `warning`, `critical`,
  `blocking`.
- **Surface families**: `trust_prompt`, `degraded_state_banner`,
  `project_doctor_finding`, `ai_review_flow`, `execution_context_sheet`,
  `support_export_heading`, `recovery_action_block`, `runtime_status`.
- **Classes**: `safety_critical_string`, `error_recovery_block`, `action_label`,
  `ai_copy_line`, `count_scope_phrase`.

### Terminology lives in the glossary, not the literal string

A message never inlines a protected term. Its `reference_template` is built from
placeholders:

- `{term:<term_id>}` resolves against the glossary to the term's canonical label.
- `{var:<name>}` resolves against the message's declared variables.

This is what makes the catalog — not a scattered literal — the source of truth for
protected terminology. `render_reference` resolves a message's controlled terms
while leaving the variables as named slots, so the same term resolves identically
everywhere it appears.

### Variable semantics

Each `MessageVariable` declares a `role` (`controlled_term`, `count`, `scope_label`,
`entity_name`, `location`, `code`, `duration`), whether its *value* is locale-neutral
(codes, counts, durations are), and whether it is `truncatable`. A controlled-term
variable can never be truncatable.

### Truncation guidance

Each message carries `TruncationGuidance`: a strategy (`never_truncate`,
`truncate_variable_tail`, `truncate_variable_middle`, `priority_drop_trailing_clause`)
and the invariant `controlled_terms_never_dropped`. Truncation may shorten a
free-text variable, never a controlled term or the next safe action.

## Locale neutrality

Machine-facing identity stays locale-neutral while human prose localizes around it.
Message ids, term ids, machine tokens, variable names, and the `{term:...}` /
`{var:...}` placeholders are lowercase ascii (`[a-z0-9_.]`). Only the canonical
labels, reserved meanings, and reference templates carry human prose. The localized
overlay fixture rewrites every reference template into a pseudo-localized form while
keeping every id and placeholder byte-for-byte identical — proving a translation can
never fork the meaning of a lifecycle/trust/policy/runtime state.

## Cross-surface reuse

The same controlled-term objects are reused across trust prompts, degraded-state
banners, Project Doctor findings, AI review flows, execution-context sheets, and
support/export headings. The `shared_reuse_term_ids` — `trust_required`,
`policy_blocked`, `stale`, `cached` — must each appear on at least
`SHARED_TERM_MIN_REUSE_SURFACES` (3) distinct surface families. `cross_surface_reuse`
maps each term to the surfaces that embed it, and validation fails if a shared term
collapses to fewer surfaces.

## Validation invariants

`SafetyCriticalStringCatalog::validate` enforces, among others:

- record kind, schema version, and identity are present;
- the four closed inventories match the canonical token lists;
- term ids and machine tokens are unique and locale-neutral, and every term is
  `never_softened`;
- message ids are unique and locale-neutral, and every consumer-surface list is
  non-empty;
- every `{term:...}` / `{var:...}` placeholder resolves to a declared term/variable,
  and every declared term/variable is used by the template;
- a term is only embedded on a surface its `allowed_surfaces` permits;
- an `error_recovery_block` carries all four reserved parts (`what_failed`,
  `likely_cause`, `what_still_works`, `next_safe_action`);
- an `ai_copy_line` never overstates confidence or autonomy;
- a `count_scope_phrase` declares a count variable and discloses a freshness term;
- every audience, severity, and surface family is represented;
- each shared reuse term spans at least three surfaces;
- the trust-review and consumer-projection invariants all hold;
- the export carries no raw boundary material.

## Fixtures

The fixtures are valid, export-safe catalog packets minted from the same seed
builder as the canonical export by `aureline_shell_m5_safety_critical_strings`. See
[the fixtures README](../../../fixtures/content/m5-safety-critical-strings/README.md).
