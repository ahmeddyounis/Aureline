# M5 Loading / Pending / Warning-Error / Degraded State-Block Contract Primitive

- Packet: `m5-loading-pending-degraded-state-contract-primitive:stable:0001`
- Label: `M5 loading / pending / warning-error / degraded state-block contract primitive: block kind, degraded state (loading/pending/warning-error/degraded), derived presentation posture, warning-vs-error severity, required non-color cues, required disclosures (state cause / owner / block reason / recovery action), recovery-disclosure class, and the loading-vs-pending / warning-vs-error / error-vs-degraded distinctness plus submission-lineage, what-still-works, and next-safe-action guarantees`
- Blocks: 6 (6 stable)
- Presentations: loading_treatment, pending_treatment, warning_error_treatment, degraded_treatment
- Non-color cues: loading_progress_indicator, pending_submission_attribution, warning_consequence_glyph, error_consequence_glyph, degraded_reduced_capability_glyph, recovery_affordance
- Degraded states: loading, pending, warning_error, degraded
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Blocks

- **Form**: `stable`
  - Owner: Form workflow owner
  - Scope: The form renders the shared degraded-state contract so a submitted save action shows as pending — attributed to the exact user action, not a generic background spinner — and a validation error names its consequence, its recovery path, and keeps the submission lineage so the activity center and support export can reconstruct what the user did
  - Worked states: 2
    - `block:settings-form.save-workspace` (`pending` / `informational`) → `pending_treatment` (non-color cues 1, submitted `true`, explainable `false`, recovery `true`)
    - `block:settings-form.save-workspace` (`warning_error` / `error`) → `warning_error_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
- **Background Job Row**: `stable`
  - Owner: Activity center owner
  - Scope: The background job row renders the shared degraded-state contract so a running job shows background loading progress with no submission attribution it does not own, and a failed job names its consequence, its retry path, and the submission lineage of the run — a health regression the activity center and support export can reconstruct, never a bare spinner that hides the failure
  - Worked states: 2
    - `block:activity.index-rebuild-run` (`loading` / `informational`) → `loading_treatment` (non-color cues 1, submitted `false`, explainable `false`, recovery `false`)
    - `block:activity.index-rebuild-run` (`warning_error` / `error`) → `warning_error_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
- **Banner**: `stable`
  - Owner: Shell banner owner
  - Scope: The banner renders the shared degraded-state contract so a sync-behind warning names its consequence and recovery path without blocking the workflow, and an offline degraded banner names its reduced fallback scope and what still works — a warning glyph or a reduced-capability glyph with an explicit next safe action, never a color-only banner that collapses a warning into a hard error
  - Worked states: 2
    - `block:shell-banner.sync-behind` (`warning_error` / `warning`) → `warning_error_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
    - `block:shell-banner.offline-mode` (`degraded` / `reduced`) → `degraded_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
- **Card**: `stable`
  - Owner: Dashboard card owner
  - Scope: The card renders the shared degraded-state contract so a loading card shows honest background progress and a degraded card names its lowered freshness, what still works, and the refresh path — never presenting a stale or partial card as fully fresh, and never a color-only dimming that hides the reduced certainty
  - Worked states: 2
    - `block:dashboard.throughput-card` (`loading` / `informational`) → `loading_treatment` (non-color cues 1, submitted `false`, explainable `false`, recovery `false`)
    - `block:dashboard.throughput-card` (`degraded` / `reduced`) → `degraded_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
- **Dense Row**: `stable`
  - Owner: Dense collection owner
  - Scope: The dense row renders the shared degraded-state contract so an inline edit shows as pending attributed to the exact user action rather than generic loading, and a partial-data row names its degraded scope, what still works, and the recovery path — never a spinner that hides which action is in flight and never a color-only treatment that collapses pending into loading
  - Worked states: 2
    - `block:results-row.rename-item` (`pending` / `informational`) → `pending_treatment` (non-color cues 1, submitted `true`, explainable `false`, recovery `true`)
    - `block:results-row.partial-row` (`degraded` / `reduced`) → `degraded_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
- **Review Sheet**: `stable`
  - Owner: Review workflow owner
  - Scope: The review sheet renders the shared degraded-state contract so a policy-blocked approval names its consequence, its recovery path, and the submission lineage of the approval attempt, and a reduced-context review names what still works and states honestly when no recovery is available — never an error toast that drops the submission lineage or a degraded sheet that hides how much context is missing
  - Worked states: 2
    - `block:review-sheet.approve-change` (`warning_error` / `error`) → `warning_error_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `true`)
    - `block:review-sheet.reduced-context` (`degraded` / `reduced`) → `degraded_treatment` (non-color cues 2, submitted `false`, explainable `true`, recovery `false`)
