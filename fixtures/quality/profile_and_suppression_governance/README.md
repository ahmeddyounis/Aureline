# Quality Profile and Suppression Governance Fixtures

These fixtures anchor the governed quality plane implemented by
`aureline-runtime::quality` and the schemas under
[`/schemas/quality/`](../../../schemas/quality/).

The examples cover:

- effective profile resolution with policy locks, command overrides,
  workspace settings, imported tool-native config, and fallback defaults;
- quality-action proposal admission for trivia-only, broad, and
  policy-escalated actions;
- one quality session that uses the same proposal vocabulary as UI, CLI,
  review, and support surfaces;
- governed suppression and baseline records with owner, actor, expiry or
  review behavior, evidence refs, release visibility, and reopen rules;
- release-visible debt packets that preserve effective profile refs, save
  participant ordering, fix-safety classes, preview thresholds, support-export
  refs, and separate suppressed versus baselined debt counts.

IDs are opaque examples and are not planning identifiers.
