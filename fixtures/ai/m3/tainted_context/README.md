# Tainted Context Beta Fixtures

These fixtures exercise the beta tainted-context packet consumed by AI,
review, docs/help, support export, and CLI surfaces.

The checked support export covers four policy outcomes:

| Fixture | Effective mode | Why |
|---|---|---|
| `tainted_context_beta_support_export.json` | `explain_only` | partial external docs cannot drive tool or write authority |
| `tainted_context_beta_support_export.json` | `local_only` | external tool output removes remote/provider widening |
| `tainted_context_beta_support_export.json` | `preview_only` | terminal output can produce a review preview but cannot apply |
| `tainted_context_beta_support_export.json` | `blocked` | suspicious pasted text attempted authority widening |

Raw bodies, paths, provider payloads, and credentials are excluded. Rows carry
opaque refs, detector tokens, policy decisions, and approval fence refs only.
