# M5 AI Mode Certification

- Packet: `m5-ai-mode-certification:stable:0001`
- Schema: `schemas/ai/certify-ai-workflow-scorecards-red-team-packs-and-downgrade-rules-for-each-shipped-m5-ai-mode.schema.json`
- Support export: `artifacts/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/support_export.json`
- Fixture: `fixtures/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/`

## Coverage

Each shipped M5 AI mode carries a workflow scorecard, a red-team pack, and downgrade rules.

- Inline edit, patch review, explain, and debug are certified Stable; every scorecard dimension passes its threshold.
- Test, refactor, and branch or worktree agents are certified Beta; no scorecard dimension fails its threshold.
- Every mode covers all seven trust scorecard dimensions and all eight required red-team vectors.
- The four always-applicable red-team vectors — prompt injection, tainted-context exfiltration, credential leak in export, and stale-evidence promotion — are blocked or mitigated on every claimed mode.
- Every mode carries a `proof_stale` downgrade rule that narrows the claim, plus required evidence packet refs.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The certification reuses the frozen M5 AI workflow matrix qualification and downgrade vocabularies. It proves that no certified mode can stay greener than its evidence: stale proof narrows the claim, every downgrade rule narrows rather than hides, and no claimed mode leaves an always-applicable attack vector uncovered.
