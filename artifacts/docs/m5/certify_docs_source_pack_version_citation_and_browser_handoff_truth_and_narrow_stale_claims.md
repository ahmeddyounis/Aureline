# M5 Documentation-Claim Certification

- Packet: `m5-docs-claim-certification:stable:0001`
- Label: `M5 Documentation-Claim Certification`
- Profiles: 5 (5 certified, 0 narrowed/retest-pending/held/blocked)
- Evidence classes: 5 (each covered by at least one profile)
- Downgrade rules: 7 (auto-enforced)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Profiles

- **docs_browser**: `stable` / `certified`
  - Scope: Documentation browser, search, and result rows: source class, version match, freshness, and mirror/offline posture stay visible; docs-pack rows carry lifecycle state; external views route through a disclosed browser handoff
  - Evidence: source_class, docs_pack_lifecycle, version_match, browser_handoff
- **help_about**: `stable` / `certified`
  - Scope: Help / About / service-health surface: explains which documentation source and version back each answer and routes external help through a disclosed browser handoff
  - Evidence: source_class, version_match, browser_handoff
- **onboarding_learning**: `beta` / `certified`
  - Scope: Onboarding, learning, glossary, and guided-tour surface: glossary cards and tour steps carry a citation basis and disclose source class and version match
  - Evidence: source_class, version_match, citation_set
- **ai_explanation**: `beta` / `certified`
  - Scope: AI explanation surface: every derived explanation binds to a citation set, discloses source class and version match, and routes provider-console exits through a disclosed browser handoff
  - Evidence: source_class, version_match, citation_set, browser_handoff
- **support_export**: `stable` / `certified`
  - Scope: Support / export packet surface: carries source-class, docs-pack-lifecycle, version-match, citation-set, and browser-handoff truth from one packet set without raw document bodies or provider payloads
  - Evidence: source_class, docs_pack_lifecycle, version_match, citation_set, browser_handoff
