# M5 Attention And Lifecycle Localization Audit

This audit proves that durable-attention vocabulary — activity-center rows and
partitions, job-row badges, notification summaries, lifecycle-state copy, and
quiet-hours/admin suppression messaging — uses one governed localized vocabulary
across the claimed M5 profiles (es-MX, ja-JP, ar-SA), rather than per-surface or
per-locale wording drift.

## What This Proves

- **One governed vocabulary, not per-surface wording.** 44 durable-attention and
  lifecycle terms (34 durable) across 5 domains — lifecycle states, activity
  partitions, job-row badges, quiet-hours modes, and suppression reasons — each
  bind to a single stable `term_key`. The shell binds every canonical surface
  state to one of those keys, so the activity center and durable job rows share
  one `running`, and an activity row held by quiet hours, a quiet-hours
  suppression reason, and a quiet-hours fanout decision share one
  `held_quiet_hours`.
- **Severity, scope, and action order are stable across locales.** Severity rank
  and action-order index are locale-neutral metadata bound to the stable term,
  never derived from translated prose. Translation cannot make a state read
  softer or stronger, and cannot reorder it. Each domain's action order is dense
  from zero.
- **Truncation never hides severity.** Long translations truncate to the view
  budget (24 graphemes) without hiding the severity icon or scope; severity is
  preserved under truncation on every term and locale.
- **Support, docs, and exports map back to one glossary.** The metadata-only
  support export keeps the canonical token, the governed `term_key`, and the
  severity rank while omitting the translated body, so a localized attention or
  lifecycle state always maps back to one stable glossary entry without semantic
  drift.

## Canonical Machine Source

- Glossary truth: `aureline_i18n::AttentionVocabularyGlossary`
  (`seeded_attention_vocabulary_glossary`).
- Parity audit: `aureline_i18n::AttentionVocabularyParityReport`
  (`build_attention_vocabulary_parity_report`).
- Drift proof: `aureline_i18n::AttentionVocabularyDriftScenarioSet`.
- Shell binding/projection: `aureline_shell::i18n::attention_vocabulary`.
- Fixtures: `fixtures/i18n/activity-center-and-notifications/`.
- Schemas: `schemas/i18n/attention-lifecycle-glossary.schema.json`,
  `schemas/i18n/attention-localization-parity.schema.json`.
- Human glossary: `docs/i18n/attention-and-lifecycle-glossary.md`.

## Current Posture

Parity state: **green** (3 of 3 claimed locales governed; 0 narrowed; 0 blocked;
0 drift findings).

| Locale | Direction | Localized | Truncated | Max expansion | Claim |
| --- | --- | --- | --- | --- | --- |
| es-MX | left-to-right | 44 / 44 | 9 | 210% | green |
| ja-JP | left-to-right | 44 / 44 | 0 | 120% | green |
| ar-SA | right-to-left | 44 / 44 | 1 | 283% | green |

| Domain | Terms | Action order dense |
| --- | --- | --- |
| lifecycle_state | 10 | yes |
| activity_partition | 4 | yes |
| job_row_badge | 9 | yes |
| quiet_hours_mode | 10 | yes |
| suppression_reason | 11 | yes |

## Guardrail: Translation Cannot Change Severity

Six drift scenarios prove the gate blocks rather than ships when translation
alters meaning:

- a softened severity (a failed job translated as informational) → blocked;
- a strengthened severity (a completion translated as a failure) → blocked;
- a reordered action in the row sequence → blocked;
- broken lexical consistency (one source word translated two ways) → blocked;
- a term collision (two distinct states sharing one translation) → blocked;
- a severity hidden under truncation → blocked.

A source-language fallback narrows a locale (still understandable) rather than
blocking it; a severity, order, lexical, collision, or truncation drift blocks
it.

## Verification

```sh
cargo test -p aureline-i18n --test attention_vocabulary --locked
cargo test -p aureline-i18n --lib attention_vocabulary --locked
cargo test -p aureline-shell --lib i18n:: --locked
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- attention-vocab-validate
```

The records carry no raw provider payloads or credential bodies — only stable
term keys, locale-neutral severity ranks and order, localized labels, and
export-safe metrics.
