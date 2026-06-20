# Attention And Lifecycle Vocabulary Glossary

Durable-attention surfaces must say the same thing in every locale. The activity
center, job-row badges, notification summaries, lifecycle-state copy, and
quiet-hours/admin suppression messaging all describe the same small set of
states — running, queued, needs approval, failed, held during quiet hours,
suppressed by an administrator — and a user who reads "failed" on one surface and
a softer word for the same state on another, or in another language, cannot trust
what they see.

This glossary is the single governed vocabulary those surfaces draw from. It is
not a new attention model and it does not rewrite the activity center; it governs
the words. Each durable-attention concept has exactly one stable term, one
canonical meaning, one locale-neutral severity, and one place in the action
order, and every surface and locale renders from that.

## The Single Governed Vocabulary

The canonical source is the checked-in glossary packet, not prose copied between
surfaces:

- Truth: `aureline_i18n::seeded_attention_vocabulary_glossary` and the
  `AttentionVocabularyGlossary` record.
- Fixture: `fixtures/i18n/activity-center-and-notifications/glossary.json`.
- Schema: `schemas/i18n/attention-lifecycle-glossary.schema.json`.

Every term carries:

- a stable, locale-neutral `term_key` (for example `attention.lifecycle.failed`)
  that support, docs, and exported packets anchor on;
- a `domain` — `lifecycle_state`, `activity_partition`, `job_row_badge`,
  `quiet_hours_mode`, or `suppression_reason`;
- a locale-neutral `severity_rank` and `action_order_index` that translation can
  never soften, strengthen, or reorder;
- one canonical source-language `definition` that fixes the meaning once; and
- per-locale translations carrying only the localized label plus the
  reviewer-asserted severity rank and order.

## Severity Ranks

Severity is metadata bound to the stable term, never derived from the translated
prose, so a translated label cannot make a state read softer or stronger than the
state it stands for. Ranks, from least to most severe:

`informational` < `in_progress` < `pending` < `success` < `needs_attention` <
`degraded` < `failure` < `critical`.

## Lifecycle States

The activity center (`ActivityRowStateClass`) and durable job rows
(`DurableJobRowStateClass`) share these. They do not each invent their own
wording: `running` is one term used by both.

| Term key | Severity | Meaning |
| --- | --- | --- |
| `attention.lifecycle.running` | in_progress | Work is actively executing. |
| `attention.lifecycle.queued` | pending | Work is waiting in the queue to start. |
| `attention.lifecycle.preparing` | in_progress | Work is preparing before execution. |
| `attention.lifecycle.needs_approval` | needs_attention | Work paused awaiting approval. |
| `attention.lifecycle.completed` | success | Work finished successfully. |
| `attention.lifecycle.partially_completed` | degraded | Some parts incomplete or skipped. |
| `attention.lifecycle.failed` | failure | Work ended in failure. |
| `attention.lifecycle.cancelled` | informational | Cancelled before completing. |
| `attention.lifecycle.superseded` | informational | Replaced by a newer run. |
| `attention.lifecycle.history_only` | informational | Terminal row kept for history. |

## Activity-Center Partitions

The grouping headers (`ActivityPartition`).

| Term key | Severity | Meaning |
| --- | --- | --- |
| `attention.partition.current_work` | in_progress | Work in progress now. |
| `attention.partition.needs_attention` | needs_attention | Work requiring a person to act. |
| `attention.partition.completed` | success | Finished work. |
| `attention.partition.suppressed_held` | informational | Work held or suppressed from active surfaces. |

## Job-Row Badges

The aggregate badge counts (`BadgeClass`). The running badge shares the source
word "Running" with the lifecycle state, so it translates identically.

| Term key | Severity |
| --- | --- |
| `attention.badge.needs_review` | needs_attention |
| `attention.badge.failed_runs` | failure |
| `attention.badge.mentions` | informational |
| `attention.badge.security_notices` | critical |
| `attention.badge.session_requests` | needs_attention |
| `attention.badge.offline_publish_pending` | pending |
| `attention.badge.durable_running_count` | in_progress |
| `attention.badge.held_or_suppressed_count` | informational |
| `attention.badge.completion_unread` | success |

## Quiet-Hours, Focus, And Admin Modes

The active attention mode (`QuietHoursMode`).

| Term key | Severity |
| --- | --- |
| `attention.quiet_hours.none` | informational |
| `attention.quiet_hours.quiet_hours` | informational |
| `attention.quiet_hours.do_not_disturb` | informational |
| `attention.quiet_hours.focus_mode` | informational |
| `attention.quiet_hours.presentation` | informational |
| `attention.quiet_hours.screen_share` | informational |
| `attention.quiet_hours.privacy_mode` | informational |
| `attention.quiet_hours.reduced_attention_policy` | informational |
| `attention.quiet_hours.power_saver` | informational |
| `attention.quiet_hours.admin_suppression` | needs_attention |

## Suppression Reasons

Why a fanout was held, suppressed, deduped, muted, or released
(`SuppressionReason`, `QuietHoursDecisionClass`). Mode-driven suppression reuses
the quiet-hours mode terms above rather than coining a new phrase.

| Term key | Severity |
| --- | --- |
| `attention.suppression.not_suppressed` | informational |
| `attention.suppression.held_quiet_hours` | informational |
| `attention.suppression.suppressed_by_policy` | needs_attention |
| `attention.suppression.admin_suppression` | needs_attention |
| `attention.suppression.deduped` | informational |
| `attention.suppression.muted_by_user` | informational |
| `attention.suppression.snoozed_by_user` | informational |
| `attention.suppression.reduced_attention` | informational |
| `attention.suppression.power_saver_paused` | informational |
| `attention.suppression.released_from_hold` | informational |
| `attention.suppression.critical_bypass` | critical |

## How Surfaces Bind To The Vocabulary

The shell projects every canonical durable-attention state onto a governed term:
`aureline_shell::i18n::attention_vocabulary` (`project_user_attention_vocabulary`
for surfaces, `project_support_attention_vocabulary` for the metadata-only
support export). The binding is exhaustive: adding a state to a canonical surface
enum fails to compile until it is bound to a glossary term. Cross-surface
agreement is checked — an activity row's `running` and a durable job row's
`running` resolve to one term key; an activity row held by quiet hours, a
quiet-hours suppression reason, and a fanout quiet-hours decision all resolve to
`attention.suppression.held_quiet_hours`.

## Mapping Localized States Back To The Glossary

Support scripts, docs, and exported packets never reverse-engineer a localized
string. The metadata-only support export keeps the canonical token, the governed
`term_key`, and the severity rank while omitting the translated body, so a
localized attention or lifecycle state always maps back to one stable glossary
entry.

## Parity And Drift Are Release-Gated

The glossary is audited per locale, and a softened or strengthened severity, a
reordered action, broken lexical consistency, a term collision, or a severity
hidden under truncation blocks the locale instead of shipping. See
`artifacts/i18n/m5-attention-localization-audit.md`.

## Verification

```sh
cargo test -p aureline-i18n --test attention_vocabulary --locked
cargo test -p aureline-i18n --lib attention_vocabulary --locked
cargo test -p aureline-shell --lib i18n:: --locked
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- attention-vocab-validate
```
