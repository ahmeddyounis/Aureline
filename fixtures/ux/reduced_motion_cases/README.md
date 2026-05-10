# Token / state / reduced-motion audit cases

This folder pins the **token-adoption, component-state, and reduced-motion**
audit cases for the protected M1 shell surfaces (shell chrome, Start Center,
search palette, and trust/scope-truth surfaces).

Each case is a YAML record (`token_state_audit_case_record`) that names:

- the canonical source files that render the surface,
- the `require_*` token IDs the surface MUST keep loading from the shared
  semantic-token registry (so theme/density/contrast switching keeps working),
- the `ComponentStates::*` symbols the surface MUST keep referencing (so
  focus-visible, selected, hover, etc. keep mapping to one shared treatment),
- the motion-preset fixtures the surface MUST keep referencing through the
  shared `aureline_ui::motion` module (so reduced-motion fallbacks remain
  intact), and
- a named **failure drill** with a forced input plus the `check_id` the audit
  must report when that input is forced — never a manual demo.

The cases are consumed by:

- `tests/ux/token_state_audit/run_token_state_audit.py` — the unattended
  runner that emits the durable JSON capture under
  `artifacts/milestones/m1/captures/`.
- `artifacts/ux/m1_token_and_motion_audit.md` — the reviewer-facing landing
  page anchoring the protected walk and failure drill.
- `artifacts/accessibility/m1/token_motion_review.md` — the accessibility
  review packet that cites the audit during M1 exit review.

The audit follows the project rule: **truth lives in the shared design-system
contracts**, never re-derived per surface. Every entry in a case must resolve
back to a checked-in token, state symbol, or motion-preset fixture.
