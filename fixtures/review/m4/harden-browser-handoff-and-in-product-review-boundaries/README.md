# Boundary-hardening fixtures

| Fixture | Scenario | Invariant checklist |
|---|---|---|
| `boundary_hardened_with_reversible_handoff.json` | Happy path: reversible typed browser handoff, local and provider agree, fresh boundary, enforceable ownership disclosed. | Provider identity disclosed; return path present and valid; handoff reversible; no hidden authority; raw escape hatches absent. |
| `boundary_degraded_missing_return_path.json` | Degraded: browser handoff is typed but missing return path; provider authoritative. | Missing return path explicitly flagged; boundary not hardened; actionable preview still available. |
| `boundary_degraded_hidden_authority.json` | Degraded: hidden provider authority detected behind local chrome; local and provider disagree. | Hidden authority detected; boundary degraded; no mutation authority widened silently. |
| `boundary_degraded_ownership_ambiguous.json` | Degraded: ownership signals conflict at boundary (advisory vs enforceable for same scope). | Ownership conflict present; boundary degraded; reversible handoff still valid. |
