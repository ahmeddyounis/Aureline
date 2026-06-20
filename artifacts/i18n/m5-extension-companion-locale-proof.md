# M5 Extension And Companion Locale Support Proof

Canonical machine source:

- Support report: [`/fixtures/i18n/extension-companion-pack-compat/support_report.json`](../../fixtures/i18n/extension-companion-pack-compat/support_report.json)
- Schema: [`/schemas/i18n/extension-locale-pack.schema.json`](../../schemas/i18n/extension-locale-pack.schema.json)
- Runtime contract: `aureline_i18n::contributed_locale`
- Extensions projection: `aureline_extensions::{seeded_contributed_locale_manifests, host_stable_labels_protected}`
- Shell inspector: `aureline_shell::i18n::contributed_support::ContributedLocaleSupportView`
- Human page: [`/docs/i18n/extension-and-companion-localization.md`](../../docs/i18n/extension-and-companion-localization.md)

## What This Proves

Extension-owned and companion locale packs are first-class, governed
contributions, not English-only afterthoughts and not a hole in the trust model.
The seeded `ContributedLocaleSupportReport` proves three things the first-party
delivery lane cannot:

1. **Contributed packs declare themselves.** Each manifest carries an owner
   class (`extension_owned_pack` or `companion_overlay_pack`), a support mode, a
   compatibility build range, fallback behavior, owned surface families, and a
   reserved namespace.
2. **Contributed packs cannot replace host-stable labels.** Trust, policy,
   capability, and lifecycle vocabulary is host-owned and read-only to
   contributed packs; the report's `validate()` rejects any override attempt,
   namespace collision, or attempt to own policy/legal/recovery text.
3. **Contributed surfaces degrade truthfully and attributably.** Every support
   row resolves to an explicit apply-or-degrade decision with a reason,
   discloses missing support on a claimed localized profile, and is attributed
   to a localization issue source.

## Contributed Packs Degrade Fully, Never Partially

The seeded report exercises every degrade reason across both owner classes. The
decision is re-derived from the recorded inputs by `decide_contributed_support`
and re-checked by `validate()`, so a hand-edited row cannot misstate skew.

| Row | Owner | Decision | Reason | Effective locale | Issue source |
| --- | --- | --- | --- | --- | --- |
| `ext:notebook-charts:fr-fr` | extension | apply | `not_degraded` | `fr-FR` | extension |
| `ext:docs-helper:de-de` | extension | degrade | `pack_blocked_signature_failure` | `en-US` | extension |
| `ext:legacy-runner:ja-jp` | extension | degrade | `no_contributed_pack_for_locale` | `en-US` | extension |
| `ext:profiler-views:es-mx` | extension | degrade | `pack_build_outside_compatibility_range` | `en-US` | extension |
| `companion:browser-handoff:fr-fr` | companion | apply | `not_degraded` | `fr-FR` | companion |
| `companion:browser-handoff:ja-jp` | companion | degrade | `companion_scope_narrower_than_desktop` | `en-US` | companion |

A blocked, skewed, or absent pack drops every key to host source language rather
than applying a single stale string, so no surface ever sits half-localized with
mixed-language trust vocabulary.

## Host-Stable Labels Stay Canonical

All four host-stable label classes are guarded, and every manifest renders them
read-only (`may_override_host_stable_labels = false`). Every support row — even
the degraded ones — preserves `trust_label`, `policy_label`, `capability_label`,
and `lifecycle_label`. The seeded report's `guardrail_clean` summary flag is
`true`: no contributed pack overrides host-stable labels and every degrade is
disclosed without claiming a localized profile.

The validation gate enforces this. The inline tests prove that an override
attempt, a reserved-namespace collision, and an attempt to own policy/legal text
each fail validation.

## A Narrower Companion Is Disclosed, Not A Defect

The companion `ja-JP` row degrades for `companion_scope_narrower_than_desktop`.
That is the documented design: a companion surface may be narrower than desktop.
The row discloses the degrade and routes to source language, but it is **not**
counted as missing support and never claims a localized profile. This keeps the
contract from promising cross-client full localization parity it never intends to
deliver.

## Issue Source Attribution

The shell `ContributedLocaleSupportView` joins this report with the first-party
compatibility report so support tooling reads one view and attributes every
localization issue to its source: `first_party_pack`, `extension_pack`, or
`companion_overlay`. Each row exposes a same-surface "view original" route to the
host source-language text.

## How To Regenerate And Verify

```sh
# Regenerate the canonical fixture.
cargo run -q -p aureline-i18n --example dump_contributed_locale_support \
  > fixtures/i18n/extension-companion-pack-compat/support_report.json

# Fixture replay, host-protection, and degrade-handling checks.
cargo test -p aureline-i18n contributed
cargo test -p aureline-shell i18n
cargo test -p aureline-extensions locale
```
