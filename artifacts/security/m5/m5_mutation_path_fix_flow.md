# M5 Mutation-Path Fix Flows And Auditable Suppressions

- Packet: `m5-mutation-path-fix-flow:stable:0001`
- Case: `case:m5-mutation-path-fix-flow:stable`
- Paths: 4 · with findings: 4 · suppressed: 4
- Fix-flow modes: previewable_diff, review_sheet

## Mutation paths

- **save** → fix flow `previewable_diff` (Preview fix diff)
  - Findings: true · silent rewrite blocked: true · preview required: true
  - Fix kinds: bidi
  - Suppression: scope `file`, actor `actor:user:dana`, audit `audit:suppression:save:0001`
- **format** → fix flow `previewable_diff` (Preview fix diff)
  - Findings: true · silent rewrite blocked: true · preview required: true
  - Fix kinds: invisible
  - Suppression: scope `workspace`, actor `actor:user:dana`, audit `audit:suppression:format:0001`
- **organize_imports** → fix flow `previewable_diff` (Preview fix diff)
  - Findings: true · silent rewrite blocked: true · preview required: true
  - Fix kinds: confusable
  - Suppression: scope `occurrence`, actor `actor:user:dana`, audit `audit:suppression:organize:0001`
- **ai_apply** → fix flow `review_sheet` (Review proposed change)
  - Findings: true · silent rewrite blocked: true · preview required: true
  - Fix kinds: bidi, confusable, invisible
  - Suppression: scope `admin_policy`, actor `actor:admin:policy-owner`, audit `audit:suppression:ai-apply:0001`
