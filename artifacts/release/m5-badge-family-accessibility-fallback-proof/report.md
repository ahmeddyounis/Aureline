# M5 Badge-Family Accessibility & Auto-Narrowing

- Packet: `m5-badge-family-accessibility-fallback:stable:0001`
- As of: `2026-07-08T00:00:00Z`
- Families: 6 certified across 6 / 6 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:support-class-badge** (support_class) — family=support_class keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_claim effective_claim=full_claim status=parity
- **a11y:lifecycle-badge** (lifecycle) — family=lifecycle keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=supported effective_claim=supported status=parity
- **a11y:evidence-freshness-badge** (evidence_freshness) — family=evidence_freshness keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_claim effective_claim=provisional status=narrowed_disclosed
  - Auto-narrow: full_claim → provisional (dimension=evidence_freshness, trigger=evidence_freshness_hidden) — Evidence stale — freshness shown from last-known proof until re-verification lands
- **a11y:channel-badge** (channel) — family=channel keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_claim effective_claim=limited status=narrowed_disclosed
  - Auto-narrow: full_claim → limited (dimension=channel_posture, trigger=channel_value_unstated) — Channel reassignment in flight — channel shown limited until the new channel settles
- **a11y:deployment-scope-badge** (deployment_scope) — family=deployment_scope keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_claim effective_claim=imported status=narrowed_disclosed
  - Auto-narrow: full_claim → imported (dimension=deployment_scope, trigger=deployment_scope_unstated) — Deployment scope from imported mirror evidence — shown imported until locally re-proven
- **a11y:compatibility-state-badge** (compatibility_state) — family=compatibility_state keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_claim effective_claim=policy_blocked status=narrowed_disclosed
  - Auto-narrow: full_claim → policy_blocked (dimension=compatibility_state, trigger=compatibility_state_unstated) — Compatibility blocked by policy — host compatibility not confirmable until an approver signs off
