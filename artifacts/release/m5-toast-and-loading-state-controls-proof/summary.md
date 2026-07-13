# M5 Toast and Loading-State Controls

- Packet: `m5-toast-and-loading-state-controls:stable:0001`
- Label: `M5 toast and loading-state controls with acknowledgement-only semantics, one bounded action where appropriate, durable-object back-links whenever the outcome matters after dismissal, skeleton / retained-content / stable-placeholder / partial-streaming / blocked-waiting loading treatments rather than one spinner, and no toast-only truth or full-screen spinner across shell, review, settings, help, support, and support-export surfaces`
- Consumer surfaces: 6
- Toast durabilities: transient_acknowledgment, mirrored_to_activity_center, dismissible_by_user, auto_dismiss_timed, action_retained_elsewhere, toast_only_truth_disallowed
- Loading treatments: skeleton, retained_previous_content, stable_placeholder, partial_results_streaming, blocked_waiting, treatment_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell / entry surface owner
  - Scope: The shell toast is a transient confirmation, and its loading state is a layout-preserving skeleton; both degrade honestly when a durable backlink is missing or a useful pane is blanked
  - Toast examples: 2 / loading-state examples: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review toast is a durable-outcome acknowledgement backed by the review queue, and its loading state retains the previous results while refreshing; both degrade honestly when the toast becomes the only durable truth or a full-screen spinner is used
  - Toast examples: 2 / loading-state examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings toast is a reversible-action acknowledgement with one bounded action backed by a record, and its loading state is a stable placeholder; both degrade honestly when the durability is toast-only or readiness is overclaimed
  - Toast examples: 2 / loading-state examples: 2
- **help_ui**: `stable`
  - Owner: Help surface owner
  - Scope: The help toast is a non-blocking notice, and its loading state streams partial search results; both degrade honestly when the acknowledgement scope is unresolved or partial content is not preserved
  - Toast examples: 2 / loading-state examples: 2
- **support_ui**: `stable`
  - Owner: Support surface owner
  - Scope: The support toast is a background handoff backed by a support record, and its loading state names a blocked-waiting state; both degrade honestly when a present action is unbounded or the loading purpose is unstated
  - Toast examples: 2 / loading-state examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved toast and loading-state truth, so a screenshot-only toast or loading state is visible in evidence rather than hidden, and the reason a toast appeared or a loading state persisted can be reconstructed at capture time without losing object identity
  - Toast examples: 2 / loading-state examples: 2
