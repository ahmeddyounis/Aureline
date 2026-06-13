# M5 Shared Suspicious-Text Detector Parity

- Packet: `m5-suspicious-text-parity:stable:0001`
- Case: `case:m5-suspicious-text-parity:stable`
- Detector outcome: `sanitize`
- Finding classes: bidi_control, invisible_formatting, mixed_script_confusable
- Threat classes: hidden_codepoint_smuggling, identity_confusable_spoof, text_reordering_spoof

## Surfaces

- **notebook_output** (ordinary_browsing): 3 warning(s), raw inspection reachable: true
- **docs_browser_panel** (ordinary_browsing): 3 warning(s), raw inspection reachable: true
- **marketplace_install_update** (strong_decision_strict_identity): 3 warning(s), raw inspection reachable: true
- **remote_host_attach** (strong_decision_strict_identity): 3 warning(s), raw inspection reachable: true
- **collaboration_share** (strong_decision_strict_identity): 3 warning(s), raw inspection reachable: true
- **ai_evidence_viewer** (ordinary_browsing): 3 warning(s), raw inspection reachable: true
- **provider_policy_overlay** (strong_decision_strict_identity): 3 warning(s), raw inspection reachable: true

## Support/admin threat-class cues

- `text_reordering_spoof` (high): 7 warning(s) across 7 surface(s)
- `hidden_codepoint_smuggling` (high): 7 warning(s) across 7 surface(s)
- `identity_confusable_spoof` (critical): 7 warning(s) across 7 surface(s)
