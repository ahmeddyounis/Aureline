# M5 Iconography and Illustration Registries

- Packet: `m5-iconography-and-illustration-registries:stable:0001`
- Label: `M5 iconography and illustration registries with canonical shell / action / status / navigation / file-type / trust-overlay icon meaning classes, tooltip and accessible-label parity, stable metaphor reuse, distinct file-type-versus-shell/status boundaries, and secondary, non-anthropomorphic illustration that never impersonates operational or security truth across shell, explorer, tab, result-row, onboarding, and support surfaces`
- Consumer surfaces: 6
- Meaning classes: shell_icon, action_icon, status_icon, navigation_icon, file_type_icon, trust_status_overlay, meaning_unclassified
- Illustration placements: empty_state_secondary, onboarding_secondary, decorative_accent, calm_non_anthropomorphic, subordinate_to_messaging, none_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves its chrome and navigation icons through the canonical semantic-labeled grammar and keeps its decorative accent secondary; an unlabeled destructive icon and an illustration that impersonates a security shield both degrade honestly instead of reading as a clean pass
  - Icon entries: 3 / illustration entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor renders its action icons in the tab strip with tooltip parity and keeps its tab accent calm and secondary; a file-type icon that collapses into shell / status meaning and an illustration that outgrows its secondary boundary both degrade honestly
  - Icon entries: 2 / illustration entries: 2
- **onboarding_ui**: `stable`
  - Owner: Onboarding surface owner
  - Scope: The onboarding wizard renders status icons in result rows and keeps its welcome illustration a secondary onboarding accent; an unclassified icon meaning class and an illustration that carries no placement both degrade honestly instead of standing in for state
  - Icon entries: 2 / illustration entries: 2
- **marketplace_ui**: `stable`
  - Owner: Marketplace / explorer surface owner
  - Scope: The explorer renders distinct file-type icons and keeps its empty-state illustration a secondary accent; a private extension icon grammar inlined instead of a canonical token and an illustration that replaces the operational messaging both degrade honestly
  - Icon entries: 2 / illustration entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings and result surfaces render trust / status overlays distinct from shell and file-type icons and keep the empty-results illustration subordinate to the messaging; an unstable icon metaphor and an illustration with an unstated token both degrade honestly
  - Icon entries: 2 / illustration entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved icon and illustration truth, so an unlabeled icon, a boundary collapse, or an illustration standing in for operational truth is visible in evidence rather than hidden behind a bare glyph
  - Icon entries: 2 / illustration entries: 2
